import React, { useState, useEffect, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Search, Terminal, Settings, Server, RefreshCw, Cpu, X, ChevronRight } from "lucide-react";

interface Command {
  id: string;
  title: string;
  description: string;
  icon: React.ReactNode;
  action: string;
  shortcut?: string;
}

interface CommandPaletteProps {
  isOpen: boolean;
  onClose: () => void;
  onAction: (action: string, data?: any) => void;
}

const ALL_COMMANDS: Command[] = [
  { id: "new-ws", title: "Create new Workspace", description: "Initialize a new isolated environment", icon: <Server className="w-4 h-4" />, action: "NEW_WORKSPACE", shortcut: "N" },
  { id: "reload-ws", title: "Reload Workspaces", description: "Refresh the workspace list from the backend", icon: <RefreshCw className="w-4 h-4" />, action: "RELOAD_WORKSPACES", shortcut: "R" },
  { id: "check-ready", title: "Check System Readiness", description: "Re-evaluate host system components", icon: <Settings className="w-4 h-4" />, action: "CHECK_READINESS" },
  { id: "view-platform", title: "View Platform Info", description: "Inspect host CPU, memory, and GPU capabilities", icon: <Cpu className="w-4 h-4" />, action: "VIEW_PLATFORM" },
  { id: "open-terminal", title: "Open Terminal", description: "Launch a pseudo-terminal session", icon: <Terminal className="w-4 h-4" />, action: "OPEN_TERMINAL", shortcut: "T" },
];

export const CommandPalette: React.FC<CommandPaletteProps> = ({ isOpen, onClose, onAction }) => {
  const [query, setQuery] = useState("");
  const [selectedIdx, setSelectedIdx] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);

  const filtered = ALL_COMMANDS.filter(
    (c) =>
      c.title.toLowerCase().includes(query.toLowerCase()) ||
      c.description.toLowerCase().includes(query.toLowerCase())
  );

  useEffect(() => {
    if (isOpen) {
      setQuery("");
      setSelectedIdx(0);
      setTimeout(() => inputRef.current?.focus(), 50);
    }
  }, [isOpen]);

  useEffect(() => {
    setSelectedIdx(0);
  }, [query]);

  useEffect(() => {
    // Scroll selected item into view
    const el = listRef.current?.children[selectedIdx] as HTMLElement | undefined;
    el?.scrollIntoView({ block: "nearest" });
  }, [selectedIdx]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isOpen) return;

      if (e.key === "Escape") {
        e.preventDefault();
        onClose();
      } else if (e.key === "ArrowDown") {
        e.preventDefault();
        setSelectedIdx((prev) => Math.min(prev + 1, filtered.length - 1));
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        setSelectedIdx((prev) => Math.max(prev - 1, 0));
      } else if (e.key === "Enter" && filtered.length > 0) {
        e.preventDefault();
        const cmd = filtered[selectedIdx];
        if (cmd) {
          onAction(cmd.action);
          onClose();
        }
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, onClose, onAction, filtered, selectedIdx]);

  return (
    <AnimatePresence>
      {isOpen && (
        <div className="fixed inset-0 z-[100] flex items-start justify-center pt-[12vh] px-4">
          {/* Backdrop */}
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.15 }}
            onClick={onClose}
            className="fixed inset-0 bg-zinc-950/75 backdrop-blur-sm"
          />

          {/* Palette Container */}
          <motion.div
            initial={{ opacity: 0, y: -16, scale: 0.97 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: -16, scale: 0.97 }}
            transition={{ type: "spring", stiffness: 400, damping: 32 }}
            className="relative w-full max-w-xl bg-zinc-950 border border-zinc-800/80 rounded-2xl shadow-2xl overflow-hidden ring-1 ring-white/[0.06]"
          >
            {/* Search Input */}
            <div className="flex items-center px-4 py-3.5 border-b border-zinc-800/60 bg-zinc-900/20">
              <Search className="w-4 h-4 text-zinc-500 mr-3 shrink-0" />
              <input
                ref={inputRef}
                type="text"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Search commands..."
                className="flex-1 bg-transparent border-none outline-none text-base text-zinc-100 placeholder-zinc-600"
              />
              {query && (
                <button onClick={() => setQuery("")} className="ml-2 p-1 text-zinc-600 hover:text-zinc-400 rounded transition-colors">
                  <X className="w-3.5 h-3.5" />
                </button>
              )}
            </div>

            {/* Results */}
            <div ref={listRef} className="max-h-80 overflow-y-auto p-1.5">
              {filtered.length === 0 ? (
                <div className="py-10 text-center text-zinc-600 text-sm">
                  No commands found for "{query}"
                </div>
              ) : (
                filtered.map((cmd, idx) => (
                  <button
                    key={cmd.id}
                    onClick={() => { onAction(cmd.action); onClose(); }}
                    onMouseEnter={() => setSelectedIdx(idx)}
                    className={`w-full flex items-center px-3 py-2.5 rounded-xl text-sm text-left group transition-all ${
                      idx === selectedIdx
                        ? "bg-emerald-500/10 text-white"
                        : "text-zinc-400 hover:text-zinc-200"
                    }`}
                  >
                    <span className={`mr-3 shrink-0 transition-colors ${idx === selectedIdx ? "text-emerald-400" : "text-zinc-600"}`}>
                      {cmd.icon}
                    </span>
                    <div className="flex-1 min-w-0">
                      <div className={`font-medium truncate ${idx === selectedIdx ? "text-zinc-100" : "text-zinc-300"}`}>
                        {cmd.title}
                      </div>
                      <div className="text-xs text-zinc-600 truncate">{cmd.description}</div>
                    </div>
                    {cmd.shortcut && (
                      <kbd className="ml-3 shrink-0 text-[10px] font-sans px-1.5 py-0.5 bg-zinc-800 text-zinc-500 rounded border border-zinc-700">
                        {cmd.shortcut}
                      </kbd>
                    )}
                    <ChevronRight className={`ml-2 w-3.5 h-3.5 shrink-0 transition-opacity ${idx === selectedIdx ? "opacity-60" : "opacity-0"}`} />
                  </button>
                ))
              )}
            </div>

            {/* Footer */}
            <div className="px-4 py-2.5 border-t border-zinc-800/50 bg-zinc-900/20 flex items-center gap-4 text-[11px] text-zinc-600">
              <span><kbd className="font-sans px-1 py-0.5 bg-zinc-800 rounded border border-zinc-700 text-zinc-500">↑↓</kbd> navigate</span>
              <span><kbd className="font-sans px-1 py-0.5 bg-zinc-800 rounded border border-zinc-700 text-zinc-500">↵</kbd> select</span>
              <span><kbd className="font-sans px-1 py-0.5 bg-zinc-800 rounded border border-zinc-700 text-zinc-500">Esc</kbd> close</span>
              {filtered.length > 0 && (
                <span className="ml-auto text-zinc-700">{filtered.length} result{filtered.length !== 1 ? "s" : ""}</span>
              )}
            </div>
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
};
