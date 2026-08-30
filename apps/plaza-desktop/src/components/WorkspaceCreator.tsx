import React, { useState } from "react";
import { createWorkspace, CreateWorkspaceRequest } from "../api";
import { X, Sparkles, Cpu, HardDrive, ChevronDown } from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";

interface WorkspaceCreatorProps {
  isOpen?: boolean;
  onClose: () => void;
  onSuccess?: () => void;
  onCreated?: () => void;
}

const IMAGES = [
  { value: "ubuntu:24.04", label: "Ubuntu 24.04 LTS", tag: "Linux", color: "text-orange-400" },
  { value: "freebsd:14.1", label: "FreeBSD 14.1", tag: "BSD", color: "text-blue-400" },
  { value: "openbsd:7.5", label: "OpenBSD 7.5", tag: "BSD", color: "text-yellow-400" },
  { value: "netbsd:10.0", label: "NetBSD 10.0", tag: "BSD", color: "text-cyan-400" },
  { value: "python:3.12", label: "Python 3.12 Dev", tag: "Dev", color: "text-emerald-400" },
  { value: "rust:latest", label: "Rust Development", tag: "Dev", color: "text-orange-500" },
  { value: "node:22", label: "Node.js 22 LTS", tag: "Dev", color: "text-green-400" },
  { value: "cuda:12.0", label: "CUDA 12 AI Runtime", tag: "AI/ML", color: "text-purple-400" },
];

