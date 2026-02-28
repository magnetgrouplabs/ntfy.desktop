# Tauri Windows GitHub Actions Build Guide

Based on comprehensive research of Tauri Windows build requirements and analysis of successful workflows.

## Key Findings

### Current Project Status ✅
- **Tauri 2.0** is properly configured
- **Windows build dependencies** are correctly specified in Cargo.toml
- **Existing workflows** successfully build Windows artifacts
- **Bundle configuration** supports both MSI and portable EXE formats

### Windows-Specific Requirements

#### GitHub Actions Environment
- **Runner**: `windows-latest` (includes Windows SDK and Visual Studio)
- **Rust toolchain**: `stable-x86_64-msvc`
- **Target**: `x86_64-pc-windows-msvc`

#### Build Tools Available
- **Windows SDK**: Pre-installed on GitHub runners
- **Visual Studio Build Tools**: Included
- **WiX Toolset**: Required for MSI generation
- **NSIS**: Required for EXE installer generation

## Optimized Workflow Configurations

### 1. Windows Build Workflow (`windows-build.yml`)
```yaml
# Triggers on code changes, validates Windows builds
# Includes comprehensive environment verification
```

### 2. Windows Release Workflow (`windows-release.yml`)
```yaml
# Creates releases on version tags
# Uses tauri-action for automated release creation
# Includes installer validation
```

## Best Practices

### Dependency Management
```yaml
- Use `npm ci` for reproducible builds
- Cache Rust dependencies with Swatinem/rust-cache
- Specify exact Rust toolchain and target
```

### Build Optimization
```yaml
- Use `--release` flag for production builds
- Set `RUSTFLAGS: "-C target-cpu=native"` for performance
- Enable verbose logging with `--verbose`
```

### Installer Validation
```powershell
- File size checks (minimum 1KB)
- File integrity validation
- PE header verification for executables
- Multiple installer format support (.msi, .exe, -setup.exe)
```

## Key Commands for Windows Builds

### Basic Build
```bash
npm run tauri:build
```

### Release Build with Optimizations
```bash
npm run tauri:build -- --release --verbose
```

### Platform-Specific Targeting
```bash
# For 64-bit Windows (default)
tauri build --target x86_64-pc-windows-msvc

# For 32-bit Windows
tauri build --target i686-pc-windows-msvc

# For ARM64 Windows
tauri build --target aarch64-pc-windows-msvc
```

## Installer Types

### MSI Installer (.msi)
- **Format**: Windows Installer package
- **Usage**: System-wide installation
- **Requirements**: WiX Toolset
- **Advantages**: Standard Windows installer experience

### NSIS Installer (-setup.exe)
- **Format**: Nullsoft Scriptable Install System
- **Usage**: User-friendly installation wizard
- **Requirements**: NSIS
- **Advantages**: Portable, customizable

### Portable Executable (.exe)
- **Format**: Standalone executable
- **Usage**: No installation required
- **Advantages**: Simple distribution

## Environment Verification

GitHub Actions Windows runners include:
- ✅ Windows SDK
- ✅ Visual Studio Build Tools
- ✅ WiX Toolset
- ✅ NSIS
- ✅ WebView2 Runtime (for testing)

## Performance Optimizations

### Rust Compiler Flags
```yaml
RUSTFLAGS: "-C target-cpu=native"
```

### Dependency Caching
```yaml
- actions/setup-node with npm cache
- Swatinem/rust-cache for Rust dependencies
```

## Security Considerations

### Code Signing (Optional)
```yaml
# Requires certificate setup
env:
  TAURI_PRIVATE_KEY: ${{ secrets.TAURI_PRIVATE_KEY }}
  TAURI_KEY_PASSWORD: ${{ secrets.TAURI_KEY_PASSWORD }}
```

### WebView2 Installation
- **Default**: Downloads bootstrapper if needed
- **Offline**: Can embed WebView2 installer (~127MB)
- **Fixed version**: Embed specific WebView2 version (~180MB)

## Troubleshooting

### Common Issues
1. **Missing Windows SDK**: Verify runner environment
2. **WiX Toolset errors**: Check toolchain installation
3. **WebView2 issues**: Configure installation mode in tauri.conf.json

### Debug Commands
```powershell
# Verify Windows SDK
Get-ChildItem "C:\Program Files (x86)\Windows Kits"

# Verify Visual Studio
Get-ChildItem "C:\Program Files\Microsoft Visual Studio"

# Verify WiX Toolset
Get-Command heat.exe, candle.exe, light.exe
```

## Conclusion

The provided workflows implement industry best practices for Tauri Windows builds on GitHub Actions:

✅ **Comprehensive environment verification**
✅ **Optimized build performance**
✅ **Multiple installer format support**
✅ **Automated release creation**
✅ **Installation validation**

These workflows ensure reliable Windows builds with proper error handling and validation.
