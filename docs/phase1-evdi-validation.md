# Tab Screen Phase 1 EVDI Validation Record

**文档信息**

- 项目代号: `tab-screen`
- 文档类型: Phase 1 Validation Record
- 文档目标: 记录 `evdi + libevdi` 作为主显示后端的实际验证结果、前置条件、命令、观察结论与已知限制
- 适用范围: Rust 服务端 `display-backend` / `capture` / `app-cli`
- 当前版本: `v1.0`
- 文档状态: Validated on target environment

## 结论摘要

Phase 1 已确认 `evdi + libevdi` 可作为 `tab-screen` MVP 的主 `DisplayBackend` 路线。

在本次目标环境中，已验证以下结论成立：

- 可通过 `evdi` 内核模块与 `libevdi` 用户态库创建虚拟显示器。
- 可通过 `evdi_open_attached_to_fixed` 成功打开并管理 EVDI 设备句柄。
- 可按命令生命周期完成“创建显示器 -> 等待 -> 抓取一帧 -> 销毁显示器”。
- 同一逻辑显示名重复运行验证命令时，可复用同一稳定逻辑身份。
- 抓帧链路可返回非空像素帧，证明后续编码 Phase 可继续推进。
- 该路线依赖特权权限，更适合系统级 root service，而不是 `systemd --user`。

本阶段不声称已经完成：

- 全 compositor 通用兼容性验证；
- 端到端串流闭环；
- 稳定显示布局是否在所有桌面环境中都能仅依靠相同 EDID/逻辑身份完全复用；
- 多显示器、多客户端、多编码器矩阵验证。

## 目标环境

本次验证基于以下实际环境信息：

- 操作系统: Arch Linux
- 内核版本: `6.19.11-arch1-1`
- 桌面会话类型: `wayland`
- 当前桌面 / compositor: `niri`
- `evdi-dkms` 版本: `1.14.15-1`
- `linux-headers` 版本: `6.19.11.arch1-1`
- 本机已安装 `libevdi`
- 本机头文件路径: `/usr/include/evdi_lib.h`

## 关键实现决策

### 1. 主后端选择

Phase 1 选择 `evdi + libevdi` 作为主显示后端。

原因：

- 已在目标环境上验证可创建和销毁虚拟显示器；
- 可通过用户态 API 明确控制连接、断开、缓冲区注册、事件处理与抓帧；
- 比依赖当前用户 Wayland 会话私有协议更适合作为 MVP 的统一 Rust 后端入口；
- 满足先验证后端、再推进闭环的路线图要求。

### 2. 打开设备接口选择

本机头文件明确显示：

- `evdi_open_attached_to` 已被标注为 deprecated；
- 应使用 `evdi_open_attached_to_fixed`。

因此实现中统一使用：

- `evdi_open_attached_to_fixed`

而不使用旧接口。

### 3. 服务模型调整

由于动态创建 / 打开 EVDI DRM 节点通常需要管理员权限，本项目的服务模型从“优先 `systemd --user`”调整为：

- 优先系统级特权服务；
- 服务进程不再以“必须直接依赖当前用户 Wayland 会话”为前提；
- 仍保持懒创建语义：没有会话就不创建虚拟显示器。

### 4. 稳定身份策略

Phase 1 对“稳定命名”的结论是：

- 不依赖 `cardX` 编号稳定；
- 稳定身份由逻辑显示名映射层负责；
- 后端通过稳定生成的 EDID 监视器身份信息辅助桌面环境识别；
- 本次验证已证明同一逻辑显示名可重复创建 / 销毁并得到一致的逻辑标识输入。

## Arch Linux 前置步骤

在 Arch Linux 上启用 `evdi`，至少需要以下步骤。

### 1. 安装 `evdi-dkms`

安装 AUR 包：

- `evdi-dkms`

该包提供 `evdi` DKMS 内核模块。

### 2. 安装 `linux-headers`

安装与当前运行内核匹配的：

- `linux-headers`

否则 DKMS 模块无法正确构建。

