export default function CliPage() {
  const commands = [
    {
      name: 'plaza workspace create <NAME>',
      desc: 'Creates a new workspace entry with specified image, path, or template.',
      example: 'cargo run -p plaza-cli -- workspace create frontend-subworkspace --image node:20-alpine',
    },
    {
      name: 'plaza workspace list',
      desc: 'Lists all registered workspaces along with ID, Name, State, PURI, and Runtime Image.',
      example: 'cargo run -p plaza-cli -- workspace list',
    },
    {
      name: 'plaza workspace inspect <ID|NAME>',
      desc: 'Displays comprehensive health, PURI, creation date, and configuration state of a workspace by name or UUID.',
      example: 'cargo run -p plaza-cli -- workspace inspect frontend-subworkspace',
    },
    {
      name: 'plaza workspace exec <ID|NAME> <CMD...>',
      desc: 'Executes a command inside the isolated execution container of a workspace.',
      example: 'cargo run -p plaza-cli -- workspace exec frontend-subworkspace npm run dev',
    },
    {
      name: 'plaza workspace start <ID|NAME>',
      desc: 'Triggers lifecycle state transition from Stopped to Running.',
      example: 'cargo run -p plaza-cli -- workspace start frontend-subworkspace',
    },
    {
      name: 'plaza workspace stop <ID|NAME>',
      desc: 'Triggers graceful shutdown state transition from Running to Stopped.',
      example: 'cargo run -p plaza-cli -- workspace stop frontend-subworkspace',
    },
    {
      name: 'plaza validate',
      desc: 'Executes 16-stage QA validation suite across all 23 crates and prints compliance grade.',
      example: 'cargo run -p plaza-cli -- validate',
    },
  ];

  return (
    <article className="prose prose-invert max-w-none">
      <div className="border-b border-zinc-800 pb-6 mb-8">
        <span className="text-xs font-semibold text-indigo-400 uppercase tracking-wide">CLI Reference</span>
        <h1 className="text-4xl font-extrabold text-white mt-2">Plaza CLI Command Reference</h1>
        <p className="text-zinc-400 mt-2 text-lg">
          Complete reference for `plaza-cli` subcommands, options, positional arguments, and exit codes.
        </p>
      </div>

      <div className="space-y-6">
        {commands.map((cmd, idx) => (
          <div key={idx} className="rounded-xl border border-zinc-800 bg-zinc-900/60 p-6 space-y-3">
            <h3 className="font-mono font-bold text-indigo-400 text-lg">{cmd.name}</h3>
            <p className="text-sm text-zinc-300">{cmd.desc}</p>
            <div className="rounded-lg bg-zinc-950 p-3 border border-zinc-800">
              <span className="text-xs text-zinc-400 block mb-1 font-sans">Example usage:</span>
              <code className="text-xs font-mono text-emerald-400">{cmd.example}</code>
            </div>
          </div>
        ))}
      </div>
    </article>
  );
}
