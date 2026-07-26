import React, { useState } from "react";
import { Globe, Search, Filter, Download, Star, ShieldCheck, Tag, Plus, Terminal } from "lucide-react";

interface RegistryViewProps {
  onCreateFromImage?: (imageName: string) => void;
}

export const RegistryView: React.FC<RegistryViewProps> = ({ onCreateFromImage }) => {
  const [searchQuery, setSearchQuery] = useState("");
  const [category, setCategory] = useState("all");

  const registryItems = [
    {
      id: "1",
      name: "plaza/ubuntu-24.04-cuda",
      description: "Official Ubuntu 24.04 LTS pre-configured with NVIDIA CUDA 12.5 and cuDNN 9.",
      downloads: "14.2k",
      stars: 489,
      size: "4.2 GB",
      verified: true,
      tags: ["cuda", "ubuntu", "python3.11", "ai"],
      category: "ai",
    },
    {
      id: "2",
      name: "plaza/rust-tokio-developer",
      description: "Ultra-fast Rust development environment with sccache, cargo-watch, and LLVM 18.",
      downloads: "8.9k",
      stars: 340,
      size: "850 MB",
      verified: true,
      tags: ["rust", "cargo", "tokio", "systems"],
      category: "backend",
    },
    {
      id: "3",
      name: "plaza/alpine-edge-minimal",
      description: "Minimalist 84MB Linux runtime environment for ultra low overhead microservices.",
      downloads: "28.5k",
      stars: 612,
      size: "84 MB",
      verified: true,
      tags: ["alpine", "minimal", "musl"],
      category: "backend",
    },
    {
      id: "4",
      name: "plaza/pytorch-2.3-jupyter",
      description: "Data science runtime featuring JupyterLab, PyTorch 2.3, Pandas, and TorchVision.",
      downloads: "11.4k",
      stars: 512,
      size: "5.8 GB",
      verified: true,
      tags: ["pytorch", "jupyter", "datascience"],
      category: "ai",
    },
    {
      id: "5",
      name: "plaza/fedora-workstation-vnc",
      description: "Fedora 40 Desktop environment with Wayland and web-based VNC graphical access.",
      downloads: "5.2k",
      stars: 215,
      size: "3.1 GB",
      verified: false,
      tags: ["fedora", "gui", "vnc", "desktop"],
      category: "desktop",
    },
  ];

  const filtered = registryItems.filter((item) => {
    const matchesSearch =
      item.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      item.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
      item.tags.some((t) => t.toLowerCase().includes(searchQuery.toLowerCase()));
    const matchesCat = category === "all" || item.category === category;
    return matchesSearch && matchesCat;
  });

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6 select-none">
      {/* Top Banner */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <h2 className="text-2xl font-black text-slate-100 tracking-tight flex items-center gap-2">
            <Globe className="w-6 h-6 text-cyan-400" />
            Image Registry Hub
          </h2>
          <p className="text-xs text-slate-400">
            Browse, pull, and deploy verified container and virtual machine base images.
          </p>
        </div>

        <div className="flex items-center gap-3">
          <div className="relative">
            <Search className="w-4 h-4 text-slate-400 absolute left-3 top-2.5" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search images or tags..."
              className="pl-9 pr-4 py-2 bg-slate-900/80 border border-slate-800 rounded-xl text-xs text-slate-100 placeholder-slate-500 outline-none focus:border-cyan-500/40 w-64 transition"
            />
          </div>
        </div>
      </div>

      {/* Categories */}
      <div className="flex items-center gap-2 border-b border-slate-800 pb-3">
        {["all", "ai", "backend", "desktop"].map((cat) => (
          <button
            key={cat}
            onClick={() => setCategory(cat)}
            className={`px-3 py-1.5 rounded-xl text-xs font-semibold capitalize transition ${
              category === cat
                ? "bg-cyan-500/20 text-cyan-400 border border-cyan-500/40"
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-900/60"
            }`}
          >
            {cat}
          </button>
        ))}
      </div>

      {/* Image Cards Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
        {filtered.map((item) => (
          <div
            key={item.id}
            className="p-5 rounded-2xl glass-card border border-slate-800 flex flex-col justify-between space-y-4 hover:border-cyan-500/40 transition"
          >
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-xs font-bold text-slate-100 font-mono flex items-center gap-1.5">
                  {item.name}
                  {item.verified && (
                    <span title="Verified Plaza Image">
                      <ShieldCheck className="w-3.5 h-3.5 text-cyan-400" />
                    </span>
                  )}
                </span>
                <span className="text-[10px] font-mono text-slate-400 bg-slate-900 px-2 py-0.5 rounded border border-slate-800">
                  {item.size}
                </span>
              </div>
              <p className="text-xs text-slate-400 line-clamp-2 leading-relaxed">{item.description}</p>
            </div>

            {/* Tags */}
            <div className="flex flex-wrap gap-1.5">
              {item.tags.map((tag) => (
                <span
                  key={tag}
                  className="px-2 py-0.5 rounded-md text-[10px] font-mono bg-slate-900 text-slate-400 border border-slate-800"
                >
                  #{tag}
                </span>
              ))}
            </div>

            {/* Stats & Actions */}
            <div className="pt-3 border-t border-slate-800/80 flex items-center justify-between">
              <div className="flex items-center gap-3 text-[11px] text-slate-400 font-mono">
                <span className="flex items-center gap-1">
                  <Download className="w-3.5 h-3.5 text-cyan-400" /> {item.downloads}
                </span>
                <span className="flex items-center gap-1">
                  <Star className="w-3.5 h-3.5 text-amber-400" /> {item.stars}
                </span>
              </div>

              <button
                onClick={() => onCreateFromImage && onCreateFromImage(item.name)}
                className="flex items-center gap-1.5 px-3 py-1.5 bg-cyan-500/10 hover:bg-cyan-500/20 text-cyan-400 border border-cyan-500/30 rounded-xl text-xs font-semibold transition active:scale-95"
              >
                <Plus className="w-3.5 h-3.5" /> Deploy
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
