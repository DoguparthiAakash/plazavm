import React, { useState, useEffect, useRef } from "react";
import { ThemeProvider } from "./components/ui/ThemeContext";
import { ToastProvider, useToast } from "./components/ui/Toast";
import { WorkspaceCreator } from "./components/WorkspaceCreator";
import { ConfigEditor } from "./components/ConfigEditor";
import { CommandPalette } from "./components/CommandPalette";
import { PlatformView } from "./components/PlatformView";
import { TerminalModal } from "./components/ui/TerminalModal";
import {
  fetchWorkspaces, startWorkspace, stopWorkspace, deleteWorkspace,
  checkSystemReadiness, execInWorkspace, onPlazaEvent, PlazaEvent,
  WorkspaceDto
} from "./api";
import {
  Terminal, Play, Square, Trash2, Plus, Settings, Layers,
  Cpu, Activity, ChevronRight, Search, RefreshCw,
  Circle, CheckCircle2, XCircle, Radio, Zap, ExternalLink
} from "lucide-react";

type View = "workspaces" | "platform" | "readiness" | "events";

const Dashboard: React.FC = () => {
  const [view, setView] = useState<View>("workspaces");
  const [workspaces, setWorkspaces] = useState<WorkspaceDto[]>([]);
  const [showCreator, setShowCreator] = useState(false);
  const [editingConfigId, setEditingConfigId] = useState<string | null>(null);
  const [terminalWs, setTerminalWs] = useState<WorkspaceDto | null>(null);
  const [showPalette, setShowPalette] = useState(false);
  const [readiness, setReadiness] = useState<Record<string, boolean>>({});
  const [wsLoading, setWsLoading] = useState(false);
  const [events, setEvents] = useState<PlazaEvent[]>([]);
  const [execWs, setExecWs] = useState<WorkspaceDto | null>(null);
  const [execCmd, setExecCmd] = useState("");
  const [execOutput, setExecOutput] = useState<string[]>([]);
  const [execRunning, setExecRunning] = useState(false);
  const { addToast } = useToast();

  const reloadWorkspaces = async () => {
    setWsLoading(true);
    try {
      const data = await fetchWorkspaces();
      setWorkspaces(data || []);
    } catch (err) {
      console.error(err);
      addToast({ type: "error", title: "Error", message: "Failed to fetch workspaces." });
    } finally {
      setWsLoading(false);
    }
  };

  const loadReadiness = async () => {
    try {
      const status = await checkSystemReadiness();
      setReadiness(status);
    } catch (err) {
      console.error(err);
    }
  };

  // Live event streaming
  useEffect(() => {
    const unsubscribe = onPlazaEvent((event: PlazaEvent) => {
      setEvents(prev => {
        const next = [...prev, event];
        return next.length > 200 ? next.slice(-200) : next;
      });

      // Auto-refresh workspaces on relevant events
      if (event.type.startsWith('workspace_') || event.type === 'workspaces_updated') {
        reloadWorkspaces();
      }
    });
    return () => { if (typeof unsubscribe === 'function') unsubscribe(); };
  }, []);

  useEffect(() => {
    reloadWorkspaces();
    loadReadiness();
  }, []);

  // Ctrl+K global hotkey
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setShowPalette((prev) => !prev);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  const handlePaletteAction = (action: string) => {
    switch (action) {
      case "NEW_WORKSPACE":
        setView("workspaces");
        setShowCreator(true);
        break;
      case "RELOAD_WORKSPACES":
        reloadWorkspaces();
        addToast({ type: "info", title: "Reloaded", message: "Workspace list refreshed." });
        break;
      case "CHECK_READINESS":
        setView("readiness");
        loadReadiness();
        break;
      case "VIEW_PLATFORM":
        setView("platform");
        break;
      case "VIEW_EVENTS":
        setView("events");
        break;
      case "OPEN_TERMINAL":
        if (workspaces.length > 0) setTerminalWs(workspaces[0]);
        else addToast({ type: "error", title: "No Workspaces", message: "Create a workspace first." });
        break;
    }
  };

  const handleStart = async (ws: WorkspaceDto) => {
    try {
      await startWorkspace(ws.id);
      addToast({ type: "success", title: "Started", message: `${ws.name} is starting.` });
      setTimeout(reloadWorkspaces, 2000);
    } catch (err: any) {
      addToast({ type: "error", title: "Start Failed", message: err.toString() });
    }
  };

  const handleStop = async (ws: WorkspaceDto) => {
    try {
      await stopWorkspace(ws.id);
      addToast({ type: "info", title: "Stopped", message: `${ws.name} was stopped.` });
      setTimeout(reloadWorkspaces, 2000);
    } catch (err: any) {
      addToast({ type: "error", title: "Stop Failed", message: err.toString() });
    }
  };

  const handleDelete = async (ws: WorkspaceDto) => {
    try {
      await deleteWorkspace(ws.id);
      addToast({ type: "info", title: "Deleted", message: `${ws.name} was deleted.` });
      reloadWorkspaces();
    } catch (err: any) {
      addToast({ type: "error", title: "Delete Failed", message: err.toString() });
    }
  };

  const handleExec = async (ws: WorkspaceDto, cmd: string) => {
    setExecWs(ws);
    setExecCmd(cmd);
    setExecRunning(true);
    setExecOutput([`$ ${cmd}`, 'Executing in workspace guest...']);
    try {
      const result = await execInWorkspace(ws.id, cmd);
      setExecOutput(prev => [
        ...prev,
        ...(result.output ? result.output.split('\n').filter(l => l.trim()) : []),
        ...(result.error ? result.error.split('\n').filter(l => l.trim()).map(l => `[stderr] ${l}`) : []),
        '✓ Command completed',
      ]);
    } catch (err: any) {
      setExecOutput(prev => [...prev, `✗ Error: ${err.message}`]);
    } finally {
      setExecRunning(false);
    }
  };

  const runningCount = workspaces.filter((w) => w.state === "running").length;
  const readyCount = Object.values(readiness).filter(Boolean).length;
  const totalChecks = Object.keys(readiness).length;

  return (
    <div className="flex h-screen w-screen bg-zinc-950 text-zinc-100 overflow-hidden select-none">
      {/* ─── SIDEBAR ─── */}
      <aside className="w-56 shrink-0 flex flex-col bg-zinc-900/40 border-r border-zinc-800/50">
        {/* Logo */}
        <div className="px-4 py-4 flex items-center gap-3 border-b border-zinc-800/50">
          <div className="w-7 h-7 rounded-lg bg-gradient-to-br from-emerald-500/30 to-emerald-600/10 border border-emerald-500/20 flex items-center justify-center">
            <Terminal className="w-4 h-4 text-emerald-400" />
          </div>
          <span className="text-sm font-bold tracking-tight text-white">PlazaVM</span>
          <span className="ml-auto text-[10px] text-zinc-600 font-mono">dp1</span>
        </div>

        {/* Search / Palette trigger */}
        <div className="px-3 py-3">
          <button
            onClick={() => setShowPalette(true)}
            className="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg bg-zinc-800/50 border border-zinc-700/40 text-zinc-500 text-xs hover:border-zinc-600 hover:text-zinc-400 transition-all"
          >
            <Search className="w-3.5 h-3.5" />
            <span className="flex-1 text-left">Commands...</span>
            <kbd className="text-[9px] px-1 py-0.5 bg-zinc-700/60 rounded text-zinc-600 font-sans">⌘K</kbd>
          </button>
        </div>

        {/* Navigation */}
        <nav className="flex-1 px-2 space-y-0.5 overflow-y-auto">
          <p className="text-[10px] font-semibold text-zinc-600 uppercase tracking-widest px-2 mb-1 mt-1">Main</p>
          <NavItem
            icon={<Layers className="w-4 h-4" />}
            label="Workspaces"
            active={view === "workspaces"}
            badge={runningCount > 0 ? String(runningCount) : undefined}
            onClick={() => setView("workspaces")}
          />
          <NavItem
            icon={<Cpu className="w-4 h-4" />}
            label="Platform"
            active={view === "platform"}
            onClick={() => setView("platform")}
          />
          <NavItem
            icon={<Activity className="w-4 h-4" />}
            label="Readiness"
            active={view === "readiness"}
            badge={totalChecks > 0 ? `${readyCount}/${totalChecks}` : undefined}
            badgeVariant={readyCount < totalChecks ? "warn" : "ok"}
            onClick={() => setView("readiness")}
          />
          <NavItem
            icon={<Radio className="w-4 h-4" />}
            label="Live Events"
            active={view === "events"}
            badge={events.length > 0 ? String(events.length) : undefined}
            onClick={() => setView("events")}
          />
        </nav>

        {/* Bottom actions */}
        <div className="px-2 pb-3 border-t border-zinc-800/50 pt-2 space-y-0.5">
          <button
            onClick={reloadWorkspaces}
            className="w-full flex items-center gap-2 px-2.5 py-2 rounded-lg text-xs text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/50 transition-all"
          >
            <RefreshCw className={`w-3.5 h-3.5 ${wsLoading ? "animate-spin" : ""}`} />
            Refresh
          </button>
        </div>
      </aside>

      {/* ─── MAIN CONTENT ─── */}
      <main className="flex-1 flex flex-col min-w-0">
        {/* Top Bar */}
        <header className="h-12 border-b border-zinc-800/50 flex items-center justify-between px-5 bg-zinc-900/10 shrink-0">
          <div className="flex items-center gap-1.5 text-sm text-zinc-400">
            <span className="text-zinc-600">PlazaVM</span>
            <ChevronRight className="w-3 h-3 text-zinc-700" />
            <span className="text-zinc-300 font-medium capitalize">{view}</span>
            {events.length > 0 && (
              <span className="ml-2 flex items-center gap-1 text-[10px] text-emerald-500">
                <Radio className="w-3 h-3 animate-pulse" /> LIVE
              </span>
            )}
          </div>
          {view === "workspaces" && (
            <button
              onClick={() => setShowCreator(true)}
              className="flex items-center gap-1.5 bg-emerald-600 hover:bg-emerald-500 text-white px-3 py-1.5 rounded-lg text-xs font-semibold transition-colors shadow-lg shadow-emerald-600/20"
            >
              <Plus className="w-3.5 h-3.5" />
              New Workspace
            </button>
          )}
        </header>

        {/* View Content */}
        {view === "workspaces" && (
          <WorkspacesView
            workspaces={workspaces}
            loading={wsLoading}
            onStart={handleStart}
            onStop={handleStop}
            onDelete={handleDelete}
            onConfig={(id) => setEditingConfigId(id)}
            onTerminal={(ws) => setTerminalWs(ws)}
            onExec={handleExec}
            onNew={() => setShowCreator(true)}
          />
        )}
        {view === "platform" && <PlatformView />}
        {view === "readiness" && <ReadinessView readiness={readiness} onRefresh={loadReadiness} />}
        {view === "events" && (
          <EventsView events={events} onClear={() => setEvents([])} />
        )}
      </main>

      {/* ─── EXEC OUTPUT PANEL ─── */}
      {execWs && (
        <div className="fixed bottom-0 right-0 w-[500px] max-h-[300px] bg-zinc-950 border border-zinc-800 rounded-tl-xl shadow-2xl z-50 flex flex-col">
          <div className="flex items-center justify-between px-3 py-2 border-b border-zinc-800">
            <div className="flex items-center gap-2">
              <Zap className="w-3.5 h-3.5 text-emerald-400" />
              <span className="text-xs font-medium text-zinc-300">
                Guest Exec — {execWs.name}
              </span>
              {execRunning && <span className="text-[10px] text-amber-400 animate-pulse">RUNNING</span>}
            </div>
            <button onClick={() => setExecWs(null)} className="text-zinc-500 hover:text-white text-xs">✕</button>
          </div>
          <div className="flex-1 overflow-y-auto p-3 font-mono text-[11px] text-zinc-400 space-y-0.5">
            {execOutput.map((line, i) => (
              <div key={i} className={`${
                line.startsWith('$') ? 'text-emerald-400 font-bold' :
                line.startsWith('[') ? 'text-amber-400' :
                line.startsWith('✓') ? 'text-emerald-400' :
                line.startsWith('✗') ? 'text-red-400' : ''
              }`}>{line}</div>
            ))}
          </div>
        </div>
      )}

      {/* ─── MODALS ─── */}
      {showCreator && (
        <WorkspaceCreator
          onClose={() => setShowCreator(false)}
          onCreated={() => {
            setShowCreator(false);
            reloadWorkspaces();
            addToast({ type: "success", title: "Created", message: "New workspace is initializing." });
          }}
        />
      )}
      {editingConfigId && (
        <ConfigEditor workspaceId={editingConfigId} onClose={() => setEditingConfigId(null)} />
      )}
      {terminalWs && (
        <TerminalModal
          isOpen={!!terminalWs}
          workspaceName={terminalWs.name}
          onClose={() => setTerminalWs(null)}
        />
      )}
      <CommandPalette isOpen={showPalette} onClose={() => setShowPalette(false)} onAction={handlePaletteAction} />
    </div>
  );
};

