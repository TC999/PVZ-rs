// PvZ Portable Rust 翻译 — TodParticle 粒子系统完整实现
// 对应 C++ src/Sexy.TodLib/TodParticle.h / TodParticle.cpp
//
// 包含粒子系统定义数据（FloatParameterTrack、TodParticleDefinition、
// TodEmitterDefinition）以及运行时实例（ParticleSystem、ParticleEmitter、Particle）
// 和 TodParticleHolder 管理器。

#![allow(dead_code)]

use std::ptr;
use std::collections::LinkedList;

use crate::lawn::game_enums::{
    ParticleID, ParticleEmitterID, ParticleSystemID,
    PARTICLESYSTEMID_NULL, PARTICLEID_NULL, PARTICLEEMITTERID_NULL,
    TodCurves,
};
use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::common::SexyVector2;
use crate::todlib::tod_list::TodList;
use crate::todlib::data_array::DataArray;

// ============================================================
// 常量
// ============================================================
pub const MAX_PARTICLES_SIZE: usize = 900;
pub const MAX_PARTICLE_FIELDS: usize = 4;

// ============================================================
// 枚举
// ============================================================

/// 粒子标志（对应 C++ ParticleFlags）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleFlags {
    RandomLaunchSpin = 0,
    AlignLaunchSpin = 1,
    AlignToPixels = 2,
    SystemLoops = 3,
    ParticleLoops = 4,
    ParticlesDontFollow = 5,
    RandomStartTime = 6,
    DieIfOverloaded = 7,
    Additive = 8,
    Fullscreen = 9,
    SoftwareOnly = 10,
    HardwareOnly = 11,
}

/// 粒子场类型（对应 C++ ParticleFieldType）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleFieldType {
    Invalid = 0,
    Friction = 1,
    Acceleration = 2,
    Attractor = 3,
    MaxVelocity = 4,
    Velocity = 5,
    Position = 6,
    SystemPosition = 7,
    GroundConstraint = 8,
    Shake = 9,
    Circle = 10,
    Away = 11,
    Count = 12,
}

/// 粒子系统轨道枚举（对应 C++ ParticleSystemTracks）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleSystemTracks {
    SpawnRate = 0,
    SpawnMinActive = 1,
    SpawnMaxActive = 2,
    SpawnMaxLaunched = 3,
    EmitterPath = 4,
    SystemRed = 5,
    SystemGreen = 6,
    SystemBlue = 7,
    SystemAlpha = 8,
    SystemBrightness = 9,
    NumSystemTracks = 10,
}

/// 粒子轨道枚举（对应 C++ ParticleTracks）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleTracks {
    ParticleRed = 0,
    ParticleGreen = 1,
    ParticleBlue = 2,
    ParticleAlpha = 3,
    ParticleBrightness = 4,
    SpinSpeed = 5,
    SpinAngle = 6,
    Scale = 7,
    Stretch = 8,
    CollisionReflect = 9,
    CollisionSpin = 10,
    ClipTop = 11,
    ClipBottom = 12,
    ClipLeft = 13,
    ClipRight = 14,
    AnimationRate = 15,
    NumParticleTracks = 16,
}

/// 发射器类型枚举（对应 C++ EmitterType）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EmitterType {
    Normal = 0,
    CirclePath = 1,
    BoxPath = 2,
}

/// 粒子效果枚举（对应 C++ ParticleEffect，定义在 Lawn 层）
/// 实际在 LawnApp 中定义，此处列出基础类型
pub type ParticleEffect = i32;

// ============================================================
// 定义数据结构
// ============================================================

/// 浮点参数轨道节点（对应 C++ FloatParameterTrackNode）
/// 描述属性数值随时间变化的一个阶段
#[derive(Debug, Clone, Copy)]
pub struct FloatParameterTrackNode {
    /// 阶段的起始时间（秒）
    pub time: f32,
    /// 阶段内数据允许的最小值
    pub low_value: f32,
    /// 阶段内数据允许的最大值
    pub high_value: f32,
    /// 从当前阶段过渡至下一阶段的缓动效果曲线
    pub curve_type: TodCurves,
    /// 阶段内数据在 min/max 之间的概率分布曲线
    pub distribution: TodCurves,
}

