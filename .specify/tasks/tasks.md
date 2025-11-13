# Tasks: TomatoTask - Pomodoro Timer with TodoList

**Input**: [Implementation Plan](../plans/implementation-plan.md), [Specifications](../specs/tomatotask-core.md)
**Branch**: `claude/tomatotask-setup-011CV5fqQiDPnEwd4zk32iBp`

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1-US10)
- **File paths**: Exact locations per implementation plan

## Phase 1: Setup & Boilerplate Integration

**Purpose**: Clone boilerplate and configure project foundation

- [ ] T001 Clone Tauri2-Svelte5-Shadcn boilerplate from https://github.com/alysonhower/tauri2-svelte5-shadcn
- [ ] T002 [P] Update package.json with project name "TomatoTask"
- [ ] T003 [P] Configure TypeScript strict mode in tsconfig.json
- [ ] T004 [P] Configure Rust Clippy in src-tauri/Cargo.toml with deny warnings
- [ ] T005 [P] Setup ESLint with French comment enforcement rule
- [ ] T006 [P] Configure Prettier for consistent formatting
- [ ] T007 [P] Setup Vitest for unit testing in vite.config.ts
- [ ] T008 [P] Install Playwright for E2E testing
- [ ] T009 [P] Add i18n dependency (svelte-i18n)
- [ ] T010 [P] Add Tauri plugins: notification, system-tray in Cargo.toml

---

## Phase 2: Foundation (Blocking Prerequisites)

**Purpose**: Core infrastructure required before any feature implementation

**⚠️ CRITICAL**: No user story work until this phase is complete

### Database Setup

- [ ] T011 Create database schema migration file src-tauri/src/db/migrations.rs with all tables (projects, tasks, pomodoro_sessions, settings)
- [ ] T012 [P] Add SQLite indexes for performance (task project_id, sessions date, task completion)
- [ ] T013 [P] Implement database connection manager in src-tauri/src/db/connection.rs
- [ ] T014 [P] Create Rust data models in src-tauri/src/db/models.rs (Task, Project, PomodoroSession, Settings)
- [ ] T015 Insert default settings record in migration (singleton pattern)

### Type Definitions

- [ ] T016 [P] Create TypeScript types in src/lib/types/task.ts (Task, CreateTaskInput, Project)
- [ ] T017 [P] Create TypeScript types in src/lib/types/timer.ts (PomodoroSession, TimerState)
- [ ] T018 [P] Create TypeScript types in src/lib/types/settings.ts (Settings, Theme, Language)

### Base Query Layer

- [ ] T019 [P] Implement prepared queries in src-tauri/src/db/queries/tasks.rs (select, insert, update, delete, toggle_completion)
- [ ] T020 [P] Implement prepared queries in src-tauri/src/db/queries/projects.rs (select_all, insert, delete)
- [ ] T021 [P] Implement prepared queries in src-tauri/src/db/queries/sessions.rs (insert, update, select_by_date_range)
- [ ] T022 [P] Implement prepared queries in src-tauri/src/db/queries/settings.rs (select, update)

### Tauri Commands Scaffolding

- [ ] T023 [P] Create Tauri task commands in src-tauri/src/commands/tasks.rs (get_tasks, get_task, create_task, update_task, delete_task, toggle_task_completion)
- [ ] T024 [P] Create Tauri project commands in src-tauri/src/commands/projects.rs (get_projects, create_project, delete_project)
- [ ] T025 [P] Create Tauri timer commands in src-tauri/src/commands/timer.rs (create_session, complete_session)
- [ ] T026 [P] Create Tauri settings commands in src-tauri/src/commands/settings.rs (get_settings, update_settings)
- [ ] T027 [P] Create Tauri summary commands in src-tauri/src/commands/summary.rs (get_daily_summary, get_weekly_summary)
- [ ] T028 Register all Tauri commands in src-tauri/src/main.rs

### i18n Setup

- [ ] T029 [P] Create English translations file src/lib/i18n/en.json (all UI strings)
- [ ] T030 [P] Create French translations file src/lib/i18n/fr.json
- [ ] T031 [P] Create Spanish translations file src/lib/i18n/es.json
- [ ] T032 [P] Create Italian translations file src/lib/i18n/it.json
- [ ] T033 [P] Create German translations file src/lib/i18n/de.json
- [ ] T034 Initialize svelte-i18n in src/main.ts with locale detection

### Base Infrastructure

