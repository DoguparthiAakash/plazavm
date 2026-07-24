# PowerShell release build and packaging script for PlazaVM v2 Developer Preview (DP1)

Param (
    [string]$Target = "x86_64-pc-windows-msvc"
)

$ErrorActionPreference = "Stop"

Write-Host "===============================================================" -ForegroundColor Cyan
Write-Host "📦 Building PlazaVM v2 Developer Preview (DP1) Packages..." -ForegroundColor Cyan
Write-Host "===============================================================" -ForegroundColor Cyan

$ReleaseDir = "release"
$InstallersDir = Join-Path $ReleaseDir "installers"
$PortableDir = Join-Path $ReleaseDir "portable"
$ChecksumsDir = Join-Path $ReleaseDir "checksums"
$ManifestsDir = Join-Path $ReleaseDir "manifests"

New-Item -ItemType Directory -Force -Path $InstallersDir | Out-Null
New-Item -ItemType Directory -Force -Path $PortableDir | Out-Null
New-Item -ItemType Directory -Force -Path $ChecksumsDir | Out-Null
New-Item -ItemType Directory -Force -Path $ManifestsDir | Out-Null

# 1. Build CLI binary
Write-Host "🔨 Compiling plaza-cli release binary..." -ForegroundColor Yellow
cargo build --release -p plaza-cli

# 2. Package Portable ZIP
Write-Host "🤐 Creating Portable ZIP package..." -ForegroundColor Yellow
$ZipPath = Join-Path $PortableDir "PlazaVM_v0.1.0-dp1_win_x64_portable.zip"
$CliExe = "target\release\plaza-cli.exe"

if (Test-Path $CliExe) {
    Compress-Archive -Path $CliExe, "README.md", "LICENSE" -DestinationPath $ZipPath -Force
    Write-Host "  ✓ Portable ZIP created: $ZipPath" -ForegroundColor Green
}

# 3. Generate SHA256 Checksums
Write-Host "🔒 Generating SHA256 Checksums..." -ForegroundColor Yellow
$ChecksumFile = Join-Path $ChecksumsDir "SHA256SUMS_v0.1.0-dp1.txt"
$Files = Get-ChildItem -Path $InstallersDir, $PortableDir -Recurse -File

$ChecksumContent = ""
foreach ($file in $Files) {
    $hash = (Get-FileHash -Path $file.FullName -Algorithm SHA256).Hash.ToLower()
    $ChecksumContent += "$hash  $($file.Name)`n"
}

Set-Content -Path $ChecksumFile -Value $ChecksumContent
Write-Host "  ✓ SHA256 Checksums written to: $ChecksumFile" -ForegroundColor Green

Write-Host "===============================================================" -ForegroundColor Cyan
Write-Host "✨ Packaging Complete for PlazaVM DP1!" -ForegroundColor Cyan
Write-Host "===============================================================" -ForegroundColor Cyan
