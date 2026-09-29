import { invoke as tauriInvoke } from "@tauri-apps/api/core";

// Unified IPC Dispatcher supporting Electron, Tauri, and Standalone Browser Fallbacks
async function safeInvoke<T>(cmd: string, args?: any): Promise<T> {
  const win = window as any;
  if (win.electronAPI && typeof win.electronAPI.invoke === "function") {
    try {
      return await win.electronAPI.invoke(cmd, args);
    } catch (err) {
      console.warn(`[Electron IPC] Command '${cmd}' failed or unhandled:`, err);
      throw err;
    }
  }
  try {
    return await tauriInvoke<T>(cmd, args);
  } catch (err) {
    console.warn(`[Tauri IPC] Command '${cmd}' failed or not in Tauri environment:`, err);
    throw err;
  }
}

export interface WorkspaceDto {
  id: string;
  name: string;
  description?: string;
  state: string;
  runtime_backend?: string;
  health: string;
  cpu_cores: number;
  memory_mb: number;
  created_at: string;
  project_path?: string;
}

export interface CreateWorkspaceRequest {
  name: string;
  image?: string;
  cpu_cores?: number;
  memory_mb?: number;
  runtime?: string;
  path?: string;
}

export interface SystemMetrics {
  cpu_usage_pct: number;
  memory_used_mb: number;
  memory_total_mb: number;
  active_workspaces: number;
  event_throughput_sec: number;
}

export interface HostCapabilities {
  os: { name: string; arch: string };
  cpu: { model: string; cores_logical: number };
  memory: { total_mb: number };
  gpu: Array<{ name: string; vram_mb: number }>;
}

export interface PlazaEvent {
  id: number;
  type: string;
  data: any;
  timestamp: string;
}

// ─── Workspace Operations ───
export async function fetchWorkspaces(): Promise<WorkspaceDto[]> {
  return await safeInvoke<WorkspaceDto[]>("list_workspaces");
}

export async function createWorkspace(request: CreateWorkspaceRequest): Promise<WorkspaceDto> {
  return await safeInvoke<WorkspaceDto>("create_workspace", { request });
}

export async function startWorkspace(id: string): Promise<void> {
  return await safeInvoke("start_workspace", id);
}

export async function stopWorkspace(id: string): Promise<void> {
  return await safeInvoke("stop_workspace", id);
}

export async function deleteWorkspace(id: string): Promise<void> {
  return await safeInvoke("delete_workspace", id);
}

export async function checkWorkspaceRunning(id: string): Promise<{ running: boolean; pid?: number }> {
  return await safeInvoke("check_workspace_running", id);
}

export async function execInWorkspace(workspaceId: string, command: string): Promise<{ output: string; error: string }> {
  return await safeInvoke("exec_in_workspace", { workspaceId, command });
}

export async function buildWorkspaceImage(workspaceId: string, sourcePath: string, buildCommand?: string): Promise<{ output: string }> {
  return await safeInvoke("build_workspace_image", { workspaceId, sourcePath, buildCommand });
}

// ─── Platform & System ───
export async function fetchPlatformInfo(): Promise<HostCapabilities> {
  return await safeInvoke<HostCapabilities>("get_platform_info");
}

export async function openLogFolder(): Promise<string> {
  return await safeInvoke<string>("open_log_folder");
}

export async function checkSystemReadiness(): Promise<Record<string, boolean>> {
  return await safeInvoke<Record<string, boolean>>("check_system_readiness");
}

export async function getSystemMetrics(): Promise<SystemMetrics> {
  return await safeInvoke<SystemMetrics>("get_system_metrics");
}

// ─── Configuration ───
export async function getWorkspaceConfig(id: string): Promise<any> {
  return await safeInvoke<any>("get_workspace_config", id);
}

export async function saveWorkspaceConfig(id: string, configJson: any): Promise<void> {
  return await safeInvoke<void>("save_workspace_config", { id, configJson });
}

// ─── Network ───
export async function getNetworkStatus(workspaceId: string): Promise<{ raw: string }> {
  return await safeInvoke<{ raw: string }>("network_status", workspaceId);
}

export async function getNetworkBridges(): Promise<{ raw: string }> {
  return await safeInvoke<{ raw: string }>("network_bridge_list");
}

// ─── Doctor ───
export async function runDoctor(): Promise<{ raw: string }> {
  return await safeInvoke<{ raw: string }>("doctor");
}

// ─── Live Event Streaming ───
type EventCallback = (event: PlazaEvent) => void;
let eventListeners: EventCallback[] = [];

export function onPlazaEvent(callback: EventCallback): () => void {
  const win = window as any;
  if (win.electronAPI && typeof win.electronAPI.onEvent === "function") {
    win.electronAPI.onEvent(callback);
    eventListeners.push(callback);
    return () => {
      eventListeners = eventListeners.filter(l => l !== callback);
    };
  }
  // Fallback: poll for events
  let lastSeq = 0;
  const interval = setInterval(async () => {
    try {
      const events: PlazaEvent[] = await safeInvoke("get_events", lastSeq);
      events.forEach(e => {
        lastSeq = e.id;
        callback(e);
      });
    } catch {}
  }, 2000);
  return () => clearInterval(interval);
}

export function removePlazaEventListeners(): void {
  const win = window as any;
  if (win.electronAPI && typeof win.electronAPI.removeEventListeners === "function") {
    win.electronAPI.removeEventListeners();
  }
  eventListeners = [];
}
