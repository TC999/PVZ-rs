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

    /// 更新所有附着效果（对应 C++ Attachment::Update，Attachment.cpp:104）
    pub fn update(&mut self) {
        // 对应 C++: 遍历效果数组，更新子实体；已死亡/缺失的移除（memmove 前移语义）
        let mut i = 0usize;
        while i < self.num_effects as usize {
            let a_attach_effect = &self.effect_array[i];
            let mut is_empty = true;
            match a_attach_effect.effect_type {
                EffectType::Particle => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_ps) = es.particle_systems.get_mut(a_attach_effect.effect_id as usize) {
                                if !a_ps.dead {
                                    a_ps.update();
                                    is_empty = false;
                                }
                            }
                        }
                    }
                }
                // 对应 C++ Attachment::Update（Attachment.cpp:125）：Trail 分支 — 存活时更新轨迹
                EffectType::Trail => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_trail) = es.trails.get_mut(a_attach_effect.effect_id as usize) {
                                if !a_trail.m_dead {
                                    a_trail.update();
                                    is_empty = false;
                                }
                            }
                        }
                    }
                }
                EffectType::Reanim => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_reanim) = es.reanimations.get_mut(a_attach_effect.effect_id as usize) {
                                if !a_reanim.m_dead {
                                    a_reanim.update();
                                    is_empty = false;
                                }
                            }
                        }
                    }
                }
                EffectType::Attachment => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_attach) = es.attachments.get_mut(a_attach_effect.effect_id as usize) {
                                a_attach.update();
                                is_empty = false;
                            }
                        }
                    }
                }
                EffectType::Trail | EffectType::Other => {}
            }

            if is_empty {
                // 对应 C++: memmove 前移剩余效果并递减数量（AttachEffect 非 Copy，用 clone）
                let a_num_remaining = self.num_effects as usize - i - 1;
                for k in 0..a_num_remaining {
                    self.effect_array[i + k] = self.effect_array[i + k + 1].clone();
                }
                self.num_effects -= 1;
                // 重新检查当前位置（对应 C++ --i）
            } else {
                i += 1;
            }
        }

        if self.num_effects == 0 {
            self.dead = true;
        }
    }

    /// 设置位置（对应 C++ Attachment::SetPosition，:181）
    pub fn set_position(&mut self, position: &SexyVector2) {
        for i in 0..self.num_effects as usize {
            let a_new_pos = self.effect_array[i].offset * *position;
            match self.effect_array[i].effect_type {
                EffectType::Particle => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_ps) = es.particle_systems.get_mut(self.effect_array[i].effect_id as usize) {
                                a_ps.system_move(a_new_pos.x, a_new_pos.y);
                            }
                        }
                    }
                }
                // 对应 C++ Attachment::SetPosition（Attachment.cpp:202）：Trail 分支 — 追加轨迹点
                EffectType::Trail => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_trail) = es.trails.get_mut(self.effect_array[i].effect_id as usize) {
                                a_trail.add_point(a_new_pos.x, a_new_pos.y);
                            }
                        }
                    }
                }
                EffectType::Reanim => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_reanim) = es.reanimations.get_mut(self.effect_array[i].effect_id as usize) {
                                a_reanim.set_position(a_new_pos.x, a_new_pos.y);
                            }
                        }
                    }
                }
                EffectType::Attachment => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_attach) = es.attachments.get_mut(self.effect_array[i].effect_id as usize) {
                                a_attach.set_position(&a_new_pos);
                            }
                        }
                    }
                }
                EffectType::Trail | EffectType::Other => {}
            }
        }
    }

    /// 设置矩阵（对应 C++ Attachment::SetMatrix，:402）
    pub fn set_matrix(&mut self, matrix: &SexyMatrix3) {
        for i in 0..self.num_effects as usize {
            // 对应 C++: SexyTransform2D aPosition = theMatrix * aAttachEffect->mOffset
            let a_position = matrix.multiply(&self.effect_array[i].offset);
            match self.effect_array[i].effect_type {
                EffectType::Particle => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_ps) = es.particle_systems.get_mut(self.effect_array[i].effect_id as usize) {
                                a_ps.system_move(a_position.m[0][2], a_position.m[1][2]);
                            }
                        }
                    }
                }
                // [TRANSLATION_NOTE]: C++ 中 Trail 分支依赖 Trail 系统（Rust 暂无）
                EffectType::Reanim => {
                    // [TRANSLATION_NOTE]: C++ 中 aReanimation->mOverlayMatrix = aPosition（渲染矩阵）；
                    // Rust Reanimation 无 m_overlay_matrix 字段，以平移分量近似
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_reanim) = es.reanimations.get_mut(self.effect_array[i].effect_id as usize) {
                                a_reanim.set_position(a_position.m[0][2], a_position.m[1][2]);
                            }
                        }
                    }
                }
                EffectType::Attachment => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_attach) = es.attachments.get_mut(self.effect_array[i].effect_id as usize) {
                                a_attach.set_matrix(&a_position);
                            }
                        }
                    }
                }
                EffectType::Trail | EffectType::Other => {}
            }
        }
    }

    /// 覆盖颜色（对应 C++ Attachment::OverrideColor，:238）
    pub fn override_color(&mut self, color: &Color) {
        for i in 0..self.num_effects as usize {
            match self.effect_array[i].effect_type {
                EffectType::Particle => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_ps) = es.particle_systems.get_mut(self.effect_array[i].effect_id as usize) {
                                a_ps.override_color("", color);
                            }
                        }
                    }
                }
                EffectType::Reanim => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_reanim) = es.reanimations.get_mut(self.effect_array[i].effect_id as usize) {
                                a_reanim.m_color_override = *color;
                            }
                        }
                    }
                }
                EffectType::Attachment => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_attach) = es.attachments.get_mut(self.effect_array[i].effect_id as usize) {
                                a_attach.override_color(color);
                            }
                        }
                    }
                }
                EffectType::Trail | EffectType::Other => {}
            }
        }
    }

    /// 覆盖缩放（对应 C++ Attachment::OverrideScale，:339）
    pub fn override_scale(&mut self, scale: f32) {
        for i in 0..self.num_effects as usize {
            match self.effect_array[i].effect_type {
                EffectType::Particle => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_ps) = es.particle_systems.get_mut(self.effect_array[i].effect_id as usize) {
                                a_ps.override_scale("", scale);
                            }
                        }
                    }
                }
                EffectType::Reanim => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_reanim) = es.reanimations.get_mut(self.effect_array[i].effect_id as usize) {
                                a_reanim.m_override_scale_x = scale;
                                a_reanim.m_override_scale_y = scale;
                            }
                        }
                    }
                }
                EffectType::Attachment => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_attach) = es.attachments.get_mut(self.effect_array[i].effect_id as usize) {
                                a_attach.override_scale(scale);
                            }
                        }
                    }
                }
                EffectType::Trail | EffectType::Other => {}
            }
        }
    }

    /// 交叉淡出（对应 C++ Attachment::CrossFade，:384：对附着的粒子系统生效）
    pub fn cross_fade(&mut self, name: &str) {
        for i in 0..self.num_effects as usize {
            if self.effect_array[i].effect_type == EffectType::Particle {
                if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                    if let Some(es) = app.effect_system.as_mut() {
                        if let Some(a_ps) = es.particle_systems.get_mut(self.effect_array[i].effect_id as usize) {
                            a_ps.cross_fade(name);
                        }
                    }
                }
            }
        }
    }

    /// 绘制（对应 C++ Attachment::Draw，:458）
    pub fn draw(&self, g: &mut Graphics, parent_hidden: bool) {
        for i in 0..self.num_effects as usize {
            let a_attach_effect = &self.effect_array[i];
            // 对应 C++: 父级隐藏时跳过 dont_draw_if_parent_hidden 的效果
            if parent_hidden && a_attach_effect.dont_draw_if_parent_hidden {
                continue;
            }
            match a_attach_effect.effect_type {
                EffectType::Particle => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_ref() {
                            if let Some(a_ps) = es.particle_systems.get(a_attach_effect.effect_id as usize) {
                                a_ps.draw(g);
                            }
                        }
                    }
                }
                // [TRANSLATION_NOTE]: C++ 中 Trail 分支依赖 Trail 系统（Rust 暂无）
                EffectType::Reanim => {
                    // [TRANSLATION_NOTE]: C++ 中 aReanimation->DrawAllRenderGroups(g)；
                    // Rust 以 Reanimation::draw 近似（渲染组遍历依赖 reanim 完整实现）
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_ref() {
                            if let Some(a_reanim) = es.reanimations.get(a_attach_effect.effect_id as usize) {
                                a_reanim.draw(g);
                            }
                        }
                    }
                }
                EffectType::Attachment => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_ref() {
                            if let Some(a_attach) = es.attachments.get(a_attach_effect.effect_id as usize) {
                                a_attach.draw(g, parent_hidden);
                            }
                        }
                    }
                }
                EffectType::Trail | EffectType::Other => {}
            }
        }
    }

    /// 销毁（对应 C++ Attachment::AttachmentDie，:591）
    pub fn die(&mut self) {
        // 对应 C++: 无效果时直接标记死亡
        if self.num_effects == 0 {
            self.dead = true;
            return;
        }
        for i in 0..self.num_effects as usize {
            match self.effect_array[i].effect_type {
                EffectType::Particle => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_ps) = es.particle_systems.get_mut(self.effect_array[i].effect_id as usize) {
                                a_ps.particle_system_die();
                            }
                        }
                    }
                }
                // [TRANSLATION_NOTE]: C++ 中 Trail 分支依赖 Trail 系统（Rust 暂无）
                EffectType::Reanim => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_reanim) = es.reanimations.get_mut(self.effect_array[i].effect_id as usize) {
                                a_reanim.reanimation_die();
                            }
                        }
                    }
                }
                EffectType::Attachment => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_attach) = es.attachments.get_mut(self.effect_array[i].effect_id as usize) {
                                a_attach.die();
                            }
                        }
                    }
                }
                EffectType::Trail | EffectType::Other => {}
            }
            // 对应 C++: aAttachEffect->mEffectID = 0
            self.effect_array[i].effect_id = 0;
        }
        self.num_effects = 0;
        self.dead = true;
    }

    /// 分离（对应 C++ Attachment::Detach，:523）
    pub fn detach(&mut self) {
        for i in 0..self.num_effects as usize {
            match self.effect_array[i].effect_type {
                EffectType::Particle => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_ps) = es.particle_systems.get_mut(self.effect_array[i].effect_id as usize) {
                                a_ps.is_attachment = false;
                            }
                        }
                    }
                }
                // [TRANSLATION_NOTE]: C++ 中 Trail 分支依赖 Trail 系统（Rust 暂无）
                EffectType::Reanim => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_reanim) = es.reanimations.get_mut(self.effect_array[i].effect_id as usize) {
                                a_reanim.m_is_attachment = false;
                            }
                        }
                    }
                }
                EffectType::Attachment => {
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_attach) = es.attachments.get_mut(self.effect_array[i].effect_id as usize) {
                                a_attach.detach();
                            }
                        }
                    }
                }
                EffectType::Trail | EffectType::Other => {}
            }
            // 对应 C++: aAttachEffect->mEffectID = 0
            self.effect_array[i].effect_id = 0;
        }
        self.num_effects = 0;
        self.dead = true;
    }

    /// 传播颜色
    pub fn propogate_color(
        &mut self,
        color: &Color,
        enable_additive_color: bool,
        additive_color: &Color,
        enable_overlay_color: bool,
        overlay_color: &Color,
    ) {
        // 对应 C++ Attachment::PropogateColor: 遍历效果数组，向子实体传播颜色
        for i in 0..self.num_effects as usize {
            let a_attach_effect = &self.effect_array[i];
            if a_attach_effect.dont_propogate_color {
                continue;
            }
            match a_attach_effect.effect_type {
                EffectType::Particle => {
                    // [TRANSLATION_NOTE]: C++ 中为粒子的 OverrideColor/OverrideExtraAdditiveDraw，
                    // 依赖 tod_particle 完整实现（当前 stub）
                }
                EffectType::Reanim => {
                    let a_effect_id = a_attach_effect.effect_id;
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_reanim) = es.reanimations.get_mut(a_effect_id as usize) {
                                a_reanim.m_color_override = *color;
                                a_reanim.m_extra_additive_color = *additive_color;
                                a_reanim.m_enable_extra_additive_draw = enable_additive_color;
                                a_reanim.m_extra_overlay_color = *overlay_color;
                                a_reanim.m_enable_extra_overlay_draw = enable_overlay_color;
                                a_reanim.propogate_color_to_attachments();
                            }
                        }
                    }
                }
                EffectType::Attachment => {
                    let a_effect_id = a_attach_effect.effect_id;
                    if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                        if let Some(es) = app.effect_system.as_mut() {
                            if let Some(a_attachment) = es.attachments.get_mut(a_effect_id as usize) {
                                a_attachment.propogate_color(
                                    color,
                                    enable_additive_color,
                                    additive_color,
                                    enable_overlay_color,
                                    overlay_color,
                                );
                            }
                        }
                    }
                }
                EffectType::Trail | EffectType::Other => {}
            }
        }
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
        // 对应 C++ AttachmentHolder::InitializeHolder（Attachment.cpp:681）
        self.attachments.initialize(1024, "attachments");
    }

    pub fn dispose_holder(&mut self) {
        // 对应 C++ AttachmentHolder::DisposeHolder（:686）
        self.attachments.dispose();
    }

    pub fn alloc_attachment(&mut self) -> Option<*mut Attachment> {
        // 对应 C++ AttachmentHolder::AllocAttachment
        let ptr = self.attachments.alloc();
        if ptr.is_null() {
            return None;
        }
        Some(ptr)
    }
}

