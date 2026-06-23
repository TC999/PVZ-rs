// PvZ Portable Rust 翻译 — CursorObject（光标对象/拖放）
// 对应 C++ src/Lawn/CursorObject.h / CursorObject.cpp

use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;

/// 光标对象 — 用于拖放植物等操作
pub struct CursorObject {
    pub cursor_type: CursorType,
    pub seed_type: SeedType,
    pub imitater_type: SeedType,
    pub x: i32,
    pub y: i32,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub active: bool,
}

impl CursorObject {
    pub fn new() -> Self {
        CursorObject {
            cursor_type: CursorType::Normal,
            seed_type: SeedType::None,
            imitater_type: SeedType::None,
            x: 0,
            y: 0,
            mouse_x: 0,
            mouse_y: 0,
            active: false,
        }
    }

    /// 初始化拖放
    pub fn init(&mut self, ctype: CursorType, seed_type: SeedType) {
        self.cursor_type = ctype;
        self.seed_type = seed_type;
        self.active = true;
    }

    /// 更新位置
    pub fn update_position(&mut self, mx: i32, my: i32) {
        self.mouse_x = mx;
        self.mouse_y = my;
        self.x = mx - 25;
        self.y = my - 25;
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics) {}

    /// 完成拖放
    pub fn deactivate(&mut self) {
        self.active = false;
        self.cursor_type = CursorType::Normal;
    }
}

impl Default for CursorObject {
    fn default() -> Self {
        CursorObject::new()
    }
}
