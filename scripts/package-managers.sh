#!/bin/bash

# Package Manager Distribution Script for notify.desktop

set -e

echo "Setting up package manager distribution..."

# Create package manager directories
mkdir -p dist/package-managers/chocolatey
mkdir -p dist/package-managers/homebrew
mkdir -p dist/package-managers/aur

# Chocolatey package
cat > dist/package-managers/chocolatey/notify.desktop.nuspec << 'EOF'
<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>notify.desktop</id>
    <version>0.1.0</version>
    <title>notify.desktop</title>
    <authors>Your Name</authors>
    <projectUrl>https://github.com/yourusername/notify.desktop</projectUrl>
    <description>A cross-platform desktop notification application</description>
    <tags>notify desktop notifications cross-platform</tags>
    <license type="expression">MIT</license>
    <packageSourceUrl>https://github.com/yourusername/notify.desktop</packageSourceUrl>
  </metadata>
  <files>
    <file src="tools\**" target="tools" />
  </files>
</package>
EOF

mkdir -p dist/package-managers/chocolatey/tools
cat > dist/package-managers/chocolatey/tools/chocolateyinstall.ps1 << 'EOF'
$packageName = 'notify.desktop'
$fileType = 'msi'
$silentArgs = '/quiet'
$url = "https://github.com/yourusername/notify.desktop/releases/download/v0.1.0/notify.desktop_0.1.0_x64_en-US.msi"

Install-ChocolateyPackage $packageName $fileType $silentArgs $url
EOF

# Homebrew formula
cat > dist/package-managers/homebrew/notify-desktop.rb << 'EOF'
class NotifyDesktop < Formula
  desc "A cross-platform desktop notification application"
  homepage "https://github.com/yourusername/notify.desktop"
  url "https://github.com/yourusername/notify.desktop/releases/download/v0.1.0/notify.desktop-macos-portable.tar.gz"
  sha256 "" # Fill with actual SHA256
  license "MIT"

  depends_on macos: "10.13"

  def install
    prefix.install "notify.desktop.app"
    bin.write_exec_script prefix/"notify.desktop.app/Contents/MacOS/notify.desktop"
  end

  def caveats
    <<~EOS
      notify.desktop has been installed to:
        #{prefix}/notify.desktop.app

      You can also run it directly from:
        #{bin}/notify.desktop
    EOS
  end

  test do
    system "#{bin}/notify.desktop", "--version"
  end
end
EOF

# AUR package (Arch Linux)
cat > dist/package-managers/aur/PKGBUILD << 'EOF'
# Maintainer: Your Name <your@email.com>

pkgname=notify-desktop
pkgver=0.1.0
pkgrel=1
pkgdesc="A cross-platform desktop notification application"
arch=('x86_64')
url="https://github.com/yourusername/notify.desktop"
license=('MIT')
depends=('webkit2gtk' 'libayatana-appindicator')
source=("https://github.com/yourusername/notify.desktop/releases/download/v$pkgver/notify.desktop-linux-portable.tar.gz")
sha256sums=('') # Fill with actual SHA256

package() {
  cd "$srcdir"
  
  # Install binary
  install -Dm755 "notify.desktop" "$pkgdir/usr/bin/notify.desktop"
  
  # Install desktop file
  install -Dm644 "notify.desktop.desktop" "$pkgdir/usr/share/applications/notify.desktop.desktop"
  
  # Install icons
  install -Dm644 "icons/32x32.png" "$pkgdir/usr/share/icons/hicolor/32x32/apps/notify.desktop.png"
  install -Dm644 "icons/128x128.png" "$pkgdir/usr/share/icons/hicolor/128x128/apps/notify.desktop.png"
}
EOF

# Create installation documentation
cat > dist/package-managers/INSTALLATION.md << 'EOF'
# Package Manager Installation

## Chocolatey (Windows)

```powershell
choco install notify.desktop
```

## Homebrew (macOS)

```bash
brew tap yourusername/notify-desktop
brew install notify-desktop
```

## AUR (Arch Linux)

```bash
yay -S notify-desktop
# or
paru -S notify-desktop
```

## Manual Installation

### Windows
1. Download the MSI installer from the releases page
2. Run the installer
3. Or use the portable executable

### Linux
1. Download the AppImage or DEB package
2. Make executable: `chmod +x notify.desktop*.AppImage`
3. Run: `./notify.desktop*.AppImage`

### macOS
1. Download the DMG file
2. Drag the app to Applications folder
3. Or use the portable version
EOF

echo "Package manager distribution files created!"
echo "Files are in dist/package-managers/ directory"