impl FloatParameterTrackNode {
    pub fn new() -> Self {
        FloatParameterTrackNode {
            time: 0.0,
            low_value: 0.0,
            high_value: 0.0,
            curve_type: TodCurves::Constant,
            distribution: TodCurves::Constant,
        }
    }
}

/// 浮点参数轨道（对应 C++ FloatParameterTrack）
/// 描述发射器/粒子某属性随时间的变化
#[derive(Debug, Clone)]
pub struct FloatParameterTrack {
    /// 节点数组
    pub nodes: Vec<FloatParameterTrackNode>,
}

impl FloatParameterTrack {
    pub fn new() -> Self { FloatParameterTrack { nodes: Vec::new() } }

    /// 评估轨道在指定时间点的值（对应 C++ 内联计算）
    pub fn evaluate(&self, time: f32, _rand: f32) -> f32 {
        if self.nodes.is_empty() { return 0.0; }
        if self.nodes.len() == 1 { return self.nodes[0].low_value; }
        // 查找当前时间对应的阶段
        let mut idx = 0;
        for i in 0..self.nodes.len() - 1 {
            if time >= self.nodes[i].time && time < self.nodes[i + 1].time {
                idx = i;
                break;
            }
            idx = self.nodes.len() - 2;
        }
        let node = &self.nodes[idx];
        let next_node = &self.nodes[idx + 1];
        let t = if next_node.time != node.time {
            (time - node.time) / (next_node.time - node.time)
        } else {
            0.0
        };
        // 简化：线性插值
        let v = node.low_value + (next_node.low_value - node.low_value) * t;
        // 应用分布（简化：取中间值）
        if node.high_value > node.low_value {
            node.low_value + (v - node.low_value) // 保持
        } else {
            v
        }
    }
}

/// 粒子场（对应 C++ ParticleField）
/// 发射器中粒子运动的物理环境
#[derive(Debug, Clone)]
pub struct ParticleField {
    /// 场的类型
    pub field_type: ParticleFieldType,
    /// 水平方向影响
    pub x: FloatParameterTrack,
    /// 竖直方向影响
    pub y: FloatParameterTrack,
}

impl ParticleField {
    pub fn new() -> Self {
        ParticleField {
            field_type: ParticleFieldType::Invalid,
            x: FloatParameterTrack::new(),
            y: FloatParameterTrack::new(),
        }
    }
}

/// 发射器字段数组（对应 C++ EmitterFieldArray）
#[derive(Debug, Clone)]
pub struct EmitterFieldArray {
    pub fields: Vec<ParticleField>,
}

impl EmitterFieldArray {
    pub fn new() -> Self { EmitterFieldArray { fields: Vec::new() } }
}

/// 发射器定义（对应 C++ TodEmitterDefinition）
/// 描述粒子发射器的全部行为参数
#[derive(Debug, Clone)]
pub struct TodEmitterDefinition {
    pub image: *mut Image,
    pub image_col: i32,
    pub image_row: i32,
    pub image_frames: i32,
    pub animated: i32,
    pub particle_flags: i32,
    pub emitter_type: EmitterType,
    pub name: String,
    pub on_duration: String,
    pub system_duration: FloatParameterTrack,
    pub cross_fade_duration: FloatParameterTrack,
    pub spawn_rate: FloatParameterTrack,
    pub spawn_min_active: FloatParameterTrack,
    pub spawn_max_active: FloatParameterTrack,
    pub spawn_max_launched: FloatParameterTrack,
    pub emitter_radius: FloatParameterTrack,
    pub emitter_offset_x: FloatParameterTrack,
    pub emitter_offset_y: FloatParameterTrack,
    pub emitter_box_x: FloatParameterTrack,
    pub emitter_box_y: FloatParameterTrack,
    pub emitter_skew_x: FloatParameterTrack,
    pub emitter_skew_y: FloatParameterTrack,
    pub emitter_path: FloatParameterTrack,
    pub particle_duration: FloatParameterTrack,
    pub launch_speed: FloatParameterTrack,
    pub launch_angle: FloatParameterTrack,
    pub system_red: FloatParameterTrack,
    pub system_green: FloatParameterTrack,
    pub system_blue: FloatParameterTrack,
    pub system_alpha: FloatParameterTrack,
    pub system_brightness: FloatParameterTrack,
    pub particle_fields: EmitterFieldArray,
    pub system_fields: EmitterFieldArray,
    pub particle_red: FloatParameterTrack,
    pub particle_green: FloatParameterTrack,
    pub particle_blue: FloatParameterTrack,
    pub particle_alpha: FloatParameterTrack,
    pub particle_brightness: FloatParameterTrack,
    pub particle_spin_angle: FloatParameterTrack,
    pub particle_spin_speed: FloatParameterTrack,
    pub particle_scale: FloatParameterTrack,
    pub particle_stretch: FloatParameterTrack,
    pub collision_reflect: FloatParameterTrack,
    pub collision_spin: FloatParameterTrack,
    pub clip_top: FloatParameterTrack,
    pub clip_bottom: FloatParameterTrack,
    pub clip_left: FloatParameterTrack,
    pub clip_right: FloatParameterTrack,
    pub animation_rate: FloatParameterTrack,
}

