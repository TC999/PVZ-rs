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
    pub plant_images: [Option<*mut MemoryImage>; NUM_SEED_TYPES],  // 对应 C++ mPlantImages[SeedType::NUM_SEED_TYPES]
    pub image_variation_list: ImageVariationList,
    pub lawn_mowers: [Option<*mut MemoryImage>; 4],    // NUM_MOWER_TYPES
    pub zombie_images: [Option<*mut MemoryImage>; NUM_CACHED_ZOMBIE_TYPES], // 对应 C++ mZombieImages[ZombieType::NUM_CACHED_ZOMBIE_TYPES]
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
}

impl ReanimatorCache {
    pub fn new() -> Self {
        ReanimatorCache {
            plant_images: [None; NUM_SEED_TYPES],
            image_variation_list: TodList::new(),
            lawn_mowers: [None; 4],
            zombie_images: [None; NUM_CACHED_ZOMBIE_TYPES],
            app: None,
        }
    }

    pub fn reanimator_cache_initialize(&mut self) {
        // 对应 C++ ReanimatorCacheInitialize（ReanimationLawn.cpp:340）：mApp = gSexyAppBase + 清零三个图像数组
        self.app = crate::lawn::lawn_app::LawnApp::instance().map(|app| app as *mut _);
        self.plant_images = [None; NUM_SEED_TYPES];
        self.lawn_mowers = [None; 4];
        self.zombie_images = [None; NUM_CACHED_ZOMBIE_TYPES];
    }

    pub fn reanimator_cache_dispose(&mut self) {
        // [TRANSLATION_NOTE]: C++ 中 delete 各图像并置 nullptr（图像由缓存独占）且
        // while 循环 RemoveHead 释放变体列表节点；Rust 图像对象由 resource_manager
        // 统一管理，此处仅清引用（对应置空语义），列表以新空列表替换（无已加节点）
        self.plant_images = [None; NUM_SEED_TYPES];
        self.image_variation_list = TodList::new();
        self.lawn_mowers = [None; 4];
        self.zombie_images = [None; NUM_CACHED_ZOMBIE_TYPES];
    }

    pub fn update_reanimation_for_variation(&self, reanim: &mut Reanimation, draw_variation: DrawVariation) {
        // 对应 C++ UpdateReanimationForVariation
        let dv = draw_variation as i32;
        if dv >= DrawVariation::MarigoldWhite as i32 && dv <= DrawVariation::MarigoldLightGreen as i32 {
            let a_variation_index = (dv - DrawVariation::MarigoldWhite as i32) as usize;
            let marigold_variations = [
                crate::framework::color::Color::new(255, 255, 255, 255),
                crate::framework::color::Color::new(230, 30, 195, 255),
                crate::framework::color::Color::new(250, 125, 5, 255),
                crate::framework::color::Color::new(255, 145, 215, 255),
                crate::framework::color::Color::new(160, 255, 245, 255),
                crate::framework::color::Color::new(230, 30, 30, 255),
                crate::framework::color::Color::new(5, 130, 255, 255),
                crate::framework::color::Color::new(195, 55, 235, 255),
                crate::framework::color::Color::new(235, 210, 255, 255),
                crate::framework::color::Color::new(255, 245, 55, 255),
                crate::framework::color::Color::new(180, 255, 105, 255),
            ];
            if a_variation_index < marigold_variations.len() {
                reanim.m_color_override = marigold_variations[a_variation_index];
            }
        } else {
            match draw_variation {
                DrawVariation::Imitater => {
                    // [TRANSLATION_NOTE]: FILTER_EFFECT_WASHED_OUT 依赖滤镜系统，暂以颜色近似
                    reanim.m_color_override = crate::framework::color::Color::new(200, 200, 200, 255);
                }
                DrawVariation::ImitaterLess => {
                    reanim.m_color_override = crate::framework::color::Color::new(220, 220, 220, 255);
                }
                DrawVariation::ZenGarden => {
                    reanim.set_frames_for_layer("anim_zengarden");
                }
                DrawVariation::ZenGardenWater => {
                    reanim.set_frames_for_layer("anim_waterplants");
                }
                DrawVariation::Aquarium => {
                    reanim.set_frames_for_layer("anim_idle_aquarium");
                }
                DrawVariation::SproutNoFlower => {
                    reanim.set_frames_for_layer("anim_idle_noflower");
                }
                _ => {}
            }
        }
    }

