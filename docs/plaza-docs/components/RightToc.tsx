import { List } from 'lucide-react';

interface TocItem {
  id: string;
  title: string;
}

interface RightTocProps {
  items?: TocItem[];
}

export function RightToc({ items = [] }: RightTocProps) {
  if (!items || items.length === 0) return null;

  return (
    <aside className="w-56 shrink-0 hidden xl:block sticky top-20 h-[calc(100vh-5rem)] overflow-y-auto pl-6 border-l border-zinc-800/80 text-xs">
      <div className="flex items-center gap-2 mb-3 text-zinc-400 font-semibold uppercase tracking-wider text-[11px]">
        <List className="h-3.5 w-3.5 text-indigo-400" />
        <span>On This Page</span>
      </div>
      <ul className="space-y-2 text-zinc-400">
        {items.map((item, idx) => (
          <li key={idx}>
            <a
              href={`#${item.id}`}
              className="block hover:text-indigo-400 transition-colors leading-relaxed line-clamp-1"
            >
              {item.title}
            </a>
          </li>
        ))}
      </ul>
    </aside>
  );
}
