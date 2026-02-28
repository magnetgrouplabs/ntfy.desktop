# Performance Testing Framework

This directory contains a comprehensive performance testing framework for ntfy.desktop that compares the original Electron app with our new Tauri implementation.

## Overview

The framework provides:
- **Startup Time Measurement**: Cold and warm startup benchmarks
- **Resource Monitoring**: Memory and CPU usage tracking
- **Network Resilience Testing**: Reconnection behavior and outage handling
- **Comparative Analysis**: Side-by-side Electron vs Tauri comparisons
- **Automated Reporting**: README-ready metrics and recommendations

## Directory Structure

```
performance-tests/
├── scripts/                 # Test scripts
│   ├── startup-benchmark.js     # Original startup benchmark
│   ├── performance-benchmark.js # Comprehensive performance tests
│   └── network-resilience.js    # Network resilience tests
├── baseline/                # Baseline Electron performance data
├── comparison/              # Comparative test results
├── electron-app/            # Original Electron application
└── README.md               # This file
```

## Quick Start

### 1. Run Baseline Tests (Electron Only)

```bash
cd performance-tests
node scripts/performance-benchmark.js baseline
```

This establishes the Electron performance baseline.

### 2. Run Comparative Tests

```bash
node scripts/performance-benchmark.js comparison
```

This runs side-by-side tests comparing Electron and Tauri.

### 3. Test Network Resilience

```bash
node scripts/network-resilience.js
```

## Test Metrics

### Startup Performance
- **Cold Startup**: Application launch from terminated state
- **Warm Startup**: Application restart after recent usage
- **Target**: <3s cold, <1s warm

### Resource Usage
- **Memory**: Average and peak memory consumption
- **CPU**: Background and active CPU usage
- **Target**: <100MB memory, <5% CPU

### Network Resilience
- **Reconnection Time**: Recovery after network outage
- **Retry Behavior**: Connection retry patterns
- **Error Handling**: Graceful degradation

## Baseline Performance (Electron)

Based on current measurements:
- **Startup Time**: ~5 seconds (cold)
- **Memory Usage**: 150-250MB
- **CPU Usage**: 3-8% (idle)

## Expected Tauri Improvements

Target improvements with Tauri migration:
- **Startup Time**: 60-80% reduction (1-2 seconds)
- **Memory Usage**: 70-90% reduction (15-75MB)
- **CPU Usage**: 50-80% reduction (1-4%)

## Automated Testing

The framework supports automated CI/CD integration:

```bash
# Run all tests
npm run test:performance

# Generate comparison report
npm run test:comparison

# Update baseline
npm run test:baseline
```

## Results Format

Test results are saved as JSON files in `baseline/` and `comparison/` directories:

```json
{
  "timestamp": "2024-01-01T00:00:00.000Z",
  "platform": "win32",
  "electron": {
    "startup": {
      "avg": 5123,
      "min": 4890,
      "max": 5421,
      "median": 5102
    },
    "memory": {
      "avg": 187.5,
      "min": 152.3,
      "max": 243.1,
      "median": 189.2
    }
  },
  "tauri": {
    "startup": {
      "avg": 1423,
      "min": 1289,
      "max": 1567,
      "median": 1412
    },
    "memory": {
      "avg": 42.1,
      "min": 38.5,
      "max": 46.7,
      "median": 41.8
    }
  },
  "improvements": {
    "startup": {
      "percentage": "72.2",
      "absolute": "3700"
    },
    "memory": {
      "percentage": "77.5",
      "absolute": "145.4"
    }
  }
}
```

## Integration with Development

### Pre-commit Checks

Add performance regression checks:
```bash
# In package.json scripts
"pre-commit": "node performance-tests/scripts/performance-benchmark.js comparison"
```

### CI/CD Pipeline

Example GitHub Actions workflow:
```yaml
name: Performance Tests
on: [push, pull_request]

jobs:
  performance:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm install
      - run: node performance-tests/scripts/performance-benchmark.js comparison
      - uses: actions/upload-artifact@v3
        with:
          name: performance-results
          path: performance-tests/comparison/
```

## Customization

### Adding New Metrics

Edit `scripts/performance-benchmark.js` to add new performance metrics:

```javascript
async measureNewMetric(appType) {
  // Implement custom metric measurement
  return results;
}
```

### Adjusting Test Parameters

Modify test parameters in the script constructors:

```javascript
// Change iteration counts
await this.measureStartupTime(appType, 10); // 10 iterations

// Change test durations
await this.measureMemoryUsage(appType, 30000); // 30 seconds
```

## Troubleshooting

### Common Issues

1. **App not starting**: Ensure both Electron and Tauri apps build successfully
2. **Permission errors**: Run tests with appropriate permissions
3. **Resource monitoring failures**: Check platform-specific monitoring tools

### Platform Support

- **Windows**: Uses `tasklist` and `wmic` commands
- **Linux/macOS**: Uses `ps` and `pgrep` commands

## Contributing

When adding new tests:
1. Follow existing code patterns
2. Include proper error handling
3. Add relevant metrics to comparison reports
4. Update this README with new capabilities