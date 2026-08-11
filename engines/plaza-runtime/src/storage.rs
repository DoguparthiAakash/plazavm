use std::sync::Arc;
use tokio::sync::Mutex;
use plaza_image::VirtualBlockDevice;

/// A runtime-facing handle/adapter over the underlying image/block storage.
///
/// Wraps a `VirtualBlockDevice` (typically the COW writable layer) with
/// safe ownership semantics for runtimes (e.g. `Arc<Mutex<>>`) so that
/// multiple async tasks (like an NBD server) can access it safely.
#[derive(Clone)]
pub struct RuntimeStorage {
    /// The underlying synchronized block device.
    pub device: Arc<Mutex<dyn VirtualBlockDevice>>,
}

impl RuntimeStorage {
    /// Wrap an existing VirtualBlockDevice into a RuntimeStorage.
    pub fn new<T: VirtualBlockDevice + 'static>(device: T) -> Self {
        Self {
            device: Arc::new(Mutex::new(device)),
        }
    }
}
