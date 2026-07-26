import React from "react";
import { WorkspaceDto, startWorkspace, stopWorkspace } from "../api";
import { WorkspaceCard } from "./WorkspaceCard";
import { Cpu } from "lucide-react";

interface WorkspaceListProps {
  workspaces: WorkspaceDto[];
  onRefresh: () => void;
  onSelectWorkspace: (ws: WorkspaceDto) => void;
}

export const WorkspaceList: React.FC<WorkspaceListProps> = ({
  workspaces,
  onRefresh,
  onSelectWorkspace,
}) => {
  const handleStart = async (id: string) => {
    await startWorkspace(id);
    onRefresh();
  };

  const handleStop = async (id: string) => {
    await stopWorkspace(id);
    onRefresh();
  };

  if (workspaces.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center p-16 text-center border border-dashed border-slate-800/80 rounded-2xl bg-slate-900/30">
        <div className="w-12 h-12 rounded-2xl bg-slate-800/60 border border-slate-700/60 flex items-center justify-center text-cyan-400 mb-4">
          <Cpu className="w-6 h-6" />
        </div>
        <h3 className="text-base font-extrabold text-slate-100">No Workspaces Found</h3>
        <p className="text-xs text-slate-400 max-w-sm mt-1 mb-6">
          Create your first Workspace. PlazaVM will automatically select the optimal runtime backend.
        </p>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
      {workspaces.map((ws) => (
        <WorkspaceCard
          key={ws.id}
          workspace={ws}
          onStart={(id) => handleStart(id as string)}
          onStop={(id) => handleStop(id as string)}
          onSelect={onSelectWorkspace}
        />
      ))}
    </div>
  );
};
