# 05 · 设计决策记录（ADR）

> 规则
> 1. 状态为 **已决定** 的，AI 必须遵守，不得另起方案。
> 2. 状态为 **待决定** 的，AI 遇到相关问题必须停下来问人，**不得自行选择**。
> 3. 新决策由人批准后才能写入；AI 可以起草"选项与建议"。

## ADR 模板

```
### ADR-NNN 标题
- 状态：待决定 / 已决定 / 已废弃（被 ADR-MMM 取代）
- 背景：
- 选项：A … / B … / C …
- 结论：
- 理由：
- 影响范围：（哪些 crate / 文件）
- 需要返工的已有代码：
```

---

### ADR-001 渲染后端
- 状态：**待决定**【待核对：Rust 仓库当前实际用的是什么】
- 背景：C++ 版使用 SDL2 + OpenGL ES 2.0（桌面 GL 2.1 回退）。
- 选项：A. `sdl2` + `glow`（最贴近原实现）／B. `winit` + `wgpu`／C. 其他
- 约束：`pvz-core` 不得依赖渲染库；渲染通过 trait 隔离。
- 结论：待定

### ADR-002 音频与 MO3 音乐
- 状态：**待决定**
- 背景：C++ 版使用 SDL Mixer X 的分支并通过 libopenmpt 支持 MO3。
- 选项：A. 绑定 libopenmpt + 通用音频输出／B. 先不实现音乐，仅音效／C. 其他
- 结论：待定

### ADR-003 对象引用方式与 unsafe 策略
- 状态：**建议，待你确认**
- 现状：约 2700 处 `unsafe`、1265 处裸指针；对象互相以裸指针持有（C++ 同构）。
- 建议：**阶段 1（行为等价前）保持现状**，不做大重构；禁止新增 `static mut`；新增 `unsafe` 带 `// SAFETY:`；释放走延迟队列。**阶段 2（行为等价达成后）**再逐步把裸指针换成 ID/借用。
- 理由：现在同时翻译与重构，会让回放对照无法定位问题。
- 影响：所有 `lawn/`、`framework/`。

### ADR-004 数学函数与浮点确定性
- 状态：**待决定**
- 背景：`sinf/cosf/sqrtf/atan2f` 等在不同平台 libm 上结果可能有最后一位差异；回放哈希对这类差异敏感。
- 选项：A. 使用 `std` 提供的 `f32` 方法并接受平台差异／B. 引入 `libm` crate 保证跨平台一致／C. 逐函数核对 C++ 实际使用的实现
- 要求：决定前，所有调用点用统一封装 `sexy_core::math::{sin, cos, …}`，方便后续替换。
- 结论：待定

### ADR-005 Bug 修复策略
- 状态：**已决定**
- 结论：默认**完全保持原版行为**（包括怪异行为）。C++ 的 `DO_FIX_BUGS` 对应 Cargo feature `fix_bugs`，每一条修复独立成块、用 `#[cfg(feature = "fix_bugs")]` 包裹，默认关闭。
- 理由：保证 `.dmo` 回放与 C++（默认配置）一致。

### ADR-006 加载模型与线程
- 状态：**待决定**（建议方案 1）
- 现状：同步加载，占住主线程（见 `07`）。
- 选项：方案 1 主线程分片加载 / 方案 2 后台线程（CPU）+ 主线程上传（GL）
- 约束：GL 调用只在主线程；任何线程 panic 必须使进程可见退出；差异登记 TRANSLATION_NOTE。

### ADR-007 全局状态
- 状态：**已决定（保持现状，禁止新增）**
- 结论：存量约 217 处 `static mut` 保持；新增一律禁止，改为挂在 `LawnApp` 或显式参数上。

### ADR-008 UI/Widget 体系的表达方式
- 状态：**待决定**
- 背景：SexyAppFramework 的 `Widget / WidgetManager / Dialog` 基于继承与虚函数、父子互相引用。
- 选项：A. `trait Widget` + 由 `WidgetManager` 持有的 `Box<dyn Widget>` + ID 引用／B. 枚举封闭集合／C. 其他
- 结论：待定

### ADR-009 存档兼容范围
- 状态：**待决定**
- 背景：`user*.dat` 与 `.v4` 需互通；旧版 `game*.dat` 是原始内存转储；`cache32/cache64` 是本地启动缓存。
- 建议（待确认）：`user*.dat`、`.v4` 完整兼容；旧 `.dat` 只做读取迁移；缓存文件不兼容，不模仿格式。
- 结论：待定

### ADR-010 Widget 延迟释放
- 状态：**已决定**
- 结论：Widget 的释放必须等价于 C++ `SafeDeleteWidget`：先从 WidgetManager 摘除，放入延迟释放列表，在主循环的安全点统一释放。**禁止在 Widget 自身或其所有者的方法执行期间 `Box::from_raw`。**
- 需要返工：`lawn_app.rs::loading_completed`（TitleScreen）及其他类似调用点（需逐一排查 `Box::from_raw`）。

### ADR-011 许可证
- 状态：**待核对**
- 事实：上游 C++ 为 LGPL-3.0-or-later，本仓库 README 声明 GPL-3.0；`SexyAppFramework` 另含 PopCap Games Framework License。
- 要求：保留各文件原有版权/许可声明，`SexyAppFramework` 派生文件保留其许可文本。
