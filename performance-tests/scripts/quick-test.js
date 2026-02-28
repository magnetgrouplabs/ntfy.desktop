#!/usr/bin/env node

/**
 * Quick performance test to demonstrate the framework
 * Runs a simplified version of the comparison tests
 */

import { spawn, exec } from 'child_process';
import { performance } from 'perf_hooks';

class QuickTest {
  constructor() {
    this.results = {};
  }

  async measureStartup(appType) {
    console.log(`Measuring ${appType} startup time...`);
    
    await this.killApp(appType);
    await this.delay(2000);
    
    const startTime = performance.now();
    
    try {
      await this.startApp(appType);
      await this.waitForAppStart(appType, 10000);
      const endTime = performance.now();
      
      await this.killApp(appType);
      
      return endTime - startTime;
    } catch (error) {
      console.error(`${appType} startup test failed:`, error);
      return null;
    }
  }

  async startApp(appType) {
    return new Promise((resolve, reject) => {
      let command, args, cwd;
      
      if (appType === 'electron') {
        cwd = './electron-app/src';
        command = 'npm';
        args = ['start'];
      } else if (appType === 'tauri') {
        cwd = '../';
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
      let processName = appType === 'electron' ? 'Electron' : 'ntfy.desktop';
      
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
      
      let processName = appType === 'electron' ? 'Electron' : 'ntfy.desktop';
      
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

  async delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  async run() {
    console.log('Running quick performance comparison...\n');
    
    // Measure Electron startup
    const electronTime = await this.measureStartup('electron');
    
    if (electronTime) {
      console.log(`Electron startup: ${electronTime.toFixed(0)}ms`);
    }
    
    await this.delay(3000);
    
    // Measure Tauri startup
    const tauriTime = await this.measureStartup('tauri');
    
    if (tauriTime) {
      console.log(`Tauri startup: ${tauriTime.toFixed(0)}ms`);
    }
    
    // Calculate improvement
    if (electronTime && tauriTime) {
      const improvement = ((electronTime - tauriTime) / electronTime * 100).toFixed(1);
      console.log(`\nImprovement: ${improvement}% faster`);
      
      console.log('\n=== QUICK TEST RESULTS ===');
      console.log(`Electron: ${electronTime.toFixed(0)}ms`);
      console.log(`Tauri: ${tauriTime.toFixed(0)}ms`);
      console.log(`Improvement: ${improvement}%`);
    } else {
      console.log('\nTest incomplete. One or both applications failed to start.');
    }
    
    console.log('\nFor comprehensive testing, run:');
    console.log('npm run test:comparison');
  }
}

// Run test if script is executed directly
if (import.meta.url.includes(process.argv[1])) {
  const test = new QuickTest();
  test.run().catch(console.error);
}

export default QuickTest;