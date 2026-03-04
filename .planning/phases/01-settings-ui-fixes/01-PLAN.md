---
phase: 01-settings-ui-fixes
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - src/settings.html
autonomous: true
requirements:
  - SETT-01
  - SETT-02
  - PERS-01
  - LAYOUT-01
  - LAYOUT-02
user_setup: []

must_haves:
  truths:
    - "User can click Test Credentials and see success/error indicator"
    - "User can click Save and window closes automatically"
    - "Settings fields populate with saved values when window reopens"
    - "General page shows start_hidden, quit_on_close, dev_tools toggles only"
    - "Notifications page shows poll_rate, datetime_format, sounds, persistence settings"
  artifacts:
    - path: "src/settings.html"
      provides: "Settings UI with test credentials button and proper page layouts"
      min_lines: 540
  key_links:
    - from: "src/settings.html"
      to: "test_ntfy_connection"
      via: "invoke('test_ntfy_connection') in testCredentials()"
      pattern: "invoke\\('test_ntfy_connection'"
---

<objective>
Fix all settings window bugs for stable release.

Purpose: Ensure the Settings UI works correctly - users can save settings, test credentials, and see correct page layouts.
Output: Fixed settings.html with Test Credentials button and verified existing functionality.
</objective>

<execution_context>
@C:/Users/anthony/.claude/get-shit-done/workflows/execute-plan.md
@C:/Users/anthony/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md

@src/settings.html
@src-tauri/src/config.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add Test Credentials button to Authentication page</name>
  <files>src/settings.html</files>
  <action>
Add a "Test Credentials" button to the Authentication (Token) page that:
1. Appears below the credential input fields (after line 293, after the password hint paragraph)
2. Has a clear visual indicator for success/failure states (green checkmark for success, red X for error)
3. Calls the existing `test_ntfy_connection` backend command
4. Shows "Testing..." state while in progress
5. Displays error message on failure

Implementation:
- Add HTML structure for test button and result div after the password hint
- Add `testCredentials()` async function in the script section
- Add `escapeHtml()` helper function for safe error display
- Style the result div for success (green) and error (red) states
- Disable button during testing, re-enable after completion
  </action>
  <verify>
Manual verification:
1. Open Settings → Authorization page
2. Enter valid credentials (token or user/pass)
3. Click "Test Credentials" button
4. Verify button shows "Testing..." while in progress
5. Verify success indicator appears (green checkmark) for valid credentials
6. Test with invalid credentials to verify error indicator (red X with message)
  </verify>
  <done>
- Test Credentials button visible on Authorization page
- Button shows "Testing..." during operation
- Success indicator (green checkmark) appears for valid credentials
- Error indicator (red X with message) appears for invalid credentials
- Button re-enables after test completes
  </done>
</task>

<task type="auto">
  <name>Task 2: Verify existing settings functionality</name>
  <files>src/settings.html</files>
  <action>
Verify that existing functionality satisfies SETT-01, PERS-01, LAYOUT-01, and LAYOUT-02:

SETT-01 (Save closes window):
- The saveSettings() function already calls closeThisWindow() after saving
- Verify this behavior works correctly

PERS-01 (Settings load on reopen):
- The loadSettings() function already loads config and populates fields
- Verify fields populate correctly for each page

LAYOUT-01 (General page toggles):
- Verify General page shows exactly 3 toggles: start_hidden, quit_on_close, dev_tools
- No other input types on this page

LAYOUT-02 (Notifications page settings):
- Verify Notifications page shows: poll_rate, datetime_format, notification_sound, urgent_notification_sound, persistent_notifications_mode
- Verify urgent_priority_threshold is included (related to notifications)

No code changes needed for these - just verification that existing code is correct.
  </action>
  <verify>
Manual verification checklist:
1. Open Settings → Instance, change URL, click Save → window closes
2. Reopen Settings → Instance → previous URL is populated
3. Open Settings → General → see exactly 3 toggles (start_hidden, quit_on_close, dev_tools)
4. Open Settings → Notifications → see poll_rate, datetime_format, sound selectors, persistence mode
5. Reopen Settings multiple times → fields always populate correctly
  </verify>
  <done>
- Save button closes window (SETT-01) ✓
- Settings load correctly on reopen (PERS-01) ✓
- General page has exactly 3 toggles (LAYOUT-01) ✓
- Notifications page has correct fields (LAYOUT-02) ✓
  </done>
</task>

</tasks>

<verification>
Before declaring plan complete:
- [ ] Build succeeds: `npm run tauri:dev` starts without errors
- [ ] Test Credentials button exists on Authorization page
- [ ] Test Credentials button works with success/error indicators
- [ ] Save button closes window
- [ ] Settings persist and reload correctly
- [ ] General page shows only toggles
- [ ] Notifications page shows correct fields
</verification>

<success_criteria>
- All requirements (SETT-01, SETT-02, PERS-01, LAYOUT-01, LAYOUT-02) satisfied
- Settings UI functional and stable
- No regressions in existing settings behavior
</success_criteria>

<output>
After completion, create `.planning/phases/01-settings-ui-fixes/01-SUMMARY.md`
</output>