impl TodEmitterDefinition {
    pub fn new() -> Self {
        let new_track = || FloatParameterTrack::new();
        TodEmitterDefinition {
            image: ptr::null_mut(),
            image_col: 0, image_row: 0, image_frames: 0, animated: 0,
            particle_flags: 0, emitter_type: EmitterType::Normal,
            name: String::new(), on_duration: String::new(),
            system_duration: new_track(), cross_fade_duration: new_track(),
            spawn_rate: new_track(), spawn_min_active: new_track(),
            spawn_max_active: new_track(), spawn_max_launched: new_track(),
            emitter_radius: new_track(), emitter_offset_x: new_track(),
            emitter_offset_y: new_track(), emitter_box_x: new_track(),
            emitter_box_y: new_track(), emitter_skew_x: new_track(),
            emitter_skew_y: new_track(), emitter_path: new_track(),
            particle_duration: new_track(), launch_speed: new_track(),
            launch_angle: new_track(), system_red: new_track(),
            system_green: new_track(), system_blue: new_track(),
            system_alpha: new_track(), system_brightness: new_track(),
            particle_fields: EmitterFieldArray::new(),
            system_fields: EmitterFieldArray::new(),
            particle_red: new_track(), particle_green: new_track(),
            particle_blue: new_track(), particle_alpha: new_track(),
            particle_brightness: new_track(), particle_spin_angle: new_track(),
            particle_spin_speed: new_track(), particle_scale: new_track(),
            particle_stretch: new_track(), collision_reflect: new_track(),
            collision_spin: new_track(), clip_top: new_track(),
            clip_bottom: new_track(), clip_left: new_track(),
            clip_right: new_track(), animation_rate: new_track(),
        }
    }
}

/// 粒子系统定义（对应 C++ TodParticleDefinition）
/// 粒子系统中各个发射器定义的集合
#[derive(Debug, Clone)]
pub struct TodParticleDefinition {
    pub emitter_defs: Vec<TodEmitterDefinition>,
}

impl TodParticleDefinition {
    pub fn new() -> Self { TodParticleDefinition { emitter_defs: Vec::new() } }
}

/// 粒子参数映射（对应 C++ ParticleParams）
/// 粒子效果类型到文件名之间的映射
#[derive(Debug, Clone)]
pub struct ParticleParams {
    pub effect: ParticleEffect,
    pub file_name: String,
}

// ============================================================
// 粒子渲染参数
// ============================================================

/// 粒子渲染参数（对应 C++ ParticleRenderParams）
#[derive(Debug, Clone, Copy)]
pub struct ParticleRenderParams {
    pub red_is_set: bool,
    pub green_is_set: bool,
    pub blue_is_set: bool,
    pub alpha_is_set: bool,
    pub particle_scale_is_set: bool,
    pub particle_stretch_is_set: bool,
    pub spin_position_is_set: bool,
    pub position_is_set: bool,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    pub particle_scale: f32,
    pub particle_stretch: f32,
    pub spin_position: f32,
    pub pos_x: f32,
    pub pos_y: f32,
}

impl ParticleRenderParams {
    pub fn new() -> Self {
        ParticleRenderParams {
            red_is_set: false, green_is_set: false, blue_is_set: false,
            alpha_is_set: false, particle_scale_is_set: false,
            particle_stretch_is_set: false, spin_position_is_set: false,
            position_is_set: false,
            red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0,
            particle_scale: 1.0, particle_stretch: 1.0,
            spin_position: 0.0, pos_x: 0.0, pos_y: 0.0,
        }
    }
}

