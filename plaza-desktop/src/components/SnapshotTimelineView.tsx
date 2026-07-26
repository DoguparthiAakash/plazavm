import React, { useState, useEffect } from "react";
import { GitCommit, RotateCcw, Clock, User, Package, PlusCircle } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

export const SnapshotTimelineView: React.FC = () => {
  const [timeline, setTimeline] = useState<any[]>([]);

  useEffect(() => {
    invoke<any[]>("get_snapshot_timeline").then(setTimeline).catch(() => {});
  }, []);

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6 select-none">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-extrabold text-slate-100 tracking-tight flex items-center gap-3">
            <GitCommit className="w-6 h-6 text-purple-400" /> Workspace State Control (WSC) Timeline
          </h2>
          <p className="text-xs text-slate-400 mt-1">
            Git-like immutable commit timeline & instant copy-on-write state rollback
          </p>
        </div>

        <button className="px-4 py-2 bg-gradient-to-r from-purple-500 to-indigo-500 text-slate-950 font-bold rounded-xl text-xs shadow-lg shadow-purple-500/20 transition flex items-center gap-2">
          <PlusCircle className="w-4 h-4" /> Create Snapshot Commit
        </button>
      </div>

      <div className="relative border-l-2 border-slate-800 ml-4 space-y-8 py-4">
        {timeline.map((commit, i) => (
          <div key={i} className="relative pl-8 group">
            {/* Timeline Dot */}
            <div className="absolute -left-[9px] top-1.5 w-4 h-4 rounded-full bg-slate-950 border-2 border-purple-400 group-hover:bg-purple-400 transition" />

            <div className="bg-slate-900/70 border border-slate-800/80 rounded-2xl p-5 hover:border-purple-500/40 transition shadow-lg space-y-3">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <span className="font-mono text-xs font-bold text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded-lg border border-purple-500/30">
                    {commit.commit_id}
                  </span>
                  <h3 className="font-bold text-sm text-slate-100">{commit.message}</h3>
                </div>

                <button className="px-3 py-1 bg-purple-500/10 hover:bg-purple-500/20 text-purple-400 border border-purple-500/30 rounded-lg text-xs font-semibold transition flex items-center gap-1.5">
                  <RotateCcw className="w-3.5 h-3.5" /> Rollback to Here
                </button>
              </div>

              <div className="flex items-center gap-6 text-xs text-slate-400">
                <span className="flex items-center gap-1.5">
                  <User className="w-3.5 h-3.5 text-slate-500" /> {commit.author}
                </span>
                <span className="flex items-center gap-1.5">
                  <Clock className="w-3.5 h-3.5 text-slate-500" /> {commit.timestamp}
                </span>
                <span className="flex items-center gap-1.5">
                  <Package className="w-3.5 h-3.5 text-slate-500" /> {commit.packages_count} Packages
                </span>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