/* ─── SUB-VIEWS ─── */

const WorkspacesView: React.FC<{
  workspaces: WorkspaceDto[];
  loading: boolean;
  onStart: (ws: WorkspaceDto) => void;
  onStop: (ws: WorkspaceDto) => void;
  onDelete: (ws: WorkspaceDto) => void;
  onConfig: (id: string) => void;
  onTerminal: (ws: WorkspaceDto) => void;
  onExec: (ws: WorkspaceDto, cmd: string) => void;
  onNew: () => void;
}> = ({ workspaces, loading, onStart, onStop, onDelete, onConfig, onTerminal, onExec, onNew }) => (
  <div className="flex-1 overflow-y-auto p-5">
    {loading && workspaces.length === 0 ? (
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {[1, 2, 3].map((i) => (
          <div key={i} className="bg-zinc-900/40 border border-zinc-800/60 rounded-xl h-44 animate-pulse" />
        ))}
      </div>
    ) : workspaces.length === 0 ? (
      <div className="h-full flex flex-col items-center justify-center text-center py-20">
        <div className="w-14 h-14 rounded-2xl bg-zinc-900 border border-zinc-800 flex items-center justify-center mb-5">
          <Layers className="w-7 h-7 text-zinc-700" />
        </div>
        <h2 className="text-lg font-semibold text-zinc-200 mb-2">No workspaces yet</h2>
        <p className="text-zinc-500 text-sm max-w-xs mb-6">
          Create your first isolated environment to get started with PlazaVM.
        </p>
        <button
          onClick={onNew}
          className="flex items-center gap-2 bg-emerald-600 hover:bg-emerald-500 text-white px-5 py-2 rounded-lg text-sm font-semibold transition-colors shadow-lg shadow-emerald-600/20"
        >
          <Plus className="w-4 h-4" /> New Workspace
        </button>
      </div>
    ) : (
      <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4">
        {workspaces.map((ws) => (
          <WorkspaceCard
            key={ws.id}
            ws={ws}
            onStart={() => onStart(ws)}
            onStop={() => onStop(ws)}
            onDelete={() => onDelete(ws)}
            onConfig={() => onConfig(ws.id)}
            onTerminal={() => onTerminal(ws)}
            onExec={(cmd) => onExec(ws, cmd)}
          />
        ))}
        {/* Add card */}
        <button
          onClick={onNew}
          className="bg-zinc-900/20 border border-dashed border-zinc-800 rounded-xl p-5 flex flex-col items-center justify-center gap-3 text-zinc-600 hover:border-zinc-600 hover:text-zinc-400 hover:bg-zinc-900/40 transition-all min-h-[160px]"
        >
          <Plus className="w-6 h-6" />
          <span className="text-sm font-medium">New Workspace</span>
        </button>
      </div>
    )}
  </div>
);

