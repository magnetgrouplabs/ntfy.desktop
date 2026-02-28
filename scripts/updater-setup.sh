#!/bin/bash

# Automatic Updates Setup Script for notify.desktop

set -e

echo "Setting up automatic updates configuration..."

# Enable updater in tauri.conf.json
if [ -f "src-tauri/tauri.conf.json" ]; then
    # Create backup
    cp src-tauri/tauri.conf.json src-tauri/tauri.conf.json.backup
    
    # Use jq to update the configuration
    if command -v jq &> /dev/null; then
        jq '.tauri.updater.active = true | .tauri.updater.endpoints = ["https://github.com/yourusername/notify.desktop/releases/latest/download/latest.json"] | .tauri.updater.pubkey = "YOUR_PUBLIC_KEY_HERE"' src-tauri/tauri.conf.json > src-tauri/tauri.conf.json.tmp
        mv src-tauri/tauri.conf.json.tmp src-tauri/tauri.conf.json
    else
        echo "jq not found, manually updating tauri.conf.json..."
        # Manual update if jq not available
        sed -i.bak 's/"updater": {[^}]*}/"updater": {
      "active": true,
      "endpoints": ["https:\/\/github.com\/yourusername\/notify.desktop\/releases\/latest\/download\/latest.json"],
      "pubkey": "YOUR_PUBLIC_KEY_HERE"
    }/g' src-tauri/tauri.conf.json
    fi
fi

# Create updater configuration directory
mkdir -p scripts/updater

# Create updater configuration
cat > scripts/updater/updater-config.json << 'EOF'
{
  "version": "2026.02.26.1",
  "notes": "Initial release",
  "pub_date": "2024-01-01T00:00:00Z",
  "platforms": {
    "darwin-x86_64": {
      "signature": "",
      "url": "https://github.com/yourusername/notify.desktop/releases/download/v0.1.0/notify.desktop_0.1.0_x64.dmg"
    },
    "darwin-aarch64": {
      "signature": "",
      "url": "https://github.com/yourusername/notify.desktop/releases/download/v0.1.0/notify.desktop_0.1.0_aarch64.dmg"
    },
    "linux-x86_64": {
      "signature": "",
      "url": "https://github.com/yourusername/notify.desktop/releases/download/v0.1.0/notify.desktop_0.1.0_amd64.AppImage"
    },
    "windows-x86_64": {
      "signature": "",
      "url": "https://github.com/yourusername/notify.desktop/releases/download/v0.1.0/notify.desktop_0.1.0_x64_en-US.msi"
    }
  }
}
EOF

# Create update manifest generator script
cat > scripts/updater/generate-manifest.sh << 'EOF'
#!/bin/bash

# Update Manifest Generator Script

set -e

VERSION=""
NOTES=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --notes)
            NOTES="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --version VERSION    Version number (required)"
            echo "  --notes NOTES       Release notes"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

if [ -z "$VERSION" ]; then
    echo "Error: --version is required"
    exit 1
fi

# Generate manifest
cat > latest.json << EOF
{
  "version": "$VERSION",
  "notes": "$NOTES",
  "pub_date": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "platforms": {
    "darwin-x86_64": {
      "signature": "",
      "url": "https://github.com/yourusername/notify.desktop/releases/download/v$VERSION/notify.desktop_${VERSION}_x64.dmg"
    },
    "darwin-aarch64": {
      "signature": "",
      "url": "https://github.com/yourusername/notify.desktop/releases/download/v$VERSION/notify.desktop_${VERSION}_aarch64.dmg"
    },
    "linux-x86_64": {
      "signature": "",
      "url": "https://github.com/yourusername/notify.desktop/releases/download/v$VERSION/notify.desktop_${VERSION}_amd64.AppImage"
    },
    "windows-x86_64": {
      "signature": "",
      "url": "https://github.com/yourusername/notify.desktop/releases/download/v$VERSION/notify.desktop_${VERSION}_x64_en-US.msi"
    }
  }
}
EOF

echo "Update manifest generated: latest.json"
echo "Upload this file to your GitHub release as 'latest.json'"
EOF

