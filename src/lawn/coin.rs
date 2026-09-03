// PvZ Portable Rust 翻译 — Coin（硬币/物品掉落）
// 对应 C++ src/Lawn/Coin.h / Coin.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;

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
    pub attachment_id: AttachmentID,
    pub needs_bouncy_arrow: bool,
    pub has_bouncy_arrow: bool,
    pub times_dropped: i32,
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
            attachment_id: ATTACHMENTID_NULL,
            needs_bouncy_arrow: false,
            has_bouncy_arrow: false,
            times_dropped: 0,
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
                self.value = 10;
            },
            CoinType::Gold => {
                self.value = 50;
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

        // [TRANSLATION_NOTE]: AttachmentUpdateAndMove 暂未实现
        // 钻石/金钱类硬币有位置偏移 + 颜色/缩放覆盖 + 移动中隐藏动画
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

    /// 更新掉落物理（对应 C++ Coin::UpdateFall）
    pub fn update_fall(&mut self) {
        if self.coin_motion == CoinMotion::FromSky {
            self.vel_y += 0.5;
            self.pos_x += self.vel_x;
            self.pos_y += self.vel_y;
            if self.pos_y >= self.destination_y && self.destination_y > 0.0 {
                self.pos_y = self.destination_y;
                self.vel_y = 0.0;
                self.coin_motion = CoinMotion::Coin;
            }
        } else if self.coin_motion == CoinMotion::FromPlant {
            self.vel_y -= 1.5;
            self.pos_y += self.vel_y;
            if self.vel_y < 0.0 && self.vel_y > -3.0 {
                self.coin_motion = CoinMotion::FromSky;
                self.destination_y = self.pos_y + 100.0;
            }
        } else {
            // 通用重力
            if self.pos_y + self.vel_y < self.ground_y || self.ground_y == 0.0 {
                self.pos_y += self.vel_y;
                self.vel_y += 0.15;
                self.pos_x += self.vel_x;
            } else {
                if !self.hit_ground {
                    self.hit_ground = true;
                    self.play_ground_sound();
                }
                self.pos_y = self.ground_y;
                // 消失计时（对应 C++ UpdateFall：IsLevelAward/IsPresentWithAdvice 不消失）
                if !self.is_level_award() && !self.is_present_with_advice() {
                    self.disappear_counter += 1;
                    if self.disappear_counter >= self.get_disappear_time() {
                        self.start_fade();
                    }
                }
            }
        }

        // 帧动画
        self.counter += 1;
        if self.counter >= 10 {
            self.frame = (self.frame + 1) % 12;
            self.counter = 0;
        }
    }

    /// 更新收集动画（对应 C++ Coin::UpdateCollected）
    /// 硬币飞向目标位置（阳光→左上角、金钱→硬币银行、礼物→解锁提示位置）
    pub fn update_collected(&mut self) {
        // UpdateCollected — 硬币飞向目标位置
        if self.is_sun {
            // 阳光飞向左上角
            let dest_x = 15.0;
            let dest_y = 0.0;
            let dx = (self.pos_x - dest_x).abs();
            let dy = (self.pos_y - dest_y).abs();
            if self.pos_x > dest_x { self.pos_x -= dx / 21.0; }
            else if self.pos_x < dest_x { self.pos_x += dx / 21.0; }
            if self.pos_y > dest_y { self.pos_y -= dy / 21.0; }
            else if self.pos_y < dest_y { self.pos_y += dy / 21.0; }
            self.collection_distance = (dy * dy + dx * dx).sqrt();
            if self.collection_distance < 8.0 {
                self.score_coin();
            }
        } else if self.is_money() {
            // 金钱飞向硬币银行
            let dest_x = 39.0;
            let dest_y = 558.0;
            let dx = (self.pos_x - dest_x).abs();
            let dy = (self.pos_y - dest_y).abs();
            if self.pos_x > dest_x { self.pos_x -= dx / 21.0; }
            else if self.pos_x < dest_x { self.pos_x += dx / 21.0; }
            if self.pos_y > dest_y { self.pos_y -= dy / 21.0; }
            else if self.pos_y < dest_y { self.pos_y += dy / 21.0; }
            self.collection_distance = (dy * dy + dx * dx).sqrt();
            self.scale = (self.collection_distance * 0.05).clamp(0.5, 1.0);
            if self.collection_distance < 12.0 {
                self.score_coin();
            }
        } else {
            self.dead = true;
        }
    }

    /// 绘制（对应 C++ Coin::Draw）
    /// 按硬币类型选择不同绘制方式：阳光/金钱/钻石/礼物/种子包
    pub fn draw(&self, _g: &mut Graphics) {
        // [TRANSLATION_NOTE]: 完整绘制依赖 IMAGE_REANIM_SUN/IMAGE_COIN_SILVER 等资源
        // 阳光：使用 Reanimation 绘制，支持缩放和闪烁效果
        // 金钱：使用 IMAGE_COIN_SILVER/GOLD/DIAMOND 精灵图
        // 礼物：使用 IMAGE_PRESENT 精灵图
        // 种子包：使用 IMAGE_PACKET_PLANTS 精灵图
    }

    /// 收集硬币（对应 C++ Coin::Collect 简化版）
    pub fn collect(&mut self) {
        if self.dead { return; }

        self.is_being_collected = true;

        // 阳光/金钱计分
        if self.is_sun {
            if let Some(board) = self.base.get_board_mut() {
                board.add_sun_money(self.value);
            }
        } else if self.is_money() {
            if let Some(board) = self.base.get_board_mut() {
                board.add_sun_money(self.value);
            }
        }

        // [TRANSLATION_NOTE]: 特殊硬币类型（礼物/巧克力/种子/关卡奖励）
        // 的处理逻辑暂未实现

        self.fade_count = 0;
        // [TRANSLATION_NOTE]: AttachmentDetachCrossFade 暂未实现
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
            // [TRANSLATION_NOTE]: PlaySample(SOUND_SEEDLIFT) 未接入
            return;
        }
        if self.coin_type == CoinType::Silver || self.coin_type == CoinType::Gold {
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Coin as i32);
            }
            return;
        }
        if self.coin_type == CoinType::Diamond {
            // [TRANSLATION_NOTE]: PlaySample(SOUND_DIAMOND) 未接入
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
            // [TRANSLATION_NOTE]: PlayerInfo.AddCoins 暂未实现
            if let Some(board) = self.base.get_board_mut() {
                board.add_sun_money(self.value);
            }
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
        self.dead = true;
        // [TRANSLATION_NOTE]: AttachmentDie(mAttachmentID) 暂未实现
    }

    /// 获取硬币值（静态，对应 C++ GetCoinValue）
    pub fn get_coin_value(coin_type: CoinType) -> i32 {
        match coin_type {
            CoinType::Silver => 10,
            CoinType::Gold => 50,
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


