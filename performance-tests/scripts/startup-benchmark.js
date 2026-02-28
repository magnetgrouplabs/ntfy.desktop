#!/usr/bin/env node

/**
 * Startup performance benchmark for ntfy.desktop
 * Measures cold and warm startup times
 */

import { spawn, exec } from 'child_process';
import { performance } from 'perf_hooks';
import fs from 'fs';
import path from 'path';

const RESULTS_DIR = '../baseline';
const BUILD_PATH = '../dist';
const APP_NAME = 'ntfy.desktop';

class StartupBenchmark {
  constructor() {
    this.results = {
      coldStartup: [],
      warmStartup: [],
      memoryUsage: [],
      cpuUsage: []
    };
  }

  async runColdStartupTest(iterations = 10) {
    console.log(`Running ${iterations} cold startup tests...`);
    
    for (let i = 0; i < iterations; i++) {
      console.log(`Iteration ${i + 1}/${iterations}`);
      
      // Kill any running instances
      await this.killApp();
      
      // Wait for process to fully terminate
      await this.delay(2000);
      
      const startTime = performance.now();
      
      try {
        const appProcess = spawn('npm', ['run', 'tauri', 'dev'], {
          detached: true,
          stdio: 'ignore'
        });
        
        // Wait for app to start
        await this.waitForAppStart(5000);
        const endTime = performance.now();
        
        this.results.coldStartup.push(endTime - startTime);
        
        // Kill app immediately after measurement
        await this.killApp();
        
      } catch (error) {
        console.error('Cold startup test failed:', error);
      }
      
      await this.delay(1000);
    }
  }

  async runWarmStartupTest(iterations = 10) {
    console.log(`Running ${iterations} warm startup tests...`);
    
    // Start app once to warm up
    console.log('Warming up app...');
    await this.startApp();
    await this.delay(3000);
    await this.killApp();
    await this.delay(1000);
    
    for (let i = 0; i < iterations; i++) {
      console.log(`Iteration ${i + 1}/${iterations}`);
      
      const startTime = performance.now();
      
      try {
        await this.startApp();
        await this.waitForAppStart(3000);
        const endTime = performance.now();
        
        this.results.warmStartup.push(endTime - startTime);
        
        await this.killApp();
        
      } catch (error) {
        console.error('Warm startup test failed:', error);
      }
      
      await this.delay(500);
    }
  }

  async startApp() {
    return new Promise((resolve, reject) => {
      const process = spawn('npm', ['run', 'tauri', 'dev'], {
        detached: true,
        stdio: 'ignore'
      });
      
      process.on('error', reject);
      process.unref();
      
      // Give process time to start
      setTimeout(resolve, 100);
    });
  }

  async killApp() {
    return new Promise((resolve) => {
      if (process.platform === 'win32') {
        exec('taskkill /F /IM ntfy.desktop.exe', () => resolve());
      } else {
        exec('pkill -f ntfy.desktop', () => resolve());
      }
    });
  }

  async waitForAppStart(timeout = 5000) {
    return new Promise((resolve) => {
      const startTime = Date.now();
      
      const checkInterval = setInterval(() => {
        exec('tasklist | findstr ntfy.desktop', (error, stdout) => {
          if (stdout || Date.now() - startTime > timeout) {
            clearInterval(checkInterval);
            resolve();
          }
        });
      }, 100);
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

  generateReport() {
    const coldStats = this.calculateStats(this.results.coldStartup);
    const warmStats = this.calculateStats(this.results.warmStartup);
    
    const report = {
      timestamp: new Date().toISOString(),
      platform: process.platform,
      coldStartup: coldStats,
      warmStartup: warmStats,
      recommendations: this.generateRecommendations(coldStats, warmStats)
    };
    
    return report;
  }

  generateRecommendations(coldStats, warmStats) {
    const recommendations = [];
    
    if (coldStats && coldStats.avg > 3000) {
      recommendations.push('Cold startup time >3s - consider optimizing initial resource loading');
    }
    
    if (warmStats && warmStats.avg > 1000) {
      recommendations.push('Warm startup time >1s - investigate caching opportunities');
    }
    
    if (coldStats && warmStats && coldStats.avg / warmStats.avg > 3) {
      recommendations.push('Large gap between cold/warm startup - optimize resource initialization');
    }
    
    return recommendations.length > 0 ? recommendations : ['Startup performance is within acceptable ranges'];
  }

  async saveResults() {
    const report = this.generateReport();
    const filename = `startup-${Date.now()}.json`;
    const filepath = path.join(RESULTS_DIR, filename);
    
    if (!fs.existsSync(RESULTS_DIR)) {
      fs.mkdirSync(RESULTS_DIR, { recursive: true });
    }
    
    fs.writeFileSync(filepath, JSON.stringify(report, null, 2));
    console.log(`Results saved to: ${filepath}`);
    
    return report;
  }

  async run() {
    console.log('Starting startup performance benchmark...\n');
    
    await this.runColdStartupTest(5);
    await this.runWarmStartupTest(5);
    
    const report = await this.saveResults();
    
    console.log('\n=== STARTUP PERFORMANCE RESULTS ===');
    console.log(`Cold Startup: ${report.coldStartup.avg.toFixed(0)}ms avg`);
    console.log(`Warm Startup: ${report.warmStartup.avg.toFixed(0)}ms avg`);
    console.log('\nRecommendations:');
    report.recommendations.forEach(rec => console.log(`- ${rec}`));
  }
}

// Run benchmark if script is executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  const benchmark = new StartupBenchmark();
  benchmark.run().catch(console.error);
}

export default StartupBenchmark;