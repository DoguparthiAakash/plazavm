//! Root-level integration test validating the PlazaVM platform end-to-end.

#[tokio::test]
async fn test_root_platform_bootstrap_and_scan() {
    let detector = plaza_foundation::platform::PlatformDetector::new();
    let caps = detector.scan().await.expect("Platform scan failed");

    assert!(!caps.os.name.is_empty());
    assert!(caps.cpu.cores_logical > 0);
    assert!(caps.memory.total_mb > 0);
}

