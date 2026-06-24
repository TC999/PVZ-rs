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
    pub reanimations: Vec<Reanimation>,
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

    /// 添加动画
    pub fn add_reanimation(&mut self, reanim: Reanimation) -> ReanimationID {
        let id = self.reanimations.len() as ReanimationID;
        self.reanimations.push(reanim);
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
