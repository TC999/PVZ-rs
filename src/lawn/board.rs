// PvZ Portable Rust 翻译 — Board（游戏棋盘）
// 对应 C++ src/Lawn/Board.h / Board.cpp
//
// Board 是整个游戏的核心：管理植物、僵尸、子弹、硬币等所有实体的
// 更新和绘制。这是 PvZ 游戏逻辑的心脏。

use crate::lawn::game_enums::*;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::plant::Plant;
use crate::lawn::zombie::Zombie;
use crate::lawn::projectile::Projectile;
use crate::lawn::coin::Coin;
use crate::lawn::lawn_mower::LawnMower;
use crate::lawn::grid_item::{GridItem, GridItemType};
use crate::lawn::seed_packet::SeedPacket;
use crate::lawn::challenge::Challenge;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;
use crate::framework::color::Color;
use crate::framework::common;
use crate::todlib::tod_particle::ParticleSystem;
use crate::todlib::reanimator::Reanimation;
use crate::todlib::tod_common::TodSmoothArray;

pub const MAX_GRID_SIZE_X: usize = 9;
pub const MAX_GRID_SIZE_Y: usize = 6;
pub const MAX_PLANTS: usize = 200;
pub const MAX_ZOMBIES: usize = 200;
pub const MAX_PROJECTILES: usize = 200;
pub const MAX_COINS: usize = 200;
pub const MAX_LAWN_MOWERS: usize = 6;
pub const MAX_GRID_ITEMS: usize = 50;
pub const LAST_STAND_FLAGS: i32 = 5;
pub const MAX_ZOMBIES_IN_WAVE: usize = 50;
pub const MAX_ZOMBIE_WAVES: usize = 100;
pub const MAX_RENDER_ITEMS: usize = 2048;
pub const PROGRESS_METER_COUNTER: i32 = 150;

/// 命中结果（对应 C++ HitResult）
pub struct HitResult {
    pub object: Option<usize>,   // 索引而非裸指针
    pub object_type: GameObjectType,
}

/// 排序优先级（对应 C++ RenderItemSortFunc）
#[derive(Clone, Copy)]
pub struct RenderItem {
    pub render_object_type: RenderObjectType,
    pub z_pos: i32,
    pub object_index: usize,  // 对应实体列表中的索引
}

/// 僵尸生成选择器（对应 C++ ZombiePicker）
#[derive(Clone)]
pub struct ZombiePicker {
    pub zombie_count: i32,
    pub zombie_points: i32,
    pub zombie_type_count: [i32; 37],  // NUM_ZOMBIE_TYPES
    pub all_waves_zombie_type_count: [i32; 37],
}

impl ZombiePicker {
    pub fn new() -> Self {
        ZombiePicker {
            zombie_count: 0,
            zombie_points: 0,
            zombie_type_count: [0; 37],
            all_waves_zombie_type_count: [0; 37],
        }
    }

    pub fn init_for_wave(&mut self) {
        self.zombie_count = 0;
        self.zombie_points = 0;
        self.zombie_type_count = [0; 37];
    }

    pub fn init(&mut self) {
        self.init_for_wave();
        self.all_waves_zombie_type_count = [0; 37];
    }
}

/// 棋盘上某格子的植物信息（对应 C++ PlantsOnLawn）
pub struct PlantsOnLawn {
    pub under_plant: Option<usize>,
    pub pumpkin_plant: Option<usize>,
    pub flying_plant: Option<usize>,
    pub normal_plant: Option<usize>,
}

/// Bungee 僵尸掉落网格（对应 C++ BungeeDropGrid）
pub struct BungeeDropGrid {
    pub grid_array: Vec<(i32, i32, i32)>,  // (grid_x, grid_y, weight)
}

/// 将行号转换为 Y 坐标
pub fn row_to_y(row: i32) -> i32 {
    80 + row * 100
}

/// 将 Y 坐标转换为行号
pub fn y_to_row(y: i32) -> i32 {
    ((y - 80) / 100).clamp(0, 5)
}

/// 将 X 坐标转换为列号
pub fn x_to_col(x: i32) -> i32 {
    ((x - 40) / 80).clamp(0, 8)
}

/// 创建渲染排序值（对应 C++ MakeRenderOrder）
pub fn make_render_order(layer: i32, row: i32, layer_offset: i32) -> i32 {
    row * 10000 + layer + layer_offset  // RENDER_LAYER_ROW_OFFSET = 10000
}

/// RenderItem 排序比较函数（对应 C++ RenderItemSortFunc）
pub fn render_item_sort_func(a: &RenderItem, b: &RenderItem) -> std::cmp::Ordering {
    a.z_pos.cmp(&b.z_pos)
}

/// 检查僵尸类型是否能进入水池（对应 C++ Zombie::ZombieTypeCanGoInPool）
pub fn zombie_type_can_go_in_pool(zombie_type: ZombieType) -> bool {
    matches!(zombie_type,
        ZombieType::Normal | ZombieType::TrafficCone | ZombieType::Pail
        | ZombieType::Flag | ZombieType::Snorkel | ZombieType::DolphinRider
        | ZombieType::PeaHead | ZombieType::WallnutHead | ZombieType::JalapenoHead
        | ZombieType::GatlingHead | ZombieType::TallnutHead
    )
}

/// 检查僵尸类型是否能上高地（对应 C++ Zombie::ZombieTypeCanGoOnHighGround）
pub fn zombie_type_can_go_on_high_ground(zombie_type: ZombieType) -> bool {
    zombie_type != ZombieType::Zamboni && zombie_type != ZombieType::Bobsled
}

/// 计算矩形重叠宽度（对应 C++ GetRectOverlap）
pub fn get_rect_overlap(rect1: &Rect, rect2: &Rect) -> i32 {
    let (rmin, rmax, xmax) = if rect1.x < rect2.x {
        (rect1.x + rect1.width, rect2.x + rect2.width, rect2.x)
    } else {
        (rect2.x + rect2.width, rect1.x + rect1.width, rect1.x)
    };

    let rmin = if rmin > xmax && rmin > rmax { rmax } else { rmin };
    rmin - xmax
}

/// 检测圆形与矩形是否重叠（对应 C++ GetCircleRectOverlap）
pub fn get_circle_rect_overlap(circle_x: i32, circle_y: i32, radius: i32, rect: &Rect) -> bool {
    let dx;
    let x_out;
    if circle_x < rect.x {
        x_out = true;
        dx = rect.x - circle_x;
    } else if circle_x > rect.x + rect.width {
        x_out = true;
        dx = circle_x - rect.x - rect.width;
    } else {
        x_out = false;
        dx = 0;
    }

    let dy;
    let y_out;
    if circle_y < rect.y {
        y_out = true;
        dy = rect.y - circle_y;
    } else if circle_y > rect.y + rect.height {
        y_out = true;
        dy = circle_y - rect.y - rect.height;
    } else {
        y_out = false;
        dy = 0;
    }

    if !x_out && !y_out {
        return true;
    }

    dx * dx + dy * dy <= radius * radius
}

/// 全局变量：是否已显示更多阳光教程（对应 C++ gShownMoreSunTutorial）
pub static mut G_SHOWN_MORE_SUN_TUTORIAL: bool = false;

/// 玩家初始化（对应 C++ BoardInitForPlayer）
pub fn board_init_for_player() {
    unsafe { G_SHOWN_MORE_SUN_TUTORIAL = false; }
}

/// 游戏棋盘 — 管理整个关卡的所有游戏元素
/// 对应 C++ Board.h（头文件翻译已完成）和 Board.cpp（方法实现翻译中）
pub struct Board {
    pub app: Option<*mut LawnApp>,

    // 游戏模式与关卡
    pub game_mode: GameMode,
    pub level: i32,
    pub m_width: i32,
    pub m_height: i32,
    pub m_board_result: BoardResult,
    pub m_advice: AdviceType,

    // 网格
    pub grid_plants: [[Option<PlantID>; MAX_GRID_SIZE_X]; MAX_GRID_SIZE_Y],
    pub grid_square_type: [[GridSquareType; MAX_GRID_SIZE_X]; MAX_GRID_SIZE_Y],
    pub grid_cel_look: [[i32; MAX_GRID_SIZE_X]; MAX_GRID_SIZE_Y],
    pub grid_cel_offset: [[[i32; 2]; MAX_GRID_SIZE_X]; MAX_GRID_SIZE_Y],
    pub grid_cel_fog: [[i32; MAX_GRID_SIZE_Y + 1]; MAX_GRID_SIZE_X],

    // 实体列表
    pub plants: Vec<Plant>,
    pub zombies: Vec<Zombie>,
    pub projectiles: Vec<Projectile>,
    pub coins: Vec<Coin>,
    pub lawn_mowers: Vec<LawnMower>,
    pub grid_items: Vec<GridItem>,
    pub seed_bank: Vec<SeedPacket>,

    // UI 元素
    pub menu_button: Option<i32>,   // 简化：暂不引入 GameButton
    pub store_button: Option<i32>,
    pub ignore_mouse_up: bool,
    pub m_paused: bool,

    // 阳光
    pub m_sun_count: i32,
    pub m_sun_countdown: i32,
    pub m_num_suns_fallen: i32,

    // 金钱
    pub m_coin_bank: i64,
    pub m_coin_bank_fade_count: i32,

    // 关卡状态
    pub m_wave_count: i32,
    pub m_current_wave: i32,
    pub m_total_waves: i32,
    pub m_num_waves: i32,
    pub m_total_spawned_waves: i32,
    pub m_out_of_money_counter: i32,
    pub m_wave_countdown: i32,
    pub m_zombie_countdown: i32,
    pub m_flag_countdown: i32,
    pub m_level_complete: bool,
    pub m_game_over: bool,
    pub m_game_over_countdown: i32,
    pub m_board_fade_out_counter: i32,
    pub m_next_survival_stage_counter: i32,
    pub m_score_next_mower_counter: i32,
    pub m_level_award_spawned: bool,
    pub m_progress_meter_width: i32,
    pub m_flag_raise_counter: i32,
    pub m_sod_position: i32,

    // 行状态
    pub m_plant_row: [PlantRowType; MAX_GRID_SIZE_Y],
    pub m_wave_row_got_lawn_mowered: [i32; MAX_GRID_SIZE_Y],
    pub m_row_picking_array: [TodSmoothArray; MAX_GRID_SIZE_Y],

