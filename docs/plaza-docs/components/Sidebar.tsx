'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { BookOpen, Terminal, Cpu, Shield, Layers, FileText } from 'lucide-react';

export function Sidebar() {
  const pathname = usePathname();

  const sections = [
    {
      title: 'Getting Started',
      icon: BookOpen,
      links: [
        { label: 'Introduction', href: '/docs/getting-started' },
        { label: 'Quick Start Guide', href: '/docs/getting-started/quickstart' },
      ],
    },
    {
      title: 'Core Concepts',
      icon: Cpu,
      links: [
        { label: 'Workspace-First Paradigm', href: '/docs/concepts/workspace-first' },
        { label: 'Plaza URI (PURI)', href: '/docs/concepts/puri' },
      ],
    },
    {
      title: 'Architecture & Engine',
      icon: Layers,
      links: [
        { label: 'Layered Architecture', href: '/docs/architecture' },
      ],
    },
    {
      title: 'Crates & SDKs',
      icon: Shield,
      links: [
        { label: '23 Core Workspace Crates', href: '/docs/crates' },
      ],
    },
    {
      title: 'Specifications & Specs',
      icon: FileText,
      links: [
        { label: 'PS-0001..15 & PST Standards', href: '/docs/specifications' },
      ],
    },
    {
      title: 'CLI Subcommands',
      icon: Terminal,
      links: [
        { label: 'plaza workspace create', href: '/docs/cli' },
      ],
    },
  ];

  return (
    <aside className="w-64 shrink-0 border-r border-zinc-800 bg-zinc-950/90 p-5 text-sm hidden lg:block sticky top-16 h-[calc(100vh-4rem)] overflow-y-auto">
      <div className="space-y-6">
        {sections.map((section, idx) => {
          const Icon = section.icon;
          return (
            <div key={idx}>
              <div className="flex items-center gap-2 mb-2 px-2 text-zinc-300">
                <Icon className="h-4 w-4 text-indigo-400" />
                <h3 className="font-bold text-xs uppercase tracking-wider text-zinc-300">
                  {section.title}
                </h3>
              </div>
              <ul className="space-y-1 border-l border-zinc-800/80 ml-4 pl-3">
                {section.links.map((link, lIdx) => {
                  const isActive = pathname === link.href;
                  return (
                    <li key={lIdx}>
                      <Link
                        href={link.href}
                        className={`block rounded-lg px-3 py-1.5 text-xs font-medium transition-all ${
                          isActive
                            ? 'bg-indigo-600/10 text-indigo-400 border-l-2 border-indigo-500 font-semibold -ml-[13px] pl-3'
                            : 'text-zinc-400 hover:text-zinc-200 hover:bg-zinc-900/50'
                        }`}
                      >
                        {link.label}
                      </Link>
                    </li>
                  );
                })}
              </ul>
            </div>
          );
        })}
      </div>
    </aside>
  );
}
