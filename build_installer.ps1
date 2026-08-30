<#
.SYNOPSIS
Builds and packages the PlazaVM Windows Installer using Inno Setup.

.DESCRIPTION
This script builds the PlazaVM CLI and Desktop GUI applications,
stages them in a temporary directory, and executes Inno Setup compiler
to generate a final PlazaVM_Setup.exe.
#>

$ErrorActionPreference = "Stop"

Write-Host "[1/6] Validating prerequisites..." -ForegroundColor Cyan

# 1. Validate Prereqs
$cargo = Get-Command "cargo" -ErrorAction SilentlyContinue
if (-not $cargo) {
    # Check if cargo is available directly at the common path
    $cargoPath = "$env:USERPROFILE\.cargo\bin\cargo.exe"
    if (Test-Path $cargoPath) {
        $cargo = $cargoPath
    } else {
        Write-Error "cargo was not found. Please install Rust/Cargo."
        exit 1
    }
}

$npm = Get-Command "npm" -ErrorAction SilentlyContinue
if (-not $npm) {
    Write-Error "npm was not found. Please install Node.js."
    exit 1
}

$iscc = $null
$isccCmd = Get-Command "iscc" -ErrorAction SilentlyContinue
if ($isccCmd) {
    $iscc = $isccCmd.Source
} elseif ($env:INNO_SETUP_PATH -and (Test-Path "$env:INNO_SETUP_PATH\iscc.exe")) {
    $iscc = "$env:INNO_SETUP_PATH\iscc.exe"
} else {
    # Check common locations
    $commonPath = "C:\Program Files (x86)\Inno Setup 6\iscc.exe"
    if (Test-Path $commonPath) {
        $iscc = $commonPath
    } else {
        Write-Error "Inno Setup (iscc.exe) was not found. Please install Inno Setup 6 or configure INNO_SETUP_PATH."
        exit 1
    }
}
Write-Host "OK. Using Inno Setup: $iscc" -ForegroundColor Green

# 2. Determine version
Write-Host "`n[2/6] Detecting PlazaVM version..." -ForegroundColor Cyan
$manifestPath = Join-Path $PSScriptRoot "Cargo.toml"
if (-not (Test-Path $manifestPath)) {
    Write-Error "Cargo.toml not found at root."
    exit 1
}
$versionLine = Get-Content $manifestPath | Where-Object { $_ -match '^version\s*=\s*"([^"]+)"' } | Select-Object -First 1
if ($versionLine -match '^version\s*=\s*"([^"]+)"') {
    $AppVersion = $matches[1]
    # InnoSetup strictly requires X.Y.Z or X.Y.Z.W format. Remove alpha/beta/dp tags for the AppVersion property, but we can pass AppVersion full.
    # However, let's keep the full version for Display, but create a numeric one if needed.
} else {
    Write-Error "Could not determine version from Cargo.toml workspace package configuration."
    exit 1
}
Write-Host "OK. Version: $AppVersion" -ForegroundColor Green

# 3. Build CLI
Write-Host "`n[3/6] Building PlazaVM CLI..." -ForegroundColor Cyan
Push-Location $PSScriptRoot
& $cargo build --release --bin plaza-cli
$cliExit = $LASTEXITCODE
Pop-Location
if ($cliExit -ne 0) {
    Write-Error "CLI build failed."
    exit 1
}
$cliArtifact = Join-Path $PSScriptRoot "target\release\plaza-cli.exe"
if (-not (Test-Path $cliArtifact)) {
    Write-Error "CLI artifact not found at $cliArtifact"
    exit 1
}
Write-Host "OK. CLI built." -ForegroundColor Green

# 4. Build Desktop
Write-Host "`n[4/6] Building PlazaVM Desktop..." -ForegroundColor Cyan
$desktopDir = Join-Path $PSScriptRoot "apps\plaza-desktop"
Push-Location $desktopDir
Write-Host "Running npm install..."
& $npm install
if ($LASTEXITCODE -ne 0) { Write-Error "npm install failed."; exit 1 }

Write-Host "Running Tauri build..."
& $npm run tauri build
$desktopExit = $LASTEXITCODE
Pop-Location
if ($desktopExit -ne 0) {
    Write-Error "Desktop build failed."
    exit 1
}

$desktopArtifact = Join-Path $PSScriptRoot "target\release\plaza-desktop.exe"
if (-not (Test-Path $desktopArtifact)) {
    $desktopArtifact = Join-Path $PSScriptRoot "target\release\PlazaVM.exe"
    if (-not (Test-Path $desktopArtifact)) {
        Write-Error "Desktop artifact not found. Please check Tauri output directories."
        exit 1
    }
}
Write-Host "OK. Desktop built." -ForegroundColor Green


# 5. Staging
Write-Host "`n[5/6] Preparing staging directory..." -ForegroundColor Cyan
$stagingDir = Join-Path $PSScriptRoot "build\installer\windows\staging"
if (Test-Path $stagingDir) {
    Remove-Item -Recurse -Force $stagingDir
}
New-Item -ItemType Directory -Force -Path $stagingDir | Out-Null

Copy-Item $cliArtifact -Destination (Join-Path $stagingDir "plaza.exe") -Force
Copy-Item $desktopArtifact -Destination (Join-Path $stagingDir "plaza-desktop.exe") -Force
Write-Host "OK. Artifacts staged." -ForegroundColor Green

# 6. Build Installer
Write-Host "`n[6/6] Building PlazaVM_Setup.exe..." -ForegroundColor Cyan
$installerScript = Join-Path $PSScriptRoot "installer\windows\plazavm.iss"
if (-not (Test-Path $installerScript)) {
    Write-Error "Installer script not found at $installerScript"
    exit 1
}

$outputDir = Join-Path $PSScriptRoot "release"
New-Item -ItemType Directory -Force -Path $outputDir | Out-Null

# InnoSetup requires AppVersion to be stripped of -dp1 for VersionInfoVersion, but we can pass both.
$numericVersion = $AppVersion -replace '-.*',''

& $iscc "/DAppVersion=$AppVersion" "/DNumericVersion=$numericVersion" "/O$outputDir" "/FPlazaVM_Setup" $installerScript
if ($LASTEXITCODE -ne 0) {
    Write-Error "Inno Setup compilation failed."
    exit 1
}

$setupArtifact = Join-Path $outputDir "PlazaVM_Setup.exe"
if (-not (Test-Path $setupArtifact)) {
    Write-Error "PlazaVM_Setup.exe was not generated."
    exit 1
}

Write-Host "`n===============================================" -ForegroundColor Cyan
Write-Host "PlazaVM installer created successfully." -ForegroundColor Green
Write-Host "Path: $setupArtifact" -ForegroundColor Yellow
Write-Host "Size: $((Get-Item $setupArtifact).Length / 1MB).ToString('0.00') MB" -ForegroundColor Yellow
Write-Host "===============================================" -ForegroundColor Cyan
