// PvZ Portable Rust 翻译 — Projectile（子弹/投射物）
// 对应 C++ src/Lawn/Projectile.h / Projectile.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ProjectileMotion {
    Straight = 0,
    Lobbed,
    Floating,
    Threepeater,
    Star,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ProjectileFlags {
    None = 0,
    Glow = 1,
    Ice = 2,
    Butter = 4,
    Fire = 8,
    Acid = 16,
    PlantHead = 32,
    Stinky = 64,
    Cross = 128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ProjectileType {
    Pea = 0,
    Snowpea,
    Cabbage,
    Melon,
    Kernel,
    Butter,
    Stinky,
    Spore,
    Star,
    Spike,
    Cactus,
    Guod,
    Cobcannon,
}

/// 子弹/投射物
pub struct Projectile {
    pub base: GameObject,

    pub projectile_type: ProjectileType,
    pub motion: ProjectileMotion,
    pub pos_x: f32,
    pub pos_y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub damage: i32,
    pub damage_flags: u32,
    pub target_x: f32,
    pub target_y: f32,
    pub frame: i32,
    pub num_frames: i32,
    pub anim_counter: i32,
    pub projectile_flags: u32,
    pub from_cobcannon: bool,
    pub dead: bool,
    pub shadow: bool,
}

impl Projectile {
    pub fn new() -> Self {
        Projectile {
            base: GameObject::new(),
            projectile_type: ProjectileType::Pea,
            motion: ProjectileMotion::Straight,
            pos_x: 0.0,
            pos_y: 0.0,
            vel_x: 6.0,
            vel_y: 0.0,
            damage: 20,
            damage_flags: 0,
            target_x: 0.0,
            target_y: 0.0,
            frame: 0,
            num_frames: 1,
            anim_counter: 0,
            projectile_flags: 0,
            from_cobcannon: false,
            dead: false,
            shadow: false,
        }
    }

    /// 初始化子弹
    pub fn projectile_initialize(&mut self, x: f32, y: f32, row: i32, seed_type: SeedType) {
        self.pos_x = x;
        self.pos_y = y;
        self.base.row = row;

        match seed_type {
            SeedType::Peashooter | SeedType::Repeater | SeedType::Gatlingpea |
            SeedType::Threepeater | SeedType::Splitpea => {
                self.projectile_type = ProjectileType::Pea;
                self.motion = ProjectileMotion::Straight;
                self.vel_x = 6.0;
            },
            SeedType::Snowpea => {
                self.projectile_type = ProjectileType::Snowpea;
                self.motion = ProjectileMotion::Straight;
                self.vel_x = 6.0;
                self.damage_flags = 2; // DAMAGE_FREEZE
            },
            SeedType::Cabbagepult => {
                self.projectile_type = ProjectileType::Cabbage;
                self.motion = ProjectileMotion::Lobbed;
                self.vel_x = 3.0;
            },
            SeedType::Melonpult => {
                self.projectile_type = ProjectileType::Melon;
                self.motion = ProjectileMotion::Lobbed;
                self.vel_x = 3.0;
                self.damage = 40;
            },
            SeedType::Kernelpult => {
                self.projectile_type = ProjectileType::Kernel;
                self.motion = ProjectileMotion::Lobbed;
                self.vel_x = 3.0;
                self.damage = 10;
            },
            SeedType::Puffshroom => {
                self.projectile_type = ProjectileType::Spore;
                self.motion = ProjectileMotion::Floating;
                self.vel_x = 3.0;
                self.damage = 10;
            },
            SeedType::Starfruit => {
                self.projectile_type = ProjectileType::Star;
                self.motion = ProjectileMotion::Star;
                self.vel_x = 5.0;
                self.damage = 20;
            },
            SeedType::Cactus => {
                self.projectile_type = ProjectileType::Cactus;
                self.motion = ProjectileMotion::Straight;
                self.vel_x = 6.0;
            },
            _ => {
                self.projectile_type = ProjectileType::Pea;
                self.motion = ProjectileMotion::Straight;
            }
        }
    }

    /// 更新子弹
    pub fn update(&mut self) {
        if self.dead { return; }

        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;

        // 更新动画帧
        self.anim_counter += 1;
        if self.num_frames > 1 && self.anim_counter >= 4 {
            self.frame = (self.frame + 1) % self.num_frames;
            self.anim_counter = 0;
        }

        // 如果飞出屏幕则标记死亡
        if self.pos_x > 900.0 || self.pos_x < -50.0 || self.pos_y < -50.0 || self.pos_y > 650.0 {
            self.dead = true;
        }
    }

    /// 绘制子弹
    pub fn draw(&self, _g: &mut Graphics) {}

    /// 获取子弹矩形
    pub fn get_projectile_rect(&self) -> Rect {
        Rect::new(
            (self.pos_x - 5.0) as i32,
            (self.pos_y - 5.0) as i32,
            10,
            10,
        )
    }

    /// 产生溅射伤害（西瓜/冰瓜）
    pub fn do_splash_damage(&self) {
        // 查找范围内的僵尸并造成范围伤害
    }
}

impl Default for Projectile {
    fn default() -> Self {
        Projectile::new()
    }
}
