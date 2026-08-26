use plaza_foundation::config::machine_section::{CpuSection, MachineSection, MemorySection};
use plaza_foundation::core::CapabilityPolicy;
use plaza_image::block::{
    CowWritableLayer, FileBackedImmutableLayer, ImmutableLayer, VirtualBlockDevice,
};
use plaza_image::composer::LayeredBlockDevice;
use plaza_runtime::backends::qemu::QemuPlugin;
use plaza_runtime::{MachineConfig, RuntimeBackend, RuntimeStorage};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;

fn get_test_image() -> PathBuf {
    match std::env::var("PLAZA_TEST_IMAGE") {
        Ok(path) => PathBuf::from(path),
        Err(_) => panic!("PLAZA_TEST_IMAGE environment variable is required for this integration test. Please provide a path to an Alpine minimal image."),
    }
}

async fn get_file_modified_time(path: &PathBuf) -> std::time::SystemTime {
    let meta = tokio::fs::metadata(path).await.unwrap();
    meta.modified().unwrap()
}

fn test_machine_config(workspace_id: &str) -> MachineConfig {
    MachineConfig {
        workspace_id: workspace_id.into(),
        instance_id: format!("{}-instance", workspace_id),
        machine: MachineSection {
            cpu: CpuSection { cores: 1 },
            memory: MemorySection {
                size: "256MiB".into(),
            },
            ..MachineSection::default()
        },
        capabilities: CapabilityPolicy::default(),
        boot_device: PathBuf::from("nbd"), // Ignored by qemu_plugin, it sets it up via the NBD socket.
        ..Default::default()
    }
}

async fn setup_storage(
    tmp_dir: &std::path::Path,
    base_image: &PathBuf,
    cow_name: &str,
) -> RuntimeStorage {
    let base_layer = FileBackedImmutableLayer::open(base_image.clone())
        .await
        .unwrap();
    let size = base_layer.size();

    let cow_path = tmp_dir.join(cow_name);
    let cow_layer = CowWritableLayer::create(cow_path, size).await.unwrap();

    let layered = LayeredBlockDevice::new(vec![Arc::new(base_layer)], cow_layer);
    RuntimeStorage::new(layered)
}

#[tokio::test]
async fn test_qemu_guest_boot_and_persistence() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
    let base_image = get_test_image();
    let base_hash_before = get_file_modified_time(&base_image).await;

    let tmp = tempfile::tempdir().unwrap();
    let storage = setup_storage(tmp.path(), &base_image, "cow.bin").await;

    let plugin = QemuPlugin::new();
    if !plugin.is_available().await {
        panic!("QEMU is not available");
    }

    let config = test_machine_config("test-boot");
    let instance = plugin.create(&config, storage.clone()).await.unwrap();

    // 1. Start QEMU and wait for guest to be ready
    plugin.start(&instance.id).await.unwrap();

    // Wait up to 60 seconds for PLAZA_GUEST_READY or login prompt
    plugin
        .wait_for_ready(&instance.id, Duration::from_secs(300))
        .await
        .unwrap();

    // Login as root
    plugin.send_command(&instance.id, "root").await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Send an empty password if prompted
    plugin.send_command(&instance.id, "").await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;

    // 2. Perform Disk Write
    plugin
        .send_command(&instance.id, "echo 'modified_plaza_state' > /state.txt")
        .await
        .unwrap();
    plugin.send_command(&instance.id, "sync").await.unwrap();

    // Sleep briefly to allow guest flush
    tokio::time::sleep(Duration::from_secs(2)).await;

    // 3. Stop guest
    plugin.stop(&instance.id).await.unwrap();

    // 4. Verify base image remains unchanged
    let base_hash_after = get_file_modified_time(&base_image).await;
    assert_eq!(
        base_hash_before, base_hash_after,
        "Base image MUST remain unchanged"
    );

    // 5. Restart with same COW storage
    let instance2 = plugin.create(&config, storage.clone()).await.unwrap();
    plugin.start(&instance2.id).await.unwrap();

    plugin
        .wait_for_ready(&instance2.id, Duration::from_secs(300))
        .await
        .unwrap();

    plugin.send_command(&instance2.id, "root").await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;
    plugin.send_command(&instance2.id, "").await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;

    // 6. Verify persistence
    // We will execute a cat command.
    // Make sure we wait long enough. We use a unique string so we don't accidentally match login.
    plugin
        .send_command(
            &instance2.id,
            "cat /state.txt | grep 'modified_plaza_state' && echo 'SUCCESS_PLAZA_GUEST_READY'",
        )
        .await
        .unwrap();

    // Wait for the specific output to prove it exists.
    plugin
        .wait_for_ready(&instance2.id, Duration::from_secs(10))
        .await
        .unwrap();

    plugin.stop(&instance2.id).await.unwrap();
}

