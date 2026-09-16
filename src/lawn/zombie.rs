// PvZ Portable Rust 翻译 — Zombie（僵尸类）
// 对应 C++ src/Lawn/Zombie.h / Zombie.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::lawn::plant::Plant;
use crate::todlib::reanimator::{Reanimation, ReanimationType, ReanimLoopType};
use crate::todlib::tod_particle::ParticleSystem;
use crate::todlib::attachment::{Attachment, attach_reanim, find_reanim_attachment};
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::rect::Rect;
use crate::framework::color::Color;
use crate::framework::sexy_matrix::SexyMatrix3;
use crate::framework::common::{Rand, RandRange, RandFloat};
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::board::Board;
use crate::lawn::zombatar::*;

pub const MAX_ZOMBIE_FOLLOWERS: usize = 4;
pub const NUM_BACKUP_DANCERS: usize = 4;
pub const ZOMBIE_BACKUP_DANCER_RISE_HEIGHT: i32 = -200;
pub const BUNGEE_ZOMBIE_HEIGHT: i32 = 3000;
pub const ZOMBIE_LIMP_SPEED_FACTOR: i32 = 2;
pub const POGO_BOUNCE_TIME: i32 = 80;
pub const DOLPHIN_JUMP_TIME: i32 = 120;
pub const BOBSLED_CRASH_TIME: i32 = 150;
pub const CHILLED_SPEED_FACTOR: f32 = 0.4;
pub const THOWN_ZOMBIE_GRAVITY: f32 = 0.05;
pub const HIGH_GROUND_HEIGHT: f32 = 30.0; // C++ GameConstants.h HIGH_GROUND_HEIGHT = 30
pub const DAMAGE_PER_EAT: i32 = 4; // C++ TICKS_BETWEEN_EATS / DAMAGE_PER_EAT
pub const NUM_BOSS_BUNGEES: usize = 3; // C++ Zombie.h NUM_BOSS_BUNGEES

/// Boss 召唤僵尸列表（对应 C++ gBossZombieList）
pub const BOSS_ZOMBIE_LIST: [ZombieType; 12] = [
    ZombieType::TrafficCone, ZombieType::Pail, ZombieType::Football, ZombieType::Polevaulter,
    ZombieType::JackInTheBox, ZombieType::Ladder, ZombieType::Zamboni, ZombieType::Catapult,
    ZombieType::Pogo, ZombieType::Newspaper, ZombieType::Door, ZombieType::Gargantuar,
];

// C++ Zombie.cpp 文件级常量
const CLIP_HEIGHT_LIMIT: f32 = -100.0; // C++ constexpr CLIP_HEIGHT_LIMIT = -100.0f
const ZOMBIE_MINDCONTROLLED_COLOR: Color = Color { r: 128, g: 64, b: 192, a: 255 }; // C++ Color(128, 64, 192, 255)

/// 缩放旋转矩阵（对应 C++ PvzpLib/PvzpCommon.cpp PvzpScaleRotateTransformMatrix）
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

// C++ DamageFlags 位索引（对应 ConstEnums.h DamageFlags）
const DAMAGE_BYPASSES_SHIELD: u32 = 0;
const DAMAGE_HITS_SHIELD_AND_BODY: u32 = 1;
const DAMAGE_FREEZE: u32 = 2;
const DAMAGE_DOESNT_CAUSE_FLASH: u32 = 3;
const DAMAGE_DOESNT_LEAVE_BODY: u32 = 4;
const DAMAGE_SPIKE: u32 = 5;

/// 测试位标志（对应 C++ TestBit）
pub fn test_bit(flags: u32, bit: u32) -> bool {
    (flags & (1 << bit)) != 0
}

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
    pub zombatar_head_reanim_id: ReanimationID,
    pub fireball_row: i32,
    pub is_fire_ball: bool,
    pub mowered_reanim_id: ReanimationID,
    pub last_portal_x: i32,
}

impl Zombie {
    /// 预加载僵尸资源（对应 C++ Zombie::PreloadZombieResources）
    pub fn preload_zombie_resources(zombie_type: ZombieType) {
        use crate::todlib::reanim_loader::reanimator_ensure_definition_loaded;

        let a_zombie_def = get_zombie_definition(zombie_type);
        if a_zombie_def.reanimation_type != ReanimationType::None {
            reanimator_ensure_definition_loaded(a_zombie_def.reanimation_type);
        }

        if zombie_type == ZombieType::Digger {
            reanimator_ensure_definition_loaded(ReanimationType::DiggerDirt);
            reanimator_ensure_definition_loaded(ReanimationType::ZombieCharredDigger);
        } else if zombie_type == ZombieType::Boss {
            reanimator_ensure_definition_loaded(ReanimationType::BossDriver);
            reanimator_ensure_definition_loaded(ReanimationType::BossFireball);
            reanimator_ensure_definition_loaded(ReanimationType::BossIceball);

            for &a_zombie_type in BOSS_ZOMBIE_LIST.iter() {
                let a_def = get_zombie_definition(a_zombie_type);
                reanimator_ensure_definition_loaded(a_def.reanimation_type);
            }
        } else if zombie_type == ZombieType::Dancer {
            reanimator_ensure_definition_loaded(ReanimationType::BackupDancer);
        } else if zombie_type == ZombieType::Gargantuar || zombie_type == ZombieType::RedeEyeGargantuar {
            reanimator_ensure_definition_loaded(ReanimationType::Imp);
            reanimator_ensure_definition_loaded(ReanimationType::ZombieCharredImp);
            reanimator_ensure_definition_loaded(ReanimationType::ZombieCharredGargantuar);
        } else if zombie_type == ZombieType::Zamboni {
            reanimator_ensure_definition_loaded(ReanimationType::Imp);
            reanimator_ensure_definition_loaded(ReanimationType::ZombieCharredZamboni);
        } else if zombie_type == ZombieType::Catapult {
            reanimator_ensure_definition_loaded(ReanimationType::ZombieCharredCatapult);
        }

        reanimator_ensure_definition_loaded(ReanimationType::Puff);
        reanimator_ensure_definition_loaded(ReanimationType::ZombieCharred);
        reanimator_ensure_definition_loaded(ReanimationType::LawnMoweredZombie);
    }

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
            zombatar_head_reanim_id: REANIMATIONID_NULL,
            fireball_row: 0,
            is_fire_ball: false,
            mowered_reanim_id: REANIMATIONID_NULL,
            last_portal_x: 0,
        }
    }

    /// 初始化僵尸（对应 C++ Zombie::ZombieInitialize）
    pub fn zombie_initialize(&mut self, row: i32, ztype: ZombieType, variant: bool, parent: Option<&mut Zombie>, from_wave: i32) {
        self.zombie_type = ztype;
        self.variant = variant;
        self.base.row = row;
        self.from_wave = from_wave;

        // C++: mPosX = 780 + Rand(ZOMBIE_START_RANDOM_OFFSET)
        let start_x = 780 + RandRange(40);
        self.pos_x = start_x as f32;
        self.pos_y = self.get_pos_y_based_on_row(row);
        self.vel_x = 0.0;
        self.vel_z = 0.0;
        self.base.width = 120;
        self.base.height = 120;
        self.frame = 0;
        self.prev_frame = 0;
        self.is_eating = false;
        self.just_got_shot_counter = 0;
        self.shield_just_got_shot_counter = 0;
        self.shield_recoil_counter = 0;
        self.chilled_counter = 0;
        self.ice_trap_counter = 0;
        self.buttered_counter = 0;
        self.mind_controlled = false;
        self.blowing_away = false;
        self.has_head = true;
        self.has_arm = true;
        self.has_object = false;
        self.in_pool = false;
        self.on_high_ground = false;
        self.helm_type = HelmType::None;
        self.shield_type = ShieldType::None;
        self.yucky_face = false;
        self.yucky_face_counter = 0;
        self.anim_counter = 0;
        self.groan_counter = RandRange(300) + 100;
        self.anim_ticks_per_frame = 12;
        self.anim_frames = 12;
        self.zombie_age = 0;
        self.target_col = -1;
        self.zombie_phase = ZombiePhase::Normal;
        self.zombie_height = ZombieHeight::Normal;
        self.phase_counter = 0;
        self.hit_umbrella = false;
        self.dropped_loot = false;
        self.related_zombie_id = ZOMBIEID_NULL;
        self.zombie_rect = Rect::new(36, 0, 42, 115);
        self.zombie_attack_rect = Rect::new(50, 0, 20, 115);
        self.playing_song = false;
        self.zombie_fade = -1;
        self.flat_tires = false;
        self.scale_zombie = 1.0;
        self.use_ladder_col = -1;
        self.shield_health = 0;
        self.helm_health = 0;
        self.altitude = 0.0;
        self.flying_health = 0;
        self.original_anim_rate = 0.0;
        self.attachment_id = ATTACHMENTID_NULL;
        self.summon_counter = 0;
        self.boss_stomp_counter = -1;
        self.boss_bungee_counter = -1;
        self.boss_head_counter = -1;
        self.body_reanim_id = REANIMATIONID_NULL;
        self.target_plant_id = PLANTID_NULL;
        self.boss_mode = 0;
        self.boss_fire_ball_reanim_id = REANIMATIONID_NULL;
        self.special_head_reanim_id = REANIMATIONID_NULL;
        self.zombatar_head_reanim_id = REANIMATIONID_NULL;
        self.target_row = -1;
        self.fireball_row = -1;
        self.is_fire_ball = false;
        self.mowered_reanim_id = REANIMATIONID_NULL;
        self.last_portal_x = -1;
        for i in 0..MAX_ZOMBIE_FOLLOWERS {
            self.follower_zombie_ids[i] = ZOMBIEID_NULL;
        }

        // 旗帜波次偏移
        if let Some(board) = self.base.get_board() {
            if board.is_flag_wave(from_wave) {
                self.pos_x += 40.0;
            }
        }

        self.pick_random_speed();
        self.body_health = 270;

        let a_zombie_def = get_zombie_definition(ztype);
        let mut a_render_layer = RENDER_LAYER_ZOMBIE;
        let mut a_render_offset = 4;

        // 加载重动画
        if a_zombie_def.reanimation_type != ReanimationType::None {
            self.load_reanim(a_zombie_def.reanimation_type);
        }

        // 根据僵尸类型初始化
        match ztype {
            ZombieType::Normal => {
                self.load_plain_zombie_reanim();
            }
            ZombieType::DuckyTube => {
                self.load_plain_zombie_reanim();
            }
            ZombieType::TrafficCone => {
                self.load_plain_zombie_reanim();
                self.reanim_show_prefix("anim_cone", 0); // RENDER_GROUP_NORMAL
                self.reanim_show_prefix("anim_hair", -1); // RENDER_GROUP_HIDDEN
                self.helm_type = HelmType::TrafficCone;
                self.helm_health = 370;
            }
            ZombieType::Pail => {
                self.load_plain_zombie_reanim();
                self.reanim_show_prefix("anim_bucket", 0); // RENDER_GROUP_NORMAL
                self.reanim_show_prefix("anim_hair", -1); // RENDER_GROUP_HIDDEN
                self.helm_type = HelmType::Pail;
                self.helm_health = 1100;
            }
            ZombieType::Door => {
                self.shield_type = ShieldType::Door;
                self.shield_health = 1100;
                self.load_plain_zombie_reanim();
                self.attach_shield();
            }
            ZombieType::Yeti => {
                self.body_health = 1350;
                self.phase_counter = RandRange(500) + 1000;
                self.has_object = true;
                self.zombie_attack_rect = Rect::new(20, 0, 50, 115);
            }
            ZombieType::Ladder => {
                self.body_health = 500;
                self.shield_health = 500;
                self.shield_type = ShieldType::Ladder;
                self.zombie_attack_rect = Rect::new(10, 0, 50, 115);
                if self.is_on_board() {
                    self.zombie_phase = ZombiePhase::LadderCarrying;
                    self.start_walk_anim(0);
                }
                self.attach_shield();
            }
            ZombieType::Bungee => {
                self.body_health = 450;
                self.anim_frames = 4;
                self.altitude = BUNGEE_ZOMBIE_HEIGHT as f32 + RandRange(150) as f32;
                self.vel_x = 0.0;
                if self.is_on_board() {
                    self.pick_bungee_zombie_target(-1);
                    if self.dead {
                        return;
                    }
                    self.zombie_phase = ZombiePhase::BungeeDiving;
                } else {
                    self.zombie_phase = ZombiePhase::BungeeCutscene;
                    self.phase_counter = RandRange(200);
                }
                self.play_zombie_reanim("anim_drop", ReanimLoopType::Loop, 0, 24.0);
                a_render_layer = RENDER_LAYER_GRAVE_STONE;
                a_render_offset = 7;
                self.zombie_rect = Rect::new(-20, 22, 110, 94);
                self.zombie_attack_rect = Rect::new(0, 0, 0, 0);
                self.variant = false;
            }
            ZombieType::Football => {
                self.zombie_rect = Rect::new(50, 0, 57, 115);
                self.reanim_show_prefix("anim_hair", -1); // RENDER_GROUP_HIDDEN
                self.helm_type = HelmType::FootballHelmet;
                self.helm_health = 1400;
                self.anim_ticks_per_frame = 6;
                self.variant = false;
            }
            ZombieType::Digger => {
                self.helm_type = HelmType::Digger;
                self.helm_health = 100;
                self.variant = false;
                self.has_object = true;
                self.zombie_rect = Rect::new(50, 0, 28, 115);
                if !self.is_on_board() {
                    self.zombie_phase = ZombiePhase::DiggerCutscene;
                } else {
                    self.zombie_phase = ZombiePhase::DiggerTunneling;
                    self.add_attached_particle(60, 100, ParticleEffect::DiggerTunnel);
                    a_render_offset = 7;
                    self.play_zombie_reanim("anim_dig", ReanimLoopType::LoopFullOffset, 0, 12.0);
                    self.pick_random_speed();
                }
            }
            ZombieType::Polevaulter => {
                self.body_health = 500;
                self.anim_ticks_per_frame = 6;
                self.zombie_phase = ZombiePhase::PolevaulterPreVault;
                self.has_object = true;
                self.variant = false;
                self.pos_x = 900.0 + RandRange(10) as f32; // WIDE_BOARD_WIDTH + 70
                if self.is_on_board() {
                    self.play_zombie_reanim("anim_run", ReanimLoopType::Loop, 0, 0.0);
                    self.pick_random_speed();
                }
                self.zombie_attack_rect = Rect::new(-29, 0, 70, 115);
            }
            ZombieType::DolphinRider => {
                self.body_health = 500;
                self.anim_ticks_per_frame = 6;
                self.zombie_phase = ZombiePhase::DolphinWalking;
                self.variant = false;
                if self.is_on_board() {
                    self.play_zombie_reanim("anim_walkdolphin", ReanimLoopType::Loop, 0, 0.0);
                    self.pick_random_speed();
                }
                self.setup_water_track("zombie_dolphinrider_whitewater");
                self.setup_water_track("zombie_dolphinrider_dolphininwater");
            }
            ZombieType::Gargantuar | ZombieType::RedeEyeGargantuar => {
                self.base.width = 180;
                self.base.height = 180;
                self.body_health = 3000;
                self.anim_frames = 24;
                self.anim_ticks_per_frame = 8;
                self.pos_x = 900.0 + RandRange(10) as f32; // WIDE_BOARD_WIDTH + 45
                self.zombie_rect = Rect::new(-17, -38, 125, 154);
                self.zombie_attack_rect = Rect::new(-30, -38, 89, 154);
                self.variant = false;
                a_render_offset = 8;
                self.has_object = true;
                if ztype == ZombieType::RedeEyeGargantuar {
                    self.body_health = 6000;
                }
            }
            ZombieType::Zamboni => {
                self.body_health = 1350;
                self.anim_frames = 2;
                self.anim_ticks_per_frame = 8;
                self.pos_x = 900.0 + RandRange(10) as f32; // WIDE_BOARD_WIDTH + Rand(10)
                a_render_offset = 8;
                self.play_zombie_reanim("anim_drive", ReanimLoopType::Loop, 0, 12.0);
                self.zombie_rect = Rect::new(0, -13, 153, 140);
                self.zombie_attack_rect = Rect::new(10, -13, 133, 140);
                self.variant = false;
            }
            ZombieType::Catapult => {
                self.body_health = 850;
                self.pos_x = 900.0 + 25.0 + RandRange(10) as f32;
                self.summon_counter = 20;
                if self.is_on_board() {
                    self.play_zombie_reanim("anim_walk", ReanimLoopType::Loop, 0, 5.5);
                } else {
                    self.play_zombie_reanim("anim_idle", ReanimLoopType::Loop, 0, 8.0);
                }
                self.zombie_rect = Rect::new(0, -13, 153, 140);
                self.zombie_attack_rect = Rect::new(10, -13, 133, 140);
                self.variant = false;
            }
            ZombieType::Snorkel => {
                self.zombie_rect = Rect::new(12, 0, 62, 115);
                self.zombie_attack_rect = Rect::new(-5, 0, 55, 115);
                self.setup_water_track("Zombie_snorkle_whitewater");
                self.setup_water_track("Zombie_snorkle_whitewater2");
                self.variant = false;
                self.zombie_phase = ZombiePhase::SnorkelWalking;
            }
            ZombieType::JackInTheBox => {
                self.body_health = 500;
                self.anim_ticks_per_frame = 6;
                let a_distance = 450 + RandRange(300);
                if RandRange(20) == 0 {
                    self.phase_counter = ((a_distance as f32 / 3.0 / (self.vel_x * ZOMBIE_LIMP_SPEED_FACTOR as f32)) as i32)
                        .max(1);
                } else {
                    self.phase_counter = (a_distance as f32 / self.vel_x * ZOMBIE_LIMP_SPEED_FACTOR as f32) as i32;
                }
                self.zombie_attack_rect = Rect::new(20, 0, 50, 115);
                if self.is_on_board() {
                    self.zombie_phase = ZombiePhase::JackInTheBoxRunning;
                }
            }
            ZombieType::Bobsled => {
                a_render_offset = 3;
                if let Some(parent_zombie) = parent {
                    let mut a_position = 0;
                    while a_position < 3 && parent_zombie.follower_zombie_ids[a_position] != ZOMBIEID_NULL {
                        a_position += 1;
                    }
                    // 注意：这里需要 Board 的 ZombieGetID 方法
                    // 暂时跳过设置 follower
                    self.pos_x = parent_zombie.pos_x + (a_position + 1) as f32 * 50.0;
                    if a_position == 0 {
                        a_render_offset = 1;
                        self.altitude = 9.0;
                    } else if a_position == 1 {
                        a_render_offset = 2;
                        self.altitude = -7.0;
                    } else {
                        a_render_offset = 0;
                        self.altitude = 9.0;
                    }
                } else {
                    self.pos_x = 900.0 + 80.0;
                    self.zombie_rect = Rect::new(-50, 0, 275, 115);
                    // 依赖底层系统
                    self.helm_health = 300;
                    self.altitude = -10.0;
                }
                self.vel_x = 0.6;
                self.zombie_phase = ZombiePhase::BobsledSliding;
                self.phase_counter = 500;
                self.variant = false;
                if from_wave == Zombie::ZOMBIE_WAVE_CUTSCENE {
                    self.play_zombie_reanim("anim_jump", ReanimLoopType::PlayOnceAndHold, 0, 20.0);
                    self.altitude = 18.0;
                } else if self.is_on_board() {
                    self.play_zombie_reanim("anim_push", ReanimLoopType::Loop, 0, 30.0);
                }
            }
            ZombieType::Flag => {
                self.has_object = true;
                self.load_plain_zombie_reanim();
                self.pos_x = 900.0; // WIDE_BOARD_WIDTH
            }
            ZombieType::Pogo => {
                self.variant = false;
                self.zombie_phase = ZombiePhase::PogoBouncing;
                self.phase_counter = RandRange(POGO_BOUNCE_TIME) + 1;
                self.has_object = true;
                self.body_health = 500;
                self.zombie_attack_rect = Rect::new(10, 0, 30, 115);
                self.play_zombie_reanim("anim_pogo", ReanimLoopType::PlayOnceAndHold, 0, 40.0);
            }
            ZombieType::Newspaper => {
                self.zombie_attack_rect = Rect::new(20, 0, 50, 115);
                self.zombie_phase = ZombiePhase::NewspaperReading;
                self.shield_type = ShieldType::Newspaper;
                self.shield_health = 150;
                self.variant = false;
                self.attach_shield();
            }
            ZombieType::Balloon => {
                if self.is_on_board() {
                    self.altitude = 25.0;
                    self.zombie_phase = ZombiePhase::BalloonFlying;
                    self.play_zombie_reanim("anim_idle", ReanimLoopType::Loop, 0, 0.0);
                } else {
                    let a_anim_rate = 8.0 + RandFloat(2.0);
                    self.set_anim_rate(a_anim_rate);
                }
                self.flying_health = 20;
                self.zombie_rect = Rect::new(36, 30, 42, 115);
                self.zombie_attack_rect = Rect::new(20, 30, 50, 115);
                self.variant = false;
            }
            ZombieType::Dancer => {
                self.scale_zombie = 0.8;
                if !self.is_on_board() {
                    self.play_zombie_reanim("anim_armraise", ReanimLoopType::Loop, 0, 12.0);
                } else {
                    self.zombie_phase = ZombiePhase::DancerDancingIn;
                    self.vel_x = 0.5;
                    self.phase_counter = 300 + RandRange(12);
                    self.play_zombie_reanim("anim_moonwalk", ReanimLoopType::Loop, 0, 24.0);
                }
                self.body_health = 500;
                self.variant = false;
            }
            ZombieType::BackupDancer => {
                self.scale_zombie = 0.8;
                if !self.is_on_board() {
                    self.play_zombie_reanim("anim_armraise", ReanimLoopType::Loop, 0, 12.0);
                }
                self.zombie_phase = ZombiePhase::DancerDancingLeft;
                self.variant = false;
            }
            ZombieType::Imp => {
                if !self.is_on_board() {
                    self.play_zombie_reanim("anim_walk", ReanimLoopType::Loop, 0, 12.0);
                }
            }
            ZombieType::Boss => {
                self.pos_x = 0.0;
                self.pos_y = 0.0;
                self.zombie_rect = Rect::new(700, 80, 90, 430);
                self.zombie_attack_rect = Rect::new(0, 0, 0, 0);
                a_render_layer = RENDER_LAYER_TOP;
                self.body_health = 40000;
                if self.is_on_board() {
                    self.play_zombie_reanim("anim_enter", ReanimLoopType::PlayOnceAndHold, 0, 12.0);
                    self.summon_counter = 500;
                    self.boss_head_counter = 5000;
                    self.zombie_phase = ZombiePhase::BossEnter;
                } else {
                    self.play_zombie_reanim("anim_head_idle", ReanimLoopType::Loop, 0, 12.0);
                }
                self.boss_setup_reanim();
            }
            ZombieType::PeaHead => {
                self.load_plain_zombie_reanim();
                self.reanim_show_prefix("anim_hair", -1);
                self.reanim_show_prefix("anim_head2", -1);
                self.phase_counter = 150;
                self.variant = false;
            }
            ZombieType::WallnutHead => {
                self.load_plain_zombie_reanim();
                self.reanim_show_prefix("anim_hair", -1);
                self.reanim_show_prefix("anim_head", -1);
                // 依赖底层系统
                self.helm_health = 1100;
                self.variant = false;
            }
            ZombieType::TallnutHead => {
                self.load_plain_zombie_reanim();
                self.reanim_show_prefix("anim_hair", -1);
                self.reanim_show_prefix("anim_head", -1);
                // 依赖底层系统
                self.helm_health = 2200;
                self.variant = false;
                self.pos_x += 30.0;
            }
            ZombieType::JalapenoHead => {
                self.load_plain_zombie_reanim();
                self.reanim_show_prefix("anim_hair", -1);
                self.reanim_show_prefix("anim_head", -1);
                self.variant = false;
                self.body_health = 500;
                let a_distance = 275 + RandRange(175);
                self.phase_counter = (a_distance as f32 / self.vel_x * ZOMBIE_LIMP_SPEED_FACTOR as f32) as i32;
            }
            ZombieType::GatlingHead => {
                self.load_plain_zombie_reanim();
                self.reanim_show_prefix("anim_hair", -1);
                self.reanim_show_prefix("anim_head2", -1);
                self.phase_counter = 150;
                self.variant = false;
            }
            ZombieType::SquashHead => {
                self.load_plain_zombie_reanim();
                self.reanim_show_prefix("anim_hair", -1);
                self.reanim_show_prefix("anim_head2", -1);
                self.zombie_phase = ZombiePhase::SquashPreLaunch;
                self.variant = false;
            }
            // 对应 C++ Zombie.cpp:851-855：case ZOMBIE_CACHED_POLEVAULTER_WITH_POLE /
            // NUM_ZOMBIE_TYPES / NUM_CACHED_ZOMBIE_TYPES / ZOMBIE_INVALID 均为空分支（break），
            // 该类型不作为实际出场僵尸，仅用于 ReanimatorCache 缓存带杆帧
            ZombieType::CachedPolevaulterWithPole => {}
            _ => {}
        }

        // 后处理
        if let Some(app) = self.base.get_app() {
            if self.is_on_board() && app.game_mode == GameMode::ChallengeZombiquarium {
                self.play_zombie_reanim("anim_aquarium_swim", ReanimLoopType::Loop, 0, 8.0 + RandFloat(2.0));
                self.zombie_height = ZombieHeight::Zombiquarium;
                self.zombie_phase = ZombiePhase::ZombiquariumDrift;
                self.phase_counter = 200;
                self.body_health = 200;
                self.summon_counter = RandRange(200) + 200;
            }
        }

        // LittleTrouble 模式缩放（对应 C++ IsLittleTroubleLevel）
        // 依赖底层系统
        /*if let Some(app) = self.base.get_app() {
            if app.is_little_trouble_level() && (self.is_on_board() || from_wave == Zombie::ZOMBIE_WAVE_CUTSCENE) {
                self.scale_zombie = 0.5;
                self.body_health /= 4;
                self.helm_health /= 4;
                self.shield_health /= 4;
                self.flying_health /= 4;
            }
        }*/

        self.update_anim_speed();
        if self.variant {
            self.reanim_show_prefix("anim_tongue", 0); // RENDER_GROUP_NORMAL
        }

        self.body_max_health = self.body_health;
        self.helm_max_health = self.helm_health;
        self.shield_max_health = self.shield_health;
        self.flying_max_health = self.flying_health;
        self.dead = false;
        self.base.x = self.pos_x as i32;
        self.base.y = self.pos_y as i32;
        self.base.render_order = crate::lawn::board::make_render_order(a_render_layer, row, a_render_offset);

        if self.zombie_height == ZombieHeight::Zombiquarium {
            self.body_max_health = 300;
        }

        if self.is_on_board() {
            self.play_zombie_appear_sound();
        }

        self.update_reanim();
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

    pub fn pick_random_speed(&mut self) {
        // 对应 C++ Zombie::PickRandomSpeed：按僵尸类型/阶段设置 mVelX 与 mAnimTicksPerFrame
        if self.zombie_phase == ZombiePhase::SnorkelWalkingInPool {
            self.vel_x = 0.3;
        } else if self.zombie_phase == ZombiePhase::DiggerWalking {
            if let Some(app) = self.base.get_app() {
                if app.is_izombie_level() {
                    self.vel_x = 0.23;
                } else {
                    self.vel_x = 0.12;
                }
            } else {
                self.vel_x = 0.12;
            }
        } else if self.zombie_type == ZombieType::Imp && self.base.get_app().map_or(false, |a| a.is_izombie_level()) {
            self.vel_x = 0.9;
        } else if self.zombie_phase == ZombiePhase::YetiRunning {
            self.vel_x = 0.8;
        } else if self.zombie_type == ZombieType::Yeti {
            self.vel_x = 0.4;
        } else if self.zombie_type == ZombieType::Dancer
            || self.zombie_type == ZombieType::BackupDancer
            || self.zombie_type == ZombieType::Pogo
            || self.zombie_type == ZombieType::Flag
        {
            self.vel_x = 0.45;
        } else if self.zombie_phase == ZombiePhase::DiggerTunneling
            || self.zombie_phase == ZombiePhase::PolevaulterPreVault
            || self.zombie_type == ZombieType::Football
            || self.zombie_type == ZombieType::Snorkel
            || self.zombie_type == ZombieType::JackInTheBox
        {
            self.vel_x = 0.66 + RandFloat(0.02);
        } else if self.zombie_phase == ZombiePhase::LadderCarrying || self.zombie_type == ZombieType::SquashHead {
            self.vel_x = 0.79 + RandFloat(0.02);
        } else if self.zombie_phase == ZombiePhase::NewspaperMad
            || self.zombie_phase == ZombiePhase::DolphinWalking
            || self.zombie_phase == ZombiePhase::DolphinWalkingWithoutDolphin
        {
            self.vel_x = 0.89 + RandFloat(0.02);
        } else {
            self.vel_x = 0.23 + RandFloat(0.14);
            if self.vel_x < 0.3 {
                self.anim_ticks_per_frame = 12;
            } else {
                self.anim_ticks_per_frame = 15;
            }
        }

        self.update_anim_speed();
    }

    /// 更新僵尸（对应 C++ Zombie::Update）
    pub fn update(&mut self) {
        self.zombie_age += 1;
        let mut do_update = false;
        // 依赖底层系统
        // if app.game_scene == GameScenes::LevelIntro && self.zombie_type == ZombieType::Boss { do_update = true; }
        if self.is_on_board() {
            if let Some(board) = self.base.get_board() {
                if board.m_cut_scene.map_or(false, |c| unsafe { (*c).should_run_upsell_board() }) {
                    do_update = true;
                }
            }
        }
        if let Some(app) = self.base.get_app() {
            if app.game_scene == crate::lawn::lawn_app::GameScenes::Playing || !self.is_on_board() || self.from_wave == Zombie::ZOMBIE_WAVE_WINNER {
                do_update = true;
            }
        }

        if do_update {
            if self.zombie_phase == ZombiePhase::Burned {
                self.update_burn();
            } else if self.zombie_phase == ZombiePhase::Mowered {
                self.update_mowered();
            } else if self.zombie_phase == ZombiePhase::Dying {
                self.update_death();
                self.update_zombie_walking();
            } else {
                if self.phase_counter > 0 && !self.is_immobilized() {
                    self.phase_counter -= 1;
                }

                // 依赖底层系统
                if self.is_on_board() {
                    self.update_playing();
                }

                if self.zombie_type == ZombieType::Bungee {
                    self.update_zombie_bungee();
                }
                if self.zombie_type == ZombieType::Pogo {
                    self.update_zombie_pogo();
                }

                self.animate();
            }

            self.just_got_shot_counter -= 1;
            if self.shield_just_got_shot_counter > 0 {
                self.shield_just_got_shot_counter -= 1;
            }
            if self.shield_recoil_counter > 0 {
                self.shield_recoil_counter -= 1;
            }
            if self.zombie_fade > 0 {
                self.zombie_fade -= 1;
                if self.zombie_fade == 0 {
                    self.die_no_loot();
                }
            }

            self.base.x = self.pos_x as i32;
            self.base.y = self.pos_y as i32;

            // 同步身体 reanim 位置（对应 C++ UpdateReanim 中 SetPosition）
            if self.body_reanim_id != REANIMATIONID_NULL {
                if let Some(app) = self.base.get_app_mut() {
                    if let Some(reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                        reanim.set_position(self.pos_x, self.pos_y);
                    }
                }
            }

            self.update_reanim();
        }
    }

    /// 更新燃烧效果（对应 C++ Zombie::UpdateBurn）
    fn update_burn(&mut self) {
        self.phase_counter -= 1;
        if self.phase_counter == 0 {
            self.die_with_loot();
        }
    }

    /// 更新被碾压效果（对应 C++ Zombie::UpdateMowered）
    fn update_mowered(&mut self) {
        // C++: mApp->ReanimationTryToGet(mMoweredReanimID) 为空或循环完成（mLoopCount > 0）时
        //      掉落头/手臂并死亡掉物
        let a_mowered_done = self.base.get_app().map_or(true, |app| {
            app.reanimation_get(self.mowered_reanim_id)
                .map_or(true, |r| r.m_loop_count > 0)
        });
        if a_mowered_done {
            self.drop_head(0);
            self.drop_arm(0);
            self.die_with_loot();
        }
    }

    /// 更新僵尸的 Playing 阶段行为（对应 C++ Zombie::UpdatePlaying）
    fn update_playing(&mut self) {
        self.groan_counter -= 1;

        // 冰陷阱递减
        if self.ice_trap_counter > 0 {
            self.ice_trap_counter -= 1;
            if self.ice_trap_counter == 0 {
                self.remove_ice_trap();
                self.add_attached_particle(75, 106, ParticleEffect::IceTrapRelease);
            }
        }
        // 冻结递减
        if self.chilled_counter > 0 {
            self.chilled_counter -= 1;
            if self.chilled_counter == 0 {
                self.update_anim_speed();
            }
        }
        // 黄油递减
        if self.buttered_counter > 0 {
            self.buttered_counter -= 1;
            if self.buttered_counter == 0 {
                self.remove_butter();
            }
        }

        // 从墓碑升起
        if self.zombie_phase == ZombiePhase::RisingFromGrave {
            self.update_zombie_rise_from_grave();
            return;
        }

        if !self.is_immobilized() {
            self.update_actions();
            self.update_zombie_position();
            self.check_for_pool();
            self.check_for_high_ground();
            self.check_for_board_edge();
        }

        if self.zombie_type == ZombieType::Boss {
            self.update_boss();
        }

        if !self.is_dead_or_dying() && self.from_wave != Zombie::ZOMBIE_WAVE_WINNER {
            let is_dying = if !self.has_head {
                true
            } else if self.zombie_type == ZombieType::Zamboni || self.zombie_type == ZombieType::Catapult {
                self.body_health < 200
            } else {
                false
            };

            if is_dying {
                let mut a_damage = 1;
                if self.zombie_type == ZombieType::Yeti {
                    a_damage = 10;
                }
                if self.body_max_health >= 500 {
                    a_damage = 3;
                }
                if RandRange(5) == 0 {
                    self.take_damage(a_damage, 9);
                }
            }
        }
    }

    /// 更新僵尸行为（对应 C++ Zombie::UpdateActions）
    pub fn update_actions(&mut self) {
        if self.zombie_height == ZombieHeight::UpLadder {
            self.update_climbing_ladder();
        }
        if self.zombie_height == ZombieHeight::OutOfPool || self.zombie_height == ZombieHeight::InToPool || self.in_pool {
            self.update_zombie_pool();
        }
        if self.zombie_height == ZombieHeight::UpToHighGround || self.zombie_height == ZombieHeight::DownOffHighGround {
            self.update_zombie_high_ground();
        }
        if self.zombie_height == ZombieHeight::Falling {
            self.update_zombie_falling();
        }
        if self.zombie_height == ZombieHeight::InToChimney {
            self.update_zombie_chimney();
        }

        if self.zombie_type == ZombieType::Polevaulter {
            self.update_zombie_polevaulter();
        }
        if self.zombie_type == ZombieType::Catapult {
            self.update_zombie_catapult();
        }
        if self.zombie_type == ZombieType::DolphinRider {
            self.update_zombie_dolphin_rider();
        }
        if self.zombie_type == ZombieType::Snorkel {
            self.update_zombie_snorkel();
        }
        if self.zombie_type == ZombieType::Balloon {
            self.update_zombie_flyer();
        }
        if self.zombie_type == ZombieType::Newspaper {
            self.update_zombie_newspaper();
        }
        if self.zombie_type == ZombieType::Digger {
            self.update_zombie_digger();
        }
        if self.zombie_type == ZombieType::JackInTheBox {
            self.update_zombie_jack_in_the_box();
        }
        if self.zombie_type == ZombieType::Gargantuar || self.zombie_type == ZombieType::RedeEyeGargantuar {
            self.update_zombie_gargantuar();
        }
        if self.zombie_type == ZombieType::Bobsled {
            self.update_zombie_bobsled();
        }
        if self.zombie_type == ZombieType::Zamboni {
            self.update_zamboni();
        }
        if self.zombie_type == ZombieType::Ladder {
            self.update_ladder();
        }
        if self.zombie_type == ZombieType::Yeti {
            self.update_yeti();
        }
        if self.zombie_type == ZombieType::Dancer {
            self.update_zombie_dancer();
        }
        if self.zombie_type == ZombieType::BackupDancer {
            self.update_zombie_backup_dancer();
        }
        if self.zombie_type == ZombieType::Imp {
            self.update_zombie_imp();
        }
    }

    /// 更新爬梯子（对应 C++ UpdateClimbingLadder）
    pub fn update_climbing_ladder(&mut self) {
        let mut a_dist_off_ground = self.altitude;
        if self.on_high_ground {
            a_dist_off_ground -= 100.0; // HIGH_GROUND_HEIGHT
        }
        let a_ladder_origin_x = self.base.x + (5.0 + a_dist_off_ground * 0.5) as i32;
        if let Some(board) = self.base.get_board() {
            if board.get_ladder_at(a_ladder_origin_x, self.base.row).is_none() {
                self.zombie_height = ZombieHeight::Falling;
                return;
            }
        }
        self.altitude += 0.8;
        if self.vel_x < 0.5 {
            self.pos_x -= 0.5;
        }
        let mut a_target_height = 90.0;
        if self.on_high_ground {
            a_target_height += 100.0; // HIGH_GROUND_HEIGHT
        }
        if self.altitude >= a_target_height {
            self.zombie_height = ZombieHeight::Falling;
        }
    }

    /// 检查棋盘边缘（对应 C++ CheckForBoardEdge）
    pub fn check_for_board_edge(&mut self) {
        if self.is_walking_backwards() && self.pos_x > 850.0 {
            self.die_no_loot();
            return;
        }

        let mut a_edge_x = -100; // BOARD_EDGE
        if self.zombie_type == ZombieType::Gargantuar || self.zombie_type == ZombieType::RedeEyeGargantuar || self.zombie_type == ZombieType::Polevaulter {
            a_edge_x = -150;
        } else if self.zombie_type == ZombieType::Catapult || self.zombie_type == ZombieType::Football || self.zombie_type == ZombieType::Zamboni {
            a_edge_x = -175;
        } else if self.zombie_type == ZombieType::BackupDancer || self.zombie_type == ZombieType::Dancer || self.zombie_type == ZombieType::Snorkel {
            a_edge_x = -130;
        }

        if self.base.x <= a_edge_x && self.has_head {
            if let Some(app) = self.base.get_app() {
                if app.is_izombie_level() {
                    self.die_no_loot();
                } else {
                    if let Some(board) = self.base.get_board_mut() {
                        board.zombies_won_no_zombie();
                    }
                }
            }
        }
        if self.base.x <= a_edge_x + 70 && !self.has_head {
            self.take_damage(1800, 9);
        }
    }

    /// 检查僵尸脚步声（对应 C++ Zombie::CheckForZombieStep，Zombie.cpp:4243-4249）
    pub fn check_for_zombie_step(&mut self) {
        if (self.zombie_type == ZombieType::Zamboni || self.zombie_type == ZombieType::Catapult) && !self.flat_tires {
            self.check_squish(ZombieAttackType::DriveOver);
        }
    }

    /// 更新僵尸位置（对应 C++ UpdateZombiePosition）
    pub fn update_zombie_position(&mut self) {
        if self.zombie_type == ZombieType::Bungee || self.zombie_type == ZombieType::Boss
            || self.zombie_phase == ZombiePhase::RisingFromGrave || self.zombie_height == ZombieHeight::Zombiquarium
        {
            return;
        }

        self.update_zombie_walking();
        self.check_for_zombie_step();

        if self.blowing_away {
            self.pos_x += 10.0;
            if self.base.x > 850 {
                self.die_with_loot();
                return;
            }
        }

        if self.zombie_height == ZombieHeight::Normal {
            let a_desired_y = self.get_pos_y_based_on_row(self.base.row);
            if self.pos_y < a_desired_y {
                let diff = a_desired_y - self.pos_y;
                self.pos_y += diff.min(1.0);
            } else if self.pos_y > a_desired_y {
                let diff = self.pos_y - a_desired_y;
                self.pos_y -= diff.min(1.0);
            }
        }
    }

    /// 移除冰陷阱（对应 C++ Zombie::RemoveIceTrap，Zombie.cpp:8372-8383）
    pub fn remove_ice_trap(&mut self) {
        self.ice_trap_counter = 0;
        if self.zombie_type == ZombieType::Balloon {
            self.balloon_propeller_hat_spin(true);
        }
        self.update_anim_speed();
        self.start_zombie_sound();
    }

    /// 移除黄油（对应 C++ Zombie::RemoveButter，Zombie.cpp:8485-8513）
    pub fn remove_butter(&mut self) {
        if self.zombie_type == ZombieType::Balloon {
            self.balloon_propeller_hat_spin(true);
        }

        // 对应 C++: IsZombotany(mZombieType) 时按头部动画调整特殊头部 reanim 速率
        if Self::is_zombotany(self.zombie_type) {
            let a_head_anim_rate = self
                .base
                .get_app()
                .and_then(|app| app.reanimation_get(self.special_head_reanim_id))
                .map(|a_head_reanim| {
                    if self.zombie_type == ZombieType::PeaHead
                        && a_head_reanim.is_anim_playing("anim_shooting")
                    {
                        35.0
                    } else if self.zombie_type == ZombieType::GatlingHead
                        && a_head_reanim.is_anim_playing("anim_shooting")
                    {
                        38.0
                    } else {
                        15.0
                    }
                });
            if let Some(a_rate) = a_head_anim_rate {
                if let Some(app) = self.base.get_app_mut() {
                    if let Some(a_head_reanim) = app.reanimation_get_mut(self.special_head_reanim_id)
                    {
                        a_head_reanim.m_anim_rate = a_rate;
                    }
                }
            }
        }

        self.buttered_counter = 0;
        self.update_anim_speed();
        self.start_zombie_sound();
    }

    /// 检查进入泳池（对应 C++ Zombie::CheckForPool，Zombie.cpp:6920-6957）
    pub fn check_for_pool(&mut self) {
        if !Self::zombie_type_can_go_in_pool(self.zombie_type) || self.is_flying() {
            return;
        }
        if self.zombie_type == ZombieType::DolphinRider || self.zombie_type == ZombieType::Snorkel {
            return;
        }
        if self.zombie_height == ZombieHeight::InToPool
            || self.zombie_height == ZombieHeight::OutOfPool
        {
            return;
        }

        // 对应 C++: IsPoolSquare(PixelToGridX(mX + 75, mY)) && IsPoolSquare(PixelToGridX(mX + 45, mY)) && mX < 680
        let a_is_pool_square = if let Some(b) = self.base.get_board() {
            let a_grid_x_left = b.pixel_to_grid_x(self.pos_x as i32 + 75, self.pos_y as i32);
            let a_grid_x_right = b.pixel_to_grid_x(self.pos_x as i32 + 45, self.pos_y as i32);
            b.is_pool_square(a_grid_x_left, self.base.row)
                && b.is_pool_square(a_grid_x_right, self.base.row)
                && self.pos_x < 680.0
        } else {
            false
        };

        if !self.in_pool && a_is_pool_square {
            let a_ice_trap_counter = self.base.get_board().map_or(0, |b| b.m_ice_trap_counter);
            if a_ice_trap_counter > 0 {
                // 对应 C++: mIceTrapCounter = mBoard->mIceTrapCounter; ApplyChill(true);
                self.ice_trap_counter = a_ice_trap_counter;
                self.apply_chill(true);
            } else {
                // 对应 C++: mZombieHeight = HEIGHT_IN_TO_POOL; mInPool = true; PoolSplash(true);
                self.zombie_height = ZombieHeight::InToPool;
                self.in_pool = true;
                self.pool_splash(true);
            }
        } else if self.in_pool && !a_is_pool_square {
            // 对应 C++: mZombieHeight = HEIGHT_OUT_OF_POOL; StartWalkAnim(0); PoolSplash(false);
            self.zombie_height = ZombieHeight::OutOfPool;
            self.start_walk_anim(0);
            self.pool_splash(false);
        }
    }

    /// 检查屋顶高台（对应 C++ Zombie::CheckForHighGround，Zombie.cpp:6967-6983）
    pub fn check_for_high_ground(&mut self) {
        if self.zombie_height != ZombieHeight::Normal || self.zombie_type == ZombieType::Bungee {
            return;
        }

        // 对应 C++: IsOnHighGround() 基于 grid square type，可双向升降
        let a_is_high_ground = self.is_on_high_ground();
        if !self.on_high_ground && a_is_high_ground {
            self.zombie_height = ZombieHeight::UpToHighGround;
            self.on_high_ground = true;
        } else if self.on_high_ground && !a_is_high_ground {
            self.zombie_height = ZombieHeight::DownOffHighGround;
        }
    }

    /// 更新从墓碑升起（对应 C++ UpdateZombieRiseFromGrave）
    pub fn update_zombie_rise_from_grave(&mut self) {
        // 对应 C++：本函数不递减 mPhaseCounter（统一在 Zombie::Update 中每帧递减一次）
        if self.in_pool {
            self.altitude = crate::todlib::tod_common::tod_animate_curve(
                50, 0, self.phase_counter, -150, -40, TodCurves::Linear,
            ) as f32 * self.scale_zombie;
        } else {
            self.altitude = crate::todlib::tod_common::tod_animate_curve(
                50, 0, self.phase_counter, -200, 0, TodCurves::Linear,
            ) as f32;
        }
        if self.phase_counter == 0 {
            self.zombie_phase = ZombiePhase::Normal;
            // 对应 C++: if (IsOnHighGround()) mAltitude = HIGH_GROUND_HEIGHT;
            if self.on_high_ground {
                self.altitude = HIGH_GROUND_HEIGHT;
            }
            // [TRANSLATION_NOTE]: C++ 中 mInPool 分支的 ReanimIgnoreClipRect("Zombie_duckytube", true)
            // 等 reanim 操作依赖贴图系统，Rust stub 暂缺。
        }
    }

    /// 更新泳池僵尸（对应 C++ Zombie::UpdateZombiePool，Zombie.cpp:3261-3293）
    pub fn update_zombie_pool(&mut self) {
        if self.zombie_height == ZombieHeight::OutOfPool {
            self.altitude += 1.0;
            if self.zombie_type == ZombieType::Snorkel { self.altitude += 1.0; }
            if self.altitude >= 0.0 { self.altitude = 0.0; self.zombie_height = ZombieHeight::Normal; self.in_pool = false; }
        } else if self.zombie_height == ZombieHeight::InToPool {
            self.altitude -= 1.0;
            // 对应 C++: int aDepth = -40 * mScaleZombie;
            let a_depth = -40.0 * self.scale_zombie;
            if self.altitude <= a_depth {
                self.altitude = a_depth;
                self.zombie_height = ZombieHeight::Normal;
                self.start_walk_anim(0);
            }
        } else if self.zombie_height == ZombieHeight::DraggedUnder {
            // 对应 C++: HEIGHT_DRAGGED_UNDER 仅下沉
            self.altitude -= 1.0;
        }
    }

    /// 更新屋顶高台僵尸（对应 C++ Zombie::UpdateZombieHighGround，Zombie.cpp:3295-3319）
    pub fn update_zombie_high_ground(&mut self) {
        if self.zombie_type == ZombieType::Pogo { return; }
        if self.zombie_height == ZombieHeight::UpToHighGround {
            self.altitude += 1.0;
            if self.altitude >= HIGH_GROUND_HEIGHT { self.altitude = HIGH_GROUND_HEIGHT; self.zombie_height = ZombieHeight::Normal; }
        } else if self.zombie_height == ZombieHeight::DownOffHighGround {
            self.altitude -= 1.0;
            if self.altitude <= 0.0 { self.altitude = 0.0; self.zombie_height = ZombieHeight::Normal; self.on_high_ground = false; }
        }
    }

    /// 更新掉落僵尸（对应 C++ Zombie::UpdateZombieFalling，Zombie.cpp:3321-3337）
    pub fn update_zombie_falling(&mut self) {
        self.altitude -= 1.0;
        if self.zombie_phase == ZombiePhase::PolevaulterPreVault { self.altitude -= 1.0; }

        // 对应 C++: 高台上时落点高度为 HIGH_GROUND_HEIGHT
        let mut a_ground_height = 0.0;
        if self.is_on_high_ground() {
            a_ground_height = HIGH_GROUND_HEIGHT;
        }
        if self.altitude <= a_ground_height { self.altitude = a_ground_height; self.zombie_height = ZombieHeight::Normal; }
    }

    /// 更新烟囱僵尸（对应 C++ Zombie::UpdateZombieChimney，Zombie.cpp:9625-9631）
    pub fn update_zombie_chimney(&mut self) {
        if let Some(b) = self.base.get_board() {
            if b.m_background_type == BackgroundType::Roof || b.m_background_type == BackgroundType::Boss {
                // 对应 C++: mAltitude = PvzpAnimateCurve(4000, 5000, mBoard->mCutScene->mCutsceneTime, 200, 0, CURVE_EASE_IN);
                let a_cutscene_time = b.m_cut_scene.map_or(0, |cs| unsafe { (*cs).m_cutscene_time });
                self.altitude = crate::todlib::tod_common::tod_animate_curve(
                    4000, 5000, a_cutscene_time, 200, 0, TodCurves::EaseIn,
                ) as f32;
            }
        }
    }

    /// 更新僵尸撑杆跳（对应 C++ UpdateZombiePolevaulter）
    pub fn update_zombie_polevaulter(&mut self) {
        if self.zombie_phase == ZombiePhase::PolevaulterPreVault && self.has_head && self.zombie_height == ZombieHeight::Normal {
            // 依赖底层系统
            // 有植物在前方 → 跳越
            let has_plant_ahead = self.base.x > 50 && self.base.x < 750;
            if has_plant_ahead {
                self.zombie_phase = ZombiePhase::PolevaulterInVault;
                self.play_zombie_reanim("anim_jump", ReanimLoopType::PlayOnceAndHold, 20, 24.0);
                self.has_object = false;
            }
        } else if self.zombie_phase == ZombiePhase::PolevaulterInVault {
            // 依赖底层系统
            // 简化处理：直接结束跳跃
            self.pos_x -= 150.0;
            self.base.x = self.pos_x as i32;
            self.zombie_phase = ZombiePhase::PolevaulterPostVault;
            self.zombie_attack_rect = Rect::new(50, 0, 20, 115);
            self.start_walk_anim(0);
        }
    }

    /// 投石车僵尸射击（对应 C++ ZombieCatapultFire）
    pub fn zombie_catapult_fire(&mut self, target_x: Option<i32>, target_y: Option<i32>) {
        let a_origin_x = self.pos_x + 113.0;
        let a_origin_y = self.pos_y - 44.0;
        let (a_target_x, a_target_y) = match (target_x, target_y) {
            (Some(tx), Some(ty)) => (tx as f32, ty as f32),
            _ => (self.pos_x - 300.0, 0.0),
        };

        // 抛投音效
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::Basketball as i32);
        }

        // 依赖底层系统
        // 的 ProjectileType::BASKETBALL 与 MOTION_LOBBED 设置需后续扩展 Projectile 系统
        let a_range_x = (a_origin_x - a_target_x - 20.0).max(40.0);
        let a_range_y = a_target_y - a_origin_y;
        // 计算抛物线参数（保留原始公式）
        let _vel_x = -a_range_x / 120.0;
        let _vel_z = a_range_y / 120.0 - 7.0;
        let _acc_z = 0.115;
        // 暂不实际发射投射物
    }

    /// 寻找投石目标（对应 C++ FindCatapultTarget）
    pub fn find_catapult_target(&self) -> bool {
        if let Some(board) = self.base.get_board() {
            for plant in &board.plants {
                if plant.dead {
                    continue;
                }
                if plant.base.row == self.base.row && self.base.x >= plant.base.x + 100 {
                    // 依赖底层系统
                    if plant.seed_type != SeedType::Spikeweed && plant.seed_type != SeedType::Spikerock {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// 更新投石车僵尸（对应 C++ UpdateZombieCatapult）
    pub fn update_zombie_catapult(&mut self) {
        if self.zombie_phase == ZombiePhase::Normal {
            if self.pos_x <= 650.0 && self.find_catapult_target() && self.summon_counter > 0 {
                self.zombie_phase = ZombiePhase::CatapultLaunching;
                self.phase_counter = 300;
                self.play_zombie_reanim("anim_shoot", ReanimLoopType::PlayOnceAndHold, 0, 24.0);
            }
        } else if self.zombie_phase == ZombiePhase::CatapultLaunching {
            // 依赖底层系统
            // 简化：anim_counter % 40 触发一次发射
            if self.anim_counter % 40 == 0 {
                let has_target = self.find_catapult_target();
                // 依赖底层系统
                // 当前简化为用僵尸前方的默认位置
                let target_x = if has_target { Some(self.base.x - 200) } else { None };
                self.zombie_catapult_fire(target_x, None);
            }

            // 动画循环结束后处理
            // 依赖底层系统
            if self.anim_counter % 120 == 0 {
                self.summon_counter -= 1;
                if self.summon_counter == 0 {
                    self.play_zombie_reanim("anim_walk", ReanimLoopType::Loop, 20, 6.0);
                    self.zombie_phase = ZombiePhase::Normal;
                } else {
                    self.play_zombie_reanim("anim_idle", ReanimLoopType::Loop, 20, 12.0);
                    self.zombie_phase = ZombiePhase::CatapultReloading;
                }
            }
        } else if self.zombie_phase == ZombiePhase::CatapultReloading && self.phase_counter == 0 {
            if self.find_catapult_target() {
                self.zombie_phase = ZombiePhase::CatapultLaunching;
                self.phase_counter = 300;
                self.play_zombie_reanim("anim_shoot", ReanimLoopType::PlayOnceAndHold, 20, 24.0);
            } else {
                self.play_zombie_reanim("anim_walk", ReanimLoopType::Loop, 20, 6.0);
                self.zombie_phase = ZombiePhase::Normal;
            }
        }
    }

    /// 更新海豚骑士（对应 C++ UpdateZombieDolphinRider）
    pub fn update_zombie_dolphin_rider(&mut self) {
        // 依赖底层系统
        let a_backwards = self.is_walking_backwards();

        if self.zombie_phase == ZombiePhase::DolphinWalking && !a_backwards {
            if self.base.x > 700 && self.base.x <= 720 {
                self.zombie_phase = ZombiePhase::DolphinIntoPool;
                self.play_zombie_reanim("anim_jumpinpool", ReanimLoopType::PlayOnceAndHold, 20, 16.0);
            }
        } else if self.zombie_phase == ZombiePhase::DolphinIntoPool {
            // 依赖底层系统
            // 依赖底层系统
            self.pos_x -= 70.0;
            self.zombie_phase = ZombiePhase::DolphinRiding;
            self.in_pool = true;
            self.zombie_attack_rect = Rect::new(-29, 0, 70, 115);
            self.play_zombie_reanim("anim_ride", ReanimLoopType::LoopFullOffset, 0, 12.0);
        } else if self.zombie_phase == ZombiePhase::DolphinRiding {
            if self.base.x <= 10 {
                self.altitude = -40.0;
                self.zombie_height = ZombieHeight::OutOfPool;
                self.zombie_phase = ZombiePhase::DolphinWalking;
                // 依赖底层系统
                self.play_zombie_reanim("anim_walkdolphin", ReanimLoopType::Loop, 0, 0.0);
                self.pick_random_speed();
                return;
            }

            if self.has_head {
                // 依赖底层系统
                // 若有植物在前方，触发跳跃
                let has_plant_ahead = self.base.x > 50 && self.base.x < 700;
                if has_plant_ahead {
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::DolphinBeforeJumping as i32);
                    }
                    self.vel_x = 0.5;
                    self.zombie_phase = ZombiePhase::DolphinInJump;
                    self.phase_counter = DOLPHIN_JUMP_TIME;
                    self.play_zombie_reanim("anim_dolphinjump", ReanimLoopType::PlayOnceAndHold, 0, 10.0);
                }
            }
        } else if self.zombie_phase == ZombiePhase::DolphinInJump {
            // 跳跃高度曲线
            self.altitude = crate::todlib::tod_common::tod_animate_curve_float(
                DOLPHIN_JUMP_TIME, 0, self.phase_counter, 0.0, 10.0, TodCurves::Linear,
            );

            // 依赖底层系统
            // 简化：phase_counter 为 0 时结束跳跃
            if self.phase_counter == 0 {
                self.zombie_attack_rect = Rect::new(30, 0, 30, 115);
                self.zombie_rect = Rect::new(20, 0, 42, 115);
                self.zombie_phase = ZombiePhase::DolphinWalkingInPool;
                self.start_walk_anim(0);
            }
        } else if self.zombie_phase == ZombiePhase::DolphinWalkingInPool {
            if (self.base.x <= 10 && !a_backwards) || (self.base.x > 680 && a_backwards) {
                self.altitude = -40.0;
                self.zombie_height = ZombieHeight::OutOfPool;
                self.zombie_phase = ZombiePhase::DolphinWalkingWithoutDolphin;
                // 依赖底层系统
                self.play_zombie_reanim("anim_walk", ReanimLoopType::Loop, 0, 0.0);
                self.pick_random_speed();
            }
        }
    }

    /// 更新潜水僵尸（对应 C++ UpdateZombieSnorkel）
    pub fn update_zombie_snorkel(&mut self) {
        let a_backwards = self.is_walking_backwards();

        if self.zombie_phase == ZombiePhase::SnorkelWalking && !a_backwards {
            if self.base.x > 700 && self.base.x <= 720 {
                self.vel_x = 0.2;
                self.zombie_phase = ZombiePhase::SnorkelIntoPool;
                self.play_zombie_reanim("anim_jumpinpool", ReanimLoopType::PlayOnceAndHold, 20, 16.0);
            }
        } else if self.zombie_phase == ZombiePhase::SnorkelIntoPool {
            // 依赖底层系统
            // 简化处理
            self.zombie_phase = ZombiePhase::SnorkelWalkingInPool;
            self.in_pool = true;
            self.play_zombie_reanim("anim_swim", ReanimLoopType::LoopFullOffset, 0, 12.0);
        } else if self.zombie_phase == ZombiePhase::SnorkelWalkingInPool {
            if !self.has_head {
                self.take_damage(1800, 9);
            } else if self.base.x <= 25 && !a_backwards {
                self.altitude = -90.0;
                self.pos_x -= 15.0;
                self.zombie_phase = ZombiePhase::SnorkelWalking;
                self.zombie_height = ZombieHeight::OutOfPool;
                // 依赖底层系统
                self.start_walk_anim(0);
            } else if self.base.x > 640 && a_backwards {
                self.altitude = -90.0;
                self.pos_x += 15.0;
                self.zombie_phase = ZombiePhase::SnorkelWalking;
                self.zombie_height = ZombieHeight::OutOfPool;
                // 依赖底层系统
                self.start_walk_anim(0);
            } else if self.is_eating {
                self.zombie_phase = ZombiePhase::SnorkelUpToEat;
                self.play_zombie_reanim("anim_uptoeat", ReanimLoopType::PlayOnceAndHold, 0, 24.0);
            }
        } else if self.zombie_phase == ZombiePhase::SnorkelUpToEat {
            if !self.is_eating {
                self.zombie_phase = ZombiePhase::SnorkelDownFromEat;
                self.play_zombie_reanim("anim_uptoeat", ReanimLoopType::PlayOnceAndHold, 0, -24.0);
            } else {
                // 依赖底层系统
                self.zombie_phase = ZombiePhase::SnorkelEatingInPool;
                self.play_zombie_reanim("anim_eat", ReanimLoopType::Loop, 0, 0.0);
            }
        } else if self.zombie_phase == ZombiePhase::SnorkelEatingInPool {
            if !self.is_eating {
                self.zombie_phase = ZombiePhase::SnorkelDownFromEat;
                self.play_zombie_reanim("anim_uptoeat", ReanimLoopType::PlayOnceAndHold, 0, -24.0);
            }
        } else if self.zombie_phase == ZombiePhase::SnorkelDownFromEat {
            // 依赖底层系统
            self.zombie_phase = ZombiePhase::SnorkelWalkingInPool;
            self.play_zombie_reanim("anim_swim", ReanimLoopType::LoopFullOffset, 0, 0.0);
            self.pick_random_speed();
        }
    }

    /// 更新气球僵尸（对应 C++ UpdateZombieFlyer）
    pub fn update_zombie_flyer(&mut self) {
        // 依赖底层系统
        //if let Some(app) = self.base.get_app() {
        //    if app.game_mode == GameMode::ChallengeHighGravity && self.pos_x < 720.0 {
        //        self.altitude -= 0.1;
        //        if self.altitude < -35.0 {
        //            self.land_flyer(0);
        //        }
        //    }
        //}

        if self.zombie_phase == ZombiePhase::BalloonPopping {
            // 依赖底层系统
            self.zombie_phase = ZombiePhase::BalloonWalking;
            self.start_walk_anim(0);
        }

        // IZombie 模式目标检测
        if let Some(app) = self.base.get_app() {
            if app.is_izombie_level() && self.zombie_phase == ZombiePhase::BalloonFlying {
                // 依赖底层系统
                self.land_flyer(0);
            }
        }
    }

    /// 更新报纸僵尸（对应 C++ UpdateZombieNewspaper）
    pub fn update_zombie_newspaper(&mut self) {
        if self.zombie_phase == ZombiePhase::NewspaperMaddening {
            // 依赖底层系统
            self.zombie_phase = ZombiePhase::NewspaperMad;
            if let Some(board) = self.base.get_board() {
                if board.count_zombies_on_screen() <= 10 && self.has_head {
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::NewspaperRarrgh as i32);
                    }
                }
            }
            self.start_walk_anim(20);
            // 依赖底层系统
        }
    }

    /// 矿工僵尸丢镐（对应 C++ DiggerLoseAxe）
    pub fn digger_lose_axe(&mut self) {
        if self.zombie_phase == ZombiePhase::DiggerTunneling {
            self.zombie_phase = ZombiePhase::DiggerTunnelingPauseWithoutAxe;
            self.phase_counter = 200;
            self.set_anim_rate(0.0);
            self.update_anim_speed();
            // 依赖底层系统
            self.stop_zombie_sound();
        }

        self.has_object = false;
        // 依赖底层系统
    }

    /// 更新矿工僵尸（对应 C++ UpdateZombieDigger）
    pub fn update_zombie_digger(&mut self) {
        if self.zombie_phase == ZombiePhase::DiggerTunneling {
            if self.pos_x < 10.0 {
                self.altitude = -120.0;
                self.zombie_phase = ZombiePhase::DiggerRising;
                self.phase_counter = 130;
                self.play_zombie_reanim("anim_drill", ReanimLoopType::Loop, 0, 20.0);

                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::DirtRise as i32);
                    app.play_foley(crate::todlib::tod_foley::FoleyType::WakeUp as i32);
                }
                // 依赖底层系统
                self.stop_zombie_sound();
                // 依赖底层系统
            }
        } else if self.zombie_phase == ZombiePhase::DiggerRising {
            if self.phase_counter > 40 {
                self.altitude = crate::todlib::tod_common::tod_animate_curve(
                    130, 40, self.phase_counter, -120, 20, TodCurves::EaseOut,
                ) as f32;
            } else {
                self.altitude = crate::todlib::tod_common::tod_animate_curve(
                    30, 0, self.phase_counter, 20, 0, TodCurves::EaseIn,
                ) as f32;
            }

            if self.phase_counter == 30 {
                self.play_zombie_reanim("anim_landing", ReanimLoopType::PlayOnceAndHold, 0, 12.0);
            }

            if self.phase_counter == 0 {
                self.altitude = 0.0;
                self.zombie_phase = ZombiePhase::DiggerStunned;
                self.play_zombie_reanim("anim_dizzy", ReanimLoopType::Loop, 10, 12.0);
            }
        } else if self.zombie_phase == ZombiePhase::DiggerTunnelingPauseWithoutAxe {
            if self.phase_counter == 150 {
                // 依赖底层系统
            }

            if self.phase_counter == 0 {
                self.altitude = -120.0;
                self.zombie_phase = ZombiePhase::DiggerRiseWithoutAxe;
                self.phase_counter = 130;
                self.play_zombie_reanim("anim_landing", ReanimLoopType::PlayOnceAndHold, 0, 0.0);

                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::DirtRise as i32);
                }
                // 依赖底层系统
            }
        } else if self.zombie_phase == ZombiePhase::DiggerRiseWithoutAxe {
            if self.phase_counter > 40 {
                self.altitude = crate::todlib::tod_common::tod_animate_curve(
                    130, 40, self.phase_counter, -120, 20, TodCurves::EaseOut,
                ) as f32;
            } else {
                self.altitude = crate::todlib::tod_common::tod_animate_curve(
                    30, 0, self.phase_counter, 20, 0, TodCurves::EaseIn,
                ) as f32;
            }

            if self.phase_counter == 30 {
                self.play_zombie_reanim("anim_landing", ReanimLoopType::PlayOnceAndHold, 20, 12.0);
            }

            if self.phase_counter == 0 {
                self.altitude = 0.0;
                self.zombie_phase = ZombiePhase::DiggerWalkingWithoutAxe;
                self.start_walk_anim(20);
            }
        } else if self.zombie_phase == ZombiePhase::DiggerStunned {
            // 依赖底层系统
            self.zombie_phase = ZombiePhase::DiggerWalking;
            self.start_walk_anim(20);
        }
    }

    /// 更新小丑僵尸（对应 C++ UpdateZombieJackInTheBox）
    pub fn update_zombie_jack_in_the_box(&mut self) {
        // 对应 C++ UpdateZombieJackInTheBox（Zombie.cpp:2030）
        // 半径常量：JACK_IN_THE_BOX_ZOMBIE_RADIUS = 115 / JACK_IN_THE_BOX_PLANT_RADIUS = 90
        if self.zombie_phase == ZombiePhase::JackInTheBoxRunning {
            if self.phase_counter <= 0 && self.has_head {
                self.phase_counter = 110;
                self.zombie_phase = ZombiePhase::JackInTheBoxPopping;
                self.stop_zombie_sound();
                // [TRANSLATION_NOTE]: C++ 的 mApp->PlaySample(SOUND_BOING) 为样本音，
                // Rust 音效层（PlaySample）未接入，暂略
                self.play_zombie_reanim("anim_pop", ReanimLoopType::PlayOnceAndHold, 20, 28.0);
            }
        } else if self.zombie_phase == ZombiePhase::JackInTheBoxPopping {
            if self.phase_counter == 80 {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::JackSurprise as i32);
                }
            }

            if self.phase_counter <= 0 {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Explosion as i32);
                }

                let a_pos_x = self.base.x + self.base.width / 2;
                let a_pos_y = self.base.y + self.base.height / 2;
                let a_row = self.base.row;
                let a_mind_controlled = self.mind_controlled;
                if let Some(board) = self.base.get_board_mut() {
                    if a_mind_controlled {
                        // 对应 C++: 精神控制小丑只炸僵尸（flags 127）
                        board.kill_all_zombies_in_radius(
                            a_row, a_pos_x, a_pos_y, 115, 1, true, 127);
                    } else {
                        // 对应 C++: 非精神控制炸僵尸（flags 255）+ 炸植物（半径 90）
                        board.kill_all_zombies_in_radius(
                            a_row, a_pos_x, a_pos_y, 115, 1, true, 255);
                        board.kill_all_plants_in_radius(a_pos_x, a_pos_y, 90);
                    }
                    // 对应 C++: mBoard->ShakeBoard(4, -6)
                    board.shake_board(4, -6);
                }
                // [TRANSLATION_NOTE]: C++ 的 PARTICLE_JACKEXPLODE 粒子依赖 AddPvzpParticle，暂略

                self.die_no_loot();

                // 对应 C++: scary potter 关卡的瓦罐连锁爆炸
                if let Some(app) = self.base.get_app() {
                    if app.is_scary_potter_level() {
                        if let Some(board) = self.base.get_board_mut() {
                            if let Some(challenge) = board.challenge.as_mut() {
                                challenge.scary_potter_jack_explode(a_pos_x, a_pos_y);
                            }
                        }
                    }
                }
            }
        }
    }

    /// 更新伽刚特尔（对应 C++ UpdateZombieGargantuar）
    pub fn update_zombie_gargantuar(&mut self) {
        if self.zombie_phase == ZombiePhase::GargantuarSmashing {
            // 对应 C++: aBodyReanim->ShouldTriggerTimedEvent(0.64f)
            let a_triggered = self.base.get_app().map_or(false, |app| {
                app.reanimation_get(self.body_reanim_id)
                    .map_or(false, |r| r.should_trigger_timed_event(0.64))
            });
            if a_triggered {
                // 对应 C++: FindPlantTarget(ATTACKTYPE_CHEW)（借用规避：先提取目标信息）
                let a_target_info: Option<(usize, i32, i32, bool)> =
                    self.find_plant_target_index(ZombieAttackType::Chew).and_then(|idx| {
                        if let Some(board) = self.base.get_board() {
                            let p = &board.plants[idx];
                            Some((idx, p.plant_col, p.base.row, p.seed_type == SeedType::Spikerock))
                        } else {
                            None
                        }
                    });
                if let Some((idx, a_col, a_row, a_is_spikerock)) = a_target_info {
                    if a_is_spikerock {
                        // 对应 C++: TakeDamage(20, 32U) + SpikeRockTakeDamage + 植物死亡则碾压
                        self.take_damage(20, 32);
                        let a_plant_dead = if let Some(board) = self.base.get_board_mut() {
                            if let Some(p) = board.plants.get_mut(idx) {
                                p.spike_rock_take_damage();
                                p.plant_health <= 0
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        if a_plant_dead {
                            self.squish_all_in_square(a_col, a_row, ZombieAttackType::Chew);
                        }
                    } else {
                        self.squish_all_in_square(a_col, a_row, ZombieAttackType::Chew);
                    }
                }

                // 对应 C++: PlayFoley(FOLEY_THUMP) + ShakeBoard(0, 3)
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Thump as i32);
                }
                if let Some(board) = self.base.get_board_mut() {
                    board.shake_board(0, 3);
                }

                // 对应 C++: scary potter 关卡的瓦罐被砸开
                if self.base.get_app().map_or(false, |app| app.is_scary_potter_level()) {
                    let a_row = self.base.row;
                    let a_pos_x = self.base.x;
                    if let Some(board) = self.base.get_board_mut() {
                        let a_grid_x = board.pixel_to_grid_x_keep_on_board(a_pos_x, 0);
                        if let Some(challenge) = board.challenge.as_mut() {
                            // 对应 C++: mBoard->GetScaryPotAt(aGridX, mRow)；Rust 以 grid_items 查找
                            let a_scary_idx = board
                                .grid_items
                                .iter()
                                .position(|item| {
                                    !item.dead
                                        && item.grid_item_type == crate::lawn::grid_item::GridItemType::ScaryPot
                                        && item.grid_x == a_grid_x
                                        && item.grid_y == a_row
                                });
                            if let Some(sidx) = a_scary_idx {
                                challenge.scary_potter_open_pot(&mut board.grid_items[sidx]);
                            }
                        }
                    }
                }
                // [TRANSLATION_NOTE]: C++ IZombie 分支（IZombieGetBrainTarget/SquishBrain）依赖 IZombie 系统，暂未接入
            }

            // 对应 C++: aBodyReanim->mLoopCount > 0 → 回 Normal
            let a_loop_done = self.base.get_app().map_or(false, |app| {
                app.reanimation_get(self.body_reanim_id).map_or(false, |r| r.m_loop_count > 0)
            });
            if a_loop_done {
                self.zombie_phase = ZombiePhase::Normal;
                self.start_walk_anim(20);
            }
            return;
        }

        let a_throwing_distance = self.pos_x - 360.0;

        if self.zombie_phase == ZombiePhase::GargantuarThrowing {
            // 触发扔小鬼事件
            if self.anim_counter % 40 == 0 {
                self.has_object = false;
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Swing as i32);
                }

                let mut a_throwing_distance = a_throwing_distance;
                let mut a_min_throw_distance = 40.0;
                let mut stage_has_roof = false;
                if let Some(board) = self.base.get_board() {
                    stage_has_roof = board.stage_has_roof();
                }
                if stage_has_roof {
                    a_throwing_distance -= 180.0;
                    a_min_throw_distance = -140.0;
                }
                if a_throwing_distance < a_min_throw_distance {
                    a_throwing_distance = a_min_throw_distance;
                } else if a_throwing_distance > 140.0 {
                    a_throwing_distance -= RandFloat(100.0);
                }

                                // C++ 2169-2195: 生成并配置小鬼僵尸
                let from_wave = self.from_wave;
                let row = self.base.row;
                let render_order = self.base.render_order;
                let a_pos_x = self.pos_x;
                let a_chilled = self.chilled_counter;
                let pos_y = self.get_pos_y_based_on_row(row);
                let vel_z = 0.5 * (a_throwing_distance / 3.0) * crate::lawn::zombie::THOWN_ZOMBIE_GRAVITY;
                if let Some(board) = self.base.get_board_mut() {
                    // C++: aZombieImp = mBoard->AddZombie(ZOMBIE_IMP, mFromWave); if (nullptr) return;
                    let a_imp_idx = board.add_zombie_in_row(ZombieType::Imp, row, from_wave);
                    if let Some(a_imp) = board.zombies.get_mut(a_imp_idx) {
                        // C++: mPosX = mPosX - 133.0f; mPosY = GetPosYBasedOnRow(mRow); SetRow(mRow);
                        a_imp.pos_x = a_pos_x - 133.0;
                        a_imp.pos_y = pos_y;
                        a_imp.base.row = row;
                        // C++: mVariant = false; mAltitude = 88.0f; mRenderOrder = mRenderOrder + 1;
                        a_imp.variant = false;
                        a_imp.altitude = 88.0;
                        a_imp.base.render_order = render_order + 1;
                        // C++: mZombiePhase = PHASE_IMP_GETTING_THROWN; mVelX = 3.0f（DO_FIX_BUGS 分支未启用）
                        a_imp.zombie_phase = ZombiePhase::ImpGettingThrown;
                        a_imp.vel_x = 3.0;
                        // C++: mChilledCounter = mChilledCounter;
                        a_imp.chilled_counter = a_chilled;
                        // C++: mVelZ = 0.5f * (aThrowingDistance / aZombieImp->mVelX) * THOWN_ZOMBIE_GRAVITY;
                        a_imp.vel_z = vel_z;
                        // C++: PlayZombieReanim("anim_thrown", REANIM_PLAY_ONCE_AND_HOLD, 0, 18.0f); UpdateReanim();
                        a_imp.play_zombie_reanim("anim_thrown", ReanimLoopType::PlayOnceAndHold, 0, 18.0);
                        a_imp.update_reanim();
                    }
                }
                // C++: mApp->PlayFoley(FOLEY_IMP)
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Imp as i32);
                }
            }

            // 动画循环结束后回 Normal
            if self.anim_counter % 120 == 0 {
                self.zombie_phase = ZombiePhase::Normal;
                self.start_walk_anim(20);
            }
            return;
        }

        if self.is_immobilized() || !self.has_head {
            return;
        }

        // 血量少于一半且有物体 → 扔小鬼
        if self.has_object && self.body_health < self.body_max_health / 2 && a_throwing_distance > 40.0 {
            self.zombie_phase = ZombiePhase::GargantuarThrowing;
            self.play_zombie_reanim("anim_throw", ReanimLoopType::PlayOnceAndHold, 20, 24.0);
            return;
        }

        // 有植物目标 → 砸击
        // 依赖底层系统
        let plant_target = self.plant_col_below();
        if plant_target != -1 {
            self.zombie_phase = ZombiePhase::GargantuarSmashing;
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::LowGroan as i32);
            }
            self.play_zombie_reanim("anim_smash", ReanimLoopType::PlayOnceAndHold, 20, 16.0);
        }
    }

    /// 获取伽刚特尔下方植物的列（对应 C++ FindPlantTarget 简化）
    pub fn plant_col_below(&self) -> i32 {
        if let Some(idx) = self.find_plant_target_index(ZombieAttackType::Chew) {
            if let Some(board) = self.base.get_board() {
                if idx < board.plants.len() {
                    return board.plants[idx].plant_col;
                }
            }
        }
        -1
    }

    /// 碾压某格子内的植物（对应 C++ Zombie::SquishAllInSquare，Zombie.cpp:6476-6498）
    pub fn squish_all_in_square(&mut self, x: i32, y: i32, attack_type: ZombieAttackType) {
        if let Some(board) = self.base.get_board_mut() {
            let idxs: Vec<usize> = board.plants.iter().enumerate()
                .filter(|(_, p)| !p.dead && p.base.row == y && p.plant_col == x
                    && !(attack_type == ZombieAttackType::DriveOver && p.is_spiky())
                    && p.seed_type != SeedType::Spikerock)
                .map(|(i, _)| i)
                .collect();
            for idx in idxs {
                // 对应 C++: mBoard->mPlantsEaten++; aPlant->Squish();
                board.m_plants_eaten += 1;
                board.plants[idx].squish();
            }
        }
    }

    /// 碾压检查（对应 C++ CheckSquish）
    pub fn check_squish(&mut self, attack_type: ZombieAttackType) {
        let attack_rect = self.get_zombie_attack_rect();
        let mut squish_x = -1;
        let mut squish_y = -1;
        if let Some(board) = self.base.get_board() {
            for plant in &board.plants {
                if plant.dead { continue; }
                if plant.base.row == self.base.row {
                    let plant_rect = plant.get_plant_rect();
                    if crate::lawn::board::get_rect_overlap(&attack_rect, &plant_rect) >= 20
                        && self.can_target_plant(plant, attack_type)
                        && !plant.is_spiky()
                    {
                        squish_x = plant.plant_col;
                        squish_y = plant.base.row;
                        break;
                    }
                }
            }
        }
        if squish_x != -1 {
            self.squish_all_in_square(squish_x, squish_y, attack_type);
        }

        // 对应 C++: if (mApp->IsIZombieLevel()) { GridItem* aBrain = mBoard->mChallenge->IZombieGetBrainTarget(this);
        //          if (aBrain) mBoard->mChallenge->IZombieSquishBrain(aBrain); }
        // [TRANSLATION_NOTE]: Challenge 的 IZombieGetBrainTarget / IZombieSquishBrain 尚未接入，暂缺
    }

    /// 更新小鬼僵尸（对应 C++ UpdateZombieImp）
    pub fn update_zombie_imp(&mut self) {
        if self.zombie_phase == ZombiePhase::ImpGettingThrown {
            self.vel_z -= crate::lawn::zombie::THOWN_ZOMBIE_GRAVITY;
            self.altitude += self.vel_z;
            self.pos_x -= self.vel_x;

            let a_diff_y = self.get_pos_y_based_on_row(self.base.row) - self.pos_y;
            self.pos_y += a_diff_y;
            self.altitude += a_diff_y;
            if self.altitude <= 0.0 {
                self.altitude = 0.0;
                self.zombie_phase = ZombiePhase::ImpLanding;
                self.play_zombie_reanim("anim_land", ReanimLoopType::PlayOnceAndHold, 0, 24.0);
            }
        } else if self.zombie_phase == ZombiePhase::ImpLanding {
            // 依赖底层系统
            // 简化：直接回 Normal
            self.zombie_phase = ZombiePhase::Normal;
            self.start_walk_anim(0);
        }
    }

    /// 更新雪橇僵尸（对应 C++ BobsledCrash）
    pub fn bobsled_crash(&mut self) {
        self.altitude = 0.0;
        self.zombie_rect = Rect::new(36, 0, 42, 115);
        self.zombie_phase = ZombiePhase::BobsledCrashing;
        self.phase_counter = BOBSLED_CRASH_TIME;
        self.start_walk_anim(0);

        // 依赖底层系统
    }

    /// 更新雪橇僵尸（对应 C++ UpdateZombieBobsled）
    pub fn update_zombie_bobsled(&mut self) {
        if self.zombie_phase == ZombiePhase::BobsledCrashing {
            if self.phase_counter == 0 {
                self.zombie_phase = ZombiePhase::Normal;
                if self.get_bobsled_position() == 0 {
                    // 依赖底层系统
                    self.pick_random_speed();
                }
            }
            return;
        }

        if self.zombie_phase == ZombiePhase::BobsledSliding {
            if self.phase_counter == 0 {
                self.zombie_phase = ZombiePhase::BobsledBoarding;
                self.play_zombie_reanim("anim_jump", ReanimLoopType::PlayOnceAndHold, 0, 20.0);
            }
        } else {
            if self.zombie_phase != ZombiePhase::BobsledBoarding {
                return;
            }

            // 依赖底层系统
            let a_position = self.get_bobsled_position();
            if a_position == 1 || a_position == 3 {
                self.altitude = 8.0;
            } else {
                self.altitude = -9.0;
            }
        }

        // 冰面维持
        let my_pos_x = self.pos_x;
        let my_row = self.base.row;
        let bobsled_pos = self.get_bobsled_position();
        let (ice_timer_set, need_damage) = if let Some(board) = self.base.get_board_mut() {
            let row = my_row as usize;
            board.m_ice_timer[row] = board.m_ice_timer[row].max(500);
            let edge = board.m_ice_min_x[row];
            let behind_edge = my_pos_x + 10.0 < edge as f32 && bobsled_pos == 0;
            (true, behind_edge)
        } else {
            (false, false)
        };
        if ice_timer_set && need_damage {
            self.take_damage(6, 8);
        }
    }

    /// 更新冰车僵尸（对应 C++ UpdateZamboni）
    pub fn update_zamboni(&mut self) {
        if self.pos_x > 400.0 && !self.flat_tires {
            // PvzpAnimateCurveFloat(700, 300, mPosX, 0.25f, 0.05f, CURVE_LINEAR)
            self.vel_x = crate::todlib::tod_common::tod_animate_curve_float(
                700, 300, self.pos_x as i32, 0.25, 0.05, TodCurves::Linear,
            );
        } else if self.flat_tires && self.vel_x > 0.0005 {
            self.vel_x -= 0.0005;
        }

        let my_pos_x = self.pos_x as i32;
        let my_row = self.base.row;
        let mut an_ice_x = my_pos_x + 118;
        let has_roof = if let Some(board) = self.base.get_board() {
            board.stage_has_roof()
        } else {
            false
        };
        if has_roof {
            an_ice_x = an_ice_x.max(500);
        } else {
            an_ice_x = an_ice_x.max(25);
        }

        if an_ice_x < 800 {
            let is_bobsled_bonanza = if let Some(app) = self.base.get_app() {
                app.game_mode == GameMode::ChallengeBobsledBonanza
            } else {
                false
            };
            if let Some(board) = self.base.get_board_mut() {
                let row = my_row as usize;
                board.m_ice_min_x[row] = board.m_ice_min_x[row].min(an_ice_x);
                if is_bobsled_bonanza {
                    board.m_ice_timer[row] = i32::MAX;
                } else {
                    board.m_ice_timer[row] = 3000;
                }
            }
        }
    }

    /// 更新梯子僵尸（对应 C++ UpdateLadder）
    pub fn update_ladder(&mut self) {
        if self.mind_controlled || !self.has_head || self.is_dead_or_dying() {
            return;
        }

        if self.zombie_phase == ZombiePhase::LadderCarrying && self.zombie_height == ZombieHeight::Normal {
            // 依赖底层系统
            let has_plant_ahead = self.base.x > 50 && self.base.x < 700;
            if has_plant_ahead {
                self.stop_eating();
                self.zombie_phase = ZombiePhase::LadderPlacing;
                self.play_zombie_reanim("anim_placeladder", ReanimLoopType::PlayOnceAndHold, 10, 24.0);
            }
        } else if self.zombie_phase == ZombiePhase::LadderPlacing {
            // 依赖底层系统
            // 放梯子
            let plant_col = self.target_col;
            let plant_row = self.base.row;
            if let Some(board) = self.base.get_board_mut() {
                board.add_ladder(plant_col, plant_row);
            }
            self.zombie_height = ZombieHeight::UpLadder;
            self.use_ladder_col = self.target_col;
            // 依赖底层系统
        }
    }

    /// 召唤伴舞（对应 C++ SummonBackupDancer）
    pub fn summon_backup_dancer(&mut self, row: i32, pos_x: i32) -> ZombieID {
        // 依赖底层系统
        // 但 AddZombie 返回 Option<&mut Zombie> 无法在此持久返回 ID
        if let Some(board) = self.base.get_board() {
            if !board.row_can_have_zombie_type(row, ZombieType::BackupDancer) {
                return ZOMBIEID_NULL;
            }
        }
        let from_wave = self.from_wave;
        if let Some(board) = self.base.get_board_mut() {
            board.add_zombie(ZombieType::BackupDancer, from_wave);
        }
        ZOMBIEID_NULL
    }

    /// 召唤全部伴舞（对应 C++ SummonBackupDancers）
    pub fn summon_backup_dancers(&mut self) {
        if !self.has_head {
            return;
        }

        for i in 0..NUM_BACKUP_DANCERS {
            // 依赖底层系统
            let (a_row, a_pos_x) = match i {
                0 => (self.base.row - 1, self.base.x),
                1 => (self.base.row + 1, self.base.x),
                2 => (self.base.row, self.base.x - 100),
                3 => (self.base.row, self.base.x + 100),
                _ => (0, 0),
            };
            let _id = self.summon_backup_dancer(a_row, a_pos_x);
            // self.follower_zombie_ids[i] = id;
        }
    }

    /// 是否需要更多伴舞（对应 C++ NeedsMoreBackupDancers）
    pub fn needs_more_backup_dancers(&self) -> bool {
        if let Some(board) = self.base.get_board() {
            for i in 0..NUM_BACKUP_DANCERS {
                // 依赖底层系统
                if i == 0 && !board.row_can_have_zombie_type(self.base.row - 1, ZombieType::BackupDancer) {
                    continue;
                }
                if i == 1 && !board.row_can_have_zombie_type(self.base.row + 1, ZombieType::BackupDancer) {
                    continue;
                }
                return true;
            }
        }
        false
    }

    /// 获取舞蹈帧（对应 C++ GetDancerFrame）
    pub fn get_dancer_frame(&self) -> i32 {
        if self.from_wave == Zombie::ZOMBIE_WAVE_UI || self.is_immobilized() {
            return 0;
        }

        let mut a_frame_length = 20;
        let mut a_frames_count = 23;
        if self.zombie_phase == ZombiePhase::DancerDancingIn {
            a_frames_count = 11;
            a_frame_length = 10;
        }

        let counter = if let Some(board) = self.base.get_board() {
            board.m_main_counter as i32
        } else {
            0
        };
        (counter % (a_frame_length * a_frames_count)) / a_frame_length
    }

    /// 获取舞蹈相位（对应 C++ GetDancerPhase）
    pub fn get_dancer_phase(&self) -> ZombiePhase {
        let a_frame = self.get_dancer_frame();
        if a_frame <= 11 {
            ZombiePhase::DancerDancingLeft
        } else if a_frame <= 12 {
            ZombiePhase::DancerWalkToRaise
        } else if a_frame <= 18 {
            ZombiePhase::DancerRaiseLeft1
        } else {
            ZombiePhase::DancerRaiseLeft2
        }
    }

    /// 更新舞王僵尸（对应 C++ UpdateZombieDancer）
    pub fn update_zombie_dancer(&mut self) {
        if self.is_eating {
            return;
        }

        // 召唤倒计时
        if self.summon_counter > 0 {
            self.summon_counter -= 1;
            if self.summon_counter == 0 {
                if self.get_dancer_frame() == 12 && self.has_head && self.pos_x < 700.0 {
                    self.zombie_phase = ZombiePhase::DancerSnappingFingersWithLight;
                    self.play_zombie_reanim("anim_point", ReanimLoopType::PlayOnceAndHold, 20, 24.0);
                } else {
                    self.summon_counter = 1;
                }
            }
        }

        if self.zombie_phase == ZombiePhase::DancerDancingIn {
            if self.has_head && self.phase_counter == 0 {
                self.zombie_phase = ZombiePhase::DancerSnappingFingers;
                self.play_zombie_reanim("anim_point", ReanimLoopType::PlayOnceAndHold, 20, 24.0);
                self.pick_random_speed();
            }
        } else if self.zombie_phase == ZombiePhase::DancerSnappingFingers
            || self.zombie_phase == ZombiePhase::DancerSnappingFingersWithLight
        {
            // 依赖底层系统
            if self.zombie_phase == ZombiePhase::DancerSnappingFingers {
                if let Some(board) = self.base.get_board() {
                    if board.count_zombies_on_screen() <= 15 {
                        if let Some(app) = self.base.get_app() {
                            app.play_foley(crate::todlib::tod_foley::FoleyType::Dancer as i32);
                        }
                    }
                }
            }
            self.summon_backup_dancers();
            self.zombie_phase = ZombiePhase::DancerSnappingFingersHold;
            self.phase_counter = 200;
        } else {
            if self.zombie_phase == ZombiePhase::DancerSnappingFingersHold {
                if self.phase_counter != 0 {
                    return;
                }
                self.zombie_phase = ZombiePhase::DancerDancingLeft;
                self.play_zombie_reanim("anim_walk", ReanimLoopType::Loop, 20, 0.0);
            }

            // 获取舞步相位并切换
            let a_dancer_phase = self.get_dancer_phase();
            if a_dancer_phase != self.zombie_phase {
                match a_dancer_phase {
                    ZombiePhase::DancerDancingLeft => {
                        self.zombie_phase = a_dancer_phase;
                        self.play_zombie_reanim("anim_walk", ReanimLoopType::Loop, 10, 0.0);
                    }
                    ZombiePhase::DancerWalkToRaise => {
                        self.zombie_phase = a_dancer_phase;
                        self.play_zombie_reanim("anim_armraise", ReanimLoopType::Loop, 10, 18.0);
                    }
                    ZombiePhase::DancerRaiseLeft1 | ZombiePhase::DancerRaiseLeft2 => {
                        self.zombie_phase = a_dancer_phase;
                        self.play_zombie_reanim("anim_armraise", ReanimLoopType::Loop, 10, 18.0);
                    }
                    _ => {}
                }
            }

            if self.has_head && self.summon_counter == 0 && self.needs_more_backup_dancers() {
                self.summon_counter = 100;
            }
        }
    }

    /// 更新伴舞僵尸（对应 C++ UpdateZombieBackupDancer）
    pub fn update_zombie_backup_dancer(&mut self) {
        if self.is_eating {
            return;
        }

        if self.zombie_phase == ZombiePhase::DancerRising {
            // PvzpAnimateCurve(150, 0, mPhaseCounter, -200, 0, CURVE_LINEAR)
            self.altitude = crate::todlib::tod_common::tod_animate_curve(
                150, 0, self.phase_counter, ZOMBIE_BACKUP_DANCER_RISE_HEIGHT, 0, TodCurves::Linear,
            ) as f32;

            if self.phase_counter != 0 {
                return;
            }
        }

        // 获取舞步相位并切换
        let a_dancer_phase = self.get_dancer_phase();
        if a_dancer_phase != self.zombie_phase {
            match a_dancer_phase {
                ZombiePhase::DancerDancingLeft => {
                    self.zombie_phase = a_dancer_phase;
                    self.play_zombie_reanim("anim_walk", ReanimLoopType::Loop, 10, 0.0);
                }
                ZombiePhase::DancerWalkToRaise => {
                    self.zombie_phase = a_dancer_phase;
                    self.play_zombie_reanim("anim_armraise", ReanimLoopType::Loop, 10, 18.0);
                }
                ZombiePhase::DancerRaiseLeft1 | ZombiePhase::DancerRaiseLeft2 => {
                    self.zombie_phase = a_dancer_phase;
                    self.play_zombie_reanim("anim_armraise", ReanimLoopType::Loop, 10, 18.0);
                }
                _ => {}
            }
        }
    }

    /// 更新 Boss（对应 C++ Zombie::UpdateBoss，Zombie.cpp:10219-10472）
    pub fn update_boss(&mut self) {
        // C++ UpdateBoss：mPhaseCounter 由主 Update 递减，这里不重复
        let a_game_scene = self.base.get_app().map_or(crate::lawn::lawn_app::GameScenes::Playing, |app| app.game_scene);
        if a_game_scene == crate::lawn::lawn_app::GameScenes::LevelIntro {
            // 开场动画：两个时间点触发跺脚音效 + 震屏
            let a_triggered = self.base.get_app()
                .and_then(|app| app.reanimation_get(self.body_reanim_id))
                .map_or(false, |r| r.should_trigger_timed_event(0.24) || r.should_trigger_timed_event(0.79));
            if a_triggered {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Thump as i32);
                }
                if let Some(board) = self.base.get_board_mut() {
                    board.shake_board(1, 4);
                }
            }
            return;
        }

        self.update_boss_fireball();
        if self.ice_trap_counter == 0 {
            if self.summon_counter > 0 {
                self.summon_counter -= 1;
            }
            if self.boss_bungee_counter > 0 {
                self.boss_bungee_counter -= 1;
            }
            if self.boss_stomp_counter > 0 {
                self.boss_stomp_counter -= 1;
            }
            if self.boss_head_counter > 0 {
                self.boss_head_counter -= 1;
            }

            // 头部动画速率：冻结时 6fps，否则若为 0 恢复为 12fps
            if self.chilled_counter > 0 {
                if let Some(app) = self.base.get_app_mut() {
                    if let Some(head) = app.reanimation_get_mut(self.special_head_reanim_id) {
                        head.m_anim_rate = 6.0;
                    }
                }
            } else {
                let head_rate_zero = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.special_head_reanim_id))
                    .map_or(false, |h| h.m_anim_rate == 0.0);
                if head_rate_zero {
                    if let Some(app) = self.base.get_app_mut() {
                        if let Some(head) = app.reanimation_get_mut(self.special_head_reanim_id) {
                            head.m_anim_rate = 12.0;
                        }
                    }
                }
            }
        } else {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(head) = app.reanimation_get_mut(self.special_head_reanim_id) {
                    head.m_anim_rate = 0.0;
                }
            }
        }

        match self.zombie_phase {
            ZombiePhase::BossEnter => {
                self.boss_play_idle();
            }
            ZombiePhase::BossIdle => {
                if self.body_health == 1 {
                    self.play_death_anim(0);
                    return;
                }
                if self.phase_counter > 0 {
                    return;
                }

                let a_damage_index = self.get_body_damage_index();
                if a_damage_index != self.boss_mode {
                    self.boss_mode = a_damage_index;
                    if self.boss_mode == 1 {
                        // 进入伤害阶段 1：立即释放一次蹦极
                        self.boss_bungee_attack();
                    } else {
                        // 进入伤害阶段 2：立即丢一次 RV
                        self.boss_rv_attack();
                    }
                } else if self.boss_stomp_counter == 0 {
                    self.boss_stomp_attack();
                } else if self.boss_bungee_counter == 0 {
                    // 冒险模式 1/4、其余 1/2 概率丢 RV，否则放蹦极
                    let a_rand = if self.base.get_app().map_or(false, |a| a.is_adventure_mode()) { 4 } else { 2 };
                    if RandRange(a_rand) == 0 {
                        self.boss_bungee_counter = 4000 + RandRange(1001);  // RandRangeInt(4000, 5000)
                        self.boss_rv_attack();
                    } else {
                        self.boss_bungee_attack();
                    }
                } else if self.boss_head_counter == 0 {
                    self.boss_head_attack();
                } else if self.summon_counter == 0 {
                    self.boss_spawn_attack();
                } else {
                    self.phase_counter = 100 + RandRange(101);  // RandRangeInt(100, 200)
                }
            }
            ZombiePhase::BossSpawning => {
                let a_trigger = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.should_trigger_timed_event(0.6));
                if a_trigger {
                    self.boss_spawn_contact();
                }
                let a_looped = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.m_loop_count > 0);
                if a_looped {
                    self.boss_play_idle();
                }
            }
            ZombiePhase::BossStomping => {
                let a_trigger_time = if self.target_row >= 2 { 0.55 } else { 0.5 };
                let a_trigger = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.should_trigger_timed_event(a_trigger_time));
                if a_trigger {
                    self.boss_stomp_contact();
                }
                let a_looped = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.m_loop_count > 0);
                if a_looped {
                    self.boss_play_idle();
                }
            }
            ZombiePhase::BossBungeesEnter => {
                let a_trigger = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.should_trigger_timed_event(0.4));
                if a_trigger {
                    self.boss_bungee_spawn();
                }
            }
            ZombiePhase::BossBungeesDrop => {
                if self.boss_are_bungees_done() {
                    self.boss_bungee_leave();
                }
            }
            ZombiePhase::BossBungeesLeave => {
                let a_looped = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.m_loop_count > 0);
                if a_looped {
                    self.boss_play_idle();
                }
            }
            ZombiePhase::BossDropRv => {
                let a_trigger = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.should_trigger_timed_event(0.65));
                if a_trigger {
                    self.boss_rv_landing();
                }
                let a_looped = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.m_loop_count > 0);
                if a_looped {
                    self.boss_play_idle();
                }
            }
            ZombiePhase::BossHeadEnter => {
                let a_damage2_trigger = self.get_body_damage_index() == 2
                    && self.base.get_app()
                        .and_then(|app| app.reanimation_get(self.body_reanim_id))
                        .map_or(false, |r| r.should_trigger_timed_event(0.37));
                if a_damage2_trigger {
                    self.apply_boss_smoke_particles(true);
                }
                let a_hydraulic_trigger = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.should_trigger_timed_event(0.55));
                if a_hydraulic_trigger {
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::Hydraulic as i32);
                    }
                }
                let a_looped = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.m_loop_count > 0);
                if a_looped {
                    self.zombie_phase = ZombiePhase::BossHeadIdleBeforeSpit;
                    self.play_zombie_reanim("anim_head_idle", ReanimLoopType::Loop, 0, 12.0);
                    self.phase_counter = 500;
                }
            }
            ZombiePhase::BossHeadIdleBeforeSpit => {
                if self.body_health == 1 {
                    self.boss_start_death();
                } else if self.phase_counter == 0 {
                    self.boss_head_spit();
                }
            }
            ZombiePhase::BossHeadSpit => {
                let a_effect_trigger = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.should_trigger_timed_event(0.37));
                if a_effect_trigger {
                    self.boss_head_spit_effect();
                }
                let a_contact_trigger = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.should_trigger_timed_event(0.42));
                if a_contact_trigger {
                    self.boss_head_spit_contact();
                }
                let a_looped = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.m_loop_count > 0);
                if a_looped {
                    if let Some(app) = self.base.get_app_mut() {
                        if let Some(head) = app.reanimation_get_mut(self.special_head_reanim_id) {
                            head.play_reanim("anim_idle", ReanimLoopType::Loop, 20, 18.0);
                        }
                    }
                    self.zombie_phase = ZombiePhase::BossHeadIdleAfterSpit;
                    self.play_zombie_reanim("anim_head_idle", ReanimLoopType::Loop, 0, 12.0);
                    self.phase_counter = 300;
                }
            }
            ZombiePhase::BossHeadIdleAfterSpit => {
                if self.body_health == 1 {
                    self.boss_start_death();
                } else if self.phase_counter == 0 {
                    self.zombie_phase = ZombiePhase::BossHeadLeave;
                    self.play_zombie_reanim("anim_head_leave", ReanimLoopType::PlayOnceAndHold, 0, 12.0);
                }
            }
            ZombiePhase::BossHeadLeave => {
                let a_clear_chill = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.should_trigger_timed_event(0.23));
                if a_clear_chill {
                    self.chilled_counter = 0;
                    self.update_anim_speed();
                }
                let a_thump = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.should_trigger_timed_event(0.48) || r.should_trigger_timed_event(0.8));
                if a_thump {
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::Thump as i32);
                    }
                }
                let a_looped = self.base.get_app()
                    .and_then(|app| app.reanimation_get(self.body_reanim_id))
                    .map_or(false, |r| r.m_loop_count > 0);
                if a_looped {
                    self.apply_boss_smoke_particles(false);
                    self.boss_play_idle();
                }
            }
            _ => {
                // C++: else 分支为 PVZP_ASSERT(false)——所有 Boss 阶段均已覆盖，Rust 不应到达
            }
        }
    }
    fn find_plant_target_index(&self, attack_type: ZombieAttackType) -> Option<usize> {
        let attack_rect = self.get_zombie_attack_rect();
        let board = self.base.get_board()?;
        for (i, plant) in board.plants.iter().enumerate() {
            if plant.dead {
                continue;
            }
            if plant.base.row == self.base.row {
                let plant_rect = plant.get_plant_rect();
                if crate::lawn::board::get_rect_overlap(&attack_rect, &plant_rect) >= 20
                    && self.can_target_plant(plant, attack_type)
                {
                    return Some(i);
                }
            }
        }
        None
    }

    fn can_target_plant(&self, plant: &Plant, attack_type: ZombieAttackType) -> bool {
        let board = match self.base.get_board() {
            Some(b) => b,
            None => return false,
        };
        if self.base.get_app().map_or(false, |app| app.is_wallnut_bowling_level())
            && attack_type != ZombieAttackType::Vault
        {
            return false;
        }

        if plant.not_on_ground() || plant.seed_type == SeedType::Tanglekelp {
            return false;
        }

        if !self.in_pool && board.is_pool_square(plant.plant_col, plant.base.row) {
            return false;
        }

        if self.zombie_phase == ZombiePhase::DiggerTunneling {
            return plant.seed_type == SeedType::PotatoMine && plant.state == PlantState::NotReady;
        }

        if plant.is_spiky() {
            return matches!(self.zombie_type,
                ZombieType::Gargantuar | ZombieType::RedeEyeGargantuar | ZombieType::Zamboni)
                || board.is_pool_square(plant.plant_col, plant.base.row)
                || board.get_flower_pot_at(plant.plant_col, plant.base.row).is_some();
        }

        if attack_type == ZombieAttackType::DriveOver {
            if matches!(plant.seed_type,
                SeedType::Cherrybomb | SeedType::Jalapeno | SeedType::Blover | SeedType::Squash)
            {
                return false;
            }
            if matches!(plant.seed_type, SeedType::Doomshroom | SeedType::Iceshroom) {
                return plant.is_asleep;
            }
        }

        if self.zombie_phase == ZombiePhase::LadderCarrying
            || self.zombie_phase == ZombiePhase::LadderPlacing
        {
            let mut a_place_ladder = matches!(plant.seed_type,
                SeedType::Wallnut | SeedType::Tallnut | SeedType::Pumpkinshell);
            if board.get_ladder_at(plant.plant_col, plant.base.row).is_some() {
                a_place_ladder = false;
            }
            if (attack_type == ZombieAttackType::Chew && a_place_ladder)
                || (attack_type == ZombieAttackType::Ladder && !a_place_ladder)
            {
                return false;
            }
        }

        if attack_type == ZombieAttackType::Chew {
            if let Some(top_plant) = board.get_top_plant_at_any(plant.plant_col, plant.base.row) {
                if !std::ptr::eq(top_plant, plant) && self.can_target_plant(top_plant, attack_type) {
                    return false;
                }
            }
        }

        if attack_type == ZombieAttackType::Vault {
            if let Some(top_plant) = board.get_top_plant_at_any(plant.plant_col, plant.base.row) {
                if !std::ptr::eq(top_plant, plant) && self.can_target_plant(top_plant, attack_type) {
                    return false;
                }
            }
        }

        true
    }

    pub fn remove_cold_effects(&mut self) {
        if self.ice_trap_counter > 0 {
            self.remove_ice_trap();
        }
        if self.chilled_counter > 0 {
            self.chilled_counter = 0;
            self.update_anim_speed();
        }
    }

    /// 烧毁整行僵尸（对应 C++ BurnRow）
    pub fn burn_row(&mut self, the_row: i32) {
        let board = match self.base.board { Some(b) => b, None => return };
        unsafe {
            let b = &mut *board;
            for zombie in b.zombies.iter_mut() {
                if zombie.dead { continue; }
                if (zombie.zombie_type == ZombieType::Boss || zombie.base.row == the_row) && zombie.effected_by_damage(127) {
                    zombie.remove_cold_effects();
                    zombie.apply_burn();
                }
            }
            for item in b.grid_items.iter_mut() {
                if item.dead { continue; }
                if item.grid_y == the_row && item.grid_item_type == crate::lawn::grid_item::GridItemType::Ladder {
                    item.grid_item_die();
                }
            }
            if let Some(boss) = b.get_boss_zombie_mut() {
                if boss.fireball_row == the_row {
                    boss.boss_destroy_iceball_in_row();
                }
            }
        }
    }

    pub fn boss_play_idle(&mut self) {
        self.zombie_phase = ZombiePhase::BossIdle;
        self.phase_counter = 100 + RandRange(101);  // RandRangeInt(100, 200)
        self.play_zombie_reanim("anim_idle", ReanimLoopType::Loop, 0, 6.0);
    }

    pub fn boss_rv_attack(&mut self) {
        self.remove_cold_effects();
        self.zombie_phase = ZombiePhase::BossDropRv;
        let a_has_6_rows = self.base.get_board().map_or(false, |b| b.stage_has_6_rows());
        self.target_row = if a_has_6_rows { RandRange(5) } else { RandRange(4) };  // RandRangeInt(0, 4/3)（DO_FIX_BUGS 泳池 Boss 兼容）
        self.target_col = RandRange(3);  // RandRangeInt(0, 2)

        self.play_zombie_reanim("anim_RV_1", ReanimLoopType::PlayOnceAndHold, 20, 16.0);
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::HydraulicShort as i32);
        }
    }

    pub fn boss_rv_landing(&mut self) {
        let a_target_row = self.target_row;
        let a_target_col = self.target_col;
        if let Some(board) = self.base.get_board_mut() {
            let mut to_squish: Vec<usize> = board.plants.iter().enumerate()
                .filter(|(_, p)| {
                    !p.dead && p.base.row >= a_target_row && p.base.row <= a_target_row + 1
                        && p.plant_col >= a_target_col && p.plant_col <= a_target_col + 2
                })
                .map(|(i, _)| i)
                .collect();
            for idx in to_squish.drain(..) {
                // C++: aPlant->Squish()
                board.plants[idx].squish();
            }
            board.shake_board(1, 2);
        }
        if let Some(app) = self.base.get_app() {
            app.play_sample(crate::framework::resources::ResourceId::SoundRvthrow as i32);
        }

        self.summon_counter = 500;
        self.boss_head_counter = 5000;
        if self.boss_mode >= 1 {
            self.boss_stomp_counter = 4000;
        }
        if self.boss_mode >= 2 {
            self.boss_bungee_counter = 6500;
        }
    }

    pub fn boss_spawn_attack(&mut self) {
        self.remove_cold_effects();
        self.zombie_phase = ZombiePhase::BossSpawning;
        if self.boss_mode == 0 {
            self.summon_counter = 450 + RandRange(101);  // RandRangeInt(450, 550)
        } else if self.boss_mode == 1 {
            self.summon_counter = 350 + RandRange(101);  // RandRangeInt(350, 450)
        } else if self.boss_mode == 2 {
            self.summon_counter = 150 + RandRange(101);  // RandRangeInt(150, 250)
        }

        if let Some(board) = self.base.get_board_mut() {
            self.target_row = board.pick_row_for_new_zombie(ZombieType::Normal);
        }

        let a_track_name = match self.target_row {
            0 => "anim_spawn_1",
            1 => "anim_spawn_2",
            2 => "anim_spawn_3",
            3 => "anim_spawn_4",
            _ => "anim_spawn_5",
        };
        self.play_zombie_reanim(a_track_name, ReanimLoopType::PlayOnceAndHold, 20, 12.0);
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::HydraulicShort as i32);
        }
    }

    pub fn boss_spawn_contact(&mut self) {
        let a_zombie_type = if self.zombie_age < 3500 {
            ZombieType::Normal
        } else if self.zombie_age < 8000 {
            ZombieType::TrafficCone
        } else if self.zombie_age < 12500 {
            ZombieType::Pail
        } else {
            let mut a_zombie_type_count = BOSS_ZOMBIE_LIST.len();
            if self.target_row == 0 {
                // C++: PVZP_ASSERT(gBossZombieList[aZombieTypeCount - 1] == ZOMBIE_GARGANTUAR)
                a_zombie_type_count -= 1;
            }
            BOSS_ZOMBIE_LIST[RandRange(a_zombie_type_count as i32) as usize]
        };

        if let Some(board) = self.base.get_board_mut() {
            let idx = board.add_zombie_in_row(a_zombie_type, self.target_row, 0);
            // C++: aZombie->mPosX = 600.0f
            board.zombies[idx].pos_x = 600.0;
        }
    }

    pub fn boss_can_stomp_row(&self, row: i32) -> bool {
        if let Some(board) = self.base.get_board() {
            for plant in &board.plants {
                if plant.dead {
                    continue;
                }
                if !plant.not_on_ground() && plant.base.row >= row && plant.base.row <= row + 1 && plant.plant_col >= 5 {
                    return true;
                }
            }
        }
        false
    }

    pub fn boss_stomp_attack(&mut self) {
        self.remove_cold_effects();
        self.zombie_phase = ZombiePhase::BossStomping;
        self.boss_stomp_counter = 5500 + RandRange(1001);  // RandRangeInt(5500, 6500)

        let mut a_rows_count = 0;
        let mut a_row_array = [0i32; 4];
        for i in 0..4 {
            if self.boss_can_stomp_row(i) {
                a_row_array[a_rows_count as usize] = i;
                a_rows_count += 1;
            }
        }
        if a_rows_count == 0 {
            return;
        }

        self.target_row = a_row_array[RandRange(a_rows_count) as usize];  // PvzpPickFromArray

        let a_track_name = match self.target_row {
            0 => "anim_stomp_1",
            1 => "anim_stomp_2",
            2 => "anim_stomp_3",
            _ => "anim_stomp_4",
        };
        self.play_zombie_reanim(a_track_name, ReanimLoopType::PlayOnceAndHold, 20, 12.0);
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::HydraulicShort as i32);
        }
    }

    pub fn boss_stomp_contact(&mut self) {
        let a_target_row = self.target_row;
        if let Some(board) = self.base.get_board_mut() {
            let mut to_squish: Vec<usize> = board.plants.iter().enumerate()
                .filter(|(_, p)| {
                    !p.dead && p.base.row >= a_target_row && p.base.row <= a_target_row + 1 && p.plant_col >= 5
                })
                .map(|(i, _)| i)
                .collect();
            for idx in to_squish.drain(..) {
                // C++: aPlant->Squish()
                board.plants[idx].squish();
            }
            board.shake_board(1, 4);
        }
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::Thump as i32);
        }
    }

    pub fn boss_bungee_attack(&mut self) {
        self.remove_cold_effects();
        self.zombie_phase = ZombiePhase::BossBungeesEnter;
        self.boss_bungee_counter = 4000 + RandRange(1001);  // RandRangeInt(4000, 5000)
        self.target_col = RandRange(3);  // RandRangeInt(0, 2)

        self.play_zombie_reanim("anim_bungee_1_enter", ReanimLoopType::PlayOnceAndHold, 20, 12.0);
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::HydraulicShort as i32);
            app.play_foley(crate::todlib::tod_foley::FoleyType::BungeeScream as i32);
        }
    }

    pub fn boss_bungee_spawn(&mut self) {
        self.zombie_phase = ZombiePhase::BossBungeesDrop;

        if let Some(board) = self.base.get_board_mut() {
            for i in 0..NUM_BOSS_BUNGEES {
                let idx = board.add_zombie_in_row(ZombieType::Bungee, 0, 0);
                let a_target_col = self.target_col + i as i32;
                {
                    let z = &mut board.zombies[idx];
                    z.pick_bungee_zombie_target(a_target_col);
                    z.altitude = z.pos_y - 30.0;
                }
                // C++: mFollowerZombieID[i] = mBoard->ZombieGetID(aZombie)
                // [TRANSLATION_NOTE]: Rust 侧 ZombieID 即 zombies Vec 索引，add_zombie_in_row 返回的 idx 等价于 ZombieGetID
                self.follower_zombie_ids[i] = idx as ZombieID;
            }
        }
    }

    pub fn boss_bungee_leave(&mut self) {
        self.zombie_phase = ZombiePhase::BossBungeesLeave;

        let follower_ids = self.follower_zombie_ids;
        if let Some(board) = self.base.get_board_mut() {
            for i in 0..NUM_BOSS_BUNGEES {
                if let Some(zombie) = board.zombie_try_to_get_mut(follower_ids[i]) {
                    // C++: if (aZombie && aZombie->mButteredCounter > 0) aZombie->DieWithLoot()
                    if zombie.buttered_counter > 0 {
                        zombie.die_with_loot();
                    }
                }
            }
        }

        self.play_zombie_reanim("anim_bungee_1_leave", ReanimLoopType::PlayOnceAndHold, 20, 18.0);
    }

    pub fn boss_are_bungees_done(&self) -> bool {
        let mut a_bungees_remaining = 0;
        if let Some(board) = self.base.get_board() {
            for i in 0..NUM_BOSS_BUNGEES {
                if let Some(zombie) = board.zombie_try_to_get(self.follower_zombie_ids[i]) {
                    // C++: if (aZombie->mZombiePhase == PHASE_BUNGEE_RISING) return true;
                    if zombie.zombie_phase == ZombiePhase::BungeeRising {
                        return true;
                    }
                    a_bungees_remaining += 1;
                }
            }
        }
        a_bungees_remaining == 0
    }

    pub fn boss_head_attack(&mut self) {
        self.zombie_phase = ZombiePhase::BossHeadEnter;
        self.boss_head_counter = 4000 + RandRange(1001);  // RandRangeInt(4000, 5000)

        self.play_zombie_reanim("anim_head_enter", ReanimLoopType::PlayOnceAndHold, 20, 12.0);
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::HydraulicShort as i32);
        }
    }

    pub fn boss_head_spit(&mut self) {
        // C++: 若已有旧火球动画先销毁
        if let Some(app) = self.base.get_app_mut() {
            if let Some(fireball) = app.reanimation_get_mut(self.boss_fire_ball_reanim_id) {
                fireball.reanimation_die();
            }
        }
        self.boss_fire_ball_reanim_id = REANIMATIONID_NULL;

        self.zombie_phase = ZombiePhase::BossHeadSpit;
        let a_has_6_rows = self.base.get_board().map_or(false, |b| b.stage_has_6_rows());
        self.fireball_row = if a_has_6_rows { RandRange(6) } else { RandRange(5) };  // RandRangeInt(0, 5/4)
        self.is_fire_ball = RandRange(2) == 0;  // RandRangeInt(0, 1) == 0

        let a_track_name = match self.fireball_row {
            0 => "anim_head_attack_1",
            1 => "anim_head_attack_2",
            2 => "anim_head_attack_3",
            3 => "anim_head_attack_4",
            _ => "anim_head_attack_5",
        };
        self.play_zombie_reanim(a_track_name, ReanimLoopType::PlayOnceAndHold, 20, 12.0);

        // C++: 身体动画根据火/冰球覆盖眼睛与嘴部辉光
        if let Some(app) = self.base.get_app_mut() {
            if let Some(body) = app.reanimation_get_mut(self.body_reanim_id) {
                if self.is_fire_ball {
                    body.set_image_override("Boss_eyeglow_red", std::ptr::null_mut());
                    body.set_image_override("Boss_mouthglow_red", std::ptr::null_mut());
                } else {
                    // [TRANSLATION_NOTE]: C++ 传 IMAGE_REANIM_ZOMBIE_BOSS_EYEGLOW_BLUE / MOUTHGLOW_BLUE，
                    // Rust 图像系统未接入这两张资源，暂以空指针占位
                    body.set_image_override("Boss_eyeglow_red", std::ptr::null_mut());
                    body.set_image_override("Boss_mouthglow_red", std::ptr::null_mut());
                }
            }
        }

        // C++: 头部动画播放 anim_drive
        if let Some(app) = self.base.get_app_mut() {
            if let Some(head) = app.reanimation_get_mut(self.special_head_reanim_id) {
                head.play_reanim("anim_drive", ReanimLoopType::Loop, 20, 36.0);
            }
        }
    }

    pub fn boss_destroy_iceball_in_row(&mut self) {
        let fireball_id = self.boss_fire_ball_reanim_id;
        if fireball_id == REANIMATIONID_NULL {
            return;
        }
        // C++: if (aFireBallReanim && !mIsFireBall)
        if self.is_fire_ball {
            return;
        }
        let mut a_pos_x = 0.0f32;
        let mut a_pos_y = 0.0f32;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(fireball) = app.reanimation_get_mut(fireball_id) {
                // C++: mOverlayMatrix.m02 + 80, m12 + 80
                a_pos_x = fireball.m_x + 80.0;
                a_pos_y = fireball.m_y + 80.0;
            }
        }
        // C++: AddPvzpParticle(aPosX, aPosY, 400000, PARTICLE_ICEBALL_DEATH)
        if let Some(app) = self.base.get_app_mut() {
            app.add_tod_particle(a_pos_x, a_pos_y, 400000, ParticleEffect::IceballDeath as i32);
        }
        if let Some(app) = self.base.get_app_mut() {
            if let Some(fireball) = app.reanimation_get_mut(fireball_id) {
                fireball.reanimation_die();
            }
        }
        self.boss_fire_ball_reanim_id = REANIMATIONID_NULL;
        if let Some(board) = self.base.get_board_mut() {
            board.remove_particle_by_type(ParticleEffect::IceballTrail);
        }
    }

    pub fn boss_destroy_fireball(&mut self) {
        let fireball_id = self.boss_fire_ball_reanim_id;
        if fireball_id == REANIMATIONID_NULL {
            return;
        }
        // C++: if (aFireBallReanim && mIsFireBall)
        if !self.is_fire_ball {
            return;
        }
        let mut a_pos_x = 0.0f32;
        let mut a_pos_y = 0.0f32;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(fireball) = app.reanimation_get_mut(fireball_id) {
                // C++: mOverlayMatrix.m02 + 80, m12 + 40
                a_pos_x = fireball.m_x + 80.0;
                a_pos_y = fireball.m_y + 40.0;
            }
        }
        // C++: 向四周放出 6 个辣椒火动画
        for i in 0..6 {
            let a_angle = 2.0 * std::f32::consts::PI * i as f32 / 6.0 + std::f32::consts::PI / 2.0;
            let a_reanim_ptr = self.base.get_app_mut().and_then(|app| {
                app.add_reanimation(
                    a_pos_x + 60.0 * a_angle.sin(),
                    a_pos_y + 60.0 * a_angle.cos(),
                    400000,
                    ReanimationType::JalapenoFire as i32,
                )
            });
            if let Some(ptr) = a_reanim_ptr {
                if let Some(app) = self.base.get_app_mut() {
                    let a_id = app.reanimation_get_id(ptr);
                    if let Some(reanim) = app.reanimation_get_mut(a_id) {
                        // C++: mAnimTime = 0.2; mLoopType = PLAY_ONCE_FULL_LAST_FRAME; mAnimRate = RandRangeFloat(20, 25)
                        reanim.m_anim_time = 0.2;
                        reanim.m_loop_type = ReanimLoopType::PlayOnceFullLastFrame;
                        reanim.m_anim_rate = 20.0 + RandFloat(5.0);
                    }
                }
            }
        }
        if let Some(app) = self.base.get_app_mut() {
            if let Some(fireball) = app.reanimation_get_mut(fireball_id) {
                fireball.reanimation_die();
            }
        }
        self.boss_fire_ball_reanim_id = REANIMATIONID_NULL;
        if let Some(board) = self.base.get_board_mut() {
            board.remove_particle_by_type(ParticleEffect::FireballTrail);
        }
    }

    pub fn boss_head_spit_effect(&mut self) {
        // C++: 从身体动画的 Boss_jaw 轨道取火焰生成位置
        let mut a_flame_pos_x = self.pos_x;
        let mut a_flame_pos_y = self.pos_y;
        if let Some(app) = self.base.get_app() {
            if let Some(body) = app.reanimation_get(self.body_reanim_id) {
                let a_track_index = body.find_track_index("Boss_jaw");
                let mut a_transform = crate::todlib::definition::ReanimatorTransform::default();
                if body.get_current_transform(a_track_index, &mut a_transform) {
                    a_flame_pos_x = self.pos_x + a_transform.m_trans_x + 100.0;
                    a_flame_pos_y = self.pos_y + a_transform.m_trans_y + 50.0;
                }
            }
        }
        let a_render_order = self.base.render_order + 2;
        if let Some(app) = self.base.get_app_mut() {
            // C++: AddPvzpParticle(... PARTICLE_ZOMBIE_BOSS_FIREBALL)
            app.add_tod_particle(a_flame_pos_x, a_flame_pos_y, a_render_order, ParticleEffect::ZombieBossFireball as i32);
        }
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::BossBoulderAttack as i32);
        }
    }

    pub fn boss_head_spit_contact(&mut self) {
        // C++: PVZP_ASSERT(!mApp->ReanimationTryToGet(mBossFireBallReanimID))
        let fireball_row = self.fireball_row;
        let is_fire_ball = self.is_fire_ball;
        let a_render_order = self.base.render_order + 1;
        let a_pos_y = self.base.get_board()
            .map_or(0.0, |b| b.get_pos_y_based_on_row(550.0, fireball_row)) - 90.0;

        let a_fireball_ptr = if is_fire_ball {
            self.base.get_app_mut().and_then(|app| {
                app.add_reanimation(455.0, a_pos_y, a_render_order, ReanimationType::BossFireball as i32)
            })
        } else {
            self.base.get_app_mut().and_then(|app| {
                app.add_reanimation(455.0, a_pos_y, a_render_order, ReanimationType::BossIceball as i32)
            })
        };

        if let Some(ptr) = a_fireball_ptr {
            if let Some(app) = self.base.get_app_mut() {
                let a_fireball_id = app.reanimation_get_id(ptr);
                if let Some(fireball) = app.reanimation_get_mut(a_fireball_id) {
                    fireball.play_reanim("anim_form", ReanimLoopType::PlayOnceAndHold, 0, 16.0);
                    fireball.m_is_attachment = true;
                    if is_fire_ball {
                        // C++: RENDER_GROUP_BOSS_FIREBALL_ADDITIVE
                        fireball.assign_render_group_to_track("additive", 7);
                        fireball.assign_render_group_to_track("superglow", 7);
                    } else {
                        fireball.assign_render_group_to_track("ice_highlight", 7);
                    }
                }
                self.boss_fire_ball_reanim_id = a_fireball_id;
            }
        }
        if let Some(app) = self.base.get_app_mut() {
            if let Some(head) = app.reanimation_get_mut(self.special_head_reanim_id) {
                head.play_reanim("anim_laugh", ReanimLoopType::Loop, 20, 18.0);
            }
        }
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::HydraulicShort as i32);
        }
    }

    pub fn update_boss_fireball(&mut self) {
        let fireball_id = self.boss_fire_ball_reanim_id;
        if fireball_id == REANIMATIONID_NULL {
            return;
        }

        // C++: aSpeed = GetTrackVelocity("_ground"); m02 -= aSpeed
        let mut a_speed = 0.0f32;
        let mut a_pos_x = 0.0f32;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(fireball) = app.reanimation_get_mut(fireball_id) {
                a_speed = fireball.get_track_velocity("_ground");
                fireball.m_x -= a_speed;
                a_pos_x = fireball.m_x;
            }
        }

        let fireball_row = self.fireball_row;
        let a_pos_y = self.base.get_board()
            .map_or(0.0, |b| b.get_pos_y_based_on_row(a_pos_x + 75.0, fireball_row)) - 90.0;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(fireball) = app.reanimation_get_mut(fireball_id) {
                // C++: mOverlayMatrix.m12 = aPosY
                fireball.m_y = a_pos_y;
            }
        }

        if a_pos_x < -180.0 {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(fireball) = app.reanimation_get_mut(fireball_id) {
                    fireball.reanimation_die();
                }
            }
            self.boss_fire_ball_reanim_id = REANIMATIONID_NULL;
        }

        // C++: SquishAllInSquare(PixelToGridX(aPosX + 75, aPosY), mFireballRow, ATTACKTYPE_DRIVE_OVER)
        let a_grid_x = self.base.get_board()
            .map_or(0, |b| b.pixel_to_grid_x((a_pos_x + 75.0) as i32, a_pos_y as i32));
        self.squish_all_in_square(a_grid_x, fireball_row, ZombieAttackType::DriveOver);

        // C++: 碾压火球所在行的割草机
        if let Some(board) = self.base.get_board_mut() {
            for mower in &mut board.lawn_mowers {
                if mower.dead {
                    continue;
                }
                if mower.mower_state != LawnMowerState::Squished
                    && mower.base.row == fireball_row
                    && mower.pos_x > a_pos_x
                    && mower.pos_x < a_pos_x + 50.0
                {
                    mower.squish_mower();
                }
            }
        }

        // C++: 火/冰球滚动与拖尾
        let is_fire_ball = self.is_fire_ball;
        let mut a_loop_type = ReanimLoopType::PlayOnceAndHold;
        let mut a_loop_count = 0i32;
        if let Some(app) = self.base.get_app() {
            if let Some(fireball) = app.reanimation_get(fireball_id) {
                a_loop_type = fireball.m_loop_type;
                a_loop_count = fireball.m_loop_count;
            }
        }
        if a_loop_type == ReanimLoopType::PlayOnceAndHold && a_loop_count > 0 {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(fireball) = app.reanimation_get_mut(fireball_id) {
                    fireball.play_reanim("anim_role", ReanimLoopType::Loop, 0, 2.0);
                    fireball.m_render_order = crate::lawn::board::make_render_order(
                        crate::lawn::game_enums::RENDER_LAYER_PARTICLE,
                        fireball_row,
                        0,
                    );
                }
            }
        }
        if a_loop_type == ReanimLoopType::Loop && RandRange(10) == 0 {
            let a_ball_pos_x = a_pos_x + 100.0 + RandFloat(20.0);  // RandRangeFloat(0, 20)
            let a_ball_pos_y = self.base.get_board()
                .map_or(0.0, |b| b.get_pos_y_based_on_row(a_ball_pos_x - 40.0, fireball_row))
                + 90.0 + (-50.0 + RandFloat(50.0));  // RandRangeFloat(-50, 0)
            let a_render_position = crate::lawn::board::make_render_order(
                crate::lawn::game_enums::RENDER_LAYER_GRAVE_STONE,
                fireball_row,
                6,
            );
            if let Some(app) = self.base.get_app_mut() {
                if is_fire_ball {
                    app.add_tod_particle(a_ball_pos_x, a_ball_pos_y, a_render_position, ParticleEffect::FireballTrail as i32);
                } else {
                    app.add_tod_particle(a_ball_pos_x, a_ball_pos_y, a_render_position, ParticleEffect::IceballTrail as i32);
                }
            }
        }

        if let Some(app) = self.base.get_app_mut() {
            if let Some(fireball) = app.reanimation_get_mut(fireball_id) {
                fireball.update();
            }
        }
    }

    pub fn boss_start_death(&mut self) {
        self.zombie_phase = ZombiePhase::BossHeadLeave;
        self.play_zombie_reanim("anim_head_leave", ReanimLoopType::PlayOnceAndHold, 0, 24.0);

        // C++: AddPvzpParticle(700, 150, 400000, PARTICLE_BOSS_EXPLOSION)
        if let Some(app) = self.base.get_app_mut() {
            app.add_tod_particle(700.0, 150.0, 400000, ParticleEffect::BossExplosion as i32);
        }
        if let Some(app) = self.base.get_app() {
            app.play_sample(crate::framework::resources::ResourceId::SoundBossexplosion as i32);
            app.play_foley(crate::todlib::tod_foley::FoleyType::Gargantudeath as i32);
        }

        self.boss_die();
    }

    pub fn boss_die(&mut self) {
        if !self.is_on_board() {
            return;
        }

        // C++: 销毁火球动画并清理火/冰球
        let fireball_id = self.boss_fire_ball_reanim_id;
        if fireball_id != REANIMATIONID_NULL {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(fireball) = app.reanimation_get_mut(fireball_id) {
                    fireball.reanimation_die();
                }
            }
            self.boss_fire_ball_reanim_id = REANIMATIONID_NULL;
            self.boss_destroy_iceball_in_row();
            self.boss_destroy_fireball();
        }

        // C++: mApp->mMusic->FadeOut(200)
        if let Some(app) = self.base.get_app_mut() {
            if let Some(music) = app.music.as_mut() {
                music.fade_out(200);
            }
        }

        let self_ptr = self as *const Zombie;
        if let Some(board) = self.base.get_board_mut() {
            for idx in 0..board.zombies.len() {
                let z_ptr = &board.zombies[idx] as *const Zombie;
                if std::ptr::eq(z_ptr, self_ptr) {
                    continue;  // C++: aZombie != this
                }
                if !board.zombies[idx].dead && !board.zombies[idx].is_dead_or_dying() {
                    board.zombies[idx].die_with_loot();
                }
            }
        }
        self.remove_cold_effects();
    }

    pub fn apply_boss_smoke_particles(&mut self, the_enable: bool) {
        // [TRANSLATION_NOTE]: C++ 用 AttachEffect 将 PARTICLE_ZAMBONI_SMOKE 附着到 "Boss_head" 轨道，
        // Rust 侧 AttachEffect/粒子系统为 stub，仅保留 Enable 时生成粒子的骨架
        if the_enable {
            if let Some(app) = self.base.get_app_mut() {
                app.add_tod_particle(0.0, 0.0, 0, ParticleEffect::ZamboniSmoke as i32);
                app.add_tod_particle(0.0, 0.0, 0, ParticleEffect::ZamboniSmoke as i32);
                // C++: 血量低于 BOSS_FLASH_HEALTH_FRACTION(10) 时再多一个
                if self.body_health < self.body_max_health / 10 {
                    app.add_tod_particle(0.0, 0.0, 0, ParticleEffect::ZamboniSmoke as i32);
                }
            }
        }
    }

    /// 被割草机碾压（对应 C++ MowDown）
    pub fn mow_down(&mut self) {
        if self.dead || self.zombie_phase == ZombiePhase::Mowered || self.zombie_type == ZombieType::Boss {
            return;
        }
        if self.zombie_type == ZombieType::Catapult || self.zombie_type == ZombieType::Zamboni {
            self.die_with_loot();
            return;
        }
        if self.zombie_phase == ZombiePhase::Dying
            || self.zombie_phase == ZombiePhase::PolevaulterInVault
            || self.zombie_phase == ZombiePhase::RisingFromGrave
            || self.zombie_phase == ZombiePhase::DancerRising
            || self.zombie_phase == ZombiePhase::SnorkelIntoPool
            || self.zombie_phase == ZombiePhase::Burned
            || self.zombie_type == ZombieType::Gargantuar
            || self.zombie_type == ZombieType::RedeEyeGargantuar
            || self.zombie_type == ZombieType::Bungee
            || self.zombie_type == ZombieType::Digger
            || self.zombie_type == ZombieType::Imp
            || self.zombie_type == ZombieType::Yeti
            || self.zombie_type == ZombieType::DolphinRider
            || self.is_bobsled_team_with_sled()
            || self.is_flying()
            || self.in_pool
        {
            // 泳池行不掉落部位
            let is_pool = self.base.board.map_or(false, |b| unsafe {
                let row = self.base.row as usize;
                row < (*b).m_plant_row.len() && (*b).m_plant_row[row] == PlantRowType::Pool
            });
            if !is_pool {
                self.drop_head(0);
                self.drop_arm(0);
                self.drop_helm(0);
                self.drop_shield(0);
            }
            self.die_with_loot();
            return;
        }

        if self.ice_trap_counter > 0 {
            self.remove_ice_trap();
        }
        self.buttered_counter = self.buttered_counter.min(0);

        self.drop_shield(0);
        self.drop_helm(0);
        if self.zombie_type == ZombieType::Flag {
            self.drop_flag();
        } else if self.zombie_type == ZombieType::Polevaulter {
            self.drop_pole();
        } else if self.zombie_type == ZombieType::Newspaper || self.zombie_type == ZombieType::Balloon {
            self.drop_head(0);
        } else if self.zombie_type == ZombieType::Pogo {
            self.drop_head(0);
            self.altitude = 0.0;
        }

        self.zombie_phase = ZombiePhase::Mowered;
        self.drop_loot();
    }

    pub fn start_eating(&mut self) {
        if self.is_eating {
            return;
        }
        self.is_eating = true;

        if self.zombie_phase == ZombiePhase::DiggerTunneling {
            return;
        }
        if self.zombie_phase == ZombiePhase::LadderCarrying {
            self.play_zombie_reanim("anim_laddereat", ReanimLoopType::Loop, 20, 0.0);
        } else if self.zombie_phase == ZombiePhase::NewspaperMad {
            self.play_zombie_reanim("anim_eat_nopaper", ReanimLoopType::Loop, 20, 0.0);
        } else {
            if self.zombie_type != ZombieType::Snorkel {
                self.play_zombie_reanim("anim_eat", ReanimLoopType::Loop, 20, 0.0);
            }
            if self.shield_type == ShieldType::Door {
                self.show_door_arms(false);
            }
        }
    }

    pub fn eat_zombie(&mut self, zombie: &mut Zombie) {
        zombie.take_damage(DAMAGE_PER_EAT, 9);  // DAMAGE_PER_EAT = TICKS_BETWEEN_EATS = 4
        self.start_eating();
        if zombie.body_health <= 0 {
            // C++: mApp->PlaySample(SOUND_GULP)——吞吃音效未接入
        }
    }

    pub fn is_zombotany(zombie_type: ZombieType) -> bool {
        matches!(zombie_type,
            ZombieType::PeaHead | ZombieType::WallnutHead | ZombieType::TallnutHead
            | ZombieType::JalapenoHead | ZombieType::GatlingHead | ZombieType::SquashHead)
    }

    pub fn can_be_chilled(&self) -> bool {
        if self.zombie_type == ZombieType::Zamboni || self.is_bobsled_team_with_sled() {
            return false;
        }
        if self.is_dead_or_dying() {
            return false;
        }
        if self.zombie_phase == ZombiePhase::DiggerTunneling
            || self.zombie_phase == ZombiePhase::DiggerRising
            || self.zombie_phase == ZombiePhase::DiggerTunnelingPauseWithoutAxe
            || self.zombie_phase == ZombiePhase::DiggerRiseWithoutAxe
            || self.zombie_phase == ZombiePhase::RisingFromGrave
            || self.zombie_phase == ZombiePhase::DancerRising
        {
            return false;
        }
        if self.mind_controlled {
            return false;
        }
        self.zombie_type != ZombieType::Boss
            || self.zombie_phase == ZombiePhase::BossHeadIdleBeforeSpit
            || self.zombie_phase == ZombiePhase::BossHeadIdleAfterSpit
            || self.zombie_phase == ZombiePhase::BossHeadSpit
    }

    pub fn can_be_frozen(&self) -> bool {
        if !self.can_be_chilled() {
            return false;
        }
        if self.zombie_phase == ZombiePhase::PolevaulterInVault
            || self.zombie_phase == ZombiePhase::DolphinIntoPool
            || self.zombie_phase == ZombiePhase::DolphinInJump
            || self.zombie_phase == ZombiePhase::SnorkelIntoPool
            || self.is_flying()
            || self.zombie_phase == ZombiePhase::ImpGettingThrown
            || self.zombie_phase == ZombiePhase::ImpLanding
            || self.zombie_phase == ZombiePhase::BobsledCrashing
            || self.zombie_phase == ZombiePhase::JackInTheBoxPopping
            || self.zombie_phase == ZombiePhase::SquashRising
            || self.zombie_phase == ZombiePhase::SquashFalling
            || self.zombie_phase == ZombiePhase::SquashDoneFalling
            || self.is_bouncing_pogo()
        {
            return false;
        }
        self.zombie_type != ZombieType::Bungee || self.zombie_phase == ZombiePhase::BungeeAtBottom
    }

    pub fn is_fire_resistant(&self) -> bool {
        self.zombie_type == ZombieType::Catapult
            || self.zombie_type == ZombieType::Zamboni
            || self.shield_type == ShieldType::Door
            || self.shield_type == ShieldType::Ladder
    }

    pub fn effected_by_damage(&self, damage_range_flags: u32) -> bool {
        // 对应 C++ EffectedByDamage：按伤害范围标志与僵尸状态判定是否有效伤害
        if !test_bit(damage_range_flags, 5) && self.is_dead_or_dying() {
            return false;
        }
        if test_bit(damage_range_flags, 7) {
            if !self.mind_controlled {
                return false;
            }
        } else if self.mind_controlled {
            return false;
        }
        if self.zombie_type == ZombieType::Bungee
            && self.zombie_phase != ZombiePhase::BungeeAtBottom
            && self.zombie_phase != ZombiePhase::BungeeGrabbing
        {
            return false;
        }
        if self.zombie_height == ZombieHeight::GettingBungeeDropped {
            return false;
        }
        if self.zombie_type == ZombieType::Boss {
            // C++: mZombieType == ZOMBIE_BOSS 时仅特定头部阶段可被伤害
            if self.zombie_phase == ZombiePhase::BossHeadEnter {
                return false;
            }
            if self.zombie_phase == ZombiePhase::BossHeadLeave {
                return false;
            }
        }
        true
    }

    /// 是否站在荆棘草上（对应 C++ IsStandingOnSpikeweed）
    pub fn is_standing_on_spikeweed(&self) -> bool {
        if self.zombie_type == ZombieType::Zamboni || self.zombie_type == ZombieType::Catapult {
            return false;
        }
        let zombie_rect = self.get_zombie_rect();
        let board = match self.base.board { Some(b) => b, None => return false };
        unsafe {
            let b = &*board;
            for plant in &b.plants {
                if plant.dead { continue; }
                if plant.base.row == self.base.row && plant.is_spiky() && !plant.not_on_ground()
                    && (!self.on_high_ground || plant.is_on_high_ground())
                {
                    let plant_attack_rect = plant.get_plant_attack_rect(PlantWeapon::Primary);
                    if crate::lawn::board::get_rect_overlap(&plant_attack_rect, &zombie_rect) > 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn is_tangle_kelp_target(&self) -> bool {
        if self.zombie_height == ZombieHeight::DraggedUnder {
            return true;
        }
        // C++ 默认返回 false（其余情况不判定为有效伤害）
        false
    }

    pub fn is_squash_target(&self, the_except: Option<&Plant>) -> bool {
        let board = match self.base.get_board() {
            Some(b) => b,
            None => return false,
        };

        let an_id = match board.zombies.iter().position(|z| std::ptr::eq(z, self)) {
            Some(i) => i as ZombieID,
            None => ZOMBIEID_NULL,
        };

        for plant in &board.plants {
            if plant.dead {
                continue;
            }
            let is_the_except = match the_except {
                Some(p) => std::ptr::eq(plant, p),
                None => false,
            };
            if !is_the_except && plant.seed_type == SeedType::Squash && plant.target_zombie_id == an_id {
                return true;
            }
        }

        false
    }

    pub fn apply_butter(&mut self) {
        if !self.has_head || !self.can_be_frozen() {
            return;
        }
        if self.zombie_type == ZombieType::Zamboni || self.zombie_type == ZombieType::Boss
            || self.is_tangle_kelp_target() || self.is_bobsled_team_with_sled() || self.is_flying()
        {
            return;
        }

        self.buttered_counter = 400;
        // C++: ZombieTryToGet(mRelatedZombieID) 后互清 related ID（Zombie.cpp:8523）；Rust 未保留该清理

        if self.zombie_type == ZombieType::Pogo {
            self.altitude = 0.0;
            if self.on_high_ground {
                self.altitude += HIGH_GROUND_HEIGHT;
            }
        } else if self.zombie_type == ZombieType::Balloon {
            // C++: BalloonPropellerHatSpin(false)——气球螺旋桨停转
        } else if Self::is_zombotany(self.zombie_type) {
            // C++: aHeadReanim->mAnimRate = 0.0f——植物头僵尸停止动画（reanim 覆盖未接入）
        }

        self.update_anim_speed();
        self.stop_zombie_sound();
    }

    pub fn apply_burn(&mut self) {
        if self.dead || self.zombie_phase == ZombiePhase::Burned {
            return;
        }

        if self.body_health >= 1800 || self.zombie_type == ZombieType::Boss {
            self.take_damage(1800, 18);
            return;
        }

        if self.zombie_type == ZombieType::SquashHead && !self.has_head {
            // C++: RemoveReanimation(mSpecialHeadReanimID) + 置空（Zombie.cpp:8672）
            self.special_head_reanim_id = REANIMATIONID_NULL;
        }

        if self.ice_trap_counter > 0 {
            self.remove_ice_trap();
        }
        self.buttered_counter = self.buttered_counter.min(0);

        // C++: AttachmentDetachCrossFadeParticleType(PARTICLE_ZAMBONI_SMOKE) + BungeeDropPlant()
        //（附着粒子/绳降依赖未接入，见 Zombie.cpp:8680）

        if self.zombie_phase == ZombiePhase::Dying
            || self.zombie_phase == ZombiePhase::PolevaulterInVault
            || self.zombie_phase == ZombiePhase::ImpGettingThrown
            || self.zombie_phase == ZombiePhase::RisingFromGrave
            || self.zombie_phase == ZombiePhase::DancerRising
            || self.zombie_phase == ZombiePhase::DolphinIntoPool
            || self.zombie_phase == ZombiePhase::DolphinInJump
            || self.zombie_phase == ZombiePhase::DolphinRiding
            || self.zombie_phase == ZombiePhase::SnorkelIntoPool
            || self.zombie_phase == ZombiePhase::DiggerTunneling
            || self.zombie_phase == ZombiePhase::DiggerTunnelingPauseWithoutAxe
            || self.zombie_phase == ZombiePhase::DiggerRising
            || self.zombie_phase == ZombiePhase::DiggerRiseWithoutAxe
            || self.zombie_phase == ZombiePhase::Mowered
            || self.in_pool
        {
            self.die_with_loot();
        } else if self.zombie_type == ZombieType::Bungee
            || self.zombie_type == ZombieType::Yeti
            || Self::is_zombotany(self.zombie_type)
            || self.is_bobsled_team_with_sled()
            || self.is_flying()
            || !self.has_head
        {
            self.set_anim_rate(0.0);
            self.zombie_phase = ZombiePhase::Burned;
            self.phase_counter = 300;
            self.just_got_shot_counter = 0;
            self.drop_loot();

            if self.zombie_type == ZombieType::Balloon {
                // C++: BalloonPropellerHatSpin(false)——燃烧气球停转螺旋桨（Zombie.cpp:8712）
            }
        } else {
            // C++ 其余情况：DieWithLoot()
            self.die_with_loot();
        }
        // 对应 C++ ApplyBurn（Zombie.cpp:8660）结束
    }

    pub fn hit_ice_trap(&mut self) {
        let mut cold = false;
        if self.chilled_counter > 0 || self.ice_trap_counter != 0 {
            cold = true;
        }

        self.apply_chill(true);
        if !self.can_be_frozen() {
            return;
        }

        if self.in_pool {
            self.ice_trap_counter = 300;
        } else if cold {
            self.ice_trap_counter = 300 + RandRange(100);  // RandRangeInt(300, 400)
        } else {
            self.ice_trap_counter = 400 + RandRange(200);  // RandRangeInt(400, 600)
        }

        self.stop_zombie_sound();
        if self.zombie_type == ZombieType::Balloon {
            // C++: 冰阱命中时气球螺旋桨停转（BalloonPropellerHatSpin(false)）
        }
        if self.zombie_phase == ZombiePhase::BossHeadSpit {
            // C++: Boss 吐息阶段冻结特殊头动画（mSpecialHeadReanimID）
        }

        self.take_damage(20, 1);
        self.update_anim_speed();
    }

    pub fn convert_to_normal_zombie(&mut self) {
        self.stop_zombie_sound();
        self.pos_y = self.get_pos_y_based_on_row(self.base.row);
        self.base.x = self.pos_x as i32;
        self.base.y = self.pos_y as i32;

        self.zombie_type = ZombieType::Normal;
        self.zombie_phase = ZombiePhase::Normal;
        self.zombie_attack_rect = Rect::new(50, 0, 20, 115);

        self.anim_frames = 12;
        self.anim_ticks_per_frame = 12;
        self.phase_counter = 0;

        self.pick_random_speed();
    }

    pub fn rise_from_grave(&mut self, col: i32, row: i32) {
        // PVZP_ASSERT(mZombiePhase == PHASE_ZOMBIE_NORMAL)
        let has_pool = self.base.get_board().map_or(false, |b| b.stage_has_pool());

        if let Some(board) = self.base.get_board() {
            self.pos_x = board.grid_to_pixel_x(col, self.base.row) as f32 - 25.0;
        }
        self.pos_y = self.get_pos_y_based_on_row(row);
        self.set_row(row);
        self.base.x = self.pos_x as i32;
        self.base.y = self.pos_y as i32;
        self.altitude = -200.0; // CLIP_HEIGHT_OFF
        self.zombie_phase = ZombiePhase::RisingFromGrave;
        self.phase_counter = 150;

        if has_pool {
            self.altitude = -150.0;
            self.in_pool = true;
            self.phase_counter = 50;
            self.zombie_height = ZombieHeight::Normal;
            self.start_walk_anim(0);
            // C++: 水池分支完成（进入泳池行走，见 RiseFromGrave Zombie.cpp:8184）
        } else {
            // C++: 非水池分支（陆地出土）
        }
    }

    pub fn walk_into_house(&mut self) {
        // 对应 C++ Zombie::WalkIntoHouse（Zombie.cpp:9633）：胜利后僵尸走入屋内
        self.from_wave = Zombie::ZOMBIE_WAVE_WINNER;
        self.reanim_reenable_clipping();

        if self.zombie_phase == ZombiePhase::PolevaulterPreVault {
            self.zombie_phase = ZombiePhase::PolevaulterPostVault;
            self.start_walk_anim(0);
        }

        let a_background = self.base.get_board().map_or(BackgroundType::Day, |b| b.m_background_type);
        let has_pool = self.base.get_board().map_or(false, |b| b.stage_has_pool());

        if a_background == BackgroundType::Day
            || a_background == BackgroundType::Night
            || a_background == BackgroundType::Pool
            || a_background == BackgroundType::Fog
        {
            self.pos_y = 290.0;
            self.base.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_ZOMBIE, 2, 0);

            if self.zombie_type == ZombieType::Gargantuar
                || self.zombie_type == ZombieType::RedeEyeGargantuar
            {
                self.pos_y += 30.0;
            } else if self.zombie_phase == ZombiePhase::PolevaulterPreVault {
                self.pos_x += 35.0;
            } else if self.zombie_type == ZombieType::Zamboni {
                self.pos_y += 15.0;
            }

            if has_pool {
                if self.zombie_type == ZombieType::Football {
                    self.pos_x -= 10.0;
                } else {
                    self.pos_x -= 80.0;
                }
            }
        } else if a_background == BackgroundType::Roof || a_background == BackgroundType::Boss {
            self.pos_x = -180.0;
            self.pos_y = 250.0;
            self.zombie_height = ZombieHeight::InToChimney;
            self.base.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_GRAVE_STONE, 0, 2);

            if self.zombie_type == ZombieType::Gargantuar
                || self.zombie_type == ZombieType::RedeEyeGargantuar
            {
                self.pos_y += 5.0;
            } else if self.zombie_type == ZombieType::Football {
                self.pos_x -= 14.0;
            } else if self.zombie_type == ZombieType::Zamboni {
                self.pos_x -= 28.0;
            }
            // C++: 位置微调后完成胜利行走（WalkIntoHouse 结束）
        }
    }


    /// 弹簧折断（对应 C++ PogoBreak）
    pub fn pogo_break(&mut self, damage_flags: u32) {
        if !self.has_object {
            return;
        }

        if !test_bit(damage_flags, DAMAGE_DOESNT_LEAVE_BODY) {
            // 依赖底层系统
        }

        self.zombie_height = ZombieHeight::Falling;
        self.zombie_phase = ZombiePhase::Normal;
        self.start_walk_anim(0);
        self.zombie_rect = Rect::new(36, 17, 42, 115);
        self.zombie_attack_rect = Rect::new(20, 17, 50, 115);
        self.shield_health = 0;
        self.shield_type = ShieldType::None;
        self.has_object = false;
    }

    /// 是否在弹跳（对应 C++ IsBouncingPogo）
    pub fn is_bouncing_pogo(&self) -> bool {
        let phase = self.zombie_phase as i32;
        phase >= ZombiePhase::PogoBouncing as i32 && phase <= ZombiePhase::PogoForwardBounce7 as i32
    }

    /// 更新弹簧僵尸（对应 C++ UpdateZombiePogo）
    pub fn update_zombie_pogo(&mut self) {
        if self.is_dead_or_dying() || self.is_immobilized() || !self.is_bouncing_pogo()
            || self.zombie_height == ZombieHeight::InToChimney
        {
            return;
        }

        let mut a_height = 40.0;
        let phase_i = self.zombie_phase as i32;
        if phase_i >= ZombiePhase::PogoHighBounce1 as i32 && phase_i <= ZombiePhase::PogoHighBounce6 as i32 {
            a_height = 50.0 + 20.0 * (self.zombie_phase as i32 - ZombiePhase::PogoHighBounce1 as i32) as f32;
        } else if self.zombie_phase == ZombiePhase::PogoForwardBounce2 {
            a_height = 90.0;
        } else if self.zombie_phase == ZombiePhase::PogoForwardBounce7 {
            a_height = 170.0;
        }
        // PvzpAnimateCurveFloat(POGO_BOUNCE_TIME, 0, mPhaseCounter, 9.0f, aHeight + 9.0f, CURVE_BOUNCE_SLOW_MIDDLE)
        self.altitude = crate::todlib::tod_common::tod_animate_curve_float(
            POGO_BOUNCE_TIME, 0, self.phase_counter, 9.0, a_height + 9.0, TodCurves::BounceSlowMiddle,
        );
        self.frame = (3 - self.altitude as i32 / 3).clamp(0, 3);

        // 在棋板上时播放弹簧音效
        if self.is_on_board() && self.phase_counter == 5 {
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::PogoZombie as i32);
            }
        }

        // 高台处理
        if self.zombie_height == ZombieHeight::UpToHighGround {
            self.altitude += 100.0; // HIGH_GROUND_HEIGHT
            self.zombie_height = ZombieHeight::Normal;
        } else if self.zombie_height == ZombieHeight::DownOffHighGround {
            self.on_high_ground = false;
            self.zombie_height = ZombieHeight::Normal;
        } else if self.on_high_ground {
            self.altitude += 100.0; // HIGH_GROUND_HEIGHT
        }

        // 前跳遇到高坚果
        if self.zombie_phase == ZombiePhase::PogoForwardBounce2 && self.phase_counter == 70 {
            // 依赖底层系统
            // 若目标是高坚果 → PogoBreak
            let target_plant = self.find_tallnut_target();
            if target_plant {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Bonk as i32);
                }
                self.shield_type = ShieldType::None;
                self.pogo_break(0);
                return;
            }
        }

        if self.phase_counter != 0 {
            return;
        }

        // 相位切换
        let a_plant = if self.is_on_board() {
            self.find_vault_target()
        } else {
            false
        };
        if !a_plant {
            self.zombie_phase = ZombiePhase::PogoBouncing;
            self.pick_random_speed();
            self.phase_counter = POGO_BOUNCE_TIME;
            return;
        }

        if self.zombie_phase == ZombiePhase::PogoHighBounce1 {
            self.zombie_phase = ZombiePhase::PogoForwardBounce2;
            // mVelX = (mX - aPlant->mX + 60) / POGO_BOUNCE_TIME
            self.vel_x = (self.base.x as f32 - 300.0 + 60.0) / POGO_BOUNCE_TIME as f32;
            self.phase_counter = POGO_BOUNCE_TIME;
        } else {
            self.zombie_phase = ZombiePhase::PogoHighBounce1;
            self.vel_x = 0.0;
            self.phase_counter = POGO_BOUNCE_TIME;
        }
    }

    /// 寻找可跳越的植物目标（对应 C++ FindPlantTarget(ATTACKTYPE_VAULT) 简化）
    fn find_vault_target(&self) -> bool {
        self.find_plant_target_index(ZombieAttackType::Vault).is_some()
    }

    /// 寻找高坚果目标（对应 C++ FindPlantTarget 检查 SEED_TALLNUT 简化）
    fn find_tallnut_target(&self) -> bool {
        if let Some(idx) = self.find_plant_target_index(ZombieAttackType::Vault) {
            if let Some(board) = self.base.get_board() {
                if idx < board.plants.len() {
                    return board.plants[idx].seed_type == SeedType::Tallnut;
                }
            }
        }
        false
    }

    /// 更新僵尸行走（对应 C++ Zombie::Animate）
    pub fn animate(&mut self) {
        self.prev_frame = self.frame;
        // 某些相位下不播放动画
        if self.zombie_phase == ZombiePhase::JackInTheBoxPopping
            || self.zombie_phase == ZombiePhase::NewspaperMaddening
            || self.zombie_phase == ZombiePhase::DiggerRising
            || self.zombie_phase == ZombiePhase::DiggerTunnelingPauseWithoutAxe
            || self.zombie_phase == ZombiePhase::DiggerRiseWithoutAxe
            || self.zombie_phase == ZombiePhase::DiggerStunned
            || self.is_immobilized()
        {
            return;
        }

        self.anim_counter += 1;
        if self.yucky_face {
            self.update_yucky_face();
        }

        if self.is_eating && self.has_head {
            // 进食动画帧速率为 6
            let a_frame_length = 6;
            if self.anim_counter >= a_frame_length {
                self.anim_counter = 0;
                self.frame = (self.frame + 1) % 2;
            }
        } else {
            if self.anim_counter >= self.anim_ticks_per_frame {
                self.anim_counter = 0;
                self.frame = (self.frame + 1) % self.anim_frames;
            }
        }
    }

    /// 是否被冻结/黄油（对应 C++ IsImmobilizied）
    pub fn is_immobilized(&self) -> bool {
        self.ice_trap_counter > 0 || self.buttered_counter > 0
    }

    /// 更新恶心表情（对应 C++ UpdateYuckyFace）
    pub fn update_yucky_face(&mut self) {
        self.yucky_face_counter += 1;
        if self.yucky_face_counter > 270 {
            self.yucky_face = false;
            self.yucky_face_counter = 0;
        }
    }

    /// 更新僵尸行走（对应 C++ UpdateZombieWalking）
    pub fn update_zombie_walking(&mut self) {
        if self.zombie_not_walking() {
            return;
        }

        self.base.x = self.pos_x as i32;
        self.base.y = self.pos_y as i32;

        // 计算速度
        let a_speed = self.vel_x;
        let a_speed = if self.is_moving_at_chilled_speed() { a_speed * CHILLED_SPEED_FACTOR } else { a_speed };

        if self.is_walking_backwards() || self.zombie_phase == ZombiePhase::DancerDancingIn {
            self.pos_x += a_speed;
        } else {
            self.pos_x -= a_speed;
        }
    }

    /// 是否正在以冻结速度移动（对应 C++ IsMovingAtChilledSpeed）
    pub fn is_moving_at_chilled_speed(&self) -> bool {
        self.chilled_counter > 0
    }

    /// 是否在倒走（对应 C++ IsWalkingBackwards）
    pub fn is_walking_backwards(&self) -> bool {
        if self.mind_controlled {
            return true;
        }

        if self.zombie_height == ZombieHeight::Zombiquarium {
            if self.vel_z < 1.5707964f32 || self.vel_z > 4.712389f32 {
                return true;
            }
        }

        if self.zombie_type == ZombieType::Digger {
            if self.zombie_phase == ZombiePhase::DiggerRising
                || self.zombie_phase == ZombiePhase::DiggerStunned
                || self.zombie_phase == ZombiePhase::DiggerWalking
            {
                return true;
            } else if self.zombie_phase == ZombiePhase::Dying
                || self.zombie_phase == ZombiePhase::Burned
                || self.zombie_phase == ZombiePhase::Mowered
            {
                return self.has_object;
            }

            return false;
        }

        self.zombie_type == ZombieType::Yeti && !self.has_object
    }

    /// 僵尸是否不该走动（对应 C++ ZombieNotWalking）
    pub fn zombie_not_walking(&self) -> bool {
        if self.is_eating || self.is_immobilized() {
            return true;
        }
        if matches!(self.zombie_phase,
            ZombiePhase::JackInTheBoxPopping | ZombiePhase::NewspaperMaddening
            | ZombiePhase::GargantuarThrowing | ZombiePhase::GargantuarSmashing
            | ZombiePhase::CatapultLaunching | ZombiePhase::CatapultReloading
            | ZombiePhase::DiggerRising | ZombiePhase::DiggerTunnelingPauseWithoutAxe
            | ZombiePhase::DiggerRiseWithoutAxe | ZombiePhase::DiggerStunned
            | ZombiePhase::DancerSnappingFingers | ZombiePhase::DancerSnappingFingersWithLight
            | ZombiePhase::DancerSnappingFingersHold | ZombiePhase::DancerRising
            | ZombiePhase::ImpGettingThrown | ZombiePhase::ImpLanding
            | ZombiePhase::LadderPlacing
        ) {
            return true;
        }
        if self.zombie_height == ZombieHeight::InToChimney
            || self.zombie_height == ZombieHeight::GettingBungeeDropped
            || self.zombie_height == ZombieHeight::Zombiquarium
        {
            return true;
        }
        if matches!(self.zombie_type, ZombieType::Bungee | ZombieType::Boss) {
            return true;
        }
        if matches!(self.zombie_phase,
            ZombiePhase::DancerRaiseLeft1 | ZombiePhase::DancerWalkToRaise
            | ZombiePhase::DancerRaiseLeft2
        ) {
            return true;
        }
        false
    }

    /// 计算攻击目标的前瞻位置（对应 C++ ZombieTargetLeadX）
    pub fn zombie_target_lead_x(&self, the_time: f32) -> f32 {
        let mut a_speed = self.vel_x;
        if self.chilled_counter > 0 {
            a_speed *= CHILLED_SPEED_FACTOR;
        }
        if self.is_walking_backwards() {
            a_speed = -a_speed;
        }
        if self.zombie_not_walking() {
            a_speed = 0.0;
        }
        let a_zombie_rect = self.get_zombie_rect();
        let a_current_pos_x = (a_zombie_rect.x + a_zombie_rect.width / 2) as f32;
        let a_displacement_x = a_speed * the_time;
        a_current_pos_x - a_displacement_x
    }

    /// 判断僵尸是否已死或正在死亡（对应 C++ IsDeadOrDying）
    pub fn is_dead_or_dying(&self) -> bool {
        self.dead || self.zombie_phase == ZombiePhase::Dying 
            || self.zombie_phase == ZombiePhase::Burned 
            || self.zombie_phase == ZombiePhase::Mowered
    }

    /// 更新雪人僵尸行为（对应 C++ UpdateYeti）
    pub fn update_yeti(&mut self) {
        if self.mind_controlled || !self.has_head || self.is_dead_or_dying() {
            return;
        }
        if self.zombie_phase == ZombiePhase::Normal && self.phase_counter == 0 {
            self.zombie_phase = ZombiePhase::YetiRunning;
            self.has_object = false;
            self.pick_random_speed();
        }
    }

    /// 启动僵尸音效（对应 C++ StartZombieSound）
    pub fn start_zombie_sound(&mut self) {
        if self.playing_song {
            return;
        }
        if self.zombie_phase == ZombiePhase::JackInTheBoxRunning && self.has_head {
            if let Some(app) = self.base.app {
                unsafe { (*app).play_foley(crate::todlib::tod_foley::FoleyType::JackInTheBox as i32); }
            }
            self.playing_song = true;
        } else if self.zombie_phase == ZombiePhase::DiggerTunneling {
            if let Some(app) = self.base.app {
                unsafe { (*app).play_foley(crate::todlib::tod_foley::FoleyType::Digger as i32); }
            }
            self.playing_song = true;
        }
    }

    /// 吃植物
    /// 对应 C++ Zombie::EatPlant（Zombie.cpp 7026）
    /// 通过 base 裸指针访问 board/app，与项目内 burn_row 等既有模式一致。
    pub fn eat_plant(&mut self, plant: &mut Plant) {
        if self.zombie_phase == ZombiePhase::DancerDancingIn {
            self.phase_counter = 1;
            return;
        }

        if self.yucky_face {
            return;
        }

        // C++: mBoard->GetLadderAt(thePlant->mPlantCol, thePlant->mRow)
        let board_ptr = self.base.board;
        if let Some(board) = board_ptr {
            unsafe {
                let b = &*board;
                if b.get_ladder_at(plant.plant_col, plant.base.row).is_some() {
                    if self.zombie_type != ZombieType::Digger {
                        self.stop_eating();
                        if self.zombie_height == ZombieHeight::Normal && self.use_ladder_col != plant.plant_col {
                            self.zombie_height = ZombieHeight::UpLadder;
                            self.use_ladder_col = plant.plant_col;
                        }
                        return;
                    }
                }
            }
        }

        self.start_eating();
        if plant.seed_type == SeedType::Jalapeno ||
            plant.seed_type == SeedType::Cherrybomb ||
            plant.seed_type == SeedType::Doomshroom ||
            plant.seed_type == SeedType::Iceshroom ||
            plant.seed_type == SeedType::Hypnoshroom ||
            plant.state == PlantState::FlowerpotInvulnerable ||
            plant.state == PlantState::LilypadInvulnerable ||
            plant.state == PlantState::SquashLook ||
            plant.state == PlantState::SquashPreLaunch
        {
            if !plant.is_asleep {
                return;
            }
        }
        if plant.seed_type == SeedType::PotatoMine && plant.state != PlantState::NotReady {
            return;
        }

        let mut triggered = false;
        if plant.seed_type == SeedType::Blover {
            triggered = true;
        }
        if plant.seed_type == SeedType::Iceshroom && !plant.is_asleep {
            triggered = true;
        }
        if triggered {
            plant.do_special();
            return;
        }

        if self.chilled_counter > 0 && self.zombie_age % 2 == 1 {
            return;
        }

        let app_ptr = self.base.app;
        if let Some(app) = app_ptr {
            unsafe {
                if (*app).is_izombie_level() && plant.seed_type == SeedType::Sunflower {
                    let a_stage_before_chew = plant.plant_health / 40;
                    let a_stage_after_chew = (plant.plant_health - DAMAGE_PER_EAT) / 40;
                    // C++: if this chew lowers the plant's health by at least one stage
                    if a_stage_after_chew < a_stage_before_chew || plant.plant_health - DAMAGE_PER_EAT <= 0 {
                        if let Some(board) = board_ptr {
                            (*board).add_coin(plant.pos_x, plant.pos_y, CoinType::Sun, CoinMotion::FromPlant);
                        }
                    }
                }
            }
        }

        plant.plant_health -= DAMAGE_PER_EAT;
        plant.recently_eaten_countdown = 50;
        if let Some(app) = app_ptr {
            unsafe {
                if (*app).is_izombie_level() && self.just_got_shot_counter < -500 {
                    if plant.seed_type == SeedType::Wallnut ||
                        plant.seed_type == SeedType::Tallnut ||
                        plant.seed_type == SeedType::Pumpkinshell
                    {
                        plant.plant_health -= DAMAGE_PER_EAT;
                    }
                }
            }
        }

        if plant.plant_health <= 0 {
            if let Some(app) = app_ptr {
                unsafe {
                    (*app).play_sample(crate::framework::resources::ResourceId::SoundGulp as i32);
                }
            }
            if let Some(board) = board_ptr {
                unsafe {
                    let b = &mut *board;
                    b.m_plants_eaten += 1;
                    plant.die();
                    if let Some(challenge) = &mut b.challenge {
                        challenge.zombie_ate_plant(plant);
                    }
                    if b.level >= 2 && b.level <= 4 {
                        if let Some(app) = app_ptr {
                            if (*app).is_first_time_adventure_mode() &&
                                plant.plant_col > 4 && b.plants.len() < 15 &&
                                plant.seed_type == SeedType::Peashooter
                            {
                                b.display_advice(
                                    "[ADVICE_PEASHOOTER_DIED]",
                                    MessageStyle::HintTallFast as i32,
                                    AdviceType::PeashooterDied,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    /// 停止进食
    pub fn stop_eating(&mut self) {
        if !self.is_eating {
            return;
        }
        self.is_eating = false;

        if self.zombie_phase == ZombiePhase::DiggerTunneling {
            return;
        }

        if self.zombie_type != ZombieType::Snorkel {
            self.start_walk_anim(20);
        }

        if self.shield_type == ShieldType::Door {
            self.show_door_arms(true);
        }

        self.update_anim_speed();
    }

    /// 受伤（对应 C++ Zombie::TakeDamage）
    pub fn take_damage(&mut self, damage: i32, damage_flags: u32) {
        if self.zombie_phase == ZombiePhase::JackInTheBoxPopping || self.is_dead_or_dying() {
            return;
        }

        let mut damage_remaining = damage;

        // 先伤害飞行物（气球）
        if self.is_flying() {
            damage_remaining = self.take_flying_damage(damage_remaining, damage_flags);
        }
        // 再伤害盾牌
        if damage_remaining > 0 && self.shield_type != ShieldType::None && !test_bit(damage_flags, DAMAGE_BYPASSES_SHIELD) {
            damage_remaining = self.take_shield_damage(damage_remaining, damage_flags);
            if test_bit(damage_flags, DAMAGE_HITS_SHIELD_AND_BODY) {
                damage_remaining = damage;
            }
        }
        // 再伤害头盔
        if damage_remaining > 0 && self.helm_type != HelmType::None {
            damage_remaining = self.take_helm_damage(damage_remaining, damage_flags);
        }
        // 最后伤害身体
        if damage_remaining > 0 {
            self.take_body_damage(damage_remaining, damage_flags);
        }
    }

    /// 盾牌受伤（对应 C++ TakeShieldDamage）
    pub fn take_shield_damage(&mut self, damage: i32, damage_flags: u32) -> i32 {
        if !test_bit(damage_flags, DAMAGE_DOESNT_CAUSE_FLASH) {
            self.shield_just_got_shot_counter = 25;
            self.just_got_shot_counter = self.just_got_shot_counter.max(0);
        }

        if !test_bit(damage_flags, DAMAGE_DOESNT_CAUSE_FLASH) && !test_bit(damage_flags, DAMAGE_HITS_SHIELD_AND_BODY) {
            self.shield_recoil_counter = 12;
            if self.shield_type == ShieldType::Door || self.shield_type == ShieldType::Ladder {
                // 播放盾牌受击音效 — PlayFoley(FOLEY_SHIELD_HIT)
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::ShieldHit as i32);
                }
            }
        }

        let damage_actual = self.shield_health.min(damage);
        let damage_remaining = damage - damage_actual;
        self.shield_health -= damage_actual;
        if self.shield_health == 0 {
            self.drop_shield(damage_flags);
            return damage_remaining;
        }

        damage_remaining
    }

    /// 头盔受伤（对应 C++ TakeHelmDamage）
    pub fn take_helm_damage(&mut self, damage: i32, damage_flags: u32) -> i32 {
        if !test_bit(damage_flags, DAMAGE_DOESNT_CAUSE_FLASH) {
            self.just_got_shot_counter = 25;
        }

        let damage_actual = self.helm_health.min(damage);
        let damage_remaining = damage - damage_actual;
        self.helm_health -= damage_actual;
        if test_bit(damage_flags, DAMAGE_FREEZE) {
            self.apply_chill(false);
        }
        if self.helm_health == 0 {
            self.drop_helm(damage_flags);
            return damage_remaining;
        }

        damage_remaining
    }

    /// 飞行物受伤（对应 C++ TakeFlyingDamage）
    pub fn take_flying_damage(&mut self, damage: i32, damage_flags: u32) -> i32 {
        if !test_bit(damage_flags, DAMAGE_DOESNT_CAUSE_FLASH) {
            self.just_got_shot_counter = 25;
        }

        let damage_actual = self.flying_health.min(damage);
        let damage_remaining = damage - damage_actual;
        self.flying_health -= damage_actual;
        if self.flying_health == 0 {
            self.land_flyer(damage_flags);
        }

        damage_remaining
    }

    /// 身体受伤（对应 C++ TakeBodyDamage）
    pub fn take_body_damage(&mut self, damage: i32, damage_flags: u32) {
        if !test_bit(damage_flags, DAMAGE_DOESNT_CAUSE_FLASH) {
            self.just_got_shot_counter = 25;
        }

        if test_bit(damage_flags, DAMAGE_FREEZE) {
            self.apply_chill(false);
        }

        let body_health_origin = self.body_health;
        self.body_health -= damage;
        if self.body_health <= 0 {
            self.body_health = 0;
            self.play_death_anim(damage_flags);
            self.drop_loot();
            return;
        }

        // 特殊僵尸类型的受伤处理
        match self.zombie_type {
            ZombieType::Zamboni | ZombieType::Catapult => {
                if test_bit(damage_flags, DAMAGE_SPIKE) || self.body_health <= 0 {
                    if self.zombie_type == ZombieType::Zamboni {
                        self.zamboni_death(damage_flags);
                    } else {
                        self.catapult_death(damage_flags);
                    }
                }
            }
            _ => {
                self.update_damage_states(damage_flags);
            }
        }

        if self.body_health <= 0 {
            self.body_health = 0;
            self.play_death_anim(damage_flags);
            self.drop_loot();
        }
    }

    /// 更新受伤状态（对应 C++ UpdateDamageStates）
    pub fn update_damage_states(&mut self, damage_flags: u32) {
        if !self.can_lose_body_parts() {
            return;
        }

        if self.has_arm && self.body_health < 2 * self.body_max_health / 3 && self.body_health > 0 {
            self.drop_arm(damage_flags);
        }

        if self.has_head && self.body_health < self.body_max_health / 3 {
            self.drop_head(damage_flags);
            self.drop_loot();
            self.stop_zombie_sound();

            if self.zombie_phase == ZombiePhase::SnorkelWalkingInPool {
                self.die_no_loot();
            }
        }
    }

    /// 是否能失去身体部位（对应 C++ CanLoseBodyParts）
    pub fn can_lose_body_parts(&self) -> bool {
        self.zombie_type != ZombieType::Zamboni
            && self.zombie_type != ZombieType::Bungee
            && self.zombie_type != ZombieType::Catapult
            && self.zombie_type != ZombieType::Gargantuar
            && self.zombie_type != ZombieType::RedeEyeGargantuar
            && self.zombie_type != ZombieType::Boss
            && self.zombie_height != ZombieHeight::Zombiquarium
            && !self.is_flying()
            && !self.is_bobsled_team_with_sled()
    }

    /// 是否在飞行（对应 C++ IsFlying）
    pub fn is_flying(&self) -> bool {
        self.zombie_phase == ZombiePhase::BalloonFlying || self.zombie_phase == ZombiePhase::BalloonPopping
    }

    /// 寻找攻击目标僵尸（对应 C++ FindZombieTarget）
    pub fn find_zombie_target(&self) -> Option<*mut Zombie> {
        if self.zombie_phase == ZombiePhase::DiggerTunneling {
            return None;
        }
        let attack_rect = self.get_zombie_attack_rect();
        let board = match self.base.board { Some(b) => b, None => return None };
        unsafe {
            let b = &*board;
            for zombie in &b.zombies {
                if zombie.dead { continue; }
                if self.mind_controlled != zombie.mind_controlled
                    && !zombie.is_flying()
                    && zombie.zombie_phase != ZombiePhase::DiggerTunneling
                    && zombie.zombie_phase != ZombiePhase::BungeeDiving
                    && zombie.zombie_phase != ZombiePhase::BungeeDivingScreaming
                    && zombie.zombie_phase != ZombiePhase::BungeeRising
                    && zombie.zombie_height != ZombieHeight::GettingBungeeDropped
                    && !zombie.is_dead_or_dying()
                    && zombie.base.row == self.base.row
                {
                    let zombie_rect = zombie.get_zombie_rect();
                    let overlap = crate::lawn::board::get_rect_overlap(&attack_rect, &zombie_rect);
                    if overlap >= 20 || (overlap >= 0 && zombie.is_eating) {
                        return Some(zombie as *const _ as *mut Zombie);
                    }
                }
            }
        }
        None
    }

    /// 是否抓住猎物（对应 C++ CheckIfPreyCaught）
    pub fn check_if_prey_caught(&mut self) {
        if self.zombie_type == ZombieType::Bungee
            || self.zombie_type == ZombieType::Gargantuar
            || self.zombie_type == ZombieType::RedeEyeGargantuar
            || self.zombie_type == ZombieType::Zamboni
            || self.zombie_type == ZombieType::Catapult
            || self.zombie_type == ZombieType::Boss
            || self.is_bouncing_pogo()
            || self.is_bobsled_team_with_sled()
            || matches!(self.zombie_phase,
                ZombiePhase::PolevaulterInVault | ZombiePhase::PolevaulterPreVault
                | ZombiePhase::NewspaperMaddening | ZombiePhase::DiggerRising
                | ZombiePhase::DiggerTunnelingPauseWithoutAxe | ZombiePhase::DiggerRiseWithoutAxe
                | ZombiePhase::DiggerStunned | ZombiePhase::RisingFromGrave
                | ZombiePhase::ImpGettingThrown | ZombiePhase::ImpLanding
                | ZombiePhase::DancerRising | ZombiePhase::DancerSnappingFingers
                | ZombiePhase::DancerSnappingFingersWithLight | ZombiePhase::DancerSnappingFingersHold
                | ZombiePhase::DolphinWalking | ZombiePhase::DolphinWalkingWithoutDolphin
                | ZombiePhase::DolphinIntoPool | ZombiePhase::DolphinRiding
                | ZombiePhase::DolphinInJump | ZombiePhase::SnorkelIntoPool
                | ZombiePhase::SnorkelWalking | ZombiePhase::LadderPlacing
            )
            || self.zombie_height == ZombieHeight::GettingBungeeDropped
            || self.zombie_height == ZombieHeight::UpLadder
            || self.zombie_height == ZombieHeight::InToPool
            || self.zombie_height == ZombieHeight::OutOfPool
            || self.is_tangle_kelp_target()
            || self.zombie_height == ZombieHeight::Falling
            || !self.has_head
            || self.is_flying()
        {
            return;
        }
        let mut ticks_between_eats = DAMAGE_PER_EAT;
        if self.chilled_counter > 0 {
            ticks_between_eats *= 2;
        }
        if self.zombie_age % ticks_between_eats != 0 {
            return;
        }
        if let Some(target) = self.find_zombie_target() {
            let target = unsafe { &mut *target };
            self.eat_zombie(target);
            return;
        }
        if !self.mind_controlled {
            if self.find_plant_target_index(ZombieAttackType::Chew).is_some() {
                self.start_eating();
            }
        }
        if self.is_eating {
            self.stop_eating();
        }
    }

    /// 寻找最近的大脑（对应 C++ ZombiquariumFindClosestBrain）
    pub fn zombiquarium_find_closest_brain(&mut self) -> bool {
        let board = match self.base.board { Some(b) => b, None => return false };
        unsafe {
            let b = &*board;
            if b.has_level_award_dropped() || self.body_health > 150 {
                return false;
            }
            let mut brain_closest: Option<usize> = None;
            let mut distance_closest = 0.0f32;
            for (i, item) in b.grid_items.iter().enumerate() {
                if item.dead { continue; }
                if item.grid_item_type == crate::lawn::grid_item::GridItemType::Brain && item.counter >= 15 {
                    let dx = item.pos_x + 15.0 - (self.pos_x + 50.0);
                    let dy = item.pos_y + 15.0 - (self.pos_y + 40.0);
                    let distance = (dx * dx + dy * dy).sqrt();
                    if brain_closest.is_none() || distance < distance_closest {
                        distance_closest = distance;
                        brain_closest = Some(i);
                    }
                }
            }
            if let Some(idx) = brain_closest {
                let b = &mut *board;
                let brain = &mut b.grid_items[idx];
                if distance_closest < 50.0 {
                    brain.grid_item_die();
                    if let Some(app) = self.base.app {
                        (*app).play_foley(crate::todlib::tod_foley::FoleyType::Slurp as i32);
                    }
                    self.body_health += 200;
                    self.body_health = self.body_health.min(self.body_max_health);
                    self.play_zombie_reanim("anim_aquarium_bite", ReanimLoopType::PlayOnceAndHold, 10, 24.0);
                    self.zombie_phase = ZombiePhase::ZombiquariumBite;
                    self.phase_counter = 200;
                    return false;
                }
                let range_y = brain.pos_y + 15.0 - (self.pos_y + 40.0);
                let range_x = brain.pos_x + 15.0 - (self.pos_x + 50.0);
                self.vel_z = range_y.atan2(range_x);
                if self.vel_z < 0.0 {
                    self.vel_z += std::f32::consts::PI * 2.0;
                }
                self.zombie_phase = ZombiePhase::ZombiquariumAccel;
                return true;
            }
        }
        false
    }

    /// 更新水族馆僵尸（对应 C++ UpdateZombiquarium）
    pub fn update_zombiquarium(&mut self) {
        if self.is_dead_or_dying() {
            return;
        }
        if self.zombie_phase == ZombiePhase::ZombiquariumBite {
            let mut reanim_loop = false;
            if let Some(app) = self.base.app {
                if let Some(reanim) = unsafe { (*app).reanimation_get(self.body_reanim_id) } {
                    reanim_loop = reanim.m_loop_count > 0;
                }
            }
            if reanim_loop {
                let anim_rate = crate::todlib::tod_common::rand_range_float(8.0, 10.0);
                self.play_zombie_reanim("anim_aquarium_swim", ReanimLoopType::Loop, 20, anim_rate);
                self.zombie_phase = ZombiePhase::ZombiquariumDrift;
                self.phase_counter = 100;
            }
        } else if !self.zombiquarium_find_closest_brain() && self.phase_counter == 0 {
            let phase_hit = crate::framework::common::rand_range(7);
            if phase_hit <= 4 {
                self.zombie_phase = ZombiePhase::ZombiquariumAccel;
                self.vel_z = crate::todlib::tod_common::rand_range_float(0.0, std::f32::consts::PI * 2.0);
                self.phase_counter = crate::todlib::tod_common::rand_range_int(300, 1000);
            } else if phase_hit == 5 {
                self.zombie_phase = ZombiePhase::ZombiquariumBackAndForth;
                self.vel_z = 0.0;
                self.phase_counter = crate::todlib::tod_common::rand_range_int(300, 1000);
            } else {
                self.zombie_phase = ZombiePhase::ZombiquariumBackAndForth;
                self.vel_z = std::f32::consts::PI;
                self.phase_counter = crate::todlib::tod_common::rand_range_int(300, 1000);
            }
        }

        let vel_x = self.vel_z.cos();
        let vel_y = self.vel_z.sin();
        let mut is_out_of_bounds = false;
        if self.pos_x < 0.0 && vel_x < 0.0 {
            is_out_of_bounds = true;
        } else if self.pos_x > 680.0 && vel_x > 0.0 {
            is_out_of_bounds = true;
        } else if self.pos_y < 0.0 && vel_y < 0.0 {
            is_out_of_bounds = true;
        } else if self.pos_y > 500.0 && vel_y > 0.0 {
            is_out_of_bounds = true;
        }
        if is_out_of_bounds {
            self.vel_z += std::f32::consts::PI / 2.0;
        }

        if self.phase_counter > 0 {
            self.phase_counter -= 1;
        }
    }

    /// 该僵尸类型是否能进入泳池（对应 C++ ZombieTypeCanGoInPool）
    pub fn zombie_type_can_go_in_pool(zombie_type: ZombieType) -> bool {
        matches!(zombie_type,
            ZombieType::Normal | ZombieType::TrafficCone | ZombieType::Pail
            | ZombieType::Flag | ZombieType::Snorkel | ZombieType::DolphinRider
            | ZombieType::PeaHead | ZombieType::WallnutHead | ZombieType::JalapenoHead
            | ZombieType::GatlingHead | ZombieType::TallnutHead
        )
    }

    /// 该僵尸类型是否能上屋顶高台（对应 C++ ZombieTypeCanGoOnHighGround）
    pub fn zombie_type_can_go_on_high_ground(zombie_type: ZombieType) -> bool {
        zombie_type != ZombieType::Zamboni && zombie_type != ZombieType::Bobsled
    }

    /// 是否位于屋顶高台（对应 C++ IsOnHighGround）
    pub fn is_on_high_ground(&self) -> bool {
        if !self.is_on_board() { return false; }
        let board = match self.base.board { Some(b) => b, None => return false };
        unsafe {
            let b = &*board;
            let grid_x = b.pixel_to_grid_x_keep_on_board(self.pos_x as i32 + 75, self.pos_y as i32);
            if grid_x < 0 || grid_x as usize >= b.grid_square_type.len() { return false; }
            let row = self.base.row as usize;
            if row >= b.grid_square_type[0].len() { return false; }
            b.grid_square_type[grid_x as usize][row] == GridSquareType::HighGround
        }
    }

    /// 是否有阴影（对应 C++ HasShadow）
    pub fn has_shadow(&self) -> bool {
        if matches!(self.zombie_phase,
            ZombiePhase::Dying | ZombiePhase::DiggerRising
            | ZombiePhase::DiggerTunnelingPauseWithoutAxe | ZombiePhase::DiggerRiseWithoutAxe
            | ZombiePhase::DiggerTunneling | ZombiePhase::RisingFromGrave
            | ZombiePhase::DancerRising | ZombiePhase::BobsledBoarding
            | ZombiePhase::PolevaulterInVault | ZombiePhase::DolphinIntoPool
            | ZombiePhase::SnorkelIntoPool
        ) {
            return false;
        }
        if matches!(self.zombie_type, ZombieType::Zamboni | ZombieType::Catapult | ZombieType::Boss) {
            return false;
        }
        if self.zombie_type == ZombieType::Bungee {
            if !self.is_on_board() || self.hit_umbrella {
                return false;
            }
        }
        if self.zombie_height == ZombieHeight::DraggedUnder
            || self.zombie_height == ZombieHeight::InToChimney
            || self.zombie_height == ZombieHeight::GettingBungeeDropped
        {
            return false;
        }
        if self.in_pool {
            return false;
        }
        true
    }

    /// 获取身体伤害指数（对应 C++ GetBodyDamageIndex）
    pub fn get_body_damage_index(&self) -> i32 {
        if self.zombie_type == ZombieType::Boss {
            if self.body_health < self.body_max_health / 2 {
                return 2;
            }
            if self.body_health < self.body_max_health * 4 / 5 {
                return 1;
            }
            return 0;
        }
        if self.body_health < self.body_max_health / 3 {
            return 2;
        }
        if self.body_health < self.body_max_health * 2 / 3 {
            return 1;
        }
        0
    }

    /// 获取头盔伤害指数（对应 C++ GetHelmDamageIndex）
    pub fn get_helm_damage_index(&self) -> i32 {
        if self.helm_health < self.helm_max_health / 3 {
            return 2;
        }
        if self.helm_health < self.helm_max_health * 2 / 3 {
            return 1;
        }
        0
    }

    /// 获取盾牌伤害指数（对应 C++ GetShieldDamageIndex）
    pub fn get_shield_damage_index(&self) -> i32 {
        if self.shield_health < self.shield_max_health / 3 {
            return 2;
        }
        if self.shield_health < self.shield_max_health * 2 / 3 {
            return 1;
        }
        0
    }

    /// 掉旗帜（对应 C++ DropFlag）
    pub fn drop_flag(&mut self) {
        if self.zombie_type != ZombieType::Flag || !self.has_object {
            return;
        }
        // mApp->RemoveReanimation(mSpecialHeadReanimID) 暂未接入
        self.reanim_show_prefix("anim_innerarm", 0); // RENDER_GROUP_NORMAL
        self.reanim_show_track("Zombie_flaghand", -1); // RENDER_GROUP_HIDDEN
        self.reanim_show_track("Zombie_innerarm_screendoor", -1); // RENDER_GROUP_HIDDEN
        self.has_object = false;
    }

    /// 掉撑杆（对应 C++ DropPole）
    pub fn drop_pole(&mut self) {
        if self.zombie_type != ZombieType::Polevaulter {
            return;
        }
        self.reanim_show_prefix("Zombie_polevaulter_innerarm", -1); // RENDER_GROUP_HIDDEN
        self.reanim_show_prefix("Zombie_polevaulter_innerhand", -1); // RENDER_GROUP_HIDDEN
        self.reanim_show_prefix("Zombie_polevaulter_pole", -1); // RENDER_GROUP_HIDDEN
    }

    /// 掉头盔（对应 C++ DropHelm）
    pub fn drop_helm(&mut self, damage_flags: u32) {
        if self.helm_type == HelmType::None {
            return;
        }
        // 依赖底层系统
        self.helm_type = HelmType::None;
        let _ = damage_flags;
    }

    /// 掉盾牌（对应 C++ DropShield）
    pub fn drop_shield(&mut self, damage_flags: u32) {
        if self.shield_type == ShieldType::None {
            return;
        }
        // 依赖底层系统
        self.shield_type = ShieldType::None;
        let _ = damage_flags;
    }

    /// 掉手臂（对应 C++ DropArm）
    pub fn drop_arm(&mut self, damage_flags: u32) {
        // C++ DropArm：CanLoseBodyParts + 盾牌/海豚/报纸相位 + mHasArm 前置检查（Zombie.cpp:3878）
        if !self.can_lose_body_parts() {
            return;
        }
        if self.shield_type == ShieldType::Door || self.shield_type == ShieldType::Newspaper {
            return;
        }
        if self.zombie_phase == ZombiePhase::SnorkelIntoPool
            || self.zombie_phase == ZombiePhase::DolphinWalking
            || self.zombie_phase == ZombiePhase::DolphinIntoPool
            || self.zombie_phase == ZombiePhase::DolphinRiding
            || self.zombie_phase == ZombiePhase::DolphinInJump
            || self.zombie_phase == ZombiePhase::NewspaperReading
        {
            return;
        }
        if !self.has_arm {
            return;
        }

        self.has_arm = false;
        self.setup_reanim_for_lost_arm(damage_flags);
        // [TRANSLATION_NOTE]: C++ 末尾 PlayFoley(FOLEY_LIMBS_POP) 音效未接入
    }

    /// 掉头（对应 C++ DropHead：CanLoseBodyParts + mHasHead 前置检查，Zombie.cpp:3508）
    pub fn drop_head(&mut self, damage_flags: u32) {
        if !self.can_lose_body_parts() || !self.has_head {
            return;
        }

        if self.buttered_counter > 0 {
            self.buttered_counter = 0;
            self.update_anim_speed();
        }

        self.has_head = false;
        self.setup_reanim_for_lost_head();
        if test_bit(damage_flags, DAMAGE_DOESNT_LEAVE_BODY) {
            return;
        }
        // [TRANSLATION_NOTE]: 头部掉落粒子（PARTICLE_ZOMBIE_HEAD 等）+ FOLEY_LIMBS_POP 未接入
    }

    /// 隐藏头部 reanim 图层（对应 C++ SetupReanimForLostHead）
    pub fn setup_reanim_for_lost_head(&mut self) {
        self.reanim_show_prefix("anim_head", -1); // RENDER_GROUP_HIDDEN
        self.reanim_show_prefix("anim_hair", -1); // RENDER_GROUP_HIDDEN
        self.reanim_show_prefix("anim_tongue", -1); // RENDER_GROUP_HIDDEN
    }

    /// 隐藏手臂 reanim 图层（对应 C++ SetupReanimForLostArm）
    pub fn setup_reanim_for_lost_arm(&mut self, _damage_flags: u32) {
        match self.zombie_type {
            ZombieType::Football => {
                self.reanim_show_prefix("Zombie_football_leftarm_lower", -1);
                self.reanim_show_prefix("Zombie_football_leftarm_hand", -1);
            }
            ZombieType::Newspaper => {
                self.reanim_show_track("Zombie_paper_hands", -1);
                self.reanim_show_track("Zombie_paper_leftarm_lower", -1);
            }
            ZombieType::Polevaulter => {
                self.reanim_show_track("Zombie_polevaulter_outerarm_lower", -1);
                self.reanim_show_track("Zombie_outerarm_hand", -1);
            }
            ZombieType::Dancer => {
                self.reanim_show_track("Zombie_disco_outerarm_lower", -1);
                self.reanim_show_track("Zombie_disco_outerhand_point", -1);
            }
            ZombieType::BackupDancer => {
                self.reanim_show_track("Zombie_disco_outerarm_lower", -1);
                self.reanim_show_track("Zombie_disco_outerhand", -1);
            }
            _ => {
                self.reanim_show_prefix("Zombie_outerarm_lower", -1);
                self.reanim_show_prefix("Zombie_outerarm_hand", -1);
            }
        }
        // [TRANSLATION_NOTE]: C++ 按类型设置手臂上部图片覆盖（IMAGE_REANIM_ZOMBIE_*_UPPER2 等）
        // 并生成 PARTICLE_ZOMBIE_ARM 粒子 — 图片/粒子未接入
    }

    /// 是否有鬼脸图片（对应 C++ HasYuckyFaceImage）
    pub fn has_yucky_face_image(&self) -> bool {
        let board = match self.base.board { Some(b) => b, None => return false };
        unsafe {
            if (*board).m_future_mode {
                return false;
            }
        }
        matches!(self.zombie_type,
            ZombieType::Normal | ZombieType::TrafficCone | ZombieType::Pail |
            ZombieType::Flag | ZombieType::Door | ZombieType::DuckyTube |
            ZombieType::Dancer | ZombieType::BackupDancer | ZombieType::Newspaper |
            ZombieType::Polevaulter)
    }

    /// 显示/隐藏鬼脸（对应 C++ ShowYuckyFace）
    pub fn show_yucky_face(&mut self, the_show: bool) {
        // [TRANSLATION_NOTE]: C++ 通过 aBodyReanim->SetImageOverride("anim_head1", IMAGE_REANIM_ZOMBIE_HEAD_GROSSOUT)
        // + AssignRenderGroupToTrack 隐藏/恢复 head2/head_jaw/tongue — reanim 未接入
        let _ = the_show;
    }

    /// 精神控制（对应 C++ StartMindControlled）
    pub fn start_mind_controlled(&mut self) {
        // [TRANSLATION_NOTE]: PlaySample(SOUND_MINDCONTROLLED) 未接入
        self.mind_controlled = true;
        self.last_portal_x = -1;

        if self.zombie_type == ZombieType::Dancer {
            for i in 0..NUM_BACKUP_DANCERS {
                self.follower_zombie_ids[i] = ZOMBIEID_NULL;
            }
        } else if self.zombie_type == ZombieType::BackupDancer {
            let my_id = self.base.get_board().map_or(0, |b| b.zombie_get_id(self));
            if let Some(board) = self.base.get_board_mut() {
                if let Some(leader) = board.zombie_try_to_get_mut(self.related_zombie_id) {
                    for i in 0..NUM_BACKUP_DANCERS {
                        if leader.follower_zombie_ids[i] == my_id {
                            leader.follower_zombie_ids[i] = ZOMBIEID_NULL;
                            break;
                        }
                    }
                }
            }
            self.related_zombie_id = ZOMBIEID_NULL;
        } else {
            if let Some(board) = self.base.get_board_mut() {
                if let Some(zombie) = board.zombie_try_to_get_mut(self.related_zombie_id) {
                    zombie.related_zombie_id = ZOMBIEID_NULL;
                }
            }
            self.related_zombie_id = ZOMBIEID_NULL;
        }
    }

    /// 雪橇队整队死亡（对应 C++ BobsledDie）
    pub fn bobsled_die(&mut self) {
        if !self.is_bobsled_team_with_sled() || !self.is_on_board() {
            return;
        }

        let board = match self.base.board { Some(b) => b, None => return };
        let mut leader_id = self.related_zombie_id;
        if leader_id == ZOMBIEID_NULL {
            leader_id = unsafe { (*board).zombie_get_id(self) };
        }
        let mut follower_ids = [ZOMBIEID_NULL; MAX_ZOMBIE_FOLLOWERS];
        unsafe {
            if let Some(leader) = (*board).zombie_get(leader_id) {
                follower_ids = leader.follower_zombie_ids;
            }
        }

        if leader_id == unsafe { (*board).zombie_get_id(self) } {
            self.die_no_loot();
        } else {
            unsafe {
                if let Some(leader) = (*board).zombie_try_to_get_mut(leader_id) {
                    if !leader.dead {
                        leader.die_no_loot();
                    }
                }
            }
        }
        for &fid in follower_ids.iter() {
            if fid == ZOMBIEID_NULL { continue; }
            unsafe {
                if let Some(zombie) = (*board).zombie_try_to_get_mut(fid) {
                    if !zombie.dead {
                        zombie.die_no_loot();
                    }
                }
            }
        }
    }

    /// 雪橇队燃烧（对应 C++ BobsledBurn）
    pub fn bobsled_burn(&mut self) {
        if !self.is_bobsled_team_with_sled() {
            return;
        }

        let board = match self.base.board { Some(b) => b, None => return };
        let mut leader_id = self.related_zombie_id;
        if leader_id == ZOMBIEID_NULL {
            leader_id = unsafe { (*board).zombie_get_id(self) };
        }
        let mut follower_ids = [ZOMBIEID_NULL; MAX_ZOMBIE_FOLLOWERS];
        unsafe {
            if let Some(leader) = (*board).zombie_get(leader_id) {
                follower_ids = leader.follower_zombie_ids;
            }
        }

        if leader_id == unsafe { (*board).zombie_get_id(self) } {
            self.apply_burn();
        } else {
            unsafe {
                if let Some(leader) = (*board).zombie_try_to_get_mut(leader_id) {
                    leader.apply_burn();
                }
            }
        }
        for &fid in follower_ids.iter() {
            if fid == ZOMBIEID_NULL { continue; }
            unsafe {
                if let Some(zombie) = (*board).zombie_try_to_get_mut(fid) {
                    zombie.die_no_loot();
                }
            }
        }
    }

    /// 蹦极放下植物（对应 C++ BungeeDropPlant）
    pub fn bungee_drop_plant(&mut self) {
        if self.zombie_phase != ZombiePhase::BungeeGrabbing {
            return;
        }
        let board = match self.base.board { Some(b) => b, None => return };
        unsafe {
            let b = &mut *board;
            if let Some(plant) = b.plants.get_mut(self.target_plant_id as usize) {
                if plant.on_bungee_state == PlantOnBungeeState::GettingGrabbedByBungee {
                    plant.on_bungee_state = PlantOnBungeeState::NotOnBungee;
                } else if plant.on_bungee_state == PlantOnBungeeState::RisingWithBungee {
                    plant.die();
                }
                self.target_plant_id = PLANTID_NULL;
            }
        }
    }

    /// 蹦极死亡（对应 C++ BungeeDie）
    pub fn bungee_die(&mut self) {
        self.bungee_drop_plant();

        let board = match self.base.board { Some(b) => b, None => return };
        unsafe {
            let b = &mut *board;
            if let Some(plant) = b.plants.get_mut(self.target_plant_id as usize) {
                if !plant.dead {
                    b.m_plants_eaten += 1;
                    plant.die();
                }
            }
        }
        if let Some(board) = self.base.get_board_mut() {
            if let Some(zombie) = board.zombie_try_to_get_mut(self.related_zombie_id) {
                if !zombie.dead {
                    zombie.die_no_loot();
                }
            }
        }
    }

    /// 啃咬音效（对应 C++ AnimateChewSound）
    pub fn animate_chew_sound(&mut self) {
        if self.zombie_phase == ZombiePhase::SnorkelUpToEat {
            return;
        }

        let plant_idx = self.find_plant_target_index(ZombieAttackType::Chew);
        if let Some(idx) = plant_idx {
            let mut is_hypno = false;
            let mut is_garlic = false;
            let mut is_nut = false;
            if let Some(board) = self.base.get_board() {
                if let Some(plant) = board.plants.get(idx) {
                    if plant.seed_type == SeedType::Hypnoshroom && !plant.is_asleep {
                        is_hypno = true;
                    } else if plant.seed_type == SeedType::Garlic {
                        is_garlic = true;
                    } else if matches!(plant.seed_type, SeedType::Wallnut | SeedType::Tallnut | SeedType::Pumpkinshell) {
                        is_nut = true;
                    }
                }
            }
            if is_hypno {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Floop as i32);
                }
                if let Some(board) = self.base.get_board_mut() {
                    if let Some(plant) = board.plants.get_mut(idx) {
                        plant.die();
                    }
                }
                self.start_mind_controlled();
                // [TRANSLATION_NOTE]: PARTICLE_MIND_CONTROL 粒子未接入
                self.try_spawn_level_award();
                self.vel_x = 0.17;
                self.anim_ticks_per_frame = 18;
                self.update_anim_speed();
            } else if is_garlic {
                if !self.yucky_face {
                    self.yucky_face = true;
                    self.yucky_face_counter = 0;
                    self.update_anim_speed();
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::Chomp as i32);
                    }
                }
            } else if is_nut {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::ChompSoft as i32);
                }
            } else {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Chomp as i32);
                }
            }
        } else {
            if let Some(app) = self.base.get_app() {
                if self.mind_controlled {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::ChompSoft as i32);
                } else {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Chomp as i32);
                }
            }
        }
    }

    /// 啃咬特效（对应 C++ AnimateChewEffect）
    pub fn animate_chew_effect(&mut self) {
        if self.zombie_phase == ZombiePhase::SnorkelUpToEat {
            return;
        }

        // [TRANSLATION_NOTE]: C++ IsIZombieLevel 时 IZombieGetBrainTarget 的透明计数未接入
        let plant_idx = self.find_plant_target_index(ZombieAttackType::Chew);
        if let Some(idx) = plant_idx {
            if let Some(board) = self.base.get_board_mut() {
                if let Some(plant) = board.plants.get_mut(idx) {
                    // [TRANSLATION_NOTE]: Wallnut/Tallnut 咀嚼粒子（PARTICLE_WALLNUT_EAT_SMALL）未接入
                    plant.eaten_flash_countdown = plant.eaten_flash_countdown.max(25);
                }
            }
        }
    }

    /// 豌豆头射击（对应 C++ UpdateZombiePeaHead，Zombie.cpp 2316-2358）
    pub fn update_zombie_pea_head(&mut self) {
        if !self.has_head {
            return;
        }

        if self.phase_counter == 35 {
            // C++: aHeadReanim->PlayReanim("anim_shooting", PLAY_ONCE_AND_HOLD, 20, 35.0f)
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_head_reanim) = app.reanimation_get_mut(self.special_head_reanim_id) {
                    a_head_reanim.play_reanim("anim_shooting", ReanimLoopType::PlayOnceAndHold, 20, 35.0);
                }
            }
        } else if self.phase_counter == 0 {
            // C++: aHeadReanim->PlayReanim("anim_head_idle", PLAY_ONCE_AND_HOLD, 20, 15.0f)
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_head_reanim) = app.reanimation_get_mut(self.special_head_reanim_id) {
                    a_head_reanim.play_reanim("anim_head_idle", ReanimLoopType::PlayOnceAndHold, 20, 15.0);
                }
                app.play_foley(crate::todlib::tod_foley::FoleyType::Throw as i32);
            }

            // C++: 从身体 reanim 的 anim_head1 变换读取发射原点
            let mut a_origin_x = self.pos_x - 9.0;
            let mut a_origin_y = self.pos_y + 6.0 - self.altitude;
            if let Some(app) = self.base.get_app() {
                if let Some(a_body_reanim) = app.reanimation_get(self.body_reanim_id) {
                    let a_track_index = a_body_reanim.find_track_index("anim_head1");
                    let mut a_transform = crate::todlib::definition::ReanimatorTransform {
                        m_trans_x: 0.0,
                        m_trans_y: 0.0,
                        m_skew_x: 0.0,
                        m_skew_y: 0.0,
                        m_scale_x: 1.0,
                        m_scale_y: 1.0,
                        m_alpha: 1.0,
                        m_frame: 0.0,
                        m_image: 0,
                        m_visible: true,
                        m_font: 0,
                        m_text: 0,
                        m_color: Color::WHITE,
                        m_extra_int: 0,
                        m_extra_float: 0.0,
                    };
                    if a_body_reanim.get_current_transform(a_track_index, &mut a_transform) {
                        a_origin_x = self.pos_x + a_transform.m_trans_x - 9.0;
                        a_origin_y = self.pos_y + a_transform.m_trans_y + 6.0 - self.altitude;
                    }
                }
            }

            // C++: AddProjectile(PROJECTILE_ZOMBIE_PEA) + mMotionType = MOTION_BACKWARDS
            let a_row = self.base.row;
            if let Some(board) = self.base.get_board_mut() {
                let idx = board.add_projectile(a_origin_x, a_origin_y, a_row, SeedType::Peashooter);
                board.projectiles[idx].projectile_type = crate::lawn::projectile::ProjectileType::ZombiePea;
                board.projectiles[idx].motion = crate::lawn::projectile::ProjectileMotion::Backwards;
            }
            // [TRANSLATION_NOTE]: C++ DO_FIX_BUGS 分支（mMindControlled 时改发友方豌豆）未翻译
            self.phase_counter = 150;
        }
    }

    /// 辣椒头爆炸（对应 C++ UpdateZombieJalapenoHead）
    pub fn update_zombie_jalapeno_head(&mut self) {
        if !self.has_head {
            return;
        }

        if self.phase_counter == 0 {
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::JalapenoIgnite as i32);
                app.play_foley(crate::todlib::tod_foley::FoleyType::Juicy as i32);
            }
            let row = self.base.row;
            if let Some(board) = self.base.get_board_mut() {
                board.do_fwoosh(row);
                board.shake_board(3, -4);
            }
            if let Some(board) = self.base.get_board_mut() {
                let mut to_kill: Vec<usize> = Vec::new();
                for (i, plant) in board.plants.iter().enumerate() {
                    if plant.dead { continue; }
                    if plant.base.row == row && !plant.not_on_ground() {
                        to_kill.push(i);
                    }
                }
                for idx in to_kill {
                    board.m_plants_eaten += 1;
                    board.plants[idx].die();
                }
            }
            self.die_no_loot();
        }
    }

    /// 加特林头射击（对应 C++ UpdateZombieGatlingHead）
    pub fn update_zombie_gatling_head(&mut self) {
        if !self.has_head {
            return;
        }

        if self.phase_counter == 100 {
            // C++: aHeadReanim = mApp->ReanimationGet(mSpecialHeadReanimID);
            //      aHeadReanim->PlayReanim("anim_shooting", REANIM_PLAY_ONCE_AND_HOLD, 20, 38.0f)
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_head_reanim) = app.reanimation_get_mut(self.special_head_reanim_id) {
                    a_head_reanim.play_reanim("anim_shooting", ReanimLoopType::PlayOnceAndHold, 20, 38.0);
                }
            }
        } else if matches!(self.phase_counter, 18 | 35 | 51 | 68) {
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Throw as i32);
            }
            let a_origin_x = self.pos_x - 9.0;
            let a_origin_y = self.pos_y + 6.0;
            let a_row = self.base.row;
            if let Some(board) = self.base.get_board_mut() {
                let idx = board.add_projectile(a_origin_x, a_origin_y, a_row, SeedType::Peashooter);
                board.projectiles[idx].projectile_type = crate::lawn::projectile::ProjectileType::ZombiePea;
                // [TRANSLATION_NOTE]: C++ MOTION_BACKWARDS（ZombiePea 向左飞行）未在 Rust 枚举中
            }
        } else if self.phase_counter == 0 {
            // C++: aHeadReanim->PlayReanim("anim_idle", REANIM_LOOP, 20, 12.0f)
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_head_reanim) = app.reanimation_get_mut(self.special_head_reanim_id) {
                    a_head_reanim.play_reanim("anim_idle", ReanimLoopType::Loop, 20, 12.0);
                }
            }
            self.phase_counter = 150;
        }
    }

    /// 倭瓜头跳起攻击（对应 C++ UpdateZombieSquashHead）
    pub fn update_zombie_squash_head(&mut self) {
        if self.has_head && self.is_eating && self.zombie_phase == ZombiePhase::SquashPreLaunch {
            self.stop_eating();
            self.play_zombie_reanim("anim_idle", ReanimLoopType::Loop, 20, 12.0);
            self.has_head = false;
            // C++: aHeadReanim->PlayReanim("anim_jumpup", REANIM_PLAY_ONCE_AND_HOLD, 20, 24.0f);
            //      mRenderOrder = mRenderOrder + 1; SetPosition(mPosX + 6, mPosY - 21);
            //      OverrideScale(0.75, 0.75); mOverlayMatrix.m10 = 0
            //（AttachmentDetach(aTrackInstance->mAttachmentID) 依赖附着系统未接入）
            let a_render_order = self.base.render_order + 1;
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_head_reanim) = app.reanimation_get_mut(self.special_head_reanim_id) {
                    a_head_reanim.play_reanim("anim_jumpup", ReanimLoopType::PlayOnceAndHold, 20, 24.0);
                    a_head_reanim.m_render_order = a_render_order;
                    a_head_reanim.set_position(self.pos_x + 6.0, self.pos_y - 21.0);
                    a_head_reanim.override_scale(0.75, 0.75);
                    a_head_reanim.m_overlay_matrix.m[1][0] = 0.0;
                }
            }
            self.zombie_phase = ZombiePhase::SquashRising;
            self.phase_counter = 95;
        }

        if self.zombie_phase == ZombiePhase::SquashRising {
            let board = match self.base.board { Some(b) => b, None => return };
            let a_dest_x = unsafe { (*board).grid_to_pixel_x((*board).pixel_to_grid_x_keep_on_board(self.base.x, self.base.y), self.base.row) };
            // C++: aPosX = PvzpAnimateCurve(50, 20, mPhaseCounter, 0, aDestX - mPosX, CURVE_EASE_IN_OUT);
            //      aPosY = PvzpAnimateCurve(50, 20, mPhaseCounter, 0, -20, CURVE_EASE_IN_OUT);
            //      aHeadReanim->SetPosition(mPosX + aPosX + 6, mPosY + aPosY - 21)
            let a_pos_x = crate::todlib::tod_common::tod_animate_curve(
                50, 20, self.phase_counter, 0, a_dest_x - self.pos_x as i32, TodCurves::EaseInOut,
            );
            let a_pos_y = crate::todlib::tod_common::tod_animate_curve(
                50, 20, self.phase_counter, 0, -20, TodCurves::EaseInOut,
            );
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_head_reanim) = app.reanimation_get_mut(self.special_head_reanim_id) {
                    a_head_reanim.set_position(self.pos_x + a_pos_x as f32 + 6.0, self.pos_y + a_pos_y as f32 - 21.0);
                }
            }
            if self.phase_counter == 0 {
                // C++: aHeadReanim->PlayReanim("anim_jumpdown", REANIM_PLAY_ONCE_AND_HOLD, 0, 60.0f)
                if let Some(app) = self.base.get_app_mut() {
                    if let Some(a_head_reanim) = app.reanimation_get_mut(self.special_head_reanim_id) {
                        a_head_reanim.play_reanim("anim_jumpdown", ReanimLoopType::PlayOnceAndHold, 0, 60.0);
                    }
                }
                self.zombie_phase = ZombiePhase::SquashFalling;
                self.phase_counter = 10;
            }
        }

        if self.zombie_phase == ZombiePhase::SquashFalling {
            // C++: aPosY = PvzpAnimateCurve(10, 0, mPhaseCounter, -20, 74, CURVE_LINEAR);
            //      aHeadReanim->SetPosition(mPosX + 6 + aDestX - mPosX, mPosY - 21 + aPosY)
            let board = match self.base.board { Some(b) => b, None => return };
            let a_dest_x = unsafe { (*board).grid_to_pixel_x((*board).pixel_to_grid_x_keep_on_board(self.base.x, self.base.y), self.base.row) };
            let a_pos_y = crate::todlib::tod_common::tod_animate_curve(
                10, 0, self.phase_counter, -20, 74, TodCurves::Linear,
            );
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_head_reanim) = app.reanimation_get_mut(self.special_head_reanim_id) {
                    a_head_reanim.set_position(self.pos_x + 6.0 + a_dest_x as f32 - self.pos_x, self.pos_y - 21.0 + a_pos_y as f32);
                }
            }
            // 对应 C++ Zombie.cpp:2584/2587: SquishAllInSquare(PixelToGridXKeepOnBoard(mX, mY), mRow, ATTACKTYPE_CHEW)
            if self.phase_counter == 2 {
                let a_grid_x = unsafe { (*board).pixel_to_grid_x_keep_on_board(self.base.x, self.base.y) };
                let a_row = self.base.row;
                self.squish_all_in_square(a_grid_x, a_row, ZombieAttackType::Chew);
            }
            if self.phase_counter == 0 {
                self.zombie_phase = ZombiePhase::SquashDoneFalling;
                self.phase_counter = 100;
                if let Some(board) = self.base.get_board_mut() {
                    board.shake_board(1, 4);
                }
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Thump as i32);
                }
            }
        }

        if self.zombie_phase == ZombiePhase::SquashDoneFalling && self.phase_counter == 0 {
            // C++: aHeadReanim->ReanimationDie(); mSpecialHeadReanimID = REANIMATIONID_NULL;
            //      TakeDamage(1800, 9U)
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_head_reanim) = app.reanimation_get_mut(self.special_head_reanim_id) {
                    a_head_reanim.reanimation_die();
                }
            }
            self.special_head_reanim_id = REANIMATIONID_NULL;
            self.take_damage(1800, 9);
        }
    }

    /// 附加 reanim（对应 C++ AddAttachedReanim，Zombie.cpp）
    pub fn add_attached_reanim(&mut self, the_pos_x: i32, the_pos_y: i32, the_reanim_type: ReanimationType) -> Option<*mut Reanimation> {
        if self.dead {
            return None;
        }
        // C++: aReanim = mApp->AddReanimation(mX + thePosX, mY + thePosY, 0, theReanimType)
        let app = self.base.get_app_mut()?;
        let a_reanim = app.add_reanimation(
            self.pos_x + the_pos_x as f32,
            self.pos_y + the_pos_y as f32,
            0,
            the_reanim_type as i32,
        )?;
        // C++: if (aReanim) AttachReanim(mAttachmentID, aReanim, thePosX, thePosY)
        crate::todlib::attachment::attach_reanim(
            &mut self.attachment_id,
            a_reanim as *mut std::ffi::c_void,
            the_pos_x as f32,
            the_pos_y as f32,
        );
        Some(a_reanim)
    }

    /// 气球螺旋桨旋转（对应 C++ BalloonPropellerHatSpin）
    pub fn balloon_propeller_hat_spin(&mut self, the_spinning: bool) {
        // [TRANSLATION_NOTE]: C++ 用 ReanimationGet（强制存在）+ GetTrackInstanceByName("hat") 后直接
        // FindReanimAttachment(mAttachmentID)；此处以 Option 安全化。find_reanim_attachment 为附着接入层（当前 stub 恒 None）
        let mut a_hat_attachment_id: AttachmentID = 0;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                if let Some(a_hat_track_instance) = a_body_reanim.get_track_instance_by_name("hat") {
                    a_hat_attachment_id = a_hat_track_instance.m_attachment_id;
                }
            }
        }
        if let Some(a_propeller_reanim) = find_reanim_attachment(&mut a_hat_attachment_id) {
            let a_propeller_reanim = a_propeller_reanim as *mut Reanimation;
            if the_spinning {
                unsafe {
                    // C++: aPropellerReanim->mAnimRate = aPropellerReanim->mDefinition->mFPS;
                    if let Some(a_definition) = (*a_propeller_reanim).m_definition {
                        (*a_propeller_reanim).m_anim_rate = (*a_definition).m_fps;
                    }
                }
            } else {
                unsafe {
                    (*a_propeller_reanim).m_anim_rate = 0.0;
                }
            }
        }
    }

    /// 覆盖粒子缩放（对应 C++ OverrideParticleScale）
    pub fn override_particle_scale(&mut self, a_particle: *mut ParticleSystem) {
        if !a_particle.is_null() {
            // [TRANSLATION_NOTE]: C++ 传 nullptr 表示作用于全部 emitter，Rust 接口以空串占位
            unsafe {
                (*a_particle).override_scale("", self.scale_zombie);
            }
        }
    }

    /// 覆盖粒子颜色（对应 C++ OverrideParticleColor）
    pub fn override_particle_color(&mut self, a_particle: *mut ParticleSystem) {
        if !a_particle.is_null() {
            if self.mind_controlled {
                unsafe {
                    (*a_particle).override_color("", &ZOMBIE_MINDCONTROLLED_COLOR);
                    (*a_particle).override_extra_additive_draw("", true);
                }
            } else if self.chilled_counter > 0 || self.ice_trap_counter > 0 {
                unsafe {
                    (*a_particle).override_color("", &Color::new(75, 75, 255, 255));
                    (*a_particle).override_extra_additive_draw("", true);
                }
            }
        }
    }

    /// 应用 Zombatar 头部（对应 C++ ApplyZombatarHead）
    pub fn apply_zombatar_head(&mut self, the_record: &[u8]) {
        // 渲染组常量：RENDER_GROUP_ZOMBATAR_HEAD=3（Zombie.cpp），RENDER_GROUP_HIDDEN=-1（PvzpLib/Reanimator.h）
        let render_group_zombatar_head: i32 = 3;
        let render_group_hidden: i32 = -1;

        // C++ 全局 IMAGE_BLANK（extern Image*）→ 从资源管理器取图
        let a_image_blank: *mut Image = {
            let mut a_ptr: *mut Image = std::ptr::null_mut();
            if let Some(app) = self.base.get_app() {
                if let Some(a_rm_ptr) = app.base.resource_manager {
                    unsafe {
                        let a_rm = &*a_rm_ptr;
                        let a_shared = a_rm.get_image("IMAGE_BLANK");
                        let a_img_ptr = a_shared.as_image_ptr();
                        if !a_img_ptr.is_null() {
                            a_ptr = a_img_ptr;
                        }
                    }
                }
            }
            a_ptr
        };

        // [TRANSLATION_NOTE]: C++ GetTrackInstanceByName("anim_head1")/AttachReanim 假定对象存在；Rust 侧 Option 安全化，
        // mAttachmentID 为值类型提前复制，避免跨操作借用冲突
        let mut a_track_attachment_id: AttachmentID = 0;
        if let Some(app) = self.base.get_app_mut() {
            // C++: aBodyReanim = mApp->ReanimationTryToGet(mBodyReanimID); if (!aBodyReanim) return;
            if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                // C++: aTrackInstance->mImageOverride = IMAGE_BLANK;
                if let Some(a_track_instance) = a_body_reanim.get_track_instance_by_name("anim_head1") {
                    a_track_instance.m_image_override = a_image_blank;
                    a_track_attachment_id = a_track_instance.m_attachment_id;
                }
                a_body_reanim.assign_render_group_to_track("anim_head1", render_group_zombatar_head);
                a_body_reanim.assign_render_group_to_prefix("anim_head2", render_group_hidden);
                a_body_reanim.assign_render_group_to_prefix("anim_hair", render_group_hidden);
                a_body_reanim.m_frame_base_pose = 0;
            }
        }

        // C++: aHeadReanim = mApp->ReanimationTryToGet(mZombatarHeadReanimID); if (!aHeadReanim) { 新建 }
        let a_head_exists = self
            .base
            .get_app()
            .and_then(|app| app.reanimation_get(self.zombatar_head_reanim_id))
            .is_some();
        if !a_head_exists {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_head_ptr) = app.add_reanimation(0.0, 0.0, 0, ReanimationType::ZombatarHead as i32) {
                    unsafe {
                        (*a_head_ptr).play_reanim("anim_head_idle", ReanimLoopType::Loop, 0, 15.0);
                    }
                    self.zombatar_head_reanim_id = app.reanimation_get_id(a_head_ptr);
                    // C++: AttachEffect* aAttachEffect = AttachReanim(aTrackInstance->mAttachmentID, aHeadReanim, 0.0f, 0.0f);
                    //       PvzpScaleRotateTransformMatrix(aAttachEffect->mOffset, -20.0f, -1.0f, 0.2f, 1.0f, 1.0f);
                    if let Some(a_attach_effect) = attach_reanim(&mut a_track_attachment_id, a_head_ptr as *mut std::ffi::c_void, 0.0, 0.0) {
                        unsafe {
                            pvzp_scale_rotate_transform_matrix(&mut (*a_attach_effect).offset, -20.0, -1.0, 0.2, 1.0, 1.0);
                        }
                    }
                }
            }
        }

        // 对头部 reanim 统一设置渲染组/前缀隐藏 + 按记录装配部件轨道（C++ ApplyZombatarHead 后半段）
        if let Some(app) = self.base.get_app_mut() {
            if let Some(a_head_reanim) = app.reanimation_get_mut(self.zombatar_head_reanim_id) {
                a_head_reanim.assign_render_group_to_track("anim_hair", render_group_hidden);
                a_head_reanim.assign_render_group_to_prefix("hats_", render_group_hidden);
                a_head_reanim.assign_render_group_to_prefix("hair_", render_group_hidden);
                a_head_reanim.assign_render_group_to_prefix("facialHair_", render_group_hidden);
                a_head_reanim.assign_render_group_to_prefix("accessories_", render_group_hidden);
                a_head_reanim.assign_render_group_to_prefix("eyeWear_", render_group_hidden);
                a_head_reanim.assign_render_group_to_prefix("tidBits_", render_group_hidden);

                // C++ RuntimePart 表（static constexpr aRuntimeParts[]）
                struct RuntimePart {
                    m_part_slot: i32,
                    m_color_slot: i32,
                    m_max_count: i32,
                    m_prefix: &'static str,
                    m_page: ZombatarPage,
                    m_remap_accessory: bool,
                    m_compact_track_range: bool,
                }
                const A_RUNTIME_PARTS: [RuntimePart; 6] = [
                    RuntimePart { m_part_slot: ZOMBATAR_SLOT_HATS, m_color_slot: ZOMBATAR_SLOT_HATS_COLOR, m_max_count: 14, m_prefix: "hats_", m_page: ZombatarPage::Hats, m_remap_accessory: false, m_compact_track_range: false },
                    RuntimePart { m_part_slot: ZOMBATAR_SLOT_HAIR, m_color_slot: ZOMBATAR_SLOT_HAIR_COLOR, m_max_count: 16, m_prefix: "hair_", m_page: ZombatarPage::Hair, m_remap_accessory: false, m_compact_track_range: false },
                    RuntimePart { m_part_slot: ZOMBATAR_SLOT_TIDBITS, m_color_slot: ZOMBATAR_SLOT_TIDBITS_COLOR, m_max_count: 14, m_prefix: "tidBits_", m_page: ZombatarPage::Tidbits, m_remap_accessory: false, m_compact_track_range: false },
                    RuntimePart { m_part_slot: ZOMBATAR_SLOT_EYEWEAR, m_color_slot: ZOMBATAR_SLOT_EYEWEAR_COLOR, m_max_count: 16, m_prefix: "eyeWear_", m_page: ZombatarPage::Eyewear, m_remap_accessory: false, m_compact_track_range: false },
                    RuntimePart { m_part_slot: ZOMBATAR_SLOT_ACCESSORY, m_color_slot: ZOMBATAR_SLOT_ACCESSORY_COLOR, m_max_count: 15, m_prefix: "accessories_", m_page: ZombatarPage::Accessory, m_remap_accessory: true, m_compact_track_range: false },
                    RuntimePart { m_part_slot: ZOMBATAR_SLOT_FACIAL_HAIR, m_color_slot: ZOMBATAR_SLOT_FACIAL_HAIR_COLOR, m_max_count: 25, m_prefix: "facialHair_", m_page: ZombatarPage::FacialHair, m_remap_accessory: false, m_compact_track_range: true },
                ];

                for a_part in A_RUNTIME_PARTS.iter() {
                    // C++: int aPartIndex = ZombatarReadSignedRecordSlot(theRecord, aPart.mPartSlot);
                    let a_part_index = zombatar_read_signed_record_slot(the_record, a_part.m_part_slot);
                    if a_part_index < 0 || a_part_index >= a_part.m_max_count {
                        continue;
                    }
                    let mut a_track_index = a_part_index;
                    if a_part.m_compact_track_range && a_track_index > 16 {
                        a_track_index -= a_track_index / 17;
                    }
                    if a_part.m_remap_accessory {
                        a_track_index = zombatar_remap_accessory_for_runtime(a_track_index);
                    }
                    let a_track_name = zombatar_track_name(a_part.m_prefix, a_track_index);

                    // C++: const int aDrawOrder = aLayout ? aLayout->mDrawOrder : 0;
                    let a_layout = get_part_layout(a_part.m_page, a_part_index);
                    let a_draw_order = match a_layout {
                        Some(a_layout) => a_layout.m_draw_order,
                        None => 0,
                    };
                    if a_head_reanim.track_exists(&a_track_name) {
                        a_head_reanim.assign_render_group_to_track(&a_track_name, a_draw_order);
                        if let Some(a_track_instance) = a_head_reanim.get_track_instance_by_name(&a_track_name) {
                            a_track_instance.m_track_color = zombatar_get_color(zombatar_read_signed_record_slot(the_record, a_part.m_color_slot));
                        }
                    }

                    // C++: some parts exist only as a "_line" detail track without a base track
                    let a_line_track_name = a_track_name.clone() + "_line";
                    if a_head_reanim.track_exists(&a_line_track_name) {
                        a_head_reanim.assign_render_group_to_track(&a_line_track_name, a_draw_order + 1);
                    }
                }
            }
        }
    }

    /// 绘制僵尸部位（对应 C++ DrawZombiePart；C++ 注释"normally never called"）
    pub fn draw_zombie_part(&self, g: &mut Graphics, image: *mut Image, frame: i32, row: i32, draw_pos: &ZombieDrawPosition) {
        // 裸指针图片访问（对应 C++ Image*）
        let a_cel_width;
        let a_cel_height;
        unsafe {
            let a_image = &*image;
            a_cel_width = a_image.get_cel_width();
            a_cel_height = a_image.get_cel_height();
        }
        let mut an_offset_x = draw_pos.image_offset_x;
        let mut an_offset_y = draw_pos.image_offset_y + draw_pos.body_y;
        if self.zombie_phase == ZombiePhase::PolevaulterInVault {
            an_offset_x -= 120.0;
            an_offset_y -= 120.0;
        }
        if self.zombie_phase == ZombiePhase::DiggerTunneling {
            an_offset_y += 50.0;
        }
        if self.zombie_type == ZombieType::Zamboni {
            an_offset_y -= 19.0;
        }

        let mut a_draw_height = a_cel_height as f32;
        if draw_pos.clip_height > CLIP_HEIGHT_LIMIT {
            a_draw_height = (a_cel_height as f32 - draw_pos.clip_height).clamp(0.0, a_cel_height as f32);
        }

        let mut an_alpha = 255;
        if self.zombie_fade >= 0 {
            an_alpha = (255 * self.zombie_fade / 10).clamp(0, 255);
            g.set_colorize_images(true);
            g.set_color(&Color::new(255, 255, 255, an_alpha as u8));
        }

        let mut a_mirror = false;
        if self.zombie_phase == ZombiePhase::DancerDancingIn || self.zombie_phase == ZombiePhase::DancerDancingLeft {
            let a_frame = self.get_dancer_frame();
            if !self.is_eating && (a_frame == 12 || a_frame == 13 || a_frame == 14 || a_frame == 18 || a_frame == 19 || a_frame == 20) {
                a_mirror = true;
                an_offset_x -= 30.0;
            }
        }
        if a_mirror {
            an_offset_x = -an_offset_x;
        }

        let a_src_rect = Rect::new(frame * a_cel_width, row * a_cel_height, a_cel_width, a_draw_height as i32);
        let a_dest_rect = Rect::new(an_offset_x as i32, an_offset_y as i32, a_cel_width, a_draw_height as i32);
        if self.zombie_phase == ZombiePhase::Burned {
            if self.mind_controlled {
                a_mirror = true;
            }

            g.set_colorize_images(true);
            g.set_color(&Color::BLACK);
            unsafe {
                g.draw_image_mirror_stretch(&*image, &a_dest_rect, &a_src_rect, a_mirror);
            }
        } else if self.mind_controlled {
            a_mirror = true;
            g.set_colorize_images(true);
            let mut a_mincontrolled_color = ZOMBIE_MINDCONTROLLED_COLOR;
            a_mincontrolled_color.a = an_alpha as u8;
            g.set_color(&a_mincontrolled_color);
            unsafe {
                g.draw_image_mirror_stretch(&*image, &a_dest_rect, &a_src_rect, a_mirror);
            }

            g.set_draw_mode(crate::framework::graphics::graphics::DrawMode::Additive as i32);
            unsafe {
                g.draw_image_mirror_stretch(&*image, &a_dest_rect, &a_src_rect, a_mirror);
            }
            g.set_draw_mode(crate::framework::graphics::graphics::DrawMode::Normal as i32);
        } else if self.chilled_counter > 0 || self.ice_trap_counter > 0 {
            g.set_colorize_images(true);
            g.set_color(&Color::new(75, 75, 255, an_alpha as u8));
            unsafe {
                g.draw_image_mirror_stretch(&*image, &a_dest_rect, &a_src_rect, a_mirror);
            }

            g.set_draw_mode(crate::framework::graphics::graphics::DrawMode::Additive as i32);
            unsafe {
                g.draw_image_mirror_stretch(&*image, &a_dest_rect, &a_src_rect, a_mirror);
            }
            g.set_draw_mode(crate::framework::graphics::graphics::DrawMode::Normal as i32);
        } else {
            unsafe {
                g.draw_image_mirror_stretch(&*image, &a_dest_rect, &a_src_rect, a_mirror);
            }
        }

        if self.just_got_shot_counter > 0 {
            g.set_draw_mode(crate::framework::graphics::graphics::DrawMode::Additive as i32);
            g.set_colorize_images(true);
            let a_grayness = self.just_got_shot_counter * 10;
            g.set_color(&Color::new(a_grayness as u8, a_grayness as u8, a_grayness as u8, 255));
            unsafe {
                g.draw_image_mirror_stretch(&*image, &a_dest_rect, &a_src_rect, a_mirror);
            }
            g.set_draw_mode(crate::framework::graphics::graphics::DrawMode::Normal as i32);
        }

        g.set_colorize_images(false);
    }

    /// 绘制僵尸头部（对应 C++ DrawZombieHead，C++ 中已注释为死代码）
    pub fn draw_zombie_head(&self, _g: &mut Graphics, _draw_pos: &crate::lawn::zombie::ZombieDrawPosition, _frame: i32) {
        // [TRANSLATION_NOTE]: C++ 中该函数已被整段注释（死代码），保留签名占位
    }

    /// 分部位绘制僵尸（对应 C++ DrawZombieWithParts，C++ 中已注释为死代码）
    pub fn draw_zombie_with_parts(&self, _g: &mut Graphics, _draw_pos: &crate::lawn::zombie::ZombieDrawPosition) {
        // [TRANSLATION_NOTE]: C++ 中该函数已被整段注释（死代码），保留签名占位
    }

    /// 气球僵尸落地（对应 C++ LandFlyer）
    pub fn land_flyer(&mut self, damage_flags: u32) {
        if !test_bit(damage_flags, DAMAGE_DOESNT_LEAVE_BODY) && self.zombie_phase == ZombiePhase::BalloonFlying {
            // 依赖底层系统
            self.zombie_phase = ZombiePhase::BalloonPopping;
            self.play_zombie_reanim("anim_pop", ReanimLoopType::PlayOnceAndHold, 20, 24.0);
        }

        if let Some(board) = self.base.get_board() {
            if board.m_plant_row[self.base.row as usize] == PlantRowType::Pool {
                self.die_with_loot();
                return;
            }
        }
        self.zombie_height = ZombieHeight::Falling;
    }

    /// 播放死亡动画（对应 C++ PlayDeathAnim）
    pub fn play_death_anim(&mut self, damage_flags: u32) {
        if self.zombie_phase == ZombiePhase::Dying || self.zombie_phase == ZombiePhase::Burned || self.zombie_phase == ZombiePhase::Mowered {
            return;
        }

        // 冰陷阱/黄油/恶心表情清理
        // 依赖底层系统

        self.stop_eating();
        if self.shield_type != ShieldType::None {
            self.drop_shield(1);
        }

        self.vel_x = 0.0;
        self.zombie_phase = ZombiePhase::Dying;

        // 不同僵尸类型的死亡动画速率
        let a_death_anim_rate = match self.zombie_type {
            ZombieType::Football => 24.0,
            ZombieType::Gargantuar | ZombieType::RedeEyeGargantuar => 14.0,
            ZombieType::Snorkel => 14.0,
            ZombieType::Digger => 18.0,
            ZombieType::Yeti => 14.0,
            ZombieType::Boss => 18.0,
            _ => 24.0 + RandFloat(6.0),
        };

        // 选择死亡动画轨道
        let mut a_death_track = "anim_death";
        // 依赖底层系统
        let _ = damage_flags;

        self.play_zombie_reanim(a_death_track, ReanimLoopType::PlayOnceAndHold, 20, a_death_anim_rate);
    }

    /// 更新死亡状态（对应 C++ UpdateDeath，Zombie.cpp:9068）
    pub fn update_death(&mut self) {
        // 对应 C++: aBodyReanim 取不到 → DieNoLoot
        let a_body_null = self.base.get_app().map_or(true, |app| {
            app.reanimation_get(self.body_reanim_id).is_none()
        });
        if a_body_null {
            self.die_no_loot();
            return;
        }

        // 对应 C++: 下落高度 → UpdateZombieFalling
        if self.zombie_height == crate::lawn::game_enums::ZombieHeight::Falling {
            self.update_zombie_falling();
        }

        // 对应 C++: 伽刚特尔倒地震屏（0.89 / 0.98 时刻）
        if self.zombie_type == ZombieType::Gargantuar || self.zombie_type == ZombieType::RedeEyeGargantuar {
            let a_trigger_89 = self.base.get_app().map_or(false, |app| {
                app.reanimation_get(self.body_reanim_id)
                    .map_or(false, |r| r.should_trigger_timed_event(0.89))
            });
            let a_trigger_98 = self.base.get_app().map_or(false, |app| {
                app.reanimation_get(self.body_reanim_id)
                    .map_or(false, |r| r.should_trigger_timed_event(0.98))
            });
            if a_trigger_89 {
                if let Some(board) = self.base.get_board_mut() { board.shake_board(0, 3); }
            } else if a_trigger_98 {
                if let Some(board) = self.base.get_board_mut() { board.shake_board(0, 1); }
            }
        }

        // 对应 C++: 非泳池时按类型选择倒地时刻 aFallTime
        if !self.in_pool {
            let mut a_fall_time = -1.0f32;
            match self.zombie_type {
                ZombieType::Snorkel | ZombieType::Zamboni | ZombieType::DolphinRider
                | ZombieType::Bungee | ZombieType::Catapult | ZombieType::Imp
                | ZombieType::Boss => { a_fall_time = -1.0; }
                ZombieType::Normal | ZombieType::Flag | ZombieType::TrafficCone
                | ZombieType::Pail | ZombieType::Door | ZombieType::PeaHead
                | ZombieType::WallnutHead | ZombieType::TallnutHead
                | ZombieType::JalapenoHead | ZombieType::GatlingHead
                | ZombieType::SquashHead | ZombieType::DuckyTube => {
                    let a_superlong = self.base.get_app().map_or(false, |app| {
                        app.reanimation_get(self.body_reanim_id)
                            .map_or(false, |r| r.is_anim_playing("anim_superlongdeath"))
                    });
                    let a_death2 = self.base.get_app().map_or(false, |app| {
                        app.reanimation_get(self.body_reanim_id)
                            .map_or(false, |r| r.is_anim_playing("anim_death2"))
                    });
                    if a_superlong {
                        a_fall_time = 0.788;
                    } else if a_death2 {
                        a_fall_time = 0.71;
                    } else {
                        a_fall_time = 0.77;
                    }
                }
                ZombieType::Polevaulter => a_fall_time = 0.68,
                ZombieType::Football => a_fall_time = 0.52,
                ZombieType::Newspaper => a_fall_time = 0.63,
                ZombieType::Dancer | ZombieType::BackupDancer => a_fall_time = 0.83,
                ZombieType::Bobsled => a_fall_time = 0.81,
                ZombieType::JackInTheBox => a_fall_time = 0.64,
                ZombieType::Balloon => a_fall_time = 0.68,
                ZombieType::Digger => a_fall_time = 0.85,
                ZombieType::Pogo => a_fall_time = 0.84,
                ZombieType::Yeti => a_fall_time = 0.68,
                ZombieType::Ladder => a_fall_time = 0.62,
                ZombieType::Gargantuar | ZombieType::RedeEyeGargantuar => a_fall_time = 0.86,
                _ => {}
            }

            // 对应 C++: 到达倒地时刻 → 音效/雏菊
            if a_fall_time > 0.0 {
                let a_trigger_fall = self.base.get_app().map_or(false, |app| {
                    app.reanimation_get(self.body_reanim_id)
                        .map_or(false, |r| r.should_trigger_timed_event(a_fall_time))
                });
                if a_trigger_fall {
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::ZombieFalling as i32);
                        if self.zombie_type == ZombieType::Gargantuar
                            || self.zombie_type == ZombieType::RedeEyeGargantuar
                        {
                            app.play_foley(crate::todlib::tod_foley::FoleyType::Thump as i32);
                        }
                    }
                    // 对应 C++: mBoard->mDaisyMode → DoDaisies()
                    let a_daisy = self.base.get_board().map_or(false, |b| b.m_daisy_mode);
                    if a_daisy {
                        self.do_daisies();
                    }
                }
            }
        }

        // 对应 C++: Boss 死亡爆炸与旗子
        if self.zombie_type == ZombieType::Boss {
            let a_boss_trigger = [0.1f32, 0.12, 0.15, 0.19, 0.2, 0.26, 0.3, 0.4, 0.42, 0.5, 0.58, 0.61, 0.71]
                .iter().any(|&t| {
                    self.base.get_app().map_or(false, |app| {
                        app.reanimation_get(self.body_reanim_id)
                            .map_or(false, |r| r.should_trigger_timed_event(t))
                    })
                });
            if a_boss_trigger {
                // 对应 C++: 随机位置 BOSS_EXPLOSION 粒子 + FOLEY_BOSS_EXPLOSION_SMALL
                let a_explosion_x = 600.0 + RandFloat(150.0);
                let a_explosion_y = 50.0 + RandFloat(250.0);
                if let Some(app) = self.base.get_app_mut() {
                    (*app).add_tod_particle(
                        a_explosion_x, a_explosion_y,
                        crate::lawn::game_enums::RENDER_LAYER_TOP as i32,
                        crate::lawn::game_enums::ParticleEffect::BossExplosion as i32);
                    app.play_foley(crate::todlib::tod_foley::FoleyType::BossExplosionSmall as i32);
                }
            }
            // [TRANSLATION_NOTE]: C++ 中 0.93 震屏、0.99 头部旗子 reanim（mSpecialHeadReanimID）、
            // 头部旗子循环与 DropLoot 依赖头部 reanim/粒子封装，暂略
        }

        // 对应 C++: Zamboni/Catapult 倒计时爆炸，或通用 fade
        if self.zombie_type == ZombieType::Zamboni {
            if self.phase_counter > 0 {
                self.phase_counter -= 1;
                if self.phase_counter == 0 {
                    // [TRANSLATION_NOTE]: PARTICLE_ZAMBONI_EXPLOSION2/EXPLOSION 按 anim_wheelie2 判定，暂略
                    self.die_with_loot();
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::Explosion as i32);
                    }
                }
            }
        } else if self.zombie_type == ZombieType::Catapult {
            self.phase_counter -= 1;
            if self.phase_counter == 0 {
                // [TRANSLATION_NOTE]: PARTICLE_CATAPULT_EXPLOSION 粒子，暂略
                self.die_with_loot();
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Explosion as i32);
                }
            }
        } else if self.zombie_fade == -1 && self.zombie_type != ZombieType::Boss {
            let a_loop_done = self.base.get_app().map_or(false, |app| {
                app.reanimation_get(self.body_reanim_id)
                    .map_or(false, |r| r.m_loop_count > 0)
            });
            if a_loop_done {
                self.zombie_fade = if self.in_pool { 10 } else { 100 };
            }
        }
    }

    /// 冰车僵尸死亡（对应 C++ ZamboniDeath，Zombie.cpp:6498）
    pub fn zamboni_death(&mut self, damage_flags: u32) {
        if crate::lawn::zombie::test_bit(damage_flags, 5) {
            // 对应 C++: DAMAGE_SPIKE → 瘪胎
            self.flat_tires = true;
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::TirePop as i32);
            }
            self.zombie_phase = ZombiePhase::Dying;
            let a_part_x = self.pos_x + 29.0;
            let a_part_y = self.pos_y + 114.0;
            let a_render = self.base.render_order + 1;
            if let Some(app) = self.base.get_app_mut() {
                (*app).add_tod_particle(
                    a_part_x, a_part_y, a_render,
                    crate::lawn::game_enums::ParticleEffect::ZamboniTire as i32);
            }
            self.vel_x = 0.0;
            if RandRange(4) == 0 && self.pos_x < 600.0 {
                self.play_zombie_reanim("anim_wheelie2", ReanimLoopType::PlayOnceAndHold, 10, 10.0);
                self.phase_counter = 280;
            } else {
                // [TRANSLATION_NOTE]: C++ 中 PARTICLE_ZAMBONI_SMOKE + AttachParticleToTrack 粒子，暂略
                self.phase_counter = 280;
                self.play_zombie_reanim("anim_wheelie1", ReanimLoopType::PlayOnceAndHold, 10, 12.0);
            }
        } else {
            // 对应 C++: 非地刺击杀 → 直接爆炸
            let a_render = self.base.render_order + 1;
            if let Some(app) = self.base.get_app_mut() {
                (*app).add_tod_particle(
                    self.pos_x + 80.0, self.pos_y + 60.0, a_render,
                    crate::lawn::game_enums::ParticleEffect::ZamboniExplosion as i32);
            }
            self.die_with_loot();
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Explosion as i32);
            }
        }
    }

    /// 投石车僵尸死亡（对应 C++ CatapultDeath，Zombie.cpp:6534）
    pub fn catapult_death(&mut self, damage_flags: u32) {
        if crate::lawn::zombie::test_bit(damage_flags, 5) {
            // 对应 C++: DAMAGE_SPIKE → 轮胎爆胎
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::TirePop as i32);
            }
            self.zombie_phase = ZombiePhase::Dying;
            let a_part_x = self.pos_x + 29.0;
            let a_part_y = self.pos_y + 114.0;
            let a_render = self.base.render_order + 1;
            if let Some(app) = self.base.get_app_mut() {
                (*app).add_tod_particle(
                    a_part_x, a_part_y, a_render,
                    crate::lawn::game_enums::ParticleEffect::ZamboniTire as i32);
            }
            self.vel_x = 0.0;
            // [TRANSLATION_NOTE]: C++ 中 AddAttachedParticle(47,77,PARTICLE_ZAMBONI_SMOKE) 粒子，暂略
            self.phase_counter = 280;
            self.play_zombie_reanim("anim_bounce", ReanimLoopType::PlayOnceAndHold, 10, 12.0);
        }
    }

    /// 停止僵尸音效（对应 C++ StopZombieSound）
    pub fn stop_zombie_sound(&mut self) {
        use crate::todlib::tod_foley::FoleyType;

        if self.zombie_type == ZombieType::Dancer || self.zombie_type == ZombieType::BackupDancer {
            let mut a_stop_sound = true;

            if let Some(board) = self.base.get_board() {
                for a_zombie in &board.zombies {
                    if a_zombie.dead {
                        continue;
                    }
                    if a_zombie.has_head && !a_zombie.is_dead_or_dying() && a_zombie.is_on_board() &&
                        (a_zombie.zombie_type == ZombieType::Dancer || a_zombie.zombie_type == ZombieType::BackupDancer)
                    {
                        a_stop_sound = false;
                        break;
                    }
                }
            }

            if a_stop_sound {
                if let Some(app) = self.base.get_app() {
                    if let Some(ss) = &app.sound_system {
                        ss.stop_foley(FoleyType::Dancer);
                    }
                }
            }
        }

        if self.playing_song {
            self.playing_song = false;

            if self.zombie_type == ZombieType::JackInTheBox {
                if let Some(app) = self.base.get_app() {
                    if let Some(ss) = &app.sound_system {
                        ss.stop_foley(FoleyType::JackInTheBox);
                    }
                }
            } else if self.zombie_type == ZombieType::Digger {
                if let Some(app) = self.base.get_app() {
                    if let Some(ss) = &app.sound_system {
                        ss.stop_foley(FoleyType::Digger);
                    }
                }
            }
        }
    }

    /// 施加冻结（对应 C++ ApplyChill，Zombie.cpp 7519）
    pub fn apply_chill(&mut self, is_ice_trap: bool) {
        if !self.can_be_chilled() {
            return;
        }

        if self.chilled_counter == 0 {
            if let Some(app) = self.base.app {
                unsafe {
                    (*app).play_foley(crate::todlib::tod_foley::FoleyType::Frozen as i32);
                }
            }
        }

        let mut a_chill_time = 1000;
        if is_ice_trap {
            a_chill_time = 2000;
        }
        self.chilled_counter = a_chill_time.max(self.chilled_counter);

        self.update_anim_speed();
    }

    /// 直接死亡（对应 C++ DieNoLoot）
    pub fn die_no_loot(&mut self) {
        self.dead = true;
        self.zombie_phase = ZombiePhase::Dying;
        if self.playing_song {
            self.stop_zombie_sound();
        }
    }

    /// 死亡并掉落物品（对应 C++ DieWithLoot）
    /// C++: DieNoLoot(); DropLoot(); —— mDroppedLoot 由 DropLoot 内部按 C++ 顺序管理，
    /// 不能在进入 DropLoot 前预置，否则 DropLoot 首次判断即短路返回。
    pub fn die_with_loot(&mut self) {
        self.die_no_loot();
        self.drop_loot();
    }

    /// 尝试生成关卡奖励（对应 C++ TrySpawnLevelAward）
    pub fn try_spawn_level_award(&mut self) -> bool {
        let board = match self.base.board { Some(b) => b, None => return false };
        let app = match self.base.app { Some(a) => a, None => return false };
        unsafe {
            if !self.is_on_board()
                || (*board).has_level_award_dropped()
                || (*board).m_level_complete
                || self.dropped_loot
            {
                return false;
            }
            if (*app).is_final_boss_level() {
                if self.zombie_type != ZombieType::Boss {
                    return false;
                }
            } else if (*app).is_scary_potter_level() {
                let completed = (*board).challenge.as_ref().map_or(false, |c| c.scary_potter_is_completed() != 0);
                if !completed {
                    return false;
                }
            } else if (*app).is_continuous_challenge()
                || (*board).m_current_wave < (*board).m_num_waves
                || (*board).are_enemy_zombies_on_screen()
            {
                return false;
            }
            if (*app).is_whack_a_zombie_level() && (*board).m_zombie_count_down > 0 {
                return false;
            }

            (*board).m_level_award_spawned = true;
            (*app).board_result = BoardResult::Won;

            let zombie_rect = self.get_zombie_rect();
            let center_x = zombie_rect.x + zombie_rect.width / 2;
            let center_y = zombie_rect.y + zombie_rect.height / 2;

            let mut coin_type = CoinType::AwardMoneyBag;
            if (*app).is_scary_potter_level() && !(*board).is_final_scary_potter_stage() {
                coin_type = CoinType::None;
                let gx = (*board).pixel_to_grid_x_keep_on_board(self.pos_x as i32 + 75, self.pos_y as i32);
                if let Some(challenge) = &mut (*board).challenge {
                    challenge.puzzle_phase_complete(gx, self.base.row);
                }
            } else if (*board).is_survival_stage_with_repick() {
                coin_type = CoinType::None;
            } else if (*board).is_last_stand_stage_with_repick() {
                coin_type = CoinType::None;
                // FadeOutLevel 与阳光雨暂未接入
            } else if !(*app).is_adventure_mode() {
                if (*app).has_beaten_challenge((*app).game_mode) {
                    coin_type = CoinType::AwardMoneyBag;
                } else {
                    coin_type = CoinType::Trophy;
                }
            }

            let mut coin_motion = CoinMotion::Coin;
            if self.zombie_type == ZombieType::Boss {
                coin_motion = CoinMotion::FromBoss;
            }
            if coin_type != CoinType::None {
                (*app).play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
                (*board).add_coin(center_x as f32, center_y as f32, coin_type, coin_motion);
            }
            self.dropped_loot = true;
        }
        true
    }

    /// 掉落雏菊（对应 C++ DoDaisies）
    pub fn do_daisies(&mut self) {
        if self.is_walking_backwards() {
            return;
        }
        if let Some(board) = self.base.get_board() {
            let row = self.base.row as usize;
            if row < board.m_plant_row.len() && board.m_plant_row[row] == PlantRowType::Pool {
                return;
            }
            if matches!(self.zombie_type, ZombieType::Bobsled | ZombieType::Zamboni | ZombieType::Catapult) {
                return;
            }
            if board.stage_has_roof() {
                return;
            }
            let mut offset_x = 20.0f32;
            let mut offset_y = 100.0f32;
            if matches!(self.zombie_type, ZombieType::Football | ZombieType::Dancer | ZombieType::BackupDancer) {
                offset_x += 160.0;
            } else if self.zombie_type == ZombieType::Pogo {
                offset_y += 120.0;
            } else if self.zombie_type == ZombieType::Balloon {
                offset_y += 30.0;
                offset_x += 110.0;
            }
            if board.stage_has_grave_stones() {
                offset_y += 15.0;
            }
            if let Some(app) = self.base.app {
                unsafe {
                    (*app).add_tod_particle(self.pos_x + offset_x, self.pos_y + offset_y, self.base.render_order, crate::lawn::game_enums::ParticleEffect::ZombieDaisies as i32);
                }
            }
        }
    }

    /// 掉落物品（对应 C++ Zombie::DropLoot）
    /// C++ 顺序：图鉴击杀标记 → 雪人标记 → TrySpawnLevelAward →
    /// 判定 mDroppedLoot / 关卡奖励已掉落 / 能否掉落 → 小麻烦关卡 3/4 概率跳过 →
    /// 水族馆与 I, Zombie 关卡跳过 → 雪人掉 4 钻石，其余掉 DropLootPiece。
    pub fn drop_loot(&mut self) {
        if !self.is_on_board() {
            return;
        }
        crate::lawn::widget::almanac_dialog::almanac_player_defeated_zombie(self.zombie_type);
        if self.zombie_type == ZombieType::Yeti {
            if let Some(board) = self.base.board {
                unsafe {
                    (*board).m_killed_yeti = true;
                }
            }
        }
        self.try_spawn_level_award();
        let board = match self.base.board {
            Some(b) => b,
            None => return,
        };
        let app = match self.base.app {
            Some(a) => a,
            None => return,
        };
        unsafe {
            if self.dropped_loot || (*board).has_level_award_dropped() || !(*board).can_drop_loot() {
                return;
            }
            self.dropped_loot = true;
            let zombie_value = get_zombie_definition(self.zombie_type).zombie_value;
            if (*app).is_little_trouble_level() && RandRange(4) != 0 {
                // C++: Little Trouble 关卡 75% 概率不掉落任何东西
                return;
            }
            if (*app).game_mode == GameMode::ChallengeZombiquarium
                || crate::lawn::widget::challenge_screen::ChallengeScreen::is_i_zombie_level((*app).game_mode)
            {
                return;
            }
            let zombie_rect = self.get_zombie_rect();
            let center_x = zombie_rect.x + zombie_rect.width / 2;
            let center_y = zombie_rect.y + zombie_rect.height / 4;
            if self.zombie_type == ZombieType::Yeti {
                (*app).play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
                (*board).add_coin((center_x - 20) as f32, center_y as f32, CoinType::Diamond, CoinMotion::Coin);
                (*board).add_coin((center_x - 30) as f32, center_y as f32, CoinType::Diamond, CoinMotion::Coin);
                (*board).add_coin((center_x - 40) as f32, center_y as f32, CoinType::Diamond, CoinMotion::Coin);
                (*board).add_coin((center_x - 50) as f32, center_y as f32, CoinType::Diamond, CoinMotion::Coin);
            } else {
                (*board).drop_loot_piece(center_x, center_y, zombie_value);
            }
        }
    }

    /// 绘制僵尸（对应 C++ Zombie::DrawZombie 简化：绘制身体 reanim + 影子）
    pub fn draw(&self, g: &mut Graphics) {
        // C++ Zombie::Draw
        if self.zombie_height == ZombieHeight::GettingBungeeDropped {
            return;
        }

        let a_draw_pos = self.get_draw_pos();

        // C++: SCENE_ZOMBIES_WON 场景需裁剪
        let zombies_won = self.base.get_app().map_or(false, |a| a.game_scene == crate::lawn::lawn_app::GameScenes::ZombiesWon);
        if zombies_won && !self.setup_draw_zombie_won(g) {
            return;
        }

        if self.ice_trap_counter > 0 {
            self.draw_ice_trap(g, &a_draw_pos, false);
        }
        // C++: Invisighoul 模式隐藏（除 UI 僵尸）
        let invisible_mode = self.base.get_app().map_or(false, |a| a.game_mode == GameMode::ChallengeInvisighoul);
        if !invisible_mode || self.from_wave == Zombie::ZOMBIE_WAVE_UI {
            if self.body_reanim_id != REANIMATIONID_NULL {
                self.draw_reanim(g, &a_draw_pos, 0); // RENDER_GROUP_NORMAL
            }
        }
        if self.ice_trap_counter > 0 {
            self.draw_ice_trap(g, &a_draw_pos, true);
        }
        if self.buttered_counter > 0 {
            self.draw_butter(g, &a_draw_pos);
        }

        // 对应 C++ Zombie.cpp:6337 AttachmentDraw(mAttachmentID, &theParticleGraphics, false)
        if self.attachment_id != crate::lawn::game_enums::ATTACHMENTID_NULL {
            let mut a_attachment_id = self.attachment_id;
            crate::todlib::attachment::attachment_draw(&mut a_attachment_id, g, false);
        }

        g.clear_clip_rect();
    }

    /// 核心绘制（对应 C++ Zombie::DrawReanim）
    pub fn draw_reanim(&self, g: &mut Graphics, the_draw_pos: &ZombieDrawPosition, the_base_render_group: i32) {
        const CLIP_HEIGHT_LIMIT: f32 = -100.0;
        const BOSS_FLASH_HEALTH_FRACTION: i32 = 10;

        if the_draw_pos.clip_height > CLIP_HEIGHT_LIMIT {
            let a_draw_height = 120.0 - the_draw_pos.clip_height + 71.0;
            g.set_clip_rect_xywh(
                (the_draw_pos.image_offset_x - 200.0) as i32,
                (the_draw_pos.image_offset_y + the_draw_pos.body_y - 78.0) as i32,
                520,
                a_draw_height as i32,
            );
        }

        let a_fade_alpha = if self.zombie_fade >= 0 {
            (255 * self.zombie_fade / 10).clamp(0, 255) as u8
        } else { 255 };

        let mut a_color_override = crate::framework::color::Color::new(255, 255, 255, a_fade_alpha);
        let mut a_extra_additive_color = crate::framework::color::Color::BLACK;
        let mut a_enable_extra_additive_draw = false;
        if self.zombie_phase == ZombiePhase::Burned {
            a_color_override = crate::framework::color::Color::new(0, 0, 0, a_fade_alpha);
            a_extra_additive_color = crate::framework::color::Color::BLACK;
            a_enable_extra_additive_draw = false;
        } else if self.zombie_type == ZombieType::Boss
            && self.zombie_phase != ZombiePhase::Dying
            && self.body_health < self.body_max_health / BOSS_FLASH_HEALTH_FRACTION
        {
            let main_counter = self.base.get_board().map_or(0, |b| b.m_main_counter);
            let a_grayness = crate::todlib::tod_common::tod_animate_curve(
                0, 39, (main_counter % 40) as i32, 155, 255, TodCurves::Bounce,
            ) as u8;
            if self.chilled_counter > 0 || self.ice_trap_counter > 0 {
                let a_cold_color = crate::todlib::tod_common::tod_animate_curve(
                    0, 39, (main_counter % 40) as i32, 65, 75, TodCurves::Bounce,
                ) as u8;
                a_color_override = crate::framework::color::Color::new(a_cold_color, a_cold_color, a_grayness, a_fade_alpha);
            } else {
                a_color_override = crate::framework::color::Color::new(a_grayness, a_grayness, a_grayness, a_fade_alpha);
            }
            a_extra_additive_color = crate::framework::color::Color::BLACK;
            a_enable_extra_additive_draw = false;
        } else if self.mind_controlled {
            a_color_override = crate::framework::color::Color::new(128, 64, 192, a_fade_alpha); // ZOMBIE_MINDCONTROLLED_COLOR
            a_extra_additive_color = a_color_override;
            a_enable_extra_additive_draw = true;
        } else if self.chilled_counter > 0 || self.ice_trap_counter > 0 {
            a_color_override = crate::framework::color::Color::new(75, 75, 255, a_fade_alpha);
            a_extra_additive_color = a_color_override;
            a_enable_extra_additive_draw = true;
        } else if self.zombie_height == ZombieHeight::Zombiquarium && self.body_health < 100 {
            a_color_override = crate::framework::color::Color::new(100, 150, 25, a_fade_alpha);
            a_extra_additive_color = a_color_override;
            a_enable_extra_additive_draw = true;
        }
        if self.just_got_shot_counter > 0 && !self.is_bobsled_team_with_sled() {
            let a_grayness = (self.just_got_shot_counter * 10) as u8;
            let a_highlight_color = crate::framework::color::Color::new(a_grayness, a_grayness, a_grayness, 255);
            // C++ ColorAdd：分量相加后截断到 255
            a_extra_additive_color = crate::framework::color::Color::new(
                (a_highlight_color.r as u16 + a_extra_additive_color.r as u16).min(255) as u8,
                (a_highlight_color.g as u16 + a_extra_additive_color.g as u16).min(255) as u8,
                (a_highlight_color.b as u16 + a_extra_additive_color.b as u16).min(255) as u8,
                (a_highlight_color.a as u16 + a_extra_additive_color.a as u16).min(255) as u8,
            );
            a_enable_extra_additive_draw = true;
        }

        // C++: aBodyReanim->mColorOverride = ... （&self 下经裸指针获取可变 reanim）
        if let Some(app_ptr) = self.base.app {
            unsafe {
                let app = &mut *app_ptr;
                if let Some(body) = app.reanimation_get_mut(self.body_reanim_id) {
                    body.m_color_override = a_color_override;
                    body.m_extra_additive_color = a_extra_additive_color;
                    body.m_enable_extra_additive_draw = a_enable_extra_additive_draw;
                }
            }
        }

        // C++: 按僵尸类型选择绘制路径（雪橇/蹦极/舞者/普通）
        if self.zombie_type == ZombieType::Bobsled {
            self.draw_bobsled_reanim(g, the_draw_pos, true);
            if let Some(app) = self.base.get_app() {
                if let Some(body) = app.reanimation_get(self.body_reanim_id) {
                    body.draw_render_group(g, the_base_render_group);
                }
            }
            self.draw_bobsled_reanim(g, the_draw_pos, false);
        } else if self.zombie_type == ZombieType::Bungee {
            self.draw_bungee_reanim(g);
        } else if self.zombie_type == ZombieType::Dancer {
            self.draw_dancer_reanim(g);
        } else {
            if let Some(app) = self.base.get_app() {
                if let Some(body) = app.reanimation_get(self.body_reanim_id) {
                    body.draw_render_group(g, the_base_render_group);
                }
            }
        }

        // C++: 盾牌渲染组
        if self.shield_type != ShieldType::None {
            if let Some(app_ptr) = self.base.app {
                unsafe {
                    let app = &mut *app_ptr;
                    if let Some(body) = app.reanimation_get_mut(self.body_reanim_id) {
                        if self.zombie_phase == ZombiePhase::Burned {
                            body.m_color_override = crate::framework::color::Color::new(0, 0, 0, a_fade_alpha);
                            body.m_extra_additive_color = crate::framework::color::Color::BLACK;
                            body.m_enable_extra_additive_draw = false;
                        } else if self.shield_just_got_shot_counter > 0 {
                            let a_grayness = (self.shield_just_got_shot_counter * 10) as u8;
                            body.m_color_override = crate::framework::color::Color::new(a_grayness, a_grayness, a_grayness, a_fade_alpha);
                            body.m_extra_additive_color = crate::framework::color::Color::WHITE;
                            body.m_enable_extra_additive_draw = true;
                        } else {
                            body.m_color_override = crate::framework::color::Color::new(255, 255, 255, a_fade_alpha);
                            body.m_extra_additive_color = crate::framework::color::Color::BLACK;
                            body.m_enable_extra_additive_draw = false;
                        }
                    }
                }
            }
            let a_shield_hit_offset = if self.shield_recoil_counter > 0 {
                crate::todlib::tod_common::tod_animate_curve_float(
                    12, 0, self.shield_recoil_counter, 3.0, 0.0, TodCurves::Linear,
                )
            } else { 0.0 };
            g.translate_f(a_shield_hit_offset, 0.0);
            if let Some(app) = self.base.get_app() {
                if let Some(body) = app.reanimation_get(self.body_reanim_id) {
                    body.draw_render_group(g, 1); // RENDER_GROUP_SHIELD
                }
            }
            g.translate_f(-a_shield_hit_offset, 0.0);
        }

        // C++: 报纸/门/梯子盾牌在覆盖层之上
        if self.shield_type == ShieldType::Newspaper
            || self.shield_type == ShieldType::Door
            || self.shield_type == ShieldType::Ladder
        {
            if let Some(app_ptr) = self.base.app {
                unsafe {
                    let app = &mut *app_ptr;
                    if let Some(body) = app.reanimation_get_mut(self.body_reanim_id) {
                        body.m_color_override = a_color_override;
                        body.m_extra_additive_color = a_extra_additive_color;
                        body.m_enable_extra_additive_draw = a_enable_extra_additive_draw;
                    }
                }
            }
            if let Some(app) = self.base.get_app() {
                if let Some(body) = app.reanimation_get(self.body_reanim_id) {
                    body.draw_render_group(g, 3); // RENDER_GROUP_OVER_SHIELD
                }
            }
        }

        g.clear_clip_rect();
    }

    /// 绘制雪橇（对应 C++ Zombie::DrawBobsledReanim）
    pub fn draw_bobsled_reanim(&self, g: &mut Graphics, the_draw_pos: &ZombieDrawPosition, the_before_zombie: bool) {
        // [TRANSLATION_NOTE]: 雪橇绘制依赖 BOBSLED 图片资源与领队僵尸，图片未接入时跳过
        let _ = (g, the_draw_pos, the_before_zombie);
    }

    /// 绘制蹦极（对应 C++ Zombie::DrawBungeeReanim）
    pub fn draw_bungee_reanim(&self, g: &mut Graphics) {
        // C++: 先画蹦极绳，再画本体，再画被抓目标
        self.draw_bungee_cord(g, -22);
        if let Some(app) = self.base.get_app() {
            if let Some(body) = app.reanimation_get(self.body_reanim_id) {
                body.draw(g);
            }
        }
        // [TRANSLATION_NOTE]: 被抓僵尸/植物绘制依赖 board 查找与 graphics 拷贝，暂以注释保留
        let _ = g;
    }

    /// 绘制蹦极目标标记（对应 C++ Zombie::DrawBungeeTarget）
    pub fn draw_bungee_target(&self, g: &mut Graphics) {
        if !self.is_on_board() || self.base.get_app().map_or(false, |a| a.is_final_boss_level()) {
            return;
        }
        if self.zombie_phase == ZombiePhase::BungeeHitOuchy || self.zombie_phase == ZombiePhase::BungeeRising {
            return;
        }
        if self.related_zombie_id != ZOMBIEID_NULL {
            return;
        }
        let a_draw_pos = self.get_draw_pos();
        let mut a_target_x = self.base.x as f32 + 10.0;
        let mut a_target_y = self.base.y as f32 + 60.0 + a_draw_pos.body_y + a_draw_pos.image_offset_y;
        if self.zombie_phase == ZombiePhase::BungeeDiving || self.zombie_phase == ZombiePhase::BungeeDivingScreaming {
            a_target_x += crate::todlib::tod_common::tod_animate_curve_float(
                3000, 2600, self.altitude as i32, 30.0, 0.0, TodCurves::Linear,
            );
            a_target_y += crate::todlib::tod_common::tod_animate_curve_float(
                3000, 2600, self.altitude as i32, -600.0, 0.0, TodCurves::Linear,
            );
        }
        if let Some(img) = self.get_zombie_image("bungeetarget") {
            g.draw_image_f_xy(img, a_target_x, a_target_y + self.altitude);
        }
    }

    /// 绘制舞者聚光灯与本体（对应 C++ Zombie::DrawDancerReanim）
    pub fn draw_dancer_reanim(&self, g: &mut Graphics) {
        let mut a_spot_light_color = crate::framework::color::Color::new(250, 250, 160, 255);
        let mut a_draw_spot_light = false;
        if self.zombie_phase != ZombiePhase::DancerDancingIn
            && self.zombie_phase != ZombiePhase::DancerSnappingFingers
            && self.zombie_phase != ZombiePhase::Normal
            && self.zombie_phase != ZombiePhase::Dying
        {
            a_draw_spot_light = true;
            match if self.zombie_age >= 700 { self.zombie_age / 100 * 7 % 5 } else { 0 } {
                1 => a_spot_light_color = crate::framework::color::Color::new(114, 234, 170, 255),
                2 => a_spot_light_color = crate::framework::color::Color::new(216, 126, 202, 255),
                3 => a_spot_light_color = crate::framework::color::Color::new(90, 110, 140, 255),
                4 => a_spot_light_color = crate::framework::color::Color::new(240, 90, 130, 255),
                _ => {}
            }
            g.set_colorize_images(true);
            g.set_color(&a_spot_light_color);
            // [TRANSLATION_NOTE]: IMAGE_SPOTLIGHT2 图片未接入，跳过
            g.set_colorize_images(false);
        }

        if let Some(app) = self.base.get_app() {
            if let Some(body) = app.reanimation_get(self.body_reanim_id) {
                body.draw(g);
            }
        }
        let _ = a_spot_light_color;
        let _ = a_draw_spot_light;
    }

    /// 绘制蹦极绳（对应 C++ Zombie::DrawBungeeCord）
    pub fn draw_bungee_cord(&self, g: &mut Graphics, the_offset_x: i32) {
        let a_cord_cel_height = if let Some(img) = self.get_zombie_image("bungeecord") {
            (img.get_cel_height() as f32 * self.scale_zombie) as i32
        } else {
            (20.0 * self.scale_zombie) as i32
        };
        let (a_pos_x, a_pos_y) = self.get_track_position("Zombie_bungi_body");
        let _ = a_pos_x;
        // C++: 从 aPosY 向上逐段绘制
        if a_cord_cel_height > 0 {
            let mut y = a_pos_y - a_cord_cel_height as f32;
            while y > -a_cord_cel_height as f32 {
                if let Some(img) = self.get_zombie_image("bungeecord") {
                    let scale = self.scale_zombie;
                    g.set_scale(scale, scale, 0.0, 0.0);
                    g.draw_image_f_xy(img, (the_offset_x + 61 - 4) as f32 - 4.0 / scale, y - self.pos_y);
                    g.set_scale(1.0, 1.0, 0.0, 0.0);
                }
                y -= a_cord_cel_height as f32;
            }
        }
        g.clear_clip_rect();
    }

    /// 获取轨道位置（对应 C++ Zombie::GetTrackPosition）
    /// 简化：使用身体 reanim 的位置；轨道矩阵未接入时返回僵尸位置
    pub fn get_track_position(&self, track_name: &str) -> (f32, f32) {
        let body_pos = self.base.get_app()
            .and_then(|app| app.reanimation_get(self.body_reanim_id))
            .map(|r| (r.m_x, r.m_y));
        match body_pos {
            Some((x, y)) => {
                // [TRANSLATION_NOTE]: C++ 用 FindTrackIndex + GetTrackMatrix 得到轨道绝对位置；
                // Rust 侧以 reanim 位置 + 僵尸位置近似
                let _ = track_name;
                (x + self.pos_x, y + self.pos_y)
            }
            None => (self.pos_x, self.pos_y),
        }
    }

    /// 获胜场景绘制准备（对应 C++ Zombie::SetupDrawZombieWon）
    pub fn setup_draw_zombie_won(&self, g: &mut Graphics) -> bool {
        if self.from_wave != Zombie::ZOMBIE_WAVE_WINNER {
            return true;
        }
        // [TRANSLATION_NOTE]: mBoard->mCutScene->ShowZombieWalking() 依赖过场系统，暂假定可见
        // C++: 按背景类型裁剪（Rust BackgroundType：Day/Night、Pool/Fog、Roof/Boss）
        let background = self.base.get_board().map_or(BackgroundType::Day, |b| b.m_background_type);
        match background {
            BackgroundType::Day | BackgroundType::Night => {
                g.set_clip_rect_xywh(-123 - self.base.x, -self.base.y, 900, 600);
            }
            BackgroundType::Pool | BackgroundType::Fog => {
                g.set_clip_rect_xywh(-172 - self.base.x, -self.base.y, 900, 600);
            }
            BackgroundType::Roof | BackgroundType::Boss => {
                g.set_clip_rect_xywh(-220 - self.base.x, -self.base.y, 900, 187);
            }
            _ => {}
        }
        true
    }

    /// 绘制冰陷阱（对应 C++ Zombie::DrawIceTrap）
    pub fn draw_ice_trap(&self, g: &mut Graphics, the_draw_pos: &ZombieDrawPosition, the_front: bool) {
        if self.in_pool || self.zombie_type == ZombieType::Boss {
            return;
        }
        let mut a_offset_x = 46.0;
        let mut a_offset_y = the_draw_pos.body_y + 92.0;
        let mut a_scale = 1.0;
        match self.zombie_type {
            ZombieType::Pogo => {
                a_offset_x -= 10.0;
                a_offset_y += 20.0;
            }
            ZombieType::Gargantuar | ZombieType::RedeEyeGargantuar => {
                a_offset_x -= 20.0;
                a_offset_y -= 7.0;
                a_scale = 1.6;
            }
            ZombieType::Bungee => {
                a_offset_x -= 45.0;
                a_offset_y -= 23.0;
                a_scale = 1.2;
            }
            ZombieType::Digger => { a_offset_x -= 27.0; }
            ZombieType::Catapult => { a_offset_x += 32.0; }
            ZombieType::Balloon => {
                a_offset_x -= 9.0;
                a_offset_y += 27.0;
            }
            _ => {}
        }
        let img_id = if the_front { "icetrap" } else { "icetrap2" };
        if let Some(img) = self.get_zombie_image(img_id) {
            g.set_scale(a_scale, a_scale, 0.0, 0.0);
            g.draw_image_f_xy(img, a_offset_x, a_offset_y);
            g.set_scale(1.0, 1.0, 0.0, 0.0);
        }
    }

    /// 绘制黄油（对应 C++ Zombie::DrawButter）
    pub fn draw_butter(&self, g: &mut Graphics, the_draw_pos: &ZombieDrawPosition) {
        let mut a_offset_x = self.pos_x + the_draw_pos.image_offset_x + the_draw_pos.head_x as f32 + 11.0;
        let mut a_offset_y = self.pos_y + the_draw_pos.image_offset_y + the_draw_pos.head_y as f32 + the_draw_pos.body_y + 21.0;
        let mut a_scale = 1.0;
        if self.zombie_phase == ZombiePhase::NewspaperMaddening {
            let (tx, ty) = self.get_track_position("anim_head_look");
            a_offset_x = tx;
            a_offset_y = ty;
        } else if self.zombie_type == ZombieType::Catapult {
            let (tx, ty) = self.get_track_position("Zombie_catapult_driver_head");
            a_offset_x = tx;
            a_offset_y = ty;
        } else if self.body_reanim_id != REANIMATIONID_NULL {
            let (tx, ty) = self.get_track_position("anim_head1");
            a_offset_x = tx;
            a_offset_y = ty;
        }
        a_offset_x -= self.pos_x + 29.0;
        a_offset_y -= self.pos_y + 36.0;

        match self.zombie_type {
            ZombieType::Pogo => { a_offset_y -= 5.0; }
            ZombieType::Gargantuar | ZombieType::RedeEyeGargantuar => {
                a_offset_x -= 5.0;
                a_offset_y -= 15.0;
                a_scale = 1.2;
            }
            ZombieType::SquashHead => {
                a_offset_x += 6.0;
                a_offset_y -= 9.0;
            }
            ZombieType::WallnutHead => {
                a_offset_x -= 6.0;
                a_offset_y -= 1.0;
            }
            ZombieType::TallnutHead => {
                a_offset_x -= 24.0;
                a_offset_y -= 39.0;
            }
            _ => {}
        }

        if let Some(img) = self.get_zombie_image("reanim_cornpult_butter_splat") {
            g.set_scale(a_scale, a_scale, 0.0, 0.0);
            g.draw_image_f_xy(img, a_offset_x, a_offset_y);
            g.set_scale(1.0, 1.0, 0.0, 0.0);
        }
    }

    /// 绘制影子（对应 C++ Zombie::DrawShadow）
    pub fn draw_shadow(&self, g: &mut Graphics) {
        const HIGH_GROUND_HEIGHT: f32 = 30.0;
        const BUNGEE_ZOMBIE_HEIGHT: f32 = 3000.0;

        let a_draw_pos = self.get_draw_pos();
        let zombies_won = self.base.get_app().map_or(false, |a| a.game_scene == crate::lawn::lawn_app::GameScenes::ZombiesWon);
        if zombies_won && !self.setup_draw_zombie_won(g) {
            return;
        }
        // 水族馆模式影子很蠢，不绘制
        if self.base.get_app().map_or(false, |a| a.game_mode == GameMode::ChallengeZombiquarium) {
            return;
        }

        let mut a_shadow_type = 0;
        let mut a_shadow_offset_x = a_draw_pos.image_offset_x;
        let mut a_shadow_offset_y = a_draw_pos.image_offset_y + a_draw_pos.body_y;
        let mut a_scale = self.scale_zombie;
        a_shadow_offset_x += self.scale_zombie * 20.0 - 20.0;
        if self.is_on_board() && self.base.get_board().map_or(false, |b| b.stage_is_night()) {
            a_shadow_type = 1;
        }

        let backwards = self.is_walking_backwards();
        match self.zombie_type {
            ZombieType::Football => {
                a_shadow_offset_x += if backwards { -11.0 * self.scale_zombie } else { 20.0 + 21.0 * self.scale_zombie };
                a_shadow_offset_y += 16.0;
            }
            ZombieType::Newspaper => {
                a_shadow_offset_x += if backwards { 5.0 } else { 29.0 };
            }
            ZombieType::Polevaulter => {
                a_shadow_offset_x += if backwards { -5.0 } else { 36.0 };
                a_shadow_offset_y += 11.0;
            }
            ZombieType::Bobsled => {
                a_shadow_offset_x += if backwards { 13.0 } else { 20.0 };
                a_shadow_offset_y += 13.0;
            }
            ZombieType::Imp => {
                a_scale *= 0.6;
                a_shadow_offset_y += 7.0;
                a_shadow_offset_x += if backwards { 13.0 } else { 25.0 };
            }
            ZombieType::Digger => {
                a_shadow_offset_y += 5.0;
                a_shadow_offset_x += if backwards { 14.0 } else { 17.0 };
            }
            ZombieType::Snorkel => {
                a_shadow_offset_y += 5.0;
                a_shadow_offset_x += if backwards { -2.0 } else { 35.0 };
            }
            ZombieType::DolphinRider => {
                a_shadow_offset_y += 11.0;
                a_shadow_offset_x += if backwards { 15.0 } else { 19.0 };
            }
            ZombieType::Yeti => {
                a_shadow_offset_y += 20.0;
                a_shadow_offset_x += if backwards { 20.0 } else { 3.0 };
            }
            ZombieType::Gargantuar | ZombieType::RedeEyeGargantuar => {
                a_scale *= 1.5;
                a_shadow_offset_x += 27.0;
                a_shadow_offset_y += 7.0;
            }
            _ => {
                // C++: mApp->ReanimationTryToGet(mBodyReanimID) != nullptr
                if self.base.get_app().and_then(|app| app.reanimation_get(self.body_reanim_id)).is_some() {
                    a_shadow_offset_x += if backwards { 11.0 } else { 23.0 };
                } else {
                    a_shadow_offset_x += if backwards { -2.0 } else { 35.0 };
                }
            }
        }

        if self.zombie_type == ZombieType::Newspaper {
            a_shadow_offset_y += 4.0;
        } else if self.zombie_type == ZombieType::Balloon {
            a_shadow_offset_y += 13.0;
        } else if self.zombie_type == ZombieType::Bungee {
            a_shadow_offset_x -= 12.0;
            a_scale = crate::todlib::tod_common::tod_animate_curve_float(
                (BUNGEE_ZOMBIE_HEIGHT - 1000.0) as i32, 100, self.altitude as i32, 0.1, 1.5, TodCurves::Linear,
            );
        }

        if self.zombie_height == ZombieHeight::UpLadder
            || self.zombie_height == ZombieHeight::Falling
            || self.zombie_phase == ZombiePhase::ImpGettingThrown
            || self.zombie_type == ZombieType::Bungee
            || self.is_bouncing_pogo()
            || self.is_flying()
        {
            a_shadow_offset_y += self.altitude;
            if self.is_on_high_ground() {
                a_shadow_offset_y -= HIGH_GROUND_HEIGHT;
            }
        }

        if self.in_pool {
            if let Some(img) = self.get_zombie_image("whitewater_shadow") {
                g.set_scale(a_scale, a_scale, 0.0, 0.0);
                g.draw_image_f_xy(img, a_shadow_offset_x, a_shadow_offset_y + 67.0);
                g.set_scale(1.0, 1.0, 0.0, 0.0);
            }
        } else {
            let img_id = if a_shadow_type == 0 { "plantshadow" } else { "plantshadow2" };
            if let Some(img) = self.get_zombie_image(img_id) {
                g.set_scale(a_scale, a_scale, 0.0, 0.0);
                g.draw_image_f_xy(img, a_shadow_offset_x, a_shadow_offset_y + 92.0);
                g.set_scale(1.0, 1.0, 0.0, 0.0);
            }
        }

        g.clear_clip_rect();
    }

    /// 绘制 Boss 部位（对应 C++ Zombie::DrawBossPart）
    pub fn draw_boss_part(&self, g: &mut Graphics, the_boss_part: BossPart) {
        let a_draw_pos = self.get_draw_pos();
        // C++ 渲染组：BACK_LEG=4 / FRONT_LEG=5 / MAIN=0 / BACK_ARM=6 / FIREBALL
        match the_boss_part {
            BossPart::BackLeg => self.draw_reanim(g, &a_draw_pos, 4),
            BossPart::FrontLeg => self.draw_reanim(g, &a_draw_pos, 5),
            BossPart::Main => self.draw_reanim(g, &a_draw_pos, 0),
            BossPart::BackArm => self.draw_boss_back_arm(g, &a_draw_pos),
            BossPart::Fireball => self.draw_boss_fire_ball(g),
        }
    }

    /// 绘制 Boss 后臂（对应 C++ Zombie::DrawBossBackArm）
    /// 通过临时平移身体 reanim 的 overlay 位置实现手臂偏移
    pub fn draw_boss_back_arm(&self, g: &mut Graphics, the_draw_pos: &ZombieDrawPosition) {
        let mut a_image_offset_x = 0.0f32;
        let mut a_image_offset_y = 0.0f32;
        if self.zombie_phase == ZombiePhase::BossDropRv {
            a_image_offset_y = (self.target_row - 1) as f32 * 85.0 - self.target_col as f32 * 20.0;
            a_image_offset_x = self.target_col as f32 * 80.0;
        } else if self.zombie_phase == ZombiePhase::BossBungeesEnter
            || self.zombie_phase == ZombiePhase::BossBungeesDrop
            || self.zombie_phase == ZombiePhase::BossBungeesLeave
        {
            a_image_offset_x = self.target_col as f32 * 80.0 - 23.0;
        }

        // C++: aBodyReanim->mOverlayMatrix.m02 += aImageOffsetX; m12 += aImageOffsetY
        if let Some(app_ptr) = self.base.app {
            unsafe {
                let app = &mut *app_ptr;
                if let Some(body) = app.reanimation_get_mut(self.body_reanim_id) {
                    body.m_x += a_image_offset_x;
                    body.m_y += a_image_offset_y;
                }
            }
        }
        self.draw_reanim(g, the_draw_pos, 6); // RENDER_GROUP_BOSS_BACK_ARM
        if let Some(app_ptr) = self.base.app {
            unsafe {
                let app = &mut *app_ptr;
                if let Some(body) = app.reanimation_get_mut(self.body_reanim_id) {
                    body.m_x -= a_image_offset_x;
                    body.m_y -= a_image_offset_y;
                }
            }
        }
    }

    /// 绘制 Boss 火球（对应 C++ Zombie::DrawBossFireBall）
    pub fn draw_boss_fire_ball(&self, g: &mut Graphics) {
        let fireball_id = self.boss_fire_ball_reanim_id;
        if fireball_id == REANIMATIONID_NULL {
            return;
        }
        if let Some(app) = self.base.get_app() {
            if let Some(fireball) = app.reanimation_get(fireball_id) {
                fireball.draw_render_group(g, 0); // RENDER_GROUP_NORMAL
                // C++: DRAWMODE_ADDITIVE + RENDER_GROUP_BOSS_FIREBALL_ADDITIVE(7)
                g.set_draw_mode(1);
                fireball.draw_render_group(g, 7);
                g.set_draw_mode(0);
                // C++: DRAWMODE_NORMAL + RENDER_GROUP_BOSS_FIREBALL_TOP(8)
                fireball.draw_render_group(g, 8);
            }
        }
    }

    /// 获取僵尸图片（通过资源管理器按小写 id 获取 Image 指针）
    fn get_zombie_image(&self, id: &str) -> Option<&crate::framework::graphics::image::Image> {
        let app = self.base.get_app()?;
        let rm = app.base.resource_manager?;
        let shared = unsafe { (*rm).get_image(id) };
        unsafe {
            if !shared.unshared_image.is_null() {
                return Some(&(*(shared.unshared_image)).base);
            }
            if !shared.shared_image.is_null() {
                return Some(&(*(*(shared.shared_image)).image).base.base);
            }
        }
        None
    }

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

    /// 获取雪橇位置（对应 C++ GetBobsledPosition）
    /// 返回 -1 表示不是雪橇队；0 表示领队；1~3 表示跟随位置
    pub fn get_bobsled_position(&self) -> i32 {
        if self.zombie_type != ZombieType::Bobsled {
            return -1;
        }
        if self.related_zombie_id == ZOMBIEID_NULL && self.follower_zombie_ids[0] == ZOMBIEID_NULL {
            return -1;
        }
        if self.related_zombie_id == ZOMBIEID_NULL {
            return 0;
        }
        -1
    }

    /// 判断是否是有雪橇的雪橇队（对应 C++ IsBobsledTeamWithSled）
    pub fn is_bobsled_team_with_sled(&self) -> bool {
        self.get_bobsled_position() != -1
    }

    /// 设置行
    pub fn set_row(&mut self, row: i32) {
        self.base.row = row;
        self.pos_y = crate::lawn::board::row_to_y(row) as f32;
    }

    /// 获取渲染位置（对应 C++ Zombie::GetDrawPos）
    pub fn get_draw_pos(&self) -> ZombieDrawPosition {
        const CLIP_HEIGHT_LIMIT: f32 = -100.0;
        const CLIP_HEIGHT_OFF: f32 = -200.0;
        const HIGH_GROUND_HEIGHT: f32 = 30.0;
        const BUNGEE_ZOMBIE_HEIGHT: f32 = 3000.0;

        let mut pos = ZombieDrawPosition::new();
        pos.image_offset_x = self.pos_x - self.base.x as f32;
        pos.image_offset_y = self.pos_y - self.base.y as f32;

        if self.is_eating {
            pos.head_x = 47;
            pos.head_y = 4;
        } else {
            // C++: 按 mFrame 选择头偏移
            match self.frame {
                0 => { pos.head_x = 50; pos.head_y = 2; }
                1 => { pos.head_x = 49; pos.head_y = 1; }
                2 => { pos.head_x = 49; pos.head_y = 2; }
                3 => { pos.head_x = 48; pos.head_y = 4; }
                4 => { pos.head_x = 48; pos.head_y = 5; }
                5 => { pos.head_x = 48; pos.head_y = 4; }
                6 => { pos.head_x = 48; pos.head_y = 2; }
                7 => { pos.head_x = 49; pos.head_y = 1; }
                8 => { pos.head_x = 49; pos.head_y = 2; }
                9 => { pos.head_x = 50; pos.head_y = 4; }
                10 => { pos.head_x = 50; pos.head_y = 5; }
                _ => { pos.head_x = 50; pos.head_y = 4; }
            }
        }
        pos.arm_y = pos.head_y / 2;

        // C++: 按僵尸类型微调偏移
        match self.zombie_type {
            ZombieType::Football => { pos.image_offset_y -= 16.0; }
            ZombieType::Yeti => { pos.image_offset_y -= 20.0; }
            ZombieType::Catapult => {
                pos.image_offset_x -= 25.0;
                pos.image_offset_y -= 18.0;
            }
            ZombieType::Pogo => { pos.image_offset_y += 16.0; }
            ZombieType::Balloon => { pos.image_offset_y += 17.0; }
            ZombieType::Polevaulter => {
                pos.image_offset_x -= 6.0;
                pos.image_offset_y -= 11.0;
            }
            ZombieType::Zamboni => {
                pos.image_offset_x += 68.0;
                pos.image_offset_y -= 23.0;
            }
            ZombieType::Gargantuar | ZombieType::RedeEyeGargantuar => {
                pos.image_offset_y -= 8.0;
            }
            ZombieType::Bobsled => { pos.image_offset_y -= 12.0; }
            _ => {}
        }

        if self.zombie_phase == ZombiePhase::RisingFromGrave {
            pos.body_y = -self.altitude;
            if self.in_pool {
                pos.clip_height = pos.body_y;
            } else {
                let a_height_limit = self.phase_counter.min(40) as f32;
                pos.clip_height = pos.body_y + a_height_limit;
            }
            if self.on_high_ground {
                pos.body_y -= HIGH_GROUND_HEIGHT;
            }
            return pos;
        }

        if self.zombie_type == ZombieType::DolphinRider {
            pos.body_y = -self.altitude;
            pos.clip_height = CLIP_HEIGHT_OFF;
            let a_body_time = self.base.get_app()
                .and_then(|app| app.reanimation_get(self.body_reanim_id))
                .map_or(0.0, |r| r.m_anim_time);
            if self.zombie_phase == ZombiePhase::DolphinIntoPool {
                if a_body_time >= 0.56 && a_body_time <= 0.65 {
                    pos.clip_height = 0.0;
                } else if a_body_time >= 0.75 {
                    pos.clip_height = -self.altitude - 10.0;
                }
            } else if self.zombie_phase == ZombiePhase::DolphinRiding {
                pos.image_offset_x += 70.0;
                pos.clip_height = if self.zombie_height == ZombieHeight::DraggedUnder {
                    -self.altitude - 15.0
                } else {
                    -self.altitude - 10.0
                };
            } else if self.zombie_phase == ZombiePhase::DolphinInJump {
                pos.image_offset_x += 70.0 + self.altitude;
                if a_body_time <= 0.06 {
                    pos.clip_height = -self.altitude - 10.0;
                } else if a_body_time >= 0.5 && a_body_time <= 0.76 {
                    pos.clip_height = -13.0;
                }
            } else if self.zombie_phase == ZombiePhase::DolphinWalkingInPool
                || self.zombie_phase == ZombiePhase::Dying
            {
                pos.image_offset_y += 50.0;
                if self.zombie_phase == ZombiePhase::Dying {
                    pos.clip_height = -self.altitude + 44.0;
                } else if self.zombie_height == ZombieHeight::DraggedUnder {
                    pos.clip_height = -self.altitude + 36.0;
                }
            } else if (self.zombie_phase == ZombiePhase::DolphinWalking
                || self.zombie_phase == ZombiePhase::DolphinWalkingWithoutDolphin)
                && self.zombie_height == ZombieHeight::OutOfPool
            {
                pos.clip_height = -self.altitude;
            }
        } else if self.zombie_type == ZombieType::Snorkel {
            pos.body_y = -self.altitude;
            pos.clip_height = CLIP_HEIGHT_OFF;
            let a_body_time = self.base.get_app()
                .and_then(|app| app.reanimation_get(self.body_reanim_id))
                .map_or(0.0, |r| r.m_anim_time);
            if self.zombie_phase == ZombiePhase::SnorkelIntoPool {
                if a_body_time >= 0.8 {
                    pos.clip_height = -10.0;
                }
            } else if self.in_pool {
                pos.clip_height = -self.altitude - 5.0;
                pos.clip_height += 20.0 - 20.0 * self.scale_zombie;
            }
        } else if self.in_pool {
            pos.body_y = -self.altitude;
            pos.clip_height = -self.altitude - 7.0;
            pos.clip_height += 10.0 - 10.0 * self.scale_zombie;
            if self.is_eating {
                pos.clip_height += 7.0;
            }
        } else if self.zombie_phase == ZombiePhase::DancerRising {
            pos.body_y = -self.altitude;
            pos.clip_height = -self.altitude;
            if self.on_high_ground {
                pos.body_y -= HIGH_GROUND_HEIGHT;
            }
        } else if self.zombie_phase == ZombiePhase::DiggerRising
            || self.zombie_phase == ZombiePhase::DiggerRiseWithoutAxe
        {
            pos.body_y = -self.altitude;
            pos.clip_height = if self.phase_counter > 20 {
                -self.altitude
            } else {
                CLIP_HEIGHT_OFF
            };
        } else if self.zombie_type == ZombieType::Bungee {
            pos.body_y = -self.altitude;
            pos.image_offset_x -= 18.0;
            if self.on_high_ground {
                pos.body_y -= HIGH_GROUND_HEIGHT;
            }
            pos.clip_height = CLIP_HEIGHT_OFF;
        } else {
            pos.body_y = -self.altitude;
            pos.clip_height = CLIP_HEIGHT_OFF;
        }
        pos
    }

    // ========== 辅助函数（stub，对应 C++ Zombie 方法） ==========

    /// 加载重动画（对应 C++ LoadReanim）
    pub fn load_reanim(&mut self, reanim_type: ReanimationType) -> Option<*mut Reanimation> {
        // 对应 C++ Zombie::LoadReanim
        let render_order = self.base.render_order;
        let app = self.base.get_app_mut()?;
        let body_reanim = app.add_reanimation(self.pos_x, self.pos_y, render_order, reanim_type as i32)?;
        self.body_reanim_id = app.reanimation_get_id(body_reanim);
        unsafe {
            (*body_reanim).m_loop_type = ReanimLoopType::Loop;
            (*body_reanim).m_is_attachment = true;
            (*body_reanim).m_override_scale_x = self.scale_zombie;
            (*body_reanim).m_override_scale_y = self.scale_zombie;
        }

        if !self.is_on_board() {
            // 不在棋盘：随机取 idle 动画帧起点
            if RandRange(4) > 0 && unsafe { (*body_reanim).track_exists("anim_idle2") } {
                self.play_zombie_reanim("anim_idle2", ReanimLoopType::Loop, 0, 12.0 + RandFloat(12.0));  // RandRangeFloat(12,24)
            } else if unsafe { (*body_reanim).track_exists("anim_idle") } {
                self.play_zombie_reanim("anim_idle", ReanimLoopType::Loop, 0, 12.0 + RandFloat(6.0));  // RandRangeFloat(12,18)
            }
            unsafe { (*body_reanim).m_anim_time = RandFloat(0.99); }
        } else {
            self.start_walk_anim(0);
        }

        Some(body_reanim)
    }

    /// 加载普通僵尸重动画（对应 C++ LoadPlainZombieReanim）
    pub fn load_plain_zombie_reanim(&mut self) {
        self.zombie_attack_rect = Rect::new(20, 0, 50, 115);
        let reanim_id = self.body_reanim_id;
        let zombie_type = self.zombie_type;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(body) = app.reanimation_get_mut(reanim_id) {
                Self::setup_reanim_layers(body, zombie_type);
            }
        }
        let (mustache_mode, future_mode) = self.base.get_board().map_or((false, false), |b| (b.m_mustache_mode, b.m_future_mode));
        self.enable_mustache(mustache_mode);
        self.enable_future(future_mode);
        // C++: 泳池行或鸭圈僵尸显示水花轨道
        let is_pool_row = self.base.get_board().map_or(false, |b| {
            (b.m_plant_row.get(self.base.row as usize).map_or(false, |&r| r == PlantRowType::Pool)
                && self.from_wave != Zombie::ZOMBIE_WAVE_CUTSCENE)
        });
        if is_pool_row || self.zombie_type == ZombieType::DuckyTube {
            self.reanim_show_prefix("zombie_duckytube", 0); // RENDER_GROUP_NORMAL
            self.reanim_ignore_clip_rect("Zombie_duckytube", true);
            self.reanim_ignore_clip_rect("Zombie_outerarm_hand", true);
            self.reanim_ignore_clip_rect("Zombie_innerarm3", true);
            self.setup_water_track("Zombie_whitewater");
            self.setup_water_track("Zombie_whitewater2");
        }
    }

    /// 播放僵尸动画（对应 C++ PlayZombieReanim）
    pub fn play_zombie_reanim(&mut self, track_name: &str, loop_type: ReanimLoopType, blend_time: i32, anim_rate: f32) {
        if let Some(app) = self.base.get_app_mut() {
            if let Some(body) = app.reanimation_get_mut(self.body_reanim_id) {
                body.play_reanim(track_name, loop_type, blend_time, anim_rate);
            }
        }
        if anim_rate != 0.0 {
            self.original_anim_rate = anim_rate;
        }
        self.update_anim_speed();
    }

    /// 显示前缀动画轨道（对应 C++ ReanimShowPrefix）
    pub fn reanim_show_prefix(&mut self, track_prefix: &str, render_group: i32) {
        // C++: TryToGet(mBodyReanimID) 非空时 AssignRenderGroupToPrefix
        if let Some(app) = self.base.get_app_mut() {
            if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                a_body_reanim.assign_render_group_to_prefix(track_prefix, render_group);
            }
        }
    }

    /// 显示动画轨道（对应 C++ ReanimShowTrack）
    pub fn reanim_show_track(&mut self, track_name: &str, render_group: i32) {
        // C++: TryToGet(mBodyReanimID) 非空时 AssignRenderGroupToTrack
        if let Some(app) = self.base.get_app_mut() {
            if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                a_body_reanim.assign_render_group_to_track(track_name, render_group);
            }
        }
    }

    /// 是否在棋盘上（对应 C++ IsOnBoard）
    pub fn is_on_board(&self) -> bool {
        if self.from_wave == Zombie::ZOMBIE_WAVE_CUTSCENE || self.from_wave == Zombie::ZOMBIE_WAVE_UI {
            return false;
        }
        true
    }

    /// 装备盾牌（对应 C++ AttachShield）
    pub fn attach_shield(&mut self) {
        let shield_type = self.shield_type;
        let reanim_id = self.body_reanim_id;
        let mut track_name = "";
        if shield_type == ShieldType::Door {
            self.show_door_arms(true);
            self.reanim_show_prefix("Zombie_outerarm_screendoor", 3); // RENDER_GROUP_OVER_SHIELD
            track_name = "anim_screendoor";
        } else if shield_type == ShieldType::Newspaper {
            self.reanim_show_prefix("Zombie_paper_hands", 3); // RENDER_GROUP_OVER_SHIELD
            track_name = "Zombie_paper_paper";
        } else if shield_type == ShieldType::Ladder {
            self.reanim_show_prefix("Zombie_outerarm", 3); // RENDER_GROUP_OVER_SHIELD
            track_name = "Zombie_ladder_1";
        }
        if !track_name.is_empty() {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(body) = app.reanimation_get_mut(reanim_id) {
                    body.assign_render_group_to_track(track_name, 1); // RENDER_GROUP_SHIELD
                }
            }
        }
    }

    /// 卸下盾牌（对应 C++ DetachShield）
    pub fn detach_shield(&mut self) {
        let shield_type = self.shield_type;
        if shield_type == ShieldType::Door {
            self.show_door_arms(false);
        } else if shield_type == ShieldType::Newspaper {
            self.reanim_show_prefix("Zombie_paper_hands", 0); // RENDER_GROUP_NORMAL
        } else if shield_type == ShieldType::Ladder {
            self.reanim_show_prefix("Zombie_outerarm", 0); // RENDER_GROUP_NORMAL
            self.zombie_phase = ZombiePhase::Normal;
            if self.is_eating {
                self.play_zombie_reanim("anim_eat", ReanimLoopType::Loop, 20, 0.0);
            }
            // [TRANSLATION_NOTE]: C++ 中 Ladder 盾牌移除还会 ReanimShowPrefix("anim_zipline") 等 — reanim 未接入
        }
    }

    /// 设置水下动画轨道（对应 C++ SetupWaterTrack）
    pub fn setup_water_track(&mut self, track_name: &str) {
        // C++: ReanimationGet(mBodyReanimID) 非空时对轨道实例设置忽略附加色/颜色覆盖/裁剪
        if let Some(app) = self.base.get_app_mut() {
            if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                if let Some(a_track_instance) = a_body_reanim.get_track_instance_by_name(track_name) {
                    a_track_instance.m_ignore_extra_additive_color = true;
                    a_track_instance.m_ignore_color_override = true;
                    a_track_instance.m_ignore_clip_rect = true;
                }
            }
        }
    }

    /// 添加附加粒子（对应 C++ AddAttachedParticle）
    /// 附加粒子（对应 C++ Zombie::AddAttachedParticle，Zombie.cpp:8341）
    pub fn add_attached_particle(
        &mut self,
        pos_x: i32,
        pos_y: i32,
        effect: ParticleEffect,
    ) -> Option<*mut ParticleSystem> {
        if self.dead {
            return None;
        }

        if crate::todlib::attachment::is_full_of_attachments(self.attachment_id) {
            return None;
        }

        let a_particle = {
            let a_particle_x = self.base.x as f32 + pos_x as f32;
            let a_particle_y = self.base.y as f32 + pos_y as f32;
            if let Some(app) = self.base.get_app_mut() {
                app.add_tod_particle(a_particle_x, a_particle_y, 0, effect as i32)
            } else {
                None
            }
        };
        if let Some(a_particle) = a_particle {
            crate::todlib::attachment::attach_particle(
                &mut self.attachment_id,
                a_particle as *mut std::ffi::c_void,
                pos_x as f32,
                pos_y as f32,
            );
        }
        a_particle
    }

    /// 设置动画速率（对应 C++ SetAnimRate）
    pub fn set_anim_rate(&mut self, anim_rate: f32) {
        self.original_anim_rate = anim_rate;
        self.apply_anim_rate(anim_rate);
    }

    /// 应用动画速率（对应 C++ ApplyAnimRate）
    pub fn apply_anim_rate(&mut self, anim_rate: f32) {
        let reanim_id = self.body_reanim_id;
        let a_rate = if self.is_moving_at_chilled_speed() { anim_rate * 0.5 } else { anim_rate };
        if let Some(app) = self.base.get_app_mut() {
            if let Some(body) = app.reanimation_get_mut(reanim_id) {
                body.m_anim_rate = a_rate;
            }
        }
    }

    /// 更新动画速度（对应 C++ UpdateAnimSpeed）
    pub fn update_anim_speed(&mut self) {
        if self.chilled_counter > 0 { self.anim_ticks_per_frame = 5; }
        else { self.anim_ticks_per_frame = 2; }
    }

    /// 更新重动画（对应 C++ UpdateReanim）
    pub fn update_reanim(&mut self) {
        // 预先复制 self 字段，避免与 body 的可变借用冲突
        let zombie_type = self.zombie_type;
        let zombie_phase = self.zombie_phase;
        let body_damage_index = self.get_body_damage_index();
        let summon_counter = self.summon_counter;
        let body_health = self.body_health;
        let scale_zombie = self.scale_zombie;
        let is_walking_backwards = self.is_walking_backwards();
        let is_eating = self.is_eating;
        let mind_controlled = self.mind_controlled;
        // 预先计算绘制位置（避免与 body 的可变借用冲突）
        let a_draw_pos = self.get_draw_pos();

        let reanim_id = self.body_reanim_id;
        let body = self.base.get_app_mut().and_then(|app| app.reanimation_get_mut(reanim_id));
        let body = match body {
            Some(b) => b,
            None => return,
        };

        // Catapult 受伤图片覆盖（对应 C++ SetImageOverride）
        if zombie_type == ZombieType::Catapult {
            if body_damage_index == 2 || zombie_phase == ZombiePhase::Dying {
                // [TRANSLATION_NOTE]: C++ SetImageOverride("Zombie_catapult_pole", ...WITHBALL/WITHOUT) — 图片未接入
            } else if summon_counter == 0 {
                // [TRANSLATION_NOTE]: C++ SetImageOverride("Zombie_catapult_pole", IMAGE_REANIM_ZOMBIE_CATAPULT_POLE) — 图片未接入
            }
        }

        // 计算绘制位置偏移（a_draw_pos 已在 body 借用前预计算）
        let mut an_offset_x = a_draw_pos.image_offset_x + 15.0;
        let mut an_offset_y = a_draw_pos.image_offset_y + a_draw_pos.body_y - 28.0 + 20.0;
        if (zombie_type == ZombieType::Zamboni || zombie_type == ZombieType::Catapult)
            && zombie_phase != ZombiePhase::Burned
        {
            if zombie_phase == ZombiePhase::Dying {
                // [TRANSLATION_NOTE]: C++ PvzpAnimateCurveFloatTime(0.7,1.0,animTime,0,1,EASE_OUT) 抖动未接入
                an_offset_x += crate::framework::common::rand_float(1.0) - 0.5;
                an_offset_y += crate::framework::common::rand_float(1.0) - 0.5;
            } else if body_health < 200 {
                an_offset_x += crate::framework::common::rand_float(2.0) - 1.0;
                an_offset_y += crate::framework::common::rand_float(2.0) - 1.0;
            }
        }
        if zombie_type == ZombieType::Football && scale_zombie < 1.0 {
            an_offset_y += 20.0 - scale_zombie * 20.0;
        }

        // 反向走路（对应 C++ anOpposite）
        let mut an_opposite = false;
        if is_walking_backwards {
            an_opposite = true;
        }
        if zombie_type == ZombieType::Dancer || zombie_type == ZombieType::BackupDancer {
            an_opposite = false;
            if zombie_phase == ZombiePhase::DancerDancingIn && !is_eating {
                an_opposite = true;
            }
            if mind_controlled {
                an_opposite = !an_opposite;
            }
        }
        if an_opposite {
            an_offset_x += 90.0 * scale_zombie;
        }

        // [TRANSLATION_NOTE]: C++ 中 mOverlayMatrix 设置/PropogateColorToAttachments —
        // Rust Reanimation 有 m_overlay_matrix 字段但该处未用矩阵，以 override_scale/set_position 近似
        body.override_scale(scale_zombie, scale_zombie);
        body.set_position(
            an_offset_x + 30.0 - scale_zombie * 30.0,
            an_offset_y + 120.0 - scale_zombie * 120.0,
        );

        // Mowered reanim 同步（对应 C++ mMoweredReanimID 部分）
        // [TRANSLATION_NOTE]: Rust 侧 mowered_reanim_id 的 Update/overlay 同步未接入

        body.update();
    }

    /// Boss 重动画设置（对应 C++ BossSetupReanim）
    pub fn boss_setup_reanim(&mut self) {
        let a_body_reanim_id = self.body_reanim_id;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(body) = app.reanimation_get_mut(a_body_reanim_id) {
                // C++: RENDER_GROUP_BOSS_BACK_LEG=4 / FRONT_LEG=5 / BACK_ARM=6
                body.assign_render_group_to_prefix("Boss_innerleg", 4);
                body.assign_render_group_to_prefix("Boss_outerleg", 5);
                body.assign_render_group_to_prefix("Boss_body2", 5);
                body.assign_render_group_to_prefix("Boss_innerarm", 6);
                body.assign_render_group_to_prefix("Boss_RV", 6);
            }
        }

        // C++: 创建 Boss 驾驶员头部动画
        let a_head_ptr = self.base.get_app_mut().and_then(|app| {
            app.add_reanimation(0.0, 0.0, 0, ReanimationType::BossDriver as i32)
        });
        if let Some(ptr) = a_head_ptr {
            if let Some(app) = self.base.get_app_mut() {
                let a_head_id = app.reanimation_get_id(ptr);
                if let Some(head) = app.reanimation_get_mut(a_head_id) {
                    head.play_reanim("anim_idle", ReanimLoopType::Loop, 0, 18.0);
                }
                self.special_head_reanim_id = a_head_id;
            }
        }

        // 对应 C++ Zombie.cpp:10517-10518:
        // aTrackInstance = GetTrackInstanceByName("Boss_head2")
        // AttachReanim(aTrackInstance->mAttachmentID, aHeadReanim, 28.0f, -84.0f)
        if let Some(app) = self.base.get_app_mut() {
            // 先取驾驶员头部 reanim 指针（special_head_reanim_id 对应 aHeadReanim）
            let a_head_reanim_ptr: *mut crate::todlib::reanimator::Reanimation =
                app.reanimation_get_mut(self.special_head_reanim_id)
                    .map_or(std::ptr::null_mut(), |r| r as *mut crate::todlib::reanimator::Reanimation);
            if let Some(body) = app.reanimation_get_mut(a_body_reanim_id) {
                if let Some(track_instance) = body.get_track_instance_by_name("Boss_head2") {
                    let mut a_attachment_id = crate::lawn::game_enums::ATTACHMENTID_NULL;
                    let _ = crate::todlib::attachment::attach_reanim(
                        &mut a_attachment_id,
                        a_head_reanim_ptr as *mut std::ffi::c_void,
                        28.0,
                        -84.0,
                    );
                    track_instance.m_attachment_id = a_attachment_id;
                }
                body.m_frame_base_pose = 0;
            }
        }
    }

    /// 设置僵尸旗帜重动画（对应 C++ SetupZombatarFlagReanim）
    pub fn setup_zombatar_flag_reanim(&mut self, record_index: i32) {
        if record_index < 0 {
            return;
        }
        let record = self.base.get_app().and_then(|app| {
            app.player_info.as_ref().map(|p| p.m_zombatar_data.clone())
        });
        if let Some(data) = record {
            if data.is_empty() {
                return;
            }
            let offset = (record_index as usize) * 0x48usize; // ZOMBATAR_RECORD_SIZE
            if offset < data.len() {
                let a_record = &data[offset..];
                self.apply_zombatar_head(a_record);
            }
        }
    }

    /// 安装门板手臂（对应 C++ SetupDoorArms，静态方法）
    pub fn setup_door_arms(reanim: *mut Reanimation, show: bool) {
        if reanim.is_null() {
            return;
        }
        // RENDER_GROUP_NORMAL=0, RENDER_GROUP_HIDDEN=-1
        let a_arm_group = if show { -1 } else { 0 };
        let a_door_group = if show { 0 } else { -1 };
        unsafe {
            let r = &mut *reanim;
            r.assign_render_group_to_prefix("Zombie_outerarm_hand", a_arm_group);
            r.assign_render_group_to_prefix("Zombie_outerarm_lower", a_arm_group);
            r.assign_render_group_to_prefix("Zombie_outerarm_upper", a_arm_group);
            r.assign_render_group_to_prefix("anim_innerarm", a_arm_group);
            r.assign_render_group_to_prefix("Zombie_outerarm_screendoor", a_door_group);
            r.assign_render_group_to_prefix("Zombie_innerarm_screendoor", a_door_group);
            r.assign_render_group_to_prefix("Zombie_innerarm_screendoor_hand", a_door_group);
        }
    }

    /// 设置重动画层（对应 C++ SetupReanimLayers，静态方法）
    pub fn setup_reanim_layers(reanim: *mut Reanimation, zombie_type: ZombieType) {
        if reanim.is_null() {
            return;
        }
        unsafe {
            let r = &mut *reanim;
            // 默认隐藏所有可装备层
            r.assign_render_group_to_prefix("anim_cone", -1);
            r.assign_render_group_to_prefix("anim_bucket", -1);
            r.assign_render_group_to_prefix("anim_screendoor", -1);
            r.assign_render_group_to_prefix("Zombie_flaghand", -1);
            r.assign_render_group_to_prefix("Zombie_duckytube", -1);
            r.assign_render_group_to_prefix("anim_tongue", -1);
            r.assign_render_group_to_prefix("Zombie_mustache", -1);
            Self::setup_door_arms(reanim, false);

            match zombie_type {
                ZombieType::TrafficCone => {
                    r.assign_render_group_to_prefix("anim_cone", 0);
                    r.assign_render_group_to_prefix("anim_hair", -1);
                }
                ZombieType::Pail => {
                    r.assign_render_group_to_prefix("anim_bucket", 0);
                    r.assign_render_group_to_prefix("anim_hair", -1);
                }
                ZombieType::Door => {
                    Self::setup_door_arms(reanim, true);
                }
                ZombieType::Newspaper => {
                    r.assign_render_group_to_prefix("Zombie_paper_paper", -1);
                }
                ZombieType::Flag => {
                    r.assign_render_group_to_prefix("anim_innerarm", -1);
                    r.assign_render_group_to_track("Zombie_flaghand", 0);
                    r.assign_render_group_to_track("Zombie_innerarm_screendoor", 0);
                }
                ZombieType::DuckyTube => {
                    r.assign_render_group_to_prefix("Zombie_duckytube", 0);
                }
                _ => {}
            }
        }
    }

    /// 显示门板手臂（对应 C++ ShowDoorArms）
    pub fn show_door_arms(&mut self, show: bool) {
        let reanim_id = self.body_reanim_id;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(body) = app.reanimation_get_mut(reanim_id) {
                Self::setup_door_arms(body, show);
            }
        }
        if !self.has_arm {
            self.reanim_show_prefix("Zombie_outerarm_lower", -1);
            self.reanim_show_prefix("Zombie_outerarm_hand", -1);
        }
    }

    /// 忽略裁剪矩形（对应 C++ ReanimIgnoreClipRect）
    pub fn reanim_ignore_clip_rect(&mut self, track_name: &str, ignore_clip_rect: bool) {
        // C++: ReanimationGet(mBodyReanimID)；遍历定义轨道，名字匹配（strcasecmp）的轨道实例置 mIgnoreClipRect
        if let Some(app) = self.base.get_app_mut() {
            if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                if let Some(def_ptr) = a_body_reanim.m_definition {
                    unsafe {
                        let a_def_ref = &*def_ptr;
                        for (i, a_track) in a_def_ref.m_tracks.iter().enumerate() {
                            if a_track.m_name.eq_ignore_ascii_case(track_name) {
                                if let Some(a_track_instance) = a_body_reanim.m_track_instances.get_mut(i) {
                                    a_track_instance.m_ignore_clip_rect = ignore_clip_rect;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// 重新启用裁剪（对应 C++ ReanimReenableClipping）
    pub fn reanim_reenable_clipping(&mut self) {
        // C++: 全部轨道实例的 mIgnoreClipRect 置 false
        if let Some(app) = self.base.get_app_mut() {
            if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                for a_track_instance in &mut a_body_reanim.m_track_instances {
                    a_track_instance.m_ignore_clip_rect = false;
                }
            }
        }
    }

    /// 开始行走动画（对应 C++ StartWalkAnim）
    pub fn start_walk_anim(&mut self, blend_time: i32) {
        // C++: TryToGet(mBodyReanimID) 为空直接返回
        let a_body_exists = self.base.get_app().map_or(false, |app| {
            app.reanimation_get(self.body_reanim_id).is_some()
        });
        if !a_body_exists {
            return;
        }

        self.pick_random_speed();
        if self.zombie_phase == ZombiePhase::LadderCarrying {
            self.play_zombie_reanim("anim_ladderwalk", ReanimLoopType::Loop, blend_time, 0.0);
        } else if self.zombie_phase == ZombiePhase::NewspaperMad {
            self.play_zombie_reanim("anim_walk_nopaper", ReanimLoopType::Loop, blend_time, 0.0);
        } else if self.in_pool && self.zombie_height != ZombieHeight::InToPool && self.zombie_height != ZombieHeight::OutOfPool
            && self.base.get_app().map_or(false, |app| {
                app.reanimation_get(self.body_reanim_id).map_or(false, |r| r.track_exists("anim_swim"))
            })
        {
            self.play_zombie_reanim("anim_swim", ReanimLoopType::Loop, blend_time, 0.0);
        } else if (self.zombie_type == ZombieType::Normal || self.zombie_type == ZombieType::TrafficCone || self.zombie_type == ZombieType::Pail)
            && self.base.get_board().map_or(false, |b| b.m_dance_mode)
        {
            self.play_zombie_reanim("anim_dance", ReanimLoopType::Loop, blend_time, 0.0);
        } else {
            let mut a_walk_anim_variant = RandRange(2);
            if self.zombie_type == ZombieType::PeaHead {
                a_walk_anim_variant = 0;
            }
            if self.zombie_type == ZombieType::Flag {
                a_walk_anim_variant = 0;
            }

            if a_walk_anim_variant == 0 && self.base.get_app().map_or(false, |app| {
                app.reanimation_get(self.body_reanim_id).map_or(false, |r| r.track_exists("anim_walk2"))
            }) {
                self.play_zombie_reanim("anim_walk2", ReanimLoopType::Loop, blend_time, 0.0);
            } else if self.base.get_app().map_or(false, |app| {
                app.reanimation_get(self.body_reanim_id).map_or(false, |r| r.track_exists("anim_walk"))
            }) {
                self.play_zombie_reanim("anim_walk", ReanimLoopType::Loop, blend_time, 0.0);
            }
        }
    }

    /// 启用胡子模式（对应 C++ EnableMustache）
    pub fn enable_mustache(&mut self, enable: bool) {
        use crate::todlib::reanim_loader::{reanimator_get_image, resolve_reanim_image_name};

        if self.from_wave == Zombie::ZOMBIE_WAVE_UI {
            return;
        }
        if !self.has_head || Zombie::is_zombotany(self.zombie_type) {
            return;
        }

        // C++: TryToGet(mBodyReanimID) 为空或 TrackExists("Zombie_mustache") 为假直接返回
        let a_track_exists = self.base.get_app().map_or(false, |app| {
            app.reanimation_get(self.body_reanim_id)
                .map_or(false, |r| r.track_exists("Zombie_mustache"))
        });
        if !a_track_exists {
            return;
        }

        if enable {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                    a_body_reanim.assign_render_group_to_prefix(
                        "Zombie_mustache",
                        crate::todlib::reanimator::RENDER_GROUP_NORMAL,
                    );

                    // C++: RandRangeInt(1, 3)
                    match 1 + RandRange(3) {
                        1 => a_body_reanim.set_image_override("Zombie_mustache", std::ptr::null_mut()),
                        2 => a_body_reanim.set_image_override(
                            "Zombie_mustache",
                            reanimator_get_image(resolve_reanim_image_name("IMAGE_REANIM_ZOMBIE_MUSTACHE2"))
                                .unwrap_or(std::ptr::null_mut()),
                        ),
                        3 => a_body_reanim.set_image_override(
                            "Zombie_mustache",
                            reanimator_get_image(resolve_reanim_image_name("IMAGE_REANIM_ZOMBIE_MUSTACHE3"))
                                .unwrap_or(std::ptr::null_mut()),
                        ),
                        _ => {}
                    }
                }
            }
        } else {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                    a_body_reanim.assign_render_group_to_prefix(
                        "Zombie_mustache",
                        crate::todlib::reanimator::RENDER_GROUP_HIDDEN,
                    );
                }
            }
        }
    }

    /// 启用未来模式（对应 C++ EnableFuture）
    pub fn enable_future(&mut self, enable: bool) {
        use crate::todlib::reanim_loader::{reanimator_get_image, resolve_reanim_image_name};

        if self.from_wave == Zombie::ZOMBIE_WAVE_UI || Zombie::is_zombotany(self.zombie_type) {
            return;
        }

        // C++: TryToGet(mBodyReanimID) 为空或 mReanimationType != REANIM_ZOMBIE 直接返回
        let a_is_plain_zombie = self.base.get_app().map_or(false, |app| {
            app.reanimation_get(self.body_reanim_id)
                .map_or(false, |r| r.reanim_type == ReanimationType::Zombie)
        });
        if !a_is_plain_zombie {
            return;
        }

        if enable {
            // C++: static_cast<unsigned int>(mBoard->ZombieGetID(this)) % 4
            let a_id = self.base.get_board().map_or(0, |b| b.zombie_get_id(self) % 4);
            let mut a_image = std::ptr::null_mut();
            match a_id {
                0 => a_image = reanimator_get_image(resolve_reanim_image_name("IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES1"))
                    .unwrap_or(std::ptr::null_mut()),
                1 => a_image = reanimator_get_image(resolve_reanim_image_name("IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES2"))
                    .unwrap_or(std::ptr::null_mut()),
                2 => a_image = reanimator_get_image(resolve_reanim_image_name("IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES3"))
                    .unwrap_or(std::ptr::null_mut()),
                3 => a_image = reanimator_get_image(resolve_reanim_image_name("IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES4"))
                    .unwrap_or(std::ptr::null_mut()),
                _ => debug_assert!(false), // C++: PVZP_ASSERT(false)
            }

            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                    a_body_reanim.set_image_override("anim_head1", a_image);
                }
            }
        } else {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                    a_body_reanim.set_image_override("anim_head1", std::ptr::null_mut());
                }
            }
        }
    }

    /// 播放僵尸出现音效（对应 C++ PlayZombieAppearSound）
    pub fn play_zombie_appear_sound(&mut self) {
        let foley = match self.zombie_type {
            ZombieType::DolphinRider => crate::todlib::tod_foley::FoleyType::DolphinAppears,
            ZombieType::Balloon => crate::todlib::tod_foley::FoleyType::BalloonInflate,
            ZombieType::Zamboni => crate::todlib::tod_foley::FoleyType::Zamboni,
            _ => return,
        };
        if let Some(app) = self.base.app {
            unsafe { (*app).play_foley(foley as i32); }
        }
    }

    /// 拖入水下（对应 C++ DragUnder）
    pub fn drag_under(&mut self) {
        self.zombie_height = ZombieHeight::DraggedUnder;
        self.stop_eating();
        self.reanim_reenable_clipping();
    }

    /// 启用舞蹈（对应 C++ EnableDance）
    pub fn enable_dance(&mut self) {
        if !self.is_on_board() {
            return;
        }
        if self.zombie_not_walking() || self.is_dead_or_dying() {
            return;
        }
        if self.zombie_type == ZombieType::Normal
            || self.zombie_type == ZombieType::TrafficCone
            || self.zombie_type == ZombieType::Pail
        {
            self.start_walk_anim(0);
        }
    }

    /// 泳池水花（对应 C++ PoolSplash）
    pub fn pool_splash(&mut self, in_to_pool_sound: bool) {
        let mut offset_x = 23.0f32;
        let mut offset_y = 78.0f32;
        if self.zombie_phase == ZombiePhase::SnorkelWalkingInPool {
            offset_x -= 37.0;
            offset_y += 8.0;
        }
        if let Some(app) = self.base.app {
            unsafe {
                (*app).add_reanimation(self.pos_x + offset_x, self.pos_y + offset_y, self.base.render_order + 1, crate::lawn::game_enums::ReanimationType::Splash as i32);
                (*app).add_tod_particle(self.pos_x + offset_x + 37.0, self.pos_y + offset_y + 42.0, self.base.render_order + 1, crate::lawn::game_enums::ParticleEffect::PlantingPool as i32);
                let foley = if in_to_pool_sound {
                    crate::todlib::tod_foley::FoleyType::ZombieSplash
                } else {
                    crate::todlib::tod_foley::FoleyType::PlantWater
                };
                (*app).play_foley(foley as i32);
            }
        }
    }

    /// 基于行获取 Y 位置（对应 C++ GetPosYBasedOnRow）
pub fn get_pos_y_based_on_row(&mut self, row: i32) -> f32 {
        if !self.is_on_board() {
            return 0.0;
        }

        if self.on_high_ground {
            if self.altitude < HIGH_GROUND_HEIGHT {
                self.zombie_height = ZombieHeight::UpToHighGround;
            }
            self.on_high_ground = true;
        }

        let mut a_pos_y = match self.base.get_board() {
            Some(board) => board.get_pos_y_based_on_row(self.pos_x + 40.0, row) - 30.0,
            None => 0.0,
        };
        if self.zombie_type == ZombieType::Balloon {
            a_pos_y -= 30.0;
        } else if self.zombie_type == ZombieType::Pogo {
            a_pos_y -= 16.0;
        }

        a_pos_y
    }

    /// 选择蹦极僵尸目标（对应 C++ PickBungeeZombieTarget）
    pub fn pick_bungee_zombie_target(&mut self, column: i32) {
        let mut allow_sunflower_target = true;
        if let Some(board) = self.base.get_board() {
            if self.count_bungees_targeting_sunflowers() == board.count_sunflowers() - 1 {
                allow_sunflower_target = false;
            }
        }

        // 先计算候选格子（需要借用 board，退出借用后再修改 self）
        let (picks_result, dead_flag) = {
            let mut picks = Vec::new();
            if let Some(board) = self.base.get_board() {
                for x in 0..crate::lawn::board::MAX_GRID_SIZE_X as i32 {
                    if column == -1 || column == x {
                        for y in 0..crate::lawn::board::MAX_GRID_SIZE_Y as i32 {
                            let mut a_weight = 1;
                            if board.get_grave_stone_at(x, y).is_some()
                                || board.grid_square_type[y as usize][x as usize] == GridSquareType::Dirt
                            {
                                continue;
                            }

                            if let Some(a_plant) = board.get_top_plant_at_any(x, y) {
                                if !allow_sunflower_target && a_plant.makes_sun() {
                                    continue;
                                }
                                if a_plant.seed_type == SeedType::Gravebuster || a_plant.seed_type == SeedType::Cobcannon {
                                    continue;
                                }
                                a_weight = 10000;
                            }

                            if !board.bungee_is_targeting_cell(x, y) {
                                picks.push(crate::todlib::tod_common::TodWeightedGridArray { x, y, weight: a_weight });
                            }
                        }
                    }
                }
            }
            let dead_flag = picks.is_empty();
            (picks, dead_flag)
        };

        if dead_flag {
            self.die_no_loot();
            return;
        }

        // 从加权数组中随机选择（此时不再借用 board）
        let pick_count = picks_result.len();
        let chosen = crate::todlib::tod_common::tod_pick_from_weighted_grid_array(
            &mut picks_result.clone(), pick_count,
        );
        if let Some(idx) = chosen {
            self.target_col = picks_result[idx].x;
            let chosen_y = picks_result[idx].y;
            let chosen_x = picks_result[idx].x;
            self.set_row(chosen_y);
            if let Some(board) = self.base.get_board() {
                self.pos_x = board.grid_to_pixel_x(chosen_x, self.base.row) as f32;
            }
            self.pos_y = self.get_pos_y_based_on_row(self.base.row);
        }
    }

    /// 统计正在瞄准向日葵的蹦极僵尸数量（对应 C++ CountBungeesTargetingSunFlowers）
    pub fn count_bungees_targeting_sunflowers(&self) -> i32 {
        let mut count = 0;
        if let Some(board) = self.base.get_board() {
            for zombie in &board.zombies {
                if zombie.dead {
                    continue;
                }
                if !zombie.is_dead_or_dying() && zombie.zombie_type == ZombieType::Bungee && zombie.target_col != -1 {
                    if let Some(a_plant) = board.get_top_plant_at_any(zombie.target_col, zombie.base.row) {
                        if a_plant.makes_sun() {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    /// 蹦极投放僵尸（对应 C++ BungeeDropZombie）
    pub fn bungee_drop_zombie(&mut self, dropped_zombie: &mut Zombie, grid_x: i32, grid_y: i32) {
        self.target_col = grid_x;
        self.set_row(grid_y);
        if let Some(board) = self.base.get_board() {
            self.pos_x = board.grid_to_pixel_x(self.target_col, self.base.row) as f32;
        }
        self.pos_y = self.get_pos_y_based_on_row(self.base.row);
        self.play_zombie_reanim("anim_raise", ReanimLoopType::PlayOnceAndHold, 0, 36.0);
        // mRelatedZombieID = mBoard->ZombieGetID(theDroppedZombie) — ZombieGetID 未实现，暂用占位
        // self.related_zombie_id = 0;

        dropped_zombie.pos_x = self.pos_x - 15.0;
        dropped_zombie.set_row(grid_y);
        dropped_zombie.pos_y = dropped_zombie.get_pos_y_based_on_row(grid_y);
        dropped_zombie.zombie_height = ZombieHeight::GettingBungeeDropped;
        dropped_zombie.play_zombie_reanim("anim_idle", ReanimLoopType::Loop, 0, 0.0);
        dropped_zombie.base.render_order = self.base.render_order + 1;
    }

    /// 蹦极偷取目标（对应 C++ BungeeStealTarget）
    pub fn bungee_steal_target(&mut self) {
        self.play_zombie_reanim("anim_grab", ReanimLoopType::PlayOnceAndHold, 20, 24.0);
        if let Some(board) = self.base.get_board() {
            if let Some(a_plant) = board.get_top_plant_at_any(self.target_col, self.base.row) {
                if a_plant.seed_type != SeedType::Cobcannon && a_plant.seed_type != SeedType::Gravebuster {
                    // mTargetPlantID = (PlantID)mBoard->mPlants.DataArrayGetID(aPlant)
                    // aPlant->mOnBungeeState = GETTING_GRABBED_BY_BUNGEE
                    self.base.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_PROJECTILE, self.base.row, 0);
                }
            }
        }
    }

    /// 蹦极提起目标（对应 C++ BungeeLiftTarget）
    pub fn bungee_lift_target(&mut self) {
        self.play_zombie_reanim("anim_raise", ReanimLoopType::PlayOnceAndHold, 0, 36.0);
        // 依赖底层系统
    }

    /// 蹦极着陆（对应 C++ BungeeLanding）
    pub fn bungee_landing(&mut self) {
        if self.zombie_phase == ZombiePhase::BungeeDiving && self.altitude < 1500.0 {
            self.zombie_phase = ZombiePhase::BungeeDivingScreaming;
        }

        if self.altitude > 40.0 {
            return;
        }

        if let Some(board) = self.base.get_board() {
            if let Some(_a_plant) = board.find_umbrella_plant(self.target_col, self.base.row) {
                // 荷叶伞弹开蹦极
                self.zombie_phase = ZombiePhase::BungeeRising;
                self.base.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_TOP, 0, 1);
                self.hit_umbrella = true;
                return;
            }
        }

        if self.altitude > 0.0 {
            return;
        }

        self.altitude = 0.0;
        // 依赖底层系统
        // 若携带僵尸则释放它，否则进入底部状态
        self.zombie_phase = ZombiePhase::BungeeAtBottom;
        self.phase_counter = 300;
        self.play_zombie_reanim("anim_idle", ReanimLoopType::Loop, 5, 24.0);
    }

    /// 更新蹦极僵尸（对应 C++ UpdateZombieBungee）
    pub fn update_zombie_bungee(&mut self) {
        if self.is_dead_or_dying() || self.is_immobilized() {
            return;
        }

        if self.zombie_phase == ZombiePhase::BungeeDiving || self.zombie_phase == ZombiePhase::BungeeDivingScreaming {
            self.altitude -= 8.0;
            self.bungee_landing();
        } else if self.zombie_phase == ZombiePhase::BungeeAtBottom {
            if self.phase_counter <= 0 {
                self.bungee_steal_target();
                self.zombie_phase = ZombiePhase::BungeeGrabbing;
            }
        } else if self.zombie_phase == ZombiePhase::BungeeGrabbing {
            // Reanimation mLoopCount > 0 检查 — Reanimation 系统暂未实现
            self.bungee_lift_target();
            self.zombie_phase = ZombiePhase::BungeeRising;
        } else if self.zombie_phase == ZombiePhase::BungeeHitOuchy {
            if self.phase_counter <= 0 {
                self.die_with_loot();
            }
        } else if self.zombie_phase == ZombiePhase::BungeeRising {
            self.altitude += 8.0;
            if self.altitude >= 600.0 {
                self.die_no_loot();
            }
        } else if self.zombie_phase == ZombiePhase::BungeeCutscene {
            self.altitude = crate::todlib::tod_common::tod_animate_curve(200, 0, self.phase_counter, 40, 0, TodCurves::SinWave) as f32;
            if self.phase_counter <= 0 {
                self.phase_counter = 200;
            }
        }

        self.base.x = self.pos_x as i32;
        self.base.y = self.pos_y as i32;
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
    ZombieDefinition { zombie_type: ZombieType::Normal, reanimation_type: ReanimationType::Zombie, zombie_value: 1, starting_level: 1, first_allowed_wave: 1, pick_weight: 4000, zombie_name: "ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Flag, reanimation_type: ReanimationType::Zombie, zombie_value: 1, starting_level: 1, first_allowed_wave: 1, pick_weight: 0, zombie_name: "FLAG_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::TrafficCone, reanimation_type: ReanimationType::Zombie, zombie_value: 2, starting_level: 3, first_allowed_wave: 1, pick_weight: 4000, zombie_name: "CONEHEAD_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Polevaulter, reanimation_type: ReanimationType::Polevaulter, zombie_value: 2, starting_level: 6, first_allowed_wave: 5, pick_weight: 2000, zombie_name: "POLE_VAULTING_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Pail, reanimation_type: ReanimationType::Zombie, zombie_value: 4, starting_level: 8, first_allowed_wave: 1, pick_weight: 3000, zombie_name: "BUCKETHEAD_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Newspaper, reanimation_type: ReanimationType::ZombieNewspaper, zombie_value: 2, starting_level: 11, first_allowed_wave: 1, pick_weight: 1000, zombie_name: "NEWSPAPER_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Door, reanimation_type: ReanimationType::Zombie, zombie_value: 4, starting_level: 13, first_allowed_wave: 5, pick_weight: 3500, zombie_name: "SCREEN_DOOR_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Football, reanimation_type: ReanimationType::ZombieFootball, zombie_value: 7, starting_level: 16, first_allowed_wave: 5, pick_weight: 2000, zombie_name: "FOOTBALL_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Dancer, reanimation_type: ReanimationType::Dancer, zombie_value: 5, starting_level: 18, first_allowed_wave: 5, pick_weight: 1000, zombie_name: "DANCING_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::BackupDancer, reanimation_type: ReanimationType::BackupDancer, zombie_value: 1, starting_level: 18, first_allowed_wave: 1, pick_weight: 0, zombie_name: "BACKUP_DANCER" },
    ZombieDefinition { zombie_type: ZombieType::DuckyTube, reanimation_type: ReanimationType::Zombie, zombie_value: 1, starting_level: 21, first_allowed_wave: 5, pick_weight: 0, zombie_name: "DUCKY_TUBE_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Snorkel, reanimation_type: ReanimationType::Snorkel, zombie_value: 3, starting_level: 23, first_allowed_wave: 10, pick_weight: 2000, zombie_name: "SNORKEL_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Zamboni, reanimation_type: ReanimationType::ZombieZamboni, zombie_value: 7, starting_level: 26, first_allowed_wave: 10, pick_weight: 2000, zombie_name: "ZOMBONI" },
    ZombieDefinition { zombie_type: ZombieType::Bobsled, reanimation_type: ReanimationType::Bobsled, zombie_value: 3, starting_level: 26, first_allowed_wave: 10, pick_weight: 2000, zombie_name: "ZOMBIE_BOBSLED_TEAM" },
    ZombieDefinition { zombie_type: ZombieType::DolphinRider, reanimation_type: ReanimationType::ZombieDolphinrider, zombie_value: 3, starting_level: 28, first_allowed_wave: 10, pick_weight: 1500, zombie_name: "DOLPHIN_RIDER_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::JackInTheBox, reanimation_type: ReanimationType::Jackinthebox, zombie_value: 3, starting_level: 31, first_allowed_wave: 10, pick_weight: 1000, zombie_name: "JACK_IN_THE_BOX_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Balloon, reanimation_type: ReanimationType::Balloon, zombie_value: 2, starting_level: 33, first_allowed_wave: 10, pick_weight: 2000, zombie_name: "BALLOON_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Digger, reanimation_type: ReanimationType::Digger, zombie_value: 4, starting_level: 36, first_allowed_wave: 10, pick_weight: 1000, zombie_name: "DIGGER_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Pogo, reanimation_type: ReanimationType::Pogo, zombie_value: 4, starting_level: 38, first_allowed_wave: 10, pick_weight: 1000, zombie_name: "POGO_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Yeti, reanimation_type: ReanimationType::Yeti, zombie_value: 4, starting_level: 40, first_allowed_wave: 1, pick_weight: 1, zombie_name: "ZOMBIE_YETI" },
    ZombieDefinition { zombie_type: ZombieType::Bungee, reanimation_type: ReanimationType::Bungee, zombie_value: 3, starting_level: 41, first_allowed_wave: 10, pick_weight: 1000, zombie_name: "BUNGEE_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Ladder, reanimation_type: ReanimationType::Ladder, zombie_value: 4, starting_level: 43, first_allowed_wave: 10, pick_weight: 1000, zombie_name: "LADDER_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Catapult, reanimation_type: ReanimationType::Catapult, zombie_value: 5, starting_level: 46, first_allowed_wave: 10, pick_weight: 1500, zombie_name: "CATAPULT_ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::Gargantuar, reanimation_type: ReanimationType::Gargantuar, zombie_value: 10, starting_level: 48, first_allowed_wave: 15, pick_weight: 1500, zombie_name: "GARGANTUAR" },
    ZombieDefinition { zombie_type: ZombieType::Imp, reanimation_type: ReanimationType::Imp, zombie_value: 10, starting_level: 48, first_allowed_wave: 1, pick_weight: 0, zombie_name: "IMP" },
    ZombieDefinition { zombie_type: ZombieType::Boss, reanimation_type: ReanimationType::Boss, zombie_value: 10, starting_level: 50, first_allowed_wave: 1, pick_weight: 0, zombie_name: "BOSS" },
    ZombieDefinition { zombie_type: ZombieType::PeaHead, reanimation_type: ReanimationType::Zombie, zombie_value: 1, starting_level: 99, first_allowed_wave: 1, pick_weight: 4000, zombie_name: "ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::WallnutHead, reanimation_type: ReanimationType::Zombie, zombie_value: 4, starting_level: 99, first_allowed_wave: 1, pick_weight: 3000, zombie_name: "ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::JalapenoHead, reanimation_type: ReanimationType::Zombie, zombie_value: 3, starting_level: 99, first_allowed_wave: 10, pick_weight: 1000, zombie_name: "ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::GatlingHead, reanimation_type: ReanimationType::Zombie, zombie_value: 3, starting_level: 99, first_allowed_wave: 10, pick_weight: 2000, zombie_name: "ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::SquashHead, reanimation_type: ReanimationType::Zombie, zombie_value: 3, starting_level: 99, first_allowed_wave: 10, pick_weight: 2000, zombie_name: "ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::TallnutHead, reanimation_type: ReanimationType::Zombie, zombie_value: 4, starting_level: 99, first_allowed_wave: 10, pick_weight: 2000, zombie_name: "ZOMBIE" },
    ZombieDefinition { zombie_type: ZombieType::RedeEyeGargantuar, reanimation_type: ReanimationType::Gargantuar, zombie_value: 10, starting_level: 48, first_allowed_wave: 15, pick_weight: 6000, zombie_name: "REDEYED_GARGANTUAR" },
];

/// 僵尸类型在各关卡的允许表（对应 C++ gZombieAllowedLevels，Challenge.cpp:75-293）
/// 索引 = ZombieType 枚举值（0..32），每项 50 个关卡（level-1 索引，clamp 到 0..49）
pub static G_ZOMBIE_ALLOWED_LEVELS: [[u8; 50]; NUM_ZOMBIE_TYPES as usize] = [
    // ZOMBIE_NORMAL: 全关卡允许
    [1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1],
    // ZOMBIE_FLAG
    [1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1],
    // ZOMBIE_TRAFFIC_CONE
    [0,0,1,1,1,1,1,1,1,1, 0,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1],
    // ZOMBIE_POLEVAULTER
    [0,0,0,0,0,1,1,0,1,1, 0,0,0,1,1,0,0,0,0,0, 0,0,0,1,0,0,0,0,1,0, 0,0,0,0,0,0,0,0,0,0, 0,1,0,0,0,0,0,0,0,0],
    // ZOMBIE_PAIL
    [0,0,0,0,0,0,0,1,1,1, 0,1,0,0,1,0,0,0,0,0, 0,1,0,1,0,0,1,0,1,1, 0,0,0,0,0,0,1,0,1,1, 0,1,0,0,1,0,0,0,1,1],
    // ZOMBIE_NEWSPAPER
    [0,0,0,0,0,0,0,0,0,0, 1,1,0,0,1,0,0,0,0,0, 0,1,0,1,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0],
    // ZOMBIE_DOOR
    [0,0,0,0,0,0,0,0,0,0, 0,0,1,1,0,0,1,0,1,1, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0],
    // ZOMBIE_FOOTBALL
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,1,1,0,0,1, 0,1,0,0,1,0,0,0,0,0, 0,1,0,0,0,0,0,0,0,0, 0,0,0,1,0,0,0,0,0,0],
    // ZOMBIE_DANCER
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,1,1,1, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0],
    // ZOMBIE_BACKUP_DANCER
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,1,1,1, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0],
    // ZOMBIE_DUCKY_TUBE
    [0; 50],
    // ZOMBIE_SNORKEL
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,1,1,1,0,1,0,0,1, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0],
    // ZOMBIE_ZAMBONI
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,1,1,0,1,1, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0],
    // ZOMBIE_BOBSLED
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,1,1,0,1,1, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0],
    // ZOMBIE_DOLPHIN_RIDER
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,1,1,1, 0,0,0,1,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0],
    // ZOMBIE_JACK_IN_THE_BOX
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 1,1,0,0,0,0,1,0,0,1, 0,0,0,0,0,0,0,0,1,1],
    // ZOMBIE_BALLOON
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,1,1,0,0,0,0,1,1, 0,0,0,0,0,0,0,0,0,0],
    // ZOMBIE_DIGGER
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,1,1,0,0,1, 0,0,0,0,0,0,0,0,0,0],
    // ZOMBIE_POGO
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,1,1,1, 0,0,0,1,0,0,0,0,0,0],
    // ZOMBIE_YETI
    [0; 50],
    // ZOMBIE_BUNGEE
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 1,1,0,0,0,0,1,0,1,1],
    // ZOMBIE_LADDER
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,1,1,1,0,1,0,1,1],
    // ZOMBIE_CATAPULT
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,1,1,0,1,1],
    // ZOMBIE_GARGANTUAR
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,1,1,1],
    // ZOMBIE_IMP
    [0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,1,1,1],
    // ZOMBIE_BOSS
    [0; 50],
    // ZOMBIE_PEA_HEAD
    [0; 50],
    // ZOMBIE_WALLNUT_HEAD
    [0; 50],
    // ZOMBIE_JALAPENO_HEAD
    [0; 50],
    // ZOMBIE_GATLING_HEAD
    [0; 50],
    // ZOMBIE_SQUASH_HEAD
    [0; 50],
    // ZOMBIE_TALLNUT_HEAD
    [0; 50],
    // ZOMBIE_REDEYE_GARGANTUAR
    [0; 50],
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
