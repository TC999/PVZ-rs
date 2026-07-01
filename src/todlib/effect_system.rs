// PvZ Portable Rust 翻译 — EffectSystem（特效系统）
// 对应 C++ src/Sexy.TodLib/EffectSystem.h / EffectSystem.cpp

#![allow(dead_code)]

use crate::lawn::game_enums::{ParticleID, ReanimationID, CoinID, ATTACHMENTID_NULL};
use crate::todlib::reanimator::Reanimation;
use crate::todlib::tod_particle::ParticleSystem;
use crate::todlib::attachment::Attachment;
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
    /// 使用 Box<Reanimation> 确保指针稳定，外部代码可通过 raw pointer 直接修改动画属性
    pub reanimations: Vec<Box<Reanimation>>,
    pub particle_systems: Vec<ParticleSystem>,
    pub attachments: Vec<Attachment>,
}

impl EffectSystem {
    pub fn new() -> Self {
        EffectSystem {
            reanimations: Vec::new(),
            particle_systems: Vec::new(),
            attachments: Vec::new(),
        }
    }

    /// 更新所有特效
    pub fn update(&mut self) {
        for reanim in &mut self.reanimations {
            reanim.update();
        }
        for ps in &mut self.particle_systems {
            ps.update();
        }
        // 移除已完成的
        self.reanimations.retain(|r| !r.is_completely_done());
        self.particle_systems.retain(|ps| !ps.dead);
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

    /// 添加动画（index-based ID 版本）
    pub fn add_reanimation(&mut self, reanim: Reanimation) -> ReanimationID {
        let id = self.reanimations.len() as ReanimationID;
        self.reanimations.push(Box::new(reanim));
        id
    }

    /// 添加粒子系统
    pub fn add_particle_system(&mut self, ps: ParticleSystem) -> ParticleID {
        let id = self.particle_systems.len() as ParticleID;
        self.particle_systems.push(ps);
        id
    }

    /// 移除动画
    pub fn remove_reanimation(&mut self, _id: ReanimationID) {
        // 找到并移除
    }

    /// 移除粒子系统
    pub fn remove_particle_system(&mut self, _id: ParticleID) {
        // 找到并移除
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
