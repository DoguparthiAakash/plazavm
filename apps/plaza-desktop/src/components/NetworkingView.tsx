import React, { useState } from "react";
import { Network, Shield, ArrowRightLeft, Radio, Plus, RefreshCw, CheckCircle2 } from "lucide-react";

export const NetworkingView: React.FC = () => {
  const [portForwards, setPortForwards] = useState([
    { id: "1", hostPort: "8080", guestPort: "8080", workspace: "ubuntu-cuda-dev", protocol: "TCP", status: "Active" },
    { id: "2", hostPort: "3000", guestPort: "3000", workspace: "rust-microservices", protocol: "TCP", status: "Active" },
    { id: "3", hostPort: "8888", guestPort: "8888", workspace: "ubuntu-cuda-dev", protocol: "TCP", status: "Active" },
    { id: "4", hostPort: "5432", guestPort: "5432", workspace: "postgres-db", protocol: "TCP", status: "Active" },
  ]);

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6 select-none">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-black text-slate-100 tracking-tight flex items-center gap-2">
            <Network className="w-6 h-6 text-cyan-400" />
            Virtual Networking Topology
          </h2>
          <p className="text-xs text-slate-400">
            Manage virtual bridges, host port bindings, DNS resolvers, and isolation rules.
          </p>
        </div>

        <button className="flex items-center gap-2 px-4 py-2.5 bg-gradient-to-r from-cyan-500 to-teal-500 text-slate-950 font-bold rounded-xl text-xs shadow-lg shadow-cyan-500/20 transition active:scale-95">
          <Plus className="w-4 h-4 stroke-[3]" /> Add Port Binding
        </button>
      </div>

      {/* Network Adapters Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="p-5 rounded-2xl glass-card border border-slate-800 space-y-3">
          <div className="flex items-center justify-between">
            <span className="text-xs font-bold text-slate-100 flex items-center gap-2">
              <Radio className="w-4 h-4 text-cyan-400" /> plaza-br0 (NAT Bridge)
            </span>
            <span className="text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/30">
              Active
            </span>
          </div>
          <p className="text-xs text-slate-400 font-mono">IP: 172.28.0.1 / Subnet: 172.28.0.0/16</p>
          <div className="flex items-center justify-between text-[11px] text-slate-500 pt-2 border-t border-slate-800">
            <span>4 Workspaces Attached</span>
            <span className="font-mono text-cyan-400">1.2 Gbps Tx/Rx</span>
          </div>
        </div>

        <div className="p-5 rounded-2xl glass-card border border-slate-800 space-y-3">
          <div className="flex items-center justify-between">
            <span className="text-xs font-bold text-slate-100 flex items-center gap-2">
              <Shield className="w-4 h-4 text-teal-400" /> DirectSocket (VSock)
            </span>
            <span className="text-[10px] font-mono text-teal-400 bg-teal-500/10 px-2 py-0.5 rounded border border-teal-500/30">
              Zero-Overhead
            </span>
          </div>
          <p className="text-xs text-slate-400 font-mono">CID: 32 / Port range: 1024-65535</p>
          <div className="flex items-center justify-between text-[11px] text-slate-500 pt-2 border-t border-slate-800">
            <span>Kernel Fast-Path</span>
            <span className="font-mono text-teal-400">&lt; 0.1ms Latency</span>
          </div>
        </div>

        <div className="p-5 rounded-2xl glass-card border border-slate-800 space-y-3">
          <div className="flex items-center justify-between">
            <span className="text-xs font-bold text-slate-100 flex items-center gap-2">
              <ArrowRightLeft className="w-4 h-4 text-amber-400" /> Internal DNS Resolver
            </span>
            <span className="text-[10px] font-mono text-amber-400 bg-amber-500/10 px-2 py-0.5 rounded border border-amber-500/30">
              *.plaza.internal
            </span>
          </div>
          <p className="text-xs text-slate-400 font-mono">Upstream: 1.1.1.1, 8.8.8.8</p>
          <div className="flex items-center justify-between text-[11px] text-slate-500 pt-2 border-t border-slate-800">
            <span>Auto mDNS Enabled</span>
            <span className="font-mono text-amber-400">Ready</span>
          </div>
        </div>
      </div>

      {/* Port Forwarding Matrix Table */}
      <div className="p-6 rounded-3xl glass-card border border-slate-800 space-y-4">
        <div className="flex items-center justify-between">
          <h3 className="text-sm font-extrabold text-slate-100 flex items-center gap-2">
            <ArrowRightLeft className="w-4 h-4 text-cyan-400" /> Active Port Forwarding Rules
          </h3>
          <span className="text-xs font-mono text-slate-400">4 Forwarders Active</span>
        </div>

        <div className="overflow-x-auto">
          <table className="w-full text-left text-xs border-collapse">
            <thead>
              <tr className="border-b border-slate-800 text-slate-400 font-mono uppercase text-[10px]">
                <th className="py-2.5 px-3">Host Port</th>
                <th className="py-2.5 px-3">Guest Port</th>
                <th className="py-2.5 px-3">Target Workspace</th>
                <th className="py-2.5 px-3">Protocol</th>
                <th className="py-2.5 px-3">Status</th>
                <th className="py-2.5 px-3 text-right">Actions</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-800/60 font-mono text-slate-200">
              {portForwards.map((item) => (
                <tr key={item.id} className="hover:bg-slate-900/60 transition">
                  <td className="py-3 px-3 font-bold text-cyan-400">localhost:{item.hostPort}</td>
                  <td className="py-3 px-3 text-slate-300">:{item.guestPort}</td>
                  <td className="py-3 px-3 font-sans text-slate-200 font-semibold">{item.workspace}</td>
                  <td className="py-3 px-3 text-slate-400">{item.protocol}</td>
                  <td className="py-3 px-3">
                    <span className="px-2 py-0.5 rounded text-[10px] bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 flex items-center gap-1 w-max">
                      <CheckCircle2 className="w-3 h-3" /> {item.status}
                    </span>
                  </td>
                  <td className="py-3 px-3 text-right font-sans">
                    <button className="text-slate-400 hover:text-red-400 text-xs font-medium transition">
                      Remove
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};