export const WorkspaceCreator: React.FC<WorkspaceCreatorProps> = ({
  isOpen = true,
  onClose,
  onSuccess,
  onCreated,
}) => {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [image, setImage] = useState("ubuntu:24.04");
  const [cores, setCores] = useState(2);
  const [memory, setMemory] = useState(2048);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  if (!isOpen) return null;

  const selectedImg = IMAGES.find((i) => i.value === image) || IMAGES[0];

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) { setError("Workspace name is required."); return; }
    setError("");
    setLoading(true);
    try {
      const req: CreateWorkspaceRequest = { name: name.trim(), image, cpu_cores: cores, memory_mb: memory };
      await createWorkspace(req);
      if (onSuccess) onSuccess();
      if (onCreated) onCreated();
      onClose();
    } catch (err: any) {
      setError(err?.toString() || "Failed to create workspace.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <AnimatePresence>
      {isOpen && (
        <div className="fixed inset-0 bg-zinc-950/80 backdrop-blur-sm flex items-center justify-center p-4 z-50">
          <motion.div
            initial={{ opacity: 0, scale: 0.95, y: 10 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.95, y: 10 }}
            transition={{ type: "spring", stiffness: 300, damping: 28 }}
            className="bg-zinc-950 border border-zinc-800/80 rounded-2xl w-full max-w-lg overflow-hidden shadow-2xl ring-1 ring-white/5"
          >
            {/* Header */}
            <div className="flex items-center justify-between px-6 py-4 border-b border-zinc-800/50 bg-zinc-900/30">
              <div className="flex items-center gap-2.5">
                <div className="w-7 h-7 rounded-lg bg-emerald-500/10 flex items-center justify-center">
                  <Sparkles className="w-4 h-4 text-emerald-400" />
                </div>
                <h2 className="font-semibold text-white text-base">New Workspace</h2>
              </div>
              <button onClick={onClose} className="p-1.5 text-zinc-500 hover:text-white hover:bg-zinc-800 rounded-lg transition-colors">
                <X className="w-4 h-4" />
              </button>
            </div>

            {/* Form Body */}
            <form onSubmit={handleSubmit} className="p-6 space-y-5">
              {/* Name */}
              <div>
                <label className="block text-xs font-medium text-zinc-400 mb-1.5 uppercase tracking-wider">
                  Workspace Name <span className="text-emerald-500">*</span>
                </label>
                <input
                  type="text"
                  required
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="e.g. python-ai-lab"
                  className="w-full bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2.5 text-sm text-white placeholder-zinc-600 focus:outline-none focus:border-emerald-500/50 focus:ring-1 focus:ring-emerald-500/20 transition-all"
                />
              </div>

              {/* Description */}
              <div>
                <label className="block text-xs font-medium text-zinc-400 mb-1.5 uppercase tracking-wider">
                  Description <span className="text-zinc-600">(Optional)</span>
                </label>
                <input
                  type="text"
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  placeholder="What will you build in this workspace?"
                  className="w-full bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2.5 text-sm text-white placeholder-zinc-600 focus:outline-none focus:border-emerald-500/50 focus:ring-1 focus:ring-emerald-500/20 transition-all"
                />
              </div>

              {/* Image Select */}
              <div>
                <label className="block text-xs font-medium text-zinc-400 mb-1.5 uppercase tracking-wider">
                  Base Environment
                </label>
                <div className="relative">
                  <select
                    value={image}
                    onChange={(e) => setImage(e.target.value)}
                    className="w-full appearance-none bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2.5 text-sm text-white focus:outline-none focus:border-emerald-500/50 focus:ring-1 focus:ring-emerald-500/20 transition-all pr-10 cursor-pointer"
                  >
                    {IMAGES.map((img) => (
                      <option key={img.value} value={img.value}>
                        [{img.tag}] {img.label}
                      </option>
                    ))}
                  </select>
                  <ChevronDown className="absolute right-3 top-1/2 -translate-y-1/2 w-4 h-4 text-zinc-500 pointer-events-none" />
                </div>
                <p className={`text-xs mt-1.5 ${selectedImg.color}`}>
                  Selected: {selectedImg.label}
                </p>
              </div>

              {/* Resources */}
              <div>
                <label className="block text-xs font-medium text-zinc-400 mb-1.5 uppercase tracking-wider">
                  Resource Allocation
                </label>
                <div className="grid grid-cols-2 gap-3">
                  <div className="bg-zinc-900/70 border border-zinc-800 rounded-lg p-3">
                    <div className="flex items-center gap-2 mb-2">
                      <Cpu className="w-3.5 h-3.5 text-blue-400" />
                      <span className="text-xs text-zinc-400">CPU Cores</span>
                    </div>
                    <input
                      type="number"
                      min={1}
                      max={32}
                      value={cores}
                      onChange={(e) => setCores(Number(e.target.value))}
                      className="w-full bg-zinc-950 border border-zinc-700/50 rounded px-2 py-1.5 text-sm text-white focus:outline-none focus:border-blue-500/50 text-center font-mono"
                    />
                  </div>
                  <div className="bg-zinc-900/70 border border-zinc-800 rounded-lg p-3">
                    <div className="flex items-center gap-2 mb-2">
                      <HardDrive className="w-3.5 h-3.5 text-violet-400" />
                      <span className="text-xs text-zinc-400">Memory (MB)</span>
                    </div>
                    <input
                      type="number"
                      min={512}
                      step={512}
                      max={65536}
                      value={memory}
                      onChange={(e) => setMemory(Number(e.target.value))}
                      className="w-full bg-zinc-950 border border-zinc-700/50 rounded px-2 py-1.5 text-sm text-white focus:outline-none focus:border-violet-500/50 text-center font-mono"
                    />
                  </div>
                </div>
                <p className="text-xs text-zinc-600 mt-1.5">
                  {(memory / 1024).toFixed(1)} GB RAM · {cores} vCPU{cores > 1 ? "s" : ""}
                </p>
              </div>

              {/* Error */}
              {error && (
                <div className="bg-red-950/40 border border-red-800/40 rounded-lg px-3 py-2 text-sm text-red-400">
                  {error}
                </div>
              )}

              {/* Actions */}
              <div className="flex justify-end gap-2 pt-1">
                <button
                  type="button"
                  onClick={onClose}
                  className="px-4 py-2 text-sm text-zinc-400 hover:text-white hover:bg-zinc-800 rounded-lg transition-colors"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  disabled={loading}
                  className="px-5 py-2 rounded-lg text-sm bg-emerald-600 hover:bg-emerald-500 text-white font-medium shadow-lg shadow-emerald-600/20 disabled:opacity-50 disabled:cursor-not-allowed transition-all flex items-center gap-2"
                >
                  {loading ? (
                    <>
                      <svg className="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                        <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                        <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                      </svg>
                      Creating...
                    </>
                  ) : (
                    <>
                      <Sparkles className="w-4 h-4" />
                      Create Workspace
                    </>
                  )}
                </button>
              </div>
            </form>
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
};
