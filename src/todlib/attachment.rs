// PvZ Portable Rust 翻译 — Attachment（动画附着物）
// 对应 C++ src/Sexy.TodLib/Attachment.h / Attachment.cpp

#![allow(dead_code)]

use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::color::Color;
use crate::framework::common::SexyVector2;
use crate::framework::sexy_matrix::SexyMatrix3;
use crate::todlib::data_array::DataArray;

pub const MAX_EFFECTS_PER_ATTACHMENT: usize = 16;

/// 附着效果单元（对应 C++ AttachEffect）
#[derive(Debug, Clone)]
pub struct AttachEffect {
    pub effect_id: u32,
    pub effect_type: EffectType,
    pub offset: SexyMatrix3,
    pub dont_draw_if_parent_hidden: bool,
    pub dont_propogate_color: bool,
}

impl Default for AttachEffect {
    fn default() -> Self {
        AttachEffect {
            effect_id: 0,
            effect_type: EffectType::Particle,
            offset: SexyMatrix3::identity(),
            dont_draw_if_parent_hidden: false,
            dont_propogate_color: false,
        }
    }
}

/// 附着器信息（对应 C++ AttacherInfo）
#[derive(Debug, Clone)]
pub struct AttacherInfo {
    pub reanim_name: String,
    pub track_name: String,
    pub anim_rate: f32,
    pub loop_type: ReanimLoopType,
}

impl Default for AttacherInfo {
    fn default() -> Self {
        AttacherInfo {
            reanim_name: String::new(),
            track_name: String::new(),
            anim_rate: 1.0,
            loop_type: ReanimLoopType::Loop,
        }
    }
}

/// 动画附着物（对应 C++ Attachment）
#[derive(Debug, Clone)]
pub struct Attachment {
    pub effect_array: [AttachEffect; MAX_EFFECTS_PER_ATTACHMENT],
    pub num_effects: i32,
    pub dead: bool,
}

impl Attachment {
    pub fn new() -> Self {
        Attachment {
            effect_array: Default::default(),
            num_effects: 0,
            dead: false,
        }
    }

    /// 更新所有附着效果
    pub fn update(&mut self) {
        // TODO: 从 Attachment.cpp 翻译
    }

    /// 设置位置
    pub fn set_position(&mut self, _position: &SexyVector2) {
        // TODO: 从 Attachment.cpp 翻译
    }

    /// 设置矩阵
    pub fn set_matrix(&mut self, _matrix: &SexyMatrix3) {
        // TODO: 从 Attachment.cpp 翻译
    }

    /// 覆盖颜色
    pub fn override_color(&mut self, _color: &Color) {
        // TODO: 从 Attachment.cpp 翻译
    }

    /// 覆盖缩放
    pub fn override_scale(&mut self, _scale: f32) {
        // TODO: 从 Attachment.cpp 翻译
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics, _parent_hidden: bool) {
        // TODO: 从 Attachment.cpp 翻译
    }

    /// 销毁
    pub fn die(&mut self) {
        // TODO: 从 Attachment.cpp 翻译
    }

    /// 分离
    pub fn detach(&mut self) {
        // TODO: 从 Attachment.cpp 翻译
    }

    /// 交叉淡出
    pub fn cross_fade(&mut self, _name: &str) {
        // TODO: 从 Attachment.cpp 翻译
    }

    /// 传播颜色
    pub fn propogate_color(
        &mut self,
        _color: &Color,
        _enable_additive_color: bool,
        _additive_color: &Color,
        _enable_overlay_color: bool,
        _overlay_color: &Color,
    ) {
        // TODO: 从 Attachment.cpp 翻译
    }
}

impl Default for Attachment {
    fn default() -> Self {
        Attachment::new()
    }
}

/// 附着物持有者（对应 C++ AttachmentHolder）
pub struct AttachmentHolder {
    pub attachments: DataArray<Attachment>,
}

impl AttachmentHolder {
    pub fn new() -> Self {
        AttachmentHolder {
            attachments: DataArray::new(),
        }
    }

    pub fn initialize_holder(&mut self) {
        // TODO: 从 Attachment.cpp 翻译
    }

    pub fn dispose_holder(&mut self) {
        // TODO: 从 Attachment.cpp 翻译
    }

    pub fn alloc_attachment(&mut self) -> Option<*mut Attachment> {
        // TODO: 从 Attachment.cpp 翻译
        None
    }
}

// ── 自由函数 ──────────────────────────────────────────

/// 附着重动画效果
pub fn attach_reanim(
    _attachment_id: &mut AttachmentID,
    _reanimation: *mut std::ffi::c_void,
    _offset_x: f32,
    _offset_y: f32,
) -> Option<*mut AttachEffect> {
    None
}

/// 附着粒子效果
pub fn attach_particle(
    _attachment_id: &mut AttachmentID,
    _particle_system: *mut std::ffi::c_void,
    _offset_x: f32,
    _offset_y: f32,
) -> Option<*mut AttachEffect> {
    None
}

/// 附着拖尾效果
pub fn attach_trail(
    _attachment_id: &mut AttachmentID,
    _trail: *mut std::ffi::c_void,
    _offset_x: f32,
    _offset_y: f32,
) -> Option<*mut AttachEffect> {
    None
}

/// 创建效果附着
pub fn create_effect_attachment(
    _attachment_id: &mut AttachmentID,
    _effect_type: EffectType,
    _data_id: u32,
    _offset_x: f32,
    _offset_y: f32,
) -> Option<*mut AttachEffect> {
    None
}

/// 查找第一个附着
pub fn find_first_attachment(_attachment_id: &mut AttachmentID) -> Option<*mut AttachEffect> {
    None
}

/// 查找重动画附着
pub fn find_reanim_attachment(_attachment_id: &mut AttachmentID) -> Option<*mut std::ffi::c_void> {
    None
}

/// 清理已死亡的效果（对应 C++ PruneDeadEffects）
/// 遍历附着的所有效果，移除已死亡的效果；
/// 如果所有效果都死亡，则标记附着物本身为死亡
pub fn prune_dead_effects(attachment: &mut Attachment) {
    let mut i = 0;
    while i < attachment.num_effects {
        let effect = &attachment.effect_array[i as usize];
        let still_alive = match effect.effect_type {
            EffectType::Particle
            | EffectType::Trail
            | EffectType::Reanim
            | EffectType::Attachment => {
                // 简化：当 effect_id != 0 时认为仍然存活
                effect.effect_id != 0
            }
            EffectType::Other => true,
        };

        if !still_alive {
            // 移除当前效果：将后续效果前移（使用 clone 代替 copy）
            let remaining = (attachment.num_effects - i - 1) as usize;
            if remaining > 0 {
                for j in 0..remaining {
                    attachment.effect_array[(i as usize) + j] = attachment.effect_array[(i as usize) + j + 1].clone();
                }
            }
            attachment.num_effects -= 1;
            // 不递增 i，继续检查新的当前位置
        } else {
            i += 1;
        }
    }

    if attachment.num_effects == 0 {
        attachment.dead = true;
    }
}
