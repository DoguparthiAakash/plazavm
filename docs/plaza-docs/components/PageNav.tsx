import Link from 'next/link';
import { ArrowLeft, ArrowRight } from 'lucide-react';

interface PageNavProps {
  prev?: { title: string; href: string };
  next?: { title: string; href: string };
}

export function PageNav({ prev, next }: PageNavProps) {
  return (
    <div className="mt-12 flex flex-col sm:flex-row items-center justify-between gap-4 border-t border-zinc-800 pt-8">
      {prev ? (
        <Link
          href={prev.href}
          className="group flex flex-1 items-center gap-3 rounded-xl border border-zinc-800 bg-zinc-900/60 p-4 hover:border-indigo-500/50 hover:bg-zinc-900 transition-all w-full"
        >
          <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-zinc-800 text-zinc-400 group-hover:bg-indigo-600 group-hover:text-white transition-all shrink-0">
            <ArrowLeft className="h-4 w-4" />
          </div>
          <div>
            <span className="text-[10px] font-bold text-zinc-500 uppercase tracking-wider block">Previous Topic</span>
            <span className="text-xs sm:text-sm font-semibold text-zinc-200 group-hover:text-indigo-300 transition-colors">
              {prev.title}
            </span>
          </div>
        </Link>
      ) : <div className="flex-1" />}

      {next ? (
        <Link
          href={next.href}
          className="group flex flex-1 items-center justify-end text-right gap-3 rounded-xl border border-zinc-800 bg-zinc-900/60 p-4 hover:border-indigo-500/50 hover:bg-zinc-900 transition-all w-full"
        >
          <div>
            <span className="text-[10px] font-bold text-zinc-500 uppercase tracking-wider block">Next Topic</span>
            <span className="text-xs sm:text-sm font-semibold text-zinc-200 group-hover:text-indigo-300 transition-colors">
              {next.title}
            </span>
          </div>
          <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-zinc-800 text-zinc-400 group-hover:bg-indigo-600 group-hover:text-white transition-all shrink-0">
            <ArrowRight className="h-4 w-4" />
          </div>
        </Link>
      ) : <div className="flex-1" />}
    </div>
  );
}
