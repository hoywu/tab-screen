## Phase 0 Plan

- Scope: define `DisplayBackend`, `DisplayHandle`, probe/result structs, and a no-op placeholder backend for compile-time integration.
- Deliverables: a compilable Rust library crate containing the backend abstraction surface required by the architecture document.
- Boundaries: no real Wayland backend implementation and no Phase 1 validation yet.
- Validation: workspace `cargo check`.

## Self-Review

- The plan intentionally stops at abstractions because backend viability must be proven separately in Phase 1.
- Exposing probe/result structs now gives `doctor` and `probe` a stable type surface to grow into later.
