import React, { useState, useEffect } from "react";
import { getWorkspaceConfig, saveWorkspaceConfig } from "../api";
import { Settings, Save, X, Server, HardDrive, LayoutList, Terminal } from "lucide-react";
import { useToast } from "./ui/Toast";

interface ConfigEditorProps {
  workspaceId: string;
  onClose: () => void;
}

export const ConfigEditor: React.FC<ConfigEditorProps> = ({ workspaceId, onClose }) => {
  const [config, setConfig] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const { addToast } = useToast();

  useEffect(() => {
    const loadConfig = async () => {
      try {
        const data = await getWorkspaceConfig(workspaceId);
        setConfig(data);
      } catch (err: any) {
        addToast({ type: "error", title: "Config Error", message: err.toString() });
      } finally {
        setLoading(false);
      }
    };
    loadConfig();
  }, [workspaceId]);

  const handleSave = async () => {
    try {
      setSaving(true);
      await saveWorkspaceConfig(workspaceId, config);
      addToast({ type: "success", title: "Saved", message: "Configuration saved successfully." });
      onClose();
    } catch (err: any) {
      addToast({ type: "error", title: "Save Error", message: err.toString() });
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="fixed inset-0 bg-zinc-950/80 backdrop-blur-sm flex items-center justify-center z-50">
        <div className="animate-pulse text-emerald-500 flex items-center gap-2">
          <Settings className="w-5 h-5 animate-spin" /> Loading config...
        </div>
      </div>
    );
  }

  if (!config) {
    return (
      <div className="fixed inset-0 bg-zinc-950/80 backdrop-blur-sm flex items-center justify-center z-50">
        <div className="bg-zinc-900 border border-zinc-800 p-6 rounded-xl max-w-md w-full">
          <h2 className="text-xl text-white mb-4">Error</h2>
          <p className="text-zinc-400 mb-6">Could not load configuration.</p>
          <button onClick={onClose} className="w-full bg-zinc-800 text-white py-2 rounded-lg">Close</button>
        </div>
      </div>
    );
  }

  // Fallback defaults if fields missing
  const name = config.name || "";
  const description = config.description || "";
  const memory_mb = config.memory_mb || 512;
  const cpus = config.cpus || 1;
  const backend = config.backend || "qemu";

  return (
    <div className="fixed inset-0 bg-zinc-950/80 backdrop-blur-sm flex items-center justify-center z-50 p-4">
      <div className="bg-zinc-950 border border-zinc-800/80 rounded-2xl w-full max-w-2xl flex flex-col shadow-2xl overflow-hidden ring-1 ring-white/10">
        <div className="h-14 border-b border-zinc-800/50 flex items-center justify-between px-6 bg-zinc-900/50">
          <div className="flex items-center gap-3">
            <Settings className="w-5 h-5 text-emerald-400" />
            <h2 className="text-lg font-semibold text-zinc-100">Environment Editor</h2>
          </div>
          <button onClick={onClose} className="text-zinc-400 hover:text-white transition-colors">
            <X className="w-5 h-5" />
          </button>
        </div>
        
        <div className="p-6 overflow-y-auto max-h-[70vh] space-y-8 bg-zinc-950/50">
          {/* General Section */}
          <section>
            <h3 className="text-sm font-semibold text-emerald-400 uppercase tracking-wider mb-4 flex items-center gap-2">
              <LayoutList className="w-4 h-4" /> General
            </h3>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-xs text-zinc-400 mb-1">Name</label>
                <input
                  type="text"
                  value={name}
                  onChange={(e) => setConfig({ ...config, name: e.target.value })}
                  className="w-full bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2 text-sm text-zinc-200 focus:outline-none focus:border-emerald-500/50"
                />
              </div>
              <div>
                <label className="block text-xs text-zinc-400 mb-1">Backend</label>
                <select
                  value={backend}
                  onChange={(e) => setConfig({ ...config, backend: e.target.value })}
                  className="w-full bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2 text-sm text-zinc-200 focus:outline-none focus:border-emerald-500/50"
                >
                  <option value="qemu">QEMU</option>
                  <option value="v86">V86</option>
                  <option value="bhyve">Bhyve</option>
                </select>
              </div>
              <div className="col-span-2">
                <label className="block text-xs text-zinc-400 mb-1">Description</label>
                <input
                  type="text"
                  value={description}
                  onChange={(e) => setConfig({ ...config, description: e.target.value })}
                  className="w-full bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2 text-sm text-zinc-200 focus:outline-none focus:border-emerald-500/50"
                />
              </div>
            </div>
          </section>

          {/* Resources Section */}
          <section>
            <h3 className="text-sm font-semibold text-emerald-400 uppercase tracking-wider mb-4 flex items-center gap-2">
              <Server className="w-4 h-4" /> Resources
            </h3>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-xs text-zinc-400 mb-1">Memory (MB)</label>
                <input
                  type="number"
                  value={memory_mb}
                  onChange={(e) => setConfig({ ...config, memory_mb: parseInt(e.target.value) || 512 })}
                  className="w-full bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2 text-sm text-zinc-200 focus:outline-none focus:border-emerald-500/50"
                />
              </div>
              <div>
                <label className="block text-xs text-zinc-400 mb-1">CPU Cores</label>
                <input
                  type="number"
                  value={cpus}
                  onChange={(e) => setConfig({ ...config, cpus: parseInt(e.target.value) || 1 })}
                  className="w-full bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2 text-sm text-zinc-200 focus:outline-none focus:border-emerald-500/50"
                />
              </div>
            </div>
          </section>

          {/* Mounts Section */}
          <section>
            <h3 className="text-sm font-semibold text-emerald-400 uppercase tracking-wider mb-4 flex items-center gap-2">
              <HardDrive className="w-4 h-4" /> Mounts
            </h3>
            <div className="space-y-2">
              {(config.mounts || []).map((mount: any, idx: number) => (
                <div key={idx} className="flex items-center gap-2 bg-zinc-900/50 p-2 rounded-lg border border-zinc-800/50">
                  <input
                    type="text"
                    value={mount.host_path || ""}
                    placeholder="Host Path"
                    onChange={(e) => {
                      const newMounts = [...config.mounts];
                      newMounts[idx].host_path = e.target.value;
                      setConfig({ ...config, mounts: newMounts });
                    }}
                    className="flex-1 bg-zinc-900 border border-zinc-800 rounded-md px-2 py-1 text-xs text-zinc-200"
                  />
                  <span className="text-zinc-500">→</span>
                  <input
                    type="text"
                    value={mount.guest_path || ""}
                    placeholder="Guest Path"
                    onChange={(e) => {
                      const newMounts = [...config.mounts];
                      newMounts[idx].guest_path = e.target.value;
                      setConfig({ ...config, mounts: newMounts });
                    }}
                    className="flex-1 bg-zinc-900 border border-zinc-800 rounded-md px-2 py-1 text-xs text-zinc-200"
                  />
                  <button
                    onClick={() => {
                      const newMounts = config.mounts.filter((_: any, i: number) => i !== idx);
                      setConfig({ ...config, mounts: newMounts });
                    }}
                    className="p-1 text-red-400 hover:bg-red-500/20 rounded"
                  >
                    <X className="w-4 h-4" />
                  </button>
                </div>
              ))}
              <button
                onClick={() => {
                  const newMounts = [...(config.mounts || []), { host_path: "", guest_path: "" }];
                  setConfig({ ...config, mounts: newMounts });
                }}
                className="text-xs text-emerald-400 hover:text-emerald-300 font-medium"
              >
                + Add Mount
              </button>
            </div>
          </section>

          {/* Environment Variables */}
          <section>
            <h3 className="text-sm font-semibold text-emerald-400 uppercase tracking-wider mb-4 flex items-center gap-2">
              <Terminal className="w-4 h-4" /> Environment Variables
            </h3>
            <div className="space-y-2">
              {Object.entries(config.env || {}).map(([key, val], idx) => (
                <div key={idx} className="flex items-center gap-2 bg-zinc-900/50 p-2 rounded-lg border border-zinc-800/50">
                  <input
                    type="text"
                    value={key}
                    placeholder="KEY"
                    onChange={(e) => {
                      const newEnv = { ...config.env };
                      const value = newEnv[key];
                      delete newEnv[key];
                      newEnv[e.target.value] = value;
                      setConfig({ ...config, env: newEnv });
                    }}
                    className="w-1/3 bg-zinc-900 border border-zinc-800 rounded-md px-2 py-1 text-xs text-zinc-200"
                  />
                  <span className="text-zinc-500">=</span>
                  <input
                    type="text"
                    value={val as string}
                    placeholder="VALUE"
                    onChange={(e) => {
                      const newEnv = { ...config.env };
                      newEnv[key] = e.target.value;
                      setConfig({ ...config, env: newEnv });
                    }}
                    className="flex-1 bg-zinc-900 border border-zinc-800 rounded-md px-2 py-1 text-xs text-zinc-200"
                  />
                  <button
                    onClick={() => {
                      const newEnv = { ...config.env };
                      delete newEnv[key];
                      setConfig({ ...config, env: newEnv });
                    }}
                    className="p-1 text-red-400 hover:bg-red-500/20 rounded"
                  >
                    <X className="w-4 h-4" />
                  </button>
                </div>
              ))}
              <button
                onClick={() => {
                  const newEnv = { ...(config.env || {}), ["NEW_VAR"]: "" };
                  setConfig({ ...config, env: newEnv });
                }}
                className="text-xs text-emerald-400 hover:text-emerald-300 font-medium"
              >
                + Add Variable
              </button>
            </div>
          </section>
        </div>
        
        <div className="p-4 border-t border-zinc-800/50 bg-zinc-900/50 flex justify-end gap-3">
          <button
            onClick={onClose}
            className="px-4 py-2 text-sm font-medium text-zinc-400 hover:text-white"
          >
            Cancel
          </button>
          <button
            onClick={handleSave}
            disabled={saving}
            className="flex items-center gap-2 bg-emerald-600 hover:bg-emerald-500 text-white px-6 py-2 rounded-lg text-sm font-medium transition-colors shadow-lg shadow-emerald-500/20 disabled:opacity-50"
          >
            <Save className="w-4 h-4" />
            {saving ? "Saving..." : "Save Config"}
          </button>
        </div>
      </div>
    </div>
  );
};
