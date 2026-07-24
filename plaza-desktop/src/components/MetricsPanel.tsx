import React from "react";
import { SystemMetricsSnapshot } from "../api";
import { Activity, Cpu, HardDrive } from "lucide-react";

interface MetricsPanelProps {
  metrics: SystemMetricsSnapshot | null;
}

export const MetricsPanel: React.FC<MetricsPanelProps> = ({ metrics }) => {
  if (!metrics) {
    return <div className="text-slate-400">Loading system metrics...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* CPU Usage Card */}
        <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <Cpu className="w-5 h-5 text-blue-400" />
              <h3 className="font-semibold text-white">CPU Utilization</h3>
            </div>
            <span className="text-2xl font-bold text-blue-400">
              {metrics.cpu_usage_pct.toFixed(1)}%
            </span>
          </div>
          <div className="w-full bg-slate-800 rounded-full h-3 overflow-hidden">
            <div
              className="bg-gradient-to-r from-blue-600 to-indigo-500 h-full transition-all duration-500"
              style={{ width: `${Math.min(metrics.cpu_usage_pct, 100)}%` }}
            ></div>
          </div>
        </div>

        {/* Memory Usage Card */}
        <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <HardDrive className="w-5 h-5 text-indigo-400" />
              <h3 className="font-semibold text-white">Memory Utilization</h3>
            </div>
            <span className="text-2xl font-bold text-indigo-400">
              {metrics.memory_usage_pct.toFixed(1)}%
            </span>
          </div>
          <div className="w-full bg-slate-800 rounded-full h-3 overflow-hidden">
            <div
              className="bg-gradient-to-r from-indigo-600 to-purple-500 h-full transition-all duration-500"
              style={{ width: `${Math.min(metrics.memory_usage_pct, 100)}%` }}
            ></div>
          </div>
          <div className="text-xs text-slate-400 mt-3 flex justify-between">
            <span>Used: {metrics.memory_used_mb} MB</span>
            <span>Total: {metrics.memory_total_mb} MB</span>
          </div>
        </div>
      </div>
    </div>
  );
};
