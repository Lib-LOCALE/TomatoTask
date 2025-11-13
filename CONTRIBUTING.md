# Contributing to TomatoTask

Thank you for your interest in contributing to TomatoTask!

## 🛠️ Development Setup

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
```

### Database Schema

SQLite database with 4 tables:
- `projects` - Project organization with colors
- `tasks` - Task items with Pomodoro tracking
- `pomodoro_sessions` - Historical session data
- `settings` - User preferences and configuration

## 🛠️ Available Scripts

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

## 🤝 Contributing Guidelines

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'feat: add some amazing feature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Code Style

- All Rust code comments in French (as per project requirements)
- TypeScript strict mode enabled
- Rust Clippy compliance
- DRY principle enforced throughout

## 📞 Support

For bugs, questions, and discussions please use the [GitHub Issues](https://github.com/AnthonyMahe/TomatoTask/issues).
