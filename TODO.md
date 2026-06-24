# 🌱 PvZ Portable C++ → Rust 翻译 — 详尽任务清单

> 生成日期：2026-06-18
> 范围：`src/Lawn/` + `src/Sexy.TodLib/` + `src/SexyAppFramework/`
> 进度表示：`[ ]` = 未开始，`[~]` = 部分完成，`[✓]` = 已翻译（结构体骨架），`[✗]` = 需重写

---

## 一、SexyAppFramework（框架层）

### 1.1 图形系统

<!-- ==================================================== -->
<!-- 1.1.1 Graphics                                               -->
<!-- ==================================================== -->
- [ ] **1.1.1 Graphics 图形绘制上下文**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/Graphics.{h,cpp}`（43KB）
  - **核心依赖项：** `Color`, `Rect`, `Image`, `Font`, `SexyMatrix3`
  - **Rust 实现要点：**
    - 翻译所有绘制方法：`DrawImage`, `DrawString`, `FillRect`, `DrawRect`, `DrawLine`, `SetClipRect`, `PushState`/`PopState`
    - 颜色混合模式（`DrawMode` 枚举）
    - 已有 `graphics.rs`（130 行骨架），需填充全部方法体
    - 注意 `Graphics` 持有对 `Image` 内部缓冲区的引用——用借用检查器确保生命周期安全

<!-- ==================================================== -->
<!-- 1.1.2 Image / MemoryImage / GLImage                      -->
<!-- ==================================================== -->
- [ ] **1.1.2 Image（图像基类）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/Image.{h,cpp}`（5KB + 6KB）
  - **核心依赖项：** `MemoryImage`, `Color`
  - **Rust 实现要点：**
    - 已有 `image.rs`（61 行骨架），需实现像素数据管理
    - 用 `Vec<u8>` 存储像素缓冲，暴露 `width`/`height`/`GetPixel`/`SetPixel`

- [ ] **1.1.3 MemoryImage（内存图像）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/MemoryImage.{h,cpp}`（6KB + 45KB）
  - **核心依赖项：** `Image`, `Graphics`, `DDImage`
  - **Rust 实现要点：**
    - 已有 `memory_image.rs`（111 行骨架）
    - 实现 `SetImageData`, `Blt`, `StretchBlt` 等方法
    - 像素格式支持：RGBA8888, RGB565, RGB555
    - 需要 `unsafe` 操作像素内存，封装为 safe Rust 方法

- [ ] **1.1.4 GLImage（OpenGL 纹理图像）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/GLImage.{h,cpp}`（3KB + 7KB）
  - **核心依赖项：** `Image`, `MemoryImage`, OpenGL
  - **Rust 实现要点：**
    - 已有 `gl_image.rs`（73 行骨架）
    - 管理 OpenGL 纹理 ID 的生命周期（`Drop` trait 释放纹理）
    - 实现 `UploadToGPU`, `Bind` 等方法

- [ ] **1.1.5 Font（字体基类）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/Font.{h,cpp}`（2KB + 1KB）
  - **核心依赖项：** `Image`, `Graphics`
  - **Rust 实现要点：**
    - 已有 `font.rs`（51 行骨架）
    - 实现字符尺寸度量、字符串宽度计算

- [ ] **1.1.6 ImageFont（图像字体）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/ImageFont.{h,cpp}`（5KB + 46KB）
  - **核心依赖项：** `Font`, `Image`, `XMLParser`, `Color`
  - **Rust 实现要点：**
    - 从 XML 解析字体定义（字符映射、字间距）
    - 实现 `DrawString`——从图集中切割字符子图并渲染
    - 大量字符布局逻辑

- [ ] **1.1.7 NativeDisplay（原生显示）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/NativeDisplay.{h,cpp}`（1.5KB + 1.6KB）
  - **核心依赖项：** `Graphics`
  - **Rust 实现要点：**
    - 已有 `native_display.rs`（30 行骨架）
    - 简单的屏幕清空和交换链管理包装

- [ ] **1.1.8 GLInterface（OpenGL 接口层）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/GLInterface.{h,cpp}`（8KB + 45KB）
  - **核心依赖项：** `SDL2`, `OpenGL`, `MemoryImage`, `Graphics`
  - **Rust 实现要点：**
    - 已有 `gl_interface.rs`（2238 行，已大量翻译）
    - 检查翻译完整性：`Set3D`, `DrawTriangle`, `PresentFrame` 等方法体
    - 包装 OpenGL 状态机调用

- [ ] **1.1.9 SWTri（软件三角形渲染器）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/SWTri.{h,cpp}`（31KB + 31KB）
  - **核心依赖项：** `Image`, `Graphics`, `SexyMatrix3`
  - **Rust 实现要点：**
    - 软件纹理三角形渲染管线
    - 大量像素格式变体函数（~40 个 `TodDrawTriangle_*` 函数）
    - 需要 `unsafe` 操作像素缓冲区，性能敏感
    - 建议用泛型 + trait 消除大量重复，或用宏生成

- [ ] **1.1.10 Color（颜色）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/Color.{h,cpp}`（1.8KB + 3KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 已有 `color.rs`（81 行），基本完成
    - 注意 RGBA 顺序和 `u32` 打包/解包

- [ ] **1.1.11 SharedImage（共享图像）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/SharedImage.{h,cpp}`（2KB + 3KB）
  - **核心依赖项：** `Image`, `ResourceManager`
  - **Rust 实现要点：**
    - 引用计数图像管理，可用 `Rc<RefCell<Image>>` 或 `Arc<Mutex<Image>>` 替代

- [ ] **1.1.12 Quantize（颜色量化）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/Quantize.{h,cpp}`（1KB + 2KB）
  - **核心依赖项：** `Image`
  - **Rust 实现要点：**
    - 颜色量化算法（中值切割），用于将图像转为 256 色

- [ ] **1.1.13 TriVertex（三角形顶点结构）**
  - **C++ 源文件：** `src/SexyAppFramework/graphics/TriVertex.h`（1.7KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 简单的顶点结构体定义，已在 `gl_interface.rs` 中部分实现

### 1.2 Widget 控件系统

- [ ] **1.2.1 Widget（控件基类）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/Widget.{h,cpp}`（5KB + 12KB）
  - **核心依赖项：** `Graphics`, `WidgetManager`, `Rect`
  - **Rust 实现要点：**
    - 已有 `widget.rs`（139 行骨架）
    - 实现事件方法：`Update`, `Draw`, `MouseDown`, `MouseUp`, `KeyDown`, `KeyChar`
    - 控件树管理：`AddWidget`, `RemoveWidget`, 父子关系
    - 焦点管理、可见性、可用性

- [ ] **1.2.2 WidgetContainer（控件容器）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/WidgetContainer.{h,cpp}`（3KB + 15KB）
  - **核心依赖项：** `Widget`, `Graphics`, `WidgetManager`
  - **Rust 实现要点：**
    - 已有 `widget_container.rs`（80 行骨架）
    - 实现子控件的更新/绘制分发、事件传递
    - Z 顺序排序渲染

- [ ] **1.2.3 WidgetManager（控件管理器）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/WidgetManager.{h,cpp}`（4KB + 18KB）
  - **核心依赖项：** `Widget`, `WidgetContainer`, `Graphics`, `Image`
  - **Rust 实现要点：**
    - 已有 `widget_manager.rs`（150 行骨架）
    - 全局事件循环调度：鼠标、键盘、定时器
    - 控件叠加层管理、拖拽处理、弹出菜单
    - 每个 `WidgetManager` 持有 `Graphics` 引用

- [ ] **1.2.4 ButtonWidget（按钮控件）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/ButtonWidget.{h,cpp}`（2KB + 8KB）
  - **核心依赖项：** `Widget`, `ButtonListener`, `Image`
  - **Rust 实现要点：**
    - 已有 `button_widget.rs`（17 行骨架）
    - 实现按钮状态（Normal/Hover/Pressed/Disabled），事件回调
    - 按钮图片可能为 3~4 帧动画

