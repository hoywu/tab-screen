# Tab Screen Implementation Status

**文档信息**

- 项目代号: `tab-screen`
- 文档类型: Implementation Status / Handoff
- 文档目标: 让新的 Agent 在中途接手时，快速理解当前实现进度、已完成内容、阻塞点和下一步入口
- 更新规则: 每完成一个完整模块后必须更新本文件
- 当前版本: `v1.0`

**使用规则**

- 本文件是“当前实现真相源”，关注的是已经落地的实现状态，而不是计划。
- 每次完成一个完整模块编码后，必须同步更新以下部分。
- 如果实现与 `docs/architecture.md` 或 `docs/implementation-roadmap.md` 有偏离，必须在本文件明确记录原因。
- 每次模块完成时，必须同时记录与该模块相匹配的测试或验证结果；没有测试或验证结果，不应标记为完成。
- 如果只完成了一半，不要把模块标记为 `done`，而应标记为 `in_progress` 并写清剩余工作。

模块状态约定。

- `not_started`: 未开始
- `in_progress`: 已开工但未形成可交付闭环
- `blocked`: 因外部依赖或技术问题阻塞
- `done`: 已完成并具备对应测试或验证结果

**当前快照**

- 当前阶段: `Phase 0 / done`
- 总体状态: Phase 0 已完成；Rust workspace 与各 crate 骨架、Flutter Android App 骨架、控制器/存储占位、Android 原生插件壳、CLI 命令树以及最小测试入口均已落地并完成验证
- 最新可用文档: `docs/prd.md`、`docs/architecture.md`、`docs/implementation-roadmap.md`、`docs/implementation-status.md`、`AGENTS.md`
- 建议下一步: 从 `Phase 0` 开始搭建 Rust workspace、Flutter App 骨架和协议/配置 crate，并在每个原子模块完成时同步补齐必要测试、更新本文件与提交 Conventional Commit

**模块状态总表**

| 模块 | 状态 | 负责人 | 说明 | 下一步 |
| --- | --- | --- | --- | --- |
| 文档基线 `prd` | done | 已完成 | 产品需求已定义 | 作为验收依据保留 |
| 文档基线 `architecture` | done | 已完成 | MVP 架构、接口、协议和状态机已定义 | 编码时按此为默认基线 |
| 文档基线 `implementation-roadmap` | done | 已完成 | 阶段、门槛和任务顺序已定义 | 按阶段推进 |
| 仓库级 `AGENTS.md` | done | OpenCode | 已落地仓库级执行约束，明确先读 `docs/`、及时更新状态文档、每个原子模块完成后提交 Conventional Commit，并要求编码时同步编写必要测试 | 后续实现严格遵守 |
| Rust workspace | done | OpenCode | 已创建根 `Cargo.toml`、工作区成员列表、统一依赖版本，以及 `crates/` 模块目录骨架 | 开始实现各 crate 代码骨架 |
| `crates/protocol` | done | OpenCode | 已定义控制消息、错误码、协商基础类型、媒体包头模型，并用单元测试验证关键序列化形态 | 后续按 Phase 2/3 扩展协议字段 |
| `crates/config` | done | OpenCode | 已定义 raw/normalized/effective 三层配置骨架、默认值、规范化与校验入口，并用单元测试验证默认配置与 TOML 输出 | 后续补 CLI/ENV/file 合并逻辑 |
| `crates/server-core` | done | OpenCode | 已建立会话状态枚举、`SessionManager` 占位、协议版本校验与统一服务端错误类型 | 后续在 Phase 2 接入真实状态机与会话编排 |
| `crates/display-backend` | done | OpenCode | 已定义 `DisplayBackend`/`DisplayHandle` 抽象、probe 结果结构和 no-op 占位后端 | Phase 1 进入真实后端验证与实现 |
| `crates/capture` | done | OpenCode | 已定义原始帧格式、像素格式、帧结构与 `CaptureSource` 抽象 | Phase 1 对接真实捕获链路 |
| `crates/encoder` | done | OpenCode | 已定义编码器 probe/config/reconfigure/encoded frame 模型与 `EncoderBackend` 抽象 | Phase 1/2 对接真实 H.264 编码路径 |
| `crates/transport` | done | OpenCode | 已定义会话路径、心跳默认值、传输配置与服务端占位结构 | Phase 2 实现 WebSocket 监听与帧分流 |
| `crates/app-cli` | done | OpenCode | 已实现 `serve/doctor/probe/print-default-config/usb/version` 的 `clap` 命令树与 tracing 初始化，占位命令可运行 | Phase 1/5 补充真实逻辑 |
| `apps/android` Flutter 壳 | done | OpenCode | 已用 `flutter create` 建立 Android App，补齐 5 个页面路由、3 个控制器、偏好存储仓储、基础主题和最小 widget/仓储测试 | Phase 2 接入传输与解码链路 |
| Android 原生解码插件 | done | OpenCode | 已在 Android `MainActivity` 与 Dart 平台层中建立原生解码方法通道壳，暴露后续 `MediaCodec` 所需 API 占位 | Phase 2 实现真实解码与渲染 |
| `systemd --user` 集成 | not_started | 待定 | unit 和说明未落地 | Phase 5 完成 |
| `doctor/probe` | not_started | 待定 | 诊断命令未落地 | 依赖后端 probe 结果 |
| USB `adb reverse` | not_started | 待定 | USB 接入未落地 | LAN 跑通后复用协议接入 |

