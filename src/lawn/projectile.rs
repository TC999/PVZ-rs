// PvZ Portable Rust 翻译 — Projectile（子弹/投射物）
// 对应 C++ src/Lawn/Projectile.h / Projectile.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;
use crate::framework::common::{Rand, RandRange, RandFloat};

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
    pub pos_z: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,
    pub acc_z: f32,
    pub damage: i32,
    pub damage_flags: u32,
    pub target_x: f32,
    pub target_y: f32,
    pub cob_target_x: f32,
    pub cob_target_row: i32,
    pub frame: i32,
    pub num_frames: i32,
    pub anim_counter: i32,
    pub anim_ticks_per_frame: i32,
    pub projectile_flags: u32,
    pub damage_range_flags: u32,
    pub from_cobcannon: bool,
    pub dead: bool,
    pub shadow: bool,
    pub shadow_y: f32,
    pub hit_torchwood_grid_x: i32,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub projectile_age: i32,
    pub click_backoff_counter: i32,
    pub on_high_ground: bool,
    pub target_zombie_id: ZombieID,
    pub attachment_id: AttachmentID,
}

impl Projectile {
    pub fn new() -> Self {
        Projectile {
            base: GameObject::new(),
            projectile_type: ProjectileType::Pea,
            motion: ProjectileMotion::Straight,
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            vel_x: 0.0,
            vel_y: 0.0,
            vel_z: 0.0,
            acc_z: 0.0,
            damage: 20,
            damage_flags: 0,
            target_x: 0.0,
            target_y: 0.0,
            cob_target_x: 0.0,
            cob_target_row: 0,
            frame: 0,
            num_frames: 1,
            anim_counter: 0,
            anim_ticks_per_frame: 0,
            projectile_flags: 0,
            damage_range_flags: 0,
            from_cobcannon: false,
            dead: false,
            shadow: false,
            shadow_y: 0.0,
            hit_torchwood_grid_x: -1,
            rotation: 0.0,
            rotation_speed: 0.0,
            projectile_age: 0,
            click_backoff_counter: 0,
            on_high_ground: false,
            target_zombie_id: ZOMBIEID_NULL,
            attachment_id: ATTACHMENTID_NULL,
        }
    }

    /// 初始化子弹（对应 C++ Projectile::ProjectileInitialize）
    pub fn projectile_initialize(&mut self, x: f32, y: f32, row: i32, seed_type: SeedType) {
        self.projectile_type = match seed_type {
            SeedType::Peashooter | SeedType::Repeater | SeedType::Gatlingpea |
            SeedType::Threepeater | SeedType::Splitpea => ProjectileType::Pea,
            SeedType::Snowpea => ProjectileType::Snowpea,
            SeedType::Cabbagepult => ProjectileType::Cabbage,
            SeedType::Melonpult => ProjectileType::Melon,
            SeedType::Kernelpult => ProjectileType::Kernel,
            SeedType::Puffshroom => ProjectileType::Spore,
            SeedType::Starfruit => ProjectileType::Star,
            SeedType::Cactus => ProjectileType::Cactus,
            _ => ProjectileType::Pea,
        };
        self.pos_x = x;
        self.pos_y = y;
        self.pos_z = 0.0;
        self.vel_x = 0.0;
        self.vel_y = 0.0;
        self.vel_z = 0.0;
        self.acc_z = 0.0;
        self.base.row = row;
        self.hit_torchwood_grid_x = -1;
        self.motion = ProjectileMotion::Straight;
        self.frame = 0;
        self.num_frames = 1;
        self.base.width = 40;
        self.base.height = 40;
        self.projectile_age = 0;
        self.click_backoff_counter = 0;
        self.anim_ticks_per_frame = 0;
        self.dead = false;
        self.attachment_id = ATTACHMENTID_NULL;
        self.target_zombie_id = ZOMBIEID_NULL;
        self.rotation = 0.0;
        self.rotation_speed = 0.0;
        self.damage_range_flags = 0;
        self.cob_target_x = 0.0;
        self.cob_target_row = 0;
        self.from_cobcannon = false;
        self.shadow_y = 0.0;

        // 设置阴影 Y
        if let Some(board) = self.base.get_board() {
            let a_grid_x = board.pixel_to_grid_x_keep_on_board(x as i32, y as i32);
            self.shadow_y = board.grid_to_pixel_y(a_grid_x, row) as f32 + 67.0;
            self.on_high_ground = board.grid_square_type[row as usize][a_grid_x as usize] == GridSquareType::HighGround;
            if board.stage_has_roof() && x < 480.0 {
                self.shadow_y -= 12.0;
            }
        }

        self.base.render_order = RENDER_LAYER_PROJECTILE;
        self.anim_counter = 0;
        self.base.x = self.pos_x as i32;
        self.base.y = self.pos_y as i32;
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

/// 子弹定义表条目（对应 C++ ProjectileDefinition）
/// 存储每种子弹类型的静态属性：类型、图像行索引、伤害值
/// 全局数组 gProjectileDefinition[NUM_PROJECTILES] 将在 Projectile.cpp 主体翻译时实现
#[derive(Debug, Clone, Copy)]
pub struct ProjectileDefinition {
    pub projectile_type: ProjectileType,
    pub image_row: i32,
    pub damage: i32,
}
