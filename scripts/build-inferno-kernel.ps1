# build-inferno-kernel.ps1
#
# Builds a minimal Inferno native kernel for x86 (386) that can boot under QEMU.
# This script must be run with Docker Desktop running on Windows.
#
# Prerequisites:
#   - Docker Desktop running
#   - Internet connection for cloning Inferno source
#
# Output:
#   - inferno-386 kernel binary cached in $env:TEMP\plaza-inferno-cache\
#
# Usage:
#   .\scripts\build-inferno-kernel.ps1

$ErrorActionPreference = "Stop"

$CacheDir = Join-Path $env:TEMP "plaza-inferno-cache"
$BuildDir = Join-Path $env:TEMP "inferno-build"
$KernelCache = Join-Path $CacheDir "inferno-386"

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  PlazaVM Inferno Kernel Builder" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Cache directory: $CacheDir"
Write-Host "Build directory:  $BuildDir"
Write-Host ""

# Step 1: Check Docker availability
Write-Host "[1/6] Checking Docker availability..." -ForegroundColor Yellow
try {
    docker info 2>$null | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "Docker not running" }
    Write-Host "  ✓ Docker is available" -ForegroundColor Green
} catch {
    Write-Host "ERROR: Docker is not running." -ForegroundColor Red
    Write-Host ""
    Write-Host "Please start Docker Desktop and try again."
    exit 1
}
Write-Host ""

# Step 2: Clone Inferno source (if not already cloned)
Write-Host "[2/6] Cloning Inferno OS source..." -ForegroundColor Yellow
if (Test-Path $BuildDir) {
    Write-Host "  ✓ Build directory already exists, using cached source" -ForegroundColor Green
} else {
    git clone --depth 1 https://github.com/inferno-os/inferno-os.git $BuildDir
    Write-Host "  ✓ Cloned Inferno source" -ForegroundColor Green
}
Write-Host ""

# Step 3: Build the Docker image
Write-Host "[3/6] Building Inferno kernel via Docker..." -ForegroundColor Yellow
Write-Host "  (This may take 5-10 minutes on first build)" -ForegroundColor Gray

$dockerfileContent = @"
FROM i386/ubuntu:devel

RUN apt-get -y update && apt-get install -y gcc libc6-dev && rm -rf /var/lib/apt/lists/*

ENV INFERNO=/usr/inferno
COPY . `$INFERNO
WORKDIR `$INFERNO

RUN echo > mkconfig ROOT=`$INFERNO
RUN echo >>mkconfig TKSTYLE=std
RUN echo >>mkconfig SYSHOST=Linux
RUN echo >>mkconfig SYSTARG=Linux
RUN echo >>mkconfig OBJTYPE=386
RUN echo >>mkconfig 'OBJDIR=`$SYSTARG/`$OBJTYPE'
RUN echo >>mkconfig '<`$ROOT/mkfiles/mkhost-`$SYSHOST'
RUN echo >>mkconfig '<`$ROOT/mkfiles/mkfile-`$SYSTARG-`$OBJTYPE'

RUN ./makemk.sh
ENV PATH="`$INFERNO/Linux/386/bin:`$PATH"

RUN mk nuke
RUN mk install

WORKDIR `$INFERNO/os/pc
RUN mk 'CONF=pc' nuke
RUN mk 'CONF=pc'

RUN mkdir -p /output
RUN cp -v `$INFERNO/os/pc/pc/inferno.386 /output/inferno.386

WORKDIR /output
"@

$dockerfileContent | Out-File -FilePath (Join-Path $BuildDir "Dockerfile.plaza") -Encoding UTF8

docker build -t plaza-inferno-builder -f (Join-Path $BuildDir "Dockerfile.plaza") $BuildDir
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ✗ Docker build failed" -ForegroundColor Red
    exit 1
}
Write-Host "  ✓ Docker build complete" -ForegroundColor Green
Write-Host ""

# Step 4: Extract the kernel
Write-Host "[4/6] Extracting kernel binary..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $CacheDir | Out-Null

$containerId = docker create plaza-inferno-builder
docker cp "${containerId}:/output/inferno.386" $KernelCache
docker rm $containerId | Out-Null

Write-Host "  ✓ Kernel extracted to $KernelCache" -ForegroundColor Green
Write-Host ""

# Step 5: Verify the kernel
Write-Host "[5/6] Verifying kernel..." -ForegroundColor Yellow
if (Test-Path $KernelCache) {
    $kernelSize = (Get-Item $KernelCache).Length
    Write-Host "  ✓ Kernel file exists: $KernelCache" -ForegroundColor Green
    Write-Host "  ✓ Kernel size: $kernelSize bytes" -ForegroundColor Green
} else {
    Write-Host "  ✗ Kernel file not found at $KernelCache" -ForegroundColor Red
    exit 1
}
Write-Host ""

# Step 6: Report results
Write-Host "[6/6] Build complete!" -ForegroundColor Yellow
Write-Host ""
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  Inferno Kernel Build Summary" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Kernel path:    $KernelCache"
Write-Host "Kernel size:    $kernelSize bytes"
Write-Host "Target arch:    386 (x86)"
Write-Host "Boot method:    QEMU -kernel flag"
Write-Host ""
Write-Host "To test the kernel:" -ForegroundColor Yellow
Write-Host "  qemu-system-i386 -kernel $KernelCache -m 64 -nographic -serial stdio"
Write-Host ""
Write-Host "To use with PlazaVM:" -ForegroundColor Yellow
Write-Host "  The kernel will be automatically discovered by InfernoGuestRuntime"
Write-Host "  on the next workspace creation."
Write-Host ""
