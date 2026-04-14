# Tab Screen Architecture

**文档信息**

- 项目代号: `tab-screen`
- 文档类型: Architecture Document
- 文档目标: 将 `docs/prd.md` 落实为可编码的 MVP 架构基线
- 适用范围: Rust 服务端 + Flutter Android 客户端
- 当前版本: `v1.0`
- 文档状态: Ready for implementation

**设计目标**

- 让后续 Agent 可以按统一模块边界开始编码，而不是边写边重构架构。
- 对 PRD 中尚未落地的实现细节给出最小但明确的技术决策。
- 保留 Wayland 显示后端和编码后端的替换能力，但不为 Post-MVP 过度设计。
- 明确系统中哪些部分必须先验证，哪些部分可以在验证通过后稳定展开实现。

**架构原则**

- 本地优先: MVP 仅服务 LAN 和 `adb reverse` 两种本地链路。
- 单会话优先: v1 服务端一次只维护一个活动副屏会话。
- 显示与串流解耦: 虚拟显示参数和串流参数分两阶段决策。
- 懒创建: 未连接客户端前不创建虚拟显示器。
- 稳定命名: 同一客户端多次连接必须映射到稳定显示器名称。
- 明确协商: 客户端请求不是最终结果，服务端始终返回实际生效值。
- 强诊断: 启动失败、后端缺失、能力不匹配都必须可诊断。
- 最小正确实现: 先打通固定参数闭环，再做协商、USB、设置页和增强能力。

**MVP 具体技术选择**

本节是为了减少后续实现分歧，除非 Phase 0/1 验证明确证明不可行，否则按以下选择实现。

- Rust 工作区组织服务端代码。
- Flutter Android App 负责 UI、连接管理和会话状态展示。
- 服务端异步运行时使用 `tokio`。
- CLI 使用 `clap`。
- 配置使用 `serde + toml`。
- 日志使用 `tracing`，输出面向 `journalctl --user` 友好的结构化文本。
- 协议统一跑在单个 TCP 端口上。
- MVP 传输层使用单条 WebSocket 连接。
- WebSocket 文本帧承载控制协议 JSON 消息。
- WebSocket 二进制帧承载媒体数据包。
- 媒体编码首版只要求 `H.264/AVC 8-bit SDR` 必通。
- Android 解码首选 `MediaCodec`，失败时允许软件回退或明确提示不支持。
- 服务端稳定身份映射持久化到用户态 state 目录。
- USB 模式不设计独立协议，直接复用 LAN 同一传输与协议栈。

选择 WebSocket 的原因很简单。

- Flutter 和 Rust 都有成熟实现，便于尽快打通闭环。
- `adb reverse` 和 LAN 都能透明复用。
- 文本控制帧 + 二进制媒体帧足够支撑 MVP。
- 即使存在 TCP 队头阻塞，也符合 MVP 的本地优先目标；低延迟 UDP/QUIC 留到 Post-MVP。

**系统上下文**

```text
Android Tablet App
  ├─ Home / Connect / Settings / Diagnostics / Fullscreen
  ├─ Session Controller
  ├─ WebSocket Transport
  └─ MediaCodec Decoder + Texture/Surface Renderer
            ↑
            │ LAN TCP or adb reverse TCP
            ↓
Rust Server CLI (`tab-screen serve`)
  ├─ Config + CLI + Logging
  ├─ Session Manager
  ├─ Client Identity Store
  ├─ Negotiation Engine
  ├─ Display Backend
  ├─ Capture Source
  ├─ Encoder Backend
  └─ WebSocket Control/Media Transport
            ↓
Linux Wayland Session
  ├─ Virtual Display Output
  └─ Desktop remembers per-display layout/scale
```

**仓库建议结构**

```text
.
├─ crates/
│  ├─ protocol/
│  ├─ config/
│  ├─ server-core/
│  ├─ display-backend/
│  ├─ capture/
│  ├─ encoder/
│  ├─ transport/
│  └─ app-cli/
├─ apps/
│  └─ android/
│     ├─ lib/
│     ├─ android/
│     └─ native/
└─ docs/
   ├─ prd.md
   ├─ architecture.md
   └─ implementation-roadmap.md
```

说明如下。