// ── 自由函数 ──────────────────────────────────────────

/// 附着重动画效果（对应 C++ AttachReanim，Attachment.cpp）
pub fn attach_reanim(
    the_attachment_id: &mut AttachmentID,
    the_reanimation: *mut std::ffi::c_void,
    the_offset_x: f32,
    the_offset_y: f32,
) -> Option<*mut AttachEffect> {
    let app = crate::lawn::lawn_app::LawnApp::instance()?;
    let es = app.effect_system.as_mut()?;
    // C++: mReanimations.DataArrayGetID(theReanimation) —— Rust Vec 按指针查索引
    let a_reanim_id = es.reanimations.iter().position(|r| {
        std::ptr::eq(r as *const crate::todlib::reanimator::Reanimation,
                     the_reanimation as *const crate::todlib::reanimator::Reanimation)
    })? as u32;
    let a_attach_effect = create_effect_attachment(
        the_attachment_id, EffectType::Reanim, a_reanim_id, the_offset_x, the_offset_y,
    )?;
    // C++: theReanimation->mIsAttachment = true
    if let Some(a_reanim) = es.reanimations.get_mut(a_reanim_id as usize) {
        a_reanim.m_is_attachment = true;
    }
    Some(a_attach_effect)
}

/// 附着粒子效果（对应 C++ AttachParticle，Attachment.cpp）
pub fn attach_particle(
    the_attachment_id: &mut AttachmentID,
    the_particle_system: *mut std::ffi::c_void,
    the_offset_x: f32,
    the_offset_y: f32,
) -> Option<*mut AttachEffect> {
    let app = crate::lawn::lawn_app::LawnApp::instance()?;
    let es = app.effect_system.as_mut()?;
    let a_ps_id = es.particle_systems.iter().position(|ps| {
        std::ptr::eq(ps as *const crate::todlib::tod_particle::TodParticleSystem,
                     the_particle_system as *const crate::todlib::tod_particle::TodParticleSystem)
    })? as u32;
    let a_attach_effect = create_effect_attachment(
        the_attachment_id, EffectType::Particle, a_ps_id, the_offset_x, the_offset_y,
    )?;
    if let Some(a_ps) = es.particle_systems.get_mut(a_ps_id as usize) {
        a_ps.is_attachment = true;
    }
    Some(a_attach_effect)
}

