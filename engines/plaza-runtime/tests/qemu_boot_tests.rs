//! Progressive Boot Acceptance Tests for the QEMU backend.
//!
//! Tests 1–2: Discovery and TCG configuration (no QEMU binary needed).
//! Test 3: NBD server construction (no QEMU binary needed).
//! Tests 4+: Require a live QEMU and are skipped when unavailable.

use plaza_foundation::config::machine_section::{CpuSection, MachineSection, MemorySection};
use plaza_foundation::core::CapabilityPolicy;
use plaza_runtime::MachineConfig;
use plaza_runtime::backends::qemu::adapter::QemuAdapter;
use std::path::PathBuf;

fn test_machine_config() -> MachineConfig {
    MachineConfig {
        workspace_id: "test-ws".into(),
        instance_id: "test-instance".into(),
        machine: MachineSection {
            cpu: CpuSection { cores: 2 },
            memory: MemorySection {
                size: "512MiB".into(),
            },
            ..MachineSection::default()
        },
        capabilities: CapabilityPolicy::default(),
        boot_device: PathBuf::from("/tmp/test-boot.img"),
        ..Default::default()
    }
}

// ─── Test 1: QEMU discovery ──────────────────────────────────────────

#[test]
fn test_1_qemu_discovered() {
    let output = std::process::Command::new("qemu-system-x86_64")
        .arg("--version")
        .output();

    match output {
        Ok(out) => {
            let version = String::from_utf8_lossy(&out.stdout);
            println!("QEMU found: {}", version.trim());
            assert!(out.status.success());
        }
        Err(e) => {
            println!("QEMU not found in PATH (ok for CI): {}", e);
        }
    }
}

// ─── Test 2: TCG configuration generation ────────────────────────────

#[test]
fn test_2_tcg_config_generated() {
    let config = test_machine_config();
    let adapter = QemuAdapter::new(PathBuf::from("qemu-system-x86_64"));
    let adapter = adapter.apply_config(&config).unwrap();
    let (binary, args) = adapter.into_command();

    let args_str: Vec<String> = args.iter().map(|a| a.to_string_lossy().to_string()).collect();

    // Binary path
    assert_eq!(binary, PathBuf::from("qemu-system-x86_64"));

    // MUST have TCG acceleration — no KVM, no WHPX, no HVF
    assert!(args_str.contains(&"-accel".to_string()));
    assert!(args_str.contains(&"tcg,thread=multi".to_string()));

    // MUST be headless
    assert!(args_str.contains(&"-nodefaults".to_string()));
    assert!(args_str.contains(&"-display".to_string()));
    assert!(args_str.contains(&"none".to_string()));

    // SMP / memory
    assert!(args_str.contains(&"-smp".to_string()));
    assert!(args_str.contains(&"2".to_string()));
    assert!(args_str.contains(&"-m".to_string()));
    assert!(args_str.contains(&"512M".to_string()));

    // Boot drive
    assert!(args_str.contains(&"-drive".to_string()));

    // Architecture constraint: MUST NOT contain hardware-virtualisation flags
    for arg in &args_str {
        assert!(!arg.contains("kvm"), "KVM is forbidden: {}", arg);
        assert!(!arg.contains("whpx"), "WHPX is forbidden: {}", arg);
        assert!(!arg.contains("hvf"), "HVF is forbidden: {}", arg);
    }

    println!("Generated QEMU args: {:?}", args_str);
}

// ─── Test 2b: Default-deny network ──────────────────────────────────

#[test]
fn test_2b_default_deny_no_network() {
    let config = test_machine_config();
    let adapter = QemuAdapter::new(PathBuf::from("qemu-system-x86_64"));
    let adapter = adapter.apply_config(&config).unwrap();
    let (_, args) = adapter.into_command();

    let args_str: Vec<String> = args.iter().map(|a| a.to_string_lossy().to_string()).collect();

    assert!(
        !args_str.contains(&"-netdev".to_string()),
        "Network should be denied by default"
    );
}

// ─── Test 3: NBD server construction ────────────────────────────────

