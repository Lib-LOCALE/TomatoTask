# 🍅 TomatoTask

> A modern, feature-rich Pomodoro Timer application with task management, built with Tauri 2, Svelte 5, and SQLite.

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/AnthonyMahe/TomatoTask)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-orange.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-5-red.svg)](https://svelte.dev/)

## ✨ Features

### 🎯 Core Functionality

- **Pomodoro Timer** - Customizable work/break durations (default: 25/5/15 minutes)
- **Task Management** - Complete CRUD operations with project organization
- **Project Organization** - Color-coded projects with task filtering
- **Daily & Weekly Statistics** - Track your productivity with detailed summaries
- **System Tray Integration** - Minimize to tray with native notifications

### 🌍 Internationalization

Full support for 5 languages:
- 🇬🇧 English
- 🇫🇷 Français
- 🇪🇸 Español
- 🇮🇹 Italiano
- 🇩🇪 Deutsch

### 🎨 User Experience

- **Theme Support** - Light and dark modes with persistence
- **Keyboard Shortcuts** - Quick actions (Ctrl+S, Ctrl+N, Ctrl+L, Ctrl+/)
- **Visual Feedback** - Animated completion celebrations
- **Responsive Design** - Modern UI with Tailwind CSS and Shadcn components

## 📸 Screenshots

> _Screenshots coming soon_

## 🚀 Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://www.rust-lang.org/) (latest stable)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

```bash
# Clone the repository
git clone https://github.com/AnthonyMahe/TomatoTask.git
cd TomatoTask

# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Build for Production

```bash
# Build the application
npm run tauri build

# The installer will be in src-tauri/target/release/bundle/
```

## 📖 Usage

### Timer Controls

- **Start/Pause**: Click the timer button or press `Ctrl+S`
- **Session Types**: Automatic cycling between Work → Short Break → Long Break
- **Auto-advance**: Configurable automatic session transitions

### Task Management

- **Create Task**: Click "New Task" or press `Ctrl+N`
- **Edit Task**: Click the edit icon on any task card
- **Delete Task**: Click the delete icon with confirmation
- **Assign to Project**: Select project when creating/editing tasks
- **Track Pomodoros**: Visual progress bar shows completed/estimated pomodoros

### Projects

- **Create Project**: Click "+" in the projects panel
- **Color Coding**: Choose from 8 preset colors or custom color
- **Filter Tasks**: Click any project to filter tasks
- **Edit/Delete**: Hover over project for actions

### Settings

Access via the ⚙️ button (top-right):

- **Timer Settings**: Customize work/break durations and auto-start options
- **Appearance**: Toggle theme and select language
- **Keyboard Shortcuts**: View all shortcuts with `Ctrl+/`

### System Tray

- **Minimize to Tray**: Close window to minimize (doesn't quit)
- **Show/Hide**: Click tray icon or use tray menu
- **Notifications**: Native system notifications for timer events
- **Quit**: Right-click tray icon → Quit

## 🏗️ Architecture

### Technology Stack

**Frontend:**
- [Svelte 5](https://svelte.dev/) - Reactive UI framework with Runes API
- [TypeScript](https://www.typescriptlang.org/) - Type-safe JavaScript
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework
- [Shadcn-svelte](https://www.shadcn-svelte.com/) - Reusable component library
- [svelte-i18n](https://github.com/kaisermann/svelte-i18n) - Internationalization

**Backend:**
- [Tauri 2](https://tauri.app/) - Lightweight desktop application framework
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [SQLite](https://www.sqlite.org/) - Embedded database
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite bindings for Rust

### Project Structure

```
TomatoTask/
├── src/                          # Frontend source
│   ├── lib/
│   │   ├── components/          # Svelte components
│   │   │   ├── timer/          # Timer-related components
│   │   │   ├── tasks/          # Task management components
│   │   │   ├── projects/       # Project organization components
│   │   │   ├── summary/        # Statistics components
│   │   │   ├── settings/       # Settings and preferences
│   │   │   └── keyboard/       # Keyboard shortcuts
│   │   ├── services/           # Business logic layer
│   │   ├── stores/             # Reactive state management
│   │   ├── types/              # TypeScript type definitions
│   │   ├── utils/              # Utility functions
│   │   └── i18n/               # Translation files
│   ├── App.svelte              # Main application component
│   └── main.ts                 # Application entry point
├── src-tauri/                   # Backend source
│   ├── src/
│   │   ├── commands/           # Tauri commands (API)
│   │   ├── db/                 # Database layer
│   │   │   ├── migrations.rs  # Schema migrations
│   │   │   ├── models.rs      # Data models
│   │   │   └── queries/       # SQL queries
│   │   ├── tray.rs             # System tray integration
│   │   ├── notifications.rs    # System notifications
│   │   └── lib.rs              # Application entry point
│   └── tauri.conf.json         # Tauri configuration
└── IMPLEMENTATION_STATUS.md     # Detailed implementation docs
```

### Database Schema

SQLite database with 4 tables:
- `projects` - Project organization with colors
- `tasks` - Task items with Pomodoro tracking
- `pomodoro_sessions` - Historical session data
- `settings` - User preferences and configuration

## 🛠️ Development

### Available Scripts

```bash
# Development
npm run dev          # Start Vite dev server
npm run tauri dev    # Run Tauri in development mode

# Building
npm run build        # Build frontend
npm run tauri build  # Build complete application

# Type Checking
npm run check        # Run Svelte and TypeScript checks

# Preview
npm run preview      # Preview production build
```

### Tauri Commands

The application exposes 23 Tauri commands:

**Tasks** (7): `get_tasks`, `get_task`, `create_task`, `update_task`, `delete_task`, `toggle_task_completion`, `get_tasks_by_project`

**Projects** (4): `get_projects`, `create_project`, `update_project`, `delete_project`

**Sessions** (4): `create_session`, `complete_session`, `interrupt_session`, `get_sessions_by_date_range`

**Settings** (2): `get_settings`, `update_settings`

**Summary** (2): `get_daily_summary`, `get_weekly_summary`

**Notifications** (1): `send_custom_notification`

**Tray** (4): `toggle_window`, `show_window`, `hide_window`, `quit_app`

### Code Style

- All Rust code comments in French (as per project requirements)
- TypeScript strict mode enabled
- Rust Clippy compliance
- DRY principle enforced throughout

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+S` | Start/Stop timer |
| `Ctrl+N` | New task |
| `Ctrl+L` | Language selector |
| `Ctrl+/` | Show shortcuts help |

## 📊 Statistics

- **Total Lines of Code**: ~8,500+
- **Components**: 24 Svelte components
- **Services**: 6 TypeScript services
- **Stores**: 4 reactive stores (timer, tasks, settings, projects)
- **Tauri Commands**: 23 backend commands
- **i18n Keys**: 125+ translation keys
- **Supported Languages**: 5
- **Commits**: 19

## 🗺️ Roadmap

### Completed ✅

- ✅ US1 - Basic Pomodoro Timer
- ✅ US2 - Task Management
- ✅ US3 - Task-Pomodoro Integration
- ✅ US4 - Daily Summary
- ✅ US5 - Settings Panel
- ✅ US6 - Language Selector
- ✅ US7 - Theme Toggle
- ✅ US8 - System Tray & Notifications
- ✅ US9 - Project Organization
- ✅ US10 - Keyboard Shortcuts Help

### Future Enhancements 🔮

- [ ] Cloud sync for multi-device support
- [ ] Advanced analytics and charts
- [ ] Custom timer sounds
- [ ] Pomodoro history timeline
- [ ] Export data (CSV, JSON)
- [ ] Integration with task management tools (Trello, Notion, etc.)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'feat: add some amazing feature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/) - Thanks to the Tauri team
- UI components from [Shadcn-svelte](https://www.shadcn-svelte.com/)
- Inspired by the [Pomodoro Technique](https://francescocirillo.com/pages/pomodoro-technique) by Francesco Cirillo
- Boilerplate from [tauri2-svelte5-shadcn](https://github.com/alysonhower/tauri2-svelte5-shadcn)

## 📞 Support

For bugs, questions, and discussions please use the [GitHub Issues](https://github.com/AnthonyMahe/TomatoTask/issues).

---

**Made with ❤️ and 🍅 by [Anthony Mahé](https://github.com/AnthonyMahe)**

**Developed following Spec-Driven Development with [GitHub Spec Kit](https://github.com/speckit-ai/speckit)**
