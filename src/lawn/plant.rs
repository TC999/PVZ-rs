// PvZ Portable Rust 翻译 — Plant（植物类）
// 对应 C++ src/Lawn/Plant.h / Plant.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::lawn::board::Board;
use crate::lawn::zombie::Zombie;
use crate::todlib::reanimator::{Reanimation, ReanimationType, ReanimLoopType};
use crate::todlib::tod_particle::ParticleSystem;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::rect::Rect;
use crate::framework::color::Color;
use crate::framework::common::{Rand, RandRange, RandFloat};

pub const MAX_MAGNET_ITEMS: usize = 5;

// PlantSubClass, PlantWeapon, PlantOnBungeeState, PlantState, MagnetItemType
// 已移至 game_enums.rs 中定义，通过 use crate::lawn::game_enums::* 导入。

#[derive(Debug, Clone, Copy)]
pub struct MagnetItem {
    pub pos_x: f32,
    pub pos_y: f32,
    pub dest_offset_x: f32,
    pub dest_offset_y: f32,
    pub item_type: MagnetItemType,
}

impl MagnetItem {
    pub fn new() -> Self {
        MagnetItem {
            pos_x: 0.0,
            pos_y: 0.0,
            dest_offset_x: 0.0,
            dest_offset_y: 0.0,
            item_type: MagnetItemType::None,
        }
    }
}

/// 植物类
pub struct Plant {
    pub base: GameObject,

    pub seed_type: SeedType,
    pub plant_col: i32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub anim_counter: i32,
    pub frame: i32,
    pub frame_length: i32,
    pub num_frames: i32,
    pub state: PlantState,
    pub plant_health: i32,
    pub plant_max_health: i32,
    pub subclass: i32,
    pub disappear_countdown: i32,
    pub do_special_countdown: i32,
    pub state_countdown: i32,
    pub launch_counter: i32,
    pub launch_rate: i32,
    pub plant_rect: Rect,
    pub plant_attack_rect: Rect,
    pub target_x: i32,
    pub target_y: i32,
    pub start_row: i32,
    pub particle_id: ParticleSystemID,
    pub shooting_counter: i32,
    pub body_reanim_id: ReanimationID,
    pub head_reanim_id: ReanimationID,
    pub head_reanim_id2: ReanimationID,
    pub head_reanim_id3: ReanimationID,
    pub blink_reanim_id: ReanimationID,
    pub light_reanim_id: ReanimationID,
    pub sleeping_reanim_id: ReanimationID,
    pub blink_countdown: i32,
    pub recently_eaten_countdown: i32,
    pub eaten_flash_countdown: i32,
    pub beghouled_flash_countdown: i32,
    pub shake_offset_x: f32,
    pub shake_offset_y: f32,
    pub magnet_items: [MagnetItem; MAX_MAGNET_ITEMS],
    pub target_zombie_id: ZombieID,
    pub wake_up_counter: i32,
    pub on_bungee_state: PlantOnBungeeState,
    pub imitater_type: SeedType,
    pub potted_plant_index: i32,
    pub anim_ping: bool,
    pub dead: bool,
    pub squished: bool,
    pub is_asleep: bool,
    pub is_on_board: bool,
    pub highlighted: bool,
}

// [TRANSLATION_NOTE]: sentinel pointers for nut/garlic/pumpkin crack images (replace with real image resources)
static CRACK_IMAGE_1: u8 = 0;
static CRACK_IMAGE_2: u8 = 0;

impl Plant {
    pub fn new() -> Self {
        Plant {
            base: GameObject::new(),
            seed_type: SeedType::Peashooter,
            plant_col: 0,
            pos_x: 0.0,
            pos_y: 0.0,
            anim_counter: 0,
            frame: 0,
            frame_length: 0,
            num_frames: 0,
            state: PlantState::NotReady,
            plant_health: 0,
            plant_max_health: 0,
            subclass: 0,
            disappear_countdown: 0,
            do_special_countdown: 0,
            state_countdown: 0,
            launch_counter: 0,
            launch_rate: 0,
            plant_rect: Rect::ZERO,
            plant_attack_rect: Rect::ZERO,
            target_x: 0,
            target_y: 0,
            start_row: 0,
            particle_id: PARTICLESYSTEMID_NULL,
            shooting_counter: 0,
            body_reanim_id: REANIMATIONID_NULL,
            head_reanim_id: REANIMATIONID_NULL,
            head_reanim_id2: REANIMATIONID_NULL,
            head_reanim_id3: REANIMATIONID_NULL,
            blink_reanim_id: REANIMATIONID_NULL,
            light_reanim_id: REANIMATIONID_NULL,
            sleeping_reanim_id: REANIMATIONID_NULL,
            blink_countdown: 0,
            recently_eaten_countdown: 0,
            eaten_flash_countdown: 0,
            beghouled_flash_countdown: 0,
            shake_offset_x: 0.0,
            shake_offset_y: 0.0,
            magnet_items: [MagnetItem::new(), MagnetItem::new(), MagnetItem::new(), MagnetItem::new(), MagnetItem::new()],
            target_zombie_id: ZOMBIEID_NULL,
            wake_up_counter: 0,
            on_bungee_state: PlantOnBungeeState::NotOnBungee,
            imitater_type: SeedType::None,
            potted_plant_index: -1,
            anim_ping: false,
            dead: false,
            squished: false,
            is_asleep: false,
            is_on_board: false,
            highlighted: false,
        }
    }

    /// 初始化植物（对应 C++ Plant::PlantInitialize）
    pub fn plant_initialize(&mut self, grid_x: i32, grid_y: i32, seed_type: SeedType, imitater_type: SeedType) {
        self.plant_col = grid_x;
        self.base.row = grid_y;

        // 从 Board 获取像素坐标
        let (pixel_x, pixel_y) = if let Some(board) = self.base.get_board() {
            (board.grid_to_pixel_x(grid_x, grid_y), board.grid_to_pixel_y(grid_x, grid_y))
        } else {
            (0, 0)
        };
        self.base.x = pixel_x;
        self.base.y = pixel_y;
        self.pos_x = self.base.x as f32;
        self.pos_y = self.base.y as f32;

        self.anim_counter = 0;
        self.anim_ping = true;
        self.frame = 0;
        self.shooting_counter = 0;
        self.shake_offset_x = 0.0;
        self.shake_offset_y = 0.0;
        self.frame_length = RandRange(6) + 12; // RandRangeInt(12, 18)
        self.target_x = -1;
        self.target_y = -1;
        self.start_row = self.base.row;
        self.num_frames = 5;
        self.state = PlantState::NotReady;
        self.dead = false;
        self.squished = false;
        self.seed_type = seed_type;
        self.imitater_type = imitater_type;
        self.plant_health = 300;
        self.do_special_countdown = 0;
        self.disappear_countdown = 200;
        self.state_countdown = 0;
        self.particle_id = PARTICLESYSTEMID_NULL;
        self.body_reanim_id = REANIMATIONID_NULL;
        self.head_reanim_id = REANIMATIONID_NULL;
        self.head_reanim_id2 = REANIMATIONID_NULL;
        self.head_reanim_id3 = REANIMATIONID_NULL;
        self.blink_reanim_id = REANIMATIONID_NULL;
        self.light_reanim_id = REANIMATIONID_NULL;
        self.sleeping_reanim_id = REANIMATIONID_NULL;
        self.blink_countdown = 0;
        self.recently_eaten_countdown = 0;
        self.eaten_flash_countdown = 0;
        self.beghouled_flash_countdown = 0;
        self.base.width = 80;
        self.base.height = 80;
        // memset(mMagnetItems, 0, sizeof(mMagnetItems)) — 已通过初始化默认值处理
        self.is_asleep = false;
        self.wake_up_counter = 0;
        self.on_bungee_state = PlantOnBungeeState::NotOnBungee;
        self.potted_plant_index = -1;

        let a_plant_def = get_plant_definition(seed_type);
        self.launch_rate = a_plant_def.launch_rate;
        self.subclass = a_plant_def.sub_class as i32;
        self.base.render_order = self.calc_render_order();

        // 加载重动画
        if a_plant_def.reanimation_type != ReanimationType::None {
            let a_offset_y = Self::plant_draw_height_offset(self.base.get_board(), Some(self), seed_type, grid_x, grid_y);
            let render_order = self.base.render_order;
            if let Some(app) = self.base.get_app_mut() {
                if let Some(body_reanim) = app.add_reanimation(0.0, a_offset_y, render_order + 1, a_plant_def.reanimation_type as i32) {
                    unsafe {
                        (*body_reanim).m_loop_type = ReanimLoopType::Loop;
                        (*body_reanim).m_anim_rate = 10.0 + RandFloat(5.0);
                        if (*body_reanim).track_exists("anim_idle") {
                            (*body_reanim).set_frames_for_layer("anim_idle");
                        }
                        (*body_reanim).m_is_attachment = true;
                    }
                    self.body_reanim_id = app.reanimation_get_id(body_reanim);
                }
            }
            self.blink_countdown = 400 + RandRange(400);
        }

        // 夜间植物睡眠
        if Self::is_nocturnal(seed_type) {
            if let Some(board) = self.base.get_board() {
                if !board.stage_is_night() {
                    self.set_sleeping(true);
                }
            }
        }

        // 发射计数器初始化
        if self.launch_rate > 0 {
            if self.makes_sun() {
                self.launch_counter = RandRange(self.launch_rate / 2) + 300;
            } else {
                self.launch_counter = RandRange(self.launch_rate);
            }
        } else {
            self.launch_counter = 0;
        }

        // 根据种子类型初始化
        match seed_type {
            SeedType::Blover => {
                self.do_special_countdown = 50;
                if self.is_in_play() {
                    // Reanimation 操作在 stub 中
                    self.set_body_reanim_frame("anim_blow");
                    self.set_body_reanim_loop(ReanimLoopType::PlayOnceAndHold);
                    self.set_body_reanim_rate(20.0);
                } else {
                    self.set_body_reanim_frame("anim_idle");
                    self.set_body_reanim_rate(10.0);
                }
            }
            SeedType::Peashooter | SeedType::Snowpea | SeedType::Repeater |
            SeedType::Leftpeater | SeedType::Gatlingpea => {
                self.set_body_reanim_rate(15.0 + RandFloat(5.0));
                // 头部动画 — 通过 stub 处理
                self.add_head_reanim("anim_head_idle");
            }
            SeedType::Splitpea => {
                self.set_body_reanim_rate(15.0 + RandFloat(5.0));
                self.add_head_reanim("anim_head_idle");
                self.add_head_reanim2("anim_splitpea_idle");
            }
            SeedType::Threepeater => {
                self.set_body_reanim_rate(15.0 + RandFloat(5.0));
                self.add_head_reanim("anim_head_idle1");
                self.add_head_reanim2("anim_head_idle2");
                self.add_head_reanim3("anim_head_idle3");
            }
            SeedType::Wallnut => {
                self.plant_health = 4000;
                self.blink_countdown = 1000 + RandRange(1000);
            }
            SeedType::ExplodeONut => {
                self.plant_health = 4000;
                self.blink_countdown = 1000 + RandRange(1000);
            }
            SeedType::GiantWallnut => {
                self.plant_health = 4000;
                self.blink_countdown = 1000 + RandRange(1000);
            }
            SeedType::Tallnut => {
                self.plant_health = 8000;
                self.base.height = 80;
                self.blink_countdown = 1000 + RandRange(1000);
            }
            SeedType::Garlic => {
                self.plant_health = 400;
            }
            SeedType::GoldMagnet | SeedType::Magnetshroom => {
                // 对应 C++ Plant.cpp:456-458: aBodyReanim->SetTruncateDisappearingFrames()
                if let Some(app) = self.base.app {
                    unsafe {
                        if let Some(a_body) = (*app).reanimation_get_mut(self.body_reanim_id) {
                            a_body.set_truncate_disappearing_frames(None, true);
                        }
                    }
                }
            }
            SeedType::Imitater => {
                self.set_body_reanim_rate(25.0 + RandFloat(5.0));
                self.state_countdown = 200;
            }
            SeedType::Cherrybomb | SeedType::Jalapeno => {
                if self.is_in_play() {
                    self.do_special_countdown = 100;
                    self.set_body_reanim_frame("anim_explode");
                    self.set_body_reanim_loop(ReanimLoopType::PlayOnceAndHold);
                }
            }
            SeedType::PotatoMine => {
                self.set_body_reanim_rate(12.0);
                if self.is_in_play() {
                    self.state_countdown = 1500;
                } else {
                    self.set_body_reanim_frame("anim_armed");
                    self.state = PlantState::PotatoArmed;
                }
            }
            SeedType::Gravebuster => {
                if self.is_in_play() {
                    self.set_body_reanim_frame("anim_land");
                    self.set_body_reanim_loop(ReanimLoopType::PlayOnceAndHold);
                    self.state = PlantState::GravebusterLanding;
                }
            }
            SeedType::Sunshroom => {
                self.set_body_reanim_frame_base_pose(6);
                if self.is_in_play() {
                    self.base.x += RandRange(10) - 5;
                    self.base.y += RandRange(10) - 5;
                } else if self.is_asleep {
                    self.set_body_reanim_frame("anim_bigsleep");
                } else {
                    self.set_body_reanim_frame("anim_bigidle");
                }
                self.state = PlantState::SunshroomSmall;
                self.state_countdown = 12000;
            }
            SeedType::Puffshroom | SeedType::Seashroom => {
                if self.is_in_play() {
                    self.base.x += RandRange(10) - 5;
                    self.base.y += RandRange(6) - 3;
                }
            }
            SeedType::Pumpkinshell => {
                self.plant_health = 4000;
                self.base.width = 120;
            }
            SeedType::Chomper => {
                self.state = PlantState::Ready;
            }
            SeedType::Plantern => {
                self.state_countdown = 50;
            }
            SeedType::Cactus => {
                self.state = PlantState::CactusLow;
            }
            SeedType::InstantCoffee => {
                self.do_special_countdown = 100;
            }
            SeedType::Scaredyshroom => {
                self.state = PlantState::Ready;
            }
            SeedType::Cobcannon => {
                if self.is_in_play() {
                    self.state = PlantState::CobcannonArming;
                    self.state_countdown = 500;
                    self.set_body_reanim_frame("anim_unarmed_idle");
                }
            }
            SeedType::Kernelpult => {
                // 对应 C++ Plant.cpp:452-454: aBodyReanim->AssignRenderGroupToPrefix("Cornpult_butter", RENDER_GROUP_HIDDEN)
                if let Some(app) = self.base.app {
                    unsafe {
                        if let Some(a_body) = (*app).reanimation_get_mut(self.body_reanim_id) {
                            a_body.assign_render_group_to_prefix(
                                "Cornpult_butter",
                                crate::todlib::reanimator::RENDER_GROUP_HIDDEN,
                            );
                        }
                    }
                }
            }
            SeedType::Spikerock => {
                self.plant_health = 450;
            }
            SeedType::Flowerpot => {
                if self.is_in_play() {
                    self.state = PlantState::FlowerpotInvulnerable;
                    self.state_countdown = 100;
                }
            }
            SeedType::Lilypad => {
                if self.is_in_play() {
                    self.state = PlantState::LilypadInvulnerable;
                    self.state_countdown = 100;
                }
            }
            SeedType::Tanglekelp => {
                // 对应 C++ Plant.cpp:479-481: aBodyReanim->SetTruncateDisappearingFrames()
                if let Some(app) = self.base.app {
                    unsafe {
                        if let Some(a_body) = (*app).reanimation_get_mut(self.body_reanim_id) {
                            a_body.set_truncate_disappearing_frames(None, true);
                        }
                    }
                }
            }
            _ => {}
        }

        // Big Time 模式翻倍血量
        if let Some(app) = self.base.get_app() {
            if app.game_mode == GameMode::ChallengeBigTime &&
                (seed_type == SeedType::Wallnut || seed_type == SeedType::Sunflower || seed_type == SeedType::Marigold) {
                self.plant_health *= 2;
            }
        }

        self.plant_max_health = self.plant_health;

        // 花盆处理
        if seed_type != SeedType::Flowerpot && self.is_on_board {
            if let Some(board) = self.base.get_board() {
                if let Some(_flower_pot) = board.get_flower_pot_at(grid_x, grid_y) {
                    // 通过 app 设置花盆动画速率
                }
            }
        }
    }

    /// 更新植物（对应 C++ Plant::Update）
    pub fn update(&mut self) {
        if self.dead { return; }

        // 对应 C++ Plant::Update 的 doUpdate 四情形判定（Plant.cpp:2856-2865）
        let app_ptr = self.base.app;
        let on_board = self.is_on_board();
        let mut do_update = false;
        if let Some(app) = app_ptr {
            unsafe {
                if on_board
                    && (*app).game_scene == crate::lawn::lawn_app::GameScenes::LevelIntro
                    && (*app).is_wallnut_bowling_level()
                {
                    do_update = true;
                } else if on_board && (*app).game_mode == GameMode::ChallengeZenGarden {
                    do_update = true;
                } else if on_board
                    && self.base.get_board().map_or(false, |b| unsafe {
                        b.m_cut_scene.map_or(false, |c| unsafe { (*c).should_run_upsell_board() })
                    })
                {
                    do_update = true;
                } else if !on_board || (*app).game_scene == crate::lawn::lawn_app::GameScenes::Playing {
                    do_update = true;
                }
            }
        }

        if do_update {
            self.update_abilities();
            self.animate();

            if self.plant_health < 0 {
                self.die();
            }

            // C++ Update 中 UpdateReanim() 内的重动画颜色更新（Plant.cpp:2745）
            self.update_reanim_color();
        }
    }

    /// 对应 C++ Plant::Animate（Plant.cpp:3419-3493）
    /// 帧推进与受损闪白/眨眼/压扁短路的动画主体
    pub fn animate(&mut self) {
        // C++: (CHERRYBOMB || JALAPENO) && 非禅园模式时随机抖动
        if (self.seed_type == SeedType::Cherrybomb || self.seed_type == SeedType::Jalapeno)
            && self.base.app.map_or(false, |a| unsafe {
                (*a).game_mode != GameMode::ChallengeZenGarden
            })
        {
            self.shake_offset_x = crate::todlib::tod_common::rand_range_float(-1.0, 1.0);
            self.shake_offset_y = crate::todlib::tod_common::rand_range_float(-1.0, 1.0);
        }

        if self.recently_eaten_countdown > 0 {
            self.recently_eaten_countdown -= 1;
        }
        if self.eaten_flash_countdown > 0 {
            self.eaten_flash_countdown -= 1;
        }
        if self.beghouled_flash_countdown > 0 {
            self.beghouled_flash_countdown -= 1;
        }

        if self.squished {
            self.frame = 0;
            return;
        }

        // C++: 坚果/大蒜/南瓜受损动画分支（Plant.cpp:3447-3458）
        if self.seed_type == SeedType::Wallnut || self.seed_type == SeedType::Tallnut {
            self.animate_nuts();
        } else if self.seed_type == SeedType::Garlic {
            self.animate_garlic();
        } else if self.seed_type == SeedType::Pumpkinshell {
            self.animate_pumpkin();
        }

        // C++: UpdateBlink()（Plant.cpp:3459）
        self.update_blink();

        // C++: mAnimPing/mAnimCounter 帧推进（Plant.cpp:3461-3482）
        if self.anim_ping {
            if self.anim_counter < self.frame_length * self.num_frames - 1 {
                self.anim_counter += 1;
            } else {
                self.anim_ping = false;
                self.anim_counter -= self.frame_length;
            }
        } else if self.anim_counter > 0 {
            self.anim_counter -= 1;
        } else {
            self.anim_ping = true;
            self.anim_counter += self.frame_length;
        }
        self.frame = self.anim_counter / self.frame_length;
    }

    /// 是否不在土地上（对应 C++ NotOnGround）
    pub fn not_on_ground(&self) -> bool {
        if self.seed_type == SeedType::Squash {
            if self.state == PlantState::SquashRising
                || self.state == PlantState::SquashFalling
                || self.state == PlantState::SquashDoneFalling
            {
                return true;
            }
        }
        self.squished || self.on_bungee_state != PlantOnBungeeState::NotOnBungee || self.dead
    }

    /// 是否尖刺植物（对应 C++ Plant::IsSpiky）
    pub fn is_spiky(&self) -> bool {
        self.seed_type == SeedType::Spikeweed || self.seed_type == SeedType::Spikerock
    }

    /// 是否位于高台（对应 C++ Plant::IsOnHighGround）
    pub fn is_on_high_ground(&self) -> bool {
        let board = match self.base.board { Some(b) => b, None => return false };
        unsafe {
            let b = &*board;
            let gx = self.plant_col as usize;
            let gy = self.base.row as usize;
            if gx >= b.grid_square_type.len() || gy >= b.grid_square_type[0].len() {
                return false;
            }
            b.grid_square_type[gx][gy] == GridSquareType::HighGround
        }
    }

    /// 获取植物矩形（对应 C++ Plant::GetPlantRect）
    pub fn get_plant_rect(&self) -> Rect {
        if self.seed_type == SeedType::Tallnut {
            Rect::new(self.base.x + 10, self.base.y, self.base.width, self.base.height)
        } else if self.seed_type == SeedType::Pumpkinshell {
            Rect::new(self.base.x, self.base.y, self.base.width - 20, self.base.height)
        } else if self.seed_type == SeedType::Cobcannon {
            Rect::new(self.base.x, self.base.y, 140, 80)
        } else {
            Rect::new(self.base.x + 10, self.base.y, self.base.width - 20, self.base.height)
        }
    }

    /// 更新射手类植物（对应 C++ UpdateShooter）
    pub fn update_shooter(&mut self) {
        self.launch_counter -= 1;
        if self.launch_counter <= 0 {
            self.launch_counter = self.launch_rate - 15 + RandRange(15); // Rand(15)

            match self.seed_type {
                SeedType::Threepeater => {
                    self.launch_threepeater();
                }
                SeedType::Starfruit => {
                    self.launch_star_fruit();
                }
                SeedType::Splitpea => {
                    self.find_target_and_fire(self.base.row, PlantWeapon::Primary);
                    self.find_target_and_fire(self.base.row, PlantWeapon::Secondary);
                }
                SeedType::Cactus => {
                    if self.state == PlantState::CactusHigh {
                        self.find_target_and_fire(self.base.row, PlantWeapon::Primary);
                    } else if self.state == PlantState::CactusLow {
                        self.find_target_and_fire(self.base.row, PlantWeapon::Secondary);
                    }
                }
                _ => {
                    self.find_target_and_fire(self.base.row, PlantWeapon::Primary);
                }
            }
        }

        // 二次射击（Cattail/Repeater/Splitpea）
        if self.launch_counter == 50 && self.seed_type == SeedType::Cattail {
            self.find_target_and_fire(self.base.row, PlantWeapon::Primary);
        }
        if self.launch_counter == 25 {
            match self.seed_type {
                SeedType::Repeater | SeedType::Leftpeater => {
                    self.find_target_and_fire(self.base.row, PlantWeapon::Primary);
                }
                SeedType::Splitpea => {
                    self.find_target_and_fire(self.base.row, PlantWeapon::Secondary);
                }
                _ => {}
            }
        }
    }

