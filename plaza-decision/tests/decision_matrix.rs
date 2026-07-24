use plaza_config::IntentConfig;
use plaza_core::types::{Architecture, OperatingSystem};
use plaza_decision::DecisionEngine;
use plaza_events::EventBus;
use plaza_platform::PlatformDetector;
use plaza_plugin::PluginHost;
use plaza_workspace::model::{RuntimeBackendPreference, RuntimeKind, WorkspaceSpec};
use std::sync::Arc;

async fn setup_decision_engine() -> DecisionEngine {
    let platform = Arc::new(PlatformDetector::new());
    platform.scan().await.unwrap();

    let event_bus = Arc::new(EventBus::new());
    let plugin_dir = plaza_core::paths::plugin_dir();
    let plugin_host = Arc::new(PluginHost::new(event_bus, plugin_dir));

    // Register plugins
    plugin_host
        .register_runtime_plugin(Arc::new(docker_plugin::DockerPlugin::new()))
        .await
        .unwrap();
    plugin_host
        .register_runtime_plugin(Arc::new(qemu_plugin::QemuPlugin::new()))
        .await
        .unwrap();
    plugin_host
        .register_runtime_plugin(Arc::new(virtualbox_plugin::VirtualBoxPlugin::new()))
        .await
        .unwrap();

    DecisionEngine::new(platform, plugin_host)
}

#[tokio::test]
async fn matrix_scenario_linux_development_auto_docker() {
    let engine = setup_decision_engine().await;

    let mut spec = WorkspaceSpec::default();
    spec.runtime.kind = RuntimeKind::Container;
    spec.runtime.os = OperatingSystem::Linux;
    spec.runtime.arch = Architecture::X86_64;
    spec.runtime.backend = RuntimeBackendPreference::Auto;

    let decision = engine.decide(spec).await.expect("decision should succeed");
    assert_eq!(decision.selected_backend.backend_id, "docker");
}

#[tokio::test]
async fn matrix_scenario_full_vm_virtualbox() {
    let engine = setup_decision_engine().await;

    let mut spec = WorkspaceSpec::default();
    spec.runtime.kind = RuntimeKind::VirtualMachine;
    spec.runtime.os = OperatingSystem::Windows;
    spec.runtime.arch = Architecture::X86_64;
    spec.runtime.backend = RuntimeBackendPreference::Auto;

    let decision = engine.decide(spec).await.expect("decision should succeed");
    assert_eq!(decision.selected_backend.backend_id, "virtualbox");
}

#[tokio::test]
async fn matrix_scenario_pinned_backend_override() {
    let engine = setup_decision_engine().await;

    let mut spec = WorkspaceSpec::default();
    spec.runtime.kind = RuntimeKind::Container;
    spec.runtime.backend = RuntimeBackendPreference::Pinned("qemu".into());

    let decision = engine.decide(spec).await.expect("decision should succeed");
    assert_eq!(decision.selected_backend.backend_id, "qemu");
    assert!(decision.selected_backend.reason.contains("pinned"));
}

#[tokio::test]
async fn matrix_scenario_intent_ai_workload() {
    let engine = setup_decision_engine().await;

    let mut spec = WorkspaceSpec::default();
    spec.intent = Some(IntentConfig {
        purpose: Some("AI Research PyTorch".into()),
        performance: Some("high".into()),
        startup: Some("fast".into()),
        gpu: Some("preferred".into()),
        security: Some("standard".into()),
    });

    let decision = engine.decide(spec).await.expect("decision should succeed");
    assert!(decision.resource_plan.gpu_enabled);
    assert_eq!(decision.resource_plan.cpu_cores, 4);
    assert_eq!(decision.resource_plan.memory_mb, 4096);
}
