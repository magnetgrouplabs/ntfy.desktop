# Windows Build Script for ntfy.desktop

param(
    [switch]$Release = $false,
    [switch]$Sign = $false,
    [string]$CertThumbprint = ""
)

Write-Host "Building ntfy.desktop for Windows..." -ForegroundColor Green

# Install dependencies
Write-Host "Installing dependencies..." -ForegroundColor Yellow
npm install

# Update version for release builds
if ($Release) {
    Write-Host "Updating version for release build..." -ForegroundColor Yellow
    bash scripts/versioning.sh --release
}

# Build frontend
Write-Host "Building frontend..." -ForegroundColor Yellow
npm run build

# Set build mode
$BuildMode = if ($Release) { "--release" } else { "" }

# Build Tauri application
Write-Host "Building Tauri application..." -ForegroundColor Yellow
if ($Sign -and $CertThumbprint) {
    $env:TAURI_PRIVATE_KEY = "" # Will be set externally
    $env:TAURI_KEY_PASSWORD = "" # Will be set externally
    $env:CERT_THUMBPRINT = $CertThumbprint

    npm run tauri:build $BuildMode
} else {
    npm run tauri:build $BuildMode
}

# Copy portable executable
Write-Host "Creating portable distribution..." -ForegroundColor Yellow
$TargetDir = "dist/windows"
New-Item -ItemType Directory -Force -Path $TargetDir

$ExePath = "src-tauri/target/release/ntfy-desktop.exe"
if (Test-Path $ExePath) {
    Copy-Item $ExePath "$TargetDir/ntfy.desktop-portable.exe"
    Write-Host "Portable executable created: $TargetDir/ntfy.desktop-portable.exe" -ForegroundColor Green
}

# Check for MSI installer
$MsiPath = "src-tauri/target/release/bundle/msi/*.msi"
if (Test-Path $MsiPath) {
    Copy-Item $MsiPath $TargetDir
    Write-Host "MSI installer created in: $TargetDir" -ForegroundColor Green
}

Write-Host "Windows build completed!" -ForegroundColor Green
