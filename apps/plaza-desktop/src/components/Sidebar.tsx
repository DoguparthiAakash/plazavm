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
    <aside className="w-64 bg-zinc-950 border-r border-zinc-800 flex flex-col justify-between p-4 select-none shrink-0 z-20">
      <div className="space-y-5 overflow-y-auto pr-1">
        {/* Brand Header */}
        <div className="flex items-center gap-3 px-2">
          <div className="w-9 h-9 rounded bg-blue-600 flex items-center justify-center font-bold text-white">
            P
          </div>
          <div>
            <h1 className="font-bold text-zinc-100 text-sm tracking-tight">
              Plaza Desktop
            </h1>
            <div className="text-[10px] text-zinc-500 font-mono font-medium">
              Control Center v1.0
            </div>
          </div>
        </div>

        {/* CTA Button */}
        <button
          onClick={onCreateWorkspace}
          className="w-full flex items-center justify-center gap-2 py-2.5 px-4 bg-blue-600 hover:bg-blue-500 text-white font-semibold rounded-md text-xs transition-colors duration-200"
        >
          <Plus className="w-4 h-4 stroke-[3]" /> New Workspace
        </button>

        {/* Nav Groupings */}
        <div className="space-y-4">
          {navSections.map((section) => (
            <div key={section.title} className="space-y-1">
              <div className="px-3 text-[9px] font-mono font-semibold tracking-wider text-zinc-500 uppercase">
                {section.title}
              </div>
              {section.items.map((item) => {
                const Icon = item.icon;
                const active = activeTab === item.id;
                return (
                  <button
                    key={item.id}
                    onClick={() => onTabChange(item.id)}
                    className={`w-full flex items-center justify-between px-3 py-2 rounded-md text-xs transition-colors duration-150 ${
                      active
                        ? "bg-zinc-800 text-zinc-100 font-medium"
                        : "text-zinc-400 hover:text-zinc-100 hover:bg-zinc-900"
                    }`}
                  >
                    <div className="flex items-center gap-2.5">
                      <Icon className={`w-4 h-4 ${active ? "text-blue-500" : "text-zinc-400"}`} />
                      <span>{item.label}</span>
                    </div>
                  </button>
                );
              })}
            </div>
          ))}
        </div>
      </div>

      {/* Footer */}
      <div className="pt-3 border-t border-zinc-800">
        <button
          onClick={onOpenShortcuts}
          className="w-full flex items-center justify-between px-3 py-2 rounded-md text-[11px] text-zinc-400 hover:text-zinc-200 hover:bg-zinc-900 transition-colors"
        >
          <span className="flex items-center gap-2 font-medium">
            <Keyboard className="w-3.5 h-3.5" /> Shortcuts
          </span>
          <kbd className="font-mono text-[9px] bg-zinc-900 px-1.5 py-0.5 rounded border border-zinc-700 text-zinc-300">
            Ctrl+K
          </kbd>
        </button>
      </div>
    </aside>
  );
};
