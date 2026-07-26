import React, { useState } from "react";
import { HardDrive, Folder, Minimize2, Plus, Database, CheckCircle2, RefreshCw } from "lucide-react";

export const StorageView: React.FC = () => {
  const [isCompacting, setIsCompacting] = useState(false);
  const [compactionDone, setCompactionDone] = useState(false);

  const volumes = [
    { id: "1", name: "plaza-data-volume-01", size: "12.4 GB", max: "50 GB", type: "Sparse ext4 (VHDX)", attachedTo: "ubuntu-cuda-dev", format: "ext4" },
    { id: "2", name: "cargo-target-cache", size: "4.8 GB", max: "20 GB", type: "Shared Cache Volume", attachedTo: "rust-microservices", format: "btrfs" },
    { id: "3", name: "node-modules-global", size: "2.1 GB", max: "10 GB", type: "Shared NPM Cache", attachedTo: "node-web-app", format: "ext4" },
  ];

  const handleShrinkDisks = () => {
    setIsCompacting(true);
    setCompactionDone(false);
    setTimeout(() => {
      setIsCompacting(false);
      setCompactionDone(true);
    }, 2500);
  };

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6 select-none">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-black text-slate-100 tracking-tight flex items-center gap-2">
            <HardDrive className="w-6 h-6 text-cyan-400" />
            Storage & Persistent Volumes
          </h2>
          <p className="text-xs text-slate-400">
            Manage sparse virtual hard drives (VHDX), mount points, and automated disk compaction.
          </p>
        </div>

        <div className="flex items-center gap-3">
          <button
            onClick={handleShrinkDisks}
            disabled={isCompacting}
            className="flex items-center gap-2 px-4 py-2.5 bg-slate-900 hover:bg-slate-800 text-cyan-400 font-semibold rounded-xl text-xs border border-cyan-500/30 transition active:scale-95 disabled:opacity-50"
          >
            {isCompacting ? (
              <RefreshCw className="w-4 h-4 animate-spin text-cyan-400" />
            ) : (
              <Minimize2 className="w-4 h-4" />
            )}
            {isCompacting ? "Compacting VHDX..." : "Shrink Sparse Disks"}
          </button>

          <button className="flex items-center gap-2 px-4 py-2.5 bg-gradient-to-r from-cyan-500 to-teal-500 text-slate-950 font-bold rounded-xl text-xs shadow-lg shadow-cyan-500/20 transition active:scale-95">
            <Plus className="w-4 h-4 stroke-[3]" /> Create Volume
          </button>
        </div>
      </div>

      {compactionDone && (
        <div className="p-4 rounded-2xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 text-xs font-semibold flex items-center justify-between animate-in fade-in">
          <span className="flex items-center gap-2">
            <CheckCircle2 className="w-4 h-4" /> Disk Compaction Complete: Reclaimed 3.8 GB of unused host storage.
          </span>
          <button onClick={() => setCompactionDone(false)} className="text-xs hover:underline">
            Dismiss
          </button>
        </div>
      )}

      {/* Storage Volumes Grid */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-5">
        {volumes.map((vol) => (
          <div
            key={vol.id}
            className="p-5 rounded-2xl glass-card border border-slate-800 flex flex-col justify-between space-y-4 hover:border-cyan-500/40 transition"
          >
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Database className="w-4 h-4 text-cyan-400" />
                  <h3 className="text-xs font-bold text-slate-100 font-mono">{vol.name}</h3>
                </div>
                <span className="text-[10px] font-mono text-slate-400 bg-slate-900 px-2 py-0.5 rounded border border-slate-800">
                  {vol.format}
                </span>
              </div>
              <p className="text-[11px] text-slate-400 font-mono">Type: {vol.type}</p>
            </div>

            <div className="space-y-2">
              <div className="flex items-center justify-between text-xs font-mono">
                <span className="text-slate-400">Used: {vol.size}</span>
                <span className="text-slate-500">Max: {vol.max}</span>
              </div>
              <div className="w-full bg-slate-900 rounded-full h-2 overflow-hidden border border-slate-800">
                <div className="bg-gradient-to-r from-cyan-500 to-teal-400 h-full rounded-full w-[25%]" />
              </div>
            </div>

            <div className="pt-3 border-t border-slate-800/80 flex items-center justify-between text-[11px]">
              <span className="text-slate-400">Attached:</span>
              <span className="font-semibold text-cyan-300 font-mono">{vol.attachedTo}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
