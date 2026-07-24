import React, { useEffect, useState } from "react";
import { fetchPlugins, PluginDto } from "../api";
import { Plug, CheckCircle2, AlertCircle, RefreshCw } from "lucide-react";

export const PluginManagerView: React.FC = () => {
  const [plugins, setPlugins] = useState<PluginDto[]>([]);

  const loadPlugins = () => {
    fetchPlugins().then(setPlugins);
  };

  useEffect(() => {
    loadPlugins();
  }, []);

  return (
    <div className="p-6 max-w-5xl mx-auto">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h2 className="text-xl font-bold text-slate-100">Runtime Execution Plugin Host</h2>
          <p className="text-xs text-slate-400">Registered runtime backend plugins and manifest capability matrix</p>
        </div>
        <button
          onClick={loadPlugins}
          className="flex items-center gap-2 px-3.5 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-medium rounded-lg border border-slate-700 transition"
        >
          <RefreshCw className="w-3.5 h-3.5" /> Refresh Plugins
        </button>
      </div>

      <div className="grid grid-cols-2 gap-4 mb-6">
        {plugins.map((plugin) => (
          <div key={plugin.id} className="bg-slate-900 border border-slate-800 rounded-xl p-5">
            <div className="flex items-start justify-between mb-3">
              <div className="flex items-center gap-2.5">
                <div className="p-2 bg-cyan-500/10 text-cyan-400 rounded-lg">
                  <Plug className="w-4 h-4" />
                </div>
                <div>
                  <h3 className="text-sm font-semibold text-slate-200">{plugin.name}</h3>
                  <div className="text-[11px] font-mono text-slate-500">ID: {plugin.id} | v{plugin.manifest.version}</div>
                </div>
              </div>
              <span className="flex items-center gap-1.5 px-2.5 py-0.5 bg-emerald-500/10 text-emerald-400 rounded text-[11px] font-medium border border-emerald-500/20">
                <CheckCircle2 className="w-3 h-3" /> Healthy
              </span>
            </div>

            <p className="text-xs text-slate-400 mb-4">{plugin.manifest.description}</p>

            <div className="text-[11px] font-semibold text-slate-500 uppercase tracking-wider mb-2">Capabilities</div>
            <div className="flex flex-wrap gap-1.5">
              {plugin.manifest.capabilities.map((cap, i) => (
                <span key={i} className="px-2 py-0.5 bg-slate-800 text-slate-300 rounded text-[10px] border border-slate-700">
                  {cap}
                </span>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
