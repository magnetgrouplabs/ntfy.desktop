#!/usr/bin/env node

/**
 * Verifies that both Electron and Tauri apps are set up correctly
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

class SetupVerifier {
  constructor() {
    this.checks = [];
  }

  checkFileExists(filePath, description) {
    const exists = fs.existsSync(filePath);
    this.checks.push({
      description,
      status: exists ? '✅' : '❌',
      path: filePath
    });
    return exists;
  }

  checkDirectoryExists(dirPath, description) {
    const exists = fs.existsSync(dirPath) && fs.statSync(dirPath).isDirectory();
    this.checks.push({
      description,
      status: exists ? '✅' : '❌',
      path: dirPath
    });
    return exists;
  }

  checkPackageJson(dirPath, description) {
    const packagePath = path.join(dirPath, 'package.json');
    const exists = fs.existsSync(packagePath);
    
    let valid = false;
    if (exists) {
      try {
        const pkg = JSON.parse(fs.readFileSync(packagePath, 'utf8'));
        valid = pkg.name && pkg.scripts;
      } catch (error) {
        valid = false;
      }
    }
    
    this.checks.push({
      description,
      status: valid ? '✅' : '❌',
      path: packagePath
    });
    return valid;
  }

  verifyElectronApp() {
    console.log('Verifying Electron app setup...');
    
    const electronDir = path.join(__dirname, '../electron-app/src');
    
    this.checkDirectoryExists(electronDir, 'Electron app directory');
    this.checkPackageJson(electronDir, 'Electron package.json');
    this.checkFileExists(path.join(electronDir, 'index.js'), 'Electron main process');
    this.checkFileExists(path.join(electronDir, 'renderer.js'), 'Electron renderer');
  }

  verifyTauriApp() {
    console.log('Verifying Tauri app setup...');
    
    const tauriDir = path.join(__dirname, '../../');
    const tauriSrcDir = path.join(tauriDir, 'src-tauri');
    
    this.checkDirectoryExists(tauriDir, 'Tauri project directory');
    this.checkPackageJson(tauriDir, 'Tauri package.json');
    this.checkDirectoryExists(tauriSrcDir, 'Tauri src directory');
    this.checkFileExists(path.join(tauriSrcDir, 'Cargo.toml'), 'Tauri Cargo.toml');
    this.checkFileExists(path.join(tauriSrcDir, 'src/main.rs'), 'Tauri main.rs');
  }

  verifyPerformanceTests() {
    console.log('Verifying performance test setup...');
    
    const scriptsDir = path.join(__dirname, '../scripts');
    
    this.checkDirectoryExists(scriptsDir, 'Performance scripts directory');
    this.checkFileExists(path.join(scriptsDir, 'performance-benchmark.js'), 'Performance benchmark script');
    this.checkFileExists(path.join(scriptsDir, 'network-resilience.js'), 'Network resilience script');
    this.checkFileExists(path.join(scriptsDir, 'generate-report.js'), 'Report generator script');
    this.checkFileExists(path.join(__dirname, '../package.json'), 'Performance tests package.json');
  }

  generateReport() {
    console.log('\n=== SETUP VERIFICATION REPORT ===\n');
    
    this.checks.forEach(check => {
      console.log(`${check.status} ${check.description}`);
      console.log(`   Path: ${check.path}\n`);
    });
    
    const passed = this.checks.every(check => check.status === '✅');
    
    if (passed) {
      console.log('✅ All checks passed! Setup is ready for performance testing.');
      console.log('\nNext steps:');
      console.log('1. Run: npm run test:baseline (to establish Electron baseline)');
      console.log('2. Run: npm run test:comparison (to compare Electron vs Tauri)');
      console.log('3. Run: npm run performance:report (to generate README-ready report)');
    } else {
      console.log('❌ Some checks failed. Please fix the issues above.');
      console.log('\nCommon issues:');
      console.log('- Ensure Electron app is built: cd electron-app/src && npm install');
      console.log('- Ensure Tauri app is built: npm install && npm run tauri:build');
      console.log('- Check file paths and permissions');
    }
    
    return passed;
  }

  run() {
    console.log('Verifying ntfy.desktop performance testing setup...\n');
    
    this.verifyElectronApp();
    this.verifyTauriApp();
    this.verifyPerformanceTests();
    
    return this.generateReport();
  }
}

// Run verifier if script is executed directly
if (import.meta.url === `file://${__filename}`) {
  const verifier = new SetupVerifier();
  const success = verifier.run();
  process.exit(success ? 0 : 1);
}

export default SetupVerifier;