import React, { useState } from "react";
import { Cpu, HardDrive, Zap, Sliders, Shield, Save, CheckCircle2 } from "lucide-react";

export const ResourcesView: React.FC = () => {
  const [maxCpuCores, setMaxCpuCores] = useState(12);
  const [maxRamGb, setMaxRamGb] = useState(24);
  const [gpuMemoryLimit, setGpuMemoryLimit] = useState(12);
  const [saved, setSaved] = useState(false);

  const handleSave = () => {
    setSaved(true);
    setTimeout(() => setSaved(false), 3000);
  };

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6 select-none">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-black text-slate-100 tracking-tight flex items-center gap-2">
            <Sliders className="w-6 h-6 text-cyan-400" />
            Hardware Resource Limits
          </h2>
          <p className="text-xs text-slate-400">
            Configure host CPU core affinity, RAM memory caps, and NVIDIA CUDA GPU memory limits.
          </p>
        </div>

        <button
          onClick={handleSave}
          className="flex items-center gap-2 px-4 py-2.5 bg-gradient-to-r from-cyan-500 to-teal-500 text-slate-950 font-bold rounded-xl text-xs shadow-lg shadow-cyan-500/20 transition active:scale-95"
        >
          <Save className="w-4 h-4" /> Save Allocations
        </button>
      </div>

      {saved && (
        <div className="p-4 rounded-2xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 text-xs font-semibold flex items-center gap-2 animate-in fade-in">
          <CheckCircle2 className="w-4 h-4" /> Host hardware resource allocations updated and applied to PUR runtime.
        </div>
      )}

      {/* Allocation Controls */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* CPU Allocation */}
        <div className="p-6 rounded-3xl glass-card border border-slate-800 space-y-6">
          <div className="flex items-center gap-3">
            <div className="p-2.5 rounded-2xl bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
              <Cpu className="w-6 h-6" />
            </div>
            <div>
              <h3 className="text-sm font-extrabold text-slate-100">CPU Core Affinity</h3>
              <p className="text-[11px] text-slate-400">Host AMD Ryzen 7 (16 Threads)</p>
            </div>
          </div>

          <div className="space-y-3">
            <div className="flex items-center justify-between font-mono text-xs">
              <span className="text-slate-400">Allowed Logical Cores:</span>
              <span className="font-bold text-cyan-400">{maxCpuCores} Cores</span>
            </div>
            <input
              type="range"
              min="1"
              max="16"
              value={maxCpuCores}
              onChange={(e) => setMaxCpuCores(parseInt(e.target.value))}
              className="w-full accent-cyan-400 bg-slate-900 rounded-lg cursor-pointer"
            />
            <p className="text-[11px] text-slate-500 leading-relaxed">
              Restricts hypervisor workers to assigned logical CPU cores to prevent host stutter.
            </p>
          </div>
        </div>

        {/* RAM Allocation */}
        <div className="p-6 rounded-3xl glass-card border border-slate-800 space-y-6">
          <div className="flex items-center gap-3">
            <div className="p-2.5 rounded-2xl bg-teal-500/10 text-teal-400 border border-teal-500/20">
              <HardDrive className="w-6 h-6" />
            </div>
            <div>
              <h3 className="text-sm font-extrabold text-slate-100">RAM Limit Cap</h3>
              <p className="text-[11px] text-slate-400">32.0 GB Physical Host RAM</p>
            </div>
          </div>

          <div className="space-y-3">
            <div className="flex items-center justify-between font-mono text-xs">
              <span className="text-slate-400">Max Guest Memory:</span>
              <span className="font-bold text-teal-400">{maxRamGb} GB RAM</span>
            </div>
            <input
              type="range"
              min="2"
              max="32"
              step="2"
              value={maxRamGb}
              onChange={(e) => setMaxRamGb(parseInt(e.target.value))}
              className="w-full accent-teal-400 bg-slate-900 rounded-lg cursor-pointer"
            />
            <p className="text-[11px] text-slate-500 leading-relaxed">
              PurDaemon dynamic memory ballooning automatically returns unused pages to host OS.
            </p>
          </div>
        </div>

        {/* GPU Passthrough */}
        <div className="p-6 rounded-3xl glass-card border border-slate-800 space-y-6">
          <div className="flex items-center gap-3">
            <div className="p-2.5 rounded-2xl bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
              <Zap className="w-6 h-6" />
            </div>
            <div>
              <h3 className="text-sm font-extrabold text-slate-100">GPU VRAM Cap</h3>
              <p className="text-[11px] text-slate-400">NVIDIA RTX 4080 (16GB VRAM)</p>
            </div>
          </div>

          <div className="space-y-3">
            <div className="flex items-center justify-between font-mono text-xs">
              <span className="text-slate-400">VRAM Budget:</span>
              <span className="font-bold text-emerald-400">{gpuMemoryLimit} GB VRAM</span>
            </div>
            <input
              type="range"
              min="1"
              max="16"
              value={gpuMemoryLimit}
              onChange={(e) => setGpuMemoryLimit(parseInt(e.target.value))}
              className="w-full accent-emerald-400 bg-slate-900 rounded-lg cursor-pointer"
            />
            <p className="text-[11px] text-slate-500 leading-relaxed">
              Enables CUDA vGPU partitioning for concurrent PyTorch and TensorRT workloads.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};
