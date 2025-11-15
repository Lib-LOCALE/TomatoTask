# Changelog

All notable changes to TomatoTask will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-11-13

### Added

#### Core Features (Phase 3 MVP)

**US1 - Pomodoro Timer (P1)**
- Customizable timer with work/short break/long break durations
- Circular progress indicator with SVG animation
- Timer controls (Start, Pause, Resume, Reset)
- Session type display (Work, Short Break, Long Break)
- Auto-advance between sessions (configurable)
- Visual completion feedback with celebration animation
- Real-time display of current task being worked on

**US2 - Task Management (P1)**
- Complete CRUD operations for tasks
- Task list with filters (All, Active, Completed)
- Task cards with progress indicators
- Pomodoro estimation and tracking (0-5 🍅)
- Task completion toggle
- Task-to-project assignment
- Empty state with call-to-action
- Modal-based task creation/editing

**US3 - Task-Pomodoro Integration (P2)**
- Link tasks to timer sessions
- Display current task in timer
- Automatic Pomodoro increment on work session completion
- Visual badge showing task progress (completed/estimated 🍅)
- Task selection from timer interface

**US4 - Daily Summary (P2)**
- Daily and weekly statistics views
- Metrics tracked:
  - Completed tasks count
  - Completed Pomodoros count
  - Total focus time (minutes)
- Weekly breakdown with per-day details
- Auto-refresh every 60 seconds
- Summary cards with icons and formatting

**US5 - Settings Panel (P3)**
- Comprehensive settings modal
- Timer configuration:
  - Work duration (1-60 minutes)
  - Short break duration (1-30 minutes)
  - Long break duration (1-60 minutes)
  - Pomodoros until long break (2-10)
  - Auto-start breaks (checkbox)
  - Auto-start Pomodoros (checkbox)
- Appearance settings:
  - Theme toggle (light/dark)
  - Language selector
- Save confirmation with visual feedback
- Reset to defaults with confirmation

**US6 - Language Selector (P2)**
- Support for 5 languages:
  - 🇬🇧 English (default)
  - 🇫🇷 Français
  - 🇪🇸 Español
  - 🇮🇹 Italiano
  - 🇩🇪 Deutsch
- 125+ translation keys
- Runtime language switching without reload
- Language persistence in settings
- Modal and dropdown variants
- Accessible via Ctrl+L shortcut

**US7 - Theme Toggle (P3)**
- Light and dark mode support
- Toggle button with sun/moon icons
- Floating button in header
- Integration in settings panel
- Theme persistence in database
- Automatic application on startup
- Complete Tailwind dark: class support

**US8 - System Tray & Notifications (P3)**
- System tray icon for all platforms (Windows, macOS, Linux)
- Tray tooltip showing timer state
- Window management commands:
  - Toggle window visibility
  - Show window
  - Hide window
  - Quit application
- Native system notifications:
  - Work session complete
  - Break session complete
  - Task complete
  - Custom notifications API
