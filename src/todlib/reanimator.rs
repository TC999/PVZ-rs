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

// use crate::lawn::game_enums::ReanimationType 替代本地简化枚举（146 变体，与 C++ 一致）
pub use crate::lawn::game_enums::ReanimationType;

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
    // 渲染位置覆盖（用于 EffectSystem 中的独立定位）
    pub m_x: f32,
    pub m_y: f32,
    pub m_override_scale_x: f32,
    pub m_override_scale_y: f32,
// 新增字段
    pub m_anim_rate: f32,
    pub m_is_attachment: bool,
    pub m_frame_base_pose: i32,
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
            m_x: 0.0,
            m_y: 0.0,
            m_override_scale_x: 1.0,
            m_override_scale_y: 1.0,
            m_anim_rate: 12.0,
            m_is_attachment: false,
            m_frame_base_pose: 0,
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
    pub fn draw(&self, g: &mut Graphics) {
        let Some(def_ptr) = self.m_definition else { return };
        unsafe {
            let def = &*def_ptr;
            if def.m_tracks.is_empty() {
                return;
            }
            let time = self.get_frame_time();
            for (i, track_def) in def.m_tracks.iter().enumerate() {
                let ti = match self.m_track_instances.get(i) {
                    Some(t) => t,
                    None => continue,
                };
                if !ti.m_last_visible {
                    continue;
                }
                let frames = &track_def.m_transforms;
                if frames.is_empty() {
                    continue;
                }
                // 当前帧索引：按动画时长取模（对应 C++ Ge tTransformByTime 简化）
                let total_time = if def.m_fps > 0.0 { def.m_fps } else { 1.0 };
                let frame_idx = ((time % total_time * total_time) as usize) % frames.len();
                let t = &frames[frame_idx];
                if !t.m_visible {
                    continue;
                }
                let px = self.m_x + t.m_trans_x;
                let py = self.m_y + t.m_trans_y;

                // 真实图片绘制：从图片名索引取标准 PNG 并绘制
                // 对应 C++ Reanimation::DrawRenderGroup 的 SetImageTransform + DrawImage
                let scale_x = self.m_override_scale_x * t.m_scale_x;
                let scale_y = self.m_override_scale_y * t.m_scale_y;
                if let Some(img_ptr) = crate::todlib::reanim_loader::reanimator_get_image(t.m_image) {
                    let img = unsafe { &*img_ptr };
                    if img.width > 0 && img.height > 0 {
                        // 设置透明度（通过颜色覆盖）
                        let alpha = (t.m_alpha * 255.0).clamp(0.0, 255.0) as u8;
                        g.set_color(&crate::framework::color::Color::new(255, 255, 255, alpha));
                        if (scale_x - 1.0).abs() > 0.001 || (scale_y - 1.0).abs() > 0.001 {
                            let w = (img.width as f32 * scale_x).round() as i32;
                            let h = (img.height as f32 * scale_y).round() as i32;
                            g.set_scale(scale_x, scale_y, 0.0, 0.0);
                            g.draw_image_f_xy(img, px, py);
                            g.set_scale(1.0, 1.0, 0.0, 0.0);
                        } else {
                            g.draw_image_f_xy(img, px, py);
                        }
                        continue;
                    }
                }

                // 图片缺失时的占位回退（按轨道索引着色，便于调试）
                let w = (40.0 * scale_x).max(2.0) as i32;
                let h = (40.0 * scale_y).max(2.0) as i32;
                let alpha = (t.m_alpha * 255.0).clamp(0.0, 255.0) as u8;
                let hue = (i * 47) % 256;
                g.set_color(&crate::framework::color::Color::new(hue as u8, 128, 255 - hue as u8, alpha));
                g.fill_rect_xywh(px as i32, py as i32, w, h);
            }
        }
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

    /// 递归查找子动画（对应 C++ FindSubReanim）
    /// 在当前动画及其所有附着的子动画中查找指定类型
    pub fn find_sub_reanim(&mut self, reanim_type: ReanimationType) -> Option<&mut Self> {
        if self.reanim_type == reanim_type {
            return Some(self);
        }
        // 遍历轨道实例，查找附着动画
        for track in &self.m_track_instances {
            // 查找附着动画（简化实现）
            // 完整版本需要遍历 AttachEffect 并递归
            if track.m_last_visible {
                // 这里可以通过 AttachmentSystem 查找子 Reanimation
                // 但目前简化处理，仅返回自身匹配
            }
        }
        None
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

    /// 设置位置（对应 C++ SetPosition）
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.m_x = x;
        self.m_y = y;
    }

    /// 覆盖缩放（对应 C++ OverrideScale）
    pub fn override_scale(&mut self, sx: f32, sy: f32) {
        self.m_override_scale_x = sx;
        self.m_override_scale_y = sy;
    }

    /// 检查轨道是否存在（对应 C++ TrackExists）
    pub fn track_exists(&self, name: &str) -> bool {
        if let Some(def) = self.m_definition {
            unsafe {
                return (*def).m_tracks.iter().any(|t| t.m_name == name);
            }
        }
        false
    }

    /// 按类型初始化动画（对应 C++ ReanimationInitializeType）
    pub fn reanimation_initialize_type(&mut self, x: f32, y: f32, reanim_type: ReanimationType) {
        let def_ptr = crate::todlib::reanim_loader::reanimator_get_definition(reanim_type);
        self.reanim_type = reanim_type;
        if let Some(def_ptr) = def_ptr {
            unsafe {
                let def_ref = &*def_ptr;
                self.m_definition = Some(def_ptr);
                self.m_fps = def_ref.m_fps;
                self.m_anim_rate = def_ref.m_fps;
                let track_count = def_ref.m_tracks.len();
                self.m_track_instances = (0..track_count)
                    .map(|_| crate::todlib::definition::ReanimatorTrackInstance::new())
                    .collect();
            }
        }
        self.m_x = x;
        self.m_y = y;
        self.m_anim_time = 0.0;
        self.m_loop_count = 0;
    }

    /// 设置动画类型（从定义名称查找）（对应 C++ SetReanimType + ReanimationInitializeType）
    pub fn set_reanim(&mut self, x: f32, y: f32, reanim_type: ReanimationType) {
        self.reanimation_initialize_type(x, y, reanim_type);
    }

    /// 播放指定轨道（对应 C++ Reanimation::PlayReanim）
    pub fn play_reanim(&mut self, track_name: &str, loop_type: ReanimLoopType, blend_time: i32, anim_rate: f32) {
        let _ = blend_time;
        self.m_loop_type = loop_type;
        if anim_rate > 0.0 {
            self.m_anim_rate = anim_rate;
        }
        // 记录当前播放轨道（通过 frame 归零近似）
        self.m_anim_time = 0.0;
        self.m_loop_count = 0;
        // 查找轨道并重置其动画时间（简化：直接全局归零）
        if let Some(def) = self.m_definition {
            unsafe {
                for (i, t) in (*def).m_tracks.iter().enumerate() {
                    if t.m_name == track_name {
                        if let Some(ti) = self.m_track_instances.get_mut(i) {
                            ti.m_anim_time = 0.0;
                        }
                        break;
                    }
                }
            }
        }
    }

    /// 设置帧层（对应 C++ SetFramesForLayer）
    pub fn set_frames_for_layer(&mut self, _layer: &str) {
        // [TRANSLATION_NOTE]: 完整实现需要 layer 帧区间计算，当前为骨架
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
