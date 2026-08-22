import React from "react";
import { Search, Bell, Sun, Moon, Eye, ShieldCheck } from "lucide-react";
import { useTheme } from "./ui/ThemeContext";

interface TopBarProps {
  onOpenSearch: () => void;
  onOpenNotifications: () => void;
  activeBackend?: string;
}

export const TopBar: React.FC<TopBarProps> = ({
  onOpenSearch,
  onOpenNotifications,
  activeBackend = "Plaza v86 Runtime",
}) => {
  const { theme, toggleTheme } = useTheme();

  return (
    <header className="h-14 bg-zinc-950 border-b border-zinc-800 px-6 flex items-center justify-between select-none shrink-0 z-10">
      {/* Left: Backend System Indicator */}
      <div className="flex items-center gap-3">
        <div className="flex items-center gap-2 px-2.5 py-1 rounded bg-zinc-900 border border-zinc-800 text-xs font-mono">
          <span className="w-1.5 h-1.5 rounded-full bg-green-500" />
          <span className="text-zinc-400 font-medium">Backend:</span>
          <span className="text-zinc-100 font-medium">{activeBackend}</span>
        </div>

        <div className="hidden sm:flex items-center gap-1.5 text-[11px] text-zinc-500 font-mono">
          <ShieldCheck className="w-3.5 h-3.5 text-zinc-400" />
          <span>Native x86 Emulation</span>
        </div>
      </div>

      {/* Center: Global Search Bar Trigger */}
      <div className="flex-1 max-w-md mx-6">
        <button
          onClick={onOpenSearch}
          className="w-full flex items-center justify-between px-3 py-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 hover:border-zinc-600 rounded text-xs text-zinc-400 transition-colors"
        >
          <span className="flex items-center gap-2">
            <Search className="w-3.5 h-3.5 text-zinc-500" />
            <span>
              Search workspaces, images, settings...
            </span>
          </span>
          <kbd className="font-mono text-[10px] bg-zinc-950 px-1.5 py-0.5 rounded border border-zinc-800 text-zinc-500">
            Ctrl+Shift+P
          </kbd>
        </button>
      </div>

      {/* Right Actions: Theme Toggle, Notifications, User */}
      <div className="flex items-center gap-3">
        {/* Theme Switcher Toggle */}
        <button
          onClick={toggleTheme}
          className="p-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-400 hover:text-zinc-200 rounded transition-colors"
          title={`Current Theme: ${theme.toUpperCase()} (Click to toggle)`}
        >
          {theme === "dark" && <Moon className="w-4 h-4" />}
          {theme === "light" && <Sun className="w-4 h-4" />}
          {theme === "high-contrast" && <Eye className="w-4 h-4" />}
        </button>

        {/* Notifications Bell */}
        <button
          onClick={onOpenNotifications}
          className="p-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-400 hover:text-zinc-200 rounded transition-colors relative"
          title="Notifications"
        >
          <Bell className="w-4 h-4" />
          <span className="absolute top-1 right-1 w-1.5 h-1.5 rounded-full bg-blue-500" />
        </button>

        {/* User Status Avatar */}
        <div className="flex items-center gap-2 pl-3 border-l border-zinc-800">
          <div className="w-6 h-6 rounded bg-zinc-800 border border-zinc-700 flex items-center justify-center text-zinc-300 font-bold text-xs">
            D
          </div>
          <div className="hidden md:block text-left">
            <div className="text-xs font-medium text-zinc-200 leading-none mb-0.5">Developer</div>
            <div className="text-[9px] font-mono text-zinc-500 leading-tight">Local Connection</div>
          </div>
        </div>
      </div>
    </header>
  );
};
