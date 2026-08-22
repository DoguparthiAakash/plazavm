import React from "react";
import {
  Play,
  Square,
  Terminal,
  Maximize2,
  Cpu,
  HardDrive,
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
      className="glass-card rounded-lg p-5 select-none cursor-pointer flex flex-col justify-between space-y-4 hover:border-zinc-500 transition-colors duration-200"
    >
      {/* Top Header */}
      <div className="flex items-start justify-between">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded border border-zinc-700 bg-zinc-800 flex items-center justify-center text-zinc-300">
            <Terminal className="w-4 h-4" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h3 className="font-semibold text-sm text-zinc-100">
                {workspace.name}
              </h3>
              <span className="text-[9px] font-mono px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-400 border border-zinc-700 font-medium">
                {envBadge}
              </span>
            </div>
            <p className="text-[11px] text-zinc-400 line-clamp-1 mt-0.5">{workspace.description}</p>
          </div>
        </div>

        {/* Status Pill */}
        <span
          className={`px-2 py-0.5 rounded text-[10px] font-mono font-medium flex items-center gap-1.5 border transition-colors ${
            isRunning
              ? "bg-zinc-800 text-green-400 border-zinc-700"
              : "bg-zinc-800 text-zinc-400 border-zinc-700"
          }`}
        >
          <span
            className={`w-1.5 h-1.5 rounded-full ${
              isRunning ? "bg-green-500" : "bg-zinc-500"
            }`}
          />
          {workspace.state.toUpperCase()}
        </span>
      </div>

      {/* Resource Allocation Bars */}
      <div className="space-y-2 pt-3 border-t border-zinc-800 text-xs">
        <div className="flex items-center justify-between text-[11px] font-mono text-zinc-400">
          <span className="flex items-center gap-1.5">
            <Cpu className="w-3.5 h-3.5 text-zinc-500" /> {workspace.cpu_cores} vCPUs
          </span>
          <span className="flex items-center gap-1.5">
            <HardDrive className="w-3.5 h-3.5 text-zinc-500" /> {workspace.memory_mb} MB RAM
          </span>
        </div>

        <div className="w-full bg-zinc-900 rounded-sm h-1 overflow-hidden border border-zinc-800">
          <div
            className="bg-blue-500 h-full rounded-sm transition-all duration-500"
            style={{ width: isRunning ? "42%" : "0%" }}
          />
        </div>
      </div>

      {/* Footer Info & Actions */}
      <div className="flex items-center justify-between pt-2">
        {/* Quick Action Buttons */}
        <div className="flex items-center gap-1.5">
          {isRunning ? (
            <button
              onClick={(e) => {
                e.stopPropagation();
                onStop(workspace.id);
              }}
              className="px-2.5 py-1.5 bg-zinc-800 hover:bg-zinc-700 text-red-400 border border-zinc-700 rounded text-xs font-medium transition-colors flex items-center gap-1.5"
            >
              <Square className="w-3 h-3 fill-current" /> Stop
            </button>
          ) : (
            <button
              onClick={(e) => {
                e.stopPropagation();
                onStart(workspace.id);
              }}
              className="px-2.5 py-1.5 bg-zinc-800 hover:bg-zinc-700 text-green-400 border border-zinc-700 rounded text-xs font-medium transition-colors flex items-center gap-1.5"
            >
              <Play className="w-3 h-3 fill-current" /> Start
            </button>
          )}

          <button
            onClick={(e) => {
              e.stopPropagation();
              onSelect(workspace);
            }}
            className="px-2.5 py-1.5 bg-zinc-800 hover:bg-zinc-700 text-blue-400 border border-zinc-700 rounded text-xs font-medium transition-colors flex items-center gap-1.5"
          >
            <Maximize2 className="w-3 h-3" /> Open
          </button>
        </div>

        {/* Backend Badge */}
        <span className="font-mono text-[10px] text-zinc-500 bg-zinc-900 px-1.5 py-0.5 rounded border border-zinc-800">
          {workspace.runtime_backend || "Plaza v86"}
        </span>
      </div>
    </div>
  );
};
