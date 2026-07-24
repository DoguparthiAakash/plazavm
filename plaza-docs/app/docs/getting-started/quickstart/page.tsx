import { ProgramizCodeBox } from '@/components/ProgramizCodeBox';
import { ProgramizTakeaway } from '@/components/ProgramizTakeaway';
import { PageNav } from '@/components/PageNav';

export default function QuickStartPage() {
  return (
    <article className="prose prose-invert max-w-none">
      <div className="flex items-center gap-2 text-xs text-zinc-400 mb-4">
        <span>Docs</span>
        <span>/</span>
        <span>Getting Started</span>
        <span>/</span>
        <span className="text-indigo-400 font-semibold">Quick Start Guide</span>
      </div>

      <div className="border-b border-zinc-800 pb-6 mb-8">
        <h1 className="text-4xl font-extrabold text-white tracking-tight">Quick Start Guide</h1>
        <p className="text-zinc-400 mt-2 text-lg">
          Learn how to execute commands and manage workspaces in under 2 minutes.
        </p>
      </div>

      <ProgramizCodeBox
        title="Command Execution in Workspace"
        code={`# Execute command in workspace 'frontend-subworkspace'
cargo run -p plaza-cli -- workspace exec frontend-subworkspace npm run dev`}
        output={`Executing inside workspace 'frontend-subworkspace' [63450edb-f1b2-4236-82e0-3c9a3b01304f]: npm run dev
Exec process completed (exit status 0)`}
      />

      <ProgramizTakeaway type="takeaway" title="Workspace Execution Rules">
        You can execute commands directly by passing the workspace **Name** or **UUID**. Subcommands automatically resolve names to their corresponding canonical PURI identifier.
      </ProgramizTakeaway>

      <PageNav
        prev={{ title: 'Introduction to PlazaVM', href: '/docs/getting-started' }}
        next={{ title: 'Workspace-First Paradigm', href: '/docs/concepts/workspace-first' }}
      />
    </article>
  );
}
