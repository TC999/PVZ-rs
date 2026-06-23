// PvZ Portable Rust 翻译 — 原生显示接口
// 对应 C++ SexyAppFramework/graphics/NativeDisplay.h / NativeDisplay.cpp

#![allow(dead_code)]

/// 原生显示基类
pub struct NativeDisplay {
    pub is_windowed: bool,
    pub is_3d: bool,
}

impl NativeDisplay {
    pub fn new() -> Self {
        NativeDisplay {
            is_windowed: true,
            is_3d: true,
        }
    }

    /// 切换窗口/全屏模式
    pub fn set_windowed(&mut self, windowed: bool) {
        self.is_windowed = windowed;
    }
}

impl Default for NativeDisplay {
    fn default() -> Self {
        NativeDisplay::new()
    }
}