**已完成内容**

1. 完成 `docs/prd.md` 的产品需求定义。
2. 完成 `docs/architecture.md` 的系统架构设计。
3. 完成 `docs/implementation-roadmap.md` 的分阶段路线图。
4. 明确了 MVP 默认技术路线，包括 Rust workspace + Flutter Android、单会话模型、WebSocket 单连接承载控制与媒体、`H.264` 为 MVP 必达媒体格式，以及显示参数与串流参数两阶段决策。
5. 新增仓库根 `AGENTS.md`，将后续 Agent 的起手阅读顺序、阶段推进顺序、状态文档维护要求和提交规范固定为仓库约束。
6. 在仓库级执行约束与核心实现文档中明确：编码时必须同步编写必要测试或留下验证记录，没有测试或验证结果的改动不视为完成。
7. 按模块创建了 `crates/` 与 `apps/android/` 目录，并为 `protocol/config/server-core/display-backend/capture/encoder/transport/app-cli/android/native` 落地 `IMPLEMENTATION_PLAN.md`，满足“先写模块计划再编码”的仓库约束。
8. 创建了 Rust workspace 根 `Cargo.toml`，统一声明工作区成员与 Phase 0 需要的核心依赖版本。
9. 完成 Rust Phase 0 骨架：`protocol` 定义控制消息与错误码，`config` 定义三层配置模型与默认值，`server-core/display-backend/capture/encoder/transport` 定义核心抽象接口，`app-cli` 建立命令树与日志初始化。
10. 新增与 Rust 骨架匹配的自动化验证：`cargo test -p protocol -p config -p server-core` 全通过，`cargo check` 全工作区通过，`cargo run -p tab-screen -- --help` 能输出命令帮助。
11. 使用 `flutter create` 建立 `apps/android` 工程，补齐 `Home/Connect/Fullscreen/Settings/Diagnostics` 五个页面占位、`SessionController/SettingsController/DiagnosticsController`、`PreferencesRepository` 以及原生解码插件 Dart/Kotlin 壳。
12. 完成 Flutter Phase 0 验证：`flutter analyze` 无问题，`flutter test` 通过，应用启动后的首页和本地偏好存储路径已有最小自动化覆盖。

**当前未开始但已确定的实现基线**

- 目录约定: `crates/` 放 Rust 子 crate，`apps/android/` 放 Flutter 客户端。
- 协议基线: WebSocket 文本 JSON 控制消息 + 二进制媒体帧。
- 服务端配置优先级: `CLI > ENV > file > default`。
- 会话模型: 单活动副屏会话。
- 身份模型: 客户端本地稳定 ID + 服务端持久化显示名映射。

**关键阻塞与风险**

当前最大的阻塞不是代码，而是技术验证尚未开始。