const WorkspaceCard: React.FC<{
  ws: WorkspaceDto;
  onStart: () => void;
  onStop: () => void;
  onDelete: () => void;
  onConfig: () => void;
  onTerminal: () => void;
  onExec: (cmd: string) => void;
}> = ({ ws, onStart, onStop, onDelete, onConfig, onTerminal, onExec }) => {
  const isRunning = ws.state === "running";
  const isStopped = ws.state === "stopped";

  return (
    <div className={`bg-zinc-900/50 border rounded-xl p-4 flex flex-col gap-4 transition-all hover:border-zinc-700 ${
      isRunning ? "border-emerald-800/40" : "border-zinc-800/60"
    }`}>
      {/* Card Header */}
      <div className="flex items-start justify-between">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            <div className={`w-2 h-2 rounded-full shrink-0 ${isRunning ? "bg-emerald-500 shadow-sm shadow-emerald-500/50" : "bg-zinc-600"}`} />
            <h3 className="text-sm font-semibold text-zinc-100 truncate">{ws.name}</h3>
            <span className={`text-[9px] font-bold uppercase px-1.5 py-0.5 rounded-full shrink-0 ${
              isRunning ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20" : "bg-zinc-800 text-zinc-500 border border-zinc-700"
            }`}>{ws.state}</span>
          </div>
          <p className="text-[11px] text-zinc-600 font-mono truncate">{ws.id}</p>
          {ws.project_path && (
            <p className="text-[10px] text-zinc-600 mt-0.5 flex items-center gap-1">
              <ExternalLink className="w-2.5 h-2.5" />
              <span className="truncate max-w-[200px]">{ws.project_path}</span>
            </p>
          )}
        </div>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-3 gap-2 text-xs">
        <div className="bg-zinc-900/60 border border-zinc-800/40 rounded-lg p-2 text-center">
          <div className="text-zinc-600 text-[10px] mb-0.5">Backend</div>
          <div className="text-zinc-300 font-medium truncate">{ws.runtime_backend || "QEMU"}</div>
        </div>
        <div className="bg-zinc-900/60 border border-zinc-800/40 rounded-lg p-2 text-center">
          <div className="text-zinc-600 text-[10px] mb-0.5">vCPU</div>
          <div className="text-zinc-300 font-medium">{ws.cpu_cores}</div>
        </div>
        <div className="bg-zinc-900/60 border border-zinc-800/40 rounded-lg p-2 text-center">
          <div className="text-zinc-600 text-[10px] mb-0.5">RAM</div>
          <div className="text-zinc-300 font-medium">{(ws.memory_mb / 1024).toFixed(0)}G</div>
        </div>
      </div>

      {/* Quick exec buttons */}
      {isRunning && (
        <div className="flex items-center gap-1.5 flex-wrap">
          <button
            onClick={() => onExec("ls /workspace")}
            title="List workspace"
            className="px-2 py-1 rounded text-[10px] font-mono bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 transition-all"
          >
            ls
          </button>
          <button
            onClick={() => onExec("pwd")}
            title="Print working directory"
            className="px-2 py-1 rounded text-[10px] font-mono bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 transition-all"
          >
            pwd
          </button>
          <button
            onClick={() => onExec("uname -a")}
            title="System info"
            className="px-2 py-1 rounded text-[10px] font-mono bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 transition-all"
          >
            uname
          </button>
          <button
            onClick={() => onExec("df -h")}
            title="Disk usage"
            className="px-2 py-1 rounded text-[10px] font-mono bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 transition-all"
          >
            df
          </button>
        </div>
      )}

      {/* Action Buttons */}
      <div className="flex items-center gap-1.5">
        <button
          onClick={onStart}
          disabled={isRunning}
          title="Start"
          className="flex-1 flex items-center justify-center gap-1 py-1.5 rounded-lg text-xs font-medium bg-zinc-800 hover:bg-emerald-600 hover:text-white text-zinc-300 disabled:opacity-30 disabled:cursor-not-allowed transition-all"
        >
          <Play className="w-3.5 h-3.5" /> Start
        </button>
        <button
          onClick={onStop}
          disabled={isStopped || !isRunning}
          title="Stop"
          className="flex-1 flex items-center justify-center gap-1 py-1.5 rounded-lg text-xs font-medium bg-zinc-800 hover:bg-amber-600/70 hover:text-white text-zinc-300 disabled:opacity-30 disabled:cursor-not-allowed transition-all"
        >
          <Square className="w-3.5 h-3.5" /> Stop
        </button>
        <button
          onClick={onTerminal}
          title="Terminal"
          className="py-1.5 px-2.5 rounded-lg text-xs bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 transition-all"
        >
          <Terminal className="w-3.5 h-3.5" />
        </button>
        <button
          onClick={onConfig}
          title="Configuration"
          className="py-1.5 px-2.5 rounded-lg text-xs bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 transition-all"
        >
          <Settings className="w-3.5 h-3.5" />
        </button>
        <button
          onClick={onDelete}
          title="Delete"
          className="py-1.5 px-2.5 rounded-lg text-xs bg-zinc-800 hover:bg-red-900/50 text-zinc-500 hover:text-red-400 transition-all"
        >
          <Trash2 className="w-3.5 h-3.5" />
        </button>
      </div>
    </div>
  );
};

