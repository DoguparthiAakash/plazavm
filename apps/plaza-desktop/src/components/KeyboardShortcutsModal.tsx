import React from "react";
import { X, Keyboard } from "lucide-react";

interface ShortcutsProps {
  isOpen: boolean;
  onClose: () => void;
}

export const KeyboardShortcutsModal: React.FC<ShortcutsProps> = ({ isOpen, onClose }) => {
  if (!isOpen) return null;

  const shortcuts = [
    { key: "Ctrl + K / Cmd + K", action: "Open Instant Command Palette" },
    { key: "Ctrl + N", action: "Create New Workspace" },
    { key: "Ctrl + Shift + P", action: "Open Platform Inspector" },
    { key: "Ctrl + Shift + V", action: "Run QA Certification Pipeline" },
    { key: "Ctrl + Shift + D", action: "Generate Diagnostic Bundle ZIP" },
    { key: "Ctrl + Shift + L", action: "Open Application Log Folder" },
    { key: "Esc", action: "Close Modals / Overlays" },
  ];

  return (
    <div className="fixed inset-0 bg-slate-950/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div className="bg-slate-900 border border-slate-800 rounded-xl max-w-lg w-full p-6 shadow-2xl">
        <div className="flex items-center justify-between mb-4 border-b border-slate-800 pb-3">
          <div className="flex items-center gap-2 text-cyan-400">
            <Keyboard className="w-5 h-5" />
            <h3 className="text-base font-bold text-slate-100">Keyboard Shortcuts & Accessibility</h3>
          </div>
          <button onClick={onClose} className="text-slate-400 hover:text-slate-200">
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="space-y-2 mb-6">
          {shortcuts.map((s, i) => (
            <div key={i} className="flex justify-between items-center py-1.5 border-b border-slate-800/60 text-xs">
              <span className="text-slate-300">{s.action}</span>
              <kbd className="font-mono text-[11px] bg-slate-800 px-2 py-0.5 rounded border border-slate-700 text-cyan-300">
                {s.key}
              </kbd>
            </div>
          ))}
        </div>

        <div className="flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-medium rounded-lg"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};