    // 草地
    pub m_background_type: BackgroundType,
    pub m_pool_occupied: [bool; MAX_GRID_SIZE_Y],
    pub m_roof: bool,
    pub m_fog: bool,
    pub m_fog_offset: f32,
    pub m_fog_blown_count_down: i32,
    pub m_enable_grave_stones: bool,
    pub m_special_grave_stone_x: i32,
    pub m_special_grave_stone_y: i32,
    pub m_final_boss_killed: bool,
    pub m_show_shovel: bool,
    pub m_killed_yeti: bool,

    // 挑战/小游戏
    pub challenge: Option<Challenge>,

    // 计时器
    pub m_tutorial_state: TutorialState,
    pub m_update_count: i32,
    pub m_main_counter: u32,
    pub m_effect_counter: u32,
    pub m_draw_count: u32,
    pub m_rise_from_grave_counter: i32,

    // 序列号
    pub m_board_rand_seed: u32,

    // 冻结/减速效果
    pub m_ice_timer: [i32; MAX_GRID_SIZE_Y],
    pub m_ice_min_x: [i32; MAX_GRID_SIZE_Y],
    pub m_ice_particle: i32,

    // 屏幕震动
    pub m_shake_counter: i32,
    pub m_shake_amount_x: i32,
    pub m_shake_amount_y: i32,

    // 鼠标状态
    pub m_prev_mouse_x: i32,
    pub m_prev_mouse_y: i32,

    // 调试
    pub m_debug_text_mode: DebugTextMode,

    // 统计
    pub m_graves_cleared: u32,
    pub m_plants_eaten: u32,
    pub m_plants_shoveled: u32,

