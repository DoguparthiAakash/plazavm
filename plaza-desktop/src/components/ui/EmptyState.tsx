import React from "react";
import { Plus, Download, Sparkles, FolderPlus } from "lucide-react";

interface EmptyStateProps {
  title: string;
  description: string;
  actionLabel?: string;
  onAction?: () => void;
  secondaryLabel?: string;
  onSecondaryAction?: () => void;
  iconType?: "workspace" | "image" | "snapshot" | "package";
}

export const EmptyState: React.FC<EmptyStateProps> = ({
  title,
  description,
  actionLabel,
  onAction,
  secondaryLabel,
  onSecondaryAction,
  iconType = "workspace",
}) => {
  return (
    <div className="flex flex-col items-center justify-center p-12 text-center border border-dashed border-slate-800/80 rounded-3xl bg-slate-950/40 backdrop-blur-md select-none my-6">
      {/* SVG Illustration Container */}
      <div className="relative mb-6">
        <div className="w-24 h-24 rounded-3xl bg-gradient-to-tr from-cyan-500/10 via-teal-500/10 to-blue-500/10 border border-cyan-500/20 flex items-center justify-center shadow-xl shadow-cyan-500/5">
          {iconType === "workspace" && (
            <svg className="w-12 h-12 text-cyan-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
              <rect x="3" y="3" width="18" height="18" rx="4" />
              <path d="M7 8l4 4-4 4" />
              <line x1="13" y1="16" x2="17" y2="16" />
            </svg>
          )}

          {iconType === "image" && (
            <svg className="w-12 h-12 text-teal-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
              <polygon points="12 2 2 7 12 12 22 7 12 2" />
              <polyline points="2 17 12 22 22 17" />
              <polyline points="2 12 12 17 22 12" />
            </svg>
          )}

          {iconType === "snapshot" && (
            <svg className="w-12 h-12 text-amber-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
              <circle cx="12" cy="12" r="9" />
              <path d="M12 7v5l3 3" />
            </svg>
          )}

          {iconType === "package" && (
            <svg className="w-12 h-12 text-emerald-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
              <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
              <polyline points="3.27 6.96 12 12.01 20.73 6.96" />
              <line x1="12" y1="22.08" x2="12" y2="12" />
            </svg>
          )}
        </div>
        <div className="absolute -top-1 -right-1 w-6 h-6 rounded-full bg-cyan-500/20 border border-cyan-400/40 flex items-center justify-center">
          <Sparkles className="w-3 h-3 text-cyan-300" />
        </div>
      </div>

      <h3 className="text-lg font-extrabold text-slate-100 tracking-tight">{title}</h3>
      <p className="text-xs text-slate-400 max-w-md mt-1.5 leading-relaxed">{description}</p>

      <div className="flex items-center gap-3 mt-6">
        {actionLabel && onAction && (
          <button
            onClick={onAction}
            className="flex items-center gap-2 px-4 py-2.5 bg-gradient-to-r from-cyan-500 to-teal-500 hover:from-cyan-400 hover:to-teal-400 text-slate-950 font-bold rounded-xl text-xs shadow-lg shadow-cyan-500/20 transition active:scale-95"
          >
            <Plus className="w-4 h-4 stroke-[3]" />
            {actionLabel}
          </button>
        )}

        {secondaryLabel && onSecondaryAction && (
          <button
            onClick={onSecondaryAction}
            className="flex items-center gap-2 px-4 py-2.5 bg-slate-900/80 hover:bg-slate-800 text-slate-200 font-semibold rounded-xl text-xs border border-slate-700/60 transition active:scale-95"
          >
            <Download className="w-4 h-4" />
            {secondaryLabel}
          </button>
        )}
      </div>
    </div>
  );
};
