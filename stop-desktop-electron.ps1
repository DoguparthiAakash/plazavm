Write-Host "========================================================" -ForegroundColor Red
Write-Host "[Plaza Desktop] Terminating Electron Control Center..." -ForegroundColor Red
Write-Host "========================================================" -ForegroundColor Red

$procs = Get-Process -Name "electron" -ErrorAction SilentlyContinue

if ($procs) {
    $procs | Stop-Process -Force
    Write-Host "[SUCCESS] Electron process terminated successfully!" -ForegroundColor Green
} else {
    Write-Host "[INFO] No running electron process found." -ForegroundColor Yellow
}
