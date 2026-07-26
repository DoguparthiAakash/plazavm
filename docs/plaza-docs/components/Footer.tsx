import Link from 'next/link';
import { Layers, Github, Shield, Terminal, CheckCircle2 } from 'lucide-react';

export function Footer() {
  return (
    <footer className="w-full border-t border-zinc-800 bg-zinc-950 text-zinc-400 text-xs py-12">
      <div className="mx-auto max-w-7xl px-6 grid grid-cols-1 md:grid-cols-4 gap-8">
        <div className="space-y-3">
          <div className="flex items-center gap-2 text-white font-bold text-base">
            <div className="flex h-7 w-7 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600 text-white">
              <Layers className="h-4 w-4" />
            </div>
            <span>PlazaVM Foundation</span>
          </div>
          <p className="text-zinc-400 text-xs leading-relaxed">
            The Intelligent Workspace Computing Engine. Isolated, reproducible, and portable execution above the Linux kernel.
          </p>
          <div className="flex items-center gap-2 pt-2">
            <span className="flex h-2 w-2 rounded-full bg-emerald-500 animate-pulse" />
            <span className="text-[11px] text-zinc-300 font-mono">Architecture v1.0 Certified</span>
          </div>
        </div>

        <div>
          <h4 className="font-bold text-zinc-200 uppercase tracking-wider text-[11px] mb-3">Documentation</h4>
          <ul className="space-y-2">
            <li><Link href="/docs/getting-started" className="hover:text-indigo-400 transition-colors">Getting Started</Link></li>
            <li><Link href="/docs/concepts/workspace-first" className="hover:text-indigo-400 transition-colors">Workspace-First Paradigm</Link></li>
            <li><Link href="/docs/architecture" className="hover:text-indigo-400 transition-colors">Layered System Architecture</Link></li>
            <li><Link href="/docs/crates" className="hover:text-indigo-400 transition-colors">23 Core Workspace Crates</Link></li>
          </ul>
        </div>

        <div>
          <h4 className="font-bold text-zinc-200 uppercase tracking-wider text-[11px] mb-3">Specifications</h4>
          <ul className="space-y-2">
            <li><Link href="/docs/specifications" className="hover:text-indigo-400 transition-colors">PS-0001 Workspace Spec</Link></li>
            <li><Link href="/docs/specifications" className="hover:text-indigo-400 transition-colors">PST-0001 Naming Standard</Link></li>
            <li><Link href="/docs/cli" className="hover:text-indigo-400 transition-colors">CLI Subcommand Guide</Link></li>
            <li><Link href="/docs/getting-started/quickstart" className="hover:text-indigo-400 transition-colors">2-Minute Quick Start</Link></li>
          </ul>
        </div>

        <div>
          <h4 className="font-bold text-zinc-200 uppercase tracking-wider text-[11px] mb-3">Community & License</h4>
          <ul className="space-y-2">
            <li>
              <a href="https://github.com/DoguparthiAakash/plazavm" target="_blank" rel="noreferrer" className="flex items-center gap-1.5 hover:text-white transition-colors">
                <Github className="h-3.5 w-3.5" />
                <span>GitHub Repository</span>
              </a>
            </li>
            <li className="text-zinc-500">MIT & Apache-2.0 Dual Licensed</li>
            <li className="text-zinc-500">Built with Next.js 14 & Tailwind CSS</li>
          </ul>
        </div>
      </div>

      <div className="mx-auto max-w-7xl px-6 mt-8 pt-6 border-t border-zinc-900 flex flex-col sm:flex-row items-center justify-between text-zinc-500 text-[11px]">
        <span>© 2026 PlazaVM Foundation. All rights reserved.</span>
        <span>Built for Enterprise Software Engineering</span>
      </div>
    </footer>
  );
}