    /// 更新生产类植物（对应 C++ UpdateProductionPlant）
    pub fn update_production_plant(&mut self) {
        // 对应 C++: if (!IsInPlay() || mApp->IsIZombieLevel() ||
        //                mApp->mGameMode == GAMEMODE_UPSELL || mApp->mGameMode == GAMEMODE_INTRO) return;
        if !self.is_in_play() { return; }
        let app_mode = self.base.get_app().map_or(GameMode::Adventure, |app| app.game_mode);
        if app_mode == GameMode::Upsell || app_mode == GameMode::Intro { return; }
        if self.base.get_app().map_or(true, |app| app.is_izombie_level()) { return; }

        // 对应 C++: if (mBoard->HasLevelAwardDropped()) return;
        let award_dropped = self
            .base
            .get_board_mut()
            .map_or(false, |board| board.has_level_award_dropped());
        if award_dropped { return; }

        // 对应 C++: 金盏花在最后一波进入 Ending 状态，倒计时结束后停止生产
        if self.seed_type == SeedType::Marigold {
            let last_wave = self
                .base
                .get_board_mut()
                .map_or(false, |board| board.m_current_wave == board.m_total_waves);
            if last_wave {
                if self.state != PlantState::MarigoldEnding {
                    self.state = PlantState::MarigoldEnding;
                    self.state_countdown = 6000;
                } else if self.state_countdown <= 0 {
                    return;
                }
            }
        }

        // 对应 C++: Last Stand 模式仅在 Onslaught 阶段生产
        if app_mode == GameMode::ChallengeLastStand {
            let onslaught = self.base.get_board_mut().map_or(false, |board| {
                board
                    .challenge
                    .as_ref()
                    .map_or(false, |challenge| {
                        challenge.challenge_state == ChallengeState::LastStandOnslaught
                    })
            });
            if !onslaught { return; }
        }

        // 对应 C++: mLaunchCounter--;
        self.launch_counter -= 1;
        if self.launch_counter <= 100 {
            // 对应 C++: mEatenFlashCountdown =
            //     max(mEatenFlashCountdown, PvzpAnimateCurve(100, 0, mLaunchCounter, 0, 100, CURVE_LINEAR));
            let flash_countdown = crate::todlib::tod_common::tod_animate_curve(
                100, 0, self.launch_counter, 0, 100, crate::lawn::game_enums::TodCurves::Linear,
            );
            self.eaten_flash_countdown = self.eaten_flash_countdown.max(flash_countdown);
        }
        if self.launch_counter <= 0 {
            // 对应 C++: mLaunchCounter = RandRangeInt(mLaunchRate - 150, mLaunchRate);
            self.launch_counter = self.launch_rate - 150 + RandRange(150);
            // 对应 C++: mApp->PlayFoley(FOLEY_SPAWN_SUN);
            if let Some(app) = self.base.get_app_mut() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
            }

            let plant_x = self.base.x;
            let plant_y = self.base.y;
            // 对应 C++: 按种子类型产出对应货币
            match self.seed_type {
                SeedType::Sunshroom => {
                    if let Some(board) = self.base.get_board_mut() {
                        if self.state == PlantState::SunshroomSmall {
                            board.add_coin(plant_x as f32, plant_y as f32, CoinType::SmallSun, CoinMotion::FromPlant);
                        } else {
                            board.add_coin(plant_x as f32, plant_y as f32, CoinType::Sun, CoinMotion::FromPlant);
                        }
                    }
                }
                SeedType::Sunflower => {
                    if let Some(board) = self.base.get_board_mut() {
                        board.add_coin(plant_x as f32, plant_y as f32, CoinType::Sun, CoinMotion::FromPlant);
                    }
                }
                SeedType::Twinsunflower => {
                    if let Some(board) = self.base.get_board_mut() {
                        board.add_coin(plant_x as f32, plant_y as f32, CoinType::Sun, CoinMotion::FromPlant);
                        board.add_coin(plant_x as f32, plant_y as f32, CoinType::Sun, CoinMotion::FromPlant);
                    }
                }
                SeedType::Marigold => {
                    if let Some(board) = self.base.get_board_mut() {
                        let coin_type = if RandRange(100) < 10 { CoinType::Gold } else { CoinType::Silver };
                        board.add_coin(plant_x as f32, plant_y as f32, coin_type, CoinMotion::Coin);
                    }
                }
                _ => {}
            }

            // 对应 C++: BIG_TIME 模式的额外生产
            if app_mode == GameMode::ChallengeBigTime {
                match self.seed_type {
                    SeedType::Sunflower => {
                        if let Some(board) = self.base.get_board_mut() {
                            board.add_coin(plant_x as f32, plant_y as f32, CoinType::Sun, CoinMotion::FromPlant);
                        }
                    }
                    SeedType::Marigold => {
                        if let Some(board) = self.base.get_board_mut() {
                            board.add_coin(plant_x as f32, plant_y as f32, CoinType::Silver, CoinMotion::Coin);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// 寻找目标并开火（对应 C++ Plant::FindTargetAndFire）
    pub fn find_target_and_fire(&mut self, the_row: i32, weapon: PlantWeapon) -> bool {
        // 在 Board 中找到目标僵尸（对应 C++ FindTargetZombie）
        let target_zombie_id = self.find_target_zombie(the_row, weapon);
        if target_zombie_id.is_none() {
            return false;
        }

        // 对应 C++ Plant::FindTargetAndFire（Plant.cpp:731-820）
        self.end_blink();
        let app = match self.base.app {
            Some(a) => a,
            None => return false,
        };

        if self.seed_type == SeedType::Splitpea && weapon == PlantWeapon::Secondary {
            // 对应 C++: aHeadReanim2->StartBlend(20); PlayOnceAndHold; 35fps;
            // SetFramesForLayer("anim_splitpea_shooting"); mShootingCounter = 26
            unsafe {
                if let Some(a_head2) = (*app).reanimation_get_mut(self.head_reanim_id2) {
                    a_head2.start_blend(20);
                    a_head2.m_loop_type = ReanimLoopType::PlayOnceAndHold;
                    a_head2.m_anim_rate = 35.0;
                    a_head2.set_frames_for_layer("anim_splitpea_shooting");
                }
            }
            self.shooting_counter = 26;
        } else if unsafe { (*app).reanimation_get(self.head_reanim_id) }
            .map_or(false, |r| r.track_exists("anim_shooting"))
        {
            // 对应 C++: aHeadReanim->StartBlend(20); PlayOnceAndHold; 35fps; SetFramesForLayer("anim_shooting")
            unsafe {
                if let Some(a_head) = (*app).reanimation_get_mut(self.head_reanim_id) {
                    a_head.start_blend(20);
                    a_head.m_loop_type = ReanimLoopType::PlayOnceAndHold;
                    a_head.m_anim_rate = 35.0;
                    a_head.set_frames_for_layer("anim_shooting");
                }
            }
            self.shooting_counter = 33;
            if self.seed_type == SeedType::Repeater
                || self.seed_type == SeedType::Splitpea
                || self.seed_type == SeedType::Leftpeater
            {
                // 对应 C++: aHeadReanim->mAnimRate = 45; mShootingCounter = 26
                unsafe {
                    if let Some(a_head) = (*app).reanimation_get_mut(self.head_reanim_id) {
                        a_head.m_anim_rate = 45.0;
                    }
                }
                self.shooting_counter = 26;
            } else if self.seed_type == SeedType::Gatlingpea {
                // 对应 C++: aHeadReanim->mAnimRate = 38; mShootingCounter = 100
                unsafe {
                    if let Some(a_head) = (*app).reanimation_get_mut(self.head_reanim_id) {
                        a_head.m_anim_rate = 38.0;
                    }
                }
                self.shooting_counter = 100;
            }
        } else if self.state == PlantState::CactusHigh {
            // 对应 C++: PlayBodyReanim("anim_shootinghigh", PLAY_ONCE_AND_HOLD, 20, 35); mShootingCounter = 23
            self.play_body_reanim("anim_shootinghigh", ReanimLoopType::PlayOnceAndHold, 20, 35.0);
            self.shooting_counter = 23;
        } else if self.seed_type == SeedType::Gloomshroom {
            // 对应 C++: PlayBodyReanim("anim_shooting", ..., 14fps); mShootingCounter = 200
            self.play_body_reanim("anim_shooting", ReanimLoopType::PlayOnceAndHold, 20, 14.0);
            self.shooting_counter = 200;
        } else if self.seed_type == SeedType::Cattail {
            // 对应 C++: PlayBodyReanim("anim_shooting", ..., 30fps); mShootingCounter = 50
            self.play_body_reanim("anim_shooting", ReanimLoopType::PlayOnceAndHold, 20, 30.0);
            self.shooting_counter = 50;
        } else if unsafe { (*app).reanimation_get(self.body_reanim_id) }
            .map_or(false, |r| r.track_exists("anim_shooting"))
        {
            // 对应 C++: PlayBodyReanim("anim_shooting", PLAY_ONCE_AND_HOLD, 20, 35)
            self.play_body_reanim("anim_shooting", ReanimLoopType::PlayOnceAndHold, 20, 35.0);
            self.shooting_counter = match self.seed_type {
                SeedType::Fumeshroom => 50,
                SeedType::Puffshroom => 29,
                SeedType::Scaredyshroom => 25,
                SeedType::Cabbagepult => 32,
                SeedType::Melonpult | SeedType::Wintermelon => 36,
                SeedType::Kernelpult => {
                    // 对应 C++: Rand(4) == 0 → 切换 butter 渲染组
                    if RandRange(4) == 0 {
                        unsafe {
                            if let Some(a_body) = (*app).reanimation_get_mut(self.body_reanim_id) {
                                a_body.assign_render_group_to_prefix("Cornpult_butter", crate::todlib::reanimator::RENDER_GROUP_NORMAL);
                                a_body.assign_render_group_to_prefix("Cornpult_kernal", crate::todlib::reanimator::RENDER_GROUP_HIDDEN);
                            }
                        }
                        self.state = PlantState::KernelpultButter;
                    }
                    30
                }
                SeedType::Cactus => 35,
                _ => 29,
            };
        } else {
            // 对应 C++: else Fire(aZombie, theRow, thePlantWeapon)
            self.fire(target_zombie_id, the_row, weapon);
            return true;
        }
        true
    }

    /// 发射三发子弹（对应 C++ LaunchThreepeater）
    pub fn launch_threepeater(&mut self) {
        let x = self.base.x + 40;
        let y = self.base.y + 20;
        let row = self.base.row;
        if let Some(board) = self.base.get_board_mut() {
            board.add_projectile(x as f32, y as f32, row, self.seed_type);
            if board.row_can_have_zombies(row - 1) {
                board.add_projectile(x as f32, (y - 20) as f32, row - 1, self.seed_type);
            }
            if board.row_can_have_zombies(row + 1) {
                board.add_projectile(x as f32, (y + 20) as f32, row + 1, self.seed_type);
            }
        }
    }

    /// 发射星星（对应 C++ StarFruitFire）
    pub fn launch_star_fruit(&mut self) {
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::Throw as i32);
        }
        let a_shoot_angle_x = (30.0f32).to_radians().cos() * 3.33;
        let a_shoot_angle_y = (30.0f32).to_radians().sin() * 3.33;
        let x = (self.base.x + 25) as f32;
        let y = (self.base.y + 25) as f32;
        let row = self.base.row;
        let a_damage_range_flags = self.get_damage_range_flags(PlantWeapon::Primary);
        if let Some(board) = self.base.get_board_mut() {
            for i in 0..5 {
                let idx = board.add_projectile(x, y, row, SeedType::Starfruit);
                let (vel_x, vel_y) = match i {
                    0 => (-3.33, 0.0),
                    1 => (0.0, 3.33),
                    2 => (0.0, -3.33),
                    3 => (a_shoot_angle_x, a_shoot_angle_y),
                    _ => (a_shoot_angle_x, -a_shoot_angle_y),
                };
                board.projectiles[idx].vel_x = vel_x;
                board.projectiles[idx].vel_y = vel_y;
                board.projectiles[idx].motion = crate::lawn::projectile::ProjectileMotion::Star;
                board.projectiles[idx].damage_flags = a_damage_range_flags;
            }
        }
    }

    /// 开火（对应 C++ Plant::Fire：按 seed_type/weapon 决定弹种、伤害、特效）
    pub fn fire(&mut self, target_zombie_id: Option<ZombieID>, the_row: i32, the_plant_weapon: PlantWeapon) {
        // 特殊植物直接造成范围伤害
        if self.seed_type == SeedType::Fumeshroom {
            self.do_row_area_damage(20, 2);
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Fume as i32);
            }
            return;
        }
        if self.seed_type == SeedType::Gloomshroom {
            self.do_row_area_damage(20, 2);
            return;
        }
        if self.seed_type == SeedType::Starfruit {
            self.launch_star_fruit();
            return;
        }

        // 按 seed_type 选择弹种（对应 C++ switch）
        let mut a_projectile_type = match self.seed_type {
            SeedType::Peashooter | SeedType::Repeater | SeedType::Threepeater
            | SeedType::Splitpea | SeedType::Gatlingpea | SeedType::Leftpeater => crate::lawn::projectile::ProjectileType::Pea,
            SeedType::Snowpea => crate::lawn::projectile::ProjectileType::Snowpea,
            SeedType::Puffshroom | SeedType::Scaredyshroom | SeedType::Seashroom => crate::lawn::projectile::ProjectileType::Puff,
            SeedType::Cactus | SeedType::Cattail => crate::lawn::projectile::ProjectileType::Spike,
            SeedType::Cabbagepult => crate::lawn::projectile::ProjectileType::Cabbage,
            SeedType::Kernelpult => crate::lawn::projectile::ProjectileType::Kernel,
            SeedType::Melonpult => crate::lawn::projectile::ProjectileType::Melon,
            SeedType::Wintermelon => crate::lawn::projectile::ProjectileType::Wintermelon,
            SeedType::Cobcannon => crate::lawn::projectile::ProjectileType::Cobbig,
            _ => crate::lawn::projectile::ProjectileType::Pea,
        };
        if self.seed_type == SeedType::Kernelpult && the_plant_weapon == PlantWeapon::Secondary {
            a_projectile_type = crate::lawn::projectile::ProjectileType::Butter;
        }

        // 音效（对应 C++ FOLEY_THROW / FOLEY_SNOW_PEA_SPARKLES / FOLEY_PUFF）
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::Throw as i32);
            if self.seed_type == SeedType::Snowpea || self.seed_type == SeedType::Wintermelon {
                app.play_foley(crate::todlib::tod_foley::FoleyType::SnowPeaSparkles as i32);
            } else if self.seed_type == SeedType::Puffshroom
                || self.seed_type == SeedType::Scaredyshroom
                || self.seed_type == SeedType::Seashroom
            {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Puff as i32);
            }
        }

        // 计算发射原点（对应 C++ 大量 if-else 偏移）
        let m_x = self.base.x;
        let m_y = self.base.y;
        let m_row = self.base.row;
        let mut a_origin_x;
        let mut a_origin_y;
        if self.seed_type == SeedType::Puffshroom {
            a_origin_x = m_x + 40;
            a_origin_y = m_y + 40;
        } else if self.seed_type == SeedType::Seashroom {
            a_origin_x = m_x + 45;
            a_origin_y = m_y + 63;
        } else if self.seed_type == SeedType::Cabbagepult {
            a_origin_x = m_x + 5;
            a_origin_y = m_y - 12;
        } else if self.seed_type == SeedType::Melonpult || self.seed_type == SeedType::Wintermelon {
            a_origin_x = m_x + 25;
            a_origin_y = m_y - 46;
        } else if self.seed_type == SeedType::Cattail {
            a_origin_x = m_x + 20;
            a_origin_y = m_y - 3;
        } else if self.seed_type == SeedType::Kernelpult && the_plant_weapon == PlantWeapon::Primary {
            a_origin_x = m_x + 19;
            a_origin_y = m_y - 37;
        } else if self.seed_type == SeedType::Kernelpult && the_plant_weapon == PlantWeapon::Secondary {
            a_origin_x = m_x + 12;
            a_origin_y = m_y - 56;
        } else if self.seed_type == SeedType::Peashooter
            || self.seed_type == SeedType::Snowpea
            || self.seed_type == SeedType::Repeater
        {
            let (a_offset_x, a_offset_y) = self.get_pea_head_offset();
            a_origin_x = m_x + a_offset_x + 24;
            a_origin_y = m_y + a_offset_y - 33;
        } else if self.seed_type == SeedType::Leftpeater {
            let (a_offset_x, a_offset_y) = self.get_pea_head_offset();
            a_origin_x = m_x + a_offset_x - 57;
            a_origin_y = m_y + a_offset_y - 33;
        } else if self.seed_type == SeedType::Gatlingpea {
            let (a_offset_x, a_offset_y) = self.get_pea_head_offset();
            a_origin_x = m_x + a_offset_x + 34;
            a_origin_y = m_y + a_offset_y - 33;
        } else if self.seed_type == SeedType::Splitpea {
            let (a_offset_x, a_offset_y) = self.get_pea_head_offset();
            a_origin_y = m_y + a_offset_y - 33;
            if the_plant_weapon == PlantWeapon::Secondary {
                a_origin_x = m_x + a_offset_x - 64;
            } else {
                a_origin_x = m_x + a_offset_x + 24;
            }
        } else if self.seed_type == SeedType::Threepeater {
            a_origin_x = m_x + 45;
            a_origin_y = m_y + 10;
        } else if self.seed_type == SeedType::Scaredyshroom {
            a_origin_x = m_x + 29;
            a_origin_y = m_y + 21;
        } else if self.seed_type == SeedType::Cactus {
            if the_plant_weapon == PlantWeapon::Primary {
                a_origin_x = m_x + 93;
                a_origin_y = m_y - 50;
            } else {
                a_origin_x = m_x + 70;
                a_origin_y = m_y + 23;
            }
        } else if self.seed_type == SeedType::Cobcannon {
            a_origin_x = m_x - 44;
            a_origin_y = m_y - 184;
        } else {
            a_origin_x = m_x + 10;
            a_origin_y = m_y + 5;
        }
        if let Some(board) = self.base.get_board() {
            if board.get_flower_pot_at(self.plant_col, m_row).is_some() {
                a_origin_y -= 5;
            }
        }
        // [TRANSLATION_NOTE]: C++ Snowpea/Puffshroom/Scaredyshroom 枪口粒子未接入

        // 投石车目标预测（对应 C++ LOBED 弹道计算）
        let mut a_range_x = 700.0f32 - a_origin_x as f32;
        let mut a_range_y = 0.0f32;
        if self.seed_type == SeedType::Cabbagepult
            || self.seed_type == SeedType::Kernelpult
            || self.seed_type == SeedType::Melonpult
            || self.seed_type == SeedType::Wintermelon
        {
            if let Some(board) = self.base.get_board() {
                if let Some(zombie) = target_zombie_id.and_then(|id| board.zombie_try_to_get(id)) {
                    let z_rect = zombie.get_zombie_rect();
                    a_range_x = zombie.zombie_target_lead_x(50.0) - a_origin_x as f32 - 30.0;
                    a_range_y = z_rect.y as f32 - a_origin_y as f32;
                    if zombie.zombie_phase == ZombiePhase::DolphinRiding {
                        a_range_x -= 60.0;
                    }
                    if zombie.zombie_type == ZombieType::Pogo && zombie.has_object {
                        a_range_x -= 60.0;
                    }
                    if zombie.zombie_phase == ZombiePhase::SnorkelWalkingInPool {
                        a_range_x -= 40.0;
                    }
                    if zombie.zombie_type == ZombieType::Boss {
                        a_range_y = board.grid_to_pixel_y(8, m_row) as f32 - a_origin_y as f32;
                    }
                }
            }
            if a_range_x < 40.0 {
                a_range_x = 40.0;
            }
        }

        // 预计算，避免在 board 借用期间访问 self（借用检查）
        let a_damage_range_flags = self.get_damage_range_flags(the_plant_weapon);
        let cob_target_x = self.target_x as f32 - 40.0;
        let cob_target_row = if let Some(board) = self.base.get_board() {
            board.pixel_to_grid_y_keep_on_board(self.target_x, self.target_y)
        } else {
            0
        };

        // 发射子弹并设置弹道参数
        if let Some(board) = self.base.get_board_mut() {
            let idx = board.add_projectile(a_origin_x as f32, a_origin_y as f32, the_row, self.seed_type);
            let proj = &mut board.projectiles[idx];
            proj.projectile_type = a_projectile_type;
            proj.damage_range_flags = a_damage_range_flags;

            if self.seed_type == SeedType::Cabbagepult
                || self.seed_type == SeedType::Kernelpult
                || self.seed_type == SeedType::Melonpult
                || self.seed_type == SeedType::Wintermelon
            {
                proj.motion = crate::lawn::projectile::ProjectileMotion::Lobbed;
                proj.vel_x = a_range_x / 120.0;
                proj.vel_y = 0.0;
                proj.vel_z = a_range_y / 120.0 - 7.0;
                proj.acc_z = 0.115;
            } else if self.seed_type == SeedType::Threepeater {
                if the_row < m_row {
                    proj.motion = crate::lawn::projectile::ProjectileMotion::Threepeater;
                    proj.vel_y = -3.0;
                    proj.shadow_y += 80.0;
                } else if the_row > m_row {
                    proj.motion = crate::lawn::projectile::ProjectileMotion::Threepeater;
                    proj.vel_y = 3.0;
                    proj.shadow_y -= 80.0;
                }
            } else if self.seed_type == SeedType::Puffshroom || self.seed_type == SeedType::Seashroom {
                // C++: aProjectile->mMotionType = MOTION_PUFF（Plant.cpp:4740）
                proj.motion = crate::lawn::projectile::ProjectileMotion::Puff;
            } else if self.seed_type == SeedType::Splitpea && the_plant_weapon == PlantWeapon::Secondary {
                // C++: aProjectile->mMotionType = MOTION_BACKWARDS（Plant.cpp:4744，向左由运动逻辑处理）
                proj.motion = crate::lawn::projectile::ProjectileMotion::Backwards;
            } else if self.seed_type == SeedType::Leftpeater {
                // C++: aProjectile->mMotionType = MOTION_BACKWARDS（Plant.cpp:4748）
                proj.motion = crate::lawn::projectile::ProjectileMotion::Backwards;
            } else if self.seed_type == SeedType::Cattail {
                // C++: aProjectile->mMotionType = MOTION_HOMING（Plant.cpp:4753）+ mTargetZombieID
                proj.motion = crate::lawn::projectile::ProjectileMotion::Homing;
                proj.target_zombie_id = target_zombie_id.unwrap_or(ZOMBIEID_NULL);
            } else if self.seed_type == SeedType::Cobcannon {
                proj.motion = crate::lawn::projectile::ProjectileMotion::Lobbed;
                proj.vel_x = 0.001;
                proj.vel_y = 0.0;
                proj.acc_z = 0.0;
                proj.vel_z = -8.0;
                proj.cob_target_x = cob_target_x;
                proj.cob_target_row = cob_target_row;
            }
        }
    }

    /// 寻找目标僵尸（对应 C++ Plant::FindTargetZombie）
    /// 返回僵尸在 mZombies 中的索引（Rust 侧用 Vec 索引作为 ZombieID）
    pub fn find_target_zombie(&self, row: i32, weapon: PlantWeapon) -> Option<ZombieID> {
        if let Some(board) = self.base.get_board() {
            let attack_rect = self.get_plant_attack_rect(weapon);
            let mut best_id = None;
            let mut best_weight = -999999;
            for (i, zombie) in board.zombies.iter().enumerate() {
                if zombie.dead { continue; }
                let row_dev = if zombie.zombie_type == ZombieType::Boss { 0 } else { zombie.base.row - row };
                if row_dev != 0 { continue; }
                let z_rect = zombie.get_zombie_rect();
                if crate::lawn::board::get_rect_overlap(&attack_rect, &z_rect) >= 0 {
                    let weight = -z_rect.x;
                    if best_id.is_none() || weight > best_weight {
                        best_weight = weight;
                        best_id = Some(i as u32);
                    }
                }
            }
            best_id
        } else { None }
    }

    /// 是否找到星星果实目标（对应 C++ FindStarFruitTarget）
    pub fn find_star_fruit_target(&self) -> bool {
        if self.recently_eaten_countdown > 0 {
            return true;
        }
        let damage_range_flags = self.get_damage_range_flags(PlantWeapon::Primary);
        let center_star_x = self.base.x + 40;
        let center_star_y = self.base.y + 40;
        let board = match self.base.board { Some(b) => b, None => return false };
        unsafe {
            let b = &*board;
            for zombie in &b.zombies {
                if zombie.dead { continue; }
                let zombie_rect = zombie.get_zombie_rect();
                if !zombie.effected_by_damage(damage_range_flags) { continue; }
                if zombie.zombie_type == ZombieType::Boss && self.plant_col >= 5 {
                    return true;
                }
                if zombie.base.row == self.base.row {
                    if zombie_rect.x + zombie_rect.width < center_star_x {
                        return true;
                    }
                } else {
                    let mut rect = zombie_rect;
                    if zombie.zombie_type == ZombieType::Digger {
                        rect.width += 10;
                    }
                    let proj_x = (center_star_x - (rect.x + rect.width / 2)) as f32;
                    let proj_y = (center_star_y - (rect.y + rect.height / 2)) as f32;
                    let projectile_time = (proj_x * proj_x + proj_y * proj_y).sqrt() / 3.33;
                    let zombie_hit_x = zombie.zombie_target_lead_x(projectile_time) as i32 - rect.width / 2;
                    if zombie_hit_x + rect.width > center_star_x && zombie_hit_x < center_star_x {
                        return true;
                    }
                    let center_zombie_x = zombie_hit_x + rect.width / 2;
                    let center_zombie_y = rect.y + rect.height / 2;
                    let angle = ((center_zombie_y - center_star_y) as f32).atan2((center_zombie_x - center_star_x) as f32).to_degrees();
                    if (zombie.base.row - self.base.row).abs() < 2 {
                        if (angle > 20.0 && angle < 40.0) || (angle < -25.0 && angle > -45.0) {
                            return true;
                        }
                    } else {
                        if (angle > 25.0 && angle < 35.0) || (angle < -28.0 && angle > -38.0) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// 鼠标按下（对应 C++ Plant::MouseDown）
    pub fn mouse_down(&mut self, x: i32, y: i32, click_count: i32) {
        if click_count < 0 {
            return;
        }
        if self.state == PlantState::CobcannonReady {
            let plant_id = self as *mut Plant as usize as u32;
            if let Some(board) = self.base.get_board_mut() {
                board.clear_cursor();
                board.cursor_object.seed_type = SeedType::None;
                board.cursor_object.cursor_type = CursorType::CobcannonTarget;
                board.cursor_object.seed_bank_index = -1;
                board.cursor_object.coin_id = COINID_NULL;
                // 当前 PlantID 用指针转换（对应 C++ DataArrayGetID）
                board.cursor_object.cob_cannon_plant_id = plant_id;
                board.m_cob_cannon_cursor_delay_counter = 30;
                board.m_cob_cannon_mouse_x = x;
                board.m_cob_cannon_mouse_y = y;
            }
        }
    }

    /// 是否在棋盘上（对应 C++ Plant::IsOnBoard）
    pub fn is_on_board(&self) -> bool {
        self.is_on_board && self.base.board.is_some()
    }

    /// 植物死亡（对应 C++ Plant::Die）
    pub fn die(&mut self) {
        // 对应 C++: Tanglekelp（海草）死亡时拖死目标僵尸
        if self.is_on_board() && self.seed_type == SeedType::Tanglekelp {
            let a_target_zombie_id = self.target_zombie_id;
            let a_has_target = self
                .base
                .get_board()
                .map_or(false, |board| board.zombie_try_to_get(a_target_zombie_id).is_some());
            if a_has_target {
                if let Some(board) = self.base.get_board_mut() {
                    if let Some(a_zombie) = board.zombie_try_to_get_mut(a_target_zombie_id) {
                        a_zombie.die_with_loot();
                    }
                }
            }
        }

        self.dead = true;
        self.is_on_board = false;

        // 对应 C++: RemoveEffects()
        self.remove_effects();

        // 对应 C++: 非飞行植物移除所在格的梯子（IsOnBoard 以 board 存在判定，
        // 与 is_on_board 字段解耦——C++ 中 mDead 置位后 IsOnBoard 仍成立）
        if !Self::is_flying(self.seed_type) && self.base.board.is_some() {
            let a_ladder_idx = self
                .base
                .get_board()
                .and_then(|board| {
                    board.grid_items.iter().position(|item| {
                        !item.dead
                            && item.grid_item_type == crate::lawn::grid_item::GridItemType::Ladder
                            && item.grid_x == self.plant_col
                            && item.grid_y == self.base.row
                    })
                });
            if let Some(idx) = a_ladder_idx {
                if let Some(board) = self.base.get_board_mut() {
                    board.grid_items[idx].grid_item_die();
                }
            }
        }

        // [TRANSLATION_NOTE]: C++ 中花盆顶部植物的 pot reanim 动画加速依赖 reanim 系统（stub），暂略
    }

    /// 绘制植物（对应 C++ Plant::Draw 简化：绘制 body reanim + 阴影）
    pub fn draw(&self, g: &mut Graphics) {
        // 位置已在 update_abilities 中通过 SetPosition 同步
        if let Some(app) = self.base.get_app() {
            if let Some(reanim) = app.reanimation_get(self.body_reanim_id) {
                reanim.draw(g);
                self.draw_shadow(g, 0.0, 0.0);
                return;
            }
        }

        // 无 reanim 时的占位回退：按种子类型着色（便于整体渲染验证）
        let (r, gr, b) = match self.seed_type {
            SeedType::Sunflower | SeedType::Twinsunflower => (240, 200, 40),
            SeedType::Wallnut | SeedType::Tallnut => (150, 100, 60),
            SeedType::Cherrybomb | SeedType::Jalapeno | SeedType::Doomshroom => (220, 40, 40),
            SeedType::Lilypad | SeedType::Tanglekelp | SeedType::Seashroom => (60, 160, 200),
            _ => (80, 160, 70),
        };
        g.set_color(&crate::framework::color::Color::new(r, gr, b, 255));
        g.fill_rect_xywh(self.base.x, self.base.y, 60, 60);
        self.draw_shadow(g, 0.0, 0.0);
    }

    /// 绘制影子（对应 C++ Plant::DrawShadow，半透明占位）
    pub fn draw_shadow(&self, g: &mut Graphics, _offset_x: f32, _offset_y: f32) {
        g.set_color(&crate::framework::color::Color::new(0, 0, 0, 70));
        g.fill_rect_xywh(self.base.x + 2, self.base.y + 18, 56, 6);
    }

    /// 获取豌豆头偏移（对应 C++ Plant::GetPeaHeadOffset）
    /// 返回 (offset_x, offset_y)，取 body reanim 的 anim_stem 或 anim_idle 轨道当前变换
    pub fn get_pea_head_offset(&self) -> (i32, i32) {
        let mut a_track_index = 0;
        if let Some(app) = self.base.get_app() {
            if let Some(body) = app.reanimation_get(self.body_reanim_id) {
                if body.track_exists("anim_stem") {
                    a_track_index = body.find_track_index("anim_stem");
                } else if body.track_exists("anim_idle") {
                    a_track_index = body.find_track_index("anim_idle");
                }
                let mut a_transform = crate::todlib::definition::ReanimatorTransform::default();
                if body.get_current_transform(a_track_index, &mut a_transform) {
                    return (a_transform.m_trans_x as i32, a_transform.m_trans_y as i32);
                }
            }
        }
        (0, 0)
    }

    /// 窝瓜找目标（对应 C++ Plant::FindSquashTarget）
    /// 返回命中僵尸索引；无目标返回 None
    pub fn find_squash_target(&self) -> Option<usize> {
        let a_damage_range_flags = self.get_damage_range_flags(PlantWeapon::Primary);
        let a_attack_rect = self.get_plant_attack_rect(PlantWeapon::Primary);
        let board = self.base.board?;
        let b = unsafe { &*board };
        let mut a_closest_range = 0;
        let mut a_closest_idx: Option<usize> = None;
        for (idx, zombie) in b.zombies.iter().enumerate() {
            if zombie.dead { continue; }
            if (zombie.base.row == self.base.row || zombie.zombie_type == ZombieType::Boss)
                && zombie.has_head
                && !zombie.is_tangle_kelp_target()
                && zombie.effected_by_damage(a_damage_range_flags)
            {
                let a_zombie_rect = zombie.get_zombie_rect();
                let valid_phase = (zombie.zombie_phase == ZombiePhase::PolevaulterPreVault && a_zombie_rect.x < self.base.x + 20)
                    || (zombie.zombie_phase != ZombiePhase::PolevaulterPreVault
                        && zombie.zombie_phase != ZombiePhase::PolevaulterInVault
                        && zombie.zombie_phase != ZombiePhase::SnorkelIntoPool
                        && zombie.zombie_phase != ZombiePhase::DolphinIntoPool
                        && zombie.zombie_phase != ZombiePhase::DolphinRiding
                        && zombie.zombie_phase != ZombiePhase::DolphinInJump
                        && !zombie.is_bobsled_team_with_sled());
                if !valid_phase { continue; }
                let a_range = -crate::lawn::board::get_rect_overlap(&a_attack_rect, &a_zombie_rect);
                if a_range <= if zombie.is_eating { 110 } else { 70 } {
                    let mut a_plant_x = a_attack_rect.x;
                    if zombie.zombie_phase == ZombiePhase::PolevaulterPostVault
                        || zombie.zombie_phase == ZombiePhase::PolevaulterPreVault
                        || zombie.zombie_phase == ZombiePhase::DolphinWalkingInPool
                        || zombie.zombie_type == ZombieType::Imp
                        || zombie.zombie_type == ZombieType::Football
                    {
                        a_plant_x = a_attack_rect.x - 60;
                    }
                    if zombie.is_walking_backwards() || a_zombie_rect.x + a_zombie_rect.width >= a_plant_x {
                        if a_closest_idx.is_none() || a_range < a_closest_range {
                            a_closest_idx = Some(idx);
                            a_closest_range = a_range;
                        }
                    }
                }
            }
        }
        a_closest_idx
    }

    /// 窝瓜碾压伤害（对应 C++ DoSquashDamage）
    pub fn do_squash_damage(&mut self) {
        let a_damage_range_flags = self.get_damage_range_flags(PlantWeapon::Primary);
        let a_attack_rect = self.get_plant_attack_rect(PlantWeapon::Primary);
        let my_row = self.base.row;

        if let Some(board) = self.base.get_board_mut() {
            for (_idx, zombie) in board.zombies.iter_mut().enumerate() {
                if zombie.dead { continue; }
                if (zombie.base.row == my_row || zombie.zombie_type == ZombieType::Boss) {
                    let z_rect = zombie.get_zombie_rect();
                    if crate::lawn::board::get_rect_overlap(&a_attack_rect, &z_rect) > 0 {                        zombie.take_damage(1800, 18u32);
                    }
                }
            }
        }
    }

    /// 行范围伤害（对应 C++ DoRowAreaDamage）
    pub fn do_row_area_damage(&mut self, damage: i32, damage_flags: u32) {
        let a_damage_range_flags = self.get_damage_range_flags(PlantWeapon::Primary);
        let a_attack_rect = self.get_plant_attack_rect(PlantWeapon::Primary);
        let my_row = self.base.row;
        let my_seed_type = self.seed_type;
        let is_on_high_ground = self.is_on_high_ground();
        let board = match self.base.board { Some(b) => b, None => return };

        // 借用规避：先在 board 借用内收集命中目标索引，退出后统一结算
        let mut targets: Vec<(usize, ZombieType)> = Vec::new();
        unsafe {
            let b = &*board;
            for (idx, zombie) in b.zombies.iter().enumerate() {
                if zombie.dead { continue; }
                let a_diff_y = if zombie.zombie_type == ZombieType::Boss {
                    0
                } else {
                    zombie.base.row - my_row
                };
                if my_seed_type == SeedType::Gloomshroom {
                    if a_diff_y < -1 || a_diff_y > 1 { continue; }
                } else if a_diff_y != 0 {
                    continue;
                }

                if zombie.on_high_ground == is_on_high_ground
                    && zombie.effected_by_damage(a_damage_range_flags)
                {
                    let z_rect = zombie.get_zombie_rect();
                    if crate::lawn::board::get_rect_overlap(&a_attack_rect, &z_rect) > 0 {
                        targets.push((idx, zombie.zombie_type));
                    }
                }
            }
        }

        for (idx, zombie_type) in targets {
            let is_zamboni_or_catapult =
                zombie_type == ZombieType::Zamboni || zombie_type == ZombieType::Catapult;

            // 对应 C++: 地刺击中轮胎/投石车：aDamage = 1800；若带 DAMAGE_SPIKE，
            // 尖刺石自身掉血/死亡（先于僵尸结算，与 C++ 顺序一致）
            if is_zamboni_or_catapult && crate::lawn::zombie::test_bit(damage_flags, 5) {
                if self.seed_type == SeedType::Spikerock {
                    self.spike_rock_take_damage();
                } else {
                    self.die();
                }
            }

            let a_damage = if is_zamboni_or_catapult {
                1800
            } else {
                damage
            };
            if let Some(board) = self.base.get_board_mut() {
                if let Some(zombie) = board.zombies.get_mut(idx) {
                    zombie.take_damage(a_damage, damage_flags);
                }
            }
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Splat as i32);
            }
        }
    }

    /// 获取伤害范围标志（对应 C++ GetDamageRangeFlags）
    pub fn get_damage_range_flags(&self, weapon: PlantWeapon) -> u32 {
        match self.seed_type {
            SeedType::Cactus => {
                if weapon == PlantWeapon::Secondary { 1 } else { 2 }
            }
            SeedType::Cherrybomb | SeedType::Jalapeno | SeedType::Cobcannon | SeedType::Doomshroom => 127,
            SeedType::Melonpult | SeedType::Cabbagepult | SeedType::Kernelpult | SeedType::Wintermelon => 13,
            SeedType::PotatoMine => 77,
            SeedType::Squash => 13,
            SeedType::Puffshroom | SeedType::Seashroom | SeedType::Fumeshroom
            | SeedType::Gloomshroom | SeedType::Chomper => 9,
            SeedType::Cattail => 11,
            SeedType::Tanglekelp => 5,
            SeedType::GiantWallnut => 17,
            _ => 1,
        }
    }

    /// 获取植物攻击矩形（对应 C++ GetPlantAttackRect）
    pub fn get_plant_attack_rect(&self, weapon: PlantWeapon) -> Rect {
        let m_x = self.base.x;
        let m_y = self.base.y;
        let m_width = self.base.width;
        let m_height = self.base.height;
        if self.base.get_app().map_or(false, |app| app.is_wallnut_bowling_level()) {
            return Rect::new(m_x, m_y, m_width - 20, m_height);
        } else if weapon == PlantWeapon::Secondary && self.seed_type == SeedType::Splitpea {
            return Rect::new(0, m_y, m_x + 16, m_height);
        }
        match self.seed_type {
            SeedType::Leftpeater => Rect::new(0, m_y, m_x, m_height),
            SeedType::Squash => Rect::new(m_x + 20, m_y, m_width - 35, m_height),
            SeedType::Chomper => Rect::new(m_x + 80, m_y, 40, m_height),
            SeedType::Spikeweed | SeedType::Spikerock => Rect::new(m_x + 20, m_y, m_width - 50, m_height),
            SeedType::PotatoMine => Rect::new(m_x, m_y, m_width - 25, m_height),
            SeedType::Torchwood => Rect::new(m_x + 50, m_y, 30, m_height),
            SeedType::Puffshroom | SeedType::Seashroom => Rect::new(m_x + 60, m_y, 230, m_height),
            SeedType::Fumeshroom => Rect::new(m_x + 60, m_y, 340, m_height),
            SeedType::Gloomshroom => Rect::new(m_x - 80, m_y - 80, 240, 240),
            SeedType::Tanglekelp => Rect::new(m_x, m_y, m_width, m_height),
            SeedType::Cattail => Rect::new(-BOARD_WIDTH, -BOARD_HEIGHT, BOARD_WIDTH * 2, BOARD_HEIGHT * 2),
            _ => Rect::new(m_x + 60, m_y, BOARD_WIDTH, m_height),
        }
    }

    /// 到最近僵尸的距离（对应 C++ DistanceToClosestZombie）
    pub fn distance_to_closest_zombie(&self) -> i32 {
        let damage_range_flags = self.get_damage_range_flags(PlantWeapon::Primary);
        let attack_rect = self.get_plant_attack_rect(PlantWeapon::Primary);
        let mut closest_distance = 1000;
        let board = match self.base.board { Some(b) => b, None => return closest_distance };
        unsafe {
            let b = &*board;
            for zombie in &b.zombies {
                if zombie.dead { continue; }
                if zombie.base.row == self.base.row && zombie.effected_by_damage(damage_range_flags) {
                    let zombie_rect = zombie.get_zombie_rect();
                    let distance = -crate::lawn::board::get_rect_overlap(&attack_rect, &zombie_rect);
                    if distance < closest_distance {
                        closest_distance = distance.max(0);
                    }
                }
            }
        }
        closest_distance
    }

    /// 更新特殊能力（对应 C++ UpdateAbilities）
    pub fn update_abilities(&mut self) {
        if !self.is_in_play() {
            return;
        }

        // 同步 body reanim 位置（对应 C++ UpdateReanim 的 SetPosition）
        if self.body_reanim_id != REANIMATIONID_NULL {
            let a_offset_y = Self::plant_draw_height_offset(self.base.get_board(), Some(self), self.seed_type, self.plant_col, self.base.row);
            if let Some(app) = self.base.get_app_mut() {
                if let Some(reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                    reanim.set_position(self.pos_x, self.pos_y + a_offset_y);
                }
            }
        }

        // 正在消失或被压碎
        if self.state == PlantState::DoingSpecial || self.squished {
            self.disappear_countdown -= 1;
            if self.disappear_countdown < 0 {
                self.die();
                return;
            }
        }

        // 唤醒倒计时
        if self.wake_up_counter > 0 {
            self.wake_up_counter -= 1;
            if self.wake_up_counter == 0 {
                self.set_sleeping(false);
            }
        }

        if self.is_asleep || self.squished || self.on_bungee_state != PlantOnBungeeState::NotOnBungee {
            return;
        }

        // 更新射击
        self.update_shooting();

        // 状态计数
        if self.state_countdown > 0 {
            self.state_countdown -= 1;
        }

        // C++: if (mApp->IsWallnutBowlingLevel()) { UpdateBowling(); return; }
        if self.base.get_app().map_or(false, |app| app.is_wallnut_bowling_level()) {
            self.update_bowling();
            return;
        }

        // 特殊植物更新分发
        match self.seed_type {
            SeedType::Squash => self.update_squash(),
            SeedType::Doomshroom => self.update_doom_shroom(),
            SeedType::Iceshroom => self.update_ice_shroom(),
            SeedType::Chomper => self.update_chomper(),
            SeedType::Blover => self.update_blover(),
            SeedType::Flowerpot => self.update_flower_pot(),
            SeedType::Lilypad => self.update_lilypad(),
            SeedType::Imitater => self.update_imitater(),
            SeedType::InstantCoffee => self.update_coffee_bean(),
            SeedType::Umbrella => self.update_umbrella(),
            SeedType::Cobcannon => self.update_cob_cannon(),
            SeedType::Cactus => self.update_cactus(),
            SeedType::Magnetshroom => self.update_magnet_shroom(),
            SeedType::GoldMagnet => self.update_gold_magnet_shroom(),
            SeedType::Sunshroom => self.update_sun_shroom(),
            // 对应 C++: else if (MakesSun() || mSeedType == SEED_MARIGOLD) UpdateProductionPlant();
            SeedType::Sunflower | SeedType::Twinsunflower | SeedType::Marigold => self.update_production_plant(),
            SeedType::Gravebuster => self.update_grave_buster(),
            SeedType::Torchwood => self.update_torchwood(),
            SeedType::PotatoMine => self.update_potato(),
            SeedType::Spikeweed | SeedType::Spikerock => self.update_spikeweed(),
            SeedType::Tanglekelp => self.update_tanglekelp(),
            SeedType::Scaredyshroom => self.update_scaredy_shroom(),
            _ => {}
        }

        // 射手类更新
        if self.subclass == PlantSubClass::Shooter as i32 {
            self.update_shooter();
        }

        // 特殊倒计时
        if self.do_special_countdown > 0 {
            self.do_special_countdown -= 1;
            if self.do_special_countdown == 0 {
                self.do_special();
            }
        }
    }

    /// 更新重动画颜色（对应 C++ UpdateReanimColor，Plant.cpp:2640）
    pub fn update_reanim_color(&mut self) {
        if !self.is_on_board() {
            return;
        }

        // C++: aBodyReanim = mApp->ReanimationTryToGet(mBodyReanimID); if (!aBodyReanim) return;
        if self
            .base
            .get_app()
            .and_then(|app| app.reanimation_get(self.body_reanim_id))
            .is_none()
        {
            return;
        }

        let a_seed_type = self.base.get_board().map_or(SeedType::None, |b| b.get_seed_type_in_cursor());
        let mut a_color_override;

        // C++: mBoard->mCursorObject->mCursorType == CURSOR_TYPE_PLANT_FROM_GLOVE
        let mut is_on_glove = false;
        if let Some(board) = self.base.get_board() {
            if board.cursor_object.cursor_type == CursorType::PlantFromGlove {
                let a_glove_plant_id = board.cursor_object.glove_plant_id;
                // C++: mBoard->mPlants.DataArrayTryToGet(aGlovePlantID)
                let a_plant = board.plants.get(a_glove_plant_id as usize).map(|p| p as *const Plant);
                if let Some(a_plant) = a_plant {
                    unsafe {
                        if (*a_plant).plant_col == self.plant_col && (*a_plant).base.row == self.base.row {
                            is_on_glove = true;
                        }
                    }
                }
            }
        }

        if is_on_glove {
            a_color_override = Color::new(128, 128, 128, 255);
        } else if self.is_part_of_upgradable_to(a_seed_type)
            && self
                .base
                .get_board()
                .map_or(false, |b| b.can_plant_at(self.plant_col, self.base.row, a_seed_type) == PlantingReason::Ok)
        {
            let a_counter = self.base.get_board().map_or(0, |b| b.m_main_counter);
            a_color_override = crate::todlib::tod_common::get_flashing_color(a_counter, 90);
        } else if a_seed_type == SeedType::Cobcannon
            && self.seed_type == SeedType::Kernelpult
            && self
                .base
                .get_board()
                .map_or(false, |b| b.can_plant_at(self.plant_col - 1, self.base.row, a_seed_type) == PlantingReason::Ok)
        {
            let a_counter = self.base.get_board().map_or(0, |b| b.m_main_counter);
            a_color_override = crate::todlib::tod_common::get_flashing_color(a_counter, 90);
        } else if self.seed_type == SeedType::ExplodeONut {
            a_color_override = Color::new(255, 64, 64, 255);
        } else {
            a_color_override = Color::new(255, 255, 255, 255);
        }

        // C++: aBodyReanim 的颜色/附加绘制设置（此处重新取 mutable 引用应用）
        if let Some(a_body_reanim) = self
            .base
            .get_app_mut()
            .and_then(|app| app.reanimation_get_mut(self.body_reanim_id))
        {
            a_body_reanim.m_color_override = a_color_override;

            if self.highlighted {
                a_body_reanim.m_extra_additive_color = Color::new(255, 255, 255, 196);
                a_body_reanim.m_enable_extra_additive_draw = true;
                if self.imitater_type == SeedType::Imitater {
                    a_body_reanim.m_extra_additive_color = Color::new(255, 255, 255, 92);
                }
            } else if self.beghouled_flash_countdown > 0 {
                let an_alpha = crate::todlib::tod_common::tod_animate_curve(
                    50, 0, self.beghouled_flash_countdown % 50, 1, 128, TodCurves::Bounce,
                );
                a_body_reanim.m_extra_additive_color = Color::new(255, 255, 255, an_alpha as u8);
                a_body_reanim.m_enable_extra_additive_draw = true;
            } else if self.eaten_flash_countdown > 0 {
                let a_grayness = crate::todlib::tod_common::clamp_int(
                    self.eaten_flash_countdown * 3,
                    0,
                    if self.imitater_type == SeedType::Imitater { 128 } else { 255 },
                ) as u8;
                a_body_reanim.m_extra_additive_color = Color::new(a_grayness, a_grayness, a_grayness, 255);
                a_body_reanim.m_enable_extra_additive_draw = true;
            } else {
                a_body_reanim.m_enable_extra_additive_draw = false;
            }

            if self.beghouled_flash_countdown > 0 {
                let an_alpha = crate::todlib::tod_common::tod_animate_curve(
                    50, 0, self.beghouled_flash_countdown % 50, 1, 128, TodCurves::Bounce,
                );
                a_body_reanim.m_extra_overlay_color = Color::new(255, 255, 255, an_alpha as u8);
                a_body_reanim.m_enable_extra_overlay_draw = true;
            } else {
                a_body_reanim.m_enable_extra_overlay_draw = false;
            }

            // [TRANSLATION_NOTE]: C++ 末尾 aBodyReanim->PropogateColorToAttachments()
            // （把颜色覆盖传播给附件 reanim）；Rust 附件颜色传播未接入
        }
    }

    /// 更新重动画位置/缩放（对应 C++ Plant::UpdateReanim，Plant.cpp:2739）
    pub fn update_reanim(&mut self) {
        // ReanimationTryToGet(mBodyReanimID)：取不到直接返回
        let has_body_reanim = {
            let app = match self.base.get_app_mut() {
                Some(a) => a,
                None => return,
            };
            app.reanimation_get_mut(self.body_reanim_id).is_some()
        };
        if !has_body_reanim {
            return;
        }

        self.update_reanim_color();

        let seed_type = self.seed_type;
        let mut a_offset_x = self.shake_offset_x;
        let mut a_offset_y = Self::plant_draw_height_offset(
            self.base.get_board(), Some(self), seed_type, self.plant_col, self.base.row,
        );
        let mut a_scale_x = 1.0;
        let mut a_scale_y = 1.0;

        let a_game_mode = self.base.get_app().map(|a| a.game_mode);
        if a_game_mode == Some(GameMode::ChallengeBigTime)
            && (seed_type == SeedType::Wallnut
                || seed_type == SeedType::Sunflower
                || seed_type == SeedType::Marigold)
        {
            a_scale_x = 1.5;
            a_scale_y = 1.5;
            a_offset_x -= 20.0;
            a_offset_y -= 40.0;
        }
        if seed_type == SeedType::GiantWallnut {
            a_scale_x = 2.0;
            a_scale_y = 2.0;
            a_offset_x -= 76.0;
            a_offset_y -= 64.0;
        }
        if seed_type == SeedType::InstantCoffee {
            a_scale_x = 0.8;
            a_scale_y = 0.8;
            a_offset_x += 12.0;
            a_offset_y += 10.0;
        }
        if seed_type == SeedType::PotatoMine {
            a_scale_x = 0.8;
            a_scale_y = 0.8;
            a_offset_x += 12.0;
            a_offset_y += 12.0;
        }
        if self.state == PlantState::GravebusterEating {
            a_offset_y += crate::todlib::tod_common::tod_animate_curve_float(
                400, 0, self.state_countdown, 0.0, 30.0, TodCurves::Linear,
            );
        }
        if self.wake_up_counter > 0 {
            let a_scale_factor = crate::todlib::tod_common::tod_animate_curve_float(
                70, 0, self.wake_up_counter, 1.0, 0.8, TodCurves::EaseSinWave,
            );
            a_scale_y *= a_scale_factor;
            a_offset_y += 80.0 - 80.0 * a_scale_factor;
        }

        // aBodyReanim->Update()
        if let Some(app) = self.base.get_app_mut() {
            if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                a_body_reanim.update();
            }
        }

        if seed_type == SeedType::Leftpeater {
            a_offset_x += 80.0 * a_scale_x;
            a_scale_x *= -1.0;
        }

        // 盆栽分支（mPottedPlantIndex != -1）
        if self.potted_plant_index != -1 {
            let potted_index = self.potted_plant_index as usize;
            let zen_garden = self.base.get_app().and_then(|app| app.zen_garden);
            if let Some(zg) = zen_garden {
                unsafe {
                    let a_potted_plant = (*zg).potted_plant_from_index(potted_index);
                    if let Some(a_potted_plant) = a_potted_plant {
                        if (*a_potted_plant).facing == crate::lawn::system::player_info::FacingDirection::Left {
                            a_offset_x += 80.0 * a_scale_x;
                            a_scale_x *= -1.0;
                        }

                        let mut a_offset_x_start = 0.0;
                        let mut a_offset_x_end = 0.0;
                        let mut a_offset_y_start = 0.0;
                        let mut a_offset_y_end = 0.0;
                        let mut a_scale_start = 0.0;
                        let mut a_scale_end = 0.0;
                        if (*a_potted_plant).plant_age == PottedPlantAge::Small {
                            a_offset_x_start = 20.0;
                            a_offset_x_end = 20.0;
                            a_offset_y_start = 40.0;
                            a_offset_y_end = 40.0;
                            a_scale_start = 0.5;
                            a_scale_end = 0.5;
                        } else if (*a_potted_plant).plant_age == PottedPlantAge::Medium {
                            a_offset_x_start = 20.0;
                            a_offset_x_end = 10.0;
                            a_offset_y_start = 40.0;
                            a_offset_y_end = 20.0;
                            a_scale_start = 0.5;
                            a_scale_end = 0.75;
                        } else {
                            a_offset_x_start = 10.0;
                            a_offset_x_end = 0.0;
                            a_offset_y_start = 20.0;
                            a_offset_y_end = 0.0;
                            a_scale_start = 0.75;
                            a_scale_end = 1.0;
                        }

                        let a_animated_offset_x = crate::todlib::tod_common::tod_animate_curve_float(
                            100, 0, self.state_countdown, a_offset_x_start, a_offset_x_end, TodCurves::Linear,
                        );
                        let a_animated_offset_y = crate::todlib::tod_common::tod_animate_curve_float(
                            100, 0, self.state_countdown, a_offset_y_start, a_offset_y_end, TodCurves::Linear,
                        );
                        let a_animated_scale = crate::todlib::tod_common::tod_animate_curve_float(
                            100, 0, self.state_countdown, a_scale_start, a_scale_end, TodCurves::Linear,
                        );

                        a_offset_x += a_animated_offset_x * a_scale_x;
                        a_offset_y += a_animated_offset_y * a_scale_y;
                        a_scale_x *= a_animated_scale;
                        a_scale_y *= a_animated_scale;
                        a_offset_x += crate::lawn::zen_garden::ZenGarden::zen_plant_offset_x(&*a_potted_plant);
                        a_offset_y += (*zg).plant_potted_draw_height_offset(seed_type, a_scale_y);
                    }
                }
            }
        }

        // aBodyReanim->SetPosition(aOffsetX, aOffsetY); OverrideScale(aScaleX, aScaleY)
        if let Some(app) = self.base.get_app_mut() {
            if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                a_body_reanim.set_position(a_offset_x, a_offset_y);
                a_body_reanim.override_scale(a_scale_x, a_scale_y);
            }
        }
    }

    /// 坚果类受损动画（对应 C++ AnimateNuts，Plant.cpp:3090）
// [TRANSLATION_NOTE]: C++ 使用 IMAGE_REANIM_WALLNUT_CRACKED1/2 等全局图片资源；
// Rust 图片未接入，用两个静态哨兵指针表示不同裂纹等级，使 Get/SetImageOverride 的
// 记录比较成立（C++ 以 Image* 指针比较决定裂纹覆盖与粒子只触发一次）。
    pub fn animate_nuts(&mut self) {
        let a_track_to_override = match self.seed_type {
            SeedType::Wallnut => "anim_face",
            SeedType::Tallnut => "anim_idle",
            _ => return,
        };
        // C++: aCracked1/aCracked2 = IMAGE_REANIM_*_CRACKED1/2；哨兵指针代替（接入图片后替换为资源指针）
        let a_cracked1: *mut crate::framework::graphics::image::Image = &CRACK_IMAGE_1 as *const u8 as *mut _;
        let a_cracked2: *mut crate::framework::graphics::image::Image = &CRACK_IMAGE_2 as *const u8 as *mut _;

        let mut a_pos_x = self.pos_x as i32 + 40;
        let mut a_pos_y = self.pos_y as i32 + 10;
        if self.seed_type == SeedType::Tallnut {
            a_pos_y -= 32;
        }

        // C++: aBodyReanim->GetImageOverride / SetImageOverride + AddPvzpParticle
        let mut a_spawn_particle = false;
        if let Some(a_body_reanim) = self
            .base
            .get_app_mut()
            .and_then(|app| app.reanimation_get_mut(self.body_reanim_id))
        {
            let a_image_override = a_body_reanim.get_image_override(a_track_to_override);
            if self.plant_health < self.plant_max_health / 3 {
                if a_image_override != a_cracked2 {
                    a_body_reanim.set_image_override(a_track_to_override, a_cracked2);
                    a_spawn_particle = true;
                }
            } else if self.plant_health < self.plant_max_health * 2 / 3 {
                if a_image_override != a_cracked1 {
                    a_body_reanim.set_image_override(a_track_to_override, a_cracked1);
                    a_spawn_particle = true;
                }
            } else {
                a_body_reanim.set_image_override(a_track_to_override, std::ptr::null_mut());
            }
        }
        if a_spawn_particle {
            self.add_attached_particle(a_pos_x, a_pos_y, self.base.render_order + 4, ParticleEffect::WallnutEatLarge);
        }

        // C++: if (IsInPlay() && !mApp->IsIZombieLevel())
        if self.is_in_play() {
            if let Some(app) = self.base.get_app() {
                if app.is_izombie_level() {
                    return;
                }
            }
            if let Some(a_body_reanim) = self
                .base
                .get_app_mut()
                .and_then(|app| app.reanimation_get_mut(self.body_reanim_id))
            {
                if self.recently_eaten_countdown > 0 {
                    a_body_reanim.m_anim_rate = 0.1;
                    return;
                }
                if a_body_reanim.m_anim_rate < 1.0 && self.on_bungee_state != PlantOnBungeeState::RisingWithBungee {
                    a_body_reanim.m_anim_rate = crate::todlib::tod_common::rand_range_float(10.0, 15.0);
                }
            }
        }
    }

    /// 大蒜受损动画（对应 C++ AnimateGarlic，Plant.cpp:3157）
    pub fn animate_garlic(&mut self) {
        // [TRANSLATION_NOTE]: C++ 使用 IMAGE_REANIM_GARLIC_BODY2/BODY3 全局图片，Rust 图片未接入用空指针占位
        let a_body_reanim = self
            .base
            .get_app_mut()
            .and_then(|app| app.reanimation_get_mut(self.body_reanim_id));
        let Some(a_body_reanim) = a_body_reanim else { return; };
        let a_image_override = a_body_reanim.get_image_override("anim_face");

        if self.plant_health < self.plant_max_health / 3 {
            // C++: aImageOverride != IMAGE_REANIM_GARLIC_BODY3 → SetImageOverride(BODY3) + 隐藏茎
            if a_image_override != (&CRACK_IMAGE_2 as *const u8 as *mut crate::framework::graphics::image::Image) {
                a_body_reanim.set_image_override("anim_face", &CRACK_IMAGE_2 as *const u8 as *mut _);
                a_body_reanim.assign_render_group_to_prefix("Garlic_stem", -1); // RENDER_GROUP_HIDDEN
            }
        } else if self.plant_health < self.plant_max_health * 2 / 3 {
            // C++: aImageOverride != IMAGE_REANIM_GARLIC_BODY2 → SetImageOverride(BODY2)
            if a_image_override != (&CRACK_IMAGE_1 as *const u8 as *mut crate::framework::graphics::image::Image) {
                a_body_reanim.set_image_override("anim_face", &CRACK_IMAGE_1 as *const u8 as *mut _);
            }
        } else {
            // C++: SetImageOverride(nullptr) —— 健康恢复时清除覆盖
            a_body_reanim.set_image_override("anim_face", std::ptr::null_mut());
        }
    }

    /// 南瓜受损动画（对应 C++ AnimatePumpkin，Plant.cpp:3183）
    pub fn animate_pumpkin(&mut self) {
        // [TRANSLATION_NOTE]: C++ 使用 IMAGE_REANIM_PUMPKIN_DAMAGE1/DAMAGE3，图片未接入用哨兵指针
        let a_body_reanim = self
            .base
            .get_app_mut()
            .and_then(|app| app.reanimation_get_mut(self.body_reanim_id));
        let Some(a_body_reanim) = a_body_reanim else { return; };
        let a_image_override = a_body_reanim.get_image_override("Pumpkin_front");

        if self.plant_health < self.plant_max_health / 3 {
            // C++: != IMAGE_REANIM_PUMPKIN_DAMAGE3 → SetImageOverride(DAMAGE3)
            if a_image_override != (&CRACK_IMAGE_2 as *const u8 as *mut crate::framework::graphics::image::Image) {
                a_body_reanim.set_image_override("Pumpkin_front", &CRACK_IMAGE_2 as *const u8 as *mut _);
            }
        } else if self.plant_health < self.plant_max_health * 2 / 3 {
            // C++: != IMAGE_REANIM_PUMPKIN_DAMAGE1 → SetImageOverride(DAMAGE1)
            if a_image_override != (&CRACK_IMAGE_1 as *const u8 as *mut crate::framework::graphics::image::Image) {
                a_body_reanim.set_image_override("Pumpkin_front", &CRACK_IMAGE_1 as *const u8 as *mut _);
            }
        } else {
            // C++: SetImageOverride(nullptr)
            a_body_reanim.set_image_override("Pumpkin_front", std::ptr::null_mut());
        }
    }

    /// 磁铁物是否绘制在顶层（对应 C++ DrawMagnetItemsOnTop，Plant.cpp:1971）
    pub fn draw_magnet_items_on_top(&self) -> bool {
        if self.seed_type == SeedType::GoldMagnet {
            for item in &self.magnet_items {
                if item.item_type != MagnetItemType::None {
                    return true;
                }
            }
            return false;
        }

        if self.seed_type == SeedType::Magnetshroom {
            for item in &self.magnet_items {
                if item.item_type != MagnetItemType::None {
                    // C++: SexyVector2 aVectorToPlant(mX + mDestOffsetX - mPosX, mY + mDestOffsetY - mPosY)
                    let a_dx = self.pos_x + item.dest_offset_x - item.pos_x;
                    let a_dy = self.pos_y + item.dest_offset_y - item.pos_y;
                    if (a_dx * a_dx + a_dy * a_dy).sqrt() > 20.0 {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// 绘制磁铁吸附物（对应 C++ DrawMagnetItems，Plant.cpp:3691）
    pub fn draw_magnet_items(&self, g: &mut Graphics) {
        let a_offset_x = 0.0f32;
        let a_offset_y = Self::plant_draw_height_offset(
            self.base.get_board(),
            Some(self),
            self.seed_type,
            self.plant_col,
            self.base.row,
        );

        for item in &self.magnet_items {
            if item.item_type == MagnetItemType::None {
                continue;
            }
            let mut a_cel_row = 0;
            let mut a_cel_col = 0;
            let mut a_image: *mut crate::framework::graphics::image::Image = std::ptr::null_mut();
            let mut a_scale = 0.8f32;

            // [TRANSLATION_NOTE]: C++ 使用 IMAGE_REANIM_ZOMBIE_BUCKET1/2/3、FOOTBALL_HELMET、
            // SCREENDOOR、LADDER、JACKBOX、DIGGER_PICKAXE、COIN_*、DIAMOND 等全局图片，
            // Rust 图片资源未接入，用空指针占位
            match item.item_type {
                MagnetItemType::Pail1 => {}
                MagnetItemType::Pail2 => {}
                MagnetItemType::Pail3 => {}
                MagnetItemType::FootballHelmet1 => {}
                MagnetItemType::FootballHelmet2 => {}
                MagnetItemType::FootballHelmet3 => {}
                MagnetItemType::Door1 => {}
                MagnetItemType::Door2 => {}
                MagnetItemType::Door3 => {}
                MagnetItemType::Pogo1 | MagnetItemType::Pogo2 | MagnetItemType::Pogo3 => {
                    a_cel_col = item.item_type as i32 - MagnetItemType::Pogo1 as i32;
                }
                MagnetItemType::Ladder1 => {}
                MagnetItemType::Ladder2 => {}
                MagnetItemType::Ladder3 => {}
                MagnetItemType::LadderPlaced => {}
                MagnetItemType::JackInTheBox => {}
                MagnetItemType::PickAxe => {}
                MagnetItemType::SilverCoin | MagnetItemType::GoldCoin | MagnetItemType::Diamond => {
                    a_scale = 1.0;
                }
                MagnetItemType::None => {}
            }

            if a_image.is_null() {
                continue;
            }

            let a_pos_x = (item.pos_x - self.pos_x + a_offset_x) as i32;
            let a_pos_y = (item.pos_y - self.pos_y + a_offset_y) as i32;
            unsafe {
                let a_img = &*a_image;
                if a_scale == 1.0 {
                    // C++: g->DrawImageCel(aImage, x, y, aCelCol, aCelRow)
                    g.draw_image_cel_rc(a_img, a_pos_x, a_pos_y, a_cel_col, a_cel_row);
                } else {
                    // [TRANSLATION_NOTE]: C++ PvzpDrawImageCelScaledF 用矩阵绕 cel 中心缩放；
                    // Rust 无矩阵 cel 绘制，用 dest_rect 缩放近似（保持左上角位置）
                    let cw = a_img.width / a_img.num_cols.max(1);
                    let ch = a_img.height / a_img.num_rows.max(1);
                    let dest = crate::framework::rect::Rect::new(
                        a_pos_x,
                        a_pos_y,
                        (cw as f32 * a_scale) as i32,
                        (ch as f32 * a_scale) as i32,
                    );
                    g.draw_image_cel_dest_rc(a_img, &dest, a_cel_col, a_cel_row);
                }
            }
        }
    }

    /// 保龄球滚动更新（对应 C++ UpdateBowling，Plant.cpp:2361）
    pub fn update_bowling(&mut self) {
        // C++: Reanimation* aBodyReanim = mApp->ReanimationTryToGet(mBodyReanimID);
        //      if (aBodyReanim && aBodyReanim->TrackExists("_ground")) { mX -= aSpeed; }
        self.pos_x -= self
            .base
            .get_app()
            .and_then(|app| app.reanimation_get(self.body_reanim_id))
            .and_then(|r| if r.track_exists("_ground") { Some(r.get_track_velocity("_ground")) } else { None })
            .map_or(0.0, |a_speed| {
                if self.seed_type == SeedType::GiantWallnut {
                    a_speed * 2.0
                } else {
                    a_speed
                }
            });
        if self.pos_x > 800.0 {
            self.die();
        }

        // C++: mState == STATE_BOWLING_UP → mY -= 2;  else mState == STATE_BOWLING_DOWN → mY += 2
        if self.state == PlantState::BowlingUp {
            self.pos_y -= 2.0;
        } else if self.state == PlantState::BowlingDown {
            self.pos_y += 2.0;
        }
        // C++: int aDistToGrid = mBoard->GridToPixelY(0, mRow) - mY; 超出 ±2 直接返回
        let a_dist_to_grid = self.base.get_board().map_or(0, |b| b.grid_to_pixel_y(0, self.base.row)) - self.pos_y as i32;
        if a_dist_to_grid < -2 || a_dist_to_grid > 2 {
            return;
        }

        let mut a_new_state = self.state;
        if self.state == PlantState::BowlingUp && self.base.row <= 0 {
            a_new_state = PlantState::BowlingDown;
        } else if self.state == PlantState::BowlingDown && self.base.row >= 4 {
            a_new_state = PlantState::BowlingUp;
        }

        // C++: Zombie* aZombie = FindTargetZombie(mRow, WEAPON_PRIMARY);
        let a_target_zombie = self.find_target_zombie(self.base.row, PlantWeapon::Primary);
        if a_target_zombie.is_some() {
            let a_pos_x = self.pos_x as i32 + self.base.width / 2;
            let a_pos_y = self.pos_y as i32 + self.base.height / 2;

            if self.seed_type == SeedType::ExplodeONut {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Cherrybomb as i32);
                    app.play_sample(crate::framework::resources::ResourceId::SoundBowlingimpact2 as i32);
                }
                let a_damage_range_flags = self.get_damage_range_flags(PlantWeapon::Primary) | 32u32;
                let a_row = self.base.row;
                if let Some(board) = self.base.get_board_mut() {
                    board.kill_all_zombies_in_radius(a_row, a_pos_x, a_pos_y, 90, 1, true, a_damage_range_flags);
                    board.shake_board(3, -4);
                }
                self.add_attached_particle(a_pos_x, a_pos_y, crate::lawn::game_enums::RENDER_LAYER_TOP, ParticleEffect::Powie);
                self.die();
                return;
            }

            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::BowlingImpact as i32);
            }
            if let Some(board) = self.base.get_board_mut() {
                board.shake_board(1, -2);
            }

            // C++: 按是否巨核桃/门盾/其他盾/头盔分派伤害
            let a_zombie = a_target_zombie.unwrap();
            let mut a_zombie_take_damage = false;
            let mut a_zombie_take_shield_damage = false;
            let mut a_zombie_take_helm_damage = false;
            if let Some(board) = self.base.get_board_mut() {
                let a_zombie_ref = &mut board.zombies[a_zombie as usize];
                if self.seed_type == SeedType::GiantWallnut {
                    a_zombie_take_damage = true;
                } else if a_zombie_ref.shield_type == ShieldType::Door && self.state != PlantState::NotReady {
                    a_zombie_take_damage = true;
                } else if a_zombie_ref.shield_type != ShieldType::None {
                    a_zombie_take_shield_damage = true;
                } else if a_zombie_ref.helm_type != HelmType::None {
                    if a_zombie_ref.helm_type == HelmType::Pail {
                        if let Some(app) = self.base.get_app() {
                            app.play_foley(crate::todlib::tod_foley::FoleyType::ShieldHit as i32);
                        }
                    } else if a_zombie_ref.helm_type == HelmType::TrafficCone {
                        if let Some(app) = self.base.get_app() {
                            app.play_foley(crate::todlib::tod_foley::FoleyType::PlasticHit as i32);
                        }
                    }
                    a_zombie_take_helm_damage = true;
                } else {
                    a_zombie_take_damage = true;
                }
            }
            // [TRANSLATION_NOTE]: C++ 中 zombie 指针跨多个分支直接调用伤害接口；
            // Rust 借用规则分派后统一执行，行为一致
            if let Some(board) = self.base.get_board_mut() {
                let a_zombie_ref = &mut board.zombies[a_zombie as usize];
                if a_zombie_take_damage {
                    a_zombie_ref.take_damage(1800, 0);
                } else if a_zombie_take_shield_damage {
                    a_zombie_ref.take_shield_damage(400, 0);
                } else if a_zombie_take_helm_damage {
                    a_zombie_ref.take_helm_damage(900, 0);
                }
            }

            // C++: 普通核桃击中 2/3/4/5 次后掉落银币/金币（非首次冒险或已过 10 关）
            let a_drop_coins = self
                .base
                .get_app()
                .map_or(false, |app| !app.is_first_time_adventure_mode() || app.player_info.as_ref().map_or(false, |p| p.get_level() > 10));
            if a_drop_coins && self.seed_type == SeedType::Wallnut {
                self.launch_counter += 1;
                if self.launch_counter == 2 {
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
                    }
                    if let Some(board) = self.base.get_board_mut() {
                        board.add_coin(a_pos_x as f32, a_pos_y as f32, CoinType::Silver, CoinMotion::Coin);
                    }
                } else if self.launch_counter == 3 {
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
                    }
                    if let Some(board) = self.base.get_board_mut() {
                        board.add_coin(a_pos_x as f32 - 5.0, a_pos_y as f32, CoinType::Silver, CoinMotion::Coin);
                        board.add_coin(a_pos_x as f32 + 5.0, a_pos_y as f32, CoinType::Silver, CoinMotion::Coin);
                    }
                } else if self.launch_counter == 4 {
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
                    }
                    if let Some(board) = self.base.get_board_mut() {
                        board.add_coin(a_pos_x as f32 - 10.0, a_pos_y as f32, CoinType::Silver, CoinMotion::Coin);
                        board.add_coin(a_pos_x as f32, a_pos_y as f32, CoinType::Silver, CoinMotion::Coin);
                        board.add_coin(a_pos_x as f32 + 10.0, a_pos_y as f32, CoinType::Silver, CoinMotion::Coin);
                    }
                } else if self.launch_counter >= 5 {
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
                    }
                    if let Some(board) = self.base.get_board_mut() {
                        board.add_coin(a_pos_x as f32, a_pos_y as f32, CoinType::Gold, CoinMotion::Coin);
                    }
                    // [TRANSLATION_NOTE]: C++: ReportAchievement::GiveAchievement(mApp, RollSomeHeads, true)；
                    // Rust 成就系统未接入
                }
            }

            // C++: 非巨核桃：按行号/状态随机转向
            if self.seed_type != SeedType::GiantWallnut {
                if self.base.row == 4 || self.state == PlantState::BowlingDown {
                    a_new_state = PlantState::BowlingUp;
                } else if self.base.row == 0 || self.state == PlantState::BowlingUp {
                    a_new_state = PlantState::BowlingDown;
                } else {
                    // C++: Sexy::Rand(2) ? UP : DOWN
                    a_new_state = if crate::framework::common::rand() % 2 != 0 {
                        PlantState::BowlingUp
                    } else {
                        PlantState::BowlingDown
                    };
                }
            }
        }

        // C++: 应用新状态并移动行
        if a_new_state == PlantState::BowlingUp {
            self.base.row -= 1;
            self.state = PlantState::BowlingUp;
            self.base.render_order = self.calc_render_order();
        } else if a_new_state == PlantState::BowlingDown {
            self.state = PlantState::BowlingDown;
            self.base.render_order = self.calc_render_order();
            self.base.row += 1;
        }
    }

    /// 更新射击（对应 C++ UpdateShooting）
    pub fn update_shooting(&mut self) {
        // 对应 C++: if (NotOnGround() || mShootingCounter == 0) return;
        // [TRANSLATION_NOTE]: C++ NotOnGround() 依赖 mPlantHeight 等字段，Rust 暂无对应，跳过。
        if self.shooting_counter == 0 { return; }

        self.shooting_counter -= 1;

        // 对应 C++: Gloomshroom 冒烟粒子时刻（粒子系统 stub，仅保留 Fire 时刻）
        if self.seed_type == SeedType::Gloomshroom {
            if self.shooting_counter == 126
                || self.shooting_counter == 98
                || self.shooting_counter == 70
                || self.shooting_counter == 42
            {
                self.fire(None, self.base.row, PlantWeapon::Primary);
            }
        } else if self.seed_type == SeedType::Gatlingpea {
            // 对应 C++: Gatlingpea 四连射
            if self.shooting_counter == 18
                || self.shooting_counter == 35
                || self.shooting_counter == 51
                || self.shooting_counter == 68
            {
                self.fire(None, self.base.row, PlantWeapon::Primary);
            }
        } else if self.seed_type == SeedType::Cattail {
            // 对应 C++: Cattail 计数 19 时瞄准当前目标发射
            if self.shooting_counter == 19 {
                let a_target = self.find_target_zombie(self.base.row, PlantWeapon::Primary);
                self.fire(a_target, self.base.row, PlantWeapon::Primary);
            }
        } else if self.shooting_counter == 1 {
            // 对应 C++ mShootingCounter == 1 分支
            // [TRANSLATION_NOTE]: Threepeater/Splitpea 的头部 reanim 动画判定依赖
            // reanim 完整实现（当前 stub），简化为直接发射，数值节奏一致。
            if self.state == PlantState::CactusLow {
                self.fire(None, self.base.row, PlantWeapon::Secondary);
            } else if self.seed_type == SeedType::Cabbagepult
                || self.seed_type == SeedType::Kernelpult
                || self.seed_type == SeedType::Melonpult
                || self.seed_type == SeedType::Wintermelon
            {
                // 对应 C++ pult 分支：Kernelpult butter 判定切换弹种（渲染组切换依赖 reanim stub）
                let a_plant_weapon = if self.seed_type == SeedType::Kernelpult
                    && self.state == PlantState::KernelpultButter
                {
                    self.state = PlantState::NotReady;
                    PlantWeapon::Secondary
                } else {
                    PlantWeapon::Primary
                };
                let a_target = self.find_target_zombie(self.base.row, a_plant_weapon);
                self.fire(a_target, self.base.row, a_plant_weapon);
            } else {
                self.fire(None, self.base.row, PlantWeapon::Primary);
            }
            return;
        }

        if self.shooting_counter != 0 { return; }

        // 对应 C++ mShootingCounter == 0：头部/身体动画复位为 idle，计数器保活为 1。
        // [TRANSLATION_NOTE]: StartBlend/SetFramesForLayer 依赖 reanim 完整实现（当前 stub），仅保留计数逻辑。
        self.shooting_counter = 1;
    }

    /// 静态辅助函数
    pub fn get_cost(seed_type: SeedType, _imitater_type: SeedType) -> i32 {
        match seed_type {
            SeedType::Peashooter | SeedType::Puffshroom => 100,
            SeedType::Sunflower | SeedType::Sunshroom => 50,
            SeedType::Cherrybomb => 150,
            SeedType::Wallnut => 50,
            SeedType::PotatoMine => 25,
            SeedType::Snowpea => 175,
            SeedType::Chomper => 150,
            SeedType::Repeater => 200,
            SeedType::Fumeshroom => 75,
            SeedType::Gravebuster => 75,
            SeedType::Hypnoshroom => 75,
            SeedType::Scaredyshroom => 25,
            SeedType::Iceshroom => 75,
            SeedType::Doomshroom => 125,
            SeedType::Lilypad => 25,
            SeedType::Squash => 50,
            SeedType::Threepeater => 325,
            SeedType::Tanglekelp => 25,
            SeedType::Jalapeno => 125,
            SeedType::Spikeweed => 100,
            SeedType::Torchwood => 175,
            SeedType::Tallnut => 125,
            SeedType::Cactus => 175,
            SeedType::Blover => 100,
            SeedType::Splitpea => 125,
            SeedType::Starfruit => 125,
            SeedType::Pumpkinshell => 125,
            SeedType::Magnetshroom => 100,
            SeedType::Cabbagepult => 100,
            SeedType::Kernelpult => 100,
            SeedType::InstantCoffee => 75,
            SeedType::Garlic => 50,
            SeedType::Umbrella => 100,
            SeedType::Marigold => 50,
            SeedType::Melonpult => 300,
            _ => 0,
        }
    }

    pub fn is_nocturnal(seed_type: SeedType) -> bool {
        matches!(seed_type, SeedType::Puffshroom | SeedType::Sunshroom |
            SeedType::Fumeshroom | SeedType::Hypnoshroom | SeedType::Scaredyshroom |
            SeedType::Iceshroom | SeedType::Doomshroom | SeedType::Magnetshroom |
            SeedType::Gloomshroom)
    }

    pub fn is_fungus(seed_type: SeedType) -> bool {
        matches!(seed_type, SeedType::Puffshroom | SeedType::Sunshroom |
            SeedType::Fumeshroom | SeedType::Hypnoshroom | SeedType::Scaredyshroom |
            SeedType::Iceshroom | SeedType::Doomshroom | SeedType::Magnetshroom |
            SeedType::Gloomshroom | SeedType::Seashroom)
    }

    pub fn is_aquatic(seed_type: SeedType) -> bool {
        matches!(seed_type, SeedType::Lilypad | SeedType::Tanglekelp |
            SeedType::Cattail | SeedType::Seashroom)
    }

    pub fn is_flying(seed_type: SeedType) -> bool {
        matches!(seed_type, SeedType::Cattail | SeedType::Cactus)
    }

    /// 附加粒子效果（对应 C++ AddAttachedParticle：先销毁旧粒子再添加新的）
    pub fn add_attached_particle(&mut self, pos_x: i32, pos_y: i32, render_position: i32, effect: ParticleEffect) -> Option<*mut ParticleSystem> {
        if let Some(app) = self.base.app {
            unsafe {
                // 对应 C++ ParticleTryToGet(mParticleID) 后的 ParticleSystemDie
                if let Some(es) = (*app).effect_system.as_mut() {
                    if let Some(ps) = es.particle_systems.get_mut(self.particle_id as usize) {
                        ps.particle_system_die();
                    }
                }
                let a_new_particle = (*app).add_tod_particle(pos_x as f32, pos_y as f32, render_position, effect as i32);
                if let Some(p) = a_new_particle {
                    // 对应 C++ ParticleGetID(aNewParticle)：遍历查找指针匹配的 ID
                    if let Some(es) = (*app).effect_system.as_ref() {
                        if let Some(idx) = es.particle_systems.iter().position(|ps| std::ptr::eq(ps, p)) {
                            self.particle_id = idx as ParticleSystemID;
                        }
                    }
                }
                a_new_particle
            }
        } else {
            None
        }
    }

    pub fn is_upgrade(seed_type: SeedType) -> bool {
        matches!(seed_type, SeedType::Gatlingpea | SeedType::Twinsunflower |
            SeedType::Gloomshroom | SeedType::Cattail | SeedType::Wintermelon |
            SeedType::GoldMagnet | SeedType::Spikerock | SeedType::Cobcannon)
    }

    /// 预加载植物资源（对应 C++ Plant::PreloadPlantResources）
    pub fn preload_plant_resources(seed_type: SeedType) {
        let a_plant_def = get_plant_definition(seed_type);
        if a_plant_def.reanimation_type != ReanimationType::None {
            crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(a_plant_def.reanimation_type);
        }

        if seed_type == SeedType::Cherrybomb {
            crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::ZombieCharred);
        } else if seed_type == SeedType::Jalapeno {
            crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::JalapenoFire);
        } else if seed_type == SeedType::Torchwood {
            crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::FirePea);
            crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::JalapenoFire);
        } else if Plant::is_nocturnal(seed_type) {
            crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::Sleeping);
        }
    }

    /// 绘制植物种子图像（对应 C++ Plant::DrawSeedType）
    pub fn draw_seed_type(
        g: &mut Graphics,
        seed_type: SeedType,
        imitater_type: SeedType,
        draw_variation: DrawVariation,
        pos_x: f32,
        pos_y: f32,
    ) {
        // 模仿者变体选择
        let mut a_seed_type = seed_type;
        let mut a_draw_variation = draw_variation;
        if seed_type == SeedType::Imitater && imitater_type != SeedType::None {
            a_seed_type = imitater_type;
            a_draw_variation = DrawVariation::Imitater;
            if matches!(imitater_type, SeedType::Hypnoshroom | SeedType::Squash
                | SeedType::PotatoMine | SeedType::Garlic | SeedType::Lilypad)
            {
                a_draw_variation = DrawVariation::ImitaterLess;
            }
        } else if draw_variation == DrawVariation::Normal && seed_type == SeedType::Tanglekelp {
            a_draw_variation = DrawVariation::Aquarium;
        }

        // 缩放（对应 C++ aSeedG.mScaleX/mScaleY）
        let mut a_scale_x = g.scale_x as f32;
        let mut a_scale_y = g.scale_y as f32;
        let mut a_offset_x = 0.0f32;
        let mut a_offset_y = 0.0f32;

        if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
            if app.game_mode == GameMode::ChallengeBigTime
                && matches!(a_seed_type, SeedType::Wallnut | SeedType::Sunflower | SeedType::Marigold)
            {
                a_scale_x *= 1.5;
                a_scale_y *= 1.5;
                a_offset_x = -20.0;
                a_offset_y = -40.0;
            }
        }
        if a_seed_type == SeedType::Leftpeater {
            a_offset_x += a_scale_x * 80.0;
            a_scale_x *= -1.0;
        }

        if crate::lawn::challenge::Challenge::is_zombie_seed_type(a_seed_type) != 0 {
            let a_zombie_type = crate::lawn::challenge::Challenge::i_zombie_seed_type_to_zombie_type(a_seed_type);
            if a_zombie_type == ZombieType::Dancer {
                a_scale_x *= 0.8;
                a_scale_y *= 0.8;
                a_offset_x = 20.0;
                a_offset_y = 42.0;
            }
            // 对应 C++ gLawnApp->mReanimatorCache->DrawCachedZombie(...)
            if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                unsafe {
                    if let Some(cache) = app.m_reanimator_cache {
                        (*cache).draw_cached_zombie(g, pos_x + a_offset_x, pos_y + a_offset_y, a_zombie_type);
                    }
                }
            }
        } else {
            let a_plant_def = get_plant_definition(a_seed_type);
            // [TRANSLATION_NOTE]: C++ 中 SEED_GIANT_WALLNUT 以 IMAGE_REANIM_WALLNUT_BODY
            // 特殊绘制；Rust 侧图片资源未接入，暂略
            if a_plant_def.reanimation_type != ReanimationType::None {
                // 对应 C++ gLawnApp->mReanimatorCache->DrawCachedPlant(...)
                if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
                    unsafe {
                        if let Some(cache) = app.m_reanimator_cache {
                            (*cache).draw_cached_plant(g, pos_x + a_offset_x, pos_y + a_offset_y, a_seed_type, a_draw_variation);
                        }
                    }
                }
            } else {
                // [TRANSLATION_NOTE]: C++ 中非 reanim 植物按 cel 绘制 Plant::GetImage；
                // Rust 侧 get_image/植物图片未接入，暂略
            }
        }
    }

    /// 是否可升级（对应 C++ IsUpgradableTo）
    pub fn is_upgradable_to(&self, upgraded_type: SeedType) -> bool {
        if upgraded_type == SeedType::Gatlingpea && self.seed_type == SeedType::Repeater {
            return true;
        }
        if upgraded_type == SeedType::Wintermelon && self.seed_type == SeedType::Melonpult {
            return true;
        }
        if upgraded_type == SeedType::Twinsunflower && self.seed_type == SeedType::Sunflower {
            return true;
        }
        if upgraded_type == SeedType::Spikerock && self.seed_type == SeedType::Spikeweed {
            return true;
        }
        if upgraded_type == SeedType::Cobcannon && self.seed_type == SeedType::Kernelpult {
            return self.base.board.map_or(false, |b| unsafe {
                (*b).is_valid_cob_cannon_spot(self.plant_col, self.base.row)
            });
        }
        if upgraded_type == SeedType::GoldMagnet && self.seed_type == SeedType::Magnetshroom {
            return true;
        }
        if upgraded_type == SeedType::Gloomshroom && self.seed_type == SeedType::Fumeshroom {
            return true;
        }
        if upgraded_type == SeedType::Cattail && self.seed_type == SeedType::Lilypad {
            let top_plant = self.base.board.map_or(false, |b| unsafe {
                (*b).get_top_plant_at_any(self.plant_col, self.base.row).map_or(true, |p| p.seed_type != SeedType::Cattail)
            });
            return top_plant;
        }
        false
    }

    /// 是否为可升级的一部分（对应 C++ IsPartOfUpgradableTo）
    pub fn is_part_of_upgradable_to(&self, upgraded_type: SeedType) -> bool {
        if upgraded_type == SeedType::Cobcannon && self.seed_type == SeedType::Kernelpult {
            return self.base.board.map_or(false, |b| unsafe {
                (*b).is_valid_cob_cannon_spot(self.plant_col, self.base.row)
                    || (*b).is_valid_cob_cannon_spot(self.plant_col - 1, self.base.row)
            });
        }
        self.is_upgradable_to(upgraded_type)
    }

    pub fn get_health_for_type(seed_type: SeedType) -> i32 {
        match seed_type {
            SeedType::Wallnut => 4000,
            SeedType::Tallnut => 8000,
            SeedType::Pumpkinshell => 4000,
            SeedType::Garlic => 4000,
            SeedType::ExplodeONut => 4000,
            SeedType::GiantWallnut => 9000,
            _ => 300,
        }
    }

    pub fn get_launch_rate_for_type(seed_type: SeedType) -> i32 {
        match seed_type {
            SeedType::Repeater | SeedType::Gatlingpea | SeedType::Threepeater => 7,
            SeedType::Splitpea => 10,
            SeedType::Starfruit => 10,
            SeedType::Cabbagepult => 15,
            SeedType::Kernelpult => 20,
            SeedType::Melonpult => 20,
            _ => 10,
        }
    }

    pub fn get_frame_length_for_type(_seed_type: SeedType) -> i32 { 10 }
    pub fn get_num_frames_for_type(_seed_type: SeedType) -> i32 { 1 }

    pub fn makes_sun(&self) -> bool {
        matches!(self.seed_type, SeedType::Sunflower | SeedType::Twinsunflower |
            SeedType::Sunshroom | SeedType::Marigold)
    }

    // ========== 辅助函数（stub，对应 C++ Plant 方法） ==========

    /// 是否在游戏中（对应 C++ IsInPlay）
    pub fn is_in_play(&self) -> bool { self.is_on_board }

    /// 设置睡眠状态（对应 C++ SetSleeping）
    pub fn set_sleeping(&mut self, is_asleep: bool) {
        if self.is_asleep == is_asleep || self.not_on_ground() {
            return;
        }

        self.is_asleep = is_asleep;
        if is_asleep {
            let mut a_pos_x = self.base.x as f32 + 50.0;
            let mut a_pos_y = self.base.y as f32 + 40.0;
            if self.seed_type == SeedType::Fumeshroom {
                a_pos_x += 12.0;
            } else if self.seed_type == SeedType::Scaredyshroom {
                a_pos_y -= 20.0;
            } else if self.seed_type == SeedType::Gloomshroom {
                a_pos_y -= 12.0;
            }

            let a_render_order = self.base.render_order + 2;
            if let Some(app) = self.base.get_app_mut() {
                if let Some(a_sleep_reanim) = app.add_reanimation(a_pos_x, a_pos_y, a_render_order, ReanimationType::Sleeping as i32) {
                    // [TRANSLATION_NOTE]: C++ 设置 mLoopType/mAnimRate/mAnimTime — Rust 侧指针访问受限，
                    // 通过 reanimation_get_id 记录后由 reanim 系统更新
                    self.sleeping_reanim_id = app.reanimation_get_id(a_sleep_reanim);
                }
                // [TRANSLATION_NOTE]: 上述 reanim 字段（LOOP、RandRangeFloat(6,8)、RandRangeFloat(0,0.9)）未接入
            }
        } else {
            if let Some(app) = self.base.get_app_mut() {
                app.remove_reanimation(self.sleeping_reanim_id);
                self.sleeping_reanim_id = REANIMATIONID_NULL;
            }
        }

        // 身体动画切换（对应 C++ 后半段）
        let reanim_id = self.body_reanim_id;
        let has_reanim = self.base.get_app_mut().map_or(false, |app| app.reanimation_get_mut(reanim_id).is_some());
        if !has_reanim {
            return;
        }

        if is_asleep {
            if !self.is_in_play() && self.seed_type == SeedType::Sunshroom {
                self.set_body_reanim_frame("anim_bigsleep");
            } else if self.base.get_app().and_then(|app| app.reanimation_get(reanim_id)).map_or(false, |r| r.track_exists("anim_sleep")) {
                let a_anim_time = self.base.get_app().and_then(|app| app.reanimation_get(reanim_id)).map_or(0.0, |r| r.m_anim_time);
                self.set_body_reanim_frame("anim_sleep");
                if let Some(app) = self.base.get_app_mut() {
                    if let Some(reanim) = app.reanimation_get_mut(reanim_id) {
                        reanim.m_anim_time = a_anim_time;
                    }
                }
            } else {
                self.set_body_reanim_rate(1.0);
            }
            self.end_blink();
        } else {
            if !self.is_in_play() && self.seed_type == SeedType::Sunshroom {
                self.set_body_reanim_frame("anim_bigidle");
            } else if self.base.get_app().and_then(|app| app.reanimation_get(reanim_id)).map_or(false, |r| r.track_exists("anim_idle")) {
                let a_anim_time = self.base.get_app().and_then(|app| app.reanimation_get(reanim_id)).map_or(0.0, |r| r.m_anim_time);
                self.set_body_reanim_frame("anim_idle");
                if let Some(app) = self.base.get_app_mut() {
                    if let Some(reanim) = app.reanimation_get_mut(reanim_id) {
                        reanim.m_anim_time = a_anim_time;
                    }
                }
            }
            if self.is_in_play() && self.base.get_app().and_then(|app| app.reanimation_get(reanim_id)).map_or(0.0, |r| r.m_anim_rate) < 2.0 {
                // C++ RandRangeFloat(10.0f, 15.0f) — 用 10 + rand_float(5) 近似
                self.set_body_reanim_rate(10.0 + crate::framework::common::rand_float(5.0));
            }
        }
    }

    /// 附加眨眼动画（对应 C++ AttachBlinkAnim）
    pub fn attach_blink_anim(&mut self, _body_reanim: *mut Reanimation) -> Option<*mut Reanimation> {
        // [TRANSLATION_NOTE]: 完整实现需要 Attachment 系统挂载 blink 动画，当前仅确定轨道名
        let _ = self.seed_type;
        None
    }

    /// 执行眨眼（对应 C++ DoBlink）
    pub fn do_blink(&mut self) {
        self.blink_countdown = 400 + RandRange(400);
        if self.not_on_ground() || self.shooting_counter != 0 {
            return;
        }
        if self.seed_type == SeedType::PotatoMine && self.state != PlantState::PotatoArmed {
            return;
        }
        if matches!(self.state,
            PlantState::CactusRising | PlantState::CactusHigh | PlantState::CactusLowering
            | PlantState::MagnetshroomSucking | PlantState::MagnetshroomCharging
        ) {
            return;
        }
        self.end_blink();
        if matches!(self.seed_type, SeedType::Wallnut | SeedType::Tallnut
            | SeedType::ExplodeONut | SeedType::GiantWallnut)
        {
            self.blink_countdown = 1000 + RandRange(1000);
        }
        if self.attach_blink_anim(self.body_reanim_id as *mut Reanimation).is_none() {
            return;
        }
        // [TRANSLATION_NOTE]: AssignRenderGroupToPrefix("anim_eye", HIDDEN) 依赖 reanim，暂不接入
    }

    /// 结束眨眼（对应 C++ EndBlink）
    pub fn end_blink(&mut self) {
        if self.blink_reanim_id != REANIMATIONID_NULL {
            if let Some(app) = self.base.app {
                unsafe { (*app).remove_reanimation(self.blink_reanim_id); }
            }
            self.blink_reanim_id = REANIMATIONID_NULL;
        }
    }

    /// 更新眨眼（对应 C++ UpdateBlink）
    pub fn update_blink(&mut self) {
        if self.blink_reanim_id != REANIMATIONID_NULL {
            let loop_done = self.base.app.map_or(false, |app| unsafe {
                (*app).reanimation_get(self.blink_reanim_id).map_or(true, |r| r.m_loop_count > 0)
            });
            if loop_done {
                self.end_blink();
            }
        }
        if self.is_asleep {
            return;
        }
        if self.blink_countdown > 0 {
            self.blink_countdown -= 1;
            if self.blink_countdown == 0 {
                self.do_blink();
            }
        }
    }

    /// 计算渲染顺序（对应 C++ CalcRenderOrder）
    pub fn calc_render_order(&self) -> i32 {
        // 对应 C++ Plant::CalcRenderOrder
        let mut order = PlantOrder::Normal;
        let mut layer = crate::lawn::game_enums::RENDER_LAYER_PLANT;
        let mut seed_type = self.seed_type;
        if self.seed_type == SeedType::Imitater && self.imitater_type != SeedType::None {
            seed_type = self.imitater_type;
        }
        if self.base.app.map_or(false, |app| unsafe { (*app).is_wallnut_bowling_level() }) {
            layer = crate::lawn::game_enums::RENDER_LAYER_PROJECTILE;
        } else if seed_type == SeedType::Pumpkinshell {
            order = PlantOrder::Pumpkin;
        } else if Self::is_flying(seed_type) {
            order = PlantOrder::Flyer;
        } else if seed_type == SeedType::Flowerpot
            || (seed_type == SeedType::Lilypad
                && self.base.app.map_or(false, |app| unsafe { (*app).game_mode != GameMode::ChallengeZenGarden }))
        {
            order = PlantOrder::Lilypad;
        }
        crate::lawn::board::make_render_order(layer, self.base.row, order as i32 * 5 - self.base.x + 800)
    }

    /// 设置身体动画帧（对应 C++ 设置 Reanimation 的 FramesForLayer）
    pub fn set_body_reanim_frame(&mut self, layer: &str) {
        let reanim_id = self.body_reanim_id;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(reanim) = app.reanimation_get_mut(reanim_id) {
                reanim.set_frames_for_layer(layer);
            }
        }
    }

    /// 设置身体动画速率（对应 C++ 设置 Reanimation 的 mAnimRate）
    pub fn set_body_reanim_rate(&mut self, rate: f32) {
        let reanim_id = self.body_reanim_id;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(reanim) = app.reanimation_get_mut(reanim_id) {
                reanim.m_anim_rate = rate;
            }
        }
    }

    /// 设置身体动画循环模式（对应 C++ 设置 Reanimation 的 mLoopType）
    pub fn set_body_reanim_loop(&mut self, loop_type: ReanimLoopType) {
        let reanim_id = self.body_reanim_id;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(reanim) = app.reanimation_get_mut(reanim_id) {
                reanim.m_loop_type = loop_type;
            }
        }
    }

    /// 设置帧基础姿势（对应 C++ 设置 Reanimation 的 mFrameBasePose）
    pub fn set_body_reanim_frame_base_pose(&mut self, pose: i32) {
        let reanim_id = self.body_reanim_id;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(reanim) = app.reanimation_get_mut(reanim_id) {
                reanim.m_frame_base_pose = pose;
            }
        }
    }

    /// 添加头部重动画（对应 C++ 添加头部 Reanimation）
    pub fn add_head_reanim(&mut self, layer: &str) {
        let reanim_id = self.head_reanim_id;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(reanim) = app.reanimation_get_mut(reanim_id) {
                reanim.set_frames_for_layer(layer);
            }
        }
    }

    /// 添加头部重动画2（对应 C++ 添加第二个头部 Reanimation）
    pub fn add_head_reanim2(&mut self, layer: &str) {
        let reanim_id = self.head_reanim_id2;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(reanim) = app.reanimation_get_mut(reanim_id) {
                reanim.set_frames_for_layer(layer);
            }
        }
    }

    /// 添加头部重动画3（对应 C++ 添加第三个头部 Reanimation）
    pub fn add_head_reanim3(&mut self, layer: &str) {
        let reanim_id = self.head_reanim_id3;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(reanim) = app.reanimation_get_mut(reanim_id) {
                reanim.set_frames_for_layer(layer);
            }
        }
    }

    /// 播放身体重动画（对应 C++ Plant::PlayBodyReanim）
    pub fn play_body_reanim(&mut self, track_name: &str, loop_type: ReanimLoopType, blend_time: i32, anim_rate: f32) {
        let reanim_id = self.body_reanim_id;
        if let Some(app) = self.base.get_app_mut() {
            if let Some(reanim) = app.reanimation_get_mut(reanim_id) {
                // C++: if (theBlendTime > 0) aBodyReanim->StartBlend(theBlendTime)
                if blend_time > 0 {
                    // [TRANSLATION_NOTE]: StartBlend 未在 Rust Reanimation 中实现
                }
                if anim_rate > 0.0 {
                    reanim.m_anim_rate = anim_rate;
                }
                reanim.m_loop_type = loop_type;
                reanim.m_loop_count = 0;
                reanim.set_frames_for_layer(track_name);
            }
        }
    }

    /// 播放待机动画（对应 C++ Plant::PlayIdleAnim）
    pub fn play_idle_anim(&mut self, rate: f32) {
        let reanim_id = self.body_reanim_id;
        let has_reanim = self.base.get_app_mut().map_or(false, |app| app.reanimation_get_mut(reanim_id).is_some());
        if !has_reanim {
            return;
        }
        self.play_body_reanim("anim_idle", ReanimLoopType::Loop, 20, rate);
        if self.base.get_app().map_or(false, |app| app.is_izombie_level()) {
            if let Some(app) = self.base.get_app_mut() {
                if let Some(reanim) = app.reanimation_get_mut(reanim_id) {
                    reanim.m_anim_rate = 0.0;
                }
            }
        }
    }

    /// 绘制高度偏移（对应 C++ PlantDrawHeightOffset，静态函数；thePlant 可为 nullptr）
    pub fn plant_draw_height_offset(board: Option<&Board>, plant: Option<&Plant>, seed_type: SeedType, grid_x: i32, grid_y: i32) -> f32 {
        let mut a_height_offset = 0.0;

        let mut do_floating = false;
        if Self::is_flying(seed_type) {
            do_floating = false;
        } else if board.is_none() {
            if Self::is_aquatic(seed_type) {
                do_floating = true;
            }
        } else if board.map_or(false, |b| b.is_pool_square(grid_x, grid_y)) {
            do_floating = true;
        } else if plant.is_some() && board.map_or(false, |b| b.m_background_type == BackgroundType::Zombiquarium) {
            do_floating = true;
        }

        if do_floating {
            let a_counter = match board {
                Some(b) => b.m_main_counter,
                None => crate::lawn::lawn_app::LawnApp::instance().map_or(0, |app| app.m_app_counter),
            };
            let a_pos = grid_y as f32 * std::f32::consts::PI + grid_x as f32 * 0.25 * std::f32::consts::PI;
            let a_time = (a_counter % 200) as f32 * (2.0 * std::f32::consts::PI / 200.0);
            let a_floating_height = (a_pos + a_time).sin() * 2.0;
            a_height_offset += a_floating_height;
        }

        if let Some(b) = board {
            if plant.map_or(true, |p| !p.squished) {
                if let Some(a_pot) = b.get_flower_pot_at(grid_x, grid_y) {
                    if !a_pot.squished && seed_type != SeedType::Flowerpot {
                        a_height_offset += Self::plant_flower_pot_height_offset(seed_type, 1.0);
                    }
                }
            }
        }

        if seed_type == SeedType::Flowerpot {
            a_height_offset += 26.0;
        } else if seed_type == SeedType::Lilypad {
            a_height_offset += 25.0;
        } else if seed_type == SeedType::Starfruit {
            a_height_offset += 10.0;
        } else if seed_type == SeedType::Tanglekelp {
            a_height_offset += 24.0;
        } else if seed_type == SeedType::Seashroom {
            a_height_offset += 28.0;
        } else if seed_type == SeedType::InstantCoffee {
            a_height_offset -= 20.0;
        } else if seed_type == SeedType::Cactus {
            return a_height_offset;
        } else if seed_type == SeedType::Pumpkinshell {
            a_height_offset += 15.0;
        } else if seed_type == SeedType::Puffshroom {
            a_height_offset += 5.0;
        } else if seed_type == SeedType::Scaredyshroom {
            a_height_offset -= 14.0;
        } else if seed_type == SeedType::Gravebuster {
            a_height_offset -= 40.0;
        } else if seed_type == SeedType::Spikeweed || seed_type == SeedType::Spikerock {
            let mut a_bottom_row = 4;
            if board.map_or(false, |b| b.stage_has_6_rows()) {
                a_bottom_row = 5;
            }

            if seed_type == SeedType::Spikerock {
                a_height_offset += 6.0;
            }

            let app_game_mode = crate::lawn::lawn_app::LawnApp::instance()
                .map_or(GameMode::Adventure, |app| app.game_mode);
            if board.map_or(false, |b| b.get_flower_pot_at(grid_x, grid_y).is_some())
                && app_game_mode != GameMode::ChallengeZenGarden
            {
                a_height_offset += 5.0;
            } else if board.map_or(false, |b| b.stage_has_roof()) {
                a_height_offset += 15.0;
            } else if board.map_or(false, |b| b.is_pool_square(grid_x, grid_y)) {
                a_height_offset += 0.0;
            } else if grid_y == a_bottom_row && grid_x >= 7 && board.map_or(false, |b| b.stage_has_6_rows()) {
                a_height_offset += 1.0;
            } else if grid_y == a_bottom_row && grid_x < 7 {
                a_height_offset += 12.0;
            } else {
                a_height_offset += 15.0;
            }
        }

        a_height_offset
    }

    /// 花盆高度偏移（对应 C++ PlantFlowerPotHeightOffset）
    pub fn plant_flower_pot_height_offset(seed_type: SeedType, flower_pot_scale: f32) -> f32 {
        let mut a_height_offset = -5.0 * flower_pot_scale;
        let mut a_scale_offset_fix = 0.0;

        match seed_type {
            SeedType::Chomper | SeedType::Plantern => {
                a_height_offset -= 5.0;
            }
            SeedType::Scaredyshroom => {
                a_height_offset += 5.0;
                a_scale_offset_fix -= 8.0;
            }
            SeedType::Sunshroom | SeedType::Puffshroom => {
                a_scale_offset_fix -= 4.0;
            }
            SeedType::Hypnoshroom | SeedType::Magnetshroom | SeedType::Peashooter
            | SeedType::Repeater | SeedType::Leftpeater | SeedType::Snowpea
            | SeedType::Threepeater | SeedType::Sunflower | SeedType::Marigold
            | SeedType::Cabbagepult | SeedType::Melonpult | SeedType::Tanglekelp
            | SeedType::Blover | SeedType::Spikeweed => {
                a_scale_offset_fix -= 8.0;
            }
            SeedType::Seashroom | SeedType::PotatoMine => {
                a_scale_offset_fix -= 4.0;
            }
            SeedType::Lilypad => {
                a_scale_offset_fix -= 16.0;
            }
            SeedType::InstantCoffee => {
                a_scale_offset_fix -= 20.0;
            }
            _ => {}
        }

        a_height_offset + (flower_pot_scale * a_scale_offset_fix - a_scale_offset_fix)
    }

    // ========== 特殊植物更新 stub ==========
    pub fn update_doom_shroom(&mut self) {
        if self.is_asleep || self.state == PlantState::DoingSpecial {
            return;
        }
        self.state = PlantState::DoingSpecial;
        self.do_special_countdown = 100;
        // [TRANSLATION_NOTE]: C++ 设置 anim_explode 动画 + SetShakeOverride + FOLEY_REVERSE_EXPLOSION — reanim 未接入
        self.play_body_reanim("anim_explode", ReanimLoopType::PlayOnceAndHold, 0, 23.0);
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::ReverseExplosion as i32);
        }
    }

    pub fn update_ice_shroom(&mut self) {
        if !self.is_asleep && self.state != PlantState::DoingSpecial {
            self.state = PlantState::DoingSpecial;
            self.do_special_countdown = 100;
            // [TRANSLATION_NOTE]: C++ 设置 anim_explode 动画 + FOLEY_ICE — reanim 未接入
        }
    }
    pub fn update_chomper(&mut self) {
        if self.state == PlantState::Ready {
            if self.find_target_zombie(self.base.row, PlantWeapon::Primary).is_some() {
                self.play_body_reanim("anim_bite", ReanimLoopType::PlayOnceAndHold, 20, 24.0);
                self.state = PlantState::ChomperBiting;
                self.state_countdown = 70;
            }
        } else if self.state == PlantState::ChomperBiting {
            if self.state_countdown == 0 {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::BigChomp as i32);
                }
                let a_zombie_id = self.find_target_zombie(self.base.row, PlantWeapon::Primary);
                // doBite：Gargantuar/RedeyeGargantuar/Boss 只能咬 40 伤害
                let mut do_bite = false;
                if let Some(board) = self.base.get_board() {
                    if let Some(zombie) = a_zombie_id.and_then(|id| board.zombie_try_to_get(id)) {
                        if zombie.zombie_type == ZombieType::Gargantuar
                            || zombie.zombie_type == ZombieType::RedeEyeGargantuar
                            || zombie.zombie_type == ZombieType::Boss
                        {
                            do_bite = true;
                        }
                    }
                }
                // doMiss：无目标，或（未被冻结时）弹跳/撑杆跳
                let mut do_miss = false;
                if a_zombie_id.is_none() {
                    do_miss = true;
                } else if let Some(board) = self.base.get_board() {
                    if let Some(zombie) = a_zombie_id.and_then(|id| board.zombie_try_to_get(id)) {
                        if !zombie.is_immobilized() {
                            if zombie.is_bouncing_pogo()
                                || zombie.zombie_phase == ZombiePhase::PolevaulterInVault
                                || zombie.zombie_phase == ZombiePhase::PolevaulterPreVault
                            {
                                do_miss = true;
                            }
                        }
                    }
                }
                if do_bite {
                    if let Some(app) = self.base.get_app() {
                        app.play_foley(crate::todlib::tod_foley::FoleyType::Splat as i32);
                    }
                    if let Some(board) = self.base.get_board_mut() {
                        if let Some(zombie) = a_zombie_id.and_then(|id| board.zombie_try_to_get_mut(id)) {
                            zombie.take_damage(40, 0);
                        }
                    }
                    self.state = PlantState::ChomperBitingMissed;
                } else if do_miss {
                    self.state = PlantState::ChomperBitingMissed;
                } else {
                    if let Some(board) = self.base.get_board_mut() {
                        if let Some(zombie) = a_zombie_id.and_then(|id| board.zombie_try_to_get_mut(id)) {
                            zombie.die_with_loot();
                        }
                    }
                    self.state = PlantState::ChomperBitingGotOne;
                }
            }
        } else if self.state == PlantState::ChomperBitingGotOne {
            // C++: if (aBodyReanim->mLoopCount > 0)
            let a_loop = self.base.get_app().and_then(|app| app.reanimation_get(self.body_reanim_id)).map_or(0, |r| r.m_loop_count);
            if a_loop > 0 {
                self.play_body_reanim("anim_chew", ReanimLoopType::Loop, 0, 15.0);
                // C++: if (mApp->IsIZombieLevel()) aBodyReanim->mAnimRate = 0;
                if let Some(app) = self.base.get_app_mut() {
                    if app.is_izombie_level() {
                        if let Some(r) = app.reanimation_get_mut(self.body_reanim_id) {
                            r.m_anim_rate = 0.0;
                        }
                    }
                }
                self.state = PlantState::ChomperDigesting;
                self.state_countdown = 4000;
            }
        } else if self.state == PlantState::ChomperDigesting {
            if self.state_countdown == 0 {
                self.play_body_reanim("anim_swallow", ReanimLoopType::PlayOnceAndHold, 20, 12.0);
                self.state = PlantState::ChomperSwallowing;
            }
        } else if self.state == PlantState::ChomperSwallowing || self.state == PlantState::ChomperBitingMissed {
            // C++: if (aBodyReanim->mLoopCount > 0) { PlayIdleAnim(0.0f); mState = STATE_READY; }
            let a_loop = self.base.get_app().and_then(|app| app.reanimation_get(self.body_reanim_id)).map_or(0, |r| r.m_loop_count);
            if a_loop > 0 {
                self.play_idle_anim(0.0);
                self.state = PlantState::Ready;
            }
        }
    }

    pub fn update_torchwood(&mut self) {
        // 对应 C++ UpdateTorchwood
        let attack_rect = self.get_plant_attack_rect(PlantWeapon::Primary);
        let my_row = self.base.row;
        let my_col = self.plant_col;
        if let Some(board) = self.base.get_board_mut() {
            let mut convert: Vec<(usize, bool)> = Vec::new(); // (idx, is_pea)
            for (i, proj) in board.projectiles.iter().enumerate() {
                if proj.dead { continue; }
                if proj.base.row == my_row
                    && (proj.projectile_type == crate::lawn::projectile::ProjectileType::Pea
                        || proj.projectile_type == crate::lawn::projectile::ProjectileType::Snowpea)
                {
                    let proj_rect = proj.get_projectile_rect();
                    if crate::lawn::board::get_rect_overlap(&attack_rect, &proj_rect) >= 10 {
                        convert.push((i, proj.projectile_type == crate::lawn::projectile::ProjectileType::Pea));
                    }
                }
            }
            for (idx, is_pea) in convert {
                let proj = &mut board.projectiles[idx];
                if is_pea {
                    proj.convert_to_fireball(my_col);
                } else {
                    proj.convert_to_pea(my_col);
                }
            }
        }
    }
    /// 发射五角星（对应 C++ Plant::StarFruitFire，Plant.cpp:920）
    pub fn star_fruit_fire(&mut self) {
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::Throw as i32);
        }

        let a_shoot_angle_x = (30.0f32 * std::f32::consts::PI / 180.0).cos() * 3.33;
        let a_shoot_angle_y = (30.0f32 * std::f32::consts::PI / 180.0).sin() * 3.33;
        let a_damage_range_flags = self.get_damage_range_flags(PlantWeapon::Primary);
        let row = self.base.row;
        for i in 0..5 {
            let a_projectile_idx = {
                let board = match self.base.get_board_mut() {
                    Some(b) => b,
                    None => return,
                };
                board.add_projectile(self.pos_x + 25.0, self.pos_y + 25.0, row, SeedType::Starfruit)
            };
            if let Some(board) = self.base.get_board_mut() {
                if let Some(a_projectile) = board.projectiles.get_mut(a_projectile_idx) {
                    a_projectile.damage_range_flags = a_damage_range_flags;
                    a_projectile.motion = crate::lawn::projectile::ProjectileMotion::Star;

                    match i {
                        0 => { a_projectile.vel_x = -3.33; a_projectile.vel_y = 0.0; }
                        1 => { a_projectile.vel_x = 0.0; a_projectile.vel_y = 3.33; }
                        2 => { a_projectile.vel_x = 0.0; a_projectile.vel_y = -3.33; }
                        3 => { a_projectile.vel_x = a_shoot_angle_x; a_projectile.vel_y = a_shoot_angle_y; }
                        4 => { a_projectile.vel_x = a_shoot_angle_x; a_projectile.vel_y = -a_shoot_angle_y; }
                        _ => { /* C++: PVZP_ASSERT(false) */ }
                    }
                }
            }
        }
    }

    pub fn update_blover(&mut self) {
        // 对应 C++ UpdateBlover（Plant.cpp:1612）：循环动画帧设置
        if let Some(app) = self.base.get_app_mut() {
            if let Some(a_body_reanim) = app.reanimation_get_mut(self.body_reanim_id) {
                if a_body_reanim.m_loop_count > 0
                    && a_body_reanim.m_loop_type != crate::todlib::reanimator::ReanimLoopType::Loop
                {
                    a_body_reanim.set_frames_for_layer("anim_loop");
                    a_body_reanim.m_loop_type = crate::todlib::reanimator::ReanimLoopType::Loop;
                }
            }
        }
    }

    pub fn update_flower_pot(&mut self) {
        if self.state == PlantState::FlowerpotInvulnerable && self.state_countdown == 0 {
            self.state = PlantState::NotReady;
        }
    }

    pub fn update_lilypad(&mut self) {
        if self.state == PlantState::LilypadInvulnerable && self.state_countdown == 0 {
            self.state = PlantState::NotReady;
        }
    }

    pub fn update_cob_cannon(&mut self) {
        // C++ Plant::UpdateCobCannon（Plant.cpp:1668-1704）
        if self.state == PlantState::CobcannonArming {
            if self.state_countdown == 0 {
                self.state = PlantState::CobcannonLoading;
                // C++: PlayBodyReanim("anim_charge", REANIM_PLAY_ONCE_AND_HOLD, 20, 12.0f)
                self.play_body_reanim("anim_charge", ReanimLoopType::PlayOnceAndHold, 20, 12.0);
            }
        } else if self.state == PlantState::CobcannonLoading {
            // C++: if (aBodyReanim->ShouldTriggerTimedEvent(0.5f)) PlayFoley(FOLEY_SHOOP)
            if let Some(app) = self.base.get_app() {
                let a_trigger = app.reanimation_get(self.body_reanim_id).map_or(false, |r| r.should_trigger_timed_event(0.5));
                if a_trigger {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Shoop as i32);
                }
            }
            // C++: if (aBodyReanim->mLoopCount > 0) { mState = READY; PlayIdleAnim(12.0f); }
            let a_loop = self.base.get_app().and_then(|app| app.reanimation_get(self.body_reanim_id)).map_or(0, |r| r.m_loop_count);
            if a_loop > 0 {
                self.state = PlantState::CobcannonReady;
                self.play_idle_anim(12.0);
            }
        } else if self.state == PlantState::CobcannonReady {
            // C++: aCobTrack = GetTrackInstanceByName("CobCannon_cob");
            //      aCobTrack->mTrackColor = GetFlashingColor(mBoard->mMainCounter, 75);
            let a_main_counter = self.base.get_board().map_or(0, |b| unsafe { (*b).m_main_counter });
            if let Some(app) = self.base.get_app_mut() {
                if let Some(r) = app.reanimation_get_mut(self.body_reanim_id) {
                    if let Some(ti) = r.get_track_instance_by_name("CobCannon_cob") {
                        ti.m_track_color = crate::todlib::tod_common::get_flashing_color(a_main_counter, 75);
                    }
                }
            }
        } else if self.state == PlantState::CobcannonFiring {
            // C++: if (aBodyReanim->ShouldTriggerTimedEvent(0.48f)) PlayFoley(FOLEY_COB_LAUNCH)
            if let Some(app) = self.base.get_app() {
                let a_trigger = app.reanimation_get(self.body_reanim_id).map_or(false, |r| r.should_trigger_timed_event(0.48));
                if a_trigger {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::CobLaunch as i32);
                }
            }
        }
    }
    pub fn update_imitater(&mut self) {
        // 对应 C++ UpdateImitater
        if self.state != PlantState::ImitaterMorphing {
            if self.state_countdown == 0 {
                self.state = PlantState::ImitaterMorphing;
                self.play_body_reanim("anim_explode", ReanimLoopType::PlayOnceAndHold, 0, 26.0);
            }
        } else {
            // [TRANSLATION_NOTE]: ShouldTriggerTimedEvent(0.8) + PARTICLE_IMITATER_MORPH；mLoopCount → ImitaterMorph — reanim 未接入
            self.imitater_morph();
        }
    }

    /// 模仿者变身（对应 C++ ImitaterMorph）
    pub fn imitater_morph(&mut self) {
        let a_col = self.plant_col;
        let a_row = self.base.row;
        let a_imitater_type = self.imitater_type;
        self.die();
        if let Some(board) = self.base.get_board_mut() {
            board.add_plant(a_col, a_row, a_imitater_type, SeedType::Imitater);
            // [TRANSLATION_NOTE]: C++ AddPlant 后设置 reanim 的 FilterEffect(WASHED_OUT / LESS_WASHED_OUT) — reanim 未接入
        }
    }

    /// 移除特效（对应 C++ Plant::RemoveEffects）
    pub fn remove_effects(&mut self) {
        if let Some(app) = self.base.app {
            unsafe {
                (*app).remove_particle(self.particle_id);
                (*app).remove_reanimation(self.body_reanim_id);
                (*app).remove_reanimation(self.head_reanim_id);
                (*app).remove_reanimation(self.head_reanim_id2);
                (*app).remove_reanimation(self.head_reanim_id3);
                (*app).remove_reanimation(self.light_reanim_id);
                (*app).remove_reanimation(self.blink_reanim_id);
                (*app).remove_reanimation(self.sleeping_reanim_id);
            }
        }
    }

    /// 压扁植物（对应 C++ Plant::Squish）
    pub fn squish(&mut self) {
        if self.not_on_ground() {
            return;
        }        if !self.is_asleep {
            if matches!(self.seed_type,
                SeedType::Cherrybomb | SeedType::Jalapeno | SeedType::Doomshroom | SeedType::Iceshroom)
            {
                self.do_special();
                return;
            } else if self.seed_type == SeedType::PotatoMine && self.state != PlantState::NotReady {
                self.do_special();
                return;
            }
        }
        if self.seed_type == SeedType::Squash && self.state != PlantState::NotReady {
            return;
        }
        self.squished = true;
        self.disappear_countdown = 500;
        // [TRANSLATION_NOTE]: PlayFoley(FOLEY_SQUISH) + RemoveEffects + 梯子 GridItemDie + 后续 Zombatar 处理 — 声音/particle 未接入
        self.dead = true;
    }

    /// 冻结全场僵尸（对应 C++ Plant::IceZombies）
    pub fn ice_zombies(&mut self) {
        if let Some(board) = self.base.get_board_mut() {
            for idx in 0..board.zombies.len() {
                if board.zombies[idx].dead {
                    continue;
                }
                board.zombies[idx].hit_ice_trap();
            }
            board.m_ice_trap_counter = 300;
            // [TRANSLATION_NOTE]: mPoolSparklyParticleID + ParticleTryToGet 启停粒子 — particle 未接入
            if let Some(boss) = board.get_boss_zombie_mut() {
                boss.boss_destroy_fireball();
            }
        }
    }

    /// 灼烧一行（对应 C++ Plant::BurnRow）
    pub fn burn_row(&mut self, row: i32) {
        let a_damage_range_flags = self.get_damage_range_flags(PlantWeapon::Primary);
        if let Some(board) = self.base.get_board_mut() {
            for idx in 0..board.zombies.len() {
                if board.zombies[idx].dead {
                    continue;
                }
                if (board.zombies[idx].zombie_type == ZombieType::Boss || board.zombies[idx].base.row == row)
                    && board.zombies[idx].effected_by_damage(a_damage_range_flags)
                {
                    board.zombies[idx].remove_cold_effects();
                    board.zombies[idx].apply_burn();
                }
            }
            for idx in 0..board.grid_items.len() {
                if board.grid_items[idx].dead {
                    continue;
                }
                if board.grid_items[idx].grid_y == row
                    && board.grid_items[idx].grid_item_type == crate::lawn::grid_item::GridItemType::Ladder
                {
                    board.grid_items[idx].grid_item_die();
                }
            }
            if let Some(boss) = board.get_boss_zombie_mut() {
                if boss.fireball_row == row {
                    boss.boss_destroy_iceball_in_row();
                }
            }
        }
    }

    pub fn update_coffee_bean(&mut self) {
        if self.state == PlantState::DoingSpecial {
            // 依赖底层系统
            // self.die();
        }
    }

    pub fn update_umbrella(&mut self) {
        if self.state == PlantState::UmbrellaTriggered {
            if self.state_countdown == 0 {
                self.state = PlantState::UmbrellaReflecting;
            }
        } else if self.state == PlantState::UmbrellaReflecting {
            // 依赖底层系统
            self.state = PlantState::NotReady;
        }
    }

    pub fn update_cactus(&mut self) {
        // C++ Plant::UpdateCactus（Plant.cpp:1707-1748）
        if self.shooting_counter > 0 {
            return;
        }
        // C++: aBodyReanim = mApp->ReanimationGet(mBodyReanimID)
        let body_reanim_ptr = self.base.get_app_mut().and_then(|app| {
            app.reanimation_get_mut(self.body_reanim_id).map(|r| r as *mut crate::todlib::reanimator::Reanimation)
        });
        let a_body_fps = unsafe { body_reanim_ptr.map_or(12.0, |p| (*p).m_fps) };

        if self.state == PlantState::CactusRising {
            // C++: if (aBodyReanim->mLoopCount > 0)
            let a_loop_count = unsafe { body_reanim_ptr.map_or(0, |p| (*p).m_loop_count) };
            if a_loop_count > 0 {
                self.state = PlantState::CactusHigh;
                // C++: PlayBodyReanim("anim_idlehigh", REANIM_LOOP, 20, 0.0f)
                self.play_body_reanim("anim_idlehigh", ReanimLoopType::Loop, 20, 0.0);
                // C++: if (mApp->IsIZombieLevel()) aBodyReanim->mAnimRate = 0;
                if let Some(app) = self.base.get_app() {
                    if app.is_izombie_level() {
                        if let Some(p) = body_reanim_ptr {
                            unsafe { (*p).m_anim_rate = 0.0; }
                        }
                    }
                }
                self.launch_counter = 1;
            }
        } else if self.state == PlantState::CactusHigh {
            // C++: if (FindTargetZombie(mRow, WEAPON_PRIMARY) == nullptr)
            if self.find_target_zombie(self.base.row, PlantWeapon::Primary).is_none() {
                self.state = PlantState::CactusLowering;
                // C++: PlayBodyReanim("anim_lower", REANIM_PLAY_ONCE_AND_HOLD, 20, aBodyReanim->mDefinition->mFPS)
                self.play_body_reanim("anim_lower", ReanimLoopType::PlayOnceAndHold, 20, a_body_fps);
            }
        } else if self.state == PlantState::CactusLowering {
            // C++: if (aBodyReanim->mLoopCount > 0)
            let a_loop_count = unsafe { body_reanim_ptr.map_or(0, |p| (*p).m_loop_count) };
            if a_loop_count > 0 {
                self.state = PlantState::CactusLow;
                // C++: PlayIdleAnim(0.0f)
                self.play_idle_anim(0.0);
            }
        } else if self.find_target_zombie(self.base.row, PlantWeapon::Primary).is_some() {
            self.state = PlantState::CactusRising;
            // C++: PlayBodyReanim("anim_rise", REANIM_PLAY_ONCE_AND_HOLD, 20, aBodyReanim->mDefinition->mFPS)
            self.play_body_reanim("anim_rise", ReanimLoopType::PlayOnceAndHold, 20, a_body_fps);
            // C++: mApp->PlayFoley(FOLEY_PLANTGROW)
            if let Some(app) = self.base.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::PlantGrow as i32);
            }
        }
    }

    /// 获取空闲磁力物品槽（对应 C++ GetFreeMagnetItem），返回槽位下标
    fn get_free_magnet_item_idx(&self) -> Option<usize> {
        if self.seed_type == SeedType::GoldMagnet {
            for (i, item) in self.magnet_items.iter().enumerate() {
                if item.item_type == MagnetItemType::None {
                    return Some(i);
                }
            }
            None
        } else {
            Some(0)
        }
    }

    /// 磁力菇吸走僵尸装备（对应 C++ MagnetShroomAttactItem）
    fn magnet_shroom_attack_item(&mut self, zombie_idx: usize) {
        // [TRANSLATION_NOTE]: C++ 用 GetTrackPosition 获取部件动画位置并减去图片尺寸；
        // Rust 简化用僵尸坐标，图片尺寸/轨道信息依赖 reanim 系统未接入
        let z_info = self.base.get_board().and_then(|board| {
            board.zombies.get(zombie_idx).map(|z| {
                (
                    z.helm_type,
                    z.get_helm_damage_index(),
                    z.shield_type,
                    z.get_shield_damage_index(),
                    z.zombie_type,
                    z.zombie_phase,
                    z.has_arm,
                    z.is_eating,
                    z.pos_x,
                    z.pos_y,
                )
            })
        });
        let Some((helm_type, helm_damage_idx, shield_type, shield_damage_idx, zombie_type, zombie_phase, has_arm, zombie_eating, z_pos_x, z_pos_y)) = z_info else { return };

        self.state = PlantState::MagnetshroomSucking;
        self.state_countdown = 1500;
        self.play_body_reanim("anim_shooting", ReanimLoopType::PlayOnceAndHold, 20, 12.0);
        if let Some(app) = self.base.get_app() {
            app.play_foley(crate::todlib::tod_foley::FoleyType::Magnetshroom as i32);
        }

        let a_magnet_idx = match self.get_free_magnet_item_idx() {
            Some(i) => i,
            None => return,
        };
        let a_magnet_item = &mut self.magnet_items[a_magnet_idx];
        a_magnet_item.pos_x = z_pos_x;
        a_magnet_item.pos_y = z_pos_y;
        a_magnet_item.dest_offset_x = -10.0 + RandFloat(20.0) + 25.0;  // RandRangeFloat(-10,10) + 25
        a_magnet_item.dest_offset_y = -10.0 + RandFloat(20.0) + 20.0;  // RandRangeFloat(-10,10) + 20

        if helm_type == HelmType::Pail {
            // MAGNET_ITEM_PAIL_1 + mDamageIndex
            a_magnet_item.item_type = match helm_damage_idx {
                0 => MagnetItemType::Pail1,
                1 => MagnetItemType::Pail2,
                _ => MagnetItemType::Pail3,
            };
        } else if helm_type == HelmType::FootballHelmet {
            a_magnet_item.item_type = match helm_damage_idx {
                0 => MagnetItemType::FootballHelmet1,
                1 => MagnetItemType::FootballHelmet2,
                _ => MagnetItemType::FootballHelmet3,
            };
        } else if shield_type == ShieldType::Door {
            a_magnet_item.item_type = match shield_damage_idx {
                0 => MagnetItemType::Door1,
                1 => MagnetItemType::Door2,
                _ => MagnetItemType::Door3,
            };
        } else if shield_type == ShieldType::Ladder {
            a_magnet_item.item_type = match shield_damage_idx {
                0 => MagnetItemType::Ladder1,
                1 => MagnetItemType::Ladder2,
                _ => MagnetItemType::Ladder3,
            };
        } else if zombie_type == ZombieType::Pogo {
            a_magnet_item.item_type = if has_arm { MagnetItemType::Pogo1 } else { MagnetItemType::Pogo3 };
        } else if zombie_phase == ZombiePhase::JackInTheBoxRunning {
            a_magnet_item.item_type = MagnetItemType::JackInTheBox;
        } else if zombie_type == ZombieType::Digger {
            a_magnet_item.item_type = MagnetItemType::PickAxe;
        }

        // 第二步：修改僵尸（头盔/盾牌/相位）
        if let Some(board) = self.base.get_board_mut() {
            let z = &mut board.zombies[zombie_idx];
            if helm_type == HelmType::Pail || helm_type == HelmType::FootballHelmet {
                z.helm_health = 0;
                z.helm_type = HelmType::None;
                // [TRANSLATION_NOTE]: ReanimShowPrefix(anim_bucket/anim_hair) — reanim 未接入
            } else if shield_type == ShieldType::Door {
                z.detach_shield();
                z.zombie_phase = ZombiePhase::Normal;
                if !zombie_eating {
                    z.start_walk_anim(0);
                }
            } else if shield_type == ShieldType::Ladder {
                z.detach_shield();
            } else if zombie_type == ZombieType::Pogo {
                z.pogo_break(16);
            } else if zombie_phase == ZombiePhase::JackInTheBoxRunning {
                z.stop_zombie_sound();
                z.pick_random_speed();
                z.zombie_phase = ZombiePhase::Normal;
                // [TRANSLATION_NOTE]: ReanimShowPrefix(Zombie_jackbox_box/handle) — reanim 未接入
            } else if zombie_type == ZombieType::Digger {
                z.digger_lose_axe();
                // [TRANSLATION_NOTE]: GetTrackPosition(Zombie_digger_pickaxe) — reanim 未接入
            }
        }
    }

    /// 更新磁力物品吸附移动（对应 C++ UpdateMagnetShroom / UpdateGoldMagnetShroom 的吸附部分）
    fn update_magnet_items(&mut self) {
        let plant_x = self.base.x as f32;
        let plant_y = self.base.y as f32;
        for i in 0..MAX_MAGNET_ITEMS {
            let item = &mut self.magnet_items[i];
            if item.item_type != MagnetItemType::None {
                let vec_x = plant_x + item.dest_offset_x - item.pos_x;
                let vec_y = plant_y + item.dest_offset_y - item.pos_y;
                let magnitude = (vec_x * vec_x + vec_y * vec_y).sqrt();
                if magnitude > 20.0 {
                    item.pos_x += vec_x * 0.05;
                    item.pos_y += vec_y * 0.05;
                }
            }
        }
    }

    pub fn update_magnet_shroom(&mut self) {
        self.update_magnet_items();

        if self.state == PlantState::MagnetshroomCharging {
            if self.state_countdown == 0 {
                self.state = PlantState::Ready;
                // C++: aAnimRate = RandRangeFloat(10.0f, 15.0f); PlayBodyReanim("anim_idle", REANIM_LOOP, 30, aAnimRate)
                let a_anim_rate = crate::todlib::tod_common::rand_range_float(10.0, 15.0);
                self.play_body_reanim("anim_idle", ReanimLoopType::Loop, 30, a_anim_rate);
                // C++: if (mApp->IsIZombieLevel()) aBodyReanim->mAnimRate = 0.0f
                if let Some(app) = self.base.get_app_mut() {
                    if app.is_izombie_level() {
                        if let Some(r) = app.reanimation_get_mut(self.body_reanim_id) {
                            r.m_anim_rate = 0.0;
                        }
                    }
                }
                self.magnet_items[0].item_type = MagnetItemType::None;
            }
        } else if self.state == PlantState::MagnetshroomSucking {
            // C++: if (aBodyReanim->mLoopCount > 0)
            let a_loop = self.base.get_app().and_then(|app| app.reanimation_get(self.body_reanim_id)).map_or(0, |r| r.m_loop_count);
            if a_loop > 0 {
                // C++: PlayBodyReanim("anim_nonactive_idle2", REANIM_LOOP, 20, 2.0f)
                self.play_body_reanim("anim_nonactive_idle2", ReanimLoopType::Loop, 20, 2.0);
                if let Some(app) = self.base.get_app_mut() {
                    if app.is_izombie_level() {
                        if let Some(r) = app.reanimation_get_mut(self.body_reanim_id) {
                            r.m_anim_rate = 0.0;
                        }
                    }
                }
                self.state = PlantState::MagnetshroomCharging;
            }
        } else {
            // 找最近的、可被吸走装备的僵尸（对应 C++ GetCircleRectOverlap + 距离加权）
            let z_target = self.base.get_board().and_then(|board| {
                let mut closest: Option<(usize, f32)> = None;
                for (idx, z) in board.zombies.iter().enumerate() {
                    if z.dead { continue; }
                    let a_diff_y = z.base.row - self.base.row;
                    if z.mind_controlled || !z.has_head {
                        continue;
                    }
                    if z.zombie_height != ZombieHeight::Normal || z.zombie_phase == ZombiePhase::RisingFromGrave {
                        continue;
                    }
                    if z.is_dead_or_dying() {
                        continue;
                    }
                    let z_rect = z.get_zombie_rect();
                    if z_rect.x > BOARD_WIDTH || a_diff_y > 2 || a_diff_y < -2 {
                        continue;
                    }
                    if z.zombie_phase == ZombiePhase::DiggerTunneling
                        || z.zombie_phase == ZombiePhase::DiggerStunned
                        || z.zombie_phase == ZombiePhase::DiggerWalking
                        || z.zombie_type == ZombieType::Pogo
                    {
                        if !z.has_object {
                            continue;
                        }
                    } else if !(z.helm_type == HelmType::Pail
                        || z.helm_type == HelmType::FootballHelmet
                        || z.shield_type == ShieldType::Door
                        || z.shield_type == ShieldType::Ladder
                        || z.zombie_phase == ZombiePhase::JackInTheBoxRunning)
                    {
                        continue;
                    }

                    let a_radius = if z.is_eating { 320 } else { 270 };
                    if crate::lawn::board::get_circle_rect_overlap(self.base.x, self.base.y + 20, a_radius, &z_rect) {
                        let dx = (self.base.x as f32 - z_rect.x as f32);
                        let dy = (self.base.y as f32 - z_rect.y as f32);
                        let mut a_distance = (dx * dx + dy * dy).sqrt();
                        a_distance += (a_diff_y.abs() * 80) as f32;
                        if closest.map_or(true, |(_, d)| a_distance < d) {
                            closest = Some((idx, a_distance));
                        }
                    }
                }
                closest
            });
            if let Some((zombie_idx, _)) = z_target {
                self.magnet_shroom_attack_item(zombie_idx);
                return;
            }

            // 找最近的梯子（对应 C++ 梯子吸附分支）
            let ladder_target = self.base.get_board().and_then(|board| {
                let mut closest: Option<(usize, f32)> = None;
                for (idx, gi) in board.grid_items.iter().enumerate() {
                    if gi.dead { continue; }
                    if gi.grid_item_type == crate::lawn::grid_item::GridItemType::Ladder {
                        let a_diff_x = (gi.grid_x - self.plant_col).abs();
                        let a_diff_y = (gi.grid_y - self.base.row).abs();
                        let a_square_distance = a_diff_x.max(a_diff_y);
                        if a_square_distance <= 2 {
                            let a_distance = a_square_distance as f32 + a_diff_y as f32 * 0.05;
                            if closest.map_or(true, |(_, d)| a_distance < d) {
                                closest = Some((idx, a_distance));
                            }
                        }
                    }
                }
                closest
            });
            if let Some((grid_idx, _)) = ladder_target {
                self.state = PlantState::MagnetshroomSucking;
                self.state_countdown = 1500;
                self.play_body_reanim("anim_shooting", ReanimLoopType::PlayOnceAndHold, 20, 12.0);
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Magnetshroom as i32);
                }
                if let Some(board) = self.base.get_board_mut() {
                    board.grid_items[grid_idx].grid_item_die();
                    // [TRANSLATION_NOTE]: mPosX/Y 用 GridToPixelX/Y + 40 — 简化为植物坐标
                    let a_magnet_idx = match self.get_free_magnet_item_idx() {
                        Some(i) => i,
                        None => return,
                    };
                    let item = &mut self.magnet_items[a_magnet_idx];
                    item.pos_x = self.base.x as f32 + 40.0;
                    item.pos_y = self.base.y as f32;
                    item.dest_offset_x = -10.0 + RandFloat(20.0) + 10.0;
                    item.dest_offset_y = -10.0 + RandFloat(20.0);
                    item.item_type = MagnetItemType::LadderPlaced;
                }
            }
        }
    }

    /// 寻找最近的金钱（对应 C++ FindGoldMagnetTarget），返回 coins 下标
    fn find_gold_magnet_target(&self) -> Option<usize> {
        let plant_x = self.base.x + self.base.width / 2;
        let plant_y = self.base.y + self.base.height / 2;
        self.base.get_board().and_then(|board| {
            let mut closest: Option<(usize, f32)> = None;
            for (idx, coin) in board.coins.iter().enumerate() {
                if coin.dead { continue; }
                // [TRANSLATION_NOTE]: C++ 检查 mCoinMotion != COIN_MOTION_FROM_PRESENT；Rust CoinMotion 无 FromPresent 变体
                if coin.is_money() && !coin.is_being_collected && coin.coin_age >= 50 {
                    let dx = plant_x as f32 - (coin.pos_x + 15.0);
                    let dy = plant_y as f32 - (coin.pos_y + 15.0);
                    let a_distance = (dx * dx + dy * dy).sqrt();
                    if closest.map_or(true, |(_, d)| a_distance < d) {
                        closest = Some((idx, a_distance));
                    }
                }
            }
            closest.map(|(i, _)| i)
        })
    }

    /// 黄金磁力菇吸金币（对应 C++ GoldMagnetFindTargets）
    fn gold_magnet_find_targets(&mut self) {
        if self.get_free_magnet_item_idx().is_none() {
            // C++: PVZP_ASSERT(false)
            return;
        }
        loop {
            let free_idx = match self.get_free_magnet_item_idx() {
                Some(i) => i,
                None => break,
            };
            let coin_idx = match self.find_gold_magnet_target() {
                Some(i) => i,
                None => break,
            };
            let coin_info = match self.base.get_board().and_then(|b| b.coins.get(coin_idx)).map(|c| (c.coin_type, c.pos_x, c.pos_y)) {
                Some(v) => v,
                None => break,
            };
            let (coin_type, coin_x, coin_y) = coin_info;
            let item = &mut self.magnet_items[free_idx];
            item.pos_x = coin_x + 15.0;
            item.pos_y = coin_y + 15.0;
            item.dest_offset_x = 20.0 + RandFloat(20.0);   // RandRangeFloat(20,40)
            item.dest_offset_y = -20.0 + RandFloat(20.0) + 20.0;  // RandRangeFloat(-20,0)+20
            item.item_type = match coin_type {
                CoinType::Silver => MagnetItemType::SilverCoin,
                CoinType::Gold => MagnetItemType::GoldCoin,
                CoinType::Diamond => MagnetItemType::Diamond,
                _ => {
                    // C++: PVZP_ASSERT(false); return;
                    return;
                }
            };
            if let Some(board) = self.base.get_board_mut() {
                board.coins[coin_idx].die();
            }
        }
    }

    /// 是否已有黄金磁力菇正在吸（对应 C++ IsAGoldMagnetAboutToSuck）
    fn is_a_gold_magnet_about_to_suck(&self) -> bool {
        if let Some(board) = self.base.get_board() {
            for plant in &board.plants {
                if plant.dead { continue; }
                if !plant.not_on_ground() && plant.seed_type == SeedType::GoldMagnet
                    && plant.state == PlantState::MagnetshroomSucking
                {
                    // [TRANSLATION_NOTE]: C++ 检查 aBodyReanim->mAnimTime < 0.5f — reanim 未接入
                    return true;
                }
            }
        }
        false
    }

    pub fn update_gold_magnet_shroom(&mut self) {
        // 先更新吸附移动与吸到口袋的判定（对应 C++ UpdateGoldMagnetShroom 前段）
        let plant_x = self.base.x as f32;
        let plant_y = self.base.y as f32;
        let mut a_is_sucking_coin = false;
        for i in 0..MAX_MAGNET_ITEMS {
            let item = &mut self.magnet_items[i];
            if item.item_type != MagnetItemType::None {
                let vec_x = plant_x + item.dest_offset_x - item.pos_x;
                let vec_y = plant_y + item.dest_offset_y - item.pos_y;
                let a_distance = (vec_x * vec_x + vec_y * vec_y).sqrt();
                if a_distance < 20.0 {
                    let a_coin_type = match item.item_type {
                        MagnetItemType::SilverCoin => CoinType::Silver,
                        MagnetItemType::GoldCoin => CoinType::Gold,
                        MagnetItemType::Diamond => CoinType::Diamond,
                        _ => {
                            // C++: PVZP_ASSERT(false); return;
                            return;
                        }
                    };
                    let a_value = crate::lawn::coin::Coin::get_coin_value(a_coin_type);
                    if let Some(app) = self.base.get_app_mut() {
                        if let Some(pi) = app.player_info.as_mut() {
                            pi.add_coins(a_value);
                        }
                    }
                    if let Some(board) = self.base.get_board_mut() {
                        board.m_coins_collected += a_value;
                    }
                    // [TRANSLATION_NOTE]: PlayFoley(FOLEY_COIN) — 声音未接入
                    item.item_type = MagnetItemType::None;
                } else {
                    // C++: aSpeed = PvzpAnimateCurveFloatTime(30, 0, aDistance, 0.02, 0.05, CURVE_LINEAR)
                    // 简化：保持与更新磁力菇一致的 0.05 系数
                    let a_speed = 0.05;
                    item.pos_x += vec_x * a_speed;
                    item.pos_y += vec_y * a_speed;
                    a_is_sucking_coin = true;
                }
            }
        }

        if self.state == PlantState::MagnetshroomCharging {
            if self.state_countdown == 0 {
                self.state = PlantState::Ready;
            }
        } else if self.state == PlantState::MagnetshroomSucking {
            // [TRANSLATION_NOTE]: ShouldTriggerTimedEvent(0.4) → GoldMagnetFindTargets + FOLEY_MAGNETSHROOM；mLoopCount → 回充
            self.gold_magnet_find_targets();
            if !a_is_sucking_coin {
                self.play_idle_anim(14.0);
                self.state = PlantState::MagnetshroomCharging;
                self.state_countdown = 200 + RandRange(101);  // RandRangeInt(200, 300)
            }
        } else if !self.is_a_gold_magnet_about_to_suck() && RandRange(50) == 0 && self.find_gold_magnet_target().is_some() {
            if let Some(board) = self.base.get_board_mut() {
                board.show_coin_bank(0);  // [TRANSLATION_NOTE]: 时长参数简化
            }
            self.state = PlantState::MagnetshroomSucking;
            self.play_body_reanim("anim_attract", ReanimLoopType::PlayOnceAndHold, 20, 12.0);
        }
    }

    pub fn update_grave_buster(&mut self) {
        if self.state == PlantState::GravebusterLanding {
            // 依赖底层系统
            self.state = PlantState::GravebusterEating;
            self.state_countdown = 400;
        } else if self.state == PlantState::GravebusterEating && self.state_countdown == 0 {
            // 依赖底层系统
        }
    }
    pub fn update_potato(&mut self) {
        if self.state == PlantState::NotReady {
            if self.state_countdown == 0 {
                // [TRANSLATION_NOTE]: PARTICLE_POTATO_MINE_RISE + FOLEY_DIRT_RISE 未接入
                self.play_body_reanim("anim_rise", ReanimLoopType::PlayOnceAndHold, 20, 18.0);
                self.state = PlantState::PotatoRising;
            }
        } else if self.state == PlantState::PotatoRising {
            // [TRANSLATION_NOTE]: C++ 需 aBodyReanim->mLoopCount>0 + anim_glow light reanim — reanim 未接入
            self.play_body_reanim("anim_armed", ReanimLoopType::Loop, 0, 12.0);
            self.state = PlantState::PotatoArmed;
            self.blink_countdown = 400 + RandRange(4000);
        } else if self.state == PlantState::PotatoArmed {
            if self.find_target_zombie(self.base.row, PlantWeapon::Primary).is_some() {
                self.do_special();
            } else {
                // [TRANSLATION_NOTE]: light reanim mFrameCount = PvzpAnimateCurve(200, 50, DistanceToClosestZombie(), 10, 3) — reanim 未接入
                let _dist = self.distance_to_closest_zombie();
            }
        }
    }

    pub fn update_squash(&mut self) {
        // C++ Plant::UpdateSquash（Plant.cpp:1482-1613）
        if self.state == PlantState::NotReady {
            if let Some(a_zombie_idx) = self.find_squash_target() {
                if let Some(board) = self.base.board {
                    unsafe {
                        let zombies_ref = &(*board).zombies;
                        if let Some(zombie) = zombies_ref.get(a_zombie_idx) {
                            // C++: mTargetZombieID = mBoard->ZombieGetID(aZombie);
                            self.target_zombie_id = (*board).zombie_get_id(zombie);
                            // C++: mTargetX = aZombie->ZombieTargetLeadX(0.0f) - mWidth / 2;
                            self.target_x = (zombie.zombie_target_lead_x(0.0) - self.base.width as f32 / 2.0) as i32;
                        }
                    }
                }
                self.state = PlantState::SquashLook;
                self.state_countdown = 80;
                // C++: PlayBodyReanim(mTargetX < mX ? "anim_lookleft" : "anim_lookright", REANIM_PLAY_ONCE_AND_HOLD, 10, 24.0f)
                let a_look = if self.target_x < self.base.x { "anim_lookleft" } else { "anim_lookright" };
                self.play_body_reanim(a_look, ReanimLoopType::PlayOnceAndHold, 10, 24.0);
                // C++: mApp->PlayFoley(FOLEY_SQUASH_HMM)
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::SquashHmm as i32);
                }
            }
        } else if self.state == PlantState::SquashLook {
            if self.state_countdown <= 0 {
                // C++: PlayBodyReanim("anim_jumpup", REANIM_PLAY_ONCE_AND_HOLD, 20, 24.0f)
                self.play_body_reanim("anim_jumpup", ReanimLoopType::PlayOnceAndHold, 20, 24.0);
                self.state = PlantState::SquashPreLaunch;
                self.state_countdown = 45;
            }
        } else if self.state == PlantState::SquashPreLaunch {
            if self.state_countdown <= 0 {
                if let Some(a_zombie_idx) = self.find_squash_target() {
                    if let Some(board) = self.base.board {
                        unsafe {
                            let zombies_ref = &(*board).zombies;
                            if let Some(zombie) = zombies_ref.get(a_zombie_idx) {
                                // C++: mTargetX = aZombie->ZombieTargetLeadX(30.0f) - mWidth / 2;
                                self.target_x = (zombie.zombie_target_lead_x(30.0) - self.base.width as f32 / 2.0) as i32;
                            }
                        }
                    }
                }
                self.state = PlantState::SquashRising;
                self.state_countdown = 50;
                // C++: mRenderOrder = Board::MakeRenderOrder(RENDER_LAYER_PARTICLE, mRow, 0);
                self.base.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_PARTICLE, self.base.row, 0);
            }
        } else if self.state == PlantState::SquashRising
            || self.state == PlantState::SquashFalling
            || self.state == PlantState::SquashDoneFalling
        {
            // C++ else 分支（1488-1607）
            let a_target_col;
            let a_dest_y;
            let board_ptr = self.base.board;
            if let Some(board) = board_ptr {
                unsafe {
                    // C++: aTargetCol = mBoard->PixelToGridXKeepOnBoard(mTargetX, mY);
                    a_target_col = (*board).pixel_to_grid_x_keep_on_board(self.target_x, self.base.y);
                    // C++: aDestY = mBoard->GridToPixelY(aTargetCol, mRow) + 8;
                    a_dest_y = (*board).grid_to_pixel_y(a_target_col, self.base.row) + 8;
                }
            } else {
                return;
            }

            if self.state == PlantState::SquashRising {
                // C++: mX = PvzpAnimateCurve(50, 20, mStateCountdown, GridToPixelX(mPlantCol, mStartRow), mTargetX, CURVE_EASE_IN_OUT);
                //      mY = PvzpAnimateCurve(50, 20, mStateCountdown, GridToPixelY(mPlantCol, mStartRow), aDestY - 120, CURVE_EASE_IN_OUT);
                if let Some(board) = board_ptr {
                    unsafe {
                        let a_start_x = (*board).grid_to_pixel_x(self.plant_col, self.start_row);
                        let a_start_y = (*board).grid_to_pixel_y(self.plant_col, self.start_row);
                        self.base.x = crate::todlib::tod_common::tod_animate_curve(
                            50, 20, self.state_countdown, a_start_x, self.target_x, TodCurves::EaseInOut,
                        );
                        self.base.y = crate::todlib::tod_common::tod_animate_curve(
                            50, 20, self.state_countdown, a_start_y, a_dest_y - 120, TodCurves::EaseInOut,
                        );
                    }
                }
                if self.state_countdown == 0 {
                    // C++: PlayBodyReanim("anim_jumpdown", REANIM_PLAY_ONCE_AND_HOLD, 0, 60.0f);
                    self.play_body_reanim("anim_jumpdown", ReanimLoopType::PlayOnceAndHold, 0, 60.0);
                    self.state = PlantState::SquashFalling;
                    self.state_countdown = 10;
                }
            } else if self.state == PlantState::SquashFalling {
                // C++: mY = PvzpAnimateCurve(10, 0, mStateCountdown, aDestY - 120, aDestY, CURVE_EASE_IN_OUT);
                if let Some(board) = board_ptr {
                    unsafe {
                        self.base.y = crate::todlib::tod_common::tod_animate_curve(
                            10, 0, self.state_countdown, a_dest_y - 120, a_dest_y, TodCurves::EaseInOut,
                        );
                    }
                }
                if self.state_countdown == 5 {
                    self.do_squash_damage();
                }
                if self.state_countdown == 0 {
                    let a_is_pool = board_ptr.map_or(false, |b| unsafe { (*b).is_pool_square(a_target_col, self.base.row) });
                    if a_is_pool {
                        // C++: AddReanimation(mX - 11, mY + 20, mRenderOrder + 1, REANIM_SPLASH) + FOLEY_SPLAT + SOUND_ZOMBIESPLASH + Die()
                        let a_splash_x = self.base.x - 11;
                        let a_splash_y = self.base.y + 20;
                        let a_splash_render = self.base.render_order + 1;
                        if let Some(app) = self.base.get_app_mut() {
                            app.add_reanimation(
                                a_splash_x as f32,
                                a_splash_y as f32,
                                a_splash_render,
                                ReanimationType::Splash as i32,
                            );
                            app.play_foley(crate::todlib::tod_foley::FoleyType::Splat as i32);
                            app.play_sample(unsafe { crate::todlib::tod_foley::SOUND_ZOMBIESPLASH });
                        }
                        self.die();
                    } else {
                        // C++ 1564-1571: 陆地落定
                        self.state = PlantState::SquashDoneFalling;
                        self.state_countdown = 100;
                        // C++: mBoard->ShakeBoard(1, 4); mApp->PlayFoley(FOLEY_THUMP);
                        if let Some(board) = board_ptr {
                            unsafe { (*board).shake_board(1, 4); }
                        }
                        if let Some(app) = self.base.get_app() {
                            app.play_foley(crate::todlib::tod_foley::FoleyType::Thump as i32);
                        }
                        // C++: aOffsetY = mBoard->StageHasRoof() ? 69.0f : 80.0f;
                        let a_offset_y = board_ptr.map_or(80.0, |b| unsafe {
                            if (*b).stage_has_roof() { 69.0 } else { 80.0 }
                        });
                        // C++: mApp->AddPvzpParticle(mX + 40, mY + aOffsetY, mRenderOrder + 4, PARTICLE_DUST_SQUASH);
                        let a_dust_x = self.base.x + 40;
                        let a_dust_y = self.base.y as f32 + a_offset_y;
                        let a_dust_render = self.base.render_order + 4;
                        if let Some(app) = self.base.get_app_mut() {
                            app.add_tod_particle(
                                a_dust_x as f32,
                                a_dust_y,
                                a_dust_render,
                                ParticleEffect::DustSquash as i32,
                            );
                        }
                    }
                }
            } else if self.state == PlantState::SquashDoneFalling {
                if self.state_countdown == 0 {
                    self.die();
                }
            }
        }
    }
    pub fn update_tanglekelp(&mut self) {
        if self.state != PlantState::TanglekelpGrabbing {
            let a_zombie_id = self.find_target_zombie(self.base.row, PlantWeapon::Primary);
            if let Some(zombie_id) = a_zombie_id {
                if let Some(app) = self.base.get_app() {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::Floop as i32);
                }
                self.state = PlantState::TanglekelpGrabbing;
                self.state_countdown = 100;
                self.target_zombie_id = zombie_id;
                if let Some(board) = self.base.get_board_mut() {
                    if let Some(zombie) = board.zombie_try_to_get_mut(zombie_id) {
                        zombie.pool_splash(false);
                    }
                }
                // [TRANSLATION_NOTE]: AddAttachedReanim(REANIM_TANGLEKELP) + Snorkel/Dolphin 偏移 — reanim 未接入
            }
        } else {
            if self.state_countdown == 50 {
                if let Some(board) = self.base.get_board_mut() {
                    if let Some(zombie) = board.zombie_try_to_get_mut(self.target_zombie_id) {
                        zombie.drag_under();
                        zombie.pool_splash(false);
                    }
                }
            }
            if self.state_countdown == 20 {
                // [TRANSLATION_NOTE]: REANIM_SPLASH + PARTICLE_PLANTING_POOL + FOLEY_ZOMBIE_ENTERING_WATER 未接入
            }
            if self.state_countdown == 0 {
                self.die();
                if let Some(board) = self.base.get_board_mut() {
                    if let Some(zombie) = board.zombie_try_to_get_mut(self.target_zombie_id) {
                        zombie.die_with_loot();
                    }
                }
            }
        }
    }
    pub fn update_scaredy_shroom(&mut self) {
        if self.shooting_counter > 0 {
            return;
        }

        // 检测附近是否有僵尸（对应 C++ 圆形判定：120 半径、行差 ±1、非被控、非死亡）
        let mut has_zombie_nearby = false;
        let my_x = self.base.x;
        let my_y = self.base.y;
        let my_row = self.base.row;
        if let Some(board) = self.base.get_board() {
            for zombie in &board.zombies {
                if zombie.dead { continue; }
                let z_rect = zombie.get_zombie_rect();
                let a_diff_y = if zombie.zombie_type == ZombieType::Boss { 0 } else { zombie.base.row - my_row };
                if !zombie.mind_controlled
                    && !zombie.is_dead_or_dying()
                    && a_diff_y <= 1 && a_diff_y >= -1
                    && crate::lawn::board::get_circle_rect_overlap(my_x, my_y + 20, 120, &z_rect)
                {
                    has_zombie_nearby = true;
                    break;
                }
            }
        }

        // 状态机：Ready→Lowering→Scared→Raising→Ready
        if self.state == PlantState::Ready {
            if has_zombie_nearby {
                self.state = PlantState::ScaredyshroomLowering;
                self.play_body_reanim("anim_scared", ReanimLoopType::PlayOnceAndHold, 10, 10.0);
            }
        } else if self.state == PlantState::ScaredyshroomLowering {
            // [TRANSLATION_NOTE]: C++ 需 aBodyReanim->mLoopCount>0 — reanim 未接入
            self.play_body_reanim("anim_scaredidle", ReanimLoopType::Loop, 10, 0.0);
            self.state = PlantState::ScaredyshroomScared;
        } else if self.state == PlantState::ScaredyshroomScared {
            if !has_zombie_nearby {
                self.state = PlantState::ScaredyshroomRaising;
                let a_anim_rate = 7.0 + RandFloat(5.0); // RandRangeFloat(7.0, 12.0)
                self.play_body_reanim("anim_grow", ReanimLoopType::PlayOnceAndHold, 10, a_anim_rate);
            }
        } else if self.state == PlantState::ScaredyshroomRaising {
            // [TRANSLATION_NOTE]: C++ 需 aBodyReanim->mLoopCount>0 — reanim 未接入
            let a_anim_rate = 10.0 + RandFloat(5.0); // RandRangeFloat(10.0, 15.0)
            self.play_idle_anim(a_anim_rate);
            self.state = PlantState::Ready;
        }

        // 非 Ready 状态重置发射计数器
        if self.state != PlantState::Ready {
            self.launch_counter = self.launch_rate;
        }
    }

    pub fn update_spikeweed(&mut self) {
        // 对应 C++ UpdateSpikeweed
        if self.state == PlantState::SpikeweedAttacking {
            if self.state_countdown == 0 {
                self.state = PlantState::NotReady;
                self.play_idle_anim(12.0 + RandFloat(3.0));  // RandRangeFloat(12, 15)
            } else if self.seed_type == SeedType::Spikerock {
                if self.state_countdown == 70 || self.state_countdown == 32 {
                    self.do_row_area_damage(20, 33);
                }
            } else if self.state_countdown == 75 {
                self.do_row_area_damage(20, 33);
            }
        } else if self.find_target_zombie(self.base.row, PlantWeapon::Primary).is_some() {
            self.spikeweed_attack();
        }
    }

    /// 尖刺攻击（对应 C++ SpikeweedAttack）
    pub fn spikeweed_attack(&mut self) {
        // C++: PVZP_ASSERT(IsSpiky())
        if self.state != PlantState::SpikeweedAttacking {
            self.play_body_reanim("anim_attack", ReanimLoopType::PlayOnceAndHold, 20, 18.0);
            // [TRANSLATION_NOTE]: PlaySample(SOUND_THROW) — 声音未接入
            self.state = PlantState::SpikeweedAttacking;
            self.state_countdown = 100;
        }
    }

    /// 尖刺岩石受击（对应 C++ Plant::SpikeRockTakeDamage）
    pub fn spike_rock_take_damage(&mut self) {
        self.spikeweed_attack();
        self.plant_health -= 50;
        // [TRANSLATION_NOTE]: AssignRenderGroupToTrack("bigspike3/2", HIDDEN) 依赖 reanim，暂不接入
        if self.plant_health <= 0 {
            self.die();
        }
    }

    pub fn update_sun_shroom(&mut self) {
        if self.state == PlantState::SunshroomSmall {
            if self.state_countdown == 0 {
                self.state = PlantState::SunshroomGrowing;
            }
            self.update_production_plant();
        } else if self.state == PlantState::SunshroomGrowing {
            self.state = PlantState::SunshroomBig;
        } else {
            self.update_production_plant();
        }
    }
    /// 吹走飞行僵尸（对应 C++ Plant::BlowAwayFliers）
    pub fn blow_away_fliers(&mut self) {
        if let Some(board) = self.base.get_board_mut() {
            for zombie in &mut board.zombies {
                if zombie.dead { continue; }
                if !zombie.is_dead_or_dying() {
                    // 只吹走气球飞行中的僵尸；爆裂中的气球不包含
                    if zombie.zombie_phase == ZombiePhase::BalloonFlying {
                        zombie.blowing_away = true;
                    }
                }
            }
        }
        if let Some(board) = self.base.get_board_mut() {
            board.m_fog_blown_count_down = 4000;
        }
    }

    /// 杀死周围植物（对应 C++ Plant::KillAllPlantsNearDoom）
    pub fn kill_all_plants_near_doom(&mut self) {
        let my_row = self.base.row;
        let my_col = self.plant_col;
        if let Some(board) = self.base.get_board_mut() {
            let cols: Vec<usize> = board.plants.iter().enumerate()
                .filter(|(_, p)| !p.dead && p.base.row == my_row && p.plant_col == my_col)
                .map(|(i, _)| i)
                .collect();
            for idx in cols {
                board.plants[idx].die();
            }
        }
    }

    pub fn do_special(&mut self) {
        let a_pos_x = self.base.x + self.base.width / 2;
        let a_pos_y = self.base.y + self.base.height / 2;

        match self.seed_type {
            SeedType::Blover => {
                if self.state != PlantState::DoingSpecial {
                    self.state = PlantState::DoingSpecial;
                    // 依赖底层系统
                }
            }
            SeedType::Cherrybomb => {
                // 依赖底层系统
                self.die();
            }
            SeedType::Doomshroom => {
                // 依赖底层系统
                self.die();
            }
            SeedType::Jalapeno => {
                // 依赖底层系统
                self.die();
            }
            SeedType::Umbrella => {
                if self.state != PlantState::UmbrellaTriggered && self.state != PlantState::UmbrellaReflecting {
                    self.state = PlantState::UmbrellaTriggered;
                    self.state_countdown = 5;
                }
            }
            SeedType::Iceshroom => {
                // 依赖底层系统
                self.die();
            }
            SeedType::PotatoMine => {
                // 依赖底层系统
                self.die();
            }
            SeedType::InstantCoffee => {
                // 依赖底层系统
                self.state = PlantState::DoingSpecial;
            }
            _ => {}
        }
    }
