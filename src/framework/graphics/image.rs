// PvZ Portable Rust 翻译 — 图像基类
// 对应 C++ SexyAppFramework/graphics/Image.h / Image.cpp

#![allow(dead_code)]

use crate::framework::rect::Rect;
use crate::framework::color::Color;

/// 图像基类
#[derive(Debug)]
pub struct Image {
    pub width: i32,
    pub height: i32,
    /// 图像的宽度（可能小于实际宽度用于像素对齐）
    pub width_align: i32,
    pub x: f64,
    pub y: f64,
    /// 是否已提交到显卡
    pub committed_bits: bool,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        Image {
            width,
            height,
            width_align: 4,
            x: 0.0,
            y: 0.0,
            committed_bits: false,
        }
    }

    pub fn get_width(&self) -> i32 { self.width }
    pub fn get_height(&self) -> i32 { self.height }
    pub fn get_x(&self) -> f64 { self.x }
    pub fn get_y(&self) -> f64 { self.y }

    pub fn set_x(&mut self, x: f64) { self.x = x; }
    pub fn set_y(&mut self, y: f64) { self.y = y; }
    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// 获取图像的逻辑矩形区域
    pub fn get_rect(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }
}

impl Default for Image {
    fn default() -> Self {
        Image::new(0, 0)
    }
}
