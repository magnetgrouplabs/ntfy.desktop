#!/bin/bash

# Code Signing Setup Script for notify.desktop

set -e

echo "Setting up code signing configuration..."

# Create signing directory
mkdir -p scripts/signing

# Create signing guide
cat > scripts/signing/SIGNING_GUIDE.md << 'EOF'
# Code Signing Guide

This guide explains how to set up code signing for notify.desktop distributions.

## Windows Signing

### Requirements:
- Windows code signing certificate (.pfx file)
- Certificate password
- Tauri private key

### Setup:
1. Obtain a code signing certificate from a trusted CA
2. Export the certificate as a .pfx file
3. Set the following GitHub secrets:
   - `WINDOWS_CERT_PFX`: Base64 encoded .pfx file
   - `WINDOWS_CERT_PASSWORD`: Certificate password
   - `TAURI_PRIVATE_KEY`: Tauri private key
   - `TAURI_KEY_PASSWORD`: Tauri key password

### Automated Signing:
The GitHub Actions workflow will automatically sign Windows binaries when these secrets are set.

## macOS Signing

### Requirements:
- Apple Developer Account
- App Store Connect access
- Developer ID Application certificate

### Setup:
1. Enroll in Apple Developer Program
2. Create Developer ID Application certificate
3. Set the following GitHub secrets:
   - `APPLE_CERTIFICATE`: Base64 encoded .p12 certificate
   - `APPLE_CERT_PASSWORD`: Certificate password
   - `APPLE_ID`: Apple ID for notarization
   - `APPLE_ID_PASSWORD`: App-specific password
   - `TEAM_ID`: Developer team ID

### Automated Notarization:
The GitHub Actions workflow will automatically notarize macOS builds when these secrets are set.

## Linux Signing

Linux distributions are typically signed using GPG keys for package repositories.

### Setup:
1. Generate GPG key: `gpg --full-generate-key`
2. Export public key: `gpg --armor --export your-email@example.com > public.key`
3. Set GitHub secret: `LINUX_GPG_PRIVATE_KEY`

## Tauri Signing

Tauri requires a private key for signing bundles across all platforms.

### Generate Tauri Key:
```bash
# Generate new key
tauri signer generate -w ~/.tauri/tauri-signer.key

# Save the private key as a GitHub secret
cat ~/.tauri/tauri-signer.key | base64
```

Set GitHub secret: `TAURI_PRIVATE_KEY` with the base64 encoded key.

## GitHub Secrets Reference

| Secret Name | Platform | Purpose |
|-------------|----------|---------|
| TAURI_PRIVATE_KEY | All | Tauri bundle signing key |
| TAURI_KEY_PASSWORD | All | Tauri key password |
| WINDOWS_CERT_PFX | Windows | Code signing certificate |
| WINDOWS_CERT_PASSWORD | Windows | Certificate password |
| APPLE_CERTIFICATE | macOS | Developer ID certificate |
| APPLE_CERT_PASSWORD | macOS | Certificate password |
| APPLE_ID | macOS | Apple ID for notarization |
| APPLE_ID_PASSWORD | macOS | App-specific password |
| TEAM_ID | macOS | Developer team ID |
| LINUX_GPG_PRIVATE_KEY | Linux | GPG key for package signing |

## Testing Signing

To test signing locally:

### Windows:
```powershell
# Set environment variables
$env:TAURI_PRIVATE_KEY = "your-key"
$env:TAURI_KEY_PASSWORD = "your-password"

# Build with signing
npm run tauri:build -- --release
```

### macOS:
```bash
# Set environment variables
export TAURI_PRIVATE_KEY="your-key"
export TAURI_KEY_PASSWORD="your-password"
export APPLE_CERTIFICATE="your-cert"
export APPLE_CERT_PASSWORD="your-password"

# Build with signing
npm run tauri:build -- --release
```

## Troubleshooting

### Common Issues:
1. **Certificate not trusted**: Ensure certificate is from a trusted CA
2. **Notarization fails**: Check Apple ID and app-specific password
3. **Tauri signing fails**: Verify private key format and password

For detailed troubleshooting, refer to the Tauri documentation.
EOF

# Create signing scripts
cat > scripts/signing/sign-windows.ps1 << 'EOF'
# Windows Signing Script

param(
    [string]$CertPath,
    [string]$CertPassword,
    [string]$TauriKey,
    [string]$TauriPassword
)

