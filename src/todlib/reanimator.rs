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
    // C++ ConstEnums.h ReanimLoopType 追加（Boss 火焰球等使用）
    PlayOnceFullLastFrame,
    PlayOnceFullLastFrameAndHold,
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
    // 对应 C++ mExtraAdditiveColor（附加加法混合颜色，DrawReanim 使用）
    pub m_extra_additive_color: Color,
    // 对应 C++ mEnableExtraAdditiveDraw（是否启用附加加法绘制）
    pub m_enable_extra_additive_draw: bool,
    // 对应 C++ mExtraOverlayColor（覆盖色，TreeOfWisdomDraw 使用）
    pub m_extra_overlay_color: Color,
    // 对应 C++ mEnableExtraOverlayDraw（是否启用覆盖色绘制）
    pub m_enable_extra_overlay_draw: bool,
    // 对应 C++ mLastFrameTime（上一帧动画时间，用于 ShouldTriggerTimedEvent）
    pub m_last_anim_time: f32,
    // 对应 C++ mRenderOrder（渲染顺序）
    pub m_render_order: i32,
    // 对应 C++ mDead（动画是否已请求销毁）
    pub m_dead: bool,
    // 对应 C++ mFrameStart/mFrameCount（SetFramesForLayer 计算的帧区间）
    pub m_frame_start: i32,
    pub m_frame_count: i32,
    // 对应 C++ mOverlayMatrix（轨道绘制时叠加的 3x3 矩阵，DrawRenderGroup 应用）
    pub m_overlay_matrix: crate::framework::sexy_matrix::SexyMatrix3,
    // 对应 C++ mFilterEffect（Reanimator.h:210，轨道绘制前对图像应用滤镜）
    pub m_filter_effect: crate::todlib::filter_effect::FilterEffectType,
    // 对应 C++ 的轨道图片覆盖表（trackName -> Image）
    pub m_image_overrides: Vec<(String, *mut Image)>,
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
            m_extra_additive_color: Color::BLACK,
            m_enable_extra_additive_draw: false,
            m_extra_overlay_color: Color::WHITE,
            m_enable_extra_overlay_draw: false,
            m_last_anim_time: 0.0,
            m_render_order: 0,
            m_dead: false,
            m_frame_start: 0,
            m_frame_count: 0,
            m_overlay_matrix: crate::framework::sexy_matrix::SexyMatrix3::identity(),
            m_filter_effect: crate::todlib::filter_effect::FilterEffectType::None,
            m_image_overrides: Vec::new(),
        }
    }

    /// 更新动画（对应 C++ Reanimation::Update，Reanimator.cpp:416）
    /// m_anim_time 为 0~1 归一化时间；推进量 = SECONDS_PER_UPDATE * mAnimRate / mFrameCount
    pub fn update(&mut self) {
        if self.m_frame_count == 0 || self.m_dead {
            return;
        }
        self.m_last_anim_time = self.m_anim_time;
        self.m_anim_time += SECONDS_PER_UPDATE as f32 * self.m_anim_rate / self.m_frame_count as f32;

        if self.m_anim_rate > 0.0 {
            match self.m_loop_type {
                ReanimLoopType::Loop | ReanimLoopType::LoopFullOffset => {
                    while self.m_anim_time >= 1.0 {
                        self.m_loop_count += 1;
                        self.m_anim_time -= 1.0;
                    }
                }
                ReanimLoopType::PlayOnceAndRemove | ReanimLoopType::PlayOnceFullLastFrame => {
                    if self.m_anim_time >= 1.0 {
                        self.m_loop_count = 1;
                        self.m_anim_time = 1.0;
                        self.m_dead = true;
                    }
                }
                ReanimLoopType::PlayOnceAndHold | ReanimLoopType::PlayOnceFullLastFrameAndHold => {
                    if self.m_anim_time >= 1.0 {
                        self.m_loop_count = 1;
                        self.m_anim_time = 1.0;
                    }
                }
                // [TRANSLATION_NOTE]: C++ 当前版本无 REANIM_PLAY_ONCE_AND_RETURN_TO_ZERO；
                // 保持既有 Rust 行为（播完停在末尾）
                _ => {
                    if self.m_anim_time >= 1.0 {
                        self.m_loop_count = 1;
                        self.m_anim_time = 1.0;
                    }
                }
            }
        } else if self.m_anim_rate < 0.0 {
            match self.m_loop_type {
                ReanimLoopType::Loop | ReanimLoopType::LoopFullOffset => {
                    while self.m_anim_time < 0.0 {
                        self.m_loop_count += 1;
                        self.m_anim_time += 1.0;
                    }
                }
                ReanimLoopType::PlayOnceAndRemove | ReanimLoopType::PlayOnceFullLastFrame => {
                    if self.m_anim_time < 0.0 {
                        self.m_loop_count = 1;
                        self.m_anim_time = 0.0;
                        self.m_dead = true;
                    }
                }
                ReanimLoopType::PlayOnceAndHold | ReanimLoopType::PlayOnceFullLastFrameAndHold => {
                    if self.m_anim_time < 0.0 {
                        self.m_loop_count = 1;
                        self.m_anim_time = 0.0;
                    }
                }
                _ => {
                    if self.m_anim_time < 0.0 {
                        self.m_loop_count = 1;
                        self.m_anim_time = 0.0;
                    }
                }
            }
        }

        // 逐轨道更新（对应 C++ Reanimation::Update 后半段，Reanimator.cpp:494-515）
        if let Some(def) = self.m_definition {
            unsafe {
                let def_ref = &*def;
                let track_count = def_ref.m_tracks.len();
                for a_track_index in 0..track_count {
                    let a_track_name = &def_ref.m_tracks[a_track_index].m_name;
                    // 对应 C++: if (mBlendCounter > 0) mBlendCounter--;
                    if let Some(a_track) = self.m_track_instances.get_mut(a_track_index) {
                        if a_track.m_blend_count > 0.0 {
                            a_track.m_blend_count -= 1.0;
                        }
                        // 对应 C++: if (mShakeOverride != 0) 随机化 shake
                        if a_track.m_shake_override != 0.0 {
                            a_track.m_shake_x = crate::todlib::tod_common::rand_range_float(
                                -a_track.m_shake_override, a_track.m_shake_override);
                            a_track.m_shake_y = crate::todlib::tod_common::rand_range_float(
                                -a_track.m_shake_override, a_track.m_shake_override);
                        }
                    }
                    // 对应 C++: if (strncasecmp(name, "attacher__", 10) == 0) UpdateAttacherTrack
                    // （放 get_mut 借用之后，避免与 &mut self 冲突）
                    if a_track_name.len() >= 10
                        && a_track_name[..10].eq_ignore_ascii_case("attacher__")
                    {
                        self.update_attacher_track(a_track_index);
                    }
                    // 对应 C++: if (mAttachmentID != ATTACHMENTID_NULL)
                    //   GetAttachmentOverlayMatrix + AttachmentUpdateAndSetMatrix
                    if let Some(a_track) = self.m_track_instances.get(a_track_index) {
                        if a_track.m_attachment_id != crate::lawn::game_enums::ATTACHMENTID_NULL {
                            let mut a_attachment_id = a_track.m_attachment_id;
                            let a_overlay_matrix = self.get_attachment_overlay_matrix(a_track_index as i32);
                            crate::todlib::attachment::attachment_update_and_set_matrix(
                                &mut a_attachment_id,
                                &a_overlay_matrix,
                            );
                        }
                    }
                }
            }
        }
    }

    /// 绘制全部轨道（对应 C++ Reanimation::Draw = DrawRenderGroup(NORMAL)）
    pub fn draw(&self, g: &mut Graphics) {
        self.draw_render_group(g, 0); // RENDER_GROUP_NORMAL
    }

    /// 按渲染组绘制（对应 C++ Reanimation::DrawRenderGroup + DrawTrack，Reanimator.cpp:635-937）
    /// 完整矩阵链：pivot 居中 → MatrixFromTransform → × mOverlayMatrix → 平移(shake + g 平移)
    pub fn draw_render_group(&self, g: &mut Graphics, the_render_group: i32) {
        let Some(def_ptr) = self.m_definition else { return };
        if self.m_dead {
            return;
        }
        unsafe {
            let def = &*def_ptr;
            if def.m_tracks.is_empty() {
                return;
            }
            for (i, track_def) in def.m_tracks.iter().enumerate() {
                let ti = match self.m_track_instances.get(i) {
                    Some(t) => t,
                    None => continue,
                };
                if ti.m_render_group != the_render_group {
                    continue;
                }
                let frames = &track_def.m_transforms;
                if frames.is_empty() {
                    continue;
                }
                // C++ GetCurrentTransform（Reanimator.cpp:550）：GetFrameTime → GetTransformAtTime 帧间插值 → BlendTransform
                let mut a_transform = crate::todlib::definition::ReanimatorTransform::default();
                if !self.get_current_transform(i as i32, &mut a_transform) {
                    continue;
                }
                // C++ DrawTrack：aImageFrame = FloatRoundToInt(mFrame)；< 0 不绘制
                let a_image_frame = a_transform.m_frame.round() as i32;
                if a_image_frame < 0 {
                    continue;
                }

                // 对应 C++: aColor = mTrackColor × mColorOverride（mIgnoreColorOverride 时不乘）
                // ColorsMultiply：逐通道乘法（/255）
                let colors_multiply = |a: &Color, b: &Color| -> Color {
                    Color::new(
                        ((a.r as u32 * b.r as u32) / 255) as u8,
                        ((a.g as u32 * b.g as u32) / 255) as u8,
                        ((a.b as u32 * b.b as u32) / 255) as u8,
                        ((a.a as u32 * b.a as u32) / 255) as u8,
                    )
                };
                let mut a_color = ti.m_track_color;
                if !ti.m_ignore_color_override {
                    a_color = colors_multiply(&a_color, &self.m_color_override);
                }
                if g.get_colorize_images() {
                    a_color = colors_multiply(&a_color, g.get_color());
                }
                let a_image_alpha = (a_transform.m_alpha * a_color.a as f32).round().clamp(0.0, 255.0) as i32;
                if a_image_alpha <= 0 {
                    continue;
                }
                a_color.a = a_image_alpha as u8;

                // 对应 C++: mEnableExtraAdditiveDraw / mEnableExtraOverlayDraw 附加色
                let a_extra_additive_color;
                if self.m_enable_extra_additive_draw {
                    let mut c = self.m_extra_additive_color;
                    c.a = ((c.a as u32 * a_image_alpha as u32) / 255) as u8;
                    a_extra_additive_color = c;
                } else {
                    a_extra_additive_color = Color::BLACK;
                }
                let a_extra_overlay_color;
                if self.m_enable_extra_overlay_draw {
                    let mut c = self.m_extra_overlay_color;
                    c.a = ((c.a as u32 * a_image_alpha as u32) / 255) as u8;
                    a_extra_overlay_color = c;
                } else {
                    a_extra_overlay_color = Color::WHITE;
                }

                // 对应 C++: mIgnoreClipRect → 全屏裁剪；否则用 g 当前裁剪
                let a_clip_rect = if ti.m_ignore_clip_rect {
                    crate::framework::rect::Rect::new(0, 0, crate::lawn::game_enums::BOARD_WIDTH, crate::lawn::game_enums::BOARD_HEIGHT)
                } else {
                    g.clip_rect
                };
                // 对应 C++ DrawTrack: g->SetClipRect(aClipRect)（绘制后恢复）
                let a_old_clip_rect = g.clip_rect;
                if g.clip_rect != a_clip_rect {
                    g.set_clip_rect(&a_clip_rect);
                }
                let restore_clip = g.clip_rect != a_old_clip_rect;

                // 对应 C++: aImage（transform 图片 or mImageOverride），atlas 编码句柄解码
                let mut a_image: *mut crate::framework::graphics::image::Image = std::ptr::null_mut();
                let mut a_atlas_image: Option<crate::todlib::reanim_atlas::ReanimAtlasImage> = None;
                let atlas_ptr: Option<*mut crate::todlib::reanim_atlas::ReanimAtlas> = def.m_reanim_atlas;
                if atlas_ptr.is_some() {
                    // 对应 C++: GetEncodedReanimAtlas(aImage) 解码 atlas 句柄
                    if a_transform.m_image > 0 {
                        let a_atlas = &*atlas_ptr.unwrap();
                        a_atlas_image = a_atlas.get_encoded_reanim_atlas(a_transform.m_image).copied();
                        if a_atlas_image.is_none() && a_transform.m_image <= 1000 {
                            // Invalid encoded handle
                        } else if a_atlas_image.is_none() {
                            a_image = crate::todlib::reanim_loader::reanimator_get_image(a_transform.m_image)
                                .unwrap_or(std::ptr::null_mut());
                        }
                    }
                } else {
                    a_image = crate::todlib::reanim_loader::reanimator_get_image(a_transform.m_image)
                        .unwrap_or(std::ptr::null_mut());
                }
                if !ti.m_image_override.is_null() {
                    a_image = ti.m_image_override;
                    a_atlas_image = None;
                }

                // 对应 C++: 计算 pivot 矩阵（图片中心 / atlas 图中心 / font 文本）
                let mut a_matrix = crate::framework::sexy_matrix::SexyMatrix3::identity();
                let mut a_full_screen = false;
                if let Some(a_pivot_atlas) = a_atlas_image {
                    a_matrix = crate::framework::sexy_matrix::SexyMatrix3::new_from_values(
                        1.0, 0.0, a_pivot_atlas.width as f32 * 0.5,
                        0.0, 1.0, a_pivot_atlas.height as f32 * 0.5,
                        0.0, 0.0, 1.0,
                    );
                } else if !a_image.is_null() {
                    let a_img = &*a_image;
                    let a_cel_width = a_img.get_cel_width();
                    let a_cel_height = a_img.get_cel_height();
                    a_matrix = crate::framework::sexy_matrix::SexyMatrix3::new_from_values(
                        1.0, 0.0, a_cel_width as f32 * 0.5,
                        0.0, 1.0, a_cel_height as f32 * 0.5,
                        0.0, 0.0, 1.0,
                    );
                } else {
                    // 无图无文本：仅 fullscreen 轨道继续（对应 C++: strcasecmp(name, "fullscreen") == 0）
                    if !track_def.m_name.eq_ignore_ascii_case("fullscreen") {
                        continue;
                    }
                    a_full_screen = true;
                }

                // 对应 C++: aTransformMatrix = MatrixFromTransform(aTransform)
                // aMatrix = aMatrix × aTransformMatrix × mOverlayMatrix，再平移 shake + g 平移
                let a_transform_matrix = Self::matrix_from_transform(&a_transform);
                a_matrix = a_matrix.multiply(&a_transform_matrix);
                a_matrix = a_matrix.multiply(&self.m_overlay_matrix);
                // 对应 C++ SexyMatrix3Translation(aMatrix, mShakeX + g->mTransX, mShakeY + g->mTransY)
                a_matrix.m[0][2] += ti.m_shake_x + g.trans_x as f32;
                a_matrix.m[1][2] += ti.m_shake_y + g.trans_y as f32;
                // 应用 m_override_scale（对应 C++ OverrideScale 语义；m_overlay_matrix 已含或单独叠加）
                let a_scale_x = self.m_override_scale_x;
                let a_scale_y = self.m_override_scale_y;
                if (a_scale_x - 1.0).abs() > 0.001 || (a_scale_y - 1.0).abs() > 0.001 {
                    a_matrix.m[0][0] *= a_scale_x;
                    a_matrix.m[1][1] *= a_scale_y;
                }

                // 对应 C++: atlas 分支 — 从 atlas 内存图绘制源矩形
                if let Some(a_atlas_img) = a_atlas_image {
                    let a_src_rect = crate::framework::rect::Rect::new(
                        a_atlas_img.x, a_atlas_img.y, a_atlas_img.width, a_atlas_img.height);
                    let mut a_atlas_memory = (*atlas_ptr.unwrap()).memory_image;
                    if !a_atlas_memory.is_null() {
                        // 对应 C++: FilterEffectGetImage(aImage, mFilterEffect)
                        let a_base = &mut (*a_atlas_memory).base as *mut crate::framework::graphics::image::Image;
                        let a_effect_image = crate::todlib::filter_effect::filter_effect_get_image(
                            a_base, self.m_filter_effect);
                        g.set_color(&a_color);
                        g.draw_image_matrix_src(
                            unsafe { &*a_effect_image }, &a_matrix, &a_src_rect,
                            0.0, 0.0,
                        );
                        if self.m_enable_extra_additive_draw && !ti.m_ignore_extra_additive_color {
                            let a_old_mode = g.get_draw_mode();
                            g.set_draw_mode(2); // DRAWMODE_ADDITIVE
                            g.set_color(&a_extra_additive_color);
                            g.draw_image_matrix_src(
                                unsafe { &*a_effect_image }, &a_matrix, &a_src_rect,
                                0.0, 0.0,
                            );
                            g.set_draw_mode(a_old_mode);
                        }
                        if self.m_enable_extra_overlay_draw {
                            let a_white = crate::todlib::filter_effect::filter_effect_get_image(
                                a_base, crate::todlib::filter_effect::FilterEffectType::White);
                            g.set_color(&a_extra_overlay_color);
                            g.draw_image_matrix_src(
                                unsafe { &*a_white }, &a_matrix, &a_src_rect,
                                0.0, 0.0,
                            );
                        }
                        if restore_clip {
                            g.set_clip_rect(&a_old_clip_rect);
                        }
                        continue;
                    }
                }

                // 对应 C++: 普通图片分支
                if !a_image.is_null() {
                    let a_img = &*a_image;
                    if a_img.width <= 0 || a_img.height <= 0 {
                        if restore_clip {
                            g.set_clip_rect(&a_old_clip_rect);
                        }
                        continue;
                    }
                    // 对应 C++: while (aImageFrame >= aImage->mNumCols) aImageFrame -= mNumCols
                    let mut a_cel_frame = a_image_frame;
                    let a_num_cols = a_img.num_cols.max(1);
                    while a_cel_frame >= a_num_cols {
                        a_cel_frame -= a_num_cols;
                    }
                    let a_cel_width = a_img.get_cel_width();
                    let a_src_rect = crate::framework::rect::Rect::new(
                        a_cel_frame * a_cel_width, 0, a_cel_width, a_img.get_cel_height());
                    let a_effect_image = crate::todlib::filter_effect::filter_effect_get_image(
                        a_image, self.m_filter_effect);
                    g.set_color(&a_color);
                    g.draw_image_matrix_src(
                        unsafe { &*a_effect_image }, &a_matrix, &a_src_rect,
                        0.0, 0.0,
                    );
                    if self.m_enable_extra_additive_draw && !ti.m_ignore_extra_additive_color {
                        let a_old_mode = g.get_draw_mode();
                        g.set_draw_mode(2); // DRAWMODE_ADDITIVE
                        g.set_color(&a_extra_additive_color);
                        g.draw_image_matrix_src(
                            unsafe { &*a_effect_image }, &a_matrix, &a_src_rect,
                            0.0, 0.0,
                        );
                        g.set_draw_mode(a_old_mode);
                    }
                    if self.m_enable_extra_overlay_draw {
                        let a_white = crate::todlib::filter_effect::filter_effect_get_image(
                            a_image, crate::todlib::filter_effect::FilterEffectType::White);
                        g.set_color(&a_extra_overlay_color);
                        g.draw_image_matrix_src(
                            unsafe { &*a_white }, &a_matrix, &a_src_rect,
                            0.0, 0.0,
                        );
                    }
                    if restore_clip {
                        g.set_clip_rect(&a_old_clip_rect);
                    }
                    continue;
                }

                // 对应 C++: font/text 分支（PvzpDrawStringMatrix）
                if a_transform.m_font >= 0 {
                    // [TRANSLATION_NOTE]: 字体矩阵绘制（PvzpDrawStringMatrix）依赖字体表接入，
                    // 属第 5 项调用点接线范围；文本轨道暂以普通 draw_string 近似
                    if a_transform.m_text >= 0 {
                        let text_idx = a_transform.m_text as usize;
                        if let Some(a_text) = track_def.m_texts.get(text_idx) {
                            g.set_color(&a_color);
                            g.draw_string(a_text, a_matrix.m[0][2] as i32, a_matrix.m[1][2] as i32);
                        }
                    }
                    if restore_clip {
                        g.set_clip_rect(&a_old_clip_rect);
                    }
                    continue;
                }

                // 对应 C++: fullscreen 轨道 — 全屏填色
                if a_full_screen {
                    let a_old_color = *g.get_color();
                    g.set_color(&a_color);
                    g.fill_rect_xywh(
                        -g.trans_x as i32, -g.trans_y as i32,
                        crate::lawn::game_enums::BOARD_WIDTH, crate::lawn::game_enums::BOARD_HEIGHT);
                    g.set_color(&a_old_color);
                }
                if restore_clip {
                    g.set_clip_rect(&a_old_clip_rect);
                }
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
        // 对应 C++: 遍历轨道实例的附着动画并递归（借用规避：先收集 attachment id）
        let a_attachment_ids: Vec<crate::lawn::game_enums::AttachmentID> =
            self.m_track_instances.iter().map(|t| t.m_attachment_id).collect();
        for mut a_attachment_id in a_attachment_ids {
            if let Some(a_reanim) = crate::todlib::attachment::find_reanim_attachment(&mut a_attachment_id) {
                unsafe {
                    let a_reanim_ref = &mut *(a_reanim.cast::<Reanimation>());
                    if let Some(a_sub_reanim) = a_reanim_ref.find_sub_reanim(reanim_type) {
                        return Some(a_sub_reanim);
                    }
                }
            }
        }
        None
    }

    /// 获取当前帧的完整时间（对应 C++ 归一化 mAnimTime，0~1）
    pub fn get_frame_time(&self) -> f32 {
        self.m_anim_time
    }

    /// 动画是否已完全结束（对应 C++ EffectSystem::ProcessDeleteQueue 的 mDead 判据）
    pub fn is_completely_done(&self) -> bool {
        if self.m_dead {
            return true;
        }
        if let Some(def) = self.m_definition {
            unsafe {
                // 对应 C++ PlayOnce 类 loop 播完即 mDead；非循环类型在 m_anim_time 到 1.0 后
                // 不再推进，判定为结束
                if self.m_anim_rate > 0.0
                    && (self.m_loop_type == ReanimLoopType::PlayOnceAndRemove
                        || self.m_loop_type == ReanimLoopType::PlayOnceFullLastFrame)
                    && self.m_anim_time >= 1.0
                {
                    return true;
                }
                let _ = def;
            }
        }
        false
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

    /// 按类型初始化动画（对应 C++ ReanimationInitializeType + ReanimationInitialize）
    pub fn reanimation_initialize_type(&mut self, x: f32, y: f32, reanim_type: ReanimationType) {
        let def_ptr = crate::todlib::reanim_loader::reanimator_get_definition(reanim_type);
        self.reanim_type = reanim_type;
        if let Some(def_ptr) = def_ptr {
            unsafe {
                let def_ref = &mut *def_ptr;
                self.m_definition = Some(def_ptr);
                self.m_fps = def_ref.m_fps;
                self.m_anim_rate = def_ref.m_fps;
                // 对应 C++ ReanimationInitialize: ReanimationCreateAtlas(theDefinition, mReanimationType)
                // 仅在定义尚未创建图集时创建
                if def_ref.m_reanim_atlas.is_none() {
                    def_ref.m_reanim_atlas =
                        crate::todlib::reanim_atlas::ReanimAtlas::create_from_definition(def_ref);
                }
                let track_count = def_ref.m_tracks.len();
                self.m_track_instances = (0..track_count)
                    .map(|_| crate::todlib::definition::ReanimatorTrackInstance::new())
                    .collect();
                // 对应 C++ ReanimationInitialize（Reanimator.cpp:400-413）：
                // mFrameCount = tracks[0].mTransforms.count
                self.m_frame_count = def_ref
                    .m_tracks
                    .first()
                    .map_or(0, |t| t.m_transforms.len() as i32);
            }
        }
        self.m_x = x;
        self.m_y = y;
        self.m_anim_time = 0.0;
        self.m_last_anim_time = -1.0;
        self.m_loop_count = 0;
        self.m_dead = false;
    }

    /// 设置动画类型（从定义名称查找）（对应 C++ SetReanimType + ReanimationInitializeType）
    pub fn set_reanim(&mut self, x: f32, y: f32, reanim_type: ReanimationType) {
        self.reanimation_initialize_type(x, y, reanim_type);
    }

    /// 播放指定轨道（对应 C++ Reanimation::PlayReanim，Reanimator.cpp:1298）
    pub fn play_reanim(&mut self, track_name: &str, loop_type: ReanimLoopType, blend_time: i32, anim_rate: f32) {
        // 对应 C++: if (theBlendTime > 0) StartBlend(theBlendTime)
        if blend_time > 0 {
            self.start_blend(blend_time);
        }
        // 对应 C++: if (theAnimRate != 0) mAnimRate = theAnimRate
        if anim_rate != 0.0 {
            self.m_anim_rate = anim_rate;
        }
        self.m_loop_type = loop_type;
        self.m_loop_count = 0;
        // 对应 C++: SetFramesForLayer(theTrackName)
        self.set_frames_for_layer(track_name);
    }

    /// 设置帧层（对应 C++ SetFramesForLayer）
    pub fn set_frames_for_layer(&mut self, layer: &str) {
        // 对应 C++ SetFramesForLayer: 重置动画时间
        if self.m_anim_rate >= 0.0 {
            self.m_anim_time = 0.0;
        } else {
            self.m_anim_time = 0.9999999;
        }
        self.m_last_anim_time = -1.0;

        let (a_frame_start, a_frame_count) = self.get_frames_for_layer(layer);
        self.m_frame_start = a_frame_start;
        self.m_frame_count = a_frame_count;
    }

    /// 计算指定轨道的帧区间（对应 C++ GetFramesForLayer：从首个非空白帧到最后一个非空白帧）
    fn get_frames_for_layer(&self, track_name: &str) -> (i32, i32) {
        let def = match self.m_definition {
            Some(def) => def,
            None => return (0, 1),
        };
        unsafe {
            let def_ref = &*def;
            if def_ref.m_tracks.is_empty() {
                return (0, 0);
            }
            let a_track_index = self.find_track_index(track_name);
            let mut a_frame_start = 0;
            let mut a_frame_count = 1;
            if a_track_index >= 0 && (a_track_index as usize) < def_ref.m_tracks.len() {
                let a_track = &def_ref.m_tracks[a_track_index as usize];
                // 第一个非空白帧（mFrame >= 0）
                for i in 0..a_track.m_transforms.len() {
                    if a_track.m_transforms[i].m_frame >= 0.0 {
                        a_frame_start = i as i32;
                        break;
                    }
                }
                // 从起始帧到最后一个非空白帧的跨度
                for j in (a_frame_start as usize)..a_track.m_transforms.len() {
                    if a_track.m_transforms[j].m_frame >= 0.0 {
                        a_frame_count = j as i32 - a_frame_start + 1;
                    }
                }
            }
            (a_frame_start, a_frame_count)
        }
    }

    /// 从指定轨道设置基础姿态（对应 C++ SetBasePoseFromAnim）
    pub fn set_base_pose_from_anim(&mut self, track_name: &str) {
        let (a_frame_start, _) = self.get_frames_for_layer(track_name);
        self.m_frame_base_pose = a_frame_start;
    }

    /// 指定轨道动画是否正在播放（对应 C++ IsAnimPlaying）
    pub fn is_anim_playing(&self, track_name: &str) -> bool {
        let (a_frame_start, a_frame_count) = self.get_frames_for_layer(track_name);
        self.m_frame_start == a_frame_start && self.m_frame_count == a_frame_count
    }

    /// 仅显示指定轨道，其余隐藏（对应 C++ ShowOnlyTrack）
    pub fn show_only_track(&mut self, track_name: &str) {
        if let Some(def) = self.m_definition {
            unsafe {
                let def_ref = &*def;
                for (i, track_def) in def_ref.m_tracks.iter().enumerate() {
                    if let Some(ti) = self.m_track_instances.get_mut(i) {
                        ti.m_render_group = if track_def.m_name.eq_ignore_ascii_case(track_name) {
                            RENDER_GROUP_NORMAL
                        } else {
                            RENDER_GROUP_HIDDEN
                        };
                    }
                }
            }
        }
    }

    /// 轨道是否正在显示（对应 C++ IsTrackShowing，Reanimator.cpp:1241：
    /// 用 GetFrameTime 的 mAnimFrameAfterInt 判断下一帧是否非空白）
    pub fn is_track_showing(&self, track_name: &str) -> bool {
        let track_index = self.find_track_index(track_name);
        if track_index < 0 {
            return false;
        }
        let def = match self.m_definition {
            Some(def) => def,
            None => return false,
        };
        unsafe {
            let def_ref = &*def;
            if track_index as usize >= def_ref.m_tracks.len() {
                return false;
            }
            let a_track = &def_ref.m_tracks[track_index as usize];
            if a_track.m_transforms.is_empty() {
                return false;
            }
            // 对应 C++ GetFrameTime(&aFrameTime) 的 mAnimFrameAfterInt
            let a_frame_time = self.get_frame_time_frame();
            let idx = (a_frame_time.anim_frame_after_int.max(0) as usize)
                .min(a_track.m_transforms.len() - 1);
            a_track.m_transforms[idx].m_frame >= 0.0
        }
    }

    /// 开始混合（对应 C++ StartBlend：记录当前变换为混合源）
    pub fn start_blend(&mut self, blend_time: i32) {
        let def = match self.m_definition {
            Some(def) => def,
            None => return,
        };
        unsafe {
            let def_ref = &*def;
            let track_count = def_ref.m_tracks.len();
            for a_track_index in 0..track_count {
                let mut a_transform = ReanimatorTransform::default();
                if self.get_current_transform(a_track_index as i32, &mut a_transform) {
                    // 对应 C++: FloatRoundToInt(mFrame) >= 0（非空白帧才记录混合源）
                    if a_transform.m_frame.round() as i32 >= 0 {
                        if let Some(a_track_instance) = self.m_track_instances.get_mut(a_track_index) {
                            a_track_instance.m_blend_transform = a_transform;
                            a_track_instance.m_blend_time = blend_time;
                            a_track_instance.m_blend_count = blend_time as f32;
                            // 对应 C++: 清空 font/text/image
                            a_track_instance.m_blend_transform.m_font = -1;
                            a_track_instance.m_blend_transform.m_text = -1;
                            a_track_instance.m_blend_transform.m_image = -1;
                        }
                    }
                }
            }
        }
    }

    /// 混合两个变换（对应 C++ BlendTransform，Reanimator.cpp:518）
    /// 对 trans/scale/alpha 做 FloatLerp；skew 相差超过 180° 时忽略 theTransform2 的 skew
    fn blend_transform(
        the_result: &mut ReanimatorTransform,
        the_transform1: &ReanimatorTransform,
        the_transform2: &ReanimatorTransform,
        the_blend_factor: f32,
    ) {
        the_result.m_trans_x = crate::todlib::tod_common::lerp(the_transform1.m_trans_x, the_transform2.m_trans_x, the_blend_factor);
        the_result.m_trans_y = crate::todlib::tod_common::lerp(the_transform1.m_trans_y, the_transform2.m_trans_y, the_blend_factor);
        the_result.m_scale_x = crate::todlib::tod_common::lerp(the_transform1.m_scale_x, the_transform2.m_scale_x, the_blend_factor);
        the_result.m_scale_y = crate::todlib::tod_common::lerp(the_transform1.m_scale_y, the_transform2.m_scale_y, the_blend_factor);
        the_result.m_alpha = crate::todlib::tod_common::lerp(the_transform1.m_alpha, the_transform2.m_alpha, the_blend_factor);

        let mut a_skew_x2 = the_transform2.m_skew_x;
        let mut a_skew_y2 = the_transform2.m_skew_y;
        // C++: skew 相差超过 180° 时 theTransform2 的 skew 被忽略（源码实现为直接取 theTransform1 的 skew）
        while a_skew_x2 > the_transform1.m_skew_x + 180.0 {
            a_skew_x2 = the_transform1.m_skew_x;
        }
        while a_skew_x2 < the_transform1.m_skew_x - 180.0 {
            a_skew_x2 = the_transform1.m_skew_x;
        }
        while a_skew_y2 > the_transform1.m_skew_y + 180.0 {
            a_skew_y2 = the_transform1.m_skew_y;
        }
        while a_skew_y2 < the_transform1.m_skew_y - 180.0 {
            a_skew_y2 = the_transform1.m_skew_y;
        }
        the_result.m_skew_x = crate::todlib::tod_common::lerp(the_transform1.m_skew_x, a_skew_x2, the_blend_factor);
        the_result.m_skew_y = crate::todlib::tod_common::lerp(the_transform1.m_skew_y, a_skew_y2, the_blend_factor);
        the_result.m_frame = the_transform1.m_frame;
    }

    /// 给前缀分配渲染组（对应 C++ AssignRenderGroupToPrefix）
    pub fn assign_render_group_to_prefix(&mut self, track_prefix: &str, render_group: i32) {
        if let Some(def) = self.m_definition {
            unsafe {
                for (i, track_def) in (*def).m_tracks.iter().enumerate() {
                    // C++: strcasecmp 前缀匹配
                    if track_def.m_name.len() >= track_prefix.len()
                        && track_def.m_name[..track_prefix.len()].eq_ignore_ascii_case(track_prefix)
                    {
                        if let Some(ti) = self.m_track_instances.get_mut(i) {
                            ti.m_render_group = render_group;
                        }
                    }
                }
            }
        }
    }

    /// 给指定轨道分配渲染组（对应 C++ AssignRenderGroupToTrack，仅第一个精确匹配）
    pub fn assign_render_group_to_track(&mut self, track_name: &str, render_group: i32) {
        if let Some(def) = self.m_definition {
            unsafe {
                for (i, track_def) in (*def).m_tracks.iter().enumerate() {
                    if track_def.m_name.eq_ignore_ascii_case(track_name) {
                        if let Some(ti) = self.m_track_instances.get_mut(i) {
                            ti.m_render_group = render_group;
                        }
                        return;
                    }
                }
            }
        }
    }

    /// 获取轨道图片覆盖（对应 C++ GetImageOverride）
    pub fn get_image_override(&self, track_name: &str) -> *mut Image {
        for (name, img) in &self.m_image_overrides {
            if name.eq_ignore_ascii_case(track_name) {
                return *img;
            }
        }
        std::ptr::null_mut()
    }

    /// 设置轨道图片覆盖（对应 C++ SetImageOverride，nullptr 表示清除覆盖）
    pub fn set_image_override(&mut self, track_name: &str, image: *mut Image) {
        if image.is_null() {
            self.m_image_overrides.retain(|(n, _)| !n.eq_ignore_ascii_case(track_name));
            return;
        }
        for (name, img) in self.m_image_overrides.iter_mut() {
            if name.eq_ignore_ascii_case(track_name) {
                *img = image;
                return;
            }
        }
        self.m_image_overrides.push((track_name.to_string(), image));
    }

    /// 触发定时事件判定（对应 C++ ShouldTriggerTimedEvent，Reanimator.cpp:1288）
    /// 判断动画时间是否在本帧内越过 theEventTime（0~1 归一化时间）
    pub fn should_trigger_timed_event(&self, event_time: f32) -> bool {
        // 对应 C++: 无动画/反向播放/未播放 → false
        if self.m_frame_count == 0 || self.m_last_anim_time <= 0.0 || self.m_anim_rate <= 0.0 {
            return false;
        }
        let a_last = self.m_last_anim_time;
        let a_anim = self.m_anim_time;
        if a_anim >= a_last {
            // 对应 C++: 正常情况，触发区间 [mLastFrameTime, mAnimTime]
            return event_time >= a_last && event_time < a_anim;
        }
        // 对应 C++: 已回绕到下一循环，触发区间 [0, mAnimTime] ∪ [mLastFrameTime, 1]
        event_time >= a_last || event_time < a_anim
    }

    /// 销毁动画（对应 C++ Reanimation::ReanimationDie，Reanimator.cpp:1084）
    /// 置 mDead 并遍历轨道调用 AttachmentDie 清理附件
    pub fn reanimation_die(&mut self) {
        if !self.m_dead {
            self.m_dead = true;
            let attachment_ids: Vec<crate::lawn::game_enums::AttachmentID> =
                self.m_track_instances.iter().map(|t| t.m_attachment_id).collect();
            for mut a_attachment_id in attachment_ids {
                // 对应 C++: AttachmentDie(mTrackInstances[aTrackIndex].mAttachmentID)
                crate::todlib::attachment::attachment_die(&mut a_attachment_id);
            }
        }
    }

    /// 查找轨道索引（对应 C++ FindTrackIndex，找不到返回 -1）
    pub fn find_track_index(&self, track_name: &str) -> i32 {
        if let Some(def) = self.m_definition {
            unsafe {
                for (i, track) in (*def).m_tracks.iter().enumerate() {
                    if track.m_name.eq_ignore_ascii_case(track_name) {
                        return i as i32;
                    }
                }
            }
        }
        -1
    }

    /// 获取帧时间结构（对应 C++ Reanimation::GetFrameTime）
    /// 返回当前动画时间对应的前/后整数帧与插值分数
    pub fn get_frame_time_frame(&self) -> ReanimatorFrameTime {
        let mut a_frame_time = ReanimatorFrameTime::default();
        if let Some(def) = self.m_definition {
            unsafe {
                let def_ref = &*def;
                if def_ref.m_tracks.is_empty() {
                    return a_frame_time;
                }
                if self.m_frame_count <= 0 {
                    return a_frame_time;
                }
                // 对应 C++: 完整末帧类型不减少帧数；其余类型减一（mFrameCount - 1）
                // [TRANSLATION_NOTE]: C++ 的 REANIM_LOOP_FULL_LAST_FRAME 在 Rust ReanimLoopType 中不存在
                let a_frame_count = if self.m_loop_type == ReanimLoopType::PlayOnceFullLastFrame
                    || self.m_loop_type == ReanimLoopType::PlayOnceFullLastFrameAndHold
                {
                    self.m_frame_count
                } else {
                    self.m_frame_count - 1
                };
                let a_anim_position = self.m_frame_start as f32 + self.m_anim_time * a_frame_count as f32;
                let a_anim_frame_before = a_anim_position.floor();
                a_frame_time.fraction = a_anim_position - a_anim_frame_before;
                a_frame_time.anim_frame_before_int = a_anim_frame_before.round() as i32;
                // 对应 C++: 在最后一帧时 before/after 相同
                if a_frame_time.anim_frame_before_int >= self.m_frame_start + self.m_frame_count - 1 {
                    a_frame_time.anim_frame_before_int = self.m_frame_start + self.m_frame_count - 1;
                    a_frame_time.anim_frame_after_int = a_frame_time.anim_frame_before_int;
                } else {
                    a_frame_time.anim_frame_after_int = a_frame_time.anim_frame_before_int + 1;
                }
            }
        }
        a_frame_time
    }

    /// 获取轨道在指定帧时间的变换（对应 C++ GetTransformAtTime，前后帧插值）
    pub fn get_transform_at_time(
        &self,
        track_index: i32,
        out: &mut ReanimatorTransform,
        frame_time: &ReanimatorFrameTime,
    ) -> bool {
        let def = match self.m_definition {
            Some(def) => def,
            None => return false,
        };
        unsafe {
            let def_ref = &*def;
            if track_index < 0 || track_index as usize >= def_ref.m_tracks.len() {
                return false;
            }
            let a_track = &def_ref.m_tracks[track_index as usize];
            if a_track.m_transforms.is_empty() {
                return false;
            }
            let before_idx = (frame_time.anim_frame_before_int.max(0) as usize)
                .min(a_track.m_transforms.len() - 1);
            let after_idx = (frame_time.anim_frame_after_int.max(0) as usize)
                .min(a_track.m_transforms.len() - 1);
            let a_trans_before = a_track.m_transforms[before_idx];
            let a_trans_after = a_track.m_transforms[after_idx];
            let m_fraction = frame_time.fraction;

            // 对应 C++: FloatLerp 插值各项
            out.m_trans_x = crate::todlib::tod_common::lerp(a_trans_before.m_trans_x, a_trans_after.m_trans_x, m_fraction);
            out.m_trans_y = crate::todlib::tod_common::lerp(a_trans_before.m_trans_y, a_trans_after.m_trans_y, m_fraction);
            out.m_skew_x = crate::todlib::tod_common::lerp(a_trans_before.m_skew_x, a_trans_after.m_skew_x, m_fraction);
            out.m_skew_y = crate::todlib::tod_common::lerp(a_trans_before.m_skew_y, a_trans_after.m_skew_y, m_fraction);
            out.m_scale_x = crate::todlib::tod_common::lerp(a_trans_before.m_scale_x, a_trans_after.m_scale_x, m_fraction);
            out.m_scale_y = crate::todlib::tod_common::lerp(a_trans_before.m_scale_y, a_trans_after.m_scale_y, m_fraction);
            out.m_alpha = crate::todlib::tod_common::lerp(a_trans_before.m_alpha, a_trans_after.m_alpha, m_fraction);
            out.m_image = a_trans_before.m_image;
            out.m_font = a_trans_before.m_font;
            out.m_text = a_trans_before.m_text;

            // 对应 C++: 截断消失帧（过渡到空白帧时直接裁掉）
            let a_truncate = self.m_track_instances
                .get(track_index as usize)
                .map_or(false, |ti| ti.m_truncate_disappearing_frames);
            if a_trans_before.m_frame != -1.0
                && a_trans_after.m_frame == -1.0
                && m_fraction > 0.0
                && a_truncate
            {
                out.m_frame = -1.0;
            } else {
                out.m_frame = a_trans_before.m_frame;
            }
            true
        }
    }

    /// 获取当前变换（对应 C++ GetCurrentTransform，Reanimator.cpp:550：
    /// GetFrameTime → GetTransformAtTime 插值 → BlendTransform 混合）
    pub fn get_current_transform(&self, track_index: i32, out: &mut crate::todlib::definition::ReanimatorTransform) -> bool {
        if self.m_definition.is_none() {
            return false;
        }
        unsafe {
            let def_ref = &*self.m_definition.unwrap();
            if track_index < 0 || track_index as usize >= def_ref.m_tracks.len() {
                return false;
            }
            if def_ref.m_tracks[track_index as usize].m_transforms.is_empty() {
                return false;
            }
        }
        let a_frame_time = self.get_frame_time_frame();
        if !self.get_transform_at_time(track_index, out, &a_frame_time) {
            return false;
        }
        // C++: FloatRoundToInt(mFrame) >= 0 且 mBlendCounter > 0 → BlendTransform
        if out.m_frame.round() as i32 >= 0 {
            if let Some(a_track) = self.m_track_instances.get(track_index as usize) {
                if a_track.m_blend_count > 0.0 && a_track.m_blend_time > 0 {
                    let a_blend_factor = a_track.m_blend_count / a_track.m_blend_time as f32;
                    let a_blend_source = a_track.m_blend_transform;
                    let a_current = *out; // C++ BlendTransform 的 theTransform1 参数（同对象别名，Rust 用拷贝）
                    Self::blend_transform(out, &a_current, &a_blend_source, a_blend_factor);
                }
            }
        }
        true
    }

    /// 获取轨道速度（对应 C++ GetTrackVelocity，Reanimator.cpp:1229：
    /// 用 GetFrameTime 的 before/after 整数帧 x 位移 × SECONDS_PER_UPDATE × mAnimRate）
    pub fn get_track_velocity(&self, track_name: &str) -> f32 {
        const SECONDS_PER_UPDATE: f32 = 0.02; // C++ SECONDS_PER_UPDATE
        let track_index = self.find_track_index(track_name);
        if track_index < 0 { return 0.0; }
        if let Some(def) = self.m_definition {
            unsafe {
                let def_ref = &*def;
                if track_index as usize >= def_ref.m_tracks.len() { return 0.0; }
                let track = &def_ref.m_tracks[track_index as usize];
                if track.m_transforms.len() < 2 { return 0.0; }
                let a_frame_time = self.get_frame_time_frame();
                let after = (a_frame_time.anim_frame_after_int.max(0) as usize)
                    .min(track.m_transforms.len() - 1);
                let before = (a_frame_time.anim_frame_before_int.max(0) as usize)
                    .min(track.m_transforms.len() - 1);
                let a_dis = track.m_transforms[after].m_trans_x - track.m_transforms[before].m_trans_x;
                return a_dis * SECONDS_PER_UPDATE * self.m_anim_rate;
            }
        }
        0.0
    }

    /// 传播颜色到附着动画（对应 C++ PropogateColorToAttachments）
    pub fn propogate_color_to_attachments(&self) {
        let a_attachment_ids: Vec<crate::lawn::game_enums::AttachmentID> =
            self.m_track_instances.iter().map(|t| t.m_attachment_id).collect();
        for mut a_attachment_id in a_attachment_ids {
            crate::todlib::attachment::attachment_propogate_color(
                &mut a_attachment_id,
                &self.m_color_override,
                self.m_enable_extra_additive_draw,
                &self.m_extra_additive_color,
                self.m_enable_extra_overlay_draw,
                &self.m_extra_overlay_color,
            );
        }
    }

    /// 从变换构建 3x3 矩阵（对应 C++ Reanimation::MatrixFromTransform）
    pub fn matrix_from_transform(the_transform: &ReanimatorTransform) -> crate::framework::sexy_matrix::SexyMatrix3 {
        // 对应 C++: aSkewX = -DEG_TO_RAD(mSkewX) / aSkewY = -DEG_TO_RAD(mSkewY)
        let a_skew_x = -the_transform.m_skew_x.to_radians();
        let a_skew_y = -the_transform.m_skew_y.to_radians();
        crate::framework::sexy_matrix::SexyMatrix3::new_from_values(
            a_skew_x.cos() * the_transform.m_scale_x,  // m00
            a_skew_y.sin() * the_transform.m_scale_y,  // m01
            the_transform.m_trans_x,                   // m02
            -a_skew_x.sin() * the_transform.m_scale_x, // m10
            a_skew_y.cos() * the_transform.m_scale_y,  // m11
            the_transform.m_trans_y,                   // m12
            0.0,                                       // m20
            0.0,                                       // m21
            1.0,                                       // m22
        )
    }

    /// 设置轨道震动覆盖（对应 C++ SetShakeOverride）
    pub fn set_shake_override(&mut self, track_name: &str, shake_amount: f32) {
        if let Some(track_instance) = self.get_track_instance_by_name(track_name) {
            track_instance.m_shake_override = shake_amount;
        }
    }

    /// 设置截断消失帧（对应 C++ SetTruncateDisappearingFrames；track_name 为 None 时作用于全部轨道）
    pub fn set_truncate_disappearing_frames(&mut self, track_name: Option<&str>, value: bool) {
        match track_name {
            None => {
                for track_instance in self.m_track_instances.iter_mut() {
                    track_instance.m_truncate_disappearing_frames = value;
                }
            }
            Some(name) => {
                if let Some(track_instance) = self.get_track_instance_by_name(name) {
                    track_instance.m_truncate_disappearing_frames = value;
                }
            }
        }
    }

    /// 附着到另一动画（对应 C++ AttachToAnotherReanimation）
    pub fn attach_to_another_reanimation(&mut self, the_attach_reanim: &mut Reanimation, track_name: &str) {
        // 对应 C++: 目标动画无轨道则返回
        let a_track_count = the_attach_reanim
            .m_definition
            .map_or(0, |def| unsafe { (*def).m_tracks.len() });
        if a_track_count == 0 {
            return;
        }
        // 对应 C++: mFrameBasePose == -1 时使用当前动画的起始帧作为基础姿态
        if the_attach_reanim.m_frame_base_pose == -1 {
            the_attach_reanim.m_frame_base_pose = the_attach_reanim.m_frame_start;
        }
        if let Some(a_track_instance) = the_attach_reanim.get_track_instance_by_name(track_name) {
            // 对应 C++: AttachReanim(theTrackInstance->mAttachmentID, this, 0.0f, 0.0f)
            let mut a_attachment_id = a_track_instance.m_attachment_id;
            crate::todlib::attachment::attach_reanim(
                &mut a_attachment_id,
                self as *mut Reanimation as *mut std::ffi::c_void,
                0.0,
                0.0,
            );
            a_track_instance.m_attachment_id = a_attachment_id;
        }
    }

    /// 获取轨道基础姿态矩阵（对应 C++ GetTrackBasePoseMatrix）
    pub fn get_track_base_pose_matrix(&self, track_index: i32) -> crate::framework::sexy_matrix::SexyMatrix3 {
        // 对应 C++: mFrameBasePose == NO_BASE_POSE 时单位矩阵
        if self.m_frame_base_pose == NO_BASE_POSE {
            return crate::framework::sexy_matrix::SexyMatrix3::identity();
        }
        // 对应 C++: aBasePos = mFrameBasePose == -1 ? mFrameStart : mFrameBasePose
        let a_base_pos = if self.m_frame_base_pose == -1 {
            self.m_frame_start
        } else {
            self.m_frame_base_pose
        };
        let a_start_time = ReanimatorFrameTime {
            fraction: 0.0,
            anim_frame_before_int: a_base_pos,
            anim_frame_after_int: a_base_pos + 1,
        };
        let mut a_transform_start = ReanimatorTransform::default();
        if self.get_transform_at_time(track_index, &mut a_transform_start, &a_start_time) {
            Self::matrix_from_transform(&a_transform_start)
        } else {
            crate::framework::sexy_matrix::SexyMatrix3::identity()
        }
    }

    /// 获取轨道附件叠加矩阵（对应 C++ Reanimation::GetAttachmentOverlayMatrix，Reanimator.cpp:1007）
    /// 当前变换矩阵 × overlay × 基础姿态矩阵的逆
    pub fn get_attachment_overlay_matrix(&self, track_index: i32) -> crate::framework::sexy_matrix::SexyMatrix3 {
        let mut a_transform = ReanimatorTransform::default();
        self.get_current_transform(track_index, &mut a_transform);
        let a_transform_matrix = Self::matrix_from_transform(&a_transform);
        // 对应 C++: SexyMatrix3Multiply(aTransformMatrix, mOverlayMatrix, aTransformMatrix)
        let a_transform_matrix = a_transform_matrix.multiply(&self.m_overlay_matrix);
        let a_base_pose_matrix = self.get_track_base_pose_matrix(track_index);
        let a_base_pose_matrix_inv = a_base_pose_matrix.inverse();
        // 对应 C++: theOverlayMatrix = aTransformMatrix * aBasePoseMatrixInv
        a_transform_matrix.multiply(&a_base_pose_matrix_inv)
    }

    /// 获取当前轨道图片（对应 C++ GetCurrentTrackImage）
    pub fn get_current_track_image(&self, track_name: &str) -> *mut Image {
        let a_track_index = self.find_track_index(track_name);
        if a_track_index < 0 {
            return std::ptr::null_mut();
        }
        // 对应 C++: mImageOverride 优先
        if let Some(a_track_instance) = self.m_track_instances.get(a_track_index as usize) {
            if !a_track_instance.m_image_override.is_null() {
                return a_track_instance.m_image_override;
            }
        }
        let mut a_transform = ReanimatorTransform::default();
        if self.get_current_transform(a_track_index, &mut a_transform) {
            // 对应 C++: atlas 编码句柄不映射稳定源图指针 → 返回空
            let a_atlas = self
                .m_definition
                .map(|def| unsafe { (*def).m_reanim_atlas })
                .flatten();
            if let Some(atlas_ptr) = a_atlas {
                unsafe {
                    let a_atlas = &*atlas_ptr;
                    if a_transform.m_image > 0
                        && a_atlas.get_encoded_reanim_atlas(a_transform.m_image).is_some()
                    {
                        return std::ptr::null_mut();
                    }
                }
            }
            return crate::todlib::reanim_loader::reanimator_get_image(a_transform.m_image)
                .unwrap_or(std::ptr::null_mut());
        }
        std::ptr::null_mut()
    }

    /// 按名称获取轨道实例（对应 C++ GetTrackInstanceByName）
    pub fn get_track_instance_by_name(&mut self, track_name: &str) -> Option<&mut crate::todlib::definition::ReanimatorTrackInstance> {
        let idx = self.find_track_index(track_name);
        if idx < 0 { return None; }
        self.m_track_instances.get_mut(idx as usize)
    }

    /// 解析附着器轨道（对应 C++ ParseAttacherTrack，Reanimator.cpp:1311）
    /// attacher 轨道名格式：attacher__REANIMNAME__TRACKNAME[TAG1][TAG2]...
    /// m_text 为文本表索引（-1 无文本）；此处经定义表取文本后解析
    pub fn parse_attacher_track(&self, track_index: i32, out: &mut crate::todlib::attachment::AttacherInfo) {
        out.reanim_name = String::new();
        out.track_name = String::new();
        out.anim_rate = 12.0;
        out.loop_type = crate::lawn::game_enums::ReanimLoopType::Loop;
        let def = match self.m_definition {
            Some(d) => d,
            None => return,
        };
        unsafe {
            let def_ref = &*def;
            if track_index < 0 || track_index as usize >= def_ref.m_tracks.len() {
                return;
            }
            let a_track_def = &def_ref.m_tracks[track_index as usize];
            let mut a_transform = crate::todlib::definition::ReanimatorTransform::default();
            if !self.get_current_transform(track_index, &mut a_transform) {
                return;
            }
            // 对应 C++: if (mFrame == -1.0f) return（空白帧）
            if a_transform.m_frame == -1.0 {
                return;
            }
            // 从文本表取 mText 字符串
            let a_text = if a_transform.m_text >= 0 {
                a_track_def.m_texts.get(a_transform.m_text as usize).map(|s| s.as_str()).unwrap_or("")
            } else {
                ""
            };
            if a_text.is_empty() {
                return;
            }
            // 对应 C++: strstr(mText, "__") — 找 reanim 名前分隔符
            let a_reanim_name = a_text.find("__");
            let a_reanim_name = match a_reanim_name {
                Some(i) => i,
                None => return,
            };
            let a_rest = &a_text[a_reanim_name + 2..];
            // 对应 C++: strstr(aReanimName + 2, "[") 与 strstr(aReanimName + 2, "__")
            let a_tags = a_rest.find('[');
            let a_track_name = a_rest.find("__");
            if let (Some(a_tags), Some(a_track_name)) = (a_tags, a_track_name) {
                if a_tags < a_track_name {
                    return; // "__" 在 "[" 之后 → 无效
                }
            }
            if let Some(a_track_name) = a_track_name {
                // track name defined
                out.reanim_name = a_rest[..a_track_name].to_string();
                if let Some(a_tags) = a_tags {
                    out.track_name = a_rest[a_track_name + 2..a_tags].to_string();
                } else {
                    out.track_name = a_rest[a_track_name + 2..].to_string();
                }
            } else if let Some(a_tags) = a_tags {
                out.reanim_name = a_rest[..a_tags].to_string();
            } else {
                out.reanim_name = a_rest.to_string();
            }
            // 读取标签（对应 C++ while (aTags) 循环）
            let mut a_tag_abs: Option<usize> = a_tags;
            while let Some(a_tag_start) = a_tag_abs {
                let a_after = &a_rest[a_tag_start + 1..];
                let a_tag_end = match a_after.find(']') {
                    Some(e) => e,
                    None => break, // 无闭合 "]"
                };
                let a_code = &a_after[..a_tag_end];
                // 对应 C++: sscanf(aCode, "%f") 成功 → anim rate
                if let Ok(a_rate) = a_code.trim().parse::<f32>() {
                    out.anim_rate = a_rate;
                } else if a_code.eq_ignore_ascii_case("hold") {
                    out.loop_type = crate::lawn::game_enums::ReanimLoopType::PlayOnceAndHold;
                } else if a_code.eq_ignore_ascii_case("once") {
                    out.loop_type = crate::lawn::game_enums::ReanimLoopType::PlayOnce;
                }
                // 找下一个 "["
                let a_next = a_after[a_tag_end + 1..].find('[');
                a_tag_abs = a_next.map(|i| a_tag_start + 1 + a_tag_end + 1 + i);
            }
        }
    }

    /// 更新附着器轨道（对应 C++ UpdateAttacherTrack，Reanimator.cpp:1311）
    /// 按轨道文本解析出的 reanim 名查找类型并附着/同步动画
    pub fn update_attacher_track(&mut self, track_index: usize) {
        let mut a_attacher_info = crate::todlib::attachment::AttacherInfo::default();
        self.parse_attacher_track(track_index as i32, &mut a_attacher_info);

        // 对应 C++: 由文件名反查 ReanimationType（gReanimationParamArray 遍历）
        let mut a_reanimation_type: ReanimationType = ReanimationType::None;
        if !a_attacher_info.reanim_name.is_empty() {
            let a_reanim_file_name = format!("reanim/{}.reanim", a_attacher_info.reanim_name);
            let app = crate::lawn::lawn_app::LawnApp::instance();
            if let Some(app) = app {
                // 遍历全部 reanim 类型，找文件名匹配
                for t in 0..crate::lawn::game_enums::ReanimationType::NumReanims as i32 {
                    let t_enum = unsafe { std::mem::transmute::<i32, ReanimationType>(t) };
                    if let Some(path) = crate::todlib::reanim_loader::get_reanim_file_path(t_enum) {
                        if path.eq_ignore_ascii_case(&a_reanim_file_name) {
                            a_reanimation_type = t_enum;
                            break;
                        }
                    }
                }
            }
        }
        if a_reanimation_type == ReanimationType::None {
            // 对应 C++: 无匹配 → AttachmentDie + return
            let mut a_attachment_id = self.m_track_instances[track_index].m_attachment_id;
            crate::todlib::attachment::attachment_die(&mut a_attachment_id);
            self.m_track_instances[track_index].m_attachment_id = a_attachment_id;
            return;
        }

        // 对应 C++: FindReanimAttachment + 类型不匹配 → 重新分配
        let mut a_track_attachment_id = self.m_track_instances[track_index].m_attachment_id;
        let mut a_attach_reanim = crate::todlib::attachment::find_reanim_attachment(&mut a_track_attachment_id);
        let mut a_attach_reanim: Option<*mut Reanimation> =
            a_attach_reanim.map(|p| p as *mut Reanimation);
        let a_need_alloc = match a_attach_reanim {
            Some(r) => unsafe { (*r).reanim_type != a_reanimation_type },
            None => true,
        };
        if a_need_alloc {
            crate::todlib::attachment::attachment_die(&mut a_track_attachment_id);
            let a_new_reanim = crate::lawn::lawn_app::LawnApp::instance().and_then(|app| {
                let mut app = app;
                unsafe { (&mut *app).add_reanimation(0.0, 0.0, 0, a_reanimation_type as i32) }
            });
            if let Some(a_new_reanim) = a_new_reanim {
                unsafe {
                    let a_new = &mut *a_new_reanim;
                    a_new.m_loop_type = match a_attacher_info.loop_type {
                        crate::lawn::game_enums::ReanimLoopType::Loop => ReanimLoopType::Loop,
                        crate::lawn::game_enums::ReanimLoopType::LoopFullLastFrame => ReanimLoopType::LoopFullOffset,
                        crate::lawn::game_enums::ReanimLoopType::PlayOnce => ReanimLoopType::PlayOnceAndRemove,
                        crate::lawn::game_enums::ReanimLoopType::PlayOnceAndHold => ReanimLoopType::PlayOnceAndHold,
                        crate::lawn::game_enums::ReanimLoopType::PlayOnceFullLastFrame => ReanimLoopType::PlayOnceFullLastFrame,
                        crate::lawn::game_enums::ReanimLoopType::PlayOnceAndHoldFullLastFrame => ReanimLoopType::PlayOnceFullLastFrameAndHold,
                    };
                    a_new.m_anim_rate = a_attacher_info.anim_rate;
                    crate::todlib::attachment::attach_reanim(
                        &mut a_track_attachment_id,
                        a_new as *mut Reanimation as *mut std::ffi::c_void,
                        0.0,
                        0.0,
                    );
                }
                // 对应 C++: mFrameBasePose = NO_BASE_POSE（附着后无基础姿态帧）
                self.m_frame_base_pose = NO_BASE_POSE;
                a_attach_reanim = Some(a_new_reanim);
            }
        }
        self.m_track_instances[track_index].m_attachment_id = a_track_attachment_id;

        // 对应 C++: track name 非空时同步轨道播放
        if !a_attacher_info.track_name.is_empty() {
            if let Some(a_attach_reanim) = a_attach_reanim {
                unsafe {
                    let a_attach = &mut *a_attach_reanim;
                    let (a_anim_frame_start, a_anim_frame_count) = a_attach.get_frames_for_layer(&a_attacher_info.track_name);
                    if a_attach.m_frame_start != a_anim_frame_start || a_attach.m_frame_count != a_anim_frame_count {
                        a_attach.start_blend(20);
                        a_attach.set_frames_for_layer(&a_attacher_info.track_name);
                    }
                    a_attach.m_anim_rate = a_attacher_info.anim_rate;
                    a_attach.m_loop_type = match a_attacher_info.loop_type {
                        crate::lawn::game_enums::ReanimLoopType::Loop => ReanimLoopType::Loop,
                        crate::lawn::game_enums::ReanimLoopType::LoopFullLastFrame => ReanimLoopType::LoopFullOffset,
                        crate::lawn::game_enums::ReanimLoopType::PlayOnce => ReanimLoopType::PlayOnceAndRemove,
                        crate::lawn::game_enums::ReanimLoopType::PlayOnceAndHold => ReanimLoopType::PlayOnceAndHold,
                        crate::lawn::game_enums::ReanimLoopType::PlayOnceFullLastFrame => ReanimLoopType::PlayOnceFullLastFrame,
                        crate::lawn::game_enums::ReanimLoopType::PlayOnceAndHoldFullLastFrame => ReanimLoopType::PlayOnceFullLastFrameAndHold,
                    };
                }
            }
        }
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
        // 对应 C++ ReanimationHolder::InitializeHolder: mReanimations.DataArrayInitialize(1024U, "reanims")
        self.reanimations.initialize(1024, "reanims");
    }

    pub fn dispose_holder(&mut self) {
        // 对应 C++ ReanimationHolder::DisposeHolder
        self.reanimations.free_all();
        self.reanimations.dispose();
    }

    pub fn alloc_reanimation(
        &mut self,
        x: f32,
        y: f32,
        render_order: i32,
        reanim_type: ReanimationType,
    ) -> Option<*mut Reanimation> {
        // 对应 C++ ReanimationHolder::AllocReanimation
        let a_reanim_ptr = self.reanimations.alloc();
        unsafe {
            (*a_reanim_ptr).m_render_order = render_order;
            // [TRANSLATION_NOTE]: C++ 中 aReanim->mReanimationHolder = this（原数据数组回溯）；
            // Rust Reanimation 无 m_reanimation_holder 字段，此关联暂缺
            (*a_reanim_ptr).reanimation_initialize_type(x, y, reanim_type);
        }
        Some(a_reanim_ptr)
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