- Minimize to tray on close (doesn't quit)

**US9 - Project Organization (P3)**
- Create, read, update, delete projects
- Color-coded projects (8 presets + custom)
- Project list with task count badges
- Filter tasks by project
- "All Projects" view
- Project name validation (max 50 chars)
- Project assignment in task forms
- Hover actions for edit/delete
- Confirmation dialogs for destructive actions

**US10 - Keyboard Shortcuts Help (P2)**
- Shortcuts help modal (Ctrl+/)
- Categorized shortcuts:
  - Timer controls
  - Task management
  - Settings
  - Help
- Visual kbd badges
- Accessible design
- Quick reference

#### Foundation Layer (Phase 2)

**Database Layer**
- SQLite database with migrations system
- 4 tables: projects, tasks, pomodoro_sessions, settings
- Foreign key constraints
- Indexes for performance
- Auto-updated timestamps
- Schema versioning

**Backend (Rust/Tauri)**
- 23 Tauri commands for frontend-backend communication
- Modular command structure
- Database connection pooling
- Error handling with Result types
- Serde JSON serialization (camelCase ↔ snake_case)
- Type-safe query functions
- Unit tests for database queries

**Frontend (TypeScript/Svelte 5)**
- Svelte 5 Runes API ($state, $derived, $effect)
- 24 reactive components
- 6 service layers for business logic
- 4 reactive stores (timer, tasks, settings, projects)
- Type-safe TypeScript interfaces
- Validation utilities
- Keyboard shortcut management
- i18n service with locale detection

**UI/UX**
- Tailwind CSS styling
- Shadcn-svelte components
- Responsive layout (1200x800 default)
- Split sidebar (Projects 1/3, Tasks 2/3)
- Floating action buttons
- Modal dialogs with backdrop
- Loading states
- Empty states
- Error messages

### Technical

**Architecture**
- Clean separation: UI → Services → Stores → Backend
- DRY principle throughout
- Modular component design
- Reusable utilities
- Centralized state management

**Performance**
- Database queries <50ms target
- Prepared statements
- Efficient reactivity with Svelte Runes
- Minimal re-renders
- Lazy loading where applicable

**Code Quality**
- All Rust comments in French (project requirement)
- TypeScript strict mode
- Rust Clippy compliance
- Consistent code style
- Comprehensive error handling

**Development Tools**
- Vite for fast builds
- Hot Module Replacement (HMR)
- TypeScript checking
- Svelte component checking
- ESLint configuration

### Configuration

**Tauri Configuration**
- Application: TomatoTask v1.0.0
- Identifier: com.tomatotask.app
- Window: 1200x800, resizable
- System tray enabled
- Native notifications enabled

**Build Targets**
- Windows (MSI, NSIS)
- macOS (DMG, App Bundle)
- Linux (AppImage, Deb, RPM)

### Statistics

- **Total Commits**: 19
- **Total Files**: 45+
- **Lines of Code**: ~8,500+
- **Components**: 24 Svelte components
- **Services**: 6 TypeScript services
- **Stores**: 4 reactive stores
- **Tauri Commands**: 23 backend commands
- **Rust Modules**: 8 modules
- **i18n Keys**: 125+ translation keys
- **Supported Languages**: 5
- **Keyboard Shortcuts**: 4 global shortcuts

## [1.0.8] - 2025-11-15

### Fixed

**CRITICAL: Linux AppImage Black Screen Issue**
- **Removed Google Fonts external dependency** that was causing black screen on Linux AppImage
  - WebKit2GTK on Linux was blocking external font resources
  - Replaced with reliable system fonts that work across all platforms
  - Fonts now use: ui-monospace, system-ui, and standard fallbacks
- **Added proper Content Security Policy (CSP)** in Tauri configuration
  - Prevents external resource loading issues
  - Enhances application security
  - Works reliably on all platforms (Windows, Linux, macOS)
- **Improved error handling in Rust backend**
  - Better error messages with eprintln! for debugging
  - More descriptive error context for database initialization
  - Helps identify issues during application startup

### Changed

- **Updated HTML metadata**
  - Changed title from "Vite + Svelte + TS" to "TomatoTask - Pomodoro Timer"
  - Fixed favicon path (now uses favicon.png instead of vite.svg)
  - Added meta description for better app identification
- **Synchronized version numbers** across all configuration files
  - package.json: 1.0.8
  - Cargo.toml: 1.0.8 (was 1.0.4)
  - tauri.conf.json: 1.0.8

## [1.0.7] - 2025-11-15

### Changed
- Version bump to 1.0.7
- Updated Windows icon to tomatoIcon.ico

## [1.0.6] - 2025-11-14

### Changed
- Improved pixel art design system
- Fixed icon paths for better consistency
- Version synchronization improvements

## [1.0.5] - 2025-11-14

### Changed
- Major UX improvements
- Enhanced UI consistency
- Bug fixes and performance improvements

## [1.0.4] - 2025-11-14

### Added

**Onboarding Flow**
- Complete 3-step onboarding experience for first-time users
  - Step 1: Language selection with flag emojis 🇬🇧 🇫🇷 🇪🇸 🇮🇹 🇩🇪
  - Step 2: Pomodoro technique explained in simple, accessible terms
  - Step 3: Quick start guide with keyboard shortcuts
- localStorage-based completion tracking (shows only once)
- Fully translated in all 5 supported languages
- Added "back" and "next" translations to common section

**New Icon**
- Added tomatoTask30x30.png for better system tray display
- Updated tray icon configuration

### Changed

**UI/UX Improvements**
- Redesigned language selector with flag emojis
  - 🇬🇧 English, 🇫🇷 Français, 🇪🇸 Español, 🇮🇹 Italiano, 🇩🇪 Deutsch
  - Real flag emojis for better visual recognition
  - Added checkmark icon to clearly show the selected language
  - Improved button styling with better hover states and transitions
  - Enhanced visual hierarchy with larger, more accessible buttons
  - Flags appear in both dropdown and button variants

**Dark Mode Enhancement**
- Complete redesign of dark mode color palette for better visual harmony
  - Softer background with subtle blue tint to reduce eye strain
  - Modern purple-violet primary color (more vibrant and appealing)
  - Improved contrast ratios across all UI elements
  - Better border and shadow definitions
  - Elegant cyan accent color for highlights
  - More sophisticated muted colors for secondary elements
  - Inspired by modern design systems (GitHub Dark, VS Code Dark)

### Fixed
- **CRITICAL**: Fixed Svelte build error that caused GitHub Actions to fail
  - Changed `class:bg-primary/10` to `class:bg-accent` (Svelte doesn't support `/` in dynamic class bindings)
  - All builds now pass successfully ✅
- Confirmed all modals properly close when clicking outside (backdrop click)
- Improved modal backdrop with consistent blur effect across all dialogs
- Updated author information in package.json and Cargo.toml (AnthonyMahe)
- Synchronized version numbers across all configuration files (1.0.4)
- Fixed .gitignore typo (.speify → .specify)
- Removed .claude/ and .specify/ development files from git tracking

## [Unreleased]

### Planned Features

- Cloud sync for multi-device support
- Advanced analytics with charts
- Custom timer sounds and alerts
- Pomodoro history timeline view
- Export data (CSV, JSON, PDF)
- Integration with external task tools (Trello, Notion, Todoist)
- Break reminders and stretch suggestions
- Focus mode (disable distractions)
- Team collaboration features
- Mobile companion app
- Browser extension
- Spotify/Music integration for focus sessions

---

## Version History

- **1.0.8** (2025-11-15) - CRITICAL FIX: Linux AppImage black screen resolved
  - Removed external Google Fonts dependency causing Linux issues
  - Added proper CSP for better security and compatibility
  - Improved error handling in backend
  - Fixed HTML metadata and version synchronization
- **1.0.7** (2025-11-15) - Windows icon update
- **1.0.6** (2025-11-14) - Design system improvements
- **1.0.5** (2025-11-14) - UX improvements and bug fixes
- **1.0.4** (2025-11-14) - UX improvements and dark mode enhancement
  - Redesigned language selector with visual indicators
  - Complete dark mode color palette redesign
  - Modal improvements
- **1.0.0** (2025-11-13) - Initial release with all core features
  - All 10 user stories implemented (US1-US10)
  - Production-ready application
  - Full internationalization (5 languages)
  - Complete documentation

[1.0.8]: https://github.com/AnthonyMahe/TomatoTask/releases/tag/v1.0.8
[1.0.7]: https://github.com/AnthonyMahe/TomatoTask/releases/tag/v1.0.7
[1.0.6]: https://github.com/AnthonyMahe/TomatoTask/releases/tag/v1.0.6
[1.0.5]: https://github.com/AnthonyMahe/TomatoTask/releases/tag/v1.0.5
[1.0.4]: https://github.com/AnthonyMahe/TomatoTask/releases/tag/v1.0.4
[1.0.0]: https://github.com/AnthonyMahe/TomatoTask/releases/tag/v1.0.0
[Unreleased]: https://github.com/AnthonyMahe/TomatoTask/compare/v1.0.8...HEAD
