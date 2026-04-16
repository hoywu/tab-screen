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

## 当前快照

- 当前阶段: `Phase 1 / done`
- 总体状态: Phase 0 已完成，Phase 1 已基于 `evdi + libevdi` 路径完成主显示后端与最小捕获验证；`doctor`、`probe` 与独立显示验证命令已落地，并在本机 Arch Linux + `niri` 环境完成两次同名创建/销毁与非空抓帧实测
- 最新可用文档: `docs/prd.md`、`docs/architecture.md`、`docs/implementation-roadmap.md`、`docs/implementation-status.md`、`docs/phase1-evdi-validation.md`、`AGENTS.md`
- 建议下一步: 进入 `Phase 2`，围绕已验证的 `evdi` 后端实现固定参数最小闭环：WebSocket 传输、最小会话状态机、固定参数捕获到 H.264 编码链路，以及 Android 端最小 `MediaCodec` 解码显示

## 模块状态总表

| 模块 | 状态 | 负责人 | 说明 | 下一步 |
| --- | --- | --- | --- | --- |
| 文档基线 `prd` | done | 已完成 | 产品需求已定义；Phase 1 后需与 `evdi` 系统级特权服务模型保持同步 | 后续作为验收依据继续维护 |
| 文档基线 `architecture` | done | 已完成 | MVP 架构、接口、协议和状态机已定义；Phase 1 已确认 `evdi` 为当前主显示后端路径 | Phase 2 继续按此基线实现 |
| 文档基线 `implementation-roadmap` | done | 已完成 | 阶段、门槛和任务顺序已定义；Phase 1 门槛已满足 | 转入 Phase 2 固定参数闭环 |
| 仓库级 `AGENTS.md` | done | OpenCode | 已落地仓库级执行约束，明确先读 `docs/`、及时更新状态文档、每个原子模块完成后提交 Conventional Commit，并要求编码时同步编写必要测试 | 后续实现严格遵守 |
| Rust workspace | done | OpenCode | 已创建根 `Cargo.toml`、工作区成员列表、统一依赖版本，以及 `crates/` 模块目录骨架 | 继续承载后续实现 |
| `crates/protocol` | done | OpenCode | 已定义控制消息、错误码、协商基础类型、媒体包头模型，并用单元测试验证关键序列化形态 | 后续按 Phase 2/3 扩展协议字段 |
| `crates/config` | done | OpenCode | 已定义 raw/normalized/effective 三层配置骨架、默认值、规范化与校验入口，并用单元测试验证默认配置与 TOML 输出 | 后续补 CLI/ENV/file 合并逻辑 |
| `crates/server-core` | done | OpenCode | 已建立会话状态枚举、`SessionManager` 占位、协议版本校验与统一服务端错误类型 | Phase 2 接入真实状态机与会话编排 |
| `crates/display-backend` | done | OpenCode | 已从 no-op 占位扩展到 `evdi` 真实后端，支持 probe、创建/销毁、EDID 生成、稳定逻辑名派生，以及通过显示句柄获取最小捕获源 | Phase 2 对接真实会话编排与编码链路 |
| `crates/capture` | done | OpenCode | 已补齐 packed frame size 计算与验证辅助逻辑，并由 `display-backend` 中的 `evdi` 捕获源驱动最小非空抓帧 | Phase 2 对接编码器输入与连续帧循环 |
| `crates/encoder` | done | OpenCode | 已定义编码器 probe/config/reconfigure/encoded frame 模型与 `EncoderBackend` 抽象 | Phase 2 对接真实 H.264 编码路径 |
| `crates/transport` | done | OpenCode | 已定义会话路径、心跳默认值、传输配置与服务端占位结构 | Phase 2 实现 WebSocket 监听与帧分流 |
| `crates/app-cli` | done | OpenCode | 已将 `doctor`/`probe` 从占位升级为 Phase 1 可用实现，并新增显示验证子命令，可执行 `probe -> create -> capture -> destroy` 独立验证 | Phase 2 接入最小 `serve` 真实运行逻辑 |
| `apps/android` Flutter 壳 | done | OpenCode | 已用 `flutter create` 建立 Android App，补齐 5 个页面路由、3 个控制器、偏好存储仓储、基础主题和最小 widget/仓储测试 | Phase 2 接入传输与解码链路 |
| Android 原生解码插件 | done | OpenCode | 已在 Android `MainActivity` 与 Dart 平台层中建立原生解码方法通道壳，暴露后续 `MediaCodec` 所需 API 占位 | Phase 2 实现真实解码与渲染 |
| 系统级服务集成 | not_started | 待定 | Phase 1 已确认服务模型应偏向系统级特权服务而非 `systemd --user`；示例 unit 与安装说明尚未落地 | Phase 5 提供系统级 service unit 与部署文档 |
| `doctor/probe` | done | OpenCode | 已实现 `evdi` 环境检查、后端探测与独立显示验证命令，并完成本机实测 | Phase 5 再扩展编码器、ADB、端口等检查项 |
| USB `adb reverse` | not_started | 待定 | USB 接入未落地 | LAN 跑通后复用协议接入 |

