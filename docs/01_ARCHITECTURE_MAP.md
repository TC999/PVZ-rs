# 01 · 架构映射：C++ → Rust（以仓库实际结构为准）

> 本表依据当前仓库实际目录生成。AI 不得自行调整模块边界；需要变更先提出，由人批准。

## 1. 目录映射

单 crate，`src/` 与 `cpp/src/` 的对应关系：

| C++（`cpp/src/`） | Rust（`src/`） | 规模（Rust 行数） | 说明 |
|---|---|---|---|
| `SexyAppFramework/`（`SexyAppBase.cpp` `SexyApp.cpp` `Common.cpp`） | `framework/`（`sexy_app_base.rs` `common.rs`） | 2552 / 577 | 应用基类、主循环 |
| `SexyAppFramework/graphics/` | `framework/graphics/` | gl_interface 2258、graphics 1862、image 1938 等 | GL 渲染 |
| `SexyAppFramework/widget/` | `framework/widget/` | ~4,800 | Widget 体系 |
| `SexyAppFramework/paklib/` | `framework/paklib/` | 964 | `.pak` 读取 |
| `SexyAppFramework/sound/` | `framework/sound/` | ~1,100 | SDL_mixer 封装 |
| `SexyAppFramework/misc/ imagelib/` | `framework/*.rs`、`framework/imagelib/` | | MTRand、Rect、XML 等 |
| `SexyAppFramework/glad/ fcaseopen/ platform/` | `ffi/` | 748（opengl）等 | 第三方/平台胶水 |
| `PvzpLib/`（Attachment、Reanimator、Trail、Pvzp*、DataArray…） | `todlib/` | tod_particle 2892、reanimator 1602 等 | **注意：C++ 目录名是 `PvzpLib`，Rust 里叫 `todlib`** |
| `Lawn/`（Board、Plant、Zombie、Challenge、Coin…） | `lawn/` | board 11417、zombie 11272、challenge 6210、plant 5132 | 游戏核心 |
| `Lawn/System/` | `lawn/system/` | save_game 2845 等 | 存档、音乐、玩家信息 |
| `Lawn/Widget/` | `lawn/widget/` | zombatar_widget 1910、store_screen 1592 等 | 各界面 |
| `LawnApp.cpp/.h`、`ConstEnums.h`、`Resources.cpp` | `lawn/lawn_app.rs`、`lawn/game_enums.rs`、`framework/resources.rs` | 3359 / 1878 / 897 | |
| `main.cpp` | `main.rs` | 57 | |
| `android/ ios/ wasm/ archlinux/ CMake/ vcpkg*` | — | — | **不翻译** |

规模总览（来自 `ANALYSIS_REPORT.md`）：C++ 约 194k 行 → Rust 约 98k 行；**差一半不等于缺一半**（改名、注释块、第三方都会影响），但骨架化函数确实大量存在。

## 2. 依赖方向

```
main → lawn → todlib → framework → ffi
              lawn  → framework
```

- `framework` / `todlib` 不得反向依赖 `lawn`。
- 目前 `lawn` 直接调用 `framework` 的渲染对象；后续若要做无渲染回放，需引入渲染 trait（见 `05` ADR-001，**待决定**）。

## 3. 线程与加载（现状）

| 项 | C++ | Rust 现状 |
|---|---|---|
| 资源/子系统加载 | `LoadingThreadProc` 后台线程 + 原子标志 | **同步**，在主线程 `TitleScreen` 首次更新时一次性执行（`title_screen.rs:468` → `lawn_app.rs:3086`） |
| 进度条 | 主线程读原子进度 | 加载期间主线程被占用，**加载完成前不会重绘** |
| GL 调用 | 主线程 | 主线程（同步加载下天然满足） |

详见 `07_BOOT_SEQUENCE.md`。

## 4. 全局状态

现状：约 200 处 `static mut`、`LawnApp` 与各 Widget 通过裸指针互相持有。
**策略见 `05` ADR-003/007**：存量不动，禁止新增 `static mut`。

## 5. 文件级映射与状态

文件级清单（含难度、枚举、类）已由 `TODO.md` 自动生成，但**状态标记可能过期**。真实差异以 `ANALYSIS_REPORT.md` 为准；状态汇总维护在 `04_PROGRESS.md`。
