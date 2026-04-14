## Phase 0 Plan

- Scope: define raw frame types, pixel/format enums, and the `CaptureSource` trait expected by display and encoder layers.
- Deliverables: a compilable Rust library crate with minimal frame data structures and helper constructors.
- Boundaries: no actual capture backend implementation in Phase 0.
- Validation: workspace `cargo check`.

## Self-Review

- The plan creates only the shared capture contract, which is enough for later backend work and keeps Phase 0 small.
- Helper constructors reduce boilerplate in later tests without forcing a concrete backend choice.
