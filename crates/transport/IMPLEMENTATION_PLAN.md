## Phase 0 Plan

- Scope: define transport-facing session path constants, heartbeat defaults, and placeholder server types that later phases will implement with WebSocket I/O.
- Deliverables: a compilable Rust library crate with protocol-facing transport constants and minimal API skeletons.
- Boundaries: no active listener, socket handling, or binary frame processing yet.
- Validation: workspace `cargo check`.

## Self-Review

- The plan avoids fake networking while still creating a dedicated crate boundary for later Phase 2 work.
- Shared constants now reduce duplication between CLI startup and future client/server transport code.
