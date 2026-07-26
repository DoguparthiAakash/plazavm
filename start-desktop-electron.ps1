Write-Host "========================================================" -ForegroundColor Cyan
Write-Host "[Plaza Desktop] Launching Electron Control Center..." -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan

Set-Location "$PSScriptRoot\plaza-desktop"
npm run build
npx electron .
