// PvZ Portable Rust 翻译 — ImitaterDialog（模仿者选择对话框）
// 对应 C++ src/Lawn/Widget/ImitaterDialog.h / ImitaterDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::lawn::game_enums::SeedType;
use crate::lawn::tool_tip_widget::ToolTipWidget;

/// 模仿者对话框 — 选择模仿者要模仿的植物
pub struct ImitaterDialog {
    pub tool_tip: Option<*mut ToolTipWidget>,
    pub tool_tip_seed: SeedType,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

impl ImitaterDialog {
    pub fn new() -> Self {
        ImitaterDialog {
            tool_tip: None,
            tool_tip_seed: SeedType::None,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            visible: true,
        }
    }

    pub fn seed_hit_test(&self, _x: i32, _y: i32) -> SeedType {
        SeedType::None
    }

    pub fn update_cursor(&mut self) {}

    pub fn update(&mut self) {}

    pub fn get_seed_position(index: i32, x: &mut i32, y: &mut i32) {
        *x = 0;
        *y = 0;
    }

    pub fn draw(&self, _g: &mut Graphics) {}

    pub fn show_tool_tip(&mut self) {}

    pub fn remove_tool_tip(&mut self) {}

    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {}
}
