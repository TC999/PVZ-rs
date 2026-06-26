// PvZ Portable Rust 翻译 — CheatDialog（作弊对话框）
// 对应 C++ src/Lawn/Widget/CheatDialog.h / CheatDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::lawn_app::LawnApp;

/// 作弊对话框 — 输入关卡编号直接跳关
pub struct CheatDialog {
    pub app: Option<*mut LawnApp>,
    pub level_edit_widget: Option<*mut EditWidget>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

/// 编辑框部件（简化版，对应 C++ EditWidget）
pub struct EditWidget {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

impl CheatDialog {
    pub fn new(app: Option<*mut LawnApp>) -> Self {
        CheatDialog {
            app,
            level_edit_widget: None,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            visible: true,
        }
    }

    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.x = x;
        self.y = y;
        self.width = w;
        self.height = h;
    }

    pub fn draw(&self, _g: &mut Graphics) {}

    pub fn update(&mut self) {}

    pub fn key_down(&mut self, _key: i32) {}

    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32) {}

    pub fn added_to_manager(&mut self, _manager: &mut WidgetManager) {}

    pub fn removed_from_manager(&mut self, _manager: &mut WidgetManager) {}

    pub fn edit_widget_text(&mut self, _id: i32, _text: &str) {}

    pub fn allow_char(&self, _id: i32, _ch: char) -> bool {
        true
    }

    pub fn apply_cheat(&mut self) -> bool {
        false
    }

    pub fn get_preferred_height(&self, _width: i32) -> i32 {
        0
    }
}
