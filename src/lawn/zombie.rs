// PvZ Portable Rust 翻译 — Zombie（僵尸类）
// 对应 C++ src/Lawn/Zombie.h / Zombie.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::lawn::plant::Plant;
use crate::todlib::reanimator::{Reanimation, ReanimationType, ReanimLoopType};
use crate::todlib::tod_particle::ParticleSystem;
use crate::todlib::attachment::Attachment;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::rect::Rect;
use crate::framework::color::Color;
use crate::framework::common::{Rand, RandRange, RandFloat};
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::board::Board;

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
                    // [TRANSLATION_NOTE]: HelmType::Bobsled 在 game_enums 中尚未定义，暂不设置
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
                // [TRANSLATION_NOTE]: HelmType::Wallnut 在 game_enums 中尚未定义
                self.helm_health = 1100;
                self.variant = false;
            }
            ZombieType::TallnutHead => {
                self.load_plain_zombie_reanim();
                self.reanim_show_prefix("anim_hair", -1);
                self.reanim_show_prefix("anim_head", -1);
                // [TRANSLATION_NOTE]: HelmType::Tallnut 在 game_enums 中尚未定义
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
        // [TRANSLATION_NOTE]: is_little_trouble_level 尚未在 LawnApp 中实现，暂不启用
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

    fn pick_random_speed(&mut self) {
        // 对应 C++ Zombie::PickRandomSpeed
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
        // [TRANSLATION_NOTE]: lawn_app::GameScenes 尚无 LevelIntro 变体，暂时跳过
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

                // [TRANSLATION_NOTE]: lawn_app::GameScenes 尚无 ZombiesWon 变体，暂时跳过
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
    fn update_mowered(&mut self) {}

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
                        board.zombies_won();
                    }
                }
            }
        }
        if self.base.x <= a_edge_x + 70 && !self.has_head {
            self.take_damage(1800, 9);
        }
    }

    /// 检查僵尸脚步声（对应 C++ CheckForZombieStep）
    pub fn check_for_zombie_step(&mut self) {
        if (self.zombie_type == ZombieType::Zamboni || self.zombie_type == ZombieType::Catapult) && !self.flat_tires {
            // CheckSquish(ATTACKTYPE_DRIVE_OVER) — stub
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

    /// 移除冰陷阱（对应 C++ RemoveIceTrap，stub）
    pub fn remove_ice_trap(&mut self) {}

    /// 移除黄油（对应 C++ RemoveButter，stub）
    pub fn remove_butter(&mut self) {}

    /// 检查进入泳池（对应 C++ CheckForPool，stub）
    pub fn check_for_pool(&mut self) {}

    /// 检查屋顶高台（对应 C++ CheckForHighGround，stub）
    pub fn check_for_high_ground(&mut self) {}

    /// 更新从墓碑升起（对应 C++ UpdateZombieRiseFromGrave，stub）
    pub fn update_zombie_rise_from_grave(&mut self) {}

    /// 更新泳池僵尸（对应 C++ UpdateZombiePool，stub）
    pub fn update_zombie_pool(&mut self) {}

    /// 更新屋顶高台僵尸（对应 C++ UpdateZombieHighGround，stub）
    pub fn update_zombie_high_ground(&mut self) {}

    /// 更新掉落僵尸（对应 C++ UpdateZombieFalling，stub）
    pub fn update_zombie_falling(&mut self) {}

    /// 更新烟囱僵尸（对应 C++ UpdateZombieChimney，stub）
    pub fn update_zombie_chimney(&mut self) {}

    /// 更新僵尸撑杆跳（对应 C++ UpdateZombiePolevaulter，stub）
    pub fn update_zombie_polevaulter(&mut self) {}

    /// 更新投石车僵尸（对应 C++ UpdateZombieCatapult，stub）
    pub fn update_zombie_catapult(&mut self) {}

    /// 更新海豚骑士（对应 C++ UpdateZombieDolphinRider，stub）
    pub fn update_zombie_dolphin_rider(&mut self) {}

    /// 更新潜水僵尸（对应 C++ UpdateZombieSnorkel，stub）
    pub fn update_zombie_snorkel(&mut self) {}

    /// 更新气球僵尸（对应 C++ UpdateZombieFlyer，stub）
    pub fn update_zombie_flyer(&mut self) {}

    /// 更新报纸僵尸（对应 C++ UpdateZombieNewspaper，stub）
    pub fn update_zombie_newspaper(&mut self) {}

    /// 更新矿工僵尸（对应 C++ UpdateZombieDigger，stub）
    pub fn update_zombie_digger(&mut self) {}

    /// 更新小丑僵尸（对应 C++ UpdateZombieJackInTheBox，stub）
    pub fn update_zombie_jack_in_the_box(&mut self) {}

    /// 更新伽刚特尔（对应 C++ UpdateZombieGargantuar，stub）
    pub fn update_zombie_gargantuar(&mut self) {}

    /// 更新雪橇僵尸（对应 C++ UpdateZombieBobsled，stub）
    pub fn update_zombie_bobsled(&mut self) {}

    /// 更新冰车僵尸（对应 C++ UpdateZamboni，stub）
    pub fn update_zamboni(&mut self) {}

    /// 更新梯子僵尸（对应 C++ UpdateLadder，stub）
    pub fn update_ladder(&mut self) {}

    /// 更新舞王僵尸（对应 C++ UpdateZombieDancer，stub）
    pub fn update_zombie_dancer(&mut self) {}

    /// 更新伴舞僵尸（对应 C++ UpdateZombieBackupDancer，stub）
    pub fn update_zombie_backup_dancer(&mut self) {}

    /// 更新小鬼僵尸（对应 C++ UpdateZombieImp，stub）
    pub fn update_zombie_imp(&mut self) {}

    /// 更新 Boss（对应 C++ UpdateBoss，stub）
    pub fn update_boss(&mut self) {}

    /// 更新蹦极僵尸（对应 C++ UpdateZombieBungee）
    fn update_zombie_bungee(&mut self) {}

    /// 更新弹簧僵尸（对应 C++ UpdateZombiePogo）
    fn update_zombie_pogo(&mut self) {}

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
        false
    }

    /// 僵尸是否不该走动（对应 C++ ZombieNotWalking）
    pub fn zombie_not_walking(&self) -> bool {
        false
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
        // TODO: 实现僵尸音效播放
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

    pub fn update_death(&mut self) {}

    /// 获取渲染位置
    pub fn get_draw_pos(&self) -> ZombieDrawPosition {
        let mut pos = ZombieDrawPosition::new();
        pos.body_y = self.pos_y;
        pos.image_offset_x = 0.0;
        pos.image_offset_y = 0.0;
        pos
    }

    // ========== 辅助函数（stub，对应 C++ Zombie 方法） ==========

    /// 加载重动画（对应 C++ LoadReanim）
    pub fn load_reanim(&mut self, _reanim_type: ReanimationType) -> Option<*mut Reanimation> { None }

    /// 加载普通僵尸重动画（对应 C++ LoadPlainZombieReanim）
    pub fn load_plain_zombie_reanim(&mut self) {}

    /// 播放僵尸动画（对应 C++ PlayZombieReanim）
    pub fn play_zombie_reanim(&mut self, _track_name: &str, _loop_type: ReanimLoopType, _blend_time: i32, _anim_rate: f32) {}

    /// 显示前缀动画轨道（对应 C++ ReanimShowPrefix）
    pub fn reanim_show_prefix(&mut self, _track_prefix: &str, _render_group: i32) {}

    /// 显示动画轨道（对应 C++ ReanimShowTrack）
    pub fn reanim_show_track(&mut self, _track_name: &str, _render_group: i32) {}

    /// 是否在棋盘上（对应 C++ IsOnBoard）
    pub fn is_on_board(&self) -> bool {
        if self.from_wave == Zombie::ZOMBIE_WAVE_CUTSCENE || self.from_wave == Zombie::ZOMBIE_WAVE_UI {
            return false;
        }
        true
    }

    /// 装备盾牌（对应 C++ AttachShield）
    pub fn attach_shield(&mut self) {}

    /// 卸下盾牌（对应 C++ DetachShield）
    pub fn detach_shield(&mut self) {}

    /// 设置水下动画轨道（对应 C++ SetupWaterTrack）
    pub fn setup_water_track(&mut self, _track_name: &str) {}

    /// 添加附加粒子（对应 C++ AddAttachedParticle）
    pub fn add_attached_particle(&mut self, _pos_x: i32, _pos_y: i32, _effect: ParticleEffect) -> Option<*mut ParticleSystem> { None }

    /// 设置动画速率（对应 C++ SetAnimRate）
    pub fn set_anim_rate(&mut self, _anim_rate: f32) {}

    /// 应用动画速率（对应 C++ ApplyAnimRate）
    pub fn apply_anim_rate(&mut self, _anim_rate: f32) {}

    /// 更新动画速度（对应 C++ UpdateAnimSpeed）
    pub fn update_anim_speed(&mut self) {}

    /// 更新重动画（对应 C++ UpdateReanim）
    pub fn update_reanim(&mut self) {}

    /// Boss 重动画设置（对应 C++ BossSetupReanim）
    pub fn boss_setup_reanim(&mut self) {}

    /// 设置僵尸旗帜重动画（对应 C++ SetupZombatarFlagReanim）
    pub fn setup_zombatar_flag_reanim(&mut self, _record_index: i32) {}

    /// 安装门板手臂（对应 C++ SetupDoorArms，静态方法）
    pub fn setup_door_arms(_reanim: *mut Reanimation, _show: bool) {}

    /// 设置重动画层（对应 C++ SetupReanimLayers，静态方法）
    pub fn setup_reanim_layers(_reanim: *mut Reanimation, _zombie_type: ZombieType) {}

    /// 显示门板手臂（对应 C++ ShowDoorArms）
    pub fn show_door_arms(&mut self, _show: bool) {}

    /// 忽略裁剪矩形（对应 C++ ReanimIgnoreClipRect）
    pub fn reanim_ignore_clip_rect(&mut self, _track_name: &str, _ignore_clip_rect: bool) {}

    /// 重新启用裁剪（对应 C++ ReanimReenableClipping）
    pub fn reanim_reenable_clipping(&mut self) {}

    /// 开始行走动画（对应 C++ StartWalkAnim）
    pub fn start_walk_anim(&mut self, _blend_time: i32) {}

    /// 启用胡子模式（对应 C++ EnableMustache）
    pub fn enable_mustache(&mut self, _enable: bool) {}

    /// 启用未来模式（对应 C++ EnableFuture）
    pub fn enable_future(&mut self, _enable: bool) {}

    /// 播放僵尸出现音效（对应 C++ PlayZombieAppearSound）
    pub fn play_zombie_appear_sound(&mut self) {}

    /// 基于行获取 Y 位置（对应 C++ GetPosYBasedOnRow）
    pub fn get_pos_y_based_on_row(&self, _row: i32) -> f32 { 0.0 }

    /// 选择蹦极僵尸目标（对应 C++ PickBungeeZombieTarget）
    pub fn pick_bungee_zombie_target(&mut self, _column: i32) {}
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