## 已完成内容

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
13. 将 `crates/display-backend` 的 Phase 0 no-op 后端扩展为 Phase 1 `evdi` 后端，完成 `libevdi` FFI 接入、`evdi_open_attached_to_fixed` 调用、最小 EDID 生成、稳定逻辑名派生和 `DisplayHandle` 实现。
14. 在 `crates/capture` 中补齐 packed framebuffer 大小计算、像素格式辅助逻辑及对应单元测试，为 `evdi` 抓帧路径提供基础验证能力。
15. 在 `crates/app-cli` 中实现 Phase 1 诊断与验证命令：`doctor` 输出 `evdi` 环境检查与 Arch Linux 前置步骤；`probe` 输出后端能力；`probe validate-display` 执行独立的“创建显示器 -> 等待 -> 抓一帧 -> 销毁显示器”验证流程。
16. 为 `display-backend` 增加纯 Rust 测试，覆盖 EDID 校验和、同一逻辑名生成结果稳定性、逻辑名变化导致身份字段变化，以及 `evdi` 32-bit 模式到 `RawFrameFormat` 的转换。
17. 完成 Phase 1 自动化验证：`cargo test -p capture -p display-backend -p tab-screen` 全通过；`cargo check` 全工作区通过。
18. 完成本机 Phase 1 实机验证，环境为 Arch Linux、内核 `6.19.11-arch1-1`、会话类型 `wayland`、桌面/合成器 `niri`、`evdi-dkms 1.14.15-1`、`linux-headers 6.19.11.arch1-1`。
19. 本机 `doctor` 实测通过，确认 `libevdi` 可用、`/sys/module/evdi` 存在、`/dev/dri` 存在、`/etc/modules-load.d/evdi.conf` 已配置、特权执行路径可用。
20. 本机 `probe` 实测通过，确认当前主后端为 `evdi`，支持稳定命名语义和创建/销毁路径。
21. 本机两次执行同名显示验证命令均通过，逻辑显示名均为 `Tab Screen Validation`，后端 ID 均为 `evdi:4C410403`，均成功抓取到 `1920x1200`、`stride=7680`、`9216000` 字节的非空帧，且两次都能正确销毁显示器。
22. 已新增 Phase 1 验证记录文档 `docs/phase1-evdi-validation.md`，作为当前后端可行性与实测结果的仓库内真相源。

## 当前未开始但已确定的实现基线

- 目录约定: `crates/` 放 Rust 子 crate，`apps/android/` 放 Flutter 客户端。
- 协议基线: WebSocket 文本 JSON 控制消息 + 二进制媒体帧。
- 服务端配置优先级: `CLI > ENV > file > default`。
- 会话模型: 单活动副屏会话。
- 身份模型: 客户端本地稳定 ID + 服务端持久化显示名映射。
- 主显示后端基线: `evdi + libevdi`。
- 服务运行模型基线: 优先系统级特权服务，而不是 `systemd --user`。
- 稳定显示身份策略基线: 逻辑名映射层 + EDID 中的稳定显示身份字段，而不是依赖固定 `cardX` 编号。
- Arch Linux Phase 1 前置步骤基线:
  - 安装 `evdi-dkms`
  - 安装 `linux-headers`
  - 创建 `/etc/modules-load.d/evdi.conf` 以开机自动加载 `evdi`

## 关键阻塞与风险

Phase 1 的核心风险已从“后端是否可行”转为“如何把已验证路径产品化”。

1. `evdi` 后端已在当前 Arch Linux + `niri` 环境完成创建/销毁和最小抓帧验证，但尚未形成更广泛的 compositor 支持矩阵。
2. 当前 Phase 1 的抓帧实现是正确性优先的单缓冲、同步式最小路径，尚未进入连续帧循环、性能优化、热重配置或稳态 streaming。
3. 稳定命名当前通过逻辑名与 EDID 稳定字段实现语义稳定，尚未验证在桌面环境重启、服务重启、完整客户端身份映射持久化落地后的长期表现。
4. Rust 编码链路到 Android `MediaCodec` 的 Annex B 兼容性仍未实测，仍是 Phase 2 的关键风险。
5. `doctor/probe` 目前主要覆盖显示后端可行性，尚未扩展到编码器、ADB、监听端口、配置解析结果等 Phase 5 范围。
6. 仓库文档已切换到系统级特权服务口径；后续若新增部署、状态目录或诊断相关内容，必须继续保持与该基线一致，避免再次引入 `systemd --user` 或用户态状态目录作为默认前提。
7. Flutter 当前仅完成工程壳和占位逻辑，稳定 ID 已落地到本地存储，但真实传输、解码和全屏渲染仍未实现，不应误判为可用客户端。

