$ErrorActionPreference = "Continue"
$ProjectRoot = (Resolve-Path "$PSScriptRoot\..\..").Path
$InstallerPath = "$ProjectRoot\dist\PlazaVM_Setup.ps1"
$PlazaBinDir = "$env:LOCALAPPDATA\PlazaVM\bin"

Write-Host "=========================================="
Write-Host " PlazaVM End-to-End Installation Test     "
Write-Host "=========================================="

Write-Host "`n[0] Executing Installer..."
if (Test-Path $InstallerPath) {
    powershell -ExecutionPolicy Bypass -File $InstallerPath | Out-Host
} else {
    Write-Host "❌ Error: Installer not found at $InstallerPath"
    exit 1
}

$env:PATH = "$PlazaBinDir;$env:PATH"

# Check if plaza is in PATH
$PlazaCmd = Get-Command "plaza.exe" -ErrorAction SilentlyContinue

if (-not $PlazaCmd) {
    Write-Host "❌ Error: plaza.exe is not in PATH."
    exit 1
}

Write-Host "`n[1] Testing CLI Execution & Diagnostics..."
$DoctorOutput = & plaza.exe doctor
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error: plaza doctor failed with exit code $LASTEXITCODE"
} else {
    Write-Host "✅ plaza doctor executed successfully."
    $DoctorOutput | Out-String | Write-Host
}

Write-Host "`n[2] Testing Workspace Validation Command..."
$ValidateOutput = & plaza.exe workspace validate
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error: plaza workspace validate failed with exit code $LASTEXITCODE"
} else {
    Write-Host "✅ plaza workspace validate executed successfully."
    $ValidateOutput | Out-String | Write-Host
}

Write-Host "`n[3] Testing Workspace Creation..."
$WsCreate = & plaza.exe workspace create test-project-01
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error: plaza workspace create failed."
} else {
    Write-Host "✅ Workspace created."
}

Write-Host "`n[4] Testing Benchmark Suite..."
# Test crash recovery, persistence, isolation
$BenchOutput = & plaza.exe benchmark
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error: plaza benchmark failed."
} else {
    Write-Host "✅ Benchmarks executed."
    $BenchOutput | Out-String | Write-Host
}

Write-Host "`n=========================================="
Write-Host " Validation Complete.                     "
Write-Host "=========================================="
