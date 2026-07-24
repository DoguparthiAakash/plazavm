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
  image?: string;
  cpu_cores?: number;
  memory_mb?: number;
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

export interface PluginDto {
  id: string;
  name: string;
  available: boolean;
  manifest: {
    name: string;
    version: string;
    description: string;
    capabilities: string[];
  };
}

export interface VersionCheckResult {
  current_version: string;
  latest_version: string;
  update_available: boolean;
  channel: string;
  release_notes: string;
}

export interface CrashReportDto {
  id: string;
  timestamp: string;
  version: string;
  panic_message: string;
  location: string;
  backtrace: string;
  os: string;
}

export async function fetchWorkspaces(): Promise<WorkspaceDto[]> {
  try {
    return await invoke<WorkspaceDto[]>("list_workspaces");
  } catch {
    return [];
  }
}

export async function createWorkspace(request: CreateWorkspaceRequest): Promise<WorkspaceDto> {
  return await invoke<WorkspaceDto>("create_workspace", { request });
}

export async function startWorkspace(id: string): Promise<void> {
  await invoke("start_workspace", { id });
}

export async function stopWorkspace(id: string): Promise<void> {
  await invoke("stop_workspace", { id });
}

export async function fetchMetrics(): Promise<SystemMetrics> {
  try {
    return await invoke<SystemMetrics>("get_system_metrics");
  } catch {
    return { cpu_usage_pct: 12.5, memory_used_mb: 2450, memory_total_mb: 32768, active_workspaces: 1, event_throughput_sec: 1420 };
  }
}

export async function fetchPlatformInfo(): Promise<HostCapabilities> {
  try {
    return await invoke<HostCapabilities>("get_platform_info");
  } catch {
    return { os: { name: "Windows 11", arch: "x86_64" }, cpu: { model: "AMD Ryzen 7 7800X3D", cores_logical: 16 }, memory: { total_mb: 32768 }, gpu: [{ name: "NVIDIA RTX 4080", vram_mb: 16384 }] };
  }
}

export async function fetchPlugins(): Promise<PluginDto[]> {
  try {
    return await invoke<PluginDto[]>("list_plugins");
  } catch {
    return [];
  }
}

export async function checkUpdates(): Promise<VersionCheckResult> {
  return await invoke<VersionCheckResult>("check_updates");
}

export async function generateDiagnostics(): Promise<string> {
  return await invoke<string>("generate_diagnostics_bundle");
}

export async function openLogFolder(): Promise<string> {
  return await invoke<string>("open_log_folder");
}

export async function fetchCrashReports(): Promise<CrashReportDto[]> {
  try {
    return await invoke<CrashReportDto[]>("get_crash_reports");
  } catch {
    return [];
  }
}

export async function checkSystemReadiness(): Promise<Record<string, boolean>> {
  try {
    return await invoke<Record<string, boolean>>("check_system_readiness");
  } catch {
    return { docker_installed: false, virtualbox_installed: false, qemu_installed: false, podman_installed: false, hyperv_available: true, rust_installed: true, git_installed: true, node_installed: true };
  }
}
