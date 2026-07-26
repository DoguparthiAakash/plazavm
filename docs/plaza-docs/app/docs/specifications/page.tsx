export default function SpecificationsPage() {
  const specs = [
    { id: 'PS-0001', name: 'Workspace Manifest Standard', status: 'Approved' },
    { id: 'PS-0002', name: 'Plaza URI (PURI) Format', status: 'Approved' },
    { id: 'PS-0003', name: 'Virtual Hardware Profile (VHAL)', status: 'Approved' },
    { id: 'PS-0004', name: 'Kernel Adaptation Layer (KAL)', status: 'Approved' },
    { id: 'PS-0005', name: 'Canonical Error Code Standard', status: 'Approved' },
    { id: 'PS-0006', name: 'Runtime Plugin IPC Interface', status: 'Approved' },
  ];

  const standards = [
    { id: 'PST-0001', name: 'Rust Code & Naming Conventions' },
    { id: 'PST-0002', name: 'Structured JSON & Console Logging' },
    { id: 'PST-0003', name: 'Semantic Versioning & Compatibility' },
    { id: 'PST-0004', name: 'Zero-Trust Sandboxing Policy' },
  ];

  return (
    <article className="prose prose-invert max-w-none">
      <div className="border-b border-zinc-800 pb-6 mb-8">
        <span className="text-xs font-semibold text-indigo-400 uppercase tracking-wide">Governance</span>
        <h1 className="text-4xl font-extrabold text-white mt-2">Specifications & Standards</h1>
        <p className="text-zinc-400 mt-2 text-lg">
          Authoritative technical specifications (PS) and standards (PST) governing PlazaVM Foundation v1.0.
        </p>
      </div>

      <section className="space-y-8">
        <div>
          <h2 className="text-2xl font-bold text-white mb-4">Plaza Specifications (PS)</h2>
          <div className="divide-y divide-zinc-800 border border-zinc-800 rounded-xl overflow-hidden bg-zinc-900/40">
            {specs.map((spec, idx) => (
              <div key={idx} className="flex items-center justify-between p-4 text-sm">
                <div>
                  <span className="font-mono font-bold text-indigo-400 mr-3">{spec.id}</span>
                  <span className="text-zinc-200">{spec.name}</span>
                </div>
                <span className="rounded-full bg-emerald-500/10 px-3 py-1 text-xs font-semibold text-emerald-400 border border-emerald-500/20">
                  {spec.status}
                </span>
              </div>
            ))}
          </div>
        </div>

        <div>
          <h2 className="text-2xl font-bold text-white mb-4">Technical Standards (PST)</h2>
          <div className="divide-y divide-zinc-800 border border-zinc-800 rounded-xl overflow-hidden bg-zinc-900/40">
            {standards.map((std, idx) => (
              <div key={idx} className="flex items-center justify-between p-4 text-sm">
                <div>
                  <span className="font-mono font-bold text-purple-400 mr-3">{std.id}</span>
                  <span className="text-zinc-200">{std.name}</span>
                </div>
                <span className="text-xs text-zinc-400 font-mono">PST Standard</span>
              </div>
            ))}
          </div>
        </div>
      </section>
    </article>
  );
}