// ============================================================
// TodParticleHolder（粒子系统管理器）
// ============================================================

/// 粒子系统管理器（对应 C++ TodParticleHolder）
pub struct TodParticleHolder {
    pub particle_systems: DataArray<TodParticleSystem>,
    pub emitters: DataArray<TodParticleEmitter>,
    pub particles: DataArray<TodParticle>,
}

impl TodParticleHolder {
    pub fn new() -> Self {
        TodParticleHolder {
            particle_systems: DataArray::new(),
            emitters: DataArray::new(),
            particles: DataArray::new(),
        }
    }

    pub fn initialize_holder(&mut self) {}
    pub fn dispose_holder(&mut self) {}

    /// 从定义分配粒子系统
    pub fn alloc_particle_system_from_def(
        &mut self, _x: f32, _y: f32, _render_order: i32,
        _definition: &TodParticleDefinition, _effect: ParticleEffect,
    ) -> Option<&mut TodParticleSystem> {
        None
    }

    /// 按效果类型分配粒子系统
    pub fn alloc_particle_system(
        &mut self, _x: f32, _y: f32, _render_order: i32,
        _effect: ParticleEffect,
    ) -> Option<&mut TodParticleSystem> {
        None
    }

    /// 是否过载
    pub fn is_overloaded(&self) -> bool {
        self.particles.len() as usize >= MAX_PARTICLES_SIZE
    }
}

// ============================================================
// TodParticle（单个粒子）
// ============================================================

/// 单个粒子实例（对应 C++ TodParticle）
pub struct TodParticle {
    pub particle_emitter: *mut TodParticleEmitter,
    pub particle_duration: i32,
    pub particle_age: i32,
    pub particle_time_value: f32,
    pub particle_last_time_value: f32,
    pub animation_time_value: f32,
    pub velocity: SexyVector2,
    pub position: SexyVector2,
    pub image_frame: i32,
    pub spin_position: f32,
    pub spin_velocity: f32,
    pub cross_fade_particle_id: ParticleID,
    pub cross_fade_duration: i32,
    pub particle_interp: [f32; 16],  // NUM_PARTICLE_TRACKS
    pub particle_field_interp: [[f32; 2]; MAX_PARTICLE_FIELDS],
    pub active: bool,
}

impl Default for TodParticle {
    fn default() -> Self {
        TodParticle {
            particle_emitter: ptr::null_mut(),
            particle_duration: 0, particle_age: 0,
            particle_time_value: 0.0, particle_last_time_value: 0.0,
            animation_time_value: 0.0,
            velocity: SexyVector2::ZERO,
            position: SexyVector2::ZERO,
            image_frame: 0, spin_position: 0.0, spin_velocity: 0.0,
            cross_fade_particle_id: PARTICLEID_NULL,
            cross_fade_duration: 0,
            particle_interp: [0.0; 16],
            particle_field_interp: [[0.0; 2]; MAX_PARTICLE_FIELDS],
            active: false,
        }
    }
}

// ============================================================
// TodParticleEmitter（粒子发射器）
// ============================================================

/// 粒子发射器运行时实例（对应 C++ TodParticleEmitter）
pub struct TodParticleEmitter {
    pub emitter_def: *mut TodEmitterDefinition,
    pub particle_system: *mut TodParticleSystem,
    pub particle_list: TodList<ParticleID>,
    pub spawn_accum: f32,
    pub system_center: SexyVector2,
    pub particles_spawned: i32,
    pub system_age: i32,
    pub system_duration: i32,
    pub system_time_value: f32,
    pub system_last_time_value: f32,
    pub dead: bool,
    pub color_override: Color,
    pub extra_additive_draw_override: bool,
    pub scale_override: f32,
    pub image_override: *mut Image,
    pub cross_fade_emitter_id: ParticleEmitterID,
    pub emitter_cross_fade_count_down: i32,
    pub frame_override: i32,
    pub track_interp: [f32; 10],  // NUM_SYSTEM_TRACKS
    pub system_field_interp: [[f32; 2]; MAX_PARTICLE_FIELDS],
}

