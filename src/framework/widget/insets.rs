// PvZ Portable Rust 翻译 — Insets（边距）
// 对应 C++ SexyAppFramework/widget/Insets.h / Insets.cpp

#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Insets {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Insets {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Insets { left, top, right, bottom }
    }

    pub fn zero() -> Self {
        Insets { left: 0, top: 0, right: 0, bottom: 0 }
    }
}

impl Default for Insets {
    fn default() -> Self {
        Insets::zero()
    }
}
