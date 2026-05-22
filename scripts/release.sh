#!/bin/bash
set -e

echo "Building PlazaVM binaries..."
pnpm install
pnpm run build --filter=@plazavm/desktop
pnpm run build --filter=@plazavm/cli

echo "Binaries built successfully."
