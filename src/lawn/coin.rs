// PvZ Portable Rust 翻译 — Coin（硬币/物品掉落）
// 对应 C++ src/Lawn/Coin.h / Coin.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::image::Image;

/// 对应 C++ PURCHASE_COUNT_OFFSET（购买计数偏移基线）
const PURCHASE_COUNT_OFFSET: i32 = 1000;
// 音效常量改用 tod_foley 中由 LoadingSounds 资源加载赋值的真常量（原为占位值 0）
use crate::todlib::tod_foley::{SOUND_DIAMOND, SOUND_SEEDLIFT, SOUND_SHOVEL, SOUND_TAP2};

/// 硬币/掉落物品
pub struct Coin {
    pub base: GameObject,

    pub coin_type: CoinType,
    pub coin_motion: CoinMotion,
    pub pos_x: f32,
    pub pos_y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub destination_x: f32,
    pub destination_y: f32,
    pub lifetime: i32,
    pub lifespan: i32,
    pub frame: i32,
    pub counter: i32,
    pub value: i32,
    pub alpha: u8,
    pub dead: bool,
    pub is_sun: bool,
    pub fade_count: i32,
    pub ground_y: f32,
    pub coin_age: i32,
    pub hit_ground: bool,
    pub disappear_counter: i32,
    pub scale: f32,
    pub is_being_collected: bool,
    pub collection_distance: f32,
    /// 对应 C++ mCollectX/mCollectY（收集起点坐标，用于 UpdateCollected 动画）
    pub collect_x: f32,
    pub collect_y: f32,
    pub attachment_id: AttachmentID,
    pub needs_bouncy_arrow: bool,
    pub has_bouncy_arrow: bool,
    pub times_dropped: i32,
    /// 对应 C++ mPottedPlantSpec（盆栽礼物/植物礼物携带的盆栽规格）
    pub potted_plant_spec: crate::lawn::system::player_info::PottedPlant,
    /// 对应 C++ mUsableSeedType（可种种子包礼包的种子类型）
    pub usable_seed_type: SeedType,
}

impl Coin {
    pub fn new() -> Self {
        Coin {
            base: GameObject::new(),
            coin_type: CoinType::None,
            coin_motion: CoinMotion::FromSky,
            pos_x: 0.0,
            pos_y: 0.0,
            vel_x: 0.0,
            vel_y: 0.0,
            destination_x: 0.0,
            destination_y: 0.0,
            lifetime: 0,
            lifespan: 600,
            frame: 0,
            counter: 0,
            value: 0,
            alpha: 255,
            dead: false,
            is_sun: false,
            fade_count: 0,
            ground_y: 0.0,
            coin_age: 0,
            hit_ground: false,
            disappear_counter: 0,
            scale: 1.0,
            is_being_collected: false,
            collection_distance: 0.0,
            collect_x: 0.0,
            collect_y: 0.0,
            attachment_id: ATTACHMENTID_NULL,
            needs_bouncy_arrow: false,
            has_bouncy_arrow: false,
            times_dropped: 0,
            potted_plant_spec: crate::lawn::system::player_info::PottedPlant::new(),
            usable_seed_type: SeedType::None,
        }
    }

    /// 初始化硬币
    pub fn coin_initialize(&mut self, x: f32, y: f32, coin_type: CoinType, motion: CoinMotion) {
        self.pos_x = x;
        self.pos_y = y;
        self.coin_type = coin_type;
        self.coin_motion = motion;
        self.is_sun = matches!(coin_type, CoinType::Sun | CoinType::SmallSun | CoinType::LargeSun);
        self.value = 0;
        match coin_type {
            CoinType::Sun => {
                self.value = 25;
                self.lifespan = 600;
            },
            CoinType::SmallSun => {
                self.value = 15;
                self.lifespan = 600;
            },
            CoinType::LargeSun => {
                self.value = 50;
                self.lifespan = 600;
            },
            CoinType::Silver => {
                self.value = 1; // 对应 C++ GetCoinValue(COIN_SILVER) = 1
            },
            CoinType::Gold => {
                self.value = 5; // 对应 C++ GetCoinValue(COIN_GOLD) = 5
            },
            CoinType::Diamond => {
                self.value = 100;
            },
            _ => {
                self.value = 0;
            }
        }

        // 对应 C++ CoinInitialize 中 CoinGetsBouncyArrow/PlayLaunchSound 调用
        if self.coin_gets_bouncy_arrow() {
            self.needs_bouncy_arrow = true;
        }
        // [TRANSLATION_NOTE]: C++ 判定 mCoinMotion != COIN_MOTION_FROM_PRESENT（Rust 枚举暂无该变体）
        // Rust 侧 add_coin 先调用本初始化再设置 app/board，故此处音效/棋盘依赖在调用时尚未就绪
        self.play_launch_sound();
    }

    /// 更新（对应 C++ Coin::Update）
    pub fn update(&mut self) {
        if self.dead { return; }

        self.coin_age += 1;

        // [TRANSLATION_NOTE]: 场景检查（SCENE_PLAYING/SCENE_AWARD/Upsell）暂未实现

        if self.fade_count != 0 {
            self.update_fade();
        } else if !self.is_being_collected {
            self.update_fall();
        } else {
            self.update_collected();
        }

        // 对应 C++ Coin::Update 尾部（Coin.cpp:756-773）：
        // mAttachmentID 非空时 AttachmentUpdateAndMove + OverrideColor + OverrideScale
        if self.attachment_id != ATTACHMENTID_NULL {
            let mut a_offset_x = 0.0f32;
            let mut a_offset_y = 0.0f32;
            if self.coin_type == CoinType::Diamond {
                a_offset_x = 18.0 - 18.0 * self.scale;
                a_offset_y = 13.0 - 13.0 * self.scale;
            }
            crate::todlib::attachment::attachment_update_and_move(
                &mut self.attachment_id,
                self.pos_x + a_offset_x,
                self.pos_y + a_offset_y,
            );
            let a_color = self.get_color();
            crate::todlib::attachment::attachment_override_color(
                &mut self.attachment_id,
                &crate::framework::color::Color::new(a_color.0, a_color.1, a_color.2, a_color.3),
            );
            crate::todlib::attachment::attachment_override_scale(&mut self.attachment_id, self.scale);
            if (!self.hit_ground || self.is_being_collected)
                && (self.coin_type == CoinType::Silver || self.coin_type == CoinType::Gold)
            {
                // 对应 C++: 移动中的银/金币用静态图，隐藏附件的动画
                crate::todlib::attachment::attachment_override_color(
                    &mut self.attachment_id,
                    &crate::framework::color::Color::new(0, 0, 0, 0),
                );
            }
        }
    }

    /// 获取颜色（对应 C++ GetColor）
    /// 收集中的阳光/金钱根据距离渐隐，淡出时根据 fade_count 线性减淡
    pub fn get_color(&self) -> (u8, u8, u8, u8) {
        // GetColor — 使用 Curve 动画计算 alpha
        if self.fade_count > 0 {
            let alpha = (self.alpha as i32 * self.fade_count / 15).max(0) as u8;
            (255, 255, 255, alpha)
        } else {
            (255, 255, 255, self.alpha)
        }
    }

