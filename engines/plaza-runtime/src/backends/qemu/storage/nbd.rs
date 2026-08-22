use plaza_foundation::core::{PlazaError, PlazaResult};
use crate::RuntimeStorage;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(unix)]
use tokio::net::{UnixListener as LocalListener, UnixStream as LocalStream};

#[cfg(windows)]
use tokio::net::{TcpListener as LocalListener, TcpStream as LocalStream};

const NBD_MAGIC: u64 = 0x4e42444d41474943;
const NBD_OPTS_MAGIC: u64 = 0x49484156454F5054;
const NBD_REP_MAGIC: u64 = 0x3e889045565a9;

const NBD_FLAG_FIXED_NEWSTYLE: u16 = 1 << 0;
const NBD_FLAG_NO_ZEROES: u16 = 1 << 1;

const NBD_FLAG_HAS_FLAGS: u16 = 1 << 0;
const NBD_FLAG_SEND_FLUSH: u16 = 1 << 2;

const NBD_OPT_EXPORT_NAME: u32 = 1;
const NBD_OPT_ABORT: u32 = 2;

const NBD_CMD_READ: u16 = 0;
const NBD_CMD_WRITE: u16 = 1;
const NBD_CMD_DISC: u16 = 2;
const NBD_CMD_FLUSH: u16 = 3;

/// A lightweight NBD server that translates NBD protocol requests
/// into VirtualBlockDevice operations over a Unix domain socket.
pub struct NbdServer {
    socket_path: PathBuf,
    storage: RuntimeStorage,
}

impl NbdServer {
    pub fn new(socket_path: PathBuf, storage: RuntimeStorage) -> Self {
        Self {
            socket_path,
            storage,
        }
    }

    #[cfg(unix)]
    pub async fn run(&self) -> PlazaResult<()> {
        if self.socket_path.exists() {
            tokio::fs::remove_file(&self.socket_path)
                .await
                .map_err(PlazaError::process)?;
        }
        let listener = LocalListener::bind(&self.socket_path).map_err(PlazaError::process)?;
        self.accept_loop(listener).await
    }

    #[cfg(windows)]
    pub async fn run(&self) -> PlazaResult<()> {
        // Use a random port on localhost for Windows since Unix domain sockets
        // aren't straightforward for NBD on Windows.
        let listener = LocalListener::bind("127.0.0.1:0").await.map_err(PlazaError::process)?;
        
        // Write the port to the socket_path so the client knows where to connect
        let port = listener.local_addr().map_err(PlazaError::process)?.port();
        tokio::fs::write(&self.socket_path, port.to_string())
            .await
            .map_err(PlazaError::process)?;

        self.accept_loop(listener).await
    }

    async fn accept_loop(&self, listener: LocalListener) -> PlazaResult<()> {
        loop {
            let (stream, _) = listener.accept().await.map_err(PlazaError::process)?;
            let storage = self.storage.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, storage).await {
                    eprintln!("NBD connection error: {:?}", e);
                }
            });
        }
    }
}

