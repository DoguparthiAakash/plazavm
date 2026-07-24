import React, { useEffect, useState } from "react";
import { checkSystemReadiness } from "../api";
import { CheckCircle2, XCircle, Wrench, ArrowRight } from "lucide-react";

interface OnboardingProps {
  onComplete: () => void;
}

export const OnboardingWizard: React.FC<OnboardingProps> = ({ onComplete }) => {
  const [readiness, setReadiness] = useState<Record<string, boolean>>({});
  const [step, setStep] = useState(1);

  useEffect(() => {
    checkSystemReadiness().then(setReadiness);
  }, []);

  const items = [
    { label: "Docker Container Runtime", key: "docker_installed", desc: "For containerized workspaces" },
    { label: "VirtualBox Hypervisor", key: "virtualbox_installed", desc: "For full x86_64 guest VMs" },
    { label: "QEMU Emulator", key: "qemu_installed", desc: "For cross-architecture emulation" },
    { label: "Podman Rootless Engine", key: "podman_installed", desc: "For rootless container workloads" },
    { label: "Hyper-V Hypervisor", key: "hyperv_available", desc: "Windows native hypervisor" },
    { label: "Rust Toolchain", key: "rust_installed", desc: "PlazaVM compilation environment" },
    { label: "Git VCS", key: "git_installed", desc: "Workspace repository sync" },
    { label: "Node.js Runtime", key: "node_installed", desc: "Desktop shell & frontend tooling" },
  ];

  return (
    <div className="fixed inset-0 bg-slate-950/90 backdrop-blur-md z-50 flex items-center justify-center p-6">
      <div className="bg-slate-900 border border-slate-800 rounded-xl max-w-2xl w-full p-8 shadow-2xl">
        <div className="flex items-center gap-3 mb-6">
          <div className="p-3 bg-cyan-500/10 text-cyan-400 rounded-lg">
            <Wrench className="w-6 h-6" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-slate-100">Welcome to PlazaVM Developer Preview (DP1)</h2>
            <p className="text-sm text-slate-400">First-Run Host System Readiness & Readiness Inspection</p>
          </div>
        </div>

        {step === 1 && (
          <div>
            <p className="text-sm text-slate-300 mb-6">
              PlazaVM abstracts multiple execution technologies behind a unified workspace model. Below is an audit of detected runtimes and system tools on your host:
            </p>

            <div className="grid grid-cols-2 gap-3 mb-8">
              {items.map((item) => {
                const ready = readiness[item.key] ?? false;
                return (
                  <div key={item.key} className="flex items-start gap-3 p-3 bg-slate-800/50 rounded-lg border border-slate-800">
                    {ready ? (
                      <CheckCircle2 className="w-5 h-5 text-emerald-400 shrink-0 mt-0.5" />
                    ) : (
                      <XCircle className="w-5 h-5 text-slate-500 shrink-0 mt-0.5" />
                    )}
                    <div>
                      <div className="text-xs font-semibold text-slate-200">{item.label}</div>
                      <div className="text-[11px] text-slate-400">{item.desc}</div>
                    </div>
                  </div>
                );
              })}
            </div>

            <div className="flex justify-end gap-3">
              <button
                onClick={() => setStep(2)}
                className="flex items-center gap-2 px-5 py-2.5 bg-cyan-600 hover:bg-cyan-500 text-white font-medium text-sm rounded-lg transition"
              >
                Next Step <ArrowRight className="w-4 h-4" />
              </button>
            </div>
          </div>
        )}

        {step === 2 && (
          <div>
            <h3 className="text-lg font-semibold text-slate-100 mb-2">Ready for Developer Preview Testing</h3>
            <p className="text-sm text-slate-300 mb-6">
              Your environment is initialized. In DP1, PlazaVM runs in **Dry-Run Readiness Mode** allowing you to test workspace specifications, intent resolutions, platform inspection, plugin management, and diagnostic bundle generation.
            </p>

            <div className="p-4 bg-cyan-950/30 border border-cyan-800/50 rounded-lg mb-8 text-xs text-cyan-300">
              📌 <strong>Developer Tip:</strong> Press <code className="bg-cyan-900/50 px-1.5 py-0.5 rounded text-cyan-200">Ctrl+K</code> or <code className="bg-cyan-900/50 px-1.5 py-0.5 rounded text-cyan-200">Cmd+K</code> anywhere to open the instant Command Palette.
            </div>

            <div className="flex justify-end">
              <button
                onClick={onComplete}
                className="px-6 py-2.5 bg-emerald-600 hover:bg-emerald-500 text-white font-semibold text-sm rounded-lg shadow-lg shadow-emerald-900/20 transition"
              >
                Get Started with PlazaVM
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