- [ ] T035 [P] Create validation utilities in src/lib/utils/validators.ts (validateTaskTitle, validateTimerDuration)
- [ ] T036 [P] Create time formatter utilities in src/lib/utils/time-formatter.ts (formatDuration, formatDate)
- [ ] T037 [P] Create error handling utility in src-tauri/src/utils/error.rs (custom error types, conversions)
- [ ] T038 [P] Setup keyboard shortcut handler in src/lib/utils/keyboard.ts (Ctrl+N, Ctrl+S, Ctrl+L)

**Checkpoint**: Foundation complete - parallel user story implementation can begin

---

## Phase 3: US1 - Basic Pomodoro Timer (P1) 🎯 MVP

**Goal**: Working timer that counts down from 25 minutes, handles pause/resume, transitions to breaks

**Independent Test**: Launch app → Click Start → Timer counts from 25:00 → Pause → Resume → Completes with notification

### Implementation

- [ ] T039 [P] [US1] Create timer store in src/lib/stores/timer.ts (currentTime, isRunning, sessionType, pomodoroCount state)
- [ ] T040 [P] [US1] Create settings store in src/lib/stores/settings.ts (load settings from Tauri, reactive updates)
- [ ] T041 [US1] Implement timer service in src/lib/services/timer-service.ts (startTimer, pauseTimer, resumeTimer, completeTimer logic)
- [ ] T042 [US1] Create TimerDisplay component in src/lib/components/timer/TimerDisplay.svelte (MM:SS display, circular progress)
- [ ] T043 [US1] Create TimerControls component in src/lib/components/timer/TimerControls.svelte (Start/Pause/Resume buttons)
- [ ] T044 [US1] Create PomodoroTimer component in src/lib/components/timer/PomodoroTimer.svelte (combines Display + Controls)
- [ ] T045 [US1] Implement notification service in src/lib/services/notification-service.ts (desktop notifications via Tauri)
- [ ] T046 [US1] Add audio notification in public/sounds/notification.mp3 and trigger on timer complete
- [ ] T047 [US1] Implement automatic break transitions (work → short break, 4 pomodoros → long break)
- [ ] T048 [US1] Create main timer view in src/routes/+page.svelte (integrate PomodoroTimer component)

### Tests (Optional but Recommended)

- [ ] T049 [P] [US1] Unit test timer service logic in tests/unit/timer-service.test.ts
- [ ] T050 [P] [US1] E2E test timer flow in tests/e2e/timer-flow.spec.ts (start, pause, complete)

**Checkpoint**: MVP Timer functional - user can complete Pomodoro cycles independently

---

## Phase 4: US2 - Task Management (P1) 🎯 MVP

**Goal**: Create, view, edit, delete, and complete tasks with persistence

**Independent Test**: Create task → Edit title → Mark complete → Delete task → Relaunch app → Tasks persisted

### Implementation

- [ ] T051 [P] [US2] Create tasks store in src/lib/stores/tasks.ts (taskList, selectedTask, filters state)
- [ ] T052 [US2] Implement task service in src/lib/services/task-service.ts (createTask, updateTask, deleteTask, toggleCompletion)
- [ ] T053 [US2] Create TaskCard component in src/lib/components/tasks/TaskCard.svelte (display task, checkbox, actions)
- [ ] T054 [US2] Create TaskList component in src/lib/components/tasks/TaskList.svelte (render all tasks, handle empty state)
- [ ] T055 [US2] Create TaskForm component in src/lib/components/tasks/TaskForm.svelte (title input, description textarea, validation)
- [ ] T056 [US2] Create TaskModal component using Shadcn Dialog (wrap TaskForm, handle create/edit modes)
- [ ] T057 [US2] Implement Ctrl+N keyboard shortcut to open new task modal
- [ ] T058 [US2] Add task deletion with confirmation dialog (Shadcn AlertDialog)
- [ ] T059 [US2] Create tasks view in src/routes/tasks/+page.svelte (TaskList + floating action button)
- [ ] T060 [US2] Implement task edit flow (click task → open modal with pre-filled data → save)

### Tests

- [ ] T061 [P] [US2] Unit test task service in tests/unit/task-service.test.ts
- [ ] T062 [P] [US2] Integration test task CRUD via Tauri commands in tests/integration/db-operations.test.ts
- [ ] T063 [P] [US2] E2E test task management flow in tests/e2e/task-management.spec.ts

**Checkpoint**: Task management fully functional - users can organize work independently of timer

---

