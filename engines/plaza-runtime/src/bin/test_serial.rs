//! Integration test: spawns V86Plugin → plaza-guest agent → exec over serial.

use plaza_runtime::{RuntimeBackend, MachineConfig};
use plaza_runtime::backends::v86::V86Plugin;
use plaza_runtime::storage::RuntimeStorage;
use plaza_image::block::VirtualBlockDevice;
use plaza_foundation::core::PlazaResult;
use std::sync::Arc;

/// A trivial in-memory block device for testing.
struct NullBlockDevice;

#[async_trait::async_trait]
impl VirtualBlockDevice for NullBlockDevice {
    async fn read_at(&self, _offset: u64, buffer: &mut [u8]) -> PlazaResult<usize> {
        buffer.fill(0);
        Ok(buffer.len())
    }
    async fn write_at(&mut self, _offset: u64, buffer: &[u8]) -> PlazaResult<usize> {
        Ok(buffer.len())
    }
    async fn flush(&mut self) -> PlazaResult<()> { Ok(()) }
    fn size(&self) -> u64 { 64 * 1024 * 1024 } // 64 MiB dummy
}

#[tokio::main]
async fn main() {
    println!("=== PlazaVM Serial Communication Test ===\n");

    let plugin = Arc::new(V86Plugin::default());
    let mut machine = MachineConfig::default();
    machine.machine.architecture = plaza_foundation::core::types::Architecture::X86_32;

    let storage = RuntimeStorage::new(NullBlockDevice);

    // 1. Create
    let instance = plugin.create(&machine, storage).await.unwrap();
    println!("[1/4] Created instance: {}", instance.id);

    // 2. Start (spawns plaza-guest agent process)
    plugin.start(&instance.id).await.unwrap();
    println!("[2/4] Started instance — plaza-guest agent launching...");

    // Give the agent a moment to print SUCCESS_PLAZA_GUEST_READY
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // 3. Exec a command
    println!("[3/4] Executing: echo hello from plazavm");
    match plugin.exec(&instance.id, "echo hello from plazavm").await {
        Ok(_) => println!("      ✓ Command executed successfully"),
        Err(e) => eprintln!("      ✗ Execution failed: {}", e),
    }

    // 4. Stop
    plugin.stop(&instance.id).await.unwrap();
    println!("[4/4] Stopped instance");

    println!("\n=== Test Complete ===");
}
