## Phase 0 Plan

- Scope: create the CLI entrypoint, `clap` command tree, tracing bootstrap, and placeholder command handlers for `serve`, `doctor`, `probe`, `print-default-config`, `usb adb-reverse`, and `version`.
- Deliverables: a compilable Rust binary crate whose commands return placeholder output aligned with the roadmap.
- Boundaries: no real server runtime, environment probing, or ADB execution yet.
- Validation: `cargo run -p app-cli -- --help`, representative subcommand help/output, and workspace `cargo check`.

## Self-Review

- The plan satisfies the Phase 0 CLI acceptance criteria without pretending later runtime features already exist.
- Tracing bootstrap is included now because later modules will depend on a consistent logging entrypoint.
