import Link from 'next/link';
import { Sparkles, Cpu, Shield, Zap, ArrowRight, Terminal, Layers } from 'lucide-react';

export default function Home() {
  return (
    <main className="mx-auto max-w-7xl px-6 py-16">
      {/* Hero Section */}
      <div className="flex flex-col items-center text-center">
        <div className="inline-flex items-center gap-2 rounded-full border border-indigo-500/30 bg-indigo-500/10 px-4 py-1.5 text-xs font-semibold text-indigo-400 backdrop-blur-md shadow-sm">
          <Sparkles className="h-3.5 w-3.5 text-indigo-400" />
          <span>Architecture v1.0 Frozen & Enterprise Certified</span>
        </div>

        <h1 className="mt-8 text-5xl font-extrabold tracking-tight text-white sm:text-6xl max-w-4xl leading-tight">
          The Intelligent <span className="bg-gradient-to-r from-indigo-400 via-purple-400 to-pink-400 bg-clip-text text-transparent">Workspace Computing Engine</span>
        </h1>

        <p className="mt-6 max-w-2xl text-base sm:text-lg text-zinc-400 leading-relaxed">
          PlazaVM elevates the <strong className="text-white font-semibold">Workspace</strong> into the primary execution unit for software development—isolated, reproducible, and portable above the Linux kernel.
        </p>

        <div className="mt-8 flex flex-wrap justify-center gap-4">
          <Link
            href="/docs/getting-started"
            className="flex items-center gap-2 rounded-xl bg-gradient-to-r from-indigo-600 to-purple-600 px-6 py-3 text-sm font-semibold text-white shadow-lg shadow-indigo-500/25 hover:from-indigo-500 hover:to-purple-500 transition-all"
          >
            <span>Get Started</span>
            <ArrowRight className="h-4 w-4" />
          </Link>
          <Link
            href="/docs/concepts/workspace-first"
            className="flex items-center gap-2 rounded-xl border border-zinc-800 bg-zinc-900/80 px-6 py-3 text-sm font-semibold text-zinc-200 hover:border-zinc-700 hover:bg-zinc-800 transition-all"
          >
            <Cpu className="h-4 w-4 text-purple-400" />
            <span>Read Concepts</span>
          </Link>
        </div>
      </div>

      {/* Terminal Code Preview */}
      <div className="mt-16 overflow-hidden rounded-2xl border border-zinc-800 bg-zinc-900/90 shadow-2xl">
        <div className="flex items-center justify-between border-b border-zinc-800 px-4 py-3 bg-zinc-950">
          <div className="flex items-center gap-2">
            <Terminal className="h-4 w-4 text-indigo-400" />
            <span className="text-xs font-mono font-medium text-zinc-300">plaza-cli terminal</span>
          </div>
          <div className="flex items-center gap-1.5">
            <span className="h-2.5 w-2.5 rounded-full bg-zinc-700" />
            <span className="h-2.5 w-2.5 rounded-full bg-zinc-700" />
            <span className="h-2.5 w-2.5 rounded-full bg-zinc-700" />
          </div>
        </div>
        <pre className="p-6 text-xs sm:text-sm font-mono text-indigo-300 overflow-x-auto leading-relaxed">
          <code>
{`# Create a standard workspace entry with Docker runtime
$ plaza workspace create --name my-python-app --image python:3.12-slim
Created workspace: my-python-app [14b1e830-94fa-4b12-bab2-d6e647e07326]

# Inspect workspace details and PURI header
$ plaza workspace inspect my-python-app
Workspace Details:
  ID             : 14b1e830-94fa-4b12-bab2-d6e647e07326
  Name           : my-python-app
  PURI           : plaza://workspace/14b1e830-94fa-4b12-bab2-d6e647e07326
  State          : Stopped
  Runtime Backend: Auto (Docker)

# Execute command inside isolated sandbox
$ plaza workspace exec my-python-app python main.py
Executing inside workspace 'my-python-app': python main.py
Exec process completed (exit status 0)`}
          </code>
        </pre>
      </div>

      {/* Feature Cards Grid */}
      <div className="mt-24 grid grid-cols-1 md:grid-cols-3 gap-8">
        <div className="rounded-2xl border border-zinc-800 bg-zinc-900/60 p-6 backdrop-blur-md hover:border-indigo-500/40 transition-all">
          <div className="h-10 w-10 flex items-center justify-center rounded-xl bg-indigo-500/10 text-indigo-400 mb-4 border border-indigo-500/20">
            <Zap className="h-5 w-5" />
          </div>
          <h3 className="text-lg font-bold text-white flex items-center gap-2">
            Workspace-First Paradigm
          </h3>
          <p className="mt-2 text-sm text-zinc-400 leading-relaxed">
            The workspace is the primary unit of computing. Host OS only provides hardware resources while Plaza Foundation owns runtime and isolation.
          </p>
        </div>

        <div className="rounded-2xl border border-zinc-800 bg-zinc-900/60 p-6 backdrop-blur-md hover:border-purple-500/40 transition-all">
          <div className="h-10 w-10 flex items-center justify-center rounded-xl bg-purple-500/10 text-purple-400 mb-4 border border-purple-500/20">
            <Shield className="h-5 w-5" />
          </div>
          <h3 className="text-lg font-bold text-white flex items-center gap-2">
            Kernel Adaptation Layer
          </h3>
          <p className="mt-2 text-sm text-zinc-400 leading-relaxed">
            Direct integration with Linux kernel security primitives: cgroups v2, unprivileged user namespaces, seccomp, and Landlock sandboxing.
          </p>
        </div>

        <div className="rounded-2xl border border-zinc-800 bg-zinc-900/60 p-6 backdrop-blur-md hover:border-pink-500/40 transition-all">
          <div className="h-10 w-10 flex items-center justify-center rounded-xl bg-pink-500/10 text-pink-400 mb-4 border border-pink-500/20">
            <Layers className="h-5 w-5" />
          </div>
          <h3 className="text-lg font-bold text-white flex items-center gap-2">
            Plug-and-Play Runtimes
          </h3>
          <p className="mt-2 text-sm text-zinc-400 leading-relaxed">
            Interchangeable execution backends for Docker, Podman, QEMU, VirtualBox, and Hyper-V with zero application code changes.
          </p>
        </div>
      </div>
    </main>
  );
}