## Phase 5: US3 - Task-Pomodoro Integration (P2)

**Goal**: Link tasks to Pomodoros, track estimated vs completed, show progress

**Independent Test**: Create task with 3 Pomodoros → Start timer for task → Complete 2 Pomodoros → See 2/3 progress

### Implementation

- [ ] T064 [P] [US3] Add Pomodoro estimator to TaskForm (1-5 selector using Shadcn Select or buttons)
- [ ] T065 [US3] Update TaskCard to show Pomodoro progress bar (completed/estimated)
- [ ] T066 [US3] Add task selector dropdown to timer view (Shadcn Select with task list)
- [ ] T067 [US3] Link timer start to selected task (pass task_id to create_session command)
- [ ] T068 [US3] Increment task completed_pomodoros when Pomodoro completes (call Tauri command)
- [ ] T069 [US3] Show visual indicator when task Pomodoros complete (checkmark, color change)
- [ ] T070 [US3] Allow tracking beyond estimate (7 completed vs 5 estimated shows "over estimate")

### Tests

- [ ] T071 [P] [US3] E2E test task-pomodoro flow in tests/e2e/timer-flow.spec.ts (extended)

**Checkpoint**: Task-timer integration complete - core value proposition delivered

---

## Phase 6: US4 - Daily Summary & Analytics (P2)

**Goal**: Show completed tasks and Pomodoros per day, weekly/monthly views

**Independent Test**: Complete 3 tasks and 5 Pomodoros → View summary → See counts and charts

### Implementation

