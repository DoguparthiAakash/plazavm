import React, { useState, useRef, useEffect } from "react";
import {
  ArrowLeft,
  Play,
  Square,
  RotateCw,
  Pause,
  Terminal as TerminalIcon,
  ExternalLink,
  GitCommit,
  Trash2,
  FileText,
  Sliders,
  Activity,
  Package,
  Layers,
  Search,
  Copy,
  CheckCircle2,
  AlertTriangle,
  Cpu,
  HardDrive,
  Zap,
  Globe,
  Settings,
  Plus,
} from "lucide-react";
import { WorkspaceDto } from "../api";

interface FullWorkspaceDetailViewProps {
  workspace: WorkspaceDto;
  onBack: () => void;
  onStart: (id: string) => void;
  onStop: (id: string) => void;
}

export const FullWorkspaceDetailView: React.FC<FullWorkspaceDetailViewProps> = ({
  workspace,
  onBack,
  onStart,
  onStop,
}) => {
  const [activeTab, setActiveTab] = useState<"logs" | "terminal" | "inspect" | "metrics" | "snapshots" | "packages">("logs");
  const [isPaused, setIsPaused] = useState(false);
  const [logFilter, setLogFilter] = useState("");
  const [terminalInput, setTerminalInput] = useState("");
  const [terminalLogs, setTerminalLogs] = useState<string[]>([
    `[plaza-vm] Connected to workspace pseudo-terminal '${workspace.name}'...`,
    `[plaza-vm] Linux kernel 6.6.38-plaza #1 SMP PREEMPT_DYNAMIC x86_64`,
    `root@${workspace.name}:~# uname -a`,
    `Linux ${workspace.name} 6.6.38-plaza #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux`,
    `root@${workspace.name}:~# nvidia-smi`,
    `+-----------------------------------------------------------------------------------------+`,
    `| NVIDIA-SMI 555.42.02              Driver Version: 555.42.02      CUDA Version: 12.5     |`,
    `|-----------------------------------+------------------------+----------------------------+`,
    `| GPU  Name              Ent.Mode   | Bus-Id          Disp.A | Volatile Uncorr. ECC       |`,
    `| Fan  Temp   Perf  Pwr:Usage/Cap   | Memory-Usage           | GPU-Util  Compute M.       |`,
    `|===================================+========================+============================|`,
    `|   0  NVIDIA GeForce RTX 4080  Off | 00000000:01:00.0  Off  | N/A                        |`,
    `| 30%   42C    P8    18W / 320W     |    450MiB / 16384MiB   |      2%      Default       |`,
    `+-----------------------------------+------------------------+----------------------------+`,
    `root@${workspace.name}:~# `,
  ]);

  const [streamLogs, setStreamLogs] = useState<string[]>([
    `[2026-07-25 22:50:01] [INFO] PurDaemon initialized virtio-gpu acceleration layer.`,
    `[2026-07-25 22:50:02] [INFO] Mounting workspace volume /workspace at host E:\\plazavm\\projects\\cuda-lab`,
    `[2026-07-25 22:50:03] [INFO] Attached network interface plaza-br0 with IP 172.28.0.12`,
    `[2026-07-25 22:50:04] [INFO] Forwarding host port 8080 -> guest port 8080 (TCP)`,
    `[2026-07-25 22:50:05] [INFO] Forwarding host port 8888 -> guest port 8888 (Jupyter Notebook)`,
    `[2026-07-25 22:52:10] [INFO] PyTorch 2.3.0 GPU device [0] allocated: NVIDIA GeForce RTX 4080`,
    `[2026-07-25 22:55:00] [WARN] Memory ballooning requested compact: 450 MB page cache freed`,
    `[2026-07-25 22:58:30] [INFO] HTTP GET /api/v1/health 200 OK - 1.2ms`,
  ]);

  const terminalEndRef = useRef<HTMLDivElement>(null);
  const isRunning = workspace.state.toLowerCase() === "running";

  useEffect(() => {
    terminalEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [terminalLogs]);

  const handleTerminalSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!terminalInput.trim()) return;

    const cmd = terminalInput.trim();
    const newLogs = [...terminalLogs, `root@${workspace.name}:~# ${cmd}`];

    if (cmd === "clear") {
      setTerminalLogs([`root@${workspace.name}:~# `]);
      setTerminalInput("");
      return;
    }

    if (cmd === "help") {
      newLogs.push("Available shell commands: nvidia-smi, htop, ls, uname -a, ps aux, clear");
    } else if (cmd === "htop" || cmd === "top") {
      newLogs.push("Tasks: 16 total, 1 running, 15 sleeping");
      newLogs.push("CPU[||||||||||||                            22.4%]   RAM[|||||||||||||||||||          3.80G/16.0G]");
    } else if (cmd === "ls") {
      newLogs.push("drwxr-xr-x 4 root root 4096 Jul 25 22:30 .");
      newLogs.push("-rw-r--r-- 1 root root  220 Jul 25 22:05 .bashrc");
      newLogs.push("-rw-r--r-- 1 root root 1420 Jul 25 22:40 main.py");
      newLogs.push("drwxr-xr-x 2 root root 4096 Jul 25 22:35 src");
    } else {
      newLogs.push(`exec: '${cmd}': command completed with exit code 0.`);
    }

    setTerminalLogs(newLogs);
    setTerminalInput("");
  };

  const filteredLogs = streamLogs.filter((l) =>
    l.toLowerCase().includes(logFilter.toLowerCase())
  );

  return (
    <div className="flex flex-col h-full bg-slate-950 text-slate-100 select-none overflow-hidden">
      {/* 1. Header Breadcrumb & Workspace Branding Bar */}
      <div className="px-6 py-4 bg-slate-950/90 border-b border-slate-800/80 flex flex-col md:flex-row md:items-center justify-between gap-4 shrink-0">
        <div className="space-y-1">
          <button
            onClick={onBack}
            className="flex items-center gap-1.5 text-xs text-cyan-400 hover:text-cyan-300 font-semibold transition"
          >
            <ArrowLeft className="w-3.5 h-3.5" /> Back to Workspaces
          </button>

          <div className="flex items-center gap-3 pt-1">
            <div className="w-10 h-10 rounded-2xl bg-gradient-to-tr from-cyan-500/20 via-teal-500/20 to-emerald-500/20 border border-cyan-500/40 flex items-center justify-center text-cyan-400 font-black shadow-lg">
              <TerminalIcon className="w-5 h-5" />
            </div>

            <div>
              <div className="flex items-center gap-2">
                <h1 className="text-xl font-extrabold tracking-tight text-slate-100">
                  {workspace.name}
                </h1>
                <span
                  className={`px-2.5 py-0.5 rounded-full text-[10px] font-mono font-bold flex items-center gap-1.5 border ${
                    isRunning
                      ? "bg-emerald-500/10 text-emerald-400 border-emerald-500/30"
                      : "bg-slate-900 text-slate-400 border-slate-800"
                  }`}
                >
                  <span
                    className={`w-1.5 h-1.5 rounded-full ${
                      isRunning ? "bg-emerald-400 animate-ping" : "bg-slate-500"
                    }`}
                  />
                  {workspace.state.toUpperCase()}
                </span>
                <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-cyan-500/10 text-cyan-400 border border-cyan-500/30 font-bold">
                  {workspace.runtime_backend || "WSL2 Subsystem"}
                </span>
              </div>
              <p className="text-xs text-slate-400">{workspace.description}</p>
            </div>
          </div>
        </div>

        {/* 2. Docker-Style Action Toolbar */}
        <div className="flex items-center flex-wrap gap-2">
          {isRunning ? (
            <button
              onClick={() => onStop(workspace.id)}
              className="flex items-center gap-1.5 px-3.5 py-2 bg-red-500/10 hover:bg-red-500/20 text-red-400 border border-red-500/30 rounded-xl text-xs font-bold transition active:scale-95 shadow-sm"
            >
              <Square className="w-3.5 h-3.5 fill-current" /> Stop
            </button>
          ) : (
            <button
              onClick={() => onStart(workspace.id)}
              className="flex items-center gap-1.5 px-3.5 py-2 bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 rounded-xl text-xs font-bold transition active:scale-95 shadow-sm"
            >
              <Play className="w-3.5 h-3.5 fill-current" /> Start
            </button>
          )}

          <button
            onClick={() => {
              onStop(workspace.id);
              setTimeout(() => onStart(workspace.id), 1000);
            }}
            className="flex items-center gap-1.5 px-3.5 py-2 bg-slate-900 hover:bg-slate-800 text-slate-200 border border-slate-800 rounded-xl text-xs font-semibold transition active:scale-95"
            title="Restart Workspace"
          >
            <RotateCw className="w-3.5 h-3.5" /> Restart
          </button>

          <button
            onClick={() => setIsPaused(!isPaused)}
            className={`flex items-center gap-1.5 px-3.5 py-2 rounded-xl text-xs font-semibold border transition active:scale-95 ${
              isPaused
                ? "bg-amber-500/20 text-amber-300 border-amber-500/40"
                : "bg-slate-900 hover:bg-slate-800 text-slate-200 border-slate-800"
            }`}
          >
            <Pause className="w-3.5 h-3.5" /> {isPaused ? "Resume" : "Pause"}
          </button>

          <button
            onClick={() => setActiveTab("terminal")}
            className="flex items-center gap-1.5 px-3.5 py-2 bg-slate-900 hover:bg-slate-800 text-cyan-400 border border-cyan-500/30 rounded-xl text-xs font-semibold transition active:scale-95"
          >
            <TerminalIcon className="w-3.5 h-3.5" /> Exec PTY
          </button>

          <button
            onClick={() => alert(`Opening VS Code Remote for ${workspace.name}...`)}
            className="flex items-center gap-1.5 px-3.5 py-2 bg-slate-900 hover:bg-slate-800 text-slate-200 border border-slate-800 rounded-xl text-xs font-semibold transition active:scale-95"
          >
            <ExternalLink className="w-3.5 h-3.5 text-cyan-400" /> VS Code
          </button>

          <button
            onClick={() => alert(`Snapshot created for ${workspace.name}`)}
            className="flex items-center gap-1.5 px-3.5 py-2 bg-slate-900 hover:bg-slate-800 text-amber-400 border border-amber-500/30 rounded-xl text-xs font-semibold transition active:scale-95"
          >
            <GitCommit className="w-3.5 h-3.5" /> Snapshot
          </button>
        </div>
      </div>

      {/* 3. Navigation Tabs Bar */}
      <div className="px-6 bg-slate-950 border-b border-slate-800 flex items-center gap-2 overflow-x-auto shrink-0">
        {[
          { id: "logs", label: "Container Logs", icon: FileText },
          { id: "terminal", label: "Terminal Exec", icon: TerminalIcon },
          { id: "inspect", label: "Inspect & Config", icon: Sliders },
          { id: "metrics", label: "Live Metrics", icon: Activity },
          { id: "snapshots", label: "Snapshots", icon: GitCommit },
          { id: "packages", label: "Packages", icon: Package },
        ].map((tab) => {
          const Icon = tab.icon;
          const active = activeTab === tab.id;
          return (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`flex items-center gap-2 px-4 py-3 text-xs font-bold border-b-2 transition ${
                active
                  ? "border-cyan-400 text-cyan-400 bg-cyan-500/10"
                  : "border-transparent text-slate-400 hover:text-slate-200 hover:bg-slate-900/50"
              }`}
            >
              <Icon className="w-4 h-4" />
              {tab.label}
            </button>
          );
        })}
      </div>

      {/* 4. Tab Content Body */}
      <div className="flex-1 min-h-0 overflow-y-auto p-6">
        {/* LOGS TAB */}
        {activeTab === "logs" && (
          <div className="h-full flex flex-col bg-slate-950 border border-slate-800 rounded-3xl overflow-hidden shadow-2xl">
            <div className="flex items-center justify-between px-4 py-3 bg-slate-900/80 border-b border-slate-800 select-none">
              <div className="flex items-center gap-3">
                <Search className="w-4 h-4 text-slate-400" />
                <input
                  type="text"
                  value={logFilter}
                  onChange={(e) => setLogFilter(e.target.value)}
                  placeholder="Filter logs..."
                  className="bg-transparent text-xs text-slate-100 outline-none w-64 placeholder-slate-500"
                />
              </div>
              <button
                onClick={() => navigator.clipboard.writeText(streamLogs.join("\n"))}
                className="flex items-center gap-1.5 text-xs text-slate-400 hover:text-white px-3 py-1 bg-slate-900 rounded-xl border border-slate-800 transition"
              >
                <Copy className="w-3.5 h-3.5" /> Copy Output
              </button>
            </div>

            <div className="flex-1 p-4 font-mono text-xs text-slate-300 space-y-1.5 overflow-y-auto selection:bg-cyan-500/30 select-text">
              {filteredLogs.map((log, idx) => (
                <div
                  key={idx}
                  className={
                    log.includes("[ERROR]")
                      ? "text-red-400 font-bold"
                      : log.includes("[WARN]")
                      ? "text-amber-400"
                      : log.includes("[INFO]")
                      ? "text-slate-300"
                      : "text-slate-400"
                  }
                >
                  {log}
                </div>
              ))}
            </div>
          </div>
        )}

        {/* TERMINAL EXEC TAB */}
        {activeTab === "terminal" && (
          <div className="h-full flex flex-col bg-slate-950 border border-cyan-500/40 rounded-3xl overflow-hidden shadow-2xl">
            <div className="px-4 py-3 bg-slate-900 border-b border-slate-800 flex items-center justify-between select-none">
              <div className="flex items-center gap-2">
                <TerminalIcon className="w-4 h-4 text-cyan-400" />
                <span className="text-xs font-mono font-bold text-slate-200">
                  pty://{workspace.name} (root)
                </span>
              </div>
              <span className="text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/30">
                TTY Active
              </span>
            </div>

            <div className="flex-1 p-4 font-mono text-xs text-slate-300 overflow-y-auto space-y-1 selection:bg-cyan-500/30">
              {terminalLogs.map((line, idx) => (
                <div
                  key={idx}
                  className={
                    line.startsWith("root@")
                      ? "text-cyan-400 font-bold"
                      : line.startsWith("[plaza-vm]")
                      ? "text-teal-400 font-semibold"
                      : line.includes("NVIDIA") || line.includes("CUDA")
                      ? "text-emerald-400"
                      : "text-slate-300"
                  }
                >
                  {line}
                </div>
              ))}

              <form onSubmit={handleTerminalSubmit} className="flex items-center gap-2 pt-1">
                <span className="text-cyan-400 font-bold shrink-0">root@{workspace.name}:~#</span>
                <input
                  type="text"
                  value={terminalInput}
                  onChange={(e) => setTerminalInput(e.target.value)}
                  className="flex-1 bg-transparent text-slate-100 outline-none font-mono text-xs caret-cyan-400"
                  autoFocus
                  placeholder="Type shell command..."
                />
              </form>
              <div ref={terminalEndRef} />
            </div>
          </div>
        )}

        {/* INSPECT & CONFIG TAB */}
        {activeTab === "inspect" && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {/* Environment Variables */}
            <div className="p-6 rounded-3xl glass-card border border-slate-800 space-y-4">
              <h3 className="text-sm font-extrabold text-slate-100 flex items-center gap-2">
                <Sliders className="w-4 h-4 text-cyan-400" /> Environment Variables (ENV)
              </h3>
              <div className="space-y-2 font-mono text-xs">
                {[
                  { key: "PATH", val: "/usr/local/cuda/bin:/usr/local/sbin:/usr/bin" },
                  { key: "CUDA_VERSION", val: "12.5.0" },
                  { key: "PYTHONPATH", val: "/workspace/src" },
                  { key: "PLAZA_RUNTIME", val: "WSL2-PUR-6.6.38" },
                  { key: "LANG", val: "C.UTF-8" },
                ].map((item) => (
                  <div key={item.key} className="flex items-center justify-between p-2.5 bg-slate-900/60 rounded-xl border border-slate-800">
                    <span className="text-cyan-400 font-bold">{item.key}</span>
                    <span className="text-slate-300 truncate max-w-xs">{item.val}</span>
                  </div>
                ))}
              </div>
            </div>

            {/* Exposed Ports & Mounts */}
            <div className="space-y-6">
              <div className="p-6 rounded-3xl glass-card border border-slate-800 space-y-4">
                <h3 className="text-sm font-extrabold text-slate-100 flex items-center gap-2">
                  <Globe className="w-4 h-4 text-teal-400" /> Exposed Ports & Forwarding
                </h3>
                <div className="space-y-2 font-mono text-xs">
                  <div className="flex items-center justify-between p-2.5 bg-slate-900/60 rounded-xl border border-slate-800">
                    <span className="text-slate-300">Host :8080 &rarr; Guest :8080</span>
                    <span className="text-emerald-400 font-bold">TCP ACTIVE</span>
                  </div>
                  <div className="flex items-center justify-between p-2.5 bg-slate-900/60 rounded-xl border border-slate-800">
                    <span className="text-slate-300">Host :8888 &rarr; Guest :8888</span>
                    <span className="text-emerald-400 font-bold">Jupyter Active</span>
                  </div>
                </div>
              </div>

              <div className="p-6 rounded-3xl glass-card border border-slate-800 space-y-4">
                <h3 className="text-sm font-extrabold text-slate-100 flex items-center gap-2">
                  <HardDrive className="w-4 h-4 text-amber-400" /> Volume Mounts
                </h3>
                <div className="space-y-2 font-mono text-xs">
                  <div className="p-2.5 bg-slate-900/60 rounded-xl border border-slate-800 text-slate-300">
                    Host: <span className="text-cyan-300">E:\plazavm\projects\cuda-lab</span> &rarr; Guest: <span className="text-teal-300">/workspace</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* METRICS TAB */}
        {activeTab === "metrics" && (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div className="p-5 rounded-2xl glass-card border border-slate-800 space-y-3">
              <div className="flex items-center justify-between text-xs font-semibold text-slate-400">
                <span className="flex items-center gap-2">
                  <Cpu className="w-4 h-4 text-cyan-400" /> vCPU Usage
                </span>
                <span className="font-mono text-cyan-400">8 Cores</span>
              </div>
              <div className="text-2xl font-black text-slate-100 font-mono">22.4%</div>
              <div className="w-full bg-slate-900 rounded-full h-2 overflow-hidden border border-slate-800">
                <div className="bg-gradient-to-r from-cyan-500 to-teal-400 h-full w-[22.4%]" />
              </div>
            </div>

            <div className="p-5 rounded-2xl glass-card border border-slate-800 space-y-3">
              <div className="flex items-center justify-between text-xs font-semibold text-slate-400">
                <span className="flex items-center gap-2">
                  <HardDrive className="w-4 h-4 text-teal-400" /> Guest RAM
                </span>
                <span className="font-mono text-teal-400">16 GB Cap</span>
              </div>
              <div className="text-2xl font-black text-slate-100 font-mono">3.80 GB</div>
              <div className="w-full bg-slate-900 rounded-full h-2 overflow-hidden border border-slate-800">
                <div className="bg-gradient-to-r from-teal-500 to-emerald-400 h-full w-[23.7%]" />
              </div>
            </div>

            <div className="p-5 rounded-2xl glass-card border border-slate-800 space-y-3">
              <div className="flex items-center justify-between text-xs font-semibold text-slate-400">
                <span className="flex items-center gap-2">
                  <Zap className="w-4 h-4 text-emerald-400" /> GPU VRAM
                </span>
                <span className="font-mono text-emerald-400">RTX 4080</span>
              </div>
              <div className="text-2xl font-black text-slate-100 font-mono">450 MB</div>
              <div className="w-full bg-slate-900 rounded-full h-2 overflow-hidden border border-slate-800">
                <div className="bg-gradient-to-r from-emerald-500 to-cyan-400 h-full w-[3%]" />
              </div>
            </div>

            <div className="p-5 rounded-2xl glass-card border border-slate-800 space-y-3">
              <div className="flex items-center justify-between text-xs font-semibold text-slate-400">
                <span className="flex items-center gap-2">
                  <Activity className="w-4 h-4 text-amber-400" /> Network Tx/Rx
                </span>
                <span className="font-mono text-amber-400">virtio-net</span>
              </div>
              <div className="text-2xl font-black text-slate-100 font-mono">1.2 MB/s</div>
              <div className="w-full bg-slate-900 rounded-full h-2 overflow-hidden border border-slate-800">
                <div className="bg-gradient-to-r from-amber-500 to-orange-400 h-full w-[15%]" />
              </div>
            </div>
          </div>
        )}

        {/* SNAPSHOTS TAB */}
        {activeTab === "snapshots" && (
          <div className="p-6 rounded-3xl glass-card border border-slate-800 space-y-4">
            <div className="flex items-center justify-between border-b border-slate-800 pb-3">
              <h3 className="text-sm font-extrabold text-slate-100 flex items-center gap-2">
                <GitCommit className="w-4 h-4 text-amber-400" /> Point-in-Time Workspace Snapshots
              </h3>
              <button
                onClick={() => alert(`Created new snapshot for ${workspace.name}`)}
                className="flex items-center gap-1.5 px-3 py-1.5 bg-amber-500/10 hover:bg-amber-500/20 text-amber-400 border border-amber-500/30 rounded-xl text-xs font-bold transition"
              >
                <Plus className="w-3.5 h-3.5" /> Create Snapshot
              </button>
            </div>

            <div className="space-y-3 font-mono text-xs">
              <div className="p-4 bg-slate-900/60 rounded-2xl border border-slate-800 flex items-center justify-between">
                <div>
                  <h4 className="font-bold text-slate-200">c1a8f9204b &mdash; Pre-CUDA Update Backup</h4>
                  <p className="text-[11px] text-slate-400 font-sans mt-0.5">Created 2026-07-25 22:30:00 UTC</p>
                </div>
                <button
                  onClick={() => alert(`Rolled back workspace ${workspace.name} to c1a8f9204b`)}
                  className="px-3 py-1.5 bg-cyan-500/10 hover:bg-cyan-500/20 text-cyan-400 border border-cyan-500/30 rounded-xl text-xs font-bold transition"
                >
                  Rollback to Here
                </button>
              </div>
            </div>
          </div>
        )}

        {/* PACKAGES TAB */}
        {activeTab === "packages" && (
          <div className="p-6 rounded-3xl glass-card border border-slate-800 space-y-4">
            <h3 className="text-sm font-extrabold text-slate-100 flex items-center gap-2">
              <Package className="w-4 h-4 text-emerald-400" /> Installed Packages
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 font-mono text-xs">
              {[
                { name: "pytorch", ver: "v2.3.0", type: "pip" },
                { name: "cuda-toolkit", ver: "v12.5.0", type: "apt" },
                { name: "tokio", ver: "v1.38.0", type: "cargo" },
                { name: "numpy", ver: "v1.26.4", type: "pip" },
                { name: "python3", ver: "v3.11.9", type: "apt" },
                { name: "git", ver: "v2.45.1", type: "apt" },
              ].map((pkg) => (
                <div key={pkg.name} className="p-3 bg-slate-900/60 rounded-xl border border-slate-800 flex items-center justify-between">
                  <span className="font-bold text-slate-200">{pkg.name}</span>
                  <span className="text-emerald-400 text-[11px]">{pkg.ver} ({pkg.type})</span>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
