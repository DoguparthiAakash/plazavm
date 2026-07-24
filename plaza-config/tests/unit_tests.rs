use plaza_config::{PlazaConfig, WorkspaceConfig};

#[test]
fn test_parse_valid_plaza_yaml() {
    let yaml = r#"
version: "1"
workspace:
  name: "ai-dev-lab"
  description: "PyTorch & CUDA Lab"
runtime:
  kind: container
  image: "cuda:12.0"
  backend: auto
resources:
  cpu:
    cores: 8
  memory:
    size: "16Gi"
  gpu:
    enabled: true
intent:
  purpose: "AI Research"
  performance: "high"
  gpu: "required"
"#;

    let config = WorkspaceConfig::parse_yaml(yaml).expect("should parse valid plaza.yaml");
    assert_eq!(config.workspace.name, "ai-dev-lab");
    assert_eq!(config.runtime.kind, "container");
    assert_eq!(config.resources.cpu.as_ref().unwrap().cores, 8);
    assert!(config.intent.is_some());
    let intent = config.intent.as_ref().unwrap();
    assert_eq!(intent.purpose.as_deref(), Some("AI Research"));
    config.validate().expect("config should be valid");
}

#[test]
fn test_parse_invalid_workspace_config() {
    let yaml = r#"
version: "1"
workspace:
  name: ""
runtime:
  kind: container
"#;
    let config = WorkspaceConfig::parse_yaml(yaml).expect("syntax is valid yaml");
    assert!(config.validate().is_err());
}

#[test]
fn test_parse_plaza_toml() {
    let toml_str = r#"
[server]
bind_address = "0.0.0.0"
port = 9090

[defaults]
preferred_backend = "docker"
auto_suspend_idle_mins = 30
"#;
    let app_config = PlazaConfig::parse_toml(toml_str).expect("should parse plaza.toml");
    assert_eq!(app_config.server.bind_address, "0.0.0.0");
    assert_eq!(app_config.server.port, 9090);
    assert_eq!(
        app_config.defaults.preferred_backend.as_deref(),
        Some("docker")
    );
}
