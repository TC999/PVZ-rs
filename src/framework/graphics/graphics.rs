// PvZ Portable Rust 翻译 — 2D 图形上下文
// 对应 C++ SexyAppFramework/graphics/Graphics.h / Graphics.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::image::Image;
use crate::framework::graphics::font::Font;

/// 绘制模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DrawMode {
    Normal = 0,
    Additive = 1,
    Copy = 2,
}

/// 2D 图形上下文
#[derive(Debug)]
pub struct Graphics {
    /// 当前绘制目标图像
    pub dest_image: Option<*mut Image>,
    /// 裁剪矩形
    pub clip_rect: Rect,
    /// 变换偏移
    pub trans_x: f64,
    pub trans_y: f64,
    /// 绘制模式
    pub draw_mode: DrawMode,
    /// 颜色
    pub color: Color,
    /// 字体
    pub font: Option<*mut Font>,
    /// 线宽
    pub line_width: f64,
}

impl Graphics {
    pub fn new() -> Self {
        Graphics {
            dest_image: None,
            clip_rect: Rect::ZERO,
            trans_x: 0.0,
            trans_y: 0.0,
            draw_mode: DrawMode::Normal,
            color: Color::WHITE,
            font: None,
            line_width: 1.0,
        }
    }

    /// 是否为 3D 加速
    pub fn is_3d(&self) -> bool {
        true // 游戏使用 OpenGL
    }

    /// 设置裁剪矩形
    pub fn set_clip_rect(&mut self, rect: &Rect) {
        self.clip_rect = *rect;
    }

    /// 清除裁剪矩形
    pub fn clear_clip_rect(&mut self) {
        self.clip_rect = Rect::ZERO;
    }

    /// 绘制纯色矩形
    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        // 由 GLInterface 处理实际绘制
    }

    /// 绘制图像
    pub fn draw_image(&mut self, image: &Image, x: i32, y: i32) {
        self.draw_image_rect(image, x, y, &Rect::new(0, 0, image.width, image.height));
    }

    /// 绘制图像（指定源矩形）
    pub fn draw_image_rect(&mut self, _image: &Image, _x: i32, _y: i32, _src_rect: &Rect) {
        // 由 GLInterface 处理
    }

    /// 绘制图像（指定目标矩形）
    pub fn draw_image_dest(&mut self, _image: &Image, _dest_rect: &Rect, _src_rect: &Rect) {
        // 由 GLInterface 处理
    }

    /// 设置颜色
    pub fn set_color(&mut self, color: &Color) {
        self.color = *color;
    }

    /// 设置绘制模式
    pub fn set_draw_mode(&mut self, mode: DrawMode) {
        self.draw_mode = mode;
    }

    /// 平移变换
    pub fn translate(&mut self, x: f64, y: f64) {
        self.trans_x += x;
        self.trans_y += y;
    }

    /// 获取当前变换偏移
    pub fn get_translation(&self) -> (f64, f64) {
        (self.trans_x, self.trans_y)
    }

    /// 绘制字符串
    pub fn draw_string(&mut self, _text: &str, _x: i32, _y: i32) {
        // 由字体系统处理
    }

    /// 绘制线段
    pub fn draw_line(&mut self, _start_x: f64, _start_y: f64, _end_x: f64, _end_y: f64) {
        // 由 GLInterface 处理
    }

    /// 绘制字符串（带宽度和对其方式）
    pub fn draw_string_width(&mut self, _text: &str, _x: i32, _y: i32, _width: i32, _justification: i32) {
        // 由字体系统处理
    }
}

impl Default for Graphics {
    fn default() -> Self {
        Graphics::new()
    }
}
