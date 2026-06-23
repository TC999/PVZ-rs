// PvZ Portable Rust 翻译 — MemoryImage（内存图像）
// 对应 C++ SexyAppFramework/graphics/MemoryImage.h / MemoryImage.cpp

#![allow(dead_code)]

use crate::framework::graphics::image::Image;
use crate::framework::color::Color;
use crate::framework::rect::Rect;

/// 内存图像（像素数据在系统内存中）
#[derive(Debug)]
pub struct MemoryImage {
    pub base: Image,
    /// 像素数据（RGBA 格式）
    pub bits: Vec<u32>,
    /// 原始行宽（像素）
    pub row_width: i32,
    /// 原始列高
    pub row_height: i32,
    /// 是否拥有 alpha 通道
    pub has_alpha: bool,
    /// 是否已从显卡恢复
    pub bits_changed_count: i32,
    /// 是否已提交到显卡
    pub committed: bool,
    /// 是否等比例缩放
    pub is_screen_tex: bool,
}

impl MemoryImage {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        MemoryImage {
            base: Image::new(width, height),
            bits: vec![0; size],
            row_width: width,
            row_height: height,
            has_alpha: false,
            bits_changed_count: 1,
            committed: false,
            is_screen_tex: false,
        }
    }

    pub fn get_bits(&self) -> &[u32] {
        &self.bits
    }

    pub fn get_bits_mut(&mut self) -> &mut [u32] {
        &mut self.bits
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        let idx = (y * self.base.width + x) as usize;
        if idx < self.bits.len() {
            self.bits[idx] = color.to_argb();
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        let idx = (y * self.base.width + x) as usize;
        if idx < self.bits.len() {
            Color::from_argb(self.bits[idx])
        } else {
            Color::TRANSPARENT
        }
    }

    /// 填充矩形区域
    pub fn fill_rect(&mut self, rect: &Rect, color: Color) {
        let pixel = color.to_argb();
        for y in rect.top()..rect.bottom() {
            for x in rect.left()..rect.right() {
                let idx = (y * self.base.width + x) as usize;
                if idx < self.bits.len() {
                    self.bits[idx] = pixel;
                }
            }
        }
    }

    /// 清除所有像素
    pub fn clear(&mut self, color: Color) {
        let pixel = color.to_argb();
        self.bits.fill(pixel);
    }
}
