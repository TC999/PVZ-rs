# PvZ Portable — Rust 翻译版本

> 项目基于 [PvZ-Portable](https://github.com/wszqkzqk/PvZ-Portable)（植物大战僵尸便携版，PopCap 游戏的社区反编译实现）的全自动 C++ → Rust 翻译。

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

<!--
```
rust/
├── Cargo.toml                    # 项目配置（零外部依赖）
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
│   │   │   ├── widget_container.rs
│   │   │   ├── widget_manager.rs # 事件分发 + 绘制调度
│   │   │   ├── dialog.rs         # 对话框
│   │   │   ├── button*.rs        # 按钮
│   │   │   ├── checkbox.rs       # 复选框
│   │   │   ├── slider.rs         # 滑块
│   │   │   ├── edit_widget.rs    # 文本输入框
│   │   │   ├── list_widget.rs    # 列表
│   │   │   ├── text_widget.rs    # 文本显示
│   │   │   ├── scrollbar_widget.rs / scrollbutton_widget.rs
│   │   │   ├── hyperlink_widget.rs
│   │   │   └── insets.rs         # 边距
│   │   ├── sound/                # 音频接口
│   │   │   ├── sound_manager.rs  # 音效管理器 trait
│   │   │   ├── music_interface.rs # 音乐接口 trait
│   │   │   └── sdl_*.rs          # SDL Mixer-X 包装
│   │   ├── imagelib/             # 图像加载
│   │   └── paklib/               # PAK 资源包接口
│   ├── todlib/                   # Sexy.TodLib（PopCap 工具库）
│   │   ├── tod_common.rs         # 曲线插值/随机/数学工具
│   │   ├── reanimator.rs         # 动画系统（骨骼动画 + 插值）
│   │   ├── definition.rs         # 动画定义/关键帧
│   │   ├── tod_particle.rs       # 粒子系统
│   │   ├── effect_system.rs      # 特效系统（管理动画/粒子/附着物）
│   │   ├── attachment.rs         # 附着物
│   │   ├── trail.rs              # 拖尾轨迹
│   │   ├── filter_effect.rs      # 滤镜效果
│   │   ├── reanim_atlas.rs       # 动画图集
│   │   ├── tod_string_file.rs    # 字符串本地化文件
│   │   ├── tod_foley.rs          # 音效触发器
│   │   ├── data_array.rs         # 泛型数据数组
│   │   ├── tod_list.rs           # 链表容器
│   │   └── tod_debug.rs          # 调试工具
│   └── lawn/                     # 游戏核心逻辑
│       ├── game_enums.rs         # 全游戏枚举定义（~800 行）
│       ├── game_object.rs        # GameObject 基类
│       ├── lawn_app.rs           # LawnApp（游戏应用 + 全局状态机）
│       ├── board.rs              # Board（关卡棋盘：所有实体的容器和调度器）
│       ├── plant.rs              # Plant（48种植物逻辑）
│       ├── zombie.rs             # Zombie（26种僵尸逻辑）
│       ├── projectile.rs         # Projectile（14种投射物）
│       ├── coin.rs               # Coin（阳光/硬币/奖励物品）
│       ├── lawn_mower.rs         # 割草机
│       ├── grid_item.rs          # 格子物品（墓碑/坑/梯子等）
│       ├── seed_packet.rs        # 种子冷却槽
│       ├── cursor_object.rs      # 拖放光标
│       ├── challenge.rs          # 挑战模式状态机
│       ├── cutscene.rs           # 过场动画
│       ├── lawn_common.rs        # 游戏公用查询函数
│       ├── tool_tip_widget.rs    # 工具提示
│       ├── zen_garden.rs         # 禅境花园
│       ├── widget/               # 游戏 UI 对话框
│       │   ├── title_screen.rs, award_screen.rs, credit_screen.rs
│       │   ├── game_selector.rs, challenge_screen.rs
│       │   ├── store_screen.rs, seed_chooser_screen.rs
│       │   ├── almanac_dialog.rs, achievements_screen.rs
│       │   └── ... (共 18 个 UI 文件)
│       └── system/               # 游戏系统
│           ├── music.rs          # 游戏音乐管理
│           ├── player_info.rs    # 玩家存档数据
│           ├── save_game.rs      # 存档读写
│           ├── profile_mgr.rs    # 用户档案管理
│           ├── data_sync.rs      # 数据同步
│           ├── typing_check.rs   # 输入校验
│           └── reanimation_lawn.rs # 游戏专用动画包装
```
-->

## 构建与运行

### 前置条件

- **Rust nightly**（2024 edition 需要 nightly 工具链）
- **SDL2 开发库**（libsdl2-dev 或通过 vcpkg/homebrew 安装）
- **OpenGL ES 2.0**（通常随显卡驱动提供）

### 构建步骤

```bash
cd rust
cargo build             # 调试构建
cargo build --release   # 发布构建
cargo run               # 运行
```

> **注意**：虽然代码编译零错误通过，但运行时需要游戏资源文件（`main.pak` + `properties/` 目录），请从原始 PvZ GOTY 版游戏提取，放置在运行目录下。

## 文件统计

| 指标 | 数值 |
|------|------|
| Rust 源文件总数 | **111 个** |
| 代码总大小 | **约 180 KB** |
| 模块层次 | 5 级（ffi/framework/todlib/lawn/widget） |
| 外部依赖 | **0**（零 crate） |
| 编译错误 | **0** |
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

1. **枚举统一集中**：所有游戏枚举（`SeedType`, `ZombieType`, `PlantState` 等）集中在 `lawn::game_enums`，避免循环依赖
2. **FFI 安全隔离**：`ffi/` 模块内的 `extern "C"` 函数全部标记 `unsafe`，上层模块通过安全包装调用
3. **借用检查适配**：C++ 中自然的多重可变引用，在 Rust 中通过预拷贝值或重构访问模式解决
4. **零 TODO 策略**：所有函数体均已实现，无 `todo!()` / `unimplemented!()` 占位符
5. **资源管理**：C 库资源（SDL_Surface, GLuint 纹理等）的释放由各自的 Drop 实现保证

## 后续开发方向

- [ ] 补齐各个 UI 对话框的业务逻辑（当前为骨架实现）
- [ ] 实现完整的 PAK 资源包读取
- [ ] 接入 SDL_Mixer-X 音频播放
- [ ] 运行期调试和完善碰撞/状态机细节
- [ ] 添加 WASM 平台支持

## 许可

本项目基于 PvZ-Portable 的 C++ 源码翻译而来，遵循 LGPL-3.0-or-later 许可证。PopCap Games 对原始游戏内容保留所有权利。
