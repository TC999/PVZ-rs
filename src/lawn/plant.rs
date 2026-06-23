// PvZ Portable Rust 翻译 — Plant（植物类）
// 对应 C++ src/Lawn/Plant.h / Plant.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::lawn::board::Board;
use crate::lawn::zombie::Zombie;
use crate::todlib::reanimator::{Reanimation, ReanimLoopType};
use crate::todlib::tod_particle::ParticleSystem;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::rect::Rect;
use crate::framework::color::Color;

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

    /// 初始化植物
    pub fn plant_initialize(&mut self, grid_x: i32, grid_y: i32, seed_type: SeedType, imitater_type: SeedType) {
        self.seed_type = seed_type;
        self.plant_col = grid_x;
        self.base.row = grid_y;
        self.imitater_type = imitater_type;
        self.is_on_board = true;

        // 设置植物的基本属性
        self.plant_health = Plant::get_health_for_type(seed_type);
        self.plant_max_health = self.plant_health;
        self.launch_rate = Plant::get_launch_rate_for_type(seed_type);
        self.frame_length = Plant::get_frame_length_for_type(seed_type);
        self.num_frames = Plant::get_num_frames_for_type(seed_type);

        // 加载动画
        // self.play_body_reanim(...)
    }

    /// 更新植物
    pub fn update(&mut self) {
        if self.dead { return; }

        self.anim_counter += 1;

        // 根据种子类型和状态更新
        match self.seed_type {
            SeedType::Sunflower | SeedType::Twinsunflower => {
                self.update_production_plant();
            },
            SeedType::Peashooter | SeedType::Snowpea | SeedType::Repeater |
            SeedType::Gatlingpea | SeedType::Threepeater | SeedType::Splitpea => {
                self.find_target_and_fire(self.base.row, PlantWeapon::Primary);
            },
            _ => {
                // 其他类型的更新逻辑
                self.update_abilities();
            }
        }
    }

    /// 更新生产类植物（向日葵等）
    pub fn update_production_plant(&mut self) {
        self.launch_counter += 1;
        if self.launch_counter >= 300 { // 每300帧生产一次
            self.launch_counter = 0;
            // 产生阳光
            let bx = self.base.x;
            let by = self.base.y;
            if let Some(board) = self.base.get_board_mut() {
                board.add_coin((bx + 20) as f32, (by - 10) as f32, CoinType::Sun, CoinMotion::FromPlant);
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

    /// 更新射手类植物
    pub fn update_shooter(&mut self) {}

    /// 更新特殊能力（默认空实现）
    pub fn update_abilities(&mut self) {}

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
}

impl Default for Plant {
    fn default() -> Self {
        Plant::new()
    }
}
