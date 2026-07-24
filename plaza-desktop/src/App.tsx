import React, { useState, useEffect } from "react";
import { Sidebar } from "./components/Sidebar";
import { WorkspaceList } from "./components/WorkspaceList";
import { WorkspaceCreator } from "./components/WorkspaceCreator";
import { MetricsPanel } from "./components/MetricsPanel";
import { PlatformInspectorView } from "./components/PlatformInspectorView";
import { PluginManagerView } from "./components/PluginManagerView";
import { ValidationRunnerView } from "./components/ValidationRunnerView";
import { ConfigManagerView } from "./components/ConfigManagerView";
import { OnboardingWizard } from "./components/OnboardingWizard";
import { CommandPalette } from "./components/CommandPalette";
import { StatusBar } from "./components/StatusBar";
import { KeyboardShortcutsModal } from "./components/KeyboardShortcutsModal";
import { fetchWorkspaces, WorkspaceDto } from "./api";

export const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState("workspaces");
  const [workspaces, setWorkspaces] = useState<WorkspaceDto[]>([]);
  const [showCreator, setShowCreator] = useState(false);
  const [showOnboarding, setShowOnboarding] = useState(false);
  const [showPalette, setShowPalette] = useState(false);
  const [showShortcuts, setShowShortcuts] = useState(false);

  const reloadWorkspaces = () => {
    fetchWorkspaces().then(setWorkspaces);
  };

  useEffect(() => {
    reloadWorkspaces();
    const hasSeenOnboarding = localStorage.getItem("plazavm_seen_onboarding");
    if (!hasSeenOnboarding) {
      setShowOnboarding(true);
    }
  }, []);

  const handleCompleteOnboarding = () => {
    localStorage.setItem("plazavm_seen_onboarding", "true");
    setShowOnboarding(false);
  };

  const handlePaletteAction = (action: string) => {
    if (action === "toggle-palette") {
      setShowPalette((prev) => !prev);
    } else if (action === "workspaces") {
      setActiveTab("workspaces");
    } else if (action === "platform") {
      setActiveTab("platform");
    } else if (action === "plugins") {
      setActiveTab("plugins");
    } else if (action === "validation") {
      setActiveTab("validation");
    } else if (action === "config") {
      setActiveTab("config");
    }
  };

  return (
    <div className="flex h-screen w-screen bg-slate-950 text-slate-100 font-sans overflow-hidden">
      <Sidebar
        activeTab={activeTab}
        onTabChange={setActiveTab}
        onCreateWorkspace={() => setShowCreator(true)}
        onOpenShortcuts={() => setShowShortcuts(true)}
      />

      <main className="flex-1 flex flex-col min-w-0 bg-slate-950">
        <div className="flex-1 overflow-y-auto">
          {activeTab === "workspaces" && (
            <div className="p-6 max-w-7xl mx-auto space-y-6">
              <div className="flex items-center justify-between">
                <div>
                  <h2 className="text-2xl font-bold text-slate-100">Universal Workspaces</h2>
                  <p className="text-xs text-slate-400">Manage virtualized developer environments across container and VM backends</p>
                </div>
              </div>
              <MetricsPanel />
              <WorkspaceList workspaces={workspaces} onRefresh={reloadWorkspaces} />
            </div>
          )}

          {activeTab === "platform" && <PlatformInspectorView />}
          {activeTab === "plugins" && <PluginManagerView />}
          {activeTab === "validation" && <ValidationRunnerView />}
          {activeTab === "config" && <ConfigManagerView />}
        </div>

        <StatusBar />
      </main>

      {showCreator && (
        <WorkspaceCreator
          onClose={() => setShowCreator(false)}
          onCreated={() => {
            setShowCreator(false);
            reloadWorkspaces();
          }}
        />
      )}

      {showOnboarding && <OnboardingWizard onComplete={handleCompleteOnboarding} />}

      <CommandPalette
        isOpen={showPalette}
        onClose={() => setShowPalette(false)}
        onSelectAction={handlePaletteAction}
      />

      <KeyboardShortcutsModal
        isOpen={showShortcuts}
        onClose={() => setShowShortcuts(false)}
      />
    </div>
  );
};

export default App;
