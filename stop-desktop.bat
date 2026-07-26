@echo off
title Plaza Desktop Control Center Terminator
echo ========================================================
echo [Plaza Desktop] Terminating Plaza Desktop Control Center...
echo ========================================================
taskkill /F /IM plaza-desktop.exe 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [SUCCESS] Plaza Desktop process terminated successfully.
) else (
    echo [INFO] No active plaza-desktop.exe process found.
)
