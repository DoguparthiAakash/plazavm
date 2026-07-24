'use client';

import { useState } from 'react';
import { Check, Copy } from 'lucide-react';

interface ProgramizCodeBoxProps {
  title?: string;
  code: string;
  output?: string;
  language?: string;
}

export function ProgramizCodeBox({ title = 'Example', code, output, language = 'bash' }: ProgramizCodeBoxProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="my-6 overflow-hidden rounded-xl border border-zinc-800 bg-zinc-900 shadow-xl">
      {/* Code Header Bar */}
      <div className="flex items-center justify-between border-b border-zinc-800 bg-zinc-950 px-4 py-2.5">
        <div className="flex items-center gap-2">
          <span className="h-3 w-3 rounded-full bg-indigo-500" />
          <span className="font-mono text-xs font-semibold text-zinc-300">{title}</span>
          <span className="rounded bg-zinc-800 px-2 py-0.5 font-mono text-[10px] text-zinc-400 uppercase">{language}</span>
        </div>
        <button
          onClick={handleCopy}
          className="flex items-center gap-1.5 rounded-lg bg-zinc-800 px-3 py-1 text-xs font-medium text-zinc-300 hover:bg-zinc-700 hover:text-white transition-all"
        >
          {copied ? (
            <>
              <Check className="h-3.5 w-3.5 text-emerald-400" />
              <span className="text-emerald-400">Copied!</span>
            </>
          ) : (
            <>
              <Copy className="h-3.5 w-3.5" />
              <span>Copy</span>
            </>
          )}
        </button>
      </div>

      {/* Code Block Area */}
      <pre className="overflow-x-auto p-4 font-mono text-xs sm:text-sm text-indigo-300 leading-relaxed bg-zinc-900/90">
        <code>{code}</code>
      </pre>

      {/* Programiz Output Panel (if provided) */}
      {output && (
        <div className="border-t border-zinc-800 bg-zinc-950/80 p-4">
          <span className="text-xs font-bold text-emerald-400 uppercase tracking-wider block mb-1.5">
            Output
          </span>
          <pre className="font-mono text-xs text-zinc-300 overflow-x-auto leading-relaxed">
            <code>{output}</code>
          </pre>
        </div>
      )}
    </div>
  );
}
