// PvZ Portable Rust 翻译 — 定义/动画定义模块
// 对应 C++ src/Sexy.TodLib/Definition.h / Definition.cpp

#![allow(dead_code)]

use crate::todlib::reanimator::{Reanimation, ReanimLoopType};
use crate::framework::color::Color;
use crate::framework::rect::Rect;

/// 动画定义（轨迹）
#[derive(Debug, Clone)]
pub struct ReanimatorDefinition {
    pub m_fps: f32,
    pub m_tracks: Vec<ReanimatorTrackDefinition>,
}

#[derive(Debug, Clone)]
pub struct ReanimatorTrackDefinition {
    pub m_name: String,
    pub m_parent_name: String,
    pub m_transform: ReanimatorTransform,
    pub m_shader: String,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ReanimatorTransform {
    pub m_trans_x: f32,
    pub m_trans_y: f32,
    pub m_skew_x: f32,
    pub m_skew_y: f32,
    pub m_scale_x: f32,
    pub m_scale_y: f32,
    pub m_alpha: f32,
    pub m_frame: i32,
    pub m_visible: bool,
    pub m_font: i32,
    pub m_text: i32,
    pub m_color: Color,
    pub m_extra_int: i32,
    pub m_extra_float: f32,
}

/// 动画轨迹运行时实例
#[derive(Debug, Clone)]
pub struct ReanimatorTrackInstance {
    pub m_last_visible: bool,
    pub m_anim_time: f32,
    pub m_frame_num: i32,
    pub m_last_frame_num: i32,
    pub m_blend_count: f32,
    pub m_shake_x: f32,
    pub m_shake_y: f32,
    pub m_temp_offset_x: f32,
    pub m_temp_offset_y: f32,
    pub m_cached_center_x: f32,
    pub m_cached_center_y: f32,
}

impl ReanimatorTrackInstance {
    pub fn new() -> Self {
        ReanimatorTrackInstance {
            m_last_visible: true,
            m_anim_time: 0.0,
            m_frame_num: 0,
            m_last_frame_num: -1,
            m_blend_count: 1.0,
            m_shake_x: 0.0,
            m_shake_y: 0.0,
            m_temp_offset_x: 0.0,
            m_temp_offset_y: 0.0,
            m_cached_center_x: 0.0,
            m_cached_center_y: 0.0,
        }
    }
}

impl Default for ReanimatorTransform {
    fn default() -> Self {
        ReanimatorTransform {
            m_trans_x: 0.0,
            m_trans_y: 0.0,
            m_skew_x: 0.0,
            m_skew_y: 0.0,
            m_scale_x: 1.0,
            m_scale_y: 1.0,
            m_alpha: 1.0,
            m_frame: 0,
            m_visible: true,
            m_font: -1,
            m_text: -1,
            m_color: Color::WHITE,
            m_extra_int: 0,
            m_extra_float: 0.0,
        }
    }
}
