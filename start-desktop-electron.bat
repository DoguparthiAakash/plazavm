@echo off
title Plaza Desktop Control Center (Electron)
echo ========================================================
echo [Plaza Desktop] Launching Electron Control Center...
echo ========================================================
cd /d "%~dp0plaza-desktop"

npm run build && npx electron .
