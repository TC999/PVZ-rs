// PvZ Portable Rust 翻译 — Attachment（动画附着物）
// 对应 C++ src/Sexy.TodLib/Attachment.h / Attachment.cpp

#![allow(dead_code)]

use crate::lawn::game_enums::AttachmentID;
use crate::framework::graphics::graphics::Graphics;

/// 动画附着物（用于将粒子/效果附着在实体上）
#[derive(Debug, Clone)]
pub struct Attachment {
    pub id: AttachmentID,
    pub active: bool,
    pub x: f32,
    pub y: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub effect_type: i32, // EffectType
    pub particle_id: i32,
    pub reanim_id: i32,
}

impl Attachment {
    pub fn new() -> Self {
        Attachment {
            id: 0,
            active: false,
            x: 0.0,
            y: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            effect_type: 0,
            particle_id: -1,
            reanim_id: -1,
        }
    }

    pub fn draw(&self, _g: &mut Graphics) {
        // 绘制附着效果
    }
}

impl Default for Attachment {
    fn default() -> Self {
        Attachment::new()
    }
}
