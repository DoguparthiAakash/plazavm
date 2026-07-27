# File Tree: plazavm

**Generated:** 7/26/2026, 12:01:27 AM
**Root Path:** `e:\plazavm`

```
├── .devcontainer
│   └── devcontainer.json
├── .github
│   └── workflows
│       └── ci.yml
├── assets
│   └── icons
│       └── plaza_icon.svg
├── benchmarks
│   └── baseline
│       └── v2_0_baseline.json
├── docs
│   ├── adr
│   │   ├── 0001-five-layer-architecture.md
│   │   ├── 0002-composition-root.md
│   │   ├── 0003-event-driven-controller.md
│   │   ├── ADR-0001_workspace_first_computing.md
│   │   └── README.md
│   ├── architecture
│   │   ├── diagrams
│   │   │   └── README.md
│   │   └── overview.md
│   ├── concepts
│   │   ├── intent-model.md
│   │   └── workspace-graph.md
│   ├── getting-started
│   │   └── quickstart.md
│   ├── plugins
│   │   └── development.md
│   ├── rfc
│   │   └── RFC-0001_plaza_rfc_process.md
│   ├── security
│   │   └── audit.md
│   ├── specifications
│   │   └── PS-0001_workspace_specification.md
│   ├── standards
│   │   └── PST-0001_naming_standard.md
│   ├── validation
│   │   └── framework.md
│   ├── README.md
│   ├── dp1_certification_report.md
│   └── security_audit.md
├── examples
│   ├── ai-development
│   │   └── README.md
│   ├── minimal-workspace
│   │   ├── README.md
│   │   └── plaza.yaml
│   ├── validation
│   │   └── README.md
│   └── virtualbox
│       └── README.md
├── fixtures
│   ├── configs
│   │   └── sample_workspace.yaml
│   ├── platform
│   │   └── hardware_scan.json
│   └── workspace
│       └── default_spec.json
├── plaza-ai
│   ├── src
│   │   ├── advisor.rs
│   │   ├── lib.rs
│   │   └── provider.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-api
│   ├── src
│   │   ├── bootstrap.rs
│   │   ├── diagnostics.rs
│   │   ├── dto.rs
│   │   ├── lib.rs
│   │   ├── state.rs
│   │   └── updater.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-builder
│   └── src
│       └── importer.rs
├── plaza-cli
│   ├── src
│   │   ├── validator
│   │   │   ├── dashboard.rs
│   │   │   ├── evidence.rs
│   │   │   ├── mod.rs
│   │   │   ├── reporter.rs
│   │   │   ├── runner.rs
│   │   │   └── stages.rs
│   │   ├── main.rs
│   │   └── shell.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-config
│   ├── src
│   │   ├── app_config.rs
│   │   ├── lib.rs
│   │   ├── manager.rs
│   │   └── workspace_config.rs
│   ├── tests
│   │   └── unit_tests.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-controller
│   ├── src
│   │   ├── lib.rs
│   │   ├── reconciler.rs
│   │   └── recovery.rs
│   ├── tests
│   │   └── integration_tests.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-core
│   ├── src
│   │   ├── error.rs
│   │   ├── id.rs
│   │   ├── lib.rs
│   │   ├── logging.rs
│   │   ├── object_model.rs
│   │   ├── panic_handler.rs
│   │   ├── paths.rs
│   │   ├── puri.rs
│   │   ├── security.rs
│   │   └── types.rs
│   ├── tests
│   │   └── unit_tests.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-decision
│   ├── src
│   │   ├── engine.rs
│   │   ├── intent.rs
│   │   ├── lib.rs
│   │   └── scoring.rs
│   ├── tests
│   │   └── decision_matrix.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-desktop
│   ├── electron
│   │   ├── main.cjs
│   │   └── preload.cjs
│   ├── src
│   │   ├── components
│   │   │   ├── ui
│   │   │   │   ├── EmptyState.tsx
│   │   │   │   ├── TerminalModal.tsx
│   │   │   │   ├── ThemeContext.tsx
│   │   │   │   └── Toast.tsx
│   │   │   ├── AiAssistantView.tsx
│   │   │   ├── CommandPalette.tsx
│   │   │   ├── ConfigManagerView.tsx
│   │   │   ├── FullWorkspaceDetailView.tsx
│   │   │   ├── GlobalSearchModal.tsx
│   │   │   ├── HomeDashboardView.tsx
│   │   │   ├── ImagesView.tsx
│   │   │   ├── Inspector.tsx
│   │   │   ├── KeyboardShortcutsModal.tsx
│   │   │   ├── MetricsPanel.tsx
│   │   │   ├── NetworkingView.tsx
│   │   │   ├── NotificationsDrawer.tsx
│   │   │   ├── OnboardingWizard.tsx
│   │   │   ├── PackagesView.tsx
│   │   │   ├── PlatformInfo.tsx
│   │   │   ├── PlatformInspectorView.tsx
│   │   │   ├── PluginManagerView.tsx
│   │   │   ├── PurDaemonView.tsx
│   │   │   ├── RegistryView.tsx
│   │   │   ├── ResourcesView.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   ├── SnapshotTimelineView.tsx
│   │   │   ├── StatusBar.tsx
│   │   │   ├── StorageView.tsx
│   │   │   ├── TopBar.tsx
│   │   │   ├── ValidationRunnerView.tsx
│   │   │   ├── WorkspaceCard.tsx
│   │   │   ├── WorkspaceCreator.tsx
│   │   │   └── WorkspaceList.tsx
│   │   ├── App.tsx
│   │   ├── api.ts
│   │   ├── index.css
│   │   └── main.tsx
│   ├── src-tauri
│   │   ├── icons
│   │   │   ├── 128x128.png
│   │   │   ├── 128x128@2x.png
│   │   │   ├── 32x32.png
│   │   │   ├── icon.icns
│   │   │   └── icon.ico
│   │   ├── src
│   │   │   ├── commands.rs
│   │   │   ├── lib.rs
│   │   │   └── main.rs
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   └── tauri.conf.json
│   ├── README.md
│   ├── index.html
│   ├── package-lock.json
│   ├── package.json
│   ├── postcss.config.js
│   ├── tailwind.config.js
│   ├── tsconfig.json
│   └── vite.config.ts
├── plaza-docs
│   ├── app
│   │   ├── docs
│   │   │   ├── architecture
│   │   │   │   └── page.tsx
│   │   │   ├── cli
│   │   │   │   └── page.tsx
│   │   │   ├── concepts
│   │   │   │   ├── puri
│   │   │   │   │   └── page.tsx
│   │   │   │   ├── workspace-first
│   │   │   │   │   └── page.tsx
│   │   │   │   └── page.tsx
│   │   │   ├── crates
│   │   │   │   └── page.tsx
│   │   │   ├── getting-started
│   │   │   │   ├── quickstart
│   │   │   │   │   └── page.tsx
│   │   │   │   └── page.tsx
│   │   │   ├── specifications
│   │   │   │   └── page.tsx
│   │   │   ├── layout.tsx
│   │   │   └── page.tsx
│   │   ├── globals.css
│   │   ├── layout.tsx
│   │   └── page.tsx
│   ├── components
│   │   ├── Footer.tsx
│   │   ├── Header.tsx
│   │   ├── PageNav.tsx
│   │   ├── ProgramizCodeBox.tsx
│   │   ├── ProgramizTakeaway.tsx
│   │   ├── RightToc.tsx
│   │   └── Sidebar.tsx
│   ├── next-env.d.ts
│   ├── next.config.mjs
│   ├── package-lock.json
│   ├── package.json
│   ├── postcss.config.js
│   ├── tailwind.config.ts
│   └── tsconfig.json
├── plaza-events
│   ├── src
│   │   ├── bus.rs
│   │   ├── events.rs
│   │   └── lib.rs
│   ├── tests
│   │   └── unit_tests.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-foundation
│   ├── src
│   │   ├── engine
│   │   │   ├── coordinators
│   │   │   │   ├── configuration.rs
│   │   │   │   ├── events.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── networking.rs
│   │   │   │   ├── resource.rs
│   │   │   │   ├── runtime.rs
│   │   │   │   ├── security.rs
│   │   │   │   ├── storage.rs
│   │   │   │   └── workspace.rs
│   │   │   ├── bootstrap.rs
│   │   │   ├── container.rs
│   │   │   ├── core.rs
│   │   │   ├── diagnostics.rs
│   │   │   ├── errors.rs
│   │   │   ├── health.rs
│   │   │   ├── lifecycle.rs
│   │   │   ├── metrics.rs
│   │   │   ├── mod.rs
│   │   │   ├── recovery.rs
│   │   │   ├── registry.rs
│   │   │   ├── scheduler.rs
│   │   │   └── state.rs
│   │   ├── lib.rs
│   │   ├── protocol.rs
│   │   └── registry.rs
│   └── Cargo.toml
├── plaza-monitor
│   ├── src
│   │   ├── lib.rs
│   │   └── system.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-platform
│   ├── src
│   │   ├── capabilities.rs
│   │   ├── detector.rs
│   │   ├── gpu.rs
│   │   ├── kal.rs
│   │   ├── lib.rs
│   │   ├── pal.rs
│   │   ├── pro_adapter.rs
│   │   ├── profile.rs
│   │   ├── pur_adapter.rs
│   │   ├── runtime_detection.rs
│   │   └── scoring.rs
│   ├── tests
│   │   └── platform_tests.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-plugin
│   ├── src
│   │   ├── host.rs
│   │   ├── lib.rs
│   │   └── manifest.rs
│   ├── tests
│   │   └── plugin_validation_tests.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-registry
│   ├── src
│   │   ├── importer.rs
│   │   ├── lib.rs
│   │   ├── pro_image.rs
│   │   ├── pur_image.rs
│   │   ├── runtime_images.rs
│   │   └── templates.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-resource
│   ├── src
│   │   ├── lib.rs
│   │   ├── manager.rs
│   │   ├── priority.rs
│   │   └── vhal.rs
│   ├── tests
│   │   └── unit_tests.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-runtime
│   ├── src
│   │   ├── backend.rs
│   │   ├── capabilities.rs
│   │   ├── instance.rs
│   │   └── lib.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-storage
│   ├── src
│   │   ├── event_store.rs
│   │   ├── lib.rs
│   │   ├── migrations.rs
│   │   └── repository.rs
│   ├── tests
│   │   └── recovery_tests.rs
│   ├── Cargo.toml
│   └── README.md
├── plaza-workspace
│   ├── src
│   │   ├── builder.rs
│   │   ├── capability.rs
│   │   ├── graph.rs
│   │   ├── lib.rs
│   │   ├── memory.rs
│   │   ├── model.rs
│   │   ├── pipeline.rs
│   │   ├── process.rs
│   │   ├── service.rs
│   │   ├── service_manager.rs
│   │   ├── session.rs
│   │   └── wsc.rs
│   ├── tests
│   │   └── unit_tests.rs
│   ├── Cargo.toml
│   └── README.md
├── plugins
│   ├── docker
│   │   ├── src
│   │   │   └── lib.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── hyperv
│   │   ├── src
│   │   │   └── lib.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── podman
│   │   ├── src
│   │   │   └── lib.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── qemu
│   │   ├── src
│   │   │   └── lib.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   └── virtualbox
│       ├── src
│       │   └── lib.rs
│       ├── Cargo.toml
│       └── README.md
├── release
│   ├── manifests
│   │   ├── update_feed.json
│   │   └── v0.1.0-dp1.json
│   ├── notes
│   │   └── v2.0.0.md
│   └── build_packages.ps1
├── schemas
│   ├── manifest.schema.json
│   ├── plaza.schema.json
│   ├── plugin.schema.json
│   ├── runtime.schema.json
│   └── workspace.schema.json
├── test
├── tests
│   ├── e2e
│   │   └── workspace_e2e.rs
│   └── integration
│       └── platform_integration.rs
├── tools
│   └── validator
│       └── README.md
├── .editorconfig
├── .gitignore
├── .pre-commit-config.yaml
├── CHANGELOG.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── ROADMAP.md
├── SECURITY.md
├── SUPPORT.md
├── mkdocs.yml
├── package-lock.json
├── package.json
├── start-desktop-electron.bat
├── start-desktop-electron.ps1
├── start-desktop.bat
├── start-desktop.ps1
├── stop-desktop-electron.bat
├── stop-desktop-electron.ps1
├── stop-desktop.bat
└── stop-desktop.ps1
```

---
*Generated by FileTree Pro Extension*