## Phase 1 Plan

- Scope:
 provide the minimum capture
-side implementation and validation plan required to prove the selected
 `evdi + libevdi` display backend can produce usable frame data for later encoding work.
-
 Goal: after a virtual display is created by the `display-backend` module, the capture layer must be able to attach to that backend-specific handle
, wait for mode/update events, request a framebuffer update, and return a non-empty
 `RawFrame`.

- Backend assumption: the current Phase 1 primary path is
 `evdi` on Linux, using the locally installed `libevdi` C API and the fixed open helper `evdi_open_attached_to_fixed` where applicable.

- Deliverables:
  - an `evdi`-backed `CaptureSource
` implementation that can expose `RawFrameFormat` and fetch at least one frame in
 `BGRA/RGBA 8-bit` style memory layout;

  - small capture-side helper types needed to translate `evdi_mode`, dirty rects, and framebuffer metadata into the crate’s Rust model;

  - tests for pure-Rust
 conversion logic that do not require a live `evdi
` device;
  -
 documented manual validation steps
 and observed results for live frame capture in the target Arch Linux environment.
- Boundaries:
  - no encoder integration in this phase;
  - no frame pacing, continuous streaming loop
, or performance tuning in this phase;
  - no cursor-separation or advanced
 dirty-rect optimization in this phase;
  - no attempt to support non-
`evdi` capture paths yet.

## Planned Work Items

1. Define the capture-side integration boundary with the `evdi` backend.
   - Keep the generic `CaptureSource` trait unchanged.

   - Add only the minimum backend-specific glue needed so a display handle created by the
 `display-backend` crate can vend a capture source safely.
   - Ensure ownership
 is explicit so the capture object cannot outlive the backend resources it depends on in an invalid state.

2. Implement framebuffer metadata translation.
   -
 Convert `evdi` mode information into `RawFrameFormat`.
   - Normalize width, height
, stride, and color depth into the crate’s existing types.
   - Start with one guaranteed packed 32-bit pixel format path for validation, and reject unsupported modes with clear errors.

3. Implement the minimum update flow.
   - Wait for the backend to report a valid mode before requesting frames.
   - Register a userspace buffer sized for the current mode.
   - Request an update from `evdi`.
   - Handle both immediate-ready and event-driven-ready paths.
   - Grab pixels and package them into `RawFrame`.
   - Treat an all-zero-length or structurally invalid frame as failure.

4. Handle mode changes conservatively.
   - If mode changes after buffer registration, recreate capture buffers rather than trying to hot-patch them.
   - Keep the Phase 1 logic simple and correctness-first so the validation path is easy to reason about.

5. Expose enough diagnostics for Phase 1 verification.
   - Record whether a mode was observed.
   - Record the effective width, height, stride, and bits per pixel used for capture.
   - Record whether the first successful frame had non-zero payload size and how many dirty rects were reported.
   - Return actionable errors when no mode or no update arrives within the validation timeout.

## Validation Strategy

### Automated validation

The following should be covered with normal unit tests:

- frame-buffer-size calculation for supported 32-bit packed pixel layouts;
- conversion from backend mode metadata into `RawFrameFormat`;
- error handling for zero/overflow/unsupported dimensions or stride values;
- dirty-rect normalization helpers, if such helpers are introduced.

These tests must avoid requiring a real kernel module or DRM node.

### Manual validation

Because live `evdi` capture depends on kernel-module state, DRM nodes, compositor behavior, and administrative privileges, the final Phase 1 proof must also include manual validation.

Planned validation sequence:

1. Ensure the Arch Linux host has:
   - `evdi-dkms` installed;
   - `linux-headers` installed;
   - `/etc/modules-load.d/evdi.conf` configured to auto-load `evdi`.
2. Confirm the `evdi` kernel module and `libevdi` userspace library are present.
3. Run the dedicated Phase 1 validation command that:
   - creates an `evdi` virtual display;
   - waits for the mode to become available;
   - obtains a capture source;
   - requests at least one framebuffer update;
   - reports the captured frame size and basic metadata;
   - destroys the display cleanly.
4. While the validation command is waiting for updates, place visible content on the virtual display so the compositor produces a real framebuffer update.
5. Record the observed result in repository documentation, including:
   - environment details;
   - whether creation/destroy worked repeatedly;
   - whether the same logical display name was reused across runs;
   - whether a non-empty frame was captured successfully;
   - known limitations or manual steps still required.

## Risks And Constraints

- `evdi` capture is event-driven, so a successful virtual display connection does not by itself guarantee an immediately capturable frame.
- The first usable frame may depend on compositor behavior and on whether any content has actually been rendered to the virtual output.
- Administrative privileges are commonly required for `evdi` device creation/opening.
- The installed header indicates `evdi_open_attached_to` should not be relied on directly; the integration must prefer `evdi_open_attached_to_fixed`.
- Stable user-visible display identity is expected to come from the display/backend layer via logical naming and EDID strategy, not from the capture crate alone.

## Completion Criteria

This module’s Phase 1 work is complete only when all of the following are true:

- the crate contains the minimal `evdi` capture implementation needed by the chosen backend path;
- pure-Rust helper logic has matching automated tests;
- a live validation run has captured at least one non-empty frame from an `evdi` virtual display;
- the corresponding validation evidence and limitations are recorded in the repository docs;
- this plan remains in sync with the delivered code and validation outcome.

## Self-Review

- This plan stays within the roadmap’s Phase 1 boundary: it proves capture viability without prematurely building the full streaming pipeline.
- The plan deliberately keeps the capture contract stable and pushes backend-specific complexity to the smallest practical surface.
- The plan treats live capture proof as a combination of code, automated tests for deterministic logic, and explicit manual validation evidence, which matches the repository rules.