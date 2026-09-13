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

    pub fn draw_cached_plant(&mut self, g: &mut Graphics, pos_x: f32, pos_y: f32, seed_type: SeedType, draw_variation: DrawVariation) {
        // 对应 C++ DrawCachedPlant（ReanimationLawn.cpp:377）
        // C++: PVZP_ASSERT(theSeedType >= 0 && theSeedType < SeedType::NUM_SEED_TYPES)
        let a_seed_idx = seed_type as i32;
        if a_seed_idx < 0 || a_seed_idx >= NUM_SEED_TYPES as i32 {
            return;
        }
        let a_seed_idx = a_seed_idx as usize;

        let mut a_image = None;
        if draw_variation != DrawVariation::Normal {
            // C++: 遍历 mImageVariationList 查找 (seedType, drawVariation) 匹配的缓存项
            let mut a_node = self.image_variation_list.head;
            while !a_node.is_null() {
                unsafe {
                    let a_image_variation = &(*a_node).value;
                    if a_image_variation.seed_type == seed_type && a_image_variation.draw_variation == draw_variation {
                        a_image = a_image_variation.image;
                        break;
                    }
                    a_node = (*a_node).next;
                }
            }
            if a_image.is_none() {
                a_image = self.make_cached_plant_frame(seed_type, draw_variation);
                let a_new_image_variation = ReanimCacheImageVariation {
                    seed_type,
                    draw_variation,
                    image: a_image,
                };
                // C++: mImageVariationList.AddHead(aNewImageVariation)
                self.image_variation_list.add_head_value(a_new_image_variation);
            }
        } else {
            a_image = self.plant_images[a_seed_idx];
            if a_image.is_none() {
                a_image = self.make_cached_plant_frame(seed_type, DrawVariation::Normal);
                self.plant_images[a_seed_idx] = a_image;
            }
        }

        // C++: GetPlantImageSize + DrawImage（3D 加速/scale 分支以 draw_image_f_xy 近似）
        if let Some(img) = a_image {
            let mut a_offset_x = 0i32;
            let mut a_offset_y = 0i32;
            let mut a_width = 0i32;
            let mut a_height = 0i32;
            self.get_plant_image_size(seed_type, &mut a_offset_x, &mut a_offset_y, &mut a_width, &mut a_height);
            unsafe {
                g.draw_image_f_xy(&(*img).base, pos_x + a_offset_x as f32, pos_y + a_offset_y as f32);
            }
        }
    }

    pub fn draw_cached_mower(&mut self, g: &mut Graphics, pos_x: f32, pos_y: f32, mower_type: LawnMowerType) {
        // 对应 C++ DrawCachedMower（ReanimationLawn.cpp:422）
        // C++: PVZP_ASSERT(theMowerType >= 0 && theMowerType < LawnMowerType::NUM_MOWER_TYPES)
        let a_mower_idx = mower_type as usize;
        if a_mower_idx >= NUM_MOWER_TYPES {
            return;
        }
        if self.lawn_mowers[a_mower_idx].is_none() {
            self.lawn_mowers[a_mower_idx] = self.make_cached_mower_frame(mower_type);
        }
        if let Some(img) = self.lawn_mowers[a_mower_idx] {
            // C++: PvzpDrawImageScaledF(g, mLawnMowers[theMowerType], thePosX - 20.0f, thePosY, ...)
            unsafe {
                g.draw_image_f_xy(&(*img).base, pos_x - 20.0, pos_y);
            }
        }
    }

    pub fn draw_cached_zombie(&mut self, g: &mut Graphics, pos_x: f32, pos_y: f32, zombie_type: ZombieType) {
        // 对应 C++ DrawCachedZombie（ReanimationLawn.cpp:431）
        // C++: PVZP_ASSERT(theZombieType >= 0 && theZombieType < ZombieType::NUM_CACHED_ZOMBIE_TYPES)
        let a_zombie_idx = zombie_type as i32;
        if a_zombie_idx < 0 || a_zombie_idx >= NUM_CACHED_ZOMBIE_TYPES as i32 {
            return;
        }
        let a_zombie_idx = a_zombie_idx as usize;
        if self.zombie_images[a_zombie_idx].is_none() {
            self.zombie_images[a_zombie_idx] = self.make_cached_zombie_frame(zombie_type);
        }
        if let Some(img) = self.zombie_images[a_zombie_idx] {
            // C++: PvzpDrawImageScaledF(g, mZombieImages[theZombieType], thePosX, thePosY, ...)
            unsafe {
                g.draw_image_f_xy(&(*img).base, pos_x, pos_y);
            }
        }
    }

    pub fn make_blank_memory_image(&self, width: i32, height: i32) -> Option<*mut MemoryImage> {
        // 对应 C++ MakeBlankMemoryImage（ReanimationLawn.cpp:115）：
        // new MemoryImage + 分配置零像素 + mHasTrans/mHasAlpha = true
        // [TRANSLATION_NOTE]: C++ 尾部写入 MEMORYCHECK_ID 哨兵做越界检测；Rust Vec 自带边界检查，省略
        let mut img = Box::new(MemoryImage::new(width, height));
        img.has_trans = true;
        img.has_alpha = true;
        Some(Box::into_raw(img))
    }

    pub fn make_cached_plant_frame(&self, seed_type: SeedType, draw_variation: DrawVariation) -> Option<*mut MemoryImage> {
        // 对应 C++ MakeCachedPlantFrame（ReanimationLawn.cpp:205）
        let mut a_offset_x = 0i32;
        let mut a_offset_y = 0i32;
        let mut a_width = 0i32;
        let mut a_height = 0i32;
        self.get_plant_image_size(seed_type, &mut a_offset_x, &mut a_offset_y, &mut a_width, &mut a_height);
        let mut a_memory_image = self.make_blank_memory_image(a_width, a_height)?;
        let img_ptr = a_memory_image as *mut MemoryImage;
        let mut a_memory_graphics = Graphics::new_with_image(unsafe { &mut (*img_ptr).base } as *mut _);
        a_memory_graphics.set_linear_blend(true);

        let a_plant_def = crate::lawn::plant::get_plant_definition(seed_type);
        if seed_type == SeedType::PotatoMine {
            a_memory_graphics.scale_x = 0.85;
            a_memory_graphics.scale_y = 0.85;
            self.draw_reanimator_frame(&mut a_memory_graphics, -(a_offset_x - 12) as f32, -(a_offset_y - 12) as f32, a_plant_def.reanimation_type, "anim_armed", draw_variation);
        } else if seed_type == SeedType::InstantCoffee {
            a_memory_graphics.scale_x = 0.8;
            a_memory_graphics.scale_y = 0.8;
            self.draw_reanimator_frame(&mut a_memory_graphics, -(a_offset_x - 12) as f32, -(a_offset_y - 12) as f32, a_plant_def.reanimation_type, "anim_idle", draw_variation);
        } else if seed_type == SeedType::ExplodeONut {
            a_memory_graphics.set_colorize_images(true);
            a_memory_graphics.set_color(&crate::framework::color::Color::new(255, 64, 64, 255));
            self.draw_reanimator_frame(&mut a_memory_graphics, -a_offset_x as f32, -a_offset_y as f32, a_plant_def.reanimation_type, "anim_idle", draw_variation);
        } else {
            self.draw_reanimator_frame(&mut a_memory_graphics, -a_offset_x as f32, -a_offset_y as f32, a_plant_def.reanimation_type, "anim_idle", draw_variation);
            if seed_type == SeedType::Peashooter || seed_type == SeedType::Snowpea
                || seed_type == SeedType::Repeater || seed_type == SeedType::Leftpeater
                || seed_type == SeedType::Gatlingpea
            {
                self.draw_reanimator_frame(&mut a_memory_graphics, -a_offset_x as f32, -a_offset_y as f32, a_plant_def.reanimation_type, "anim_head_idle", draw_variation);
            } else if seed_type == SeedType::Splitpea {
                self.draw_reanimator_frame(&mut a_memory_graphics, -a_offset_x as f32, -a_offset_y as f32, a_plant_def.reanimation_type, "anim_head_idle", draw_variation);
                self.draw_reanimator_frame(&mut a_memory_graphics, -a_offset_x as f32, -a_offset_y as f32, a_plant_def.reanimation_type, "anim_splitpea_idle", draw_variation);
            } else if seed_type == SeedType::Threepeater {
                self.draw_reanimator_frame(&mut a_memory_graphics, -a_offset_x as f32, -a_offset_y as f32, a_plant_def.reanimation_type, "anim_head_idle1", draw_variation);
                self.draw_reanimator_frame(&mut a_memory_graphics, -a_offset_x as f32, -a_offset_y as f32, a_plant_def.reanimation_type, "anim_head_idle3", draw_variation);
                self.draw_reanimator_frame(&mut a_memory_graphics, -a_offset_x as f32, -a_offset_y as f32, a_plant_def.reanimation_type, "anim_head_idle2", draw_variation);
            }
        }
        Some(a_memory_image)
    }

    pub fn make_cached_mower_frame(&self, mower_type: LawnMowerType) -> Option<*mut MemoryImage> {
        // 对应 C++ MakeCachedMowerFrame（ReanimationLawn.cpp:149）
        let mut a_memory_image = self.make_blank_memory_image(90, 100)?;
        let img_ptr = a_memory_image as *mut MemoryImage;
        let mut a_memory_graphics = Graphics::new_with_image(unsafe { &mut (*img_ptr).base } as *mut _);
        a_memory_graphics.set_linear_blend(true);

        match mower_type {
            LawnMowerType::Lawn => {
                a_memory_graphics.scale_x = 0.85;
                a_memory_graphics.scale_y = 0.85;
                self.draw_reanimator_frame(&mut a_memory_graphics, 10.0, 0.0, ReanimationType::Lawnmower, "anim_normal", DrawVariation::Normal);
            }
            LawnMowerType::Pool => {
                a_memory_graphics.scale_x = 0.8;
                a_memory_graphics.scale_y = 0.8;
                self.draw_reanimator_frame(&mut a_memory_graphics, 10.0, 25.0, ReanimationType::PoolCleaner, "", DrawVariation::Normal);
            }
            LawnMowerType::Roof => {
                a_memory_graphics.scale_x = 0.85;
                a_memory_graphics.scale_y = 0.85;
                self.draw_reanimator_frame(&mut a_memory_graphics, 10.0, 0.0, ReanimationType::RoofCleaner, "", DrawVariation::Normal);
            }
            LawnMowerType::SuperMower => {
                a_memory_graphics.scale_x = 0.85;
                a_memory_graphics.scale_y = 0.85;
                self.draw_reanimator_frame(&mut a_memory_graphics, 10.0, 0.0, ReanimationType::Lawnmower, "anim_tricked", DrawVariation::Normal);
            }
            // C++ default: PVZP_ASSERT(false)
            _ => return None,
        }
        Some(a_memory_image)
    }

    pub fn make_cached_zombie_frame(&self, zombie_type: ZombieType) -> Option<*mut MemoryImage> {
        // 对应 C++ MakeCachedZombieFrame（ReanimationLawn.cpp:261）
        let mut a_memory_image = self.make_blank_memory_image(200, 210)?;
        let img_ptr = a_memory_image as *mut MemoryImage;
        let mut a_memory_graphics = Graphics::new_with_image(unsafe { &mut (*img_ptr).base } as *mut _);
        a_memory_graphics.set_linear_blend(true);

        // C++: theZombieType == ZOMBIE_CACHED_POLEVAULTER_WITH_POLE → aUseZombieType = ZOMBIE_POLEVAULTER
        let a_use_zombie_type = if zombie_type == ZombieType::CachedPolevaulterWithPole {
            ZombieType::Polevaulter
        } else {
            zombie_type
        };
        let a_zombie_def = crate::lawn::zombie::get_zombie_definition(a_use_zombie_type);
        // C++: PVZP_ASSERT(aZombieDef.mReanimationType != ReanimationType::REANIM_NONE)
        if a_zombie_def.reanimation_type == ReanimationType::None {
            return Some(a_memory_image);
        }

        let mut a_pos_x = 40.0f32;
        let mut a_pos_y = 40.0f32;
        if a_zombie_def.reanimation_type == ReanimationType::Zombie {
            let mut a_reanim = Reanimation::new();
            a_reanim.reanimation_initialize_type(a_pos_x, a_pos_y, a_zombie_def.reanimation_type);
            a_reanim.set_frames_for_layer("anim_idle");
            crate::lawn::zombie::Zombie::setup_reanim_layers(&mut a_reanim as *mut Reanimation, a_use_zombie_type);

            // C++: theZombieType == ZOMBIE_DOOR 时门板渲染组归位
            if zombie_type == ZombieType::Door {
                a_reanim.assign_render_group_to_track("anim_screendoor", 0); // RENDER_GROUP_NORMAL
            } else if zombie_type == ZombieType::Flag {
                let mut a_reanim_flag = Reanimation::new();
                a_reanim_flag.reanimation_initialize_type(a_pos_x, a_pos_y, ReanimationType::Flag);
                a_reanim_flag.set_frames_for_layer("Zombie_flag");
                a_reanim_flag.draw(&mut a_memory_graphics);
            }
            a_reanim.draw(&mut a_memory_graphics);
        } else if a_zombie_def.reanimation_type == ReanimationType::Boss {
            let mut a_reanim = Reanimation::new();
            a_reanim.reanimation_initialize_type(-524.0, -88.0, a_zombie_def.reanimation_type);
            a_reanim.set_frames_for_layer("anim_head_idle");
            let mut a_reanim_driver = Reanimation::new();
            a_reanim_driver.reanimation_initialize_type(46.0, 22.0, ReanimationType::BossDriver);
            a_reanim_driver.set_frames_for_layer("anim_idle");

            a_reanim.draw(&mut a_memory_graphics);
            a_reanim_driver.draw(&mut a_memory_graphics);
            // C++: 隐藏 boss_body1/boss_neck/boss_head2 后重绘主体
            a_reanim.assign_render_group_to_track("boss_body1", -1); // RENDER_GROUP_HIDDEN
            a_reanim.assign_render_group_to_track("boss_neck", -1);
            a_reanim.assign_render_group_to_track("boss_head2", -1);
            a_reanim.draw(&mut a_memory_graphics);
        } else {
            // C++: 按类型选择轨道名（Gargantuar 仅调 aPosY）
            let mut a_track_name = "anim_idle";
            if zombie_type == ZombieType::Pogo {
                a_track_name = "anim_pogo";
            } else if zombie_type == ZombieType::CachedPolevaulterWithPole {
                a_track_name = "anim_idle";
            } else if zombie_type == ZombieType::Polevaulter {
                a_track_name = "anim_walk";
            } else if zombie_type == ZombieType::Gargantuar {
                a_pos_y = 60.0;
            }
            self.draw_reanimator_frame(&mut a_memory_graphics, a_pos_x, a_pos_y, a_zombie_def.reanimation_type, a_track_name, DrawVariation::Normal);
        }
        Some(a_memory_image)
    }

    pub fn get_plant_image_size(&self, seed_type: SeedType, offset_x: &mut i32, offset_y: &mut i32, width: &mut i32, height: &mut i32) {
        // 对应 C++ GetPlantImageSize（ReanimationLawn.cpp:133）
        *offset_x = -20;
        *offset_y = -20;
        *width = 120;
        *height = 120;
        if seed_type == SeedType::Tallnut {
            *offset_y = -40;
            *height += 40;
        } else if seed_type == SeedType::Melonpult || seed_type == SeedType::Wintermelon {
            *offset_x = -40;
            *width += 40;
        } else if seed_type == SeedType::Cobcannon {
            *width += 80;
        }
    }
}

impl Default for ReanimatorCache {
    fn default() -> Self {
        ReanimatorCache::new()
    }
}
