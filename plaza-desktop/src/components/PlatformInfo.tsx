import React from "react";
import { HostCapabilities } from "../api";
import { Cpu, HardDrive, ShieldCheck, CheckCircle2 } from "lucide-react";

interface PlatformInfoProps {
  info: HostCapabilities | null;
}

export const PlatformInfo: React.FC<PlatformInfoProps> = ({ info }) => {
  if (!info) {
    return <div className="text-slate-400">Loading platform capabilities...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-5">
          <div className="text-xs text-slate-400 uppercase tracking-wider font-semibold mb-1">
            Host OS
          </div>
          <div className="text-xl font-bold text-white">
            {info.os.name} {info.os.version}
          </div>
          <div className="text-xs text-slate-500 mt-1">Arch: {info.os.arch}</div>
        </div>

        <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-5">
          <div className="text-xs text-slate-400 uppercase tracking-wider font-semibold mb-1">
            Processor
          </div>
          <div className="text-xl font-bold text-white">{info.cpu.model || "CPU Host"}</div>
          <div className="text-xs text-slate-500 mt-1">
            {info.cpu.cores_logical} Logical Cores
          </div>
        </div>

        <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-5">
          <div className="text-xs text-slate-400 uppercase tracking-wider font-semibold mb-1">
            Total Memory
          </div>
          <div className="text-xl font-bold text-white">{info.memory.total_mb} MB</div>
          <div className="text-xs text-slate-500 mt-1">
            {info.memory.available_mb} MB Available
          </div>
        </div>
      </div>

      <div>
        <h3 className="text-md font-semibold text-white mb-3">Detected Runtime Backends</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
          {info.installed_runtimes.map((rt) => (
            <div
              key={rt.id}
              className="bg-slate-900/60 border border-slate-800 rounded-xl p-4 flex items-center justify-between"
            >
              <div className="flex items-center gap-3">
                <div className="w-9 h-9 rounded-lg bg-blue-500/10 border border-blue-500/20 flex items-center justify-center text-blue-400">
                  <ShieldCheck className="w-5 h-5" />
                </div>
                <div>
                  <div className="font-semibold text-white text-sm">{rt.name}</div>
                  <div className="text-xs text-slate-400">Version: {rt.version}</div>
                </div>
              </div>
              <div className="flex items-center gap-1 text-xs text-emerald-400 font-medium bg-emerald-500/10 border border-emerald-500/20 px-2.5 py-1 rounded-full">
                <CheckCircle2 className="w-3.5 h-3.5" />
                <span>Available</span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
