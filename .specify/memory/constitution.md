<!--
Sync Impact Report:
Version change: none → 1.0.0
Modified principles: N/A (initial version)
Added sections: All (initial constitution)
Removed sections: none
Templates requiring updates: ✅ All templates aligned
Follow-up TODOs: none
-->

# TomatoTask Constitution

## Core Principles

### I. Clean Architecture & DRY
Every component, function, and module must follow the DRY (Don't Repeat Yourself) principle strictly. Code duplication is forbidden—shared logic must be extracted into reusable utilities, composables, or services. Architecture must maintain clear separation of concerns:
- **UI Layer**: Svelte components (presentation only)
- **Business Logic**: TypeScript services and Svelte stores
- **Data Layer**: Rust/Tauri commands interfacing with SQLite

**Rationale**: Maintainability and scalability require eliminating redundancy and enforcing modular design.

### II. Code Comments in French (NON-NEGOTIABLE)
All code comments, documentation strings, and inline explanations MUST be written in French. This includes:
- Function/method documentation
- Inline code comments
- Complex logic explanations
- TODO/FIXME notes

**Rationale**: Team collaboration standard and code ownership clarity.

### III. Type Safety & Static Analysis
Strict TypeScript mode must be enabled for all frontend code. Rust backend must use Clippy with no warnings tolerated. Types must be:
- Defined explicitly (avoid `any` in TypeScript)
- Shared between frontend/backend using serializable interfaces
- Validated at compile-time wherever possible

**Rationale**: Prevent runtime errors, improve developer experience, and ensure contract compliance.

### IV. Performance Optimization
Application must be optimized for performance:
- **Minimal re-renders**: Use Svelte's reactivity efficiently, avoid unnecessary component updates
- **Efficient database queries**: Use prepared statements, indexes on frequently queried columns
- **Lazy loading**: Load components and data on-demand
- **Debouncing**: Limit high-frequency operations (search, input handlers)
- **Bundle size**: Keep production bundle under 5MB

**Performance Targets**:
- App startup: < 2 seconds
- View transitions: < 100ms
- Database queries: < 50ms
- Timer precision: ± 100ms

**Rationale**: User experience degrades rapidly with poor performance. Productivity tools must be fast and responsive.

### V. Multi-Language Support (i18n)
Application must support 5 languages: English (default), French, Spanish, Italian, German.
- All UI text must be externalized using i18n framework (svelte-i18n)
- Language switching must work without restart
- Dates, times, and numbers must respect locale conventions
- No hardcoded strings in components

**Rationale**: Accessibility to global users and market expansion.

### VI. Accessibility (WCAG 2.1 AA)
Application must meet WCAG 2.1 Level AA standards:
- **Keyboard navigation**: All features accessible via keyboard shortcuts
- **Screen readers**: Proper ARIA labels and semantic HTML
- **Contrast ratios**: Minimum 4.5:1 for normal text
- **Focus indicators**: Clear, visible focus states
- **Alternative text**: For all non-text content

**Keyboard Shortcuts**:
- `Ctrl+N`: New task
- `Ctrl+S`: Start/stop timer
- `Ctrl+L`: Language selection

**Rationale**: Inclusive design ensures usability for all users regardless of abilities.

### VII. Modular Component Architecture
All UI components must follow Shadcn-svelte patterns:
- Components must be composable and reusable
- Props must be typed and documented
- Side effects must be minimal and explicit
- Component size must stay manageable (< 300 lines)

**Rationale**: Component reusability reduces development time and ensures UI consistency.

## Technology Stack Requirements

### Mandatory Technologies
- **Framework**: Tauri 2 (Rust backend) + Svelte 5 (Runes API)
- **Language**: TypeScript (strict mode) + Rust (latest stable)
- **Styling**: TailwindCSS + Shadcn-svelte component library
- **Database**: SQLite with rusqlite
- **State Management**: Svelte stores (writable, derived, readable)
- **i18n**: svelte-i18n or equivalent
- **Notifications**: Desktop notifications + system tray integration

### Boilerplate Base
Project must be built on: https://github.com/alysonhower/tauri2-svelte5-shadcn
- Respect existing directory structure
- Extend configurations, don't replace them
- Keep boilerplate tooling intact (Vite, etc.)

### Development Tools
- **Package Manager**: npm or pnpm
- **Linter**: ESLint + Prettier for TypeScript/Svelte
- **Rust Linter**: Clippy (no warnings allowed)
- **Testing**: Vitest (unit tests) + Playwright (E2E)
- **Type Checking**: tsc --noEmit (CI enforcement)

## Feature Requirements

### Mandatory Features
1. **Pomodoro Timer**: Configurable work/break intervals
   - Default: 25 min work, 5 min short break, 15 min long break
   - Customizable via settings
   - Visual and audio notifications
   - Pause/resume functionality

2. **TodoList**: Full CRUD operations
   - Create, edit, delete tasks
   - Organize by project/category
   - Mark as complete/incomplete
   - Persistent storage in SQLite

3. **Task-to-Pomodoro Mapping**: Estimate effort per task
   - Assign 1-5 Pomodoros per task
   - Track actual vs. estimated
   - Visual progress indicators

4. **Daily Summary**: Analytics and insights
   - Completed tasks count
   - Completed Pomodoros count
   - Historical data (weekly, monthly views)

5. **Settings Panel**: User customization
   - Theme toggle (light/dark)
   - Language selection
   - Timer customization
   - Notification preferences

6. **System Integration**: Desktop features
   - System tray icon with quick actions
   - Desktop notifications
   - Auto-launch on startup (optional)

## Security & Data Integrity

### Mandatory Security Practices
- **Input Validation**: All user input validated on frontend AND backend
- **SQL Injection Prevention**: Use only prepared statements (no string concatenation)
- **XSS Prevention**: Escape user-generated content before rendering
- **Error Handling**: Never expose internal errors to UI (log safely)
- **Data Backup**: Export/import functionality for user data

## Testing Standards

### Required Test Coverage
- **Unit Tests**: Business logic, utilities, stores (minimum 70% coverage)
- **Integration Tests**: Tauri command flows, database operations
- **E2E Tests**: Critical user flows (create task, start timer, view summary)
- **Regression Tests**: All fixed bugs must have regression tests

### Test Execution Gates
- All tests must pass before commit
- CI must run full test suite on PR
- E2E tests must pass before release

## Git & Version Control

### Commit Standards
- **Atomic commits**: One logical change per commit
- **Conventional Commits**: Format: `type(scope): description`
  - Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
- **Clear messages**: Explain WHY, not just WHAT
- **Frequent commits**: Commit often, push when feature complete

### Branch Strategy
- **Development Branch**: `claude/tomatotask-setup-011CV5fqQiDPnEwd4zk32iBp`
- **Feature branches**: For experimental work
- **Main branch**: Protected, requires PR review

### Code Review Requirements
- All code must be reviewed before merge
- Reviewer must check: DRY compliance, French comments, type safety, tests

## Governance

### Constitution Authority
This constitution supersedes all other development practices and guidelines. Any conflict between this document and other guidance must be resolved in favor of the constitution.

### Amendment Process
1. Amendments must be proposed with clear rationale
2. Impact analysis on existing code required
3. Version bump according to semantic versioning:
   - **MAJOR**: Breaking principle changes
   - **MINOR**: New principles added
   - **PATCH**: Clarifications, typo fixes
4. All dependent templates and documentation must be updated

### Compliance Verification
- Every pull request must verify constitutional compliance
- Automated linting and type checking enforced in CI
- Manual review for architectural and principle adherence
- Non-compliance blocks merge

### Development Guidance
Runtime development guidance and best practices are maintained separately but must align with constitutional principles. In case of conflict, constitution wins.

**Version**: 1.0.0 | **Ratified**: 2025-11-13 | **Last Amended**: 2025-11-13
