import React from "react";
import { Bell, X, CheckCircle2, AlertTriangle, Cpu, RefreshCw, Layers } from "lucide-react";

interface NotificationsDrawerProps {
  isOpen: boolean;
  onClose: () => void;
}

export const NotificationsDrawer: React.FC<NotificationsDrawerProps> = ({ isOpen, onClose }) => {
  if (!isOpen) return null;

  const notifications = [
    {
      id: "1",
      title: "Workspace 'ubuntu-cuda-dev' Ready",
      description: "NVIDIA CUDA 12.5 driver and PyTorch 2.3 environment initialized successfully.",
      time: "2 mins ago",
      type: "success",
      icon: CheckCircle2,
    },
    {
      id: "2",
      title: "High Memory Threshold",
      description: "Host RAM usage exceeded 75%. PUR daemon auto-compacted cache pages.",
      time: "15 mins ago",
      type: "warning",
      icon: AlertTriangle,
    },
    {
      id: "3",
      title: "Plaza PUR Kernel 6.6.38 Updated",
      description: "Virtio-GPU acceleration patch applied. Restart active VMs to apply.",
      time: "1 hour ago",
      type: "info",
      icon: RefreshCw,
    },
    {
      id: "4",
      title: "Snapshot Created",
      description: "Point-in-time snapshot 'pre-cuda-update' generated for node-01.",
      time: "3 hours ago",
      type: "success",
      icon: Layers,
    },
  ];

  return (
    <div className="fixed inset-0 z-50 flex justify-end bg-slate-950/60 backdrop-blur-sm animate-in fade-in duration-200">
      <div className="w-full max-w-md bg-slate-950 border-l border-slate-800 shadow-2xl flex flex-col h-full animate-in slide-in-from-right duration-300">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-slate-800/80 bg-slate-900/50 select-none">
          <div className="flex items-center gap-2">
            <div className="p-2 rounded-xl bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
              <Bell className="w-4 h-4" />
            </div>
            <div>
              <h3 className="font-extrabold text-sm text-slate-100">System Notifications</h3>
              <p className="text-[11px] text-slate-400">Activity stream & system alerts</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-1.5 text-slate-400 hover:text-white hover:bg-slate-800 rounded-xl transition"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* List */}
        <div className="flex-1 overflow-y-auto p-4 space-y-3">
          {notifications.map((item) => {
            const Icon = item.icon;
            return (
              <div
                key={item.id}
                className="p-3.5 rounded-2xl glass-card border border-slate-800/80 space-y-2 select-none hover:border-cyan-500/30"
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Icon
                      className={`w-4 h-4 ${
                        item.type === "success"
                          ? "text-emerald-400"
                          : item.type === "warning"
                          ? "text-amber-400"
                          : "text-cyan-400"
                      }`}
                    />
                    <span className="text-xs font-bold text-slate-200">{item.title}</span>
                  </div>
                  <span className="text-[10px] text-slate-500 font-mono">{item.time}</span>
                </div>
                <p className="text-[11px] text-slate-400 leading-relaxed pl-6">{item.description}</p>
              </div>
            );
          })}
        </div>

        {/* Footer */}
        <div className="p-4 border-t border-slate-800/80 bg-slate-900/30 flex items-center justify-between">
          <span className="text-[11px] text-slate-400">4 Unread Alerts</span>
          <button className="text-[11px] font-semibold text-cyan-400 hover:text-cyan-300 transition">
            Mark all as read
          </button>
        </div>
      </div>
    </div>
  );
};
