// PvZ Portable Rust 翻译 — EffectSystem（特效系统）
// 对应 C++ src/Sexy.TodLib/EffectSystem.h / EffectSystem.cpp

#![allow(dead_code)]

use crate::lawn::game_enums::{ParticleID, ReanimationID, CoinID, ATTACHMENTID_NULL};
use crate::todlib::reanimator::Reanimation;
use crate::todlib::tod_particle::ParticleSystem;
use crate::todlib::tod_particle::MAX_PARTICLES_SIZE;
use crate::todlib::attachment::Attachment;
use crate::todlib::trail::Trail;
use crate::framework::graphics::graphics::Graphics;

/// 特效类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EffectType {
    Particle = 0,
    Trail = 1,
    Reanim = 2,
    Attachment = 3,
    Other = 4,
}

/// 特效系统 — 管理所有粒子、动画、轨迹、附着物
pub struct EffectSystem {
    pub reanimations: Vec<Reanimation>,
    pub particle_systems: Vec<ParticleSystem>,
    pub attachments: Vec<Attachment>,
    /// 对应 C++ mTrailHolder->mTrails（Rust 以 Vec 承载 Trail）
    pub trails: Vec<Trail>,
    /// 粒子发射器/粒子池（对应 C++ mParticleHolder->mEmitters/mParticles；
    /// 粒子系统本体由 particle_systems Vec 承载，emitter 分配经此 holder 完成）
    pub particle_holder: crate::todlib::tod_particle::TodParticleHolder,
}

impl EffectSystem {
    pub fn new() -> Self {
        let mut es = EffectSystem {
            reanimations: Vec::new(),
            particle_systems: Vec::new(),
            attachments: Vec::new(),
            trails: Vec::new(),
            particle_holder: crate::todlib::tod_particle::TodParticleHolder::new(),
        };
        // 对应 C++ EffectSystem 初始化：mParticleHolder->InitializeHolder()
        es.particle_holder.initialize_holder();
        es
    }

    /// 更新所有特效（对应 C++ EffectSystem::Update；
    /// 注意：C++ 中 Update 不回收对象，删除统一由 ProcessDeleteQueue 处理）
    pub fn update(&mut self) {
        // 对应 C++: for Particle（非 attachment）Update；Rust 无 mIsAttachment 区分，直接更新
        for ps in &mut self.particle_systems {
            ps.update();
        }
        // 对应 C++ EffectSystem.cpp:104：for Trail 非 attachment → Update
        for trail in &mut self.trails {
            if !trail.m_is_attachment {
                trail.update();
            }
        }
        for reanim in &mut self.reanimations {
            reanim.update();
        }
    }

    /// 处理删除队列（对应 C++ EffectSystem::ProcessDeleteQueue：回收所有 mDead 对象）
    pub fn process_delete_queue(&mut self) {
        // 对应 C++: 粒子 mDead → DataArrayFree
        self.particle_systems.retain(|ps| !ps.dead);
        // 对应 C++: 轨迹 mDead → DataArrayFree（EffectSystem.cpp:85）
        self.trails.retain(|t| !t.m_dead);
        // 对应 C++: 动画 mDead → DataArrayFree（Rust 以 is_completely_done 近似，与既有判据一致）
        self.reanimations.retain(|r| !r.is_completely_done());
        // 对应 C++: 附着物 mDead → DataArrayFree
        self.attachments.retain(|a| !a.dead);
    }

    /// 绘制所有特效
    pub fn draw(&self, g: &mut Graphics) {
        for reanim in &self.reanimations {
            reanim.draw(g);
        }
        for ps in &self.particle_systems {
            ps.draw(g);
        }
    }

    /// 添加动画
    pub fn add_reanimation(&mut self, reanim: Reanimation) -> ReanimationID {
        let id = self.reanimations.len() as ReanimationID;
        self.reanimations.push(reanim);
        id
    }

    /// 添加粒子系统（id 从 1 开始，0 保留给 PARTICLESYSTEMID_NULL）
    pub fn add_particle_system(&mut self, ps: ParticleSystem) -> ParticleID {
        let id = (self.particle_systems.len() as ParticleID) + 1;
        self.particle_systems.push(ps);
        if let Some(p) = self.particle_systems.last_mut() {
            p.self_id = id;
        }
        id
    }

