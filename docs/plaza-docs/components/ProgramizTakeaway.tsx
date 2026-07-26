import { Lightbulb, Info, AlertTriangle } from 'lucide-react';

interface ProgramizTakeawayProps {
  type?: 'takeaway' | 'note' | 'warning';
  title?: string;
  children: React.ReactNode;
}

export function ProgramizTakeaway({ type = 'takeaway', title, children }: ProgramizTakeawayProps) {
  const configs = {
    takeaway: {
      border: 'border-indigo-500/40',
      bg: 'bg-indigo-950/20',
      text: 'text-indigo-300',
      icon: Lightbulb,
      iconColor: 'text-indigo-400',
      defaultTitle: 'Key Takeaways',
    },
    note: {
      border: 'border-cyan-500/40',
      bg: 'bg-cyan-950/20',
      text: 'text-cyan-300',
      icon: Info,
      iconColor: 'text-cyan-400',
      defaultTitle: 'Important Note',
    },
    warning: {
      border: 'border-amber-500/40',
      bg: 'bg-amber-950/20',
      text: 'text-amber-300',
      icon: AlertTriangle,
      iconColor: 'text-amber-400',
      defaultTitle: 'Warning',
    },
  };

  const config = configs[type];
  const Icon = config.icon;

  return (
    <div className={`my-6 rounded-xl border ${config.border} ${config.bg} p-5 shadow-lg`}>
      <div className="flex items-center gap-2.5 mb-2">
        <Icon className={`h-5 w-5 ${config.iconColor}`} />
        <h4 className={`font-bold text-sm ${config.text}`}>{title || config.defaultTitle}</h4>
      </div>
      <div className="text-xs sm:text-sm text-zinc-300 leading-relaxed pl-7">
        {children}
      </div>
    </div>
  );
}
