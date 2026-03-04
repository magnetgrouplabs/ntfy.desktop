---
phase: 01-settings-ui-fixes
plan: 02
type: execute
wave: 1
depends_on: []
files_modified:
  - src/settings.html
  - src-tauri/src/main.rs
autonomous: false
requirements:
  - SETT-02
gap_closure: true

must_haves:
  truths:
    - "Settings is a single scrolling page with all sections visible"
    - "Test Credentials button works after entering instance URL and credentials on same page"
    - "Single 'Settings' menu item opens the unified settings window"
  artifacts:
    - path: "src/settings.html"
      provides: "Unified single-page settings UI with all sections visible"
      contains: "single scrolling layout with Instance, Authentication, Notifications, General sections"
    - path: "src-tauri/src/main.rs"
      provides: "Simplified menu with single Settings item"
      contains: "settings menu item that opens unified settings window"
  key_links:
    - from: "src-tauri/src/main.rs"
      to: "src/settings.html"
      via: "open_settings_window() without page parameter"
      pattern: "settings\\.html"
---

<objective>
Consolidate Settings UI into a single scrolling page with all sections visible, and simplify the menu to a single "Settings" item.

Purpose: Fix the fundamental structural issue where settings are split across 4 separate pages, making the Test Credentials button non-functional (requires instance_url from different page).
Output: Unified settings.html with all sections on one page, simplified menu in main.rs.
</objective>

<execution_context>
@C:/Users/anthony/.claude/get-shit-done/workflows/execute-plan.md
@C:/Users/anthony/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md
@.planning/phases/01-settings-ui-fixes/01-UAT.md

@src/settings.html
@src-tauri/src/main.rs
</context>

<interfaces>
<!-- Current structure that needs to be changed -->

From src/settings.html:
- Page routing: `const page = new URLSearchParams(window.location.search).get("page") || "instance";`
- Page containers: `<div id="page-instance" style="display:none">`, `<div id="page-token" style="display:none">`, etc.
- Page switching: `const pageEl = document.getElementById("page-" + page); if (pageEl) pageEl.style.display = "block";`
- Field mappings per page: TEXT_FIELDS and BOOL_FIELDS objects keyed by page name

From src-tauri/src/main.rs:
- Menu items: settings-instance, settings-token, settings-notifications, settings-general
- open_settings_window(app, page) function that passes page parameter
- Window labels: "settings-instance", "settings-token", etc.
</interfaces>

<tasks>

<task type="auto">
  <name>Task 1: Consolidate settings.html into single scrolling page</name>
  <files>src/settings.html</files>
  <action>
Transform the Settings UI from 4 separate page divs to a single scrolling page with all settings sections visible together.

Changes required:

1. **Remove page routing logic:**
   - Delete `const page = new URLSearchParams(window.location.search).get("page") || "instance";`
   - Delete `const windowLabel = "settings-" + page;`
   - Delete page title mapping (`pageTitles` object)
   - Delete `const pageEl = document.getElementById("page-" + page); if (pageEl) pageEl.style.display = "block";`

2. **Convert page divs to section divs:**
   - Change `<div id="page-instance" style="display:none">` to `<div class="settings-section">`
   - Change `<div id="page-token" style="display:none">` to `<div class="settings-section">`
   - Change `<div id="page-notifications" style="display:none">` to `<div class="settings-section">`
   - Change `<div id="page-general" style="display:none">` to `<div class="settings-section">`
   - All sections should be visible by default (no display:none)

3. **Update section headings:**
   - "Instance Settings" → "Instance"
   - "Authentication" → keep as is
   - "Notification Settings" → "Notifications"
   - "General Settings" → "General"

4. **Add CSS for section spacing:**
   - Add `.settings-section { margin-bottom: 32px; }` to create visual separation
   - Add `.settings-section:last-child { margin-bottom: 0; }` for last section
   - Add `.settings-section h2 { border-bottom: 1px solid var(--border-color); padding-bottom: 8px; }` for visual separation

