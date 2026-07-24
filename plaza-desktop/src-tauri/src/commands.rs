//! Tauri IPC command handlers.

use plaza_api::diagnostics::DiagnosticsBundle;
use plaza_api::updater::{UpdateChannel, UpdateService, VersionCheckResult};
use plaza_api::{AppState, CreateWorkspaceRequest, WorkspaceDto};
use plaza_config::ConfigManager;
use plaza_core::id::WorkspaceId;
use plaza_core::logging::Logger;
use plaza_core::panic_handler::{CrashHandler, CrashReport};
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

    if let Some(ws) = state
        .workspace_service
        .get_workspace(&ws_id)
        .await
        .map_err(|e| e.to_string())?
    {
        state
            .controller
            .reconcile_workspace(&ws)
            .await
            .map_err(|e| e.to_string())?;
    }

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

    if let Some(ws) = state
        .workspace_service
        .get_workspace(&ws_id)
        .await
        .map_err(|e| e.to_string())?
    {
        state
            .controller
            .reconcile_workspace(&ws)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_system_metrics(
    state: State<'_, AppState>,
) -> Result<plaza_monitor::SystemMetricsSnapshot, String> {
    Ok(state.monitor.sample())
}

#[tauri::command]
pub async fn get_platform_info(
    state: State<'_, AppState>,
) -> Result<plaza_platform::HostCapabilities, String> {
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
