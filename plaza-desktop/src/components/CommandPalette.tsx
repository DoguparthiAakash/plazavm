import React, { useState, useEffect } from "react";
import { Search, Terminal, Cpu, Plug, FileText, Settings, ShieldCheck, Folder } from "lucide-react";

interface CommandPaletteProps {
  isOpen: boolean;
  onClose: () => void;
  onSelectAction: (action: string) => void;
}

export const CommandPalette: React.FC<CommandPaletteProps> = ({ isOpen, onClose, onSelectAction }) => {
  const [query, setQuery] = useState("");

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        if (isOpen) onClose();
        else onSelectAction("toggle-palette");
      }
      if (e.key === "Escape" && isOpen) {
        onClose();
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, onClose, onSelectAction]);

  if (!isOpen) return null;

  const commands = [
    { id: "workspaces", label: "Navigate to Workspaces", icon: Terminal, group: "Navigation" },
    { id: "platform", label: "Inspect Host Hardware & Platform", icon: Cpu, group: "Diagnostics" },
    { id: "plugins", label: "Manage Runtime Execution Plugins", icon: Plug, group: "Management" },
    { id: "validation", label: "Run Automated 16-Stage QA Certification", icon: ShieldCheck, group: "QA & Testing" },
    { id: "diagnostics", label: "Generate Diagnostic Archive Bundle", icon: FileText, group: "Support" },
    { id: "logs", label: "Open Application Log Folder", icon: Folder, group: "Support" },
    { id: "config", label: "Open Configuration Manager", icon: Settings, group: "Settings" },
  ];

  const filtered = commands.filter((c) =>
    c.label.toLowerCase().includes(query.toLowerCase())
  );

  return (
    <div className="fixed inset-0 bg-slate-950/70 backdrop-blur-sm z-50 flex items-start justify-center pt-24 p-4">
      <div className="bg-slate-900 border border-slate-800 rounded-xl max-w-xl w-full shadow-2xl overflow-hidden">
        <div className="flex items-center px-4 py-3 border-b border-slate-800 bg-slate-950/50">
          <Search className="w-5 h-5 text-slate-400 mr-3" />
          <input
            type="text"
            autoFocus
            placeholder="Type a command or search action (Esc to close)..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="bg-transparent border-none text-slate-100 placeholder-slate-500 text-sm focus:outline-none w-full"
          />
          <kbd className="text-[10px] text-slate-400 bg-slate-800 px-2 py-0.5 rounded border border-slate-700">ESC</kbd>
        </div>

        <div className="max-h-80 overflow-y-auto p-2">
          {filtered.length === 0 ? (
            <div className="p-4 text-center text-sm text-slate-500">No matching commands found.</div>
          ) : (
            filtered.map((cmd) => {
              const Icon = cmd.icon;
              return (
                <button
                  key={cmd.id}
                  onClick={() => {
                    onSelectAction(cmd.id);
                    onClose();
                  }}
                  className="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg hover:bg-cyan-500/10 hover:text-cyan-400 text-slate-300 text-sm text-left transition group"
                >
                  <Icon className="w-4 h-4 text-slate-400 group-hover:text-cyan-400" />
                  <span className="flex-1">{cmd.label}</span>
                  <span className="text-[10px] text-slate-500 bg-slate-800/80 px-2 py-0.5 rounded border border-slate-800">
                    {cmd.group}
                  </span>
                </button>
              );
            })
          )}
        </div>
      </div>
    </div>
  );
};
