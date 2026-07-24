# PlazaVM v2 — Plugin Development Guide

This guide explains how to write custom execution plugins for PlazaVM v2.

---

## 🛠 Plugin Architecture Overview

Every runtime backend in PlazaVM is implemented as an independent plugin implementing the `RuntimePlugin` trait exposed by `plaza-plugin`.

### 1. Implement `RuntimePlugin` Trait

```rust
use async_trait::async_trait;
use plaza_plugin::{PluginManifest, RuntimePlugin};
use plaza_runtime::model::{BackendCapability, RuntimeKind};
use plaza_workspace::model::Workspace;

pub struct CustomPlugin;

#[async_trait]
impl RuntimePlugin for CustomPlugin {
    fn id(&self) -> &str {
        "custom-runtime"
    }

    fn display_name(&self) -> &str {
        "Custom Execution Engine"
    }

    fn manifest(&self) -> PluginManifest {
        PluginManifest {
            name: "custom-runtime".into(),
            version: semver::Version::parse("0.1.0").unwrap(),
            description: "Custom execution plugin".into(),
            capabilities: vec![
                BackendCapability::ContainerExecution,
                BackendCapability::ResourceLimits,
            ],
        }
    }

    async fn is_available(&self) -> bool {
        true
    }

    async fn start_workspace(&self, workspace: &Workspace) -> anyhow::Result<()> {
        println!("Starting workspace {} via Custom Runtime", workspace.id);
        Ok(())
    }

    async fn stop_workspace(&self, workspace: &Workspace) -> anyhow::Result<()> {
        println!("Stopping workspace {} via Custom Runtime", workspace.id);
        Ok(())
    }
}
```

---

## 🔌 Registering with PluginHost

Register your plugin inside `plaza-plugin::PluginHost`:

```rust
let bus = Arc::new(PlazaEventBus::new());
let host = PluginHost::new(bus, paths::plugin_dir());
host.register_runtime_plugin(Arc::new(CustomPlugin)).await?;
```