- `crates/protocol`: 协议消息、版本、错误码、协商数据结构。
- `crates/config`: 配置模型、默认值、校验、配置来源优先级合并。
- `crates/server-core`: 会话状态机、客户端注册、协商引擎、生命周期编排。
- `crates/display-backend`: `DisplayBackend` 抽象和具体后端实现。
- `crates/capture`: 帧定义、像素格式、帧源接口。
- `crates/encoder`: 编码抽象、硬件/软件实现、重配置逻辑。
- `crates/transport`: WebSocket 服务端、帧编解码、心跳。
- `crates/app-cli`: `serve/doctor/probe/print-default-config/usb/version` 命令入口。
- `apps/android/lib`: Flutter UI、状态管理、传输封装、持久化。
- `apps/android/native`: Android 原生解码和纹理桥接插件。

**服务端架构**

服务端进程由 6 个核心层组成。

1. 入口层

- 负责 CLI 参数解析。
- 合并 CLI、环境变量、配置文件、默认值。
- 初始化日志、state 目录、运行时和依赖对象。

2. 环境诊断层

- `doctor` 用于输出当前运行环境是否满足启动条件。
- `probe` 用于输出机器可用能力，推荐支持 JSON 输出，方便后续自动化测试。
- 诊断范围至少包括 Wayland 会话、显示后端、编码器、ADB、监听端口。

3. 会话编排层

- 负责唯一活动会话约束。
- 负责客户端连接、握手、心跳、断连和重协商。
- 负责在正确时机创建和销毁虚拟显示器。

4. 显示与捕获层

- 根据协商出的显示参数创建虚拟显示器。
- 绑定对应捕获源。
- 保证显示器名称稳定。

5. 编码与媒体层

- 将原始帧编码为 H.264 Annex B 访问单元。
- 支持码率、帧率、GOP、低延迟等基础参数。
- 支持部分参数重配置；若必须重建编码器，向客户端发状态通知。

6. 传输与状态回传层

- 负责发送控制消息和媒体帧。
- 负责会话状态、实际生效参数、错误码和统计信息回传。

**服务端状态机**

```text
Idle
  -> ClientConnected
  -> Handshaking
  -> DisplayProvisioning
  -> Negotiating
  -> Streaming
  -> Renegotiating
  -> Streaming
  -> Terminating
  -> Idle

Any state
  -> Error
  -> Terminating
  -> Idle
```

规则如下。

- `Idle` 时不得存在虚拟显示器和编码会话。
- 只有 `ClientHello` 校验通过后才能进入 `DisplayProvisioning`。
- `DisplayProvisioning` 失败时必须回滚已创建资源。
- `Streaming` 中允许处理 `UpdateStreamRequest`。
- 任意异常退出都必须触发显示器销毁和编码资源释放。

**核心抽象接口**

以下接口是后续 Rust 代码的主要边界，命名可微调，但职责不应改变。

```rust
pub trait DisplayBackend: Send + Sync {
    fn backend_name(&self) -> &'static str;
    fn probe(&self) -> anyhow::Result<DisplayBackendProbe>;
    fn supports_stable_naming(&self) -> bool;
    fn create_output(&self, spec: VirtualDisplaySpec) -> anyhow::Result<Box<dyn DisplayHandle>>;
}

pub trait DisplayHandle: Send {
    fn logical_name(&self) -> &str;
    fn backend_id(&self) -> &str;
    fn capture_source(&self) -> anyhow::Result<Box<dyn CaptureSource>>;
    fn destroy(self: Box<Self>) -> anyhow::Result<()>;
}

pub trait CaptureSource: Send {
    fn frame_format(&self) -> RawFrameFormat;
    fn next_frame(&mut self) -> anyhow::Result<RawFrame>;
}

pub trait EncoderBackend: Send {
    fn backend_name(&self) -> &'static str;
    fn probe(&self) -> anyhow::Result<EncoderProbe>;
    fn start(&mut self, config: EncoderConfig) -> anyhow::Result<()>;
    fn encode(&mut self, frame: RawFrame) -> anyhow::Result<EncodedFrame>;
    fn reconfigure(&mut self, update: EncoderReconfigure) -> anyhow::Result<ReconfigureOutcome>;
    fn stop(&mut self) -> anyhow::Result<()>;
}
```

说明如下。

- 显示后端负责“创建/销毁/命名”，不负责协商逻辑。
- 捕获源负责向编码器输出统一原始帧结构。
- 编码器抽象必须允许软件和硬件实现并存。
- `reconfigure` 必须明确返回“热更新成功”还是“需要重建”。

**客户端稳定身份与显示器命名**

客户端必须在首次安装后生成稳定 ID，并持久化在本地存储中。推荐使用 UUID v4。

服务端按如下方式处理稳定命名。

1. 接收 `client_stable_id`。
2. 在本地映射表中查找是否已有绑定记录。
3. 若不存在，则生成新的逻辑显示名并写入持久化存储。
4. 后续同一 `client_stable_id` 重连时复用相同逻辑显示名。

