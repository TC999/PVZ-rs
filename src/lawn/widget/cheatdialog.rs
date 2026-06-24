// PvZ Portable Rust 翻译 — CheatDialog
// 对应 C++ src/Lawn/Widget/CheatDialog.h / CheatDialog.cpp

#![allow(dead_code)]
use crate::framework::graphics::graphics::Graphics;

pub struct CheatDialog {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub visible: bool,
}

impl CheatDialog {
    pub fn new() -> Self { CheatDialog { x: 0, y: 0, width: 0, height: 0, visible: true } }
    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) { self.x = x; self.y = y; self.width = w; self.height = h; }
    pub fn draw(&self, _g: &mut Graphics) {}
    pub fn update(&mut self) {}
    pub fn key_down(&mut self, _key: i32) {}
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32) {}
}

