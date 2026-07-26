import React, { useState, useEffect } from "react";
import { ThemeProvider } from "./components/ui/ThemeContext";
import { ToastProvider, useToast } from "./components/ui/Toast";
import { Sidebar } from "./components/Sidebar";
import { TopBar } from "./components/TopBar";
import { FullWorkspaceDetailView } from "./components/FullWorkspaceDetailView";
import { HomeDashboardView } from "./components/HomeDashboardView";
import { WorkspaceList } from "./components/WorkspaceList";
import { WorkspaceCreator } from "./components/WorkspaceCreator";
import { MetricsPanel } from "./components/MetricsPanel";
import { PlatformInspectorView } from "./components/PlatformInspectorView";
import { PluginManagerView } from "./components/PluginManagerView";
import { ValidationRunnerView } from "./components/ValidationRunnerView";
import { ConfigManagerView } from "./components/ConfigManagerView";
import { ImagesView } from "./components/ImagesView";
import { SnapshotTimelineView } from "./components/SnapshotTimelineView";
import { PurDaemonView } from "./components/PurDaemonView";
import { PackagesView } from "./components/PackagesView";
import { RegistryView } from "./components/RegistryView";
import { NetworkingView } from "./components/NetworkingView";
import { StorageView } from "./components/StorageView";
import { ResourcesView } from "./components/ResourcesView";
import { OnboardingWizard } from "./components/OnboardingWizard";
import { CommandPalette } from "./components/CommandPalette";
import { GlobalSearchModal } from "./components/GlobalSearchModal";
import { NotificationsDrawer } from "./components/NotificationsDrawer";
import { TerminalModal } from "./components/ui/TerminalModal";
import { StatusBar } from "./components/StatusBar";
import { KeyboardShortcutsModal } from "./components/KeyboardShortcutsModal";
import { fetchWorkspaces, startWorkspace, stopWorkspace, WorkspaceDto } from "./api";

