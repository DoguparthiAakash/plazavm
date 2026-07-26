Write-Host "========================================================" -ForegroundColor Cyan
Write-Host "[Plaza Desktop] Launching Plaza Desktop Control Center..." -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan

$exePath = Join-Path $PSScriptRoot "target\debug\plaza-desktop.exe"

if (Test-Path $exePath) {
    Start-Process $exePath
    Write-Host "[SUCCESS] Plaza Desktop process started successfully!" -ForegroundColor Green
} else {
    Write-Host "[NOTICE] Target executable not found. Launching via cargo..." -ForegroundColor Yellow
    cargo run -p plaza-desktop
}
