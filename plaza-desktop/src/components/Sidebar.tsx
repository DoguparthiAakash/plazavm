import React from "react";
import { Terminal, Cpu, Plug, ShieldCheck, Settings, Plus, Keyboard } from "lucide-react";

interface SidebarProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
  onCreateWorkspace: () => void;
  onOpenShortcuts: () => void;
}

export const Sidebar: React.FC<SidebarProps> = ({
  activeTab,
  onTabChange,
  onCreateWorkspace,
  onOpenShortcuts,
}) => {
  const navItems = [
    { id: "workspaces", label: "Workspaces", icon: Terminal },
    { id: "platform", label: "Platform Inspector", icon: Cpu },
    { id: "plugins", label: "Runtime Plugins", icon: Plug },
    { id: "validation", label: "QA Certification", icon: ShieldCheck },
    { id: "config", label: "Configuration", icon: Settings },
  ];

  return (
    <aside className="w-64 bg-slate-950 border-r border-slate-800 flex flex-col justify-between p-4 select-none shrink-0">
      <div>
        <div className="flex items-center gap-3 px-2 mb-6">
          <div className="w-8 h-8 rounded-lg bg-gradient-to-tr from-cyan-600 to-emerald-500 flex items-center justify-center font-black text-slate-950 shadow-md">
            P
          </div>
          <div>
            <h1 className="font-bold text-slate-100 text-sm tracking-wide">PlazaVM v2</h1>
            <div className="text-[10px] text-emerald-400 font-mono">Dev Preview (DP1)</div>
          </div>
        </div>

        <button
          onClick={onCreateWorkspace}
          className="w-full flex items-center justify-center gap-2 py-2.5 px-4 mb-6 bg-cyan-600 hover:bg-cyan-500 text-white rounded-lg text-xs font-semibold shadow-md shadow-cyan-900/20 transition"
        >
          <Plus className="w-4 h-4" /> New Workspace
        </button>

        <nav className="space-y-1">
          {navItems.map((item) => {
            const Icon = item.icon;
            const active = activeTab === item.id;
            return (
              <button
                key={item.id}
                onClick={() => onTabChange(item.id)}
                className={`w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-xs font-medium transition ${
                  active
                    ? "bg-cyan-500/10 text-cyan-400 border border-cyan-500/20"
                    : "text-slate-400 hover:text-slate-200 hover:bg-slate-900"
                }`}
              >
                <Icon className={`w-4 h-4 ${active ? "text-cyan-400" : "text-slate-500"}`} />
                {item.label}
              </button>
            );
          })}
        </nav>
      </div>

      <div className="pt-4 border-t border-slate-900">
        <button
          onClick={onOpenShortcuts}
          className="w-full flex items-center justify-between px-3 py-2 rounded-lg text-[11px] text-slate-400 hover:text-slate-200 hover:bg-slate-900 transition"
        >
          <span className="flex items-center gap-2">
            <Keyboard className="w-3.5 h-3.5" /> Shortcuts
          </span>
          <kbd className="font-mono text-[9px] bg-slate-800 px-1.5 py-0.5 rounded border border-slate-700">Ctrl+K</kbd>
        </button>
      </div>
    </aside>
  );
};
