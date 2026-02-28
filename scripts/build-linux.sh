#!/bin/bash

# Linux Build Script for ntfy.desktop

set -e

echo "Building ntfy.desktop for Linux..."

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
            echo "  -h, --help  Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Install dependencies
echo "Installing dependencies..."
npm install

# Install Linux build dependencies
echo "Installing Linux build dependencies..."
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.0-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libappindicator3-dev

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
    npm run tauri:build $BUILD_MODE
else
    npm run tauri:build $BUILD_MODE
fi

# Create distribution directory
echo "Creating distribution packages..."
mkdir -p dist/linux

# Copy AppImage
if [ -d "src-tauri/target/release/bundle/appimage" ]; then
    cp src-tauri/target/release/bundle/appimage/*.AppImage dist/linux/
    echo "AppImage created in: dist/linux/"
fi

# Copy DEB package
if [ -d "src-tauri/target/release/bundle/deb" ]; then
    cp src-tauri/target/release/bundle/deb/*.deb dist/linux/
    echo "DEB package created in: dist/linux/"
fi

# Create portable tar.gz
echo "Creating portable tar.gz package..."
PORTABLE_DIR="ntfy.desktop-linux-portable"
mkdir -p "$PORTABLE_DIR"

# Copy binary and resources
if [ -f "src-tauri/target/release/ntfy-desktop" ]; then
    cp "src-tauri/target/release/ntfy-desktop" "$PORTABLE_DIR/"

    # Create desktop entry
    cat > "$PORTABLE_DIR/ntfy.desktop.desktop" << EOF
[Desktop Entry]
Name=ntfy.desktop
Comment=A high-performance desktop notification client for ntfy
Exec=./ntfy-desktop
Icon=ntfy-desktop
Terminal=false
Type=Application
Categories=Utility;
EOF

    # Create README
    cat > "$PORTABLE_DIR/README.md" << EOF
# ntfy.desktop Portable Linux Version

Run the application with:
\`\`\`bash
./ntfy-desktop
\`\`\`

To install system-wide, copy the desktop file to ~/.local/share/applications/
\`\`\`bash
cp ntfy.desktop.desktop ~/.local/share/applications/
\`\`\`
EOF

    # Create tar.gz
    tar -czf "dist/linux/ntfy.desktop-linux-portable.tar.gz" "$PORTABLE_DIR"
    rm -rf "$PORTABLE_DIR"
    echo "Portable tar.gz created: dist/linux/ntfy.desktop-linux-portable.tar.gz"
fi

echo "Linux build completed!"
