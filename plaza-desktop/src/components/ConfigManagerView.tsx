import React, { useState } from "react";
import { Settings, Download, Upload, RotateCcw, Save } from "lucide-react";

export const ConfigManagerView: React.FC = () => {
  const [configToml, setConfigToml] = useState(`# PlazaVM System Configuration (plaza.toml)
[system]
log_level = "info"
max_concurrent_workspaces = 8

[platform]
auto_profile_hardware = true
cuda_enabled = true

[storage]
database_file = "plaza.db"
backup_enabled = true
`);

  const [message, setMessage] = useState<string | null>(null);

  const handleSave = () => {
    setMessage("Configuration saved successfully.");
    setTimeout(() => setMessage(null), 3000);
  };

  const handleReset = () => {
    setConfigToml(`# PlazaVM System Configuration (plaza.toml)
[system]
log_level = "info"
max_concurrent_workspaces = 8

[platform]
auto_profile_hardware = true
cuda_enabled = true

[storage]
database_file = "plaza.db"
backup_enabled = true
`);
    setMessage("Configuration reset to defaults.");
    setTimeout(() => setMessage(null), 3000);
  };

  return (
    <div className="p-6 max-w-5xl mx-auto">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h2 className="text-xl font-bold text-slate-100">System Configuration Manager</h2>
          <p className="text-xs text-slate-400">Edit, import, export, or reset active system configuration (plaza.toml)</p>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={handleReset}
            className="flex items-center gap-2 px-3 py-2 bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-medium rounded-lg border border-slate-700 transition"
          >
            <RotateCcw className="w-3.5 h-3.5" /> Reset Defaults
          </button>
          <button
            onClick={handleSave}
            className="flex items-center gap-2 px-4 py-2 bg-cyan-600 hover:bg-cyan-500 text-white text-xs font-semibold rounded-lg shadow-md transition"
          >
            <Save className="w-3.5 h-3.5" /> Save Changes
          </button>
        </div>
      </div>

      {message && (
        <div className="mb-4 p-3 bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-xs rounded-lg">
          {message}
        </div>
      )}

      <div className="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <textarea
          value={configToml}
          onChange={(e) => setConfigToml(e.target.value)}
          rows={16}
          className="w-full bg-slate-950 border border-slate-800 rounded-lg p-4 font-mono text-xs text-slate-200 focus:outline-none focus:border-cyan-500"
        />
      </div>
    </div>
  );
};
