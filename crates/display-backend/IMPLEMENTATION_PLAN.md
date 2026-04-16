## Phase 1 Plan

- Scope: replace the Phase 0 placeholder direction with a concrete Phase 1
 `evdi + libevdi` backend validation and minimal capture plan for the Rust server.
- Goal: prove that `evdi` is the primary `DisplayBackend` path for MVP on the
 current Arch Linux target, and that it can create/destroy a virtual display, expose a stable logical identity, and return non-empty captured frames.
- Primary deliverables:
  - A real `evdi` backend implementation behind
 the existing `DisplayBackend` and
 `DisplayHandle` abstractions.
  - A minimal `evdi`-backed
 `CaptureSource` implementation that can register buffers
, request updates
, process `mode_changed` / `update_ready` events, and grab framebuffer pixels.
  - CLI validation entrypoints for
 `probe` and
 an explicit Phase 1 validation flow that exercise `probe -> create -> wait -> capture -> destroy`.
  - A Phase 1
 validation record in `docs/` documenting prerequisites, supported environment
 assumptions, observed results, and known limitations.
  - Synchronized updates to `docs
/prd.md`, `docs/architecture.md`, `docs/implementation-roadmap.md`, and `docs/implementation-status.md
` so repo documentation matches the new backend
 decision.

## Technical Direction

- Backend choice:
  - Use
 `evdi` as the
 Phase 1 primary backend.
  -
 Bind to the system `libevdi
` C library from Rust.
  -
 Do not use
 the deprecated `evdi
_open_attached_to`; use `evdi_open_attached_to_fixed
` because the installed header explicitly marks the
 old API as deprecated/problematic.
- Runtime
 model:
  - The
 service
 is allowed to run as a system-level privileged service because dynamic
 `evdi` DRM node creation requires elevated permissions.
  -
 Remove prior
 documentation assumptions that the MVP must rely on `systemd --user` or direct interaction with the current user’s Wayland session.
  - Keep lazy creation semantics: no virtual display before a client session is accepted.
- Stable identity
 model
 for Phase 1:
  -
 Treat stable naming as a mapping-layer responsibility
, not as a guarantee that kernel `cardX` indices remain
 fixed.
  - Preserve the existing
 `logical
_name` abstraction and make the backend consume it.
  - Encode stable,
 repeatable display identity into generated EDID metadata
 so desktop environments have a consistent
 monitor-facing identity to match against when possible.
- Capture model:
  - Use `evdi_register_buffer`, `ev
di_request_update`,
 `evdi_handle_events`, and `evdi_grab_pixels` to implement the minimal
 framebuffer capture path.
  - Start with a single-buffer synchronous capture path that is simple and diagnosable.
  - Prefer correctness and observability over throughput in Phase 1.

## Planned Module Changes

### `crates/display-backend`

- Add an internal FFI layer for the subset of `libevdi` APIs needed in Phase 1.
- Add an `EvdiDisplayBackend` implementation of `DisplayBackend`.
- Add an `EvdiDisplayHandle` implementation of `DisplayHandle`.
- Extend probe reporting to include:
  - detected `libevdi` version
  - whether the `evdi` kernel module appears available
  - whether create/destroy is expected to work in the current environment
  - required privilege / setup notes
- Generate a minimal EDID blob from `VirtualDisplaySpec`:
  - preferred mode based on width / height / refresh
  - stable monitor name derived from `logical_name`
  - stable serial derived from the same logical identity
- On create:
  - open an `evdi` handle with `evdi_open_attached_to_fixed`
  - connect the virtual monitor with generated EDID
  - store backend metadata needed by capture and destroy
- On destroy:
  - disconnect cleanly
  - close the `evdi` handle
  - make repeated destroy paths idempotent at the Rust boundary where practical

### `crates/capture`

- Keep the public `CaptureSource` trait unchanged.
- Add a concrete `evdi` capture source implementation consumed through `display-backend`.
- Track current mode, framebuffer dimensions, stride, dirty rects, and last successful frame state.
- Return `RawFrame` in a single agreed Phase 1 pixel format, with explicit documentation of what format `evdi` currently produces and what assumptions are made.
- If no valid mode has arrived yet, fail with a clear, diagnosable error instead of silently fabricating a frame.

