## Phase 0 Plan

- Scope: define the Android native decoder plugin shell and the placeholder API surface expected by the architecture document.
- Deliverables: plugin-facing placeholder classes/files integrated into the Flutter app structure, but without real decoder behavior.
- Boundaries: no MediaCodec session management in Phase 0.
- Validation: `flutter analyze` and successful Android project generation.

## Self-Review

- The plan keeps native work intentionally thin so the project has the right structure without claiming decoder support before Phase 2.
- API placeholders now reduce later renaming churn between Flutter and Android code.