/* ─── EVENTS VIEW ─── */
const EventsView: React.FC<{ events: PlazaEvent[]; onClear: () => void }> = ({ events, onClear }) => {
  const bottomRef = useRef<HTMLDivElement>(null);
  const [autoScroll, setAutoScroll] = useState(true);

  useEffect(() => {
    if (autoScroll && bottomRef.current) {
      bottomRef.current.scrollIntoView({ behavior: "smooth" });
    }
  }, [events, autoScroll]);

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      <div className="flex items-center justify-between px-5 py-3 border-b border-zinc-800/50">
        <div>
          <h2 className="text-sm font-bold text-zinc-100">Live Event Stream</h2>
          <p className="text-[11px] text-zinc-500">{events.length} events • Real-time backend updates</p>
        </div>
        <div className="flex items-center gap-2">
          <label className="flex items-center gap-1.5 text-[11px] text-zinc-500">
            <input
              type="checkbox"
              checked={autoScroll}
              onChange={(e) => setAutoScroll(e.target.checked)}
              className="rounded"
            />
            Auto-scroll
          </label>
          <button onClick={onClear} className="text-[11px] text-zinc-500 hover:text-zinc-300 px-2 py-1 rounded bg-zinc-800 hover:bg-zinc-700 transition-colors">
            Clear
          </button>
        </div>
      </div>
      <div className="flex-1 overflow-y-auto p-4 font-mono text-[11px] space-y-1">
        {events.length === 0 ? (
          <div className="text-zinc-600 text-center py-10">
            No events yet. Start or create a workspace to see live updates.
          </div>
        ) : (
          events.map((evt) => (
            <div key={evt.id} className="flex items-start gap-2 hover:bg-zinc-900/50 px-2 py-1 rounded">
              <span className="text-zinc-600 shrink-0 w-[70px]">
                {new Date(evt.timestamp).toLocaleTimeString()}
              </span>
              <span className={`shrink-0 px-1.5 py-0.5 rounded text-[9px] font-bold ${
                evt.type.includes('error') || evt.type.includes('failed') ? 'bg-red-900/40 text-red-400' :
                evt.type.includes('completed') || evt.type.includes('created') || evt.type.includes('started') ? 'bg-emerald-900/40 text-emerald-400' :
                evt.type.includes('progress') || evt.type.includes('output') ? 'bg-blue-900/40 text-blue-400' :
                'bg-zinc-800 text-zinc-400'
              }`}>
                {evt.type}
              </span>
              <span className="text-zinc-400 break-all">
                {typeof evt.data === 'object' ? JSON.stringify(evt.data).slice(0, 120) : String(evt.data)}
              </span>
            </div>
          ))
        )}
        <div ref={bottomRef} />
      </div>
    </div>
  );
};

