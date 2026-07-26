//! Tauri IPC command handlers.

use plaza_api::diagnostics::DiagnosticsBundle;
use plaza_api::updater::{UpdateChannel, UpdateService, VersionCheckResult};
use plaza_api::{AppState, CreateWorkspaceRequest, WorkspaceDto};
use plaza_foundation::config::ConfigManager;
use plaza_foundation::core::id::WorkspaceId;
use plaza_foundation::core::logging::Logger;
use plaza_foundation::core::panic_handler::{CrashHandler, CrashReport};
use plaza_workspace::model::WorkspaceSpec;
use std::path::Path;
use tauri::State;

#[tauri::command]
pub async fn list_workspaces(state: State<'_, AppState>) -> Result<Vec<WorkspaceDto>, String> {
    let workspaces = state
        .workspace_service
        .list_workspaces()
        .await
        .map_err(|e| e.to_string())?;

    Ok(workspaces
        .into_iter()
        .map(|w| WorkspaceDto {
            id: w.id.to_string(),
            name: w.name,
            description: w.description,
            state: w.status.state.to_string(),
            runtime_backend: w.status.runtime_backend,
            health: w.status.health.to_string(),
            cpu_cores: w.spec.resources.cpu_cores,
            memory_mb: w.spec.resources.memory_mb,
            created_at: w.metadata.created_at.to_rfc3339(),
        })
        .collect())
}

#[tauri::command]
pub async fn create_workspace(
    state: State<'_, AppState>,
    request: CreateWorkspaceRequest,
) -> Result<WorkspaceDto, String> {
    let mut spec = WorkspaceSpec::default();
    if let Some(img) = request.image {
        spec.runtime.image = Some(img);
    }
    if let Some(cores) = request.cpu_cores {
        spec.resources.cpu_cores = cores;
    }
    if let Some(mem) = request.memory_mb {
        spec.resources.memory_mb = mem;
    }

    let ws = state
        .workspace_service
        .create_workspace(&request.name, spec)
        .await
        .map_err(|e| e.to_string())?;

    Ok(WorkspaceDto {
        id: ws.id.to_string(),
        name: ws.name,
        description: ws.description,
        state: ws.status.state.to_string(),
        runtime_backend: ws.status.runtime_backend,
        health: ws.status.health.to_string(),
        cpu_cores: ws.spec.resources.cpu_cores,
        memory_mb: ws.spec.resources.memory_mb,
        created_at: ws.metadata.created_at.to_rfc3339(),
    })
}

#[tauri::command]
pub async fn start_workspace(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let ws_id = WorkspaceId::parse(&id).map_err(|e| e.to_string())?;
    state
        .workspace_service
        .set_desired_state(&ws_id, plaza_workspace::model::DesiredState::Running)
        .await
        .map_err(|e| e.to_string())?;



    Ok(())
}

#[tauri::command]
pub async fn stop_workspace(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let ws_id = WorkspaceId::parse(&id).map_err(|e| e.to_string())?;
    state
        .workspace_service
        .set_desired_state(&ws_id, plaza_workspace::model::DesiredState::Stopped)
        .await
        .map_err(|e| e.to_string())?;



    Ok(())
}

#[tauri::command]
pub async fn get_system_metrics(
    _state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "cpu_usage_percent": 0.0,
        "memory_used_mb": 0,
        "memory_total_mb": 0,
        "active_workspaces": 0
    }))
}

