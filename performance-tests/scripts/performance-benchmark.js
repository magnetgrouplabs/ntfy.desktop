#!/usr/bin/env node

/**
 * Comprehensive performance benchmark for ntfy.desktop
 * Compares Electron vs Tauri implementations
 */

import { spawn, exec } from 'child_process';
import { performance } from 'perf_hooks';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const RESULTS_DIR = path.join(__dirname, '../comparison');
const BASELINE_DIR = path.join(__dirname, '../baseline');

class PerformanceBenchmark {
  constructor() {
    this.results = {
      electron: {},
      tauri: {}
    };
  }

  async measureStartupTime(appType, iterations = 5) {
    console.log(`Measuring ${appType} startup time (${iterations} iterations)...`);
    
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
      console.log(`Iteration ${i + 1}/${iterations}`);
      
      await this.killApp(appType);
      await this.delay(2000);
      
      const startTime = performance.now();
      
      try {
        await this.startApp(appType);
        await this.waitForAppStart(appType, 10000);
        const endTime = performance.now();
        
        times.push(endTime - startTime);
        
        await this.killApp(appType);
        
      } catch (error) {
        console.error(`${appType} startup test failed:`, error);
      }
      
      await this.delay(1000);
    }
    
    return this.calculateStats(times);
  }

  async measureMemoryUsage(appType, duration = 10000) {
    console.log(`Measuring ${appType} memory usage for ${duration}ms...`);
    
    const memoryReadings = [];
    
    try {
      await this.startApp(appType);
      await this.waitForAppStart(appType, 5000);
      
      const startTime = Date.now();
      
      while (Date.now() - startTime < duration) {
        const usage = await this.getMemoryUsage(appType);
        if (usage > 0) {
          memoryReadings.push(usage);
        }
        await this.delay(500);
      }
      
      await this.killApp(appType);
      
    } catch (error) {
      console.error(`${appType} memory test failed:`, error);
    }
    
    return this.calculateStats(memoryReadings);
  }

  async measureCPUUsage(appType, duration = 10000) {
    console.log(`Measuring ${appType} CPU usage for ${duration}ms...`);
    
    const cpuReadings = [];
    
    try {
      await this.startApp(appType);
      await this.waitForAppStart(appType, 5000);
      
      const startTime = Date.now();
      
      while (Date.now() - startTime < duration) {
        const usage = await this.getCPUUsage(appType);
        if (usage > 0) {
          cpuReadings.push(usage);
        }
        await this.delay(500);
      }
      
      await this.killApp(appType);
      
    } catch (error) {
      console.error(`${appType} CPU test failed:`, error);
    }
    
    return this.calculateStats(cpuReadings);
  }

  async startApp(appType) {
    return new Promise((resolve, reject) => {
      let command;
      let args;
      let cwd;
      
      if (appType === 'electron') {
        // Use packaged Electron binary for fair benchmarking
        const electronBinary = path.join(__dirname, '../electron-app/build/ntfy-desktop-win32-x64/ntfy-desktop.exe');
        cwd = path.join(__dirname, '../electron-app/build/ntfy-desktop-win32-x64');
        command = electronBinary;
        args = [];
      } else if (appType === 'tauri') {
        // Use pre-built release binary for fair benchmarking
        const tauriBinary = path.join(__dirname, '../../src-tauri/target/release/ntfy-desktop.exe');
        cwd = path.join(__dirname, '../../');
        command = tauriBinary;
        args = [];
      }

      const childProcess = spawn(command, args, {
        cwd,
        detached: true,
        stdio: 'ignore',
        shell: true
      });
      
      childProcess.on('error', reject);
      childProcess.unref();
      
      setTimeout(resolve, 100);
    });
  }

  async killApp(appType) {
    return new Promise((resolve) => {
      // Both packaged binaries are named ntfy-desktop.exe
      // Since we test sequentially, killing by name is safe
      const processName = 'ntfy-desktop';

      if (process.platform === 'win32') {
        exec(`taskkill /F /IM ${processName}.exe`, () => resolve());
      } else {
        exec(`pkill -f ${processName}`, () => resolve());
      }
    });
  }

  async waitForAppStart(appType, timeout = 10000) {
    return new Promise((resolve) => {
      const startTime = Date.now();
      
      const processName = 'ntfy-desktop';

      const checkInterval = setInterval(() => {
        const command = process.platform === 'win32' 
          ? `tasklist | findstr ${processName}`
          : `ps aux | grep ${processName}`;
          
        exec(command, (error, stdout) => {
          if (stdout || Date.now() - startTime > timeout) {
            clearInterval(checkInterval);
            resolve();
          }
        });
      }, 100);
    });
  }

  async getMemoryUsage(appType) {
    return new Promise((resolve) => {
      let processName;
      
      processName = 'ntfy-desktop';

      if (process.platform === 'win32') {
        // Use PowerShell for more reliable memory measurement on Windows
        const command = `powershell -Command "Get-Process -Name '${processName}' -ErrorAction SilentlyContinue | Select-Object -ExpandProperty WorkingSet | Measure-Object -Sum | Select-Object -ExpandProperty Sum"`;
        exec(command, (error, stdout) => {
          if (stdout && stdout.trim()) {
            const memoryBytes = parseFloat(stdout.trim());
            resolve(isNaN(memoryBytes) ? 0 : memoryBytes / 1024 / 1024); // Convert to MB
          } else {
            resolve(0);
          }
        });
      } else {
        exec(`ps -o rss= -p $(pgrep ${processName})`, (error, stdout) => {
          if (stdout) {
            resolve(parseInt(stdout.trim()) / 1024); // Convert to MB
          }
          resolve(0);
        });
      }
    });
  }

  async getCPUUsage(appType) {
    return new Promise((resolve) => {
      let processName;
      
      processName = 'ntfy-desktop';

      if (process.platform === 'win32') {
        // Use PowerShell for more reliable CPU measurement on Windows
        const command = `powershell -Command "Get-Process -Name '${processName}' -ErrorAction SilentlyContinue | Select-Object -ExpandProperty CPU"`;
        exec(command, (error, stdout) => {
          if (stdout && stdout.trim()) {
            const cpu = parseFloat(stdout.trim());
            resolve(isNaN(cpu) ? 0 : cpu);
          } else {
            resolve(0);
          }
        });
      } else {
        exec(`ps -o %cpu= -p $(pgrep ${processName})`, (error, stdout) => {
          if (stdout) {
            resolve(parseFloat(stdout.trim()));
          }
          resolve(0);
        });
      }
    });
  }

  async delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  calculateStats(values) {
    if (values.length === 0) return null;
    
    const sorted = [...values].sort((a, b) => a - b);
    const sum = sorted.reduce((a, b) => a + b, 0);
    const avg = sum / sorted.length;
    const min = sorted[0];
    const max = sorted[sorted.length - 1];
    const median = sorted[Math.floor(sorted.length / 2)];
    
    return { avg, min, max, median, values: sorted };
  }

  generateComparisonReport(electronResults, tauriResults) {
    const report = {
      timestamp: new Date().toISOString(),
      platform: process.platform,
      electron: electronResults,
      tauri: tauriResults,
      improvements: this.calculateImprovements(electronResults, tauriResults),
      recommendations: this.generateRecommendations(electronResults, tauriResults)
    };
    
    return report;
  }

  calculateImprovements(electronResults, tauriResults) {
    const improvements = {};
    
    if (electronResults.startup && tauriResults.startup) {
      improvements.startup = {
        percentage: ((electronResults.startup.avg - tauriResults.startup.avg) / electronResults.startup.avg * 100).toFixed(1),
        absolute: (electronResults.startup.avg - tauriResults.startup.avg).toFixed(0)
      };
    }
    
    if (electronResults.memory && tauriResults.memory) {
      improvements.memory = {
        percentage: ((electronResults.memory.avg - tauriResults.memory.avg) / electronResults.memory.avg * 100).toFixed(1),
        absolute: (electronResults.memory.avg - tauriResults.memory.avg).toFixed(1)
      };
    }
    
    if (electronResults.cpu && tauriResults.cpu) {
      improvements.cpu = {
        percentage: ((electronResults.cpu.avg - tauriResults.cpu.avg) / electronResults.cpu.avg * 100).toFixed(1),
        absolute: (electronResults.cpu.avg - tauriResults.cpu.avg).toFixed(1)
      };
    }
    
    return improvements;
  }

  generateRecommendations(electronResults, tauriResults) {
    const recommendations = [];
    
    if (tauriResults.startup && tauriResults.startup.avg > 3000) {
      recommendations.push('Tauri startup time >3s - optimize initial resource loading');
    }
    
    if (tauriResults.memory && tauriResults.memory.avg > 100) {
      recommendations.push('Tauri memory usage >100MB - investigate memory leaks');
    }
    
    if (tauriResults.cpu && tauriResults.cpu.avg > 5) {
      recommendations.push('Tauri CPU usage >5% - optimize background processes');
    }
    
    return recommendations.length > 0 ? recommendations : ['Performance is within acceptable ranges'];
  }

  async saveResults(report, type = 'comparison') {
    const dir = type === 'baseline' ? BASELINE_DIR : RESULTS_DIR;
    const filename = `${type}-${Date.now()}.json`;
    const filepath = path.join(dir, filename);
    
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
    
    fs.writeFileSync(filepath, JSON.stringify(report, null, 2));
    console.log(`Results saved to: ${filepath}`);
    
    return filepath;
  }

  async runBaseline() {
    console.log('Running baseline performance tests for Electron app...\n');
    
    const electronResults = {
      startup: await this.measureStartupTime('electron'),
      memory: await this.measureMemoryUsage('electron'),
      cpu: await this.measureCPUUsage('electron')
    };
    
    const report = {
      timestamp: new Date().toISOString(),
      platform: process.platform,
      appType: 'electron',
      results: electronResults
    };
    
    await this.saveResults(report, 'baseline');
    
    console.log('\n=== BASELINE PERFORMANCE RESULTS ===');
    console.log(`Startup Time: ${electronResults.startup?.avg?.toFixed(0) || 'N/A'}ms`);
    console.log(`Memory Usage: ${electronResults.memory?.avg?.toFixed(1) || 'N/A'}MB`);
    console.log(`CPU Usage: ${electronResults.cpu?.avg?.toFixed(1) || 'N/A'}%`);
    
    return report;
  }

  async runComparison() {
    console.log('Running performance comparison between Electron and Tauri...\n');
    
    const electronResults = {
      startup: await this.measureStartupTime('electron'),
      memory: await this.measureMemoryUsage('electron'),
      cpu: await this.measureCPUUsage('electron')
    };
    
    const tauriResults = {
      startup: await this.measureStartupTime('tauri'),
      memory: await this.measureMemoryUsage('tauri'),
      cpu: await this.measureCPUUsage('tauri')
    };
    
    const report = this.generateComparisonReport(electronResults, tauriResults);
    await this.saveResults(report);
    
    console.log('\n=== PERFORMANCE COMPARISON RESULTS ===');
    console.log('\nElectron:');
    console.log(`  Startup: ${electronResults.startup.avg.toFixed(0)}ms`);
    console.log(`  Memory: ${electronResults.memory.avg.toFixed(1)}MB`);
    console.log(`  CPU: ${electronResults.cpu.avg.toFixed(1)}%`);
    
    console.log('\nTauri:');
    console.log(`  Startup: ${tauriResults.startup.avg.toFixed(0)}ms`);
    console.log(`  Memory: ${tauriResults.memory.avg.toFixed(1)}MB`);
    console.log(`  CPU: ${tauriResults.cpu.avg.toFixed(1)}%`);
    
    console.log('\nImprovements:');
    console.log(`  Startup: ${report.improvements.startup.percentage}% faster (${report.improvements.startup.absolute}ms)`);
    console.log(`  Memory: ${report.improvements.memory.percentage}% reduction (${report.improvements.memory.absolute}MB)`);
    console.log(`  CPU: ${report.improvements.cpu.percentage}% reduction (${report.improvements.cpu.absolute}%)`);
    
    console.log('\nRecommendations:');
    report.recommendations.forEach(rec => console.log(`- ${rec}`));
    
    return report;
  }
}

// Run benchmark when script is executed directly
const benchmark = new PerformanceBenchmark();

const args = process.argv.slice(2);
const mode = args[0] || 'comparison';

if (mode === 'baseline') {
  benchmark.runBaseline().catch(console.error);
} else {
  benchmark.runComparison().catch(console.error);
}

export default PerformanceBenchmark;