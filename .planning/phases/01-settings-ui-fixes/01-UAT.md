---
status: resolved
phase: 01-settings-ui-fixes
source: [01-SUMMARY.md, 02-SUMMARY.md]
started: 2026-03-04T17:30:00Z
updated: 2026-03-04T19:00:00Z
---

## Current Test

[testing complete - skipped remaining tests due to blocker]

## Tests

### 1. Test Credentials Button
expected: Open Settings → Authorization page. Enter valid ntfy credentials (token or user/pass). Click "Test Credentials" button. Button shows "Testing..." while in progress, then displays success indicator (green checkmark) for valid credentials. If invalid credentials, shows error indicator with error message.
result: issue
reported: "Settings is a mess - multiple separate pages instead of one. Test Connection button is under URL on Instance page and does nothing. Want single scrolling page with all settings in sections."
severity: blocker

### 2. Save Closes Window
expected: Open any Settings page. Change a setting value. Click "Save" button. The Settings window closes automatically after saving.
result: skipped
reason: Blocked by fundamental structure issue - need to test after redesign

### 3. Settings Persist
expected: Open Settings → Instance page. Change the Instance URL. Click Save. Reopen Settings → Instance page. The Instance URL field shows the value you saved. Test applies to all settings pages.
result: skipped
reason: Blocked by fundamental structure issue - need to test after redesign

### 4. General Page Layout
expected: Open Settings → General page. The page shows exactly 3 toggle switches: "Start Hidden", "Quit on Close", "Developer Tools". No other input fields or controls on this page.
result: skipped
reason: Blocked by fundamental structure issue - need to test after redesign

### 5. Notifications Page Layout
expected: Open Settings → Notifications page. The page shows: Poll Rate (number input), Date/Time Format (text input), Urgent notification threshold (dropdown), Standard Sound (dropdown with preview button), Urgent Sound (dropdown with preview button), Persistent Notifications (dropdown). All notification-related settings are on this page.
result: skipped
reason: Blocked by fundamental structure issue - need to test after redesign

## Summary

total: 5
passed: 0
issues: 1
pending: 0
skipped: 4

## Gaps

- truth: "Settings is a single scrolling page with all sections visible"
  status: resolved
  reason: "User reported: Settings is a mess with multiple separate pages. Want single scrolling page with all settings in sections (Instance, Auth, Notifications, General)."
  severity: blocker
  test: 1
  root_cause: "HTML has 4 separate page divs (page-instance, page-token, page-notifications, page-general) shown/hidden via ?page= URL param. Menu has 4 separate menu items. Need to consolidate into single page with all sections visible."
  artifacts:
    - path: "src/settings.html"
      issue: "Multiple page divs with style='display:none' toggled by URL param"
    - path: "src-tauri/src/main.rs"
      issue: "4 separate menu items (settings-instance, settings-token, settings-notifications, settings-general)"
  missing:
    - "Consolidate all page divs into single scrolling layout"
    - "Replace 4 menu items with single 'Settings' menu item"
    - "Remove ?page= URL parameter logic"
  debug_session: ""

- truth: "Test Credentials button works after entering credentials"
  status: resolved
  reason: "User reported: Test Connection button is under URL on Instance page and does nothing. Should be after credentials are entered on the same page."
  severity: blocker
  test: 1
  root_cause: "Test Credentials button on Authorization page requires instance_url which is on separate Instance page. User must configure instance on one page, then go to another page to test credentials."
  artifacts:
    - path: "src/settings.html"
      issue: "testConnection() reads instance_url from currentConfig but page doesn't show instance_url field"
  missing:
    - "Consolidate settings into single page so instance_url and credentials are together"
  debug_session: ""