Write-Host "Setting up Windows signing environment..." -ForegroundColor Green

# Set environment variables
if ($TauriKey) {
    $env:TAURI_PRIVATE_KEY = $TauriKey
    $env:TAURI_KEY_PASSWORD = $TauriPassword
}

# Import certificate if provided
if ($CertPath -and (Test-Path $CertPath)) {
    Write-Host "Importing certificate..." -ForegroundColor Yellow
    
    # Convert to base64 if needed
    if ($CertPath.EndsWith(".pfx")) {
        $cert = Get-Content $CertPath -Encoding Byte
        $base64Cert = [Convert]::ToBase64String($cert)
        $env:WINDOWS_CERT_PFX = $base64Cert
        $env:WINDOWS_CERT_PASSWORD = $CertPassword
    }
}

Write-Host "Windows signing environment ready!" -ForegroundColor Green
EOF

cat > scripts/signing/sign-macos.sh << 'EOF'
#!/bin/bash

# macOS Signing Script

set -e

CERT_PATH=""
CERT_PASSWORD=""
TAURI_KEY=""
TAURI_PASSWORD=""
APPLE_ID=""
APPLE_PASSWORD=""
TEAM_ID=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --cert-path)
            CERT_PATH="$2"
            shift 2
            ;;
        --cert-password)
            CERT_PASSWORD="$2"
            shift 2
            ;;
        --tauri-key)
            TAURI_KEY="$2"
            shift 2
            ;;
        --tauri-password)
            TAURI_PASSWORD="$2"
            shift 2
            ;;
        --apple-id)
            APPLE_ID="$2"
            shift 2
            ;;
        --apple-password)
            APPLE_PASSWORD="$2"
            shift 2
            ;;
        --team-id)
            TEAM_ID="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo "Setting up macOS signing environment..."

# Set environment variables
if [ -n "$TAURI_KEY" ]; then
    export TAURI_PRIVATE_KEY="$TAURI_KEY"
    export TAURI_KEY_PASSWORD="$TAURI_PASSWORD"
fi

if [ -n "$CERT_PATH" ] && [ -f "$CERT_PATH" ]; then
    export APPLE_CERTIFICATE="$(cat "$CERT_PATH" | base64)"
    export APPLE_CERT_PASSWORD="$CERT_PASSWORD"
fi

if [ -n "$APPLE_ID" ]; then
    export APPLE_ID="$APPLE_ID"
    export APPLE_ID_PASSWORD="$APPLE_PASSWORD"
    export TEAM_ID="$TEAM_ID"
fi

echo "macOS signing environment ready!"
EOF

# Create GitHub Actions secrets template
cat > scripts/signing/github-secrets-template.md << 'EOF'
# GitHub Secrets Template

Copy and paste these secrets into your GitHub repository settings:

## Tauri Signing (Required for all platforms)
- `TAURI_PRIVATE_KEY`: Base64 encoded Tauri private key
- `TAURI_KEY_PASSWORD`: Password for the Tauri private key

## Windows Signing (Optional)
- `WINDOWS_CERT_PFX`: Base64 encoded .pfx certificate file
- `WINDOWS_CERT_PASSWORD`: Certificate password

## macOS Signing (Optional)
- `APPLE_CERTIFICATE`: Base64 encoded .p12 certificate
- `APPLE_CERT_PASSWORD`: Certificate password
- `APPLE_ID`: Your Apple ID
- `APPLE_ID_PASSWORD`: App-specific password
- `TEAM_ID`: Developer team ID

## Linux Signing (Optional)
- `LINUX_GPG_PRIVATE_KEY`: GPG private key for package signing

## How to set secrets:
1. Go to your GitHub repository
2. Click Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Paste the secret name and value

## Getting the values:

### Tauri Private Key:
```bash
tauri signer generate -w ~/.tauri/tauri-signer.key
cat ~/.tauri/tauri-signer.key | base64
```

### Windows Certificate:
```powershell
$cert = Get-Content "certificate.pfx" -Encoding Byte
[Convert]::ToBase64String($cert)
```

### macOS Certificate:
```bash
cat "DeveloperIDApplication.p12" | base64
```
EOF

echo "Code signing setup completed!"
echo "Refer to scripts/signing/SIGNING_GUIDE.md for detailed instructions."