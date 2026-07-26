import React from "react";
import {
  LayoutDashboard,
  Terminal,
  Layers,
  GitCommit,
  HardDrive,
  Package,
  Cpu,
  Plug,
  ShieldCheck,
  Settings,
  Plus,
  Keyboard,
  Globe,
  Network,
  Sliders,
} from "lucide-react";

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
  const navSections = [
    {
      title: "OVERVIEW",
      items: [
        { id: "dashboard", label: "Dashboard", icon: LayoutDashboard },
        { id: "workspaces", label: "Workspaces", icon: Terminal },
      ],
    },
    {
      title: "RESOURCES & REGISTRY",
      items: [
        { id: "registry", label: "Image Registry", icon: Globe },
        { id: "images", label: "Runtime Images", icon: Layers },
        { id: "snapshots", label: "Snapshots", icon: GitCommit },
        { id: "packages", label: "Package Engine", icon: Package },
      ],
    },
    {
      title: "INFRASTRUCTURE",
      items: [
        { id: "pur", label: "PUR Engine", icon: HardDrive },
        { id: "networking", label: "Networking", icon: Network },
        { id: "storage", label: "Storage Volumes", icon: HardDrive },
        { id: "resources", label: "Hardware Limits", icon: Sliders },
      ],
    },
    {
      title: "SYSTEM & PLUGINS",
      items: [
        { id: "platform", label: "Platform Inspector", icon: Cpu },
        { id: "plugins", label: "Plugins", icon: Plug },
        { id: "validation", label: "QA Certification", icon: ShieldCheck },
        { id: "config", label: "Settings & Config", icon: Settings },
      ],
    },
  ];

  return (
    <aside className="w-64 bg-slate-950/80 backdrop-blur-2xl border-r border-slate-800/80 flex flex-col justify-between p-4 select-none shrink-0 shadow-2xl z-20">
      <div className="space-y-5 overflow-y-auto pr-1">
        {/* Brand Header */}
        <div className="flex items-center gap-3 px-2">
          <div className="w-9 h-9 rounded-2xl bg-gradient-to-tr from-cyan-500 via-teal-400 to-emerald-400 flex items-center justify-center font-black text-slate-950 shadow-lg shadow-cyan-500/25">
            P
          </div>
          <div>
            <h1 className="font-black text-slate-100 text-sm tracking-tight gradient-text-cyan">
              Plaza Desktop
            </h1>
            <div className="text-[10px] text-cyan-400 font-mono font-semibold">
              Control Center v1.0
            </div>
          </div>
        </div>

        {/* CTA Button */}
        <button
          onClick={onCreateWorkspace}
          className="w-full flex items-center justify-center gap-2 py-2.5 px-4 bg-gradient-to-r from-cyan-500 via-teal-500 to-emerald-500 hover:from-cyan-400 hover:to-emerald-400 text-slate-950 font-black rounded-xl text-xs shadow-lg shadow-cyan-500/20 transition-all duration-200 active:scale-95"
        >
          <Plus className="w-4 h-4 stroke-[3]" /> New Workspace
        </button>

        {/* Nav Groupings */}
        <div className="space-y-4">
          {navSections.map((section) => (
            <div key={section.title} className="space-y-1">
              <div className="px-3 text-[9px] font-mono font-bold tracking-widest text-slate-500 uppercase">
                {section.title}
              </div>
              {section.items.map((item) => {
                const Icon = item.icon;
                const active = activeTab === item.id;
                return (
                  <button
                    key={item.id}
                    onClick={() => onTabChange(item.id)}
                    className={`w-full flex items-center justify-between px-3 py-2 rounded-xl text-xs font-semibold transition-all duration-150 ${
                      active
                        ? "bg-cyan-500/15 text-cyan-300 border border-cyan-500/40 shadow-md shadow-cyan-500/10 font-bold"
                        : "text-slate-400 hover:text-slate-100 hover:bg-slate-900/60"
                    }`}
                  >
                    <div className="flex items-center gap-2.5">
                      <Icon className={`w-4 h-4 ${active ? "text-cyan-400" : "text-slate-400"}`} />
                      <span>{item.label}</span>
                    </div>
                    {active && <span className="w-1.5 h-1.5 rounded-full bg-cyan-400 shadow-sm shadow-cyan-400" />}
                  </button>
                );
              })}
            </div>
          ))}
        </div>
      </div>

      {/* Footer */}
      <div className="pt-3 border-t border-slate-900/80">
        <button
          onClick={onOpenShortcuts}
          className="w-full flex items-center justify-between px-3 py-2 rounded-xl text-[11px] text-slate-400 hover:text-slate-200 hover:bg-slate-900/60 transition"
        >
          <span className="flex items-center gap-2 font-medium">
            <Keyboard className="w-3.5 h-3.5" /> Shortcuts
          </span>
          <kbd className="font-mono text-[9px] bg-slate-900 px-1.5 py-0.5 rounded border border-slate-800 text-slate-300">
            Ctrl+K
          </kbd>
        </button>
      </div>
    </aside>
  );
};
