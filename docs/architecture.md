# Tab Screen Architecture

**文档信息**

- 项目代号: `tab-screen`
- 文档类型: Architecture Document
- 文档目标: 将 `docs/prd.md` 落实为可编码的 MVP 架构基线
- 适用范围: Rust 服务端 + Flutter Android 客户端
- 当前版本: `v1.1`
- 文档状态: Phase 1 validated baseline

**设计目标**

- 让后续 Agent 可以按统一模块边界开始编码，而不是边写边重构架构。
- 对 PRD 中尚未落地的实现细节给出最小但明确的技术决策。
- 保留显示后端和编码后端的替换能力，但不为 Post-MVP 过度设计。
- 明确系统中哪些部分必须先验证，哪些部分可以在验证通过后稳定展开实现。
- 在文档层明确 Phase 1 已选定的主显示后端、运行权限模型和验证结论，避免后续实现再次回到已否定的方向。

**架构原则**

- 本地优先: MVP 仅服务 LAN 和 `adb reverse` 两种本地链路。
- 单会话优先: v1 服务端一次只维护一个活动副屏会话。
- 显示与串流解耦: 虚拟显示参数和串流参数分两阶段决策。
- 懒创建: 未连接客户端前不创建虚拟显示器。
- 稳定命名: 同一客户端多次连接必须映射到稳定显示器名称。
- 明确协商: 客户端请求不是最终结果，服务端始终返回实际生效值。
- 强诊断: 启动失败、后端缺失、能力不匹配都必须可诊断。
- 最小正确实现: 先打通固定参数闭环，再做协商、USB、设置页和增强能力。
- 特权边界清晰: `evdi` 动态创建设备节点与打开节点通常需要管理员权限，因此服务端主运行模型采用系统级特权服务，而不是 `systemd --user`。
- 身份分层: 稳定身份由服务端映射层和 EDID 身份共同提供，不依赖 `cardX` 编号稳定。

**MVP 具体技术选择**

本节是为了减少后续实现分歧，除非 Phase 1 之后的进一步验证明确证明不可行，否则按以下选择实现。

- Rust 工作区组织服务端代码。
- Flutter Android App 负责 UI、连接管理和会话状态展示。
- 服务端异步运行时使用 `tokio`。
- CLI 使用 `clap`。
- 配置使用 `serde + toml`。
- 日志使用 `tracing`，输出面向系统服务日志查看友好的结构化文本。
- 协议统一跑在单个 TCP 端口上。
- MVP 传输层使用单条 WebSocket 连接。
- WebSocket 文本帧承载控制协议 JSON 消息。
- WebSocket 二进制帧承载媒体数据包。
- 媒体编码首版只要求 `H.264/AVC 8-bit SDR` 必通。
- Android 解码首选 `MediaCodec`，失败时允许软件回退或明确提示不支持。
- 服务端稳定身份映射持久化到系统服务可管理的 state 目录。
- USB 模式不设计独立协议，直接复用 LAN 同一传输与协议栈。
- 主显示后端选择 `evdi + libevdi`。
- `evdi` 集成调用系统安装的 `libevdi`，并优先使用 `evdi_open_attached_to_fixed`，不依赖已弃用的 `evdi_open_attached_to`。
- 服务端默认运行模型是系统级特权服务，以便创建、打开和管理 `evdi` DRM 节点。
- 显示器稳定身份通过客户端稳定 ID -> 逻辑显示名映射，再结合稳定 EDID 监视器名与序列号实现。
- Phase 1 捕获路径采用 `evdi_register_buffer` + `evdi_request_update` + `evdi_handle_events` + `evdi_grab_pixels` 的最小同步抓帧实现。

选择 WebSocket 的原因很简单。

- Flutter 和 Rust 都有成熟实现，便于尽快打通闭环。
- `adb reverse` 和 LAN 都能透明复用。
- 文本控制帧 + 二进制媒体帧足够支撑 MVP。
- 即使存在 TCP 队头阻塞，也符合 MVP 的本地优先目标；低延迟 UDP/QUIC 留到 Post-MVP。

