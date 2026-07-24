export default function ArchitecturePage() {
  return (
    <article className="prose prose-invert max-w-none">
      <div className="border-b border-zinc-800 pb-6 mb-8">
        <span className="text-xs font-semibold text-indigo-400 uppercase tracking-wide">Architecture</span>
        <h1 className="text-4xl font-extrabold text-white mt-2">PlazaVM Layered System Architecture</h1>
        <p className="text-zinc-400 mt-2 text-lg">
          Detailed technical breakdown of the 6-layer architecture governing PlazaVM Foundation v1.0.
        </p>
      </div>

      <section className="space-y-6 text-zinc-300">
        <h2 className="text-2xl font-bold text-white">System Layers</h2>

        <div className="space-y-4">
          <div className="rounded-xl border border-indigo-500/30 bg-indigo-500/10 p-5">
            <h3 className="text-lg font-bold text-indigo-400">1. User & Application Layer</h3>
            <p className="text-sm text-zinc-300 mt-1">
              `plaza-cli`, `plaza-desktop`, REST API, and IPC RPC gateway interfaces providing developer interaction endpoints.
            </p>
          </div>

          <div className="rounded-xl border border-purple-500/30 bg-purple-500/10 p-5">
            <h3 className="text-lg font-bold text-purple-400">2. Foundation Layer (`plaza-foundation`, `plaza-core`)</h3>
            <p className="text-sm text-zinc-300 mt-1">
              Core runtime orchestrator, canonical error mapper (`PZE-XXXX`), Plaza URI (`plaza://`) parser, and object header protocol.
            </p>
          </div>

          <div className="rounded-xl border border-pink-500/30 bg-pink-500/10 p-5">
            <h3 className="text-lg font-bold text-pink-400">3. Workspace Layer (`plaza-workspace`)</h3>
            <p className="text-sm text-zinc-300 mt-1">
              Transactional pipeline builder, capability database, workspace graphs, dependency resolution, and service managers.
            </p>
          </div>

          <div className="rounded-xl border border-cyan-500/30 bg-cyan-500/10 p-5">
            <h3 className="text-lg font-bold text-cyan-400">4. Resource Layer (`plaza-resource`)</h3>
            <p className="text-sm text-zinc-300 mt-1">
              Virtual Hardware Abstraction Layer (VHAL), profile kinds (Desktop, Server, AI Workstation), cgroups resource limiters.
            </p>
          </div>

          <div className="rounded-xl border border-emerald-500/30 bg-emerald-500/10 p-5">
            <h3 className="text-lg font-bold text-emerald-400">5. Platform Layer (`plaza-platform`)</h3>
            <p className="text-sm text-zinc-300 mt-1">
              Kernel Adaptation Layer (KAL) interface wrapping Linux unprivileged user namespaces, seccomp filters, and Landlock isolation.
            </p>
          </div>

          <div className="rounded-xl border border-amber-500/30 bg-amber-500/10 p-5">
            <h3 className="text-lg font-bold text-amber-400">6. Execution & Kernel Foundation</h3>
            <p className="text-sm text-zinc-300 mt-1">
              Direct host Linux Kernel, Windows Subsystem for Linux (WSL2), or Apple Hypervisor framework hardware virtualization.
            </p>
          </div>
        </div>
      </section>
    </article>
  );
}