/// 附着拖尾效果（对应 C++ AttachTrail，Attachment.cpp）
/// [TRANSLATION_NOTE]: Rust EffectSystem 尚无 trails 存储（mTrailHolder 未接入），暂保留占位
pub fn attach_trail(
    _attachment_id: &mut AttachmentID,
    _trail: *mut std::ffi::c_void,
    _offset_x: f32,
    _offset_y: f32,
) -> Option<*mut AttachEffect> {
    None
}

/// 附件是否已满（对应 C++ IsFullOfAttachments，Attachment.cpp:1009）
pub fn is_full_of_attachments(the_attachment_id: AttachmentID) -> bool {
    let app = match crate::lawn::lawn_app::LawnApp::instance() {
        Some(a) => a,
        None => return false,
    };
    let es = match app.effect_system.as_ref() {
        Some(e) => e,
        None => return false,
    };
    let a_attachment = match es.attachments.get(the_attachment_id as usize) {
        Some(a) => a,
        None => return false,
    };
    a_attachment.num_effects >= MAX_EFFECTS_PER_ATTACHMENT as i32
}

/// 创建效果附着（对应 C++ CreateEffectAttachment，Attachment.cpp）
/// attachment id 无效或已死时分配新 Attachment 并回写 id
pub fn create_effect_attachment(
    the_attachment_id: &mut AttachmentID,
    the_effect_type: EffectType,
    the_data_id: u32,
    the_offset_x: f32,
    the_offset_y: f32,
) -> Option<*mut AttachEffect> {
    let app = crate::lawn::lawn_app::LawnApp::instance()?;
    let es = app.effect_system.as_mut()?;
    unsafe {
        // C++: DataArrayTryToGet(id)；无效或 mDead → AllocAttachment
        let a_existing: Option<usize> = {
            let id = *the_attachment_id;
            if id != ATTACHMENTID_NULL {
                let idx = id as usize;
                if idx < es.attachments.len() && !es.attachments[idx].dead {
                    Some(idx)
                } else {
                    None
                }
            } else {
                None
            }
        };
        let a_attachment: *mut Attachment;
        let a_index = match a_existing {
            Some(idx) => idx,
            None => {
                es.attachments.push(Attachment::new());
                let idx = es.attachments.len() - 1;
                *the_attachment_id = idx as AttachmentID;
                idx
            }
        };
        a_attachment = &mut es.attachments[a_index];
        // C++: PVZP_ASSERT(mNumEffects < MAX_EFFECTS_PER_ATTACHMENT)
        if (*a_attachment).num_effects as usize >= MAX_EFFECTS_PER_ATTACHMENT {
            return None;
        }
        let a_attach_effect = &mut (*a_attachment).effect_array[(*a_attachment).num_effects as usize];
        a_attach_effect.effect_type = the_effect_type;
        a_attach_effect.effect_id = the_data_id;
        a_attach_effect.dont_draw_if_parent_hidden = false;
        // C++: mOffset.LoadIdentity(); mOffset.m02 = theOffsetX; mOffset.m12 = theOffsetY
        a_attach_effect.offset = SexyMatrix3::translation(the_offset_x, the_offset_y);
        (*a_attachment).num_effects += 1;
        Some(a_attach_effect as *mut AttachEffect)
    }
}

