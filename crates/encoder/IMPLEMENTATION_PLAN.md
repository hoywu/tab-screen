## Phase 0 Plan

- Scope: define encoder capability/config models, `EncoderBackend`, reconfigure outcome types, and encoded frame structures.
- Deliverables: a compilable Rust library crate exposing the media encoding abstraction required by the architecture document.
- Boundaries: no actual H.264 implementation in Phase 0.
- Validation: workspace `cargo check`.

## Self-Review

- The plan provides the contract needed by later streaming work while avoiding premature implementation choices before Phase 1/2 validation.
- Explicit reconfigure outcomes preserve the architecture's hot-update versus rebuild distinction.