### 3. 配置开机自动加载模块

创建文件：

- `/etc/modules-load.d/evdi.conf`

文件内容应包含：

```/dev/null/evdi.conf#L1-1
evdi
```

这样系统启动时会自动加载 `evdi` 模块。

## 建议但非本次强制要求的补充项

根据 `evdi` 上游文档，在某些环境下还可能需要额外模块参数或 `modprobe.d` 配置来改善兼容性，例如：

- `initial_device_count`
- `softdep`

但本次 Phase 1 验证以仓库当前最小可行路径为准，未将这些项作为完成门槛。

## Phase 1 实现范围

本阶段实际落地了以下能力：

### `crates/display-backend`

- 保留通用 `DisplayBackend` / `DisplayHandle` 抽象；
- 新增 `EvdiDisplayBackend`；
- 新增 `EvdiDisplayHandle`；
- 新增 `libevdi` FFI 接入；
- 使用 `evdi_open_attached_to_fixed` 打开设备；
- 使用生成的 EDID 连接虚拟显示器；
- 支持销毁时执行 `evdi_disconnect` 与 `evdi_close`；
- 基于逻辑显示名生成稳定 EDID 标识信息。

### `crates/capture`

- 保留通用 `CaptureSource` 抽象；
- 新增最小抓帧辅助逻辑；
- 支持 32-bit packed framebuffer 路径；
- 支持帧大小计算与边界检查；
- 为 Phase 1 抓帧验证提供基础数据结构与测试。

### `crates/app-cli`

- `doctor` 输出 Phase 1 环境诊断结果；
- `probe` 输出 `evdi` 后端探测结果；
- 新增显示验证命令路径，用于：
  - 创建显示器
  - 等待观察窗口
  - 抓取一帧
  - 销毁显示器

## 本次验证使用的命令

### 1. 自动化测试与编译检查

已通过：

- `cargo test -p capture -p display-backend -p tab-screen`
- `cargo check`

### 2. 环境诊断

使用特权执行：

- `sudo ./target/debug/tab-screen doctor`

观察结果：

- `libevdi` 可探测；
- `/sys/module/evdi` 存在；
- `/dev/dri` 存在；
- `/etc/modules-load.d/evdi.conf` 已存在；
- 以 root 权限运行时诊断通过。

### 3. 后端探测

使用特权执行：

- `sudo ./target/debug/tab-screen probe`

观察结果：

- backend: `evdi`
- supports stable naming: `yes`
- supports create/destroy: `yes`

### 4. 显示器创建 / 销毁 / 抓帧验证

使用特权执行：

- `sudo ./target/debug/tab-screen probe validate-display --display-name 'Tab Screen Validation' --capture`

并以同一逻辑显示名重复执行两次。

## 实际验证结果

### 验证 1

命令：

- `sudo ./target/debug/tab-screen probe validate-display --display-name 'Tab Screen Validation' --capture`

关键输出要点：

- 逻辑显示名: `Tab Screen Validation`
- 请求模式: `1920x1200 @ 60 Hz, 8-bit, 160 DPI`
- 后端探测结果:
  - backend=`evdi`
  - create/destroy=`yes`
  - stable_naming=`yes`
- 成功创建虚拟显示器
- backend id: `evdi:4C410403`
- 成功抓取一帧：
  - frame bytes: `9216000`
  - resolution: `1920x1200`
  - stride: `7680`
  - depth: `8bpp`
- 成功销毁虚拟显示器
- 最终结果: `PASS`

### 验证 2

命令：

- `sudo ./target/debug/tab-screen probe validate-display --display-name 'Tab Screen Validation' --capture`

关键输出要点：

- 使用与第一次完全相同的逻辑显示名；
- backend id 仍为: `evdi:4C410403`
- 再次成功完成创建 / 抓帧 / 销毁；
- 再次抓到：
  - `9216000` 字节
  - `1920x1200`
  - `stride=7680`
  - `depth=8bpp`
- 最终结果: `PASS`

