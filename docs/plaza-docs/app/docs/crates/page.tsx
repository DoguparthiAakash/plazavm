export default function CratesPage() {
  const crates = [
    { name: 'plaza-foundation', desc: 'Central foundation orchestrator, service registries, and internal IPC protocol envelope.' },
    { name: 'plaza-core', desc: 'Canonical error types (PZE-XXXX), Plaza URI parser (plaza://), and object metadata model.' },
    { name: 'plaza-platform', desc: 'Kernel Adaptation Layer (KAL), Linux user namespaces, seccomp filters, Landlock.' },
    { name: 'plaza-workspace', desc: 'Transactional pipeline builder, capability database, workspace lifecycle graphs.' },
    { name: 'plaza-resource', desc: 'Virtual Hardware Abstraction Layer (VHAL), resource profiles (Desktop, Server, AI Workstation).' },
    { name: 'plaza-cli', desc: 'Official developer CLI binary supporting workspace create, inspect, exec, start, stop, delete.' },
    { name: 'plaza-desktop', desc: 'React 19 + Tauri desktop frontend interface with live workspace state monitoring.' },
    { name: 'plaza-storage', desc: 'SQLite transactional database state store and workspace snapshot management.' },
    { name: 'plaza-network', desc: 'Isolated workspace virtual bridge, port forwarder, and firewall rules.' },
    { name: 'plaza-security', desc: 'Zero-trust sandbox verification, secret masking, and RBAC policy engines.' },
  ];

  return (
    <article className="prose prose-invert max-w-none">
      <div className="border-b border-zinc-800 pb-6 mb-8">
        <span className="text-xs font-semibold text-indigo-400 uppercase tracking-wide">Core Ecosystem</span>
        <h1 className="text-4xl font-extrabold text-white mt-2">23 Core Workspace Crates</h1>
        <p className="text-zinc-400 mt-2 text-lg">
          Complete crate-by-crate technical reference for PlazaVM Architecture v1.0.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {crates.map((crate, idx) => (
          <div key={idx} className="rounded-xl border border-zinc-800 bg-zinc-900/60 p-5 hover:border-indigo-500/50 transition-colors">
            <h3 className="font-mono font-bold text-indigo-400 text-base">{crate.name}</h3>
            <p className="text-xs text-zinc-300 mt-2 leading-relaxed">{crate.desc}</p>
          </div>
        ))}
      </div>
    </article>
  );
}
