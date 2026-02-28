#!/bin/bash

# Release Script for ntfy.desktop

set -e

VERSION=""
PLATFORM="all"
SIGN=false
DRY_RUN=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --platform)
            PLATFORM="$2"
            shift 2
            ;;
        --sign)
            SIGN=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --version VERSION    Version to release (required)"
            echo "  --platform PLATFORM  Platform to build (windows, linux, macos, all)"
            echo "  --sign               Sign the binaries (requires credentials)"
            echo "  --dry-run            Show what would be done without building"
            echo "  -h, --help          Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

    # Use auto-versioning for release builds
    if [ -z "$VERSION" ]; then
        echo "Using automatic date-based versioning..."
        VERSION=$(date +v%y.%-m.1)
    else
        # Validate version format (YY.M.X)
        if [[ ! "$VERSION" =~ ^v[0-9]{2}.[0-9]{1,2}.[0-9]+$ ]]; then
            echo "Error: Version must be in format vYY.M.X"
            exit 1
        fi
    fi

echo "Preparing ntfy.desktop release $VERSION..."

# Create dist directory
mkdir -p dist

# Update version files
if [ "$DRY_RUN" = false ]; then
    echo "Updating version to $VERSION..."
    bash scripts/versioning.sh --force-version "${VERSION#v}"
fi

# Build function
build_platform() {
    local platform=$1

    echo "Building for $platform..."

    case $platform in
        windows)
            if [ "$DRY_RUN" = false ]; then
                if [ "$SIGN" = true ]; then
                    powershell -ExecutionPolicy Bypass -File scripts/build-windows.ps1 -Release -Sign
                else
                    powershell -ExecutionPolicy Bypass -File scripts/build-windows.ps1 -Release
                fi
            fi
            ;;
        linux)
            if [ "$DRY_RUN" = false ]; then
                if [ "$SIGN" = true ]; then
                    bash scripts/build-linux.sh --release --sign
                else
                    bash scripts/build-linux.sh --release
                fi
            fi
            ;;
        macos)
            if [ "$DRY_RUN" = false ]; then
                if [ "$SIGN" = true ]; then
                    bash scripts/build-macos.sh --release --sign
                else
                    bash scripts/build-macos.sh --release
                fi
            fi
            ;;
    esac
}

# Build for specified platform(s)
case $PLATFORM in
    all)
        build_platform windows
        build_platform linux
        build_platform macos
        ;;
    windows|linux|macos)
        build_platform $PLATFORM
        ;;
    *)
        echo "Error: Invalid platform '$PLATFORM'. Must be windows, linux, macos, or all"
        exit 1
        ;;
esac

# Create release notes template
if [ "$DRY_RUN" = false ]; then
    cat > "dist/release_notes_$VERSION.md" << EOF
# ntfy.desktop $VERSION Release Notes

## What's New

- Feature 1
- Feature 2
- Bug fixes

## Installation

### Windows
- Download the MSI installer: \`ntfy-desktop_${VERSION#v}_x64_en-US.msi\`
- Or portable version: \`ntfy.desktop-portable.exe\`

### Linux
- Download the AppImage: \`ntfy-desktop_${VERSION#v}_amd64.AppImage\`
- Or DEB package: \`ntfy-desktop_${VERSION#v}_amd64.deb\`
- Or portable tar.gz: \`ntfy.desktop-linux-portable.tar.gz\`

### macOS
- Download the DMG: \`ntfy-desktop_${VERSION#v}_x64.dmg\`
- Or portable tar.gz: \`ntfy.desktop-macos-portable.tar.gz\`

## Performance

Performance testing results are included in the performance report.

## Package Managers

### Chocolatey (Windows)
\`\`\`powershell
choco install ntfy-desktop
\`\`\`

### Homebrew (macOS)
\`\`\`bash
brew install ntfy-desktop
\`\`\`

## Automatic Updates

ntfy.desktop supports automatic updates. The app will check for updates on startup.

## Support

If you encounter any issues, please open an issue on GitHub.
EOF

    echo "Release notes created: dist/release_notes_$VERSION.md"
fi

# Create package manager distribution files
if [ "$DRY_RUN" = false ]; then
    mkdir -p "dist/chocolatey/tools"

    # Chocolatey package template
    cat > "dist/chocolatey/ntfy-desktop.nuspec" << EOF
<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>ntfy-desktop</id>
    <version>${VERSION#v}</version>
    <title>ntfy.desktop</title>
    <authors>Anthony</authors>
    <projectUrl>https://github.com/magnetgrouplabs/ntfy.desktop</projectUrl>
    <description>A high-performance desktop notification client for ntfy</description>
    <tags>ntfy desktop notifications cross-platform</tags>
    <license type="expression">MIT</license>
    <packageSourceUrl>https://github.com/magnetgrouplabs/ntfy.desktop</packageSourceUrl>
  </metadata>
  <files>
    <file src="tools\**" target="tools" />
  </files>
</package>
EOF

    # Chocolatey install script
    cat > "dist/chocolatey/tools/chocolateyinstall.ps1" << EOF
\$packageName = 'ntfy-desktop'
\$fileType = 'msi'
\$silentArgs = '/quiet'
\$url = "https://github.com/magnetgrouplabs/ntfy.desktop/releases/download/$VERSION/ntfy-desktop_${VERSION#v}_x64_en-US.msi"

Install-ChocolateyPackage \$packageName \$fileType \$silentArgs \$url
EOF
fi

if [ "$DRY_RUN" = true ]; then
    echo ""
    echo "DRY RUN COMPLETED"
    echo "Version: $VERSION"
    echo "Platform: $PLATFORM"
    echo "Signing: $SIGN"
    echo ""
    echo "The following would be built:"
    echo "- Windows: MSI installer + portable EXE"
    echo "- Linux: AppImage + DEB package + portable tar.gz"
    echo "- macOS: DMG installer + portable tar.gz"
    echo ""
    echo "To actually build, run without --dry-run"
else
    echo ""
    echo "RELEASE BUILD COMPLETED"
    echo "Version: $VERSION"
    echo "Platform: $PLATFORM"
    echo ""
    echo "Next steps:"
    echo "1. Commit the version changes"
    echo "2. Create a git tag: git tag $VERSION"
    echo "3. Push the tag: git push origin $VERSION"
    echo "4. The GitHub Actions workflow will automatically build and release"
    echo ""
    echo "Distribution files are in the dist/ directory"
    echo "Versioning log: scripts/build_log.txt"
fi
