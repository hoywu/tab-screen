## Phase 0 Plan

- Scope: define shared protocol constants, message enums, negotiation models, error codes, and media packet metadata types required by the architecture document.
- Deliverables: a compilable Rust library crate with serde-friendly protocol models and unit tests covering tagged enum serialization for representative messages.
- Boundaries: no transport I/O, no session orchestration, and no business-rule negotiation logic in this module.
- Validation: `cargo test -p protocol` and workspace `cargo check`.

## Self-Review

- The plan keeps `protocol` focused on shared data structures so later crates can depend on it without pulling in runtime concerns.
- The validation target is achievable in Phase 0 and provides evidence that the wire model shape is stable enough to continue.
