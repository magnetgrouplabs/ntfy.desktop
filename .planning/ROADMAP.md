# Roadmap: Settings UI Fixes

**Project:** ntfy.desktop
**Milestone:** v26.3.2 Settings UI Fixes
**Created:** 2026-03-04

## Phase Overview

| # | Phase | Goal | Requirements | Success Criteria |
|---|-------|------|--------------|-------------------|
| 1 | Settings UI Fixes | Fix all settings window bugs | SETT-01, SETT-02, PERS-01, LAYOUT-01, LAYOUT-02 | 4 criteria |

---

## Phase 1: Settings UI Fixes

**Goal:** Fix all settings window bugs for stable release

**Requirements:**
- SETT-01: Save button closes window
- SETT-02: Test credentials button with indicator
- PERS-01: Settings load when window reopens
- LAYOUT-01: General page shows only toggles
- LAYOUT-02: Notifications page shows only notification settings

**Success Criteria:**
1. User can click Save and window closes automatically
2. User sees Test Credentials button with success/error indicator
3. Settings fields populate with saved values when window reopens
4. General page shows start_hidden, quit_on_close, dev_tools toggles
5. Notifications page shows poll_rate, sounds, persistence settings

**Notes:**
- All fixes target `src/settings.html`
- Backend commands (`save_config`, `load_config`, `test_ntfy_connection`) already exist
- Focus on frontend JavaScript logic and page visibility

---

## Phase Completion Tracking

| Phase | Status | Plans | Progress |
|-------|--------|-------|----------|
| 1 | ✓ Complete | 1/1 | 100% |

---
*Roadmap created: 2026-03-04*
*Roadmap completed: 2026-03-04*