- [ ] **1.2.5 Dialog（对话框）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/Dialog.{h,cpp}`（4KB + 12KB）
  - **核心依赖项：** `Widget`, `DialogListener`, `ButtonWidget`, `Image`
  - **Rust 实现要点：**
    - 已有 `dialog.rs`（17 行骨架）
    - 对话框布局、按钮添加、模态行为

- [ ] **1.2.6 DialogButton（对话框按钮）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/DialogButton.{h,cpp}`（1KB + 3KB）
  - **核心依赖项：** `ButtonWidget`
  - **Rust 实现要点：**
    - 已有 骨架（17 行）
    - 简单的带文字按钮

- [ ] **1.2.7 EditWidget（文本输入框）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/EditWidget.{h,cpp}`（3KB + 17KB）
  - **核心依赖项：** `Widget`, `EditListener`, `Font`
  - **Rust 实现要点：**
    - 已有 `edit_widget.rs`（17 行骨架）
    - 文本光标、选择、IME 输入支持
    - 键盘事件处理、剪贴板

- [ ] **1.2.8 ListWidget（列表控件）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/ListWidget.{h,cpp}`（3KB + 12KB）
  - **核心依赖项：** `Widget`, `ListListener`, `ScrollbarWidget`, `Font`
  - **Rust 实现要点：**
    - 已有 `list_widget.rs`（17 行骨架）
    - 实现虚拟滚动、列表项绘制、选择

- [ ] **1.2.9 ScrollbarWidget（滚动条）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/ScrollbarWidget.{h,cpp}`（3KB + 9KB）
  - **核心依赖项：** `Widget`, `ScrollListener`, `ScrollbuttonWidget`
  - **Rust 实现要点：**
    - 已有 `scrollbar_widget.rs`（17 行骨架）
    - 滑块拖拽、滚动位置计算

- [ ] **1.2.10 ScrollbuttonWidget（滚动按钮）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/ScrollbuttonWidget.{h,cpp}`（1KB + 2KB）
  - **核心依赖项：** `ButtonWidget`
  - **Rust 实现要点：** 简单按钮子类

- [ ] **1.2.11 Checkbox（复选框）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/Checkbox.{h,cpp}`（1KB + 2KB）
  - **核心依赖项：** `Widget`, `CheckboxListener`, `Image`
  - **Rust 实现要点：**
    - 已有 `checkbox.rs`（17 行骨架）
    - 选中/未选中状态

- [ ] **1.2.12 Slider（滑动条）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/Slider.{h,cpp}`（1KB + 5KB）
  - **核心依赖项：** `Widget`, `SliderListener`, `Image`
  - **Rust 实现要点：**
    - 已有 `slider.rs`（17 行骨架）
    - 拖拽值计算（0.0 ~ 1.0）

- [ ] **1.2.13 TextWidget（文本控件）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/TextWidget.{h,cpp}`（2KB + 10KB）
  - **核心依赖项：** `Widget`, `Font`, `Color`
  - **Rust 实现要点：**
    - 已有 `text_widget.rs`（17 行骨架）
    - 文本自动换行、对齐、滚动

- [ ] **1.2.14 HyperlinkWidget（超链接）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/HyperlinkWidget.{h,cpp}`（1KB + 2KB）
  - **核心依赖项：** `Widget`
  - **Rust 实现要点：** 已有骨架，点击打开 URL

- [ ] **1.2.15 Insets（边距）**
  - **C++ 源文件：** `src/SexyAppFramework/widget/Insets.{h,cpp}`（1KB + 1KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：** 已有 `insets.rs`（28 行），基本完成

- [ ] **1.2.16 ButtonListener / CheckboxListener / DialogListener / EditListener / ListListener / ScrollListener / SliderListener（监听器接口）**
  - **C++ 源文件：** 各自的 `.h` 文件（~1.2KB 每个）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 用 trait 替代 C++ 虚接口
    - 已有大部分骨架（各 17 行）

### 1.3 杂项（Misc）

- [ ] **1.3.1 SexyAppBase（应用基类）**
  - **C++ 源文件：** `src/SexyAppFramework/SexyAppBase.{h,cpp}`（19KB + 99KB）
  - **核心依赖项：** `WidgetManager`, `Graphics`, `Image`, `SoundManager`, `ResourceManager`, `PakInterface`
  - **Rust 实现要点：**
    - 已有 `sexy_app_base.rs`（569 行骨架）
    - 主循环（`UpdateMain`, `DrawMain`, `DoMainLoop`）
    - 窗口创建、初始化、事件处理
    - 资源管理器、声音管理器、存档路径管理
    - 这是整个框架的核心，约 100KB 的 C++ 代码
    - 需要 `Arc<Mutex<>>` 管理全局状态

- [ ] **1.3.2 ResourceManager（资源管理器）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/ResourceManager.{h,cpp}`（6KB + 31KB）
  - **核心依赖项：** `Image`, `Font`, `SoundManager`, `PakInterface`, `XMLParser`
  - **Rust 实现要点：**
    - 已有 `resource_manager.rs`（26 行骨架）
    - 从 `resources.xml`/`pak` 加载图像、字体、声音等资源
    - 资源缓存与引用管理

- [ ] **1.3.3 SexyMatrix（矩阵运算）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/SexyMatrix.{h,cpp}`（3KB + 9KB）
  - **核心依赖项：** `Point`, `Rect`
  - **Rust 实现要点：**
    - 已有 `sexy_matrix.rs`（163 行），基本完成
    - 2D 变换矩阵（平移、旋转、缩放）

- [ ] **1.3.4 Buffer（缓冲）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/Buffer.{h,cpp}`（2KB + 12KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 已有 `buffer.rs`（198 行）
    - 字节缓冲读写，大/小端序支持

- [ ] **1.3.5 Point / Rect（几何结构）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/Point.h`（2KB）, `misc/Rect.h`（3KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 已有 `point.rs`（89 行）, `rect.rs`（155 行），基本完成
    - 矩形碰撞检测、包含关系等

- [ ] **1.3.6 KeyCodes（键码）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/KeyCodes.{h,cpp}`（4KB + 7KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 已有 `key_codes.rs`（105 行），基本完成
    - 键码枚举映射

- [ ] **1.3.7 Common（通用工具函数）**
  - **C++ 源文件：** `src/SexyAppFramework/Common.{h,cpp}`（13KB + 14KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 已有 `common.rs`（221 行骨架）
    - 包含三角函数查找表、字符串工具、随机数、文件 I/O 等杂项
    - `MTRand`（梅森旋转随机数生成器）需要单独翻译

- [ ] **1.3.8 MTRand（随机数）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/MTRand.{h,cpp}`（0.8KB + 5KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 移植梅森旋转算法，确保种子序列与 C++ 一致（游戏需要确定性的关卡生成）

- [ ] **1.3.9 PerfTimer（性能计时器）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/PerfTimer.{h,cpp}`（4KB + 10KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 已有 `perf_timer.rs`（51 行骨架）
    - 高精度计时器包装

- [ ] **1.3.10 Debug / TodDebug（调试工具）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/Debug.{h,cpp}`（1KB + 6KB）, `src/Sexy.TodLib/TodDebug.{h,cpp}`（2KB + 5KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 断言宏、日志输出
    - 已有 `tod_debug.rs`（17 行骨架）

