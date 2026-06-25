// PvZ Portable Rust 翻译 — Zombie（僵尸类）
// 对应 C++ src/Lawn/Zombie.h / Zombie.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::lawn::plant::Plant;
use crate::todlib::reanimator::{Reanimation, ReanimationType};
use crate::todlib::tod_particle::ParticleSystem;
use crate::todlib::attachment::Attachment;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::rect::Rect;
use crate::framework::color::Color;

pub const MAX_ZOMBIE_FOLLOWERS: usize = 4;
pub const BUNGEE_ZOMBIE_HEIGHT: i32 = 3000;
pub const ZOMBIE_LIMP_SPEED_FACTOR: i32 = 2;
pub const POGO_BOUNCE_TIME: i32 = 80;
pub const DOLPHIN_JUMP_TIME: i32 = 120;
pub const CHILLED_SPEED_FACTOR: f32 = 0.4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ZombieAttackType {
    Chew = 0,
    DriveOver,
    Vault,
    Ladder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ZombieParts {
    Body = 0,
    Head,
    HeadEating,
    Tongue,
    Arm,
    Hair,
    HeadYucky,
    ArmPickaxe,
    ArmPolevault,
    ArmLeash,
    ArmFlag,
    Pogo,
    Digger,
}

// HelmType 已移至 game_enums.rs 中定义，通过 use crate::lawn::game_enums::* 导入。

#[derive(Debug, Clone, Copy)]
pub struct ZombieDrawPosition {
    pub head_x: i32,
    pub head_y: i32,
    pub arm_y: i32,
    pub body_y: f32,
    pub image_offset_x: f32,
    pub image_offset_y: f32,
    pub clip_height: f32,
}

impl ZombieDrawPosition {
    pub fn new() -> Self {
        ZombieDrawPosition {
            head_x: 0,
            head_y: 0,
            arm_y: 0,
            body_y: 0.0,
            image_offset_x: 0.0,
            image_offset_y: 0.0,
            clip_height: -200.0,
        }
    }
}

/// 僵尸类
pub struct Zombie {
    pub base: GameObject,

    pub zombie_type: ZombieType,
    pub zombie_phase: ZombiePhase,
    pub pos_x: f32,
    pub pos_y: f32,
    pub vel_x: f32,
    pub anim_counter: i32,
    pub groan_counter: i32,
    pub anim_ticks_per_frame: i32,
    pub anim_frames: i32,
    pub frame: i32,
    pub prev_frame: i32,
    pub variant: bool,
    pub is_eating: bool,
    pub just_got_shot_counter: i32,
    pub shield_just_got_shot_counter: i32,
    pub shield_recoil_counter: i32,
    pub zombie_age: i32,
    pub zombie_height: ZombieHeight,
    pub phase_counter: i32,
    pub from_wave: i32,
    pub dropped_loot: bool,
    pub zombie_fade: i32,
    pub flat_tires: bool,
    pub use_ladder_col: i32,
    pub target_col: i32,
    pub altitude: f32,
    pub hit_umbrella: bool,
    pub zombie_rect: Rect,
    pub zombie_attack_rect: Rect,
    pub chilled_counter: i32,
    pub buttered_counter: i32,
    pub ice_trap_counter: i32,
    pub mind_controlled: bool,
    pub blowing_away: bool,
    pub has_head: bool,
    pub has_arm: bool,
    pub has_object: bool,
    pub in_pool: bool,
    pub on_high_ground: bool,
    pub yucky_face: bool,
    pub yucky_face_counter: i32,
    pub helm_type: HelmType,
    pub body_health: i32,
    pub body_max_health: i32,
    pub helm_health: i32,
    pub helm_max_health: i32,
    pub shield_type: ShieldType,
    pub shield_health: i32,
    pub shield_max_health: i32,
    pub flying_health: i32,
    pub flying_max_health: i32,
    pub dead: bool,
    pub related_zombie_id: ZombieID,
    pub follower_zombie_ids: [ZombieID; MAX_ZOMBIE_FOLLOWERS],
    pub playing_song: bool,
    pub particle_offset_x: i32,
    pub particle_offset_y: i32,
    pub attachment_id: AttachmentID,
    pub summon_counter: i32,
    pub body_reanim_id: ReanimationID,
    pub scale_zombie: f32,
    pub vel_z: f32,
    pub original_anim_rate: f32,
    pub target_plant_id: PlantID,
    pub boss_mode: i32,
    pub target_row: i32,
    pub boss_bungee_counter: i32,
    pub boss_stomp_counter: i32,
    pub boss_head_counter: i32,
    pub boss_fire_ball_reanim_id: ReanimationID,
    pub special_head_reanim_id: ReanimationID,
    pub fireball_row: i32,
    pub is_fire_ball: bool,
    pub mowered_reanim_id: ReanimationID,
    pub last_portal_x: i32,
}

impl Zombie {
    pub fn new() -> Self {
        Zombie {
            base: GameObject::new(),
            zombie_type: ZombieType::Normal,
            zombie_phase: ZombiePhase::Normal,
            pos_x: 0.0,
            pos_y: 0.0,
            vel_x: 0.0,
            anim_counter: 0,
            groan_counter: 0,
            anim_ticks_per_frame: 10,
            anim_frames: 1,
            frame: 0,
            prev_frame: 0,
            variant: false,
            is_eating: false,
            just_got_shot_counter: 0,
            shield_just_got_shot_counter: 0,
            shield_recoil_counter: 0,
            zombie_age: 0,
            zombie_height: ZombieHeight::Normal,
            phase_counter: 0,
            from_wave: 0,
            dropped_loot: false,
            zombie_fade: 255,
            flat_tires: false,
            use_ladder_col: -1,
            target_col: 0,
            altitude: 0.0,
            hit_umbrella: false,
            zombie_rect: Rect::ZERO,
            zombie_attack_rect: Rect::ZERO,
            chilled_counter: 0,
            buttered_counter: 0,
            ice_trap_counter: 0,
            mind_controlled: false,
            blowing_away: false,
            has_head: true,
            has_arm: true,
            has_object: false,
            in_pool: false,
            on_high_ground: false,
            yucky_face: false,
            yucky_face_counter: 0,
            helm_type: HelmType::None,
            body_health: 100,
            body_max_health: 100,
            helm_health: 0,
            helm_max_health: 0,
            shield_type: ShieldType::None,
            shield_health: 0,
            shield_max_health: 0,
            flying_health: 0,
            flying_max_health: 0,
            dead: false,
            related_zombie_id: ZOMBIEID_NULL,
            follower_zombie_ids: [ZOMBIEID_NULL; MAX_ZOMBIE_FOLLOWERS],
            playing_song: false,
            particle_offset_x: 0,
            particle_offset_y: 0,
            attachment_id: ATTACHMENTID_NULL,
            summon_counter: 0,
            body_reanim_id: REANIMATIONID_NULL,
            scale_zombie: 1.0,
            vel_z: 0.0,
            original_anim_rate: 1.0,
            target_plant_id: PLANTID_NULL,
            boss_mode: 0,
            target_row: 0,
            boss_bungee_counter: 0,
            boss_stomp_counter: 0,
            boss_head_counter: 0,
            boss_fire_ball_reanim_id: REANIMATIONID_NULL,
            special_head_reanim_id: REANIMATIONID_NULL,
            fireball_row: 0,
            is_fire_ball: false,
            mowered_reanim_id: REANIMATIONID_NULL,
            last_portal_x: 0,
        }
    }

    /// 初始化僵尸
    pub fn zombie_initialize(&mut self, row: i32, ztype: ZombieType, variant: bool, _parent: Option<&mut Zombie>, from_wave: i32) {
        self.zombie_type = ztype;
        self.variant = variant;
        self.base.row = row;
        self.from_wave = from_wave;

        // 设置基本属性
        self.base.x = 800 + 40; // 从屏幕右侧外出现
        self.base.y = crate::lawn::board::row_to_y(row);
        self.pos_x = self.base.x as f32;
        self.pos_y = self.base.y as f32;

        self.setup_health_for_type(ztype);
        self.pick_random_speed();
    }

    fn setup_health_for_type(&mut self, ztype: ZombieType) {
        match ztype {
            ZombieType::Normal | ZombieType::Flag | ZombieType::DuckyTube => {
                self.body_health = 200;
                self.body_max_health = 200;
                self.anim_frames = 16;
                self.anim_ticks_per_frame = 6;
            },
            ZombieType::TrafficCone => {
                self.body_health = 200;
                self.body_max_health = 200;
                self.helm_health = 200;
                self.helm_max_health = 200;
                self.helm_type = HelmType::TrafficCone;
            },
            ZombieType::Pail => {
                self.body_health = 200;
                self.body_max_health = 200;
                self.helm_health = 800;
                self.helm_max_health = 800;
                self.helm_type = HelmType::Pail;
            },
            ZombieType::Football => {
                self.body_health = 800;
                self.body_max_health = 800;
                self.helm_health = 800;
                self.helm_max_health = 800;
                self.helm_type = HelmType::FootballHelmet;
            },
            ZombieType::Boss => {
                self.body_health = 40000;
                self.body_max_health = 40000;
            },
            ZombieType::Gargantuar | ZombieType::RedeEyeGargantuar => {
                self.body_health = 3000;
                self.body_max_health = 3000;
            },
            ZombieType::Imp => {
                self.body_health = 100;
                self.body_max_health = 100;
            },
            ZombieType::Zamboni => {
                self.body_health = 1200;
                self.body_max_health = 1200;
            },
            ZombieType::Catapult => {
                self.body_health = 850;
                self.body_max_health = 850;
            },
            ZombieType::Yeti => {
                self.body_health = 1000;
                self.body_max_health = 1000;
            },
            _ => {
                self.body_health = 200;
                self.body_max_health = 200;
            }
        }
    }

    fn pick_random_speed(&mut self) {
        self.vel_x = match self.zombie_type {
            ZombieType::Football => 2.5,
            ZombieType::Polevaulter => 2.0,
            ZombieType::DolphinRider => 2.0,
            ZombieType::Pogo => 2.0,
            ZombieType::Flag => 0.7,
            ZombieType::Yeti => 3.0,
            ZombieType::Imp => 1.5,
            ZombieType::Balloon => 1.5,
            ZombieType::Bungee => 0.0,
            ZombieType::Boss => 0.5,
            _ => 0.5,
        };
    }

    /// 更新僵尸
    pub fn update(&mut self) {
        if self.dead {
            self.update_death();
            return;
        }

        self.anim_counter += 1;
        self.zombie_age += 1;

        // 减速效果
        if self.chilled_counter > 0 {
            self.chilled_counter -= 1;
        }
        if self.buttered_counter > 0 {
            self.buttered_counter -= 1;
        }

        // 移动
        if !self.is_eating {
            let speed_mult = if self.chilled_counter > 0 { CHILLED_SPEED_FACTOR } else { 1.0 };
            self.pos_x -= self.vel_x * speed_mult;
        }

        // 更新攻击矩形
        self.zombie_rect = self.get_zombie_rect();
        self.zombie_attack_rect = self.get_zombie_attack_rect();

        // 更新相位
        self.update_zombie_walking();
    }

    /// 更新僵尸行走
    pub fn update_zombie_walking(&mut self) {
        self.base.x = self.pos_x as i32;
        self.base.y = self.pos_y as i32;
        if self.anim_counter % self.anim_ticks_per_frame == 0 {
            self.prev_frame = self.frame;
            self.frame = (self.frame + 1) % self.anim_frames;
        }
    }

    /// 吃植物
    pub fn eat_plant(&mut self, _plant: &mut Plant) {
        if !self.is_eating {
            self.is_eating = true;
        }
    }

    /// 停止进食
    pub fn stop_eating(&mut self) {
        self.is_eating = false;
    }

    /// 受伤
    pub fn take_damage(&mut self, damage: i32, _damage_flags: u32) {
        self.just_got_shot_counter = 10;

        // 先伤害头盔/盾牌
        if self.helm_health > 0 {
            self.helm_health -= damage;
            if self.helm_health <= 0 {
                self.helm_health = 0;
                self.helm_type = HelmType::None;
            }
        } else if self.shield_health > 0 {
            self.shield_health -= damage;
            if self.shield_health <= 0 {
                self.shield_health = 0;
                self.shield_type = ShieldType::None;
            }
        } else {
            self.body_health -= damage;
            if self.body_health <= 0 {
                self.body_health = 0;
                self.die_no_loot();
            }
        }
    }

    /// 直接死亡（不掉落物品）
    pub fn die_no_loot(&mut self) {
        self.dead = true;
        self.zombie_phase = ZombiePhase::Dying;
    }

    /// 死亡并掉落物品
    pub fn die_with_loot(&mut self) {
        self.dropped_loot = true;
        self.die_no_loot();
        self.drop_loot();
    }

    /// 掉落物品
    pub fn drop_loot(&mut self) {
        // 掉落硬币
    }

    /// 绘制僵尸
    pub fn draw(&self, _g: &mut Graphics) {}

    /// 绘制僵尸头部
    pub fn draw_zombie_head(&self, _g: &mut Graphics, _pos: &ZombieDrawPosition, _frame: i32) {}

    /// 获取僵尸矩形
    pub fn get_zombie_rect(&self) -> Rect {
        Rect::new(
            (self.pos_x - 20.0) as i32,
            (self.pos_y - 30.0) as i32,
            50,
            60,
        )
    }

    /// 获取僵尸攻击矩形
    pub fn get_zombie_attack_rect(&self) -> Rect {
        Rect::new(
            (self.pos_x - 40.0) as i32,
            (self.pos_y - 30.0) as i32,
            20,
            60,
        )
    }

    /// 设置行
    pub fn set_row(&mut self, row: i32) {
        self.base.row = row;
        self.pos_y = crate::lawn::board::row_to_y(row) as f32;
    }

    pub fn update_death(&mut self) {}

    /// 获取渲染位置
    pub fn get_draw_pos(&self) -> ZombieDrawPosition {
        let mut pos = ZombieDrawPosition::new();
        pos.body_y = self.pos_y;
        pos.image_offset_x = 0.0;
        pos.image_offset_y = 0.0;
        pos
    }
}

/// 僵尸定义（对应 C++ ZombieDefinition）
#[derive(Debug, Clone)]
pub struct ZombieDefinition {
    pub zombie_type: ZombieType,
    pub reanimation_type: ReanimationType,
    pub zombie_value: i32,
    pub starting_level: i32,
    pub first_allowed_wave: i32,
    pub pick_weight: i32,
    pub zombie_name: &'static str,
}

/// 僵尸定义表（对应 C++ gZombieDefs）
pub const ZOMBIE_DEFS: &[ZombieDefinition] = &[
    ZombieDefinition { zombie_type: ZombieType::Normal, reanimation_type: ReanimationType::ReanimZombie, zombie_value: 1, starting_level: 1, first_allowed_wave: 1, pick_weight: 4000, zombie_name: "ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Flag, reanimation_type: ReanimationType::ReanimZombie, zombie_value: 1, starting_level: 1, first_allowed_wave: 1, pick_weight: 0, zombie_name: "FLAG_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::TrafficCone, reanimation_type: ReanimationType::ReanimZombie, zombie_value: 2, starting_level: 3, first_allowed_wave: 1, pick_weight: 4000, zombie_name: "CONEHEAD_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Polevaulter, reanimation_type: ReanimationType::ReanimPolevaulter, zombie_value: 2, starting_level: 6, first_allowed_wave: 5, pick_weight: 2000, zombie_name: "POLE_VAULTING_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Pail, reanimation_type: ReanimationType::ReanimZombie, zombie_value: 4, starting_level: 8, first_allowed_wave: 1, pick_weight: 3000, zombie_name: "BUCKETHEAD_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Newspaper, reanimation_type: ReanimationType::ReanimZombieNewspaper, zombie_value: 2, starting_level: 11, first_allowed_wave: 1, pick_weight: 1000, zombie_name: "NEWSPAPER_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Door, reanimation_type: ReanimationType::ReanimZombie, zombie_value: 4, starting_level: 13, first_allowed_wave: 5, pick_weight: 3500, zombie_name: "SCREEN_DOOR_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Football, reanimation_type: ReanimationType::ReanimZombieFootball, zombie_value: 7, starting_level: 16, first_allowed_wave: 5, pick_weight: 2000, zombie_name: "FOOTBALL_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Dancer, reanimation_type: ReanimationType::ReanimDancer, zombie_value: 5, starting_level: 18, first_allowed_wave: 5, pick_weight: 1000, zombie_name: "DANCING_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::BackupDancer, reanimation_type: ReanimationType::ReanimBackupDancer, zombie_value: 1, starting_level: 18, first_allowed_wave: 1, pick_weight: 0, zombie_name: "BACKUP_DANCER" },
    ZombieDefinition { zombie_type: ZombieType::DuckyTube, reanimation_type: ReanimationType::ReanimZombie, zombie_value: 1, starting_level: 21, first_allowed_wave: 5, pick_weight: 0, zombie_name: "DUCKY_TUBE_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Snorkel, reanimation_type: ReanimationType::ReanimSnorkel, zombie_value: 3, starting_level: 23, first_allowed_wave: 10, pick_weight: 2000, zombie_name: "SNORKEL_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Zamboni, reanimation_type: ReanimationType::ReanimZombieZamboni, zombie_value: 7, starting_level: 26, first_allowed_wave: 10, pick_weight: 2000, zombie_name: "ZOMBONI" },
    ZombieDefinition { zombie_type: ZombieType::Bobsled, reanimation_type: ReanimationType::ReanimBobsled, zombie_value: 3, starting_level: 26, first_allowed_wave: 10, pick_weight: 2000, zombie_name: "ZOMBIE_BOBSLED_TEAM" },
    ZombieDefinition { zombie_type: ZombieType::DolphinRider, reanimation_type: ReanimationType::ReanimZombieDolphinrider, zombie_value: 3, starting_level: 28, first_allowed_wave: 10, pick_weight: 1500, zombie_name: "DOLPHIN_RIDER_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::JackInTheBox, reanimation_type: ReanimationType::ReanimJackinthebox, zombie_value: 3, starting_level: 31, first_allowed_wave: 10, pick_weight: 1000, zombie_name: "JACK_IN_THE_BOX_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Balloon, reanimation_type: ReanimationType::ReanimBalloon, zombie_value: 2, starting_level: 33, first_allowed_wave: 10, pick_weight: 2000, zombie_name: "BALLOON_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Digger, reanimation_type: ReanimationType::ReanimDigger, zombie_value: 4, starting_level: 36, first_allowed_wave: 10, pick_weight: 1000, zombie_name: "DIGGER_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Pogo, reanimation_type: ReanimationType::ReanimPogo, zombie_value: 4, starting_level: 38, first_allowed_wave: 10, pick_weight: 1000, zombie_name: "POGO_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Yeti, reanimation_type: ReanimationType::ReanimYeti, zombie_value: 4, starting_level: 40, first_allowed_wave: 1, pick_weight: 1, zombie_name: "ZOMBIE_YETI" },
    ZombieDefinition { zombie_type: ZombieType::Bungee, reanimation_type: ReanimationType::ReanimBungee, zombie_value: 3, starting_level: 41, first_allowed_wave: 10, pick_weight: 1000, zombie_name: "BUNGEE_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Ladder, reanimation_type: ReanimationType::ReanimLadder, zombie_value: 4, starting_level: 43, first_allowed_wave: 10, pick_weight: 1000, zombie_name: "LADDER_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Catapult, reanimation_type: ReanimationType::ReanimCatapult, zombie_value: 5, starting_level: 46, first_allowed_wave: 10, pick_weight: 1500, zombie_name: "CATAPULT_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Gargantuar, reanimation_type: ReanimationType::ReanimGargantuar, zombie_value: 10, starting_level: 48, first_allowed_wave: 15, pick_weight: 1500, zombie_name: "GARGANTUAR" },
    ZombieDefinition { zombie_type: ZombieType::Imp, reanimation_type: ReanimationType::ReanimImp, zombie_value: 10, starting_level: 48, first_allowed_wave: 1, pick_weight: 0, zombie_name: "IMP" },
    ZombieDefinition { zombie_type: ZombieType::Boss, reanimation_type: ReanimationType::ReanimBoss, zombie_value: 10, starting_level: 50, first_allowed_wave: 1, pick_weight: 0, zombie_name: "BOSS" },
];

/// 获取僵尸定义（对应 C++ GetZombieDefinition）
pub fn get_zombie_definition(ztype: ZombieType) -> &'static ZombieDefinition {
    &ZOMBIE_DEFS[ztype as usize]
}

impl Default for Zombie {
    fn default() -> Self {
        Zombie::new()
    }
}

// --- Zombie 类常量（对应 C++ Zombie 内部枚举） ---
impl Zombie {
    pub const ZOMBIE_WAVE_DEBUG: i32 = -1;
    pub const ZOMBIE_WAVE_CUTSCENE: i32 = -2;
    pub const ZOMBIE_WAVE_UI: i32 = -3;
    pub const ZOMBIE_WAVE_WINNER: i32 = -4;
}
