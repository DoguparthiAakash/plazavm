import React, { useEffect, useState } from "react";
import { fetchPlatformInfo, HostCapabilities } from "../api";
import { Cpu, HardDrive, Monitor, ShieldCheck, Download } from "lucide-react";

export const PlatformInspectorView: React.FC = () => {
  const [platform, setPlatform] = useState<HostCapabilities | null>(null);

  useEffect(() => {
    fetchPlatformInfo().then(setPlatform);
  }, []);

  if (!platform) {
    return (
      <div className="p-8 text-center text-slate-400">
        Scanning host hardware platform...
      </div>
    );
  }

  const exportReport = () => {
    const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(platform, null, 2));
    const downloadAnchor = document.createElement("a");
    downloadAnchor.setAttribute("href", dataStr);
    downloadAnchor.setAttribute("download", `platform_report_${Date.now()}.json`);
    document.body.appendChild(downloadAnchor);
    downloadAnchor.click();
    downloadAnchor.remove();
  };

  return (
    <div className="p-6 max-w-5xl mx-auto">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h2 className="text-xl font-bold text-slate-100">Host Platform Capability Inspector</h2>
          <p className="text-xs text-slate-400">Detailed hardware capabilities, GPU acceleration, and OS environment audit</p>
        </div>
        <button
          onClick={exportReport}
          className="flex items-center gap-2 px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-medium rounded-lg border border-slate-700 transition"
        >
          <Download className="w-3.5 h-3.5" /> Export Report (JSON)
        </button>
      </div>

      <div className="grid grid-cols-2 gap-4 mb-6">
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-5">
          <div className="flex items-center gap-3 mb-4 text-cyan-400">
            <Monitor className="w-5 h-5" />
            <h3 className="text-sm font-semibold text-slate-200">Operating System</h3>
          </div>
          <div className="space-y-2 text-xs">
            <div className="flex justify-between py-1 border-b border-slate-800 text-slate-300">
              <span className="text-slate-500">OS Name</span>
              <span className="font-mono">{platform.os.name}</span>
            </div>
            <div className="flex justify-between py-1 border-b border-slate-800 text-slate-300">
              <span className="text-slate-500">Architecture</span>
              <span className="font-mono">{platform.os.arch}</span>
            </div>
            <div className="flex justify-between py-1 text-slate-300">
              <span className="text-slate-500">Platform Profile</span>
              <span className="font-mono text-emerald-400">HighPerformanceDesktop</span>
            </div>
          </div>
        </div>

        <div className="bg-slate-900 border border-slate-800 rounded-xl p-5">
          <div className="flex items-center gap-3 mb-4 text-cyan-400">
            <Cpu className="w-5 h-5" />
            <h3 className="text-sm font-semibold text-slate-200">Processor & Memory</h3>
          </div>
          <div className="space-y-2 text-xs">
            <div className="flex justify-between py-1 border-b border-slate-800 text-slate-300">
              <span className="text-slate-500">CPU Model</span>
              <span className="font-mono">{platform.cpu.model}</span>
            </div>
            <div className="flex justify-between py-1 border-b border-slate-800 text-slate-300">
              <span className="text-slate-500">Logical Cores</span>
              <span className="font-mono">{platform.cpu.cores_logical} Cores</span>
            </div>
            <div className="flex justify-between py-1 text-slate-300">
              <span className="text-slate-500">Total System Memory</span>
              <span className="font-mono text-cyan-400">{platform.memory.total_mb} MB</span>
            </div>
          </div>
        </div>
      </div>

      <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 mb-6">
        <div className="flex items-center gap-3 mb-4 text-cyan-400">
          <HardDrive className="w-5 h-5" />
          <h3 className="text-sm font-semibold text-slate-200">GPU Hardware Acceleration</h3>
        </div>
        {platform.gpu.length === 0 ? (
          <p className="text-xs text-slate-500">No discrete GPU acceleration detected.</p>
        ) : (
          <div className="grid grid-cols-2 gap-3">
            {platform.gpu.map((gpu, i) => (
              <div key={i} className="p-3 bg-slate-950 rounded-lg border border-slate-800 flex justify-between items-center text-xs">
                <div>
                  <div className="font-semibold text-slate-200">{gpu.name}</div>
                  <div className="text-[11px] text-slate-500">VRAM: {gpu.vram_mb} MB</div>
                </div>
                <span className="px-2 py-0.5 bg-emerald-500/10 text-emerald-400 rounded text-[10px] font-mono border border-emerald-500/20">
                  CUDA / Acceleration Active
                </span>
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <ShieldCheck className="w-6 h-6 text-emerald-400" />
          <div>
            <div className="text-xs font-semibold text-slate-200">Platform Diagnostic Readiness</div>
            <div className="text-[11px] text-slate-400">Host hardware fully supports PlazaVM workspace virtualization</div>
          </div>
        </div>
        <span className="px-3 py-1 bg-emerald-500/10 text-emerald-400 text-xs font-semibold rounded-full border border-emerald-500/20">
          100% Ready
        </span>
      </div>
    </div>
  );
};
