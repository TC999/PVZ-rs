// PvZ Portable Rust 翻译 — LawnMower（割草机）
// 对应 C++ src/Lawn/LawnMower.h / LawnMower.cpp

use crate::lawn::game_enums::*;
use crate::lawn::game_object::GameObject;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;

/// 割草机（对应 C++ class LawnMower）
pub struct LawnMower {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub board: Option<*mut crate::lawn::board::Board>,
    pub base: GameObject,

    pub pos_x: f32,
    pub pos_y: f32,
    pub render_order: i32,
    pub anim_ticks_per_frame: i32,
    pub mower_anim_id: ReanimationID,
    pub chomp_counter: i32,
    pub rolling_in_counter: i32,
    pub squished_counter: i32,
    pub mower_state: LawnMowerState,
    pub dead: bool,
    pub visible: bool,
    pub mowing: bool,
    pub mower_type: LawnMowerType,
    pub altitude: f32,
    pub mower_height: MowerHeight,
    pub last_portal_x: i32,
}

impl LawnMower {
    pub fn new() -> Self {
        LawnMower {
            app: None,
            board: None,
            base: GameObject::new(),
            pos_x: 0.0,
            pos_y: 0.0,
            render_order: 0,
            anim_ticks_per_frame: 0,
            mower_anim_id: REANIMATIONID_NULL,
            chomp_counter: 0,
            rolling_in_counter: 0,
            squished_counter: 0,
            mower_state: LawnMowerState::Ready,
            dead: false,
            visible: false,
            mowing: false,
            mower_type: LawnMowerType::Lawn,
            altitude: 0.0,
            mower_height: MowerHeight::Land,
            last_portal_x: 0,
        }
    }

    /// 初始化割草机（对应 C++ LawnMower::LawnMowerInitialize，LawnMower.cpp:30）
    pub fn lawn_mower_initialize(&mut self, the_row: i32) {
        self.base.row = the_row;
        // C++: mPosX = -160.0f;
        self.pos_x = -160.0;
        // C++: mRenderOrder = MakeRenderOrder(RENDER_LAYER_LAWN_MOWER, theRow, 0);
        self.base.render_order =
            crate::lawn::board::make_render_order(RENDER_LAYER_LAWN_MOWER, the_row, 0);
        // C++: mPosY = mBoard->GetPosYBasedOnRow(mPosX + 40.0f, theRow) + 23.0f;
        self.pos_y = self
            .base
            .get_board()
            .map_or(0.0, |b| {
                b.get_pos_y_based_on_row(self.base.x as f32 + 40.0, the_row)
            })
            + 23.0;
        self.dead = false;
        self.mower_state = LawnMowerState::Ready;
        self.visible = true;
        self.chomp_counter = 0;
        self.rolling_in_counter = 0;
        self.squished_counter = 0;
        self.last_portal_x = -1;

        // C++ 46-61: 依屋顶 / 泳池（且已购买泳池清洁器）/ 普通草坪决定割草机类型与 reanim
        let a_stage_has_roof = self.base.get_board().map_or(false, |b| b.stage_has_roof());
        let a_is_pool_row = self
            .base
            .get_board()
            .map_or(false, |b| b.m_plant_row[the_row as usize] == PlantRowType::Pool);
        let a_has_pool_cleaner = self.base.get_app().map_or(false, |app| unsafe {
            app.player_info.as_ref().map_or(false, |pi| {
                pi.m_purchases
                    .get(crate::lawn::game_enums::StoreItem::PoolCleaner as usize)
                    .copied()
                    .unwrap_or(0)
                    != 0
            })
        });

        let a_reanim_type: i32;
        if a_stage_has_roof {
            self.mower_type = LawnMowerType::Roof;
            a_reanim_type = ReanimationType::RoofCleaner as i32;
        } else if a_is_pool_row && a_has_pool_cleaner {
            self.mower_type = LawnMowerType::Pool;
            a_reanim_type = ReanimationType::PoolCleaner as i32;
        } else {
            self.mower_type = LawnMowerType::Lawn;
            a_reanim_type = ReanimationType::Lawnmower as i32;
        }

        // C++ 63-68: 创建割草机 reanim 并初始化速率/循环/缩放
        let a_render_order = self.base.render_order;
        let a_mower_reanim = self
            .base
            .get_app_mut()
            .and_then(|app| app.add_reanimation(0.0, 18.0, a_render_order, a_reanim_type));
        if let Some(a_mower_reanim) = a_mower_reanim {
            unsafe {
                (*a_mower_reanim).m_anim_rate = 0.0;
                (*a_mower_reanim).m_loop_type = crate::todlib::reanimator::ReanimLoopType::Loop;
                (*a_mower_reanim).m_is_attachment = true;
                (*a_mower_reanim).override_scale(0.85, 0.85);
            }
            self.mower_anim_id = self
                .base
                .get_app()
                .map_or(REANIMATIONID_NULL, |app| app.reanimation_get_id(a_mower_reanim));

            // C++ 70-79: 草坪用 anim_normal；泳池额外缩放并播 anim_land
            if self.mower_type == LawnMowerType::Lawn {
                unsafe {
                    (*a_mower_reanim).set_frames_for_layer("anim_normal");
                }
            } else if self.mower_type == LawnMowerType::Pool {
                unsafe {
                    (*a_mower_reanim).override_scale(0.8, 0.8);
                    (*a_mower_reanim).set_frames_for_layer("anim_land");
                    (*a_mower_reanim).set_truncate_disappearing_frames(None, false);
                }
            }
        }

        // C++ 81-84: 超级割草机模式下把草坪割草机升级
        let a_super_mower_mode = self
            .base
            .get_board()
            .map_or(false, |b| b.m_super_mower_mode);
        if a_super_mower_mode && self.mower_type == LawnMowerType::Lawn {
            self.enable_super_mower(true);
        }
    }

