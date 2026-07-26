export default function PuriPage() {
  return (
    <article className="prose prose-invert max-w-none">
      <h1 className="text-3xl font-bold text-white">Plaza URI (PURI) Specification</h1>
      <p className="text-zinc-400 mt-2">PURI provides global, unambiguous resource naming across PlazaVM Foundation.</p>
      <pre className="rounded-xl bg-zinc-900 border border-zinc-800 p-4 text-sm font-mono text-indigo-300 mt-6">
        <code>{`plaza://<namespace>/<resource_id>[?query][#fragment]`}</code>
      </pre>
    </article>
  );
}
