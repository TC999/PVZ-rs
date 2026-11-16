# 07 · 启动与加载序列（依据当前代码）

> 症状：能显示标题加载界面，之后卡死。README 自述"标题界面加载（有 BUG）"。
> **说明：** 没有 `main.pak` 无法运行，以下结论来自静态阅读代码，标注"待验证"的需要你实际运行确认。

## 1. 实际启动流程

| 步骤 | 内容 | 位置 | 线程 |
|---|---|---|---|
| 1 | `init_lawn_string_formats`、`LawnApp::new`、命令行解析 | `main.rs:20-44` | 主 |
| 2 | `app.init()`：SDL、窗口、OpenGL | `main.rs:47` | 主 |
| 3 | `app.start()`：主循环（阻塞） | `main.rs:50` | 主 |
| 4 | `TitleScreen`：`WaitingForFirstDraw`，等第一次绘制 | `title_screen.rs:~460` | 主 |
| 5 | 第一次更新：`music_title_screen_init()` → **`start_loading_thread()`** | `title_screen.rs:463-468` | 主 |
| 6 | `start_loading_thread()`：**同步**加载 LoaderBar 图 → 属性 XML → LoadingImages / Fonts → 子系统初始化 → LoadingSounds → 置 `m_loading_thread_completed = true` | `lawn_app.rs:3086-3236` | **主** |
| 7 | 返回后 `loader_screen_is_loaded = true`，进入 `PopCapLogo` 等状态，读进度条 | `title_screen.rs:469-` | 主 |
| 8 | 检测到完成 → 启用 Start 按钮 | `title_screen.rs:625-632` | 主 |
| 9 | 点击 Start → `loading_completed()` → 移除 TitleScreen、显示 GameSelector | `title_screen.rs:166-172`、`lawn_app.rs:3059-3081` | 主 |

C++ 里第 6 步在**后台线程**，主线程继续跑主循环、重绘进度条；Rust 把它变成了**主线程里的一次长调用**。

## 2. 关键发现（按可能性排序）

### A. 同步加载占住主线程 → 窗口"卡死"（高度可疑，待验证）

第 5 步之后、第 7 步之前，主线程没有处理事件，也没有重绘。所以：

- 用户看到的是"标题加载界面静止不动"，系统可能显示"无响应"；
- 进度条在加载期间**不会推进**（进度只在加载结束后才被读取）；
- 加载越慢，越像卡死。

加载很慢的一个常见原因：`Cargo.toml` 没有 `[profile.dev]` 设置，**debug 构建下 `image` 解码数百张图片极慢**。快速验证：

```
cargo run --release
```

若 release 下能走完，问题主要是性能/同步加载；若仍然卡住，看下一节的日志定位。

建议同时在 `Cargo.toml` 加：

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

### B. 用日志判断卡在哪里

代码里已有这些日志（`eprintln!`），**最后一条出现的是哪一条，就说明卡在它之后**：

| 最后一条日志 | 含义 / 下一步 |
|---|---|
| `[LawnApp] 开始加载资源（同步模式）` | 卡在 LoaderBar 图片加载或总任务数统计；检查 `load_single_image`、`get_num_resources_image` |
| `[LawnApp] LoaderBar 资源加载完成，继续加载更多资源` | 卡在 `load_properties`、LoadingImages/Fonts、子系统初始化（粒子定义加载 `tod_particle_load_definitions` 等） |
| `[LawnApp] 资源加载完成` | 加载本身通过；问题在后续状态机，见 C、D |
| `[LawnApp] LoadingCompleted — 移除 TitleScreen，显示 GameSelector` | 卡在 `show_game_selector` 或 GameSelector 首次更新/绘制 |
| `[LawnApp] 已显示 GameSelector` | 之后的卡死在 GameSelector |

**请把你运行时最后几行日志发给我，就能直接定位。**

### C. `loading_completed()` 可能释放了正在执行的对象（use-after-free，待验证）

`LawnApp::loading_completed()` 中：

```rust
if let Some(ts) = self.title_screen.take() {
    …(*wm).remove_widget(ts);
    let _ = Box::from_raw(ts);   // 立即释放 TitleScreen
}
```

但它在 TitleScreen **自己的方法里**被调用（`button_depress`、`update` 中的 quick-load 分支），调用栈上的 `self` 属于刚被释放的对象。返回后任何对 `self` 的访问都是悬空访问，表现可能是崩溃、卡死或随机行为。

C++ 对应物使用 `SafeDeleteWidget`（延迟到安全点释放）。**Rust 应等价实现延迟释放队列**（见 `03` §3）。

### D. 其他排查点

- `load_resources` 只加载图像，声音走另一条路径；`SoundManager::load_sound` 失败时不会计入进度，但不会死循环。
- `paklib/mod.rs` 与 `resource_manager.rs` 里有 `loop {}`（paklib 370/620/634/729，resource_manager 164/180/344）：若解析游标不前进（BOM、换行、EOF）会死循环，CPU 会 100%。
- 字符串表（LawnStrings）目前**未接入**（`start_loading_thread` 注释），之后依赖字符串的界面可能异常。
- `m_loading_thread_tasks_total` 含固定常量（636、`3500*2`）；进度只做显示，不影响完成判定，但与 C++ 数值是否一致未核对。

## 3. 修复方案（待你确认，见 ADR-006）

**方案 1（推荐）：主线程分片加载。**
把"一次性加载"改成"每帧加载一小批"：

- 把加载拆成任务队列（资源组 → 单个资源 → 子系统初始化步骤）；
- 每帧在时间预算内（例如 8~10 ms）处理若干任务，然后返回主循环，让事件与重绘继续；
- 进度计数在每个任务后更新，进度条自然动起来；
- GL 调用仍全在主线程，不引入线程同步问题；
- 与 C++ 的差别要在 `TRANSLATION_NOTE` 和 `04` 中登记。

**方案 2：真线程。** 后台线程只做 CPU 工作（读 pak、解码、解析），产出数据，主线程负责纹理上传；用原子进度 + channel 通信；线程入口包 panic hook。改动大、风险高，建议方案 1 稳定之后再考虑。

无论哪种方案，第一步都是**加 panic hook 和进度日志**：

```rust
std::panic::set_hook(Box::new(|info| {
    eprintln!("PANIC: {info}\n{}", std::backtrace::Backtrace::force_capture());
    std::process::exit(1);
}));
```

## 4. 里程碑验收

| 编号 | 验收内容 | 输出 |
|---|---|---|
| L1 | `main.pak` 打开，文件表读取 | 条目数（与 C++ 对照） |
| L2 | `resources.xml` 解析 | 资源组数、各组资源数 |
| L3 | 各资源组加载完成 | 每组耗时、条目数 |
| L4 | 进度 100%，完成标志被标题屏观察到 | 日志 |
| L5 | 点击 Start 后进入 GameSelector，且不崩溃 | 日志 + 截图 |
| L6 | 加载期间窗口持续响应（可拖动）、进度条在动 | 截图/录屏 |

**AI 每完成一个里程碑，必须贴出实际运行输出。**没法运行时，必须明说"未运行验证"。

## 5. 汇报要求

涉及启动/加载的任务，汇报必须包含：达到的里程碑、最后一条日志、已排除与未排除的原因、是否在 release 与 debug 下都验证过。