    pub fn draw_reanimator_frame(&self, g: &mut Graphics, pos_x: f32, pos_y: f32, reanimation_type: ReanimationType, track_name: &str, draw_variation: DrawVariation) {
        // 对应 C++ DrawReanimatorFrame（实时创建 reanim 绘制，绕过内存缓存）
        let mut a_reanim = Reanimation::new();
        a_reanim.reanimation_initialize_type(pos_x, pos_y, reanimation_type);
        if !track_name.is_empty() && a_reanim.track_exists(track_name) {
            a_reanim.set_frames_for_layer(track_name);
        }
        if reanimation_type == ReanimationType::Sunflower {
            a_reanim.m_anim_time = 0.15;
        }
        a_reanim.assign_render_group_to_track("anim_waterline", -1); // RENDER_GROUP_HIDDEN
        if draw_variation != DrawVariation::Normal {
            self.update_reanimation_for_variation(&mut a_reanim, draw_variation);
        }
        a_reanim.draw(g);
    }

    pub fn draw_cached_plant(&self, g: &mut Graphics, pos_x: f32, pos_y: f32, seed_type: SeedType, draw_variation: DrawVariation) {
        // 对应 C++ DrawCachedPlant：实时绘制而非缓存内存图
        let a_plant_def = crate::lawn::plant::get_plant_definition(seed_type);
        let a_offset_x = -20;
        let a_offset_y = -20;
        let track = if seed_type == SeedType::PotatoMine { "anim_armed" } else { "anim_idle" };
        let mut a_reanim = Reanimation::new();
        a_reanim.reanimation_initialize_type(pos_x + a_offset_x as f32, pos_y + a_offset_y as f32, a_plant_def.reanimation_type);
        if a_reanim.track_exists(track) {
            a_reanim.set_frames_for_layer(track);
        }
        if draw_variation != DrawVariation::Normal {
            self.update_reanimation_for_variation(&mut a_reanim, draw_variation);
        }
        a_reanim.draw(g);
    }

    pub fn draw_cached_mower(&self, g: &mut Graphics, pos_x: f32, pos_y: f32, mower_type: LawnMowerType) {
        // 对应 C++ DrawCachedMower：实时绘制割草机
        let a_reanim_type = match mower_type {
            LawnMowerType::Lawn => ReanimationType::Lawnmower,
            LawnMowerType::Pool => ReanimationType::PoolCleaner,
            LawnMowerType::Roof => ReanimationType::RoofCleaner,
            LawnMowerType::SuperMower => ReanimationType::Lawnmower,
            _ => return,
        };
        let mut a_reanim = Reanimation::new();
        a_reanim.reanimation_initialize_type(pos_x - 20.0, pos_y, a_reanim_type);
        if a_reanim.track_exists("anim_normal") {
            a_reanim.set_frames_for_layer("anim_normal");
        }
        a_reanim.draw(g);
    }

    pub fn draw_cached_zombie(&self, g: &mut Graphics, pos_x: f32, pos_y: f32, zombie_type: ZombieType) {
        // 对应 C++ DrawCachedZombie：实时绘制僵尸缓存图（Boss 用头、普通用 anim_idle）
        // [TRANSLATION_NOTE]: C++ 先查 mZombieImages[theZombieType] 缓存，未命中才调
        // MakeCachedZombieFrame 生成；Rust 侧为实时绘制简化版。
        // 对应 C++ MakeCachedZombieFrame（ReanimationLawn.cpp:268）：缓存类型 34
        // 以 ZOMBIE_POLEVAULTER 定义生成带杆帧；此处先转换再查表，避免 ZOMBIE_DEFS[34] 越界
        let a_use_zombie_type = if zombie_type == ZombieType::CachedPolevaulterWithPole {
            ZombieType::Polevaulter
        } else {
            zombie_type
        };
        let a_zombie_def = crate::lawn::zombie::get_zombie_definition(a_use_zombie_type);
        if a_zombie_def.reanimation_type == ReanimationType::None {
            return;
        }
        let mut a_reanim = Reanimation::new();
        let (a_pos_x, a_pos_y) = if a_zombie_def.reanimation_type == ReanimationType::Boss {
            (-524.0, -88.0)
        } else {
            (40.0, 40.0)
        };
        a_reanim.reanimation_initialize_type(pos_x + a_pos_x, pos_y + a_pos_y, a_zombie_def.reanimation_type);
        let track = if zombie_type == ZombieType::Pogo { "anim_pogo" } else { "anim_idle" };
        if a_reanim.track_exists(track) {
            a_reanim.set_frames_for_layer(track);
        }
        a_reanim.draw(g);
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
}

impl Default for ReanimatorCache {
    fn default() -> Self {
        ReanimatorCache::new()
    }
}
