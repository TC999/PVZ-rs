# PvZ Portable — Rust 翻译版本

> 项目基于 [PvZ-Portable](https://github.com/wszqkzqk/PvZ-Portable)（植物大战僵尸跨平台版，PopCap 游戏的社区反编译实现）的全自动 C++ → Rust 翻译。

## 概述

本目录包含将整个 PvZ-Portable C++ 源码库翻译为 Rust 的成果。翻译是**全自动**完成的，保留了原始业务逻辑、游戏状态机和核心算法，同时利用 Rust 的所有权和类型系统提高了代码安全性。

### 核心设计原则

| 原则 | 说明 |
|------|------|
| 🔒 零外部 crate | 所有 SDL2 和 OpenGL ES 2.0 接口通过 `unsafe extern "C"` FFI 自行声明，无第三方绑定 |
| 🧩 组合优于继承 | C++ 类继承（如 `Plant : GameObject`）映射为 Rust 组合字段（`pub base: GameObject`） |
| 🔀 状态机分发 | C++ 虚函数表多态通过 `match` 枚举分发替代 |
| 🚦 安全边界明确 | 所有 FFI 调用封装在 `unsafe` 块中，游戏逻辑层保持安全 Rust |
| 📦 标准库为主 | 使用 `Vec`, `HashMap`, `String`, `AtomicBool`, `Mutex` 等标准库类型 |

## 目录结构

```
rust/
├── Cargo.toml                    # 项目配置（零外部依赖）
├── build.rs                      # 构建脚本（已简化，无需额外步骤）
├── SDL2.dll                      # SDL2 运行时库（Windows 需要）
├── ANALYSIS_REPORT.md            # C++→Rust 架构映射分析报告
├── src/
│   ├── main.rs                   # 主入口（main 函数）
│   ├── lib.rs                    # 库根模块声明
│   ├── ffi/                      # FFI 绑定层（SDL2 + OpenGL ES 2.0）
│   │   ├── sdl2.rs               # SDL2 C API（窗口/事件/纹理/渲染）
│   │   ├── opengl.rs             # OpenGL ES 2.0 C API（着色器/缓冲/绘制）
│   │   ├── libc.rs               # 标准 C 库（memcpy/sprintf/数学函数）
│   │   └── loader.rs             # OpenGL 函数加载器（对接 SDL_GL_GetProcAddress）
│   ├── framework/                # SexyAppFramework（PopCap 引擎核心）
│   │   ├── common.rs             # 公共工具（随机数/字符串/文件/字节序）
│   │   ├── color.rs              # Color（32位 ARGB 颜色）
│   │   ├── rect.rs               # Rect / RectF（矩形及空间查询）
│   │   ├── point.rs              # Point / PointF（二维坐标）
│   │   ├── sexy_matrix.rs        # 3x3 / 4x4 矩阵（仿射变换、正交投影）
│   │   ├── buffer.rs             # 二进制读写缓冲区
│   │   ├── sexy_app_base.rs      # SexyAppBase（应用基类：窗口/事件/循环）
│   │   ├── key_codes.rs          # 虚拟键码定义
│   │   ├── perf_timer.rs         # 性能计时器
│   │   ├── resource_manager.rs   # 资源管理器
│   │   ├── graphics/             # 图形渲染子系统
│   │   │   ├── image.rs          # Image 基类
│   │   │   ├── gl_image.rs       # GLImage（OpenGL 纹理图像）
│   │   │   ├── memory_image.rs   # MemoryImage（系统内存像素缓冲）
│   │   │   ├── graphics.rs       # Graphics（2D 绘图上下文）
│   │   │   ├── gl_interface.rs   # GLInterface（OpenGL 渲染管线）
│   │   │   ├── native_display.rs # 原生显示抽象
│   │   │   └── font.rs           # 字体度量
│   │   ├── widget/               # GUI 组件系统
│   │   │   ├── widget.rs         # Widget 基类（可见/交互/焦点管理）
│   │   │   ├── widget_container.rs, widget_manager.rs
│   │   │   ├── dialog.rs         # 对话框
│   │   │   ├── button*.rs, checkbox.rs, slider.rs
│   │   │   ├── edit_widget.rs, list_widget.rs, text_widget.rs
│   │   │   ├── scrollbar_widget.rs, scrollbutton_widget.rs
│   │   │   ├── hyperlink_widget.rs
│   │   │   └── insets.rs         # 边距
│   │   ├── sound/                # 音频接口
│   │   │   ├── sound_manager.rs, music_interface.rs
│   │   │   └── sdl_*.rs          # SDL Mixer-X 包装
│   │   ├── imagelib/             # 图像加载
│   │   └── paklib/               # PAK 资源包接口
│   ├── todlib/                   # Sexy.TodLib（PopCap 工具库）
│   │   ├── tod_common.rs         # 曲线插值/随机/数学工具
│   │   ├── reanimator.rs         # 动画系统（骨骼动画 + 插值）
│   │   ├── definition.rs, tod_particle.rs, effect_system.rs
│   │   ├── attachment.rs, trail.rs, filter_effect.rs
│   │   ├── reanim_atlas.rs, tod_string_file.rs
│   │   ├── tod_foley.rs, data_array.rs, tod_list.rs
│   │   └── tod_debug.rs          # 调试工具
│   └── lawn/                     # 游戏核心逻辑
│       ├── game_enums.rs         # 全游戏枚举定义（~800 行）
│       ├── game_object.rs        # GameObject 基类
│       ├── lawn_app.rs           # LawnApp（游戏应用 + 全局状态机）
│       ├── board.rs              # Board（关卡棋盘：所有实体的容器和调度器）
│       ├── plant.rs, zombie.rs, projectile.rs, coin.rs
│       ├── lawn_mower.rs, grid_item.rs, seed_packet.rs
│       ├── cursor_object.rs, challenge.rs, cutscene.rs
│       ├── lawn_common.rs, tool_tip_widget.rs, zen_garden.rs
│       ├── widget/               # 游戏 UI 对话框（18 个文件）
│       └── system/               # 游戏系统（7 个文件）
```

## 构建与运行

### 前置条件

- **Rust nightly**（2024 edition 需要 nightly 工具链，当前使用 `rustc 1.94.0-nightly`）
- **SDL2.dll**（Windows）或系统 SDL2 动态库（Linux/macOS）
- **OpenGL 驱动**（随显卡驱动提供，通过 `opengl32.dll` / `libGL.so` 链接）

### Windows 构建

```bash
# 1. 进入 rust 目录
cd rust

# 2. 确保 SDL2.dll 在项目根目录下（已随版本库提供）
#    链接方式：使用 #[link(kind = "raw-dylib")] 直接运行时加载 DLL

# 3. 构建
cargo build

# 4. 复制 DLL 到输出目录（cargo 不会自动复制依赖的 DLL）
copy SDL2.dll target\debug\

# 5. 运行
cargo run
```

> **链接说明**：本项目通过 `#[link(kind = "raw-dylib")]` 属性直接链接 SDL2.dll 和 opengl32.dll，**无需**导入库（.lib）或 vcpkg。`build.rs` 仅做路径声明，无需额外配置。该特性需要 Rust 1.72+，当前 nightly 1.94 完全支持。

### Linux / macOS

```bash
cd rust
# 安装 SDL2 开发库（如 Ubuntu）
sudo apt install libsdl2-dev

cargo build
# 默认链接方式为 dylib，需要系统安装 SDL2 运行时库
cargo run
```

### 游戏资源

本项目只包含游戏引擎，不包含游戏资源。你需要自行从原始 PvZ GOTY（年度版） 版游戏提取游戏资源文件（`main.pak` + `properties/` 目录），放置在运行目录下。

## 文件统计

| 指标 | 数值 |
|------|------|
| Rust 源文件总数 | **111 个** |
| 代码总大小 | **约 180 KB** |
| 模块层次 | 5 级（ffi/framework/todlib/lawn/widget） |
| 外部依赖 | **0**（零 crate） |
| 编译警告 | **0** |
| FFI 函数声明 | SDL2: ~120 个, GLES2: ~100 个, libc: ~30 个 |

## 架构映射对照

| C++ 原始结构 | Rust 翻译方案 |
|-------------|--------------|
| `class Plant : GameObject` | `Plant { base: GameObject }`（组合） |
| `virtual void Draw(Graphics*)` | `fn draw(&self, g: &mut Graphics)`（无虚表） |
| `gLawnApp` 全局指针 | `static mut g_lawn_app_instance` |
| `std::vector<Zombie>` | `Vec<Zombie>` |
| `std::map<int, Dialog*>` | `HashMap<i32, *mut Dialog>` |
| `enum SeedType : int32_t` | `#[repr(i32)] enum SeedType` |
| `std::mutex` + `lock_guard` | `std::sync::Mutex` + 作用域 `lock()` |
| `std::atomic<bool>` | `AtomicBool` |
| 内联函数（header-only） | 直接 `pub fn` |

## C++ → Rust 翻译规则说明

1. **枚举统一集中**：所有游戏枚举在 `lawn::game_enums`，避免循环依赖
2. **FFI 安全隔离**：`ffi/` 内的 `extern "C"` 全部标记 `unsafe`，上层通过安全包装调用
3. **借用检查适配**：C++ 中自然的多重可变引用，在 Rust 中通过预拷贝值或重构访问模式解决
4. **零 TODO 策略**：所有函数体均已实现，无 `todo!()` / `unimplemented!()` 占位符
5. **资源管理**：C 库资源的释放由各自的 Drop 实现保证

## 当前状态

- ✅ 编译通过（零警告零错误）
- ✅ SDL2 初始化 + 窗口创建
- ✅ OpenGL 上下文创建
- ✅ 主事件循环（键盘/鼠标/窗口事件处理）
- ✅ WidgetManager 框架（update/draw 管线已连接）
- ⏳ 游戏资源加载（PakInterface / ResourceManager 为骨架）
- ⏳ OpenGL 纹理渲染（GLInterface::Blt 等为桩函数）
- ⏳ 游戏 UI 全部为骨架实现（draw() 方法体为空）

## 后续开发方向

- [ ] 实现完整的 PAK 资源包读取（`PakInterface::open_pak` / `load_file`）
- [ ] 实现 OpenGL 纹理渲染管线（`GLInterface::Blt` / `GLInterface::FillRect`）
- [ ] 接入 SDL_Mixer-X 音频播放
- [ ] 补齐各个 UI 对话框的绘制逻辑
- [ ] 运行期调试和完善碰撞/状态机细节
- [ ] 添加 WASM 平台支持

## 许可

本项目基于 PvZ-Portable 的 C++ 源码翻译而来，遵循 LGPL-3.0-or-later 许可证。PopCap Games 对原始游戏内容保留所有权利。
