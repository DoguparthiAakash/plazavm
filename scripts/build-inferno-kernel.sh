#!/bin/bash
# build-inferno-kernel.sh
#
# Builds a minimal Inferno native kernel for x86 (386) that can boot under QEMU.
# This script must be run with Docker available and the daemon running.
#
# Prerequisites:
#   - Docker Desktop running (Windows) or Docker daemon running (Linux/Mac)
#   - Internet connection for cloning Inferno source
#
# Output:
#   - inferno-386 kernel binary cached in /tmp/plaza-inferno-cache/
#
# Usage:
#   ./scripts/build-inferno-kernel.sh

set -euo pipefail

CACHE_DIR="${TEMP:-/tmp}/plaza-inferno-cache"
BUILD_DIR="${TEMP:-/tmp}/inferno-build"
KERNEL_CACHE="${CACHE_DIR}/inferno-386"

echo "============================================"
echo "  PlazaVM Inferno Kernel Builder"
echo "============================================"
echo ""
echo "Cache directory: ${CACHE_DIR}"
echo "Build directory:  ${BUILD_DIR}"
echo ""

# Step 1: Check Docker availability
echo "[1/6] Checking Docker availability..."
if ! docker info > /dev/null 2>&1; then
    echo "ERROR: Docker is not running."
    echo ""
    echo "Please start Docker Desktop (Windows) or Docker daemon (Linux/Mac)"
    echo "and try again."
    exit 1
fi
echo "  ✓ Docker is available"
echo ""

# Step 2: Clone Inferno source (if not already cloned)
echo "[2/6] Cloning Inferno OS source..."
if [ -d "${BUILD_DIR}" ]; then
    echo "  ✓ Build directory already exists, using cached source"
else
    git clone --depth 1 https://github.com/inferno-os/inferno-os.git "${BUILD_DIR}"
    echo "  ✓ Cloned Inferno source"
fi
echo ""

# Step 3: Build the Docker image
echo "[3/6] Building Inferno kernel via Docker..."
docker build \
    -t plaza-inferno-builder \
    -f - \
    "${BUILD_DIR}" << 'DOCKERFILE'
FROM i386/ubuntu:devel

RUN apt-get -y update && apt-get install -y \
    gcc \
    libc6-dev \
    && rm -rf /var/lib/apt/lists/*

ENV INFERNO=/usr/inferno
COPY . $INFERNO
WORKDIR $INFERNO

# Setup mkconfig for Linux hosted, 386 target
RUN echo > mkconfig ROOT=$INFERNO && \
    echo >>mkconfig TKSTYLE=std && \
    echo >>mkconfig SYSHOST=Linux && \
    echo >>mkconfig SYSTARG=Linux && \
    echo >>mkconfig OBJTYPE=386 && \
    echo >>mkconfig 'OBJDIR=$SYSTARG/$OBJTYPE' && \
    echo >>mkconfig '<$ROOT/mkfiles/mkhost-$SYSHOST' && \
    echo >>mkconfig '<$ROOT/mkfiles/mkfile-$SYSTARG-$OBJTYPE'

# Build the mk tool first
RUN ./makemk.sh
ENV PATH="$INFERNO/Linux/386/bin:${PATH}"

# Build all libraries and the hosted emu (needed for Limbo compiler)
RUN mk nuke
RUN mk install

# Now build the native kernel for PC
WORKDIR $INFERNO/os/pc
RUN mk 'CONF=pc' nuke
RUN mk 'CONF=pc'

# Copy kernel to output
RUN mkdir -p /output && \
    cp -v $INFERNO/os/pc/pc/inferno.386 /output/inferno.386

WORKDIR /output
DOCKERFILE
echo "  ✓ Docker build complete"
echo ""

# Step 4: Extract the kernel
echo "[4/6] Extracting kernel binary..."
mkdir -p "${CACHE_DIR}"

# Create a temporary container to extract the kernel
CONTAINER_ID=$(docker create plaza-inferno-builder)
docker cp "${CONTAINER_ID}:/output/inferno.386" "${KERNEL_CACHE}"
docker rm "${CONTAINER_ID}" > /dev/null

echo "  ✓ Kernel extracted to ${KERNEL_CACHE}"
echo ""

# Step 5: Verify the kernel
echo "[5/6] Verifying kernel..."
if [ -f "${KERNEL_CACHE}" ]; then
    KERNEL_SIZE=$(stat -f%z "${KERNEL_CACHE}" 2>/dev/null || stat -c%s "${KERNEL_CACHE}" 2>/dev/null || echo "unknown")
    echo "  ✓ Kernel file exists: ${KERNEL_CACHE}"
    echo "  ✓ Kernel size: ${KERNEL_SIZE} bytes"
    
    # Check if it's a valid ELF binary
    if file "${KERNEL_CACHE}" | grep -q "ELF"; then
        echo "  ✓ Kernel is a valid ELF binary"
    else
        echo "  ⚠  Kernel may not be a valid ELF binary (check output)"
    fi
else
    echo "  ✗ Kernel file not found at ${KERNEL_CACHE}"
    exit 1
fi
echo ""

# Step 6: Report results
echo "[6/6] Build complete!"
echo ""
echo "============================================"
echo "  Inferno Kernel Build Summary"
echo "============================================"
echo ""
echo "Kernel path:    ${KERNEL_CACHE}"
echo "Kernel size:    ${KERNEL_SIZE} bytes"
echo "Target arch:    386 (x86)"
echo "Boot method:    QEMU -kernel flag"
echo ""
echo "To test the kernel:"
echo "  qemu-system-i386 -kernel ${KERNEL_CACHE} -m 64 -nographic -serial stdio"
echo ""
echo "To use with PlazaVM:"
echo "  The kernel will be automatically discovered by InfernoGuestRuntime"
echo "  on the next workspace creation."
echo ""