/* ─── READINESS VIEW ─── */
const ReadinessView: React.FC<{ readiness: Record<string, boolean>; onRefresh: () => void }> = ({ readiness, onRefresh }) => {
  const entries = Object.entries(readiness);
  const allGood = entries.every(([, v]) => v);

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-5">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-bold text-zinc-100">System Readiness</h1>
          <p className="text-sm text-zinc-500 mt-0.5">PlazaVM component health checks</p>
        </div>
        <button
          onClick={onRefresh}
          className="flex items-center gap-2 text-sm text-zinc-400 hover:text-white bg-zinc-800 hover:bg-zinc-700 px-3 py-1.5 rounded-lg transition-colors"
        >
          <RefreshCw className="w-4 h-4" /> Re-check
        </button>
      </div>

      {entries.length === 0 ? (
        <div className="text-zinc-500 text-sm">No readiness data. Click Re-check to run diagnostics.</div>
      ) : (
        <>
          <div className={`flex items-center gap-3 p-4 rounded-xl border ${allGood ? "bg-emerald-950/30 border-emerald-800/30" : "bg-amber-950/30 border-amber-800/30"}`}>
            {allGood
              ? <CheckCircle2 className="w-5 h-5 text-emerald-400" />
              : <Circle className="w-5 h-5 text-amber-400" />
            }
            <div>
              <p className={`font-semibold text-sm ${allGood ? "text-emerald-300" : "text-amber-300"}`}>
                {allGood ? "All systems operational" : "Some systems need attention"}
              </p>
              <p className="text-xs text-zinc-500">{entries.filter(([, v]) => v).length} of {entries.length} checks passing</p>
            </div>
          </div>

          <div className="space-y-2">
            {entries.map(([key, ok]) => (
              <div key={key} className="flex items-center justify-between bg-zinc-900/50 border border-zinc-800/60 rounded-xl px-4 py-3 hover:border-zinc-700 transition-colors">
                <div className="flex items-center gap-3">
                  {ok
                    ? <CheckCircle2 className="w-4 h-4 text-emerald-500 shrink-0" />
                    : <XCircle className="w-4 h-4 text-red-500 shrink-0" />
                  }
                  <span className="text-sm text-zinc-300 font-medium capitalize">{key.replace(/_/g, " ")}</span>
                </div>
                <span className={`text-xs font-semibold ${ok ? "text-emerald-400" : "text-red-400"}`}>
                  {ok ? "OK" : "FAIL"}
                </span>
              </div>
            ))}
          </div>
        </>
      )}
    </div>
  );
};

/* ─── NAV ITEM ─── */
const NavItem: React.FC<{
  icon: React.ReactNode;
  label: string;
  active: boolean;
  badge?: string;
  badgeVariant?: "ok" | "warn";
  onClick: () => void;
}> = ({ icon, label, active, badge, badgeVariant = "ok", onClick }) => (
  <button
    onClick={onClick}
    className={`w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-sm font-medium transition-all ${
      active
        ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20"
        : "text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/40 border border-transparent"
    }`}
  >
    <span className={active ? "text-emerald-400" : "text-zinc-600"}>{icon}</span>
    <span className="flex-1 text-left">{label}</span>
    {badge && (
      <span className={`text-[10px] font-mono px-1.5 py-0.5 rounded-full font-bold ${
        badgeVariant === "warn"
          ? "bg-amber-500/10 text-amber-400 border border-amber-500/20"
          : "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20"
      }`}>
        {badge}
      </span>
    )}
  </button>
);

/* ─── ROOT APP ─── */
export const App: React.FC = () => (
  <ThemeProvider>
    <ToastProvider>
      <Dashboard />
    </ToastProvider>
  </ThemeProvider>
);

export default App;
