// PvZ Portable Rust 翻译 — FilterEffect 滤镜效果
// 对应 C++ src/Sexy.TodLib/FilterEffect.h / FilterEffect.cpp
//
// 提供颜色滤镜效果：褪色、半褪色、黑白等，用于游戏中的僵尸死亡效果等。

#![allow(dead_code)]

use std::collections::HashMap;
use crate::framework::graphics::image::Image;
use crate::framework::graphics::memory_image::MemoryImage;

/// 滤镜效果枚举（对应 C++ FilterEffect）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FilterEffectType {
    None = -1,
    WashedOut = 0,
    LessWashedOut = 1,
    White = 2,
}

pub const NUM_FILTER_EFFECTS: usize = 3;

/// 全局滤镜缓存映射表（对应 C++ gFilterMap）
pub static mut G_FILTER_MAP: [Option<HashMap<*mut Image, *mut Image>>; NUM_FILTER_EFFECTS] =
    [None, None, None];

/// 初始化滤镜系统（对应 C++ FilterEffectInitForApp）
pub fn filter_effect_init_for_app() {
    for i in 0..NUM_FILTER_EFFECTS {
        unsafe { G_FILTER_MAP[i] = Some(HashMap::new()); }
    }
}

/// 清理滤镜系统（对应 C++ FilterEffectDisposeForApp）
pub fn filter_effect_dispose_for_app() {
    for i in 0..NUM_FILTER_EFFECTS {
        unsafe { G_FILTER_MAP[i] = None; }
    }
}

/// 应用亮度/饱和度滤镜（对应 C++ FilterEffectDoLumSat）
pub fn filter_effect_do_lum_sat(image: &mut MemoryImage, lum: f32, sat: f32) {
    let pixels = image.get_bits_u32_mut();
    for pixel in pixels.iter_mut() {
        let a = (*pixel >> 24) & 0xFF;
        let r = (*pixel >> 16) & 0xFF;
        let g = (*pixel >> 8) & 0xFF;
        let b = *pixel & 0xFF;

        // 灰度计算
        let gray = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as i32;

        // 饱和度调整
        let nr = ((gray as f32 + (r as f32 - gray as f32) * sat) + lum * 255.0).clamp(0.0, 255.0) as u8;
        let ng = ((gray as f32 + (g as f32 - gray as f32) * sat) + lum * 255.0).clamp(0.0, 255.0) as u8;
        let nb = ((gray as f32 + (b as f32 - gray as f32) * sat) + lum * 255.0).clamp(0.0, 255.0) as u8;

        *pixel = (a as u32) << 24 | (nr as u32) << 16 | (ng as u32) << 8 | nb as u32;
    }
}

/// 应用褪色效果（对应 C++ FilterEffectDoWashedOut）
pub fn filter_effect_do_washed_out(image: &mut MemoryImage) {
    filter_effect_do_lum_sat(image, 0.2, 0.4);
}

/// 应用半褪色效果（对应 C++ FilterEffectDoLessWashedOut）
pub fn filter_effect_do_less_washed_out(image: &mut MemoryImage) {
    filter_effect_do_lum_sat(image, 0.1, 0.6);
}

/// 应用纯白效果（对应 C++ FilterEffectDoWhite）
pub fn filter_effect_do_white(image: &mut MemoryImage) {
    let pixels = image.get_bits_u32_mut();
    for pixel in pixels.iter_mut() {
        let a = (*pixel >> 24) & 0xFF;
        *pixel = (a as u32) << 24 | 0x00FFFFFF;
    }
}

/// 创建滤镜后的图像副本（对应 C++ FilterEffectCreateImage）
pub fn filter_effect_create_image(src: &Image, effect: FilterEffectType) -> *mut MemoryImage {
    // 简化实现：创建 MemoryImage 并应用滤镜
    let mut mi = Box::new(MemoryImage::new(src.get_width(), src.get_height()));
    // 复制像素数据
    mi.create(src.get_width(), src.get_height());
    // 应用滤镜
    match effect {
        FilterEffectType::WashedOut => filter_effect_do_washed_out(&mut mi),
        FilterEffectType::LessWashedOut => filter_effect_do_less_washed_out(&mut mi),
        FilterEffectType::White => filter_effect_do_white(&mut mi),
        FilterEffectType::None => {}
    }
    Box::into_raw(mi)
}

/// 获取滤镜图像（带缓存，对应 C++ FilterEffectGetImage）
pub fn filter_effect_get_image(src: *mut Image, effect: FilterEffectType) -> *mut Image {
    if effect == FilterEffectType::None { return src; }
    let idx = effect as usize;
    unsafe {
        if let Some(ref map) = G_FILTER_MAP[idx] {
            if let Some(&cached) = map.get(&src) {
                return cached;
            }
        }
    }
    let new_img = filter_effect_create_image(unsafe { &*src }, effect) as *mut Image;
    unsafe {
        if let Some(ref mut map) = G_FILTER_MAP[idx] {
            map.insert(src, new_img);
        }
    }
    new_img
}