## 推荐接手顺序

新 Agent 接手时，优先按以下顺序工作。

1. 先读 `docs/phase1-evdi-validation.md`，确认 Phase 1 已验证的环境事实、命令、观察结果和限制。
2. 再读 `docs/architecture.md` 的 `MVP 具体技术选择`、`核心抽象接口`、`控制协议架构`、`配置与持久化路径`。
3. 读取 `docs/implementation-roadmap.md` 的 `Phase 2`，按固定参数最小闭环推进。
4. 在编码前先补齐目标模块目录下的 `IMPLEMENTATION_PLAN.md`，并确保计划与交付代码同步。
5. 不要回头扩展复杂 UI、HEVC、多后端矩阵或 USB 独立协议，优先把固定参数 LAN 闭环打通。

## 下一步建议

最推荐的直接起点是以下 5 个任务。

1. 在 `crates/transport` 中实现 WebSocket 监听、文本/二进制帧分流和最小心跳。
2. 在 `crates/server-core` 中接入最小会话状态机，形成 `Idle -> Handshaking -> Streaming -> Terminating` 的真实路径。
3. 在 `crates/encoder` 中实现固定参数 H.264 编码路径，并把 `evdi` 抓帧结果接入编码器。
4. 在 `apps/android` 中实现最小 WebSocket 客户端和 `MediaCodec` H.264 解码显示。
5. 在推进 Phase 2 的同一改动中同步补齐测试或人工验证记录，并继续维护 `docs/implementation-status.md` 与对应模块计划文件。

## 模块完成后必须更新的字段

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

## 变更记录

| 日期 | 变更人 | 内容 |
| --- | --- | --- |
| 2026-04-13 | OpenCode | 初始化实现状态文档，记录当前仍处于文档完成、代码未启动状态 |
| 2026-04-14 | OpenCode | 新增根级 `AGENTS.md`，并同步记录后续 Agent 必须先读 `docs/`、及时更新状态文档、每个原子模块完成后提交 Conventional Commit 的仓库约束 |
| 2026-04-14 | OpenCode | 明确 `AGENTS.md` 中 Conventional Commits 1.0.0 的提交格式与关键规则，移除外链依赖，确保仓库内可直接查阅提交规范 |
| 2026-04-14 | OpenCode | 强化仓库级与核心实现文档中的测试要求，明确编码时必须同步编写必要测试或留下验证记录，缺少验证结果的改动不视为完成 |
| 2026-04-14 | OpenCode | 开始 Phase 0 落地：创建 `crates/` 与 `apps/android/` 模块目录、各模块 `IMPLEMENTATION_PLAN.md`，并建立 Rust workspace 根配置 |
| 2026-04-14 | OpenCode | 完成 Rust Phase 0 骨架：落地协议、配置、核心接口和 CLI 占位实现，并记录 `cargo test`、`cargo check` 与 CLI 帮助输出验证结果 |
| 2026-04-14 | OpenCode | 完成 Flutter Android Phase 0 骨架：用 `flutter create` 建立 App，补齐页面/控制器/偏好存储/原生插件壳，并记录 `flutter analyze` 与 `flutter test` 验证结果 |
| 2026-04-16 | OpenCode | 启动 Phase 1：将 `display-backend`、`capture`、`app-cli` 的模块计划更新为 `evdi + libevdi` 主后端与最小捕获验证方案 |
| 2026-04-16 | OpenCode | 完成 `evdi` Phase 1 代码落地：新增 `evdi` 后端、EDID 生成、最小抓帧路径，以及 `doctor`、`probe`、`probe validate-display` 命令 |
| 2026-04-16 | OpenCode | 完成 Phase 1 自动化验证：`cargo test -p capture -p display-backend -p tab-screen` 通过，`cargo check` 通过 |
| 2026-04-16 | OpenCode | 完成 Phase 1 本机实测：在 Arch Linux + `niri` 环境两次执行同名 `evdi` 创建/销毁与非空抓帧验证均成功，确认当前主显示后端路径可进入 Phase 2 |

## 更新模板

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