    /// 获取最终种子包类型（对应 C++ GetFinalSeedPacketType）
    pub fn get_final_seed_packet_type(&self) -> SeedType {
        // [TRANSLATION_NOTE]: 首次冒险模式 1-50 关返回关卡奖励种子
        SeedType::None
    }

    /// 更新掉落物理（对应 C++ Coin::UpdateFall, Coin.cpp:481）
    pub fn update_fall(&mut self) {
        // C++ 483-493: COIN_MOTION_FROM_PRESENT
        if self.coin_motion == CoinMotion::FromPresent {
            self.pos_x += self.vel_x;
            self.pos_y += self.vel_y;
            self.vel_x *= 0.95;
            self.vel_y *= 0.95;
            if self.coin_age >= 80 {
                self.collect();
            }
        } else if self.pos_y + self.vel_y < self.ground_y {
            // C++ 494-517: 下落中
            self.pos_y += self.vel_y;
            if self.coin_motion == CoinMotion::FromPlant {
                // C++ 498-500
                self.vel_y += 0.09;
            } else if self.coin_motion == CoinMotion::Coin || self.coin_motion == CoinMotion::FromBoss {
                // C++ 501-504
                self.vel_y += 0.15;
            }

            self.pos_x += self.vel_x;
            // C++ 507-516: X 边界反弹
            if self.pos_x > BOARD_WIDTH as f32 - self.base.width as f32 && self.coin_motion != CoinMotion::FromBoss {
                self.pos_x = BOARD_WIDTH as f32 - self.base.width as f32;
                self.vel_x = -0.4 - crate::todlib::tod_common::rand_range_float(0.0, 0.4);
            } else if self.pos_x < 0.0 {
                self.pos_x = 0.0;
                self.vel_x = 0.4 + crate::todlib::tod_common::rand_range_float(0.0, 0.4);
            }
        } else {
            // C++ 518-589: 落地
            // C++ 520-565: 弹跳箭头粒子
            if self.needs_bouncy_arrow && !self.has_bouncy_arrow {
                // C++ 522-544: 按 mType 计算粒子偏移
                let mut a_particle_offset_x = (self.base.width / 2) as f32;
                let mut a_particle_offset_y = (self.base.height / 2 - 60) as f32;
                if self.coin_type == CoinType::Trophy {
                    a_particle_offset_x += 2.0;
                } else if self.coin_type == CoinType::AwardMoneyBag || self.coin_type == CoinType::AwardBagDiamond {
                    a_particle_offset_x += 2.0;
                    a_particle_offset_y -= 2.0;
                } else if self.coin_type == CoinType::AwardPresent || self.is_present_with_advice() {
                    a_particle_offset_y -= 20.0;
                } else if self.coin_type == CoinType::AwardSilverSunflower || self.coin_type == CoinType::AwardGoldSunflower {
                    a_particle_offset_x -= 6.0;
                    a_particle_offset_y -= 40.0;
                } else if self.is_money() {
                    a_particle_offset_x += 12.0;
                    a_particle_offset_y += 21.0;
                }
                // C++ 548-557: 按 mType 选择 ParticleEffect
                let a_effect = if self.coin_type == CoinType::FinalSeedPacket {
                    ParticleEffect::SeedPacket
                } else if self.is_money() {
                    ParticleEffect::CoinPickupArrow
                } else {
                    ParticleEffect::AwardPickupArrow
                };
                // C++ 559-561: aParticle = mApp->AddPvzpParticle(mPosX + offsetX, mPosY + offsetY, 0, aEffect);
                //              AttachParticle(mAttachmentID, aParticle, aParticleOffsetX, aParticleOffsetY);
                if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                    if let Some(a_particle) = app.add_tod_particle(
                        self.pos_x + a_particle_offset_x,
                        self.pos_y + a_particle_offset_y,
                        0,
                        a_effect as i32,
                    ) {
                        crate::todlib::attachment::attach_particle(
                            &mut self.attachment_id,
                            a_particle as *mut std::ffi::c_void,
                            a_particle_offset_x,
                            a_particle_offset_y,
                        );
                    }
                }
                self.has_bouncy_arrow = true;
            }

            if !self.hit_ground {
                self.hit_ground = true;
                self.play_ground_sound();
            }

            self.pos_y = self.ground_y;
            self.pos_x = self.pos_x.round();

            // C++ 577-588: LastStand 状态对消失的影响
            let is_last_stand_onslaught = self.base.board.map_or(false, |board| unsafe {
                let app_mode = (*board).app.map_or(GameMode::Adventure, |app| (*app).game_mode);
                app_mode == GameMode::ChallengeLastStand
                    && (*board).challenge.as_ref().map_or(false, |c| c.challenge_state == crate::lawn::game_enums::ChallengeState::LastStandOnslaught)
            });
            if !is_last_stand_onslaught {
                if !self.is_level_award() && !self.is_present_with_advice() {
                    self.disappear_counter += 1;
                    if self.disappear_counter >= self.get_disappear_time() {
                        self.start_fade();
                    }
                }
            }
        }

        // C++ 591-602: COIN_MOTION_FROM_PLANT 的缩放渐变
        if self.coin_motion == CoinMotion::FromPlant {
            let a_final_scale = self.get_sun_scale();
            if self.scale < a_final_scale {
                self.scale += 0.02;
            } else {
                self.scale = a_final_scale;
            }
        }