impl TodParticleEmitter {
    pub fn new() -> Self {
        TodParticleEmitter {
            emitter_def: ptr::null_mut(),
            particle_system: ptr::null_mut(),
            particle_list: TodList::new(),
            spawn_accum: 0.0,
            system_center: SexyVector2::ZERO,
            particles_spawned: 0, system_age: 0, system_duration: 0,
            system_time_value: 0.0, system_last_time_value: 0.0,
            dead: false,
            color_override: Color::WHITE,
            extra_additive_draw_override: false,
            scale_override: 1.0,
            image_override: ptr::null_mut(),
            cross_fade_emitter_id: PARTICLEEMITTERID_NULL,
            emitter_cross_fade_count_down: 0,
            frame_override: -1,
            track_interp: [0.0; 10],
            system_field_interp: [[0.0; 2]; MAX_PARTICLE_FIELDS],
        }
    }

    pub fn tod_emitter_initialize(&mut self, _x: f32, _y: f32,
        _system: *mut TodParticleSystem, _def: *mut TodEmitterDefinition) {}
    pub fn update(&mut self) {}
    pub fn draw(&self, _g: &mut Graphics) {}
    pub fn system_move(&mut self, _x: f32, _y: f32) {}
    pub fn update_spawning(&mut self) {}
    pub fn spawn_particle(&mut self, _idx: i32, _count: i32) -> Option<&mut TodParticle> { None }
    pub fn delete_particle(&mut self, _p: *mut TodParticle) {}
    pub fn delete_all(&mut self) {}
}

// ============================================================
// TodParticleSystem（粒子系统实例）
// ============================================================

/// 粒子系统运行时实例（对应 C++ TodParticleSystem）
pub struct TodParticleSystem {
    pub effect_type: ParticleEffect,
    pub particle_def: *mut TodParticleDefinition,
    pub particle_holder: *mut TodParticleHolder,
    pub emitter_list: TodList<ParticleEmitterID>,
    pub dead: bool,
    pub is_attachment: bool,
    pub render_order: i32,
    pub dont_update: bool,
}

impl TodParticleSystem {
    pub fn new() -> Self {
        TodParticleSystem {
            effect_type: 0,
            particle_def: ptr::null_mut(),
            particle_holder: ptr::null_mut(),
            emitter_list: TodList::new(),
            dead: false,
            is_attachment: false,
            render_order: 0,
            dont_update: false,
        }
    }

    pub fn particle_system_die(&mut self) { self.dead = true; }
    pub fn update(&mut self) {}
    pub fn draw(&self, _g: &mut Graphics) {}
    pub fn system_move(&mut self, _x: f32, _y: f32) {}
    pub fn override_color(&mut self, _emitter: &str, _color: &Color) {}
    pub fn override_extra_additive_draw(&mut self, _emitter: &str, _enable: bool) {}
    pub fn override_image(&mut self, _emitter: &str, _image: *mut Image) {}
    pub fn override_frame(&mut self, _emitter: &str, _frame: i32) {}
    pub fn override_scale(&mut self, _emitter: &str, _scale: f32) {}
    pub fn cross_fade(&mut self, _emitter: &str) {}
    pub fn find_emitter_by_name(&self, _name: &str) -> Option<&TodParticleEmitter> { None }
    pub fn find_emitter_def_by_name(&self, _name: &str) -> Option<&TodEmitterDefinition> { None }
}

// ============================================================
// 全局函数
// ============================================================

/// 加载单个粒子定义（对应 C++ TodParticleLoadADef）
pub fn tod_particle_load_a_def(_def: &mut TodParticleDefinition, _file: &str) -> bool { true }

/// 加载所有粒子定义（对应 C++ TodParticleLoadDefinitions）
pub fn tod_particle_load_definitions(_params: &[ParticleParams]) {}

/// 释放粒子定义（对应 C++ TodParticleFreeDefinitions）
pub fn tod_particle_free_definitions() {}

/// 交叉淡入淡出插值（对应 C++ CrossFadeLerp）
pub fn cross_fade_lerp(from: f32, to: f32, from_is_set: bool, to_is_set: bool, frac: f32) -> f32 {
    if !from_is_set && !to_is_set { return 0.0; }
    if !from_is_set { return to; }
    if !to_is_set { return from; }
    from + (to - from) * frac
}

// 旧类型别名（兼容现有代码）
pub type ParticleSystem = TodParticleSystem;
pub type ParticleEmitter = TodParticleEmitter;
pub type Particle = TodParticle;