/// 玉米炮开火（对应 C++ CobCannonFire）
    pub fn cob_cannon_fire(&mut self, target_x: i32, target_y: i32) {
        // C++: PVZP_ASSERT(mState == STATE_COBCANNON_READY)
        self.state = PlantState::CobcannonFiring;
        self.shooting_counter = 206;
        self.play_body_reanim("anim_shooting", ReanimLoopType::PlayOnceAndHold, 20, 12.0);
        self.target_x = target_x - 47;
        self.target_y = target_y;
        // [TRANSLATION_NOTE]: CobCannon_Cob 轨道颜色置白 — reanim 未接入
    }

    /// 获取植物名称字符串（对应 C++ Plant::GetNameString，静态）
    pub fn get_name_string(seed_type: SeedType, imitater_type: SeedType) -> String {
        let a_plant_def = get_plant_definition(seed_type);
        let a_name = format!("[{}]", a_plant_def.plant_name.unwrap_or(""));
        // [TRANSLATION_NOTE]: PvzpStringTranslate 字符串翻译系统未接入，直接返回原始标记
        let a_translated_name = a_name;
        if seed_type == SeedType::Imitater && imitater_type != SeedType::None {
            let a_imitater_def = get_plant_definition(imitater_type);
            let a_imitater_name = format!("[{}]", a_imitater_def.plant_name.unwrap_or(""));
            let a_translated_imitater_name = a_imitater_name;
            return format!("{} {}", a_translated_name, a_translated_imitater_name);
        }
        a_translated_name
    }

    /// 获取植物图鉴说明文案（对应 C++ Plant::GetToolTip，静态）
    /// C++: StrFormat("[%s_TOOLTIP]", aPlantDef.mPlantName) → PvzpStringTranslate
    pub fn get_tool_tip(seed_type: SeedType) -> String {
        let a_plant_def = get_plant_definition(seed_type);
        let a_tool_tip = format!("[{}_TOOLTIP]", a_plant_def.plant_name.unwrap_or(""));
        // [TRANSLATION_NOTE]: PvzpStringTranslate 字符串翻译系统未接入，直接返回原始标记
        let a_translated_tool_tip = a_tool_tip;
        a_translated_tool_tip
    }

    /// 获取刷新时间（对应 C++ Plant::GetRefreshTime，静态）
    pub fn get_refresh_time(seed_type: SeedType, imitater_type: SeedType) -> i32 {
        if crate::lawn::challenge::Challenge::is_zombie_seed_type(seed_type) != 0 {
            return 0;
        }
        if seed_type == SeedType::Imitater && imitater_type != SeedType::None {
            get_plant_definition(imitater_type).refresh_time
        } else {
            get_plant_definition(seed_type).refresh_time
        }
    }
}

