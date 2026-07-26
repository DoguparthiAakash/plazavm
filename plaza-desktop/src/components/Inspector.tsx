import React from "react";
import { X, Cpu, HardDrive, Terminal, Layers, ShieldCheck, ExternalLink, Code } from "lucide-react";
import { WorkspaceDto } from "../api";

interface InspectorProps {
  workspace: WorkspaceDto | null;
  onClose: () => void;
}

export const Inspector: React.FC<InspectorProps> = ({ workspace, onClose }) => {
  if (!workspace) return null;

  return (
    <aside className="w-80 bg-slate-950/80 backdrop-blur-xl border-l border-slate-800/60 p-5 flex flex-col justify-between select-none shrink-0 z-20 shadow-2xl">
      <div className="space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between pb-4 border-b border-slate-800/60">
          <div>
            <h3 className="font-extrabold text-sm text-slate-100">{workspace.name}</h3>
            <span className="font-mono text-[11px] text-cyan-400">{workspace.id}</span>
          </div>
          <button
            onClick={onClose}
            className="p-1.5 text-slate-400 hover:text-slate-100 hover:bg-slate-900 rounded-lg transition"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Live Status & Backend */}
        <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-4 space-y-3">
          <div className="flex items-center justify-between text-xs">
            <span className="text-slate-400">Execution State</span>
            <span className="px-2 py-0.5 rounded-full font-mono font-bold text-[10px] bg-emerald-500/10 text-emerald-400 border border-emerald-500/30">
              {workspace.state.toUpperCase()}
            </span>
          </div>
          <div className="flex items-center justify-between text-xs">
            <span className="text-slate-400">Backend Driver</span>
            <span className="font-mono text-cyan-400 font-medium">{workspace.runtime_backend}</span>
          </div>
          <div className="flex items-center justify-between text-xs">
            <span className="text-slate-400">Health Status</span>
            <span className="font-mono text-teal-400 font-medium">{workspace.health}</span>
          </div>
        </div>

        {/* Resource Telemetry */}
        <div className="space-y-3">
          <h4 className="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-2">
            <Cpu className="w-3.5 h-3.5 text-cyan-400" /> Resource Allocations
          </h4>
          <div className="grid grid-cols-2 gap-3 text-xs">
            <div className="bg-slate-900/40 border border-slate-800/60 rounded-xl p-3">
              <div className="text-[10px] text-slate-400">CPU Cores</div>
              <div className="font-mono font-bold text-slate-100 text-sm">{workspace.cpu_cores} Cores</div>
            </div>
            <div className="bg-slate-900/40 border border-slate-800/60 rounded-xl p-3">
              <div className="text-[10px] text-slate-400">RAM Limit</div>
              <div className="font-mono font-bold text-slate-100 text-sm">{workspace.memory_mb} MB</div>
            </div>
          </div>
        </div>

        {/* IDE Launchers */}
        <div className="space-y-3">
          <h4 className="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-2">
            <Code className="w-3.5 h-3.5 text-teal-400" /> Open in IDE
          </h4>
          <div className="grid grid-cols-2 gap-2">
            <button className="flex items-center justify-center gap-1.5 py-2 px-3 bg-slate-900 hover:bg-slate-800 border border-slate-800 text-slate-200 rounded-xl text-xs font-semibold transition">
              <ExternalLink className="w-3.5 h-3.5 text-cyan-400" /> VS Code
            </button>
            <button className="flex items-center justify-center gap-1.5 py-2 px-3 bg-slate-900 hover:bg-slate-800 border border-slate-800 text-slate-200 rounded-xl text-xs font-semibold transition">
              <ExternalLink className="w-3.5 h-3.5 text-teal-400" /> Cursor
            </button>
          </div>
        </div>
      </div>

      <div className="pt-4 border-t border-slate-800/60 text-[10px] text-slate-500 font-mono">
        Created: {new Date(workspace.created_at).toLocaleString()}
      </div>
    </aside>
  );
};
