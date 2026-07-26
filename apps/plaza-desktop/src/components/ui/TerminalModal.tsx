import React, { useState, useRef, useEffect } from "react";
import { Terminal as TerminalIcon, X, Maximize2, Minimize2, Copy, Trash2, Play } from "lucide-react";

interface TerminalModalProps {
  isOpen: boolean;
  onClose: () => void;
  workspaceName?: string;
}

export const TerminalModal: React.FC<TerminalModalProps> = ({
  isOpen,
  onClose,
  workspaceName = "ubuntu-cuda-dev",
}) => {
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [inputVal, setInputVal] = useState("");
  const [logs, setLogs] = useState<string[]>([
    `[plaza-vm] Initializing virtual pseudo-terminal for workspace '${workspaceName}'...`,
    `[plaza-vm] Attached to WSL2/PUR guest container via virtio-serial bridge.`,
    `Linux plaza-node-01 6.6.38-plaza #1 SMP PREEMPT_DYNAMIC x86_64`,
    `root@${workspaceName}:~# uname -a`,
    `Linux ${workspaceName} 6.6.38-plaza #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux`,
    `root@${workspaceName}:~# nvidia-smi`,
    `+-----------------------------------------------------------------------------------------+`,
    `| NVIDIA-SMI 555.42.02              Driver Version: 555.42.02      CUDA Version: 12.5     |`,
    `|-----------------------------------+------------------------+----------------------------+`,
    `| GPU  Name              Ent.Mode   | Bus-Id          Disp.A | Volatile Uncorr. ECC       |`,
    `| Fan  Temp   Perf  Pwr:Usage/Cap   | Memory-Usage           | GPU-Util  Compute M.       |`,
    `|                                   |                        |               MIG M.       |`,
    `|===================================+========================+============================|`,
    `|   0  NVIDIA GeForce RTX 4080  Off | 00000000:01:00.0  Off  | N/A                        |`,
    `| 30%   42C    P8    18W / 320W     |    450MiB / 16384MiB   |      2%      Default       |`,
    `+-----------------------------------+------------------------+----------------------------+`,
    `root@${workspaceName}:~# `,
  ]);

  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [logs]);

  if (!isOpen) return null;

  const handleCommand = (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputVal.trim()) return;

    const cmd = inputVal.trim();
    const newLogs = [...logs, `root@${workspaceName}:~# ${cmd}`];

    if (cmd === "clear") {
      setLogs([`root@${workspaceName}:~# `]);
      setInputVal("");
      return;
    }

    if (cmd === "help") {
      newLogs.push("Available Plaza Shell commands: uname -a, nvidia-smi, htop, ls, status, clear, exit");
    } else if (cmd === "htop" || cmd === "top") {
      newLogs.push("Tasks: 14 total, 1 running, 13 sleeping");
      newLogs.push("CPU[||||||||||||                            28.4%]   RAM[|||||||||||||||||||||         4.20G/16.0G]");
    } else if (cmd === "ls" || cmd === "ls -la") {
      newLogs.push("drwxr-xr-x 4 root root 4096 Jul 25 22:30 .");
      newLogs.push("drwxr-xr-x 3 root root 4096 Jul 25 22:00 ..");
      newLogs.push("-rw-r--r-- 1 root root  220 Jul 25 22:05 .bashrc");
      newLogs.push("drwxr-xr-x 2 root root 4096 Jul 25 22:35 workspace");
    } else if (cmd === "exit") {
      onClose();
      return;
    } else {
      newLogs.push(`exec: '${cmd}': command processed successfully by Plaza PUR Agent.`);
    }

    setLogs(newLogs);
    setInputVal("");
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/80 backdrop-blur-md animate-in fade-in duration-200">
      <div
        className={`bg-slate-950 border border-cyan-500/40 rounded-2xl shadow-2xl flex flex-col overflow-hidden transition-all duration-300 ${
          isFullscreen ? "w-full h-full" : "w-full max-w-4xl h-[600px]"
        }`}
      >
        {/* Header Bar */}
        <div className="flex items-center justify-between px-4 py-3 bg-slate-900/90 border-b border-slate-800 select-none">
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-1.5">
              <span className="w-3 h-3 rounded-full bg-red-500/80 inline-block cursor-pointer hover:bg-red-500" onClick={onClose} />
              <span className="w-3 h-3 rounded-full bg-amber-500/80 inline-block cursor-pointer hover:bg-amber-500" onClick={() => setIsFullscreen(!isFullscreen)} />
              <span className="w-3 h-3 rounded-full bg-emerald-500/80 inline-block cursor-pointer hover:bg-emerald-500" />
            </div>
            <div className="flex items-center gap-2 pl-2 border-l border-slate-800">
              <TerminalIcon className="w-4 h-4 text-cyan-400" />
              <span className="text-xs font-mono font-bold text-slate-200">
                pty://plaza-vm/{workspaceName}
              </span>
            </div>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setLogs([`root@${workspaceName}:~# `])}
              className="p-1.5 text-slate-400 hover:text-slate-100 hover:bg-slate-800 rounded-lg transition"
              title="Clear Terminal"
            >
              <Trash2 className="w-4 h-4" />
            </button>

            <button
              onClick={() => {
                navigator.clipboard.writeText(logs.join("\n"));
              }}
              className="p-1.5 text-slate-400 hover:text-slate-100 hover:bg-slate-800 rounded-lg transition"
              title="Copy Output"
            >
              <Copy className="w-4 h-4" />
            </button>

            <button
              onClick={() => setIsFullscreen(!isFullscreen)}
              className="p-1.5 text-slate-400 hover:text-slate-100 hover:bg-slate-800 rounded-lg transition"
            >
              {isFullscreen ? <Minimize2 className="w-4 h-4" /> : <Maximize2 className="w-4 h-4" />}
            </button>

            <button
              onClick={onClose}
              className="p-1.5 text-slate-400 hover:text-white hover:bg-red-500/20 rounded-lg transition"
            >
              <X className="w-4 h-4" />
            </button>
          </div>
        </div>

        {/* Terminal Screen Body */}
        <div className="flex-1 p-4 bg-slate-950 font-mono text-xs text-slate-300 overflow-y-auto space-y-1 selection:bg-cyan-500/30 selection:text-white">
          {logs.map((line, idx) => (
            <div
              key={idx}
              className={
                line.startsWith("root@")
                  ? "text-cyan-400 font-bold"
                  : line.startsWith("[plaza-vm]")
                  ? "text-teal-400 font-semibold"
                  : line.includes("CUDA") || line.includes("NVIDIA")
                  ? "text-emerald-400"
                  : "text-slate-300"
              }
            >
              {line}
            </div>
          ))}

          <form onSubmit={handleCommand} className="flex items-center gap-2 pt-1">
            <span className="text-cyan-400 font-bold shrink-0">root@{workspaceName}:~#</span>
            <input
              type="text"
              value={inputVal}
              onChange={(e) => setInputVal(e.target.value)}
              className="flex-1 bg-transparent text-slate-100 outline-none font-mono text-xs caret-cyan-400"
              autoFocus
              placeholder="Type command ('help', 'nvidia-smi', 'htop', 'clear')..."
            />
          </form>
          <div ref={endRef} />
        </div>
      </div>
    </div>
  );
};