- [ ] T072 [P] [US4] Create summary store in src/lib/stores/summary.ts (dailyData, weeklyData state)
- [ ] T073 [US4] Implement summary calculations in Tauri summary commands (aggregate sessions by date)
- [ ] T074 [US4] Create DailySummary component in src/lib/components/summary/DailySummary.svelte (today's stats cards)
- [ ] T075 [P] [US4] Create WeeklyChart component in src/lib/components/summary/WeeklyChart.svelte (bar chart using Chart.js or custom SVG)
- [ ] T076 [P] [US4] Create MonthlyView component in src/lib/components/summary/MonthlyView.svelte (heatmap or calendar view)
- [ ] T077 [US4] Create summary view in src/routes/summary/+page.svelte (tabs for daily/weekly/monthly)
- [ ] T078 [US4] Add date picker for historical data viewing (Shadcn Calendar)

### Tests

- [ ] T079 [P] [US4] Unit test summary calculations in tests/unit/summary.test.ts

**Checkpoint**: Analytics complete - users can track productivity patterns

---

## Phase 7: US5 - Customizable Timer Settings (P3)

**Goal**: Adjust work/break durations, customize timer behavior

**Independent Test**: Open settings → Change work to 30min → Start timer → Counts from 30:00

### Implementation

- [ ] T080 [P] [US5] Create SettingsPanel component in src/lib/components/settings/SettingsPanel.svelte (tabbed interface)
- [ ] T081 [US5] Add timer duration sliders (Shadcn Slider for work/short/long durations, 1-180 min validation)
- [ ] T082 [US5] Add Pomodoros-until-long-break input (Shadcn Input, 1-10 validation)
- [ ] T083 [US5] Add auto-start toggles (Shadcn Switch for auto-start breaks/pomodoros)
- [ ] T084 [US5] Implement settings save functionality (call update_settings Tauri command)
- [ ] T085 [US5] Add reset to defaults button
- [ ] T086 [US5] Create settings view in src/routes/settings/+page.svelte
- [ ] T087 [US5] Ensure settings persist across app restarts (verify SQLite singleton pattern)

### Tests

- [ ] T088 [P] [US5] E2E test settings persistence in tests/e2e/settings.spec.ts

**Checkpoint**: Timer customization complete - users can adapt to personal workflows

---

## Phase 8: US6 - Multi-Language Support (P2)

**Goal**: Switch between 5 languages instantly, persist preference

**Independent Test**: Press Ctrl+L → Select Français → All UI updates → Restart → Still French

### Implementation

- [ ] T089 [P] [US6] Create LanguageSelector component in src/lib/components/settings/LanguageSelector.svelte (Shadcn RadioGroup)
- [ ] T090 [US6] Implement Ctrl+L keyboard shortcut to open language modal
- [ ] T091 [US6] Wire language change to svelte-i18n locale setter
- [ ] T092 [US6] Save language preference to settings (call update_settings)
- [ ] T093 [US6] Load language from settings on app start (initialize i18n with persisted locale)
- [ ] T094 [US6] Add locale-aware date/time formatting in time-formatter.ts (use Intl.DateTimeFormat)
- [ ] T095 [US6] Verify all UI components use $t() for text (audit pass)

### Tests

- [ ] T096 [P] [US6] E2E test language switching in tests/e2e/settings.spec.ts (extended)

**Checkpoint**: Multi-language support complete - international user base accessible

---

## Phase 9: US7 - Theme Toggle (P3)

**Goal**: Light/dark theme switching with persistence

**Independent Test**: Toggle theme → UI switches colors → Restart → Theme persists

### Implementation

- [ ] T097 [P] [US7] Create ThemeToggle component in src/lib/components/settings/ThemeToggle.svelte (Shadcn Switch or toggle button)
- [ ] T098 [US7] Implement theme store or add to settings store (light/dark state)
- [ ] T099 [US7] Add Tailwind dark mode classes to all components (dark:bg-gray-900, etc.)
- [ ] T100 [US7] Wire theme toggle to settings persistence (update_settings)
- [ ] T101 [US7] Add system theme detection on first launch (prefers-color-scheme)
- [ ] T102 [US7] Apply theme class to root element (<html class="dark">)

### Tests

- [ ] T103 [P] [US7] E2E test theme toggle in tests/e2e/settings.spec.ts (extended)

**Checkpoint**: Theme support complete - comfortable use in any lighting

---

## Phase 10: US8 - System Tray Integration (P3)

**Goal**: App runs in tray, quick actions available, notifications work in background

**Independent Test**: Close window → App in tray → Click tray → Window restores → Timer continues

### Implementation

- [ ] T104 [US8] Configure Tauri system tray in src-tauri/tauri.conf.json (icon, title)
- [ ] T105 [US8] Add tray icons for different states (idle, running, break) in src-tauri/icons/
- [ ] T106 [US8] Implement tray menu in src-tauri/src/main.rs (Show/Hide, Start/Pause, Quit)
- [ ] T107 [US8] Handle window close event to minimize to tray instead of quit
- [ ] T108 [US8] Implement tray click handler to restore window
- [ ] T109 [US8] Update tray icon based on timer state (running vs idle)
- [ ] T110 [US8] Ensure desktop notifications work when window is hidden
- [ ] T111 [US8] Add tray tooltip showing current timer state ("25:00 remaining")

### Tests

- [ ] T112 [P] [US8] E2E test tray integration (manual testing required for tray interactions)

**Checkpoint**: System tray complete - seamless background operation

---

## Phase 11: US9 - Project Organization (P3)

**Goal**: Group tasks by projects, filter by project

**Independent Test**: Create project "Work" → Assign 3 tasks → Filter by "Work" → See only those 3

### Implementation

- [ ] T113 [P] [US9] Create projects store in src/lib/stores/projects.ts (projectList state)
- [ ] T114 [US9] Create ProjectSelector component for TaskForm (Shadcn Combobox or Select)
- [ ] T115 [US9] Create ProjectFilter component for TaskList (Shadcn RadioGroup or buttons)
- [ ] T116 [US9] Add create new project inline in TaskForm (quick add)
- [ ] T117 [US9] Implement project color picker (Shadcn Popover with color swatches)
- [ ] T118 [US9] Group tasks by project in TaskList (section headers)
- [ ] T119 [US9] Show project totals (estimated/completed Pomodoros per project)
- [ ] T120 [US9] Add project management modal (list, edit, delete projects)

### Tests

- [ ] T121 [P] [US9] E2E test project organization in tests/e2e/task-management.spec.ts (extended)

**Checkpoint**: Project organization complete - power users can manage complex workflows

---

## Phase 12: US10 - Comprehensive Keyboard Shortcuts (P2)

**Goal**: All features accessible via keyboard, logical tab order

**Independent Test**: Use only keyboard to create task, start timer, navigate views, change settings

### Implementation

- [ ] T122 [P] [US10] Audit all interactive elements for keyboard accessibility
- [ ] T123 [US10] Implement Tab navigation order (logical flow through UI)
- [ ] T124 [US10] Add Escape key handlers to close modals/dialogs
- [ ] T125 [US10] Add Enter key handler for form submissions
- [ ] T126 [US10] Implement Space key for timer start/pause (alternative to Ctrl+S)
- [ ] T127 [US10] Add arrow key navigation for lists (task list, project list)
- [ ] T128 [US10] Create keyboard shortcuts help modal (triggered by ?)
- [ ] T129 [US10] Add focus trap in modals (tab cycles within modal)
- [ ] T130 [US10] Ensure focus indicators visible on all elements (Tailwind ring utilities)

### Tests

- [ ] T131 [P] [US10] E2E test keyboard navigation in tests/e2e/accessibility.spec.ts (all shortcuts)

**Checkpoint**: Keyboard accessibility complete - WCAG 2.1 AA compliant

---

## Phase 13: Polish & Optimization

**Purpose**: Performance tuning, final touches, production readiness

- [ ] T132 [P] Add loading states to all async operations (Shadcn Skeleton components)
- [ ] T133 [P] Add empty states with illustrations (empty task list, no summary data)
- [ ] T134 [P] Implement optimistic UI updates (mark task complete instantly, sync in background)
- [ ] T135 [P] Add error boundaries and fallback UI
- [ ] T136 [P] Optimize bundle size (analyze with vite-bundle-visualizer)
- [ ] T137 [P] Add virtual scrolling for task list (if >100 tasks)
- [ ] T138 [P] Profile and optimize re-renders (Svelte devtools)
- [ ] T139 [P] Add database migration testing (test upgrade from v1 to v2 schema)
- [ ] T140 [P] Create app icon and splash screen (multiple sizes for all platforms)
- [ ] T141 [P] Configure Tauri build optimizations (minification, compression)
- [ ] T142 [P] Add crash reporting (optional: Sentry or similar)

---

## Phase 14: Testing & Quality Assurance

**Purpose**: Comprehensive testing, accessibility audit, security review

- [ ] T143 Run full E2E test suite on all platforms (Windows, macOS, Linux)
- [ ] T144 Perform manual accessibility testing with screen reader (NVDA/JAWS)
- [ ] T145 Run WCAG contrast checker on all color combinations
- [ ] T146 Security audit: verify no SQL injection vulnerabilities (code review + automated scan)
- [ ] T147 Security audit: verify input validation on all user inputs
- [ ] T148 Performance testing: verify startup < 2s, transitions < 100ms, queries < 50ms
- [ ] T149 Load testing: test with 10,000 tasks, verify no degradation
- [ ] T150 Run TypeScript compiler with --noEmit (zero errors)
- [ ] T151 Run Rust Clippy (zero warnings)
- [ ] T152 Run ESLint and Prettier (zero errors)
- [ ] T153 Verify all code comments are in French (automated check or manual review)
- [ ] T154 Calculate unit test coverage (verify ≥70%)

---

## Phase 15: Documentation & Deployment

**Purpose**: User documentation, build scripts, release preparation

- [ ] T155 [P] Update README.md with project description, installation instructions
- [ ] T156 [P] Create CONTRIBUTING.md with development setup guide
- [ ] T157 [P] Document keyboard shortcuts in app and README
- [ ] T158 [P] Create user guide for first-time users (optional: in-app tour)
- [ ] T159 [P] Setup GitHub Actions CI/CD (lint, test, build on PR)
- [ ] T160 [P] Configure Tauri updater for auto-updates (optional but recommended)
- [ ] T161 [P] Create release build scripts for all platforms
- [ ] T162 [P] Test installation on clean systems (Windows 10+, macOS 10.15+, Ubuntu 20.04+)
- [ ] T163 Write release notes for v1.0.0
- [ ] T164 Create demo video or screenshots for GitHub README

---

## Phase 16: First Commit & Push

**Purpose**: Commit all work and push to development branch

- [ ] T165 Stage all files with git add
- [ ] T166 Commit with message: "feat: initial TomatoTask implementation with full Pomodoro + TodoList features"
- [ ] T167 Push to branch claude/tomatotask-setup-011CV5fqQiDPnEwd4zk32iBp with -u flag

---

## Summary

**Total Tasks**: 167
**Estimated Effort**: 80-120 hours (2-3 weeks full-time)

**MVP (P1 - US1 + US2)**: Tasks T001-T063 (~40% of effort)
**Core Value (P2 - US3, US4, US6, US10)**: Tasks T064-T096 + T122-T131 (~35% of effort)
**Enhancement (P3 - US5, US7, US8, US9)**: Tasks T080-T121 (~15% of effort)
**Polish & Release**: Tasks T132-T167 (~10% of effort)

**Next Step**: Begin Phase 1 (Setup & Boilerplate Integration) → T001