# Create GitHub Actions workflow for automatic updates
cat > scripts/updater/update-workflow.yml << 'EOF'
name: Generate Update Manifest

on:
  release:
    types: [published]

jobs:
  generate-manifest:
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Generate update manifest
      run: |
        VERSION="${GITHUB_REF#refs/tags/v}"
        NOTES="${{ github.event.release.body }}"
        
        # Generate the manifest
        bash scripts/updater/generate-manifest.sh \
          --version "$VERSION" \
          --notes "$NOTES"
        
    - name: Upload manifest to release
      uses: actions/upload-release-asset@v1
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        upload_url: ${{ github.event.release.upload_url }}
        asset_path: latest.json
        asset_name: latest.json
        asset_content_type: application/json
EOF

# Create updater documentation
cat > scripts/updater/UPDATER_GUIDE.md << 'EOF'
# Automatic Updates Guide

notify.desktop supports automatic updates using Tauri's built-in updater.

## How It Works

1. The application checks for updates on startup
2. It downloads the latest.json manifest from GitHub releases
3. If a newer version is available, it prompts the user to update
4. The update is downloaded and installed automatically

## Setup Requirements

### Tauri Configuration

The updater is enabled in `src-tauri/tauri.conf.json`:

```json
"updater": {
  "active": true,
  "endpoints": [
    "https://github.com/yourusername/notify.desktop/releases/latest/download/latest.json"
  ],
  "pubkey": "YOUR_PUBLIC_KEY_HERE"
}
```

### GitHub Releases

Each release must include a `latest.json` file with the update manifest.

## Generating Update Manifests

### Manual Generation

```bash
# Generate manifest for a release
bash scripts/updater/generate-manifest.sh \
  --version "1.0.0" \
  --notes "New features and bug fixes"

# Upload the generated latest.json to your GitHub release
```

### Automated Generation

Use the GitHub Actions workflow in `scripts/updater/update-workflow.yml` to automatically generate and upload the manifest when a release is published.

## Update Manifest Format

The `latest.json` file contains:

```json
{
  "version": "1.0.0",
  "notes": "Release notes",
  "pub_date": "2024-01-01T00:00:00Z",
  "platforms": {
    "darwin-x86_64": {
      "signature": "",
      "url": "https://github.com/.../notify.desktop_1.0.0_x64.dmg"
    }
  }
}
```

## Code Signing

For automatic updates to work properly, all binaries must be signed with the same certificate.

### Windows
- Use a valid code signing certificate
- The same certificate must be used for all releases

### macOS
- Use a Developer ID certificate
- Notarize the application

### Linux
- Sign packages with GPG
- Or use AppImage update mechanisms

## Testing Updates

### Local Testing

1. Build two versions of the app (e.g., 1.0.0 and 1.0.1)
2. Install the older version
3. Publish an update manifest for the newer version
4. Launch the app and verify it detects the update

### Production Testing

1. Create a test release with a higher version number
2. Install the current production version
3. Verify the update is detected and works correctly

## Troubleshooting

### Common Issues

1. **Update not detected**: Check the endpoint URL and manifest format
2. **Download fails**: Verify the binary URLs are accessible
3. **Installation fails**: Ensure binaries are properly signed

### Debug Mode

Enable debug logging to troubleshoot update issues:

```rust
// In your Rust code
#[tauri::command]
fn check_for_updates(app: tauri::AppHandle) {
    let result = app.updater().check().await;
    println!("Update check result: {:?}", result);
}
```

## Security Considerations

- Always verify update signatures
- Use HTTPS for update endpoints
- Keep signing keys secure
- Regularly audit the update process

## Custom Update Servers

You can use custom update servers instead of GitHub:

```json
"endpoints": [
  "https://your-update-server.com/latest.json"
]
```

This allows for more control over the update process and distribution.
EOF

# Make scripts executable
chmod +x scripts/updater/generate-manifest.sh

echo "Automatic updates setup completed!"
echo "Refer to scripts/updater/UPDATER_GUIDE.md for detailed instructions."