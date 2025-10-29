#!/bin/bash

set -e

REPO_URL="https://raw.githubusercontent.com/TutTrue/Aimit/refs/heads/main"
BINARY_NAME="aimit"
INSTALL_DIR="/usr/local/bin"

detect_platform() {
  local os arch

  # Detect OS
  case "$(uname -s)" in
  Linux*) os="linux" ;;
  Darwin*) os="macos" ;;
  CYGWIN* | MINGW* | MSYS*) os="windows" ;;
  *)
    echo "Unsupported operating system: $(uname -s)" >&2
    exit 1
    ;;
  esac

  # Detect architecture
  case "$(uname -m)" in
  x86_64 | amd64) arch="x86_64" ;;
  aarch64 | arm64) arch="aarch64" ;;
  armv7l) arch="armv7" ;;
  *)
    echo "Unsupported architecture: $(uname -m)" >&2
    exit 1
    ;;
  esac

  # Special case for macOS ARM64 (Apple Silicon)
  if [[ "$os" == "macos" && "$arch" == "aarch64" ]]; then
    arch="aarch64"
  fi

  echo "${os}-${arch}"
}

get_binary_name() {
  local platform="$1"
  case "$platform" in
  *windows*) echo "${BINARY_NAME}.exe" ;;
  *) echo "$BINARY_NAME" ;;
  esac
}

download_binary() {
  local platform="$1"
  local binary_name="$2"
  local url="$REPO_URL/bin/$binary_name"

  echo "Detected platform: $platform"
  echo "Downloading binary: $binary_name..."

  # Check if wget or curl is available
  if command -v wget >/dev/null 2>&1; then
    wget -q "$url" -O "$INSTALL_DIR/$binary_name.new"
    mv "$INSTALL_DIR/$binary_name.new" "$INSTALL_DIR/$binary_name"
  elif command -v curl >/dev/null 2>&1; then
    curl -sL "$url" -o "$INSTALL_DIR/$binary_name.new"
    mv "$INSTALL_DIR/$binary_name.new" "$INSTALL_DIR/$binary_name"
  else
    echo "Error: Neither wget nor curl is available. Please install one of them." >&2
    exit 1
  fi
}

main() {
  echo "Installing aimit..."

  # Detect platform
  platform=$(detect_platform)
  binary_name=$(get_binary_name "$platform")

  # Create install directory if it doesn't exist
  if [[ ! -d "$INSTALL_DIR" ]]; then
    echo "Creating install directory: $INSTALL_DIR"
    sudo mkdir -p "$INSTALL_DIR"
  fi

  # Download the appropriate binary
  download_binary "$platform" "$binary_name"

  # Make binary executable (skip for Windows)
  if [[ "$platform" != *"windows"* ]]; then
    chmod +x "$INSTALL_DIR/$binary_name"
  fi

  # Verify installation
  if [[ -f "$INSTALL_DIR/$binary_name" ]]; then
    echo "Installation successful!"
    echo "Binary installed to: $INSTALL_DIR/$binary_name"
    echo "Platform: $platform"

    # Test if the binary works
    if [[ "$platform" != *"windows"* ]]; then
      if "$INSTALL_DIR/$binary_name" --version >/dev/null 2>&1; then
        echo "Binary is working correctly!"
      else
        echo "Warning: Binary was installed but may not be working correctly."
      fi
    fi
  else
    echo "Installation failed. Please check the script and repository." >&2
    exit 1
  fi
}

# Run main function
main "$@"

