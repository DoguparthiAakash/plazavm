import React, { useState, useEffect } from "react";
import { Layers, ShieldCheck, FileCode, HardDrive, Download, Search } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

export const ImagesView: React.FC = () => {
  const [proImages, setProImages] = useState<any[]>([]);
  const [purImages, setPurImages] = useState<any[]>([]);
  const [search, setSearch] = useState("");

  useEffect(() => {
    invoke<any[]>("get_pro_images").then(setProImages).catch(() => {});
    invoke<any[]>("get_pur_images").then(setPurImages).catch(() => {});
  }, []);

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6 select-none">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-extrabold text-slate-100 tracking-tight flex items-center gap-3">
            <Layers className="w-6 h-6 text-cyan-400" /> Runtime Image Explorer
          </h2>
          <p className="text-xs text-slate-400 mt-1">
            Immutable, content-addressed, layer-based, Ed25519 signed images (<span className="font-mono text-cyan-300">pro://</span> & <span className="font-mono text-teal-300">pri://</span>)
          </p>
        </div>

        <div className="relative">
          <Search className="w-4 h-4 text-slate-400 absolute left-3 top-2.5" />
          <input
            type="text"
            placeholder="Search images..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="pl-9 pr-4 py-2 bg-slate-900/80 border border-slate-800 rounded-xl text-xs text-slate-200 focus:outline-none focus:border-cyan-500/50 w-64"
          />
        </div>
      </div>

      {/* PRO Images Section */}
      <div className="space-y-4">
        <h3 className="text-sm font-bold text-slate-300 tracking-wide flex items-center gap-2">
          <HardDrive className="w-4 h-4 text-cyan-400" /> Plaza Runtime OS (PRO) Images (<span className="font-mono text-xs text-cyan-400">pro://</span>)
        </h3>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {proImages.map((img, i) => (
            <div
              key={i}
              className="bg-slate-900/70 border border-slate-800/80 rounded-2xl p-5 hover:border-cyan-500/40 transition shadow-lg space-y-4"
            >
              <div className="flex items-start justify-between">
                <div>
                  <h4 className="font-bold text-sm text-slate-100">{img.name}</h4>
                  <div className="font-mono text-[11px] text-cyan-400">{img.uri}</div>
                </div>
                <span className="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 flex items-center gap-1">
                  <ShieldCheck className="w-3 h-3" /> {img.signature}
                </span>
              </div>

              <div className="space-y-1 text-xs text-slate-400 font-mono text-[11px]">
                <div className="truncate">Digest: <span className="text-slate-300">{img.digest}</span></div>
                <div>Size: <span className="text-slate-300">{img.size_mb} MB</span></div>
                <div className="flex items-center gap-1">
                  <FileCode className="w-3 h-3 text-slate-400" /> SBOM Packages: <span className="text-slate-300">{img.sbom_packages}</span>
                </div>
              </div>

              <div className="pt-2 border-t border-slate-800/60 flex items-center justify-between text-xs">
                <button className="px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded-lg text-xs font-medium transition flex items-center gap-1">
                  <Download className="w-3.5 h-3.5" /> Inspect Layers
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* PUR Images Section */}
      <div className="space-y-4 pt-4">
        <h3 className="text-sm font-bold text-slate-300 tracking-wide flex items-center gap-2">
          <Layers className="w-4 h-4 text-teal-400" /> Plaza Utility Runtime (PUR) Images (<span className="font-mono text-xs text-teal-400">pri://</span>)
        </h3>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {purImages.map((img, i) => (
            <div
              key={i}
              className="bg-slate-900/70 border border-slate-800/80 rounded-2xl p-5 hover:border-teal-500/40 transition shadow-lg space-y-4"
            >
              <div className="flex items-start justify-between">
                <div>
                  <h4 className="font-bold text-sm text-slate-100">{img.name}</h4>
                  <div className="font-mono text-[11px] text-teal-400">{img.uri}</div>
                </div>
                <span className="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-teal-500/10 border border-teal-500/30 text-teal-400 flex items-center gap-1">
                  <ShieldCheck className="w-3 h-3" /> {img.signature}
                </span>
              </div>

              <div className="space-y-1 text-xs text-slate-400 font-mono text-[11px]">
                <div className="truncate">Digest: <span className="text-slate-300">{img.digest}</span></div>
                <div>Layer Size: <span className="text-slate-300">{img.size_mb} MB</span></div>
                <div>SPDX Packages: <span className="text-slate-300">{img.sbom_packages}</span></div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
