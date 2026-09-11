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
            m_image_overrides: Vec::new(),
        }
    }

    /// 更新动画
    pub fn update(&mut self) {
        if self.m_paused { return; }
        self.m_last_anim_time = self.m_anim_time;
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

    /// 绘制全部轨道（对应 C++ Reanimation::Draw = DrawRenderGroup(NORMAL)）
    pub fn draw(&self, g: &mut Graphics) {
        self.draw_render_group(g, 0); // RENDER_GROUP_NORMAL
    }

    /// 按渲染组绘制（对应 C++ Reanimation::DrawRenderGroup）
    /// 只绘制 mRenderGroup == theRenderGroup 的轨道；附加加法颜色覆盖通过颜色近似
    pub fn draw_render_group(&self, g: &mut Graphics, the_render_group: i32) {
        let Some(def_ptr) = self.m_definition else { return };
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
                if !ti.m_last_visible {
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
                // C++ DrawTrack：blank frame（mFrame < 0）不绘制
                if a_transform.m_frame < 0.0 {
                    continue;
                }
                // C++ 矩阵链 MatrixFromTransform × mOverlayMatrix × 平移(shake + g 平移)
                // [TRANSLATION_NOTE]: Graphics 无矩阵绘制接口，overlay 矩阵仅应用平移分量（m[0][2]/m[1][2]）；
                // 其余分量（旋转/斜切/缩放叠加）需 PvzpBltMatrix 等价接口，暂未接入。
                let px = self.m_x + a_transform.m_trans_x + self.m_overlay_matrix.m[0][2] + ti.m_shake_x;
                let py = self.m_y + a_transform.m_trans_y + self.m_overlay_matrix.m[1][2] + ti.m_shake_y;

                // 真实图片绘制：从图片名索引取标准 PNG 并绘制
                // 对应 C++ Reanimation::DrawRenderGroup 的 SetImageTransform + DrawImage
                let scale_x = self.m_override_scale_x * a_transform.m_scale_x;
                let scale_y = self.m_override_scale_y * a_transform.m_scale_y;
                if let Some(img_ptr) = crate::todlib::reanim_loader::reanimator_get_image(a_transform.m_image) {
                    let img = unsafe { &*img_ptr };
                    if img.width > 0 && img.height > 0 {
                        // 设置透明度（通过颜色覆盖）
                        let alpha = (a_transform.m_alpha * 255.0).clamp(0.0, 255.0) as u8;
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
                let alpha = (a_transform.m_alpha * 255.0).clamp(0.0, 255.0) as u8;
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
        if self.m_dead {
            return true;
        }
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

    /// 轨道是否正在显示（对应 C++ IsTrackShowing：当前帧对应变换非空白）
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
            // 对应 C++ GetFrameTime 的 mAnimFrameAfterInt：当前动画时间对应的整数帧
            let total_time = if def_ref.m_fps > 0.0 { def_ref.m_fps } else { 1.0 };
            let frame_idx =
                ((self.get_frame_time() % total_time) / total_time * a_track.m_transforms.len() as f32) as usize;
            let idx = frame_idx.min(a_track.m_transforms.len() - 1);
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

    /// 触发定时事件判定（对应 C++ ShouldTriggerTimedEvent）
    /// 判断动画时间是否在本帧内越过 theEventTime（0~1 归一化时间）
    pub fn should_trigger_timed_event(&self, event_time: f32) -> bool {
        let _ = event_time;
        if self.m_loop_count == 0 && self.m_last_anim_time <= 0.0 {
            return false;
        }
        if self.m_anim_rate <= 0.0 {
            return false;
        }
        let total_time = if let Some(def) = self.m_definition {
            unsafe { (*def).m_fps }
        } else { 1.0 };
        if total_time <= 0.0 { return false; }
        let cur = self.m_anim_time % total_time / total_time;
        let last = self.m_last_anim_time % total_time / total_time;
        if cur >= last {
            return event_time >= last && event_time < cur;
        }
        event_time >= last || event_time < cur
    }

    /// 销毁动画（对应 C++ ReanimationDie）
    pub fn reanimation_die(&mut self) {
        self.m_dead = true;
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

    /// 获取轨道速度（对应 C++ GetTrackVelocity，基于相邻帧 x 位移 * 帧时长 * 速率）
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
                let total_time = if def_ref.m_fps > 0.0 { def_ref.m_fps } else { 1.0 };
                let frame_time = self.get_frame_time();
                let f = (frame_time % total_time) / total_time * track.m_transforms.len() as f32;
                let after = (f as usize).min(track.m_transforms.len() - 1);
                let before = if after == 0 { track.m_transforms.len() - 1 } else { after - 1 };
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
            // [TRANSLATION_NOTE]: C++ 中 aTransform.mImage 为 Image* 且 atlas 编码图会清空；
            // Rust ReanimatorTransform.m_image 为资源 ID（i32）且 mReanimAtlas 为 stub，
            // 无 ID→Image* 映射，此处返回空指针等效于 atlas 编码图清空分支
            let _a_image_id = a_transform.m_image;
        }
        std::ptr::null_mut()
    }

    /// 按名称获取轨道实例（对应 C++ GetTrackInstanceByName）
    pub fn get_track_instance_by_name(&mut self, track_name: &str) -> Option<&mut crate::todlib::definition::ReanimatorTrackInstance> {
        let idx = self.find_track_index(track_name);
        if idx < 0 { return None; }
        self.m_track_instances.get_mut(idx as usize)
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
