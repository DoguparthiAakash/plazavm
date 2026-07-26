import { ProgramizCodeBox } from '@/components/ProgramizCodeBox';
import { ProgramizTakeaway } from '@/components/ProgramizTakeaway';
import { PageNav } from '@/components/PageNav';

export default function GettingStartedPage() {
  return (
    <article className="prose prose-invert max-w-none">
      {/* Breadcrumb Trail */}
      <div className="flex items-center gap-2 text-xs text-zinc-400 mb-4">
        <span>Docs</span>
        <span>/</span>
        <span>Getting Started</span>
        <span>/</span>
        <span className="text-indigo-400 font-semibold">Introduction</span>
      </div>

      <div className="border-b border-zinc-800 pb-6 mb-8">
        <h1 className="text-4xl font-extrabold text-white tracking-tight">Introduction to PlazaVM</h1>
        <p className="text-zinc-400 mt-3 text-lg leading-relaxed">
          PlazaVM Foundation is an **Intelligent Workspace Computing Engine** designed to turn software workspaces into primary, self-contained units of computing.
        </p>
      </div>

      {/* Programiz Signature Key Takeaways Box */}
      <ProgramizTakeaway type="takeaway" title="Key Takeaways">
        <ul className="list-disc pl-4 space-y-1">
          <li>PlazaVM operates above the Linux kernel, using unprivileged user namespaces and cgroups v2.</li>
          <li>Every workspace is identified by a unique Plaza URI: <code className="text-indigo-300">plaza://workspace/&lt;UUID&gt;</code>.</li>
          <li>Zero pollution on the host operating system—everything stays in <code className="text-indigo-300">.plaza/</code> root.</li>
        </ul>
      </ProgramizTakeaway>

      <section className="space-y-6">
        <h2 className="text-2xl font-bold text-white mt-10">Creating Your First Workspace</h2>
        <p className="text-zinc-300">
          Run the <code className="text-indigo-300">plaza workspace create</code> command to initialize a new isolated execution sandbox:
        </p>

        {/* Programiz Code + Output Component */}
        <ProgramizCodeBox
          title="Terminal — Create Workspace"
          code={`# Create an isolated workspace named 'my-app'
cargo run -p plaza-cli -- workspace create my-app

# Inspect workspace configuration and status
cargo run -p plaza-cli -- workspace inspect my-app`}
          output={`Created workspace: my-app [63450edb-f1b2-4236-82e0-3c9a3b01304f]

Workspace Details:
  ID             : 63450edb-f1b2-4236-82e0-3c9a3b01304f
  Name           : my-app
  PURI           : plaza://workspace/63450edb-f1b2-4236-82e0-3c9a3b01304f
  State          : Stopped
  Runtime Image  : ubuntu:24.04`}
        />

        <ProgramizTakeaway type="note" title="Note on Isolation">
          Workspaces require 0 MB RAM when stopped. Resources are allocated dynamically when you start or execute processes inside the workspace.
        </ProgramizTakeaway>
      </section>

      {/* Programiz Previous / Next Tutorial Navigation */}
      <PageNav
        next={{ title: 'Quick Start Guide', href: '/docs/getting-started/quickstart' }}
      />
    </article>
  );
}
