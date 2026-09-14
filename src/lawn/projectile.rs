// PvZ Portable Rust 翻译 — Projectile（子弹/投射物）
// 对应 C++ src/Lawn/Projectile.h / Projectile.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;
use crate::framework::common::{Rand, RandRange, RandFloat};
use crate::framework::sexy_matrix::SexyMatrix3;

/// 缩放旋转矩阵（对应 C++ PvzpLib/PvzpCommon.cpp PvzpScaleRotateTransformMatrix，同 zombie.rs 副本）
fn pvzp_scale_rotate_transform_matrix(m: &mut SexyMatrix3, x: f32, y: f32, rad: f32, the_scale_x: f32, the_scale_y: f32) {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ProjectileMotion {
    Straight = 0,      // MOTION_STRAIGHT
    Lobbed = 1,        // MOTION_LOBBED
    Threepeater = 2,   // MOTION_THREEPEATER
    Bee = 3,           // MOTION_BEE
    BeeBackwards = 4,  // MOTION_BEE_BACKWARDS（蜜蜂向后飞，绘制时需镜像）
    Puff = 5,          // MOTION_PUFF（直飞，随时间淡出）
    Backwards = 6,     // MOTION_BACKWARDS（ZombiePea 向左直飞）
    Star = 7,          // MOTION_STAR（斜向）
    Floating = 8,      // MOTION_FLOAT_OVER（慢速漂浮，无碰撞）
    Homing = 9,        // MOTION_HOMING（追踪）
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
    Fireball,
    Wintermelon,
    Puff,
    Basketball,
    ZombiePea,
    Cobbig,
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
    pub last_portal_x: i32,
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
            last_portal_x: -1,
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
        if self.motion == ProjectileMotion::Lobbed {
            self.update_lob_motion();
        } else {
            self.update_normal_motion();
        }

        // 碰撞检测（非 Lobbed 运动走正常碰撞检测，Lobbed 在内部处理）
        if self.motion != ProjectileMotion::Lobbed {
            self.check_for_collision();
        }

        // 坡度高度变化
        // [TRANSLATION_NOTE]: 坡度高度变化暂简化处理
        self.base.x = self.pos_x as i32;
        self.base.y = (self.pos_y + self.pos_z) as i32;
    }

    /// 更新正常运动（对应 C++ Projectile::UpdateNormalMotion，Projectile.cpp:314-380）
    pub fn update_normal_motion(&mut self) {
        match self.motion {
            ProjectileMotion::Floating => {
                // C++: MOTION_FLOAT_OVER —— velZ < 0 时上升趋缓并带旋转
                if self.vel_z < 0.0 {
                    self.vel_z += 0.002;
                    self.vel_z = self.vel_z.min(0.0);
                    self.pos_y += self.vel_z;
                    self.rotation = 0.3 - 0.7 * self.vel_z * std::f32::consts::PI * 0.25;
                }
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
                    // C++: mRow = mBoard->PixelToGridYKeepOnBoard(mPosX, mPosY)
                    if let Some(board) = self.base.get_board() {
                        self.base.row = board.pixel_to_grid_y_keep_on_board(self.pos_x as i32, self.pos_y as i32);
                    }
                }
            }
            // C++: MOTION_BACKWARDS —— ZombiePea 向左直飞（与默认向右镜像）
            ProjectileMotion::Backwards => {
                self.pos_x -= 3.33;
            }
            ProjectileMotion::Homing => {
                // C++: MOTION_HOMING（Projectile.cpp:343-365）—— 追踪目标僵尸
                if let Some(board) = self.base.get_board() {
                    if let Some(a_zombie) = board.zombies.get(self.target_zombie_id as usize) {
                        if a_zombie.effected_by_damage(self.damage_range_flags) {
                            // C++: aTargetCenter(ZombieTargetLeadX(0), aZombieRect.mY + height/2)
                            let a_zombie_rect = a_zombie.get_zombie_rect();
                            let a_target_center_x = a_zombie.zombie_target_lead_x(0.0);
                            let a_target_center_y = (a_zombie_rect.y + a_zombie_rect.height / 2) as f32;
                            // C++: aProjectileCenter(mPosX + width/2, mPosY + height/2)
                            let a_projectile_center_x = self.pos_x + self.base.width as f32 / 2.0;
                            let a_projectile_center_y = self.pos_y + self.base.height as f32 / 2.0;
                            // C++: aToTarget = (aTargetCenter - aProjectileCenter).Normalize()
                            let mut a_to_target_x = a_target_center_x - a_projectile_center_x;
                            let mut a_to_target_y = a_target_center_y - a_projectile_center_y;
                            let a_to_target_len = (a_to_target_x * a_to_target_x + a_to_target_y * a_to_target_y).sqrt();
                            if a_to_target_len > 0.0001 {
                                a_to_target_x /= a_to_target_len;
                                a_to_target_y /= a_to_target_len;
                            }
                            // C++: aMotion(mVelX, mVelY) += aToTarget * (0.001 * mProjectileAge); Normalize; * 2
                            let mut a_motion_x = self.vel_x + a_to_target_x * (0.001 * self.projectile_age as f32);
                            let mut a_motion_y = self.vel_y + a_to_target_y * (0.001 * self.projectile_age as f32);
                            let a_motion_len = (a_motion_x * a_motion_x + a_motion_y * a_motion_y).sqrt();
                            if a_motion_len > 0.0001 {
                                a_motion_x = a_motion_x / a_motion_len * 2.0;
                                a_motion_y = a_motion_y / a_motion_len * 2.0;
                            }
                            self.vel_x = a_motion_x;
                            self.vel_y = a_motion_y;
                            // C++: mRotation = -atan2(mVelY, mVelX)
                            self.rotation = -a_motion_y.atan2(a_motion_x);
                        }
                    }
                }
                self.pos_y += self.vel_y;
                self.pos_x += self.vel_x;
                self.shadow_y += self.vel_y;
                if let Some(board) = self.base.get_board() {
                    self.base.row = board.pixel_to_grid_y_keep_on_board(self.pos_x as i32, self.pos_y as i32);
                }
            }
            ProjectileMotion::Bee => {
                // C++: MOTION_BEE —— 前 60 帧上升，随后直飞
                if self.projectile_age < 60 {
                    self.pos_y -= 0.5;
                }
                self.pos_x += 3.33;
            }
            ProjectileMotion::BeeBackwards => {
                // C++: MOTION_BEE_BACKWARDS —— 前 60 帧上升，随后左飞
                if self.projectile_age < 60 {
                    self.pos_y -= 0.5;
                }
                self.pos_x -= 3.33;
            }
            _ => {
                self.pos_x += 3.33;
            }
        }

        // C++: mApp->mGameMode == GAMEMODE_CHALLENGE_HIGH_GRAVITY 时重力增强（Projectile.cpp:369-376）
        if self
            .base
            .get_app()
            .map_or(false, |app| app.game_mode == GameMode::ChallengeHighGravity)
        {
            if self.motion == ProjectileMotion::Floating {
                self.vel_z += 0.004;
            } else {
                self.vel_z += 0.2;
            }
            self.pos_y += self.vel_z;
        }
        // [TRANSLATION_NOTE]: CheckForHighGround 暂略（碰撞检测 CheckForCollision 已在 update() 调用）
    }

    /// 更新抛物线运动（对应 C++ Projectile::UpdateLobMotion）
    pub fn update_lob_motion(&mut self) {
        // 对应 C++: Cobbig 俯冲到目标后重定位落点
        if self.projectile_type == ProjectileType::Cobbig && self.pos_z < -700.0 {
            self.vel_z = 8.0;
            self.base.row = self.cob_target_row;
            self.pos_x = self.cob_target_x;
            if let Some(board) = self.base.get_board() {
                let a_cob_target_col = board.pixel_to_grid_x_keep_on_board(self.cob_target_x as i32, 0);
                self.pos_y = board.grid_to_pixel_y(a_cob_target_col, self.cob_target_row) as f32;
                self.shadow_y = self.pos_y + 67.0;
            }
            self.rotation = -std::f32::consts::PI / 2.0;
        }

        // 对应 C++: mVelZ += mAccZ;（GAMEMODE_CHALLENGE_HIGH_GRAVITY 二次重力暂略）
        self.vel_z += self.acc_z;
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
        self.pos_z += self.vel_z;

        // 对应 C++: Basketball/Cobbig 上升期不检测碰撞
        let is_rising = self.vel_z < 0.0;
        if is_rising
            && (self.projectile_type == ProjectileType::Basketball
                || self.projectile_type == ProjectileType::Cobbig)
        {
            return;
        }

        // 对应 C++: 弹龄超过 20 后检查上升/落地高度
        if self.projectile_age > 20 {
            if is_rising {
                return;
            }

            // 对应 C++ 各弹种最小碰撞高度
            let mut a_min_collision_z = 0.0;
            match self.projectile_type {
                ProjectileType::Butter => a_min_collision_z = -32.0,
                ProjectileType::Basketball => a_min_collision_z = 60.0,
                ProjectileType::Melon | ProjectileType::Wintermelon => a_min_collision_z = -35.0,
                ProjectileType::Cabbage | ProjectileType::Kernel => a_min_collision_z = -30.0,
                ProjectileType::Cobbig => a_min_collision_z = -60.0,
                _ => {}
            }
            // 对应 C++: 泳池行 +40
            if let Some(board) = self.base.get_board() {
                let a_grid_x = board.pixel_to_grid_x_keep_on_board(self.pos_x as i32, 0);
                if board.is_pool_square(a_grid_x, self.base.row) {
                    a_min_collision_z += 40.0;
                }
            }
            if self.pos_z <= a_min_collision_z {
                return;
            }
        }

        // [TRANSLATION_NOTE]: 对应 C++ FindCollisionTargetPlant（Basketball/ZombiePea 打植物、
        // 低矮植物豁免、伞叶反射）依赖 Plant::Die 链，本轮暂略。

        // 僵尸碰撞检测
        let my_pos_x = self.pos_x as i32;
        let my_pos_y = self.pos_y as i32;
        let my_row = self.base.row;

        let mut hit_zombie_idx: Option<usize> = None;
        if let Some(board) = self.base.get_board() {
            let proj_rect = Rect::new(my_pos_x - 5, my_pos_y - 5, 10, 10);
            for (idx, zombie) in board.zombies.iter().enumerate() {
                if zombie.dead { continue; }
                if zombie.zombie_type == ZombieType::Boss || zombie.base.row == my_row {
                    if zombie.is_dead_or_dying() { continue; }
                    let z_rect = zombie.get_zombie_rect();
                    if crate::lawn::board::get_rect_overlap(&proj_rect, &z_rect) >= 0 {
                        hit_zombie_idx = Some(idx);
                        break;
                    }
                }
            }
        }

        // 对应 C++: PROJECTILE_COBBIG 落点：范围内全部僵尸爆炸伤害
        if self.projectile_type == ProjectileType::Cobbig {
            let a_damage_flags = self.damage_flags;
            if let Some(board) = self.base.get_board_mut() {
                let a_before_gargantuar_count = board.get_live_gargantuar_count();
                board.kill_all_zombies_in_radius(
                    my_row,
                    my_pos_x + 80,
                    my_pos_y + 40,
                    115,
                    1,
                    true,
                    a_damage_flags,
                );
                let a_after_gargantuar_count = board.get_live_gargantuar_count();
                // [TRANSLATION_NOTE]: C++ 中 mGargantuarsKillsByCornCob 累计与 PopcornParty 成就，Rust 字段暂无
                let _gargantuars_killed = a_before_gargantuar_count - a_after_gargantuar_count;
            }
            // 对应 C++ DoImpact(nullptr) COBBIG 分支：BLASTMARK/POPCORNSPLASH 粒子（stub）
            // + PlaySample(SOUND_DOOMSHROOM)（Rust 音效系统用 FoleyType::Explosion 近似）+ ShakeBoard(3, -4)
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Explosion as i32);
            }
            if let Some(board) = self.base.get_board_mut() {
                board.shake_board(3, -4);
            }
            self.die();
            return;
        }

        if let Some(zombie_idx) = hit_zombie_idx {
            self.do_impact_by_index(zombie_idx);
        }
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

        // C++: MOTION_PUFF && mProjectileAge >= 75 → Die（Projectile.cpp:340）
        if motion == ProjectileMotion::Puff && proj_age >= 75 {
            self.die();
            return;
        }

        // C++: mPosX > WIDE_BOARD_WIDTH || mPosX + mWidth < 0 → Die（:343）
        if my_pos_x > WIDE_BOARD_WIDTH as f32 || (my_pos_x + my_width as f32) < 0.0 {
            self.die();
            return;
        }

        // C++: MOTION_HOMING —— 追踪目标僵尸的专用碰撞（:347-360）
        if motion == ProjectileMotion::Homing {
            if let Some(board) = self.base.get_board() {
                if let Some(a_zombie) = board.zombies.get(self.target_zombie_id as usize) {
                    if a_zombie.effected_by_damage(self.damage_range_flags) {
                        let a_projectile_rect = self.get_projectile_rect();
                        let a_zombie_rect = a_zombie.get_zombie_rect();
                        // C++: GetRectOverlap(aProjectileRect, aZombieRect) >= 0
                        //      && mPosY > aZombieRect.mY && mPosY < aZombieRect.mY + mHeight → DoImpact
                        if crate::lawn::board::get_rect_overlap(&a_projectile_rect, &a_zombie_rect) >= 0
                            && my_pos_y > a_zombie_rect.y as f32
                            && my_pos_y < (a_zombie_rect.y + a_zombie_rect.height) as f32
                        {
                            self.do_impact_by_index(self.target_zombie_id as usize);
                        }
                    }
                }
            }
            return;
        }

        // C++: PROJECTILE_STAR && (mPosY > 600 || mPosY < 40) → Die（:362）
        if proj_type == ProjectileType::Star && (my_pos_y > 600.0 || my_pos_y < 40.0) {
            self.die();
            return;
        }

        // C++: (PEA || STAR) && mShadowY - mPosY > 90 → return（不碰撞，:365）
        if (proj_type == ProjectileType::Pea || proj_type == ProjectileType::Star)
            && self.shadow_y - my_pos_y > 90.0
        {
            return;
        }

        // C++: MOTION_FLOAT_OVER → return（:369）
        if motion == ProjectileMotion::Floating {
            return;
        }

        // C++: PROJECTILE_ZOMBIE_PEA —— 专打植物（:371-385）
        if proj_type == ProjectileType::ZombiePea {
            let hit_plant_idx = self.find_collision_target_plant();
            if let Some(plant_idx) = hit_plant_idx {
                let a_damage = self.damage;
                if let Some(board) = self.base.get_board_mut() {
                    if let Some(plant) = board.plants.get_mut(plant_idx) {
                        plant.plant_health -= a_damage;
                        plant.eaten_flash_countdown = plant.eaten_flash_countdown.max(25);
                    }
                }
                // 对应 C++: PlayFoley(FOLEY_SPLAT) + PARTICLE_PEA_SPLAT 粒子（粒子系统 stub）
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Splat as i32);
                }
                self.die();
            }
            return;
        }

        // C++: FindCollisionTarget + aZombie->mOnHighGround && CantHitHighGround() → return（:387-394）
        let hit_zombie_idx = self.find_collision_target();

        if let Some(zombie_idx) = hit_zombie_idx {
            let a_zombie_on_high_ground = self
                .base
                .get_board()
                .and_then(|b| b.zombies.get(zombie_idx))
                .map_or(false, |z| z.on_high_ground);
            if a_zombie_on_high_ground && self.cant_hit_high_ground() {
                return;
            }
            self.do_impact_by_index(zombie_idx);
        }
    }

    /// 命中音效（对应 C++ Projectile::PlayImpactSound）
    pub fn play_impact_sound(&mut self, zombie_idx: Option<usize>) {
        let mut a_play_helm_sound = true;
        let mut a_play_splat_sound = true;

        match self.projectile_type {
            ProjectileType::Kernel => {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::KernelSplat as i32);
                }
                a_play_helm_sound = false;
                a_play_splat_sound = false;
            }
            ProjectileType::Butter => {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Butter as i32);
                }
                a_play_splat_sound = false;
            }
            ProjectileType::Fireball if self.is_splash_damage() => {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Ignite as i32);
                }
                a_play_helm_sound = false;
                a_play_splat_sound = false;
            }
            ProjectileType::Melon | ProjectileType::Wintermelon => {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::MelonImpact as i32);
                }
                a_play_splat_sound = false;
            }
            _ => {}
        }

        if a_play_helm_sound {
            if let Some(idx) = zombie_idx {
                let helm_type = self.base.get_board().and_then(|b| b.zombies.get(idx)).map(|z| z.helm_type);
                match helm_type {
                    Some(HelmType::Pail) => {
                        if let Some(app) = self.base.get_app() {
                            app.play_foley(crate::todlib::tod_foley::FoleyType::ShieldHit as i32);
                        }
                        a_play_splat_sound = false;
                    }
                    Some(HelmType::TrafficCone) | Some(HelmType::Digger) | Some(HelmType::FootballHelmet) => {
                        if let Some(app) = self.base.get_app() {
                            app.play_foley(crate::todlib::tod_foley::FoleyType::PlasticHit as i32);
                        }
                    }
                    _ => {}
                }
            }
        }

        if a_play_splat_sound {
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Splat as i32);
            }
        }
    }

    /// 通过索引对僵尸造成碰撞效果（对应 C++ DoImpact 主体）
    /// 命中结算（对应 C++ Projectile::DoImpact，Projectile.cpp:823）
    pub fn do_impact_by_index(&mut self, zombie_idx: usize) {
        self.play_impact_sound(Some(zombie_idx));

        let proj_type = self.projectile_type;
        let a_last_pos_x = self.pos_x - self.vel_x;
        let a_last_pos_y = self.pos_y + self.pos_z - self.vel_y - self.vel_z;
        let mut a_splat_pos_x = self.pos_x + 12.0;
        let mut a_splat_pos_y = self.pos_y + 12.0;
        let a_render_order = self.base.render_order + 1;

        // 对应 C++ IsSplashDamage(theZombie)：火球命中火抗僵尸不算溅射
        let mut a_zombie_is_fire_resistant = false;
        if let Some(board) = self.base.get_board() {
            if let Some(zombie) = board.zombies.get(zombie_idx) {
                a_zombie_is_fire_resistant = zombie.is_fire_resistant();
            }
        }
        let is_splash = self.is_splash_damage()
            && !(proj_type == ProjectileType::Fireball && a_zombie_is_fire_resistant);

        if is_splash {
            if proj_type == ProjectileType::Fireball {
                if let Some(board) = self.base.get_board_mut() {
                    if let Some(zombie) = board.zombies.get_mut(zombie_idx) {
                        zombie.remove_cold_effects();
                    }
                }
            }
            self.do_splash_damage(Some(zombie_idx));
        } else {
            let a_damage = self.damage;
            let a_damage_flags = self.damage_flags;
            if let Some(board) = self.base.get_board_mut() {
                if let Some(zombie) = board.zombies.get_mut(zombie_idx) {
                    zombie.take_damage(a_damage, a_damage_flags);
                }
            }
        }

        // 对应 C++ DoImpact 的 switch (mProjectileType)：粒子与附加效果
        let mut a_effect = ParticleEffect::None;
        match proj_type {
            ProjectileType::Melon => {
                if let Some(app) = self.base.get_app_mut() {
                    app.add_tod_particle(
                        a_last_pos_x + 30.0, a_last_pos_y + 30.0, a_render_order,
                        ParticleEffect::Melonsplash as i32,
                    );
                }
            }
            ProjectileType::Wintermelon => {
                if let Some(app) = self.base.get_app_mut() {
                    app.add_tod_particle(
                        a_last_pos_x + 30.0, a_last_pos_y + 30.0, a_render_order,
                        ParticleEffect::Wintermelon as i32,
                    );
                }
            }
            ProjectileType::Pea => {
                a_splat_pos_x -= 15.0;
                a_effect = ParticleEffect::PeaSplat;
            }
            ProjectileType::Snowpea => {
                a_splat_pos_x -= 15.0;
                a_effect = ParticleEffect::SnowpeaSplat;
            }
            ProjectileType::Fireball => {
                if is_splash {
                    if let Some(app) = self.base.get_app_mut() {
                        if let Some(a_fire_reanim) = app.add_reanimation(
                            self.pos_x + 38.0, self.pos_y - 20.0, a_render_order,
                            ReanimationType::JalapenoFire as i32,
                        ) {
                            // C++: mAnimTime = 0.25f; mAnimRate = 24.0f; OverrideScale(0.7f, 0.4f)
                            unsafe {
                                (*a_fire_reanim).m_anim_time = 0.25;
                                (*a_fire_reanim).m_anim_rate = 24.0;
                                (*a_fire_reanim).override_scale(0.7, 0.4);
                            }
                        }
                    }
                }
            }
            ProjectileType::Star => {
                a_effect = ParticleEffect::StarSplat;
            }
            ProjectileType::Puff => {
                a_splat_pos_x -= 20.0;
                a_effect = ParticleEffect::PuffSplat;
            }
            ProjectileType::Cabbage => {
                a_splat_pos_x = a_last_pos_x - 38.0;
                a_splat_pos_y = a_last_pos_y + 23.0;
                a_effect = ParticleEffect::CabbageSplat;
            }
            ProjectileType::Butter => {
                a_splat_pos_x = a_last_pos_x - 20.0;
                a_splat_pos_y = a_last_pos_y + 63.0;
                a_effect = ParticleEffect::ButterSplat;
                if let Some(board) = self.base.get_board_mut() {
                    if let Some(zombie) = board.zombies.get_mut(zombie_idx) {
                        zombie.apply_butter();
                    }
                }
            }
            _ => {}
        }

        // 对应 C++：theZombie->AddAttachedParticle(aPosX, aPosY, aEffect)；僵尸不存在则 AddPvzpParticle
        if a_effect != ParticleEffect::None {
            let mut a_particle_applied = false;
            if let Some(board) = self.base.get_board_mut() {
                if let Some(zombie) = board.zombies.get_mut(zombie_idx) {
                    let mut a_pos_x = a_splat_pos_x + 52.0 - zombie.base.x as f32;
                    let mut a_pos_y = a_splat_pos_y - zombie.base.y as f32;
                    if zombie.zombie_phase == ZombiePhase::SnorkelWalkingInPool
                        || zombie.zombie_phase == ZombiePhase::DolphinWalkingInPool
                    {
                        a_pos_y += 60.0;
                    }
                    if self.motion == ProjectileMotion::Backwards {
                        a_pos_x -= 80.0;
                    } else if self.pos_x > zombie.base.x as f32 + 40.0
                        && self.motion != ProjectileMotion::Lobbed
                    {
                        a_pos_x -= 60.0;
                    }
                    a_pos_y = a_pos_y.clamp(20.0, 100.0);
                    zombie.add_attached_particle(a_pos_x as i32, a_pos_y as i32, a_effect);
                    a_particle_applied = true;
                }
            }
            if !a_particle_applied {
                if let Some(app) = self.base.get_app_mut() {
                    app.add_tod_particle(
                        a_splat_pos_x, a_splat_pos_y, a_render_order, a_effect as i32,
                    );
                }
            }
        }

        self.die();
    }

    /// 绘制子弹（对应 C++ Projectile::Draw，Projectile.cpp:975）
    /// 根据 mProjectileType 选择不同图片并处理旋转/缩放/镜像与 Attachment 渲染
    pub fn draw(&self, g: &mut Graphics) {
        let a_projectile_def = self.get_projectile_def();

        let app = self.base.get_app();
        // C++ 中 IMAGE_* 为全局资源宏；Rust 侧经资源管理器按名称取图
        let mut get_projectile_image =
            |name: &str| -> *mut crate::framework::graphics::image::Image {
                match app {
                    Some(a) => crate::lawn::board::get_overlay_image(a, name),
                    None => std::ptr::null_mut(),
                }
            };

        let mut a_image: *mut crate::framework::graphics::image::Image = std::ptr::null_mut();
        let mut a_scale = 1.0f32;
        match self.projectile_type {
            ProjectileType::Cobbig => {
                a_image = get_projectile_image("IMAGE_REANIM_COBCANNON_COB");
                a_scale = 0.9;
            }
            ProjectileType::Pea | ProjectileType::ZombiePea => {
                a_image = get_projectile_image("IMAGE_PROJECTILEPEA");
            }
            ProjectileType::Snowpea => {
                a_image = get_projectile_image("IMAGE_PROJECTILESNOWPEA");
            }
            ProjectileType::Fireball => {
                a_image = std::ptr::null_mut();
            }
            ProjectileType::Spike => {
                a_image = get_projectile_image("IMAGE_PROJECTILECACTUS");
            }
            ProjectileType::Star => {
                a_image = get_projectile_image("IMAGE_PROJECTILE_STAR");
            }
            ProjectileType::Puff => {
                a_image = get_projectile_image("IMAGE_PUFFSHROOM_PUFF1");
                a_scale = crate::todlib::tod_common::tod_animate_curve_float(
                    0, 30, self.projectile_age, 0.3, 1.0, crate::lawn::game_enums::TodCurves::Linear,
                );
            }
            ProjectileType::Basketball => {
                a_image = get_projectile_image("IMAGE_REANIM_ZOMBIE_CATAPULT_BASKETBALL");
                a_scale = 1.1;
            }
            ProjectileType::Cabbage => {
                a_image = get_projectile_image("IMAGE_REANIM_CABBAGEPULT_CABBAGE");
                a_scale = 1.0;
            }
            ProjectileType::Kernel => {
                a_image = get_projectile_image("IMAGE_REANIM_CORNPULT_KERNAL");
                a_scale = 0.95;
            }
            ProjectileType::Butter => {
                a_image = get_projectile_image("IMAGE_REANIM_CORNPULT_BUTTER");
                a_scale = 0.8;
            }
            ProjectileType::Melon => {
                a_image = get_projectile_image("IMAGE_REANIM_MELONPULT_MELON");
                a_scale = 1.0;
            }
            ProjectileType::Wintermelon => {
                a_image = get_projectile_image("IMAGE_REANIM_WINTERMELON_PROJECTILE");
                a_scale = 1.0;
            }
            _ => {
                // C++: PVZP_ASSERT(false)
            }
        }

        let mut a_mirror = false;
        if self.motion == ProjectileMotion::BeeBackwards {
            a_mirror = true;
        }

        if !a_image.is_null() {
            // C++: PVZP_ASSERT(aProjectileDef.mImageRow < aImage->mNumRows)
            // C++: PVZP_ASSERT(mFrame < aImage->mNumCols)
            let a_image_ref = unsafe { &*a_image };
            let a_cel_width = a_image_ref.get_cel_width();
            let a_cel_height = a_image_ref.get_cel_height();
            let a_src_rect = Rect::new(
                a_cel_width * self.frame,
                a_cel_height * a_projectile_def.image_row,
                a_cel_width,
                a_cel_height,
            );
            if crate::todlib::tod_common::float_nearly_equal(self.rotation, 0.0, std::f32::EPSILON)
                && crate::todlib::tod_common::float_nearly_equal(a_scale, 1.0, std::f32::EPSILON)
            {
                let a_dest_rect = Rect::new(0, 0, a_cel_width, a_cel_height);
                g.draw_image_mirror_stretch(a_image_ref, &a_dest_rect, &a_src_rect, a_mirror);
            } else {
                let a_offset_x = self.pos_x + a_cel_width as f32 * 0.5;
                let a_offset_y = self.pos_z + self.pos_y + a_cel_height as f32 * 0.5;
                let mut a_transform = SexyMatrix3::identity();
                let board = self.base.get_board();
                let a_board_m_x = board.map_or(0, |b| b.m_x) as f32;
                let a_board_m_y = board.map_or(0, |b| b.m_y) as f32;
                pvzp_scale_rotate_transform_matrix(
                    &mut a_transform,
                    a_offset_x + a_board_m_x,
                    a_offset_y + a_board_m_y,
                    self.rotation,
                    a_scale,
                    a_scale,
                );
                g.draw_image_matrix_src(a_image_ref, &a_transform, &a_src_rect, 0.0, 0.0);
            }
        }

        // C++: Graphics theParticleGraphics(*g); MakeParentGraphicsFrame(&theParticleGraphics); AttachmentDraw(mAttachmentID, &theParticleGraphics, false)
        if self.attachment_id != crate::lawn::game_enums::ATTACHMENTID_NULL {
            // [TRANSLATION_NOTE]: Rust 无按 ID 的附件绘制入口（AttachmentDraw），且 Graphics 不可复制，暂略（同 coin.rs/zombie.rs）
        }
    }

    /// 绘制子弹阴影（对应 C++ Projectile::DrawShadow，Projectile.cpp:1073）
    pub fn draw_shadow(&self, g: &mut Graphics) {
        let mut a_cel_col = 0;
        let mut a_scale = 1.0;
        let mut a_stretch = 1.0;
        let mut a_offset_x = self.pos_x - self.base.x as f32;
        let mut a_offset_y = self.pos_y - self.base.y as f32;

        let my_row = self.base.row;
        let mut is_high_ground = false;
        let mut is_night = false;
        if let Some(board) = self.base.get_board() {
            let a_grid_x = board.pixel_to_grid_x_keep_on_board(self.base.x, self.base.y);
            if a_grid_x >= 0 && (a_grid_x as usize) < crate::lawn::board::MAX_GRID_SIZE_X as usize {
                if board.grid_square_type[my_row as usize][a_grid_x as usize] == GridSquareType::HighGround {
                    is_high_ground = true;
                }
            }
            is_night = board.stage_is_night();
        }
        if self.on_high_ground && !is_high_ground {
            a_offset_y += crate::lawn::zombie::HIGH_GROUND_HEIGHT;
        } else if !self.on_high_ground && is_high_ground {
            a_offset_y -= crate::lawn::zombie::HIGH_GROUND_HEIGHT;
        }

        if is_night {
            a_cel_col = 1;
        }

        let proj_type = self.projectile_type;
        match proj_type {
            ProjectileType::Pea | ProjectileType::ZombiePea => {
                a_offset_x += 3.0;
            }
            ProjectileType::Snowpea => {
                a_offset_x += -1.0;
                a_scale = 1.3;
            }
            ProjectileType::Star => {
                a_offset_x += 7.0;
            }
            ProjectileType::Cabbage | ProjectileType::Kernel | ProjectileType::Butter
            | ProjectileType::Melon | ProjectileType::Wintermelon => {
                a_offset_x += 3.0;
                a_offset_y += 10.0;
                a_scale = 1.6;
            }
            ProjectileType::Puff => {
                return;
            }
            ProjectileType::Cobbig => {
                a_scale = 1.0;
                a_stretch = 3.0;
                a_offset_x += 57.0;
            }
            ProjectileType::Fireball => {
                a_scale = 1.4;
            }
            _ => {}
        }

        if self.motion == ProjectileMotion::Lobbed {
            let a_height = (-self.pos_z).clamp(0.0, 200.0);
            a_scale *= 200.0 / (a_height + 200.0);
        }

        // PvzpDrawImageCelScaledF(g, IMAGE_PEA_SHADOWS, aOffsetX, mShadowY - mPosY + aOffsetY, aCelCol, 0, aScale*aStretch, aScale)
        let app = match self.base.get_app() {
            Some(a) => a,
            None => return,
        };
        let a_shadow_image = crate::lawn::board::get_overlay_image(app, "IMAGE_PEA_SHADOWS");
        if a_shadow_image.is_null() {
            return;
        }
        let a_shadow_image_ref = unsafe { &*a_shadow_image };
        let a_src_rect = a_shadow_image_ref.get_cel_rect(a_cel_col, 0);
        let a_cel_width = a_src_rect.width;
        let a_cel_height = a_src_rect.height;
        let a_dst_rect = Rect::new(
            a_offset_x as i32,
            (self.shadow_y - self.pos_y + a_offset_y) as i32,
            (a_cel_width as f32 * a_scale * a_stretch) as i32,
            (a_cel_height as f32 * a_scale) as i32,
        );
        g.draw_image_stretch(a_shadow_image_ref, &a_dst_rect, &a_src_rect);
    }

    /// 查找碰撞植物（对应 C++ FindCollisionTargetPlant）
    pub fn find_collision_target_plant(&self) -> Option<usize> {
        let proj_rect = self.get_projectile_rect();
        let my_row = self.base.row;
        let proj_type = self.projectile_type;

        if let Some(board) = self.base.get_board() {
            for (idx, plant) in board.plants.iter().enumerate() {
                if plant.dead { continue; }
                if plant.base.row != my_row { continue; }

                // 僵尸豌豆不能打低矮植物（Puffshroom/Sunshroom/PotatoMine/Spikeweed/Spikerock/Lilypad）
                // [TRANSLATION_NOTE]: 低矮植物检查暂未实现
                let plant_rect = plant.plant_rect;
                if crate::lawn::board::get_rect_overlap(&proj_rect, &plant_rect) > 8 {
                    return Some(idx);
                }
            }
        }
        None
    }

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

    /// 转换为火球（对应 C++ ConvertToFireball）
    pub fn convert_to_fireball(&mut self, grid_x: i32) {
        if self.hit_torchwood_grid_x == grid_x { return; }
        self.projectile_type = ProjectileType::Fireball;
        self.hit_torchwood_grid_x = grid_x;
        self.damage = 40;
        // 对应 C++: mApp->PlayFoley(FOLEY_FIREPEA)
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::FirePea as i32);
        }
        // [TRANSLATION_NOTE]: 火球 REANIM_FIRE_PEA 外观（含 MOTION_BACKWARDS 镜像）依赖 reanim stub
    }

    /// 转换为普通豌豆（对应 C++ ConvertToPea）
    pub fn convert_to_pea(&mut self, grid_x: i32) {
        if self.hit_torchwood_grid_x == grid_x { return; }
        self.projectile_type = ProjectileType::Pea;
        self.hit_torchwood_grid_x = grid_x;
        self.damage = 20;
        // [TRANSLATION_NOTE]: 音效依赖 Foley 系统
    }

    /// 是否是溅射伤害类型（对应 C++ IsSplashDamage 的类型判定部分；
    /// 火抗僵尸豁免在命中结算处按 C++ 语义处理）
    pub fn is_splash_damage(&self) -> bool {
        self.projectile_type == ProjectileType::Melon
            || self.projectile_type == ProjectileType::Wintermelon
            || self.projectile_type == ProjectileType::Fireball
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

    /// 豌豆是否即将击中火炬树桩（对应 C++ PeaAboutToHitTorchwood）
    pub fn pea_about_to_hit_torchwood(&self) -> bool {
        if self.motion != ProjectileMotion::Straight {
            return false;
        }

        if self.projectile_type != ProjectileType::Pea && self.projectile_type != ProjectileType::Snowpea {
            return false;
        }

        let board = match self.base.get_board() {
            Some(b) => b,
            None => return false,
        };
        for plant in &board.plants {
            if plant.dead {
                continue;
            }
            if plant.seed_type == SeedType::Torchwood && plant.base.row == self.base.row
                && !plant.not_on_ground() && self.hit_torchwood_grid_x != plant.plant_col
            {
                let a_plant_attack_rect = plant.get_plant_attack_rect(PlantWeapon::Primary);
                let mut a_projectile_rect = self.get_projectile_rect();
                a_projectile_rect.x += 40;

                if crate::lawn::board::get_rect_overlap(&a_plant_attack_rect, &a_projectile_rect) > 10 {
                    return true;
                }
            }
        }

        false
    }

    /// 查找碰撞僵尸（对应 C++ Projectile::FindCollisionTarget，Projectile.cpp:224）
    pub fn find_collision_target(&self) -> Option<usize> {
        // a pea about to hit a torchwood skips zombie collision ("torchwood clip" trick)
        if self.pea_about_to_hit_torchwood() {
            return None;
        }

        let a_projectile_rect = self.get_projectile_rect();
        let my_row = self.base.row;
        let proj_type = self.projectile_type;
        let proj_age = self.projectile_age;
        let vel_x = self.vel_x;
        let pos_z = self.pos_z;
        let damage_range_flags = self.damage_range_flags;

        let board = match self.base.get_board() {
            Some(b) => b,
            None => return None,
        };
        let mut best_zombie: Option<usize> = None;
        let mut a_min_x = 0;
        for (idx, zombie) in board.zombies.iter().enumerate() {
            if zombie.dead {
                continue;
            }
            if (zombie.zombie_type == ZombieType::Boss || zombie.base.row == my_row)
                && zombie.effected_by_damage(damage_range_flags)
            {
                if zombie.zombie_phase == ZombiePhase::SnorkelWalkingInPool && pos_z <= 45.0 {
                    continue;
                }

                if proj_type == ProjectileType::Star && proj_age < 25 && vel_x >= 0.0
                    && zombie.zombie_type == ZombieType::Digger
                {
                    continue;
                }

                let a_zombie_rect = zombie.get_zombie_rect();
                if crate::lawn::board::get_rect_overlap(&a_projectile_rect, &a_zombie_rect) >= 0 {
                    if best_zombie.is_none() || zombie.base.x < a_min_x {
                        best_zombie = Some(idx);
                        a_min_x = zombie.base.x;
                    }
                }
            }
        }
        best_zombie
    }

    /// 获取子弹定义（对应 C++ GetProjectileDef）
    pub fn get_projectile_def(&self) -> ProjectileDefinition {
        match self.projectile_type {
            ProjectileType::Pea | ProjectileType::Snowpea | ProjectileType::Star | ProjectileType::Spike | ProjectileType::Puff => {
                ProjectileDefinition { projectile_type: self.projectile_type, image_row: 0, damage: 20 }
            }
            ProjectileType::Cabbage | ProjectileType::Butter | ProjectileType::Fireball => {
                ProjectileDefinition { projectile_type: self.projectile_type, image_row: 0, damage: 40 }
            }
            ProjectileType::Melon | ProjectileType::Wintermelon => {
                ProjectileDefinition { projectile_type: self.projectile_type, image_row: 0, damage: 80 }
            }
            ProjectileType::Kernel => {
                ProjectileDefinition { projectile_type: self.projectile_type, image_row: 0, damage: 20 }
            }
            ProjectileType::Cobcannon => {
                ProjectileDefinition { projectile_type: self.projectile_type, image_row: 0, damage: 300 }
            }
            ProjectileType::Basketball => {
                ProjectileDefinition { projectile_type: self.projectile_type, image_row: 0, damage: 75 }
            }
            _ => {
                ProjectileDefinition { projectile_type: self.projectile_type, image_row: 0, damage: 20 }
            }
        }
    }

    /// 获取伤害标志（对应 C++ GetDamageFlags）
    pub fn get_damage_flags(&self) -> u32 {
        let mut flags = 0u32;
        // [TRANSLATION_NOTE]: 溅射/抛物线/倒走/星星+倒走 的盾牌穿越标志暂未实现
        if self.projectile_type == ProjectileType::Snowpea {
            flags |= 1 << 2; // DAMAGE_FREEZE = 2
        }
        flags
    }

    /// 无法击中高台（对应 C++ CantHitHighGround）
    pub fn cant_hit_high_ground(&self) -> bool {
        // 对应 C++ CantHitHighGround（Projectile.cpp:395-407）：
        // BACKWARDS/HOMING 恒可命中（返回 false）；
        // 其余 (PEA|SNOWPEA|STAR|PUFF|FIREBALL) 且自身不在高台时不可命中
        if self.motion == ProjectileMotion::Backwards || self.motion == ProjectileMotion::Homing {
            return false;
        }
        matches!(
            self.projectile_type,
            ProjectileType::Pea
                | ProjectileType::Snowpea
                | ProjectileType::Star
                | ProjectileType::Puff
                | ProjectileType::Fireball
        ) && !self.on_high_ground
    }

    /// 检查高台（对应 C++ CheckForHighGround）
    pub fn check_for_high_ground(&mut self) {
        if self.on_high_ground && self.cant_hit_high_ground() {
            // [TRANSLATION_NOTE]: 高台碰撞检测暂未实现
        }
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
