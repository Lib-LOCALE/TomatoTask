# Contributing to TomatoTask

Use the [build prerequisites](README.md#build-from-source) to set up Rust and your desktop build dependencies.

```sh
cargo run --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

The desktop interface lives in `ui/`. Rust connects its callbacks in `src/main.rs` and stores tasks, projects, sessions and preferences through the operations in `src/commands/` and SQLite queries in `src/db/`. Keep database migrations and JSON backups compatible with existing installations.

Use a separate data directory when testing destructive operations on Linux:

```sh
XDG_DATA_HOME="$(mktemp -d)" cargo run --locked
```

For UI changes, exercise the affected flow in both themes. For changes to tasks, settings or sessions, restart the application and verify that data persists.

Open an [issue](https://github.com/Lib-LOCALE/TomatoTask/issues) with your platform, version and reproduction steps. Follow the [code of conduct](CODE_OF_CONDUCT.md) and report vulnerabilities through the [security policy](SECURITY.md).
