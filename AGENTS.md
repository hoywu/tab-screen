# AGENTS.md

## Start Here
- Before any coding or planning, read `docs/architecture.md`, `docs/implementation-roadmap.md`, `docs/implementation-status.md`, and `docs/prd.md`.
- Treat `docs/implementation-status.md` as the handoff source of truth for current progress, blockers, and the next recommended task.
- Treat the repository documentation as the source of truth for the project. Documentation must reflect the real current state of the codebase at all times.

## Repo Reality
- The repository currently contains only `docs/`; no Rust workspace, Flutter app, CI, or task runner config exists yet.
- Do not invent build, test, lint, or run commands until those files are added and verified in the repo.

## Execution Order
- Follow the roadmap order in `docs/implementation-roadmap.md`: validate the display backend first, then the fixed-parameter streaming loop, then negotiation/UI/productization.
- Do not start complex UI, HEVC, USB-specific protocol work, or other enhancements before the display backend path is proven.
- Before implementing each module, write the complete implementation plan into that module's corresponding directory first, then self-review the plan for correctness before starting code.
- If a module directory does not exist yet, create the directory as part of that module's initial setup and place the implementation plan there before writing code.

## Required Updates
- After finishing each atomic module, update `docs/implementation-status.md` in the same change.
- In that status update, keep `当前快照`, `模块状态总表`, `已完成内容`, `关键阻塞与风险`, `下一步建议`, and `变更记录` in sync with the actual repo state.
- If implementation diverges from `docs/architecture.md` or `docs/implementation-roadmap.md`, record the reason explicitly in `docs/implementation-status.md`.
- The module implementation plan stored in the module directory must be kept in sync with the actual delivered code; if the plan changes during implementation, update it in the same change.
- If coding reveals that the current plan or other repository documentation must change, update the relevant documentation first so it reflects the new intended reality, then continue implementation.

## Commits
- After finishing each atomic module, create a git commit before starting the next module.
- Commit messages must follow Conventional Commits 1.0.0. The commit message format is:

  ```text
  <type>[optional scope]: <description>

  [optional body]

  [optional footer(s)]
  ```

  Requirements:
  - `type` is required, followed by an optional scope, an optional `!`, and a required `: `.
  - Use `feat` when a commit adds a new feature.
  - Use `fix` when a commit fixes a bug.
  - A scope may be added in parentheses after the type, for example `feat(parser): add array parsing`.
  - The description must immediately follow the prefix and be a short summary of the change.
  - A longer body may be added after one blank line.
  - One or more footers may be added after one blank line following the body.
  - Breaking changes must be indicated either by adding `!` before `:` in the prefix, or by a footer starting with `BREAKING CHANGE:`.
  - Types other than `feat` and `fix` are allowed when appropriate, such as `docs`, `refactor`, `test`, `chore`, `ci`, `build`, `perf`, and similar project-appropriate types.
- Before ending any task that has been completed successfully, create a git commit for the finished work, verify the working tree is clean, and only then stop; this applies to all kinds of changes, including code and documentation updates.

## Current Baseline Constraints
- MVP target: Rust server + Flutter Android client.
- Transport baseline: one WebSocket connection, JSON text control frames, binary media frames.
- Session model: one active secondary-screen session at a time.
- Display parameters and stream parameters are separate decisions; do not merge them into one model.
- Stable client identity and persistent display-name mapping must exist before display creation logic is treated as complete.
- During coding, add concise comments at each key logic point so readers can quickly understand the role of each important code block; avoid verbose or line-by-line commentary.