#[tokio::test]
async fn test_3_nbd_server_constructed() {
    use plaza_image::block::CowWritableLayer;
    use plaza_runtime::RuntimeStorage;

    let tmp = tempfile::tempdir().unwrap();
    let cow_path = tmp.path().join("cow_layer.bin");
    let socket_path = tmp.path().join("test-nbd.sock");

    // Create a 1 MiB COW writable layer (implements VirtualBlockDevice)
    let cow = CowWritableLayer::create(cow_path, 1024 * 1024).await.unwrap();
    let storage = RuntimeStorage::new(cow);

    let _nbd = plaza_runtime::backends::qemu::storage::nbd::NbdServer::new(socket_path.clone(), storage);

    // The server object was successfully constructed.
    // We don't call nbd.run() because that blocks forever on accept().
    // This test proves NbdServer + RuntimeStorage + VirtualBlockDevice wiring is correct.
    println!("NBD server created for socket: {:?}", socket_path);
}

// ─── Test 4: NBD protocol handshake ─────────────────────────────────

#[tokio::test]
async fn test_4_nbd_protocol_handshake() {
    use plaza_image::block::CowWritableLayer;
    use plaza_runtime::RuntimeStorage;
    use plaza_runtime::backends::qemu::storage::nbd::NbdServer;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let tmp = tempfile::tempdir().unwrap();
    let cow_path = tmp.path().join("cow_layer.bin");
    let socket_path = tmp.path().join("test-nbd.sock");

    let cow = CowWritableLayer::create(cow_path, 1024 * 1024).await.unwrap();
    let storage = RuntimeStorage::new(cow);
    let nbd = NbdServer::new(socket_path.clone(), storage);

    // Spawn server
    tokio::spawn(async move {
        let _ = nbd.run().await;
    });

    // Wait a bit for server to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Connect to server
    #[cfg(unix)]
    let mut stream = tokio::net::UnixStream::connect(&socket_path).await.unwrap();

    #[cfg(windows)]
    let mut stream = {
        let port_str = tokio::fs::read_to_string(&socket_path).await.unwrap();
        tokio::net::TcpStream::connect(format!("127.0.0.1:{}", port_str)).await.unwrap()
    };

    // 1. Initial Handshake
    let magic = stream.read_u64().await.unwrap();
    assert_eq!(magic, 0x4e42444d41474943); // NBD_MAGIC

    let opts_magic = stream.read_u64().await.unwrap();
    assert_eq!(opts_magic, 0x49484156454F5054); // NBD_OPTS_MAGIC

    let flags = stream.read_u16().await.unwrap();
    assert_eq!(flags, 3); // NBD_FLAG_FIXED_NEWSTYLE | NBD_FLAG_NO_ZEROES

    // 2. Client Flags
    stream.write_u32(0).await.unwrap();

    // 3. Option Haggling (NBD_OPT_EXPORT_NAME)
    stream.write_u64(0x49484156454F5054).await.unwrap(); // NBD_OPTS_MAGIC
    stream.write_u32(1).await.unwrap(); // NBD_OPT_EXPORT_NAME
    stream.write_u32(0).await.unwrap(); // len = 0

    // Receive export details
    let size = stream.read_u64().await.unwrap();
    assert_eq!(size, 1024 * 1024); // 1 MiB

    let export_flags = stream.read_u16().await.unwrap();
    assert_eq!(export_flags, 5); // NBD_FLAG_HAS_FLAGS | NBD_FLAG_SEND_FLUSH

    let mut zeroes = vec![0u8; 124];
    stream.read_exact(&mut zeroes).await.unwrap();
    assert_eq!(zeroes, vec![0u8; 124]);
    
    println!("NBD handshake successful");
}

// ─── Test 5: NBD protocol I/O ───────────────────────────────────────