        // 帧动画（保留 Rust 原有逻辑）
        self.counter += 1;
        if self.counter >= 10 {
            self.frame = (self.frame + 1) % 12;
            self.counter = 0;
        }
    }

    /// 更新收集动画（对应 C++ Coin::UpdateCollected, Coin.cpp:605）
    pub fn update_collected(&mut self) {
        let app_ptr = self.base.app;
        let board_ptr = self.base.board;

        // C++ 607-657: 目标坐标确定
        let mut a_dest_x;
        let mut a_dest_y;
        if self.is_sun_type() {
            // C++ 608-612
            a_dest_x = 15.0;
            a_dest_y = 0.0;
        } else if self.is_money() {
            // C++ 613-627
            a_dest_x = 39.0;
            a_dest_y = 558.0;
            let dialog_open = false;
            // [TRANSLATION_NOTE]: C++ 检查 DIALOG_STORE 是否打开；Rust 无 get_dialog，暂 false
            let _ = app_ptr;
            if dialog_open {
                a_dest_x = 662.0;
                a_dest_y = 546.0;
            } else if app_ptr.map_or(false, |app| unsafe { (*app).game_mode == GameMode::ChallengeZenGarden })
                || app_ptr.map_or(false, |app| unsafe { (*app).m_crazy_dave_state != crate::lawn::game_enums::CrazyDaveState::Off })
            {
                a_dest_x = 442.0;
            }
        } else if self.is_present_with_advice() {
            // C++ 628-632
            a_dest_x = 35.0;
            a_dest_y = 487.0;
        } else if self.coin_type == CoinType::AwardPresent || self.coin_type == CoinType::PresentPlant {
            // C++ 633-642
            self.disappear_counter += 1;
            if self.disappear_counter >= 200 {
                self.start_fade();
            }
            return;
        } else if !self.is_level_award() {
            // C++ 643-651
            if self.coin_type == CoinType::UsableSeedPacket {
                self.disappear_counter += 1;
            }
            return;
        } else {
            // C++ 652-657
            a_dest_x = 400.0 - self.base.width as f32 / 2.0;
            a_dest_y = 200.0 - self.base.height as f32 / 2.0;
            self.disappear_counter += 1;
        }

        // C++ 659-665: IsLevelAward 动画曲线
        if self.is_level_award() {
            self.scale = crate::todlib::tod_common::tod_animate_curve_float(0, 400, self.disappear_counter, 1.01, 2.0, crate::lawn::game_enums::TodCurves::EaseInOut);
            self.pos_x = crate::todlib::tod_common::tod_animate_curve_float(0, 350, self.disappear_counter, self.collect_x, a_dest_x, crate::lawn::game_enums::TodCurves::EaseOut);
            self.pos_y = crate::todlib::tod_common::tod_animate_curve_float(0, 350, self.disappear_counter, self.collect_y, a_dest_y, crate::lawn::game_enums::TodCurves::EaseOut);
            return;
        }

        // C++ 667-684: 逐帧移动
        let a_delta_x = (self.pos_x - a_dest_x).abs();
        let a_delta_y = (self.pos_y - a_dest_y).abs();
        if self.pos_x > a_dest_x {
            self.pos_x -= a_delta_x / 21.0;
        } else if self.pos_x < a_dest_x {
            self.pos_x += a_delta_x / 21.0;
        }
        if self.pos_y > a_dest_y {
            self.pos_y -= a_delta_y / 21.0;
        } else if self.pos_y < a_dest_y {
            self.pos_y += a_delta_y / 21.0;
        }
        self.collection_distance = (a_delta_y * a_delta_y + a_delta_x * a_delta_x).sqrt();

        // C++ 688-727: 到达后处理
        if self.is_present_with_advice() {
            // C++ 690-710
            if self.collection_distance < 15.0 {
                if let Some(board) = board_ptr {
                    unsafe {
                        let help_displayed = (*board).m_advice != crate::lawn::game_enums::AdviceType::UnlockedMode;
                        if !help_displayed {
                            if self.coin_type == CoinType::PresentMinigames {
                                (*board).display_advice("[UNLOCKED_MINIGAMES]", MessageStyle::HintTallUnlockMessage as i32, AdviceType::UnlockedMode);
                            } else if self.coin_type == CoinType::PresentPuzzleMode {
                                (*board).display_advice("[UNLOCKED_PUZZLE_MODE]", MessageStyle::HintTallUnlockMessage as i32, AdviceType::UnlockedMode);
                            } else {
                                (*board).display_advice("[UNLOCKED_SURVIVAL_MODE]", MessageStyle::HintTallUnlockMessage as i32, AdviceType::UnlockedMode);
                            }
                        } else {
                            // C++ 706: mHelpIndex != ADVICE_UNLOCKED_MODE || !mAdvice->IsBeingDisplayed()
                            // [TRANSLATION_NOTE]: mAdvice->IsBeingDisplayed() 未实现
                            self.die();
                        }
                    }
                }
            }
        } else {
            // C++ 714-727
            let a_scoring_distance = if self.is_money() { 12.0 } else { 8.0 };
            if self.collection_distance < a_scoring_distance {
                self.score_coin();
            }
            self.scale = (self.collection_distance * 0.05).clamp(0.5, 1.0);
            self.scale *= self.get_sun_scale();
        }
    }

    /// 绘制（对应 C++ Coin::Draw，Coin.cpp:799）
    pub fn draw(&self, g: &mut Graphics) {
        let a_color = self.get_color();
        g.set_color(&Color::new(a_color.0, a_color.1, a_color.2, a_color.3));
        let coin_type = self.coin_type;

        // 钻石发光
        if coin_type == CoinType::Diamond {
            g.set_colorize_images(true);
            if let Some(app) = self.base.get_app() {
                let a_glow = crate::lawn::board::get_overlay_image(app, "IMAGE_AWARDPICKUPGLOW");
                if !a_glow.is_null() {
                    unsafe { g.draw_image_f_xy(&*a_glow, self.pos_x - 56.0, self.pos_y - 66.0); }
                }
            }
            g.set_colorize_images(false);
        }
        // 植物礼物发光
        if coin_type == CoinType::PresentPlant {
            g.set_colorize_images(true);
            if let Some(app) = self.base.get_app() {
                let a_glow = crate::lawn::board::get_overlay_image(app, "IMAGE_AWARDPICKUPGLOW");
                if !a_glow.is_null() {
                    unsafe { g.draw_image_f_xy(&*a_glow, self.pos_x - 50.0, self.pos_y - 64.0); }
                }
            }
            g.set_colorize_images(false);
        }
        // 关卡奖励礼物（收集时）发光
        if coin_type == CoinType::AwardPresent && self.is_being_collected {
            g.set_colorize_images(true);
            if let Some(app) = self.base.get_app() {
                let a_glow = crate::lawn::board::get_overlay_image(app, "IMAGE_AWARDPICKUPGLOW");
                if !a_glow.is_null() {
                    unsafe { g.draw_image_f_xy(&*a_glow, self.pos_x - 50.0, self.pos_y - 64.0); }
                }
            }
            g.set_colorize_images(false);
        }
        // 巧克力发光
        if coin_type == CoinType::Chocolate || coin_type == CoinType::AwardChocolate {
            g.set_colorize_images(true);
            if let Some(app) = self.base.get_app() {
                let a_glow = crate::lawn::board::get_overlay_image(app, "IMAGE_AWARDPICKUPGLOW");
                if !a_glow.is_null() {
                    unsafe { g.draw_image_f_xy(&*a_glow, self.pos_x - 56.0, self.pos_y - 50.0); }
                }
            }
            g.set_colorize_images(false);
        }

        // C++: AttachmentDraw(mAttachmentID, &aAttachmentGraphics, false)
        // [TRANSLATION_NOTE]: Rust 无按 ID 的附件绘制入口（AttachmentDraw），暂略

        // C++: Silver/Gold 落地且未收集时不绘制本体
        if (coin_type == CoinType::Silver || coin_type == CoinType::Gold)
            && self.hit_ground && !self.is_being_collected
        {
            return;
        }

        if coin_type == CoinType::Diamond {
            return;
        }

        if self.is_level_award() && !self.is_being_collected {
            let a_flashing_color = crate::todlib::tod_common::get_flashing_color(self.coin_age as u32, 75);
            g.set_color(&a_flashing_color);
        }

        // Silver/Gold 底光
        if coin_type == CoinType::Silver || coin_type == CoinType::Gold {
            g.set_colorize_images(true);
            if let Some(app) = self.base.get_app() {
                let a_glow = crate::lawn::board::get_overlay_image(app, "IMAGE_REANIM_COINGLOW");
                if !a_glow.is_null() {
                    let a_glow_ref = unsafe { &*a_glow };
                    // C++: PvzpDrawImageCenterScaledF(g, IMAGE_REANIM_COINGLOW, mPosX-14, mPosY-12, mScale, mScale)
                    let a_cel_rect = a_glow_ref.get_cel_rect(0, 0);
                    let a_dst_rect = Rect::new(
                        (self.pos_x - 14.0 - (a_cel_rect.width as f32) * self.scale / 2.0) as i32,
                        (self.pos_y - 12.0 - (a_cel_rect.height as f32) * self.scale / 2.0) as i32,
                        (a_cel_rect.width as f32 * self.scale) as i32,
                        (a_cel_rect.height as f32 * self.scale) as i32,
                    );
                    g.draw_image_stretch(a_glow_ref, &a_dst_rect, &a_cel_rect);
                }
            }
            g.set_colorize_images(false);
        }

        let mut a_image: Option<*mut Image> = None;
        let mut a_image_cel_col = 0;
        let mut a_draw_scale = self.scale;
        let mut a_offset_x = 0.0;
        let mut a_offset_y = 0.0;
        if coin_type == CoinType::Silver {
            a_image = self.overlay_image("IMAGE_REANIM_COIN_SILVER_DOLLAR");
            a_offset_x = 8.0;
            a_offset_y = 10.0;
        } else if coin_type == CoinType::Gold {
            a_image = self.overlay_image("IMAGE_REANIM_COIN_GOLD_DOLLAR");
            a_offset_x = 8.0;
            a_offset_y = 10.0;
        } else if self.is_sun_type() {
            return;
        } else if coin_type == CoinType::FinalSeedPacket {
            let a_seed_type = self.get_final_seed_packet_type();
            g.set_scale(self.scale, self.scale, 0.0, 0.0);
            crate::lawn::seed_packet::draw_seed_packet(
                g,
                0.5 * (self.base.width as f32 - self.scale * self.base.width as f32) + self.pos_x,
                0.5 * (self.base.height as f32 - self.scale * self.base.height as f32) + self.pos_y,
                a_seed_type, SeedType::None, 0.0, 255, true, false,
            );
            g.set_scale(1.0, 1.0, 0.0, 0.0);
            return;
        } else if coin_type == CoinType::PresentPlant || coin_type == CoinType::AwardPresent {
            if self.is_being_collected {
                if let Some(app) = self.base.get_app() {
                    if let Some(zg) = app.zen_garden {
                        unsafe {
                            (*zg).draw_potted_plant_icon(g, self.pos_x + 10.0, self.pos_y - 20.0, &self.potted_plant_spec);
                        }
                    }
                }
                return;
            }
            a_image = self.overlay_image("IMAGE_PRESENT");
            a_offset_y = -20.0;
        } else if self.is_present_with_advice() {
            a_offset_y = -20.0;
            if self.is_being_collected {
                a_offset_x = -10.0;
                a_offset_y -= -10.0;
                a_image = self.overlay_image("IMAGE_PRESENTOPEN");
            } else {
                a_image = self.overlay_image("IMAGE_PRESENT");
            }
        } else if coin_type == CoinType::AwardMoneyBag || coin_type == CoinType::AwardBagDiamond {
            a_image = self.overlay_image("IMAGE_MONEYBAG_HI_RES");
            a_offset_x -= self.base.width as f32 / 2.0;
            a_offset_y -= self.base.height as f32 / 2.0;
            a_draw_scale *= 0.5;
        } else if coin_type == CoinType::Chocolate || coin_type == CoinType::AwardChocolate {
            a_image = self.overlay_image("IMAGE_CHOCOLATE");
        } else if coin_type == CoinType::Trophy {
            a_image = self.overlay_image("IMAGE_TROPHY_HI_RES");
            a_offset_x -= self.base.width as f32 / 2.0;
            a_offset_y -= self.base.height as f32 / 2.0;
            a_draw_scale *= 0.5;
        } else if coin_type == CoinType::AwardSilverSunflower {
            a_image = self.overlay_image("IMAGE_SUNFLOWER_TROPHY");
            a_offset_x -= 5.0;
            a_draw_scale *= 0.6;
        } else if coin_type == CoinType::AwardGoldSunflower {
            a_image = self.overlay_image("IMAGE_SUNFLOWER_TROPHY");
            a_image_cel_col = 1;
            a_offset_x -= 5.0;
            a_draw_scale *= 0.6;
        } else if coin_type == CoinType::Shovel {
            a_image = self.overlay_image("IMAGE_SHOVEL_HI_RES");
            a_offset_x -= 20.0;
            a_offset_y -= 20.0;
            a_draw_scale *= 0.5;
        } else if coin_type == CoinType::Carkeys {
            a_image = self.overlay_image("IMAGE_CARKEYS");
        } else if coin_type == CoinType::Almanac {
            a_image = self.overlay_image("IMAGE_ALMANAC");
        } else if coin_type == CoinType::Taco {
            a_image = self.overlay_image("IMAGE_TACO");
        } else if coin_type == CoinType::Vase {
            a_image = self.overlay_image("IMAGE_SCARY_POT");
        } else if coin_type == CoinType::WateringCan {
            a_image = self.overlay_image("IMAGE_WATERINGCAN");
        } else if coin_type == CoinType::Note {
            a_image = self.overlay_image("IMAGE_ZOMBIE_NOTE_SMALL");
        } else if coin_type == CoinType::UsableSeedPacket {
            let mut a_grayness = 255;
            if self.is_being_collected {
                a_grayness = 128;
            } else {
                let a_disappear_time = self.get_disappear_time();
                if self.disappear_counter > a_disappear_time - 300 && self.disappear_counter % 60 < 30 {
                    a_grayness = 192;
                }
            }
            g.set_colorize_images(true);
            crate::lawn::seed_packet::draw_seed_packet(
                g, self.pos_x, self.pos_y, self.usable_seed_type, SeedType::None,
                0.0, a_grayness, false, false,
            );
            g.set_colorize_images(false);
            return;
        } else {
            // C++: PVZP_ASSERT(false)
        }

        g.set_colorize_images(true);
        if let Some(a_image) = a_image {
            if !a_image.is_null() {
                let a_image_ref = unsafe { &*a_image };
                // C++: PvzpDrawImageCelCenterScaledF(g, aImage, mPosX + aOffsetX, mPosY + aOffsetY, aImageCelCol, aDrawScale, aDrawScale)
                let a_cel_rect = a_image_ref.get_cel_rect(a_image_cel_col, 0);
                let a_dst_rect = Rect::new(
                    (self.pos_x + a_offset_x - (a_cel_rect.width as f32) * a_draw_scale / 2.0) as i32,
                    (self.pos_y + a_offset_y - (a_cel_rect.height as f32) * a_draw_scale / 2.0) as i32,
                    (a_cel_rect.width as f32 * a_draw_scale) as i32,
                    (a_cel_rect.height as f32 * a_draw_scale) as i32,
                );
                g.draw_image_stretch(a_image_ref, &a_dst_rect, &a_cel_rect);
            }
        }
        g.set_colorize_images(false);
    }

    /// 取资源图片（辅助：不存在返回 null 指针，避免各分支重复代码）
    fn overlay_image(&self, name: &str) -> Option<*mut Image> {
        self.base.get_app().map(|app| crate::lawn::board::get_overlay_image(app, name))
    }

    /// 收集硬币（对应 C++ Coin::Collect 简化版）
    pub fn collect(&mut self) {
        // 对应 C++ Coin::Collect (Coin.cpp:1044)
        if self.dead { return; }

        self.collect_x = self.pos_x;
        self.collect_y = self.pos_y;
        self.is_being_collected = true;

        // 获取 app/board 引用用于大量分支
        let app_ptr = self.base.app;
        let board_ptr = self.base.board;

        let a_is_endless_award = {
            let mut flag = false;
            if let Some(app) = app_ptr {
                unsafe {
                    let mode = (*app).game_mode;
                    if ((*app).is_endless_izombie(mode) || (*app).is_endless_scary_potter(mode)) && self.is_level_award() {
                        flag = true;
                    }
                }
            }
            flag
        };

        // C++ 1059-1084: COIN_PRESENT_PLANT / COIN_AWARD_PRESENT
        if self.coin_type == CoinType::PresentPlant || self.coin_type == CoinType::AwardPresent {
            if let Some(app) = app_ptr {
                unsafe {
                    let zen_full = (*app).zen_garden.map_or(false, |zg| (*zg).is_zen_garden_full(false));
                    if zen_full {
                        if let Some(board) = board_ptr {
                            (*board).display_advice("[DIALOG_ZEN_GARDEN_FULL]", MessageStyle::HintFast as i32, AdviceType::None);
                        }
                    } else {
                        if let Some(board) = board_ptr {
                            (*board).m_potted_plants_collected += 1;
                            (*board).display_advice("[ADVICE_FOUND_PLANT]", MessageStyle::HintFast as i32, AdviceType::None);
                        }
                        // C++: aParticle = mApp->AddPvzpParticle(mPosX + 30, mPosY + 30, mRenderOrder + 1, PARTICLE_PRESENT_PICKUP);
                        // [TRANSLATION_NOTE]: C++ AddPvzpParticle 与 AddTodParticle 在 Rust 端同为 add_tod_particle 入口
                        (*app).add_tod_particle(
                            self.pos_x + 30.0,
                            self.pos_y + 30.0,
                            self.base.render_order + 1,
                            ParticleEffect::PresentPickup as i32,
                        );
                        if let Some(zg) = (*app).zen_garden {
                            let mut spec = self.potted_plant_spec.clone();
                            (*zg).add_potted_plant(&mut spec);
                        }
                    }
                    self.disappear_counter = 0;
                    self.fade_count = 0;
                    if a_is_endless_award {
                        // 对应 C++ Coin.cpp:1083 AttachmentDetachCrossFadeParticleType(mAttachmentID, PARTICLE_AWARD_PICKUP_ARROW, nullptr)
                        crate::todlib::attachment::attachment_detach_cross_fade_particle_type(
                            &mut self.attachment_id,
                            ParticleEffect::AwardPickupArrow,
                            None,
                        );
                        if let Some(board) = board_ptr {
                            (*board).fade_out_level();
                        }
                    }
                }
            }
            return;
        }

        // C++ 1086-1099: COIN_PRESENT_MINIGAMES
        if self.coin_type == CoinType::PresentMinigames {
            // 对应 C++ Coin.cpp:1089 AddPvzpParticle(mPosX+30, mPosY+30, mRenderOrder+1, PARTICLE_PRESENT_PICKUP)
            if let Some(app) = app_ptr {
                unsafe {
                    (*app).add_tod_particle(
                        self.pos_x + 30.0,
                        self.pos_y + 30.0,
                        self.base.render_order + 1,
                        ParticleEffect::PresentPickup as i32,
                    );
                }
            }
            self.disappear_counter = 0;
            self.fade_count = 0;
            // 对应 C++ Coin.cpp:1094 AttachmentDetachCrossFadeParticleType(mAttachmentID, PARTICLE_AWARD_PICKUP_ARROW, nullptr)
            crate::todlib::attachment::attachment_detach_cross_fade_particle_type(
                &mut self.attachment_id,
                ParticleEffect::AwardPickupArrow,
                None,
            );
            if let Some(app) = app_ptr {
                unsafe {
                    if let Some(pi) = (*app).player_info.as_mut() {
                        pi.m_has_unlocked_minigames = true;
                    }
                }
            }
            return;
        }
        // C++ 1100-1113: COIN_PRESENT_PUZZLE_MODE
        if self.coin_type == CoinType::PresentPuzzleMode {
            // 对应 C++ Coin.cpp:1103 AddPvzpParticle(PARTICLE_PRESENT_PICKUP)
            if let Some(app) = app_ptr {
                unsafe {
                    (*app).add_tod_particle(
                        self.pos_x + 30.0,
                        self.pos_y + 30.0,
                        self.base.render_order + 1,
                        ParticleEffect::PresentPickup as i32,
                    );
                }
            }
            self.disappear_counter = 0;
            self.fade_count = 0;
            // 对应 C++ Coin.cpp:1109 AttachmentDetachCrossFadeParticleType(mAttachmentID, PARTICLE_AWARD_PICKUP_ARROW, nullptr)
            crate::todlib::attachment::attachment_detach_cross_fade_particle_type(
                &mut self.attachment_id,
                ParticleEffect::AwardPickupArrow,
                None,
            );
            if let Some(app) = app_ptr {
                unsafe {
                    if let Some(pi) = (*app).player_info.as_mut() {
                        pi.m_has_unlocked_puzzle_mode = true;
                    }
                }
            }
            return;
        }
        // C++ 1114-1127: COIN_PRESENT_SURVIVAL_MODE
        if self.coin_type == CoinType::PresentSurvivalMode {
            // 对应 C++ Coin.cpp:1117 AddPvzpParticle(PARTICLE_PRESENT_PICKUP)
            if let Some(app) = app_ptr {
                unsafe {
                    (*app).add_tod_particle(
                        self.pos_x + 30.0,
                        self.pos_y + 30.0,
                        self.base.render_order + 1,
                        ParticleEffect::PresentPickup as i32,
                    );
                }
            }
            self.disappear_counter = 0;
            self.fade_count = 0;
            // 对应 C++ Coin.cpp:1123 AttachmentDetachCrossFadeParticleType(mAttachmentID, PARTICLE_AWARD_PICKUP_ARROW, nullptr)
            crate::todlib::attachment::attachment_detach_cross_fade_particle_type(
                &mut self.attachment_id,
                ParticleEffect::AwardPickupArrow,
                None,
            );
            if let Some(app) = app_ptr {
                unsafe {
                    if let Some(pi) = (*app).player_info.as_mut() {
                        pi.m_has_unlocked_survival_mode = true;
                    }
                }
            }
            return;
        }

        // C++ 1129-1156: COIN_CHOCOLATE / COIN_AWARD_CHOCOLATE
        if self.coin_type == CoinType::Chocolate || self.coin_type == CoinType::AwardChocolate {
            if let Some(app) = app_ptr {
                unsafe {
                    if let Some(board) = board_ptr {
                        (*board).m_chocolate_collected += 1;
                    }
                    // [TRANSLATION_NOTE]: AddPvzpParticle 未实现
                    let chocolate_idx = crate::lawn::game_enums::StoreItem::Chocolate as usize;
                    let cur = (*app).player_info.as_ref().map_or(0, |pi| pi.m_purchases.get(chocolate_idx).map_or(0, |v| *v));
                    if cur < PURCHASE_COUNT_OFFSET {
                        if let Some(board) = board_ptr {
                            (*board).display_advice("[ADVICE_FOUND_CHOCOLATE]", MessageStyle::HintTallFast as i32, AdviceType::None);
                        }
                        if let Some(pi) = (*app).player_info.as_mut() {
                            if chocolate_idx < pi.m_purchases.len() {
                                pi.m_purchases[chocolate_idx] = PURCHASE_COUNT_OFFSET + 1;
                            }
                        }
                    } else {
                        if let Some(pi) = (*app).player_info.as_mut() {
                            if chocolate_idx < pi.m_purchases.len() {
                                pi.m_purchases[chocolate_idx] += 1;
                            }
                        }
                    }
                    self.disappear_counter = 0;
                    self.start_fade();
                    if a_is_endless_award {
                        // 对应 C++ Coin.cpp:1150 AttachmentDetachCrossFadeParticleType(mAttachmentID, PARTICLE_AWARD_PICKUP_ARROW, nullptr)
                        crate::todlib::attachment::attachment_detach_cross_fade_particle_type(
                            &mut self.attachment_id,
                            ParticleEffect::AwardPickupArrow,
                            None,
                        );
                        if let Some(board) = board_ptr {
                            (*board).fade_out_level();
                        }
                    }
                }
            }
            return;
        }

        // C++ 1158-1244: IsLevelAward() 分支
        if self.is_level_award() {
            if let Some(app) = app_ptr {
                unsafe {
                    let mode = (*app).game_mode;
                    if a_is_endless_award {
                        if self.coin_type == CoinType::AwardBagDiamond {
                            (*app).play_sample(SOUND_DIAMOND);
                            self.fan_out_coins(CoinType::Diamond, 1);
                            self.start_fade();
                        } else if self.coin_type == CoinType::AwardMoneyBag {
                            (*app).play_foley(crate::todlib::tod_foley::FoleyType::Coin as i32);
                            self.fan_out_coins(CoinType::Gold, 5);
                            self.start_fade();
                        }
                    } else if (*app).is_scary_potter_level() {
                        if self.coin_type == CoinType::Trophy {
                            (*app).play_foley(crate::todlib::tod_foley::FoleyType::Coin as i32);
                            self.fan_out_coins(CoinType::Gold, 5);
                        } else if self.coin_type == CoinType::AwardMoneyBag {
                            (*app).play_foley(crate::todlib::tod_foley::FoleyType::Coin as i32);
                            self.fan_out_coins(CoinType::Gold, 2);
                        }
                    } else if (*app).is_adventure_mode() && board_ptr.map_or(0, |b| unsafe { (*b).level }) == 50 {
                        self.fan_out_coins(CoinType::Diamond, 3);
                    } else if self.coin_type == CoinType::AwardGoldSunflower {
                        self.fan_out_coins(CoinType::Diamond, 5);
                    } else if (*app).is_first_time_adventure_mode() && board_ptr.map_or(0, |b| unsafe { (*b).level }) == 4 {
                        (*app).play_sample(SOUND_SHOVEL);
                    } else if (*app).is_first_time_adventure_mode() {
                        let lvl = board_ptr.map_or(0, |b| unsafe { (*b).level });
                        if lvl == 24 || lvl == 34 || lvl == 44 {
                            (*app).play_sample(SOUND_TAP2);
                        }
                    } else if self.coin_type == CoinType::Trophy {
                        (*app).play_sample(SOUND_DIAMOND);
                        self.fan_out_coins(CoinType::Diamond, 1);
                    } else if self.coin_type == CoinType::AwardMoneyBag {
                        (*app).play_foley(crate::todlib::tod_foley::FoleyType::Coin as i32);
                        self.fan_out_coins(CoinType::Gold, 5);
                    } else {
                        (*app).play_sample(SOUND_SEEDLIFT);
                        (*app).play_sample(SOUND_TAP2);
                    }

                    // 对应 C++ Coin.cpp:1218 AddPvzpParticle(mPosX+30, mPosY+30, mRenderOrder+1, PARTICLE_STARBURST)
                    if let Some(app) = app_ptr {
                        unsafe {
                            (*app).add_tod_particle(
                                self.pos_x + 30.0,
                                self.pos_y + 30.0,
                                self.base.render_order + 1,
                                ParticleEffect::Starburst as i32,
                            );
                        }
                    }
                    if let Some(board) = board_ptr {
                        (*board).fade_out_level();
                    }
                    // 对应 C++ Coin.cpp:1225-1227 AttachmentDetachCrossFadeParticleType x3
                    crate::todlib::attachment::attachment_detach_cross_fade_particle_type(
                        &mut self.attachment_id,
                        ParticleEffect::SeedPacket,
                        None,
                    );
                    crate::todlib::attachment::attachment_detach_cross_fade_particle_type(
                        &mut self.attachment_id,
                        ParticleEffect::AwardPickupArrow,
                        None,
                    );
                    crate::todlib::attachment::attachment_detach_cross_fade_particle_type(
                        &mut self.attachment_id,
                        ParticleEffect::CoinPickupArrow,
                        None,
                    );

                    if self.coin_type == CoinType::Note {
                        // 对应 C++ Coin.cpp:1230 AddPvzpParticle(PARTICLE_PRESENT_PICKUP)
                        if let Some(app) = app_ptr {
                            unsafe {
                                (*app).add_tod_particle(
                                    self.pos_x + 30.0,
                                    self.pos_y + 30.0,
                                    self.base.render_order + 1,
                                    ParticleEffect::PresentPickup as i32,
                                );
                            }
                        }
                        self.start_fade();
                    } else if !a_is_endless_award {
                        // 对应 C++ Coin.cpp:1234-1240: 3D 加速时附加 PARTICLE_SEED_PACKET_PICKUP
                        // [TRANSLATION_NOTE]: Is3DAccelerated 恒真（与 Rust 全库 3D 加速默认一致），
                        // 附件粒子经 AttachParticle 挂接
                        if let Some(app) = app_ptr {
                            unsafe {
                                let a_particle_offset_x = self.base.width / 2;
                                let a_particle_offset_y = self.base.height / 2;
                                let a_particle = (*app).add_tod_particle(
                                    self.pos_x + a_particle_offset_x as f32,
                                    self.pos_y + a_particle_offset_y as f32,
                                    self.base.render_order - 1,
                                    ParticleEffect::SeedPacketPickup as i32,
                                );
                                if let Some(a_particle) = a_particle {
                                    crate::todlib::attachment::attach_particle(
                                        &mut self.attachment_id,
                                        a_particle as *mut std::ffi::c_void,
                                        a_particle_offset_x as f32,
                                        a_particle_offset_y as f32,
                                    );
                                }
                            }
                        }
                    }

                    self.disappear_counter = 0;
                }
            }
            return;
        }

        // C++ 1246-1257: COIN_USABLE_SEED_PACKET
        if self.coin_type == CoinType::UsableSeedPacket {
            if let Some(board) = board_ptr {
                unsafe {
                    (*board).cursor_object.seed_type = self.usable_seed_type;
                    (*board).cursor_object.cursor_type = CursorType::PlantFromUsableCoin;
                    // [TRANSLATION_NOTE]: mCoinID = DataArrayGetID(this) 未实现
                    self.ground_y = self.pos_y as i32 as f32;
                    self.fade_count = 0;
                }
            }
            return;
        }

        // C++ 1259-1262: IsMoney → ShowCoinBank
        if self.is_money() {
            if let Some(board) = board_ptr {
                unsafe { (*board).show_coin_bank(0); }
            }
        }

        self.fade_count = 0;

        // C++ 1266-1283: IsSun → SeedBank 闪烁 + 雾层调整
        if self.is_sun_type() {
            if let Some(board) = board_ptr {
                unsafe {
                    let bref = &mut *board;
                    if !bref.has_conveyor_belt_seed_bank() {
                        let count = bref.count_sun_being_collected();
                        for i in 0..bref.seed_bank.len() {
                            let pkt_seed = bref.seed_bank[i].seed_type;
                            let pkt_imit = bref.seed_bank[i].imitater_type;
                            let cost = bref.get_current_plant_cost(pkt_seed, pkt_imit);
                            let sun_profit = bref.m_sun_count + count - cost;
                            if sun_profit >= 0 && sun_profit < self.get_sun_value() {
                                bref.seed_bank[i].flash_if_ready();
                            }
                        }
                        if bref.stage_has_fog() {
                            self.base.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_ABOVE_UI, 0, 2);
                        }
                    }
                }
            }
        }

        // 对应 C++ Coin.cpp:1285 AttachmentDetachCrossFadeParticleType(mAttachmentID, PARTICLE_COIN_PICKUP_ARROW, nullptr)
        crate::todlib::attachment::attachment_detach_cross_fade_particle_type(
            &mut self.attachment_id,
            ParticleEffect::CoinPickupArrow,
            None,
        );

        // C++ 1286-1289: 首次冒险模式 1-11 关点击金币提示
        if let Some(app) = app_ptr {
            unsafe {
                if (*app).is_first_time_adventure_mode() {
                    if let Some(board) = board_ptr {
                        if (*board).level == 11 && (self.coin_type == CoinType::Gold || self.coin_type == CoinType::Silver) {
                            (*board).display_advice("[ADVICE_CLICKED_ON_COIN]", MessageStyle::HintFast as i32, AdviceType::ClickedOnCoin);
                        }
                    }
                }
            }
        }
    }

    /// 鼠标按下（对应 C++ Coin::MouseDown，Coin.cpp:1383）
    pub fn mouse_down(&mut self, _x: i32, _y: i32, the_click_count: i32) {
        if self.dead {
            return;
        }

        // 前置检查：mBoard == nullptr || mBoard->mPaused || mGameScene != SCENE_PLAYING || mDead
        let (a_level, a_first_time_adventure) = {
            let board = match self.base.get_board() {
                Some(b) => b,
                None => return,
            };
            if board.m_paused {
                return;
            }
            let app = match self.base.get_app() {
                Some(a) => a,
                None => return,
            };
            if app.game_scene != crate::lawn::lawn_app::GameScenes::Playing {
                return;
            }
            (board.level, app.is_first_time_adventure_mode())
        };

        if the_click_count >= 0 && !self.is_being_collected {
            self.play_collect_sound();
            self.collect();

            // 对应 C++：首次冒险模式第 1 关点击阳光提示
            if a_first_time_adventure && a_level == 1 {
                if let Some(board) = self.base.get_board_mut() {
                    board.display_advice(
                        "[ADVICE_CLICKED_ON_SUN]",
                        MessageStyle::TutorialLevel1Stay as i32,
                        AdviceType::ClickedOnSun,
                    );
                }
            }
        }
    }

    /// 扇形散开硬币（对应 C++ FanOutCoins）
    pub fn fan_out_coins(&mut self, coin_type: CoinType, num_coins: i32) {
        use std::f32::consts::PI;
        for i in 0..num_coins {
            let a_angle = PI / 2.0 + PI * (i + 1) as f32 / (num_coins + 1) as f32;
            let a_pos_x = self.pos_x + 20.0;
            let a_pos_y = self.pos_y;
            if let Some(board) = self.base.get_board_mut() {
                board.add_coin(a_pos_x, a_pos_y, coin_type, CoinMotion::FromSky);
                // [TRANSLATION_NOTE]: 设置散出硬币的初速度需在 add_coin 后修改
                // 暂不实现
            }
        }
    }

    /// 命中检测（对应 C++ Coin::MouseHitTest L1428）
    /// 根据硬币类型调整点击区域大小
    pub fn mouse_hit_test(&self, x: i32, y: i32) -> bool {
        if self.dead {
            return false;
        }

        let extra = if matches!(self.coin_type, CoinType::Sun | CoinType::SmallSun | CoinType::LargeSun) {
            15
        } else {
            0
        };

        let r = crate::framework::rect::Rect::new(
            self.pos_x as i32 - 15 - extra,
            self.pos_y as i32 - 15 - extra,
            30 + extra * 2,
            30 + extra * 2,
        );
        r.contains(x, y)
    }

    /// 是否是金钱（对应 C++ IsMoney）
    pub fn is_money(&self) -> bool {
        matches!(self.coin_type, CoinType::Silver | CoinType::Gold | CoinType::Diamond)
    }

    /// 是否是阳光（对应 C++ IsSun）
    pub fn is_sun_type(&self) -> bool {
        self.is_sun
    }

    /// 是否带建议的礼物（对应 C++ IsPresentWithAdvice）
    pub fn is_present_with_advice(&self) -> bool {
        matches!(self.coin_type,
            CoinType::PresentMinigames | CoinType::PresentPuzzleMode | CoinType::PresentSurvivalMode)
    }

    /// 是否关卡奖励（对应 C++ IsLevelAward）
    pub fn is_level_award(&self) -> bool {
        matches!(self.coin_type,
            CoinType::FinalSeedPacket | CoinType::Trophy |
            CoinType::AwardSilverSunflower | CoinType::AwardGoldSunflower |
            CoinType::Shovel | CoinType::Carkeys | CoinType::Almanac |
            CoinType::Vase | CoinType::WateringCan | CoinType::Taco |
            CoinType::Note | CoinType::AwardMoneyBag | CoinType::AwardBagDiamond |
            CoinType::AwardPresent | CoinType::AwardChocolate)
    }

    /// 是否需要弹跳箭头（对应 C++ CoinGetsBouncyArrow）
    pub fn coin_gets_bouncy_arrow(&self) -> bool {
        if self.is_level_award() {
            return true;
        }
        if self.coin_type == CoinType::Silver || self.coin_type == CoinType::Gold {
            if let Some(app) = self.base.get_app() {
                if let Some(board) = self.base.get_board() {
                    if app.is_first_time_adventure_mode() && board.level == 11 && !board.m_dropped_first_coin {
                        return true;
                    }
                }
            }
        }
        self.is_present_with_advice()
    }

    /// 获取消失时间（对应 C++ GetDisappearTime）
    pub fn get_disappear_time(&self) -> i32 {
        let mut a_time = 750;
        if self.coin_type == CoinType::Diamond
            || self.coin_type == CoinType::PresentPlant
            || self.coin_type == CoinType::Chocolate
            || self.has_bouncy_arrow
        {
            a_time = 1500;
        }
        if let Some(app) = self.base.get_app() {
            if (app.is_scary_potter_level() || app.is_slot_machine_level())
                && self.coin_type == CoinType::UsableSeedPacket
            {
                a_time = 1500;
            }
            if app.game_mode == GameMode::ChallengeZenGarden {
                a_time = 6000;
            }
        }
        a_time
    }

    /// 丢弃可用种子包（对应 C++ DroppedUsableSeed）
    pub fn dropped_usable_seed(&mut self) {
        self.is_being_collected = false;
        if self.times_dropped == 0 {
            self.disappear_counter = self.disappear_counter.min(1200);
        }
        self.times_dropped += 1;
    }

    /// 关卡奖励后尝试自动收集（对应 C++ TryAutoCollectAfterLevelAward）
    pub fn try_auto_collect_after_level_award(&mut self) {
        let mut a_can_be_auto_collected = false;
        if self.is_money() && self.coin_motion != CoinMotion::FromSky {
            // [TRANSLATION_NOTE]: C++ 比较的是 COIN_MOTION_FROM_PRESENT，Rust 枚举暂无该变体，以 FromSky 近似
            a_can_be_auto_collected = true;
        }
        if self.is_sun {
            a_can_be_auto_collected = true;
        }
        if self.coin_type == CoinType::PresentPlant
            || self.coin_type == CoinType::Chocolate
            || self.is_present_with_advice()
        {
            a_can_be_auto_collected = true;
        }
        if a_can_be_auto_collected {
            self.play_collect_sound();
            self.collect();
        }
    }

    /// 播放发射音效（对应 C++ PlayLaunchSound）
    pub fn play_launch_sound(&self) {
        if self.coin_type == CoinType::Diamond
            || self.coin_type == CoinType::Chocolate
            || self.coin_type == CoinType::AwardChocolate
            || self.coin_type == CoinType::PresentPlant
            || self.coin_type == CoinType::AwardPresent
            || self.is_present_with_advice()
        {
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Chime as i32);
            }
        }
    }

    /// 播放落地音效（对应 C++ PlayGroundSound）
    pub fn play_ground_sound(&self) {
        if self.coin_type == CoinType::Gold {
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::MoneyFalls as i32);
            }
        }
    }

    /// 播放收集音效（对应 C++ PlayCollectSound）
    pub fn play_collect_sound(&self) {
        if self.coin_type == CoinType::UsableSeedPacket {
            // C++: mApp->PlaySample(Sexy::SOUND_SEEDLIFT);
            if let Some(app) = self.base.get_app() {
                app.play_sample(unsafe { crate::todlib::tod_foley::SOUND_SEEDLIFT });
            }
            return;
        }
        if self.coin_type == CoinType::Silver || self.coin_type == CoinType::Gold {
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Coin as i32);
            }
            return;
        }
        if self.coin_type == CoinType::Diamond {
            // C++: mApp->PlaySample(Sexy::SOUND_DIAMOND);
            if let Some(app) = self.base.get_app() {
                app.play_sample(unsafe { crate::todlib::tod_foley::SOUND_DIAMOND });
            }
            return;
        }
        if self.is_sun {
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Sun as i32);
            }
            return;
        }
        if self.coin_type == CoinType::Chocolate
            || self.coin_type == CoinType::PresentPlant
            || self.is_present_with_advice()
            || self.coin_type == CoinType::AwardPresent
            || self.coin_type == CoinType::AwardChocolate
        {
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Prize as i32);
            }
            return;
        }
        if self.is_sun {
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Sun as i32);
            }
        }
    }

    /// 计分收集（对应 C++ ScoreCoin）
    pub fn score_coin(&mut self) {
        self.dead = true;
        if self.is_sun {
            if let Some(board) = self.base.get_board_mut() {
                board.add_sun_money(self.value);
            }
        } else if self.is_money() {
            // 对应 C++: mApp->mPlayerInfo->AddCoins(GetCoinValue(mType));
            let a_coin_value = Coin::get_coin_value(self.coin_type);
            if let Some(app) = self.base.get_app_mut() {
                if let Some(player) = app.player_info.as_mut() {
                    player.add_coins(a_coin_value);
                }
            }
            // 对应 C++: mBoard->mCoinsCollected += aCoinValue;
            if let Some(board) = self.base.get_board_mut() {
                board.m_coins_collected += a_coin_value;
            }
            // [TRANSLATION_NOTE]: C++ Silver/Gold 的 mLevelCoinsCollected 计数（满 30 个
            // PennyPincher 成就）与钻石的 mDiamondsCollected，Rust Board 字段暂无
        }
    }

    /// 开始淡出（对应 C++ StartFade）
    pub fn start_fade(&mut self) {
        self.fade_count = 15;
    }

    /// 更新淡出（对应 C++ UpdateFade）
    pub fn update_fade(&mut self) {
        self.fade_count -= 1;
        if self.fade_count == 0 {
            self.dead = true;
        }
    }

    /// 获取阳光缩放（对应 C++ GetSunScale）
    pub fn get_sun_scale(&self) -> f32 {
        if self.coin_type == CoinType::LargeSun { 1.5 } else { 1.0 }
    }

    /// 获取阳光值（对应 C++ GetSunValue）
    pub fn get_sun_value(&self) -> i32 {
        self.value
    }

    /// 硬币死亡（对应 C++ Die）
    pub fn die(&mut self) {
        // 对应 C++ Coin::Die（Coin.cpp:1404-1407）：
        // PVZP_ASSERT(!mBoard || mCursorObject->mCoinID != DataArrayGetID(this))
        // 即光标对象正持有的金币不应被 Die；Rust 以索引一致性近似检查
        if let Some(board) = self.base.board {
            unsafe {
                if self.coin_id != crate::lawn::game_enums::COINID_NULL
                    && (*board).cursor_object.coin_id == self.coin_id
                {
                    debug_assert!(false, "Coin::Die on coin currently held by cursor");
                }
            }
        }
        self.dead = true;
        // 对应 C++: AttachmentDie(mAttachmentID)
        crate::todlib::attachment::attachment_die(&mut self.attachment_id);
    }

    /// 获取硬币值（静态，对应 C++ GetCoinValue）
    pub fn get_coin_value(coin_type: CoinType) -> i32 {
        match coin_type {
            // 对应 C++: Silver=1 / Gold=5 / Diamond=100
            CoinType::Silver => 1,
            CoinType::Gold => 5,
            CoinType::Diamond => 100,
            _ => 0,
        }
    }
}

impl Default for Coin {
    fn default() -> Self {
        Coin::new()
    }
}


