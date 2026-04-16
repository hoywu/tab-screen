## Phase 1 Plan

- Scope: replace the Phase 0 placeholders for `doctor` and `probe` with real Phase 1 validation entrypoints for the selected `evdi + libevdi` backend.
- Deliverables:
  - `doctor` outputs environment checks for the Phase 1 display-backend path.
  - `probe` outputs the selected display backend probe result instead of placeholder text.
  - add a dedicated validation subcommand for the independent “create display -> wait -> destroy display
” workflow required by the roadmap.
  - document the command behavior and validation expectations in code comments and repo
 docs.
- Boundaries:
  - no long
-running server runtime changes beyond wiring the validation commands.
  - no Phase 2 transport/session streaming logic.
  - no Phase 5 USB `adb reverse` execution yet.
  - no final service installation automation
 yet.
- Command responsibilities:
  1. `doctor
`
     - verify `libevdi` is linkable at build/runtime through the Rust integration layer.
     - verify the
 `evdi` kernel module/device path is available enough for Phase 1 diagnostics.
     - surface Arch
 Linux prerequisites in human-readable output:
       - install
 `evdi-dkms`
       - install
 `linux-headers`
       - create `/etc/modules-load.d
/evdi.conf` to auto-load the module on boot
     - report permission expectations for dynamic EVDI node creation and explain that the backend is designed for a
 system-level root service model rather than `systemd --user`.

     - print clear failure reasons and next-step suggestions.
  2.
 `probe`
     - instantiate the real `evdi` display backend and print its probe result.

     - report whether create/destroy, capture path
, and
 stable logical naming strategy are supported.
     - include backend
 notes about dependency
 assumptions, known
 limits, and fallback expectations.
  3. validation subcommand
     - execute the
 smallest practical Phase 1 lifecycle check:
       - create
 an EVDI-backed virtual display with a supplied logical display name

       - wait for a bounded observation period
       - optionally attempt minimal capture/update polling

       - destroy the display cleanly

     - allow repeated
 runs with the same logical name to support stable-naming validation.
     - return non-zero on any failed step so the command can serve as a validation harness.


## Planned CLI Shape

- Keep existing top-level commands and extend them in a Phase 1-compatible way.

- Proposed
 shape:
  - `tab-screen doctor`
  - `tab-screen probe`
  - `tab-screen probe display`
  -
 `tab-screen probe display
-validate --display-name
 <name> [--width
 <px>] [--height <px>] [--refresh
-rate <hz>] [--wait-ms <ms>] [
--capture]`
- If implementation pressure requires fewer entrypoints, prefer preserving `doctor` and `probe` while adding exactly one explicit validation subcommand.

## Output Contract

- Human-readable by default because Phase 1 is focused on manual backend validation.
- Keep output structured and stable enough that Phase 5 can later add machine-readable modes without redesigning the command surface.
- Each command should clearly separate:
  - summary
  - checks performed
  - pass/fail result per check
  - actionable remediation steps

## Validation Plan

- Automated validation:
  - Rust unit tests for CLI argument parsing and command dispatch.
  - Rust unit tests for formatting/report rendering where practical.
- Manual validation:
  - run `cargo run -p tab-screen -- doctor`
  - run `cargo run -p tab-screen -- probe`
  - run the Phase 1 validation command twice with the same logical display name and confirm both runs complete cleanly
  - record observed environment facts and command output in repository docs
- If framebuffer capture cannot be reliably automated in CI or local tests, record that limitation explicitly and require the smallest practical manual validation evidence.

## Self-Review

- This plan keeps Phase 1 focused on proving the display backend path rather than prematurely growing runtime features.
- Adding an explicit validation command satisfies the roadmap requirement for an independent test program while reusing the existing CLI crate.
- The plan intentionally shifts operational guidance away from `systemd --user` and toward a system-level privileged service model, matching the `evdi` kernel-module-based architecture now chosen.
- Human-readable diagnostics are prioritized now, but the command structure leaves room for later JSON output in Phase 5 without needing to break the CLI surface.