#[tokio::test]
async fn test_5_nbd_protocol_io() {
    use plaza_image::block::CowWritableLayer;
    use plaza_runtime::RuntimeStorage;
    use plaza_runtime::backends::qemu::storage::nbd::NbdServer;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let tmp = tempfile::tempdir().unwrap();
    let cow_path = tmp.path().join("cow_layer_io.bin");
    let socket_path = tmp.path().join("test-nbd-io.sock");

    let cow = CowWritableLayer::create(cow_path, 1024 * 1024).await.unwrap();
    let storage = RuntimeStorage::new(cow);
    let nbd = NbdServer::new(socket_path.clone(), storage);

    tokio::spawn(async move {
        let _ = nbd.run().await;
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    #[cfg(unix)]
    let mut stream = tokio::net::UnixStream::connect(&socket_path).await.unwrap();

    #[cfg(windows)]
    let mut stream = {
        let port_str = tokio::fs::read_to_string(&socket_path).await.unwrap();
        tokio::net::TcpStream::connect(format!("127.0.0.1:{}", port_str)).await.unwrap()
    };

    // Fast-forward handshake
    let _ = stream.read_u64().await.unwrap(); // NBD_MAGIC
    let _ = stream.read_u64().await.unwrap(); // NBD_OPTS_MAGIC
    let _ = stream.read_u16().await.unwrap(); // flags

    stream.write_u32(0).await.unwrap(); // client flags

    stream.write_u64(0x49484156454F5054).await.unwrap(); // NBD_OPTS_MAGIC
    stream.write_u32(1).await.unwrap(); // NBD_OPT_EXPORT_NAME
    stream.write_u32(0).await.unwrap(); // len = 0

    let _ = stream.read_u64().await.unwrap(); // size
    let _ = stream.read_u16().await.unwrap(); // export flags
    let mut zeroes = vec![0u8; 124];
    stream.read_exact(&mut zeroes).await.unwrap();

    // ── Write Request ──
    stream.write_u32(0x25609513).await.unwrap(); // NBD_REQUEST_MAGIC
    stream.write_u16(0).await.unwrap(); // flags = 0
    stream.write_u16(1).await.unwrap(); // type = NBD_CMD_WRITE
    stream.write_u64(0x12345678).await.unwrap(); // handle
    stream.write_u64(512).await.unwrap(); // offset
    stream.write_u32(8).await.unwrap(); // length
    stream.write_all(b"PLAZA123").await.unwrap(); // data

    // Wait for Write Reply
    let magic = stream.read_u32().await.unwrap();
    assert_eq!(magic, 0x678affcc); // NBD_REPLY_MAGIC
    let error = stream.read_u32().await.unwrap();
    assert_eq!(error, 0); // Success
    let handle = stream.read_u64().await.unwrap();
    assert_eq!(handle, 0x12345678);

    // ── Read Request ──
    stream.write_u32(0x25609513).await.unwrap(); // NBD_REQUEST_MAGIC
    stream.write_u16(0).await.unwrap(); // flags = 0
    stream.write_u16(0).await.unwrap(); // type = NBD_CMD_READ
    stream.write_u64(0x87654321).await.unwrap(); // handle
    stream.write_u64(512).await.unwrap(); // offset
    stream.write_u32(8).await.unwrap(); // length

    // Wait for Read Reply
    let magic = stream.read_u32().await.unwrap();
    assert_eq!(magic, 0x678affcc); // NBD_REPLY_MAGIC
    let error = stream.read_u32().await.unwrap();
    assert_eq!(error, 0); // Success
    let handle = stream.read_u64().await.unwrap();
    assert_eq!(handle, 0x87654321);

    let mut data = vec![0u8; 8];
    stream.read_exact(&mut data).await.unwrap();
    assert_eq!(&data, b"PLAZA123");

    println!("NBD I/O successful");
}

// ─── Test 6: QemuPlugin execution ───────────────────────────────────

#[tokio::test]
async fn test_6_qemu_execution_plugin() {
    use plaza_image::block::CowWritableLayer;
    use plaza_runtime::{RuntimeBackend, RuntimeStorage};
    use plaza_runtime::backends::qemu::QemuPlugin;

    let plugin = QemuPlugin::new();
    if !plugin.is_available().await {
        println!("QEMU not found, skipping plugin execution test");
        return;
    }

    let config = test_machine_config();
    
    let tmp = tempfile::tempdir().unwrap();
    let cow_path = tmp.path().join("cow_layer_plugin.bin");
    let cow = CowWritableLayer::create(cow_path, 1024 * 1024).await.unwrap();
    let storage = RuntimeStorage::new(cow);

    let instance = plugin.create(&config, storage).await.unwrap();
    
    // Start instance
    plugin.start(&instance.id).await.unwrap();
    
    // Check status
    let status = plugin.status(&instance.id).await.unwrap();
    assert_eq!(status, plaza_runtime::RuntimeStatus::Running);
    
    // Stop instance
    plugin.stop(&instance.id).await.unwrap();
    
    let status2 = plugin.status(&instance.id).await.unwrap();
    assert_eq!(status2, plaza_runtime::RuntimeStatus::Stopped);
    
    println!("Plugin successfully orchestrated QEMU + NBD");
}
