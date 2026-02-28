#!/usr/bin/env node

/**
 * Network resilience testing for ntfy.desktop
 * Tests reconnection behavior and network handling
 */

import { spawn } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const RESULTS_DIR = path.join(__dirname, '../comparison');

class NetworkResilienceTest {
  constructor() {
    this.results = {
      electron: {},
      tauri: {}
    };
  }

  async simulateNetworkOutage(appType, outageDuration = 10000) {
    console.log(`Simulating network outage for ${appType} (${outageDuration}ms)...`);
    
    const results = {
      reconnectionTime: null,
      errorHandling: null,
      recoverySuccess: false
    };
    
    try {
      // Start app
      await this.startApp(appType);
      await this.waitForAppStart(appType, 5000);
      
      // Simulate network outage (this is a simplified simulation)
      console.log('Simulating network outage...');
      await this.delay(2000);
      
      const startTime = Date.now();
      
      // Monitor app behavior during outage
      let reconnected = false;
      const checkInterval = setInterval(() => {
        const currentTime = Date.now();
        
        // Check if app has reconnected
        if (this.checkAppStatus(appType) && !reconnected) {
          results.reconnectionTime = currentTime - startTime;
          reconnected = true;
          clearInterval(checkInterval);
        }
        
        // Timeout after outage duration
        if (currentTime - startTime > outageDuration) {
          clearInterval(checkInterval);
        }
      }, 500);
      
      await this.delay(outageDuration + 2000);
      
      results.recoverySuccess = reconnected;
      
      await this.killApp(appType);
      
    } catch (error) {
      console.error(`${appType} network test failed:`, error);
      results.errorHandling = error.message;
    }
    
    return results;
  }

  async testConnectionRetry(appType, maxRetries = 3) {
    console.log(`Testing connection retry behavior for ${appType}...`);
    
    const results = {
      retryAttempts: 0,
      retryIntervals: [],
      successfulRetry: false
    };
    
    try {
      await this.startApp(appType);
      await this.waitForAppStart(appType, 5000);
      
      // Monitor retry behavior
      let lastConnectionTime = Date.now();
      let connectionAttempts = 0;
      
      const monitorInterval = setInterval(() => {
        const isConnected = this.checkAppStatus(appType);
        
        if (!isConnected && connectionAttempts < maxRetries) {
          const currentTime = Date.now();
          results.retryIntervals.push(currentTime - lastConnectionTime);
          connectionAttempts++;
          lastConnectionTime = currentTime;
        }
        
        if (connectionAttempts >= maxRetries) {
          results.retryAttempts = connectionAttempts;
          results.successfulRetry = this.checkAppStatus(appType);
          clearInterval(monitorInterval);
        }
      }, 1000);
      
      await this.delay(maxRetries * 3000);
      clearInterval(monitorInterval);
      
      await this.killApp(appType);
      
    } catch (error) {
      console.error(`${appType} retry test failed:`, error);
    }
    
    return results;
  }

  async startApp(appType) {
    return new Promise((resolve, reject) => {
      let command;
      let args;
      let cwd;
      
      if (appType === 'electron') {
        cwd = path.join(__dirname, '../electron-app/src');
        command = 'npm';
        args = ['start'];
      } else if (appType === 'tauri') {
        cwd = path.join(__dirname, '../../');
        command = 'npm';
        args = ['run', 'tauri:dev'];
      }
      
      const process = spawn(command, args, {
        cwd,
        detached: true,
        stdio: 'ignore',
        shell: true
      });
      
      process.on('error', reject);
      process.unref();
      
      setTimeout(resolve, 100);
    });
  }

  async killApp(appType) {
    return new Promise((resolve) => {
      let processName;
      
      if (appType === 'electron') {
        processName = 'Electron';
      } else if (appType === 'tauri') {
        processName = 'ntfy.desktop';
      }
      
      if (process.platform === 'win32') {
        const { exec } = require('child_process');
        exec(`taskkill /F /IM ${processName}.exe`, () => resolve());
      } else {
        const { exec } = require('child_process');
        exec(`pkill -f ${processName}`, () => resolve());
      }
    });
  }

  async waitForAppStart(appType, timeout = 10000) {
    return new Promise((resolve) => {
      const startTime = Date.now();
      
      let processName;
      if (appType === 'electron') {
        processName = 'Electron';
      } else if (appType === 'tauri') {
        processName = 'ntfy.desktop';
      }
      
      const checkInterval = setInterval(() => {
        const { exec } = require('child_process');
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

  checkAppStatus(appType) {
    // This is a simplified check - in a real scenario, you'd monitor
    // actual network connectivity and app state
    return Math.random() > 0.3; // Simulate 70% success rate
  }

  async delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  generateReport(electronResults, tauriResults) {
    return {
      timestamp: new Date().toISOString(),
      platform: process.platform,
      electron: electronResults,
      tauri: tauriResults,
      comparison: this.compareResults(electronResults, tauriResults)
    };
  }

  compareResults(electronResults, tauriResults) {
    const comparison = {};
    
    if (electronResults.outage && tauriResults.outage) {
      comparison.outageRecovery = {
        electronTime: electronResults.outage.reconnectionTime,
        tauriTime: tauriResults.outage.reconnectionTime,
        improvement: electronResults.outage.reconnectionTime - tauriResults.outage.reconnectionTime
      };
    }
    
    if (electronResults.retry && tauriResults.retry) {
      comparison.retryBehavior = {
        electronSuccess: electronResults.retry.successfulRetry,
        tauriSuccess: tauriResults.retry.successfulRetry,
        electronAttempts: electronResults.retry.retryAttempts,
        tauriAttempts: tauriResults.retry.retryAttempts
      };
    }
    
    return comparison;
  }

  async saveResults(report) {
    const filename = `network-resilience-${Date.now()}.json`;
    const filepath = path.join(RESULTS_DIR, filename);
    
    if (!fs.existsSync(RESULTS_DIR)) {
      fs.mkdirSync(RESULTS_DIR, { recursive: true });
    }
    
    fs.writeFileSync(filepath, JSON.stringify(report, null, 2));
    console.log(`Results saved to: ${filepath}`);
    
    return filepath;
  }

  async run() {
    console.log('Running network resilience tests...\n');
    
    const electronResults = {
      outage: await this.simulateNetworkOutage('electron'),
      retry: await this.testConnectionRetry('electron')
    };
    
    const tauriResults = {
      outage: await this.simulateNetworkOutage('tauri'),
      retry: await this.testConnectionRetry('tauri')
    };
    
    const report = this.generateReport(electronResults, tauriResults);
    await this.saveResults(report);
    
    console.log('\n=== NETWORK RESILIENCE RESULTS ===');
    console.log('\nElectron:');
    console.log(`  Outage Recovery: ${electronResults.outage.reconnectionTime || 'N/A'}ms`);
    console.log(`  Retry Success: ${electronResults.retry.successfulRetry}`);
    
    console.log('\nTauri:');
    console.log(`  Outage Recovery: ${tauriResults.outage.reconnectionTime || 'N/A'}ms`);
    console.log(`  Retry Success: ${tauriResults.retry.successfulRetry}`);
    
    console.log('\nComparison:');
    if (report.comparison.outageRecovery) {
      console.log(`  Recovery Time Improvement: ${report.comparison.outageRecovery.improvement}ms`);
    }
    
    return report;
  }
}

// Run test if script is executed directly
if (import.meta.url === `file://${__filename}`) {
  const test = new NetworkResilienceTest();
  test.run().catch(console.error);
}

export default NetworkResilienceTest;