推荐 state 路径。

- `~/.local/state/tab-screen/clients.toml`
- `~/.local/state/tab-screen/runtime/`

推荐映射结构。

```toml
version = 1

[clients."3c3f0e38-7fd3-4a5f-8f60-0d7d0a2ea9a2"]
display_name = "Tab Screen 3C3F0E"
first_seen_at = "2026-04-13T10:00:00Z"
last_seen_at = "2026-04-13T12:00:00Z"
device_model = "SM-X700"
```

命名规则。

- 默认前缀来自配置 `display.name_prefix`。
- 后缀使用稳定 ID 的短哈希或短 UUID。
- 一旦落盘，不再因为设备型号变化而修改显示名。
- 若底层后端需要额外 backend-specific ID，也单独持久化，但不能影响逻辑名稳定性。

**显示参数与串流参数的两阶段决策**

阶段一是显示参数决策。

- 输入: 服务端配置、客户端设备屏幕参数、服务端后端能力。
- 默认: 跟随客户端上报的分辨率、刷新率、色深、方向、DPI。
- 强制覆盖: 若配置声明强制值，则对应字段以配置为准。
- 输出: `VirtualDisplaySpec`。

阶段二是串流参数决策。

- 输入: 服务端偏好、服务端限制、客户端解码能力、客户端覆盖请求。
- 默认: 客户端选择“遵循服务器偏好”时优先用服务器偏好。
- 覆盖: 客户端提供覆盖值时，在限制范围内尽量满足。
- 降级: 超限或能力不足时生成降级后的 `EffectiveStreamParams` 和原因列表。

两阶段必须严格分离。

- 不允许因为串流降级而改变已确定的虚拟显示器参数，除非用户主动重建会话。
- 允许显示器是 `2560x1600@60`，而串流是 `1920x1200@30`。

**配置架构**

配置优先级必须为 `CLI > ENV > file > default`。

配置模型采用三层结构。

1. 原始配置模型

- 直接映射 TOML 字段。

2. 规范化配置模型

- 将字符串分辨率解析为内部结构。
- 填充默认值。

3. 运行时有效配置模型

- 保证所有校验已通过，可直接驱动运行时。

必须把“默认值”和“限制值”分开建模，避免协商逻辑混淆。

核心模型建议如下。

```text
AppConfig
  ├─ ServerConfig
  ├─ DisplayConfig
  ├─ StreamPreferenceConfig
  ├─ StreamLimitsConfig
  ├─ EncoderConfig
  ├─ NetworkConfig
  └─ UsbConfig
```

**控制协议架构**

MVP 使用单条 WebSocket 连接，URL 约定如下。

- LAN: `ws://<host>:<port>/session`
- USB: `ws://127.0.0.1:<port>/session`

控制消息使用 JSON 文本帧，采用内部 tagged enum 风格。

```json
{
  "type": "client_hello",
  "protocol_version": 1,
  "client_stable_id": "3c3f0e38-7fd3-4a5f-8f60-0d7d0a2ea9a2",
  "device_model": "SM-X700",
  "network_mode": "lan",
  "device_screen": {
    "width": 2560,
    "height": 1600,
    "refresh_rate": 60,
    "color_depth": 8,
    "orientation": "landscape",
    "dpi": 280
  },
  "decode_caps": {
    "codecs": ["h264", "hevc"],
    "max_width": 2560,
    "max_height": 1600,
    "max_frame_rate": 60,
    "hardware_decode": true
  }
}
```

服务端响应示例。

```json
{
  "type": "server_hello",
  "protocol_version": 1,
  "server_version": "0.1.0",
  "display_backend": "auto:selected-backend",
  "available_codecs": ["h264"],
  "display_name": "Tab Screen 3C3F0E",
  "stream_preference": {
    "resolution": "1920x1200",
    "frame_rate": 60,
    "codec": "h264",
    "bitrate_kbps": 12000
  },
  "stream_limits": {
    "max_resolution": "2560x1600",
    "max_frame_rate": 60,
    "allowed_codecs": ["h264"],
    "min_bitrate_kbps": 2000,
    "max_bitrate_kbps": 30000,
    "allow_client_override": true
  }
}
```

MVP 必须支持的控制消息。

- `client_hello`
- `server_hello`
- `start_session_request`
- `start_session_response`
- `update_stream_request`
- `update_stream_response`
- `heartbeat`
- `error`
- `session_ended`
- `session_state`

`session_state` 不是 PRD 中强制要求的名字，但建议加入，专门承载 `connecting/negotiating/streaming/renegotiating/error` 等状态，避免把 UI 状态更新塞进错误消息。

