# Requirements: ntfy.desktop Settings UI Fixes

**Defined:** 2026-03-04
**Core Value:** Working settings experience where users can configure ntfy and have settings persist

## v1 Requirements

### Settings Window

- [x] **SETT-01**: User can click Save and the Settings window closes automatically
- [x] **SETT-02**: User can test ntfy instance credentials with success/error indicator

### Persistence

- [x] **PERS-01**: Settings credentials load when reopening the Settings window

### UI Layout

- [x] **LAYOUT-01**: General settings page shows start_hidden, quit_on_close, dev_tools toggles only
- [x] **LAYOUT-02**: Notification settings page shows poll_rate, datetime_format, sounds, persistence settings only

## Out of Scope

| Feature | Reason |
|---------|--------|
| New features | Stabilization release only |
| macOS/Linux fixes | Windows-focused testing |
| Notification logic changes | Settings UI focus only |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SETT-01 | Phase 1 | Complete |
| SETT-02 | Phase 1 | Complete |
| PERS-01 | Phase 1 | Complete |
| LAYOUT-01 | Phase 1 | Complete |
| LAYOUT-02 | Phase 1 | Complete |

**Coverage:**
- v1 requirements: 5 total
- Mapped to phases: 5
- Unmapped: 0 ✓
- Completed: 5/5 (100%)

---
*Requirements defined: 2026-03-04*
*All requirements completed: 2026-03-04*