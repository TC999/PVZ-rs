// PvZ Portable Rust 翻译 — ReanimationLawn（重动画缓存）
// 对应 C++ src/Lawn/System/ReanimationLawn.h / ReanimationLawn.cpp

#![allow(dead_code)]

use crate::framework::graphics::memory_image::MemoryImage;
use crate::framework::graphics::graphics::Graphics;
use crate::lawn::game_enums::*;
use crate::todlib::reanimator::Reanimation;
use crate::todlib::tod_list::TodList;

/// 重动画缓存图像变体（对应 C++ ReanimCacheImageVariation）
pub struct ReanimCacheImageVariation {
    pub seed_type: SeedType,
    pub draw_variation: DrawVariation,
    pub image: Option<*mut MemoryImage>,
}

/// 图像变体列表（对应 C++ typedef TodList<ReanimCacheImageVariation> ImageVariationList）
pub type ImageVariationList = TodList<ReanimCacheImageVariation>;

/// 重动画缓存（对应 C++ ReanimatorCache）
pub struct ReanimatorCache {
    pub plant_images: [Option<*mut MemoryImage>; 77],  // NUM_SEED_TYPES
    pub image_variation_list: ImageVariationList,
    pub lawn_mowers: [Option<*mut MemoryImage>; 4],    // NUM_MOWER_TYPES
    pub zombie_images: [Option<*mut MemoryImage>; 37], // NUM_CACHED_ZOMBIE_TYPES
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
}

impl ReanimatorCache {
    pub fn new() -> Self {
        ReanimatorCache {
            plant_images: [None; 77],
            image_variation_list: TodList::new(),
            lawn_mowers: [None; 4],
            zombie_images: [None; 37],
            app: None,
        }
    }

    pub fn reanimator_cache_initialize(&mut self) {
        // TODO: 从 ReanimationLawn.cpp 翻译
    }

    pub fn reanimator_cache_dispose(&mut self) {
        // TODO: 从 ReanimationLawn.cpp 翻译
    }

    pub fn draw_cached_plant(&self, g: &mut Graphics, pos_x: f32, pos_y: f32, seed_type: SeedType, draw_variation: DrawVariation) {
        // TODO: 从 ReanimationLawn.cpp 翻译
    }

    pub fn draw_cached_mower(&self, g: &mut Graphics, pos_x: f32, pos_y: f32, mower_type: LawnMowerType) {
        // TODO: 从 ReanimationLawn.cpp 翻译
    }

    pub fn draw_cached_zombie(&self, g: &mut Graphics, pos_x: f32, pos_y: f32, zombie_type: ZombieType) {
        // TODO: 从 ReanimationLawn.cpp 翻译
    }

    pub fn make_blank_memory_image(&self, width: i32, height: i32) -> Option<*mut MemoryImage> {
        // TODO: 从 ReanimationLawn.cpp 翻译
        None
    }

    pub fn make_cached_plant_frame(&self, seed_type: SeedType, draw_variation: DrawVariation) -> Option<*mut MemoryImage> {
        // TODO: 从 ReanimationLawn.cpp 翻译
        None
    }

    pub fn make_cached_mower_frame(&self, mower_type: LawnMowerType) -> Option<*mut MemoryImage> {
        // TODO: 从 ReanimationLawn.cpp 翻译
        None
    }

    pub fn make_cached_zombie_frame(&self, zombie_type: ZombieType) -> Option<*mut MemoryImage> {
        // TODO: 从 ReanimationLawn.cpp 翻译
        None
    }

    pub fn get_plant_image_size(&self, seed_type: SeedType, offset_x: &mut i32, offset_y: &mut i32, width: &mut i32, height: &mut i32) {
        // TODO: 从 ReanimationLawn.cpp 翻译
    }

    pub fn draw_reanimator_frame(&self, g: &mut Graphics, pos_x: f32, pos_y: f32, reanimation_type: ReanimationType, track_name: &str, draw_variation: DrawVariation) {
        // TODO: 从 ReanimationLawn.cpp 翻译
    }

    pub fn update_reanimation_for_variation(&self, reanim: &mut Reanimation, draw_variation: DrawVariation) {
        // TODO: 从 ReanimationLawn.cpp 翻译
    }
}

impl Default for ReanimatorCache {
    fn default() -> Self {
        ReanimatorCache::new()
    }
}