    // 模式标志
    pub m_mustache_mode: bool,
    pub m_future_mode: bool,
    pub m_pinata_mode: bool,
    pub m_dance_mode: bool,
    pub m_daisy_mode: bool,
    pub m_sukhbir_mode: bool,
    pub m_super_mower_mode: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            app: None,
            game_mode: GameMode::Adventure,
            level: 1,
            m_width: 800,
            m_height: 600,
            m_board_result: BoardResult::None,
            m_advice: AdviceType::None,
            grid_plants: [[None; MAX_GRID_SIZE_X]; MAX_GRID_SIZE_Y],
            grid_square_type: [[GridSquareType::Grass; MAX_GRID_SIZE_X]; MAX_GRID_SIZE_Y],
            grid_cel_look: [[0; MAX_GRID_SIZE_X]; MAX_GRID_SIZE_Y],
            grid_cel_offset: [[[0; 2]; MAX_GRID_SIZE_X]; MAX_GRID_SIZE_Y],
            grid_cel_fog: [[0; MAX_GRID_SIZE_Y + 1]; MAX_GRID_SIZE_X],
            plants: Vec::with_capacity(MAX_PLANTS),
            zombies: Vec::with_capacity(MAX_ZOMBIES),
            projectiles: Vec::with_capacity(MAX_PROJECTILES),
            coins: Vec::with_capacity(MAX_COINS),
            lawn_mowers: Vec::with_capacity(MAX_LAWN_MOWERS),
            grid_items: Vec::with_capacity(MAX_GRID_ITEMS),
            seed_bank: Vec::new(),
            menu_button: None,
            store_button: None,
            ignore_mouse_up: false,
            m_paused: false,
            m_sun_count: 50,
            m_sun_countdown: SUN_COUNTDOWN,
            m_num_suns_fallen: 0,
            m_coin_bank: 0,
            m_coin_bank_fade_count: 0,
            m_wave_count: 0,
            m_current_wave: 0,
            m_total_waves: 0,
            m_num_waves: 0,
            m_total_spawned_waves: 0,
            m_out_of_money_counter: 0,
            m_wave_countdown: 0,
            m_zombie_countdown: ZOMBIE_COUNTDOWN_FIRST_WAVE,
            m_flag_countdown: 0,
            m_level_complete: false,
            m_game_over: false,
            m_game_over_countdown: 0,
            m_board_fade_out_counter: -1,
            m_next_survival_stage_counter: 0,
            m_score_next_mower_counter: 0,
            m_level_award_spawned: false,
            m_progress_meter_width: 0,
            m_flag_raise_counter: 0,
            m_sod_position: 0,
            m_plant_row: [PlantRowType::Normal; MAX_GRID_SIZE_Y],
            m_wave_row_got_lawn_mowered: [-100; MAX_GRID_SIZE_Y],
            m_row_picking_array: [TodSmoothArray { item: 0, weight: 0.0, last_picked: 0.0, second_last_picked: 0.0 }; MAX_GRID_SIZE_Y],
            m_background_type: BackgroundType::Day,
            m_pool_occupied: [false; MAX_GRID_SIZE_Y],
            m_roof: false,
            m_fog: false,
            m_fog_offset: 0.0,
            m_fog_blown_count_down: 0,
            m_enable_grave_stones: false,
            m_special_grave_stone_x: -1,
            m_special_grave_stone_y: -1,
            m_final_boss_killed: false,
            m_show_shovel: false,
            m_killed_yeti: false,
            challenge: None,
            m_tutorial_state: TutorialState::Off,
            m_update_count: 0,
            m_main_counter: 0,
            m_effect_counter: 0,
            m_draw_count: 0,
            m_rise_from_grave_counter: 0,
            m_board_rand_seed: 0,
            m_ice_timer: [0; MAX_GRID_SIZE_Y],
            m_ice_min_x: [0; MAX_GRID_SIZE_Y],
            m_ice_particle: 0,
            m_shake_counter: 0,
            m_shake_amount_x: 0,
            m_shake_amount_y: 0,
            m_prev_mouse_x: -1,
            m_prev_mouse_y: -1,
            m_debug_text_mode: DebugTextMode::None,
            m_graves_cleared: 0,
            m_plants_eaten: 0,
            m_plants_shoveled: 0,
            m_mustache_mode: false,
            m_future_mode: false,
            m_pinata_mode: false,
            m_dance_mode: false,
            m_daisy_mode: false,
            m_sukhbir_mode: false,
            m_super_mower_mode: false,
        }
    }

    /// 初始化关卡（对应 C++ Board 构造函数和 InitLevel）
    pub fn board_init(&mut self, app: *mut LawnApp) {
        self.app = Some(app);

        // 初始化随机种子
        unsafe {
            let app_ref = &*app;
            self.m_board_rand_seed = app_ref.m_app_rand_seed as u32;
        }
        // 生存模式下使用新的随机种子
        if self.game_mode == GameMode::SurvivalNormalStage1 
            || self.game_mode == GameMode::SurvivalNormalStage2
            || self.game_mode == GameMode::SurvivalNormalStage3
            || self.game_mode == GameMode::SurvivalNormalStage4
            || self.game_mode == GameMode::SurvivalHardStage1
            || self.game_mode == GameMode::SurvivalHardStage2
            || self.game_mode == GameMode::SurvivalHardStage3
            || self.game_mode == GameMode::SurvivalHardStage4
            || self.game_mode == GameMode::SurvivalEndlessStage1
            || self.game_mode == GameMode::SurvivalEndlessStage2
            || self.game_mode == GameMode::SurvivalEndlessStage3
        {
            self.m_board_rand_seed = common::rand() as u32;
        }

        // 网格初始化
        for i in 0..MAX_GRID_SIZE_X {
            for j in 0..MAX_GRID_SIZE_Y {
                self.grid_square_type[i][j] = GridSquareType::Grass;
                self.grid_cel_look[i][j] = common::rand() % 20;
                self.grid_cel_offset[i][j][0] = (common::rand() % 10) - 5;
                self.grid_cel_offset[i][j][1] = (common::rand() % 10) - 5;
            }
            for k in 0..=MAX_GRID_SIZE_Y {
                self.grid_cel_fog[i][k] = 0;
            }
        }

        self.m_fog_offset = 0.0;
        self.m_sun_countdown = 0;
        self.m_shake_counter = 0;
        self.m_shake_amount_x = 0;
        self.m_shake_amount_y = 0;
        self.m_paused = false;
        self.m_level_award_spawned = false;
        self.m_flag_raise_counter = 0;
        self.m_level_complete = false;
        self.m_board_fade_out_counter = -1;
        self.m_next_survival_stage_counter = 0;
        self.m_score_next_mower_counter = 0;
        self.m_progress_meter_width = 0;
        self.m_fog_blown_count_down = 0;
        self.m_enable_grave_stones = false;
        self.m_advice = AdviceType::None;
        self.m_effect_counter = 0;
        self.m_draw_count = 0;
        self.m_rise_from_grave_counter = 0;
        self.m_coin_bank_fade_count = 0;
        self.m_debug_text_mode = DebugTextMode::None;
        self.m_prev_mouse_x = -1;
        self.m_prev_mouse_y = -1;
        self.ignore_mouse_up = false;

        // 设置关卡参数
        self.setup_level();

        // 初始化草地类型
        self.setup_background();

        // 初始化割草机
        for row in 0..6 {
            let mut mower = LawnMower::new();
            mower.lawn_mower_initialize(row);
            self.lawn_mowers.push(mower);
        }

        // 初始化阳光槽
        for i in 0..SEEDBANK_MAX as usize {
            let mut packet = SeedPacket::new();
            packet.seed_packet_initialize(i as i32);
            self.seed_bank.push(packet);
        }

        // 设置波次信息
        self.setup_waves();
    }

    /// 设置关卡（对应 C++ InitLevel 的部分逻辑）
    fn setup_level(&mut self) {
        // 关卡波次初始化简版
        if self.is_first_time_adventure() {
            self.m_total_waves = 1 + (self.level - 1) * 2;
            if self.m_total_waves < 2 { self.m_total_waves = 2; }
        } else {
            self.m_total_waves = 20;
        }
    }

    /// 设置背景
    fn setup_background(&mut self) {
        match self.m_background_type {
            BackgroundType::Day => {},
            BackgroundType::Night => {},
            BackgroundType::Pool => {
                self.m_pool_occupied[2] = true;
                self.m_pool_occupied[3] = true;
            },
            BackgroundType::Fog => {
                self.m_pool_occupied[2] = true;
                self.m_pool_occupied[3] = true;
                self.m_fog = true;
            },
            BackgroundType::Roof => {
                self.m_roof = true;
            },
            _ => {}
        }
    }

    /// 设置波次
    fn setup_waves(&mut self) {
        // 根据关卡设置僵尸波次
    }

    /// 主更新循环
    pub fn update(&mut self) {
        if self.m_paused || self.m_board_result != BoardResult::None { return; }
        self.m_update_count += 1;

        // 更新阳光产生
        self.update_sun_spawning();

        // 更新植物
        for plant in &mut self.plants {
            plant.update();
        }

        // 更新僵尸
        for zombie in &mut self.zombies {
            zombie.update();
        }

        // 更新子弹
        for projectile in &mut self.projectiles {
            projectile.update();
        }

        // 更新硬币
        for coin in &mut self.coins {
            coin.update();
        }

        // 更新割草机
        for mower in &mut self.lawn_mowers {
            mower.update();
        }

        // 碰撞检测
        self.check_collisions();

        // 更新波次
        self.update_waves();

        // 清理已死亡的实体
        self.cleanup_dead();

        // 检查游戏状态
        self.check_game_state();
    }

    /// 更新阳光产生（对应 C++ UpdateSunSpawning）
    fn update_sun_spawning(&mut self) {
        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });

        // 夜间、奖励已掉落、以下模式不自然产生阳光
        if self.stage_is_night()
            || self.has_level_award_dropped()
            || app_mode == GameMode::ChallengeRainingSeeds
            || app_mode == GameMode::ChallengeIceLevel
            // C++ 中还有 GAMEMODE_UPSELL 和 GAMEMODE_INTRO，Rust 枚举中暂缺
            || app_mode == GameMode::ChallengeZombiquarium
            || app_mode == GameMode::ChallengeZenGarden
            || app_mode == GameMode::ChallengeTreeOfWisdom
            || app_mode == GameMode::ChallengeLastStand
            || self.app.map_or(false, |app| unsafe { (*app).is_izombie_level() })
            || self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() })
            || self.app.map_or(false, |app| unsafe { (*app).is_squirrel_level() })
            || self.has_conveyor_belt_seed_bank()
            || self.m_tutorial_state == TutorialState::SlotMachine
        {
            return;
        }

        // 教程第 1 关：如果还没种植物，不产生阳光
        if (self.m_tutorial_state == TutorialState::Level1PickUpPeashooter
            || self.m_tutorial_state == TutorialState::Level1PlantPeashooter)
            && self.plants.is_empty()
        {
            return;
        }

        self.m_sun_countdown -= 1;
        if self.m_sun_countdown != 0 {
            return;
        }

        self.m_num_suns_fallen += 1;
        // 阳光生成间隔逐渐缩短，但不超过最大间隔
        self.m_sun_countdown = std::cmp::min(
            SUN_COUNTDOWN_MAX,
            SUN_COUNTDOWN + self.m_num_suns_fallen * 10
        ) + crate::todlib::tod_common::rand_range_int(0, SUN_COUNTDOWN_RANGE);

        // 晴天关卡产生大阳光，否则普通阳光
        let sun_type = if app_mode == GameMode::ChallengeSunnyDay {
            CoinType::LargeSun
        } else {
            CoinType::Sun
        };
        self.add_coin(
            crate::todlib::tod_common::rand_range_int(100, 649) as f32,
            60.0,
            sun_type,
            CoinMotion::FromSky,
        );
    }

    /// 更新波次
    fn update_waves(&mut self) {
        if self.m_current_wave >= self.m_total_waves { return; }

        self.m_zombie_countdown -= 1;
        if self.m_zombie_countdown <= 0 {
            self.spawn_zombie_wave();
        }
    }

    /// 生成一波僵尸
    fn spawn_zombie_wave(&mut self) {
        self.m_current_wave += 1;

        // 根据关卡和波次生成不同的僵尸
        let count = match self.level {
            1 => 3,
            2..=5 => 3 + self.m_current_wave,
            _ => 5 + self.m_current_wave,
        };

        for _ in 0..count {
            let ztype = if self.level >= 3 && crate::todlib::tod_common::rand_range_int(0, 100) < 30 {
                ZombieType::TrafficCone
            } else {
                ZombieType::Normal
            };

            let row = crate::todlib::tod_common::rand_range_int(0, 5);
            let mut zombie = Zombie::new();
            zombie.zombie_initialize(row, ztype, false, None, self.m_current_wave);
            self.zombies.push(zombie);
        }

        self.m_zombie_countdown = ZOMBIE_COUNTDOWN + crate::todlib::tod_common::rand_range_int(0, ZOMBIE_COUNTDOWN_RANGE);

        // 旗子波次
        if self.m_current_wave % 5 == 0 {
            self.m_flag_countdown = FLAG_RAISE_TIME;
        }
    }

    /// 碰撞检测
    fn check_collisions(&mut self) {
        // 子弹与僵尸碰撞
        let mut projectiles_to_remove = Vec::new();
        for (i, proj) in self.projectiles.iter().enumerate() {
            if proj.dead { continue; }
            let proj_rect = proj.get_projectile_rect();

            for zombie in &mut self.zombies {
                if zombie.dead { continue; }
                if zombie.base.row != proj.base.row { continue; }

                let z_rect = zombie.get_zombie_rect();
                if proj_rect.intersects(&z_rect) {
                    zombie.take_damage(proj.damage, proj.damage_flags);
                    projectiles_to_remove.push(i);
                    break;
                }
            }
        }

        for &i in projectiles_to_remove.iter().rev() {
            if i < self.projectiles.len() {
                self.projectiles[i].dead = true;
            }
        }

        // 僵尸与植物碰撞
        for zombie in &mut self.zombies {
            if zombie.dead || zombie.is_eating { continue; }
            let z_attack = zombie.get_zombie_attack_rect();

            for plant in &mut self.plants {
                if plant.dead || !plant.is_on_board { continue; }
                if plant.base.row != zombie.base.row { continue; }

                let p_rect = plant.plant_rect;
                if z_attack.intersects(&p_rect) {
                    zombie.eat_plant(plant);
                    break;
                }
            }
        }

        // 僵尸吃植物
        let zombie_eat_data: Vec<(usize, i32, i32)> = self.zombies.iter().enumerate()
            .filter(|(_, z)| !z.dead && z.is_eating && z.zombie_age % 4 == 0)
            .map(|(i, z)| (i, z.base.row, z.target_col))
            .collect();
        for (zombie_idx, row, target_col) in zombie_eat_data {
            if let Some(plant) = self.find_plant_at(row, target_col as usize) {
                plant.plant_health -= 20; // EAT_DAMAGE = 20
                if plant.plant_health <= 0 {
                    plant.die();
                    if let Some(zombie) = self.zombies.get_mut(zombie_idx) {
                        zombie.stop_eating();
                    }
                }
            } else {
                if let Some(zombie) = self.zombies.get_mut(zombie_idx) {
                    zombie.stop_eating();
                }
            }
        }

        // 僵尸与割草机碰撞
        for zombie in &mut self.zombies {
            if zombie.dead { continue; }
            let z_rect = zombie.get_zombie_rect();

            for mower in &mut self.lawn_mowers {
                if mower.mowing || mower.dead { continue; }
                if mower.base.row != zombie.base.row { continue; }

                let m_rect = mower.get_mower_rect();
                if z_rect.intersects(&m_rect) {
                    mower.start_mowing();
                    zombie.die_no_loot();
                }
            }
        }
    }

    /// 在指定行列查找植物
    pub fn find_plant_at(&mut self, row: i32, col: usize) -> Option<&mut Plant> {
        if col >= MAX_GRID_SIZE_X { return None; }
        let row_idx = row as usize;
        if row_idx >= MAX_GRID_SIZE_Y { return None; }

        if let Some(pid) = self.grid_plants[row_idx][col] {
            self.plants.iter_mut().find(|p| /* p.id == pid */ false)
        } else {
            None
        }
    }

    /// 在指定行查找僵尸（从指定列开始向左找最近的）
    pub fn find_zombie_in_row(&mut self, _row: i32, _from_col: i32) -> Option<&mut Zombie> {
        // 查找该行最左边的僵尸
        let first_row = self.zombies.first().map(|z| z.base.row).unwrap_or(-1);
        for zombie in &mut self.zombies {
            if zombie.dead { continue; }
            if zombie.base.row == first_row { // 简化：查找第一个
                return Some(zombie);
            }
        }
        None
    }

    /// 添加硬币
    pub fn add_coin(&mut self, x: f32, y: f32, coin_type: CoinType, motion: CoinMotion) {
        let mut coin = Coin::new();
        coin.coin_initialize(x, y, coin_type, motion);
        coin.base.board = Some(self as *mut Board); // 注意：这是 unsafe 的简化
        if let Some(app) = self.app {
            coin.base.app = Some(app);
        }
        self.coins.push(coin);
    }

    /// 添加子弹
    pub fn add_projectile(&mut self, x: f32, y: f32, row: i32, seed_type: SeedType) {
        let mut proj = Projectile::new();
        proj.projectile_initialize(x, y, row, seed_type);
        if let Some(app) = self.app {
            proj.base.app = Some(app);
        }
        proj.base.board = Some(self as *mut Board);
        self.projectiles.push(proj);
    }

    /// 添加植物到棋盘
    pub fn add_plant(&mut self, grid_x: i32, grid_y: i32, seed_type: SeedType, _imitater_type: SeedType) {
        let mut plant = Plant::new();
        plant.plant_initialize(grid_x, grid_y, seed_type, SeedType::None);
        plant.base.board = Some(self as *mut Board);
        if let Some(app) = self.app {
            plant.base.app = Some(app);
        }

        let x = LAWN_XMIN + grid_x * 80 + 10;
        let y = LAWN_YMIN + grid_y * 100 + 10;
        plant.base.x = x;
        plant.base.y = y;
        plant.pos_x = x as f32;
        plant.pos_y = y as f32;

        self.plants.push(plant);
    }

    /// 清理死亡实体
    fn cleanup_dead(&mut self) {
        self.plants.retain(|p| !p.dead);
        self.zombies.retain(|z| !z.dead);
        self.projectiles.retain(|p| !p.dead);
        self.coins.retain(|c| !c.dead);
    }

    /// 在指定位置查找指定类型的格子物品（对应 C++ GetGridItemAt）
    pub fn get_grid_item_at(&self, item_type: GridItemType, grid_x: i32, grid_y: i32) -> Option<&GridItem> {
        self.grid_items.iter().find(|item| {
            item.grid_item_type == item_type && item.grid_x == grid_x && item.grid_y == grid_y
        })
    }

    /// 获取墓碑（对应 C++ GetGraveStoneAt）
    pub fn get_grave_stone_at(&self, grid_x: i32, grid_y: i32) -> Option<&GridItem> {
        self.get_grid_item_at(GridItemType::Grave, grid_x, grid_y)
    }

    /// 获取弹坑（对应 C++ GetCraterAt）
    pub fn get_crater_at(&self, grid_x: i32, grid_y: i32) -> Option<&GridItem> {
        self.get_grid_item_at(GridItemType::Crater, grid_x, grid_y)
    }

    /// 获取梯子（对应 C++ GetLadderAt）
    pub fn get_ladder_at(&self, grid_x: i32, grid_y: i32) -> Option<&GridItem> {
        self.get_grid_item_at(GridItemType::Ladder, grid_x, grid_y)
    }

    /// 获取耙子（对应 C++ GetRake）
    pub fn get_rake(&self) -> Option<&GridItem> {
        self.grid_items.iter().find(|item| item.grid_item_type == GridItemType::Rake)
    }

    /// 获取耙子（可变引用）
    pub fn get_rake_mut(&mut self) -> Option<&mut GridItem> {
        self.grid_items.iter_mut().find(|item| item.grid_item_type == GridItemType::Rake)
    }

    /// 检查能否在该位置添加墓碑（对应 C++ CanAddGraveStoneAt）
    pub fn can_add_grave_stone_at(&self, grid_x: i32, grid_y: i32) -> bool {
        let gx = grid_x as usize;
        let gy = grid_y as usize;
        if gx >= MAX_GRID_SIZE_X || gy >= MAX_GRID_SIZE_Y {
            return false;
        }
        if self.grid_square_type[gx][gy] != GridSquareType::Grass 
            && self.grid_square_type[gx][gy] != GridSquareType::HighGround {
            return false;
        }
        !self.grid_items.iter().any(|item| {
            item.grid_x == grid_x && item.grid_y == grid_y && 
            (item.grid_item_type == GridItemType::Grave 
             || item.grid_item_type == GridItemType::Crater 
             || item.grid_item_type == GridItemType::Ladder)
        })
    }

    /// 检查屏幕上是否有活着的敌人僵尸（对应 C++ AreEnemyZombiesOnScreen）
    pub fn are_enemy_zombies_on_screen(&self) -> bool {
        self.zombies.iter().any(|z| !z.dead && !z.mind_controlled)
    }

    /// 统计屏幕上存活的僵尸数量（对应 C++ CountZombiesOnScreen）
    pub fn count_zombies_on_screen(&self) -> i32 {
        self.zombies.iter().filter(|z| !z.dead && !z.mind_controlled).count() as i32
    }

    /// 统计未触发的割草机数量（对应 C++ CountUntriggerLawnMowers）
    pub fn count_untrigger_lawn_mowers(&self) -> i32 {
        self.lawn_mowers.iter().filter(|m| !m.mowing && !m.dead).count() as i32
    }

    /// 获取每面旗帜的波次数（对应 C++ GetNumWavesPerFlag）
    pub fn get_num_waves_per_flag(&self) -> i32 {
        if self.is_first_time_adventure() && self.m_num_waves < 10 {
            self.m_num_waves
        } else {
            10
        }
    }

    /// 判断是否是旗子波次（对应 C++ IsFlagWave）
    pub fn is_flag_wave(&self, wave_number: i32) -> bool {
        if self.is_first_time_adventure() && self.level == 1 {
            return false;
        }
        let waves_per_flag = self.get_num_waves_per_flag();
        wave_number % waves_per_flag == waves_per_flag - 1
    }

    /// 判断是否是首次冒险模式
    fn is_first_time_adventure(&self) -> bool {
        unsafe {
            self.app.map_or(false, |app| (*app).is_first_time_adventure_mode())
        }
    }

    /// 僵尸类型是否只能在泳池出现（对应 C++ IsZombieTypePoolOnly，静态方法）
    pub fn is_zombie_type_pool_only(zombie_type: ZombieType) -> bool {
        zombie_type == ZombieType::Snorkel || zombie_type == ZombieType::DolphinRider
    }

    /// 僵尸类型是否只通过特殊方式生成（对应 C++ IsZombieTypeSpawnedOnly，静态方法）
    pub fn is_zombie_type_spawned_only(zombie_type: ZombieType) -> bool {
        zombie_type == ZombieType::BackupDancer 
            || zombie_type == ZombieType::Bobsled 
            || zombie_type == ZombieType::Imp
    }

    /// 检查某行特定僵尸类型是否可以生成（对应 C++ RowCanHaveZombieType）
    pub fn row_can_have_zombie_type(&self, row: i32, zombie_type: ZombieType) -> bool {
        // 1. 行必须有僵尸
        if !self.row_can_have_zombies(row) {
            return false;
        }

        // 2. 无草皮之地关卡，无草皮行在前 5 波不刷出僵尸
        // 注意：C++ 中有 GAMEMODE_CHALLENGE_RESODDED，Rust 枚举中暂无对应变体
        // 暂时跳过此检查，后续可添加

        // 获取 app 的游戏模式
        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });

        // 3. 水路不允许不能进水的僵尸（气球僵尸除外）
        if self.m_plant_row[row as usize] == PlantRowType::Pool
            && !zombie_type_can_go_in_pool(zombie_type)
            && zombie_type != ZombieType::Balloon
        {
            return false;
        }

        // 4. 高地不允许不能上高地的僵尸
        if self.m_plant_row[row as usize] == PlantRowType::HighGround
            && !zombie_type_can_go_on_high_ground(zombie_type)
        {
            return false;
        }

        // 5. 计算当前波次（含生存模式调整）
        let mut current_wave = self.m_current_wave;
        if app_mode == GameMode::ChallengeLastStand {
            if let Some(ref challenge) = self.challenge {
                current_wave += challenge.survival_stage * self.get_num_waves_per_survival_stage();
            }
        }

        // 6. 水路限制：前 5 波只出潜水/海豚僵尸
        if self.m_plant_row[row as usize] == PlantRowType::Pool {
            if current_wave < 5 && !Self::is_zombie_type_pool_only(zombie_type) {
                return false;
            }
        } else if Self::is_zombie_type_pool_only(zombie_type) {
            return false;
        }

        // 7. 雪橇僵尸需要冰道
        if zombie_type == ZombieType::Bobsled && self.m_ice_timer[row as usize] == 0 {
            return false;
        }

        // 8. 第一路不出巨人（生存模式除外）
        let is_survival = self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() });
        if row == 0 && !is_survival {
            if zombie_type == ZombieType::Gargantuar || zombie_type == ZombieType::RedeEyeGargantuar {
                return false;
            }
        }

        // 9. 非舞王僵尸或当前为泳池关卡，则可以刷出该僵尸
        if zombie_type != ZombieType::Dancer || self.stage_has_pool() {
            return true;
        }

        // 10. 舞王僵尸在非泳池关卡中，为保证能召唤伴舞僵尸，仅在中间三行刷出
        self.row_can_have_zombies(row - 1) && self.row_can_have_zombies(row + 1)
    }

    /// 检查游戏状态（胜利/失败）
    fn check_game_state(&mut self) {
        if self.m_game_over { return; }
        if self.m_level_complete { return; }

        // 检查是否有僵尸到达房子
        for zombie in &self.zombies {
            if !zombie.dead && zombie.pos_x < -50.0 {
                self.game_over();
                return;
            }
        }

        // 检查是否所有波次已完成且无僵尸
        if self.m_current_wave >= self.m_total_waves && self.zombies.is_empty() {
            self.level_complete();
        }
    }

    /// 游戏结束
    pub fn game_over(&mut self) {
        self.m_game_over = true;
        self.m_board_result = BoardResult::Lost;
        self.m_game_over_countdown = 200;
    }

    /// 关卡完成
    pub fn level_complete(&mut self) {
        self.m_level_complete = true;
        self.m_board_result = BoardResult::Won;
    }

    /// 主绘制函数
    pub fn draw(&self, g: &mut Graphics) {
        // 绘制背景
        self.draw_background(g);

        // 绘制格子物品
        for item in &self.grid_items {
            item.draw(g);
        }

        // 绘制植物
        for plant in &self.plants {
            plant.draw(g);
        }

        // 绘制僵尸
        for zombie in &self.zombies {
            zombie.draw(g);
        }

        // 绘制子弹
        for projectile in &self.projectiles {
            projectile.draw(g);
        }

        // 绘制硬币
        for coin in &self.coins {
            coin.draw(g);
        }

        // 绘制割草机
        for mower in &self.lawn_mowers {
            mower.draw(g);
        }

        // 绘制 UI（阳光计数、种子槽等）
        self.draw_ui(g);
    }

    /// 绘制背景
    fn draw_background(&self, _g: &Graphics) {
        // 根据背景类型绘制草地/夜晚/水池/屋顶等
    }

    /// 绘制 UI
    fn draw_ui(&self, _g: &Graphics) {
        // 绘制阳光数量、种子选择器等
    }

    /// 点击事件
    pub fn mouse_down(&mut self, x: i32, y: i32, _click_count: i32) {
        // 检查是否点击了种子槽
        for packet in &self.seed_bank {
            // 点击种子槽开始种植
        }

        // 检查是否点击了阳光/硬币
        for coin in &mut self.coins {
            if coin.is_sun && !coin.dead {
                let coin_rect = Rect::new(
                    coin.pos_x as i32 - 15,
                    coin.pos_y as i32 - 15,
                    30,
                    30,
                );
                if coin_rect.contains(x, y) {
                    coin.collect();
                    if coin.coin_type == CoinType::Sun {
                        self.m_sun_count += coin.value;
                    }
                    break;
                }
            }
        }
    }

    /// 键盘事件
    pub fn key_down(&mut self, key: i32) {
        match key {
            // 暂停
            80 => { // 'P'
                self.m_paused = !self.m_paused;
            },
            _ => {}
        }
    }

    // ========== 关卡特性检查 ==========

    /// 是否为夜晚关卡
    pub fn stage_is_night(&self) -> bool {
        self.m_background_type == BackgroundType::Night
    }

    /// 是否有水池
    pub fn stage_has_pool(&self) -> bool {
        self.m_background_type == BackgroundType::Pool 
            || self.m_background_type == BackgroundType::Fog
    }

    /// 是否有雾
    pub fn stage_has_fog(&self) -> bool {
        self.m_background_type == BackgroundType::Fog
    }

    /// 是否有屋顶
    pub fn stage_has_roof(&self) -> bool {
        self.m_background_type == BackgroundType::Roof
    }

    /// 是否为白天且无水池
    pub fn stage_is_day_without_pool(&self) -> bool {
        self.m_background_type == BackgroundType::Day
    }

    /// 是否有墓碑
    pub fn stage_has_grave_stones(&self) -> bool {
        if self.m_background_type == BackgroundType::Night {
            return self.level >= 2 && self.level != 10;
        }
        false
    }

    // ========== UI 交互 ==========

    /// 清除提示信息（对应 C++ ClearAdvice）
    pub fn clear_advice(&mut self, _help_index: AdviceType) {
        // TODO: 实现清除消息控件上的提示文字
        self.m_advice = AdviceType::None;
    }

    /// 是否可与界面按钮交互（对应 C++ CanInteractWithBoardButtons）
    pub fn can_interact_with_board_buttons(&self) -> bool {
        if self.m_board_fade_out_counter > 0 { return false; }
        if self.m_board_result != BoardResult::None { return false; }
        if self.m_level_complete { return false; }
        true
    }

    /// 显示金币银行（对应 C++ ShowCoinBank）
    pub fn show_coin_bank(&mut self, _duration: i32) {
        self.m_coin_bank_fade_count = _duration;
    }

    /// 检查某格是否为水池（对应 C++ IsPoolSquare）
    pub fn is_pool_square(&self, grid_x: i32, grid_y: i32) -> bool {
        if grid_x >= 0 && grid_y >= 0 && grid_x < MAX_GRID_SIZE_X as i32 && grid_y < MAX_GRID_SIZE_Y as i32 {
            self.grid_square_type[grid_x as usize][grid_y as usize] == GridSquareType::Pool
        } else {
            false
        }
    }

    /// 像素坐标 → 网格 X（对应 C++ PixelToGridX）
    pub fn pixel_to_grid_x(&self, x: i32, _y: i32) -> i32 {
        if x < LAWN_XMIN { return -1; }
        ((x - LAWN_XMIN) / 80).clamp(0, MAX_GRID_SIZE_X as i32 - 1)
    }

    /// 像素坐标 → 网格 Y（对应 C++ PixelToGridY）
    pub fn pixel_to_grid_y(&self, x: i32, y: i32) -> i32 {
        let grid_x = self.pixel_to_grid_x(x, y);
        if grid_x == -1 || y < LAWN_YMIN { return -1; }

        if self.stage_has_roof() {
            let adjusted_y = if grid_x < 5 { y - (4 - grid_x) * 20 } else { y };
            ((adjusted_y - LAWN_YMIN) / 85).clamp(0, MAX_GRID_SIZE_Y as i32 - 2)
        } else if self.stage_has_pool() {
            ((y - LAWN_YMIN) / 85).clamp(0, MAX_GRID_SIZE_Y as i32 - 1)
        } else {
            ((y - LAWN_YMIN) / 100).clamp(0, MAX_GRID_SIZE_Y as i32 - 2)
        }
    }

    /// 网格 X → 像素坐标（对应 C++ GridToPixelX）
    pub fn grid_to_pixel_x(&self, grid_x: i32, _grid_y: i32) -> i32 {
        debug_assert!(grid_x >= 0 && grid_x < MAX_GRID_SIZE_X as i32);
        debug_assert!(_grid_y >= 0 && _grid_y < MAX_GRID_SIZE_Y as i32);
        grid_x * 80 + LAWN_XMIN
    }

    /// 统计某种植物的数量（对应 C++ CountPlantByType）
    pub fn count_plant_by_type(&self, seed_type: SeedType) -> i32 {
        self.plants.iter().filter(|p| p.seed_type == seed_type && !p.dead).count() as i32
    }

    /// 统计某种僵尸的数量（对应 C++ CountZombieByType）
    pub fn count_zombie_by_type(&self, zombie_type: ZombieType) -> i32 {
        self.zombies.iter().filter(|z| z.zombie_type == zombie_type && !z.dead).count() as i32
    }

    /// 是否已掉落关卡奖励（对应 C++ HasLevelAwardDropped）
    pub fn has_level_award_dropped(&self) -> bool {
        self.m_level_award_spawned || self.m_next_survival_stage_counter > 0 || self.m_board_fade_out_counter >= 0
    }

    /// 是否有进度条（对应 C++ HasProgressMeter）
    pub fn has_progress_meter(&self) -> bool {
        if matches!(self.game_mode, 
            GameMode::ChallengeBeghouled 
            | GameMode::ChallengeBeghouledTwist
        ) {
            return true;
        }
        // m_progress_meter_width == 0 时无进度条（某些模式或关卡）
        self.m_progress_meter_width != 0
    }

    /// 获取关卡随机种子（对应 C++ GetLevelRandSeed）
    pub fn get_level_rand_seed(&self) -> i32 {
        self.m_board_rand_seed as i32
    }

    /// 获取种子银行中的卡槽数量（对应 C++ GetNumSeedsInBank）
    pub fn get_num_seeds_in_bank(&self) -> i32 {
        if self.is_first_time_adventure() && self.level < 5 { 3 } else { 6 }
    }

    /// 当前关卡是否需要选择种子（对应 C++ ChooseSeedsOnCurrentLevel）
    pub fn choose_seeds_on_current_level(&self) -> bool {
        !self.is_first_time_adventure() || self.level > 7
    }

    /// 某行是否可以生成僵尸（对应 C++ RowCanHaveZombies）
    pub fn row_can_have_zombies(&self, row: i32) -> bool {
        if row < 0 || row >= MAX_GRID_SIZE_Y as i32 { return false; }
        self.m_plant_row[row as usize] != PlantRowType::Dirt
    }

    /// 重置 FPS 统计（对应 C++ ResetFPSStats）
    pub fn reset_fps_stats(&mut self) {}

    /// 获取墓碑数量（对应 C++ GetGraveStoneCount）
    pub fn get_grave_stone_count(&self) -> i32 {
        self.grid_items.iter().filter(|item| item.grid_item_type == GridItemType::Grave).count() as i32
    }

    /// 统计某种硬币的数量（对应 C++ CountCoinByType）
    pub fn count_coin_by_type(&self, coin_type: CoinType) -> i32 {
        self.coins.iter().filter(|c| c.coin_type == coin_type && !c.dead).count() as i32
    }

    /// 获取获胜僵尸（对应 C++ GetWinningZombie）
    pub fn get_winning_zombie(&self) -> Option<&Zombie> {
        None
    }

    /// 关卡是否有 6 行（对应 C++ StageHas6Rows）
    pub fn stage_has_6_rows(&self) -> bool {
        self.stage_has_pool() || self.stage_has_fog()
    }

    /// 是否需要保存游戏（对应 C++ NeedSaveGame）
    pub fn need_save_game(&self) -> bool {
        if self.m_board_fade_out_counter > 0 { return true; }
        if self.m_level_complete { return false; }
        if self.m_game_over { return false; }
        if self.m_board_result != BoardResult::None { return false; }
        true
    }

    /// 是否可以掉落战利品（对应 C++ CanDropLoot）
    pub fn can_drop_loot(&self) -> bool {
        !self.is_first_time_adventure() || self.level >= 11
    }

    // ========== 实体创建 ==========

    /// 添加一个梯子（对应 C++ AddALadder）
    pub fn add_ladder(&mut self, grid_x: i32, grid_y: i32) {
        let mut ladder = GridItem::new();
        ladder.grid_item_type = GridItemType::Ladder;
        ladder.grid_item_initialize(GridItemType::Ladder, grid_x, grid_y);
        self.grid_items.push(ladder);
    }

    /// 添加一个弹坑（对应 C++ AddACrater）
    pub fn add_crater(&mut self, grid_x: i32, grid_y: i32) {
        let mut crater = GridItem::new();
        crater.grid_item_initialize(GridItemType::Crater, grid_x, grid_y);
        self.grid_items.push(crater);
    }

    /// 添加一个墓碑（对应 C++ AddAGraveStone）
    pub fn add_grave_stone(&mut self, grid_x: i32, grid_y: i32) {
        let mut grave = GridItem::new();
        grave.grid_item_initialize(GridItemType::Grave, grid_x, grid_y);
        self.grid_items.push(grave);
    }

    // ========== 显示/提示 ==========

    /// 立即清除提示（对应 C++ ClearAdviceImmediately）
    pub fn clear_advice_immediately(&mut self) {
        self.m_advice = AdviceType::None;
    }

    /// 显示提示（对应 C++ DisplayAdvice）
    pub fn display_advice(&mut self, _advice: &str, _style: i32, help_index: AdviceType) {
        self.m_advice = help_index;
    }

    /// 再次显示提示（对应 C++ DisplayAdviceAgain）
    pub fn display_advice_again(&mut self, _advice: &str, _style: i32, help_index: AdviceType) {
        self.m_advice = help_index;
    }

    // ========== 植物/种子 ==========

    /// 获取种子槽 X 坐标（对应 C++ GetSeedPacketPositionX）
    pub fn get_seed_packet_position_x(&self, index: i32) -> i32 {
        80 + index * 70
    }

    /// 统计向日葵数量（对应 C++ CountSunFlowers）
    pub fn count_sunflowers(&self) -> i32 {
        self.count_plant_by_type(SeedType::Sunflower) 
            + self.count_plant_by_type(SeedType::Twinsunflower)
    }

    /// 光标中是否有植物（对应 C++ IsPlantInCursor）
    pub fn is_plant_in_cursor(&self) -> bool {
        // 简化版：通过游戏逻辑判断
        false
    }

    // ========== 屏幕震动 ==========

    /// 震动屏幕（对应 C++ ShakeBoard）
    pub fn shake_board(&mut self, amount_x: i32, amount_y: i32) {
        self.m_shake_counter = 12;
        self.m_shake_amount_x = amount_x;
        self.m_shake_amount_y = amount_y;
    }

    // ========== 关卡检查 ==========

    /// 是否为白天且带水池的关卡
    pub fn stage_is_day_with_pool(&self) -> bool {
        self.m_background_type == BackgroundType::Pool
    }

    /// 僵尸是否从右侧走进来（对应 C++ StageHasZombieWalkInFromRight）
    pub fn stage_has_zombie_walk_in_from_right(&self) -> bool {
        !self.stage_has_roof() && !self.stage_has_pool()
    }

    /// 获取行选择数组中某项的值（对应 C++ 行选择相关）
    pub fn get_row_picking_item(&self, row: usize) -> i32 {
        row as i32
    }

    /// 检查僵尸能否在此行使用特定类型（对应 C++ RowCanHaveZombieType 简化版）
    pub fn is_zombie_allowed(&self, _zombie_type: ZombieType) -> bool {
        true
    }

    // ========== 关卡效果 ==========

    /// 检查某行是否有冰冻效果（对应 C++ IsIceAt）
    pub fn is_ice_at(&self, grid_x: i32, grid_y: i32) -> bool {
        let gy = grid_y as usize;
        if gy >= MAX_GRID_SIZE_Y { return false; }
        self.m_ice_timer[gy] > 0 && self.m_ice_min_x[gy] <= grid_x * 80 + LAWN_XMIN
    }

    /// 检查是否可以添加雪橇（对应 C++ CanAddBobSled）
    pub fn can_add_bob_sled(&self) -> bool {
        (0..MAX_GRID_SIZE_Y).any(|row| self.m_ice_timer[row] > 0 && self.m_ice_min_x[row] < 700)
    }

    /// 检查当前正在被收集的阳光数量
    pub fn count_sun_being_collected(&self) -> i32 {
        self.coins.iter().filter(|c| {
            c.coin_type == CoinType::Sun && !c.dead && c.lifetime > 0 && c.lifetime < 600
        }).count() as i32
    }

    /// 检查是否可以使用游戏对象
    pub fn can_use_game_object(&self, _object_type: GameObjectType) -> bool {
        true
    }

    /// 检查僵尸类型能否在某一关卡生成（对应 C++ CanZombieSpawnOnLevel）
    pub fn can_zombie_spawn_on_level(_zombie_type: ZombieType, _level: i32) -> bool {
        true
    }

    /// 获取本关新出现的僵尸类型（对应 C++ GetIntroducedZombieType）
    pub fn get_introduced_zombie_type(&self) -> ZombieType {
        let is_adventure = self.app.map_or(false, |app| unsafe { (*app).is_adventure_mode() });
        if !is_adventure || self.level == 1 {
            return ZombieType::Invalid;
        }

        // 遍历所有非 Invalid 的僵尸类型（Normal=0 到 RedeEyeGargantuar=33）
        for ztype_int in 0..NUM_ZOMBIE_TYPES {
            let zombie_type: ZombieType = unsafe { std::mem::transmute(ztype_int) };
            let zombie_def = crate::lawn::zombie::get_zombie_definition(zombie_type);
            let can_spawn = zombie_type != ZombieType::Yeti
                || self.app.map_or(false, |app| unsafe { (*app).can_spawn_yetis() });
            if can_spawn && zombie_def.starting_level == self.level {
                return zombie_type;
            }
        }
        ZombieType::Invalid
    }

    /// 选择从墓碑中升起的僵尸类型（对应 C++ PickGraveRisingZombieType）
    pub fn pick_grave_rising_zombie_type(&self) -> ZombieType {
        use crate::todlib::tod_common::{TodWeightedArray, tod_pick_from_weighted_array};

        // 最多使用 3 个条目（普通、路障、铁桶）
        let mut arr = [
            TodWeightedArray { item: ZombieType::Normal as usize, weight: 0 },
            TodWeightedArray { item: ZombieType::TrafficCone as usize, weight: 0 },
            TodWeightedArray { item: ZombieType::Pail as usize, weight: 0 },
        ];
        let mut count = 2;

        arr[0].weight = crate::lawn::zombie::get_zombie_definition(ZombieType::Normal).pick_weight;
        arr[1].weight = crate::lawn::zombie::get_zombie_definition(ZombieType::TrafficCone).pick_weight;
        if !self.stage_has_grave_stones() {
            arr[2].weight = crate::lawn::zombie::get_zombie_definition(ZombieType::Pail).pick_weight;
            count = 3;
        }

        for i in 0..count {
            let ztype_int = arr[i].item;
            let zombie_type: ZombieType = unsafe { std::mem::transmute(ztype_int as i32) };
            let def = crate::lawn::zombie::get_zombie_definition(zombie_type);
            let is_first_time = self.app.map_or(false, |app| unsafe { (*app).is_first_time_adventure_mode() });
            if is_first_time && self.level < def.starting_level {
                arr[i].weight = 0;
            }
            // C++ 中还有 !mZombieAllowed[aZombieType] 检查，但 Rust 端暂缺该字段
        }

        let idx = tod_pick_from_weighted_array(&arr[..count]);
        if idx >= 0 {
            unsafe { std::mem::transmute::<i32, ZombieType>(idx as i32) }
        } else {
            ZombieType::Normal
        }
    }

    /// 获取当前波次应生成的僵尸点数
    pub fn get_zombie_points_for_wave(&self, wave: i32) -> i32 {
        if self.is_first_time_adventure() { wave / 3 + 1 } else { wave * 2 / 5 + 1 }
    }

    // ========== 关卡初始化（InitLevel 简化版） ==========

    /// 初始化关卡（对应 C++ InitLevel）
    pub fn init_level(&mut self) {
        self.m_main_counter = 0;
        self.m_enable_grave_stones = false;
        self.m_sod_position = 0;

        // 设定关卡背景
        self.setup_background();
        // 设定关卡出怪（简化版）
        self.setup_waves();
        // 设定关卡初始阳光
        self.m_sun_count = 50;
    }

    // ========== 僵尸管理 ==========

    /// 移除所有僵尸（对应 C++ RemoveAllZombies）
    pub fn remove_all_zombies(&mut self) {
        for zombie in &mut self.zombies {
            if !zombie.dead {
                zombie.die_no_loot();
            }
        }
    }

    /// 移除开场动画僵尸（对应 C++ RemoveCutsceneZombies）
    pub fn remove_cutscene_zombies(&mut self) {
        // 简化版：移除所有僵尸
        self.remove_all_zombies();
    }

    /// 重选种子时移除僵尸（对应 C++ RemoveZombiesForRepick）
    pub fn remove_zombies_for_repick(&mut self) {
        // 简化版
        self.remove_all_zombies();
    }

    /// 检查是否可以种植（对应 C++ CanPlantAt 简化版）
    pub fn can_plant_at(&self, grid_x: i32, grid_y: i32, seed_type: SeedType) -> PlantingReason {
        if grid_x < 0 || grid_x >= MAX_GRID_SIZE_X as i32 || grid_y < 0 || grid_y >= MAX_GRID_SIZE_Y as i32 {
            return PlantingReason::NotHere;
        }
        // 检查网格类型
        let square = self.grid_square_type[grid_x as usize][grid_y as usize];
        if square == GridSquareType::Dirt || square == GridSquareType::None {
            return PlantingReason::NotHere;
        }
        // 检查墓碑
        if self.get_grave_stone_at(grid_x, grid_y).is_some() {
            return PlantingReason::NotOnGrave;
        }
        // 检查弹坑
        if self.get_crater_at(grid_x, grid_y).is_some() {
            return PlantingReason::NotOnCrater;
        }
        // 水生植物需在水池
        if seed_type == SeedType::Lilypad || seed_type == SeedType::Tanglekelp {
            if !self.is_pool_square(grid_x, grid_y) {
                return PlantingReason::OnlyInPool;
            }
        }
        PlantingReason::Ok
    }

    // ========== 鼠标事件 ==========

    /// 鼠标移动（对应 C++ MouseMove）
    pub fn mouse_move(&mut self, x: i32, y: i32) {
        self.m_prev_mouse_x = x;
        self.m_prev_mouse_y = y;
    }

    /// 鼠标拖拽（对应 C++ MouseDrag）
    pub fn mouse_drag(&mut self, x: i32, y: i32) {
        self.m_prev_mouse_x = x;
        self.m_prev_mouse_y = y;
    }

    // ========== 阳光/金钱 ==========

    /// 增加阳光/金钱（对应 C++ AddSunMoney）
    pub fn add_sun_money(&mut self, amount: i32) {
        self.m_sun_count += amount;
    }

    /// 消耗阳光/金钱（对应 C++ TakeSunMoney）
    pub fn take_sun_money(&mut self, amount: i32) -> bool {
        if self.m_sun_count >= amount {
            self.m_sun_count -= amount;
            true
        } else {
            false
        }
    }

    /// 是否可以消耗阳光/金钱（对应 C++ CanTakeSunMoney）
    pub fn can_take_sun_money(&self, amount: i32) -> bool {
        self.m_sun_count >= amount
    }

    /// 暂停/恢复（对应 C++ Pause）
    pub fn pause(&mut self, pause: bool) {
        self.m_paused = pause;
    }

    // ========== 统计 ==========

    /// 统计未触发的割草机数量（对应 C++ CountUntriggerLawnMowers）
    pub fn count_lawn_mowers_remaining(&self) -> i32 {
        self.lawn_mowers.iter().filter(|m| !m.dead && !m.mowing).count() as i32
    }

    /// 统计正在被收集的硬币数量（对应 C++ CountCoinsBeingCollected）
    pub fn count_coins_being_collected(&self) -> i32 {
        self.coins.iter().filter(|c| !c.dead && c.lifetime > 0 && c.lifetime < 600).count() as i32
    }

    // ========== 模式设置 ==========

    /// 设置胡子模式（对应 C++ SetMustacheMode）
    pub fn set_mustache_mode(&mut self, enable: bool) {
        self.m_mustache_mode = enable;
    }

    /// 设置未来模式（对应 C++ SetFutureMode）
    pub fn set_future_mode(&mut self, enable: bool) {
        self.m_future_mode = enable;
    }

    /// 设置彩带模式（对应 C++ SetPinataMode）
    pub fn set_pinata_mode(&mut self, enable: bool) {
        self.m_pinata_mode = enable;
    }

    /// 设置舞蹈模式（对应 C++ SetDanceMode）
    pub fn set_dance_mode(&mut self, enable: bool) {
        self.m_dance_mode = enable;
    }

    /// 设置雏菊模式（对应 C++ SetDaisyMode）
    pub fn set_daisy_mode(&mut self, enable: bool) {
        self.m_daisy_mode = enable;
    }

    /// 设置苏赫比尔模式（对应 C++ SetSukhbirMode）
    pub fn set_sukhbir_mode(&mut self, enable: bool) {
        self.m_sukhbir_mode = enable;
    }

    /// 设置超级割草机模式（对应 C++ SetSuperMowerMode）
    pub fn set_super_mower_mode(&mut self, enable: bool) {
        self.m_super_mower_mode = enable;
    }

    /// 检查行是否已触发射箭（割草机）
    pub fn is_row_lawn_mowered(&self, row: usize) -> bool {
        if row >= MAX_GRID_SIZE_Y { return false; }
        self.m_wave_row_got_lawn_mowered[row] >= -1
    }

    /// 获取种子银行额外宽度（对应 C++ GetSeedBankExtraWidth）
    pub fn get_seed_bank_extra_width(&self) -> i32 {
        let n = self.seed_bank.len() as i32;
        if n <= 6 { 0 } else if n == 7 { 60 } else if n == 8 { 76 } else if n == 9 { 112 } else { 153 }
    }

    /// 获取迷雾左边界列号（对应 C++ LeftFogColumn）
    pub fn left_fog_column(&self) -> i32 {
        5
    }

    /// 调整种植 Y 坐标（对应 C++ OffsetYForPlanting）
    pub fn offset_y_for_planting(&self, y: &mut i32, seed_type: SeedType) {
        if Plant::is_flying(seed_type) || seed_type == SeedType::Gravebuster {
            *y += 15;
        }
        if seed_type == SeedType::Spikeweed || seed_type == SeedType::Spikerock {
            *y -= 15;
        }
    }

    /// 种植时像素到网格 X（对应 C++ PlantingPixelToGridX）
    pub fn planting_pixel_to_grid_x(&self, x: i32, y: i32, seed_type: SeedType) -> i32 {
        let mut adjusted_y = y;
        self.offset_y_for_planting(&mut adjusted_y, seed_type);
        self.pixel_to_grid_x(x, adjusted_y)
    }

    /// 种植时像素到网格 Y（对应 C++ PlantingPixelToGridY）
    pub fn planting_pixel_to_grid_y(&self, x: i32, y: i32, seed_type: SeedType) -> i32 {
        let mut adjusted_y = y;
        self.offset_y_for_planting(&mut adjusted_y, seed_type);
        self.pixel_to_grid_y(x, adjusted_y)
    }

    /// 种植需求检查（对应 C++ PlantingRequirementsMet）
    pub fn planting_requirements_met(&self, seed_type: SeedType) -> bool {
        match seed_type {
            SeedType::Gatlingpea => self.count_plant_by_type(SeedType::Repeater) > 0,
            SeedType::Twinsunflower => self.count_plant_by_type(SeedType::Sunflower) > 0,
            SeedType::Gloomshroom => self.count_plant_by_type(SeedType::Fumeshroom) > 0,
            SeedType::Wintermelon => self.count_plant_by_type(SeedType::Melonpult) > 0,
            SeedType::GoldMagnet => self.count_plant_by_type(SeedType::Magnetshroom) > 0,
            SeedType::Spikerock => self.count_plant_by_type(SeedType::Spikeweed) > 0,
            _ => true,
        }
    }

    /// 获取每生存关卡的波次数（对应 C++ GetNumWavesPerSurvivalStage）
    pub fn get_num_waves_per_survival_stage(&self) -> i32 {
        let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        if game_mode == GameMode::ChallengeLastStand
            || self.app.map_or(false, |app| unsafe { (*app).is_survival_normal(game_mode) })
        {
            return 10;
        }
        if self.app.map_or(false, |app| unsafe { (*app).is_survival_hard(game_mode) })
            || self.app.map_or(false, |app| unsafe { (*app).is_survival_endless(game_mode) })
        {
            return 20;
        }

        debug_assert!(false, "GetNumWavesPerSurvivalStage: unexpected game mode");
        10
    }

    /// 植物是否使用加速定价（对应 C++ PlantUsesAcceleratedPricing）
    pub fn plant_uses_accelerated_pricing(&self, seed_type: SeedType) -> bool {
        Plant::is_upgrade(seed_type) // 简化为：生存无尽模式下涨价
    }

    /// 获取植物的当前价格（对应 C++ GetCurrentPlantCost）
    pub fn get_current_plant_cost(&self, seed_type: SeedType, imitater_type: SeedType) -> i32 {
        let mut cost = Plant::get_cost(seed_type, imitater_type);
        if self.plant_uses_accelerated_pricing(seed_type) {
            cost += self.count_plant_by_type(seed_type) * 50;
        }
        cost
    }

    /// 在某行查找割草机（对应 C++ FindLawnMowerInRow）
    pub fn find_lawn_mower_in_row(&self, _row: i32) -> Option<&LawnMower> {
        self.lawn_mowers.iter().find(|m| !m.dead && !m.mowing)
    }

    pub fn find_lawn_mower_in_row_mut(&mut self, _row: i32) -> Option<&mut LawnMower> {
        self.lawn_mowers.iter_mut().find(|m| !m.dead && !m.mowing)
    }

    // ========== 坐标转换（补充） ==========

    /// 像素坐标 → 网格 X，保持在棋盘内（对应 C++ PixelToGridXKeepOnBoard）
    pub fn pixel_to_grid_x_keep_on_board(&self, x: i32, y: i32) -> i32 {
        self.pixel_to_grid_x(x, y).max(0)
    }

    /// 像素坐标 → 网格 Y，保持在棋盘内（对应 C++ PixelToGridYKeepOnBoard）
    pub fn pixel_to_grid_y_keep_on_board(&self, x: i32, y: i32) -> i32 {
        self.pixel_to_grid_y(x.max(80), y).max(0)
    }

    /// 网格 Y → 像素坐标（对应 C++ GridToPixelY）
    pub fn grid_to_pixel_y(&self, grid_x: i32, grid_y: i32) -> i32 {
        let ay = if self.stage_has_roof() {
            let slope_offset = if grid_x < 5 { (5 - grid_x) * 20 } else { 0 };
            grid_y * 85 + slope_offset + LAWN_YMIN - 10
        } else if self.stage_has_pool() {
            grid_y * 85 + LAWN_YMIN
        } else {
            grid_y * 100 + LAWN_YMIN
        };
        // 高地修正
        if grid_x != -1 && grid_x < MAX_GRID_SIZE_X as i32 && grid_y < MAX_GRID_SIZE_Y as i32 {
            let gx = grid_x as usize;
            let gy = grid_y as usize;
            if self.grid_square_type[gx][gy] == GridSquareType::HighGround {
                return ay - HIGH_GROUND_HEIGHT;
            }
        }
        ay
    }

    /// 根据行获取基于位置的 Y 坐标（对应 C++ GetPosYBasedOnRow）
    pub fn get_pos_y_based_on_row(&self, pos_x: f32, row: i32) -> f32 {
        if self.stage_has_roof() {
            let mut slope_offset = 0.0;
            if pos_x < 440.0 {
                slope_offset = (440.0 - pos_x) * 0.25;
            }
            self.grid_to_pixel_y(8, row) as f32 + slope_offset
        } else {
            self.grid_to_pixel_y(0, row) as f32
        }
    }

    /// 获取冰冻效果 Z 坐标（对应 C++ GetIceZPos）
    pub fn get_ice_z_pos(&self, row: i32) -> i32 {
        make_render_order(RENDER_LAYER_GROUND, row, 2)
    }

    // ========== 植物查询 ==========

    /// 查找南瓜头（对应 C++ GetPumpkinAt）
    pub fn get_pumpkin_at(&self, grid_x: i32, grid_y: i32) -> Option<&Plant> {
        self.plants.iter().find(|p| {
            !p.dead && p.plant_col == grid_x && p.start_row == grid_y && p.seed_type == SeedType::Pumpkinshell
        })
    }

    pub fn get_pumpkin_at_mut(&mut self, grid_x: i32, grid_y: i32) -> Option<&mut Plant> {
        self.plants.iter_mut().find(|p| {
            !p.dead && p.plant_col == grid_x && p.start_row == grid_y && p.seed_type == SeedType::Pumpkinshell
        })
    }

    /// 查找花盆（对应 C++ GetFlowerPotAt）
    pub fn get_flower_pot_at(&self, grid_x: i32, grid_y: i32) -> Option<&Plant> {
        self.plants.iter().find(|p| {
            !p.dead && p.plant_col == grid_x && p.start_row == grid_y && p.seed_type == SeedType::Flowerpot
        })
    }

    /// 查找伞叶植物（对应 C++ FindUmbrellaPlant）
    pub fn find_umbrella_plant(&self, grid_x: i32, grid_y: i32) -> Option<&Plant> {
        self.plants.iter().find(|p| {
            !p.dead && p.plant_col == grid_x && p.start_row == grid_y && p.seed_type == SeedType::Umbrella
        })
    }

    /// 获取顶部植物（对应 C++ GetTopPlantAt 简化版）
    pub fn get_top_plant_at(&self, _grid_x: i32, _grid_y: i32) -> Option<&Plant> {
        self.plants.iter().find(|p| !p.dead)
    }

    // ========== UI 更新 ==========

    /// 更新层（对应 C++ UpdateLayers）
    pub fn update_layers(&mut self) {
        // 简化版：在 Rust 框架中无需操作 WidgetManager 层
    }

    /// 清除光标（对应 C++ ClearCursor 简化版）
    pub fn clear_cursor(&mut self) {
        self.m_advice = AdviceType::None;
    }

    /// 更新鼠标位置（对应 C++ UpdateMousePosition 简化版）
    pub fn update_mouse_position(&mut self, x: i32, y: i32) {
        self.m_prev_mouse_x = x;
        self.m_prev_mouse_y = y;
    }

    // ========== 进度条 ==========

    /// 进度条是否有旗帜（对应 C++ ProgressMeterHasFlags）
    pub fn progress_meter_has_flags(&self) -> bool {
        if matches!(self.game_mode, GameMode::ChallengeBeghouled | GameMode::ChallengeBeghouledTwist) {
            return true;
        }
        if self.m_progress_meter_width == 0 { return false; }
        true
    }

    // ========== 关卡状态 ==========

    /// 是否为零末尾关（对应 C++ IsFinalScaryPotterStage）
    pub fn is_final_scary_potter_stage(&self) -> bool {
        let is_scary_potter = self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() });
        if !is_scary_potter {
            return false;
        }

        let is_adventure = self.app.map_or(false, |app| unsafe { (*app).is_adventure_mode() });
        if is_adventure {
            // 冒险模式中，第 3 关（0-based index 2）是最后一关
            return self.challenge.as_ref().map_or(false, |c| c.survival_stage == 2);
        }

        // 非冒险模式：如果无尽可怕陶罐功能未实现，则总是最后一关
        // C++ 中调用 mApp->IsEndlessScaryPotter(mApp->mGameMode)
        true
    }

    /// 是否为最终生存关（对应 C++ IsFinalSurvivalStage）
    pub fn is_final_survival_stage(&self) -> bool {
        let is_survival = self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() });
        if !is_survival {
            return false;
        }

        let stage = self.challenge.as_ref().map_or(0, |c| c.survival_stage);
        let a_flags = self.get_num_waves_per_survival_stage() * (stage + 1) / self.get_num_waves_per_flag();

        let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        if self.app.map_or(false, |app| unsafe { (*app).is_survival_normal(game_mode) }) {
            return a_flags >= 5;
        }
        if self.app.map_or(false, |app| unsafe { (*app).is_survival_hard(game_mode) }) {
            return a_flags >= 10;
        }

        false
    }

    /// 是否为最后的坚守战关卡（对应 C++ IsLastStandFinalStage）
    pub fn is_last_stand_final_stage(&self) -> bool {
        let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        game_mode == GameMode::ChallengeLastStand
            && self.challenge.as_ref().map_or(false, |c| c.survival_stage == LAST_STAND_FLAGS - 1)
    }

    /// 是否为可重新选卡的生存关卡（对应 C++ IsSurvivalStageWithRepick）
    pub fn is_survival_stage_with_repick(&self) -> bool {
        let is_survival = self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() });
        is_survival && !self.is_final_survival_stage()
    }

    /// 是否为可重新选卡的坚守战关卡（对应 C++ IsLastStandStageWithRepick）
    pub fn is_last_stand_stage_with_repick(&self) -> bool {
        let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        game_mode == GameMode::ChallengeLastStand && !self.is_last_stand_final_stage()
    }

    /// 获取完成的旗帜数（对应 C++ GetSurvivalFlagsCompleted）
    pub fn get_survival_flags_completed(&self) -> i32 {
        let stage = self.challenge.as_ref().map_or(0, |c| c.survival_stage);
        stage * self.get_num_waves_per_survival_stage() / self.get_num_waves_per_flag()
    }

    /// 保存生存模式分数（对应 C++ SurvivalSaveScore）
    pub fn survival_save_score(&self) {}

    /// 保存谜题连胜（对应 C++ PuzzleSaveStreak）
    pub fn puzzle_save_streak(&self) {}

    /// 是否在挥舞铁锹教程中与 Crazy Dave 对话（对应 C++ IsScaryPotterDaveTalking）
    pub fn is_scary_potter_dave_talking(&self) -> bool {
        let is_scary_potter = self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() });
        if !is_scary_potter {
            return false;
        }
        let dave_present = self.app.map_or(false, |app| unsafe {
            (*app).m_crazy_dave_state != CrazyDaveState::NotHere
        });
        self.m_next_survival_stage_counter > 0 && dave_present
    }

    /// 僵尸获胜（对应 C++ ZombiesWon 简化版）
    pub fn zombies_won(&mut self) {
        self.m_board_result = BoardResult::Lost;
        self.m_game_over = true;
    }

    /// 处理删除队列（对应 C++ ProcessDeleteQueue）
    pub fn process_delete_queue(&mut self) {
        // Vec 的 `retain` 相当于 DataArray 的清理
        // 但在 Rust 中，我们使用 cleanup_dead() 方法
    }

    /// 停止所有僵尸声音（对应 C++ StopAllZombieSounds）
    pub fn stop_all_zombie_sounds(&mut self) {}

    // ========== 传送带/种子 ==========

    /// 是否有传送带种子槽（对应 C++ HasConveyorBeltSeedBank）
    pub fn has_conveyor_belt_seed_bank(&self) -> bool {
        matches!(self.game_mode,
            GameMode::ChallengeLittleTrouble | GameMode::ChallengePortalCombat
            | GameMode::ChallengeColumns | GameMode::ChallengeBobsledBonanza
            | GameMode::ChallengeInvisighoul | GameMode::ChallengeWallnutBowling
            | GameMode::ChallengeWallnutBowling2 | GameMode::ChallengePogoParty
            | GameMode::ChallengeDrZomboss | GameMode::ChallengeZombieNimble
            | GameMode::ChallengeMovingTarget | GameMode::ChallengeHeavyWeapons
            | GameMode::ChallengeTimeAttack
        )
    }

    /// 更新进度条（对应 C++ UpdateProgressMeter）
    pub fn update_progress_meter(&mut self) {
        // 简化版：进度条更新依赖具体波次和僵尸血量
    }

    /// 更新文本追踪检测（对应 C++ DoTypingCheck）
    pub fn do_typing_check(&self, _key: i32) {}

    // ========== 僵尸生成 ==========

    /// 在某行添加僵尸（对应 C++ AddZombieInRow 简化版）
    pub fn add_zombie_in_row(&mut self, zombie_type: ZombieType, _row: i32, from_wave: i32) {
        let mut zombie = Zombie::new();
        zombie.zombie_type = zombie_type;
        zombie.from_wave = from_wave;
        self.zombies.push(zombie);
    }

    /// 添加僵尸（对应 C++ AddZombie）
    pub fn add_zombie(&mut self, zombie_type: ZombieType, from_wave: i32) {
        let row = self.pick_row_for_new_zombie(zombie_type);
        self.add_zombie_in_row(zombie_type, row, from_wave);
    }

    /// 为新僵尸选择行（对应 C++ PickRowForNewZombie）
    pub fn pick_row_for_new_zombie(&mut self, zombie_type: ZombieType) -> i32 {
        // 获取 app 的 game_mode
        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });

        // ====================================================================================================
        // ▲ 当存在正在寻找目标僵尸的钉耙，且僵尸可以出现在钉耙所在行时，优先出现在钉耙所在行
        // ====================================================================================================
        let rake_result = {
            let has_rake = self.get_rake().map_or(false, |rake| {
                rake.grid_item_state == GridItemState::RakeAttracting
                    && self.row_can_have_zombie_type(rake.grid_y, zombie_type)
            });
            if has_rake {
                let row = self.get_rake().unwrap().grid_y;
                let rake = self.get_rake_mut().unwrap();
                rake.grid_item_state = GridItemState::RakeWaiting;
                crate::todlib::tod_common::tod_update_smooth_array_pick(
                    &mut self.m_row_picking_array, row as usize
                );
                Some(row)
            } else {
                None
            }
        };
        if let Some(row) = rake_result {
            return row;
        }

        // ====================================================================================================
        // ▲ 遍历每一行，将所有能允许该僵尸出现的行及其对应权重写入挑选数组中
        // ====================================================================================================
        for a_row in 0..MAX_GRID_SIZE_Y as i32 {
            // 如果本行不能出现目标僵尸，则将本行权重置零，并继续下一行
            if !self.row_can_have_zombie_type(a_row, zombie_type) {
                self.m_row_picking_array[a_row as usize].weight = 0.0;
            }
            // 传送门关卡中，每行的出怪概率受传送门位置影响
            else if app_mode == GameMode::ChallengePortalCombat {
                if let Some(ref challenge) = self.challenge {
                    self.m_row_picking_array[a_row as usize].weight =
                        challenge.portal_combat_row_spawn_weight(a_row);
                }
            }
            // 隐形食脑者关卡中，前 3 波第六路不出怪
            else if app_mode == GameMode::ChallengeInvisighoul
                && self.m_current_wave <= 3
                && a_row == 5
            {
                self.m_row_picking_array[a_row as usize].weight = 0.0;
            }
            // 丢车保护
            else {
                let waves_mowered = self.m_current_wave - self.m_wave_row_got_lawn_mowered[a_row as usize];
                let waves_mowered = if app_mode == GameMode::ChallengeBobsledBonanza
                    // Resodded 挑战也类似，但 Rust 枚举中暂无 ChallengeResodded
                    && self.m_current_wave == self.m_num_waves - 1
                {
                    100
                } else {
                    waves_mowered
                };

                self.m_row_picking_array[a_row as usize].weight = if waves_mowered <= 1 {
                    0.01
                } else if waves_mowered <= 2 {
                    0.5
                } else {
                    1.0
                };
            }
        }

        // 从平滑数组中按权重选择一行
        crate::todlib::tod_common::tod_pick_from_smooth_array(&mut self.m_row_picking_array)
    }

    /// 统计波中僵尸总血量（对应 C++ TotalZombiesHealthInWave 简化版）
    pub fn total_zombies_health_in_wave(&self, _wave_index: i32) -> i32 {
        0
    }

    // ========== 实体更新 ==========

    /// 更新所有游戏物体（对应 C++ UpdateGameObjects）
    pub fn update_game_objects(&mut self) {
        for plant in &mut self.plants { plant.update(); }
        for zombie in &mut self.zombies { zombie.update(); }
        for projectile in &mut self.projectiles { projectile.update(); }
        for coin in &mut self.coins { coin.update(); }
        for mower in &mut self.lawn_mowers { mower.update(); }
        for item in &mut self.grid_items { item.update(); }
    }

    // ========== 特殊格子操作 ==========

    /// 选择特殊墓碑（对应 C++ PickSpecialGraveStone 简化版）
    pub fn pick_special_grave_stone(&mut self) {
        // 简化版
    }

    /// 获取铁锹按钮矩形（对应 C++ GetShovelButtonRect 简化版）
    pub fn get_shovel_button_rect(&self) -> Rect {
        Rect::new(0, 0, 0, 0)
    }

    // ========== 计数 ==========

    /// 统计空花盆或睡莲数量（对应 C++ CountEmptyPotsOrLilies 简化版）
    pub fn count_empty_pots_or_lilies(&self, _seed_type: SeedType) -> i32 {
        0
    }

    /// 检查是否有效玉米加农炮位置（对应 C++ IsValidCobCannonSpot 简化版）
    pub fn is_valid_cob_cannon_spot(&self, _grid_x: i32, _grid_y: i32) -> bool {
        false
    }

    /// 是否有有效的玉米加农炮位置（对应 C++ HasValidCobCannonSpot）
    pub fn has_valid_cob_cannon_spot(&self) -> bool {
        false
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