async fn handle_connection(mut stream: LocalStream, storage: RuntimeStorage) -> PlazaResult<()> {
    // 1. Initial Handshake
    stream.write_u64(NBD_MAGIC).await.map_err(PlazaError::process)?;
    stream
        .write_u64(NBD_OPTS_MAGIC)
        .await
        .map_err(PlazaError::process)?;
    stream
        .write_u16(1) // NBD_FLAG_FIXED_NEWSTYLE
        .await
        .map_err(PlazaError::process)?;

    // 2. Client Flags
    let client_flags = stream.read_u32().await.map_err(PlazaError::process)?;
    eprintln!("NBD client flags: {}", client_flags);

    // 3. Option Haggling
    loop {
        let magic = stream.read_u64().await.map_err(PlazaError::process)?;
        if magic != NBD_OPTS_MAGIC {
            eprintln!("Invalid NBD option magic: {:x}", magic);
            return Err(PlazaError::process("Invalid NBD option magic"));
        }
        let opt = stream.read_u32().await.map_err(PlazaError::process)?;
        let len = stream.read_u32().await.map_err(PlazaError::process)?;
        eprintln!("NBD Option: {}, len: {}", opt, len);

        if opt == NBD_OPT_EXPORT_NAME {
            eprintln!("NBD_OPT_EXPORT_NAME");
            let mut name = vec![0u8; len as usize];
            stream.read_exact(&mut name).await.map_err(PlazaError::process)?;

            // Send export details
            let size = {
                let dev = storage.device.lock().await;
                dev.size()
            };
            stream.write_u64(size).await.map_err(PlazaError::process)?;
            stream
                .write_u16(NBD_FLAG_HAS_FLAGS | NBD_FLAG_SEND_FLUSH)
                .await
                .map_err(PlazaError::process)?;

            if (client_flags & 2) == 0 {
                // NBD_FLAG_C_NO_ZEROES not requested
                let zeroes = [0u8; 124];
                stream.write_all(&zeroes).await.map_err(PlazaError::process)?;
            }
            break;
        } else if opt == NBD_OPT_ABORT {
            eprintln!("NBD_OPT_ABORT");
            return Ok(());
        } else {
            eprintln!("Rejecting unsupported option {}", opt);
            // Reject unsupported options
            let mut data = vec![0u8; len as usize];
            stream.read_exact(&mut data).await.map_err(PlazaError::process)?;

            stream
                .write_u64(NBD_REP_MAGIC)
                .await
                .map_err(PlazaError::process)?;
            stream.write_u32(opt).await.map_err(PlazaError::process)?;
            stream.write_u32(0x80000001).await.map_err(PlazaError::process)?; // NBD_REP_ERR_UNSUP
            stream.write_u32(0).await.map_err(PlazaError::process)?;
        }
    }
    eprintln!("NBD Handshake completed");

    // 4. Transmission Phase
    loop {
        let magic = stream.read_u32().await.map_err(PlazaError::process)?;
        if magic != 0x25609513 {
            // NBD_REQUEST_MAGIC
            return Err(PlazaError::process("Invalid NBD request magic"));
        }

        let _flags = stream.read_u16().await.map_err(PlazaError::process)?;
        let type_ = stream.read_u16().await.map_err(PlazaError::process)?;
        let handle = stream.read_u64().await.map_err(PlazaError::process)?;
        let offset = stream.read_u64().await.map_err(PlazaError::process)?;
        let length = stream.read_u32().await.map_err(PlazaError::process)?;

        match type_ {
            NBD_CMD_READ => {
                eprintln!("NBD_CMD_READ offset={} len={}", offset, length);
                let mut buf = vec![0u8; length as usize];
                let dev = storage.device.lock().await;
                dev.read_at(offset, &mut buf).await?;
                drop(dev);

                stream
                    .write_u32(0x67446698)
                    .await
                    .map_err(PlazaError::process)?; // NBD_SIMPLE_REPLY_MAGIC
                stream.write_u32(0).await.map_err(PlazaError::process)?; // error code 0
                stream.write_u64(handle).await.map_err(PlazaError::process)?;
                stream.write_all(&buf).await.map_err(PlazaError::process)?;
            }
            NBD_CMD_WRITE => {
                eprintln!("NBD_CMD_WRITE offset={} len={}", offset, length);
                let mut buf = vec![0u8; length as usize];
                stream.read_exact(&mut buf).await.map_err(PlazaError::process)?;

                let mut dev = storage.device.lock().await;
                dev.write_at(offset, &buf).await?;
                drop(dev);

                stream
                    .write_u32(0x67446698)
                    .await
                    .map_err(PlazaError::process)?;
                stream.write_u32(0).await.map_err(PlazaError::process)?;
                stream.write_u64(handle).await.map_err(PlazaError::process)?;
            }
            NBD_CMD_FLUSH => {
                let mut dev = storage.device.lock().await;
                dev.flush().await?;
                drop(dev);

                stream
                    .write_u32(0x67446698)
                    .await
                    .map_err(PlazaError::process)?;
                stream.write_u32(0).await.map_err(PlazaError::process)?;
                stream.write_u64(handle).await.map_err(PlazaError::process)?;
            }
            NBD_CMD_DISC => {
                break;
            }
            _ => {
                eprintln!("Unknown NBD command: {}", type_);
                // Return error for unsupported commands
                stream
                    .write_u32(0x67446698)
                    .await
                    .map_err(PlazaError::Io)?;
                stream.write_u32(22).await.map_err(PlazaError::Io)?; // EINVAL
                stream.write_u64(handle).await.map_err(PlazaError::Io)?;
            }
        }
    }

    Ok(())
}