选择 `evdi` 作为主显示后端的原因如下。

- 它已在目标环境中完成 Phase 1 可行性验证。
- 它能由内核模块向 DRM 暴露虚拟输出，不再要求服务端直接依赖当前登录用户的 Wayland 会话控制接口。
- 它支持按需创建/销毁虚拟显示路径，符合“按连接生命周期创建与回收”的产品要求。
- 它可以通过稳定 EDID 身份配合逻辑显示名映射，为桌面侧记忆显示器布局提供可行基础。
- 它具备可直接抓取 framebuffer 的用户态接口，能支持后续编码链路。

**Phase 1 已确认的后端结论**

以下结论来自已完成的本机 Phase 1 验证，应视为当前实现基线。

- 当前主后端: `evdi`
- 用户态库版本: `libevdi 1.14.15`
- 当前内核模块版本: `evdi 1.14.15`
- 当前已验证环境: `Arch Linux`, `kernel 6.19.11-arch1-1`, `niri`, `Wayland`
- 运行权限要求: 创建和打开 `evdi` 节点时通常需要 root 或等效管理员权限
- Arch Linux 前置步骤:
  1. 安装 `evdi-dkms`
  2. 安装 `linux-headers`
  3. 创建 `/etc/modules-load.d/evdi.conf`，内容为 `evdi`，以实现开机自动加载模块
- 当前已验证能力:
  - `doctor` 可输出 `evdi` 环境检查结果
  - `probe` 可输出后端探测结果
  - 可重复执行“创建显示器 -> 等待 -> 抓取一帧 -> 销毁显示器”
  - 同一逻辑显示名重复验证时，逻辑身份和生成的 EDID 身份保持稳定
  - 已成功抓取 `1920x1200`, `stride=7680`, `9216000` 字节的非空帧
- 当前已知边界:
  - 尚未完成多 compositor 支持矩阵
  - 尚未完成持续串流与编码联调
  - 稳定身份依赖映射层 + EDID，而不是内核节点编号

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
System Service (`tab-screen serve`)
  ├─ Config + CLI + Logging
  ├─ Session Manager
  ├─ Client Identity Store
  ├─ Negotiation Engine
  ├─ Display Backend
  ├─ Capture Source
  ├─ Encoder Backend
  └─ WebSocket Control/Media Transport
            ↓
Linux DRM / EVDI
  ├─ Virtual Display Output
  ├─ EDID Identity
  └─ Framebuffer Access
            ↓
Desktop Compositor / Session
  ├─ Detects virtual output
  └─ Remembers per-display layout/scale
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
- 负责适配系统级服务启动模型下的路径与权限约束。

2. 环境诊断层

- `doctor` 用于输出当前运行环境是否满足启动条件。
- `probe` 用于输出机器可用能力，推荐支持 JSON 输出，方便后续自动化测试。
- Phase 1 下诊断范围至少包括 `evdi` 模块、`libevdi` 版本、权限状态、基础 DRM 节点、显示后端、编码器、ADB、监听端口。
- 不再把“当前用户的 Wayland 会话是否可直接操作”作为主前提检查项。

3. 会话编排层

- 负责唯一活动会话约束。
- 负责客户端连接、握手、心跳、断连和重协商。
- 负责在正确时机创建和销毁虚拟显示器。

4. 显示与捕获层

- 根据协商出的显示参数创建虚拟显示器。
- 绑定对应捕获源。
- 保证显示器逻辑身份稳定。
- 负责将逻辑显示身份映射为稳定的 EDID 身份信息。

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

**当前主后端实现切面**

Phase 1 已选定 `evdi` 后，显示后端层当前最重要的后端内部分工如下。

1. `probe`

- 查询 `libevdi` 版本。
- 检查 `evdi` 模块是否已加载。
- 给出 Arch Linux 前置步骤提醒。
- 给出权限要求与系统级服务运行建议。
- 报告是否支持创建/销毁与稳定逻辑命名。

2. `create_output`