5. **Update page title:**
   - Change `<p id="page-title">Settings</p>` to just show "Settings" (static, no JavaScript needed)
   - Remove the JavaScript that sets pageTitleEl.textContent

6. **Consolidate field mappings:**
   - Merge all TEXT_FIELDS into single array: `["instance_url", "api_token", "auth_user", "auth_pass", "poll_rate", "datetime_format", "urgent_priority_threshold", "notification_sound", "urgent_notification_sound", "persistent_notifications_mode"]`
   - Merge all BOOL_FIELDS into single array: `["self_hosted_instance", "start_hidden", "quit_on_close", "dev_tools"]`
   - Remove the per-page TEXT_FIELDS and BOOL_FIELDS objects
   - Update loadSettings() to iterate over all fields without page logic
   - Update saveSettings() to save all fields without page logic

7. **Fix the saveSettings() function:**
   - Remove `page` variable usage
   - Save all fields from the single page
   - Keep the instance_url navigation logic (only runs when instance_url changes)

8. **Add separator between sections:**
   - Add `<hr style="border:none;border-top:1px solid var(--border-color);margin:24px 0;">` between major sections for visual clarity

9. **Move Test Credentials button:**
   - Keep it in the Authentication section
   - Since instance_url is now on the same page, update testConnection() to read instance_url from the form field instead of currentConfig

The Test Credentials button will now work correctly because instance_url and credentials are on the same page.
  </action>
  <verify>
    <automated>grep -c "page-instance\|page-token\|page-notifications\|page-general" src/settings.html || echo "0"</automated>
    Expected: 0 (no separate page divs remain)
    <automated>grep -c "settings-section" src/settings.html</automated>
    Expected: 4+ (at least 4 section divs)
    <automated>grep -c 'URLSearchParams.*page' src/settings.html || echo "0"</automated>
    Expected: 0 (page routing removed)
  </verify>
  <done>
    - Settings UI is a single scrolling page with all 4 sections visible
    - No page routing logic (?page= parameter) remains
    - Test Credentials button can access instance_url from same page
    - All fields load and save correctly
  </done>
</task>

<task type="auto">
  <name>Task 2: Simplify menu to single Settings item</name>
  <files>src-tauri/src/main.rs</files>
  <action>
Replace the 4 separate Settings menu items with a single "Settings" item that opens the unified settings window.

Changes required in main.rs:

1. **In create_app_menu() function (around line 849-864):**
   Replace the 4 menu items:
   ```rust
   let instance_settings_item =
       MenuItemBuilder::with_id("settings-instance", "Instance URL").build(app)?;
   let auth_item = MenuItemBuilder::with_id("settings-token", "Authorization").build(app)?;
   let notification_settings_item =
       MenuItemBuilder::with_id("settings-notifications", "Notifications").build(app)?;
   let general_settings_item =
       MenuItemBuilder::with_id("settings-general", "General").build(app)?;
   ```
   With a single menu item:
   ```rust
   let settings_item =
       MenuItemBuilder::with_id("settings", "Settings")
       .accelerator(if cfg!(target_os = "macos") { "Cmd+," } else { "Ctrl+," })
       .build(app)?;
   ```

2. **Update settings_menu SubmenuBuilder:**
   Replace:
   ```rust
   let settings_menu = SubmenuBuilder::new(app, "Settings")
       .item(&instance_settings_item)
       .item(&auth_item)
       .item(&notification_settings_item)
       .item(&general_settings_item)
       .build()?;
   ```
   With:
   ```rust
   let settings_menu = SubmenuBuilder::new(app, "Settings")
       .item(&settings_item)
       .build()?;
   ```

