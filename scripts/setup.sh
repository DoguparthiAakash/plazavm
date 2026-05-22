#!/bin/bash
set -e

echo "Setting up PlazaVM dependencies..."

if [[ "$OSTYPE" == "darwin"* ]]; then
  echo "macOS detected. Installing QEMU via Homebrew..."
  brew install qemu
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
  if command -v apt-get &> /dev/null; then
    echo "Ubuntu/Debian detected. Installing QEMU via APT..."
    sudo apt-get update && sudo apt-get install -y qemu-system qemu-utils
  elif command -v pacman &> /dev/null; then
    echo "Arch Linux detected. Installing QEMU via Pacman..."
    sudo pacman -S --noconfirm qemu qemu-arch-extra
  else
    echo "Unsupported package manager. Please install QEMU manually."
  fi
elif [[ "$OSTYPE" == "msys"* || "$OSTYPE" == "cygwin"* ]]; then
  echo "Windows detected. Please ensure QEMU is installed via MSYS2 or standalone installer."
else
  echo "Unsupported OS for automatic setup."
fi

echo "Setup complete."
