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
    $LocalSource = "e:\plazavm\inferno-os"
    if (Test-Path $LocalSource) {
        Write-Host "  ✓ Copying from local source $LocalSource" -ForegroundColor Green
        Copy-Item -Path $LocalSource -Destination $BuildDir -Recurse -Force
    } else {
        Write-Host "  ✗ Local source $LocalSource not found, falling back to git clone" -ForegroundColor Yellow
        git clone --depth 1 https://github.com/inferno-os/inferno-os.git $BuildDir
    }
}
Write-Host ""

# Copy our plaza scripts into the build directory
$PlazaScriptsDir = Join-Path $BuildDir "plaza-scripts"
New-Item -ItemType Directory -Force -Path $PlazaScriptsDir | Out-Null
Copy-Item -Path (Join-Path $PSScriptRoot "..\engines\plaza-workspace\src\runtime\*.b") -Destination $PlazaScriptsDir -Force

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

# Compile PlazaVM Limbo scripts
WORKDIR `$INFERNO/plaza-scripts
RUN limbo -I`$INFERNO/module -o `$INFERNO/dis/inferno_workspace.dis inferno_workspace.b
RUN limbo -I`$INFERNO/module -o `$INFERNO/dis/linux_compat.dis linux_compat.b

WORKDIR `$INFERNO/os/pc
# Create a custom PlazaVM kernel configuration
RUN echo "dev" > plaza
RUN echo "    root cons arch env mnt pipe prog rtc srv dup cap" >> plaza
RUN echo "    ip ether draw pointer vga sd ds uart tinyfs" >> plaza
RUN echo "ip" >> plaza
RUN echo "    tcp udp ipifc icmp icmp6 ipmux" >> plaza
RUN echo "lib" >> plaza
RUN echo "    interp keyring sec mp draw memlayer memdraw tk math kern" >> plaza
RUN echo "link" >> plaza
RUN echo "    ether2114x ether83815 etherelnk3 ps2mouse ethermedium" >> plaza
RUN echo "misc" >> plaza
RUN echo "    vgas3 vgamach64xx cga sdata sd53c8xx uarti8250" >> plaza
RUN echo "mod" >> plaza
RUN echo "    sys draw tk keyring crypt ipints math" >> plaza
RUN echo "init" >> plaza
RUN echo "    inferno_workspace" >> plaza
RUN echo "code" >> plaza
RUN echo "    int kernel_pool_pcnt = 10; int main_pool_pcnt = 40; int heap_pool_pcnt = 20; int image_pool_pcnt = 40; int cflag=0; int swcursor=0; int consoleprint=0; int novgascreen=1;" >> plaza
RUN echo "port" >> plaza
RUN echo "    alarm alloc allocb chan dev dial dis discall exception exportfs inferno latin1 nocache nodynld parse pgrp print proc qio qlock random sysfile taslock xalloc" >> plaza
RUN echo "root" >> plaza
RUN echo "    /chan /dev /dis /env /fd /n /n/remote /net /nvfs /prog" >> plaza
RUN echo "    /dis/lib /dis/svc /dis/wm" >> plaza
RUN echo "    /dis/sh.dis /dis/ls.dis /dis/cat.dis /dis/bind.dis /dis/mount.dis /dis/pwd.dis /dis/echo.dis /dis/cd.dis" >> plaza
RUN echo "    /dis/lib/bufio.dis /dis/lib/string.dis /dis/lib/readdir.dis /dis/lib/workdir.dis /dis/lib/daytime.dis /dis/lib/auth.dis /dis/lib/ssl.dis /dis/disk/kfs.dis /dis/lib/arg.dis /dis/lib/styx.dis" >> plaza
RUN echo "    /dis/inferno_workspace.dis /dis/linux_compat.dis" >> plaza

RUN mk 'CONF=plaza' nuke
RUN mk 'CONF=plaza'

RUN mkdir -p /output
RUN cp -v `$INFERNO/os/pc/plaza/inferno.386 /output/inferno.386

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
