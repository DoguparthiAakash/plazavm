@echo off
title Plaza Desktop Control Center Terminator (Electron)
echo ========================================================
echo [Plaza Desktop] Terminating Electron Control Center...
echo ========================================================
taskkill /F /IM electron.exe 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [SUCCESS] Electron process terminated successfully.
) else (
    echo [INFO] No active electron.exe process found.
)