- [ ] **1.3.11 XMLParser（XML 解析器）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/XMLParser.{h,cpp}`（3KB + 18KB）
  - **核心依赖项：** `Buffer`
  - **Rust 实现要点：**
    - 解析 Ressources.xml、Reanim XML 定义文件等
    - 考虑用 Rust 的 `quick-xml` 或 `roxmltree` 替代手写解析器

- [ ] **1.3.12 PropertiesParser（键值属性解析器）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/PropertiesParser.{h,cpp}`（1KB + 6KB）
  - **核心依赖项：** `Buffer`
  - **Rust 实现要点：**
    - 解析 `key=value` 格式的属性文件

- [ ] **1.3.13 DescParser（描述解析器）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/DescParser.{h,cpp}`（3KB + 12KB）
  - **核心依赖项：** `Buffer`
  - **Rust 实现要点：**
    - 解析资源描述文件格式

- [ ] **1.3.14 SexyVector（向量）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/SexyVector.h`（3KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 简单的 2D/3D 向量结构体

- [ ] **1.3.15 Flags（位标志）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/Flags.{h,cpp}`（2KB + 1KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：** 位标志集合，可用 `bitflags!` 宏

- [ ] **1.3.16 ModVal（模值）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/ModVal.{h,cpp}`（3KB + 8KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：** 一个特殊的模运算数值类型

