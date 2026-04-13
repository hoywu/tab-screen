# AGENTS.md

## Start Here
- Before any coding or planning, read `docs/architecture.md`, `docs/implementation-roadmap.md`, `docs/implementation-status.md`, and `docs/prd.md`.
- Treat `docs/implementation-status.md` as the handoff source of truth for current progress, blockers, and the next recommended task.

## Repo Reality
- The repository currently contains only `docs/`; no Rust workspace, Flutter app, CI, or task runner config exists yet.
- Do not invent build, test, lint, or run commands until those files are added and verified in the repo.

## Execution Order
- Follow the roadmap order in `docs/implementation-roadmap.md`: validate the display backend first, then the fixed-parameter streaming loop, then negotiation/UI/productization.
- Do not start complex UI, HEVC, USB-specific protocol work, or other enhancements before the display backend path is proven.

## Required Updates
- After finishing each atomic module, update `docs/implementation-status.md` in the same change.
- In that status update, keep `当前快照`, `模块状态总表`, `已完成内容`, `关键阻塞与风险`, `下一步建议`, and `变更记录` in sync with the actual repo state.
- If implementation diverges from `docs/architecture.md` or `docs/implementation-roadmap.md`, record the reason explicitly in `docs/implementation-status.md`.

## Commits
- After finishing each atomic module, create a git commit before starting the next module.
- Commit messages must follow Conventional Commits 1.0.0: https://www.conventionalcommits.org/en/v1.0.0/

## Current Baseline Constraints
- MVP target: Rust server + Flutter Android client.
- Transport baseline: one WebSocket connection, JSON text control frames, binary media frames.
- Session model: one active secondary-screen session at a time.
- Display parameters and stream parameters are separate decisions; do not merge them into one model.
- Stable client identity and persistent display-name mapping must exist before display creation logic is treated as complete.
