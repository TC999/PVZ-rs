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

    /// 更新子弹（对应 C++ Projectile::Update）
    pub fn update(&mut self) {
        if self.dead { return; }

        self.projectile_age += 1;
        // [TRANSLATION_NOTE]: Scene/UpdateBoard 游戏场景检查暂略

        self.rotation += self.rotation_speed;

        self.update_motion();
        // [TRANSLATION_NOTE]: AttachmentUpdateAndMove 暂未实现
    }

    /// 更新子弹运动（对应 C++ Projectile::UpdateMotion）
    pub fn update_motion(&mut self) {
        // 动画帧更新
        if self.anim_ticks_per_frame > 0 {
            self.anim_counter = (self.anim_counter + 1) % (self.num_frames * self.anim_ticks_per_frame);
            self.frame = self.anim_counter / self.anim_ticks_per_frame;
        }

        let a_old_row = self.base.row;
        let a_old_y = self.pos_y; // 简化：GetPosYBasedOnRow

        // 运动类型分发
        self.update_normal_motion();

        // 碰撞检测
        self.check_for_collision();

        // 坡度高度变化
        // [TRANSLATION_NOTE]: 坡度高度变化暂简化处理
        self.base.x = self.pos_x as i32;
        self.base.y = (self.pos_y + self.pos_z) as i32;
    }

    /// 更新正常运动（对应 C++ Projectile::UpdateNormalMotion）
    pub fn update_normal_motion(&mut self) {
        match self.motion {
            ProjectileMotion::Lobbed => {
                // [TRANSLATION_NOTE]: 抛物线运动暂简化，后续实现 UpdateLobMotion
                self.pos_x += 3.33;
            }
            ProjectileMotion::Floating => {
                self.pos_x += 0.4;
            }
            ProjectileMotion::Threepeater => {
                self.pos_x += 3.33;
                self.pos_y += self.vel_y;
                self.vel_y *= 0.97;
                self.shadow_y += self.vel_y;
            }
            ProjectileMotion::Star => {
                self.pos_x += self.vel_x;
                self.pos_y += self.vel_y;
                self.shadow_y += self.vel_y;
                if self.vel_y != 0.0 {
                    // [TRANSLATION_NOTE]: PixelToGridYKeepOnBoard 暂未实现
                }
            }
            _ => {
                self.pos_x += 3.33;
            }
        }

        // [TRANSLATION_NOTE]: HighGravity 模式暂略
        // [TRANSLATION_NOTE]: CheckForCollision + CheckForHighGround 暂略
    }

    /// 子弹死亡（对应 C++ Projectile::Die）
    pub fn die(&mut self) {
        self.dead = true;
        // [TRANSLATION_NOTE]: AttachmentCrossFade/AttachmentDie 暂未实现
    }

    /// 检查碰撞（对应 C++ Projectile::CheckForCollision 简化版）
    pub fn check_for_collision(&mut self) {
        let my_pos_x = self.pos_x;
        let my_pos_y = self.pos_y;
        let my_width = self.base.width;
        let proj_type = self.projectile_type;
        let motion = self.motion;
        let proj_age = self.projectile_age;
        let my_row = self.base.row;

        // 生命周期检查：Puff 75帧
        if motion == ProjectileMotion::Floating && proj_age >= 75 {
            self.die();
            return;
        }

        // 飞出屏幕
        if my_pos_x > 900.0 || (my_pos_x + my_width as f32) < 0.0 {
            self.die();
            return;
        }

        // 星星飞出垂直范围
        if proj_type == ProjectileType::Star && (my_pos_y > 600.0 || my_pos_y < 40.0) {
            self.die();
            return;
        }

        // 查找碰撞僵尸（在 board 借用中查找索引，退出借用后处理）
        let hit_zombie_idx = {
            if let Some(board) = self.base.get_board() {
                let proj_rect = Rect::new(my_pos_x as i32 - 5, my_pos_y as i32 - 5, 10, 10);
                let mut found = None;
                for (idx, zombie) in board.zombies.iter().enumerate() {
                    if zombie.dead { continue; }
                    if zombie.zombie_type == ZombieType::Boss || zombie.base.row == my_row {
                        if zombie.is_dead_or_dying() { continue; }
                        let z_rect = zombie.get_zombie_rect();
                        if crate::lawn::board::get_rect_overlap(&proj_rect, &z_rect) >= 0 {
                            found = Some(idx);
                            break;
                        }
                    }
                }
                found
            } else {
                None
            }
        };

        if let Some(zombie_idx) = hit_zombie_idx {
            self.do_impact_by_index(zombie_idx);
        }
    }

    /// 通过索引对僵尸造成碰撞效果（对应 C++ DoImpact 主体）
    pub fn do_impact_by_index(&mut self, zombie_idx: usize) {
        if let Some(board) = self.base.get_board_mut() {
            if let Some(zombie) = board.zombies.get_mut(zombie_idx) {
                let a_damage = self.damage;
                let a_damage_flags = self.damage_flags;
                zombie.take_damage(a_damage, a_damage_flags);
            }
        }
        // [TRANSLATION_NOTE]: 溅射伤害/粒子效果暂略
        self.die();
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

    /// 产生溅射伤害（对应 C++ DoSplashDamage）
    pub fn do_splash_damage(&mut self, the_zombie_idx: Option<usize>) {
        // [TRANSLATION_NOTE]: 溅射伤害完整实现依赖遍历 board.zombies + 伤害计算
        // 简化版：直接对目标僵尸造成全额伤害，对其他僵尸造成 1/3 伤害
        let a_original_damage = self.damage;
        let a_splash_damage = (a_original_damage / 3).max(1);
        let a_damage_flags = self.damage_flags;
        let my_pos_x = self.pos_x as i32;
        let my_pos_y = self.pos_y as i32;
        let my_row = self.base.row;
        let proj_type = self.projectile_type;

        if let Some(board) = self.base.get_board_mut() {
            for (idx, zombie) in board.zombies.iter_mut().enumerate() {
                if zombie.dead { continue; }
                let a_row_deviation = zombie.base.row - my_row;
                if proj_type == ProjectileType::Melon {
                    if a_row_deviation > 1 || a_row_deviation < -1 {
                        continue;
                    }
                }
                let z_rect = zombie.get_zombie_rect();
                let splash_rect = Rect::new(my_pos_x - 5, my_pos_y - 5, 10, 10);
                if crate::lawn::board::get_rect_overlap(&splash_rect, &z_rect) >= 0 {
                    if Some(idx) == the_zombie_idx {
                        zombie.take_damage(a_original_damage, a_damage_flags);
                    } else {
                        zombie.take_damage(a_splash_damage, a_damage_flags);
                    }
                }
            }
        }
    }

    /// 是否是溅射伤害类型（对应 C++ IsSplashDamage）
    pub fn is_splash_damage(&self) -> bool {
        // [TRANSLATION_NOTE]: Fireball 类型在 Rust 中尚不存在 PROJECTILE_FIREBALL
        self.projectile_type == ProjectileType::Melon
    }

    /// 僵尸是否被溅射击中（对应 C++ IsZombieHitBySplash）
    pub fn is_zombie_hit_by_splash(&self, zombie_idx: usize) -> bool {
        if let Some(board) = self.base.get_board() {
            if let Some(zombie) = board.zombies.get(zombie_idx) {
                let my_pos_x = self.pos_x as i32;
                let my_pos_y = self.pos_y as i32;
                let my_width = self.base.width;
                let mut splash_rect = Rect::new(my_pos_x - 5, my_pos_y - 5, 10, 10);
                // [TRANSLATION_NOTE]: Fireball 溅射半径 100
                let a_row_deviation = zombie.base.row - self.base.row;
                if self.projectile_type == ProjectileType::Melon {
                    if a_row_deviation > 1 || a_row_deviation < -1 {
                        return false;
                    }
                }
                let z_rect = zombie.get_zombie_rect();
                return crate::lawn::board::get_rect_overlap(&splash_rect, &z_rect) >= 0;
            }
        }
        false
    }

    /// 豌豆是否即将击中火炬树桩（对应 C++ PeaAboutToHitTorchwood 简化版）
    pub fn pea_about_to_hit_torchwood(&self) -> bool {
        // [TRANSLATION_NOTE]: 火炬树桩碰撞检测依赖植物遍历，暂未实现
        false
    }

    /// 转换为火球（对应 C++ ConvertToFireball）
    pub fn convert_to_fireball(&mut self, grid_x: i32) {
        if self.hit_torchwood_grid_x == grid_x {
            return;
        }
        self.hit_torchwood_grid_x = grid_x;
        // [TRANSLATION_NOTE]: 火球类型在 Rust 的 ProjectileType 中尚不存在
        // 音效 + 火球动画 Reanimation 暂未实现
    }

    /// 转换回豌豆（对应 C++ ConvertToPea）
    pub fn convert_to_pea(&mut self, grid_x: i32) {
        if self.hit_torchwood_grid_x == grid_x {
            return;
        }
        // [TRANSLATION_NOTE]: AttachmentDie 暂未实现
        self.projectile_type = ProjectileType::Pea;
        self.hit_torchwood_grid_x = grid_x;
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