- 接收 `VirtualDisplaySpec`。
- 生成包含稳定监视器名与序列号的 EDID。
- 通过 `evdi_open_attached_to_fixed` 打开或创建 `evdi` 设备。
- 调用 `evdi_connect` 向 DRM 暴露虚拟显示器。
- 返回 `DisplayHandle`。

3. `capture_source`

- 创建 `evdi` 抓帧对象。
- 等待 `mode_changed`。
- 注册用户态 buffer。
- 请求 update。
- 在 `update_ready` 后抓取像素。
- 输出 `RawFrame`。

4. `destroy`

- 先断开显示器。
- 再关闭 `evdi` handle。
- 在 Rust 边界尽量保证幂等回收。

**客户端稳定身份与显示器命名**

客户端必须在首次安装后生成稳定 ID，并持久化在本地存储中。推荐使用 UUID v4。

服务端按如下方式处理稳定命名。

1. 接收 `client_stable_id`。
2. 在本地映射表中查找是否已有绑定记录。
3. 若不存在，则生成新的逻辑显示名并写入持久化存储。
4. 后续同一 `client_stable_id` 重连时复用相同逻辑显示名。
5. 用相同逻辑显示名生成稳定 EDID 监视器名和序列号。

推荐 state 路径。

- `/var/lib/tab-screen/clients.toml`
- `/var/lib/tab-screen/runtime/`

推荐映射结构。

```toml
version = 1

[clients."3c3f0e38-7fd3-4a5f-8f60-0d7d0a2ea9a2"]
display_name = "Tab Screen 3C3F0E"
first_seen_at = "2026-04-13T10:00:00Z"
last_seen_at = "2026-04-13T12:00:00Z"
device_model = "SM-X700"
edid_monitor_name = "TabScr3C3F0E"
edid_serial = "TSA1B2C3D4"
```

命名规则。

- 默认前缀来自配置 `display.name_prefix`。
- 后缀使用稳定 ID 的短哈希或短 UUID。
- 一旦落盘，不再因为设备型号变化而修改显示名。
- 若底层后端需要额外 backend-specific ID，也单独持久化，但不能影响逻辑名稳定性。
- 不依赖 `cardX` 编号复用来表达稳定身份。
- `evdi` 场景下，稳定身份的关键是逻辑映射 + 稳定 EDID，而不是 DRM 节点号。

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

系统级服务模型下的推荐路径如下。

- 配置: `/etc/tab-screen/config.toml`
- state: `/var/lib/tab-screen/`
- runtime: `/run/tab-screen/`
- 日志: 系统服务日志查看工具，不额外设计文件日志为默认路径

