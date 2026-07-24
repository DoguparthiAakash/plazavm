use plaza_core::id::{PluginId, RuntimeId, WorkspaceId};
use plaza_core::paths;
use plaza_core::security::{InMemorySecretStore, IsolationLevel, SecretStore, SecurityPolicy};
use plaza_core::types::{Architecture, ByteSize, OperatingSystem, Timestamp};
use plaza_core::PlazaError;

#[test]
fn test_workspace_id_uniqueness_and_formatting() {
    let id1 = WorkspaceId::new();
    let id2 = WorkspaceId::new();
    assert_ne!(id1, id2);

    let s = id1.to_string();
    let parsed = WorkspaceId::parse(&s).expect("should parse valid UUID string");
    assert_eq!(id1, parsed);
}

#[test]
fn test_plugin_and_runtime_ids() {
    let pid = PluginId::new("docker");
    assert_eq!(pid.to_string(), "docker");

    let rid = RuntimeId::new("inst-123");
    assert_eq!(rid.to_string(), "inst-123");
}

#[test]
fn test_bytesize_parsing() {
    assert_eq!(ByteSize::parse("4Gi").unwrap().as_gb(), 4);
    assert_eq!(ByteSize::parse("512Mi").unwrap().as_mb(), 512);
    assert_eq!(ByteSize::parse("2G").unwrap().as_gb(), 2);
    assert_eq!(ByteSize::parse("1024M").unwrap().as_mb(), 1024);
    assert!(ByteSize::parse("invalid").is_err());
}

#[test]
fn test_timestamp_rfc3339() {
    let ts = Timestamp::now();
    let rfc = ts.to_rfc3339();
    let parsed = Timestamp::parse(&rfc).expect("should parse RFC 3339 string");
    assert_eq!(ts.0.timestamp(), parsed.0.timestamp());
}

#[test]
fn test_architecture_and_os_display() {
    assert_eq!(Architecture::X86_64.to_string(), "x86_64");
    assert_eq!(Architecture::Aarch64.to_string(), "aarch64");
    assert_eq!(OperatingSystem::Linux.to_string(), "linux");
    assert_eq!(OperatingSystem::Windows.to_string(), "windows");
}

#[test]
fn test_security_policy_defaults() {
    let policy = SecurityPolicy::default();
    assert_eq!(policy.isolation, IsolationLevel::Standard);
    assert!(!policy.read_only_root);
    assert!(!policy.allow_privileged);
}

#[test]
fn test_in_memory_secret_store() {
    let store = InMemorySecretStore::default();
    assert!(store.get("key1").unwrap().is_none());

    store.set("key1", "secret_val").unwrap();
    assert_eq!(store.get("key1").unwrap().unwrap(), "secret_val");

    let keys = store.list_keys().unwrap();
    assert_eq!(keys, vec!["key1"]);

    store.delete("key1").unwrap();
    assert!(store.get("key1").unwrap().is_none());
}

#[test]
fn test_paths_helpers() {
    let data = paths::data_dir();
    assert!(paths::config_dir().starts_with(&data));
    assert!(paths::plugin_dir().starts_with(&data));
    assert!(paths::db_dir().starts_with(&data));
    assert!(paths::workspaces_dir().starts_with(&data));
}

#[test]
fn test_error_constructors() {
    let err = PlazaError::config("test config error");
    assert!(matches!(err, PlazaError::Config(_)));
    assert_eq!(err.to_string(), "configuration error: test config error");

    let err2 = PlazaError::storage("test storage error");
    assert!(matches!(err2, PlazaError::Storage(_)));
}