**媒体数据包格式**

WebSocket 二进制帧承载 `MediaPacketV1`。

```text
Byte 0      : packet_type (1 = video)
Byte 1      : codec (1 = h264, 2 = hevc)
Byte 2      : flags bitset (bit0 keyframe, bit1 config, bit2 end_of_stream)
Byte 3      : reserved
Byte 4..11  : pts_us (u64 LE)
Byte 12..15 : payload_len (u32 LE)
Byte 16..N  : Annex B access unit bytes
```

规则如下。

- 一条二进制帧只承载一个访问单元。
- `config` 帧用于发送 SPS/PPS 等解码配置数据。
- 客户端必须在收到 config + keyframe 后才能恢复渲染。
- `payload_len` 仅用于防御性校验，实际边界由 WebSocket frame 保证。

这样设计的原因是实现简单、足够稳定，并且适合 MediaCodec 直接喂入 Annex B 数据。

**会话生命周期**

完整时序如下。

1. 客户端建立 WebSocket 连接。
2. 客户端发送 `client_hello`。
3. 服务端校验协议版本、认证信息、设备屏幕参数和单会话限制。
4. 服务端解析稳定身份，生成或加载稳定显示名。
5. 服务端返回 `server_hello`。
6. 客户端发送 `start_session_request`，声明是否遵循服务器偏好及覆盖值。
7. 服务端先完成显示参数决策并创建虚拟显示器。
8. 服务端再完成串流参数协商。
9. 服务端启动捕获、编码和发送任务。
10. 服务端返回 `start_session_response`，包含最终生效参数。
11. 客户端初始化解码器并进入全屏显示。
12. 会话中客户端可以发送 `update_stream_request`。
13. 断连或主动结束时，服务端销毁显示器并落盘更新状态。

**并发模型**

服务端运行时建议拆为以下任务。

- 监听任务: 接受新连接。
- 会话控制任务: 解析文本控制消息、维护状态机。
- 媒体发送任务: 读取编码输出并发送二进制帧。
- 心跳任务: 周期性发送/接收保活和统计。
- 资源监控任务: 监控编码器、捕获源和显示后端异常。

即使内部是多任务，也必须通过单一 `SessionManager` 串行化对“当前活动会话”的状态修改，避免显示器重复创建或销毁顺序错乱。

**Android 客户端架构**

Flutter 端按 4 层组织。

1. Presentation

- `HomePage`
- `ConnectPage`
- `FullscreenPage`
- `SettingsPage`
- `DiagnosticsPage`

2. Application

- `SessionController`: 会话状态、连接流程、重连和页面状态汇总。
- `SettingsController`: 用户设置与“请求值/实际值”展示。
- `DiagnosticsController`: 最近错误、连接方式、统计汇总。

3. Data

- `TransportRepository`: WebSocket 建连、消息收发、心跳。
- `PreferencesRepository`: 最近连接地址、稳定 ID、本地设置持久化。
- `DecoderRepository`: 与原生插件交互。

4. Platform

- Android 原生插件负责 `MediaCodec`、`Surface/Texture` 和沉浸式显示桥接。

状态管理建议使用 `flutter_riverpod`。原因是它足够轻量，便于表达连接状态机、页面状态与异步副作用。

**Android 原生解码插件职责**

- 创建并持有 `SurfaceTexture` 或 `SurfaceView` 绑定目标。
- 封装 `MediaCodec` 初始化、喂帧、flush、重建和释放。
- 向 Flutter 暴露最小 API。

建议 API 形态。

```text
createRenderer() -> textureId
initializeDecoder(codec, width, height, configBytes)
queueAccessUnit(bytes, ptsUs, isKeyFrame)
reconfigure(codec, width, height, configBytes)
releaseSession()
```

设计原则。

- Flutter 不直接处理大块视频解码逻辑。
- Flutter 只维护会话状态和 UI，不承担高频渲染数据复制。
- 当服务端要求重建编码器时，客户端插件必须支持短时黑屏恢复。

**配置与持久化路径**

服务端推荐路径。

- 配置: `~/.config/tab-screen/config.toml`
- 用户态 state: `~/.local/state/tab-screen/`
- 运行日志: 交给 `journalctl --user`，不额外设计文件日志为默认路径。

客户端持久化项。

- `client_stable_id`
- `last_successful_server`
- `preferred_connection_mode`
- 用户设置页中可持久化的覆盖偏好
- 最近错误摘要

**诊断与可观测性**

服务端日志字段至少应包含。

