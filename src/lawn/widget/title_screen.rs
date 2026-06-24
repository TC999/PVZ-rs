// PvZ Portable Rust 翻译 — TitleScreen
// 对应 C++ src/Lawn/Widget/TitleScreen.h / TitleScreen.cpp

#![allow(dead_code)]
use crate::framework::graphics::graphics::Graphics;

pub struct TitleScreen {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub visible: bool,
}

impl TitleScreen {
    pub fn new() -> Self { TitleScreen { x: 0, y: 0, width: 0, height: 0, visible: true } }
    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) { self.x = x; self.y = y; self.width = w; self.height = h; }
    pub fn draw(&self, _g: &mut Graphics) {}
    pub fn update(&mut self) {}
    pub fn key_down(&mut self, _key: i32) {}
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32) {}
}

