import React, { useEffect, useState } from "react";
import { Sidebar } from "./components/Sidebar";
import { WorkspaceList } from "./components/WorkspaceList";
import { WorkspaceCreator } from "./components/WorkspaceCreator";
import { PlatformInfo } from "./components/PlatformInfo";
import { MetricsPanel } from "./components/MetricsPanel";
import {
  WorkspaceDto,
  HostCapabilities,
  SystemMetricsSnapshot,
  apiListWorkspaces,
  apiGetPlatformInfo,
  apiGetSystemMetrics,
} from "./api";
import { RefreshCw } from "lucide-react";

export default function App() {
  const [activeTab, setActiveTab] = useState("workspaces");
  const [workspaces, setWorkspaces] = useState<WorkspaceDto[]>([]);
  const [platformInfo, setPlatformInfo] = useState<HostCapabilities | null>(null);
  const [metrics, setMetrics] = useState<SystemMetricsSnapshot | null>(null);
  const [isCreatorOpen, setIsCreatorOpen] = useState(false);
  const [loading, setLoading] = useState(false);

  const fetchWorkspaces = async () => {
    setLoading(true);
    const data = await apiListWorkspaces();
    setWorkspaces(data);
    setLoading(false);
  };

  const fetchPlatform = async () => {
    const data = await apiGetPlatformInfo();
    setPlatformInfo(data);
  };

  const fetchMetrics = async () => {
    const data = await apiGetSystemMetrics();
    setMetrics(data);
  };

  useEffect(() => {
    fetchWorkspaces();
    fetchPlatform();
    fetchMetrics();

    const interval = setInterval(() => {
      fetchMetrics();
    }, 3000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-[#090d16] text-slate-100 font-sans">
      <Sidebar
        activeTab={activeTab}
        setActiveTab={setActiveTab}
        onNewWorkspace={() => setIsCreatorOpen(true)}
      />

      <main className="flex-1 flex flex-col h-screen overflow-y-auto">
        {/* Top Header */}
        <header className="h-16 border-b border-slate-800/60 px-8 flex items-center justify-between shrink-0 bg-slate-900/30 backdrop-blur-md">
          <div>
            <h2 className="text-lg font-bold text-white capitalize">
              {activeTab === "workspaces" && "Workspaces"}
              {activeTab === "platform" && "Host Platform Capabilities"}
              {activeTab === "metrics" && "System Monitor"}
              {activeTab === "settings" && "Application Settings"}
            </h2>
            <p className="text-xs text-slate-400">
              {activeTab === "workspaces" && "Manage your virtualization workspaces"}
              {activeTab === "platform" && "Host OS, CPU, GPU & installed runtime inspection"}
              {activeTab === "metrics" && "Live host CPU and memory resource tracking"}
              {activeTab === "settings" && "Configure PlazaVM platform preferences"}
            </p>
          </div>

          <button
            onClick={fetchWorkspaces}
            className="p-2 rounded-lg border border-slate-800 hover:bg-slate-800 text-slate-400 hover:text-white transition-colors"
            title="Refresh"
          >
            <RefreshCw className={`w-4 h-4 ${loading ? "animate-spin" : ""}`} />
          </button>
        </header>

        {/* Content Body */}
        <div className="flex-1 p-8">
          {activeTab === "workspaces" && (
            <WorkspaceList workspaces={workspaces} onRefresh={fetchWorkspaces} />
          )}

          {activeTab === "platform" && <PlatformInfo info={platformInfo} />}

          {activeTab === "metrics" && <MetricsPanel metrics={metrics} />}

          {activeTab === "settings" && (
            <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-6 max-w-xl space-y-4">
              <h3 className="font-semibold text-white">Platform Settings</h3>
              <div>
                <label className="block text-xs font-medium text-slate-300 mb-1">
                  Default Runtime Selection Strategy
                </label>
                <select className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-sm text-white focus:outline-none">
                  <option value="auto">Automatic (Decision Engine)</option>
                  <option value="docker">Prefer Docker</option>
                  <option value="qemu">Prefer QEMU</option>
                </select>
              </div>
            </div>
          )}
        </div>
      </main>

      <WorkspaceCreator
        isOpen={isCreatorOpen}
        onClose={() => setIsCreatorOpen(false)}
        onSuccess={fetchWorkspaces}
      />
    </div>
  );
};
