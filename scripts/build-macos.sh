#!/bin/bash

# macOS Build Script for ntfy.desktop

set -e

echo "Building ntfy.desktop for macOS..."

# Parse arguments
RELEASE=false
SIGN=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            RELEASE=true
            shift
            ;;
        --sign)
            SIGN=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --release    Build in release mode"
            echo "  --sign       Sign the binaries"
            echo "  -h, --help   Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Check if running on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "This script must be run on macOS"
    exit 1
fi

# Install dependencies
echo "Installing dependencies..."
npm install

# Install macOS build dependencies if needed
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Update version for release builds
if [ "$RELEASE" = true ]; then
    echo "Updating version for release build..."
    bash scripts/versioning.sh --release
fi

# Build frontend
echo "Building frontend..."
npm run build

# Set build mode
BUILD_MODE=""
if [ "$RELEASE" = true ]; then
    BUILD_MODE="--release"
fi

# Build Tauri application
echo "Building Tauri application..."
if [ "$SIGN" = true ]; then
    export TAURI_PRIVATE_KEY=""  # Will be set externally
    export TAURI_KEY_PASSWORD=""  # Will be set externally
    export APPLE_CERTIFICATE=""   # Will be set externally
    npm run tauri:build $BUILD_MODE
else
    npm run tauri:build $BUILD_MODE
fi

# Create distribution directory
echo "Creating distribution packages..."
mkdir -p dist/macos

# Copy DMG
if [ -d "src-tauri/target/release/bundle/dmg" ]; then
    cp src-tauri/target/release/bundle/dmg/*.dmg dist/macos/
    echo "DMG installer created in: dist/macos/"
fi

# Copy .app bundle
if [ -d "src-tauri/target/release/bundle/macos" ]; then
    cp -r src-tauri/target/release/bundle/macos/*.app dist/macos/
    echo "App bundle created in: dist/macos/"
fi

# Create portable tar.gz
echo "Creating portable tar.gz package..."
if [ -d "src-tauri/target/release/bundle/macos" ]; then
    APP_BUNDLE=$(find src-tauri/target/release/bundle/macos -name "*.app" | head -n1)
    if [ -n "$APP_BUNDLE" ]; then
        cp -r "$APP_BUNDLE" "dist/macos/"

        # Create tar.gz of app bundle
        tar -czf "dist/macos/ntfy.desktop-macos-portable.tar.gz" -C "dist/macos" "$(basename "$APP_BUNDLE")"
        echo "Portable tar.gz created: dist/macos/ntfy.desktop-macos-portable.tar.gz"
    fi
fi

# Create Homebrew formula template
echo "Creating Homebrew formula template..."
cat > dist/macos/ntfy-desktop.rb << 'EOF'
class NtfyDesktop < Formula
  desc "A high-performance desktop notification client for ntfy"
  homepage "https://github.com/magnetgrouplabs/ntfy.desktop"
  url "https://github.com/magnetgrouplabs/ntfy.desktop/releases/download/v1.0.0/ntfy.desktop-macos-portable.tar.gz"
  sha256 "" # Fill with actual SHA256
  license "MIT"

  depends_on macos: "10.13"

  def install
    prefix.install "ntfy.desktop.app"
    bin.write_exec_script prefix/"ntfy.desktop.app/Contents/MacOS/ntfy-desktop"
  end

  def caveats
    <<~EOS
      ntfy.desktop has been installed to:
        #{prefix}/ntfy.desktop.app

      You can also run it directly from:
        #{bin}/ntfy-desktop
    EOS
  end

  test do
    system "#{bin}/ntfy-desktop", "--version"
  end
end
EOF

echo "macOS build completed!"
