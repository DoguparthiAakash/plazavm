# PlazaVM Windows Installation Guide

This guide details how to build and install PlazaVM (CLI and Desktop GUI) on Windows using the official Inno Setup installer package.

## Prerequisites

To build the installer from source, you will need:
- **Rust Toolchain**: `cargo` and `rustc` (used to build the PlazaVM CLI).
- **Node.js**: `npm` (used to build the Tauri Desktop application).
- **Inno Setup**: Version 6+ (used to package the binaries). Ensure `iscc.exe` is in your PATH or configure the `INNO_SETUP_PATH` environment variable.

## Build Process

PlazaVM includes an automated PowerShell script to handle compilation, staging, and packaging.

Open a PowerShell terminal at the root of the repository and run:
```powershell
.\build_installer.ps1
```

The script will:
1. Validate prerequisites.
2. Detect the authoritative version from `Cargo.toml`.
3. Build the CLI binary in Release mode.
4. Build the Desktop application via Tauri in Release mode.
5. Stage the required artifacts in `build/installer/windows/staging`.
6. Invoke the Inno Setup compiler (`iscc.exe`) to generate the final installer.

### Output
The final installer will be generated at:
```text
release/PlazaVM_Setup.exe
```

## Installation

Run `PlazaVM_Setup.exe`. By default, the installer performs a **per-user** installation without requiring elevated administrator privileges (unless you explicitly change the installation directory to a protected location).

### PATH Integration
The installer automatically appends the PlazaVM installation directory to your **Current User PATH**. It does not overwrite or duplicate existing PATH entries. 

> [!NOTE]
> Open a **new** terminal window after installation to ensure the updated PATH is loaded.

## Application Usage

### PlazaVM CLI
From any new terminal, you can interact with PlazaVM using the `plaza` command:
```powershell
plaza --help
plaza workspace validate
plaza workspace start
```

### PlazaVM Desktop
You can launch the GUI using the **Start Menu shortcut** (`PlazaVM Desktop`) or the optional Desktop shortcut created during installation.

## Uninstall & Upgrade

### Upgrade
To upgrade, simply run the newer `PlazaVM_Setup.exe`. The installer will safely replace the application binaries while preserving all of your user data, workspaces, configuration (`plaza.yaml`), and VM images.

### Uninstall
You can uninstall PlazaVM via **Add or Remove Programs**. 
The uninstaller will cleanly remove the application binaries, shortcuts, and the specific PlazaVM PATH entry. **User data is intentionally preserved** and will not be deleted during uninstallation.

## Troubleshooting

- **SmartScreen Warning**: Because the installer is built locally and may be unsigned, Windows SmartScreen might display a warning. Click "More info" and then "Run anyway" if you built the installer yourself.
- **Tauri WebView2**: The desktop application relies on Microsoft WebView2 (which is pre-installed on modern Windows 10/11 systems). If the application fails to launch with a blank screen, ensure WebView2 is installed on your system.
