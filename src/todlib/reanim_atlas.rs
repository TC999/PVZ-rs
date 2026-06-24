// PvZ Portable Rust 翻译 — ReanimAtlas 动画图集
// 对应 C++ src/Sexy.TodLib/ReanimAtlas.h

#![allow(dead_code)]

use crate::framework::graphics::image::Image;
use crate::framework::graphics::memory_image::MemoryImage;

/// 图集中的单张图像信息（对应 C++ ReanimAtlasImage）
#[derive(Debug, Clone, Copy)]
pub struct ReanimAtlasImage {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub original_image: *mut Image,
}

impl ReanimAtlasImage {
    pub fn new() -> Self { ReanimAtlasImage { x: 0, y: 0, width: 0, height: 0, original_image: std::ptr::null_mut() } }
}

/// 按高度降序排序的比较函数
pub fn sort_by_non_increasing_height(a: &ReanimAtlasImage, b: &ReanimAtlasImage) -> std::cmp::Ordering {
    b.height.cmp(&a.height)
}

/// 动画图集（对应 C++ ReanimAtlas）
pub struct ReanimAtlas {
    pub image_array: Vec<ReanimAtlasImage>,
    pub memory_image: *mut MemoryImage,
}

impl ReanimAtlas {
    pub fn new() -> Self { ReanimAtlas { image_array: Vec::new(), memory_image: std::ptr::null_mut() } }

    /// 创建图集纹理（将多张图像合并到一张大纹理中）
    pub fn create(&mut self, _max_width: i32, _max_height: i32) { }
}
