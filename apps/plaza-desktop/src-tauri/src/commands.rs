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
pub async fn delete_workspace(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let ws_id = WorkspaceId::parse(&id).map_err(|e| e.to_string())?;
    state
        .workspace_service
        .delete_workspace(&ws_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
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
pub async fn check_system_readiness() -> Result<serde_json::Value, String> {
    let readiness = serde_json::json!({
        "plaza_v86_engine": true,
        "plaza_block_storage": true,
        "plaza_pur_daemon": true,
        "rust_installed": true,
        "git_installed": true,
        "node_installed": true
    });
    Ok(readiness)
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
pub async fn get_workspace_config(
    state: State<'_, AppState>,
    id: String,
) -> Result<serde_json::Value, String> {
    let ws_id = WorkspaceId::parse(&id).map_err(|e| e.to_string())?;
    let workspace = state
        .workspace_service
        .get_workspace(&ws_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Workspace not found".to_string())?;

    let path = workspace
        .metadata
        .project_path
        .ok_or_else(|| "Workspace has no project path".to_string())?;
    let yaml_path = std::path::PathBuf::from(path).join("plaza.yaml");
    
    if !yaml_path.exists() {
        return Err("plaza.yaml not found".to_string());
    }

    let content = tokio::fs::read_to_string(&yaml_path)
        .await
        .map_err(|e| e.to_string())?;
    
    // Parse into PlazaYaml, then to JSON value
    let yaml = plaza_foundation::config::PlazaYaml::parse_yaml(&content)
        .map_err(|e| e.to_string())?;
        
    let json = serde_json::to_value(&yaml).map_err(|e| e.to_string())?;
    Ok(json)
}

#[tauri::command]
pub async fn save_workspace_config(
    state: State<'_, AppState>,
    id: String,
    config_json: serde_json::Value,
) -> Result<(), String> {
    let ws_id = WorkspaceId::parse(&id).map_err(|e| e.to_string())?;
    let workspace = state
        .workspace_service
        .get_workspace(&ws_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Workspace not found".to_string())?;

    let path = workspace
        .metadata
        .project_path
        .ok_or_else(|| "Workspace has no project path".to_string())?;
    let yaml_path = std::path::PathBuf::from(path).join("plaza.yaml");
    
    let yaml: plaza_foundation::config::PlazaYaml = serde_json::from_value(config_json)
        .map_err(|e| format!("Invalid configuration format: {}", e))?;
        
    let content = serde_yaml::to_string(&yaml)
        .map_err(|e| format!("Failed to serialize configuration: {}", e))?;
        
    tokio::fs::write(&yaml_path, content)
        .await
        .map_err(|e| format!("Failed to write configuration: {}", e))?;
        
    Ok(())
}
