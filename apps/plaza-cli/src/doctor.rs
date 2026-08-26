use plaza_foundation::core::paths;
use std::path::PathBuf;
use std::process::Command;

pub struct DoctorReport {
    pub category: String,
    pub checks: Vec<CheckResult>,
}

pub struct CheckResult {
    pub name: String,
    pub status: CheckStatus,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
    Blocked,
    Unsupported,
}

impl std::fmt::Display for CheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckStatus::Pass => write!(f, "[PASS]"),
            CheckStatus::Warn => write!(f, "[WARN]"),
            CheckStatus::Fail => write!(f, "[FAIL]"),
            CheckStatus::Blocked => write!(f, "[BLOCKED]"),
            CheckStatus::Unsupported => write!(f, "[UNSUPPORTED]"),
        }
    }
}

pub fn run_diagnostics() -> Vec<DoctorReport> {
    vec![
        check_system(),
        check_plazavm(),
        check_qemu(),
        check_storage(),
        check_wasm(),
        check_security(),
    ]
}

fn check_system() -> DoctorReport {
    let mut checks = Vec::new();

    // Windows OS Check
    #[cfg(target_os = "windows")]
    checks.push(CheckResult {
        name: "OS Environment".to_string(),
        status: CheckStatus::Pass,
        message: "Windows execution environment detected".to_string(),
    });

    #[cfg(not(target_os = "windows"))]
    checks.push(CheckResult {
        name: "OS Environment".to_string(),
        status: CheckStatus::Warn,
        message: "Non-Windows environment detected (installer targets Windows)".to_string(),
    });

    // Architecture
    let arch = std::env::consts::ARCH;
    checks.push(CheckResult {
        name: "CPU Architecture".to_string(),
        status: if arch == "x86_64" {
            CheckStatus::Pass
        } else {
            CheckStatus::Warn
        },
        message: format!("Architecture: {}", arch),
    });

    DoctorReport {
        category: "System".to_string(),
        checks,
    }
}

fn check_plazavm() -> DoctorReport {
    let mut checks = Vec::new();

    // CLI Exe Path
    if let Ok(exe_path) = std::env::current_exe() {
        checks.push(CheckResult {
            name: "PlazaVM Executable".to_string(),
            status: CheckStatus::Pass,
            message: format!("Location: {}", exe_path.display()),
        });
    } else {
        checks.push(CheckResult {
            name: "PlazaVM Executable".to_string(),
            status: CheckStatus::Fail,
            message: "Unable to determine current executable path".to_string(),
        });
    }

    // Config Path
    let config_dir = paths::config_dir();
    checks.push(CheckResult {
        name: "Configuration Directory".to_string(),
        status: CheckStatus::Pass,
        message: format!("Path: {}", config_dir.display()),
    });

    DoctorReport {
        category: "PlazaVM".to_string(),
        checks,
    }
}

fn check_qemu() -> DoctorReport {
    let mut checks = Vec::new();

    // Check qemu-system-x86_64
    let qemu_cmd = if cfg!(target_os = "windows") {
        "qemu-system-x86_64.exe"
    } else {
        "qemu-system-x86_64"
    };

    // Try to run `qemu-system-x86_64 --version`
    match Command::new(qemu_cmd).arg("--version").output() {
        Ok(output) if output.status.success() => {
            let version_output = String::from_utf8_lossy(&output.stdout);
            let version = version_output
                .lines()
                .next()
                .unwrap_or("Unknown version")
                .to_string();

            checks.push(CheckResult {
                name: "QEMU-TCG".to_string(),
                status: CheckStatus::Pass,
                message: format!("Resolved in PATH. {}", version),
            });
        }
        Ok(output) => {
            checks.push(CheckResult {
                name: "QEMU-TCG".to_string(),
                status: CheckStatus::Fail,
                message: format!("Execution failed with status: {}", output.status),
            });
        }
        Err(e) => {
            checks.push(CheckResult {
                name: "QEMU-TCG".to_string(),
                status: CheckStatus::Fail,
                message: format!("Not found in PATH or failed to execute: {}. (Please install QEMU or add it to PATH)", e),
            });
        }
    }

    DoctorReport {
        category: "Runtime".to_string(),
        checks,
    }
}

fn check_storage() -> DoctorReport {
    let mut checks = Vec::new();

    // Image Directory
    let images_dir = paths::images_dir();
    checks.push(CheckResult {
        name: "Image Storage".to_string(),
        status: CheckStatus::Pass,
        message: format!("Location: {}", images_dir.display()),
    });

    // Workspace Directory
    let ws_dir = paths::workspaces_dir();
    if std::fs::create_dir_all(&ws_dir).is_ok() {
        let test_file = ws_dir.join(".write_test");
        if std::fs::write(&test_file, b"test").is_ok() {
            let _ = std::fs::remove_file(test_file);
            checks.push(CheckResult {
                name: "Workspace Storage".to_string(),
                status: CheckStatus::Pass,
                message: format!("Writable: {}", ws_dir.display()),
            });
        } else {
            checks.push(CheckResult {
                name: "Workspace Storage".to_string(),
                status: CheckStatus::Fail,
                message: format!("Not writable: {}", ws_dir.display()),
            });
        }
    } else {
        checks.push(CheckResult {
            name: "Workspace Storage".to_string(),
            status: CheckStatus::Fail,
            message: format!("Failed to create directory: {}", ws_dir.display()),
        });
    }

    DoctorReport {
        category: "Storage".to_string(),
        checks,
    }
}

fn check_wasm() -> DoctorReport {
    let mut checks = Vec::new();

    // Check for v86.wasm
    // We expect it either relative to current exe (e.g. ../wasm/v86.wasm) or in installation path.
    let mut found = false;
    let mut check_msg = "v86.wasm not found. v86-WASM backend unavailable.".to_string();

    if let Ok(exe_dir) = std::env::current_exe().map(|p| p.parent().unwrap().to_path_buf()) {
        let possible_paths = vec![
            exe_dir.join("v86.wasm"),
            exe_dir.join("../wasm/v86.wasm"),
            exe_dir.join("../../wasm/v86.wasm"),
        ];

        for p in possible_paths {
            if p.exists() {
                found = true;
                check_msg = format!("Found v86.wasm at {}", p.display());
                break;
            }
        }
    }

    checks.push(CheckResult {
        name: "v86 WASM".to_string(),
        status: if found {
            CheckStatus::Warn
        } else {
            CheckStatus::Warn
        }, // As per instructions, report WARN if incomplete
        message: if found {
            format!("{} (v86 support is currently incomplete)", check_msg)
        } else {
            check_msg
        },
    });

    DoctorReport {
        category: "WASM".to_string(),
        checks,
    }
}

fn check_security() -> DoctorReport {
    let mut checks = Vec::new();

    checks.push(CheckResult {
        name: "Privileged Virtualization".to_string(),
        status: CheckStatus::Pass,
        message: "PlazaVM operates entirely in userspace (No KVM/WHPX/Hyper-V required)"
            .to_string(),
    });

    checks.push(CheckResult {
        name: "Userspace NBD".to_string(),
        status: CheckStatus::Pass,
        message: "No kernel NBD or loopback drivers required".to_string(),
    });

    DoctorReport {
        category: "Security".to_string(),
        checks,
    }
}

pub fn print_report() {
    println!("PlazaVM Doctor");
    println!("=============\n");

    let reports = run_diagnostics();
    for report in reports {
        println!("### {}", report.category);
        for check in report.checks {
            println!("{} {}: {}", check.status, check.name, check.message);
        }
        println!();
    }
}