- [ ] **1.3.17 Ratio（比例）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/Ratio.{h,cpp}`（2KB + 1KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：** 比例/分数类型

- [ ] **1.3.18 RegEmu（注册表模拟）**
  - **C++ 源文件：** `src/SexyAppFramework/misc/RegEmu.{h,cpp}`（1KB + 6KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 用文件或内存中的键值存储替代 Windows 注册表
    - 用于保存游戏设置

### 1.4 PakLib（PAK 文件读取）

- [ ] **1.4.1 PakInterface（PAK 文件接口）**
  - **C++ 源文件：** `src/SexyAppFramework/paklib/PakInterface.{h,cpp}`（5KB + 9KB）
  - **核心依赖项：** `Buffer`, `Common`
  - **Rust 实现要点：**
    - 已有 `paklib/mod.rs`（952 行），翻译较完整
    - PAK 文件读取：TOC 解析、文件提取
    - 可替代方案：直接读取文件系统而不依赖 PAK

### 1.5 ImageLib（图像格式加载）

- [ ] **1.5.1 ImageLib（图像库）**
  - **C++ 源文件：** `src/SexyAppFramework/imagelib/ImageLib.{h,cpp}`（1KB + 35KB）
  - **核心依赖项：** `Image`, `MemoryImage`, `Buffer`
  - **Rust 实现要点：**
    - 已有 `imagelib/mod.rs`（13 行骨架）
    - 加载 PNG, JPEG 等格式——建议用 Rust 的 `image` crate 替代
    - 负责将加载的像素数据填入 `MemoryImage`

### 1.6 声音系统

- [ ] **1.6.1 SoundManager（声音管理器接口）**
  - **C++ 源文件：** `src/SexyAppFramework/sound/SoundManager.h`（2KB）
  - **核心依赖项：** `SoundInstance`
  - **Rust 实现要点：**
    - 已有 `sound_manager.rs`（9 行骨架）
    - 定义声音管理 trait：`LoadSound`, `PlaySound`, `SetVolume` 等

- [ ] **1.6.2 SoundInstance（声音实例接口）**
  - **C++ 源文件：** `src/SexyAppFramework/sound/SoundInstance.h`（1KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 已有 `sound_instance.rs`（9 行骨架）
    - 定义声音实例 trait：`Play`, `Stop`, `SetVolume`, `IsPlaying` 等

- [ ] **1.6.3 SDLSoundManager（SDL 声音管理器）**
  - **C++ 源文件：** `src/SexyAppFramework/sound/SDLSoundManager.{h,cpp}`（2KB + 10KB）
  - **核心依赖项：** `SoundManager`, `SDLSoundInstance`, SDL2_mixer
  - **Rust 实现要点：**
    - 已有 `sdl_sound_manager.rs`（48 行骨架）
    - 基于 SDL2_mixer 的声音管理实现
    - 需绑定/包装 SDL_mixer C API

- [ ] **1.6.4 SDLSoundInstance（SDL 声音实例）**
  - **C++ 源文件：** `src/SexyAppFramework/sound/SDLSoundInstance.{h,cpp}`（3KB + 9KB）
  - **核心依赖项：** `SoundInstance`, SDL2_mixer
  - **Rust 实现要点：**
    - 已有 `sdl_sound_instance.rs`（31 行骨架）
    - 包装 SDL_mixer 的 `Mix_Chunk` 播放

- [ ] **1.6.5 SDLMusicInterface（SDL 音乐接口）**
  - **C++ 源文件：** `src/SexyAppFramework/sound/SDLMusicInterface.{h,cpp}`（2KB + 8KB）
  - **核心依赖项：** `MusicInterface`, SDL2_mixer
  - **Rust 实现要点：**
    - 已有 `sdl_music_interface.rs`（27 行骨架）
    - 包装 SDL_mixer 的 `Mix_Music` 播放

- [ ] **1.6.6 MusicInterface（音乐接口）**
  - **C++ 源文件：** `src/SexyAppFramework/sound/MusicInterface.h`（2KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 已有 `music_interface.rs`（8 行骨架）
    - 定义音乐接口 trait

---

## 二、Sexy.TodLib（引擎特效库）

### 2.1 动画系统

- [ ] **2.1.1 ReanimatorDefinition（动画定义数据结构）**
  - **C++ 源文件：** `src/Sexy.TodLib/Definition.{h,cpp}`（15KB + 69KB）
  - **核心依赖项：** `TodCommon`, `ReanimAtlas`, `TodStringFile`, `Image`
  - **Rust 实现要点：**
    - 已有 `definition.rs`（97 行骨架）
    - 解析 reanim XML 定义文件（轨道、变换帧、图集引用）
    - 数据量庞大：`gReanimatorDefCount` 约数百个动画定义
    - 结构体：`ReanimatorDefinition`, `ReanimatorTrack`, `ReanimatorTransform`, `ReanimationParams`

- [ ] **2.1.2 ReanimAtlas（动画图集）**
  - **C++ 源文件：** `src/Sexy.TodLib/ReanimAtlas.{h,cpp}`（2KB + 9KB）
  - **核心依赖项：** `Image`, `Graphics`, `MemoryImage`
  - **Rust 实现要点：**
    - 已有 `reanim_atlas.rs`（17 行骨架）
    - 将动画的所有帧打包到一张大纹理中，减少 OpenGL 状态切换

- [ ] **2.1.3 Reanimation（动画实例——核心类）**
  - **C++ 源文件：** `src/Sexy.TodLib/Reanimator.{h,cpp}`（14KB + 74KB）
  - **核心依赖项：** `ReanimatorDefinition`, `ReanimatorTrackInstance`, `Graphics`, `SexyMatrix`, `FilterEffect`, `Attachment`
  - **Rust 实现要点：**
    - 已有 `reanimator.rs`（157 行骨架）
    - **这是 TodLib 中最重要的类之一**
    - 实现动画播放：`Update`（帧推进），`Draw`（渲染），`DrawRenderGroup`
    - 变换插值：`GetCurrentTransform`, `BlendTransform`
    - 轨道管理：`FindTrackIndex`, `AssignRenderGroupToTrack`
    - 混合/淡入淡出：`StartBlend`
    - 附件系统：`AttachToAnotherReanimation`（一个动画实例可以附着到另一个的轨道上）
    - 需要 `ReanimationHolder`（`DataArray<Reanimation>`）管理所有动画实例
    - 性能关键——每帧要更新和绘制数百个动画

- [ ] **2.1.4 ReanimatorTransform（动画变换帧）**
  - **C++ 源文件：** `src/Sexy.TodLib/Reanimator.h`（内联定义）
  - **核心依赖项：** `Image`, `Font`
  - **Rust 实现要点：**
    - 结构体已在 C++ 中定义：位置、旋转、缩放、透明度、图像/字体/文字
    - 需要 `DEFAULT_FIELD_PLACEHOLDER` 标记未设置的字段

- [ ] **2.1.5 ReanimatorTrackInstance（轨道实例）**
  - **C++ 源文件：** `src/Sexy.TodLib/Reanimator.h`（内联定义）
  - **核心依赖项：** `ReanimatorTransform`, `Attachment`
  - **Rust 实现要点：**
    - 每个轨道保存运行时状态（混合计数器、图像覆盖、渲染组、颜色覆盖等）

- [ ] **2.1.6 ReanimationHolder（动画管理器）**
  - **C++ 源文件：** `src/Sexy.TodLib/Reanimator.{h,cpp}`（内联）
  - **核心依赖项：** `DataArray<Reanimation>`
  - **Rust 实现要点：**
    - 包装 `DataArray<Reanimation>`，提供 `AllocReanimation` 方法

### 2.2 粒子系统

- [ ] **2.2.1 TodParticleDefinition / TodEmitterDefinition（粒子系统定义数据）**
  - **C++ 源文件：** `src/Sexy.TodLib/TodParticle.{h,cpp}`（19KB + 68KB）；`Definition.cpp`（也涉及粒子定义加载）
  - **核心依赖项：** `Image`, `TodStringFile`, `TodCommon`
  - **Rust 实现要点：**
    - 已在 `tod_particle.rs`（107 行骨架）中定义结构体
    - 解析粒子 XML 定义文件
    - 大量 `FloatParameterTrack`（浮点参数轨道）——每个发射器约 30 个轨道
    - 粒子场系统（最多 4 个场叠加）：摩擦力、加速度、引力、斥力等

- [ ] **2.2.2 TodParticleSystem（粒子系统实例——核心类）**
  - **C++ 源文件：** `src/Sexy.TodLib/TodParticle.{h,cpp}`
  - **核心依赖项：** `TodParticleDefinition`, `TodParticleEmitter`, `TodParticle`, `TodList`, `Graphics`
  - **Rust 实现要点：**
    - 已有结构定义，需要实现：`Update`, `Draw`, `ParticleSystemDie`
    - 管理发射器列表（`TodList<ParticleEmitterID>`）
    - 提供 `OverrideColor`, `OverrideScale`, `CrossFade` 等运行时覆盖方法

- [ ] **2.2.3 TodParticleEmitter（粒子发射器——核心类）**
  - **C++ 源文件：** `src/Sexy.TodLib/TodParticle.{h,cpp}`
  - **核心依赖项：** `TodEmitterDefinition`, `TodParticleSystem`, `TodList<ParticleID>`, `TodParticle`, `Graphics`
  - **Rust 实现要点：**
    - 实现粒子生成逻辑：`UpdateSpawning`, `SpawnParticle`
    - 粒子更新：`UpdateParticle`（物理运动、场计算、生命周期）
    - 粒子场计算：`UpdateParticleField`, `UpdateSystemField`
    - 渲染：`Draw`, `DrawParticle`
    - 交叉淡入淡出：`CrossFadeParticle`, `CrossFadeEmitter`
    - **性能关键**——每帧处理数百到数千个粒子

- [ ] **2.2.4 TodParticle（单个粒子）**
  - **C++ 源文件：** `src/Sexy.TodLib/TodParticle.h`（内联）
  - **核心依赖项：** `TodParticleEmitter`, `SexyVector2`
  - **Rust 实现要点：**
    - 结构体已有定义
    - 包含位置、速度、自旋、颜色插值等运行时数据

- [ ] **2.2.5 TodParticleHolder（粒子系统管理器）**
  - **C++ 源文件：** `src/Sexy.TodLib/TodParticle.{h,cpp}`
  - **核心依赖项：** `DataArray<TodParticleSystem>`, `DataArray<TodParticleEmitter>`, `DataArray<TodParticle>`, `TodAllocator`
  - **Rust 实现要点：**
    - 管理所有粒子系统、发射器、粒子的分配和释放
    - 提供 `AllocParticleSystem` 工厂方法

- [ ] **2.2.6 FloatParameterTrack / FloatParameterTrackNode（参数轨道系统）**
  - **C++ 源文件：** `src/Sexy.TodLib/TodParticle.h`（内联）
  - **核心依赖项：** `TodCurves`
  - **Rust 实现要点：**
    - 评估曲线：给定时间 t，计算轨道值（最小~最大之间的插值）
    - `TodCurves` 枚举（线性、缓入、缓出、正弦等）已在 `game_enums.rs` 中定义
    - 这是粒子系统和动画系统的核心数学工具

### 2.3 特效与工具系统

- [ ] **2.3.1 EffectSystem（特效系统管理器）**
  - **C++ 源文件：** `src/Sexy.TodLib/EffectSystem.{h,cpp}`（23KB + 16KB）
  - **核心依赖项：** `TodParticleHolder`, `TrailHolder`, `ReanimationHolder`, `AttachmentHolder`
  - **Rust 实现要点：**
    - 已有 `effect_system.rs`（91 行骨架）
    - 全局特效实例（`gEffectSystem`），管理粒子、动画、轨迹、附件
    - `ProcessDeleteQueue` 清理已标记删除的特效对象

- [ ] **2.3.2 Attachment（附件系统）**
  - **C++ 源文件：** `src/Sexy.TodLib/Attachment.{h,cpp}`（5KB + 28KB）
  - **核心依赖项：** `DataArray`, `Reanimation`, `TodParticleSystem`
  - **Rust 实现要点：**
    - 已有 `attachment.rs`（47 行骨架）
    - `AttachmentID` 引用系统——一个附件可以是粒子系统或动画实例，附着在另一个对象的轨道上
    - `AttachmentHolder` 管理所有附件（`DataArray`）
    - `AttachEffect`、`AttacherInfo` 结构体
    - 附件更新、位置同步

- [ ] **2.3.3 Trail（轨迹系统）**
  - **C++ 源文件：** `src/Sexy.TodLib/Trail.{h,cpp}`（3KB + 11KB）
  - **核心依赖项：** `DataArray`, `Graphics`, `Image`
  - **Rust 实现要点：**
    - 已有 `trail.rs`（17 行骨架）
    - 运动轨迹渲染（如蜗牛 Stinky 的爬行痕迹）
    - `TrailHolder`、`TrailDefinition`、`TrailNode`、`TrailPoint` 等结构体

- [ ] **2.3.4 FilterEffect（滤镜效果）**
  - **C++ 源文件：** `src/Sexy.TodLib/FilterEffect.{h,cpp}`（1KB + 5KB）
  - **核心依赖项：** `Color`, `SexyMatrix`
  - **Rust 实现要点：**
    - 已有 `filter_effect.rs`（17 行骨架）
    - 颜色滤镜（I, Zombie 模式中改变植物颜色等）

- [ ] **2.3.5 TodFoley（音效管理）**
  - **C++ 源文件：** `src/Sexy.TodLib/TodFoley.{h,cpp}`（7KB + 25KB）
  - **核心依赖项：** `SoundManager`, `TodStringFile`
  - **Rust 实现要点：**
    - 已有 `tod_foley.rs`（47 行骨架）
    - 音效 ID 枚举、加载、播放管理
    - `FoleyType` 枚举（游戏中所有音效类型）

- [ ] **2.3.6 TodStringFile（字符串/数据文件读取）**
  - **C++ 源文件：** `src/Sexy.TodLib/TodStringFile.{h,cpp}`（3KB + 16KB）
  - **核心依赖项：** `Buffer`, `PakInterface`
  - **Rust 实现要点：**
    - 已有 `tod_string_file.rs`（62 行骨架）
    - 加载 `.tod` 格式的数据文件（用于 reanim/粒子定义）

- [ ] **2.3.7 TodCommon（通用工具函数）**
  - **C++ 源文件：** `src/Sexy.TodLib/TodCommon.{h,cpp}`（9KB + 45KB）
  - **核心依赖项：** `TodDebug`, `MTRand`
  - **Rust 实现要点：**
    - 已有 `tod_common.rs`（85 行骨架）
    - **大量内联数学工具**：`TodAnimateCurve`, `FloatLerp`, `TodRandom`, `ClampFloat` 等
    - 曲线上评估函数：`TodCurves` -> `TodAnimateCurve`（线性、缓动、反弹等各种缓动曲线）
    - 加权随机选择：`TodPickFromArray`, `TodWeightPickFromArray`
    - 这些函数在游戏逻辑中被高频调用

- [ ] **2.3.8 TodList（链表）**
  - **C++ 源文件：** `src/Sexy.TodLib/TodList.{h,cpp}`（5KB + 3KB）
  - **核心依赖项：** `TodAllocator`
  - **Rust 实现要点：**
    - 已有 `tod_list.rs`（17 行骨架）
    - 自定义侵入式链表实现，用于粒子系统和发射器的列表管理
    - 粒子系统和发射器的列表管理需要高效增删

- [ ] **2.3.9 DataArray（数据数组——核心容器）**
  - **C++ 源文件：** `src/Sexy.TodLib/DataArray.h`（4KB）
  - **核心依赖项：** `TodDebug`, `TodCommon`
  - **Rust 实现要点：**
    - 已有 `data_array.rs`（17 行骨架）
    - **翻译中最重要的数据结构之一**——整个游戏都用它管理实体
    - 类似对象池：用 ID（高 16 位为 key，低 16 位为索引）访问元素
    - 提供 `Alloc`, `Free`, `Get`, `TryToGet`, `IterateNext` 方法
    - Rust 实现需用 `Vec<Option<(u32, T)>>` 或自定义 `SlotMap` 风格结构
    - 安全性：ID 验证、迭代器的生命周期

- [ ] **2.3.10 TodDrawTriangle（三角形绘制函数集）**
  - **C++ 源文件：** `src/Sexy.TodLib/TodDrawTriangle.{h,cpp}`（2KB + 9KB）
  - **核心依赖项：** `SWHelper`, `SWVertex`
  - **Rust 实现要点：**
    - 已有 `tod_draw_triangle.rs`（9KB 相关代码在 SWTri 中）
    - ~60 个不同像素格式组合的函数变体
    - 用 Rust 泛型 + `macro_rules!` 或过程宏来生成变体

### 2.4 资源与定义

- [ ] **2.4.1 定义加载子系统（Reanim/Particle 定义加载）**
  - **C++ 源文件：** `src/Sexy.TodLib/Definition.cpp` 中的 `ReanimationLoadDefinition`, `ReanimatorEnsureDefinitionLoaded`, `ReanimatorLoadDefinitions`
  - **核心依赖项：** `TodStringFile`, `ReanimatorDefinition`, `TodParticleDefinition`, `XMLParser`
  - **Rust 实现要点：**
    - 从 `.tod` 文件（或 XML）加载动画定义
    - 从 XML 加载粒子系统定义
    - 全局定义数组管理

- [ ] **2.4.2 资源加载/预加载系统**
  - **C++ 源文件：** `src/Resources.{h,cpp}`（73KB + 153KB）
  - **核心依赖项：** `ImageLib`, `PakInterface`, `TodStringFile`, `ReanimatorDefinition`, `TodParticleDefinition`
  - **Rust 实现要点：**
    - **极大型文件**——153KB 的 C++ 代码
    - 定义游戏中所有资源的加载：图像、动画、粒子、字体、声音
    - 每个资源有字符串名称引用，有类型信息
    - 建议分模块：`resources::images`, `resources::reanims`, `resources::particles`, `resources::sounds`

---

## 三、Lawn（游戏主逻辑）

### 3.1 核心应用层

- [ ] **3.1.1 LawnApp（应用主类——核心类）**
  - **C++ 源文件：** `src/Lawn/LawnApp.{h,cpp}`（13KB + 86KB）（头文件在 `src/LawnApp.h`？实际查看：`src/LawnApp.h` 13KB + `src/LawnApp.cpp` 86KB）
  - **核心依赖项：** `SexyAppBase`, `Board`, `CutScene`, `Challenge`, `ZenGarden`, `EffectSystem`, `SoundManager`, `WidgetManager`
  - **Rust 实现要点：**
    - 已有 `lawn_app.rs`（252 行骨架）
    - 全局游戏状态：当前关卡、模式、分数、解锁状态
    - 管理 Board / CutScene / Challenge / ZenGarden 的生命周期
    - 加载线程：`LoadingThreadProc`——加载所有资源
    - 对话框管理（`DoDialog`, `IsDialogVisible`）
    - 游戏状态转换：主菜单 -> 关卡选择 -> 游戏中 -> 结果画面
    - 需要 `Arc<Mutex<>>` 管理全局单例状态

- [ ] **3.1.2 GameConstants（游戏常量）**
  - **C++ 源文件：** `src/GameConstants.h`（3KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 已在 `game_enums.rs` 中翻译大部分常量
    - 种子类型相关常量、关卡定义等

- [ ] **3.1.3 ConstEnums（全部枚举定义）**
  - **C++ 源文件：** `src/ConstEnums.h`（42KB）
  - **核心依赖项：** 无
  - **Rust 实现要点：**
    - 已有 `game_enums.rs`（856 行），翻译较完整
    - 包含所有游戏枚举：`SeedType`, `ZombieType`, `PlantState`, `GameMode`, `BackgroundType` 等
    - 需要检查是否有遗漏的枚举值，确保与 C++ 完全对齐

### 3.2 游戏实体层

- [ ] **3.2.1 GameObject（游戏对象基类）**
  - **C++ 源文件：** `src/Lawn/GameObject.{h,cpp}`（1KB + 1KB）
  - **核心依赖项：** `LawnApp`, `Board`, `Graphics`
  - **Rust 实现要点：**
    - 已有 `game_object.rs`（67 行骨架）
    - 所有游戏实体的基类：包含 `mApp`, `mBoard`, 位置、尺寸、可见性、渲染顺序
    - 内联方法：`BeginDraw`, `EndDraw`, `MakeParentGraphicsFrame`

- [ ] **3.2.2 Plant（植物——核心类）**
  - **C++ 源文件：** `src/Lawn/Plant.{h,cpp}`（12KB + 192KB）
  - **核心依赖项：** `GameObject`, `Board`, `Zombie`, `Reanimation`, `TodParticleSystem`, `SeedType`
  - **Rust 实现要点：**
    - 已有 `plant.rs`（376 行骨架 + 字段定义）
    - **极大型文件**——192KB C++ 代码
    - 所有植物种类（~48 种）的特有行为实现：
      - 射手类：`UpdateShooter`, `Fire`, `FindTargetAndFire`（豌豆、双发、三发、裂荚、机枪等）
      - 生产类：`UpdateProductionPlant`（向日葵、阳光菇等）
      - 特殊类：`UpdateChomper`, `UpdateSquash`, `UpdatePotato`, `UpdateCobCannon`, `UpdateMagnetShroom`
      - 辅助类：`UpdateTorchwood`, `UpdateUmbrella`, `UpdateCoffeeBean`
      - 水上类：`UpdateLilypad`, `UpdateTanglekelp`
    - 动画管理：`PlayBodyReanim`, `AttachBlinkAnim`, `UpdateReanim`
    - 绘制：`Draw`, `DrawShadow`, `DrawSeedType`
    - 伤害/死亡：`Die`, `Squish`, `TakeDamage`, `RemoveEffects`
    - 静态方法：`GetImage`, `GetCost`, `GetToolTip`, `IsNocturnal`, `IsAquatic`, `IsFlying` 等
    - `PlantDefinition` 全局定义表——所有植物属性的静态数据

- [ ] **3.2.3 Zombie（僵尸——核心类）**
  - **C++ 源文件：** `src/Lawn/Zombie.{h,cpp}`（22KB + 346KB）
  - **核心依赖项：** `GameObject`, `Board`, `Plant`, `Reanimation`, `TodParticleSystem`, `Attachment`, `ZombieType`
  - **Rust 实现要点：**
    - 已有 `zombie.rs`（478 行骨架 + 字段定义）
    - **极大型文件**——346KB C++ 代码，项目中最大的文件
    - 所有僵尸种类（~26 种）的特有行为：
      - 普通僵尸：`UpdateZombieWalking`, `EatPlant`, `FindPlantTarget`
      - 特殊僵尸：`UpdateZombiePolevaulter`, `UpdateZombieDolphinRider`, `UpdateZombieBungee`, `UpdateZombiePogo`, `UpdateZombieDigger`
      - Boss 僵尸：`UpdateBoss`, `BossRVAttack`, `BossStompAttack`, `BossHeadAttack`
      - 其他：`UpdateYeti`, `UpdateZombieDancer`, `UpdateZombieNewspaper`, `UpdateZombieCatapult`
    - 伤害系统：`TakeDamage`, `TakeBodyDamage`, `TakeHelmDamage`, `TakeShieldDamage`
    - 状态效果：`ApplyChill`, `ApplyButter`, `ApplyBurn`, `MindControlled`
    - 动画：`LoadReanim`, `DrawReanim`, `SetupReanimLayers`, `PlayZombieReanim`
    - 绘制：`Draw`, `DrawZombiePart`, `DrawBossFireBall`, `DrawShadow`, `DrawIceTrap`
    - `ZombieDefinition` 全局定义表

- [ ] **3.2.4 Projectile（弹射物）**
  - **C++ 源文件：** `src/Lawn/Projectile.{h,cpp}`（3KB + 33KB）
  - **核心依赖项：** `GameObject`, `Board`, `Zombie`, `Plant`
  - **Rust 实现要点：**
    - 已有 `projectile.rs`（201 行骨架 + 字段定义）
    - 不同弹射物行为：
      - `UpdateNormalMotion`（豌豆、冰豆、火豆等直线弹道）
      - `UpdateLobMotion`（西瓜、玉米等抛物线）
      - `UpdateCobCannonMotion`（玉米加农炮）
    - 碰撞检测：`FindCollisionTarget`, `DoImpact`, `DoSplashDamage`
    - 火炬木转换：`ConvertToFireball`, `ConvertToPea`

- [ ] **3.2.5 Coin（硬币/掉落物）**
  - **C++ 源文件：** `src/Lawn/Coin.{h,cpp}`（3KB + 48KB）
  - **核心依赖项：** `GameObject`, `Board`, `PottedPlant`
  - **Rust 实现要点：**
    - 已有 `coin.rs`（145 行骨架 + 字段定义）
    - 掉落物行为：阳光、金币、钻石、钥匙、奖杯等
    - 物理：`UpdateFall`, `UpdateCollected`
    - 收集动画：`Collect`, `StartFade`, `ScoreCoin`

- [ ] **3.2.6 LawnMower（割草机）**
  - **C++ 源文件：** `src/Lawn/LawnMower.{h,cpp}`（2KB + 14KB）
  - **核心依赖项：** `Board`, `Zombie`, `Reanimation`
  - **Rust 实现要点：**
    - 已有 `lawn_mower.rs`（90 行骨架 + 字段定义）
    - 启动、移动、碰撞僵尸、碾死僵尸
    - 水池中的割草机行为：`UpdatePool`

- [ ] **3.2.7 GridItem（网格物品）**
  - **C++ 源文件：** `src/Lawn/GridItem.{h,cpp}`（2KB + 23KB）
  - **核心依赖项：** `Board`, `Zombie`, `Reanimation`, `TodParticleSystem`
  - **Rust 实现要点：**
    - 已有 `grid_item.rs`（79 行骨架 + 字段定义）
    - 多种类型：墓碑、弹坑、梯子、传送门、花盆(Scary Potter)、松鼠、蜗牛、智慧树工具
    - 每个类型有独立的绘制和更新逻辑：
      - `DrawGraveStone`, `DrawCrater`, `DrawLadder`, `DrawPortal`, `DrawScaryPot`, `DrawSquirrel`, `DrawStinky`
      - `UpdateScaryPot`, `UpdatePortal`, `UpdateRake`, `UpdateBrain`

- [ ] **3.2.8 CursorObject / CursorPreview（光标对象/预览）**
  - **C++ 源文件：** `src/Lawn/CursorObject.{h,cpp}`（1KB + 11KB）
  - **核心依赖项：** `GameObject`, `SeedType`, `CursorType`, `Reanimation`
  - **Rust 实现要点：**
    - 已有 `cursor_object.rs`（62 行）
    - 手持植物/工具的绘制与更新（种植时跟随鼠标的动画）

- [ ] **3.2.9 SeedPacket / SeedBank（种子包/种子库）**
  - **C++ 源文件：** `src/Lawn/SeedPacket.{h,cpp}`（3KB + 32KB）
  - **核心依赖项：** `GameObject`, `Board`, `HitResult`
  - **Rust 实现要点：**
    - 已有 `seed_packet.rs`（62 行骨架 + 字段定义）
    - 冷却计时、刷新、绘制种子图标
    - 传送带模式：`UpdateConveyorBelt`
    - 老虎机模式：`SlotMachineStart`, `PickNextSlotMachineSeed`
    - `SeedBank` 管理多个 `SeedPacket`

- [ ] **3.2.10 MessageWidget（消息/提示控件）**
  - **C++ 源文件：** `src/Lawn/MessageWidget.{h,cpp}`（2KB + 14KB）
  - **核心依赖项：** `GameObject`, `Board`
  - **Rust 实现要点：** 游戏中出现的提示文字（如"一大波僵尸即将来袭"）

- [ ] **3.2.11 ToolTipWidget（悬停提示）**
  - **C++ 源文件：** `src/Lawn/ToolTipWidget.{h,cpp}`（2KB + 6KB）
  - **核心依赖项：** `GameObject`, `Board`
  - **Rust 实现要点：**
    - 已有 `tool_tip_widget.rs`（17 行骨架）
    - 植物/按钮悬停时的说明文字

### 3.3 游戏模式/场景层

- [ ] **3.3.1 Board（游戏主面板——核心类）**
  - **C++ 源文件：** `src/Lawn/Board.{h,cpp}`（21KB + 289KB）
  - **核心依赖项：** `LawnApp`, `DataArray<Zombie>`, `DataArray<Plant>`, `DataArray<Projectile>`, `DataArray<Coin>`, `DataArray<LawnMower>`, `DataArray<GridItem>`, `Challenge`, `CutScene`, `SeedBank`
  - **Rust 实现要点：**
    - 已有 `board.rs`（604 行骨架）
    - **极大型文件**——289KB C++ 代码，游戏中最大的核心调度类
    - 游戏主循环：`Update` → `UpdateGameObjects` → `Draw` → `DrawGameObjects`
    - 渲染排序：`RenderItem` 数组按 Z 深度排序 + 分类型渲染
    - 关卡初始化：`InitLevel`, `InitZombieWaves`, `LoadBackgroundImages`
    - 实体工厂：`AddPlant`, `AddZombie`, `AddProjectile`, `AddCoin`
    - 碰撞检测：`MouseHitTest`, `ZombieHitTest`
    - 网格坐标转换：`PixelToGridX/Y`, `GridToPixelX/Y`
    - 逐波生成：`SpawnZombieWave`, `UpdateZombieSpawning`, `UpdateSunSpawning`
    - 游戏状态：胜利/失败判定、暂停、关卡结束序列
    - 大约 100+ 个方法需要翻译

- [ ] **3.3.2 Challenge（挑战模式——核心类）**
  - **C++ 源文件：** `src/Lawn/Challenge.{h,cpp}`（15KB + 174KB）
  - **核心依赖项：** `Board`, `Plant`, `Zombie`, `GridItem`, `SeedPacket`, `FilterEffect`
  - **Rust 实现要点：**
    - 已有 `challenge.rs`（33 行骨架）
    - **极大型文件**——174KB C++ 代码
    - 所有特殊关卡逻辑：
      - 消消乐（Beghouled）：`UpdateBeghouled`, `BeghouledPopulateBoard`, `BeghouledRemoveMatches`
      - 老虎机（Slot Machine）：`UpdateSlotMachine`, `DrawSlotMachine`
      - 植物僵尸（I, Zombie）：`IZombieStart`, `IZombieUpdate`, `IZombieEatBrain`
      - 打地鼠（Whack-A-Zombie）：`WhackAZombieSpawning`, `MouseDownWhackAZombie`
      - 最后防线（Last Stand）：`LastStandUpdate`, `LastStandCompletedStage`
      - 暴风雨夜（Stormy Night）：`DrawStormNight`, `UpdateStormyNight`
      - 传送门（Portal）：`PortalStart`, `UpdatePortalCombat`, `MoveAPortal`
      - 花盆挑战（Scary Potter）：`ScaryPotterStart`, `ScaryPotterUpdate`
      - 智慧树（Tree of Wisdom）：`TreeOfWisdomUpdate`, `TreeOfWisdomFertilize`
      - 松鼠（Squirrel）：`SquirrelUpdate`, `SquirrelFound`
      - 传送带模式（Conveyor Belt）：`UpdateConveyorBelt`
    - 约 100+ 个方法

- [ ] **3.3.3 CutScene（开场/过场动画）**
  - **C++ 源文件：** `src/Lawn/Cutscene.{h,cpp}`（5KB + 81KB）
  - **核心依赖项：** `Board`, `ChallengeScreen`, `Zombie`
  - **Rust 实现要点：**
    - 已有 `cutscene.rs`（17 行骨架）
    - 关卡开场动画（僵尸入场、棋盘展示）
    - 种子选择界面管理
    - Crazy Dave 对话系统
    - 升级包推荐展示

- [ ] **3.3.4 ZenGarden（禅境花园）**
  - **C++ 源文件：** `src/Lawn/ZenGarden.{h,cpp}`（7KB + 87KB）
  - **核心依赖项：** `Board`, `Plant`, `GridItem`, `PottedPlant`
  - **Rust 实现要点：**
    - 已有 `zen_garden.rs`（17 行骨架）
    - 盆栽植物系统：种植、浇水、施肥、音乐、虫害处理
    - 蜗牛 Stinky 管理
    - 商店交互

### 3.4 UI 控件层（Lawn 定制）

- [ ] **3.4.1 GameButton（游戏按钮）**
  - **C++ 源文件：** `src/Lawn/Widget/GameButton.{h,cpp}`——已查看实现
  - **核心依赖项：** `ButtonWidget`, `LawnApp`
  - **Rust 实现要点：**
    - 已有 `game_button.rs`（15 行骨架）
    - 游戏中使用的各种按钮（菜单、铲子、商店等）

- [ ] **3.4.2 ChallengeScreen（挑战选择界面）**
  - **C++ 源文件：** `src/Lawn/Widget/ChallengeScreen.{h,cpp}`
  - **核心依赖项：** `Widget`, `LawnApp`, `GameButton`
  - **Rust 实现要点：**
    - 已有 `challenge_screen.rs`（15 行骨架）
    - 挑战/生存模式选择页

- [ ] **3.4.3 SeedChooserScreen（种子选择界面）**
  - **C++ 源文件：** `src/Lawn/Widget/SeedChooserScreen.{h,cpp}`
  - **核心依赖项：** `Widget`, `LawnApp`, `SeedPacket`
  - **Rust 实现要点：**
    - 已有 `seed_chooser_screen.rs`（15 行骨架）
    - 关卡开始前选择携带的植物

- [ ] **3.4.4 AlmanacDialog（图鉴对话框）**
  - **C++ 源文件：** `src/Lawn/Widget/AlmanacDialog.{h,cpp}`
  - **核心依赖项：** `Dialog`, `LawnApp`, `Plant`, `Zombie`
  - **Rust 实现要点：**
    - 已有 `almanac_dialog.rs`（15 行骨架）
    - 植物/僵尸百科

- [ ] **3.4.5 StoreScreen（商店界面）**
  - **C++ 源文件：** `src/Lawn/Widget/StoreScreen.{h,cpp}`
  - **核心依赖项：** `Widget`, `LawnApp`, `GameButton`
  - **Rust 实现要点：**
    - 已有 `store_screen.rs`（15 行骨架）
    - Crazy Dave 的汽车后备箱商店

- [ ] **3.4.6 TitleScreen（标题界面）**
  - **C++ 源文件：** `src/Lawn/Widget/TitleScreen.{h,cpp}`
  - **核心依赖项：** `Widget`, `LawnApp`, `GameButton`
  - **Rust 实现要点：**
    - 已有 `title_screen.rs`（15 行骨架）
    - 主标题界面

- [ ] **3.4.7 AwardScreen（通关奖励界面）**
  - **C++ 源文件：** `src/Lawn/Widget/AwardScreen.{h,cpp}`
  - **核心依赖项：** `Widget`, `LawnApp`, `GameButton`
  - **Rust 实现要点：**
    - 已有 `award_screen.rs`（15 行骨架）
    - 关卡通关后展示奖杯和成绩

- [ ] **3.4.8 CreditScreen（制作人员列表）**
  - **C++ 源文件：** `src/Lawn/Widget/CreditScreen.{h,cpp}`
  - **核心依赖项：** `Widget`, `LawnApp`
  - **Rust 实现要点：**
    - 已有 `credit_screen.rs`（15 行骨架）

- [ ] **3.4.9 其他对话框（Lawn 定制）**
  - **C++ 源文件：** 以下文件各 15 行骨架在 Rust 中
    - `AchievementsScreen`（成就界面）
    - `CheatDialog`（作弊对话框）
    - `ContinueDialog`（继续游戏对话框）
    - `ImitaterDialog`（模仿者选择对话框）
    - `LawnDialog`（游戏通用对话框）
    - `NewOptionsDialog`（新选项界面）
    - `NewUserDialog`（新用户对话框）
    - `UserDialog`（用户选择对话框）
  - **核心依赖项：** `Dialog`, `LawnApp`
  - **Rust 实现要点：** 以上文件均已有 15 行 Rust 骨架，需翻译完整 C++ 逻辑

### 3.5 工具与辅助层

- [ ] **3.5.1 LawnCommon（游戏通用函数）**
  - **C++ 源文件：** `src/Lawn/LawnCommon.{h,cpp}`（3KB + 4KB）
  - **核心依赖项：** `EditWidget`, `Dialog`, `Checkbox`, `Graphics`
  - **Rust 实现要点：**
    - 已有 `lawn_common.rs`（81 行骨架）
    - `TileImageHorizontally`, `TileImageVertically`（平铺绘制辅助）
    - 存档文件名生成
    - 编辑框控件创建辅助

- [ ] **3.5.2 系统模块（存档/玩家/配置）**
  - **C++ 源文件：** `src/Lawn/System/` 目录下的文件
  - **核心依赖项：** `LawnApp`, `Board`, `SexyAppBase`
  - **Rust 实现要点：**
    - 已有骨架文件：
      - `data_sync.rs`（17 行）——数据同步
      - `player_info.rs`（17 行）——玩家信息
      - `profile_mgr.rs`（17 行）——玩家档案管理
      - `save_game.rs`（17 行）——游戏存档
      - `music.rs`（17 行）——音乐管理
      - `reanimation_lawn.rs`（17 行）——Lawn 动画扩展
      - `typing_check.rs`（17 行）——打字检查（DoTypingCheck）
    - 需要完整的存档/读档序列化逻辑

---

## 四、FFI / 外部绑定层

- [ ] **4.1 SDL2 FFI 绑定**
  - **C++ 源文件：** 无（C 库对应 SDL2）
  - **Rust 文件：** `rust/src/ffi/sdl2.rs`（871 行）
  - **核心依赖项：** SDL2 原生库
  - **Rust 实现要点：**
    - 已有较完整绑定
    - 检查 `SDL_Init`, `SDL_CreateWindow`, `SDL_PollEvent`, `SDL_GL_SwapWindow` 等关键函数是否都绑定了
    - 确保和 SDL2 的 ABI 兼容

- [ ] **4.2 OpenGL FFI 绑定**
  - **C++ 源文件：** 无（对应 glad/gles2.h）
  - **Rust 文件：** `rust/src/ffi/opengl.rs`（726 行）
  - **核心依赖项：** OpenGL 库
  - **Rust 实现要点：**
    - 已有较完整绑定
    - OpenGL 函数指针加载，需要 `glGetProcAddress` 包装

- [ ] **4.3 libc FFI 绑定**
  - **Rust 文件：** `rust/src/ffi/libc.rs`（39 行）
  - **核心依赖项：** 无
  - **Rust 实现要点：** 基本 libc 函数绑定，已有

- [ ] **4.4 Loader（动态库加载器）**
  - **Rust 文件：** `rust/src/ffi/loader.rs`（17 行）
  - **核心依赖项：** 无
  - **Rust 实现要点：** OpenGL 函数加载辅助，已有

---

## 五、主入口与构建

- [ ] **5.1 main.rs（主入口）**
  - **C++ 源文件：** `src/main.cpp`（4KB）
  - **核心依赖项：** `LawnApp`
  - **Rust 实现要点：**
    - 已有 `main.rs`（33 行）
    - 创建 LawnApp → init → start → shutdown

- [ ] **5.2 lib.rs（模块声明）**
  - **Rust 文件：** `rust/src/lib.rs`（18 行）
  - **Rust 实现要点：**
    - 已有，声明 `ffi`, `framework`, `todlib`, `lawn` 四个模块

- [ ] **5.3 build.rs（构建脚本）**
  - **Rust 文件：** `rust/build.rs`（442 行）
  - **Rust 实现要点：**
    - 已有构建脚本
    - 可能包含 C 源码编译、链接标志、资源打包等

- [ ] **5.4 Cargo.toml（项目配置）**
  - **Rust 文件：** `rust/Cargo.toml`（376 行）
  - **Rust 实现要点：**
    - 已有，需确认依赖项（sdl2, image, quick-xml 等）是否齐全

---

## 六、总结与优先级

### 按翻译量级排序（C++ 源文件大小 ≈ 翻译工作量）

| 优先级 | 模块名称 | C++ 代码量 | Rust 状态 |
|--------|---------|-----------|----------|
| ★★★★★ | `Zombie`（僵尸） | 346KB + 22KB | 字段定义 ✓ 方法体 ✗ |
| ★★★★★ | `Board`（游戏面板） | 289KB + 21KB | 字段定义 ✓ 方法体 ✗ |
| ★★★★★ | `Plant`（植物） | 192KB + 12KB | 字段定义 ✓ 方法体 ✗ |
| ★★★★★ | `Challenge`（挑战模式） | 174KB + 15KB | 骨架 ✗ |
| ★★★★☆ | `Resources`（资源定义） | 153KB + 73KB | 未创建 |
| ★★★★☆ | `SexyAppBase`（应用基类） | 99KB + 19KB | 骨架 ✓ |
| ★★★★☆ | `ZenGarden`（禅境花园） | 87KB + 7KB | 骨架 ✗ |
| ★★★★☆ | `LawnApp`（应用主类） | 86KB + 13KB | 骨架 ✓ |
| ★★★★☆ | `CutScene`（过场动画） | 81KB + 5KB | 骨架 ✗ |
| ★★★★☆ | `Reanimator`（动画系统） | 74KB + 14KB | 骨架 ✓ |
| ★★★★☆ | `TodParticle`（粒子系统） | 68KB + 19KB | 骨架 ✓ |
| ★★★☆☆ | `TodCommon`（通用函数） | 45KB + 9KB | 骨架 ✓ |
| ★★★☆☆ | `GLInterface`（OpenGL接口） | 45KB + 8KB | 较完整 ✓ |
| ★★★☆☆ | `Graphics`（绘制上下文） | 43KB + 8KB | 骨架 ✓ |
| ★★★☆☆ | `ImageLib`（图像加载） | 35KB + 1KB | 骨架 ✗ |
| ★★★☆☆ | `Projectile`（弹射物） | 33KB + 3KB | 字段定义 ✓ |
| ★★★☆☆ | `SeedPacket`（种子包） | 32KB + 3KB | 字段定义 ✓ |
| ★★★☆☆ | `ResourceManager`（资源管理） | 31KB + 6KB | 骨架 ✗ |
| ★★★☆☆ | `SWTri`（软件渲染） | 31KB + 31KB | 未翻译 |
| ★★★☆☆ | `Attachment`（附件系统） | 28KB + 5KB | 骨架 ✓ |
| ★★★☆☆ | `TodFoley`（音效） | 25KB + 7KB | 骨架 ✓ |
| ★★★☆☆ | `GridItem`（网格物品） | 23KB + 2KB | 字段定义 ✓ |
| ★★☆☆☆ | 其余文件（各 < 20KB） | — | — |

### 关键里程碑建议

1. **第一阶段（基础设施）**：`DataArray`, `TodList`, `TodCommon`, `MTRand`, `SexyMatrix` → 使容器和数学库可用
2. **第二阶段（渲染管线）**：`Graphics`, `Image`, `MemoryImage`, `SexyAppBase::DoMainLoop` → 窗口能跑起来
3. **第三阶段（特效引擎）**：`Reanimator`, `TodParticle`, `EffectSystem`, `Attachment` → 动画和粒子系统工作
4. **第四阶段（游戏实体）**：`Plant`, `Zombie`, `Projectile`, `Coin`, `LawnMower`, `GridItem` → 实体逻辑完整
5. **第五阶段（游戏调度）**：`Board`, `Challenge`, `CutScene`, `ZenGarden` → 关卡可玩
6. **第六阶段（UI 与收尾）**：所有 Widget、对话框、商店、图鉴等 → 完整游戏
