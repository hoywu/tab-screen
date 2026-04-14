## Phase 0 Plan

- Scope: define raw TOML models, normalized/effective config skeletons, defaults, and a validation entrypoint matching the architecture's three-layer config model.
- Deliverables: a compilable Rust library crate with default config generation, placeholder normalization/validation flow, and unit tests for default values.
- Boundaries: no filesystem loading orchestration in this module, and no environment or CLI merge logic beyond API placeholders.
- Validation: `cargo test -p config` and workspace `cargo check`.

## Self-Review

- The plan preserves the required raw/normalized/effective separation without overbuilding the loader logic before later phases need it.
- Default generation plus validation hooks are enough for Phase 0 CLI scaffolding and later expansion.
