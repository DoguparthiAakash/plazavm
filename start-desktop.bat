@echo off
title Plaza Desktop Control Center Launcher
echo ========================================================
echo [Plaza Desktop] Launching Plaza Desktop Control Center...
echo ========================================================
cd /d "%~dp0"

if exist "%~dp0target\debug\plaza-desktop.exe" (
    start "" "%~dp0target\debug\plaza-desktop.exe"
    echo [SUCCESS] Plaza Desktop launched successfully from target\debug\plaza-desktop.exe!
) else (
    echo [NOTICE] Target executable not found. Launching via cargo run...
    cargo run -p plaza-desktop
)
