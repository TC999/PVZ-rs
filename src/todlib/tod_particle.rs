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
use crate::framework::common::{SexyVector2, RandFloat, RandRange};
use crate::todlib::tod_list::TodList;
use crate::todlib::data_array::DataArray;

// ============================================================
// 常量
// ============================================================
pub const MAX_PARTICLES_SIZE: usize = 900;

// 全局粒子定义数组（对应 C++ gParticleDefArray/gParticleDefCount，PvzpParticle.cpp）
static mut G_PARTICLE_DEF_ARRAY: Vec<TodParticleDefinition> = Vec::new();

/// 获取粒子定义（对应 C++ gParticleDefArray[theParticleEffect]）
pub fn particle_get_definition(effect: ParticleEffect) -> Option<*mut TodParticleDefinition> {
    unsafe {
        if effect < 0 || effect as usize >= G_PARTICLE_DEF_ARRAY.len() {
            return None;
        }
        Some(&mut G_PARTICLE_DEF_ARRAY[effect as usize] as *mut TodParticleDefinition)
    }
}

/// 全局粒子定义数量（对应 C++ gParticleDefCount）
pub fn particle_def_count() -> usize {
    unsafe { G_PARTICLE_DEF_ARRAY.len() }
}
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

/// 发射器类型枚举（对应 C++ EmitterType，ConstEnums.h）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EmitterType {
    Circle = 0,             // EMITTER_CIRCLE
    Box = 1,                // EMITTER_BOX
    BoxPath = 2,            // EMITTER_BOX_PATH
    CirclePath = 3,         // EMITTER_CIRCLE_PATH
    CircleEvenSpacing = 4,  // EMITTER_CIRCLE_EVEN_SPACING
}

