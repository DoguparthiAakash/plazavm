use plaza_platform::{HostCapabilities, PlatformDetector, PlatformProfile};

#[tokio::test]
async fn test_platform_detector_scan() {
    let detector = PlatformDetector::new();
    let caps = detector.scan().await.expect("scan should succeed");

    assert!(!caps.os.name.is_empty());
    assert!(caps.cpu.cores_logical >= 1);
    assert!(caps.memory.total_mb > 0);

    let profile = detector.profile().await;
    assert_ne!(profile.to_string(), "");
}

#[test]
fn test_platform_profile_classification_scenarios() {
    let mut caps = HostCapabilities {
        os: plaza_platform::HostOs {
            name: "Linux".into(),
            version: "6.5".into(),
            arch: plaza_core::types::Architecture::X86_64,
            kernel: "Linux".into(),
            is_headless: false,
        },
        cpu: plaza_platform::CpuCapabilities {
            arch: plaza_core::types::Architecture::X86_64,
            model: "Intel i9".into(),
            vendor: plaza_platform::CpuVendor::Intel,
            cores_physical: 8,
            cores_logical: 16,
            frequency_mhz: 3600,
            features: vec![],
        },
        gpu: vec![plaza_platform::GpuCapabilities {
            name: "NVIDIA RTX 4090".into(),
            vendor: plaza_platform::GpuVendor::Nvidia,
            vram_mb: 24576,
            compute: plaza_platform::GpuCompute::Cuda {
                version: "12.0".into(),
                compute_capability: "8.9".into(),
            },
            driver_version: Some("535.0".into()),
            passthrough_capable: true,
        }],
        memory: plaza_platform::MemoryInfo {
            total_mb: 65536,
            available_mb: 48000,
            swap_total_mb: 8192,
            swap_available_mb: 8192,
        },
        storage: vec![],
        virtualization: plaza_platform::VirtualizationSupport {
            hardware_virt: true,
            nested_virt: true,
            iommu: true,
            hypervisor_present: false,
            platform_hypervisor: None,
        },
        installed_runtimes: vec![],
    };

    // Scenario 1: AI Workstation
    assert_eq!(
        PlatformProfile::detect(&caps),
        PlatformProfile::AiWorkstation
    );

    // Scenario 2: Server (headless)
    caps.os.is_headless = true;
    assert_eq!(PlatformProfile::detect(&caps), PlatformProfile::Server);
}
