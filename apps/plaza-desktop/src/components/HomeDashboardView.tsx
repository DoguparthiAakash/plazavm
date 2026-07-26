import React from "react";
import {
  Cpu,
  HardDrive,
  Activity,
  Zap,
  Play,
  Square,
  Terminal,
  Clock,
  Pin,
  Sparkles,
  Plus,
  ArrowUpRight,
  ShieldCheck,
  Globe,
  Layers,
} from "lucide-react";
import { WorkspaceDto } from "../api";
import { WorkspaceCard } from "./WorkspaceCard";

interface HomeDashboardViewProps {
  workspaces: WorkspaceDto[];
  onSelectWorkspace: (ws: WorkspaceDto) => void;
  onCreateWorkspace: () => void;
  onNavigateTab: (tab: string) => void;
  onOpenTerminal: (wsName: string) => void;
  onStartWorkspace: (id: string) => void;
  onStopWorkspace: (id: string) => void;
}

export const HomeDashboardView: React.FC<HomeDashboardViewProps> = ({
  workspaces,
  onSelectWorkspace,
  onCreateWorkspace,
  onNavigateTab,
  onOpenTerminal,
  onStartWorkspace,
  onStopWorkspace,
}) => {
  const activeCount = workspaces.filter((w) => w.state.toLowerCase() === "running").length;

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-8 select-none">
      {/* Header Banner */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 bg-gradient-to-r from-slate-900/90 via-slate-900/60 to-cyan-950/40 p-6 rounded-3xl border border-slate-800/80 shadow-2xl relative overflow-hidden">
        <div className="absolute right-0 top-0 w-96 h-96 bg-cyan-500/10 blur-3xl rounded-full pointer-events-none" />
        <div className="space-y-1 z-10">
          <div className="flex items-center gap-2">
            <span className="px-2.5 py-0.5 rounded-full text-[10px] font-mono font-bold bg-cyan-500/10 text-cyan-400 border border-cyan-500/30">
              SYSTEM ACTIVE
            </span>
            <span className="text-xs text-slate-400 font-mono">PUR Kernel v6.6.38</span>
          </div>
          <h1 className="text-2xl font-black text-slate-100 tracking-tight">
            Plaza Control Center
          </h1>
          <p className="text-xs text-slate-400 max-w-xl">
            Hyper-optimized desktop runtime orchestrating containerized and virtualized environments.
          </p>
        </div>

        <div className="flex items-center gap-3 z-10">
          <button
            onClick={onCreateWorkspace}
            className="flex items-center gap-2 px-4 py-2.5 bg-gradient-to-r from-cyan-500 to-teal-500 hover:from-cyan-400 hover:to-teal-400 text-slate-950 font-bold rounded-xl text-xs shadow-xl shadow-cyan-500/20 transition active:scale-95"
          >
            <Plus className="w-4 h-4 stroke-[3]" />
            New Workspace
          </button>
          <button
            onClick={() => onNavigateTab("registry")}
            className="flex items-center gap-2 px-4 py-2.5 bg-slate-900/80 hover:bg-slate-800 text-slate-200 font-semibold rounded-xl text-xs border border-slate-700/60 transition active:scale-95"
          >
            <Globe className="w-4 h-4 text-cyan-400" />
            Explore Registry
          </button>
        </div>
      </div>

      {/* Glanceable Metrics Gauges */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {/* CPU Gauge */}
        <div className="p-4 rounded-2xl glass-card border border-slate-800 space-y-3 relative overflow-hidden group">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2 text-slate-400 text-xs font-semibold">
              <Cpu className="w-4 h-4 text-cyan-400" />
              <span>Host CPU Load</span>
            </div>
            <span className="text-[10px] font-mono text-cyan-400 bg-cyan-500/10 px-2 py-0.5 rounded border border-cyan-500/30">
              16 Cores
            </span>
          </div>
          <div className="flex items-baseline justify-between">
            <span className="text-2xl font-black text-slate-100 tracking-tight font-mono">18.4%</span>
            <span className="text-[11px] text-emerald-400 font-medium font-mono">3.8 GHz Avg</span>
          </div>
          <div className="w-full bg-slate-900 rounded-full h-2 overflow-hidden border border-slate-800">
            <div className="bg-gradient-to-r from-cyan-500 to-teal-400 h-full rounded-full w-[18.4%] transition-all duration-500" />
          </div>
        </div>

        {/* RAM Gauge */}
        <div className="p-4 rounded-2xl glass-card border border-slate-800 space-y-3 relative overflow-hidden group">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2 text-slate-400 text-xs font-semibold">
              <HardDrive className="w-4 h-4 text-teal-400" />
              <span>RAM Allocation</span>
            </div>
            <span className="text-[10px] font-mono text-teal-400 bg-teal-500/10 px-2 py-0.5 rounded border border-teal-500/30">
              32 GB Total
            </span>
          </div>
          <div className="flex items-baseline justify-between">
            <span className="text-2xl font-black text-slate-100 tracking-tight font-mono">4.2 GB</span>
            <span className="text-[11px] text-slate-400 font-mono">27.8 GB Free</span>
          </div>
          <div className="w-full bg-slate-900 rounded-full h-2 overflow-hidden border border-slate-800">
            <div className="bg-gradient-to-r from-teal-500 to-emerald-400 h-full rounded-full w-[13.1%] transition-all duration-500" />
          </div>
        </div>

        {/* GPU Gauge */}
        <div className="p-4 rounded-2xl glass-card border border-slate-800 space-y-3 relative overflow-hidden group">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2 text-slate-400 text-xs font-semibold">
              <Zap className="w-4 h-4 text-emerald-400" />
              <span>NVIDIA RTX 4080</span>
            </div>
            <span className="text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/30">
              16 GB VRAM
            </span>
          </div>
          <div className="flex items-baseline justify-between">
            <span className="text-2xl font-black text-slate-100 tracking-tight font-mono">450 MB</span>
            <span className="text-[11px] text-emerald-400 font-mono">CUDA 12.5</span>
          </div>
          <div className="w-full bg-slate-900 rounded-full h-2 overflow-hidden border border-slate-800">
            <div className="bg-gradient-to-r from-emerald-500 to-cyan-400 h-full rounded-full w-[3%] transition-all duration-500" />
          </div>
        </div>

        {/* Workspaces State */}
        <div className="p-4 rounded-2xl glass-card border border-slate-800 space-y-3 relative overflow-hidden group">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2 text-slate-400 text-xs font-semibold">
              <Activity className="w-4 h-4 text-amber-400" />
              <span>Active Environments</span>
            </div>
            <span className="text-[10px] font-mono text-amber-400 bg-amber-500/10 px-2 py-0.5 rounded border border-amber-500/30">
              {workspaces.length} Total
            </span>
          </div>
          <div className="flex items-baseline justify-between">
            <span className="text-2xl font-black text-slate-100 tracking-tight font-mono">
              {activeCount} Running
            </span>
            <span className="text-[11px] text-slate-400 font-mono">
              {workspaces.length - activeCount} Stopped
            </span>
          </div>
          <div className="w-full bg-slate-900 rounded-full h-2 overflow-hidden border border-slate-800">
            <div
              className="bg-gradient-to-r from-amber-500 to-orange-400 h-full rounded-full transition-all duration-500"
              style={{ width: `${workspaces.length ? (activeCount / workspaces.length) * 100 : 0}%` }}
            />
          </div>
        </div>
      </div>

      {/* Main Grid: Workspaces Showcase & Activity Feed */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Workspaces Section (2 Cols) */}
        <div className="lg:col-span-2 space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Terminal className="w-4 h-4 text-cyan-400" />
              <h2 className="text-base font-extrabold text-slate-100">Active Workspaces</h2>
            </div>
            <button
              onClick={() => onNavigateTab("workspaces")}
              className="text-xs text-cyan-400 hover:text-cyan-300 font-semibold flex items-center gap-1 transition"
            >
              View All ({workspaces.length}) <ArrowUpRight className="w-3.5 h-3.5" />
            </button>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {workspaces.slice(0, 4).map((ws) => (
              <WorkspaceCard
                key={ws.id}
                workspace={ws}
                onStart={onStartWorkspace}
                onStop={onStopWorkspace}
                onSelect={onSelectWorkspace}
                onOpenTerminal={() => onOpenTerminal(ws.name)}
              />
            ))}
          </div>
        </div>

        {/* Right Sidebar: Recent Activity & Featured Templates */}
        <div className="space-y-6">
          {/* Recent Activity */}
          <div className="p-5 rounded-3xl glass-card border border-slate-800 space-y-4">
            <div className="flex items-center justify-between border-b border-slate-800/80 pb-3">
              <div className="flex items-center gap-2 text-slate-100 font-extrabold text-xs">
                <Clock className="w-4 h-4 text-cyan-400" />
                <span>Recent System Activity</span>
              </div>
              <span className="text-[10px] font-mono text-slate-500">Live Stream</span>
            </div>

            <div className="space-y-3">
              <div className="flex items-start gap-3 text-xs">
                <div className="w-2 h-2 rounded-full bg-emerald-400 mt-1.5 shrink-0" />
                <div>
                  <p className="font-bold text-slate-200">Workspace Attached</p>
                  <p className="text-[11px] text-slate-400">ubuntu-cuda-dev connected via virtio-serial</p>
                  <span className="text-[10px] font-mono text-slate-500">10 mins ago</span>
                </div>
              </div>

              <div className="flex items-start gap-3 text-xs">
                <div className="w-2 h-2 rounded-full bg-cyan-400 mt-1.5 shrink-0" />
                <div>
                  <p className="font-bold text-slate-200">Snapshot Verified</p>
                  <p className="text-[11px] text-slate-400">Snapshot 'pre-cuda-update' checksum match</p>
                  <span className="text-[10px] font-mono text-slate-500">45 mins ago</span>
                </div>
              </div>

              <div className="flex items-start gap-3 text-xs">
                <div className="w-2 h-2 rounded-full bg-amber-400 mt-1.5 shrink-0" />
                <div>
                  <p className="font-bold text-slate-200">Kernel Page Compacted</p>
                  <p className="text-[11px] text-slate-400">PUR daemon released 450MB cached memory</p>
                  <span className="text-[10px] font-mono text-slate-500">2 hours ago</span>
                </div>
              </div>
            </div>
          </div>

          {/* Quick Registry Templates */}
          <div className="p-5 rounded-3xl glass-card border border-slate-800 space-y-4">
            <div className="flex items-center justify-between border-b border-slate-800/80 pb-3">
              <div className="flex items-center gap-2 text-slate-100 font-extrabold text-xs">
                <Layers className="w-4 h-4 text-teal-400" />
                <span>Featured Templates</span>
              </div>
              <button onClick={() => onNavigateTab("registry")} className="text-[10px] text-cyan-400 font-mono">
                Browser &rarr;
              </button>
            </div>

            <div className="space-y-2">
              <div
                onClick={onCreateWorkspace}
                className="p-2.5 rounded-xl bg-slate-900/60 hover:bg-slate-800 border border-slate-800 hover:border-cyan-500/40 cursor-pointer transition flex items-center justify-between"
              >
                <div>
                  <h4 className="text-xs font-bold text-slate-200">PyTorch 2.3 CUDA 12.5</h4>
                  <p className="text-[10px] text-slate-400">Deep Learning & GPU accelerated</p>
                </div>
                <span className="text-[10px] font-mono text-cyan-400 bg-cyan-500/10 px-2 py-0.5 rounded border border-cyan-500/30">
                  Deploy
                </span>
              </div>

              <div
                onClick={onCreateWorkspace}
                className="p-2.5 rounded-xl bg-slate-900/60 hover:bg-slate-800 border border-slate-800 hover:border-cyan-500/40 cursor-pointer transition flex items-center justify-between"
              >
                <div>
                  <h4 className="text-xs font-bold text-slate-200">Rust Tokio Microservice</h4>
                  <p className="text-[10px] text-slate-400">High throughput async environment</p>
                </div>
                <span className="text-[10px] font-mono text-teal-400 bg-teal-500/10 px-2 py-0.5 rounded border border-teal-500/30">
                  Deploy
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
