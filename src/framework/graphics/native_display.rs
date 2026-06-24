// PvZ Portable Rust 翻译 — 原生显示接口
// 对应 C++ SexyAppFramework/graphics/NativeDisplay.h / NativeDisplay.cpp
//
// NativeDisplay 封装了原生显示的目标像素格式信息。
// 它仅存储颜色掩码/位移参数，实际显示由 SDL2 和 OpenGL 处理。

#![allow(dead_code)]

/// 原生显示基类（对应 C++ NativeDisplay）
pub struct NativeDisplay {
    /// RGB 总位数
    pub rgb_bits: i32,
    /// 红色通道掩码
    pub red_mask: u32,
    /// 绿色通道掩码
    pub green_mask: u32,
    /// 蓝色通道掩码
    pub blue_mask: u32,
    /// 红色通道位数
    pub red_bits: i32,
    /// 绿色通道位数
    pub green_bits: i32,
    /// 蓝色通道位数
    pub blue_bits: i32,
    /// 红色通道位移
    pub red_shift: i32,
    /// 绿色通道位移
    pub green_shift: i32,
    /// 蓝色通道位移
    pub blue_shift: i32,
}

impl NativeDisplay {
    pub fn new() -> Self {
        NativeDisplay {
            rgb_bits: 0,
            red_mask: 0,
            green_mask: 0,
            blue_mask: 0,
            red_bits: 0,
            green_bits: 0,
            blue_bits: 0,
            red_shift: 0,
            green_shift: 0,
            blue_shift: 0,
        }
    }
}

impl Default for NativeDisplay {
    fn default() -> Self {
        NativeDisplay::new()
    }
}
