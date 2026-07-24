import React from "react";
import { WorkspaceDto, apiStartWorkspace, apiStopWorkspace } from "../api";
import { Play, Square, Cpu, HardDrive, ShieldCheck, ExternalLink } from "lucide-react";

interface WorkspaceListProps {
  workspaces: WorkspaceDto[];
  onRefresh: () => void;
}

export const WorkspaceList: React.FC<WorkspaceListProps> = ({ workspaces, onRefresh }) => {
  const handleStart = async (id: string) => {
    await apiStartWorkspace(id);
    onRefresh();
  };

  const handleStop = async (id: string) => {
    await apiStopWorkspace(id);
    onRefresh();
  };

  if (workspaces.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center p-16 text-center border border-dashed border-slate-800 rounded-xl bg-slate-900/20">
        <div className="w-12 h-12 rounded-full bg-slate-800 flex items-center justify-center text-slate-400 mb-4">
          <Cpu className="w-6 h-6" />
        </div>
        <h3 className="text-lg font-semibold text-white">No Workspaces Found</h3>
        <p className="text-sm text-slate-400 max-w-sm mt-1 mb-6">
          Create your first Workspace. PlazaVM will automatically select the optimal runtime backend.
        </p>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {workspaces.map((ws) => {
        const isRunning = ws.state.toLowerCase() === "running";
        return (
          <div
            key={ws.id}
            className="bg-slate-900/60 border border-slate-800 hover:border-slate-700/80 rounded-xl p-5 flex flex-col justify-between transition-all group shadow-sm hover:shadow-md"
          >
            <div>
              <div className="flex items-start justify-between gap-2 mb-2">
                <div>
                  <h3 className="font-semibold text-white group-hover:text-blue-400 transition-colors">
                    {ws.name}
                  </h3>
                  <span className="text-xs text-slate-400">{ws.description || "Workspace"}</span>
                </div>
                <div className="flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium border bg-slate-800/80 text-slate-300 border-slate-700">
                  <span
                    className={`w-2 h-2 rounded-full ${
                      isRunning ? "bg-emerald-400" : "bg-slate-500"
                    }`}
                  ></span>
                  <span className="capitalize">{ws.state}</span>
                </div>
              </div>

              {/* Resource Badges */}
              <div className="flex flex-wrap gap-2 my-4 text-xs">
                <span className="bg-slate-800/60 border border-slate-700/50 text-slate-300 px-2.5 py-1 rounded-md flex items-center gap-1">
                  <Cpu className="w-3.5 h-3.5 text-blue-400" />
                  {ws.cpu_cores} vCPU
                </span>
                <span className="bg-slate-800/60 border border-slate-700/50 text-slate-300 px-2.5 py-1 rounded-md flex items-center gap-1">
                  <HardDrive className="w-3.5 h-3.5 text-indigo-400" />
                  {ws.memory_mb} MB RAM
                </span>
                <span className="bg-slate-800/60 border border-slate-700/50 text-slate-300 px-2.5 py-1 rounded-md flex items-center gap-1">
                  <ShieldCheck className="w-3.5 h-3.5 text-emerald-400" />
                  Backend: {ws.runtime_backend || "Auto"}
                </span>
              </div>
            </div>

            {/* Actions */}
            <div className="flex items-center gap-2 pt-3 border-t border-slate-800/60">
              {isRunning ? (
                <button
                  onClick={() => handleStop(ws.id)}
                  className="flex-1 flex items-center justify-center gap-1.5 py-1.5 px-3 rounded-lg bg-red-500/10 border border-red-500/30 hover:bg-red-500/20 text-red-400 text-xs font-medium transition-colors"
                >
                  <Square className="w-3.5 h-3.5" />
                  Stop
                </button>
              ) : (
                <button
                  onClick={() => handleStart(ws.id)}
                  className="flex-1 flex items-center justify-center gap-1.5 py-1.5 px-3 rounded-lg bg-emerald-500/10 border border-emerald-500/30 hover:bg-emerald-500/20 text-emerald-400 text-xs font-medium transition-colors"
                >
                  <Play className="w-3.5 h-3.5" />
                  Start
                </button>
              )}
            </div>
          </div>
        );
      })}
    </div>
  );
};
