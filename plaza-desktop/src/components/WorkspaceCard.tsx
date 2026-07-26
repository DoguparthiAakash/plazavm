import React from "react";
import {
  Play,
  Square,
  Terminal,
  ExternalLink,
  Cpu,
  HardDrive,
  Maximize2,
} from "lucide-react";
import { WorkspaceDto } from "../api";

interface WorkspaceCardProps {
  workspace: WorkspaceDto;
  onStart: (id: string) => void;
  onStop: (id: string) => void;
  onSelect: (ws: WorkspaceDto) => void;
  onOpenTerminal?: () => void;
}

export const WorkspaceCard: React.FC<WorkspaceCardProps> = ({
  workspace,
  onStart,
  onStop,
  onSelect,
  onOpenTerminal,
}) => {
  const isRunning = workspace.state.toLowerCase() === "running";

  const nameLower = workspace.name.toLowerCase();
  const envBadge = nameLower.includes("cuda")
    ? "CUDA 12.5"
    : nameLower.includes("rust")
    ? "Rust 1.78"
    : nameLower.includes("node")
    ? "Node 22"
    : "Ubuntu 24.04";

  return (
    <div
      onClick={() => onSelect(workspace)}
      className="glass-card rounded-3xl p-5 select-none cursor-pointer flex flex-col justify-between space-y-4 hover:border-cyan-500/50 transition-all duration-200 relative group overflow-hidden"
    >
      {/* Background Subtle Accent Glow */}
      <div className="absolute top-0 right-0 w-32 h-32 bg-cyan-500/5 blur-2xl rounded-full pointer-events-none group-hover:bg-cyan-500/10 transition" />

      {/* Top Header */}
      <div className="flex items-start justify-between z-10">
        <div className="flex items-center gap-3">
          <div className="w-11 h-11 rounded-2xl bg-gradient-to-tr from-cyan-500/20 via-teal-500/20 to-emerald-500/10 border border-cyan-500/30 flex items-center justify-center text-cyan-400 font-black shadow-lg shadow-cyan-500/10">
            <Terminal className="w-5 h-5" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h3 className="font-extrabold text-sm text-slate-100 group-hover:text-cyan-300 transition">
                {workspace.name}
              </h3>
              <span className="text-[9px] font-mono px-2 py-0.5 rounded-md bg-cyan-500/10 text-cyan-400 border border-cyan-500/30 font-bold">
                {envBadge}
              </span>
            </div>
            <p className="text-[11px] text-slate-400 line-clamp-1 mt-0.5">{workspace.description}</p>
          </div>
        </div>

        {/* Status Pill */}
        <span
          className={`px-2.5 py-1 rounded-full text-[10px] font-mono font-bold flex items-center gap-1.5 border transition ${
            isRunning
              ? "bg-emerald-500/10 text-emerald-400 border-emerald-500/30 shadow-sm shadow-emerald-500/20"
              : "bg-slate-900 text-slate-400 border-slate-800"
          }`}
        >
          <span
            className={`w-1.5 h-1.5 rounded-full ${
              isRunning ? "bg-emerald-400 animate-ping" : "bg-slate-500"
            }`}
          />
          {workspace.state.toUpperCase()}
        </span>
      </div>

      {/* Resource Allocation Bars */}
      <div className="space-y-2 pt-2 border-t border-slate-800/80 text-xs z-10">
        <div className="flex items-center justify-between text-[11px] font-mono text-slate-400">
          <span className="flex items-center gap-1">
            <Cpu className="w-3.5 h-3.5 text-cyan-400" /> {workspace.cpu_cores} vCPUs
          </span>
          <span className="flex items-center gap-1">
            <HardDrive className="w-3.5 h-3.5 text-teal-400" /> {workspace.memory_mb} MB RAM
          </span>
        </div>

        <div className="w-full bg-slate-950/80 rounded-full h-1.5 overflow-hidden border border-slate-800/80">
          <div
            className="bg-gradient-to-r from-cyan-500 via-teal-400 to-emerald-400 h-full rounded-full transition-all duration-500"
            style={{ width: isRunning ? "42%" : "0%" }}
          />
        </div>
      </div>

      {/* Footer Info & Actions */}
      <div className="flex items-center justify-between pt-2 z-10">
        {/* Quick Action Buttons */}
        <div className="flex items-center gap-1.5">
          {isRunning ? (
            <button
              onClick={(e) => {
                e.stopPropagation();
                onStop(workspace.id);
              }}
              className="px-3 py-1.5 bg-red-500/10 hover:bg-red-500/20 text-red-400 border border-red-500/30 rounded-xl text-xs font-bold transition active:scale-95 flex items-center gap-1.5"
            >
              <Square className="w-3 h-3 fill-current" /> Stop
            </button>
          ) : (
            <button
              onClick={(e) => {
                e.stopPropagation();
                onStart(workspace.id);
              }}
              className="px-3 py-1.5 bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 rounded-xl text-xs font-bold transition active:scale-95 flex items-center gap-1.5"
            >
              <Play className="w-3.5 h-3.5 fill-current" /> Start
            </button>
          )}

          <button
            onClick={(e) => {
              e.stopPropagation();
              onSelect(workspace);
            }}
            className="px-3 py-1.5 bg-cyan-500/10 hover:bg-cyan-500/20 text-cyan-400 border border-cyan-500/30 rounded-xl text-xs font-bold transition active:scale-95 flex items-center gap-1.5"
          >
            <Maximize2 className="w-3 h-3" /> Open
          </button>
        </div>

        {/* Backend Badge */}
        <span className="font-mono text-[10px] text-slate-500 uppercase tracking-wider bg-slate-900 px-2 py-1 rounded-md border border-slate-800">
          {workspace.runtime_backend || "WSL2/PUR"}
        </span>
      </div>
    </div>
  );
};
