// PvZ Portable Rust 翻译 — SeedPacket（种子槽）
// 对应 C++ src/Lawn/SeedPacket.h / SeedPacket.cpp

use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;

/// 种子槽 — 玩家选择植物的 UI 元素
pub struct SeedPacket {
    pub seed_type: SeedType,
    pub imitater_type: SeedType,
    pub packet_index: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub countdown: i32,  // 冷却时间
    pub active: bool,
    pub can_afford: bool,
}

impl SeedPacket {
    pub fn new() -> Self {
        SeedPacket {
            seed_type: SeedType::None,
            imitater_type: SeedType::None,
            packet_index: 0,
            x: 0,
            y: 516,
            width: SEED_PACKET_WIDTH,
            height: SEED_PACKET_HEIGHT,
            countdown: 0,
            active: true,
            can_afford: false,
        }
    }

    /// 初始化种子槽
    pub fn seed_packet_initialize(&mut self, _index: i32) {}

    /// 更新冷却
    pub fn update(&mut self) {
        if self.countdown > 0 {
            self.countdown -= 1;
        }
        self.active = self.countdown <= 0;
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics) {}

    /// 设置冷却时间
    pub fn set_countdown(&mut self, refresh_time: i32) {
        self.countdown = refresh_time;
    }
}

impl Default for SeedPacket {
    fn default() -> Self {
        SeedPacket::new()
    }
}