### `crates/app-cli`

- Replace placeholder `doctor` / `probe` behavior with Phase 1-relevant output for the display backend.
- Add or adapt a validation-oriented CLI path that can:
  - print backend probe results
  - create an `evdi` output with a chosen logical name and mode
  - wait for mode readiness
  - attempt at least one frame capture
  - report success / failure and clean up
- Keep command output operator-friendly and suitable for later structured logging integration.

## Validation Plan

### Automated validation

- Add unit tests for:
  - EDID generation checksum correctness
  - EDID monitor name / serial stability for the same logical input
  - probe result formatting / interpretation helpers
  - failure-path handling for unavailable mode / empty capture preconditions
- Add tests that do not require real `evdi` hardware or root privileges wherever possible.
- Do not add fake integration tests that pretend a real kernel-backed `evdi` device exists.

### Manual validation

Record the following in the Phase 1 validation document:

1. Environment prerequisites
   - Arch Linux requires installing `evdi-dkms`
   - Arch Linux requires installing `linux-headers`
   - `/etc/modules-load.d/evdi.conf` must be created so the `evdi` module auto-loads on boot
2. Local environment evidence
   - installed `libevdi` header/API variant
   - detected `libevdi` version
   - detected `evdi` kernel module presence
3. Validation flow
   - run backend probe
   - create display
   - confirm virtual display appears
   - perform one or more captures
   - destroy display
   - repeat create/destroy with the same logical name
4. Observed results
   - whether create/destroy succeeded repeatedly
   - whether non-empty frame bytes were captured
   - whether the same logical identity was reused successfully
5. Known limitations
   - privileged execution requirement
   - compositor / desktop-environment behavior still to be expanded into a support matrix
   - Phase 1 validates backend viability, not end-to-end streaming

## Documentation Updates Required In The Same Change

- `docs/prd.md`
  - replace `systemd --user` preference with system-level service language appropriate for privileged `evdi` management
  - document Arch Linux prerequisites for enabling `evdi`
  - update backend assumptions from generic Wayland-session interaction to kernel-module-backed virtual output management
- `docs/architecture.md`
  - update logging / service assumptions away from `journalctl --user` wording where necessary
  - clarify that Phase 1 selects `evdi` as the primary display backend
  - clarify stable identity is provided by mapping + EDID identity, not fixed `cardX` numbering
- `docs/implementation-roadmap.md`
  - update Phase 5 wording to refer to system-level service integration rather than `systemd --user`
  - keep execution order intact
- `docs/implementation-status.md`
  - move the project snapshot from `Phase 0 / done` to `Phase 1 / in_progress` or `done` depending on actual delivered code and validation evidence
  - update blockers, next steps, and change log to reflect the `evdi` decision and results

## Boundaries

- Do not implement Phase 2 transport, encoding, session orchestration, or Android rendering work in this module.
- Do not broaden the backend matrix before `evdi` is validated as the primary path.
- Do not claim universal Wayland compositor compatibility in Phase 1.
- Do not depend on unverified assumptions about desktop remembering monitor layout solely from DRM node numbering.

## Completion Criteria

Phase 1 for this module is complete only when all of the following are true:

- `EvdiDisplayBackend` and minimal capture integration compile successfully.
- Validation-oriented CLI flow can exercise real backend probing and create/destroy behavior.
- At least one real manual validation run is documented with observed results.
- Non-empty frame capture is demonstrated or, if blocked by environment behavior, the exact blocker and smallest manual reproduction are documented.
- All affected documentation and status files are updated to reflect the real repository state.
- Matching tests for the pure-Rust portions of the implementation are included.

## Self-Review

- This plan keeps Phase 1 tightly focused on the highest-risk path: proving `evdi` create/destroy plus capture viability before any streaming work.
- The plan intentionally treats stable naming as a layered identity problem, which is more realistic than assuming kernel device indices stay fixed.
- The plan avoids overcommitting to compositor support claims until validation records exist.
- The plan keeps privileged execution explicit, matching the operational reality of `evdi`.