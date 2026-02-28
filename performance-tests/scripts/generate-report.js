#!/usr/bin/env node

/**
 * Generates README-ready performance comparison reports
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const COMPARISON_DIR = path.join(__dirname, '../comparison');
const BASELINE_DIR = path.join(__dirname, '../baseline');

class ReportGenerator {
  constructor() {
    this.comparisons = [];
    this.baselines = [];
  }

  loadResults() {
    // Load latest comparison results
    if (fs.existsSync(COMPARISON_DIR)) {
      const files = fs.readdirSync(COMPARISON_DIR)
        .filter(file => file.endsWith('.json'))
        .sort()
        .reverse();
      
      if (files.length > 0) {
        const latestFile = path.join(COMPARISON_DIR, files[0]);
        const data = JSON.parse(fs.readFileSync(latestFile, 'utf8'));
        this.comparisons.push(data);
      }
    }

    // Load baseline results
    if (fs.existsSync(BASELINE_DIR)) {
      const files = fs.readdirSync(BASELINE_DIR)
        .filter(file => file.endsWith('.json'))
        .sort()
        .reverse();
      
      if (files.length > 0) {
        const latestFile = path.join(BASELINE_DIR, files[0]);
        const data = JSON.parse(fs.readFileSync(latestFile, 'utf8'));
        this.baselines.push(data);
      }
    }
  }

  generateMarkdownReport() {
    if (this.comparisons.length === 0) {
      return '# Performance Comparison Report\n\nNo comparison data available. Run tests first.';
    }

    const latest = this.comparisons[0];
    const baseline = this.baselines[0];

    let report = `# Performance Comparison Report\n\n`;
    report += `**Generated**: ${new Date(latest.timestamp).toLocaleString()}\n`;
    report += `**Platform**: ${latest.platform}\n\n`;

    report += `## Executive Summary\n\n`;

    if (latest.improvements) {
      report += `### Key Improvements\n\n`;
      report += `| Metric | Improvement | Details |\n`;
      report += `|--------|-------------|---------|\n`;
      
      if (latest.improvements.startup) {
        report += `| Startup Time | ${latest.improvements.startup.percentage}% faster | ${latest.improvements.startup.absolute}ms reduction |\n`;
      }
      
      if (latest.improvements.memory) {
        report += `| Memory Usage | ${latest.improvements.memory.percentage}% reduction | ${latest.improvements.memory.absolute}MB saved |\n`;
      }
      
      if (latest.improvements.cpu) {
        report += `| CPU Usage | ${latest.improvements.cpu.percentage}% reduction | ${latest.improvements.cpu.absolute}% lower |\n`;
      }
      
      report += `\n`;
    }

    report += `## Detailed Metrics\n\n`;

    // Startup Time Comparison
    if (latest.electron.startup && latest.tauri.startup) {
      report += `### Startup Time (ms)\n\n`;
      report += `| Metric | Electron | Tauri | Improvement |\n`;
      report += `|--------|----------|-------|-------------|\n`;
      report += `| Average | ${latest.electron.startup.avg.toFixed(0)} | ${latest.tauri.startup.avg.toFixed(0)} | ${((latest.electron.startup.avg - latest.tauri.startup.avg) / latest.electron.startup.avg * 100).toFixed(1)}% |\n`;
      report += `| Minimum | ${latest.electron.startup.min.toFixed(0)} | ${latest.tauri.startup.min.toFixed(0)} | - |\n`;
      report += `| Maximum | ${latest.electron.startup.max.toFixed(0)} | ${latest.tauri.startup.max.toFixed(0)} | - |\n`;
      report += `| Median | ${latest.electron.startup.median.toFixed(0)} | ${latest.tauri.startup.median.toFixed(0)} | - |\n`;
      report += `\n`;
    }

    // Memory Usage Comparison
    if (latest.electron.memory && latest.tauri.memory) {
      report += `### Memory Usage (MB)\n\n`;
      report += `| Metric | Electron | Tauri | Improvement |\n`;
      report += `|--------|----------|-------|-------------|\n`;
      report += `| Average | ${latest.electron.memory.avg.toFixed(1)} | ${latest.tauri.memory.avg.toFixed(1)} | ${((latest.electron.memory.avg - latest.tauri.memory.avg) / latest.electron.memory.avg * 100).toFixed(1)}% |\n`;
      report += `| Minimum | ${latest.electron.memory.min.toFixed(1)} | ${latest.tauri.memory.min.toFixed(1)} | - |\n`;
      report += `| Maximum | ${latest.electron.memory.max.toFixed(1)} | ${latest.tauri.memory.max.toFixed(1)} | - |\n`;
      report += `| Median | ${latest.electron.memory.median.toFixed(1)} | ${latest.tauri.memory.median.toFixed(1)} | - |\n`;
      report += `\n`;
    }

    // CPU Usage Comparison
    if (latest.electron.cpu && latest.tauri.cpu) {
      report += `### CPU Usage (%)\n\n`;
      report += `| Metric | Electron | Tauri | Improvement |\n`;
      report += `|--------|----------|-------|-------------|\n`;
      report += `| Average | ${latest.electron.cpu.avg.toFixed(1)} | ${latest.tauri.cpu.avg.toFixed(1)} | ${((latest.electron.cpu.avg - latest.tauri.cpu.avg) / latest.electron.cpu.avg * 100).toFixed(1)}% |\n`;
      report += `| Minimum | ${latest.electron.cpu.min.toFixed(1)} | ${latest.tauri.cpu.min.toFixed(1)} | - |\n`;
      report += `| Maximum | ${latest.electron.cpu.max.toFixed(1)} | ${latest.tauri.cpu.max.toFixed(1)} | - |\n`;
      report += `| Median | ${latest.electron.cpu.median.toFixed(1)} | ${latest.tauri.cpu.median.toFixed(1)} | - |\n`;
      report += `\n`;
    }

    // Baseline Comparison
    if (baseline) {
      report += `## Baseline Performance\n\n`;
      report += `Based on historical Electron measurements:\n\n`;
      report += `- **Startup Time**: ~5 seconds\n`;
      report += `- **Memory Usage**: 150-250MB\n`;
      report += `- **CPU Usage**: 3-8% (idle)\n\n`;
    }

    // Recommendations
    if (latest.recommendations) {
      report += `## Recommendations\n\n`;
      latest.recommendations.forEach(rec => {
        report += `- ${rec}\n`;
      });
      report += `\n`;
    }

    // Performance Targets
    report += `## Performance Targets\n\n`;
    report += `| Metric | Target | Current | Status |\n`;
    report += `|--------|--------|---------|--------|\n`;
    
    if (latest.tauri.startup) {
      const startupStatus = latest.tauri.startup.avg < 3000 ? '✅' : '⚠️';
      report += `| Startup Time | <3s | ${(latest.tauri.startup.avg / 1000).toFixed(1)}s | ${startupStatus} |\n`;
    }
    
    if (latest.tauri.memory) {
      const memoryStatus = latest.tauri.memory.avg < 100 ? '✅' : '⚠️';
      report += `| Memory Usage | <100MB | ${latest.tauri.memory.avg.toFixed(1)}MB | ${memoryStatus} |\n`;
    }
    
    if (latest.tauri.cpu) {
      const cpuStatus = latest.tauri.cpu.avg < 5 ? '✅' : '⚠️';
      report += `| CPU Usage | <5% | ${latest.tauri.cpu.avg.toFixed(1)}% | ${cpuStatus} |\n`;
    }
    
    report += `\n`;

    return report;
  }

  generateComparisonTable() {
    if (this.comparisons.length === 0) {
      return '';
    }

    const latest = this.comparisons[0];
    
    let table = `| Metric | Electron | Tauri | Improvement |\n`;
    table += `|--------|----------|-------|-------------|\n`;
    
    if (latest.electron.startup && latest.tauri.startup) {
      const improvement = ((latest.electron.startup.avg - latest.tauri.startup.avg) / latest.electron.startup.avg * 100).toFixed(1);
      table += `| Startup (ms) | ${latest.electron.startup.avg.toFixed(0)} | ${latest.tauri.startup.avg.toFixed(0)} | ${improvement}% faster |\n`;
    }
    
    if (latest.electron.memory && latest.tauri.memory) {
      const improvement = ((latest.electron.memory.avg - latest.tauri.memory.avg) / latest.electron.memory.avg * 100).toFixed(1);
      table += `| Memory (MB) | ${latest.electron.memory.avg.toFixed(1)} | ${latest.tauri.memory.avg.toFixed(1)} | ${improvement}% reduction |\n`;
    }
    
    if (latest.electron.cpu && latest.tauri.cpu) {
      const improvement = ((latest.electron.cpu.avg - latest.tauri.cpu.avg) / latest.electron.cpu.avg * 100).toFixed(1);
      table += `| CPU (%) | ${latest.electron.cpu.avg.toFixed(1)} | ${latest.tauri.cpu.avg.toFixed(1)} | ${improvement}% reduction |\n`;
    }
    
    return table;
  }

  saveReport(content, filename = 'performance-report.md') {
    const reportPath = path.join(__dirname, '..', filename);
    fs.writeFileSync(reportPath, content);
    console.log(`Report saved to: ${reportPath}`);
    return reportPath;
  }

  run() {
    this.loadResults();
    
    const markdownReport = this.generateMarkdownReport();
    const comparisonTable = this.generateComparisonTable();
    
    this.saveReport(markdownReport, 'performance-report.md');
    
    console.log('\n=== PERFORMANCE COMPARISON TABLE ===\n');
    console.log(comparisonTable);
    
    return {
      markdown: markdownReport,
      table: comparisonTable
    };
  }
}

// Run generator if script is executed directly
if (import.meta.url === `file://${__filename}`) {
  const generator = new ReportGenerator();
  generator.run();
}

export default ReportGenerator;