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
    }

    /// 更新（对应 C++ Coin::Update）
    pub fn update(&mut self) {
        if self.dead { return; }

        self.coin_age += 1;

        if self.fade_count != 0 {
            self.update_fade();
        } else if !self.is_being_collected {
            self.update_fall();
        } else {
            self.update_collected();
        }

        // [TRANSLATION_NOTE]: AttachmentUpdateAndMove 暂未实现
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
                }
                self.pos_y = self.ground_y;
                // 消失计时
                self.disappear_counter += 1;
                if self.disappear_counter >= 600 {
                    self.start_fade();
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

    /// 更新收集动画（对应 C++ Coin::UpdateCollected 简化版）
    pub fn update_collected(&mut self) {
        // [TRANSLATION_NOTE]: 收集动画暂未实现
        self.dead = true;
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics) {}

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
