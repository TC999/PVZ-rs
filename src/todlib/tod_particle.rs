// PvZ Portable Rust 翻译 — TodParticle（粒子系统）
// 对应 C++ src/Sexy.TodLib/TodParticle.h / TodParticle.cpp

#![allow(dead_code)]

use crate::lawn::game_enums::{ParticleID, ParticleEmitterID, ParticleSystemID, PARTICLESYSTEMID_NULL};
use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::lawn::game_enums::TodCurves;

/// 粒子系统
pub struct ParticleSystem {
    pub id: ParticleSystemID,
    pub particles: Vec<Particle>,
    pub emitters: Vec<ParticleEmitter>,
    pub x: f32,
    pub y: f32,
    pub active: bool,
    pub should_free_on_completion: bool,
}

/// 粒子实例
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub scale: f32,
    pub alpha: f32,
    pub color: Color,
    pub active: bool,
}

/// 粒子发射器
pub struct ParticleEmitter {
    pub x: f32,
    pub y: f32,
    pub particle_count: i32,
    pub emission_rate: f32,
    pub active: bool,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

impl ParticleSystem {
    pub fn new() -> Self {
        ParticleSystem {
            id: PARTICLESYSTEMID_NULL,
            particles: Vec::new(),
            emitters: Vec::new(),
            x: 0.0,
            y: 0.0,
            active: false,
            should_free_on_completion: false,
        }
    }

    /// 更新粒子系统
    pub fn update(&mut self) {
        // 更新发射器
        for emitter in &mut self.emitters {
            if emitter.active {
                // 产生新粒子
            }
        }
        // 更新已有粒子
        self.particles.retain(|p| p.active);
        for particle in &mut self.particles {
            particle.x += particle.vx;
            particle.y += particle.vy;
            particle.lifetime -= 1.0;
            if particle.lifetime <= 0.0 {
                particle.active = false;
            }
            // 更新 alpha/scale 等属性
        }
    }

    /// 绘制粒子
    pub fn draw(&self, _g: &mut Graphics) {
        for particle in &self.particles {
            // 绘制每个粒子
        }
    }

    /// 发射粒子
    pub fn add_particle(&mut self, x: f32, y: f32, vx: f32, vy: f32, lifetime: f32) {
        self.particles.push(Particle {
            x, y, vx, vy,
            lifetime,
            max_lifetime: lifetime,
            scale: 1.0,
            alpha: 1.0,
            color: Color::WHITE,
            active: true,
        });
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        ParticleSystem::new()
    }
}