    /// 启动割草机（对应 C++ StartMower：状态置为 Triggered）
    pub fn start_mower(&mut self) {
        if self.mower_state == LawnMowerState::Triggered {
            return;
        }

        // 对应 C++: reanim 速率 + PlayFoley（Rust 音效系统）
        if let Some(app) = self.base.get_app() {
            if self.mower_type == LawnMowerType::Pool {
                app.play_foley(crate::todlib::tod_foley::FoleyType::PoolCleaner as i32);
            } else {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Lawnmower as i32);
            }
        }

        // 对应 C++: mBoard->mWaveRowGotLawnMowered[mRow] = mBoard->mCurrentWave;
        //           mBoard->mTriggeredLawnMowers++;
        let the_row = self.base.row;
        if let Some(board) = self.base.get_board_mut() {
            board.m_wave_row_got_lawn_mowered[the_row as usize] = board.m_current_wave;
            board.m_triggered_lawn_mowers += 1;
        }

        self.mowing = true;
        self.mower_state = LawnMowerState::Triggered;
    }

    /// 启动割草机（与 start_mower 逻辑相同）
    pub fn start_mowing(&mut self) {
        self.start_mower();
    }

    /// 更新割草机（对应 C++ LawnMower::Update）
    pub fn update(&mut self) {
        if self.dead { return; }

        // 被压碎状态
        if self.mower_state == LawnMowerState::Squished {
            self.squished_counter -= 1;
            if self.squished_counter <= 0 {
                self.die();
            }
            return;
        }

        // 滚入状态
        if self.mower_state == LawnMowerState::RollingIn {
            self.rolling_in_counter += 1;
            // PvzpAnimateCurveFloat(0, 100, rollingInCounter, -160, -21, EASE_IN_OUT)
            if self.rolling_in_counter == 100 {
                self.mower_state = LawnMowerState::Ready;
            }
            return;
        }

        // 游戏场景检查
        // [TRANSLATION_NOTE]: 场景检查暂略
        // [TRANSLATION_NOTE]: 碰撞检测→MowZombie 已统一到 Board::check_collisions（mower.update 内因借用无法取 zombie 可变引用）

        // 触发/碾压状态
        if self.mower_state == LawnMowerState::Triggered || self.mower_state == LawnMowerState::Squished {
            let mut a_speed = if self.mower_type == LawnMowerType::Pool { 2.5 } else { 3.33 };
            if self.chomp_counter > 0 {
                self.chomp_counter -= 1;
                // PvzpAnimateCurveFloat(50, 0, chompCounter, aSpeed, 1.0, BOUNCE_SLOW_MIDDLE)
            }
            self.pos_x += a_speed;

            // 泳池割草机落水
            if self.mower_type == LawnMowerType::Pool {
                self.update_pool();
            }

            // 陆地割草机进入泳池行
            if self.mower_type == LawnMowerType::Lawn {
                if let Some(board) = self.base.get_board() {
                    if board.m_plant_row[self.base.row as usize] == PlantRowType::Pool && self.pos_x > 50.0 {
                        // [TRANSLATION_NOTE]: 水花动画+音效暂未实现
                        self.die();
                    }
                }
            }

            // 飞出棋盘
            if self.pos_x > 900.0 {
                self.die();
            }
        }
    }

    /// 碾压僵尸（对应 C++ LawnMower::MowZombie）
    pub fn mow_zombie(&mut self, zombie: &mut crate::lawn::zombie::Zombie) {
        if self.mower_state == LawnMowerState::Ready {
            self.start_mower();
            self.chomp_counter = 25;
        } else if self.mower_state == LawnMowerState::Triggered {
            self.chomp_counter = 50;
        }

        if self.mower_type == LawnMowerType::Pool {
            // [TRANSLATION_NOTE]: FOLEY_SHOOP 音效 + anim_suck/anim_landsuck reanim 暂未接入
            zombie.die_with_loot();
        } else {
            // [TRANSLATION_NOTE]: FOLEY_SPLAT 音效暂未接入
            zombie.mow_down();
        }
    }

    /// 获取割草机攻击碰撞矩形（对应 C++ GetLawnMowerAttackRect）
    pub fn get_lawn_mower_attack_rect(&self) -> Rect {
        Rect::new(self.pos_x as i32, self.pos_y as i32, 50, 80)
    }

    /// 获取割草机碰撞矩形
    pub fn get_mower_rect(&self) -> Rect {
        Rect::new(self.pos_x as i32, self.pos_y as i32, 80, 80)
    }

    /// 割草机死亡（对应 C++ Die）
    pub fn die(&mut self) {
        self.dead = true;
        // [TRANSLATION_NOTE]: 完整实现需要移除 Reanimation + 检查 bonus mowers
    }

    /// 粉碎割草机（对应 C++ SquishMower：状态置为 Squished + 500 帧倒计时后 Die）
    pub fn squish_mower(&mut self) {
        self.mower_state = LawnMowerState::Squished;
        self.squished_counter = 500;
        // [TRANSLATION_NOTE]: reanim 缩放/位移 + FOLEY_SQUISH 音效依赖 reanim/Foley 系统
    }

    /// 启用超级割草机（对应 C++ EnableSuperMower，LawnMower.cpp:422）
    /// C++ 注释：Is theEnable being unused a bug?（theEnable 参数未使用，保留）
    pub fn enable_super_mower(&mut self, _enable: bool) {
        if self.mower_type == LawnMowerType::Lawn {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_mower_reanim) = app.reanimation_get_mut(self.mower_anim_id) {
                    a_mower_reanim.set_frames_for_layer("anim_tricked");
                }
            }
        }
    }

    /// 水池高度更新（对应 C++ UpdatePool，LawnMower.cpp:87）
    /// 泳池割草机进入/离开水池时的高度动画和音效
    pub fn update_pool(&mut self) {
        let is_pool_range = self.pos_x > 26.0 && self.pos_x < 660.0;

        if is_pool_range && self.mower_height == MowerHeight::Land {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_splash_reanim) = app.add_reanimation(
                    self.pos_x, self.pos_y + 25.0, self.render_order + 1,
                    ReanimationType::Splash as i32,
                ) {
                    unsafe {
                        (*a_splash_reanim).override_scale(1.2, 0.8);
                    }
                }
                app.add_tod_particle(
                    self.pos_x + 50.0, self.pos_y + 42.0, self.render_order + 1,
                    ParticleEffect::PlantingPool as i32,
                );
                app.play_foley(crate::todlib::tod_foley::FoleyType::ZombieSplash as i32);
            }
            self.mower_height = MowerHeight::DownToPool;
        } else if self.mower_height == MowerHeight::DownToPool {
            self.altitude -= 2.0;
            if self.altitude <= -28.0 {
                self.altitude = 0.0;
                self.mower_height = MowerHeight::InPool;
                if let Some(app) = self.base.get_app_mut() {
                    if let Some(a_mower_reanim) = app.reanimation_get_mut(self.mower_anim_id) {
                        a_mower_reanim.play_reanim("anim_water", crate::todlib::reanimator::ReanimLoopType::Loop, 0, 0.0);
                    }
                }
            }
        } else if self.mower_height == MowerHeight::InPool {
            if !is_pool_range {
                self.altitude = -28.0;
                self.mower_height = MowerHeight::UpToLand;
                if let Some(app) = self.base.get_app_mut() {
                    if let Some(a_splash_reanim) = app.add_reanimation(
                        self.pos_x, self.pos_y + 25.0, self.render_order + 1,
                        ReanimationType::Splash as i32,
                    ) {
                        unsafe {
                            (*a_splash_reanim).override_scale(1.2, 0.8);
                        }
                    }
                    app.add_tod_particle(
                        self.pos_x + 50.0, self.pos_y + 42.0, self.render_order + 1,
                        ParticleEffect::PlantingPool as i32,
                    );
                    app.play_foley(crate::todlib::tod_foley::FoleyType::PlantWater as i32);
                }
                if let Some(app) = self.base.get_app_mut() {
                    if let Some(a_mower_reanim) = app.reanimation_get_mut(self.mower_anim_id) {
                        a_mower_reanim.play_reanim("anim_land", crate::todlib::reanimator::ReanimLoopType::Loop, 0, 0.0);
                    }
                }
            }
        } else if self.mower_height == MowerHeight::UpToLand {
            self.altitude += 2.0;
            if self.altitude >= 0.0 {
                self.altitude = 0.0;
                self.mower_height = MowerHeight::Land;
            }
        }

        if self.mower_height == MowerHeight::InPool {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_mower_reanim) = app.reanimation_get_mut(self.mower_anim_id) {
                    if a_mower_reanim.m_loop_type == crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold
                        && a_mower_reanim.m_loop_count > 0
                    {
                        a_mower_reanim.play_reanim("anim_water", crate::todlib::reanimator::ReanimLoopType::Loop, 10, 35.0);
                    }
                }
            }
        }
    }

    /// 绘制割草机阴影（对应 C++ LawnMower::Draw 中的阴影部分，LawnMower.cpp:285-319）
    pub fn draw_shadow(&self, g: &mut Graphics) {
        if self.mower_height == MowerHeight::UpToLand
            || self.mower_height == MowerHeight::DownToPool
            || self.mower_height == MowerHeight::InPool
            || self.mower_state == LawnMowerState::Squished
        {
            return;
        }

        let mut a_shadow_type = 0;
        let mut a_scale_x = 1.0;
        let mut a_scale_y = 1.0;
        let mut is_night = false;
        if let Some(board) = self.base.get_board() {
            is_night = board.stage_is_night();
        }
        if is_night {
            a_shadow_type = 1;
        }

        let mut a_shadow_x = self.pos_x - 7.0;
        let mut a_shadow_y = self.pos_y - self.altitude + 47.0;
        if self.mower_type == LawnMowerType::Pool {
            a_shadow_x -= 17.0;
            a_shadow_y -= 8.0;
        }
        if self.mower_type == LawnMowerType::Roof {
            a_shadow_x -= 9.0;
            a_shadow_y -= 36.0;
            a_scale_y = 1.2;
            if self.mower_state == LawnMowerState::Triggered {
                a_shadow_y += 36.0;
            }
        }

        let app = match self.base.get_app() {
            Some(a) => a,
            None => return,
        };
        let a_shadow_image = if a_shadow_type == 0 {
            crate::lawn::board::get_overlay_image(app, "IMAGE_PLANTSHADOW")
        } else {
            crate::lawn::board::get_overlay_image(app, "IMAGE_PLANTSHADOW2")
        };
        if a_shadow_image.is_null() {
            return;
        }
        let a_shadow_ref = unsafe { &*a_shadow_image };
        // PvzpDrawImageCelCenterScaledF(g, image, aShadowX, aShadowY, 0, aScaleX, aScaleY)
        // [TRANSLATION_NOTE]: Rust 无 cel 中心缩放 API，用 draw_image_stretch 按中心锚点近似
        let a_cel_rect = a_shadow_ref.get_cel_rect(0, 0);
        let a_dst_rect = Rect::new(
            (a_shadow_x - (a_cel_rect.width as f32) * a_scale_x / 2.0) as i32,
            (a_shadow_y - (a_cel_rect.height as f32) * a_scale_y / 2.0) as i32,
            (a_cel_rect.width as f32 * a_scale_x) as i32,
            (a_cel_rect.height as f32 * a_scale_y) as i32,
        );
        g.draw_image_stretch(a_shadow_ref, &a_dst_rect, &a_cel_rect);
    }

    /// 绘制割草机（对应 C++ LawnMower::Draw，LawnMower.cpp:275）
    pub fn draw(&self, g: &mut Graphics) {
        if !self.visible {
            return;
        }

        self.draw_shadow(g);

        // C++: Graphics aMowerGraphics(*g) —— Rust Graphics 无 Clone，
        // 直接操作 g 的 trans/clip 并在末尾恢复，等价于副本生命周期
        let a_saved_trans_x = g.trans_x;
        let a_saved_trans_y = g.trans_y;
        let a_saved_clip_rect = g.clip_rect;
        g.trans_x += (self.pos_x + 6.0) as f64;
        g.trans_y += (self.pos_y - self.altitude) as f64;
        if self.mower_type == LawnMowerType::Pool {
            if self.mower_state == LawnMowerState::Triggered {
                g.trans_y -= 7.0;
                g.trans_x -= 10.0;
            } else {
                g.trans_y -= 33.0;
            }

            if self.mower_height == MowerHeight::UpToLand || self.mower_height == MowerHeight::DownToPool {
                g.set_clip_rect_xywh(-50, -50, 150, (132.0 + self.altitude) as i32);
            }
        } else if self.mower_type == LawnMowerType::Roof {
            if self.mower_state == LawnMowerState::Triggered {
                g.trans_y -= 4.0;
                g.trans_x -= 10.0;
            } else {
                g.trans_y -= 40.0;
            }
        }

        let app = match self.base.get_app() {
            Some(a) => a,
            None => return,
        };
        if self.mower_state == LawnMowerState::Triggered || self.mower_state == LawnMowerState::Squished {
            // mApp->ReanimationGet(mReanimID)->Draw(&aMowerGraphics)
            if let Some(a_mower_reanim) = app.reanimation_get(self.mower_anim_id) {
                a_mower_reanim.draw(g);
            }
        } else {
            // mApp->mReanimatorCache->DrawCachedMower(&aMowerGraphics, 0.0f, 19.0f, aMowerType)
            let mut a_mower_type = self.mower_type;
            if self.mower_type == LawnMowerType::Lawn
                && self.base.get_board().map_or(false, |b| b.m_super_mower_mode)
            {
                a_mower_type = LawnMowerType::SuperMower;
            }
            if let Some(cache) = app.m_reanimator_cache {
                unsafe {
                    (*cache).draw_cached_mower(g, 0.0, 19.0, a_mower_type);
                }
            }
        }

        // 恢复 trans/clip（对应 C++ 副本 Graphics 析构丢弃）
        g.trans_x = a_saved_trans_x;
        g.trans_y = a_saved_trans_y;
        g.set_clip_rect(&a_saved_clip_rect);
    }
}

impl Default for LawnMower {
    fn default() -> Self {
        LawnMower::new()
    }
}