3. **In handle_menu_event() function (around line 952-963):**
   Replace the 4 menu event handlers:
   ```rust
   "settings-instance" => {
       let _ = open_settings_window(app, "instance");
   }
   "settings-token" => {
       let _ = open_settings_window(app, "token");
   }
   "settings-notifications" => {
       let _ = open_settings_window(app, "notifications");
   }
   "settings-general" => {
       let _ = open_settings_window(app, "general");
   }
   ```
   With a single handler:
   ```rust
   "settings" => {
       let _ = open_settings_window(app, "");
   }
   ```

4. **Update open_settings_window() function (around line 879-901):**
   - Change window label from `"settings-{page}"` to just `"settings"`
   - Change settings_url from `/settings.html?page={page}` to `/settings.html`
   - Update window title from `"Settings - {page}"` to `"Settings"`

   Updated function:
   ```rust
   fn open_settings_window(app: &tauri::AppHandle, _page: &str) -> Result<(), tauri::Error> {
       let window_label = "settings";

       // Check if window already exists
       if let Some(existing_window) = app.get_webview_window(window_label) {
           let _ = existing_window.show();
           let _ = existing_window.set_focus();
           return Ok(());
       }

       tauri::WebviewWindowBuilder::new(
           app,
           window_label,
           tauri::WebviewUrl::App("/settings.html".into()),
       )
       .title("Settings")
       .inner_size(500.0, 700.0)  // Slightly taller for all sections
       .resizable(true)  // Allow resizing for longer content
       .build()?;

       Ok(())
   }
   ```

5. **Make the window taller and resizable:**
   - Change inner_size from 500.0, 600.0 to 500.0, 700.0 (more vertical space for all sections)
   - Change resizable from false to true (allow user to expand if needed)
  </action>
  <verify>
    <automated>grep -c "settings-instance\|settings-token\|settings-notifications\|settings-general" src-tauri/src/main.rs || echo "0"</automated>
    Expected: 0 (old menu items removed)
    <automated>grep -c '"settings"' src-tauri/src/main.rs</automated>
    Expected: 2+ (new settings menu item and handler)
    <automated>grep 'Cmd+,\|Ctrl+,' src-tauri/src/main.rs || echo "not found"</automated>
    Expected: Contains accelerator for Settings
  </verify>
  <done>
    - Single "Settings" menu item with Cmd+, / Ctrl+, shortcut
    - Opens unified settings window without page parameter
    - Window is taller (700px) and resizable
  </done>
</task>

<task type="checkpoint:human-verify" gate="blocking">
  <what-built>Consolidated Settings UI into single scrolling page with all sections visible, and simplified menu to single "Settings" item</what-built>
  <how-to-verify>
1. Build and run the app: `npm run tauri:dev`
2. Open Settings from menu (File → Settings or use Ctrl+, shortcut)
3. Verify all 4 sections are visible on one scrolling page: Instance, Authentication, Notifications, General
4. Enter instance URL and credentials, click Test Credentials button
5. Verify button works (shows success or error message)
6. Modify settings in different sections, click Save
7. Verify window closes
8. Reopen Settings, verify all values persisted correctly
9. Try resizing the window (should be resizable now)
  </how-to-verify>
  <resume-signal>Type "approved" or describe issues found</resume-signal>
</task>

</tasks>

<verification>
Before declaring plan complete:
- [ ] Build succeeds: `npm run tauri:dev` starts without errors
- [ ] Settings opens as single page with all sections visible
- [ ] Menu has single "Settings" item with Ctrl+, shortcut
- [ ] Test Credentials button works with instance_url on same page
- [ ] All settings load correctly on window open
- [ ] All settings save correctly and persist
- [ ] Window is taller and resizable
</verification>

<success_criteria>
- Settings UI is a single scrolling page (Gap 1 closed)
- Test Credentials button works after entering credentials on same page (Gap 2 closed)
- Menu simplified to single Settings item
- No regressions in settings save/load functionality
</success_criteria>

<output>
After completion, create `.planning/phases/01-settings-ui-fixes/02-SUMMARY.md`
</output>