import { invoke } from "@tauri-apps/api/core";

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
}

export interface CreateWorkspaceRequest {
  name: string;
  description?: string;
  runtime_kind?: string;
  image?: string;
  cpu_cores?: number;
  memory_mb?: number;
}

export interface SystemMetricsSnapshot {
  cpu_usage_pct: number;
  memory_used_mb: number;
  memory_total_mb: number;
  memory_usage_pct: number;
}

export interface HostCapabilities {
  os: { name: string; version: string; arch: string };
  cpu: { model: string; cores_logical: number };
  memory: { total_mb: number; available_mb: number };
  installed_runtimes: Array<{ id: string; name: string; version: string; health: string }>;
}

export async function apiListWorkspaces(): Promise<WorkspaceDto[]> {
  try {
    return await invoke<WorkspaceDto[]>("list_workspaces");
  } catch (e) {
    console.warn("Tauri invoke list_workspaces not available, returning mock", e);
    return [
      {
        id: "demo-ws-1",
        name: "python-dev-workspace",
        description: "Python 3.12 Development Workspace",
        state: "running",
        runtime_backend: "docker",
        health: "healthy",
        cpu_cores: 4,
        memory_mb: 4096,
        created_at: new Date().toISOString(),
      },
    ];
  }
}

export async function apiCreateWorkspace(req: CreateWorkspaceRequest): Promise<WorkspaceDto> {
  return await invoke<WorkspaceDto>("create_workspace", { request: req });
}

export async function apiStartWorkspace(id: string): Promise<void> {
  await invoke("start_workspace", { id });
}

export async function apiStopWorkspace(id: string): Promise<void> {
  await invoke("stop_workspace", { id });
}

export async function apiGetSystemMetrics(): Promise<SystemMetricsSnapshot> {
  try {
    return await invoke<SystemMetricsSnapshot>("get_system_metrics");
  } catch {
    return {
      cpu_usage_pct: 12.5,
      memory_used_mb: 8192,
      memory_total_mb: 32768,
      memory_usage_pct: 25.0,
    };
  }
}

export async function apiGetPlatformInfo(): Promise<HostCapabilities> {
  try {
    return await invoke<HostCapabilities>("get_platform_info");
  } catch {
    return {
      os: { name: "Windows", version: "11", arch: "x86_64" },
      cpu: { model: "Intel Core i9", cores_logical: 16 },
      memory: { total_mb: 32768, available_mb: 24576 },
      installed_runtimes: [
        { id: "docker", name: "Docker Engine", version: "24.0.0", health: "healthy" },
        { id: "qemu", name: "QEMU Hypervisor", version: "8.2.0", health: "healthy" },
      ],
    };
  }
}
