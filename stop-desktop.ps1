Write-Host "========================================================" -ForegroundColor Red
Write-Host "[Plaza Desktop] Terminating Plaza Desktop Control Center..." -ForegroundColor Red
Write-Host "========================================================" -ForegroundColor Red

$procs = Get-Process -Name "plaza-desktop" -ErrorAction SilentlyContinue

if ($procs) {
    $procs | Stop-Process -Force
    Write-Host "[SUCCESS] Plaza Desktop process terminated successfully!" -ForegroundColor Green
} else {
    Write-Host "[INFO] No running plaza-desktop process found." -ForegroundColor Yellow
}
