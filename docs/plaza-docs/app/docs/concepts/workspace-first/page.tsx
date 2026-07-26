import { ProgramizCodeBox } from '@/components/ProgramizCodeBox';
import { ProgramizTakeaway } from '@/components/ProgramizTakeaway';
import { PageNav } from '@/components/PageNav';

export default function WorkspaceFirstPage() {
  return (
    <article className="prose prose-invert max-w-none">
      <div className="flex items-center gap-2 text-xs text-zinc-400 mb-4">
        <span>Docs</span>
        <span>/</span>
        <span>Concepts</span>
        <span>/</span>
        <span className="text-indigo-400 font-semibold">Workspace-First Paradigm</span>
      </div>

      <div className="border-b border-zinc-800 pb-6 mb-8">
        <h1 className="text-4xl font-extrabold text-white tracking-tight">Workspace-First Computing Paradigm</h1>
        <p className="text-zinc-400 mt-2 text-lg">
          Understanding the shift from container-centric execution to true Workspace-First software engineering.
        </p>
      </div>

      <ProgramizTakeaway type="takeaway" title="Why Workspace-First?">
        The Workspace is the primary unit of compute. Host OS only provides hardware resources while Plaza Foundation owns runtime, capabilities, and isolation above the Linux kernel.
      </ProgramizTakeaway>

      <section className="space-y-6 text-zinc-300">
        <h2 className="text-2xl font-bold text-white">Plaza URI Format (`plaza://`)</h2>
        <p>Every resource in PlazaVM is globally addressable via standard Plaza URIs:</p>

        <ProgramizCodeBox
          title="PURI Examples"
          code={`plaza://workspace/63450edb-f1b2-4236-82e0-3c9a3b01304f
plaza://resource/cpu?cores=4&memory=8GB
plaza://plugin/docker-runtime@1.0.0
plaza://provider/ubuntu-24.04`}
        />
      </section>

      <PageNav
        prev={{ title: 'Quick Start Guide', href: '/docs/getting-started/quickstart' }}
        next={{ title: 'Layered System Architecture', href: '/docs/architecture' }}
      />
    </article>
  );
}
