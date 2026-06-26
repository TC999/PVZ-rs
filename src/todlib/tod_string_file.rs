// PvZ Portable Rust 翻译 — TodStringFile（字符串/本地化文件）
// 对应 C++ src/Sexy.TodLib/TodStringFile.h / TodStringFile.cpp

#![allow(dead_code)]
// 全局变量名保持与 C++ 源码一致
#![allow(non_upper_case_globals)]

use std::collections::HashMap;

/// 字符串格式（颜色标记）
#[derive(Clone)]
pub struct TodStringFormat {
    pub color: u32,
    pub tag: String,
}

/// 全局字符串格式列表
pub static mut gLawnStringFormats: Vec<TodStringFormat> = Vec::new();
pub static mut gLawnStringFormatCount: i32 = 0;

/// 初始化字符串颜色
pub fn tod_string_list_set_colors(formats: &[TodStringFormat], _count: i32) {
    unsafe {
        gLawnStringFormats = formats.to_vec();
    }
}

/// 字符串读取器
pub struct TodStringFile {
    pub strings: HashMap<String, String>,
}

impl TodStringFile {
    pub fn new() -> Self {
        TodStringFile {
            strings: HashMap::new(),
        }
    }

    /// 从文件加载字符串
    pub fn load_from_file(&mut self, _path: &str) -> bool {
        // 加载字符串文件
        true
    }

    /// 获取字符串（带颜色处理）
    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.strings.get(key)
    }

    /// 解码字符串（处理颜色标记）
    pub fn decode_string(&self, _text: &str) -> String {
        // 处理 ^color^ 标记
        String::new()
    }
}

impl Default for TodStringFile {
    fn default() -> Self {
        TodStringFile::new()
    }
}

// ══════════════════════════════════════════════════════
// ║  TodString 格式系统（对应 C++ TodStringFile.h）
// ══════════════════════════════════════════════════════

/// 字符串格式标志位（对应 C++ TodStringFormatFlag）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TodStringFormatFlag {
    IgnoreNewlines,
    HideUntilMagnetshroom,
}

/// 字符串列表格式（对应 C++ TodStringListFormat）
/// 描述一个字符串段落的格式：字体、颜色、行间距偏移、格式标志
#[derive(Debug, Clone)]
pub struct TodStringListFormat {
    pub format_name: Option<&'static str>,
    pub new_font: Option<*mut std::ffi::c_void>,   // _Font**
    pub new_color: Color,
    pub line_spacing_offset: i32,
    pub format_flags: u32,
}

impl Default for TodStringListFormat {
    fn default() -> Self {
        TodStringListFormat {
            format_name: None,
            new_font: None,
            new_color: Color::new(0, 0, 0, 0),
            line_spacing_offset: 0,
            format_flags: 0,
        }
    }
}

use crate::framework::color::Color;
