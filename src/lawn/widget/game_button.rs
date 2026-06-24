// PvZ Portable Rust 翻译 — GameButton 游戏按钮
// 对应 C++ src/Lawn/Widget/GameButton.h

#![allow(dead_code)]
use crate::framework::widget::button_widget::ButtonWidget;
use crate::framework::widget::button_listener::ButtonListener;
use crate::framework::graphics::graphics::Graphics;

/// 游戏中的自定义按钮（对应 C++ GameButton）
pub struct GameButton {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub label: String,
    pub btn_no_draw: bool,
    pub is_over: bool, pub is_down: bool, pub disabled: bool,
    pub button_image: *mut crate::framework::graphics::image::Image,
}

impl GameButton {
    pub fn new(_id: i32, _listener: Option<Box<dyn ButtonListener>>) -> Self {
        GameButton {
            x: 0, y: 0, width: 0, height: 0, label: String::new(),
            btn_no_draw: false, is_over: false, is_down: false, disabled: false,
            button_image: std::ptr::null_mut(),
        }
    }
    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) { self.x = x; self.y = y; self.width = w; self.height = h; }
    pub fn draw(&self, _g: &mut Graphics) {}
    pub fn mouse_down_btn(&mut self, _x: i32, _y: i32, _b: i32, _c: i32) { self.is_down = true; }
    pub fn mouse_up_btn(&mut self, _x: i32, _y: i32, _b: i32, _c: i32) { self.is_down = false; }
    pub fn mouse_enter(&mut self) { self.is_over = true; }
    pub fn mouse_leave(&mut self) { self.is_over = false; }
}
