## Phase 0 Plan

- Scope: define session state enums, core service error type wrappers, and placeholders for session manager and negotiation entrypoints.
- Deliverables: a compilable Rust library crate exposing the minimal state-machine surface required by the architecture document.
- Boundaries: no real networking, display creation, persistence, or streaming loops yet.
- Validation: workspace `cargo check`.

## Self-Review

- The plan keeps `server-core` at interface level so Phase 1 and Phase 2 can add behavior without reworking crate boundaries.
- Avoiding runtime implementation now matches the roadmap requirement to first establish a Phase 0 baseline.
