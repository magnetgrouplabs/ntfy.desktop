# Testing Methodology

This document details the testing methodology used to compare ntfy.desktop (Rust implementation) against the original Electron-based client.

## Test Environment

**Platform:** Windows 11 Pro (Build 22631)
**Date:** February 2026
**Test Duration:** 60 minutes per application
**Both Apps Configured With:**
- Default poll rate (60s for Rust, 30s for Electron)
- Same topics subscribed
- Connected to ntfy.sh
- Idle state (no user interaction)

## Metrics Measured

### 1. Memory Usage (Idle)

**Method:**
- Launch application and allow it to stabilize (5 minutes)
- Record working set memory using Process Monitor
- Sample every 60 seconds for 60 minutes
- Calculate average of all samples

**Tools:**
- Process Monitor (sysinternals)
- Windows Task Manager (for quick verification)

**Results:**
| Application | Average Memory | Peak Memory |
|-------------|---------------|-------------|
| Electron | ~383 MB | ~412 MB |
| Rust | ~34 MB | ~38 MB |

**Calculation:**
- Measurement: Working Set (RAM actively used by process)
- Sampling: 60 samples over 60 minutes
- Average: Sum of all samples / sample count
- Improvement: ((Electron - Rust) / Electron) × 100

### 2. Binary Size

**Method:**
- Clean build of both applications
- Measure total installation directory size
- Include all bundled resources

**Results:**
| Application | Size |
|-------------|------|
| Electron | ~196 MB |
| Rust | ~16 MB |

**Note:** Electron bundles entire Chromium runtime. Rust uses system WebView2.

### 3. CPU Usage (Idle)

**Method:**
- Allow application to stabilize
- Record CPU percentage over 60 minutes
- Measure average utilization when idle (no notifications, no user interaction)

**Tools:**
- Windows Performance Monitor
- Task Manager

**Results:**
| Application | Average CPU | Peak CPU (Idle) |
|-------------|-----------|-----------------|
| Electron | ~0.35% | ~0.8% |
| Rust | ~0.09% | ~0.2% |

### 4. Startup Time

**Method:**
- Cold start (not in memory)
- Measure from process launch to window visible
- Average of 10 launches

**Results:**
| Application | Average Startup |
|-------------|-----------------|
| Electron | ~484 ms |
| Rust | ~482 ms |

**Note:** Similar startup times because both ultimately load a webview.

## Notification Testing

### Test Cases

**Case 1: Single Notification**
- Send one test notification
- Measure time from server receive to display
- Verify icon rendering
- Verify sound playback

**Case 2: Burst Notifications (10 messages)**
- Send 10 notifications in rapid succession
- Measure handling time
- Check for duplicates or drops

**Case 3: Long-Running Stability**
- Run application for 24 hours
- Monitor memory growth
- Verify no crashes or hangs

## Feature Parity Testing

| Feature | Electron | Rust | Notes |
|---------|----------|------|-------|
| Basic Notifications | ✓ | ✓ | Both functional |
| Custom Icons | ✗ | ✓ | Rust implementation downloads and caches |
| Custom Sounds | Default only | 5 options | Rust has granular control |
| Persistence | On/Off | Off/All/Urgent | Rust has more modes |
| System Tray | ✓ | ✓ | Both functional |
| Keyboard Shortcuts | ✓ | ✓ | Rust adds native menu bar |
| Credential Storage | Plaintext | Keychain | Rust uses OS-native security |
| Auth Methods | Token | Token + Basic | Rust supports more methods |

## Testing Tools Used

### Performance Monitoring
- **Process Monitor:** Memory and CPU tracking
- **Performance Monitor:** System-level metrics
- **Task Manager:** Quick verification

### Load Testing
- Custom script using ntfy HTTP API
- Automated notification delivery
- Timing measurement with `Measure-Command`

### Stability Testing
- 24-hour continuous run
- Memory leak detection via heap snapshots
- Automated crash detection

## Known Limitations

1. **Platform Scope:** Testing conducted exclusively on Windows 11
2. **Idle State:** Measurements taken during idle; active usage patterns may differ
3. **Network Conditions:** Tests run on stable broadband connection
4. **Notification Volume:** Limited to test scenarios; production volume may vary

## Reproducibility

To reproduce these tests:

```bash
# Run benchmarks
cd performance-tests
npm run test:comparison
```

Results will vary based on:
- System specifications
- Background processes
- Network latency
- Notification frequency

## Conclusion

The Rust implementation demonstrates significant efficiency improvements:
- **91% less memory** in idle state
- **92% smaller binary** distribution
- **73% less CPU** usage
- **Comparable startup** performance

These gains are achieved through:
1. Using system WebView2 instead of bundled Chromium
2. Native Rust compilation vs JavaScript interpretation
3. Efficient async runtime (tokio)
4. Optimized binary size through release compilation

**Note:** Performance is one factor in application quality. User experience, feature set, and maintainability were also considered in the rewrite decision.
