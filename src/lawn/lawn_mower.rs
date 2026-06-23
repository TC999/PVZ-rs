// PvZ Portable Rust 翻译 — LawnMower（割草机）
// 对应 C++ src/Lawn/LawnMower.h / LawnMower.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;

/// 割草机
pub struct LawnMower {
    pub base: GameObject,

    pub pos_x: f32,
    pub pos_y: f32,
    pub vel_x: f32,
    pub mowing: bool,
    pub dead: bool,
    pub trigger_distance: f32,
    pub splash_damage: bool,
    pub frame: i32,
    pub anim_counter: i32,
}

impl LawnMower {
    pub fn new() -> Self {
        LawnMower {
            base: GameObject::new(),
            pos_x: 0.0,
            pos_y: 0.0,
            vel_x: 0.0,
            mowing: false,
            dead: false,
            trigger_distance: 40.0,
            splash_damage: false,
            frame: 0,
            anim_counter: 0,
        }
    }

    /// 初始化割草机
    pub fn lawn_mower_initialize(&mut self, row: i32) {
        self.base.row = row;
        self.pos_x = 40.0;
        self.pos_y = crate::lawn::board::row_to_y(row + 1) as f32;
    }

    /// 启动割草机
    pub fn start_mowing(&mut self) {
        self.mowing = true;
        self.vel_x = 5.0;
    }

    /// 更新
    pub fn update(&mut self) {
        if self.dead { return; }

        if self.mowing {
            self.pos_x += self.vel_x;
            self.anim_counter += 1;
            if self.anim_counter >= 5 {
                self.frame = (self.frame + 1) % 4;
                self.anim_counter = 0;
            }

            // 飞出屏幕后移除
            if self.pos_x > 900.0 {
                self.dead = true;
            }
        }
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics) {}

    /// 获取矩形
    pub fn get_mower_rect(&self) -> Rect {
        Rect::new(
            self.pos_x as i32,
            (self.pos_y - 30.0) as i32,
            80,
            60,
        )
    }
}

impl Default for LawnMower {
    fn default() -> Self {
        LawnMower::new()
    }
}