## 结果解释

### 1. 创建 / 销毁能力

已证明在本机目标环境中，`evdi` 路线可重复创建并销毁虚拟显示器。

这满足 Phase 1 的核心退出门槛之一。

### 2. 抓帧能力

已成功抓取非空像素帧：

- 1920 × 1200 × 4 = 9,216,000 字节

这与结果完全一致，说明本阶段拿到的是有效的 32-bit packed framebuffer 数据。

这满足 Phase 1 对“能捕获到非空视频帧”的核心门槛。

### 3. 稳定逻辑身份

同一 `--display-name` 重复运行两次时：

- 逻辑显示名保持一致；
- 派生 `backend id` 一致。

这证明当前实现中的稳定逻辑身份生成是可重复的。

需要注意：

- 这证明的是应用层逻辑身份与 EDID 生成策略稳定；
- 还不等于“所有桌面环境都一定会完整复用既有布局配置”；
- 更广泛的桌面布局保持行为仍需后续产品化阶段继续验证。

## `doctor` 当前检查项结论

本次 Phase 1 中，`doctor` 已具备以下实际检查能力：

- `libevdi` backend probe
- `evdi` kernel module 是否存在
- `/dev/dri` 是否存在
- `/etc/modules-load.d/evdi.conf` 是否存在
- 当前是否具备特权执行条件
- 输出 Arch Linux 前置步骤建议
- 输出服务模型建议

说明：

- `/sys/devices/virtual/drm` 当前仅作为信息项，不作为失败门槛；
- 这是因为本机验证中该路径缺失，但不影响 `evdi` 验证成功。

## 已知限制

### 1. 权限要求

当前 `evdi` 路线依赖特权权限。

因此：

- 不适合作为单纯依赖 `systemd --user` 的方案；
- 更适合系统级 root service；
- 后续文档、部署与状态说明已据此调整。

### 2. 支持矩阵仍然有限

本次只验证了：

- Arch Linux
- Wayland
- `niri`
- `evdi-dkms 1.14.15`
- `libevdi 1.14.15`

尚未验证：

- KDE Plasma Wayland
- GNOME Wayland
- Hyprland
- Sway
- Xorg
- 其他内核版本 / 发行版组合

### 3. 仅验证最小抓帧，不代表已具备串流闭环

本阶段只证明：

- 后端可创建显示器；
- 可取得非空 framebuffer；

尚未证明：

- H.264 编码路径稳定；
- WebSocket 传输稳定；
- Android 解码显示稳定。

### 4. 稳定命名仍需后续产品化验证

当前稳定性验证基于：

- 逻辑显示名
- 稳定 hash
- 稳定 EDID 身份生成

但“桌面环境是否将其视为同一历史显示器并复用布局配置”仍需在更多环境中继续观察。

## 风险与后续建议

### 当前剩余风险

1. 兼容性风险仍集中在 compositor / 桌面环境差异。
2. Phase 2 串流闭环引入编码器后，仍可能出现新的时序和格式问题。
3. 系统级服务部署后，配置路径、状态路径和权限边界需要统一。

### 建议的下一步

Phase 1 完成后，建议进入 Phase 2，按路线图继续推进：

1. 固定参数最小串流闭环；
2. 将当前 `evdi` 抓帧结果接入编码器；
3. 实现 WebSocket 单连接传输；
4. 在 Android 客户端接入最小 `MediaCodec` 解码显示；
5. 保持单会话、固定参数、手动连接，先完成第一条完整画面链路。

## 本文档对应的 Phase 1 完成判断

结合本次代码、测试与实测记录，Phase 1 已满足以下完成条件：

- 已确认主显示后端路线；
- 已明确 Arch Linux 前置条件与权限要求；
- 已具备可执行的 `doctor` / `probe` / 验证命令；
- 已能重复完成创建 / 销毁；
- 已能抓取非空帧；
- 已形成仓库内验证记录文档。

因此，本仓库可以将 Phase 1 视为完成，并进入 Phase 2。