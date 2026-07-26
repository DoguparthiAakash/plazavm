import React, { useState } from "react";
import { Search, X, Terminal, Layers, Package, Plug, Settings, FileText, Globe, Command } from "lucide-react";

interface GlobalSearchModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSelectCategory?: (tab: string) => void;
}

export const GlobalSearchModal: React.FC<GlobalSearchModalProps> = ({
  isOpen,
  onClose,
  onSelectCategory,
}) => {
  const [query, setQuery] = useState("");
  const [activeFilter, setActiveFilter] = useState("all");

  if (!isOpen) return null;

  const categories = [
    { id: "all", label: "All Items" },
    { id: "workspaces", label: "Workspaces", icon: Terminal },
    { id: "images", label: "Images", icon: Layers },
    { id: "packages", label: "Packages", icon: Package },
    { id: "registry", label: "Registry", icon: Globe },
    { id: "plugins", label: "Plugins", icon: Plug },
    { id: "settings", label: "Settings", icon: Settings },
  ];

  const searchItems = [
    { id: "ws-1", name: "ubuntu-cuda-dev", type: "workspaces", detail: "NVIDIA CUDA 12.5 & PyTorch 2.3", icon: Terminal },
    { id: "ws-2", name: "rust-microservices", type: "workspaces", detail: "Rust 1.78 & Cargo workspace", icon: Terminal },
    { id: "img-1", name: "plaza/ubuntu-24.04-cuda", type: "images", detail: "4.2 GB - CUDA Runtime base", icon: Layers },
    { id: "img-2", name: "plaza/alpine-edge-minimal", type: "images", detail: "84 MB - Ultra lightweight node", icon: Layers },
    { id: "pkg-1", name: "pytorch-cuda12", type: "packages", detail: "v2.3.0 - Deep learning framework", icon: Package },
    { id: "pkg-2", name: "tokio-async", type: "packages", detail: "v1.38.0 - Rust async runtime", icon: Package },
    { id: "reg-1", name: "ghcr.io/nvidia/cuda:12.5-devel", type: "registry", detail: "Official NVIDIA CUDA Devel Image", icon: Globe },
    { id: "plg-1", name: "Virtio-GPU Accel Plugin", type: "plugins", detail: "v1.4.0 - Hardware 3D acceleration", icon: Plug },
    { id: "cfg-1", name: "WSL2 Subsystem Integration", type: "settings", detail: "vGPU & DirectSockets config", icon: Settings },
  ];

  const filtered = searchItems.filter((item) => {
    const matchesQuery =
      item.name.toLowerCase().includes(query.toLowerCase()) ||
      item.detail.toLowerCase().includes(query.toLowerCase());
    const matchesFilter = activeFilter === "all" || item.type === activeFilter;
    return matchesQuery && matchesFilter;
  });

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-20 p-4 bg-slate-950/80 backdrop-blur-md animate-in fade-in duration-200">
      <div className="w-full max-w-2xl bg-slate-950 border border-cyan-500/40 rounded-3xl shadow-2xl overflow-hidden flex flex-col select-none">
        {/* Search Header Input */}
        <div className="flex items-center gap-3 px-5 py-4 border-b border-slate-800 bg-slate-900/60">
          <Search className="w-5 h-5 text-cyan-400 shrink-0" />
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search workspaces, images, packages, logs, registry, settings..."
            className="flex-1 bg-transparent text-sm text-slate-100 placeholder-slate-500 outline-none font-medium"
            autoFocus
          />
          {query && (
            <button onClick={() => setQuery("")} className="text-slate-400 hover:text-white">
              <X className="w-4 h-4" />
            </button>
          )}
          <kbd className="hidden sm:inline-block font-mono text-[10px] bg-slate-800 px-2 py-1 rounded-lg text-slate-400 border border-slate-700">
            ESC to close
          </kbd>
        </div>

        {/* Filter Pills */}
        <div className="flex items-center gap-1.5 px-4 py-2 bg-slate-900/40 border-b border-slate-800/80 overflow-x-auto">
          {categories.map((cat) => (
            <button
              key={cat.id}
              onClick={() => setActiveFilter(cat.id)}
              className={`px-3 py-1.5 rounded-xl text-xs font-semibold whitespace-nowrap transition ${
                activeFilter === cat.id
                  ? "bg-cyan-500/20 text-cyan-400 border border-cyan-500/40"
                  : "text-slate-400 hover:text-slate-200 hover:bg-slate-900"
              }`}
            >
              {cat.label}
            </button>
          ))}
        </div>

        {/* Results List */}
        <div className="max-h-96 overflow-y-auto p-3 space-y-1">
          {filtered.length === 0 ? (
            <div className="p-8 text-center text-slate-500 text-xs font-medium">
              No matching results found for "{query}".
            </div>
          ) : (
            filtered.map((item) => {
              const Icon = item.icon;
              return (
                <div
                  key={item.id}
                  onClick={() => {
                    if (onSelectCategory) onSelectCategory(item.type);
                    onClose();
                  }}
                  className="flex items-center justify-between p-3 rounded-2xl hover:bg-slate-900/80 cursor-pointer group transition border border-transparent hover:border-cyan-500/30"
                >
                  <div className="flex items-center gap-3">
                    <div className="w-8 h-8 rounded-xl bg-slate-900 border border-slate-800 group-hover:border-cyan-500/40 flex items-center justify-center text-slate-400 group-hover:text-cyan-400 transition">
                      <Icon className="w-4 h-4" />
                    </div>
                    <div>
                      <h4 className="text-xs font-bold text-slate-200 group-hover:text-cyan-300 transition">
                        {item.name}
                      </h4>
                      <p className="text-[11px] text-slate-400">{item.detail}</p>
                    </div>
                  </div>
                  <span className="text-[10px] font-mono uppercase tracking-wider text-slate-500 bg-slate-900 px-2 py-0.5 rounded-md border border-slate-800">
                    {item.type}
                  </span>
                </div>
              );
            })
          )}
        </div>
      </div>
    </div>
  );
};