1. 主 `DisplayBackend` 仍未验证。
2. Wayland 虚拟显示器是否支持按连接创建/销毁仍未实测。
3. 稳定命名能力是否由后端原生提供仍未确认。
4. Rust 编码链路到 Android `MediaCodec` 的 Annex B 兼容性仍未实测。
5. Phase 1 的核心风险仍未变化：显示后端可行性、稳定命名与捕获链路尚未验证，因此虽然 Phase 0 已完成，下一步仍必须立即转向后端验证，不能提前扩展 UI 或媒体功能。
6. Flutter 当前仅完成工程壳和占位逻辑，稳定 ID 已落地到本地存储，但真实传输、解码和全屏渲染仍未实现，不应误判为可用客户端。

只要上述任一项存疑，就不应跳到复杂 UI 或增强特性。

**推荐接手顺序**

新 Agent 接手时，优先按以下顺序工作。

1. 读取 `docs/architecture.md` 的 `MVP 具体技术选择`、`核心抽象接口`、`控制协议架构`。
2. 读取 `docs/implementation-roadmap.md` 的 `Phase 0` 和 `Phase 1`。
3. 搭建 Rust workspace 与 Flutter 壳。
4. 立即进入显示后端验证，而不是先做客户端 UI 细节。

**下一步建议**

最推荐的直接起点是以下 3 个任务。

1. 进入 Phase 1，在 `crates/display-backend` 中实现至少一个候选后端的 `probe` 与独立验证命令，优先验证创建/销毁和稳定命名能力。
2. 在 `crates/capture` 中接入与该显示后端配套的最小帧获取路径，并记录验证结果文档。
3. 每完成一个原子模块，同步更新本文件并创建一条符合 Conventional Commits 1.0.0 的提交。

**模块完成后必须更新的字段**

每次完成一个完整模块后，至少更新以下内容。

- `当前快照`
- `模块状态总表`
- `已完成内容`
- `关键阻塞与风险`
- `下一步建议`
- `变更记录`

完整模块的判断标准。

- 有代码落地。
- 有与改动匹配的必要测试或最小验证结果。
- 测试结果或人工验证结论已记录。
- 有明确边界，不是零散改动。

**变更记录**

| 日期 | 变更人 | 内容 |
| --- | --- | --- |
| 2026-04-13 | OpenCode | 初始化实现状态文档，记录当前仍处于文档完成、代码未启动状态 |
| 2026-04-14 | OpenCode | 新增根级 `AGENTS.md`，并同步记录后续 Agent 必须先读 `docs/`、及时更新状态文档、每个原子模块完成后提交 Conventional Commit 的仓库约束 |
| 2026-04-14 | OpenCode | 明确 `AGENTS.md` 中 Conventional Commits 1.0.0 的提交格式与关键规则，移除外链依赖，确保仓库内可直接查阅提交规范 |
| 2026-04-14 | OpenCode | 强化仓库级与核心实现文档中的测试要求，明确编码时必须同步编写必要测试或留下验证记录，缺少验证结果的改动不视为完成 |
| 2026-04-14 | OpenCode | 开始 Phase 0 落地：创建 `crates/` 与 `apps/android/` 模块目录、各模块 `IMPLEMENTATION_PLAN.md`，并建立 Rust workspace 根配置 |
| 2026-04-14 | OpenCode | 完成 Rust Phase 0 骨架：落地协议、配置、核心接口和 CLI 占位实现，并记录 `cargo test`、`cargo check` 与 CLI 帮助输出验证结果 |
| 2026-04-14 | OpenCode | 完成 Flutter Android Phase 0 骨架：用 `flutter create` 建立 App，补齐页面/控制器/偏好存储/原生插件壳，并记录 `flutter analyze` 与 `flutter test` 验证结果 |

**更新模板**

后续更新可直接参考以下模板替换对应内容。

```md
## 当前快照

- 当前阶段: `Phase X`
- 总体状态: `一句话总结`
- 建议下一步: `下一位 Agent 最应该做的事`

## 模块状态总表

| 模块 | 状态 | 负责人 | 说明 | 下一步 |
| --- | --- | --- | --- | --- |

## 已完成内容

1. ...

## 关键阻塞与风险

1. ...

## 下一步建议

1. ...

## 变更记录

| 日期 | 变更人 | 内容 |
| --- | --- | --- |
```
