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
            let a_offset_y = Self::plant_draw_height_offset(self.base.get_board(), self, seed_type, grid_x, grid_y);
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
            if app.game_mode == GameMode::ChallengeTimeAttack &&
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
        // [TRANSLATION_NOTE]: 场景判断暂略
        do_update = true;

        if do_update {
            self.update_abilities();
            // [TRANSLATION_NOTE]: Animate + UpdateReanim 暂未实现

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

    /// 更新射手类植物（对应 C++ UpdateShooter）
    pub fn update_shooter(&mut self) {
        self.launch_counter -= 1;
        if self.launch_counter <= 0 {
            self.launch_counter = self.launch_rate - 15 + RandRange(15); // Rand(15)

            match self.seed_type {
                SeedType::Threepeater => {
                    // [TRANSLATION_NOTE]: LaunchThreepeater 暂未实现
                }
                SeedType::Starfruit => {
                    // [TRANSLATION_NOTE]: LaunchStarFruit 暂未实现
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
        // [TRANSLATION_NOTE]: IsInPlay/IZombie/LastStand 检查暂略
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

    /// 发射子弹
    pub fn fire(&mut self, _target_zombie: Option<&mut Zombie>, _row: i32, _weapon: PlantWeapon) {
        let x = self.base.x;
        let y = self.base.y;
        let row = self.base.row;
        let seed_type = self.seed_type;
        if let Some(board) = self.base.get_board_mut() {
            board.add_projectile((x + 40) as f32, (y + 20) as f32, row, seed_type);
        }
    }

    /// 寻找目标僵尸
    pub fn find_target_zombie(&self, _row: i32, _weapon: PlantWeapon) -> Option<ZombieID> {
        None
    }

    /// 植物死亡
    pub fn die(&mut self) {
        self.dead = true;
        self.is_on_board = false;
    }

    /// 绘制植物
    pub fn draw(&self, _g: &mut Graphics) {}

    /// 绘制影子
    pub fn draw_shadow(&self, _g: &mut Graphics, _offset_x: f32, _offset_y: f32) {}

    /// 更新特殊能力（对应 C++ UpdateAbilities）
    pub fn update_abilities(&mut self) {
        if !self.is_in_play() {
            return;
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

    pub fn is_upgrade(seed_type: SeedType) -> bool {
        matches!(seed_type, SeedType::Gatlingpea | SeedType::Twinsunflower |
            SeedType::Gloomshroom | SeedType::Cattail | SeedType::Wintermelon |
            SeedType::GoldMagnet | SeedType::Spikerock | SeedType::Cobcannon)
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

    /// 计算渲染顺序（对应 C++ CalcRenderOrder）
    pub fn calc_render_order(&self) -> i32 { 0 }

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

    /// 绘制高度偏移（对应 C++ PlantDrawHeightOffset，静态函数）
    pub fn plant_draw_height_offset(_board: Option<&Board>, _plant: &Plant, _seed_type: SeedType, _grid_x: i32, _grid_y: i32) -> f32 { 0.0 }

    // ========== 特殊植物更新 stub ==========
    pub fn update_doom_shroom(&mut self) {}
    pub fn update_ice_shroom(&mut self) {}
    pub fn update_chomper(&mut self) {
        if self.state == PlantState::Ready {
            // [TRANSLATION_NOTE]: FindTargetZombie 暂未实现
            self.state = PlantState::ChomperBiting;
            self.state_countdown = 70;
        } else if self.state == PlantState::ChomperBiting {
            if self.state_countdown == 0 {
                // [TRANSLATION_NOTE]: 大嘴花音效/伤害/吞食判定暂未实现
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
        // [TRANSLATION_NOTE]: 火炬树桩碰撞检测依赖 Projectile 的 ConvertToFireball
        // 暂不实现
    }
    pub fn update_blover(&mut self) {}
    pub fn update_flower_pot(&mut self) {}
    pub fn update_lilypad(&mut self) {}
    pub fn update_imitater(&mut self) {}
    pub fn update_coffee_bean(&mut self) {}
    pub fn update_umbrella(&mut self) {}
    pub fn update_cob_cannon(&mut self) {}
    pub fn update_cactus(&mut self) {}
    pub fn update_magnet_shroom(&mut self) {}
    pub fn update_gold_magnet_shroom(&mut self) {}
    pub fn update_sun_shroom(&mut self) {}
    pub fn update_grave_buster(&mut self) {}
    pub fn update_potato(&mut self) {
        if self.state == PlantState::NotReady {
            if self.state_countdown == 0 {
                self.state = PlantState::PotatoRising;
                // [TRANSLATION_NOTE]: 上升粒子/音效/动画暂未实现
            }
        } else if self.state == PlantState::PotatoRising {
            // [TRANSLATION_NOTE]: mLoopCount > 0 依赖 Reanimation 系统
            self.state = PlantState::PotatoArmed;
            self.blink_countdown = 400 + RandRange(4000);
        } else if self.state == PlantState::PotatoArmed {
            // [TRANSLATION_NOTE]: FindTargetZombie 暂未实现
            // 若有僵尸接近 → DoSpecial()
        }
    }

    pub fn update_squash(&mut self) {
        if self.state == PlantState::NotReady {
            // [TRANSLATION_NOTE]: FindSquashTarget 暂未实现
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
                // [TRANSLATION_NOTE]: DoSquashDamage 暂未实现
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
    pub fn update_spikeweed(&mut self) {}
    pub fn update_tanglekelp(&mut self) {}
    pub fn update_scaredy_shroom(&mut self) {}
    pub fn do_special(&mut self) {}
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
        SeedType::Peashooter => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 0, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("PEASHOOTER") },
        SeedType::Sunflower => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 1, seed_cost: 50, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 2500, plant_name: Some("SUNFLOWER") },
        SeedType::Cherrybomb => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 3, seed_cost: 150, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("CHERRY_BOMB") },
        SeedType::Wallnut => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 2, seed_cost: 50, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("WALL_NUT") },
        SeedType::PotatoMine => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 37, seed_cost: 25, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("POTATO_MINE") },
        SeedType::Snowpea => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 4, seed_cost: 175, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("SNOW_PEA") },
        SeedType::Chomper => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 31, seed_cost: 150, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("CHOMPER") },
        SeedType::Repeater => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 5, seed_cost: 200, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("REPEATER") },
        SeedType::Puffshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 6, seed_cost: 0, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("PUFF_SHROOM") },
        SeedType::Sunshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 7, seed_cost: 25, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 2500, plant_name: Some("SUN_SHROOM") },
        SeedType::Fumeshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 9, seed_cost: 75, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("FUME_SHROOM") },
        SeedType::Gravebuster => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 40, seed_cost: 75, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("GRAVE_BUSTER") },
        SeedType::Hypnoshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 10, seed_cost: 75, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("HYPNO_SHROOM") },
        SeedType::Scaredyshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 33, seed_cost: 25, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("SCAREDY_SHROOM") },
        SeedType::Iceshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 36, seed_cost: 75, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("ICE_SHROOM") },
        SeedType::Doomshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 20, seed_cost: 125, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("DOOM_SHROOM") },
        SeedType::Lilypad => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 19, seed_cost: 25, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("LILY_PAD") },
        SeedType::Squash => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 21, seed_cost: 50, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("SQUASH") },
        SeedType::Threepeater => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 12, seed_cost: 325, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("THREEPEATER") },
        SeedType::Tanglekelp => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 17, seed_cost: 25, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("TANGLE_KELP") },
        SeedType::Jalapeno => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 11, seed_cost: 125, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("JALAPENO") },
        SeedType::Spikeweed => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 22, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("SPIKEWEED") },
        SeedType::Torchwood => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 29, seed_cost: 175, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("TORCHWOOD") },
        SeedType::Tallnut => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 28, seed_cost: 125, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("TALL_NUT") },
        SeedType::Seashroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 39, seed_cost: 0, refresh_time: 3000, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("SEA_SHROOM") },
        SeedType::Plantern => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 38, seed_cost: 25, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 2500, plant_name: Some("PLANTERN") },
        SeedType::Cactus => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 15, seed_cost: 125, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("CACTUS") },
        SeedType::Blover => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 18, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("BLOVER") },
        SeedType::Splitpea => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 32, seed_cost: 125, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("SPLIT_PEA") },
        SeedType::Starfruit => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 30, seed_cost: 125, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("STARFRUIT") },
        SeedType::Pumpkinshell => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 25, seed_cost: 125, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("PUMPKIN") },
        SeedType::Magnetshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 35, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("MAGNET_SHROOM") },
        SeedType::Cabbagepult => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 13, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 300, plant_name: Some("CABBAGE_PULT") },
        SeedType::Flowerpot => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 33, seed_cost: 25, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("FLOWER_POT") },
        SeedType::Kernelpult => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 13, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 300, plant_name: Some("KERNEL_PULT") },
        SeedType::InstantCoffee => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 33, seed_cost: 75, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("COFFEE_BEAN") },
        SeedType::Garlic => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 8, seed_cost: 50, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("GARLIC") },
        SeedType::Umbrella => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 23, seed_cost: 100, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("UMBRELLA_LEAF") },
        SeedType::Marigold => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 24, seed_cost: 50, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 2500, plant_name: Some("MARIGOLD") },
        SeedType::Melonpult => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 14, seed_cost: 300, refresh_time: 750, sub_class: PlantSubClass::Shooter, launch_rate: 300, plant_name: Some("MELON_PULT") },
        SeedType::Gatlingpea => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 5, seed_cost: 250, refresh_time: 5000, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("GATLING_PEA") },
        SeedType::Twinsunflower => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 1, seed_cost: 150, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 2500, plant_name: Some("TWIN_SUNFLOWER") },
        SeedType::Gloomshroom => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 27, seed_cost: 150, refresh_time: 5000, sub_class: PlantSubClass::Shooter, launch_rate: 200, plant_name: Some("GLOOM_SHROOM") },
        SeedType::Cattail => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 27, seed_cost: 225, refresh_time: 5000, sub_class: PlantSubClass::Shooter, launch_rate: 150, plant_name: Some("CATTAIL") },
        SeedType::Wintermelon => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 27, seed_cost: 200, refresh_time: 5000, sub_class: PlantSubClass::Shooter, launch_rate: 300, plant_name: Some("WINTER_MELON") },
        SeedType::GoldMagnet => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 27, seed_cost: 50, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("GOLD_MAGNET") },
        SeedType::Spikerock => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 27, seed_cost: 125, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("SPIKEROCK") },
        SeedType::Cobcannon => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 16, seed_cost: 500, refresh_time: 5000, sub_class: PlantSubClass::Normal, launch_rate: 600, plant_name: Some("COB_CANNON") },
        SeedType::Imitater => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 33, seed_cost: 0, refresh_time: 750, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("IMITATER") },
        SeedType::ExplodeONut => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 2, seed_cost: 0, refresh_time: 3000, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: Some("EXPLODE_O_NUT") },
        _ => PlantDefinition { seed_type, plant_image: None, reanimation_type: ReanimationType::None, packet_index: 0, seed_cost: 0, refresh_time: 0, sub_class: PlantSubClass::Normal, launch_rate: 0, plant_name: None },
    }
}
