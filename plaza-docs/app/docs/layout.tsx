import { Sidebar } from '@/components/Sidebar';
import { RightToc } from '@/components/RightToc';

export default function DocsLayout({ children }: { children: React.ReactNode }) {
  const tocItems = [
    { id: 'overview', title: 'Overview' },
    { id: 'takeaways', title: 'Key Takeaways' },
    { id: 'execution', title: 'Execution Rules' },
    { id: 'isolation', title: 'Resource Isolation' },
  ];

  return (
    <div className="flex min-h-[calc(100vh-4rem)] max-w-7xl mx-auto px-4">
      <Sidebar />
      <main className="flex-1 px-4 md:px-8 py-10 max-w-3xl overflow-hidden">
        {children}
      </main>
      <RightToc items={tocItems} />
    </div>
  );
}
