import Link from 'next/link';
import { Layers, Search, Github, Terminal, BookOpen, Cpu, Shield } from 'lucide-react';

export function Header() {
  return (
    <header className="sticky top-0 z-50 w-full glass-header border-b border-zinc-800">
      <div className="mx-auto flex h-16 max-w-7xl items-center justify-between px-6">
        <div className="flex items-center gap-8">
          <Link href="/" className="flex items-center gap-3 group">
            <div className="flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-indigo-500 via-purple-500 to-blue-600 text-white shadow-lg shadow-indigo-500/20 group-hover:scale-105 transition-transform">
              <Layers className="h-5 w-5" />
            </div>
            <div className="flex flex-col">
              <span className="text-base font-bold tracking-tight text-white flex items-center gap-2">
                PlazaVM <span className="rounded bg-indigo-500/10 px-2 py-0.5 text-[10px] font-semibold text-indigo-400 border border-indigo-500/20">v1.0</span>
              </span>
              <span className="text-[10px] text-zinc-400 font-medium -mt-0.5">Workspace Engine</span>
            </div>
          </Link>

          <nav className="hidden md:flex items-center gap-1 text-sm font-medium">
            <Link href="/docs/getting-started" className="flex items-center gap-2 rounded-lg px-3 py-2 text-zinc-300 hover:text-white hover:bg-zinc-850 transition-all">
              <BookOpen className="h-4 w-4 text-indigo-400" />
              <span>Docs</span>
            </Link>
            <Link href="/docs/concepts" className="flex items-center gap-2 rounded-lg px-3 py-2 text-zinc-300 hover:text-white hover:bg-zinc-850 transition-all">
              <Cpu className="h-4 w-4 text-purple-400" />
              <span>Concepts</span>
            </Link>
            <Link href="/docs/architecture" className="flex items-center gap-2 rounded-lg px-3 py-2 text-zinc-300 hover:text-white hover:bg-zinc-850 transition-all">
              <Layers className="h-4 w-4 text-pink-400" />
              <span>Architecture</span>
            </Link>
            <Link href="/docs/crates" className="flex items-center gap-2 rounded-lg px-3 py-2 text-zinc-300 hover:text-white hover:bg-zinc-850 transition-all">
              <Shield className="h-4 w-4 text-emerald-400" />
              <span>Core Crates</span>
            </Link>
            <Link href="/docs/cli" className="flex items-center gap-2 rounded-lg px-3 py-2 text-zinc-300 hover:text-white hover:bg-zinc-850 transition-all">
              <Terminal className="h-4 w-4 text-amber-400" />
              <span>CLI</span>
            </Link>
          </nav>
        </div>

        <div className="flex items-center gap-3">
          <div className="relative hidden sm:flex items-center">
            <Search className="absolute left-3 h-4 w-4 text-zinc-400" />
            <input
              type="text"
              placeholder="Search docs... (Ctrl+K)"
              className="w-64 rounded-xl bg-zinc-900/90 pl-9 pr-4 py-1.5 text-xs text-zinc-200 border border-zinc-800 focus:border-indigo-500 focus:outline-none transition-all placeholder:text-zinc-500"
              readOnly
            />
          </div>

          <a
            href="https://github.com/DoguparthiAakash/plazavm"
            target="_blank"
            rel="noreferrer"
            className="flex items-center gap-2 rounded-xl bg-zinc-900 px-3.5 py-2 text-xs font-semibold text-zinc-200 border border-zinc-800 hover:border-zinc-700 hover:bg-zinc-800 hover:text-white transition-all shadow-sm"
          >
            <Github className="h-4 w-4 text-white" />
            <span className="hidden sm:inline">GitHub</span>
          </a>
        </div>
      </div>
    </header>
  );
}
