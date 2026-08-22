$ErrorActionPreference = "Stop"
Write-Host "======================================"
Write-Host " PlazaVM Uninstaller "
Write-Host "======================================"

$InstallDir = "$env:LOCALAPPDATA\PlazaVM"
$BinDir = "$InstallDir\bin"

# Kill processes
Get-Process -Name "plaza" -ErrorAction SilentlyContinue | Stop-Process -Force

# Remove from PATH
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
$PathParts = $UserPath -split ";"
$NewParts = @()
foreach ($part in $PathParts) {
    if ($part -and ($part.Trim() -ne $BinDir)) {
        $NewParts += $part
    }
}
$NewPath = $NewParts -join ";"
if ($NewPath -ne $UserPath) {
    Write-Host "Removing PlazaVM from user PATH..."
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
}

# Remove directory (except itself if running from it, though usually we just force it)
if (Test-Path $InstallDir) {
    Write-Host "Deleting installation directory..."
    Remove-Item -Recurse -Force $InstallDir -ErrorAction SilentlyContinue
}
Write-Host "PlazaVM uninstalled successfully."
Start-Sleep -Seconds 3
