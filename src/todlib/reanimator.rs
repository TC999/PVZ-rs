// PvZ Portable Rust 翻译 — Reanimator（动画系统）
// 对应 C++ src/Sexy.TodLib/Reanimator.h / Reanimator.cpp

#![allow(dead_code)]

use crate::lawn::game_enums::{ReanimationID, REANIMATIONID_NULL, TodCurves};
use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::graphics::gl_interface::TriVertex;
use crate::todlib::definition::{ReanimatorDefinition, ReanimatorTrackInstance, ReanimatorTransform};
use crate::todlib::data_array::DataArray;

/// 动画类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReanimationType {
    None = -1,
    ReanimZombie = 0,
    ReanimPolevaulter,
    ReanimZombieNewspaper,
    ReanimZombieFootball,
    ReanimDancer,
    ReanimBackupDancer,
    ReanimSnorkel,
    ReanimZombieZamboni,
    ReanimBobsled,
    ReanimZombieDolphinrider,
    ReanimJackinthebox,
    ReanimBalloon,
    ReanimDigger,
    ReanimPogo,
    ReanimYeti,
    ReanimBungee,
    ReanimLadder,
    ReanimCatapult,
    ReanimGargantuar,
    ReanimImp,
    ReanimBoss,
}

/// 循环模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReanimLoopType {
    PlayOnceAndHold = 0,
    PlayOnceAndRemove,
    Loop,
    LoopFullOffset,
    PlayOnceAndReturnToZero,
    PlayOnceAndReturnToZeroHoldLastFrame,
}

/// 动画实例
#[derive(Debug)]
pub struct Reanimation {
    pub id: ReanimationID,
    pub reanim_type: ReanimationType,
    pub m_definition: Option<*mut ReanimatorDefinition>,
    pub m_track_instances: Vec<ReanimatorTrackInstance>,
    pub m_loop_type: ReanimLoopType,
    pub m_anim_time: f32,
    pub m_frame_base_time: f32,
    pub m_fps: f32,
    pub m_paused: bool,
    pub m_loop_count: i32,
    pub m_scale: f32,
    pub m_color_override: Color,
    pub m_extra_int: i32,
    pub m_extra_float: f32,
    pub m_owner: Option<*mut std::ffi::c_void>,
}

impl Reanimation {
    pub fn new() -> Self {
        Reanimation {
            id: REANIMATIONID_NULL,
            reanim_type: ReanimationType::None,
            m_definition: None,
            m_track_instances: Vec::new(),
            m_loop_type: ReanimLoopType::PlayOnceAndHold,
            m_anim_time: 0.0,
            m_frame_base_time: 0.0,
            m_fps: 12.0,
            m_paused: false,
            m_loop_count: 0,
            m_scale: 1.0,
            m_color_override: Color::WHITE,
            m_extra_int: 0,
            m_extra_float: 0.0,
            m_owner: None,
        }
    }

    /// 更新动画
    pub fn update(&mut self) {
        if self.m_paused { return; }
        self.m_anim_time += 1.0 / self.m_fps;

        // 检查是否到达结束
        if let Some(def) = self.m_definition {
            unsafe {
                let total_time = (*def).m_fps;
                if self.m_anim_time >= total_time {
                    match self.m_loop_type {
                        ReanimLoopType::Loop => {
                            self.m_anim_time = 0.0;
                            self.m_loop_count += 1;
                        },
                        ReanimLoopType::PlayOnceAndRemove => {
                            // 标记移除
                        },
                        ReanimLoopType::PlayOnceAndReturnToZero => {
                            self.m_anim_time = total_time;
                        },
                        _ => {
                            self.m_anim_time = total_time;
                        }
                    }
                }
            }
        }
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics) {
        // 渲染所有轨道实例
        // 由 GLInterface 实际处理
    }

    /// 重置动画
    pub fn reset(&mut self) {
        self.m_anim_time = 0.0;
        self.m_loop_count = 0;
    }

    /// 设置动画类型（从定义名称查找）
    pub fn set_reanim_type_from_def(&mut self) {
        // 根据定义名称设置动画类型
    }