/// 查找第一个附着（对应 C++ FindFirstAttachment，Attachment.cpp）
/// [TRANSLATION_NOTE]: 有附着效果时返回第一个（C++ 语义：返回效果数组中索引 0 的效果）
pub fn find_first_attachment(the_attachment_id: &mut AttachmentID) -> Option<*mut AttachEffect> {
    let app = crate::lawn::lawn_app::LawnApp::instance()?;
    let es = app.effect_system.as_ref()?;
    let id = *the_attachment_id;
    if id == ATTACHMENTID_NULL {
        return None;
    }
    unsafe {
        let a_attachment = es.attachments.get(id as usize)?;
        if a_attachment.dead || a_attachment.num_effects <= 0 {
            return None;
        }
        Some(&a_attachment.effect_array[0] as *const AttachEffect as *mut AttachEffect)
    }
}

/// 查找重动画附着（对应 C++ FindReanimAttachment，Attachment.cpp）
pub fn find_reanim_attachment(the_attachment_id: &mut AttachmentID) -> Option<*mut std::ffi::c_void> {
    let app = crate::lawn::lawn_app::LawnApp::instance()?;
    let es = app.effect_system.as_ref()?;
    let id = *the_attachment_id;
    if id == ATTACHMENTID_NULL {
        return None;
    }
    unsafe {
        let a_attachment = es.attachments.get(id as usize)?;
        if a_attachment.dead {
            return None;
        }
        for i in 0..a_attachment.num_effects as usize {
            let a_attach_effect = &a_attachment.effect_array[i];
            if a_attach_effect.effect_type == EffectType::Reanim {
                let a_reanim = es.reanimations.get(a_attach_effect.effect_id as usize)?;
                return Some(a_reanim as *const crate::todlib::reanimator::Reanimation as *mut std::ffi::c_void);
            }
        }
    }
    None
}

/// 传播颜色到附着物（对应 C++ AttachmentPropogateColor，Attachment.cpp）
pub fn attachment_propogate_color(
    the_attachment_id: &mut AttachmentID,
    color: &Color,
    enable_additive_color: bool,
    additive_color: &Color,
    enable_overlay_color: bool,
    overlay_color: &Color,
) {
    // 对应 C++: ATTACHMENTID_NULL 直接返回
    if *the_attachment_id == ATTACHMENTID_NULL {
        return;
    }
    let app = match crate::lawn::lawn_app::LawnApp::instance() {
        Some(app) => app,
        None => return,
    };
    let es = match app.effect_system.as_mut() {
        Some(es) => es,
        None => return,
    };
    let id = *the_attachment_id;
    if let Some(a_attachment) = es.attachments.get_mut(id as usize) {
        a_attachment.propogate_color(
            color,
            enable_additive_color,
            additive_color,
            enable_overlay_color,
            overlay_color,
        );
    }
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
