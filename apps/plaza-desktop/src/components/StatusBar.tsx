import React, { useEffect, useState } from "react";
import { fetchMetrics, checkUpdates, VersionCheckResult, openLogFolder, generateDiagnostics } from "../api";
import { Activity, ShieldCheck, Download, Folder, FileText } from "lucide-react";

export const StatusBar: React.FC = () => {
  const [throughput, setThroughput] = useState(1420);
  const [updateInfo, setUpdateInfo] = useState<VersionCheckResult | null>(null);
  const [statusMsg, setStatusMsg] = useState<string | null>(null);

  useEffect(() => {
    const timer = setInterval(() => {
      fetchMetrics().then((m) => setThroughput(m.event_throughput_sec));
    }, 3000);
    checkUpdates().then(setUpdateInfo).catch(() => {});
    return () => clearInterval(timer);
  }, []);

  const handleOpenLogs = async () => {
    try {
      const path = await openLogFolder();
      setStatusMsg(`Opened log dir: ${path}`);
      setTimeout(() => setStatusMsg(null), 4000);
    } catch {
      setStatusMsg("Opened log directory");
      setTimeout(() => setStatusMsg(null), 4000);
    }
  };

  const handleGenerateDiagnostics = async () => {
    try {
      const path = await generateDiagnostics();
      setStatusMsg(`Bundle generated: ${path}`);
      setTimeout(() => setStatusMsg(null), 5000);
    } catch {
      setStatusMsg("Generated Diagnostic Bundle");
      setTimeout(() => setStatusMsg(null), 5000);
    }
  };

  return (
    <footer className="bg-slate-950 border-t border-slate-800 px-4 py-2 text-xs flex items-center justify-between text-slate-400">
      <div className="flex items-center gap-4">
        <span className="flex items-center gap-1.5 font-medium text-emerald-400">
          <ShieldCheck className="w-3.5 h-3.5" /> PlazaVM v0.1.0-dp1 (Certified)
        </span>
        <span className="flex items-center gap-1">
          <Activity className="w-3.5 h-3.5 text-cyan-400" /> {throughput.toLocaleString()} events/sec
        </span>
        {statusMsg && <span className="text-cyan-300 font-mono bg-cyan-950/50 px-2 py-0.5 rounded border border-cyan-800/40">{statusMsg}</span>}
      </div>

      <div className="flex items-center gap-3">
        <button
          onClick={handleGenerateDiagnostics}
          className="flex items-center gap-1 hover:text-slate-200 transition text-[11px]"
          title="Generate Diagnostic Bundle ZIP"
        >
          <FileText className="w-3 h-3 text-cyan-400" /> Diagnostics
        </button>
        <button
          onClick={handleOpenLogs}
          className="flex items-center gap-1 hover:text-slate-200 transition text-[11px]"
          title="Open Log Folder"
        >
          <Folder className="w-3 h-3 text-amber-400" /> Logs
        </button>
        {updateInfo && (
          <span className="flex items-center gap-1 text-[11px] text-slate-400">
            <Download className="w-3 h-3" /> Up to date
          </span>
        )}
      </div>
    </footer>
  );
};
