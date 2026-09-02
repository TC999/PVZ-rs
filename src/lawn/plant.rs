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
                // SetTruncateDisappearingFrames — stub
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
                // AssignRenderGroupToPrefix("Cornpult_butter", RENDER_GROUP_HIDDEN) — stub
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
                // SetTruncateDisappearingFrames — stub
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

        let mut do_update = false;
        // 依赖底层系统
        do_update = true;

        if do_update {
            self.update_abilities();
            // 依赖底层系统

            if self.plant_health < 0 {
                self.die();
            }
        }
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
        // 依赖底层系统
        if self.makes_sun() {
            self.launch_counter -= 1;
            if self.launch_counter <= 0 {
                self.launch_counter = self.launch_rate;
                let bx = self.base.x;
                let by = self.base.y;
                if let Some(board) = self.base.get_board_mut() {
                    board.add_coin((bx + 20) as f32, (by - 10) as f32, CoinType::Sun, CoinMotion::FromPlant);
                }
            }
        }
    }

    /// 寻找目标并开火
    pub fn find_target_and_fire(&mut self, _the_row: i32, _weapon: PlantWeapon) -> bool {
        // 检查发射计数器
        self.shooting_counter += 1;
        if self.shooting_counter < self.launch_rate {
            return false;
        }
        self.shooting_counter = 0;

        // 预先复制值，避免借用冲突
        let row = self.base.row;
        let plant_col = self.plant_col;
        let seed_type = self.seed_type;
        let x = self.base.x;
        let y = self.base.y;

        // 在 Board 中找到目标僵尸
        if let Some(board) = self.base.get_board_mut() {
            if board.find_zombie_in_row(row, plant_col).is_some() {
                // 发射子弹
                board.add_projectile((x + 40) as f32, (y + 20) as f32, row, seed_type);
                return true;
            }
        }
        false
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

    /// 发射子弹/效果（对应 C++ Plant::Fire 简化版）
    pub fn fire(&mut self, _target_zombie: Option<&mut Zombie>, _row: i32, _weapon: PlantWeapon) {
        // 特殊植物直接造成范围伤害
        match self.seed_type {
            SeedType::Fumeshroom | SeedType::Gloomshroom => {
                // 依赖底层系统
                return;
            }
            SeedType::Starfruit => {
                // 依赖底层系统
                return;
            }
            _ => {}
        }

        // 普通植物发射子弹
        let x = self.base.x;
        let y = self.base.y;
        let row = self.base.row;
        if let Some(board) = self.base.get_board_mut() {
            board.add_projectile((x + 40) as f32, (y + 20) as f32, row, self.seed_type);
        }
    }

    /// 寻找目标僵尸
    pub fn find_target_zombie(&self, row: i32, _weapon: PlantWeapon) -> Option<ZombieID> {
        if let Some(board) = self.base.get_board() {
            let attack_rect = self.get_plant_attack_rect(_weapon);
            let mut best_id = None;
            let mut best_weight = -999999;
            for zombie in &board.zombies {
                if zombie.dead { continue; }
                let row_dev = if zombie.zombie_type == ZombieType::Boss { 0 } else { zombie.base.row - row };
                if row_dev != 0 { continue; }
                let z_rect = zombie.get_zombie_rect();
                if crate::lawn::board::get_rect_overlap(&attack_rect, &z_rect) >= 0 {
                    let weight = -z_rect.x;
                    if best_id.is_none() || weight > best_weight {
                        best_weight = weight;
                        best_id = Some(zombie.base.render_order as u32);
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

    /// 植物死亡
    pub fn die(&mut self) {
        self.dead = true;
        self.is_on_board = false;
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

        if let Some(board) = self.base.get_board_mut() {
            for (_idx, zombie) in board.zombies.iter_mut().enumerate() {
                if zombie.dead { continue; }
                let a_diff_y = if zombie.zombie_type == ZombieType::Boss { 0 } else { zombie.base.row - my_row };
                if self.seed_type == SeedType::Gloomshroom {
                    if a_diff_y < -1 || a_diff_y > 1 { continue; }
                } else if a_diff_y != 0 { continue; }

                let z_rect = zombie.get_zombie_rect();
                if crate::lawn::board::get_rect_overlap(&a_attack_rect, &z_rect) > 0 {
                    // 依赖底层系统
                }
            }
        }
    }

    /// 获取伤害范围标志（对应 C++ GetDamageRangeFlags）
    pub fn get_damage_range_flags(&self, _weapon: PlantWeapon) -> u32 {
        // 依赖底层系统
        1
    }

    /// 获取植物攻击矩形（对应 C++ GetPlantAttackRect）
    pub fn get_plant_attack_rect(&self, _weapon: PlantWeapon) -> Rect {
        Rect::new(self.base.x - 20, self.base.y - 20, self.base.width + 40, self.base.height + 40)
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

    /// 更新射击（对应 C++ UpdateShooting）
    pub fn update_shooting(&mut self) {
        self.launch_counter -= 1;
        if self.launch_counter <= 0 {
            self.launch_counter = self.launch_rate - 15 + RandRange(15);
            self.find_target_and_fire(self.base.row, PlantWeapon::Primary);
        }
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
    /// [TRANSLATION_NOTE]: C++ 中按种子类型加载对应 reanim 定义与图片；Rust 侧
    /// reanim 定义加载在 reanim_loader 中处理，此处骨架保留调用链
    pub fn preload_plant_resources(_seed_type: SeedType) {}

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
                (*b).get_top_plant_at(self.plant_col, self.base.row).map_or(true, |p| p.seed_type != SeedType::Cattail)
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
    pub fn set_sleeping(&mut self, _is_asleep: bool) {}

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
    pub fn set_body_reanim_frame(&mut self, _layer: &str) {}

    /// 设置身体动画速率（对应 C++ 设置 Reanimation 的 mAnimRate）
    pub fn set_body_reanim_rate(&mut self, _rate: f32) {}

    /// 设置身体动画循环模式（对应 C++ 设置 Reanimation 的 mLoopType）
    pub fn set_body_reanim_loop(&mut self, _loop_type: ReanimLoopType) {}

    /// 设置帧基础姿势（对应 C++ 设置 Reanimation 的 mFrameBasePose）
    pub fn set_body_reanim_frame_base_pose(&mut self, _pose: i32) {}

    /// 添加头部重动画（对应 C++ 添加头部 Reanimation）
    pub fn add_head_reanim(&mut self, _layer: &str) {}

    /// 添加头部重动画2（对应 C++ 添加第二个头部 Reanimation）
    pub fn add_head_reanim2(&mut self, _layer: &str) {}

    /// 添加头部重动画3（对应 C++ 添加第三个头部 Reanimation）
    pub fn add_head_reanim3(&mut self, _layer: &str) {}

    /// 播放身体重动画（对应 C++ Plant::PlayBodyReanim）
    pub fn play_body_reanim(&mut self, _track_name: &str, _loop_type: ReanimLoopType, _blend_time: i32, _anim_rate: f32) {
        // [TRANSLATION_NOTE]: reanim 系统未接入，仅保留调用骨架
    }

    /// 播放待机动画（对应 C++ Plant::PlayIdleAnim）
    pub fn play_idle_anim(&mut self, _rate: f32) {
        // [TRANSLATION_NOTE]: reanim 系统未接入，仅保留调用骨架
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
        // 依赖底层系统
    }

    pub fn update_ice_shroom(&mut self) {
        if !self.is_asleep && self.state != PlantState::DoingSpecial {
            self.state = PlantState::DoingSpecial;
            self.do_special_countdown = 100;
            // 依赖底层系统
        }
    }
    pub fn update_chomper(&mut self) {
        if self.state == PlantState::Ready {
            // 依赖底层系统
            self.state = PlantState::ChomperBiting;
            self.state_countdown = 70;
        } else if self.state == PlantState::ChomperBiting {
            if self.state_countdown == 0 {
                // 依赖底层系统
                self.state = PlantState::ChomperBitingGotOne;
            }
        } else if self.state == PlantState::ChomperBitingGotOne {
            self.state = PlantState::ChomperDigesting;
            self.state_countdown = 4000;
        } else if self.state == PlantState::ChomperDigesting {
            if self.state_countdown == 0 {
                self.state = PlantState::ChomperSwallowing;
            }
        } else if self.state == PlantState::ChomperSwallowing || self.state == PlantState::ChomperBitingMissed {
            self.state = PlantState::Ready;
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
    pub fn update_blover(&mut self) {
        // 依赖底层系统
        // 特殊效果由 DoSpecial → BlowAwayFliers 处理
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
        // 对应 C++ UpdateCobCannon
        if self.state == PlantState::CobcannonArming {
            if self.state_countdown == 0 {
                self.state = PlantState::CobcannonLoading;
                self.play_body_reanim("anim_charge", ReanimLoopType::PlayOnceAndHold, 20, 12.0);
            }
        } else if self.state == PlantState::CobcannonLoading {
            // [TRANSLATION_NOTE]: ShouldTriggerTimedEvent(0.5) + FOLEY_SHOOP；mLoopCount → Ready — reanim 未接入
            self.state = PlantState::CobcannonReady;
            self.play_idle_anim(12.0);
        } else if self.state == PlantState::CobcannonReady {
            // [TRANSLATION_NOTE]: CobCannon_cob 轨道闪烁颜色 — reanim 未接入
        } else if self.state == PlantState::CobcannonFiring {
            // [TRANSLATION_NOTE]: ShouldTriggerTimedEvent(0.48) + FOLEY_COB_LAUNCH — reanim 未接入
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
        if self.shooting_counter > 0 {
            return;
        }
        if self.state == PlantState::CactusRising {
            // 依赖底层系统
            self.state = PlantState::CactusHigh;
            self.launch_counter = 1;
        } else if self.state == PlantState::CactusHigh {
            // 依赖底层系统
        } else if self.state == PlantState::CactusLowering {
            // 依赖底层系统
            self.state = PlantState::CactusLow;
        } else {
            // 依赖底层系统
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
                // [TRANSLATION_NOTE]: PlayBodyReanim(anim_idle) + aBodyReanim->mAnimRate — reanim 未接入
                self.magnet_items[0].item_type = MagnetItemType::None;
            }
        } else if self.state == PlantState::MagnetshroomSucking {
            // [TRANSLATION_NOTE]: mLoopCount > 0 → 换 anim_nonactive_idle2 动画 — reanim 未接入
            self.state = PlantState::MagnetshroomCharging;
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
                self.state = PlantState::PotatoRising;
                // 依赖底层系统
            }
        } else if self.state == PlantState::PotatoRising {
            // 依赖底层系统
            self.state = PlantState::PotatoArmed;
            self.blink_countdown = 400 + RandRange(4000);
        } else if self.state == PlantState::PotatoArmed {
            // 依赖底层系统
            // 若有僵尸接近 → DoSpecial()
        }
    }

    pub fn update_squash(&mut self) {
        if self.state == PlantState::NotReady {
            // 依赖底层系统
            self.state = PlantState::SquashLook;
            self.state_countdown = 80;
        } else if self.state == PlantState::SquashLook {
            if self.state_countdown <= 0 {
                self.state = PlantState::SquashPreLaunch;
                self.state_countdown = 45;
            }
        } else if self.state == PlantState::SquashPreLaunch {
            if self.state_countdown <= 0 {
                self.state = PlantState::SquashRising;
                self.state_countdown = 50;
            }
        } else if self.state == PlantState::SquashRising {
            if self.state_countdown == 0 {
                self.state = PlantState::SquashFalling;
                self.state_countdown = 10;
            }
        } else if self.state == PlantState::SquashFalling {
            if self.state_countdown == 5 {
                self.do_squash_damage();
            }
            if self.state_countdown == 0 {
                self.state = PlantState::SquashDoneFalling;
                self.state_countdown = 100;
            }
        } else if self.state == PlantState::SquashDoneFalling {
            if self.state_countdown == 0 {
                self.die();
            }
        }
    }
    pub fn update_tanglekelp(&mut self) {
        if self.state != PlantState::TanglekelpGrabbing {
            // 依赖底层系统
            self.state = PlantState::TanglekelpGrabbing;
            self.state_countdown = 100;
        } else {
            if self.state_countdown == 0 {
                // 依赖底层系统
                // self.die();
            }
        }
    }
    pub fn update_scaredy_shroom(&mut self) {
        // 依赖底层系统
        // 状态机：Ready→ScaredyshroomLowering→Scared→Raising→Ready
        if self.state == PlantState::Ready {
            // 若有僵尸靠近 → ScaredyshroomLowering
        } else if self.state == PlantState::ScaredyshroomLowering {
            self.state = PlantState::ScaredyshroomScared;
        } else if self.state == PlantState::ScaredyshroomScared {
            // 若无僵尸靠近 → Raising
        } else if self.state == PlantState::ScaredyshroomRaising {
            self.state = PlantState::Ready;
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
        SeedType::ExplodeONut => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 2, seed_cost: 0, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("EXPLODE_O_NUT") },
        _ => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 0, seed_cost: 0, refresh_time: 0, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: None },
    }
}







