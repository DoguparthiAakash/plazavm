import React, { useEffect, useState } from "react";
import { fetchMetrics, SystemMetrics } from "../api";
import { Cpu, HardDrive, Activity } from "lucide-react";

interface MetricsPanelProps {
  metrics?: SystemMetrics | null;
}

export const MetricsPanel: React.FC<MetricsPanelProps> = ({ metrics: propMetrics }) => {
  const [internalMetrics, setInternalMetrics] = useState<SystemMetrics | null>(null);

  useEffect(() => {
    if (!propMetrics) {
      fetchMetrics().then(setInternalMetrics);
      const interval = setInterval(() => {
        fetchMetrics().then(setInternalMetrics);
      }, 3000);
      return () => clearInterval(interval);
    }
  }, [propMetrics]);

  const metrics = propMetrics || internalMetrics;

  if (!metrics) {
    return <div className="text-slate-400 text-xs p-4">Loading system metrics...</div>;
  }

  const memoryUsagePct = metrics.memory_total_mb > 0
    ? (metrics.memory_used_mb / metrics.memory_total_mb) * 100
    : 0;

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {/* CPU Usage Card */}
        <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-5">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <Cpu className="w-4 h-4 text-cyan-400" />
              <h3 className="text-xs font-semibold text-slate-200">CPU Utilization</h3>
            </div>
            <span className="text-xl font-bold text-cyan-400">
              {metrics.cpu_usage_pct.toFixed(1)}%
            </span>
          </div>
          <div className="w-full bg-slate-800 rounded-full h-2 overflow-hidden">
            <div
              className="bg-gradient-to-r from-cyan-500 to-blue-500 h-full transition-all duration-500"
              style={{ width: `${Math.min(metrics.cpu_usage_pct, 100)}%` }}
            ></div>
          </div>
        </div>

        {/* Memory Usage Card */}
        <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-5">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <HardDrive className="w-4 h-4 text-indigo-400" />
              <h3 className="text-xs font-semibold text-slate-200">Memory Utilization</h3>
            </div>
            <span className="text-xl font-bold text-indigo-400">
              {memoryUsagePct.toFixed(1)}%
            </span>
          </div>
          <div className="w-full bg-slate-800 rounded-full h-2 overflow-hidden mb-2">
            <div
              className="bg-gradient-to-r from-indigo-500 to-purple-500 h-full transition-all duration-500"
              style={{ width: `${Math.min(memoryUsagePct, 100)}%` }}
            ></div>
          </div>
          <div className="text-[11px] text-slate-400 flex justify-between">
            <span>Used: {metrics.memory_used_mb} MB</span>
            <span>Total: {metrics.memory_total_mb} MB</span>
          </div>
        </div>

        {/* System Throughput Card */}
        <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-5">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <Activity className="w-4 h-4 text-emerald-400" />
              <h3 className="text-xs font-semibold text-slate-200">Event Bus Throughput</h3>
            </div>
            <span className="text-xl font-bold text-emerald-400">
              {metrics.event_throughput_sec.toLocaleString()} /s
            </span>
          </div>
          <div className="text-[11px] text-slate-400">
            Active Workspaces: <span className="text-slate-200 font-semibold">{metrics.active_workspaces}</span>
          </div>
        </div>
      </div>
    </div>
  );
};
