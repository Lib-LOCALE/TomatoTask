# TomatoTask

**Cross-platform Pomodoro timer and task manager**
Built with Svelte, TypeScript, Tauri, Rust, and SQLite.

[Download](https://github.com/Lib-LOCALE/TomatoTask/releases/latest) · [Usage](#usage) · [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md)

![License](https://img.shields.io/badge/license-MIT-green.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-lightgrey.svg)

## Overview

TomatoTask is a local-first desktop productivity app that combines Pomodoro sessions, task management, project organization, statistics, themes, keyboard shortcuts, and native notifications.

This repository is maintained as a quality-focused desktop application project: clear documentation, structured contribution guidelines, security policy, automated checks, and release packaging.

## Screenshots

<table>
  <tr>
    <td width="50%">
      <img src=".github/images/Capture%20d'écran%202025-12-01%20095142.png" alt="Main timer interface" />
      <p align="center"><strong>Main timer interface</strong></p>
    </td>
    <td width="50%">
      <img src=".github/images/Capture%20d'écran%202025-12-01%20095150.png" alt="Task management" />
      <p align="center"><strong>Task management and projects</strong></p>
    </td>
  </tr>
  <tr>
    <td width="50%">
      <img src=".github/images/Capture%20d'écran%202025-12-01%20095157.png" alt="Statistics" />
      <p align="center"><strong>Statistics and productivity</strong></p>
    </td>
    <td width="50%">
      <img src=".github/images/Capture%20d'écran%202025-12-01%20095207.png" alt="Settings" />
      <p align="center"><strong>Settings and customization</strong></p>
    </td>
  </tr>
</table>

## Features

- Custom work, short break, and long break durations
- Visual timer progress and dynamic window title
- Task creation, filtering, completion tracking, and project organization
- Daily and weekly productivity statistics
- Data export for backup and recovery
- Light and dark themes
- Multilingual interface: English, French, Spanish, Italian, and German
- Keyboard shortcuts for common actions
- Native desktop notifications
- Windows and Linux packaging through Tauri

## Technology Stack

- Svelte 5 and TypeScript for the user interface
- Tauri and Rust for the desktop shell
- SQLite for local persistence
- Tailwind CSS for styling
- Vite for the build pipeline
- Vitest and Playwright for testing
- ESLint and Prettier for code quality

## Quality Signals

This project includes:

- `npm run check` for Svelte and TypeScript validation
- `npm run lint` for formatting and lint checks
- `npm run test:unit` for unit tests
- `npm run test:e2e` for end-to-end tests
- GitHub Actions workflows for checks, builds, releases, and security
- `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, and `CODE_OF_CONDUCT.md`

## Installation

Download the latest release:

[TomatoTask releases](https://github.com/Lib-LOCALE/TomatoTask/releases/latest)

### Windows

Download and run the x64 installer from the latest release.

### Linux

Download the AppImage, make it executable, and run it:

```bash
chmod +x TomatoTask_*_amd64.AppImage
./TomatoTask_*_amd64.AppImage
```

## Development

Install dependencies:

```bash
npm install
```

Run the development server:

```bash
npm run dev
```

Run quality checks:

```bash
npm run check
npm run lint
npm run test:unit
```

Run the Tauri app:

```bash
npm run tauri dev
```

## Usage

1. Launch TomatoTask.
2. Configure timer durations and preferences.
3. Create a task or project.
4. Start a focus session.
5. Track completed sessions and productivity statistics.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