/// 缩放旋转矩阵（对应 C++ PvzpLib/PvzpCommon.cpp PvzpScaleRotateTransformMatrix，同 zombie.rs/projectile.rs 副本）
fn pvzp_scale_rotate_transform_matrix(m: &mut crate::framework::sexy_matrix::SexyMatrix3, x: f32, y: f32, rad: f32, the_scale_x: f32, the_scale_y: f32) {
    m.m[0][0] = rad.cos() * the_scale_x;
    m.m[1][0] = -rad.sin() * the_scale_x;
    m.m[2][0] = 0.0;
    m.m[0][1] = rad.sin() * the_scale_y;
    m.m[1][1] = rad.cos() * the_scale_y;
    m.m[2][1] = 0.0;
    m.m[0][2] = x;
    m.m[1][2] = y;
    m.m[2][2] = 1.0;
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

    /// 设置默认值（对应 C++ FloatTrackSetDefault：轨道未定义时赋单节点默认值）
    pub fn set_default(&mut self, value: f32) {
        if self.nodes.is_empty() {
            self.nodes.push(FloatParameterTrackNode {
                time: 0.0,
                low_value: value,
                high_value: value,
                curve_type: TodCurves::Constant,
                distribution: TodCurves::Constant,
            });
        }
    }

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
            particle_flags: 0, emitter_type: EmitterType::Circle,
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

    pub fn initialize_holder(&mut self) {
        // 对应 C++ PvzpParticleHolder::InitializeHolder（PvzpParticle.cpp:1235）
        self.particle_systems.initialize(1024, "particle systems");
        self.emitters.initialize(1024, "emitters");
        self.particles.initialize(1024, "particles");
        // [TRANSLATION_NOTE]: C++ 中 mParticleListNodeAllocator/mEmitterListNodeAllocator
        // 节点池由 TodList 的 owned 节点所有权机制替代
    }

    pub fn dispose_holder(&mut self) {
        // 对应 C++ PvzpParticleHolder::DisposeHolder（:1244）
        self.particle_systems.dispose();
        self.emitters.dispose();
        self.particles.dispose();
    }

    /// 从定义分配粒子系统（对应 C++ AllocParticleSystemFromDef，:1258）
    pub fn alloc_particle_system_from_def(
        &mut self, x: f32, y: f32, render_order: i32,
        definition: &TodParticleDefinition, effect: ParticleEffect,
    ) -> Option<&mut TodParticleSystem> {
        // 对应 C++: 系统数组满
        if self.particle_systems.len() as u32 >= self.particle_systems.max_size() {
            return None;
        }
        // 对应 C++: 定义中的发射器数量 + 现有发射器数超过上限
        if definition.emitter_defs.len() as u32 + self.emitters.len() as u32 > self.emitters.max_size() {
            return None;
        }

        let a_system = unsafe { &mut *self.particle_systems.alloc() };
        a_system.particle_holder = self as *mut TodParticleHolder;
        // [TRANSLATION_NOTE]: Rust 侧 particle effect 参数以 i32 传递，转换为 game_enums 枚举（repr(i32)）
        let a_effect: crate::lawn::game_enums::ParticleEffect = unsafe { std::mem::transmute(effect) };
        a_system.pvzp_particle_initialize_from_def(
            x,
            y,
            render_order,
            definition as *const TodParticleDefinition as *mut TodParticleDefinition,
            a_effect,
        );
        Some(a_system)
    }

    /// 按效果类型分配粒子系统（对应 C++ AllocParticleSystem，:1277）
    pub fn alloc_particle_system(
        &mut self, x: f32, y: f32, render_order: i32,
        effect: ParticleEffect,
    ) -> Option<&mut TodParticleSystem> {
        let a_def = particle_get_definition(effect)?;
        unsafe {
            self.alloc_particle_system_from_def(x, y, render_order, &*a_def, effect)
        }
    }

    /// 是否过载（对应 C++ IsOverLoaded，:1253：任一数组超过上限）
    pub fn is_overloaded(&self) -> bool {
        self.particle_systems.len() as usize > MAX_PARTICLES_SIZE
            || self.emitters.len() as usize > MAX_PARTICLES_SIZE
            || self.particles.len() as usize > MAX_PARTICLES_SIZE
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

impl Default for TodParticleEmitter {
    fn default() -> Self {
        TodParticleEmitter::new()
    }
}

impl Default for TodParticleSystem {
    fn default() -> Self {
        TodParticleSystem::new()
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

    pub fn tod_emitter_initialize(&mut self, x: f32, y: f32,
        system: *mut TodParticleSystem, def: *mut TodEmitterDefinition) {
        // 对应 C++ PvzpEmitterInitialize（PvzpParticle.cpp:282）
        self.spawn_accum = 0.0;
        self.particles_spawned = 0;
        self.system_time_value = -1.0;
        self.system_last_time_value = -1.0;
        self.system_age = -1;
        self.dead = false;
        self.color_override = Color::WHITE;
        self.system_center.x = x;
        self.system_center.y = y;
        self.frame_override = -1;
        self.particle_system = system;
        self.scale_override = 1.0;
        self.extra_additive_draw_override = false;
        self.image_override = ptr::null_mut();
        self.system_duration = 0;
        self.emitter_def = def;

        unsafe {
            let def_ref = &*self.emitter_def;
            // 对应 C++: mSystemDuration = FloatTrackIsSet(mSystemDuration) ?
            //     FloatTrackEvaluate(mSystemDuration, 0, Rand(1)) : FloatTrackEvaluate(mParticleDuration, 0, 1)
            if !def_ref.system_duration.nodes.is_empty() {
                self.system_duration = def_ref.system_duration.evaluate(0.0, RandFloat(1.0)) as i32;
            } else {
                self.system_duration = def_ref.particle_duration.evaluate(0.0, 1.0) as i32;
            }
            self.system_duration = self.system_duration.max(1);

            // 对应 C++: system field interp 随机初始化
            let a_num_fields = def_ref.system_fields.fields.len().min(MAX_PARTICLE_FIELDS);
            for i in 0..a_num_fields {
                self.system_field_interp[i][0] = RandFloat(1.0);
                self.system_field_interp[i][1] = RandFloat(1.0);
            }
            for j in 0..10 {
                self.track_interp[j] = RandFloat(1.0);
            }
        }

        // 对应 C++: 初始化末尾调用 Update()
        self.update();
    }
    /// 更新发射器（对应 C++ PvzpParticleEmitter::Update，PvzpParticle.cpp:784）
    pub fn update(&mut self) {
        if self.dead { return; }

        self.system_age += 1;
        let mut a_die = false;
        // 对应 C++: 到达系统寿命
        {
            let def = unsafe { &*self.emitter_def };
            if self.system_age >= self.system_duration {
                if crate::lawn::zombie::test_bit(def.particle_flags as u32, ParticleFlags::SystemLoops as u32) {
                    self.system_age = 0;
                } else {
                    self.system_age = self.system_duration - 1;
                    a_die = true;
                }
            }
        }
        // 对应 C++: 淡出倒计时结束则死亡
        if self.emitter_cross_fade_count_down > 0 {
            self.emitter_cross_fade_count_down -= 1;
            if self.emitter_cross_fade_count_down == 0 {
                a_die = true;
            }
        }
        // 对应 C++: 淡出目标发射器丢失或已死亡
        if self.cross_fade_emitter_id != PARTICLEEMITTERID_NULL {
            let a_cross_fade_alive = unsafe {
                if self.particle_system.is_null() {
                    false
                } else {
                    let holder = (*self.particle_system).particle_holder;
                    if holder.is_null() {
                        false
                    } else {
                        (*holder).emitters.try_to_get(self.cross_fade_emitter_id).map_or(false, |e| !e.dead)
                    }
                }
            };
            if !a_cross_fade_alive {
                a_die = true;
            }
        }

        self.system_time_value = self.system_age as f32 / (self.system_duration - 1) as f32;
        // 对应 C++: 系统字段更新
        {
            let def = unsafe { &*self.emitter_def };
            let a_num_system_fields = def.system_fields.fields.len().min(MAX_PARTICLE_FIELDS);
            for i in 0..a_num_system_fields {
                let a_field = &def.system_fields.fields[i];
                self.update_system_field(a_field, self.system_time_value, i as i32);
            }
        }

        // 对应 C++: 粒子逐个更新，寿命结束或淡出源消失则删除
        let a_particle_ids: Vec<ParticleID> = unsafe {
            let mut ids = Vec::new();
            let mut node = self.particle_list.head;
            while !node.is_null() {
                ids.push((*node).value);
                node = (*node).next;
            }
            ids
        };
        for a_particle_id in a_particle_ids {
            let a_particle_ptr = unsafe {
                if self.particle_system.is_null() {
                    std::ptr::null_mut()
                } else {
                    let holder = (*self.particle_system).particle_holder;
                    if holder.is_null() {
                        std::ptr::null_mut()
                    } else {
                        match (*holder).particles.try_to_get_mut(a_particle_id) {
                            Some(p) => p as *mut TodParticle,
                            None => std::ptr::null_mut(),
                        }
                    }
                }
            };
            if a_particle_ptr.is_null() { continue; }
            let a_keep = { self.update_particle(unsafe { &mut *a_particle_ptr }) };
            if !a_keep {
                self.delete_particle(a_particle_ptr);
            }
        }

        self.update_spawning();

        if a_die {
            self.delete_non_cross_fading();
            if self.particle_list.count == 0 {
                self.dead = true;
                return;
            }
        }
        self.system_last_time_value = self.system_time_value;
    }
    /// 绘制粒子（对应 C++ PvzpParticleEmitter::Draw，PvzpParticle.cpp:1060）
    /// [TRANSLATION_NOTE]: C++ 按 mEmitterDef 的 SOFTWARE_ONLY/HARDWARE_ONLY 位做 3D 加速过滤；
    /// Rust 无 3D 加速检测，过滤跳过（TRANSLATION_NOTE）
    pub fn draw(&self, g: &mut Graphics) {
        let mut node = self.particle_list.head;
        while !node.is_null() {
            unsafe {
                let a_particle_id = (*node).value;
                let holder = (*self.particle_system).particle_holder;
                let a_particle_ptr = if holder.is_null() {
                    std::ptr::null_mut()
                } else {
                    match (*holder).particles.try_to_get_mut(a_particle_id) {
                        Some(p) => p as *mut TodParticle,
                        None => std::ptr::null_mut(),
                    }
                };
                if !a_particle_ptr.is_null() {
                    self.draw_particle(g, &mut *a_particle_ptr);
                }
                node = (*node).next;
            }
        }
    }

    /// 绘制单个粒子（对应 C++ PvzpParticleEmitter::DrawParticle，PvzpParticle.cpp:1024）
    /// [TRANSLATION_NOTE]: C++ 经 PvzpTriangleGroup 批处理（AddTriangle + DrawGroup）；
    /// Rust 逐粒子直接绘制（绘制顺序一致，无批处理）
    fn draw_particle(&self, g: &mut Graphics, the_particle: &mut TodParticle) {
        if the_particle.cross_fade_duration > 0 {
            return; // C++: cross-fade 源粒子不绘制
        }

        let mut a_params = ParticleRenderParams::new();
        if self.get_render_params(the_particle, &mut a_params) {
            let a_color = Color::new(
                (a_params.red.round() as i32).clamp(0, 255) as u8,
                (a_params.green.round() as i32).clamp(0, 255) as u8,
                (a_params.blue.round() as i32).clamp(0, 255) as u8,
                (a_params.alpha.round() as i32).clamp(0, 255) as u8,
            );
            if a_color.a > 0 {
                a_params.pos_x += g.trans_x as f32;
                a_params.pos_y += g.trans_y as f32;
                // [TRANSLATION_NOTE]: C++ 无图时尝试 cross-fade 源粒子渲染；
                // Rust 渲染以 emitter_def.image/image_override 为准，render_particle 内无图即返回
                self.render_particle(g, the_particle, a_color, &a_params);
            }
        }
    }

        /// 渲染单个粒子（对应 C++ RenderParticle，PvzpParticle.cpp:945）
    /// [TRANSLATION_NOTE]: C++ 经 PvzpTriangleGroup::AddTriangle 批处理；Rust 直接经
    /// Graphics 矩阵绘制（旋转/缩放一致），FULLSCREEN 全屏填充保留
    fn render_particle(&self, g: &mut Graphics, the_particle: &TodParticle, the_color: Color, the_params: &ParticleRenderParams) {
        let a_emitter_def = unsafe { &*self.emitter_def };
        let a_image = if !self.image_override.is_null() { self.image_override } else { a_emitter_def.image };
        if a_image.is_null() {
            return;
        }
        let a_image_ref = unsafe { &*a_image };

        let a_cel_width = a_image_ref.get_cel_width();
        let a_cel_height = a_image_ref.get_cel_height();
        let mut a_frame = self.frame_override;
        if a_frame == -1 {
            if !a_emitter_def.animation_rate.nodes.is_empty() {
                a_frame = ((the_particle.animation_time_value * a_emitter_def.image_frames as f32) as i32)
                    .clamp(0, a_emitter_def.image_frames - 1);
            } else if a_emitter_def.animated != 0 {
                a_frame = ((the_particle.particle_time_value * a_emitter_def.image_frames as f32) as i32)
                    .clamp(0, a_emitter_def.image_frames - 1);
            } else {
                a_frame = the_particle.image_frame;
            }
        }
        a_frame += a_emitter_def.image_col;
        if a_frame >= a_image_ref.num_cols {
            a_frame = a_image_ref.num_cols - 1;
        }

        let a_clip_top = self.particle_track_evaluate(&a_emitter_def.clip_top, the_particle, ParticleTracks::ClipTop);
        let a_clip_bottom = self.particle_track_evaluate(&a_emitter_def.clip_bottom, the_particle, ParticleTracks::ClipBottom);
        let a_clip_left = self.particle_track_evaluate(&a_emitter_def.clip_left, the_particle, ParticleTracks::ClipLeft);
        let a_clip_right = self.particle_track_evaluate(&a_emitter_def.clip_right, the_particle, ParticleTracks::ClipRight);

        let mut a_pos_x = the_params.pos_x + a_clip_left * a_cel_width as f32;
        let mut a_pos_y = the_params.pos_y + a_clip_top * a_cel_height as f32;
        let mut a_src_rect = Rect::new(
            (a_frame * a_cel_width + (a_clip_left * a_cel_width as f32).round() as i32)
                .max(0),
            (a_emitter_def.image_row.min(a_image_ref.num_rows - 1) * a_cel_height + (a_clip_top * a_cel_height as f32).round() as i32)
                .max(0),
            a_cel_width - (a_cel_width as f32 * (a_clip_left + a_clip_right)).round() as i32,
            a_cel_height - (a_cel_height as f32 * (a_clip_bottom + a_clip_top)).round() as i32,
        );
        if a_src_rect.width <= 0 || a_src_rect.height <= 0 {
            return;
        }

        // C++: PARTICLE_ALIGN_TO_PIXELS
        if crate::lawn::zombie::test_bit(a_emitter_def.particle_flags as u32, ParticleFlags::AlignToPixels as u32) {
            a_pos_x = a_pos_x.round();
            a_pos_y = a_pos_y.round();
        }
        let mut a_draw_mode = g.draw_mode;
        if crate::lawn::zombie::test_bit(a_emitter_def.particle_flags as u32, ParticleFlags::Additive as u32) {
            a_draw_mode = crate::framework::graphics::graphics::DrawMode::Additive as i32;
        }
        if crate::lawn::zombie::test_bit(a_emitter_def.particle_flags as u32, ParticleFlags::Fullscreen as u32) {
            // C++: FULLSCREEN 粒子以纯色填满屏幕
            let an_old_color = g.color;
            let an_old_draw_mode = g.draw_mode;
            g.set_color(&the_color);
            g.fill_rect_xywh(-g.trans_x as i32, -g.trans_y as i32, crate::lawn::game_enums::BOARD_WIDTH, crate::lawn::game_enums::BOARD_HEIGHT);
            g.set_color(&an_old_color);
            g.set_draw_mode(an_old_draw_mode);
        } else {
            // C++: PvzpScaleRotateTransformMatrix + AddTriangle（矩阵旋转缩放）
            let mut a_transform = crate::framework::sexy_matrix::SexyMatrix3::identity();
            pvzp_scale_rotate_transform_matrix(
                &mut a_transform,
                a_pos_x,
                a_pos_y,
                the_params.spin_position,
                the_params.particle_scale,
                the_params.particle_stretch * the_params.particle_scale,
            );
            g.set_colorize_images(true);
            g.set_color(&the_color);
            g.set_draw_mode(a_draw_mode);
            g.draw_image_matrix_src(a_image_ref, &a_transform, &a_src_rect, 0.0, 0.0);
            // C++: mExtraAdditiveDrawOverride 时以 ADDITIVE 再画一次
            if self.extra_additive_draw_override {
                g.set_draw_mode(crate::framework::graphics::graphics::DrawMode::Additive as i32);
                g.draw_image_matrix_src(a_image_ref, &a_transform, &a_src_rect, 0.0, 0.0);
            }
            g.set_draw_mode(crate::framework::graphics::graphics::DrawMode::Normal as i32);
            g.set_colorize_images(false);
        }
    }

    /// 移动发射器系统中心（对应 C++ PvzpParticleEmitter::SystemMove，:1079）
    pub fn system_move(&mut self, x: f32, y: f32) {
        let a_delta_x = x - self.system_center.x;
        let a_delta_y = y - self.system_center.y;
        // 对应 C++: FloatApproxEqual(0) 近似
        if a_delta_x.abs() < 0.0001 && a_delta_y.abs() < 0.0001 {
            return;
        }
        self.system_center.x = x;
        self.system_center.y = y;
        // 对应 C++: PARTICLES_DONT_FOLLOW 时粒子不随系统移动
        let def = unsafe { &*self.emitter_def };
        if !crate::lawn::zombie::test_bit(def.particle_flags as u32, ParticleFlags::ParticlesDontFollow as u32) {
            let a_particle_ids: Vec<ParticleID> = unsafe {
                let mut ids = Vec::new();
                let mut node = self.particle_list.head;
                while !node.is_null() {
                    ids.push((*node).value);
                    node = (*node).next;
                }
                ids
            };
            unsafe {
                if !self.particle_system.is_null() {
                    let holder = (*self.particle_system).particle_holder;
                    if !holder.is_null() {
                        for a_particle_id in a_particle_ids {
                            if let Some(a_particle) = (*holder).particles.try_to_get_mut(a_particle_id) {
                                a_particle.position.x += a_delta_x;
                                a_particle.position.y += a_delta_y;
                            }
                        }
                    }
                }
            }
        }
    }
    /// 更新生成（对应 C++ PvzpParticleEmitter::UpdateSpawning，PvzpParticle.cpp:671）
    pub fn update_spawning(&mut self) {
        // 对应 C++: 发射数据源——cross-fade 目标发射器（primary）或自身
        let a_spawning_ptr: *mut TodParticleEmitter = unsafe {
            if self.cross_fade_emitter_id != PARTICLEEMITTERID_NULL && !self.particle_system.is_null() {
                let holder = (*self.particle_system).particle_holder;
                if holder.is_null() {
                    self as *mut TodParticleEmitter
                } else {
                    match (*holder).emitters.try_to_get(self.cross_fade_emitter_id) {
                        Some(e) => e as *const TodParticleEmitter as *mut TodParticleEmitter,
                        None => self as *mut TodParticleEmitter,
                    }
                }
            } else {
                self as *mut TodParticleEmitter
            }
        };
        let a_active_count = self.particle_list.count;
        let a_spawned_count = self.particles_spawned;
        unsafe {
            let a_spawning = &mut *a_spawning_ptr;
            let a_def = &*a_spawning.emitter_def;

            a_spawning.spawn_accum += a_spawning.system_track_evaluate(&a_def.spawn_rate, ParticleSystemTracks::SpawnRate) * 0.01;
            let mut a_spawn_count = a_spawning.spawn_accum as i32;
            a_spawning.spawn_accum -= a_spawn_count as f32;

            // 对应 C++: aSpawnMinActive 下限
            let a_spawn_min_active =
                a_spawning.system_track_evaluate(&a_def.spawn_min_active, ParticleSystemTracks::SpawnMinActive) as i32;
            if a_spawn_min_active >= 0 && a_spawn_count < a_spawn_min_active - a_active_count {
                a_spawn_count = a_spawn_min_active - a_active_count;
            }
            // 对应 C++: aSpawnMaxActive 上限
            let a_spawn_max_active =
                a_spawning.system_track_evaluate(&a_def.spawn_max_active, ParticleSystemTracks::SpawnMaxActive) as i32;
            if a_spawn_max_active >= 0 && a_spawn_count > a_spawn_max_active - a_active_count {
                a_spawn_count = a_spawn_max_active - a_active_count;
            }
            // 对应 C++: aSpawnMaxLaunched 总数上限
            if !a_def.spawn_max_launched.nodes.is_empty() {
                let a_spawn_max_launched =
                    a_spawning.system_track_evaluate(&a_def.spawn_max_launched, ParticleSystemTracks::SpawnMaxLaunched) as i32;
                if a_spawn_count > a_spawn_max_launched - a_spawned_count {
                    a_spawn_count = a_spawn_max_launched - a_spawned_count;
                }
            }

            // 对应 C++: cross-fade 目标发射器（存在时新粒子全部淡入该发射器）
            let a_cross_fade_ptr: *mut TodParticleEmitter = unsafe {
                if self.cross_fade_emitter_id != PARTICLEEMITTERID_NULL && !self.particle_system.is_null() {
                    let holder = (*self.particle_system).particle_holder;
                    if holder.is_null() {
                        std::ptr::null_mut()
                    } else {
                        match (*holder).emitters.try_to_get(self.cross_fade_emitter_id) {
                            Some(e) => e as *const TodParticleEmitter as *mut TodParticleEmitter,
                            None => std::ptr::null_mut(),
                        }
                    }
                } else {
                    std::ptr::null_mut()
                }
            };
            for i in 0..a_spawn_count {
                let a_particle_ptr = a_spawning
                    .spawn_particle(i, a_spawn_count)
                    .map_or(std::ptr::null_mut(), |p| p as *mut TodParticle);
                if !a_cross_fade_ptr.is_null() && !a_particle_ptr.is_null() {
                    a_spawning.cross_fade_particle(unsafe { &mut *a_particle_ptr }, a_cross_fade_ptr);
                }
            }
        }
    }

    /// 粒子轨道评估（对应 C++ PvzpParticleEmitter::ParticleTrackEvaluate）
    fn particle_track_evaluate(&self, the_track: &FloatParameterTrack, the_particle: &TodParticle, the_particle_track: ParticleTracks) -> f32 {
        the_track.evaluate(the_particle.particle_time_value, the_particle.particle_interp[the_particle_track as usize])
    }

    /// 系统轨道评估（对应 C++ PvzpParticleEmitter::SystemTrackEvaluate）
    fn system_track_evaluate(&self, the_track: &FloatParameterTrack, the_system_track: ParticleSystemTracks) -> f32 {
        the_track.evaluate(self.system_time_value, self.track_interp[the_system_track as usize])
    }

    /// 更新粒子字段（对应 C++ PvzpParticleEmitter::UpdateParticleField，:480）
    fn update_particle_field(
        &mut self,
        the_particle: &mut TodParticle,
        the_particle_field: &ParticleField,
        the_particle_time_value: f32,
        the_field_index: i32,
    ) {
        let a_interp_x = the_particle.particle_field_interp[the_field_index as usize][0];
        let a_interp_y = the_particle.particle_field_interp[the_field_index as usize][1];
        let x = the_particle_field.x.evaluate(the_particle_time_value, a_interp_x);
        let y = the_particle_field.y.evaluate(the_particle_time_value, a_interp_y);

        match the_particle_field.field_type {
            ParticleFieldType::Invalid => {}
            ParticleFieldType::Friction => {
                the_particle.velocity.x *= 1.0 - x;
                the_particle.velocity.y *= 1.0 - y;
            }
            ParticleFieldType::Acceleration => {
                the_particle.velocity.x += 0.01 * x;
                the_particle.velocity.y += 0.01 * y;
            }
            ParticleFieldType::Attractor => {
                let a_diff_x = x - (the_particle.position.x - self.system_center.x);
                let a_diff_y = y - (the_particle.position.y - self.system_center.y);
                // 对应 C++: 加速度从粒子指向目标位置
                the_particle.velocity.x += 0.01 * a_diff_x;
                the_particle.velocity.y += 0.01 * a_diff_y;
            }
            ParticleFieldType::MaxVelocity => {
                the_particle.velocity.x = the_particle.velocity.x.clamp(-x, x);
                the_particle.velocity.y = the_particle.velocity.y.clamp(-y, y);
            }
            ParticleFieldType::Velocity => {
                the_particle.position.x += 0.01 * x;
                the_particle.position.y += 0.01 * y;
            }
            ParticleFieldType::Position => {
                let a_last_x = the_particle_field.x.evaluate(the_particle.particle_last_time_value, a_interp_x);
                let a_last_y = the_particle_field.y.evaluate(the_particle.particle_last_time_value, a_interp_y);
                the_particle.position.x += x - a_last_x;
                the_particle.position.y += y - a_last_y;
            }
            ParticleFieldType::SystemPosition => {
                // 对应 C++: FIELD_SYSTEM_POSITION 仅用于系统字段；此处为 default assert 分支
            }
            ParticleFieldType::GroundConstraint => {
                if the_particle.position.y > self.system_center.y + y {
                    // 对应 C++: 接触地面复位并反弹
                    the_particle.position.y = self.system_center.y + y;
                    let def = unsafe { &*self.emitter_def };
                    let a_collision_reflect = def.collision_reflect.evaluate(
                        the_particle_time_value,
                        the_particle.particle_interp[ParticleTracks::CollisionReflect as usize],
                    );
                    let a_collision_spin = def.collision_spin.evaluate(
                        the_particle_time_value,
                        the_particle.particle_interp[ParticleTracks::CollisionSpin as usize],
                    ) / 1000.0;
                    the_particle.spin_velocity = the_particle.velocity.y * a_collision_spin;
                    the_particle.velocity.x *= a_collision_reflect;
                    the_particle.velocity.y *= -a_collision_reflect;
                }
            }
            ParticleFieldType::Shake => {
                let a_last_x = the_particle_field.x.evaluate(the_particle.particle_last_time_value, a_interp_x);
                let a_last_y = the_particle_field.y.evaluate(the_particle.particle_last_time_value, a_interp_y);
                // 对应 C++: 撤销上一帧震动（确定性随机种子 = 上一帧年龄 * 粒子地址）
                let mut a_last_rand_seed = the_particle.particle_age - 1;
                if a_last_rand_seed == -1 {
                    a_last_rand_seed = the_particle.particle_duration - 1;
                }
                crate::framework::common::SRand(
                    (a_last_rand_seed as u32).wrapping_mul(the_particle as *const TodParticle as usize as u32),
                );
                the_particle.position.x -= a_last_x * (RandFloat(2.0) - 1.0);
                the_particle.position.y -= a_last_y * (RandFloat(2.0) - 1.0);
                // 应用本帧震动
                crate::framework::common::SRand(
                    (the_particle.particle_age as u32).wrapping_mul(the_particle as *const TodParticle as usize as u32),
                );
                the_particle.position.x += x * (RandFloat(2.0) - 1.0);
                the_particle.position.y += y * (RandFloat(2.0) - 1.0);
            }
            ParticleFieldType::Circle => {
                let a_to_center = the_particle.position - self.system_center;
                let a_motion = a_to_center.perp().normalize();
                let a_radius = a_to_center.magnitude();
                let a_motion = a_motion * (0.01 * (x + a_radius * y));
                the_particle.position += a_motion;
            }
            ParticleFieldType::Away => {
                let a_to_center = the_particle.position - self.system_center;
                let a_motion = a_to_center.normalize();
                let a_radius = a_to_center.magnitude();
                let a_motion = a_motion * (0.01 * (x + a_radius * y));
                the_particle.position += a_motion;
            }
            ParticleFieldType::Count => {}
        }
    }

    /// 更新系统字段（对应 C++ PvzpParticleEmitter::UpdateSystemField，:586）
    fn update_system_field(&mut self, the_particle_field: &ParticleField, the_particle_time_value: f32, the_field_index: i32) {
        let a_interp_x = self.system_field_interp[the_field_index as usize][0];
        let a_interp_y = self.system_field_interp[the_field_index as usize][1];
        let x = the_particle_field.x.evaluate(the_particle_time_value, a_interp_x);
        let y = the_particle_field.y.evaluate(the_particle_time_value, a_interp_y);

        match the_particle_field.field_type {
            ParticleFieldType::SystemPosition => {
                let a_last_x = the_particle_field.x.evaluate(self.system_last_time_value, a_interp_x);
                let a_last_y = the_particle_field.y.evaluate(self.system_last_time_value, a_interp_y);
                self.system_center.x += x - a_last_x;
                self.system_center.y += y - a_last_y;
            }
            _ => {
                // 对应 C++: default assert 分支
            }
        }
    }

    /// 更新粒子（对应 C++ PvzpParticleEmitter::UpdateParticle，:631）
    pub fn update_particle(&mut self, the_particle: &mut TodParticle) -> bool {
        let def = unsafe { &*self.emitter_def };
        // 对应 C++: 粒子到达寿命终点
        if the_particle.particle_age >= the_particle.particle_duration {
            if crate::lawn::zombie::test_bit(def.particle_flags as u32, ParticleFlags::ParticleLoops as u32) {
                the_particle.particle_age = 0;
            } else if the_particle.cross_fade_duration > 0 {
                the_particle.particle_age = the_particle.particle_duration - 1; // 对应 C++: 粒子在其末帧保持
            } else if def.on_duration.is_empty() || !self.cross_fade_particle_to_name(the_particle, &def.on_duration) {
                return false;
            }
        }
        // 对应 C++: 淡出源粒子消失则删除
        if the_particle.cross_fade_particle_id != PARTICLEID_NULL {
            let a_source_alive = unsafe {
                if self.particle_system.is_null() {
                    false
                } else {
                    let holder = (*self.particle_system).particle_holder;
                    !holder.is_null() && (*holder).particles.try_to_get(the_particle.cross_fade_particle_id).is_some()
                }
            };
            if !a_source_alive {
                return false;
            }
        }

        the_particle.particle_time_value =
            the_particle.particle_age as f32 / (the_particle.particle_duration - 1) as f32;
        // 对应 C++: 粒子字段逐个应用
        let a_num_fields = def.particle_fields.fields.len().min(MAX_PARTICLE_FIELDS);
        for i in 0..a_num_fields {
            let a_field = &def.particle_fields.fields[i];
            self.update_particle_field(the_particle, a_field, the_particle.particle_time_value, i as i32);
        }
        the_particle.position += the_particle.velocity;

        // 对应 C++: 自旋
        let a_spin_speed = self.particle_track_evaluate(&def.particle_spin_speed, the_particle, ParticleTracks::SpinSpeed) * 0.01;
        let a_spin_angle = self.particle_track_evaluate(&def.particle_spin_angle, the_particle, ParticleTracks::SpinAngle);
        let a_last_spin_angle = def.particle_spin_angle.evaluate(
            the_particle.particle_last_time_value,
            the_particle.particle_interp[ParticleTracks::SpinAngle as usize],
        );
        the_particle.spin_position +=
            (a_spin_speed + a_spin_angle - a_last_spin_angle).to_radians() + the_particle.spin_velocity;

        // 对应 C++: 动画速率（循环缠绕到 [0, 1)）
        if !def.animation_rate.nodes.is_empty() {
            let a_anim_time = self.particle_track_evaluate(&def.animation_rate, the_particle, ParticleTracks::AnimationRate) * 0.01;
            the_particle.animation_time_value += a_anim_time;
            while the_particle.animation_time_value >= 1.0 {
                the_particle.animation_time_value -= 1.0;
            }
            while the_particle.animation_time_value < 0.0 {
                the_particle.animation_time_value += 1.0;
            }
        }

        the_particle.particle_age += 1;
        the_particle.particle_last_time_value = the_particle.particle_time_value;
        true
    }

    /// 淡出到指定名称发射器（对应 C++ CrossFadeParticleToName，:610）
    pub fn cross_fade_particle_to_name(&mut self, the_particle: &mut TodParticle, the_emitter_name: &str) -> bool {
        // 对应 C++: 在系统定义中按名称查找目标发射器定义
        let a_def_ptr: *mut TodEmitterDefinition = unsafe {
            if self.particle_system.is_null() {
                std::ptr::null_mut()
            } else {
                match (*self.particle_system).find_emitter_def_by_name(the_emitter_name) {
                    Some(d) => d as *const TodEmitterDefinition as *mut TodEmitterDefinition,
                    None => std::ptr::null_mut(),
                }
            }
        };
        if a_def_ptr.is_null() {
            return false;
        }
        unsafe {
            let a_system = &mut *self.particle_system;
            let holder = &mut *a_system.particle_holder;
            // 对应 C++: 发射器数组满
            if holder.emitters.len() as u32 >= holder.emitters.max_size() {
                return false;
            }
            // 对应 C++: 分配目标发射器并初始化
            let a_emitter = holder.emitters.alloc();
            (*a_emitter).tod_emitter_initialize(
                self.system_center.x,
                self.system_center.y,
                a_system as *mut TodParticleSystem,
                a_def_ptr,
            );
            let a_emitter_id = holder.emitters.get_id(a_emitter);
            // 对应 C++: mParticleSystem->mEmitterList.AddTail(aEmitterID)
            a_system.emitter_list.push_back_value(a_emitter_id);
            // 对应 C++: return CrossFadeParticle(theParticle, aEmitter);
            self.cross_fade_particle(the_particle, a_emitter)
        }
    }

    /// 交叉淡入淡出粒子（对应 C++ CrossFadeParticle，:739）
    pub fn cross_fade_particle(&mut self, the_particle: &mut TodParticle, the_to_emitter: *mut TodParticleEmitter) -> bool {
        unsafe {
            // 对应 C++: 不支持连续多次淡出
            if the_particle.cross_fade_duration > 0 {
                return false;
            }
            let to_def = &*(*the_to_emitter).emitter_def;
            // 对应 C++: 目标发射器必须定义了 cross-fade 时长
            if to_def.cross_fade_duration.nodes.is_empty() {
                return false;
            }
            // 对应 C++: 目标发射器生成过渡粒子
            let a_to_particle = match (*the_to_emitter).spawn_particle(0, 1) {
                Some(p) => p,
                None => return false,
            };
            // 对应 C++: 淡出时长（继承源发射器剩余淡出时间，或随机评估）
            if self.emitter_cross_fade_count_down > 0 {
                the_particle.cross_fade_duration = self.emitter_cross_fade_count_down;
            } else {
                let a_cross_fade_duration_interp = RandRange(1) as f32;
                let a_cross_fade_duration =
                    to_def.cross_fade_duration.evaluate(self.system_time_value, a_cross_fade_duration_interp) as i32;
                the_particle.cross_fade_duration = a_cross_fade_duration.max(1);
            }
            // 对应 C++: 目标发射器粒子时长未定义时使用淡出时长
            if to_def.particle_duration.nodes.is_empty() {
                a_to_particle.particle_duration = the_particle.cross_fade_duration;
            }
            // 对应 C++: 记录源粒子（淡出源消失时过渡粒子随之删除）
            let holder = (*self.particle_system).particle_holder;
            let a_source_id = (*holder).particles.get_id(the_particle);
            a_to_particle.cross_fade_particle_id = a_source_id;
            true
        }
    }

    /// 淡出到指定发射器（对应 C++ PvzpParticleEmitter::CrossFadeEmitter）
    pub fn cross_fade_emitter(&mut self, the_to_emitter: *mut TodParticleEmitter) {
        // 对应 C++: 不支持同时淡出多个发射器
        if self.emitter_cross_fade_count_down > 0 {
            return;
        }
        let to_def = unsafe { &*(*the_to_emitter).emitter_def };
        // 对应 C++: 目标发射器必须设置了 cross-fade 时长
        if to_def.cross_fade_duration.nodes.is_empty() {
            return;
        }
        // 对应 C++: 随机淡出时长（至少 1 帧）
        let a_cross_fade_duration_interp = RandFloat(1.0);
        self.emitter_cross_fade_count_down =
            to_def.cross_fade_duration.evaluate(self.system_time_value, a_cross_fade_duration_interp) as i32;
        self.emitter_cross_fade_count_down = self.emitter_cross_fade_count_down.max(1);
        // 对应 C++: 记录淡出目标发射器 ID
        unsafe {
            let holder = (*self.particle_system).particle_holder;
            let a_id = (*holder).emitters.get_id(the_to_emitter);
            self.cross_fade_emitter_id = a_id;
        }
        // 对应 C++: 目标发射器未定义 system duration 时用淡出时长
        if to_def.system_duration.nodes.is_empty() {
            unsafe {
                (*the_to_emitter).system_duration = self.emitter_cross_fade_count_down;
            }
        }
    }

    /// 获取粒子渲染参数（对应 C++ PvzpParticleEmitter::GetRenderParams，:849）
    pub fn get_render_params(&self, the_particle: &TodParticle, the_params: &mut ParticleRenderParams) -> bool {
        let a_emitter = the_particle.particle_emitter;
        if a_emitter.is_null() {
            return false;
        }
        let a_def = unsafe { &*(*a_emitter).emitter_def };
        let a_emitter_ref = unsafe { &*a_emitter };

        // 对应 C++: 各通道"已设置"判定（系统轨道/粒子轨道/覆盖色任一存在）
        the_params.red_is_set = false;
        the_params.red_is_set |= !a_def.system_red.nodes.is_empty();
        the_params.red_is_set |= !a_def.particle_red.nodes.is_empty();
        the_params.red_is_set |= a_emitter_ref.color_override.r != 1;
        the_params.green_is_set = false;
        the_params.green_is_set |= !a_def.system_green.nodes.is_empty();
        the_params.green_is_set |= !a_def.particle_green.nodes.is_empty();
        the_params.green_is_set |= a_emitter_ref.color_override.g != 1;
        the_params.blue_is_set = false;
        the_params.blue_is_set |= !a_def.system_blue.nodes.is_empty();
        the_params.blue_is_set |= !a_def.particle_blue.nodes.is_empty();
        the_params.blue_is_set |= a_emitter_ref.color_override.b != 1;
        the_params.alpha_is_set = false;
        the_params.alpha_is_set |= !a_def.system_alpha.nodes.is_empty();
        the_params.alpha_is_set |= !a_def.particle_alpha.nodes.is_empty();
        the_params.alpha_is_set |= a_emitter_ref.color_override.a != 1;
        the_params.particle_scale_is_set = false;
        the_params.particle_scale_is_set |= !a_def.particle_scale.nodes.is_empty();
        the_params.particle_scale_is_set |= a_emitter_ref.scale_override != 1.0;
        the_params.particle_stretch_is_set = !a_def.particle_stretch.nodes.is_empty();
        // 对应 C++: 自旋判定（随机/对齐发射自旋也算已设置）
        the_params.spin_position_is_set = false;
        the_params.spin_position_is_set |= !a_def.particle_spin_speed.nodes.is_empty();
        the_params.spin_position_is_set |= !a_def.particle_spin_angle.nodes.is_empty();
        the_params.spin_position_is_set |=
            crate::lawn::zombie::test_bit(a_def.particle_flags as u32, ParticleFlags::RandomLaunchSpin as u32);
        the_params.spin_position_is_set |=
            crate::lawn::zombie::test_bit(a_def.particle_flags as u32, ParticleFlags::AlignLaunchSpin as u32);
        the_params.position_is_set = false;
        the_params.position_is_set |= a_def.particle_fields.fields.len() > 0;
        the_params.position_is_set |= !a_def.emitter_radius.nodes.is_empty();
        the_params.position_is_set |= !a_def.emitter_offset_x.nodes.is_empty();
        the_params.position_is_set |= !a_def.emitter_offset_y.nodes.is_empty();
        the_params.position_is_set |= !a_def.emitter_box_x.nodes.is_empty();
        the_params.position_is_set |= !a_def.emitter_box_y.nodes.is_empty();

        // 对应 C++: 颜色 = 粒子 × 系统 × 覆盖色 × 亮度
        let a_system_red = a_emitter_ref.system_track_evaluate(&a_def.system_red, ParticleSystemTracks::SystemRed);
        let a_system_green = a_emitter_ref.system_track_evaluate(&a_def.system_green, ParticleSystemTracks::SystemGreen);
        let a_system_blue = a_emitter_ref.system_track_evaluate(&a_def.system_blue, ParticleSystemTracks::SystemBlue);
        let a_system_alpha = a_emitter_ref.system_track_evaluate(&a_def.system_alpha, ParticleSystemTracks::SystemAlpha);
        let a_system_brightness = a_emitter_ref.system_track_evaluate(&a_def.system_brightness, ParticleSystemTracks::SystemBrightness);
        let a_particle_red = a_emitter_ref.particle_track_evaluate(&a_def.particle_red, the_particle, ParticleTracks::ParticleRed);
        let a_particle_green = a_emitter_ref.particle_track_evaluate(&a_def.particle_green, the_particle, ParticleTracks::ParticleGreen);
        let a_particle_blue = a_emitter_ref.particle_track_evaluate(&a_def.particle_blue, the_particle, ParticleTracks::ParticleBlue);
        let a_particle_alpha = a_emitter_ref.particle_track_evaluate(&a_def.particle_alpha, the_particle, ParticleTracks::ParticleAlpha);
        let a_particle_brightness = a_emitter_ref.particle_track_evaluate(&a_def.particle_brightness, the_particle, ParticleTracks::ParticleBrightness);
        let a_brightness = a_particle_brightness * a_system_brightness;
        the_params.red = a_particle_red * a_system_red * a_emitter_ref.color_override.r as f32 * a_brightness;
        the_params.green = a_particle_green * a_system_green * a_emitter_ref.color_override.g as f32 * a_brightness;
        the_params.blue = a_particle_blue * a_system_blue * a_emitter_ref.color_override.b as f32 * a_brightness;
        the_params.alpha = a_particle_alpha * a_system_alpha * a_emitter_ref.color_override.a as f32 * a_brightness;
        the_params.pos_x = the_particle.position.x;
        the_params.pos_y = the_particle.position.y;
        let a_particle_scale = a_emitter_ref.particle_track_evaluate(&a_def.particle_scale, the_particle, ParticleTracks::Scale);
        the_params.particle_stretch = a_emitter_ref.particle_track_evaluate(&a_def.particle_stretch, the_particle, ParticleTracks::Stretch);
        the_params.particle_scale = a_particle_scale * a_emitter_ref.scale_override;
        the_params.spin_position = the_particle.spin_position;

        // 对应 C++: 与淡出源粒子混合渲染参数
        let a_cross_fade_id = the_particle.cross_fade_particle_id;
        if a_cross_fade_id != PARTICLEID_NULL {
            let a_cross_fade_particle = unsafe {
                if a_emitter_ref.particle_system.is_null() {
                    None
                } else {
                    let holder = (*(a_emitter_ref.particle_system)).particle_holder;
                    if holder.is_null() {
                        None
                    } else {
                        (*holder).particles.try_to_get(a_cross_fade_id)
                    }
                }
            };
            if let Some(a_cross_fade_ref) = a_cross_fade_particle {
                let mut a_cross_fade_params = ParticleRenderParams::new();
                let a_cross_emitter = a_cross_fade_ref.particle_emitter;
                if !a_cross_emitter.is_null() && unsafe { (*a_cross_emitter).get_render_params(a_cross_fade_ref, &mut a_cross_fade_params) } {
                    let a_fraction =
                        the_particle.particle_age as f32 / (a_cross_fade_ref.cross_fade_duration - 1) as f32;
                    the_params.red = crate::todlib::tod_particle::cross_fade_lerp(
                        a_cross_fade_params.red, the_params.red, a_cross_fade_params.red_is_set, the_params.red_is_set, a_fraction);
                    the_params.green = crate::todlib::tod_particle::cross_fade_lerp(
                        a_cross_fade_params.green, the_params.green, a_cross_fade_params.green_is_set, the_params.green_is_set, a_fraction);
                    the_params.blue = crate::todlib::tod_particle::cross_fade_lerp(
                        a_cross_fade_params.blue, the_params.blue, a_cross_fade_params.blue_is_set, the_params.blue_is_set, a_fraction);
                    the_params.alpha = crate::todlib::tod_particle::cross_fade_lerp(
                        a_cross_fade_params.alpha, the_params.alpha, a_cross_fade_params.alpha_is_set, the_params.alpha_is_set, a_fraction);
                    the_params.particle_scale = crate::todlib::tod_particle::cross_fade_lerp(
                        a_cross_fade_params.particle_scale, the_params.particle_scale,
                        a_cross_fade_params.particle_scale_is_set, the_params.particle_scale_is_set, a_fraction);
                    the_params.particle_stretch = crate::todlib::tod_particle::cross_fade_lerp(
                        a_cross_fade_params.particle_stretch, the_params.particle_stretch,
                        a_cross_fade_params.particle_stretch_is_set, the_params.particle_stretch_is_set, a_fraction);
                    the_params.spin_position = crate::todlib::tod_particle::cross_fade_lerp(
                        a_cross_fade_params.spin_position, the_params.spin_position,
                        a_cross_fade_params.spin_position_is_set, the_params.spin_position_is_set, a_fraction);
                    the_params.pos_x = crate::todlib::tod_particle::cross_fade_lerp(
                        a_cross_fade_params.pos_x, the_params.pos_x, a_cross_fade_params.position_is_set, the_params.position_is_set, a_fraction);
                    the_params.pos_y = crate::todlib::tod_particle::cross_fade_lerp(
                        a_cross_fade_params.pos_y, the_params.pos_y, a_cross_fade_params.position_is_set, the_params.position_is_set, a_fraction);
                    // 对应 C++: 源端已设置的通道也计入
                    the_params.red_is_set |= a_cross_fade_params.red_is_set;
                    the_params.green_is_set |= a_cross_fade_params.green_is_set;
                    the_params.blue_is_set |= a_cross_fade_params.blue_is_set;
                    the_params.alpha_is_set |= a_cross_fade_params.alpha_is_set;
                    the_params.particle_scale_is_set |= a_cross_fade_params.particle_scale_is_set;
                    the_params.particle_stretch_is_set |= a_cross_fade_params.particle_stretch_is_set;
                    the_params.spin_position_is_set |= a_cross_fade_params.spin_position_is_set;
                    the_params.position_is_set |= a_cross_fade_params.position_is_set;
                }
            }
        }
        true
    }

    /// 删除非淡出中的粒子（对应 C++ DeleteNonCrossFading，:700）
    fn delete_non_cross_fading(&mut self) {
        let a_particle_ids: Vec<ParticleID> = unsafe {
            let mut ids = Vec::new();
            let mut node = self.particle_list.head;
            while !node.is_null() {
                ids.push((*node).value);
                node = (*node).next;
            }
            ids
        };
        for a_particle_id in a_particle_ids {
            let a_particle_ptr = unsafe {
                if self.particle_system.is_null() {
                    std::ptr::null_mut()
                } else {
                    let holder = (*self.particle_system).particle_holder;
                    if holder.is_null() {
                        std::ptr::null_mut()
                    } else {
                        match (*holder).particles.try_to_get(a_particle_id) {
                            Some(p) => p as *const TodParticle as *mut TodParticle,
                            None => std::ptr::null_mut(),
                        }
                    }
                }
            };
            if a_particle_ptr.is_null() {
                continue;
            }
            let a_cross_fade_duration = unsafe { (*a_particle_ptr).cross_fade_duration };
            if a_cross_fade_duration <= 0 {
                self.delete_particle(a_particle_ptr);
            }
        }
    }
    pub fn spawn_particle(&mut self, the_index: i32, the_spawn_count: i32) -> Option<&mut TodParticle> {
        // 对应 C++ SpawnParticle（PvzpParticle.cpp:331）
        let a_system_ptr = self.particle_system;
        if a_system_ptr.is_null() {
            return None;
        }
        unsafe {
            let a_holder_ptr = (*a_system_ptr).particle_holder;
            if a_holder_ptr.is_null() {
                return None;
            }
            let a_data_array = &mut (*a_holder_ptr).particles;
            // 对应 C++: DataArray 满则返回 nullptr
            if a_data_array.len() as u32 >= a_data_array.max_size() {
                return None;
            }
            let a_particle = &mut *a_data_array.alloc();

            let def = &*self.emitter_def;
            // 对应 C++: 粒子字段插值随机化（[0] 与 [1] 各一个随机插值）
            for i in 0..MAX_PARTICLE_FIELDS {
                a_particle.particle_field_interp[i][0] = RandFloat(1.0);
                a_particle.particle_field_interp[i][1] = RandFloat(1.0);
            }
            // 对应 C++: NUM_PARTICLE_TRACKS == 16
            for i in 0..16 {
                a_particle.particle_interp[i] = RandFloat(1.0);
            }

            let a_particle_duration_interp = RandFloat(1.0);
            let a_launch_speed_interp = RandFloat(1.0);
            let a_emitter_offset_x_interp = RandFloat(1.0);
            let a_emitter_offset_y_interp = RandFloat(1.0);
            a_particle.particle_duration =
                def.particle_duration.evaluate(self.system_time_value, a_particle_duration_interp) as i32;
            a_particle.particle_duration = a_particle.particle_duration.max(1); // duration 至少 1
            a_particle.particle_age = 0;
            a_particle.particle_emitter = self as *mut TodParticleEmitter;
            a_particle.particle_time_value = -1.0;
            a_particle.particle_last_time_value = -1.0;
            // 对应 C++: PARTICLE_RANDOM_START_TIME 时从随机年龄开始
            if crate::lawn::zombie::test_bit(
                def.particle_flags as u32,
                ParticleFlags::RandomStartTime as u32,
            ) {
                a_particle.particle_age = RandRange(a_particle.particle_duration);
            }
            let a_launch_speed = def.launch_speed.evaluate(self.system_time_value, a_launch_speed_interp) * 0.01;
            let a_launch_angle_interp = RandFloat(1.0);

            // 对应 C++: 发射角计算（按发射器类型）
            let a_launch_angle;
            if def.emitter_type == EmitterType::CirclePath {
                a_launch_angle = def.emitter_path
                    .evaluate(self.system_time_value, self.track_interp[ParticleSystemTracks::EmitterPath as usize])
                    * (2.0 * std::f32::consts::PI)
                    + def.launch_angle.evaluate(self.system_time_value, a_launch_angle_interp).to_radians();
            } else if def.emitter_type == EmitterType::CircleEvenSpacing {
                // 对应 C++: 基础角把 theSpawnCount 个粒子均布于圆周
                a_launch_angle = 2.0 * std::f32::consts::PI * the_index as f32 / the_spawn_count as f32
                    + def.launch_angle.evaluate(self.system_time_value, a_launch_angle_interp).to_radians();
            } else {
                // 对应 C++: FloatTrackIsConstantZero(mLaunchAngle)：单节点全零则随机角
                let a_angle_constant_zero = def.launch_angle.nodes.len() == 1
                    && def.launch_angle.nodes[0].low_value == 0.0
                    && def.launch_angle.nodes[0].high_value == 0.0;
                if a_angle_constant_zero {
                    a_launch_angle = RandFloat(2.0 * std::f32::consts::PI);
                } else {
                    a_launch_angle = def.launch_angle.evaluate(self.system_time_value, a_launch_angle_interp).to_radians();
                }
            }

            // 对应 C++: 位置计算（按发射器类型）
            let mut a_pos_x = 0.0;
            let mut a_pos_y = 0.0;
            match def.emitter_type {
                EmitterType::Circle | EmitterType::CirclePath | EmitterType::CircleEvenSpacing => {
                    let a_emitter_radius_interp = RandFloat(1.0);
                    let a_radius = def.emitter_radius.evaluate(self.system_time_value, a_emitter_radius_interp);
                    // 对应 C++: 角度 0 指向正下方
                    a_pos_x = a_launch_angle.sin() * a_radius;
                    a_pos_y = a_launch_angle.cos() * a_radius;
                }
                EmitterType::Box => {
                    let a_emitter_box_x_interp = RandFloat(1.0);
                    let a_emitter_box_y_interp = RandFloat(1.0);
                    a_pos_x = def.emitter_box_x.evaluate(self.system_time_value, a_emitter_box_x_interp);
                    a_pos_y = def.emitter_box_y.evaluate(self.system_time_value, a_emitter_box_y_interp);
                }
                EmitterType::BoxPath => {
                    // 对应 C++: 沿矩形边界的路径位置
                    let a_emitter_path_position = def.emitter_path
                        .evaluate(self.system_time_value, self.track_interp[ParticleSystemTracks::EmitterPath as usize]);
                    let a_min_x = def.emitter_box_x.evaluate(self.system_time_value, 0.0);
                    let a_max_x = def.emitter_box_x.evaluate(self.system_time_value, 1.0);
                    let a_min_y = def.emitter_box_y.evaluate(self.system_time_value, 0.0);
                    let a_max_y = def.emitter_box_y.evaluate(self.system_time_value, 1.0);
                    let a_distance_x = a_max_x - a_min_x;
                    let a_distance_y = a_max_y - a_min_y;
                    let a_path_pos = a_emitter_path_position * (a_distance_y + a_distance_x + a_distance_y + a_distance_x);
                    if a_path_pos < a_distance_y {
                        a_pos_x = a_min_x;
                        a_pos_y = a_min_y + a_path_pos;
                    } else if a_path_pos < a_distance_y + a_distance_x {
                        a_pos_x = a_min_x + (a_path_pos - a_distance_y);
                        a_pos_y = a_max_y;
                    } else if a_path_pos < a_distance_y + a_distance_x + a_distance_y {
                        a_pos_x = a_max_x;
                        a_pos_y = a_max_y - (a_path_pos - a_distance_y - a_distance_x);
                    } else {
                        a_pos_x = a_max_x - (a_path_pos - a_distance_y - a_distance_x - a_distance_y);
                        a_pos_y = a_min_y;
                    }
                }
            }

            // 对应 C++: 偏斜（X 偏斜随 Y 坐标缩放，反之亦然）
            let a_emitter_skew_x_interp = RandFloat(1.0);
            let a_emitter_skew_y_interp = RandFloat(1.0);
            let a_skew_x = def.emitter_skew_x.evaluate(self.system_time_value, a_emitter_skew_x_interp);
            let a_skew_y = def.emitter_skew_y.evaluate(self.system_time_value, a_emitter_skew_y_interp);
            a_particle.position.x = self.system_center.x + a_pos_x + a_pos_y * a_skew_x;
            a_particle.position.y = self.system_center.y + a_pos_y + a_pos_x * a_skew_y;
            a_particle.velocity.x = a_launch_angle.sin() * a_launch_speed;
            a_particle.velocity.y = a_launch_angle.cos() * a_launch_speed;
            a_particle.position.x += def.emitter_offset_x.evaluate(self.system_time_value, a_emitter_offset_x_interp);
            a_particle.position.y += def.emitter_offset_y.evaluate(self.system_time_value, a_emitter_offset_y_interp);

            // 对应 C++: 动画帧/旋转初值
            a_particle.animation_time_value = 0.0;
            if def.animated != 0 || !def.animation_rate.nodes.is_empty() {
                a_particle.image_frame = 0; // 动画粒子的帧稍后从时间值计算
            } else {
                a_particle.image_frame = RandRange(def.image_frames); // 固定帧粒子：随机选一帧
            }
            let a_flags = def.particle_flags as u32;
            if crate::lawn::zombie::test_bit(a_flags, ParticleFlags::RandomLaunchSpin as u32) {
                a_particle.spin_position = RandFloat(2.0 * std::f32::consts::PI);
            } else if crate::lawn::zombie::test_bit(a_flags, ParticleFlags::AlignLaunchSpin as u32) {
                a_particle.spin_position = a_launch_angle;
            } else {
                a_particle.spin_position = 0.0;
            }
            a_particle.spin_velocity = 0.0;
            a_particle.cross_fade_duration = 0;
            a_particle.cross_fade_particle_id = PARTICLEID_NULL;

            // 对应 C++: ParticleID aParticleID = DataArrayGetID(aParticle);
            let a_particle_id = a_data_array.get_id(a_particle);
            // 对应 C++: mParticleList.AddHead(aParticleID); mParticlesSpawned++;
            self.particle_list.add_head_value(a_particle_id);
            self.particles_spawned += 1;
            // 对应 C++: UpdateParticle(aParticle);
            self.update_particle(a_particle);
            Some(a_particle)
        }
    }
    /// 删除粒子（对应 C++ PvzpParticleEmitter::DeleteParticle，:770）
    pub fn delete_particle(&mut self, the_particle: *mut TodParticle) {
        unsafe {
            // 对应 C++: 递归删除淡出源粒子
            let a_cross_fade_id = (*the_particle).cross_fade_particle_id;
            if a_cross_fade_id != PARTICLEID_NULL {
                if !self.particle_system.is_null() {
                    let holder = (*self.particle_system).particle_holder;
                    if !holder.is_null() {
                        let a_cross_fade_opt = (*holder).particles.try_to_get(a_cross_fade_id);
                        if let Some(a_cross_fade_ref) = a_cross_fade_opt {
                            let a_cross_emitter = a_cross_fade_ref.particle_emitter;
                            if !a_cross_emitter.is_null() {
                                (*a_cross_emitter).delete_particle(
                                    a_cross_fade_ref as *const TodParticle as *mut TodParticle,
                                );
                            }
                            (*the_particle).cross_fade_particle_id = PARTICLEID_NULL;
                        }
                    }
                }
            }

            // 对应 C++: 从粒子链表移除并释放
            let holder = (*self.particle_system).particle_holder;
            if !holder.is_null() {
                let a_particle_id = (*holder).particles.get_id(the_particle);
                if let Some(a_node) = self.particle_list.find(a_particle_id) {
                    self.particle_list.remove(a_node);
                }
                (*holder).particles.free(the_particle);
            }
        }
    }

    /// 删除全部粒子（对应 C++ PvzpParticleEmitter::DeleteAll，:712）
    pub fn delete_all(&mut self) {
        while self.particle_list.count != 0 {
            let an_id = self.particle_list.remove_head_value().unwrap_or(ParticleID::MAX);
            unsafe {
                if !self.particle_system.is_null() {
                    let holder = (*self.particle_system).particle_holder;
                    if !holder.is_null() {
                        let a_particle_ptr = match (*holder).particles.try_to_get_mut(an_id) {
                            Some(p) => p as *mut TodParticle,
                            None => std::ptr::null_mut(),
                        };
                        if !a_particle_ptr.is_null() {
                            (*holder).particles.free(a_particle_ptr);
                        }
                    }
                }
            }
        }
    }
}

// ============================================================
// TodParticleSystem（粒子系统实例）
// ============================================================

/// 粒子系统运行时实例（对应 C++ TodParticleSystem）
pub struct TodParticleSystem {
    pub effect_type: crate::lawn::game_enums::ParticleEffect,
    pub particle_def: *mut TodParticleDefinition,
    pub particle_holder: *mut TodParticleHolder,
    pub emitter_list: TodList<ParticleEmitterID>,
    pub dead: bool,
    pub is_attachment: bool,
    pub render_order: i32,
    pub dont_update: bool,
    // 简化生命周期：系统年龄（帧），达到默认时长后消亡
    pub system_age: i32,
}

impl TodParticleSystem {
    pub fn new() -> Self {
        TodParticleSystem {
            effect_type: crate::lawn::game_enums::ParticleEffect::None,
            particle_def: ptr::null_mut(),
            particle_holder: ptr::null_mut(),
            emitter_list: TodList::new(),
            dead: false,
            is_attachment: false,
            render_order: 0,
            dont_update: false,
            system_age: 0,
        }
    }

    /// 从定义初始化粒子系统（对应 C++ PvzpParticleInitializeFromDef，PvzpParticle.cpp:257）
    pub fn pvzp_particle_initialize_from_def(
        &mut self,
        x: f32,
        y: f32,
        render_order: i32,
        definition: *mut TodParticleDefinition,
        effect: crate::lawn::game_enums::ParticleEffect,
    ) {
        self.particle_def = definition;
        self.effect_type = effect;
        self.render_order = render_order;
        let def = unsafe { &*definition };
        for a_emitter_def in &def.emitter_defs {
            // 对应 C++: 设置了 cross-fade 时长的发射器不在此创建（由淡出时生成）
            if a_emitter_def.cross_fade_duration.nodes.is_empty() {
                // 对应 C++: PARTICLE_DIE_IF_OVERLOADED + 过载 → 系统死亡
                if crate::lawn::zombie::test_bit(a_emitter_def.particle_flags as u32, ParticleFlags::DieIfOverloaded as u32)
                    && unsafe { (*self.particle_holder).is_overloaded() }
                {
                    self.particle_system_die();
                    break;
                }
                unsafe {
                    let holder = &mut *self.particle_holder;
                    let a_emitter = holder.emitters.alloc();
                    (*a_emitter).tod_emitter_initialize(
                        x,
                        y,
                        self as *mut TodParticleSystem,
                        a_emitter_def as *const TodEmitterDefinition as *mut TodEmitterDefinition,
                    );
                    let a_id = holder.emitters.get_id(a_emitter);
                    // 对应 C++: mEmitterList.AddTail(aEmitterID)
                    self.emitter_list.push_back_value(a_id);
                }
            }
        }
    }

    pub fn particle_system_die(&mut self) {
        // 对应 C++ ParticleSystemDie（PvzpParticle.cpp:319）：清空所有发射器与粒子
        if self.particle_holder.is_null() {
            self.dead = true;
            return;
        }
        let a_emitter_ids: Vec<ParticleEmitterID> = {
            let mut ids = Vec::new();
            let mut node = self.emitter_list.head;
            while !node.is_null() {
                unsafe {
                    ids.push((*node).value);
                    node = (*node).next;
                }
            }
            ids
        };
        unsafe {
            let holder = &mut *self.particle_holder;
            for a_id in a_emitter_ids {
                let a_emitter_ptr = holder.emitters.get_mut(a_id) as *mut TodParticleEmitter;
                (*a_emitter_ptr).delete_all();
                holder.emitters.free(a_emitter_ptr);
            }
        }
        // 对应 C++: mEmitterList.RemoveAll()
        while self.emitter_list.count > 0 {
            let _ = self.emitter_list.remove_head_value();
        }
        self.dead = true;
    }

    /// 更新粒子系统（对应 C++ PvzpParticleSystem::Update，PvzpParticle.cpp:722）
    pub fn update(&mut self) {
        if self.dont_update { return; }
        if self.particle_holder.is_null() {
            self.dead = true;
            return;
        }
        let a_emitter_ids: Vec<ParticleEmitterID> = {
            let mut ids = Vec::new();
            let mut node = self.emitter_list.head;
            while !node.is_null() {
                unsafe {
                    ids.push((*node).value);
                    node = (*node).next;
                }
            }
            ids
        };
        let mut a_emitter_alive = false;
        unsafe {
            let holder = &mut *self.particle_holder;
            for a_emitter_id in a_emitter_ids {
                let a_emitter = &mut *holder.emitters.get_mut(a_emitter_id);
                a_emitter.update();
                // 对应 C++: 有 cross-fade 时长且仍有粒子，或未死亡的发射器视作存活
                let a_has_cross_fade_duration = !(*a_emitter.emitter_def).cross_fade_duration.nodes.is_empty();
                if (a_has_cross_fade_duration && a_emitter.particle_list.count > 0) || !a_emitter.dead {
                    a_emitter_alive = true;
                }
            }
        }
        if !a_emitter_alive {
            self.dead = true;
        }
    }

    /// 绘制粒子系统（对应 C++ PvzpParticleSystem::Draw，PvzpParticle.cpp:1054）
    /// 遍历发射器列表，逐个绘制（C++ 经 mParticleHolder->mEmitters.DataArrayGet(id)->Draw）
    pub fn draw(&self, g: &mut Graphics) {
        let mut node = self.emitter_list.head;
        while !node.is_null() {
            unsafe {
                let a_emitter_id = (*node).value;
                let holder = self.particle_holder;
                if !holder.is_null() {
                    let a_emitter = (*holder).emitters.get(a_emitter_id);
                    if !a_emitter.dead {
                        a_emitter.draw(g);
                    }
                }
                node = (*node).next;
            }
        }
    }

    /// 移动系统中心（对应 C++ PvzpParticleSystem::SystemMove，:1073）
    pub fn system_move(&mut self, x: f32, y: f32) {
        if self.particle_holder.is_null() {
            return;
        }
        let a_emitter_ids: Vec<ParticleEmitterID> = {
            let mut ids = Vec::new();
            let mut node = self.emitter_list.head;
            while !node.is_null() {
                unsafe {
                    ids.push((*node).value);
                    node = (*node).next;
                }
            }
            ids
        };
        unsafe {
            let holder = &mut *self.particle_holder;
            for a_emitter_id in a_emitter_ids {
                holder.emitters.get_mut(a_emitter_id).system_move(x, y);
            }
        }
    }
    /// [TRANSLATION_NOTE]: C++ PvzpParticleSystem::OverrideColor (PvzpParticle.cpp:1099)
    /// iterate emitter list; empty name == C++ nullptr (match all)
    pub fn override_color(&mut self, the_emitter_name: &str, the_color: &Color) {
        let mut a_node = self.emitter_list.head;
        while !a_node.is_null() {
            unsafe {
                let a_emitter_id = (*a_node).value;
                let a_emitter = (&mut *self.particle_holder).emitters.get_mut(a_emitter_id);
                let a_matches = the_emitter_name.is_empty()
                    || a_emitter.emitter_def.as_ref().map_or(false, |d| {
                        (*d).name.eq_ignore_ascii_case(the_emitter_name)
                    });
                if a_matches {
                    a_emitter.color_override = *the_color;
                }
                a_node = (*a_node).next;
            }
        }
    }
    /// [TRANSLATION_NOTE]: C++ PvzpParticleSystem::OverrideExtraAdditiveDraw (1109)
    pub fn override_extra_additive_draw(&mut self, the_emitter_name: &str, the_enable: bool) {
        let mut a_node = self.emitter_list.head;
        while !a_node.is_null() {
            unsafe {
                let a_emitter_id = (*a_node).value;
                let a_emitter = (&mut *self.particle_holder).emitters.get_mut(a_emitter_id);
                let a_matches = the_emitter_name.is_empty()
                    || a_emitter.emitter_def.as_ref().map_or(false, |d| {
                        (*d).name.eq_ignore_ascii_case(the_emitter_name)
                    });
                if a_matches {
                    a_emitter.extra_additive_draw_override = the_enable;
                }
                a_node = (*a_node).next;
            }
        }
    }
    /// [TRANSLATION_NOTE]: C++ PvzpParticleSystem::OverrideImage (1119)
    pub fn override_image(&mut self, the_emitter_name: &str, the_image: *mut Image) {
        let mut a_node = self.emitter_list.head;
        while !a_node.is_null() {
            unsafe {
                let a_emitter_id = (*a_node).value;
                let a_emitter = (&mut *self.particle_holder).emitters.get_mut(a_emitter_id);
                let a_matches = the_emitter_name.is_empty()
                    || a_emitter.emitter_def.as_ref().map_or(false, |d| {
                        (*d).name.eq_ignore_ascii_case(the_emitter_name)
                    });
                if a_matches {
                    a_emitter.image_override = the_image;
                }
                a_node = (*a_node).next;
            }
        }
    }
    /// [TRANSLATION_NOTE]: C++ PvzpParticleSystem::OverrideFrame (1129)
    pub fn override_frame(&mut self, the_emitter_name: &str, the_frame: i32) {
        let mut a_node = self.emitter_list.head;
        while !a_node.is_null() {
            unsafe {
                let a_emitter_id = (*a_node).value;
                let a_emitter = (&mut *self.particle_holder).emitters.get_mut(a_emitter_id);
                let a_matches = the_emitter_name.is_empty()
                    || a_emitter.emitter_def.as_ref().map_or(false, |d| {
                        (*d).name.eq_ignore_ascii_case(the_emitter_name)
                    });
                if a_matches {
                    a_emitter.frame_override = the_frame;
                }
                a_node = (*a_node).next;
            }
        }
    }
    /// [TRANSLATION_NOTE]: C++ PvzpParticleSystem::OverrideScale (1139)
    pub fn override_scale(&mut self, the_emitter_name: &str, the_scale: f32) {
        let mut a_node = self.emitter_list.head;
        while !a_node.is_null() {
            unsafe {
                let a_emitter_id = (*a_node).value;
                let a_emitter = (&mut *self.particle_holder).emitters.get_mut(a_emitter_id);
                let a_matches = the_emitter_name.is_empty()
                    || a_emitter.emitter_def.as_ref().map_or(false, |d| {
                        (*d).name.eq_ignore_ascii_case(the_emitter_name)
                    });
                if a_matches {
                    a_emitter.scale_override = the_scale;
                }
                a_node = (*a_node).next;
            }
        }
    }
    /// [TRANSLATION_NOTE]: C++ PvzpParticleSystem::CrossFade (1196)
    /// depends on FloatTrackIsSet / emitter alloc backbone (untranslated), keep empty
    /// 系统级淡出（对应 C++ PvzpParticleSystem::CrossFade，PvzpParticle.cpp:1196）
    pub fn cross_fade(&mut self, emitter_name: &str) {
        // 对应 C++: 按名称查找目标发射器定义
        let a_emitter_def_ptr: *mut TodEmitterDefinition = match self.find_emitter_def_by_name(emitter_name) {
            Some(d) => d as *const TodEmitterDefinition as *mut TodEmitterDefinition,
            None => {
                return;
            }
        };
        // 对应 C++: 目标发射器必须设置了 cross-fade 时长
        let a_has_cross_fade_duration = unsafe { !(*a_emitter_def_ptr).cross_fade_duration.nodes.is_empty() };
        if !a_has_cross_fade_duration {
            return;
        }
        // 对应 C++: 发射器数量上限（当前 + 淡出新增）→ 系统死亡
        unsafe {
            let holder = &mut *self.particle_holder;
            if holder.emitters.len() as u32 + self.emitter_list.count as u32 > holder.emitters.max_size() {
                self.particle_system_die();
                return;
            }
        }
        // 遍历当前发射器，非同类的创建淡出目标
        let a_emitter_ids: Vec<ParticleEmitterID> = {
            let mut ids = Vec::new();
            let mut node = self.emitter_list.head;
            while !node.is_null() {
                unsafe {
                    ids.push((*node).value);
                    node = (*node).next;
                }
            }
            ids
        };
        for a_emitter_id in a_emitter_ids {
            unsafe {
                let holder = &mut *self.particle_holder;
                let a_same_kind = holder.emitters.get(a_emitter_id).emitter_def == a_emitter_def_ptr;
                if a_same_kind {
                    continue;
                }
                let a_system_center = holder.emitters.get(a_emitter_id).system_center;
                // 对应 C++: 分配淡出目标发射器
                let a_cross_fade_emitter = holder.emitters.alloc();
                (*a_cross_fade_emitter).tod_emitter_initialize(
                    a_system_center.x,
                    a_system_center.y,
                    self as *mut TodParticleSystem,
                    a_emitter_def_ptr,
                );
                let a_cross_fade_id = holder.emitters.get_id(a_cross_fade_emitter);
                self.emitter_list.push_back_value(a_cross_fade_id);
                // 对应 C++: aEmitter->CrossFadeEmitter(aCrossFadeEmitter)
                holder.emitters.get_mut(a_emitter_id).cross_fade_emitter(a_cross_fade_emitter);
            }
        }
    }
    /// [TRANSLATION_NOTE]: C++ PvzpParticleSystem::FindEmitterByName (1149)
    pub fn find_emitter_by_name(&self, the_name: &str) -> Option<&TodParticleEmitter> {
        let mut a_node = self.emitter_list.head;
        while !a_node.is_null() {
            unsafe {
                let a_emitter_id = (*a_node).value;
                let a_emitter = (*self.particle_holder).emitters.get(a_emitter_id);
                let a_matches = a_emitter.emitter_def.as_ref().map_or(false, |d| {
                    (*d).name.eq_ignore_ascii_case(the_name)
                });
                if a_matches {
                    return Some(a_emitter);
                }
                a_node = (*a_node).next;
            }
        }
        None
    }
    /// [TRANSLATION_NOTE]: C++ PvzpParticleSystem::FindEmitterDefByName (1160)
    pub fn find_emitter_def_by_name(&self, the_name: &str) -> Option<&TodEmitterDefinition> {
        let a_def = self.particle_def;
        unsafe {
            if !a_def.is_null() {
                for a_emitter_def in &(*a_def).emitter_defs {
                    if a_emitter_def.name.eq_ignore_ascii_case(the_name) {
                        return Some(a_emitter_def);
                    }
                }
            }
        }
        None
    }
}

// ============================================================
// 全局函数
// ============================================================

/// 加载单个粒子定义（对应 C++ TodParticleLoadADef，PvzpParticle.cpp:145）
pub fn tod_particle_load_a_def(def: &mut TodParticleDefinition, _file: &str) -> bool {
    // [TRANSLATION_NOTE]: C++ 的 DefinitionLoadXML（XML 解析）在 Rust 端暂未实现，
    // 定义由调用方/后续轮次的解析器填充；此处完成 C++ 的 FloatTrackSetDefault 默认值阶段。
    for a_emitter_def in def.emitter_defs.iter_mut() {
        a_emitter_def.system_duration.set_default(0.0);
        a_emitter_def.spawn_rate.set_default(0.0);
        a_emitter_def.spawn_min_active.set_default(-1.0);
        a_emitter_def.spawn_max_active.set_default(-1.0);
        a_emitter_def.spawn_max_launched.set_default(-1.0);
        a_emitter_def.emitter_radius.set_default(0.0);
        a_emitter_def.emitter_offset_x.set_default(0.0);
        a_emitter_def.emitter_offset_y.set_default(0.0);
        a_emitter_def.emitter_box_x.set_default(0.0);
        a_emitter_def.emitter_box_y.set_default(0.0);
        a_emitter_def.emitter_skew_x.set_default(0.0);
        a_emitter_def.emitter_skew_y.set_default(0.0);
        a_emitter_def.particle_duration.set_default(100.0);
        a_emitter_def.launch_speed.set_default(0.0);
        a_emitter_def.system_red.set_default(1.0);
        a_emitter_def.system_green.set_default(1.0);
        a_emitter_def.system_blue.set_default(1.0);
        a_emitter_def.system_alpha.set_default(1.0);
        a_emitter_def.system_brightness.set_default(1.0);
        a_emitter_def.launch_angle.set_default(0.0);
        a_emitter_def.cross_fade_duration.set_default(0.0);
        a_emitter_def.particle_red.set_default(1.0);
        a_emitter_def.particle_green.set_default(1.0);
        a_emitter_def.particle_blue.set_default(1.0);
        a_emitter_def.particle_alpha.set_default(1.0);
        a_emitter_def.particle_brightness.set_default(1.0);
        a_emitter_def.particle_spin_angle.set_default(0.0);
        a_emitter_def.particle_spin_speed.set_default(0.0);
        a_emitter_def.particle_scale.set_default(1.0);
        a_emitter_def.particle_stretch.set_default(1.0);
        a_emitter_def.collision_reflect.set_default(0.0);
        a_emitter_def.collision_spin.set_default(0.0);
        a_emitter_def.clip_top.set_default(0.0);
        a_emitter_def.clip_bottom.set_default(0.0);
        a_emitter_def.clip_left.set_default(0.0);
        a_emitter_def.clip_right.set_default(0.0);
        a_emitter_def.animation_rate.set_default(0.0);
    }
    true
}

/// 加载所有粒子定义（对应 C++ TodParticleLoadDefinitions，:204）
pub fn tod_particle_load_definitions(params: &[ParticleParams]) {
    unsafe {
        if !G_PARTICLE_DEF_ARRAY.is_empty() {
            return;
        }
        let mut a_def_array: Vec<TodParticleDefinition> = Vec::with_capacity(params.len());
        for (i, a_param) in params.iter().enumerate() {
            let mut a_def = TodParticleDefinition::new();
            tod_particle_load_a_def(&mut a_def, &a_param.file_name);
            // [TRANSLATION_NOTE]: 校验 aParam.mParticleEffect == i 依赖调用方参数表顺序
            a_def_array.push(a_def);
        }
        G_PARTICLE_DEF_ARRAY = a_def_array;
    }
}

/// 释放粒子定义（对应 C++ TodParticleFreeDefinitions，:229）
pub fn tod_particle_free_definitions() {
    unsafe {
        G_PARTICLE_DEF_ARRAY.clear();
    }
}

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