    /// 获取当前帧的完整时间
    pub fn get_frame_time(&self) -> f32 {
        if let Some(def) = self.m_definition {
            unsafe {
                let total = (*def).m_fps;
                // 确保不超出总时间
                if self.m_anim_time > total {
                    match self.m_loop_type {
                        ReanimLoopType::Loop => {
                            return self.m_anim_time % total;
                        },
                        _ => {
                            return self.m_anim_time.min(total);
                        }
                    }
                }
            }
        }
        self.m_anim_time
    }

    pub fn is_completely_done(&self) -> bool {
        if let Some(def) = self.m_definition {
            unsafe {
                return self.m_anim_time >= (*def).m_fps;
            }
        }
        true
    }
}

impl Default for Reanimation {
    fn default() -> Self {
        Reanimation::new()
    }
}

// ══════════════════════════════════════════════════════
// ║  Reanimator 辅助数据结构
// ══════════════════════════════════════════════════════

/// 重动画标志（对应 C++ ReanimFlags）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReanimFlags {
    NoAtlas,
    FastDrawInSwMode,
}

/// 重动画变换数组（对应 C++ ReanimatorTransformArray）
/// 用于定义文件中的原始格式（指针+计数）
#[derive(Debug, Clone, Copy)]
pub struct ReanimatorTransformArray {
    pub transforms: Option<*mut ReanimatorTransform>,
    pub count: i32,
}

/// 重动画轨道（对应 C++ ReanimatorTrack）
pub struct ReanimatorTrack {
    pub name: Option<&'static str>,
    pub transforms: ReanimatorTransformArray,
}

impl Default for ReanimatorTrack {
    fn default() -> Self {
        ReanimatorTrack {
            name: None,
            transforms: ReanimatorTransformArray {
                transforms: None,
                count: 0,
            },
        }
    }
}

/// 重动画轨道数组（对应 C++ ReanimatorTrackArray）
/// 用于定义文件中的原始格式（指针+计数）
#[derive(Debug, Clone, Copy)]
pub struct ReanimatorTrackArray {
    pub tracks: Option<*mut ReanimatorTrack>,
    pub count: i32,
}

/// 重动画帧时间（对应 C++ ReanimatorFrameTime）
#[derive(Debug, Clone, Copy)]
pub struct ReanimatorFrameTime {
    pub fraction: f32,
    pub anim_frame_before_int: i32,
    pub anim_frame_after_int: i32,
}

impl Default for ReanimatorFrameTime {
    fn default() -> Self {
        ReanimatorFrameTime {
            fraction: 0.0,
            anim_frame_before_int: 0,
            anim_frame_after_int: 0,
        }
    }
}

/// 重动画参数 — 动画类型与文件名的映射（对应 C++ ReanimationParams）
#[derive(Debug, Clone)]
pub struct ReanimationParams {
    pub reanim_type: ReanimationType,
    pub reanim_file_name: Option<&'static str>,
    pub reanim_param_flags: i32,
}

/// 重动画持有者（对应 C++ ReanimationHolder）
pub struct ReanimationHolder {
    pub reanimations: DataArray<Reanimation>,
}

impl ReanimationHolder {
    pub fn new() -> Self {
        ReanimationHolder {
            reanimations: DataArray::new(),
        }
    }

    pub fn initialize_holder(&mut self) {
        // TODO: 从 Reanimator.cpp 翻译
    }

    pub fn dispose_holder(&mut self) {
        // TODO: 从 Reanimator.cpp 翻译
    }

    pub fn alloc_reanimation(
        &mut self,
        _x: f32,
        _y: f32,
        _render_order: i32,
        _reanim_type: ReanimationType,
    ) -> Option<*mut Reanimation> {
        None
    }
}

// ── 常量 ──────────────────────────────────────────────

/// 默认占位值
pub const DEFAULT_FIELD_PLACEHOLDER: f32 = -10000.0;
/// 每帧更新秒数
pub const SECONDS_PER_UPDATE: f64 = 0.01;
/// 隐藏渲染组
pub const RENDER_GROUP_HIDDEN: i32 = -1;
/// 普通渲染组
pub const RENDER_GROUP_NORMAL: i32 = 0;
/// 无基础姿势
pub const NO_BASE_POSE: i32 = -2;
