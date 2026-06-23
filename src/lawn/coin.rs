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
        }
    }

    /// 初始化硬币
    pub fn coin_initialize(&mut self, x: f32, y: f32, coin_type: CoinType, motion: CoinMotion) {
        self.pos_x = x;
        self.pos_y = y;
        self.coin_type = coin_type;
        self.coin_motion = motion;
        self.is_sun = matches!(coin_type, CoinType::Sun | CoinType::SmallSun | CoinType::LargeSun);

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

    /// 更新
    pub fn update(&mut self) {
        if self.dead { return; }

        match self.coin_motion {
            CoinMotion::FromSky => {
                // 从天空掉落
                self.vel_y += 0.5;
                self.pos_x += self.vel_x;
                self.pos_y += self.vel_y;
                if self.pos_y >= self.destination_y && self.destination_y > 0.0 {
                    self.pos_y = self.destination_y;
                    self.vel_y = 0.0;
                    self.coin_motion = CoinMotion::Coin;
                }
            },
            CoinMotion::FromPlant => {
                self.vel_y -= 1.5;
                self.pos_y += self.vel_y;
                if self.vel_y < 0.0 && self.vel_y > -3.0 {
                    self.coin_motion = CoinMotion::FromSky;
                    self.destination_y = self.pos_y + 100.0;
                }
            },
            _ => {}
        }

        self.lifetime += 1;
        if self.lifetime >= self.lifespan {
            self.alpha = self.alpha.saturating_sub(3);
            if self.alpha == 0 {
                self.dead = true;
            }
        }

        // 帧动画
        self.counter += 1;
        if self.counter >= 10 {
            self.frame = (self.frame + 1) % 12;
            self.counter = 0;
        }
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics) {}

    /// 收集硬币（增加玩家金钱/阳光）
    pub fn collect(&mut self) {
        self.dead = true;
    }
}

impl Default for Coin {
    fn default() -> Self {
        Coin::new()
    }
}
