import React from "react";
import { Search, Bell, Sun, Moon, Eye, Command, ShieldCheck, Activity } from "lucide-react";
import { useTheme } from "./ui/ThemeContext";

interface TopBarProps {
  onOpenSearch: () => void;
  onOpenNotifications: () => void;
  activeBackend?: string;
}

export const TopBar: React.FC<TopBarProps> = ({
  onOpenSearch,
  onOpenNotifications,
  activeBackend = "WSL2 Subsystem",
}) => {
  const { theme, toggleTheme } = useTheme();

  return (
    <header className="h-14 bg-slate-950/80 backdrop-blur-xl border-b border-slate-800/80 px-6 flex items-center justify-between select-none shrink-0 z-10">
      {/* Left: Backend System Indicator */}
      <div className="flex items-center gap-3">
        <div className="flex items-center gap-2 px-3 py-1 rounded-full bg-slate-900 border border-slate-800 text-xs font-mono">
          <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
          <span className="text-slate-400 font-medium">Backend:</span>
          <span className="text-cyan-400 font-bold">{activeBackend}</span>
        </div>

        <div className="hidden sm:flex items-center gap-1.5 text-[11px] text-slate-400 font-mono">
          <ShieldCheck className="w-3.5 h-3.5 text-emerald-400" />
          <span>PUR Kernel 6.6.38-plaza</span>
        </div>
      </div>

      {/* Center: Global Search Bar Trigger */}
      <div className="flex-1 max-w-md mx-6">
        <button
          onClick={onOpenSearch}
          className="w-full flex items-center justify-between px-3.5 py-1.5 bg-slate-900/90 hover:bg-slate-900 border border-slate-800 hover:border-cyan-500/40 rounded-xl text-xs text-slate-400 transition shadow-inner group"
        >
          <span className="flex items-center gap-2">
            <Search className="w-3.5 h-3.5 text-slate-400 group-hover:text-cyan-400 transition" />
            <span className="group-hover:text-slate-200 transition">
              Search workspaces, images, settings...
            </span>
          </span>
          <kbd className="font-mono text-[10px] bg-slate-950 px-2 py-0.5 rounded border border-slate-800 text-slate-400">
            Ctrl+Shift+P
          </kbd>
        </button>
      </div>

      {/* Right Actions: Theme Toggle, Notifications, User */}
      <div className="flex items-center gap-3">
        {/* Theme Switcher Toggle */}
        <button
          onClick={toggleTheme}
          className="p-2 bg-slate-900/80 hover:bg-slate-800 border border-slate-800 text-slate-300 hover:text-cyan-400 rounded-xl text-xs transition relative group"
          title={`Current Theme: ${theme.toUpperCase()} (Click to toggle)`}
        >
          {theme === "dark" && <Moon className="w-4 h-4 text-cyan-400" />}
          {theme === "light" && <Sun className="w-4 h-4 text-amber-400" />}
          {theme === "high-contrast" && <Eye className="w-4 h-4 text-emerald-400" />}
        </button>

        {/* Notifications Bell */}
        <button
          onClick={onOpenNotifications}
          className="p-2 bg-slate-900/80 hover:bg-slate-800 border border-slate-800 text-slate-300 hover:text-cyan-400 rounded-xl text-xs transition relative"
          title="Notifications"
        >
          <Bell className="w-4 h-4" />
          <span className="absolute top-1.5 right-1.5 w-2 h-2 rounded-full bg-cyan-400 shadow-sm shadow-cyan-400" />
        </button>

        {/* User Status Avatar */}
        <div className="flex items-center gap-2 pl-2 border-l border-slate-800/80">
          <div className="w-7 h-7 rounded-xl bg-gradient-to-tr from-cyan-500 to-teal-400 flex items-center justify-center text-slate-950 font-black text-xs shadow-md">
            D
          </div>
          <div className="hidden md:block text-left">
            <div className="text-xs font-bold text-slate-200 leading-none">Developer Admin</div>
            <div className="text-[9px] font-mono text-emerald-400 leading-tight">Host Connected</div>
          </div>
        </div>
      </div>
    </header>
  );
};