impl Default for Plant {
    fn default() -> Self {
        Plant::new()
    }
}

/// 植物定义表条目（对应 C++ PlantDefinition）
/// 存储每个种子类型的静态属性：图像、重动画类型、花费、冷却时间等
/// 全局数组 gPlantDefs[SeedType::NUM_SEED_TYPES] 和 GetPlantDefinition() 函数
/// 将在 Plant.cpp 主体翻译时实现
#[derive(Debug, Clone, Copy)]
pub struct PlantDefinition {
    pub seed_type: SeedType,
    pub plant_image: Option<*mut Image>,
    pub reanimation_type: ReanimationType,
    pub packet_index: i32,
    pub seed_cost: i32,
    pub refresh_time: i32,
    pub sub_class: PlantSubClass,
    pub launch_rate: i32,
    pub plant_name: Option<&'static str>,
}

/// 获取植物图片（对应 C++ Plant::GetImage，Plant.cpp:3802）
/// C++: GetPlantDefinition(theSeedType).mPlantImage[0]，无图返回 nullptr
pub fn get_image(seed_type: SeedType) -> Option<*mut Image> {
    get_plant_definition(seed_type).plant_image
}

/// 获取植物定义（对应 C++ GetPlantDefinition）
pub fn get_plant_definition(seed_type: SeedType) -> PlantDefinition {
    match seed_type {
        SeedType::Peashooter => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Peashooter, packet_index: 0, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("PEASHOOTER") },
        SeedType::Sunflower => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Sunflower, packet_index: 1, seed_cost: 50, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 2500, plant_name: Some("SUNFLOWER") },
        SeedType::Cherrybomb => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Cherrybomb, packet_index: 3, seed_cost: 150, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("CHERRY_BOMB") },
        SeedType::Wallnut => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Wallnut, packet_index: 2, seed_cost: 50, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("WALL_NUT") },
        SeedType::PotatoMine => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Potatomine, packet_index: 37, seed_cost: 25, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("POTATO_MINE") },
        SeedType::Snowpea => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Snowpea, packet_index: 4, seed_cost: 175, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("SNOW_PEA") },
        SeedType::Chomper => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Chomper, packet_index: 31, seed_cost: 150, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("CHOMPER") },
        SeedType::Repeater => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Repeater, packet_index: 5, seed_cost: 200, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("REPEATER") },
        SeedType::Puffshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Puffshroom, packet_index: 6, seed_cost: 0, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("PUFF_SHROOM") },
        SeedType::Sunshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Sunshroom, packet_index: 7, seed_cost: 25, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 2500, plant_name: Some("SUN_SHROOM") },
        SeedType::Fumeshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Fumeshroom, packet_index: 9, seed_cost: 75, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("FUME_SHROOM") },
        SeedType::Gravebuster => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::GraveBuster, packet_index: 40, seed_cost: 75, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("GRAVE_BUSTER") },
        SeedType::Hypnoshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Hypnoshroom, packet_index: 10, seed_cost: 75, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("HYPNO_SHROOM") },
        SeedType::Scaredyshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Scareyshroom, packet_index: 33, seed_cost: 25, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("SCAREDY_SHROOM") },
        SeedType::Iceshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Iceshroom, packet_index: 36, seed_cost: 75, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("ICE_SHROOM") },
        SeedType::Doomshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Doomshroom, packet_index: 20, seed_cost: 125, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("DOOM_SHROOM") },
        SeedType::Lilypad => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Lilypad, packet_index: 19, seed_cost: 25, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("LILY_PAD") },
        SeedType::Squash => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Squash, packet_index: 21, seed_cost: 50, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("SQUASH") },
        SeedType::Threepeater => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Threepeater, packet_index: 12, seed_cost: 325, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("THREEPEATER") },
        SeedType::Tanglekelp => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Tanglekelp, packet_index: 17, seed_cost: 25, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("TANGLE_KELP") },
        SeedType::Jalapeno => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Jalapeno, packet_index: 11, seed_cost: 125, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("JALAPENO") },
        SeedType::Spikeweed => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Spikeweed, packet_index: 22, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("SPIKEWEED") },
        SeedType::Torchwood => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Torchwood, packet_index: 29, seed_cost: 175, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("TORCHWOOD") },
        SeedType::Tallnut => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Tallnut, packet_index: 28, seed_cost: 125, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("TALL_NUT") },
        SeedType::Seashroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Seashroom, packet_index: 39, seed_cost: 0, refresh_time: 3000, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("SEA_SHROOM") },
        SeedType::Plantern => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Plantern, packet_index: 38, seed_cost: 25, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 2500, plant_name: Some("PLANTERN") },
        SeedType::Cactus => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Cactus, packet_index: 15, seed_cost: 125, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("CACTUS") },
        SeedType::Blover => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Blover, packet_index: 18, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("BLOVER") },
        SeedType::Splitpea => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Splitpea, packet_index: 32, seed_cost: 125, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("SPLIT_PEA") },
        SeedType::Starfruit => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Starfruit, packet_index: 30, seed_cost: 125, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("STARFRUIT") },
        SeedType::Pumpkinshell => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Pumpkin, packet_index: 25, seed_cost: 125, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("PUMPKIN") },
        SeedType::Magnetshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Magnetshroom, packet_index: 35, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("MAGNET_SHROOM") },
        SeedType::Cabbagepult => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Cabbagepult, packet_index: 13, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 300, plant_name: Some("CABBAGE_PULT") },
        SeedType::Flowerpot => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::FlowerPot, packet_index: 33, seed_cost: 25, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("FLOWER_POT") },
        SeedType::Kernelpult => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Kernelpult, packet_index: 13, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 300, plant_name: Some("KERNEL_PULT") },
        SeedType::InstantCoffee => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::CoffeeBean, packet_index: 33, seed_cost: 75, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("COFFEE_BEAN") },
        SeedType::Garlic => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Garlic, packet_index: 8, seed_cost: 50, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("GARLIC") },
        SeedType::Umbrella => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Umbrellaleaf, packet_index: 23, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("UMBRELLA_LEAF") },
        SeedType::Marigold => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Marigold, packet_index: 24, seed_cost: 50, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 2500, plant_name: Some("MARIGOLD") },
        SeedType::Melonpult => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Melonpult, packet_index: 14, seed_cost: 300, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 300, plant_name: Some("MELON_PULT") },
        SeedType::Gatlingpea => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Gatlingpea, packet_index: 5, seed_cost: 250, refresh_time: 5000, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("GATLING_PEA") },
        SeedType::Twinsunflower => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::TwinSunflower, packet_index: 1, seed_cost: 150, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 2500, plant_name: Some("TWIN_SUNFLOWER") },
        SeedType::Gloomshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Gloomshroom, packet_index: 27, seed_cost: 150, refresh_time: 5000, sub_class: PlantSubClass::Shooter, launch_rate: 200, plant_name: Some("GLOOM_SHROOM") },
        SeedType::Cattail => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Cattail, packet_index: 27, seed_cost: 225, refresh_time: 5000, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("CATTAIL") },
        SeedType::Wintermelon => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::WinterMelon, packet_index: 27, seed_cost: 200, refresh_time: 5000, sub_class: PlantSubClass::Shooter, launch_rate: 300, plant_name: Some("WINTER_MELON") },
        SeedType::GoldMagnet => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::GoldMagnet, packet_index: 27, seed_cost: 50, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("GOLD_MAGNET") },
        SeedType::Spikerock => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Spikerock, packet_index: 27, seed_cost: 125, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("SPIKEROCK") },
        SeedType::Cobcannon => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Cobcannon, packet_index: 16, seed_cost: 500, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 600, plant_name: Some("COB_CANNON") },
        SeedType::Imitater => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Imitater, packet_index: 33, seed_cost: 0, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("IMITATER") },
        SeedType::ExplodeONut => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Wallnut, packet_index: 2, seed_cost: 0, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("EXPLODE_O_NUT") },
        SeedType::GiantWallnut => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Wallnut, packet_index: 2, seed_cost: 0, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("GIANT_WALLNUT") },
        SeedType::Sprout => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::ZengardenSprout, packet_index: 33, seed_cost: 0, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("SPROUT") },
        SeedType::Leftpeater => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::Repeater, packet_index: 5, seed_cost: 200, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("REPEATER") },
        _ => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 0, seed_cost: 0, refresh_time: 0, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: None },
    }
}