#[tokio::test]
async fn test_workspace_isolation() {
    let base_image = get_test_image();
    let tmp = tempfile::tempdir().unwrap();

    let storage_a = setup_storage(tmp.path(), &base_image, "cow_a.bin").await;
    let storage_b = setup_storage(tmp.path(), &base_image, "cow_b.bin").await;

    let plugin = QemuPlugin::new();
    if !plugin.is_available().await {
        panic!("QEMU is not available");
    }

    let config_a = test_machine_config("workspace-a");
    let instance_a = plugin.create(&config_a, storage_a).await.unwrap();

    let config_b = test_machine_config("workspace-b");
    let instance_b = plugin.create(&config_b, storage_b).await.unwrap();

    // Start both workspaces
    plugin.start(&instance_a.id).await.unwrap();
    plugin.start(&instance_b.id).await.unwrap();

    plugin
        .wait_for_ready(&instance_a.id, Duration::from_secs(300))
        .await
        .unwrap();
    plugin
        .wait_for_ready(&instance_b.id, Duration::from_secs(300))
        .await
        .unwrap();

    plugin.send_command(&instance_a.id, "root").await.unwrap();
    plugin.send_command(&instance_b.id, "root").await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;
    plugin.send_command(&instance_a.id, "").await.unwrap();
    plugin.send_command(&instance_b.id, "").await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Write different states
    plugin
        .send_command(&instance_a.id, "echo 'STATE_A' > /state.txt")
        .await
        .unwrap();
    plugin
        .send_command(&instance_b.id, "echo 'STATE_B' > /state.txt")
        .await
        .unwrap();
    plugin.send_command(&instance_a.id, "sync").await.unwrap();
    plugin.send_command(&instance_b.id, "sync").await.unwrap();

    tokio::time::sleep(Duration::from_secs(2)).await;

    // Verify isolation - Workspace A should have STATE_A
    plugin
        .send_command(
            &instance_a.id,
            "cat /state.txt | grep 'STATE_A' && echo 'SUCCESS_PLAZA_GUEST_READY'",
        )
        .await
        .unwrap();
    plugin
        .wait_for_ready(&instance_a.id, Duration::from_secs(10))
        .await
        .unwrap();

    // Verify isolation - Workspace B should have STATE_B
    plugin
        .send_command(
            &instance_b.id,
            "cat /state.txt | grep 'STATE_B' && echo 'SUCCESS_PLAZA_GUEST_READY'",
        )
        .await
        .unwrap();
    plugin
        .wait_for_ready(&instance_b.id, Duration::from_secs(10))
        .await
        .unwrap();

    plugin.stop(&instance_a.id).await.unwrap();
    plugin.stop(&instance_b.id).await.unwrap();
}

#[tokio::test]
async fn test_crash_recovery() {
    let base_image = get_test_image();
    let tmp = tempfile::tempdir().unwrap();
    let storage = setup_storage(tmp.path(), &base_image, "cow_crash.bin").await;

    let plugin = QemuPlugin::new();
    let config = test_machine_config("test-crash");
    let instance = plugin.create(&config, storage.clone()).await.unwrap();

    plugin.start(&instance.id).await.unwrap();
    plugin
        .wait_for_ready(&instance.id, Duration::from_secs(300))
        .await
        .unwrap();

    plugin.send_command(&instance.id, "root").await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;
    plugin.send_command(&instance.id, "").await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Force terminate
    plugin.force_stop(&instance.id).await.unwrap();

    // Status should be stopped
    let status = plugin.status(&instance.id).await.unwrap();
    assert_eq!(status, plaza_runtime::RuntimeStatus::Stopped);

    // Recovery restart
    plugin.start(&instance.id).await.unwrap();
    plugin
        .wait_for_ready(&instance.id, Duration::from_secs(300))
        .await
        .unwrap();
    plugin.stop(&instance.id).await.unwrap();
}