仍可保留用户态手动运行支持，但它不是 MVP 的主部署模型。

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
Byte 4..11  : pts_us (u64 little endian)
Byte 12..15 : payload_len (u32 little endian)
Byte 16..N  : payload
```

约束如下。

- Phase 2 默认只跑视频包。
- `config` 标记用于 SPS/PPS 等配置数据。
- `end_of_stream` 用于平滑结束播放与解码器刷新。

**显示后端接口与捕获的实现边界**

当前显示和捕获层职责进一步细化如下。

- `display-backend` 负责:
  - 后端探测
  - EDID 生成
  - `evdi` 连接与断开
  - 暴露抓帧入口
- `capture` 负责:
  - 统一 `RawFrameFormat`
  - 统一 `RawFrame`
  - 像素格式和 buffer 大小计算辅助逻辑
- `encoder` 后续只接收统一原始帧，不感知 `evdi` 细节

Phase 1 当前默认抓帧格式。

- 像素格式: `BGRA8888`
- 位深: `8-bit`
- 单 buffer、同步请求为主
- 以正确性与可诊断性优先，不做连续高性能优化

**错误处理与可诊断性**

以下错误必须有明确表现。

- `evdi` 内核模块未加载
- `libevdi` 不可用或版本不兼容
- 权限不足，无法创建或打开 `evdi` 节点
- `mode_changed` 长时间未到达
- `update_ready` 长时间未到达
- 抓取到空帧或不支持的像素格式
- 编码器不可用
- 客户端请求超出限制
- 网络中断或会话超时

推荐错误处理原则。

- 环境前置失败尽量在 `doctor` 或 `probe` 阶段暴露。
- `DisplayProvisioning` 失败时必须回滚已创建的 `evdi` 资源。
- 错误消息必须同时包含机器可读的错误码和简洁的人类可读描述。
- 对于特权相关失败，日志中必须明确提示“应使用系统级特权服务运行”。

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

- `libevdi` 版本与可调用状态
- `evdi` 模块是否已加载
- 是否具备特权运行条件
- Arch Linux 前置条件提醒
- 已选或可选显示后端
- 后端是否支持按需创建/销毁
- 后端是否支持稳定命名
- 可用编码器与编码模式
- `adb` 是否存在且可调用
- 监听地址、端口可用性
- 当前配置文件路径与解析结果

客户端诊断页至少展示。

- 当前连接方式
- 当前显示参数
- 当前串流参数
- 当前解码方式
- 最近错误摘要
- 最近一次降级原因

**Android 客户端与原生插件分层**

Flutter 和原生解码层的职责边界如下。

Flutter 层负责。

- 页面导航
- 连接状态展示
- 设置页与诊断页
- 协议消息发送与状态管理
- 最近地址与稳定 ID 持久化

Android 原生层负责。

- `MediaCodec` 初始化与重配置
- 解码输入队列
- Surface / Texture 输出
- 解码异常反馈

向 Flutter 暴露最小 API。

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

- 配置: `/etc/tab-screen/config.toml`
- state: `/var/lib/tab-screen/`
- runtime: `/run/tab-screen/`
- 运行日志: 交给系统服务日志查看链路，不额外设计文件日志为默认路径。

客户端持久化项。

- `client_stable_id`
- `last_successful_server`
- `preferred_connection_mode`
- 用户设置页中可持久化的覆盖偏好
- 最近错误摘要

**支持矩阵策略**

Phase 1 之后的支持矩阵口径如下。

- 已验证后端路径: `evdi`
- 已验证系统: `Arch Linux`
- 已验证桌面环境: `niri`
- 已验证显示协议环境: `Wayland`
- 未承诺范围:
  - 所有 Wayland compositor 全支持
  - X11/Xorg 专门路径
  - 多输出同时驱动
  - 非 `evdi` 后端作为主路径

后续如扩展矩阵，必须在 `docs/implementation-status.md` 和验证文档中同步记录新增环境与结果。

**阶段口径**

当前阶段执行口径如下。

- Phase 1 结论已确认: 主显示后端为 `evdi + libevdi`
- Phase 2 开始前，不再讨论回到 `systemd --user` 作为主部署模型
- Phase 2 应直接基于当前 `evdi` 创建/抓帧路径接入编码与 WebSocket 最小闭环
- 后续若发现系统级特权服务模型与实际部署不符，必须先更新文档再改代码

**明确延期到 Post-MVP 的内容**

- `HEVC` 正式支持
- 多 compositor 扩展
- QUIC/UDP 媒体通道
- 多客户端/多副屏
- 音频、输入、剪贴板、文件传输
- HDR、10-bit、AV1
- 高级 cursor 分离与脏区优化
- `evdi` 之外的显示后端优选策略

**最终执行口径**

后续 Agent 按本架构推进时，应遵守以下硬规则。

1. `evdi` 已是当前主显示后端，不要在 Phase 2 之前重新发散到其他主路径。
2. 任意涉及部署模型的实现与文档，都应以系统级特权服务为默认口径，而不是 `systemd --user`。
3. 稳定显示身份依赖逻辑映射与稳定 EDID，不要把 `cardX` 编号当作稳定身份。
4. 任何功能实现或缺陷修复，只要缺少必要测试或明确验证结果，就不要标记为完成。
5. 任何后续偏离本文档的实现，必须先更新文档并在 `docs/implementation-status.md` 记录原因。