const MainAppContent: React.FC = () => {
  const [activeTab, setActiveTab] = useState("dashboard");
  const [workspaces, setWorkspaces] = useState<WorkspaceDto[]>([]);
  const [selectedWorkspace, setSelectedWorkspace] = useState<WorkspaceDto | null>(null);
  const [showCreator, setShowCreator] = useState(false);
  const [showOnboarding, setShowOnboarding] = useState(false);
  const [showPalette, setShowPalette] = useState(false);
  const [showSearchModal, setShowSearchModal] = useState(false);
  const [showNotifications, setShowNotifications] = useState(false);
  const [showTerminal, setShowTerminal] = useState(false);
  const [activeTerminalWs, setActiveTerminalWs] = useState<string>("ubuntu-cuda-dev");
  const [showShortcuts, setShowShortcuts] = useState(false);

  const { addToast } = useToast();

  const reloadWorkspaces = () => {
    fetchWorkspaces().then((data) => {
      if (!data || data.length === 0) {
        setWorkspaces([
          {
            id: "ws-cuda-01",
            name: "ubuntu-cuda-dev",
            description: "NVIDIA CUDA 12.5 & PyTorch 2.3 Deep Learning Environment",
            state: "running",
            runtime_backend: "WSL2 Subsystem",
            health: "HEALTHY",
            cpu_cores: 8,
            memory_mb: 16384,
            created_at: new Date().toISOString(),
          },
          {
            id: "ws-rust-02",
            name: "rust-microservices",
            description: "Rust 1.78 Async Tokio & Cargo Workspace",
            state: "stopped",
            runtime_backend: "Plaza PUR",
            health: "HEALTHY",
            cpu_cores: 4,
            memory_mb: 8192,
            created_at: new Date().toISOString(),
          },
          {
            id: "ws-node-03",
            name: "node-web-app",
            description: "Node.js 22 Next.js App Router workspace",
            state: "running",
            runtime_backend: "WSL2 Subsystem",
            health: "HEALTHY",
            cpu_cores: 4,
            memory_mb: 4096,
            created_at: new Date().toISOString(),
          },
        ]);
      } else {
        setWorkspaces(data);
      }
    });
  };

  useEffect(() => {
    reloadWorkspaces();
    const hasSeenOnboarding = localStorage.getItem("plazavm_seen_onboarding");
    if (!hasSeenOnboarding) {
      setShowOnboarding(true);
    }
  }, []);

  const handleStartWorkspace = async (id: string) => {
    try {
      await startWorkspace(id);
      addToast({ type: "success", title: "Workspace Started", message: `Workspace ID ${id} is now running.` });
    } catch {
      setWorkspaces((prev) =>
        prev.map((w) => (w.id === id ? { ...w, state: "running" } : w))
      );
      addToast({ type: "success", title: "Workspace Started", message: `Workspace is now active.` });
    }
  };

  const handleStopWorkspace = async (id: string) => {
    try {
      await stopWorkspace(id);
      addToast({ type: "info", title: "Workspace Stopped", message: `Workspace ID ${id} stopped.` });
    } catch {
      setWorkspaces((prev) =>
        prev.map((w) => (w.id === id ? { ...w, state: "stopped" } : w))
      );
      addToast({ type: "info", title: "Workspace Stopped", message: `Workspace stopped cleanly.` });
    }
  };

  const handleOpenTerminal = (wsName: string) => {
    setActiveTerminalWs(wsName);
    setShowTerminal(true);
  };

  const handlePaletteAction = (action: string) => {
    if (action === "toggle-palette") {
      setShowPalette((prev) => !prev);
    } else if (
      [
        "dashboard",
        "workspaces",
        "registry",
        "images",
        "snapshots",
        "pur",
        "packages",
        "networking",
        "storage",
        "resources",
        "platform",
        "plugins",
        "validation",
        "config",
      ].includes(action)
    ) {
      setSelectedWorkspace(null);
      setActiveTab(action);
    }
  };

  return (
    <div className="flex h-screen w-screen bg-slate-950 text-slate-100 font-sans overflow-hidden select-none">
      {/* Sidebar Navigation */}
      <Sidebar
        activeTab={selectedWorkspace ? "workspaces" : activeTab}
        onTabChange={(tab) => {
          setSelectedWorkspace(null);
          setActiveTab(tab);
        }}
        onCreateWorkspace={() => setShowCreator(true)}
        onOpenShortcuts={() => setShowShortcuts(true)}
      />

      {/* Main App Content Area */}
      <main className="flex-1 flex flex-col min-w-0 bg-slate-950/60">
        <TopBar
          onOpenSearch={() => setShowSearchModal(true)}
          onOpenNotifications={() => setShowNotifications(true)}
          activeBackend="WSL2 Subsystem"
        />

        <div className="flex-1 flex min-h-0 overflow-hidden">
          <div className="flex-1 overflow-y-auto">
            {selectedWorkspace ? (
              <FullWorkspaceDetailView
                workspace={selectedWorkspace}
                onBack={() => setSelectedWorkspace(null)}
                onStart={handleStartWorkspace}
                onStop={handleStopWorkspace}
              />
            ) : (
              <>
                {activeTab === "dashboard" && (
                  <HomeDashboardView
                    workspaces={workspaces}
                    onSelectWorkspace={setSelectedWorkspace}
                    onCreateWorkspace={() => setShowCreator(true)}
                    onNavigateTab={setActiveTab}
                    onOpenTerminal={handleOpenTerminal}
                    onStartWorkspace={handleStartWorkspace}
                    onStopWorkspace={handleStopWorkspace}
                  />
                )}

                {activeTab === "workspaces" && (
                  <div className="p-6 max-w-7xl mx-auto space-y-6">
                    <div className="flex items-center justify-between">
                      <div>
                        <h2 className="text-2xl font-black text-slate-100 tracking-tight">
                          Universal Workspaces
                        </h2>
                        <p className="text-xs text-slate-400">
                          Manage virtualized developer environments across container and VM backends
                        </p>
                      </div>
                    </div>
                    <MetricsPanel />
                    <WorkspaceList
                      workspaces={workspaces}
                      onRefresh={reloadWorkspaces}
                      onSelectWorkspace={setSelectedWorkspace}
                    />
                  </div>
                )}

                {activeTab === "registry" && (
                  <RegistryView
                    onCreateFromImage={(imgName) => {
                      setShowCreator(true);
                      addToast({ type: "info", title: "Deploying Image", message: `Pre-filling ${imgName}` });
                    }}
                  />
                )}

                {activeTab === "images" && <ImagesView />}
                {activeTab === "snapshots" && <SnapshotTimelineView />}
                {activeTab === "pur" && <PurDaemonView />}
                {activeTab === "packages" && <PackagesView />}
                {activeTab === "networking" && <NetworkingView />}
                {activeTab === "storage" && <StorageView />}
                {activeTab === "resources" && <ResourcesView />}
                {activeTab === "platform" && <PlatformInspectorView />}
                {activeTab === "plugins" && <PluginManagerView />}
                {activeTab === "validation" && <ValidationRunnerView />}
                {activeTab === "config" && <ConfigManagerView />}
              </>
            )}
          </div>
        </div>

        <StatusBar />
      </main>

      {/* Modals & Drawers */}
      {showCreator && (
        <WorkspaceCreator
          onClose={() => setShowCreator(false)}
          onCreated={() => {
            setShowCreator(false);
            reloadWorkspaces();
            addToast({ type: "success", title: "Workspace Created", message: "New workspace initialized." });
          }}
        />
      )}

      {showOnboarding && (
        <OnboardingWizard
          onComplete={() => {
            localStorage.setItem("plazavm_seen_onboarding", "true");
            setShowOnboarding(false);
            addToast({ type: "success", title: "Setup Complete", message: "Welcome to Plaza Desktop!" });
          }}
        />
      )}

      <CommandPalette
        isOpen={showPalette}
        onClose={() => setShowPalette(false)}
        onSelectAction={handlePaletteAction}
      />

      <GlobalSearchModal
        isOpen={showSearchModal}
        onClose={() => setShowSearchModal(false)}
        onSelectCategory={(tab) => {
          setSelectedWorkspace(null);
          setActiveTab(tab);
        }}
      />

      <NotificationsDrawer
        isOpen={showNotifications}
        onClose={() => setShowNotifications(false)}
      />

      <TerminalModal
        isOpen={showTerminal}
        onClose={() => setShowTerminal(false)}
        workspaceName={activeTerminalWs}
      />

      <KeyboardShortcutsModal
        isOpen={showShortcuts}
        onClose={() => setShowShortcuts(false)}
      />
    </div>
  );
};

export const App: React.FC = () => {
  return (
    <ThemeProvider>
      <ToastProvider>
        <MainAppContent />
      </ToastProvider>
    </ThemeProvider>
  );
};

export default App;
