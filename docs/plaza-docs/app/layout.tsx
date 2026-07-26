import './globals.css';
import { Header } from '@/components/Header';
import { Footer } from '@/components/Footer';

export const metadata = {
  title: 'PlazaVM Foundation — Intelligent Workspace Computing Engine',
  description: 'Official engineering knowledge platform and documentation portal for PlazaVM Foundation Architecture v1.0.',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className="dark">
      <body className="min-h-screen bg-zinc-950 text-zinc-100 antialiased selection:bg-indigo-500 selection:text-white flex flex-col justify-between">
        <div>
          <Header />
          {children}
        </div>
        <Footer />
      </body>
    </html>
  );
}
