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
- 如果只完成了一半，不要把模块标记为 `done`，而应标记为 `in_progress` 并写清剩余工作。

模块状态约定。

- `not_started`: 未开始
- `in_progress`: 已开工但未形成可交付闭环
- `blocked`: 因外部依赖或技术问题阻塞
- `done`: 已完成并具备对应验证结果

**当前快照**

- 当前阶段: `Phase 0 / pre-implementation`
- 总体状态: 仅完成产品与工程文档，代码尚未开始落地
- 最新可用文档: `docs/prd.md`、`docs/architecture.md`、`docs/implementation-roadmap.md`
- 建议下一步: 从 `Phase 0` 开始搭建 Rust workspace、Flutter App 骨架和协议/配置 crate

**模块状态总表**

| 模块 | 状态 | 负责人 | 说明 | 下一步 |
| --- | --- | --- | --- | --- |
| 文档基线 `prd` | done | 已完成 | 产品需求已定义 | 作为验收依据保留 |
| 文档基线 `architecture` | done | 已完成 | MVP 架构、接口、协议和状态机已定义 | 编码时按此为默认基线 |
| 文档基线 `implementation-roadmap` | done | 已完成 | 阶段、门槛和任务顺序已定义 | 按阶段推进 |
| Rust workspace | not_started | 待定 | 目录和 Cargo workspace 尚未建立 | 创建 `crates/` 和根 `Cargo.toml` |
| `crates/protocol` | not_started | 待定 | 协议模型未编码 | 先定义消息、错误码、基础类型 |
| `crates/config` | not_started | 待定 | 配置模型未编码 | 先定义 TOML 模型和默认值 |
| `crates/server-core` | not_started | 待定 | 会话状态机和协商逻辑未编码 | 等 protocol/config 骨架完成后开始 |
| `crates/display-backend` | not_started | 待定 | Wayland 主后端尚未验证 | Phase 1 优先验证 |
| `crates/capture` | not_started | 待定 | 捕获链路未编码 | 随显示后端验证同步推进 |
| `crates/encoder` | not_started | 待定 | 编码链路未编码 | 最小目标是 H.264 Annex B |
| `crates/transport` | not_started | 待定 | WebSocket 传输未编码 | Phase 2 打通最小 LAN 闭环 |
| `crates/app-cli` | not_started | 待定 | CLI 命令未编码 | 先提供占位命令 |
| `apps/android` Flutter 壳 | not_started | 待定 | App 工程未建立 | 创建页面骨架和状态管理骨架 |
| Android 原生解码插件 | not_started | 待定 | MediaCodec 桥接未编码 | Phase 2 最小 H.264 解码 |
| `systemd --user` 集成 | not_started | 待定 | unit 和说明未落地 | Phase 5 完成 |
| `doctor/probe` | not_started | 待定 | 诊断命令未落地 | 依赖后端 probe 结果 |
| USB `adb reverse` | not_started | 待定 | USB 接入未落地 | LAN 跑通后复用协议接入 |

**已完成内容**

1. 完成 `docs/prd.md` 的产品需求定义。
2. 完成 `docs/architecture.md` 的系统架构设计。
3. 完成 `docs/implementation-roadmap.md` 的分阶段路线图。
4. 明确了 MVP 默认技术路线，包括 Rust workspace + Flutter Android、单会话模型、WebSocket 单连接承载控制与媒体、`H.264` 为 MVP 必达媒体格式，以及显示参数与串流参数两阶段决策。

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

只要上述任一项存疑，就不应跳到复杂 UI 或增强特性。

**推荐接手顺序**

新 Agent 接手时，优先按以下顺序工作。

1. 读取 `docs/architecture.md` 的 `MVP 具体技术选择`、`核心抽象接口`、`控制协议架构`。
2. 读取 `docs/implementation-roadmap.md` 的 `Phase 0` 和 `Phase 1`。
3. 搭建 Rust workspace 与 Flutter 壳。
4. 立即进入显示后端验证，而不是先做客户端 UI 细节。

**下一步建议**

最推荐的直接起点是以下 3 个任务。

1. 创建 Rust workspace 和 `crates/` 骨架。
2. 创建 Flutter Android App 骨架。
3. 为 `display-backend` 写出 `DisplayBackend` 抽象和最小 `probe` 骨架。

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
- 有最小验证结果。
- 有明确边界，不是零散改动。

**变更记录**

| 日期 | 变更人 | 内容 |
| --- | --- | --- |
| 2026-04-13 | OpenCode | 初始化实现状态文档，记录当前仍处于文档完成、代码未启动状态 |

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
