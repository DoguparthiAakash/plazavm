use plaza_events::EventBus;
use plaza_plugin::{PluginHost, PluginManifest, PluginType};
use std::sync::Arc;

#[tokio::test]
async fn test_plugin_manifest_deserialization() {
    let toml_str = r#"
id = "docker"
name = "Docker Engine Runtime"
version = "0.1.0"
description = "OCI container execution backend"
author = "PlazaVM Team"
plugin_type = "runtime"
capabilities = ["container", "gpu"]
platforms = ["linux", "windows"]
"#;

    let manifest: PluginManifest = toml::from_str(toml_str).expect("manifest should parse");
    assert_eq!(manifest.id, "docker");
    assert_eq!(manifest.plugin_type, PluginType::Runtime);
    assert_eq!(manifest.capabilities, vec!["container", "gpu"]);
}

#[tokio::test]
async fn test_plugin_host_registration_and_lookup() {
    let bus = Arc::new(EventBus::new());
    let host = PluginHost::new(bus, plaza_core::paths::plugin_dir());

    assert!(host.get_runtime_plugin("docker").await.is_none());

    let plugin = Arc::new(docker_plugin::DockerPlugin::new());
    host.register_runtime_plugin(plugin).await.unwrap();

    let fetched = host.get_runtime_plugin("docker").await;
    assert!(fetched.is_some());

    let plugin_ref = fetched.unwrap();
    assert_eq!(plugin_ref.id(), "docker");
    assert_eq!(plugin_ref.display_name(), "Docker Engine");
}

#[tokio::test]
async fn test_plugin_host_duplicate_registration_overwrites_cleanly() {
    let bus = Arc::new(EventBus::new());
    let host = PluginHost::new(bus, plaza_core::paths::plugin_dir());

    let plugin1 = Arc::new(docker_plugin::DockerPlugin::new());
    let plugin2 = Arc::new(docker_plugin::DockerPlugin::new());

    host.register_runtime_plugin(plugin1).await.unwrap();
    host.register_runtime_plugin(plugin2).await.unwrap();

    let available = host.available_runtime_plugins().await;
    assert_eq!(available.len(), 1);
}
