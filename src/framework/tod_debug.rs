// PvZ Portable Rust 翻译 — Debug / TodDebug 调试工具
// 对应 C++ SexyAppFramework/misc/Debug.h

#![allow(dead_code)]

/// 全局断言标志
pub static mut G_IN_ASSERT: bool = false;

/// 输出调试信息（对应 C++ OutputDebug）
pub fn output_debug(_fmt: &str) {
    #[cfg(debug_assertions)]
    eprintln!("[DEBUG] {}", _fmt);
}

/// 断言宏模拟（对应 C++ DBG_ASSERT）
#[macro_export]
macro_rules! dbg_assert {
    ($exp:expr) => {
        #[cfg(debug_assertions)]
        {
            unsafe { $crate::framework::tod_debug::G_IN_ASSERT = true; }
            assert!($exp);
            unsafe { $crate::framework::tod_debug::G_IN_ASSERT = false; }
        }
    };
}
