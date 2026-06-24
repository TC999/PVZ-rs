// PvZ Portable Rust 翻译 — 字体类型
// 对应 C++ SexyAppFramework/graphics/Font.h / Font.cpp

#![allow(dead_code)]

/// 字体基类
#[derive(Debug)]
pub struct Font {
    /// 字体名称
    pub name: String,
    /// 字体大小
    pub size: i32,
    /// 字体粗细（0-1000）
    pub bold: bool,
    /// 斜体
    pub italic: bool,
    /// 行高
    pub line_spacing: i32,
    /// 字符间距
    pub char_spacing: i32,
    /// 字体高度
    pub font_height: i32,
    /// 基线偏移
    pub ascent: i32,
}

impl Font {
    pub fn new(name: &str, size: i32) -> Self {
        Font {
            name: name.to_string(),
            size,
            bold: false,
            italic: false,
            line_spacing: size + 4,
            char_spacing: 0,
            font_height: size,
            ascent: (size as f64 * 0.85) as i32,
        }
    }

    /// 获取字符串宽度
    pub fn string_width(&self, text: &str) -> i32 {
        let chars = text.chars().count() as i32;
        (chars * self.size * 3 / 5) + (chars - 1).max(0) * self.char_spacing
    }

    /// 获取字符串高度
    pub fn string_height(&self, _text: &str) -> i32 {
        self.font_height
    }
}
