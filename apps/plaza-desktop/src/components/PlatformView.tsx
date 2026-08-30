import React, { useState, useEffect } from "react";
import { fetchPlatformInfo, HostCapabilities } from "../api";
import { Cpu, MemoryStick, Monitor, RefreshCw, Info } from "lucide-react";

export const PlatformView: React.FC = () => {
  const [info, setInfo] = useState<HostCapabilities | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await fetchPlatformInfo();
      setInfo(data);
    } catch (e: any) {
      setError(e.toString());
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(); }, []);

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-bold text-zinc-100">Host Platform</h1>
          <p className="text-sm text-zinc-500 mt-0.5">Detected hardware capabilities for workspace provisioning</p>
        </div>
        <button
          onClick={load}
          disabled={loading}
          className="flex items-center gap-2 text-sm text-zinc-400 hover:text-white bg-zinc-800 hover:bg-zinc-700 px-3 py-1.5 rounded-lg transition-colors"
        >
          <RefreshCw className={`w-4 h-4 ${loading ? "animate-spin" : ""}`} />
          Refresh
        </button>
      </div>

      {loading && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {[1, 2, 3, 4].map((i) => (
            <div key={i} className="bg-zinc-900/50 border border-zinc-800/60 rounded-xl p-5 animate-pulse h-32" />
          ))}
        </div>
      )}

      {error && (
        <div className="bg-red-950/30 border border-red-800/40 rounded-xl p-5 text-red-400 text-sm flex items-start gap-3">
          <Info className="w-4 h-4 mt-0.5 shrink-0" />
          <div>
            <p className="font-semibold mb-1">Failed to fetch platform info</p>
            <p className="text-red-400/70">{error}</p>
          </div>
        </div>
      )}

      {info && !loading && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {/* OS & Architecture */}
          <div className="bg-zinc-900/50 border border-zinc-800/60 rounded-xl p-5 hover:border-zinc-700 transition-colors">
            <div className="flex items-center gap-3 mb-4">
              <div className="w-8 h-8 rounded-lg bg-emerald-500/10 flex items-center justify-center">
                <Monitor className="w-4 h-4 text-emerald-400" />
              </div>
              <h3 className="text-sm font-semibold text-zinc-200">Operating System</h3>
            </div>
            <div className="space-y-2">
              <Row label="Name" value={info.os.name} />
              <Row label="Architecture" value={info.os.arch} />
            </div>
          </div>

          {/* CPU */}
          <div className="bg-zinc-900/50 border border-zinc-800/60 rounded-xl p-5 hover:border-zinc-700 transition-colors">
            <div className="flex items-center gap-3 mb-4">
              <div className="w-8 h-8 rounded-lg bg-blue-500/10 flex items-center justify-center">
                <Cpu className="w-4 h-4 text-blue-400" />
              </div>
              <h3 className="text-sm font-semibold text-zinc-200">Processor</h3>
            </div>
            <div className="space-y-2">
              <Row label="Model" value={info.cpu.model} />
              <Row label="Logical Cores" value={String(info.cpu.cores_logical)} />
            </div>
          </div>

          {/* Memory */}
          <div className="bg-zinc-900/50 border border-zinc-800/60 rounded-xl p-5 hover:border-zinc-700 transition-colors">
            <div className="flex items-center gap-3 mb-4">
              <div className="w-8 h-8 rounded-lg bg-violet-500/10 flex items-center justify-center">
                <MemoryStick className="w-4 h-4 text-violet-400" />
              </div>
              <h3 className="text-sm font-semibold text-zinc-200">Memory</h3>
            </div>
            <div className="space-y-2">
              <Row
                label="Total"
                value={`${(info.memory.total_mb / 1024).toFixed(1)} GB`}
              />
              <div className="mt-3">
                <div className="h-1.5 bg-zinc-800 rounded-full overflow-hidden">
                  <div className="h-full bg-gradient-to-r from-violet-600 to-violet-400 rounded-full w-1/3" />
                </div>
              </div>
            </div>
          </div>

          {/* GPU(s) */}
          <div className="bg-zinc-900/50 border border-zinc-800/60 rounded-xl p-5 hover:border-zinc-700 transition-colors">
            <div className="flex items-center gap-3 mb-4">
              <div className="w-8 h-8 rounded-lg bg-amber-500/10 flex items-center justify-center">
                <Monitor className="w-4 h-4 text-amber-400" />
              </div>
              <h3 className="text-sm font-semibold text-zinc-200">GPU</h3>
            </div>
            {info.gpu.length === 0 ? (
              <p className="text-xs text-zinc-500">No GPU detected</p>
            ) : (
              <div className="space-y-3">
                {info.gpu.map((g, i) => (
                  <div key={i} className="space-y-1">
                    <Row label="Name" value={g.name} />
                    <Row label="VRAM" value={`${(g.vram_mb / 1024).toFixed(1)} GB`} />
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

const Row: React.FC<{ label: string; value: string }> = ({ label, value }) => (
  <div className="flex items-center justify-between text-sm">
    <span className="text-zinc-500">{label}</span>
    <span className="text-zinc-200 font-mono text-xs">{value}</span>
  </div>
);
