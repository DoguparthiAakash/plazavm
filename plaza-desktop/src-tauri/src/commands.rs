//! Tauri IPC command handlers.

use plaza_api::{AppState, CreateWorkspaceRequest, WorkspaceDto};
use plaza_core::id::WorkspaceId;
use plaza_workspace::model::WorkspaceSpec;
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
