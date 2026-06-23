# PvZ Portable C++ → Rust 前置分析报告

## 1. 架构映射关系

### C++ 类 → Rust 结构体 + impl 块
| C++ 概念 | Rust 映射 |
|---------|----------|
| `class Widget` (虚继承) | `struct Widget` + `trait WidgetTrait` 或 enum 分发 |
| `class Dialog : Widget` | `struct Dialog` + `impl WidgetTrait for Dialog` |
| `SexyAppBase` (全局单例) | `static ref APP: Mutex<SexyAppBase>` 或 `RefCell` |
| `Board` (大型状态对象) | `struct Board` + `Cell`/`RefCell` 用于可变子字段 |
| 虚函数表多态 | Trait 对象 `Box<dyn WidgetTrait>` |
| 简单 getter/setter | 直接字段访问 |

### C++ 指针/引用 → Rust 所有权
| C++ 模式 | Rust 方案 |
|---------|----------|
| 裸指针 `Widget*` | `*mut Widget` (unsafe) 或 `Rc<RefCell<Widget>>` |
| 全局 `gLawnApp` | `static mut` 或 `OnceLock<Mutex<>>` |
| `std::vector<Plant>` | `Vec<Plant>` — 直接对应 |
| `std::map<int, Dialog*>` | `HashMap<i32, Rc<RefCell<Dialog>>>` |
| `std::list` | `LinkedList` 或 `VecDeque` |

### C++ 继承/多态 → Rust Trait 对象
- `ButtonListener` 接口 → `trait ButtonListener`
- `DialogListener` 接口 → `trait DialogListener`
- `SexyAppBase : ButtonListener, DialogListener` → `impl ButtonListener for SexyAppBase, impl DialogListener for SexyAppBase`

### C++ 枚举 → Rust 枚举
- `enum SeedType : int32_t` → `#[repr(i32)] enum SeedType`
- 所有标志位枚举 → `bitflags!` 宏

### C++ 宏 → Rust 宏
- `unreachable()` → `std::hint::unreachable_unchecked()`
- `LENGTH(arr)` → `arr.len()`
- 平台宏条件编译(`#ifdef`) → `#[cfg()]` 属性

## 2. 并发等价实现

| C++ | Rust |
|-----|------|
| `std::mutex` | `std::sync::Mutex` |
| `std::thread` | `std::thread` |
| `std::atomic<bool>` | `std::sync::atomic::AtomicBool` |
| `std::lock_guard` | 作用域内 `lock()` RAII |

**数据竞争风险**：
- C++ 中 `SexyAppBase::mCritSect` 保护多个可共享字段 → Rust 中每个受保护字段独立或整体放入 `Mutex`
- 加载线程与主线程之间的 `mLoadingThreadCompleted` (atomic) → `AtomicBool` + `Condvar`
- Rust 所有权系统静态防止了 C++ 中可能遗漏锁保护导致的数据竞争

## 3. 潜在风险点

### FFI 边界
- SDL2 函数通过 `extern "C"` 声明，内存由各 API 自身管理
- OpenGL 由 SDL2 管理上下文，无需额外绑定
- SDL_Surface/Cursor 等 C 资源的释放必须由 Rust 的 Drop 实现确保

### 类型转换
- `int` → `i32`, `uint32_t` → `u32`, `float` → `f32`
- `char` (signed in MSVC) → `i8` 或 `u8`，注意 C++ `char` 的符号性未定义
- `bool` → `bool`，但 C 接口的 SDL_bool → `i32`
- 注意 C++ 中 `int32_t` 枚举与 `int` 混用：Rust 需要显式 `as i32` 转换

### 事件循环
- C++ 使用 SDL_PollEvent + PeekMessage 混合 → Rust 也使用 SDL_PollEvent
- 按键状态：C++ 使用 `mCtrlDown`, `mAltDown` 标志 + IM_KeyDown 消息队列 → Rust 可用类似方式或 SDL_GetKeyboardState
- 鼠标捕获：C++ 通过 mLastX/mLastY 追踪 → Rust 同样

### 未定义行为隔离
- C++ 反编译代码中可能存在的未定义行为（如有符号整数溢出、无效指针解引用）→ 全部隔离在 `unsafe` 块中
- FFI 调用全部标记 `unsafe`
- 游戏逻辑层力求零 `unsafe`
