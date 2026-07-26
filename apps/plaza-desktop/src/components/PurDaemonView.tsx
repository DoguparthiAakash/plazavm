import React from "react";
import { HardDrive, CheckCircle2, Shield, Cpu, Activity, Database, Server } from "lucide-react";

export const PurDaemonView: React.FC = () => {
  const drivers = [
    { name: "Linux Native (cgroups v2 & OverlayFS)", status: "Active & Preferred" },
    { name: "WSL2 Subsystem", status: "Active (Windows Host)" },
    { name: "Hyper-V Backend", status: "Available" },
    { name: "Apple Virtualization Framework", status: "Available (macOS)" },
    { name: "FreeBSD Jails & ZFS", status: "Available (BSD)" },
    { name: "Docker Container Daemon", status: "Connected" },
  ];

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6 select-none">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-extrabold text-slate-100 tracking-tight flex items-center gap-3">
            <HardDrive className="w-6 h-6 text-teal-400" /> Plaza Utility Runtime (PUR) Daemon Status
          </h2>
          <p className="text-xs text-slate-400 mt-1">
            Independent, minimal immutable runtime operating environment powered by <span className="font-mono text-teal-300">purd</span>
          </p>
        </div>

        <span className="px-3 py-1 bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 font-mono text-xs font-bold rounded-xl flex items-center gap-2">
          <Activity className="w-4 h-4 animate-pulse" /> purd daemon: RUNNING
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-5">
        <div className="bg-slate-900/70 border border-slate-800/80 rounded-2xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <span className="text-xs text-slate-400 font-medium">IPC Endpoint Socket</span>
            <Server className="w-4 h-4 text-teal-400" />
          </div>
          <div className="font-mono text-xs font-bold text-slate-100 truncate">
            \\.\pipe\purd
          </div>
        </div>

        <div className="bg-slate-900/70 border border-slate-800/80 rounded-2xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <span className="text-xs text-slate-400 font-medium">OverlayFS Copy-on-Write</span>
            <Database className="w-4 h-4 text-teal-400" />
          </div>
          <div className="font-mono text-xs font-bold text-emerald-400">
            ENABLED & DEDUPLICATED
          </div>
        </div>

        <div className="bg-slate-900/70 border border-slate-800/80 rounded-2xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <span className="text-xs text-slate-400 font-medium">Security Verification</span>
            <Shield className="w-4 h-4 text-teal-400" />
          </div>
          <div className="font-mono text-xs font-bold text-teal-400">
            Ed25519 Signed & SPDX SBOM
          </div>
        </div>
      </div>

      <div className="bg-slate-900/70 border border-slate-800/80 rounded-2xl p-6 space-y-4">
        <h3 className="text-sm font-bold text-slate-200 flex items-center gap-2">
          <Cpu className="w-4 h-4 text-teal-400" /> Capability Matrix & Host Backend Adapters
        </h3>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
          {drivers.map((drv, i) => (
            <div key={i} className="flex items-center justify-between p-3 bg-slate-950/60 rounded-xl border border-slate-800/60 text-xs">
              <span className="font-medium text-slate-300">{drv.name}</span>
              <span className="flex items-center gap-1.5 font-mono text-[11px] text-emerald-400">
                <CheckCircle2 className="w-3.5 h-3.5" /> {drv.status}
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
