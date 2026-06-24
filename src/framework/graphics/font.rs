// PvZ Portable Rust 翻译 — 字体类型
// 对应 C++ SexyAppFramework/graphics/Font.h / Font.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;

/// 字体基类（对应 C++ _Font）
/// 在 C++ 中 Font 是基类，由 SysFont 和 ImageFont 继承。
/// Rust 版本将常用方法作为默认实现，子类型可以通过其他方式扩展。
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
    /// 基线偏移（从顶到基线）
    pub ascent: i32,
    /// 基线下降（从基线到底）
    pub descent: i32,
    /// 基线填充（额外留白）
    pub ascent_padding: i32,
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
            descent: (size as f64 * 0.15) as i32,
            ascent_padding: 0,
        }
    }

    /// 获取字符串宽度（默认计算）
    pub fn string_width(&self, text: &str) -> i32 {
        let mut width = 0;
        let mut prev: char = '\0';
        for c in text.chars() {
            width += self.char_width_kern(c, prev);
            prev = c;
        }
        width
    }

    /// 获取单个字符宽度（含字距调整）
    pub fn char_width_kern(&self, c: char, _prev: char) -> i32 {
        // 默认简单计算：每个字符约为 size * 0.6
        if c == ' ' {
            (self.size * 3 / 10).max(1)
        } else if c == '\t' {
            self.size * 2
        } else {
            (self.size * 6 / 10).max(1)
        }
    }

    /// 获取字符串高度
    pub fn string_height(&self, _text: &str) -> i32 {
        self.font_height
    }

    /// 获取上升高度
    pub fn get_ascent(&self) -> i32 {
        self.ascent
    }

    /// 获取上升填充
    pub fn get_ascent_padding(&self) -> i32 {
        self.ascent_padding
    }

    /// 获取下降高度
    pub fn get_descent(&self) -> i32 {
        self.descent
    }

    /// 获取行距
    pub fn get_line_spacing(&self) -> i32 {
        self.line_spacing
    }

    /// 绘制字符串（默认实现）
    /// 对于 SysFont 会使用系统字体渲染，ImageFont 使用精灵图渲染
    pub fn draw_string(
        &self,
        g: &mut crate::framework::graphics::graphics::Graphics,
        x: i32,
        y: i32,
        text: &str,
        color: &Color,
        _clip_rect: &Rect,
    ) {
        // 默认实现：使用 Graphics 填充像素模拟文字
        // 实际渲染由子类型（SysFont/ImageFont）覆盖
        let cur_x = x;
        let orig_color = g.color;
        g.set_color(color);

        // 绘制简单的占位矩形代表文本
        let text_width = self.string_width(text);
        if text_width > 0 {
            g.fill_rect_xywh(cur_x, y - self.ascent, text_width, self.font_height);
        }

        g.set_color(&orig_color);
    }
}