- `event`
- `client_id`
- `display_name`
- `session_id`
- `backend`
- `codec`
- `resolution`
- `frame_rate`
- `result`
- `error_code`

`doctor` 建议输出检查项。

- 当前是否处于 Wayland 图形会话。
- 已选或可选显示后端。
- 后端是否支持按需创建/销毁。
- 后端是否支持稳定命名。
- 可用编码器与编码模式。
- `adb` 是否存在且可调用。
- 监听地址、端口可用性。
- 当前配置文件路径与解析结果。

客户端诊断页至少展示。

- 连接方式。
- 实际生效分辨率/帧率/编码格式。
- 解码方式。
- 估算延迟。
- 最近错误。
- 当前服务器地址。

**安全架构**

MVP 不默认要求链路加密，但必须支持鉴权。

- LAN 模式默认开启 token 校验。
- USB 模式可允许沿用相同 token 逻辑，也可通过配置放宽。
- 协议消息中不得回显完整 token。
- 日志中不得打印敏感凭据。
- 协议结构中预留 `encryption` / `pairing_mode` 字段，为后续扩展加密或配对流程做兼容。

**失败处理策略**

必须统一错误码，并在协议、日志和 UI 中使用同一语义。推荐分类。

- `auth_failed`
- `protocol_version_mismatch`
- `invalid_config`
- `missing_device_screen_params`
- `display_backend_unavailable`
- `display_creation_failed`
- `encoder_unavailable`
- `decoder_unavailable`
- `parameter_out_of_range`
- `session_busy`
- `network_disconnected`
- `session_timeout`
- `internal_error`

错误处理规则。

- 可恢复错误优先通过 `error` + `session_state` 回传。
- 不可恢复错误必须结束会话并清理资源。
- 若显示器已创建但编码器失败，必须立即销毁显示器，避免桌面残留孤儿输出。

**测试策略**

编码与验证必须同步推进，测试不是收尾工作，而是每个模块交付定义的一部分。

- 新增功能或修复缺陷时，必须在同一改动中补上与行为匹配的必要测试，用来验证正确性并防止回归。
- 测试覆盖至少要触达关键成功路径、重要边界条件和相关失败路径，而不是只验证“能跑通一次”。
- 若当前阶段确实无法自动化验证，必须明确记录原因，并补充最小可执行的人工验证步骤与实际结果。

服务端。

- `protocol/config/server-core` 以单元测试为主。
- `display-backend/encoder/transport` 以集成测试和 probe 测试为主。
- 协商逻辑必须覆盖“遵循服务器偏好”“客户端覆盖”“超限降级”“拒绝请求”四类用例。

客户端。

- Flutter 控制器和存储逻辑使用单元测试。
- 协议收发与状态迁移使用 widget/integration test。
- `MediaCodec` 相关行为通过真机手测和最小自动化验证组合完成。

端到端。

- 最小闭环: 固定参数 LAN 连接并显示画面。
- 协商闭环: 修改设置后服务端返回新的实际值。
- 生命周期闭环: 连接时创建显示器，断开时销毁显示器。
- 稳定命名闭环: 同一客户端重连后显示器名称不变。

**Phase 0/1 必须验证的开放决策**

以下内容必须在正式展开编码前确认，不能靠后期补救。

- 选定的 `DisplayBackend` 主后端是否真的支持按连接创建/销毁。
- 主后端是否允许稳定命名或可通过持久映射实现等价效果。
- 该后端对应的捕获链路能否稳定输出给编码器。
- 选定的 Rust 编码栈是否能稳定产出适合 MediaCodec 的 H.264 Annex B。
- Flutter 原生插件喂入访问单元的性能是否足够支撑目标分辨率和帧率。

若其中任何一项失败，优先调整后端选型，不要继续堆叠 UI 或配置功能。

**不在 v1 解决的问题**

- 多客户端同时观看。
- 多块平板同时作为多个副屏。
- 音频、输入回传、剪贴板、文件传输。
- QUIC/UDP 媒体通道。
- HDR、10-bit、AV1。
- 面向所有 Wayland compositor 的通用兼容层。

**实施基线**

后续 Agent 在编码时应将本架构视为默认基线。

- 不要把“代码已写完但未测试”视为模块完成。
- 不要先做复杂 UI，再回头验证显示后端。
- 不要在 MVP 阶段引入第二套传输协议。
- 不要把显示参数和串流参数混成一套结构。
- 不要在未定义稳定身份持久化前实现显示器创建逻辑。
- 不要绕过 `DisplayBackend` 和 `EncoderBackend` 抽象直接把具体实现写死在 `server-core`。