    /// 从定义分配粒子系统并加入系统数组（对应 C++ PvzpParticleHolder::AllocParticleSystemFromDef）
    /// 系统本体存于 particle_systems Vec；emitter 分配经 particle_holder 完成
    pub fn alloc_particle_system_from_def(
        &mut self,
        x: f32,
        y: f32,
        render_order: i32,
        definition: *mut crate::todlib::tod_particle::TodParticleDefinition,
        effect: crate::lawn::game_enums::ParticleEffect,
    ) -> Option<*mut ParticleSystem> {
        // 对应 C++: 系统数组满
        if self.particle_systems.len() as u32 >= MAX_PARTICLES_SIZE as u32 {
            return None;
        }
        unsafe {
            let def = &*definition;
            // 对应 C++: 定义中的发射器数量 + 现有发射器数超过上限
            if def.emitter_defs.len() as u32 + self.particle_holder.emitters.len() as u32
                > self.particle_holder.emitters.max_size()
            {
                return None;
            }
        }
        let mut a_system = ParticleSystem::new();
        let id = self.add_particle_system(a_system);
        let idx = (id - 1) as usize;
        let a_system_ptr = &mut self.particle_systems[idx] as *mut ParticleSystem;
        unsafe {
            (*a_system_ptr).particle_holder = &mut self.particle_holder as *mut _;
            (*a_system_ptr).pvzp_particle_initialize_from_def(
                x,
                y,
                render_order,
                definition,
                effect,
            );
        }
        Some(a_system_ptr)
    }

    /// 按效果类型分配粒子系统（对应 C++ PvzpParticleHolder::AllocParticleSystem）
    pub fn alloc_particle_system(
        &mut self,
        x: f32,
        y: f32,
        render_order: i32,
        effect: crate::lawn::game_enums::ParticleEffect,
    ) -> Option<*mut ParticleSystem> {
        let a_def = crate::todlib::tod_particle::particle_get_definition(effect)?;
        self.alloc_particle_system_from_def(x, y, render_order, a_def, effect)
    }

    /// 按 ID 获取粒子系统（对应 C++ EffectSystem::ParticleTryToGet 语义）
    /// id=0（NULL）或粒子已回收（dead/self_id 不匹配，含 Vec retain 后索引漂移）时返回 None
    pub fn particle_try_to_get(&mut self, id: u32) -> Option<&mut ParticleSystem> {
        if id == 0 {
            return None;
        }
        self.particle_systems.get_mut((id - 1) as usize)
            .filter(|ps| ps.self_id == id && !ps.dead)
    }

    /// 由指针获取粒子系统 ID（对应 C++ EffectSystem::ParticleGetID 语义）
    pub fn particle_get_id(&self, ptr: *mut ParticleSystem) -> u32 {
        if ptr.is_null() {
            return 0;
        }
        for ps in &self.particle_systems {
            if std::ptr::eq(ps as *const ParticleSystem as *mut ParticleSystem, ptr) {
                return ps.self_id;
            }
        }
        0
    }

    /// 移除动画
    pub fn remove_reanimation(&mut self, _id: ReanimationID) {
        // 找到并移除
    }

    /// 移除粒子系统
    pub fn remove_particle_system(&mut self, _id: ParticleID) {
        // 找到并移除
    }

    /// 清空全部特效（对应 C++ EffectSystemFreeAll）
    pub fn effect_system_free_all(&mut self) {
        self.particle_systems.clear();
        self.reanimations.clear();
        self.attachments.clear();
    }
}

impl Default for EffectSystem {
    fn default() -> Self {
        EffectSystem::new()
    }
}

/// 最大三角形数量
pub const MAX_TRIANGLES: usize = 256;

/// 三角形顶点（对应 C++ TodTriVertex）
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TodTriVertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub color: u32,
}

impl Default for TodTriVertex {
    fn default() -> Self {
        TodTriVertex {
            x: 0.0,
            y: 0.0,
            u: 0.0,
            v: 0.0,
            color: 0,
        }
    }
}

/// 三角形组（对应 C++ TodTriangleGroup）
/// 存储一批需要绘制的三角形，最多 MAX_TRIANGLES 个，每个 3 个顶点
pub struct TodTriangleGroup {
    pub image: Option<*mut Image>,
    pub vert_array: [[TriVertex; 3]; MAX_TRIANGLES],
    pub triangle_count: i32,
    pub draw_mode: i32,
}

impl Default for TodTriangleGroup {
    fn default() -> Self {
        TodTriangleGroup {
            image: None,
            vert_array: unsafe { std::mem::zeroed() },
            triangle_count: 0,
            draw_mode: 0,
        }
    }
}

use crate::framework::graphics::gl_interface::TriVertex;
use crate::framework::graphics::image::Image;
