## Phase 0 Plan

- Scope: create the Flutter Android application with route skeletons, base theme, controller placeholders, repositories for stable ID and last server storage, and a native plugin shell directory.
- Deliverables: a Flutter app that launches, shows placeholder pages for the five required screens, and exposes controller/repository placeholders under `lib/`.
- Boundaries: no live transport, no MediaCodec implementation, and no completed settings or diagnostics flows yet.
- Validation: `flutter analyze` and `flutter test` for basic widget/repository coverage once the app is scaffolded.

## Self-Review

- The plan delivers the exact Phase 0 client baseline from the roadmap without jumping ahead into playback or negotiation logic.
- Including repository/controller placeholders now keeps the app structure aligned with the architecture document from the start.
