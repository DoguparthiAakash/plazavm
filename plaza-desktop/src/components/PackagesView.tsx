import React, { useState } from "react";
import { Package, Search, Download, CheckCircle, RefreshCw, Layers } from "lucide-react";

export const PackagesView: React.FC = () => {
  const [search, setSearch] = useState("");

  const packageVectors = [
    { name: "APT", system: "Debian/Ubuntu", packages: "68,000+", icon: "🐧" },
    { name: "DNF", system: "Fedora/RHEL", packages: "45,000+", icon: "🎩" },
    { name: "Pacman", system: "Arch Linux", packages: "82,000+", icon: "🏹" },
    { name: "APK", system: "Alpine Linux", packages: "22,000+", icon: "🏔️" },
    { name: "Winget", system: "Windows Package Manager", packages: "15,000+", icon: "🪟" },
    { name: "Homebrew", system: "macOS / Linux", packages: "7,000+", icon: "🍺" },
    { name: "Cargo", system: "Rust Crates", packages: "160,000+", icon: "🦀" },
    { name: "Pip", system: "Python PyPI", packages: "550,000+", icon: "🐍" },
    { name: "NPM", system: "JavaScript / TypeScript", packages: "3,000,000+", icon: "⚡" },
    { name: "Go Modules", system: "Golang Ecosystem", packages: "250,000+", icon: "🐹" },
    { name: "NuGet", system: ".NET Ecosystem", packages: "400,000+", icon: "🔷" },
    { name: "Maven / Gradle", system: "Java / JVM", packages: "500,000+", icon: "☕" },
  ];

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6 select-none">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-extrabold text-slate-100 tracking-tight flex items-center gap-3">
            <Package className="w-6 h-6 text-blue-400" /> Universal Canonical Package Engine
          </h2>
          <p className="text-xs text-slate-400 mt-1">
            Declarative package translation matrix mapping canonical specs across 14 package manager vectors
          </p>
        </div>

        <div className="relative">
          <Search className="w-4 h-4 text-slate-400 absolute left-3 top-2.5" />
          <input
            type="text"
            placeholder="Search packages across 14 vectors..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="pl-9 pr-4 py-2 bg-slate-900/80 border border-slate-800 rounded-xl text-xs text-slate-200 focus:outline-none focus:border-blue-500/50 w-72"
          />
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        {packageVectors.map((vec, i) => (
          <div
            key={i}
            className="glass-card rounded-2xl p-5 space-y-3 hover:border-blue-500/40 transition"
          >
            <div className="flex items-center justify-between">
              <span className="text-2xl">{vec.icon}</span>
              <span className="px-2 py-0.5 rounded-full text-[10px] font-mono font-bold bg-blue-500/10 text-blue-400 border border-blue-500/30">
                ACTIVE VECTOR
              </span>
            </div>

            <div>
              <h3 className="font-extrabold text-sm text-slate-100">{vec.name}</h3>
              <div className="text-[11px] text-slate-400">{vec.system}</div>
            </div>

            <div className="pt-2 border-t border-slate-800/60 flex items-center justify-between text-xs font-mono">
              <span className="text-slate-500">Available:</span>
              <span className="text-slate-200 font-bold">{vec.packages}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