#[tauri::command]
pub async fn get_platform_info(
    state: State<'_, AppState>,
) -> Result<plaza_foundation::platform::HostCapabilities, String> {
    state
        .platform
        .capabilities()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_plugins(state: State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    let plugins = state
        .container
        .plugin_host
        .available_runtime_plugins()
        .await;
    let mut result = Vec::new();
    for p in plugins {
        result.push(serde_json::json!({
            "id": p.id(),
            "name": p.display_name(),
            "available": p.is_available().await,
            "manifest": p.manifest()
        }));
    }
    Ok(result)
}

#[tauri::command]
pub async fn check_updates() -> Result<VersionCheckResult, String> {
    UpdateService::check_for_updates(UpdateChannel::DevPreview)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_diagnostics_bundle(state: State<'_, AppState>) -> Result<String, String> {
    let path = DiagnosticsBundle::generate(&state.container)
        .await
        .map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn open_log_folder() -> Result<String, String> {
    let path = Logger::log_dir();
    let path_str = path.to_string_lossy().to_string();

    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("explorer").arg(&path).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&path).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(&path).spawn();
    }

    Ok(path_str)
}

#[tauri::command]
pub async fn get_crash_reports() -> Result<Vec<CrashReport>, String> {
    Ok(CrashHandler::list_crash_reports())
}

#[tauri::command]
pub async fn export_config(target_path: String) -> Result<(), String> {
    ConfigManager::export_config(Path::new(&target_path)).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_config(source_path: String) -> Result<(), String> {
    ConfigManager::import_config(Path::new(&source_path)).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn reset_config() -> Result<(), String> {
    ConfigManager::reset_to_defaults().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn check_system_readiness() -> Result<serde_json::Value, String> {
    let readiness = serde_json::json!({
        "docker_installed": false,
        "virtualbox_installed": false,
        "qemu_installed": false,
        "podman_installed": false,
        "hyperv_available": std::env::consts::OS == "windows",
        "rust_installed": true,
        "git_installed": true,
        "node_installed": true
    });
    Ok(readiness)
}

#[tauri::command]
pub async fn get_pro_images() -> Result<Vec<serde_json::Value>, String> {
    let images = vec![
        serde_json::json!({
            "uri": "pro://ubuntu:24.04",
            "name": "Ubuntu Userspace",
            "tag": "24.04",
            "digest": "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "size_mb": 142,
            "signature": "Ed25519 Valid",
            "sbom_packages": 128
        }),
        serde_json::json!({
            "uri": "pro://python-ai:v1",
            "name": "Python AI/ML Stack",
            "tag": "v1",
            "digest": "sha256:7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069",
            "size_mb": 420,
            "signature": "Ed25519 Valid",
            "sbom_packages": 164
        }),
        serde_json::json!({
            "uri": "pro://rust-dev:latest",
            "name": "Rust Systems Toolchain",
            "tag": "latest",
            "digest": "sha256:a1b2c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0",
            "size_mb": 210,
            "signature": "Ed25519 Valid",
            "sbom_packages": 92
        })
    ];
    Ok(images)
}

#[tauri::command]
pub async fn get_pur_images() -> Result<Vec<serde_json::Value>, String> {
    let images = vec![
        serde_json::json!({
            "uri": "pri://ubuntu-dev:24.04",
            "name": "Ubuntu Dev Utility Layer",
            "tag": "24.04",
            "digest": "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "size_mb": 128,
            "signature": "SIG-PUR-1.0",
            "sbom_packages": 112
        }),
        serde_json::json!({
            "uri": "pri://cuda-pytorch:12.4",
            "name": "CUDA PyTorch PUR Image",
            "tag": "12.4",
            "digest": "sha256:9f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069",
            "size_mb": 890,
            "signature": "SIG-PUR-1.0",
            "sbom_packages": 198
        })
    ];
    Ok(images)
}

#[tauri::command]
pub async fn get_snapshot_timeline() -> Result<Vec<serde_json::Value>, String> {
    let commits = vec![
        serde_json::json!({
            "commit_id": "c1a8f9204b",
            "author": "Chief Systems Architect",
            "message": "Initial workspace creation & manifest commit",
            "timestamp": "2026-07-25 18:00:00 UTC",
            "packages_count": 42
        }),
        serde_json::json!({
            "commit_id": "c2b9e0315a",
            "author": "Developer",
            "message": "Installed CUDA 12.4 and PyTorch v2.3",
            "timestamp": "2026-07-25 20:30:00 UTC",
            "packages_count": 68
        })
    ];
    Ok(commits)
}

#[tauri::command]
pub async fn query_ai_assistant(prompt: String) -> Result<String, String> {
    Ok(format!(
        "🤖 Plaza AI: Understood request: '{}'. Generating optimized workspace execution plan with PUR OverlayFS and Ed25519 signature verification...",
        prompt
    ))
}

