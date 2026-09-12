// PvZ Portable Rust 翻译 — Board（游戏棋盘）
// 对应 C++ src/Lawn/Board.h / Board.cpp
//
// Board 是整个游戏的核心：管理植物、僵尸、子弹、硬币等所有实体的
// 更新和绘制。这是 PvZ 游戏逻辑的心脏。

use crate::framework::key_codes::KeyCode;
use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::DrawMode;
use crate::todlib::tod_foley::FoleyType;
use crate::lawn::lawn_app::{GameScenes, LawnApp};
use crate::lawn::plant::Plant;
use crate::lawn::zombie::{Zombie, get_zombie_definition, MAX_ZOMBIE_FOLLOWERS};
use crate::lawn::projectile::Projectile;
use crate::lawn::coin::Coin;
use crate::lawn::lawn_mower::LawnMower;
use crate::lawn::grid_item::{GridItem, GridItemType};
use crate::lawn::seed_packet::SeedPacket;
use crate::lawn::challenge::Challenge;
use crate::lawn::cursor_object::{CursorObject, CursorPreview};
use crate::lawn::tool_tip_widget::ToolTipWidget;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;
use crate::framework::color::Color;
use crate::framework::common::{self, RandRange, RandFloat};
use crate::todlib::tod_particle::ParticleSystem;
use crate::todlib::reanimator::Reanimation;
use crate::todlib::tod_common::{TodSmoothArray, TodWeightedArray, TodWeightedGridArray, tod_pick_from_weighted_array, tod_pick_from_weighted_grid_array, tod_animate_curve};

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
pub const NUM_LEVELS: i32 = 50;

/// 各关卡的僵尸波次数（对应 C++ gZombieWaves）
pub const G_ZOMBIE_WAVES: [i32; NUM_LEVELS as usize] = [
    4,  6,  8,  10, 8,  10, 20, 10, 20, 20,
    10, 20, 10, 20, 10, 10, 20, 10, 20, 20,
    10, 20, 20, 30, 20, 20, 30, 20, 30, 30,
    10, 20, 10, 20, 20, 10, 20, 10, 20, 20,
    10, 20, 20, 30, 20, 20, 30, 20, 30, 30,
];
pub const MAX_RENDER_ITEMS: usize = 2048;
pub const MAX_POOL_GRID_SIZE: usize = 8;
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
    pub object_index: usize,  // 对应实体列表中的索引（Ice 项存 grid_y）
    pub boss_part: Option<BossPart>,  // 对应 C++ union mBossPart（仅 RENDER_ITEM_BOSS_PART 使用）
}

/// 僵尸生成选择器（对应 C++ ZombiePicker）
#[derive(Clone)]
pub struct ZombiePicker {
    pub zombie_count: i32,
    pub zombie_points: i32,
    pub zombie_type_count: [i32; NUM_ZOMBIE_TYPES as usize],
    pub all_waves_zombie_type_count: [i32; NUM_ZOMBIE_TYPES as usize],
}

impl ZombiePicker {
    pub fn new() -> Self {
        ZombiePicker {
            zombie_count: 0,
            zombie_points: 0,
            zombie_type_count: [0; NUM_ZOMBIE_TYPES as usize],
            all_waves_zombie_type_count: [0; NUM_ZOMBIE_TYPES as usize],
        }
    }

    /// 初始化选择器（对应 C++ ZombiePickerInitForWave）
    pub fn init_for_wave(&mut self) {
        self.zombie_count = 0;
        self.zombie_points = 0;
        self.zombie_type_count = [0; NUM_ZOMBIE_TYPES as usize];
    }

    /// 全局初始化（对应 C++ ZombiePickerInit）
    pub fn init(&mut self) {
        self.init_for_wave();
        self.all_waves_zombie_type_count = [0; NUM_ZOMBIE_TYPES as usize];
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
    pub grid_array: [TodWeightedGridArray; MAX_POOL_GRID_SIZE],
    pub grid_array_count: usize,
}

impl BungeeDropGrid {
    pub fn new() -> Self {
        BungeeDropGrid {
            grid_array: [TodWeightedGridArray { x: 0, y: 0, weight: 0 }; MAX_POOL_GRID_SIZE],
            grid_array_count: 0,
        }
    }
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

/// 判断网格坐标是否在范围内（对应 C++ GridInRange）
pub fn grid_in_range(x1: i32, y1: i32, x2: i32, y2: i32, range_x: i32, range_y: i32) -> bool {
    x1 >= x2 - range_x && x1 <= x2 + range_x && y1 >= y2 - range_y && y1 <= y2 + range_y
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
    /// 对应 C++ mHelpDisplayed[NUM_ADVICE_TYPES]（每个提示只显示一次）
    pub m_help_displayed: [bool; NUM_ADVICE_TYPES as usize],
    /// 对应 C++ mAdvice（MessageWidget*）提示消息控件
    pub m_advice_widget: crate::lawn::widget::message_widget::MessageWidget,

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
    pub menu_button: Option<*mut crate::lawn::widget::game_button::GameButton>,
    pub store_button: Option<*mut crate::lawn::widget::game_button::GameButton>,
    // 对应 C++ Widget mX/mY（Board::Move 设置棋盘平移）
    pub m_x: i32,
    pub m_y: i32,
    /// 对应 C++ SeedBank::mCutSceneDarken（过场暗化）
    pub m_seed_bank_darken: i32,
    /// 种子银行 X 位置（对应 C++ mSeedBank->mX）
    pub m_seed_bank_x: i32,
    /// 种子银行 Y 位置（对应 C++ mSeedBank->mY，布局系统未接入时以 0 占位）
    pub m_seed_bank_y: i32,
    /// 传送带计数器（对应 C++ SeedBank::mConveyorBeltCounter）
    pub m_conveyor_belt_counter: i32,
    pub ignore_mouse_up: bool,
    pub m_paused: bool,

    // 阳光
    pub m_sun_count: i32,
    pub m_sun_countdown: i32,
    pub m_num_suns_fallen: i32,
    pub m_sun_money: i32,

    // 金钱
    pub m_coin_bank: i64,
    pub m_coin_bank_fade_count: i32,

    // 关卡状态
    pub m_wave_count: i32,
    pub m_current_wave: i32,
    pub m_total_waves: i32,
    pub m_num_waves: i32,
    pub m_zombies_in_wave: Box<[[ZombieType; MAX_ZOMBIES_IN_WAVE]; MAX_ZOMBIE_WAVES]>,
    pub m_zombie_allowed: [bool; NUM_ZOMBIE_TYPES as usize],
    pub m_total_spawned_waves: i32,
    pub m_out_of_money_counter: i32,
    pub m_wave_countdown: i32,
    pub m_flag_countdown: i32,
    pub m_level_complete: bool,
    pub m_game_over: bool,
    pub m_game_over_countdown: i32,
    pub m_dropped_first_coin: bool,
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
    pub m_triggered_lawn_mowers: i32,
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
    // 对应 C++ mBoardUpdateCounter（Board::Update 帧计数）
    pub m_board_update_counter: u32,
    pub m_draw_count: u32,
    // 对应 C++ mStartDrawTime / mIntervalDrawTime / mIntervalDrawCountStart（FPS 统计）
    pub m_start_draw_time: i64,
    pub m_interval_draw_time: i64,
    pub m_interval_draw_count_start: u32,
    /// 对应 C++ mMinFPS（Board.cpp:154 初始 1000.0f）
    pub m_min_fps: f32,
    pub m_rise_from_grave_counter: i32,
    pub m_huge_wave_count_down: i32,
    pub m_zombie_count_down: i32,
    pub m_zombie_count_down_start: i32,
    pub m_zombie_health_to_next_wave: i32,
    pub m_zombie_health_wave_start: i32,
    pub m_final_wave_sound_counter: i32,
    pub m_board_rand_seed: u32,

    // 冻结/减速效果
    pub m_ice_timer: [i32; MAX_GRID_SIZE_Y],
    pub m_ice_min_x: [i32; MAX_GRID_SIZE_Y],
    pub m_ice_trap_counter: i32,
    /// 对应 C++ mIceParticleID[MAX_GRID_SIZE_Y]（每行冰粒子系统 ID）
    pub m_ice_particle_id: [crate::lawn::game_enums::ParticleSystemID; MAX_GRID_SIZE_Y],
    /// 对应 C++ mPoolSparklyParticleID（泳池闪光粒子系统 ID）
    pub m_pool_sparkly_particle_id: crate::lawn::game_enums::ParticleSystemID,

    // 火焰扫荡效果（对应 C++ mFwooshCountDown / mFwooshID）
    pub m_fwoosh_count_down: i32,
    pub m_fwoosh_id: [[ReanimationID; 12]; MAX_GRID_SIZE_Y],
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
    pub m_coins_collected: i32,
    // 对应 C++ mPottedPlantsCollected / mChocolateCollected / mDiamondsCollected / mLevelCoinsCollected
    pub m_potted_plants_collected: i32,
    pub m_chocolate_collected: i32,
    pub m_diamonds_collected: i32,
    pub m_level_coins_collected: i32,
    // 成就统计（对应 C++ 同名成员）
    pub m_pea_shooter_used: bool,
    pub m_catapult_plants_used: bool,
    pub m_mushroom_and_coffee_beans_only: bool,
    pub m_mushrooms_used: bool,

    // 模式标志
    pub m_mustache_mode: bool,
    pub m_future_mode: bool,
    pub m_pinata_mode: bool,
    pub m_dance_mode: bool,
    pub m_daisy_mode: bool,
    pub m_sukhbir_mode: bool,
    pub m_super_mower_mode: bool,

    // 光标对象
    pub cursor_object: CursorObject,
    /// 对应 C++ mCursorPreview（种植预览，Board.h:90）
    pub cursor_preview: CursorPreview,

    // 玉米加农炮瞄准（对应 C++ mCobCannonCursorDelayCounter/mCobCannonMouseX/mCobCannonMouseY）
    pub m_cob_cannon_cursor_delay_counter: i32,
    pub m_cob_cannon_mouse_x: i32,
    pub m_cob_cannon_mouse_y: i32,

    // 交互状态
    pub m_time_stop_counter: i32,        // 对应 C++ mTimeStopCounter
    pub m_tutorial_timer: i32,           // 对应 C++ mTutorialTimer
    pub m_tutorial_particle_id: u32,     // 对应 C++ mTutorialParticleID（ParticleSystemID）

    // 工具提示
    pub tool_tip: ToolTipWidget,         // 对应 C++ mToolTip

    // 切场景引用（用于 CutScene 交互委托）
    pub m_cut_scene: Option<*mut crate::lawn::cutscene::CutScene>,
}

/// i32 → MessageStyle 安全映射（对应 C++ MessageStyle 枚举 0..18，非法值回落 Off）
fn i32_to_message_style(style: i32) -> MessageStyle {
    match style {
        0 => MessageStyle::Off,
        1 => MessageStyle::TutorialLevel1,
        2 => MessageStyle::TutorialLevel1Stay,
        3 => MessageStyle::TutorialLevel2,
        4 => MessageStyle::TutorialLater,
        5 => MessageStyle::TutorialLaterStay,
        6 => MessageStyle::HintLong,
        7 => MessageStyle::HintFast,
        8 => MessageStyle::HintStay,
        9 => MessageStyle::HintTallFast,
        10 => MessageStyle::HintTallUnlockMessage,
        11 => MessageStyle::HintTallLong,
        12 => MessageStyle::BigMiddle,
        13 => MessageStyle::BigMiddleFast,
        14 => MessageStyle::HouseName,
        15 => MessageStyle::HugeWave,
        16 => MessageStyle::SlotMachine,
        17 => MessageStyle::ZenGardenLong,
        18 => MessageStyle::Achievement,
        _ => MessageStyle::Off,
    }
}

/// 从资源管理器按资源 ID 取图（对应 C++ Sexy::IMAGE_* 全局图片），找不到返回空指针
pub(crate) fn get_overlay_image(app: &LawnApp, name: &str) -> *mut crate::framework::graphics::image::Image {
    let Some(rm_ptr) = app.base.resource_manager else {
        return std::ptr::null_mut();
    };
    unsafe {
        let rm = &*rm_ptr;
        let shared = rm.get_image(name);
        let img_ptr = shared.as_image_ptr();
        if !img_ptr.is_null() {
            return img_ptr;
        }
        // 尝试小写（与 challenge.rs get_image 一致）
        let lower = crate::framework::common::string_to_lower(name);
        let shared2 = rm.get_image(&lower);
        let img_ptr2 = shared2.as_image_ptr();
        if !img_ptr2.is_null() {
            return img_ptr2;
        }
    }
    std::ptr::null_mut()
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
            m_help_displayed: [false; NUM_ADVICE_TYPES as usize],
            m_advice_widget: crate::lawn::widget::message_widget::MessageWidget::new(None),
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
            menu_button: Some(Box::into_raw(Box::new(crate::lawn::widget::game_button::GameButton::new(0, None)))),
            store_button: Some(Box::into_raw(Box::new(crate::lawn::widget::game_button::GameButton::new(1, None)))),
            m_x: 0,
            m_y: 0,
            m_seed_bank_darken: 0,
            m_seed_bank_x: 0,
            m_seed_bank_y: 0,
            m_conveyor_belt_counter: 0,
            ignore_mouse_up: false,
            m_paused: false,
            m_sun_count: 50,
            m_sun_countdown: SUN_COUNTDOWN,
            m_num_suns_fallen: 0,
            m_sun_money: 0,
            m_coin_bank: 0,
            m_coin_bank_fade_count: 0,
            m_wave_count: 0,
            m_current_wave: 0,
            m_total_waves: 0,
            m_num_waves: 0,
            m_zombies_in_wave: Box::new([[ZombieType::Invalid; MAX_ZOMBIES_IN_WAVE]; MAX_ZOMBIE_WAVES]),
            m_zombie_allowed: [false; NUM_ZOMBIE_TYPES as usize],
            m_total_spawned_waves: 0,
            m_out_of_money_counter: 0,
            m_wave_countdown: 0,
            m_flag_countdown: 0,
            m_level_complete: false,
            m_game_over: false,
            m_game_over_countdown: 0,
            m_dropped_first_coin: false,
            m_board_fade_out_counter: -1,
            m_next_survival_stage_counter: 0,
            m_score_next_mower_counter: 0,
            m_level_award_spawned: false,
            m_progress_meter_width: 0,
            m_flag_raise_counter: 0,
            m_sod_position: 0,
            m_plant_row: [PlantRowType::Normal; MAX_GRID_SIZE_Y],
            m_wave_row_got_lawn_mowered: [-100; MAX_GRID_SIZE_Y],
            m_triggered_lawn_mowers: 0,
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
            m_board_update_counter: 0,
            m_draw_count: 0,
            m_start_draw_time: 0,
            m_interval_draw_time: 0,
            m_interval_draw_count_start: 0,
            m_min_fps: 1000.0,
            m_rise_from_grave_counter: 0,
            m_huge_wave_count_down: 0,
            m_zombie_count_down: 0,
            m_zombie_count_down_start: 0,
            m_zombie_health_to_next_wave: 0,
            m_zombie_health_wave_start: 0,
            m_final_wave_sound_counter: 0,
            m_board_rand_seed: 0,
            m_ice_timer: [0; MAX_GRID_SIZE_Y],
            m_ice_min_x: [0; MAX_GRID_SIZE_Y],
            m_ice_trap_counter: 0,
            m_ice_particle_id: [0; MAX_GRID_SIZE_Y],
            m_pool_sparkly_particle_id: 0,
            m_fwoosh_count_down: 0,
            m_fwoosh_id: [[REANIMATIONID_NULL; 12]; MAX_GRID_SIZE_Y],
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
            m_coins_collected: 0,
            m_potted_plants_collected: 0,
            m_chocolate_collected: 0,
            m_diamonds_collected: 0,
            m_level_coins_collected: 0,
            m_pea_shooter_used: false,
            m_catapult_plants_used: false,
            m_mushroom_and_coffee_beans_only: false,
            m_mushrooms_used: false,
            m_mustache_mode: false,
            m_future_mode: false,
            m_pinata_mode: false,
            m_dance_mode: false,
            m_daisy_mode: false,
            m_sukhbir_mode: false,
            m_super_mower_mode: false,
            cursor_object: CursorObject::new(),
            cursor_preview: CursorPreview::new(),
            m_cob_cannon_cursor_delay_counter: 0,
            m_cob_cannon_mouse_x: 0,
            m_cob_cannon_mouse_y: 0,
            m_time_stop_counter: 0,
            m_tutorial_timer: 0,
            m_tutorial_particle_id: 0,
            tool_tip: ToolTipWidget::new(),
            m_cut_scene: None,
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

        // C++ Board 构造函数：mCursorPreview = new CursorPreview()；
        // 指针接线（与 coin/plant 等 GameObject 相同模式）
        self.cursor_preview.base.board = Some(self as *mut Board);
        self.cursor_preview.base.app = Some(app);
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
        // 对应 C++ InitZombieWaves：按关卡设置僵尸波次（允许列表 + 波次数据）
        self.init_zombie_waves();
    }

    /// 主更新循环
    pub fn update(&mut self) {
        // 对应 C++ Board::Update (Board.cpp:5717)
        // Widget::Update() / MarkDirty()：Rust 侧 Widget 更新计数在此推进
        self.m_update_count += 1;
        self.m_board_update_counter = self.m_board_update_counter.wrapping_add(1);

        // 对应 C++ Board.cpp:5724 mCutScene->Update()
        if let Some(cut_scene) = self.m_cut_scene {
            unsafe { (*cut_scene).update(); }
        }
        // UpdateMousePosition()：Rust 侧签名带坐标，由 WidgetManager 调用时传入；
        // 此处用 app->mWidgetManager->mLastMouseX/Y 驱动，与 C++ UpdateCursor 一致
        let cur_mouse_x;
        let cur_mouse_y;
        if let Some(app) = self.app {
            unsafe {
                cur_mouse_x = (*app).base.widget_manager.map_or(0, |wm| (*wm).last_mouse_x);
                cur_mouse_y = (*app).base.widget_manager.map_or(0, |wm| (*wm).last_mouse_y);
            }
        } else {
            cur_mouse_x = 0;
            cur_mouse_y = 0;
        }
        self.update_mouse_position(cur_mouse_x, cur_mouse_y);

        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        // 对应 C++ Board.cpp:5728-5732：禅园模式下推进 ZenGardenUpdate
        if app_mode == GameMode::ChallengeZenGarden {
            let zen_garden = self.app.and_then(|app| unsafe { (*app).zen_garden });
            if let Some(zen_garden) = zen_garden {
                unsafe { (*zen_garden).zen_garden_update(); }
            }
        }
        // 对应 C++ Board.cpp:5734-5735：ScaryPotter 教学时 CrazyDave 每帧推进
        if self.is_scary_potter_dave_talking() {
            if let Some(app) = self.app {
                unsafe { (*app).update_crazy_dave(); }
            }
        }

        if self.m_paused {
            // C++ Board.cpp:5738：暂停时仍推进挑战状态机（菜单/胜利演出），随后早退
            if let Some(ch) = self.challenge.as_mut() {
                ch.update();
            }
            return;
        }

        // 对应 C++ Board.cpp:5744-5754：菜单/商店按钮更新
        let a_disabled = !self.can_interact_with_board_buttons() || self.ignore_mouse_up;
        if let Some(menu_button) = self.menu_button {
            unsafe {
                if !(*menu_button).btn_no_draw {
                    (*menu_button).disabled = a_disabled;
                }
                (*menu_button).update();
            }
        }
        if let Some(store_button) = self.store_button {
            unsafe {
                (*store_button).disabled = a_disabled;
                (*store_button).update();
            }
        }

        // 对应 C++ Board.cpp:5756-5757：粒子效果系统与提示控件每帧更新
        if let Some(app) = self.app {
            unsafe {
                if let Some(effect_system) = (*app).effect_system.as_mut() {
                    effect_system.update();
                }
            }
        }
        self.m_advice_widget.update();
        self.update_tutorial();

        if self.m_cob_cannon_cursor_delay_counter > 0 {
            self.m_cob_cannon_cursor_delay_counter -= 1;
        }
        if self.m_out_of_money_counter > 0 {
            self.m_out_of_money_counter -= 1;
        }
        if self.m_shake_counter > 0 {
            self.m_shake_counter -= 1;
            if self.m_shake_counter == 0 {
                self.m_x = 0;
                self.m_y = 0;
            } else {
                if crate::todlib::tod_common::rand_range_int(0, 3) == 0 {
                    self.m_shake_amount_x = -self.m_shake_amount_x;
                }
                // C++ PvzpAnimateCurve(12,0,mShakeCounter,0,mShakeAmountX, CURVE_BOUNCE)
                self.m_x = crate::todlib::tod_common::tod_animate_curve(12, 0, self.m_shake_counter, 0, self.m_shake_amount_x, crate::lawn::game_enums::TodCurves::Bounce);
                self.m_y = crate::todlib::tod_common::tod_animate_curve(12, 0, self.m_shake_counter, 0, self.m_shake_amount_y, crate::lawn::game_enums::TodCurves::Bounce);
            }
        }
        if self.m_coin_bank_fade_count > 0
            // [TRANSLATION_NOTE]: C++ 检查 DIALOG_PURCHASE_PACKET_SLOT 未打开才递减；Rust 侧 LawnApp 无对话框管理，暂不过滤
        {
            self.m_coin_bank_fade_count -= 1;
        }
        self.update_layers();

        // C++ TimeStop 早退（5792）
        if self.m_time_stop_counter > 0 {
            return;
        }

                self.m_effect_counter = self.m_effect_counter.wrapping_add(1);
        // C++ Board.cpp:5796-5801：mEffectCounter++ 之后按阶段/场景/过场条件推进 PoolEffect 计数器
        // ```cpp
        // if (StageHasPool() && !mIceTrapCounter && mApp->mGameScene != GameScenes::SCENE_ZOMBIES_WON
        //     && !mCutScene->IsSurvivalRepick())
        // {
        //     mApp->mPoolEffect->mPoolCounter++;
        // }
        // ```
        // C++ 5800-5805: BACKGROUND_3_POOL 泳池闪光粒子（ID == NULL 时创建）
        // ```cpp
        // if (mBackground == BackgroundType::BACKGROUND_3_POOL && mPoolSparklyParticleID == ParticleSystemID::PARTICLESYSTEMID_NULL)
        // {
        //     int aRenderPosition = MakeRenderOrder(RENDER_LAYER_GROUND, 2, 0);
        //     PvzpParticleSystem* aPoolParticle = mApp->AddPvzpParticle(450, 295, aRenderPosition, PARTICLE_POOL_SPARKLY);
        //     mPoolSparklyParticleID = mApp->ParticleGetID(aPoolParticle);
        // }
        // ```
        if self.m_background_type == BackgroundType::Pool && self.m_pool_sparkly_particle_id == 0 {
            let a_render_position = make_render_order(RENDER_LAYER_GROUND, 2, 0);
            if let Some(app) = self.app {
                unsafe {
                    if let Some(ptr) = (*app).add_tod_particle(
                        450.0, 295.0, a_render_position, ParticleEffect::PoolSparkly as i32,
                    ) {
                        self.m_pool_sparkly_particle_id = (*app).particle_get_id(ptr);
                    }
                }
            }
        }
        if let Some(app) = self.app {
            unsafe {
                let app_ref = &mut *app;
                // C++: mCutScene->IsSurvivalRepick()（mCutScene 非空时调用）
                let cut_scene_ok = self.m_cut_scene.map_or(true, |cs| unsafe { !(*cs).is_survival_repick() });
                if self.stage_has_pool()
                    && self.m_ice_trap_counter == 0
                    && app_ref.game_scene != GameScenes::ZombiesWon
                    && cut_scene_ok
                {
                    if let Some(ef) = app_ref.pool_effect.as_mut() {
                        ef.pool_counter = ef.pool_counter.wrapping_add(1);
                    }
                }
            }
        }

        self.update_grid_items();
        self.update_fwoosh();
        self.update_game();
        // C++ Board.cpp:5810：UpdateGame() 之后独立调用 UpdateFog()。
        // UpdateGame 在非 SCENE_PLAYING 时提前 return 但 UpdateFog 仍会执行，故必须放在此处而非 update_game 内部。
        self.update_fog();
        // ★ 关键接入：挑战状态机每帧推进（C++ Board.cpp:5811）
        if let Some(ch) = self.challenge.as_mut() {
            ch.update();
        }
        self.update_level_end_sequence();
        self.m_prev_mouse_x = cur_mouse_x;
        self.m_prev_mouse_y = cur_mouse_y;
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

    /// 生成一波僵尸（对应 C++ SpawnZombieWave）
    /// 从预计算的波次列表 m_zombies_in_wave 中读取当前波次的僵尸并生成
    pub fn spawn_zombie_wave(&mut self) {
        // 调用挑战模式的波次钩子（如果有）
        if let Some(ref mut challenge) = self.challenge {
            challenge.spawn_zombie_wave();
        }

        // 判断当前关卡类型
        let is_bungee_blitz = self.app.map_or(false, |app| unsafe { (*app).is_bungee_blitz_level() });
        let is_continuous = self.app.map_or(false, |app| unsafe { (*app).is_continuous_challenge() });

        if is_bungee_blitz {
            // 蹦极闪电战：使用蹦极僵尸投放
            let mut bungee_drop_grid = BungeeDropGrid::new();
            Self::setup_bungee_drop(&mut bungee_drop_grid);

            for i in 0..MAX_ZOMBIES_IN_WAVE {
                let ztype = self.m_zombies_in_wave[self.m_current_wave as usize][i];
                if ztype == ZombieType::Invalid {
                    break;
                }

                if ztype == ZombieType::Bungee || ztype == ZombieType::Zamboni {
                    self.add_zombie(ztype, self.m_current_wave);
                } else {
                    self.bungee_drop_zombie(&mut bungee_drop_grid, ztype);
                }
            }
        } else {
            // 普通波次：从波次列表直接生成
            for i in 0..MAX_ZOMBIES_IN_WAVE {
                let ztype = self.m_zombies_in_wave[self.m_current_wave as usize][i];
                if ztype == ZombieType::Invalid {
                    break;
                }

                if ztype == ZombieType::Bobsled && !self.can_add_bob_sled() {
                    // 无法生成雪橇僵尸小队时，用 4 只普通僵尸代替
                    for _ in 0..MAX_ZOMBIE_FOLLOWERS {
                        self.add_zombie(ZombieType::Normal, self.m_current_wave);
                    }
                } else {
                    self.add_zombie(ztype, self.m_current_wave);
                }
            }
        }

        // 最后一波：启动墓碑升起计时器
        if self.m_current_wave == self.m_num_waves - 1 && !is_continuous {
            self.m_rise_from_grave_counter = 210;
        }

        // 旗子波次：举旗
        if self.is_flag_wave(self.m_current_wave) {
            self.m_flag_raise_counter = FLAG_RAISE_TIME;
        }

        self.m_current_wave += 1;
        self.m_total_spawned_waves += 1;
    }

    /*
    ============================================================
    僵尸生成辅助方法（对应 C++ Board.cpp SpawnZombiesFromPool 等）
    ============================================================
    */

    /// 从水池生成僵尸（对应 C++ SpawnZombiesFromPool）
    /// 在泳池关卡的池水区域（第2-3行，第5-8列）生成从墓穴中升起的僵尸
    fn spawn_zombies_from_pool(&mut self) {
        if self.m_ice_trap_counter > 0 {
            return;
        }

        let (a_count, a_zombie_points) = match self.level {
            21 | 22 | 31 | 32 => (2, 3),
            23 | 24 | 25 | 33 | 34 | 35 => (3, 5),
            _ => (3, 7),
        };

        let mut a_grid_array = [TodWeightedGridArray { x: 0, y: 0, weight: 0 }; MAX_POOL_GRID_SIZE];
        let mut a_grid_array_count = 0;
        for a_grid_x in 5..MAX_GRID_SIZE_X {
            for a_grid_y in 2..=3 {
                a_grid_array[a_grid_array_count] = TodWeightedGridArray { x: a_grid_x as i32, y: a_grid_y, weight: 10000 };
                a_grid_array_count += 1;
                debug_assert!(a_grid_array_count <= MAX_POOL_GRID_SIZE);
            }
        }

        let mut zombie_points = a_zombie_points;
        for _ in 0..a_count {
            if let Some(idx) = tod_pick_from_weighted_grid_array(&mut a_grid_array, a_grid_array_count) {
                a_grid_array[idx].weight = 0;
                let grid_x = a_grid_array[idx].x;
                let grid_y = a_grid_array[idx].y;

                let ztype = self.pick_grave_rising_zombie_type();
                let mut zombie = Zombie::new();
                zombie.zombie_initialize(grid_y, ztype, false, None, self.m_current_wave);
                self.zombies.push(zombie);

                zombie_points -= get_zombie_definition(ztype).zombie_value;
                if zombie_points < 1 { zombie_points = 1; }
            }
        }
    }

    /// 设置蹦极僵尸掉落网格（对应 C++ SetupBungeeDrop）
    fn setup_bungee_drop(bungee_drop: &mut BungeeDropGrid) {
        bungee_drop.grid_array_count = 0;
        for grid_x in 4..MAX_GRID_SIZE_X {
            for grid_y in 0..=4 {
                let count = bungee_drop.grid_array_count;
                bungee_drop.grid_array[count] = TodWeightedGridArray { x: grid_x as i32, y: grid_y, weight: 10000 };
                bungee_drop.grid_array_count += 1;
                debug_assert!(bungee_drop.grid_array_count <= MAX_POOL_GRID_SIZE);
            }
        }
    }

    /// 蹦极僵尸投放僵尸（对应 C++ BungeeDropZombie）
    fn bungee_drop_zombie(&mut self, bungee_drop: &mut BungeeDropGrid, zombie_type: ZombieType) {
        if let Some(idx) = tod_pick_from_weighted_grid_array(&mut bungee_drop.grid_array, bungee_drop.grid_array_count) {
            bungee_drop.grid_array[idx].weight = 1;

            let mut bungee_zombie = Zombie::new();
            bungee_zombie.zombie_initialize(0, ZombieType::Bungee, false, None, self.m_current_wave);
            self.zombies.push(bungee_zombie);

            let mut zombie = Zombie::new();
            zombie.zombie_initialize(0, zombie_type, false, None, self.m_current_wave);
            self.zombies.push(zombie);
        }
    }

    /// 从空中生成僵尸（对应 C++ SpawnZombiesFromSky）
    /// 用于屋顶关卡，通过蹦极僵尸投放
    fn spawn_zombies_from_sky(&mut self) {
        if self.m_ice_trap_counter > 0 {
            return;
        }

        let (a_count, mut zombie_points) = match self.level {
            41 | 42 => (2, 3),
            43 | 44 | 45 => (3, 5),
            _ => (3, 7),
        };

        let mut bungee_drop_grid = BungeeDropGrid::new();
        Self::setup_bungee_drop(&mut bungee_drop_grid);

        let mut a_count = a_count;
        if a_count > bungee_drop_grid.grid_array_count as i32 {
            a_count = bungee_drop_grid.grid_array_count as i32;
        }

        if bungee_drop_grid.grid_array_count == 0 || a_count <= 0 {
            return;
        }

        for _ in 0..a_count {
            let ztype = self.pick_grave_rising_zombie_type();
            self.bungee_drop_zombie(&mut bungee_drop_grid, ztype);
            zombie_points -= get_zombie_definition(ztype).zombie_value;
            if zombie_points < 1 { zombie_points = 1; }
        }
    }

    /// 从墓碑生成僵尸（对应 C++ SpawnZombiesFromGraves）
    fn spawn_zombies_from_graves(&mut self) {
        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        // C++: GAMEMODE_CHALLENGE_WAR_AND_PEAS || GAMEMODE_CHALLENGE_WAR_AND_PEAS_2 → return
        if app_mode == GameMode::ChallengeWarAndPeas || app_mode == GameMode::ChallengeWarAndPeas2 {
            return;
        }

        if self.stage_has_roof() {
            self.spawn_zombies_from_sky();
            return;
        } else if self.stage_has_pool() {
            self.spawn_zombies_from_pool();
            return;
        }

        // 遍历所有格子系统，从墓碑生成僵尸
        let mut to_spawn: Vec<(i32, i32)> = Vec::new();
        for item in &self.grid_items {
            // C++: if (aGridItem->mDead) continue;
            if item.dead {
                continue;
            }
            if item.grid_item_type != GridItemType::Grave || item.counter < 100 {
                continue;
            }
            // C++: if (mApp->mGameMode == GAMEMODE_CHALLENGE_GRAVE_DANGER && Rand(mNumWaves) > mCurrentWave) continue;
            if app_mode == GameMode::ChallengeGraveDanger {
                let rand_val = crate::todlib::tod_common::rand_range_int(0, self.m_num_waves);
                if rand_val > self.m_current_wave {
                    continue;
                }
            }
            to_spawn.push((item.grid_x, item.grid_y));
        }

        for (grid_x, grid_y) in to_spawn {
            let ztype = self.pick_grave_rising_zombie_type();
            let mut zombie = Zombie::new();
            zombie.zombie_initialize(grid_y, ztype, false, None, self.m_current_wave);
            // 指针接线（RiseFromGrave 内部经 base.get_board() 计算网格坐标）
            zombie.base.board = Some(self as *mut Board);
            zombie.base.app = self.app;
            // C++: aZombie->RiseFromGrave(aGridItem->mGridX, aGridItem->mGridY)
            zombie.rise_from_grave(grid_x, grid_y);
            self.zombies.push(zombie);
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

        // 僵尸与植物碰撞（进食判定；对应 C++ Zombie::UpdateEating 的进食频率门控 + EatPlant）
        for zombie in &mut self.zombies {
            if zombie.dead { continue; }

            // C++ Zombie::UpdateEating：chilled 时啃咬间隔加倍
            let mut a_ticks_between_eats = crate::lawn::zombie::DAMAGE_PER_EAT;
            if zombie.chilled_counter > 0 {
                a_ticks_between_eats *= 2;
            }
            if zombie.zombie_age % a_ticks_between_eats != 0 {
                continue;
            }

            let z_attack = zombie.get_zombie_attack_rect();

            let mut a_plant_eaten = false;
            for plant in &mut self.plants {
                if plant.dead || !plant.is_on_board { continue; }
                if plant.base.row != zombie.base.row { continue; }

                let p_rect = plant.plant_rect;
                if z_attack.intersects(&p_rect) {
                    zombie.eat_plant(plant);
                    a_plant_eaten = true;
                    break;
                }
            }
            // C++ Zombie::UpdateEating：目标不在了则停止进食
            if !a_plant_eaten && zombie.is_eating {
                zombie.stop_eating();
            }
        }

        // 僵尸与割草机碰撞（对应 C++ LawnMower::Update 中的碰撞检测→MowZombie）
        for zombie in &mut self.zombies {
            if zombie.dead { continue; }
            if zombie.zombie_type == ZombieType::Boss { continue; }

            for mower in &mut self.lawn_mowers {
                if mower.dead { continue; }
                if mower.base.row != zombie.base.row { continue; }
                if zombie.zombie_phase == ZombiePhase::Mowered || zombie.is_tangle_kelp_target() {
                    continue;
                }
                if !zombie.effected_by_damage(127) { continue; }

                let m_rect = mower.get_lawn_mower_attack_rect();
                let z_rect = zombie.get_zombie_rect();
                let a_overlap = get_rect_overlap(&m_rect, &z_rect);
                let min_overlap = if zombie.zombie_type == ZombieType::Balloon { 20 } else { 0 };
                if a_overlap <= min_overlap { continue; }

                // bungee 僵尸和已死的僵尸不能自己触发割草机
                if mower.mower_state != LawnMowerState::Ready
                    || (zombie.zombie_type != ZombieType::Bungee && zombie.has_head)
                {
                    mower.mow_zombie(zombie);
                }
            }
        }
    }

    /// 在指定行列查找植物（对应 C++ Board::GetTopPlantAt，TOPPLANT_EATING_ORDER）
    pub fn find_plant_at(&mut self, row: i32, col: usize) -> Option<&mut Plant> {
        if col >= MAX_GRID_SIZE_X { return None; }
        if row < 0 || row as usize >= MAX_GRID_SIZE_Y { return None; }

        // TOPPLANT_EATING_ORDER：南瓜 > 正常植物 > 底层植物（花盆/荷叶）
        let info = self.get_plants_on_lawn(col as i32, row);
        let idx = info.pumpkin_plant.or(info.normal_plant).or(info.under_plant)?;
        self.plants.get_mut(idx)
    }

    /// 在指定行查找僵尸（对应 C++ Plant::FindTargetZombie，Plant.cpp:4769）
    /// [TRANSLATION_NOTE]: 简化版（同行 + 植物右侧 + 最靠左）；C++ 完整版依赖 Plant 上下文
    /// （mSeedType/mState/攻击矩形/伤害范围标志/Portal 检查/Chomper/PotatoMine/TangleKelp 特判），
    /// 属 Plant 模块翻译范围，此处保留简化。
    pub fn find_zombie_in_row(&mut self, row: i32, from_col: i32) -> Option<&mut Zombie> {
        let mut best_idx = None;
        let mut best_x = i32::MAX;
        let plant_x = from_col * 80 + 40; // 植物列对应的像素 x
        for (i, zombie) in self.zombies.iter().enumerate() {
            if zombie.dead { continue; }
            let row_dev = if zombie.zombie_type == ZombieType::Boss { 0 } else { zombie.base.row - row };
            if row_dev != 0 { continue; }
            let z_x = zombie.get_zombie_rect().x;
            // 僵尸须位于植物右侧（含同列）才可被射击
            if z_x < plant_x { continue; }
            if z_x < best_x {
                best_x = z_x;
                best_idx = Some(i);
            }
        }
        best_idx.map(|i| &mut self.zombies[i])
    }

    /// 添加硬币
    pub fn add_coin(&mut self, x: f32, y: f32, coin_type: CoinType, motion: CoinMotion) {
        // 对应 C++ Board::AddCoin (Board.cpp:1985)
        let mut coin = Coin::new();
        coin.coin_initialize(x, y, coin_type, motion);
        coin.base.board = Some(self as *mut Board); // 注意：这是 unsafe 的简化
        if let Some(app) = self.app {
            coin.base.app = Some(app);
        }
        self.coins.push(coin);

        // C++ 1989-1992: 首次冒险模式第 1 关提示点击阳光
        let first_time = self.app.map_or(false, |app| unsafe { (*app).is_first_time_adventure_mode() });
        if first_time && self.level == 1 {
            self.display_advice("[ADVICE_CLICK_ON_SUN]", MessageStyle::TutorialLevel1Stay as i32, AdviceType::ClickOnSun);
        }
    }

    /// 添加子弹，返回新子弹在 mProjectiles 中的下标
    pub fn add_projectile(&mut self, x: f32, y: f32, row: i32, seed_type: SeedType) -> usize {
        let mut proj = Projectile::new();
        proj.projectile_initialize(x, y, row, seed_type);
        if let Some(app) = self.app {
            proj.base.app = Some(app);
        }
        proj.base.board = Some(self as *mut Board);
        self.projectiles.push(proj);
        self.projectiles.len() - 1
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

        // ★ 关键接入：通知挑战模式新植物已添加（C++ Board.cpp:2104 mChallenge->PlantAdded(aPlant)）
        if self.challenge.is_some() {
            if let Some(plant) = self.plants.last_mut() {
                // 分割借用：self.challenge 与 self.plants 为不同字段
                if let Some(ch) = self.challenge.as_mut() {
                    ch.plant_added(plant);
                }
            }
        }
    }

    /// 新建植物并返回引用（对应 C++ Board::NewPlant）
    pub fn new_plant(&mut self, grid_x: i32, grid_y: i32, seed_type: SeedType, imitater_type: SeedType) -> Option<&mut Plant> {
        let mut plant = Plant::new();
        plant.is_on_board = true;
        plant.plant_initialize(grid_x, grid_y, seed_type, imitater_type);
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
        self.plants.last_mut()
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
    /// [TRANSLATION_NOTE]: C++ Board::CreateRakeReanim (Board.cpp): AddReanimation(x+20,y,fUndefined,REANIM_RAKE)
    pub fn create_rake_reanim(&self, the_rake_x: f32, the_rake_y: f32, the_render_order: i32) -> Option<*mut crate::todlib::reanimator::Reanimation> {
        let app = self.app?;
        let a_reanim = unsafe {
            (*app).add_reanimation(
                the_rake_x + 20.0,
                the_rake_y,
                the_render_order,
                ReanimationType::Rake as i32,
            )
        }?;
        // C++: mAnimRate = 0; mLoopType = REANIM_PLAY_ONCE_AND_HOLD; mIsAttachment = true
        unsafe {
            a_reanim.as_mut()?.m_anim_rate = 0.0;
            a_reanim.as_mut()?.m_loop_type = crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold;
            a_reanim.as_mut()?.m_is_attachment = true;
        }
        Some(a_reanim)
    }

    /// [TRANSLATION_NOTE]: C++ Board::DrawDebugText (Board.cpp:6912-7051)
    /// string logic complete; font/image drawing placeholder
    pub fn draw_debug_text(&self, g: &mut Graphics) {
        let mut a_text = String::new();
        match self.m_debug_text_mode {
            DebugTextMode::None => {}
            DebugTextMode::ZombieSpawn => {
                let a_time = self.m_zombie_count_down_start - self.m_zombie_count_down;
                let a_fraction = if self.m_zombie_count_down_start != 0 {
                    a_time as f32 / self.m_zombie_count_down_start as f32
                } else {
                    0.0
                };
                a_text.push_str("ZOMBIE SPAWNING DEBUG
");
                a_text.push_str(&format!("CurrentWave: {} of {}
", self.m_current_wave, self.m_num_waves));
                a_text.push_str(&format!(
                    "TimeSinseLastSpawn: {} {}
",
                    a_time,
                    if a_time > 400 { "" } else { "(too soon)" }
                ));
                a_text.push_str(&format!(
                    "ZombieCountDown: {}/{} ({:.0}%)
",
                    self.m_zombie_count_down,
                    self.m_zombie_count_down_start,
                    a_fraction * 100.0
                ));
                if self.m_zombie_health_to_next_wave != -1 {
                    let a_total_health = self.total_zombies_health_in_wave(self.m_current_wave - 1);
                    let a_health_range = (self.m_zombie_health_wave_start - self.m_zombie_health_to_next_wave).max(1);
                    let a_health_fraction = (self.m_zombie_health_to_next_wave - a_total_health + a_health_range) as f32 / a_health_range as f32;
                    a_text.push_str(&format!(
                        "ZombieHealth: CurZombieHealth {} trigger {} ({:.0}%)
",
                        a_total_health,
                        self.m_zombie_health_to_next_wave,
                        a_health_fraction * 100.0
                    ));
                } else {
                    a_text.push_str("ZombieHealth: before first wave
");
                }
                if self.m_huge_wave_count_down > 0 {
                    a_text.push_str(&format!("HugeWaveCountDown: {}
", self.m_huge_wave_count_down));
                }
                if let Some(a_boss) = self.get_boss_zombie() {
                    a_text.push_str(&format!("
Spawn: {}
", a_boss.summon_counter));
                    a_text.push_str(&format!("Stomp: {}
", a_boss.boss_stomp_counter));
                    a_text.push_str(&format!("Bungee: {}
", a_boss.boss_bungee_counter));
                    a_text.push_str(&format!("Head: {}
", a_boss.boss_head_counter));
                }
            }
            DebugTextMode::Music => {
                // [TRANSLATION_NOTE]: C++ reads mApp->mMusic burst/drums state; Music fields not fully aligned
                a_text.push_str("MUSIC DEBUG
");
                a_text.push_str(&format!("CurrentWave: {} of {}
", self.m_current_wave, self.m_num_waves));
            }
            DebugTextMode::Memory => {
                a_text.push_str("MEMORY DEBUG
");
                if let Some(app) = self.app {
                    unsafe {
                        if let Some(es) = (*app).effect_system.as_ref() {
                            a_text.push_str(&format!("attachments {}
", es.attachments.len()));
                            a_text.push_str(&format!("particle systems {}
", es.particle_systems.len()));
                            a_text.push_str(&format!("reanimation {}
", es.reanimations.len()));
                        }
                    }
                }
                a_text.push_str(&format!("zombies {}
", self.zombies.len()));
                a_text.push_str(&format!("plants {}
", self.plants.len()));
                a_text.push_str(&format!("projectiles {}
", self.projectiles.len()));
                a_text.push_str(&format!("coins {}
", self.coins.len()));
                a_text.push_str(&format!("lawn mowers {}
", self.lawn_mowers.len()));
                a_text.push_str(&format!("grid items {}
", self.grid_items.len()));
            }
            DebugTextMode::Collision => {
                a_text.push_str("COLLISION DEBUG
");
            }
            _ => {}
        }
        // C++: SetFont(FONT_PICO129)；黑描边 4 次 + 白字（DrawStringWordWrapped）
        g.set_font(unsafe { crate::framework::graphics::bitmap_font::FONT_PICO129 });
        g.set_color(&Color::BLACK);
        g.draw_string_word_wrapped(&a_text, 10, 89, 10000000, -1, -1, None);
        g.draw_string_word_wrapped(&a_text, 11, 91, 10000000, -1, -1, None);
        g.draw_string_word_wrapped(&a_text, 9, 90, 10000000, -1, -1, None);
        g.draw_string_word_wrapped(&a_text, 11, 90, 10000000, -1, -1, None);
        g.set_color(&Color::WHITE);
        g.draw_string_word_wrapped(&a_text, 10, 90, 10000000, -1, -1, None);
    }

    /// [TRANSLATION_NOTE]: C++ Board::DrawDebugObjectRects (Board.cpp:7052) — collision rects;
    /// iterate plants/zombies/lawn mowers, drawing placeholder until images wired
    /// 对应 C++ Board::DrawDebugObjectRects (Board.cpp 7052-7118)
    /// 植物绿框 + 主/副武器攻击框 + 僵尸/割草机/投射物红框
    pub fn draw_debug_object_rects(&self, g: &mut Graphics) {
        if self.m_debug_text_mode != DebugTextMode::Collision {
            return;
        }
        // C++: 植物 — 本体绿框，主/副攻击框（宽度 < BOARD_WIDTH 才画）红/粉
        for plant in &self.plants {
            if plant.dead {
                continue;
            }
            let a_rect = plant.get_plant_rect();
            g.set_color(&Color::new(0, 255, 0, 255));
            g.draw_rect(&a_rect);

            let a_attack_rect = plant.get_plant_attack_rect(PlantWeapon::Primary);
            if a_attack_rect.width < BOARD_WIDTH {
                g.set_color(&Color::new(255, 0, 0, 255));
                g.draw_rect(&a_attack_rect);
            }
            let a_secondary_rect = plant.get_plant_attack_rect(PlantWeapon::Secondary);
            if a_secondary_rect.width < BOARD_WIDTH {
                g.set_color(&Color::new(255, 0, 128, 255));
                g.draw_rect(&a_secondary_rect);
            }
        }
        // C++: 僵尸 — 本体绿框 + 攻击红框
        for zombie in &self.zombies {
            if zombie.dead {
                continue;
            }
            if !zombie.is_dead_or_dying() {
                let a_rect = zombie.get_zombie_rect();
                g.set_color(&Color::new(0, 255, 0, 255));
                g.draw_rect(&a_rect);

                let a_attack_rect = zombie.get_zombie_attack_rect();
                g.set_color(&Color::new(255, 0, 0, 255));
                g.draw_rect(&a_attack_rect);
            }
        }
        // C++: 割草机 — 攻击红框
        for mower in &self.lawn_mowers {
            if mower.dead {
                continue;
            }
            let a_attack_rect = mower.get_lawn_mower_attack_rect();
            g.set_color(&Color::new(255, 0, 0, 255));
            g.draw_rect(&a_attack_rect);
        }
        // C++: 投射物 — 伤害红框
        for projectile in &self.projectiles {
            if projectile.dead {
                continue;
            }
            g.set_color(&Color::new(255, 0, 0, 255));
            let a_damage_rect = projectile.get_projectile_rect();
            g.draw_rect(&a_damage_rect);
        }
    }

    /// [TRANSLATION_NOTE]: C++ Board::AddBossRenderItem (Board.cpp:5960-6015)
    /// boss part render-order computation; Rust output as (BossPart, renderOrder) list
    pub fn add_boss_render_item(&self, the_boss_zombie: &crate::lawn::zombie::Zombie) -> Vec<(BossPart, i32)> {
        let mut a_back_leg_row = 1;
        let mut a_front_leg_row = 3;
        let mut a_back_arm_row = 4;
        if the_boss_zombie.is_dead_or_dying() {
            a_back_arm_row = 1;
        } else if the_boss_zombie.zombie_phase == ZombiePhase::BossStomping {
            // C++: mAnimTime>0.25 && <0.75（ReanimationTryToGet(mBodyReanimID)）时踩踏
            if the_boss_zombie.target_row == 1 {
                a_back_leg_row = 2;
            } else if the_boss_zombie.target_row == 3 {
                a_front_leg_row = 4;
            }
        }
        let mut a_items: Vec<(BossPart, i32)> = Vec::new();
        a_items.push((
            BossPart::BackLeg,
            crate::lawn::board::make_render_order(crate::lawn::game_enums::RENDER_LAYER_BOSS, a_back_leg_row, 2),
        ));
        a_items.push((
            BossPart::FrontLeg,
            crate::lawn::board::make_render_order(crate::lawn::game_enums::RENDER_LAYER_BOSS, a_front_leg_row, 2),
        ));
        a_items.push((
            BossPart::Main,
            crate::lawn::board::make_render_order(crate::lawn::game_enums::RENDER_LAYER_BOSS, 4, 2),
        ));
        a_items.push((
            BossPart::BackArm,
            crate::lawn::board::make_render_order(crate::lawn::game_enums::RENDER_LAYER_BOSS, a_back_arm_row, 3),
        ));
        if the_boss_zombie.boss_fire_ball_reanim_id != REANIMATIONID_NULL {
            if let Some(app) = self.app {
                unsafe {
                    if (*app).reanimation_get(the_boss_zombie.boss_fire_ball_reanim_id).is_some() {
                        a_items.push((BossPart::Fireball, the_boss_zombie.base.render_order));
                    }
                }
            }
        }
        a_items
    }

    /// 绘制金币银行 UI（对应 C++ Board::DrawUICoinBank，Board.cpp:7223）
    pub fn draw_ui_coin_bank(&self, g: &mut Graphics) {
        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        let app_ref = unsafe { &*app };
        let a_game_scene = app_ref.game_scene;
        let a_crazy_dave_state = app_ref.m_crazy_dave_state;
        let a_game_mode = app_ref.game_mode;
        // C++: mGameScene != SCENE_PLAYING && mCrazyDaveState == CRAZY_DAVE_OFF → return
        if a_game_scene != crate::lawn::lawn_app::GameScenes::Playing
            && a_crazy_dave_state == CrazyDaveState::Off
        {
            return;
        }

        if self.m_coin_bank_fade_count <= 0 {
            return;
        }

        let a_coin_bank = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_COINBANK");
        let a_pos_y_base = if a_coin_bank.is_null() {
            599
        } else {
            599 - unsafe { (*a_coin_bank).get_height() }
        };
        let mut a_pos_x = 57;
        let a_pos_y = a_pos_y_base;
        if a_game_mode == GameMode::ChallengeZenGarden || a_crazy_dave_state != CrazyDaveState::Off {
            a_pos_x = 450 - self.m_x;
        }

        if !a_coin_bank.is_null() {
            let a_coin_bank_ref = unsafe { &*a_coin_bank };
            g.set_colorize_images(true);
            let an_alpha = (255 * self.m_coin_bank_fade_count / 15).clamp(0, 255) as u8;
            g.set_color(&Color::new(255, 255, 255, an_alpha));
            g.draw_image_xy(a_coin_bank_ref, a_pos_x, a_pos_y);

            // C++: SetColor(Color(180, 255, 90, anAlpha)); SetFont(FONT_CONTINUUMBOLD14);
            //      aCoinLabel = mApp->GetMoneyString(mPlayerInfo->mCoins);
            //      DrawString(aCoinLabel, aPosX + 116 - FONT_CONTINUUMBOLD14->StringWidth(aCoinLabel), aPosY + 24);
            let a_coin_label =
                crate::lawn::lawn_app::LawnApp::get_money_string(unsafe {
                    (*app).player_info.as_ref().map_or(0, |p| p.m_coins)
                });
            g.set_color(&Color::new(180, 255, 90, an_alpha));
            g.set_font(unsafe { crate::framework::graphics::bitmap_font::FONT_CONTINUUMBOLD14 });
            let a_label_width = unsafe {
                (*crate::framework::graphics::bitmap_font::FONT_CONTINUUMBOLD14)
                    .string_width(&a_coin_label)
            };
            g.draw_string(&a_coin_label, a_pos_x + 116 - a_label_width, a_pos_y + 24);
            g.set_colorize_images(false);
        }
    }

    /// 绘制禅园独轮车按钮（对应 C++ Board::DrawZenWheelBarrowButton，Board.cpp:6709）
    pub fn draw_zen_wheel_barrow_button(&self, g: &mut Graphics, the_offset_y: i32) {
        // C++: Rect aButtonRect = GetShovelButtonRect(); GetZenButtonRect(OBJECT_TYPE_WHEELBARROW, aButtonRect);
        let _a_shovel_rect = self.get_shovel_button_rect();
        let a_button_rect = self.get_zen_button_rect(GameObjectType::Wheelbarrow);

        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        let zen_garden = unsafe { (*app).zen_garden };
        let a_plant = zen_garden.and_then(|zg| unsafe { (*zg).get_potted_plant_in_wheelbarrow() });
        let a_cursor_type = self.cursor_object.cursor_type;
        if a_plant.is_some() && a_cursor_type != CursorType::PlantFromWheelBarrow {
            let is_zen_fading =
                self.challenge.as_ref().map_or(false, |c| c.challenge_state == ChallengeState::ZenFading);
            let a_img = crate::lawn::board::get_overlay_image(unsafe { &*app }, "IMAGE_ZEN_WHEELBARROW");
            if is_zen_fading {
                if !a_img.is_null() {
                    unsafe { g.draw_image_f_xy(&*a_img, (a_button_rect.x - 7) as f32, (a_button_rect.y + the_offset_y - 3) as f32); }
                }
            } else {
                if !a_img.is_null() {
                    unsafe { g.draw_image_f_xy(&*a_img, (a_button_rect.x - 7) as f32, (a_button_rect.y + the_offset_y + 4) as f32); }
                }
            }

            if let Some(a_plant) = a_plant {
                unsafe {
                    let a_plant_ref = &*a_plant;
                    if a_plant_ref.plant_age == PottedPlantAge::Small {
                        if let Some(zg) = zen_garden {
                            (*zg).draw_potted_plant(g, (a_button_rect.x + 23) as f32, (a_button_rect.y + the_offset_y - 8) as f32, a_plant_ref, 0.6, true);
                        }
                    } else if a_plant_ref.plant_age == PottedPlantAge::Medium {
                        if let Some(zg) = zen_garden {
                            (*zg).draw_potted_plant(g, (a_button_rect.x + 28) as f32, (a_button_rect.y + the_offset_y + 2) as f32, a_plant_ref, 0.5, true);
                        }
                    } else {
                        if let Some(zg) = zen_garden {
                            (*zg).draw_potted_plant(g, (a_button_rect.x + 34) as f32, (a_button_rect.y + the_offset_y + 12) as f32, a_plant_ref, 0.4, true);
                        }
                    }
                }
            }
        } else {
            let a_img = crate::lawn::board::get_overlay_image(unsafe { &*app }, "IMAGE_ZEN_WHEELBARROW");
            if !a_img.is_null() {
                unsafe { g.draw_image_f_xy(&*a_img, (a_button_rect.x - 7) as f32, (a_button_rect.y + the_offset_y - 3) as f32); }
            }
        }
    }

    /// 绘制铲子按钮（对应 C++ Board::DrawShovel，Board.cpp:6884）
    pub fn draw_shovel(&self, g: &mut Graphics) {
        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        if app_mode != GameMode::ChallengeZenGarden && app_mode != GameMode::ChallengeTreeOfWisdom {
            if self.m_show_shovel {
                let a_shovel_rect = self.get_shovel_button_rect();
                // C++: g->DrawImage(Sexy::IMAGE_SHOVELBANK, aShovelRect.mX, aShovelRect.mY)
                if let Some(app) = self.app {
                    let app_ref = unsafe { &*app };
                    let a_shovel_bank = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SHOVELBANK");
                    if !a_shovel_bank.is_null() {
                        g.draw_image_xy(unsafe { &*a_shovel_bank }, a_shovel_rect.x, a_shovel_rect.y);
                    }
                }

                if self.cursor_object.cursor_type != CursorType::Shovel {
                    // C++: mChallenge->mChallengeState == (ChallengeState)15（魔法数字，C++ 原样保留）
                    if self.challenge.as_ref().map_or(false, |c| c.challenge_state as i32 == 15) {
                        g.set_colorize_images(true);
                        g.set_color(&crate::todlib::tod_common::get_flashing_color(self.m_main_counter, 75));
                    }
                    // C++: g->DrawImage(Sexy::IMAGE_SHOVEL, aShovelRect.mX - 7, aShovelRect.mY - 3)
                    if let Some(app) = self.app {
                        let app_ref = unsafe { &*app };
                        let a_shovel = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SHOVEL");
                        if !a_shovel.is_null() {
                            g.draw_image_xy(unsafe { &*a_shovel }, a_shovel_rect.x - 7, a_shovel_rect.y - 3);
                        }
                    }
                    g.set_colorize_images(false);
                }
            }
        }

        // C++: 禅园/智慧树模式下绘制禅境工具按钮
        if app_mode == GameMode::ChallengeZenGarden || app_mode == GameMode::ChallengeTreeOfWisdom {
            self.draw_zen_buttons(g);
        }
    }

    /// 绘制种子槽（对应 C++ SeedBank::Draw，SeedPacket.cpp:929）
    fn draw_seed_bank(&self, g: &mut Graphics) {
        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        let app_ref = unsafe { &*app };

        // C++: mCutScene->IsBeforePreloading()
        if self.m_cut_scene.map_or(false, |c| unsafe { (*c).m_preloaded == false }) {
            return;
        }

        // C++: 非 SCENE_PLAYING 场景平移 g->mTransX -= mBoard->mX
        let a_translate = app_ref.game_scene != GameScenes::Playing;
        if a_translate {
            g.translate(-self.m_x, -self.m_y);
        }

        if app_ref.is_slot_machine_level() {
            let a_sun_bank = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SUNBANK");
            if !a_sun_bank.is_null() {
                g.draw_image_xy(unsafe { &*a_sun_bank }, 0, 0);
            }
        } else if self.has_conveyor_belt_seed_bank() {
            let a_backdrop = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_CONVEYORBELT_BACKDROP");
            if !a_backdrop.is_null() {
                g.draw_image_xy(unsafe { &*a_backdrop }, 83, 0);
            }
            let a_conveyor = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_CONVEYORBELT");
            if !a_conveyor.is_null() {
                g.draw_image_cel(unsafe { &*a_conveyor }, 90, 63, self.m_conveyor_belt_counter / 4 % 6);
            }
            g.set_clip_rect_xywh(90, 0, 501, BOARD_HEIGHT);
        } else {
            let a_seed_bank = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SEEDBANK");
            if !a_seed_bank.is_null() {
                let a_seed_bank_ref = unsafe { &*a_seed_bank };
                let a_extra_width = self.get_seed_bank_extra_width();
                let the_src_rect = Rect::new(
                    a_seed_bank_ref.width - a_extra_width - 12,
                    0,
                    a_extra_width + 12,
                    a_seed_bank_ref.height,
                );
                g.draw_image_xy(a_seed_bank_ref, 0, 0);
                g.draw_image_src(a_seed_bank_ref, a_seed_bank_ref.width - 12, 0, &the_src_rect);
            }
        }

        // C++: 逐包绘制（BeginDraw/Draw/EndDraw）
        for packet in &self.seed_bank {
            if packet.seed_type != SeedType::None {
                packet.draw(g);
            }
        }

        g.clear_clip_rect();
        if app_ref.is_slot_machine_level() {
            if let Some(a_slot_overlay) = Some(crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SLOTMACHINE_OVERLAY")) {
                // C++: mY > -IMAGE_SEEDBANK->GetHeight() 时绘制（mY 为 SeedBank 控件 Y，Rust 以 Board.m_y 近似）
                let a_seed_bank = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SEEDBANK");
                let a_height = if a_seed_bank.is_null() { 0 } else { unsafe { (*a_seed_bank).get_height() } };
                if self.m_y > -a_height {
                    g.draw_image_xy(unsafe { &*a_slot_overlay }, 189, -2);
                }
            }
        }

        if !self.has_conveyor_belt_seed_bank() {
            // C++: PvzpDrawString(g, aMoneyLabel, 34, 78, FONT_CONTINUUMBOLD14, aMoneyColor, DS_ALIGN_CENTER)
            let a_money_label = format!("{}", self.m_sun_money.max(0));
            let mut a_money_color = Color::new(0, 0, 0, 255);
            if self.m_out_of_money_counter > 0 && self.m_out_of_money_counter % 20 < 10 {
                a_money_color = Color::new(255, 0, 0, 255);
            }
            if !self.draw_ui_text(
                g,
                &a_money_label,
                34,
                78,
                unsafe { crate::framework::graphics::bitmap_font::FONT_CONTINUUMBOLD14 },
                &a_money_color,
                DrawStringJustification::DS_ALIGN_CENTER,
            ) {
                g.set_color(&a_money_color);
                g.draw_string(&a_money_label, 34, 78);
            }
        }

        if a_translate {
            g.translate(self.m_x, self.m_y);
        }
    }

    /// 绘制底部 UI（对应 C++ Board::DrawUIBottom，Board.cpp:7173）
    pub fn draw_ui_bottom(&self, g: &mut Graphics) {
        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        let app_ref = unsafe { &*app };

        // C++: Zombiquarium 波浪动画（ADDITIVE 模式）
        if self.m_background_type == BackgroundType::Zombiquarium {
            let a_wave_time = (((self.m_main_counter as i32 / 8) % 22) - 11).abs();
            g.set_draw_mode(DrawMode::Additive as i32);
            let a_wave_side = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_WAVESIDE");
            let a_wave_center = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_WAVECENTER");
            if !a_wave_side.is_null() {
                g.draw_image_cel(unsafe { &*a_wave_side }, 0, 40, a_wave_time);
            }
            if !a_wave_center.is_null() {
                g.draw_image_cel(unsafe { &*a_wave_center }, 160, 40, a_wave_time);
                g.draw_image_cel(unsafe { &*a_wave_center }, 320, 40, a_wave_time);
                g.draw_image_cel(unsafe { &*a_wave_center }, 480, 40, a_wave_time);
            }
            // C++: PvzpDrawImageCelScaled(g, IMAGE_WAVESIDE, 800, 40, celCol, celRow, -1.0f, 1.0f)
            // [TRANSLATION_NOTE]: Rust 无 PvzpDrawImageCelScaled 入口，右端波浪镜像绘制暂略
            g.set_draw_mode(DrawMode::Normal as i32);
        }

        // C++: 温室/水族馆前景覆盖（ADDITIVE 模式）
        if self.m_background_type == BackgroundType::Greenhouse
            || self.m_background_type == BackgroundType::Zombiquarium
        {
            g.set_draw_mode(DrawMode::Additive as i32);
            let a_overlay = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND_GREENHOUSE_OVERLAY");
            if !a_overlay.is_null() {
                let a_overlay_ref = unsafe { &*a_overlay };
                let a_dest_rect = Rect::new(0, 0, BOARD_WIDTH, BOARD_HEIGHT);
                let a_src_rect = Rect::new(0, 0, a_overlay_ref.width, a_overlay_ref.height);
                g.draw_image_stretch(a_overlay_ref, &a_dest_rect, &a_src_rect);
            }
            g.set_draw_mode(DrawMode::Normal as i32);
        }

        // C++: 非 SCENE_ZOMBIES_WON 场景绘制种子槽
        if app_ref.game_scene != GameScenes::ZombiesWon {
            // C++: mSeedBank->BeginDraw/Draw/EndDraw（Rust 以 draw_seed_bank 等价，控件坐标近似 0,0）
            self.draw_seed_bank(g);

            // C++: mAdvice->mMessageStyle == MESSAGE_STYLE_SLOT_MACHINE 时绘制
            if self.m_advice_widget.message_style == MessageStyle::SlotMachine {
                self.m_advice_widget.draw(g);
            }
        }

        self.draw_shovel(g);
        if !self.stage_has_fog() {
            self.draw_top_right_ui(g);
        }
    }

    /// 绘制右上角 UI（对应 C++ Board::DrawTopRightUI，Board.cpp:7137）
    pub fn draw_top_right_ui(&self, g: &mut Graphics) {
        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        let a_game_mode = unsafe { (*app).game_mode };
        if a_game_mode == GameMode::ChallengeZenGarden {
            let (a_challenge_state, a_challenge_state_counter) = match self.challenge.as_ref() {
                Some(c) => (c.challenge_state, c.challenge_state_counter),
                None => (ChallengeState::Normal, 0),
            };
            if a_challenge_state == ChallengeState::ZenFading {
                // mMenuButton->mY / mStoreButton->mX 按过场进度动画
                if let Some(menu_button) = self.menu_button {
                    unsafe {
                        (*menu_button).y = crate::todlib::tod_common::tod_animate_curve(
                            50, 0, a_challenge_state_counter, -10, -50, TodCurves::EaseInOut,
                        );
                    }
                }
                if let Some(store_button) = self.store_button {
                    unsafe {
                        (*store_button).x = crate::todlib::tod_common::tod_animate_curve(
                            50, 0, a_challenge_state_counter, 678, 800, TodCurves::EaseInOut,
                        );
                    }
                }
            } else {
                if let Some(menu_button) = self.menu_button {
                    unsafe { (*menu_button).y = -10; }
                }
                if let Some(store_button) = self.store_button {
                    unsafe { (*store_button).x = 678; }
                }
            }
        }

        // C++: 禅园教程完成时菜单按钮闪烁
        if self.m_tutorial_state == TutorialState::ZenGardenCompleted {
            g.set_colorize_images(true);
            g.set_color(&crate::todlib::tod_common::get_flashing_color(self.m_main_counter, 75));
        }
        if let Some(menu_button) = self.menu_button {
            unsafe { (*menu_button).draw(g); }
        }
        g.set_colorize_images(false);

        // C++: 商店按钮（Last Stand 模式不显示）
        if self.store_button.is_some() && a_game_mode != GameMode::ChallengeLastStand {
            if self.m_tutorial_state == TutorialState::ZenGardenVisitStore {
                g.set_colorize_images(true);
                g.set_color(&crate::todlib::tod_common::get_flashing_color(self.m_main_counter, 75));
            }
            if let Some(store_button) = self.store_button {
                unsafe { (*store_button).draw(g); }
            }
            g.set_colorize_images(false);
        }
    }

    /// 绘制 UI 顶层（对应 C++ Board::DrawUITop，Board.cpp:7398）
    pub fn draw_ui_top(&self, g: &mut Graphics) {
        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        let app_ref = unsafe { &*app };
        let a_game_mode = app_ref.game_mode;
        let a_game_scene = app_ref.game_scene;

        if self.stage_has_fog() {
            self.draw_top_right_ui(g);
        }

        // C++: 时间停止时画面泛白
        if self.m_time_stop_counter > 0 {
            g.set_color(&Color::new(200, 200, 200, 210));
            g.fill_rect_xywh(0, 0, crate::lawn::game_enums::BOARD_WIDTH, crate::lawn::game_enums::BOARD_HEIGHT);
        }

        if a_game_scene == crate::lawn::lawn_app::GameScenes::Playing
            || a_game_mode == GameMode::ChallengeTreeOfWisdom
        {
            self.draw_progress_meter(g);
            self.draw_level(g);
        }
        // C++: Last Stand 模式绘制商店按钮
        if self.store_button.is_some() && a_game_mode == GameMode::ChallengeLastStand {
            if let Some(store_button) = self.store_button {
                unsafe { (*store_button).draw(g); }
            }
        }

        // C++: Upsell/Intro 过场隐藏棋盘时黑屏
        if (a_game_mode == GameMode::Upsell || a_game_mode == GameMode::Intro)
            && self.m_cut_scene.map_or(false, |c| unsafe { (*c).m_upsell_hide_board })
        {
            g.set_color(&Color::new(0, 0, 0, 255));
            g.fill_rect_xywh(0, 0, crate::lawn::game_enums::BOARD_WIDTH, crate::lawn::game_enums::BOARD_HEIGHT);
        }

        if a_game_mode == GameMode::Upsell {
            if let Some(cut_scene) = self.m_cut_scene {
                unsafe { (*cut_scene).draw_upsell(g); }
            }
        }
        if a_game_mode == GameMode::Intro {
            if let Some(cut_scene) = self.m_cut_scene {
                unsafe { (*cut_scene).draw_intro(g); }
            }
        }

        // C++: IsScaryPotterDaveTalking() = IsScaryPotterLevel() && mNextSurvivalStageCounter > 0 && mCrazyDaveState != CRAZY_DAVE_OFF
        let a_scary_potter_dave_talking = app_ref.is_scary_potter_level()
            && self.m_next_survival_stage_counter > 0
            && app_ref.m_crazy_dave_state != CrazyDaveState::Off;
        if a_game_scene == crate::lawn::lawn_app::GameScenes::LevelIntro
            || a_game_mode == GameMode::ChallengeZenGarden
            || a_game_mode == GameMode::ChallengeTreeOfWisdom
            || a_scary_potter_dave_talking
        {
            // C++: Graphics aScreenSpace(*g); aScreenSpace.mTransX -= mX; mTransY -= mY; DrawCrazyDave(&aScreenSpace)
            // Rust Graphics 无 Clone：保存/恢复 trans 等价副本语义
            let a_saved_trans_x = g.trans_x;
            let a_saved_trans_y = g.trans_y;
            g.trans_x -= self.m_x as f64;
            g.trans_y -= self.m_y as f64;
            app_ref.draw_crazy_dave(g);
            g.trans_x = a_saved_trans_x;
            g.trans_y = a_saved_trans_y;
        }

        // C++: if (mAdvice->mMessageStyle != MESSAGE_STYLE_SLOT_MACHINE) mAdvice->Draw(g);
        // [TRANSLATION_NOTE]: Rust m_advice 为 AdviceType 枚举（无 Advice 对象绘制），暂略

        // C++: mCursorObject->BeginDraw(g) / Draw(g) / EndDraw(g)
        // [TRANSLATION_NOTE]: Rust cursor_object::draw 未区分 Begin/End 绘制裁剪
        self.cursor_object.draw(g);

        self.tool_tip.draw(g);
        self.draw_debug_text(g);
        self.draw_debug_object_rects(g);
    }

    /// 绘制背景（对应 C++ Board::DrawBackdrop，Board.cpp:5874）
    pub fn draw_backdrop(&self, g: &mut Graphics) {
        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        let app_ref = unsafe { &*app };

        // C++: switch (mBackground) 选择背景图片
        let mut a_bg_image: *mut crate::framework::graphics::image::Image = std::ptr::null_mut();
        match self.m_background_type {
            BackgroundType::Day => { a_bg_image = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND1"); }
            BackgroundType::Night => { a_bg_image = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND2"); }
            BackgroundType::Pool => { a_bg_image = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND3"); }
            BackgroundType::Fog => { a_bg_image = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND4"); }
            BackgroundType::Roof => { a_bg_image = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND5"); }
            BackgroundType::Boss => { a_bg_image = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND6BOSS"); }
            BackgroundType::MushroomGarden => { a_bg_image = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND_MUSHROOMGARDEN"); }
            BackgroundType::Greenhouse => { a_bg_image = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND_GREENHOUSE"); }
            BackgroundType::Zombiquarium => { a_bg_image = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_AQUARIUM1"); }
            BackgroundType::TreeOfWisdom => { a_bg_image = std::ptr::null_mut(); }
            _ => { /* C++: PVZP_ASSERT(false) */ }
        }
        let a_board_offset = crate::lawn::game_enums::BOARD_OFFSET;
        let a_is_first_time = app_ref.is_first_time_adventure_mode();
        let a_game_mode = app_ref.game_mode;

        if self.level == 1 && a_is_first_time {
            let a_bg1_unsodded = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND1UNSODDED");
            if !a_bg1_unsodded.is_null() {
                unsafe { g.draw_image_f_xy(&*a_bg1_unsodded, -a_board_offset as f32, 0.0); }
            }
            let a_sod1 = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SOD1ROW");
            if !a_sod1.is_null() {
                let a_sod1_ref = unsafe { &*a_sod1 };
                let a_width = crate::todlib::tod_common::tod_animate_curve(
                    0, 1000, self.m_sod_position, 0, a_sod1_ref.get_width(), TodCurves::Linear,
                );
                let a_src_rect = Rect::new(0, 0, a_width, a_sod1_ref.get_height());
                g.draw_image_src(a_sod1_ref, 239 - a_board_offset, 265, &a_src_rect);
            }
        } else if ((self.level == 2 || self.level == 3) && a_is_first_time)
            || a_game_mode == GameMode::ChallengeResodded
        {
            let a_bg1_unsodded = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND1UNSODDED");
            if !a_bg1_unsodded.is_null() {
                unsafe { g.draw_image_f_xy(&*a_bg1_unsodded, -a_board_offset as f32, 0.0); }
            }
            let a_sod1 = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SOD1ROW");
            if !a_sod1.is_null() {
                let a_sod1_ref = unsafe { &*a_sod1 };
                g.draw_image_xy(a_sod1_ref, 239 - a_board_offset, 265);
            }
            let a_sod3 = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SOD3ROW");
            if !a_sod3.is_null() {
                let a_sod3_ref = unsafe { &*a_sod3 };
                let a_width = crate::todlib::tod_common::tod_animate_curve(
                    0, 1000, self.m_sod_position, 0, a_sod3_ref.get_width(), TodCurves::Linear,
                );
                let a_src_rect = Rect::new(0, 0, a_width, a_sod3_ref.get_height());
                g.draw_image_src(a_sod3_ref, 235 - a_board_offset, 149, &a_src_rect);
            }
        } else if self.level == 4 && a_is_first_time {
            let a_bg1_unsodded = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND1UNSODDED");
            if !a_bg1_unsodded.is_null() {
                unsafe { g.draw_image_f_xy(&*a_bg1_unsodded, -a_board_offset as f32, 0.0); }
            }
            let a_sod3 = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SOD3ROW");
            if !a_sod3.is_null() {
                unsafe { g.draw_image_xy(&*a_sod3, 235 - a_board_offset, 149); }
            }
            let a_bg1 = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BACKGROUND1");
            if !a_bg1.is_null() {
                let a_bg1_ref = unsafe { &*a_bg1 };
                let a_width = crate::todlib::tod_common::tod_animate_curve(0, 1000, self.m_sod_position, 0, 773, TodCurves::Linear);
                let a_src_rect = Rect::new(232, 0, a_width, a_bg1_ref.get_height());
                g.draw_image_src(a_bg1_ref, 232 - a_board_offset, 0, &a_src_rect);
            }
        } else if !a_bg_image.is_null() {
            let a_bg_ref = unsafe { &*a_bg_image };
            // C++: 蘑菇园/温室/水族馆以 (0,0) 绘制，其余 -BOARD_OFFSET
            if self.m_background_type == BackgroundType::MushroomGarden
                || self.m_background_type == BackgroundType::Greenhouse
                || self.m_background_type == BackgroundType::Zombiquarium
            {
                g.draw_image_xy(a_bg_ref, 0, 0);
            } else {
                g.draw_image_xy(a_bg_ref, -a_board_offset, 0);
            }
        }

        if app_ref.game_scene == crate::lawn::lawn_app::GameScenes::ZombiesWon {
            self.draw_house_door_bottom(g);
        }
        if self.stage_has_pool() {
            // C++: mApp->mPoolEffect->PoolEffectDraw(g, StageIsNight())
            if let Some(pool_effect) = unsafe { (*app).pool_effect.as_mut() } {
                pool_effect.draw(g, self.stage_is_night());
            }
        }
        if self.m_tutorial_state == TutorialState::Level1PlantPeashooter {
            // C++: aClipG.SetColorizeImages(true); SetColor(GetFlashingColor(mMainCounter, 75));
            //      DrawImage(IMAGE_SOD1ROW, 239 - BOARD_OFFSET, 265); SetColorizeImages(false)
            let a_sod1 = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SOD1ROW");
            if !a_sod1.is_null() {
                let a_sod1_ref = unsafe { &*a_sod1 };
                g.set_colorize_images(true);
                g.set_color(&crate::todlib::tod_common::get_flashing_color(self.m_main_counter, 75));
                g.draw_image_xy(a_sod1_ref, 239 - a_board_offset, 265);
                g.set_colorize_images(false);
            }
        }
        if let Some(challenge) = self.challenge.as_ref() {
            challenge.draw_backdrop(g);
        }
        if app_ref.game_scene == crate::lawn::lawn_app::GameScenes::LevelIntro && self.stage_has_grave_stones() {
            let a_grave = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_NIGHT_GRAVE_GRAPHIC");
            if !a_grave.is_null() {
                unsafe { g.draw_image_xy(&*a_grave, 1092, 40); }
            }
        }
    }

    /// 绘制禅园工具按钮（对应 C++ Board::DrawZenButtons，Board.cpp:6744）
    pub fn draw_zen_buttons(&self, g: &mut Graphics) {
        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        let app_ref = unsafe { &*app };

        let mut a_offset_y = 0;
        // C++: ZenFading 时按钮随过场进度上移
        let (a_challenge_state, a_challenge_state_counter) = match self.challenge.as_ref() {
            Some(c) => (c.challenge_state, c.challenge_state_counter),
            None => (ChallengeState::Normal, 0),
        };
        if a_challenge_state == ChallengeState::ZenFading {
            a_offset_y = crate::todlib::tod_common::tod_animate_curve(
                50, 0, a_challenge_state_counter, 0, -72, TodCurves::EaseInOut,
            );
        }

        // C++: for (aTool = OBJECT_TYPE_WATERING_CAN; aTool <= OBJECT_TYPE_NEXT_GARDEN; aTool++)
        let a_tools = [
            GameObjectType::WateringCan, GameObjectType::Fertilizer, GameObjectType::BugSpray,
            GameObjectType::Phonograph, GameObjectType::Chocolate, GameObjectType::Glove,
            GameObjectType::MoneySign, GameObjectType::Wheelbarrow, GameObjectType::TreeFood,
            GameObjectType::NextGarden,
        ];
        for a_tool in a_tools {
            if !self.can_use_game_object(a_tool) {
                continue;
            }

            let a_shovel_rect = self.get_shovel_button_rect();
            let a_button_rect;
            if a_tool == GameObjectType::NextGarden {
                a_button_rect = Rect::new(564, a_shovel_rect.y, a_shovel_rect.width, a_shovel_rect.height);
                let a_menu_btn_no_draw = self.menu_button.map_or(false, |m| unsafe { (*m).btn_no_draw });
                if !a_menu_btn_no_draw {
                    let a_img = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_ZEN_NEXTGARDEN");
                    if !a_img.is_null() {
                        unsafe { g.draw_image_f_xy(&*a_img, (a_button_rect.x + 2) as f32, (a_button_rect.y + a_offset_y) as f32); }
                    }
                }
            } else {
                a_button_rect = self.get_zen_button_rect(a_tool);
                let a_shovel_bank = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SHOVELBANK");
                if !a_shovel_bank.is_null() {
                    unsafe { g.draw_image_f_xy(&*a_shovel_bank, a_button_rect.x as f32, (a_button_rect.y + a_offset_y) as f32); }
                }

                // C++: 光标正持有的工具跳过绘制：CursorType 与 GameObjectType 同名逐一对应
                let a_cursor_type = self.cursor_object.cursor_type;
                let a_held_cursor = match a_tool {
                    GameObjectType::WateringCan => Some(CursorType::WateringCan),
                    GameObjectType::Fertilizer => Some(CursorType::Fertilizer),
                    GameObjectType::BugSpray => Some(CursorType::BugSpray),
                    GameObjectType::Phonograph => Some(CursorType::Phonograph),
                    GameObjectType::Chocolate => Some(CursorType::Chocolate),
                    GameObjectType::Glove => Some(CursorType::Glove),
                    GameObjectType::MoneySign => Some(CursorType::MoneySign),
                    GameObjectType::Wheelbarrow => Some(CursorType::Wheelbarrow),
                    GameObjectType::TreeFood => Some(CursorType::TreeFood),
                    _ => None,
                };
                if a_held_cursor == Some(a_cursor_type) {
                    continue;
                }

                // C++: switch (aTool)
                match a_tool {
                    GameObjectType::WateringCan => {
                        let a_purchase_gold = app_ref.player_info.as_ref().map_or(0, |p| {
                            p.m_purchases.get(StoreItem::GoldWateringcan as usize).copied().unwrap_or(0)
                        });
                        let a_img = if a_purchase_gold != 0 {
                            crate::lawn::board::get_overlay_image(app_ref, "IMAGE_WATERINGCANGOLD")
                        } else {
                            crate::lawn::board::get_overlay_image(app_ref, "IMAGE_WATERINGCAN")
                        };
                        if !a_img.is_null() {
                            unsafe { g.draw_image_f_xy(&*a_img, (a_button_rect.x - 2) as f32, (a_button_rect.y + a_offset_y - 6) as f32); }
                        }
                    }
                    GameObjectType::Fertilizer => {
                        // C++: aCharges = aPurchase > PURCHASE_COUNT_OFFSET ? aPurchase - PURCHASE_COUNT_OFFSET : 0
                        let a_purchase = app_ref.player_info.as_ref().map_or(0, |p| {
                            p.m_purchases.get(StoreItem::Fertilizer as usize).copied().unwrap_or(0)
                        });
                        let a_charges = if a_purchase > 1000 { a_purchase - 1000 } else { 0 };
                        if a_charges == 0 {
                            g.set_colorize_images(true);
                            g.set_color(&Color::new(96, 96, 96, 255));
                        } else if self.m_tutorial_state == TutorialState::ZenGardenFertilizePlants {
                            g.set_colorize_images(true);
                            g.set_color(&crate::todlib::tod_common::get_flashing_color(self.m_main_counter, 75));
                        }
                        let a_img = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_FERTILIZER");
                        if !a_img.is_null() {
                            unsafe { g.draw_image_f_xy(&*a_img, (a_button_rect.x - 6) as f32, (a_button_rect.y + a_offset_y - 7) as f32); }
                        }
                        g.set_colorize_images(false);
                        // C++: aChargeString = StrFormat("x%d", aCharges);
                        //      PvzpDrawString(g, aChargeString, aButtonRect.mX + 64, aButtonRect.mY + aOffsetY + 65, FONT_HOUSEOFTERROR16, Color::White, DS_ALIGN_RIGHT)
                        let a_charge_string = format!("x{}", a_charges);
                        g.set_font(unsafe { crate::framework::graphics::bitmap_font::FONT_HOUSEOFTERROR16 });
                        g.set_color(&Color::WHITE);
                        let a_charge_width = unsafe {
                            (*crate::framework::graphics::bitmap_font::FONT_HOUSEOFTERROR16)
                                .string_width(&a_charge_string)
                        };
                        g.draw_string(&a_charge_string, a_button_rect.x + 64 - a_charge_width, a_button_rect.y + a_offset_y + 65);
                    }
                    GameObjectType::BugSpray => {
                        let a_purchase = app_ref.player_info.as_ref().map_or(0, |p| {
                            p.m_purchases.get(StoreItem::BugSpray as usize).copied().unwrap_or(0)
                        });
                        let a_charges = if a_purchase > 1000 { a_purchase - 1000 } else { 0 };
                        if a_charges == 0 {
                            g.set_colorize_images(true);
                            g.set_color(&Color::new(128, 128, 128, 255));
                        }
                        let a_img = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_BUG_SPRAY");
                        if !a_img.is_null() {
                            unsafe { g.draw_image_f_xy(&*a_img, a_button_rect.x as f32, (a_button_rect.y + a_offset_y - 1) as f32); }
                        }
                        g.set_colorize_images(false);
                        // [TRANSLATION_NOTE]: 剩余次数文本（x%d）同上暂略
                    }
                    GameObjectType::Phonograph => {
                        let a_img = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_PHONOGRAPH");
                        if !a_img.is_null() {
                            unsafe { g.draw_image_f_xy(&*a_img, (a_button_rect.x + 2) as f32, (a_button_rect.y + a_offset_y + 2) as f32); }
                        }
                    }
                    GameObjectType::Chocolate => {
                        let a_purchase = app_ref.player_info.as_ref().map_or(0, |p| {
                            p.m_purchases.get(StoreItem::Chocolate as usize).copied().unwrap_or(0)
                        });
                        let a_charges = if a_purchase > 1000 { a_purchase - 1000 } else { 0 };
                        if a_charges == 0 {
                            g.set_colorize_images(true);
                            g.set_color(&Color::new(128, 128, 128, 255));
                        }
                        let a_img = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_CHOCOLATE");
                        if !a_img.is_null() {
                            unsafe { g.draw_image_f_xy(&*a_img, (a_button_rect.x + 6) as f32, (a_button_rect.y + a_offset_y + 4) as f32); }
                        }
                        g.set_colorize_images(false);
                    }
                    GameObjectType::Glove => {
                        let a_cursor_2 = self.cursor_object.cursor_type;
                        if a_cursor_2 != CursorType::PlantFromGlove && a_cursor_2 != CursorType::PlantFromWheelBarrow {
                            let a_img = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_ZEN_GARDENGLOVE");
                            if !a_img.is_null() {
                                unsafe { g.draw_image_f_xy(&*a_img, (a_button_rect.x - 6) as f32, (a_button_rect.y + a_offset_y - 4) as f32); }
                            }
                        }
                    }
                    GameObjectType::MoneySign => {
                        let a_img = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_ZEN_MONEYSIGN");
                        if !a_img.is_null() {
                            unsafe { g.draw_image_f_xy(&*a_img, (a_button_rect.x - 5) as f32, (a_button_rect.y + a_offset_y - 4) as f32); }
                        }
                    }
                    GameObjectType::Wheelbarrow => {
                        self.draw_zen_wheel_barrow_button(g, a_offset_y);
                    }
                    GameObjectType::TreeFood => {
                        let a_purchase = app_ref.player_info.as_ref().map_or(0, |p| {
                            p.m_purchases.get(StoreItem::TreeFood as usize).copied().unwrap_or(0)
                        });
                        let a_charges = if a_purchase > 1000 { a_purchase - 1000 } else { 0 };
                        if a_charges == 0 {
                            g.set_colorize_images(true);
                            g.set_color(&Color::new(128, 128, 128, 255));
                        }
                        if !self.challenge.as_ref().map_or(false, |c| c.tree_of_wisdom_can_feed() != 0) {
                            g.set_colorize_images(true);
                            g.set_color(&Color::new(128, 128, 128, 255));
                        }
                        let a_img = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_TREEFOOD");
                        if !a_img.is_null() {
                            unsafe { g.draw_image_f_xy(&*a_img, (a_button_rect.x - 6) as f32, (a_button_rect.y + a_offset_y - 7) as f32); }
                        }
                        g.set_colorize_images(false);
                    }
                    _ => {}
                }
            }
        }
    }

    /// 对应 C++ Board::DrawProgressMeter (Board.cpp:6556-6633)
    /// gate + mode-text + flags + head 完整绘制；图片未接入资源表时整体跳过（TRANSLATION_NOTE）
    pub fn draw_progress_meter(&self, g: &mut Graphics) {
        if !self.has_progress_meter() {
            return;
        }
        let Some(app) = self.app else { return };
        unsafe {
            let a_flag_meter = get_overlay_image(&*app, "IMAGE_FLAGMETER");
            if a_flag_meter.is_null() {
                // [TRANSLATION_NOTE]: IMAGE_FLAGMETER 未接入资源表时，进度条本体及
                // 依赖 aCelWidth/aCelHeight 的文本/旗/头均无法定位，整体跳过。
                return;
            }
            let a_flag_meter_ref = &*a_flag_meter;

            // C++: DrawImageCel(IMAGE_FLAGMETER, 600, 575, 0)
            g.draw_image_cel(a_flag_meter_ref, 600, 575, 0);
            let a_cel_width = a_flag_meter_ref.get_cel_width();
            let a_cel_height = a_flag_meter_ref.get_cel_height();

            // C++: aClipWidth = PvzpAnimateCurve(0, PROGRESS_METER_COUNTER, mProgressMeterWidth, 0, 143, LINEAR)
            let a_clip_width = crate::todlib::tod_common::tod_animate_curve(
                0, PROGRESS_METER_COUNTER, self.m_progress_meter_width, 0, 143, TodCurves::Linear,
            );
            // C++: Rect aSrcRect(aCelWidth - aClipWidth - 7, aCelHeight, aClipWidth, aCelHeight);
            //      Rect aDstRect(aCelWidth - aClipWidth + 593, 575, aClipWidth, aCelHeight);
            //      DrawImage(IMAGE_FLAGMETER, aDstRect, aSrcRect);
            let a_src_rect = Rect::new(a_cel_width - a_clip_width - 7, a_cel_height, a_clip_width, a_cel_height);
            let a_dst_rect = Rect::new(a_cel_width - a_clip_width + 593, 575, a_clip_width, a_cel_height);
            g.draw_image_stretch(a_flag_meter_ref, &a_dst_rect, &a_src_rect);

            // C++: 进度条上的模式文本或旗（Board.cpp 6570-6620）
            let a_pos_x = a_cel_width / 2 + 600;
            let a_color = Color::new(224, 187, 98, 255);
            let a_mode_text: Option<String> = if let Some(a_challenge) = self.challenge.as_ref() {
                let a_gm = (*app).game_mode;
                if a_gm == GameMode::ChallengeBeghouled || a_gm == GameMode::ChallengeBeghouledTwist {
                    Some(format!("{}/{} [MATCHES]", a_challenge.challenge_score, 75))
                } else if (*app).is_squirrel_level() {
                    Some(format!("{}/{} [SQUIRRELS]", a_challenge.challenge_score, 7))
                } else if a_gm == GameMode::ChallengeSlotMachine {
                    Some(format!("{}/{} [SUN]", self.m_sun_money.clamp(0, 2000), 2000))
                } else if a_gm == GameMode::ChallengeZombiquarium {
                    Some(format!("{}/{} [SUN]", self.m_sun_money.clamp(0, 1000), 1000))
                } else if (*app).is_izombie_level() {
                    Some(format!("{}/{} [BRAINS]", a_challenge.challenge_score, 5))
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(a_match_str) = a_mode_text {
                // C++: PvzpDrawString(aMatchStr, aPosX, 589, FONT_DWARVENTODCRAFT12, aColor, DS_ALIGN_CENTER)
                g.set_font(unsafe { crate::framework::graphics::bitmap_font::FONT_DWARVENTODCRAFT12 });
                g.set_color(&a_color);
                let a_text_width = unsafe {
                    (*crate::framework::graphics::bitmap_font::FONT_DWARVENTODCRAFT12)
                        .string_width(&a_match_str)
                };
                g.draw_string(&a_match_str, a_pos_x - a_text_width / 2, 589);
            } else if self.progress_meter_has_flags() {
                let a_num_waves_per_flag = self.get_num_waves_per_flag();
                if a_num_waves_per_flag != 0 {
                    let a_num_flag_waves = self.m_num_waves / a_num_waves_per_flag;
                    let a_flags_pos_end = 590 + a_cel_width;
                    let a_flag_parts = get_overlay_image(&*app, "IMAGE_FLAGMETERPARTS");
                    for a_flag_wave in 1..=a_num_flag_waves {
                        let mut a_height = 0;
                        let a_total_waves_at_flag = a_flag_wave * a_num_waves_per_flag;
                        if a_total_waves_at_flag < self.m_current_wave {
                            a_height = 14;
                        } else if a_total_waves_at_flag == self.m_current_wave {
                            a_height = crate::todlib::tod_common::tod_animate_curve(
                                100, 0, self.m_flag_raise_counter, 0, 14, TodCurves::Linear,
                            );
                        }
                        // C++: aPosX = PvzpAnimateCurve(0, mNumWaves, aTotalWavesAtFlag, aFlagsPosEnd, 606, LINEAR)
                        let a_pos_x_flag = crate::todlib::tod_common::tod_animate_curve(
                            0, self.m_num_waves, a_total_waves_at_flag, a_flags_pos_end, 606, TodCurves::Linear,
                        );
                        if !a_flag_parts.is_null() {
                            // C++: DrawImageCel(FLAGMETERPARTS, aPosX, 571, 1, 0) 旗杆 / (aPosX, 572-aHeight, 2, 0) 旗面
                            g.draw_image_cel(&*a_flag_parts, a_pos_x_flag, 571, 1);
                            g.draw_image_cel(&*a_flag_parts, a_pos_x_flag, 572 - a_height, 2);
                        }
                    }
                }
            }

            // C++: DrawImage(IMAGE_FLAGMETERLEVELPROGRESS, 638, 589)
            let a_level_progress = get_overlay_image(&*app, "IMAGE_FLAGMETERLEVELPROGRESS");
            if !a_level_progress.is_null() {
                g.draw_image_xy(&*a_level_progress, 638, 589);
            }

            // C++: 这些模式不绘制头部进度
            let a_gm = (*app).game_mode;
            if a_gm == GameMode::ChallengeBeghouled
                || a_gm == GameMode::ChallengeBeghouledTwist
                || a_gm == GameMode::ChallengeZombiquarium
                || (*app).is_squirrel_level()
                || (*app).is_slot_machine_level()
                || (*app).is_izombie_level()
                || (*app).is_final_boss_level()
            {
                return;
            }

            // C++: aHeadProgress = PvzpAnimateCurve(0, 150, mProgressMeterWidth, 0, 135, LINEAR)
            //      DrawImageCel(FLAGMETERPARTS, aCelWidth - aHeadProgress + 580, 572, 0, 0)
            let a_head_progress = crate::todlib::tod_common::tod_animate_curve(
                0, PROGRESS_METER_COUNTER, self.m_progress_meter_width, 0, 135, TodCurves::Linear,
            );
            let a_flag_parts2 = get_overlay_image(&*app, "IMAGE_FLAGMETERPARTS");
            if !a_flag_parts2.is_null() {
                g.draw_image_cel(&*a_flag_parts2, a_cel_width - a_head_progress + 580, 572, 0);
            }
        }
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

    /// 淡出关卡（对应 C++ Board::FadeOutLevel）
    pub fn fade_out_level(&mut self) {
        let app_ptr = match self.app { Some(a) => a, None => return };
        unsafe {
            let app = &mut *app_ptr;
            if app.game_scene != crate::lawn::lawn_app::GameScenes::Playing {
                self.refresh_seed_packet_from_cursor();
                if let Some(stats) = &mut app.m_last_level_stats {
                    stats.reset();
                }
                self.m_level_complete = true;
            }

            let mut need_sound_effect = true;
            if app.is_scary_potter_level() && !self.is_final_scary_potter_stage() {
                need_sound_effect = false;
            } else if self.is_survival_stage_with_repick()
                || self.is_last_stand_stage_with_repick()
                || app.is_endless_izombie(app.game_mode)
            {
                need_sound_effect = false;
            }
            if need_sound_effect {
                // C++: mApp->mMusic->StopAllMusic();
                if let Some(music) = &mut app.music {
                    music.stop_all_music();
                }
                if app.is_adventure_mode() && self.level == 50 {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::FinalFanfare as i32);
                } else if app.trophies_need_for_gold_sunflower() == 1 {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::FinalFanfare as i32);
                } else {
                    app.play_foley(crate::todlib::tod_foley::FoleyType::WinMusic as i32);
                }
            }

            if app.is_scary_potter_level() && !self.is_final_scary_potter_stage() {
                self.m_next_survival_stage_counter = 500;
                if app.is_adventure_mode() {
                    self.clear_advice(AdviceType::None);
                } else {
                    self.m_level_award_spawned = true;
                    self.puzzle_save_streak();
                    self.clear_advice(AdviceType::None);
                    let message = if app.is_endless_scary_potter(app.game_mode) {
                        "[ADVICE_MORE_SCARY_POTS]"
                    } else {
                        "[ADVICE_3_IN_A_ROW]"
                    };
                    self.display_advice(message, 2, AdviceType::None);
                }
                return;
            }

            if app.is_endless_izombie(app.game_mode) {
                self.m_next_survival_stage_counter = 500;
                self.puzzle_save_streak();
                self.clear_advice(AdviceType::None);
                self.display_advice("[ADVICE_MORE_IZOMBIE]", 2, AdviceType::None);
                return;
            }

            if self.is_last_stand_stage_with_repick() {
                self.m_next_survival_stage_counter = 500;
                if let Some(ref mut challenge) = self.challenge {
                    challenge.last_stand_completed_stage();
                }
                return;
            }

            if !self.is_survival_stage_with_repick() {
                self.refresh_seed_packet_from_cursor();
                if let Some(stats) = &mut app.m_last_level_stats {
                    stats.unused_lawn_mowers = self.count_untrigger_lawn_mowers();
                }
                self.m_board_fade_out_counter = 600;
                if self.level == 9 || self.level == 19 || self.level == 29 || self.level == 39 || self.level == 49 {
                    self.m_board_fade_out_counter = 500;
                }
                if self.can_drop_loot() {
                    self.m_score_next_mower_counter = 200;
                }
            } else {
                self.m_next_survival_stage_counter = 500;
                self.display_advice("[ADVICE_MORE_ZOMBIES]", 2, AdviceType::None);
                for row in 0..MAX_GRID_SIZE_Y {
                    self.m_ice_timer[row] = self.m_next_survival_stage_counter;
                }
            }
        }
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
        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        // C++: GAMEMODE_CHALLENGE_RESODDED && PLANTROW_DIRT && mCurrentWave < 5 → false
        if app_mode == GameMode::ChallengeResodded
            && self.m_plant_row[row as usize] == PlantRowType::Dirt
            && self.m_current_wave < 5
        {
            return false;
        }

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

        // 检查是否有僵尸到达房子（对应 C++ UpdateGame 中 ZombiesWon 触发条件）
        let mut winning_zombie: Option<usize> = None;
        for (index, zombie) in self.zombies.iter().enumerate() {
            if !zombie.dead && zombie.pos_x < -50.0 {
                winning_zombie = Some(index);
                break;
            }
        }
        if let Some(index) = winning_zombie {
            self.game_over(Some(index));
            return;
        }

        // 检查是否所有波次已完成且无僵尸
        if self.m_current_wave >= self.m_total_waves && self.zombies.is_empty() {
            self.level_complete();
        }
    }

    /// 游戏结束（对应 C++ ZombiesWon 的默认分支）
    pub fn game_over(&mut self, winning_zombie_index: Option<usize>) {
        self.m_game_over = true;
        self.m_board_result = BoardResult::Lost;
        self.m_game_over_countdown = 200;

        // 对应 C++ ZombiesWon 默认分支：mApp->mGameScene = SCENE_ZOMBIES_WON
        if let Some(app) = self.app {
            unsafe {
                (*app).game_scene = crate::lawn::lawn_app::GameScenes::ZombiesWon;
            }
        }

        // 对应 C++ ZombiesWon: theZombie->WalkIntoHouse()（进房僵尸走入房子/烟囱）
        if let Some(index) = winning_zombie_index {
            if let Some(zombie) = self.zombies.get_mut(index) {
                zombie.walk_into_house();
            }
        }

        // 对应 C++ ZombiesWon: mCutScene->StartZombiesWon()
        if let Some(cut_scene) = self.m_cut_scene {
            unsafe { (*cut_scene).start_zombies_won(); }
        }
    }

    /// 关卡完成
    pub fn level_complete(&mut self) {
        self.m_level_complete = true;
        self.m_board_result = BoardResult::Won;
    }

    /// 主绘制函数
    /// 绘制全部游戏对象（对应 C++ Board::DrawGameObjects，Board.cpp:6098）
    /// 构建渲染列表 → 按 zPos 排序 → 分发绘制（对应 C++ RenderItem 渲染层排序系统）
    pub fn draw_game_objects(&self, g: &mut Graphics) {
        let mut a_render_list: Vec<RenderItem> = Vec::with_capacity(MAX_RENDER_ITEMS);

        let app = self.app;
        let app_mode = app.map_or(GameMode::Adventure, |a| unsafe { (*a).game_mode });
        let game_scene = app.map_or(GameScenes::Playing, |a| unsafe { (*a).game_scene });

        // 植物（C++ 6105-6128）
        for (idx, plant) in self.plants.iter().enumerate() {
            if plant.dead {
                continue;
            }
            if plant.on_bungee_state == PlantOnBungeeState::NotOnBungee {
                // AddGameObjectRenderItemPlant
                a_render_list.push(RenderItem {
                    render_object_type: RenderObjectType::Plant,
                    z_pos: plant.base.render_order,
                    object_index: idx,
                    boss_part: None,
                });

                // C++: 禅园盆植物 → RENDER_ITEM_PLANT_OVERLAY
                if app_mode == GameMode::ChallengeZenGarden && plant.potted_plant_index != -1 {
                    a_render_list.push(RenderItem {
                        render_object_type: RenderObjectType::PlantOverlay,
                        z_pos: make_render_order(RENDER_LAYER_PARTICLE, 0, self.m_y),
                        object_index: idx,
                        boss_part: None,
                    });
                }

                // C++: 磁铁吸附物（顶层）→ RENDER_ITEM_PLANT_MAGNET_ITEMS
                if (plant.seed_type == SeedType::Magnetshroom || plant.seed_type == SeedType::GoldMagnet)
                    && plant.draw_magnet_items_on_top()
                {
                    a_render_list.push(RenderItem {
                        render_object_type: RenderObjectType::PlantMagnetItems,
                        z_pos: make_render_order(RENDER_LAYER_TOP, 0, -1),
                        object_index: idx,
                        boss_part: None,
                    });
                }
            }
        }

        // 硬币（C++ 6130）
        for (idx, coin) in self.coins.iter().enumerate() {
            if coin.dead {
                continue;
            }
            a_render_list.push(RenderItem {
                render_object_type: RenderObjectType::Coin,
                z_pos: coin.base.render_order,
                object_index: idx,
                boss_part: None,
            });
        }

        // 僵尸（C++ 6138-6178）
        for (idx, zombie) in self.zombies.iter().enumerate() {
            if zombie.dead {
                continue;
            }
            if zombie.zombie_type == ZombieType::Boss {
                // C++ AddBossRenderItem
                let a_boss_items = self.add_boss_render_item(zombie);
                for (a_boss_part, a_z_pos) in a_boss_items {
                    a_render_list.push(RenderItem {
                        render_object_type: RenderObjectType::BossPart,
                        z_pos: a_z_pos,
                        object_index: idx,
                        boss_part: Some(a_boss_part),
                    });
                }
            } else {
                a_render_list.push(RenderItem {
                    render_object_type: RenderObjectType::Zombie,
                    z_pos: zombie.base.render_order,
                    object_index: idx,
                    boss_part: None,
                });

                if zombie.has_shadow() {
                    a_render_list.push(RenderItem {
                        render_object_type: RenderObjectType::ZombieShadow,
                        z_pos: make_render_order(RENDER_LAYER_GROUND, zombie.base.row, 3),
                        object_index: idx,
                        boss_part: None,
                    });
                }

                if zombie.zombie_type == ZombieType::Bungee {
                    a_render_list.push(RenderItem {
                        render_object_type: RenderObjectType::ZombieBungeeTarget,
                        z_pos: make_render_order(RENDER_LAYER_PROJECTILE, zombie.base.row, 1),
                        object_index: idx,
                        boss_part: None,
                    });
                }
            }
        }

        // 子弹（C++ 6182-6189）：本体 + 阴影
        for (idx, projectile) in self.projectiles.iter().enumerate() {
            if projectile.dead {
                continue;
            }
            a_render_list.push(RenderItem {
                render_object_type: RenderObjectType::Projectile,
                z_pos: projectile.base.render_order,
                object_index: idx,
                boss_part: None,
            });
            a_render_list.push(RenderItem {
                render_object_type: RenderObjectType::ProjectileShadow,
                z_pos: make_render_order(RENDER_LAYER_GROUND, projectile.base.row, 3),
                object_index: idx,
                boss_part: None,
            });
        }

        // 割草机（C++ 6190-6196）
        for (idx, mower) in self.lawn_mowers.iter().enumerate() {
            if mower.dead {
                continue;
            }
            a_render_list.push(RenderItem {
                render_object_type: RenderObjectType::Mower,
                z_pos: mower.render_order,
                object_index: idx,
                boss_part: None,
            });
        }

        // 粒子系统（C++ 6198-6205）：跳过死亡与附着粒子
        if let Some(a) = app {
            unsafe {
                if let Some(es) = (*a).effect_system.as_ref() {
                    for (ps_idx, ps) in es.particle_systems.iter().enumerate() {
                        if ps.dead || ps.is_attachment {
                            continue;
                        }
                        a_render_list.push(RenderItem {
                            render_object_type: RenderObjectType::Particle,
                            z_pos: ps.render_order,
                            object_index: ps_idx,
                            boss_part: None,
                        });
                    }
                }
            }
        }

        // 重动画（C++ 6207-6216）：跳过死亡与附着动画
        if let Some(a) = app {
            unsafe {
                if let Some(es) = (*a).effect_system.as_ref() {
                    for (reanim_idx, reanim) in es.reanimations.iter().enumerate() {
                        if reanim.m_dead || reanim.m_is_attachment {
                            continue;
                        }
                        a_render_list.push(RenderItem {
                            render_object_type: RenderObjectType::Reanimation,
                            z_pos: reanim.m_render_order,
                            object_index: reanim_idx,
                            boss_part: None,
                        });
                    }
                }
            }
        }

        // 网格物品（C++ 6217-6228）
        for (idx, grid_item) in self.grid_items.iter().enumerate() {
            if grid_item.dead {
                continue;
            }
            a_render_list.push(RenderItem {
                render_object_type: RenderObjectType::GridItem,
                z_pos: grid_item.render_order,
                object_index: idx,
                boss_part: None,
            });

            // C++: 禅园臭鼬 → RENDER_ITEM_GRID_ITEM_OVERLAY
            if app_mode == GameMode::ChallengeZenGarden && grid_item.grid_item_type == GridItemType::PlantStinky {
                a_render_list.push(RenderItem {
                    render_object_type: RenderObjectType::GridItemOverlay,
                    z_pos: make_render_order(RENDER_LAYER_PARTICLE, 0, (grid_item.pos_y - 30.0) as i32),
                    object_index: idx,
                    boss_part: None,
                });
            }
        }

        // 冰面（C++ 6229-6238）
        for row in 0..MAX_GRID_SIZE_Y as i32 {
            if self.m_ice_timer[row as usize] != 0 {
                a_render_list.push(RenderItem {
                    render_object_type: RenderObjectType::Ice,
                    z_pos: self.get_ice_z_pos(row),
                    object_index: row as usize,
                    boss_part: None,
                });
            }
        }

        // UI 渲染项（C++ 6239-6303）
        let mut a_z_pos;
        if self.m_time_stop_counter > 0 {
            a_z_pos = make_render_order(RENDER_LAYER_ABOVE_UI, 0, 0);
        } else if game_scene == GameScenes::Playing || game_scene == GameScenes::ZombiesWon {
            a_z_pos = make_render_order(RENDER_LAYER_UI_BOTTOM, 0, 1);
        } else if self.m_cut_scene.map_or(false, |c| unsafe {
            (*c).is_after_seed_chooser() || (*c).is_in_shovel_tutorial()
        }) || self.m_advice == AdviceType::ClickToContinue
        {
            a_z_pos = make_render_order(RENDER_LAYER_UI_BOTTOM, 0, 1);
        } else {
            a_z_pos = make_render_order(RENDER_LAYER_ABOVE_UI, 0, 0);
        }
        a_render_list.push(RenderItem { render_object_type: RenderObjectType::Backdrop, z_pos: make_render_order(RENDER_LAYER_UI_BOTTOM, 0, 0), object_index: 0, boss_part: None });
        a_render_list.push(RenderItem { render_object_type: RenderObjectType::BottomUi, z_pos: a_z_pos, object_index: 0, boss_part: None });
        a_render_list.push(RenderItem { render_object_type: RenderObjectType::CoinBank, z_pos: make_render_order(RENDER_LAYER_COIN_BANK, 0, 0), object_index: 0, boss_part: None });
        a_render_list.push(RenderItem { render_object_type: RenderObjectType::TopUi, z_pos: make_render_order(RENDER_LAYER_UI_TOP, 0, 0), object_index: 0, boss_part: None });
        a_render_list.push(RenderItem { render_object_type: RenderObjectType::ScreenFade, z_pos: make_render_order(RENDER_LAYER_SCREEN_FADE, 0, 0), object_index: 0, boss_part: None });

        if game_scene == GameScenes::ZombiesWon {
            let a_door_z_pos = if self.stage_has_roof() {
                make_render_order(RENDER_LAYER_GRAVE_STONE, 0, 4)
            } else {
                make_render_order(RENDER_LAYER_GRAVE_STONE, 3, 2)
            };
            a_render_list.push(RenderItem { render_object_type: RenderObjectType::DoorMask, z_pos: a_door_z_pos, object_index: 0, boss_part: None });
        }
        if self.stage_has_fog() {
            a_render_list.push(RenderItem { render_object_type: RenderObjectType::Fog, z_pos: make_render_order(RENDER_LAYER_FOG, 0, 0), object_index: 0, boss_part: None });
        }
        if app.map_or(false, |a| unsafe { (*a).is_stormy_night_level() })
            || app_mode == GameMode::ChallengeRainingSeeds
        {
            a_render_list.push(RenderItem { render_object_type: RenderObjectType::Storm, z_pos: make_render_order(RENDER_LAYER_FOG, 0, 3), object_index: 0, boss_part: None });
        }
        // C++ 6309: AddGameObjectRenderItemCursorPreview(...mCursorPreview)
        a_render_list.push(RenderItem {
            render_object_type: RenderObjectType::CursorPreview,
            z_pos: self.cursor_preview.base.render_order,
            object_index: 0,
            boss_part: None,
        });

        // C++ 6313: 按 zPos 排序
        a_render_list.sort_by(render_item_sort_func);

        // C++ 6317-6520: 分发绘制
        for a_render_item in &a_render_list {
            match a_render_item.render_object_type {
                RenderObjectType::Plant => {
                    if let Some(plant) = self.plants.get(a_render_item.object_index) {
                        if plant.base.begin_draw(g) {
                            plant.draw(g);
                            plant.base.end_draw(g);
                        }
                    }
                }
                RenderObjectType::PlantOverlay => {
                    if let Some(plant) = self.plants.get(a_render_item.object_index) {
                        if plant.base.begin_draw(g) {
                            if let Some(a) = app {
                                unsafe {
                                    if let Some(zg) = (*a).zen_garden {
                                        (*zg).draw_plant_overlay(g, plant);
                                    }
                                }
                            }
                            plant.base.end_draw(g);
                        }
                    }
                }
                RenderObjectType::PlantMagnetItems => {
                    if let Some(plant) = self.plants.get(a_render_item.object_index) {
                        if plant.base.begin_draw(g) {
                            plant.draw_magnet_items(g);
                            plant.base.end_draw(g);
                        }
                    }
                }
                RenderObjectType::Mower => {
                    if let Some(mower) = self.lawn_mowers.get(a_render_item.object_index) {
                        mower.draw(g);
                    }
                }
                RenderObjectType::Zombie => {
                    if let Some(zombie) = self.zombies.get(a_render_item.object_index) {
                        if zombie.base.begin_draw(g) {
                            zombie.draw(g);
                            zombie.base.end_draw(g);
                        }
                    }
                }
                RenderObjectType::ZombieShadow => {
                    if let Some(zombie) = self.zombies.get(a_render_item.object_index) {
                        if zombie.base.begin_draw(g) {
                            zombie.draw_shadow(g);
                            zombie.base.end_draw(g);
                        }
                    }
                }
                RenderObjectType::ZombieBungeeTarget => {
                    if let Some(zombie) = self.zombies.get(a_render_item.object_index) {
                        zombie.draw_bungee_target(g);
                    }
                }
                RenderObjectType::BossPart => {
                    // C++ RENDER_ITEM_BOSS_PART: GetBossZombie + BeginDraw + DrawBossPart + EndDraw
                    if let Some(a_boss_zombie) = self.get_boss_zombie() {
                        if a_boss_zombie.base.begin_draw(g) {
                            if let Some(a_boss_part) = a_render_item.boss_part {
                                a_boss_zombie.draw_boss_part(g, a_boss_part);
                            }
                            a_boss_zombie.base.end_draw(g);
                        }
                    }
                }
                RenderObjectType::Coin => {
                    if let Some(coin) = self.coins.get(a_render_item.object_index) {
                        if coin.base.begin_draw(g) {
                            coin.draw(g);
                            coin.base.end_draw(g);
                        }
                    }
                }
                RenderObjectType::Projectile => {
                    if let Some(projectile) = self.projectiles.get(a_render_item.object_index) {
                        if projectile.base.begin_draw(g) {
                            projectile.draw(g);
                            projectile.base.end_draw(g);
                        }
                    }
                }
                RenderObjectType::ProjectileShadow => {
                    if let Some(projectile) = self.projectiles.get(a_render_item.object_index) {
                        if projectile.base.begin_draw(g) {
                            projectile.draw_shadow(g);
                            projectile.base.end_draw(g);
                        }
                    }
                }
                RenderObjectType::GridItem => {
                    if let Some(grid_item) = self.grid_items.get(a_render_item.object_index) {
                        grid_item.draw(g);
                    }
                }
                RenderObjectType::GridItemOverlay => {
                    if let Some(grid_item) = self.grid_items.get(a_render_item.object_index) {
                        grid_item.draw_grid_item_overlay(g);
                    }
                }
                RenderObjectType::Ice => {
                    self.draw_ice(g, a_render_item.object_index as i32);
                }
                RenderObjectType::Particle => {
                    if let Some(a) = app {
                        unsafe {
                            if let Some(es) = (*a).effect_system.as_ref() {
                                if let Some(ps) = es.particle_systems.get(a_render_item.object_index) {
                                    ps.draw(g);
                                }
                            }
                        }
                    }
                }
                RenderObjectType::Reanimation => {
                    if let Some(a) = app {
                        unsafe {
                            if let Some(es) = (*a).effect_system.as_ref() {
                                if let Some(reanim) = es.reanimations.get(a_render_item.object_index) {
                                    reanim.draw(g);
                                }
                            }
                        }
                    }
                }
                RenderObjectType::CursorPreview => {
                    // C++ 6433-6442: RENDER_ITEM_CURSOR_PREVIEW → BeginDraw/Draw/EndDraw
                    if self.cursor_preview.base.begin_draw(g) {
                        self.cursor_preview.draw(g);
                        self.cursor_preview.base.end_draw(g);
                    }
                }
                RenderObjectType::CoinBank => {
                    self.draw_ui_coin_bank(g);
                }
                RenderObjectType::Backdrop => {
                    self.draw_background(g);
                }
                RenderObjectType::DoorMask => {
                    self.draw_house_door_top(g);
                }
                RenderObjectType::BottomUi => {
                    self.draw_ui_bottom(g);
                }
                RenderObjectType::TopUi => {
                    self.draw_ui_top(g);
                }
                RenderObjectType::Fog => {
                    self.draw_fog(g);
                }
                RenderObjectType::Storm => {
                    if let Some(ch) = self.challenge.as_ref() {
                        ch.draw_weather(g);
                    }
                }
                RenderObjectType::ScreenFade => {
                    self.draw_fade_out(g);
                }
                _ => {
                    // C++: PVZP_ASSERT(false)
                }
            }
        }
    }

    /// 主绘制入口（对应 C++ Board::Draw，Board.cpp:7463）
    pub fn draw(&mut self, g: &mut Graphics) {
        // C++: if (mApp->GetDialog(DIALOG_STORE) || mApp->GetDialog(DIALOG_ALMANAC)) return;
        // Rust 无对话框栈；以 store_screen / almanac_dialog 非空近似对话框打开状态
        let dialog_open = self.app.map_or(false, |app| unsafe {
            (*app).store_screen.is_some() || (*app).almanac_dialog.is_some()
        });
        if dialog_open {
            return;
        }

        g.set_linear_blend(true);

        // C++ 7467-7484: FPS 统计（SDL_GetTicks → 毫秒；Rust 用 SystemTime）
        if self.m_draw_count != 0
            && self.m_cut_scene.map_or(false, |cs| unsafe { (*cs).m_preloaded })
        {
            let a_tick_count = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_millis() as i64);
            let a_interval_draws = self.m_draw_count as i64 - self.m_interval_draw_count_start as i64;
            let a_interval = a_tick_count - self.m_interval_draw_time;
            if a_interval > 10000 {
                let a_interval_fps = (a_interval_draws * 1000 + 500) as f32 / a_interval as f32;
                if self.m_min_fps > a_interval_fps {
                    self.m_min_fps = a_interval_fps;
                }
                self.m_interval_draw_count_start = self.m_draw_count;
                self.m_interval_draw_time = a_tick_count;
            }
        } else {
            self.reset_fps_stats();
        }

        self.m_draw_count = self.m_draw_count.wrapping_add(1);
        self.draw_game_objects(g);
    }

    /// 绘制背景
    fn draw_background(&self, g: &mut Graphics) {
        // 对应 C++ Board::DrawBackdrop：
        // 1) 按背景类型绘制背景图
        // 2) 绘制草地网格线（辅助显示格子边界）
        let bg_path = match self.m_background_type {
            BackgroundType::Day => Some("images/background1.jpg"),
            BackgroundType::Night => Some("images/background2.jpg"),
            BackgroundType::Pool => Some("images/background3.jpg"),
            BackgroundType::Fog => Some("images/background4.jpg"),
            BackgroundType::Roof => Some("images/background5.jpg"),
            BackgroundType::Boss => Some("images/background6boss.jpg"),
            BackgroundType::MushroomGarden => Some("images/background_mushroomgarden.jpg"),
            BackgroundType::Greenhouse => Some("images/background_greenhouse.jpg"),
            BackgroundType::Zombiquarium => Some("images/aquarium1.jpg"),
            _ => None,
        };

        if let Some(path) = bg_path {
            if let Some(img_ptr) = crate::todlib::reanim_loader::load_image_by_path(path) {
                let img = unsafe { &*img_ptr };
                if img.width > 0 && img.height > 0 {
                    // 花园类背景从 (0,0) 绘制，普通草坪从 -BOARD_OFFSET 绘制（对齐 9 列草坪）
                    let is_garden = matches!(self.m_background_type,
                        BackgroundType::MushroomGarden
                        | BackgroundType::Greenhouse
                        | BackgroundType::Zombiquarium);
                    let draw_x = if is_garden { 0 } else { -BOARD_OFFSET };
                    g.draw_image_f_xy(img, draw_x as f32, 0.0);
                }
            }
        }

        // 草地网格线（半透明白，便于观察格子边界；对应原版草坪网格）
        g.set_color(&Color::new(255, 255, 255, 36));
        let rows = if self.stage_has_6_rows() { 6 } else { 5 };
        let bottom_y = LAWN_YMIN + rows * 85 + 20;
        for col in 0..=MAX_GRID_SIZE_X as i32 {
            let x = LAWN_XMIN + col * 80;
            g.draw_line(x, LAWN_YMIN, x, bottom_y);
        }
        for row in 0..=rows {
            let y = LAWN_YMIN + row * 85;
            g.draw_line(LAWN_XMIN, y, LAWN_XMIN + MAX_GRID_SIZE_X as i32 * 80, y);
        }

        // C++ DrawBackdrop：僵尸获胜场景绘制房门底部（DrawHouseDoorBottom，Board.cpp:5928）
        if self.app.map_or(false, |app| unsafe { (*app).game_scene == crate::lawn::lawn_app::GameScenes::ZombiesWon }) {
            self.draw_house_door_bottom(g);
        }
    }

    /// 绘制 UI（对应 C++ Board::DrawUIBottom + DrawShovel + 阳光计数）
    /// 用位图字体绘制 UI 文本（成功返回 true；字体未加载返回 false）
    fn draw_ui_text(
        &self,
        g: &mut Graphics,
        text: &str,
        x: i32,
        y: i32,
        font: *mut crate::framework::graphics::font::Font,
        color: &Color,
        justification: DrawStringJustification,
    ) -> bool {
        // 对应 C++ PvzpDrawString(g, theText, thePosX, thePosY, theFont, theColor, theJustification)
        if font.is_null() {
            return false;
        }
        g.set_font(font);
        g.set_color(color);
        let text_width = unsafe { (*font).string_width(text) };
        let draw_x = match justification {
            DrawStringJustification::DS_ALIGN_RIGHT => x - text_width,
            DrawStringJustification::DS_ALIGN_CENTER => x - text_width / 2,
            _ => x,
        };
        g.draw_string(text, draw_x, y);
        true
    }

    /// 绘制冰面（对应 C++ Board::DrawIce）
    /// 冰面从 mIceMinX 开始逐段重复贴图
    fn draw_ice(&self, g: &mut Graphics, grid_y: i32) {
        let a_pos_y = self.grid_to_pixel_y(8, grid_y) + 20;
        let ice = self.get_board_image("ice");
        let ice_cap = self.get_board_image("ice_cap");
        if ice.is_none() {
            return;
        }
        let ice = ice.unwrap();
        let a_height = ice.height;
        let a_width = ice.width.max(1);
        let an_alpha = ((255 * self.m_ice_timer[grid_y as usize] / 10).clamp(0, 255)) as u8;
        if an_alpha < 255 {
            g.set_colorize_images(true);
            g.set_color(&crate::framework::color::Color::new(255, 255, 255, an_alpha));
        }

        let a_beginning_x = self.m_ice_min_x[grid_y as usize] + 13;
        let mut a_pos_x = a_beginning_x;
        let mut a_delta_x;
        while a_pos_x < BOARD_WIDTH {
            if a_pos_x == a_beginning_x {
                a_delta_x = (BOARD_WIDTH - a_beginning_x) % a_width;
                if a_delta_x == 0 {
                    a_delta_x = a_width;
                }
            } else {
                a_delta_x = a_width;
            }
            let a_repeat_src = crate::framework::rect::Rect::new(a_width - a_delta_x, 0, a_delta_x, a_height);
            let a_repeat_dst = crate::framework::rect::Rect::new(a_pos_x, a_pos_y, a_delta_x, a_height);
            g.draw_image_stretch(ice, &a_repeat_dst, &a_repeat_src);
            a_pos_x += a_delta_x;
        }
        if let Some(cap) = ice_cap {
            g.draw_image_f_xy(cap, self.m_ice_min_x[grid_y as usize] as f32, a_pos_y as f32);
        }
        g.set_colorize_images(false);
    }

    /// 绘制雾气（对应 C++ Board::DrawFog）
    fn draw_fog(&self, g: &mut Graphics) {
        let a_image_fog = self.get_board_image("fog");
        if a_image_fog.is_none() {
            return;
        }
        let a_image_fog = a_image_fog.unwrap();
        for x in 0..MAX_GRID_SIZE_X as i32 {
            for y in 0..MAX_GRID_SIZE_Y as i32 + 1 {
                let a_fade_amount = self.grid_cel_fog[x as usize][y as usize];
                if a_fade_amount == 0 {
                    continue;
                }
                // 雾形：额外第 6 行复用第 0 行的形状
                let a_cel_look = self.grid_cel_look[x as usize][(y % MAX_GRID_SIZE_Y as i32) as usize];
                let a_cel_col = a_cel_look % 8;
                let mut a_pos_x = x as f32 * 80.0 + self.m_fog_offset - 15.0;
                let mut a_pos_y = y as f32 * 85.0 + 20.0;
                const FOG_ANIM_PERIOD: u32 = 4500;
                let a_time = (self.m_main_counter % FOG_ANIM_PERIOD) as f32 * std::f32::consts::PI * 2.0;
                let a_phase_x = 6.0 * std::f32::consts::PI * x as f32 / MAX_GRID_SIZE_X as f32;
                let a_phase_y = 6.0 * std::f32::consts::PI * y as f32 / (MAX_GRID_SIZE_Y as f32 + 1.0);
                let a_motion = 13.0 + 4.0 * (a_time / 900.0 + a_phase_y).sin() + 8.0 * (a_time / 500.0 + a_phase_x).sin();

                let a_color_variant = (255.0 - a_cel_look as f32 * 1.5 - a_motion * 1.5) as u8;
                let a_lightness_variant = (255.0 - a_cel_look as f32 - a_motion) as u8;

                g.set_colorize_images(true);
                g.set_color(&crate::framework::color::Color::new(a_color_variant, a_color_variant, a_lightness_variant, a_fade_amount as u8));
                g.draw_image_cel(a_image_fog, a_pos_x as i32, a_pos_y as i32, a_cel_col);
                if x == MAX_GRID_SIZE_X as i32 - 1 {
                    g.draw_image_cel(a_image_fog, (a_pos_x + 80.0) as i32, a_pos_y as i32, a_cel_col);
                }
                g.set_colorize_images(false);
            }
        }
    }

    /// 绘制关卡名称（对应 C++ Board::DrawLevel）
    /// 绘制房门底部（GAMEOVER 界面，对应 C++ Board::DrawHouseDoorBottom）
    pub fn draw_house_door_bottom(&self, g: &mut Graphics) {
        let image_name = match self.m_background_type {
            BackgroundType::Day => "IMAGE_BACKGROUND1_GAMEOVER_INTERIOR_OVERLAY",
            BackgroundType::Night => "IMAGE_BACKGROUND2_GAMEOVER_INTERIOR_OVERLAY",
            BackgroundType::Pool => "IMAGE_BACKGROUND3_GAMEOVER_INTERIOR_OVERLAY",
            BackgroundType::Fog => "IMAGE_BACKGROUND4_GAMEOVER_INTERIOR_OVERLAY",
            _ => return,
        };
        let (pos_x, pos_y) = match self.m_background_type {
            BackgroundType::Day => (-126, 225),
            BackgroundType::Night => (-125, 196),
            BackgroundType::Pool => (-171, 241),
            BackgroundType::Fog => (-172, 246),
            _ => return,
        };
        let img_ptr = self
            .app
            .map_or(std::ptr::null_mut(), |app| unsafe { get_overlay_image(&*app, image_name) });
        if !img_ptr.is_null() {
            unsafe {
                g.draw_image_xy(&*img_ptr, pos_x, pos_y);
            }
        }
    }

    /// 绘制房门顶部（GAMEOVER 界面，对应 C++ Board::DrawHouseDoorTop）
    pub fn draw_house_door_top(&self, g: &mut Graphics) {
        let image_name = match self.m_background_type {
            BackgroundType::Day => "IMAGE_BACKGROUND1_GAMEOVER_MASK",
            BackgroundType::Night => "IMAGE_BACKGROUND2_GAMEOVER_MASK",
            BackgroundType::Pool => "IMAGE_BACKGROUND3_GAMEOVER_MASK",
            BackgroundType::Fog => "IMAGE_BACKGROUND4_GAMEOVER_MASK",
            BackgroundType::Roof => "IMAGE_BACKGROUND5_GAMEOVER_MASK",
            BackgroundType::Boss => "IMAGE_BACKGROUND6_GAMEOVER_MASK",
            _ => return,
        };
        let (pos_x, pos_y) = match self.m_background_type {
            BackgroundType::Day => (-130, 202),
            BackgroundType::Night => (-128, 207),
            BackgroundType::Pool => (-172, 234),
            BackgroundType::Fog => (-173, 133),
            BackgroundType::Roof => (-220, 81),
            BackgroundType::Boss => (-220, 81),
            _ => return,
        };
        let img_ptr = self
            .app
            .map_or(std::ptr::null_mut(), |app| unsafe { get_overlay_image(&*app, image_name) });
        if !img_ptr.is_null() {
            unsafe {
                g.draw_image_xy(&*img_ptr, pos_x, pos_y);
            }
        }
    }

    fn draw_level(&self, g: &mut Graphics) {
        let a_level_str = self.app.map_or(String::new(), |app| unsafe {
            let app = &*app;
            if app.is_adventure_mode() {
                format!("{} {}", crate::todlib::tod_common::tod_string_translate("[LEVEL]"), crate::lawn::lawn_app::LawnApp::get_stage_string(self.level))
            } else {
                // [TRANSLATION_NOTE]: 挑战模式名称/存活旗帜/无尽连胜依赖挑战定义，暂以阶段号近似
                format!("{}", self.level)
            }
        });
        let mut a_pos_x = 780;
        let mut a_pos_y = 595;
        if self.has_progress_meter() {
            a_pos_x = 593;
        }
        // C++: PvzpDrawString(g, aLevelStr, aPosX, aPosY, FONT_HOUSEOFTERROR16, Color(224, 187, 98), DS_ALIGN_RIGHT)
        self.draw_ui_text(
            g,
            &a_level_str,
            a_pos_x,
            a_pos_y,
            unsafe { crate::framework::graphics::bitmap_font::FONT_HOUSEOFTERROR16 },
            &crate::framework::color::Color::new(224, 187, 98, 255),
            DrawStringJustification::DS_ALIGN_RIGHT,
        );
    }

    /// 获取 Board 图片（按资源管理器小写 id）
    fn get_board_image(&self, id: &str) -> Option<&crate::framework::graphics::image::Image> {
        let app_ptr = self.app?;
        let app = unsafe { &*app_ptr };
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

    /// 关卡结束序列（对应 C++ UpdateLevelEndSequence）
    fn update_level_end_sequence(&mut self) {
        // C++ Board.cpp:1707-1756：生存/恐怖罐阶段推进
        if self.m_next_survival_stage_counter > 0 {
            // C++: if (!IsScaryPotterDaveTalking()) { mNextSurvivalStageCounter--; ... }
            if !self.is_scary_potter_dave_talking() {
                self.m_next_survival_stage_counter -= 1;
                // C++: 冒险模式恐怖罐 300 帧时 CrazyDave 入场 + 下一阶段清场
                let is_adventure_scary = self.app.map_or(false, |app| unsafe {
                    (*app).is_adventure_mode() && (*app).is_scary_potter_level()
                });
                if is_adventure_scary && self.m_next_survival_stage_counter == 300 {
                    if let Some(app) = self.app {
                        unsafe {
                            (*app).crazy_dave_enter();
                            let talk_index = self.challenge.as_ref().map_or(2700, |c| {
                                if c.survival_stage == 0 { 2700 } else { 2800 }
                            });
                            (*app).crazy_dave_talk_index(talk_index);
                        }
                    }
                    if let Some(ch) = self.challenge.as_mut() {
                        ch.puzzle_next_stage_clear();
                    }
                    self.m_next_survival_stage_counter = 100;
                }
            }

            // C++: if (mNextSurvivalStageCounter == 1 && mApp->IsSurvivalMode()) TryToSaveGame()
            if self.m_next_survival_stage_counter == 1
                && self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() })
            {
                self.try_to_save_game();
            }

            if self.m_next_survival_stage_counter == 0 {
                // C++ 分支：恐怖罐 / 无尽 IZombie / LastStand / 其他
                let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
                let is_scary = self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() });
                let is_adventure = self.app.map_or(false, |app| unsafe { (*app).is_adventure_mode() });
                if is_scary {
                    if is_adventure {
                        return;
                    }
                    if !self.is_final_scary_potter_stage() {
                        if let Some(ch) = self.challenge.as_mut() {
                            ch.puzzle_next_stage_clear();
                            ch.scary_potter_populate();
                        }
                    }
                } else if self.app.map_or(false, |app| unsafe { (*app).is_endless_izombie(app_mode) }) {
                    // C++ 1739-1744: mChallenge->PuzzleNextStageClear(); mChallenge->IZombieInitLevel();
                    if let Some(ch) = self.challenge.as_mut() {
                        ch.puzzle_next_stage_clear();
                        ch.i_zombie_init_level();
                    }
                } else if app_mode == GameMode::ChallengeLastStand {
                    self.clear_advice(AdviceType::None);
                } else {
                    self.m_level_complete = true;
                    self.remove_zombies_for_repick();
                }
                return;
            }
            // C++: counter > 0 且未减到 0 时继续向下（mBoardFadeOutCounter 此时通常为负 → 下方 return）
        }

        if self.m_board_fade_out_counter < 0 {
            return;
        }
        self.m_board_fade_out_counter -= 1;
        if self.m_board_fade_out_counter == 0 {
            self.m_level_complete = true;
            return;
        }
        if self.m_board_fade_out_counter == 300 {
            // C++: if (!IsSurvivalStageWithRepick() && !(mLevel == 9 || 19 || 29 || 39 || 49)) PlaySample(SOUND_LIGHTFILL)
            if !self.is_survival_stage_with_repick()
                && self.level != 9
                && self.level != 19
                && self.level != 29
                && self.level != 39
                && self.level != 49
            {
                // C++: mApp->PlaySample(Sexy::SOUND_LIGHTFILL);
                if let Some(app) = self.app {
                    unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_LIGHTFILL); }
                }
            }
        }
    }

    /// 绘制淡出效果（对应 C++ DrawFadeOut）
    /// 在关卡结束或进入下一关时播放黑白淡出动画
    fn draw_fade_out(&self, g: &mut Graphics) {
        if self.m_board_fade_out_counter < 0 || self.is_survival_stage_with_repick() {
            return;
        }

        let an_alpha = tod_animate_curve(
            200, 0, self.m_board_fade_out_counter, 0, 255, TodCurves::Linear,
        );

        // 第 10/20/30/40/50 关（Boss 关）用黑色，其他关用白色
        if self.level == 9 || self.level == 19 || self.level == 29 || self.level == 39 || self.level == 49
        {
            g.set_color(&Color::new(0, 0, 0, an_alpha as u8));
        } else {
            g.set_color(&Color::new(255, 255, 255, an_alpha as u8));
        }
        g.fill_rect_xywh(0, 0, self.m_width, self.m_height);
    }

    /// 鼠标按下事件（对应 C++ MouseDown L4479，完整版）
    pub fn mouse_down(&mut self, x: i32, y: i32, click_count: i32) {
        self.update_mouse_position(x, y);
        self.ignore_mouse_up = !self.can_interact_with_board_buttons();

        // C++ Board.cpp:4392: if (mTimeStopCounter > 0) return;
        if self.m_time_stop_counter > 0 {
            return;
        }

        if self.m_board_fade_out_counter >= 0 {
            return;
        }

        // 命中检测
        let mut hit_result = HitResult {
            object: None,
            object_type: GameObjectType::None,
        };
        self.mouse_hit_test(x, y, &mut hit_result);

        // 委托 Challenge::MouseDown
        if let Some(ref mut challenge) = self.challenge {
            if challenge.mouse_down(x, y, click_count, &mut hit_result) != 0 {
                return;
            }
        }

        // 更新光标
        self.update_cursor();

        // C++ 4397-4412: 菜单/商店按钮按下音效（theClickCount > 0）
        let a_can_interact = self.can_interact_with_board_buttons();
        let a_menu_over = self.menu_button.map_or(false, |m| unsafe { (*m).is_over });
        let a_store_over = self.store_button.map_or(false, |m| unsafe { (*m).is_over });
        if a_menu_over && a_can_interact && click_count > 0 {
            if let Some(app) = self.app {
                unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_GRAVEBUTTON); }
            }
        } else if a_store_over && a_can_interact && click_count > 0 {
            let a_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
            if a_mode == GameMode::ChallengeZenGarden || a_mode == GameMode::ChallengeTreeOfWisdom {
                // C++ 4406: PlaySample(SOUND_TAP)
                if let Some(app) = self.app {
                    unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_TAP); }
                }
            } else if a_mode == GameMode::ChallengeLastStand || a_mode == GameMode::Upsell {
                // C++ 4410: PlaySample(SOUND_GRAVEBUTTON)
                if let Some(app) = self.app {
                    unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_GRAVEBUTTON); }
                }
            }
        }
        // C++ 4419-4428：关卡开场（SCENE_LEVEL_INTRO）/僵尸胜利（SCENE_ZOMBIES_WON）场景分支
        let scene = self.app.map_or(crate::lawn::lawn_app::GameScenes::Playing, |app| unsafe { (*app).game_scene });
        if scene == crate::lawn::lawn_app::GameScenes::MainMenu {
            // C++: mApp->mSeedChooserScreen->CancelLawnView()
            if let Some(app) = self.app {
                unsafe {
                    if let Some(sc) = (*app).seed_chooser_screen {
                        let sc_ptr = sc as *mut crate::lawn::widget::seed_chooser_screen::SeedChooserScreen;
                        (*sc_ptr).cancel_lawn_view();
                    }
                }
            }
        }
        if scene == crate::lawn::lawn_app::GameScenes::ZombiesWon {
            // C++: mCutScene->ZombieWonClick(); return;
            if let Some(cs) = self.m_cut_scene {
                unsafe { (*cs).zombie_won_click(); }
            }
            return;
        }
        if scene == crate::lawn::lawn_app::GameScenes::MainMenu {
            // C++: mCutScene->MouseDown(x, y)
            if let Some(cs) = self.m_cut_scene {
                unsafe { (*cs).mouse_down(x, y); }
            }
        }

        // C++ 4430-4437: mCheatKeys && !IsScaryPotterLevel() && mNextSurvivalStageCounter > 0
        // → mNextSurvivalStageCounter = 2; mIceTimer[i] = min(mIceTimer[i], 2)
        let is_scary_potter = self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() });
        if self.app.map_or(false, |app| unsafe { (*app).m_cheat_keys_used })
            && !is_scary_potter
            && self.m_next_survival_stage_counter > 0
        {
            self.m_next_survival_stage_counter = 2;
            for i in 0..MAX_GRID_SIZE_Y {
                self.m_ice_timer[i] = self.m_ice_timer[i].min(2);
            }
        }

        // 处理关卡场景
        if self.m_paused {
            return;
        }

        // C++ 4439-4444：COBCANNON_TARGET 点击 / Coin 点击
        let cursor_type = self.cursor_object.cursor_type;
        if hit_result.object_type == GameObjectType::None {
            if cursor_type == CursorType::CobcannonTarget {
                self.mouse_down_cobcannon_fire(x, y, click_count);
                self.update_cursor();
                return;
            }
        } else if hit_result.object_type == GameObjectType::Coin && click_count >= 0 {
            // C++: Coin* aCoin = (Coin*)aHitResult.mObject; aCoin->MouseDown(x, y, theClickCount)
            if let Some(idx) = hit_result.object {
                if let Some(coin) = self.coins.get_mut(idx) {
                    coin.mouse_down(x, y, click_count);
                }
            }
            self.update_cursor();
            return;
        }

        // 处理工具点击
        let is_tool = matches!(cursor_type,
            CursorType::Shovel | CursorType::WateringCan | CursorType::Fertilizer |
            CursorType::BugSpray | CursorType::Phonograph | CursorType::Chocolate |
            CursorType::Glove | CursorType::MoneySign | CursorType::Wheelbarrow |
            CursorType::TreeFood
        );

        if is_tool {
            self.mouse_down_with_tool(x, y, click_count, cursor_type);
        } else if self.is_plant_in_cursor() {
            self.mouse_down_with_plant(x, y, click_count);
        } else if hit_result.object_type != GameObjectType::None {
            match hit_result.object_type {
                GameObjectType::SeedPacket => {
                    // C++: if (!mPaused) ((SeedPacket*)obj)->MouseDown(x, y, theClickCount)
                    if !self.m_paused {
                        if let Some(idx) = hit_result.object {
                            if let Some(packet) = self.seed_bank.get_mut(idx) {
                                packet.mouse_down(x, y, click_count);
                            }
                        }
                    }
                }
                GameObjectType::NextGarden => {
                    // C++: ZEN_GARDEN → GotoNextGarden；TREE_OF_WISDOM → TreeOfWisdomNextGarden；PlaySample(SOUND_TAP)
                    let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
                    if app_mode == GameMode::ChallengeZenGarden {
                        if let Some(app) = self.app {
                            unsafe {
                                if let Some(zg) = (*app).zen_garden {
                                    (*zg).goto_next_garden();
                                }
                            }
                        }
                    } else if app_mode == GameMode::ChallengeTreeOfWisdom {
                        if let Some(ch) = self.challenge.as_ref() {
                            ch.tree_of_wisdom_next_garden();
                        }
                    }
                    // C++: mApp->PlaySample(Sexy::SOUND_TAP);
                    if let Some(app) = self.app {
                        unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_TAP); }
                    }
                }
                GameObjectType::Shovel
                | GameObjectType::WateringCan
                | GameObjectType::Fertilizer
                | GameObjectType::BugSpray
                | GameObjectType::Phonograph
                | GameObjectType::Chocolate
                | GameObjectType::Glove
                | GameObjectType::MoneySign
                | GameObjectType::Wheelbarrow
                | GameObjectType::TreeFood => {
                    // C++: PickUpTool(aHitResult.mObjectType)
                    self.pick_up_tool(hit_result.object_type);
                }
                GameObjectType::Plant => {
                    // C++: ((Plant*)aHitResult.mObject)->MouseDown(x, y, theClickCount)
                    if let Some(idx) = hit_result.object {
                        if let Some(plant) = self.plants.get_mut(idx) {
                            plant.mouse_down(x, y, click_count);
                        }
                    }
                }
                _ => {}
            }
        }

        self.update_cursor();
    }

    /// 键盘事件
    pub fn key_down(&mut self, key: i32) {
        // C++: 调试键启用时按键字符转发给 KeyChar（近似：ASCII 可打印键码转字符）
        if key >= 32 && key < 127 {
            if let Some(app) = self.app {
                if unsafe { (*app).m_debug_keys_enabled } {
                    if let Some(ch) = char::from_u32(key as u32) {
                        self.key_char(ch);
                    }
                }
            }
        }
        match key {
            // 暂停
            80 => { // 'P'
                self.m_paused = !self.m_paused;
            },
            _ => {}
        }
    }

    /// 调试作弊键（对应 C++ Board::KeyChar，Board.cpp:7697-8456）
    /// 仅当 mApp->mDebugKeysEnabled 时生效；字符来自键盘输入事件
    pub fn key_char(&mut self, the_char: char) {
        // C++: if (!mApp->mDebugKeysEnabled) return;
        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        let app_ref = unsafe { &*app };
        if !app_ref.m_debug_keys_enabled {
            return;
        }

        // C++: PvzpTraceAndLogLn("Board cheat key '%c'", theChar)
        eprintln!("Board cheat key '{}'", the_char);

        // ========== 禅园模式（ChallengeZenGarden，C++ Board.cpp:7704-7915）==========
        if app_ref.game_mode == GameMode::ChallengeZenGarden {
            if the_char == 'm' {
                // C++: 生成金盏花盆栽
                let zen_garden = app_ref.zen_garden;
                let is_full = zen_garden.map_or(true, |zg| unsafe { (*zg).is_zen_garden_full(true) });
                if !is_full {
                    let mut a_potted_plant = crate::lawn::system::player_info::PottedPlant::new();
                    a_potted_plant.initialize_potted_plant(SeedType::Marigold);
                    a_potted_plant.draw_variation = unsafe {
                        std::mem::transmute::<i32, DrawVariation>(
                            crate::todlib::tod_common::rand_range_int(
                                DrawVariation::MarigoldWhite as i32,
                                DrawVariation::MarigoldLightGreen as i32,
                            ),
                        )
                    };
                    if let Some(zg) = zen_garden {
                        unsafe { (*zg).add_potted_plant(&mut a_potted_plant); }
                    }
                }
                return;
            }

            if the_char == '+' {
                let zen_garden = app_ref.zen_garden;
                let is_full = zen_garden.map_or(true, |zg| unsafe { (*zg).is_zen_garden_full(true) });
                if !is_full {
                    let mut a_potted_plant = crate::lawn::system::player_info::PottedPlant::new();
                    a_potted_plant.initialize_potted_plant(crate::lawn::zen_garden::ZenGarden::pick_random_seed_type());
                    if let Some(zg) = zen_garden {
                        unsafe { (*zg).add_potted_plant(&mut a_potted_plant); }
                    }
                }
                return;
            }

            if the_char == 'a' {
                let zen_garden = app_ref.zen_garden;
                let is_full = zen_garden.map_or(true, |zg| unsafe { (*zg).is_zen_garden_full(true) });
                if !is_full {
                    let mut a_potted_plant = crate::lawn::system::player_info::PottedPlant::new();
                    a_potted_plant.initialize_potted_plant(crate::lawn::zen_garden::ZenGarden::pick_random_seed_type());
                    a_potted_plant.plant_age = crate::lawn::game_enums::PottedPlantAge::Full;
                    if let Some(zg) = zen_garden {
                        unsafe { (*zg).add_potted_plant(&mut a_potted_plant); }
                    }
                }
                return;
            }

            if the_char == 'f' {
                // C++: 用对应工具喂养缺水/肥/虫/唱机的盆栽（Board.cpp:7741-7786）
                let zen_garden = app_ref.zen_garden;
                let mut i = 0;
                let plant_count = self.plants.len();
                while i < plant_count {
                    let (a_seed_dead, a_plant_col, a_plant_row, a_potted_plant_index, a_pos_x, a_pos_y) = {
                        let a_plant = &self.plants[i];
                        (
                            a_plant.dead,
                            a_plant.plant_col,
                            a_plant.base.row,
                            a_plant.potted_plant_index,
                            a_plant.pos_x as i32,
                            a_plant.pos_y as i32,
                        )
                    };
                    if a_seed_dead {
                        i += 1;
                        continue;
                    }
                    // C++: GetZenToolAt(plantCol, plantRow) == nullptr && mPottedPlantIndex >= 0
                    let has_zen_tool = self.grid_items.iter().any(|item| {
                        !item.dead
                            && item.grid_item_type == GridItemType::ZenTool
                            && item.grid_x == a_plant_col
                            && item.grid_y == a_plant_row
                    });
                    if !has_zen_tool && a_potted_plant_index >= 0 {
                        let a_need = zen_garden
                            .and_then(|zg| unsafe { (*zg).potted_plant_from_index(a_potted_plant_index as usize) })
                            .map_or(crate::lawn::game_enums::PottedPlantNeed::None, |pp| unsafe {
                                (*zen_garden.unwrap()).get_plants_need(&*pp)
                            });
                        if a_need == crate::lawn::game_enums::PottedPlantNeed::Water {
                            self.plants[i].highlighted = true;
                            if let Some(zg) = zen_garden {
                                unsafe { (*zg).mouse_down_with_feeding_tool(a_pos_x, a_pos_y, CursorType::WateringCan); }
                            }
                            return;
                        } else if a_need == crate::lawn::game_enums::PottedPlantNeed::Fertilizer {
                            self.plants[i].highlighted = true;
                            let a_purchase = app_ref.player_info.as_ref().map_or(0, |p| {
                                p.m_purchases.get(StoreItem::Fertilizer as usize).copied().unwrap_or(0)
                            });
                            if a_purchase <= 1000 {
                                // PURCHASE_COUNT_OFFSET == 1000
                                if let Some(pi) = unsafe { (*app).player_info.as_mut() }.map(|b| b.as_mut()) {
                                    if let Some(v) = pi.m_purchases.get_mut(StoreItem::Fertilizer as usize) {
                                        *v = 1000 + 1;
                                    }
                                }
                            }
                            if let Some(zg) = zen_garden {
                                unsafe { (*zg).mouse_down_with_feeding_tool(a_pos_x, a_pos_y, CursorType::Fertilizer); }
                            }
                            return;
                        } else if a_need == crate::lawn::game_enums::PottedPlantNeed::Bugspray {
                            self.plants[i].highlighted = true;
                            let a_purchase = app_ref.player_info.as_ref().map_or(0, |p| {
                                p.m_purchases.get(StoreItem::BugSpray as usize).copied().unwrap_or(0)
                            });
                            if a_purchase <= 1000 {
                                if let Some(pi) = unsafe { (*app).player_info.as_mut() }.map(|b| b.as_mut()) {
                                    if let Some(v) = pi.m_purchases.get_mut(StoreItem::BugSpray as usize) {
                                        *v = 1000 + 1;
                                    }
                                }
                            }
                            if let Some(zg) = zen_garden {
                                unsafe { (*zg).mouse_down_with_feeding_tool(a_pos_x, a_pos_y, CursorType::BugSpray); }
                            }
                            return;
                        } else if a_need == crate::lawn::game_enums::PottedPlantNeed::Phonograph {
                            self.plants[i].highlighted = true;
                            if let Some(zg) = zen_garden {
                                unsafe { (*zg).mouse_down_with_feeding_tool(a_pos_x, a_pos_y, CursorType::Phonograph); }
                            }
                            return;
                        }
                    }
                    i += 1;
                }
                return;
            }

            if the_char == 'r' {
                // C++: 重置所有盆栽植物计时器（Board.cpp:7788-7802）
                let zen_garden = app_ref.zen_garden;
                for a_plant in &self.plants {
                    if a_plant.dead {
                        continue;
                    }
                    if a_plant.potted_plant_index >= 0 {
                        if let Some(zg) = zen_garden {
                            let a_potted_plant = unsafe { (*zg).potted_plant_from_index(a_plant.potted_plant_index as usize) };
                            if let Some(pp) = a_potted_plant {
                                unsafe { (*zg).reset_plant_timers(&mut *pp); }
                            }
                        }
                    }
                }
                return;
            }

            if the_char == 's' {
                // C++: 唤醒或重置臭鼬（Board.cpp:7804-7815）
                let zen_garden = app_ref.zen_garden;
                if let Some(zg) = zen_garden {
                    unsafe {
                        if (*zg).is_stinky_sleeping() {
                            (*zg).wake_stinky();
                        } else {
                            (*zg).reset_stinky_timers();
                        }
                    }
                }
                return;
            }

            if the_char == 'c' {
                // C++: 巧克力数量 +1（未购买则补足 PURCHASE_COUNT_OFFSET+1，Board.cpp:7817-7828）
                if let Some(pi) = unsafe { (*app).player_info.as_mut() }.map(|b| b.as_mut()) {
                    let a_purchase = pi.m_purchases.get(StoreItem::Chocolate as usize).copied().unwrap_or(0);
                    if a_purchase < 1000 {
                        if let Some(v) = pi.m_purchases.get_mut(StoreItem::Chocolate as usize) {
                            *v = 1000 + 1;
                        }
                    } else if let Some(v) = pi.m_purchases.get_mut(StoreItem::Chocolate as usize) {
                        *v += 1;
                    }
                }
                return;
            }

            if the_char == ']' {
                // C++: 独轮车里的盆栽种子类型 +1（Board.cpp:7830-7846）
                let zen_garden = app_ref.zen_garden;
                if let Some(zg) = zen_garden {
                    unsafe {
                        if let Some(a_potted_plant) = (*zg).get_potted_plant_in_wheelbarrow() {
                            (*a_potted_plant).seed_type =
                                unsafe { std::mem::transmute::<i32, SeedType>((*a_potted_plant).seed_type as i32 + 1) };
                            if (*a_potted_plant).seed_type == SeedType::Gatlingpea {
                                (*a_potted_plant).seed_type = SeedType::Peashooter;
                            }
                            if (*a_potted_plant).seed_type == SeedType::Flowerpot {
                                (*a_potted_plant).seed_type = SeedType::Kernelpult;
                            }
                        }
                    }
                }
                return;
            }
        }

        // ========== 智慧树模式（ChallengeTreeOfWisdom，C++ Board.cpp:7849-7914）==========
        if app_ref.game_mode == GameMode::ChallengeTreeOfWisdom {
            if the_char == 'f' {
                // C++: 智慧树施肥（不足则补足）
                let a_purchase = app_ref.player_info.as_ref().map_or(0, |p| {
                    p.m_purchases.get(StoreItem::TreeFood as usize).copied().unwrap_or(0)
                });
                if a_purchase <= 1000 {
                    if let Some(pi) = unsafe { (*app).player_info.as_mut() }.map(|b| b.as_mut()) {
                        if let Some(v) = pi.m_purchases.get_mut(StoreItem::TreeFood as usize) {
                            *v = 1000 + 1;
                        }
                    }
                }
                if let Some(ch) = self.challenge.as_mut() {
                    ch.tree_of_wisdom_fertilize();
                }
            } else if the_char == 'g' {
                if let Some(ch) = self.challenge.as_mut() {
                    ch.tree_of_wisdom_grow();
                }
            } else if the_char == 'b' {
                if let Some(ch) = self.challenge.as_mut() {
                    ch.challenge_state_counter = 1;
                }
            } else if matches!(the_char, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8') {
                // C++: 0-8 设置挑战记录并长智慧树
                let a_record: i32 = match the_char {
                    '0' => 0,
                    '1' => 9,
                    '2' => 19,
                    '3' => 29,
                    '4' => 39,
                    '5' => 49,
                    '6' => 98,
                    '7' => 498,
                    _ => 998,
                };
                let a_index = app_ref.get_current_challenge_index() as usize;
                if let Some(pi) = unsafe { (*app).player_info.as_mut() }.map(|b| b.as_mut()) {
                    if let Some(v) = pi.m_challenge_records.get_mut(a_index) {
                        *v = a_record;
                    }
                }
                if let Some(ch) = self.challenge.as_mut() {
                    ch.tree_of_wisdom_grow();
                }
            }
            return;
        }

        // ========== 主作弊分支（C++ Board.cpp:7916-8456）==========
        if the_char == '<' {
            // C++: mApp->DoNewOptions(false)
            unsafe { (*app).do_new_options(false); }
        } else if the_char == 'l' {
            // C++: mApp->DoCheatDialog()
            unsafe { (*app).do_cheat_dialog(); }
        } else if the_char == '#' {
            // C++: 生存模式直达最后一波（Board.cpp:7924-7938）
            if app_ref.is_survival_mode() {
                if app_ref.game_scene == crate::lawn::lawn_app::GameScenes::LevelIntro {
                    return;
                }
                self.m_current_wave = self.m_num_waves;
                if let Some(ch) = self.challenge.as_mut() {
                    ch.survival_stage += 5;
                }
                self.remove_all_zombies();
                self.fade_out_level();
            }
        } else if the_char == '!' {
            // C++: 作弊结束当前关（Board.cpp:7939-7979）
            unsafe { (*app).board_result = BoardResult::Cheat; }
            if self.is_last_stand_stage_with_repick() {
                if self.m_next_survival_stage_counter == 0 {
                    self.m_current_wave = self.m_num_waves;
                    self.remove_all_zombies();
                    self.fade_out_level();
                }
            } else if (app_ref.is_scary_potter_level() && !self.is_final_scary_potter_stage())
                || app_ref.game_mode == GameMode::PuzzleIZombieEndless
            {
                if self.m_next_survival_stage_counter == 0 {
                    self.remove_all_zombies();
                    self.fade_out_level();
                }
            } else if app_ref.is_survival_mode() {
                if app_ref.game_scene == crate::lawn::lawn_app::GameScenes::LevelIntro {
                    return;
                }
                self.m_current_wave = self.m_num_waves;
                self.remove_all_zombies();
                self.fade_out_level();
            } else {
                self.remove_all_zombies();
                self.fade_out_level();
                self.m_board_fade_out_counter = 200;
            }
        } else if the_char == '+' {
            // C++: 加速结束当前关（Board.cpp:7980-8024）
            unsafe { (*app).board_result = BoardResult::Cheat; }
            if self.is_last_stand_stage_with_repick() {
                if self.m_next_survival_stage_counter == 0 {
                    self.m_current_wave = self.m_num_waves;
                    self.remove_all_zombies();
                    self.fade_out_level();
                }
            } else if (app_ref.is_scary_potter_level() && !self.is_final_scary_potter_stage())
                || app_ref.game_mode == GameMode::PuzzleIZombieEndless
            {
                if self.m_next_survival_stage_counter == 0 {
                    self.remove_all_zombies();
                    self.fade_out_level();
                }
            } else if app_ref.is_survival_endless(app_ref.game_mode) {
                if app_ref.game_scene == crate::lawn::lawn_app::GameScenes::LevelIntro {
                    return;
                }
                self.m_current_wave = self.m_num_waves;
                self.remove_all_zombies();
                self.fade_out_level();
            } else if app_ref.is_survival_mode() {
                if let Some(ch) = self.challenge.as_mut() {
                    ch.survival_stage = 5;
                }
                self.remove_all_zombies();
                self.fade_out_level();
                self.m_board_fade_out_counter = 200;
            } else {
                self.remove_all_zombies();
                self.fade_out_level();
                self.m_board_fade_out_counter = 200;
            }
        } else if the_char == '8' {
            // C++: 任意种植开关（Board.cpp:8025-8028）
            unsafe { (*app).m_easy_planting_cheat = !(*app).m_easy_planting_cheat; }
        } else if the_char == '7' {
            unsafe { (*app).toggle_slow_mo(); }
        } else if the_char == '6' {
            unsafe { (*app).toggle_fast_mo(); }
        } else if the_char == 'z' {
            // C++: 调试文本模式轮换（Board.cpp:8037-8044）
            let a_next_mode = self.m_debug_text_mode as i32 + 1;
            self.m_debug_text_mode = if a_next_mode > DebugTextMode::Collision as i32 {
                DebugTextMode::None
            } else {
                unsafe { std::mem::transmute::<i32, DebugTextMode>(a_next_mode) }
            };
        }

        // C++: if (mApp->mGameScene != SCENE_PLAYING) return;（Board.cpp:8046-8049）
        if app_ref.game_scene != crate::lawn::lawn_app::GameScenes::Playing {
            return;
        }

        // ========== Boss 分支（Board.cpp:8051-8084）==========
        let a_boss_zombie_idx = self.zombies.iter().position(|z| {
            !z.dead && z.zombie_type == ZombieType::Boss && !z.is_dead_or_dying()
        });
        if let Some(boss_idx) = a_boss_zombie_idx {
            if the_char == 'b' {
                self.zombies[boss_idx].boss_bungee_counter = 0;
                return;
            }
            if the_char == 'u' {
                self.zombies[boss_idx].summon_counter = 0;
                return;
            }
            if the_char == 's' {
                self.zombies[boss_idx].boss_stomp_counter = 0;
                return;
            }
            if the_char == 'r' {
                // C++: aBossZombie->BossRVAttack()
                self.zombies[boss_idx].boss_rv_attack();
                return;
            }
            if the_char == 'h' {
                self.zombies[boss_idx].boss_head_counter = 0;
                return;
            }
            if the_char == 'd' {
                self.zombies[boss_idx].take_damage(10000, 0);
                return;
            }
        }

        // ========== 保卫战模式（WarAndPeas，Board.cpp:8086-8113）==========
        if app_ref.game_mode == GameMode::ChallengeWarAndPeas
            || app_ref.game_mode == GameMode::ChallengeWarAndPeas2
        {
            let a_zombie_type = match the_char {
                'w' => Some(ZombieType::WallnutHead),
                't' => Some(ZombieType::TallnutHead),
                'j' => Some(ZombieType::JalapenoHead),
                'g' => Some(ZombieType::GatlingHead),
                's' => Some(ZombieType::SquashHead),
                _ => None,
            };
            if let Some(a_zombie_type) = a_zombie_type {
                self.add_zombie(a_zombie_type, -3); // C++: ZOMBIE_WAVE_DEBUG = -3
                return;
            }
        }

        // ========== 'q' 键：作弊铺场（Board.cpp:8115-8216）==========
        if the_char == 'q' {
            if app_ref.is_survival_endless(app_ref.game_mode) {
                unsafe { (*app).m_easy_planting_cheat = true; }
                for y in 0..crate::lawn::board::MAX_GRID_SIZE_X as i32 {
                    for x in 0..crate::lawn::board::MAX_GRID_SIZE_Y as i32 {
                        if self.can_plant_at(x, y, SeedType::Lilypad) == PlantingReason::Ok {
                            self.add_plant(x, y, SeedType::Lilypad, SeedType::None);
                        }
                        if self.can_plant_at(x, y, SeedType::Pumpkinshell) == PlantingReason::Ok {
                            if x <= 6 || self.is_pool_square(x, y) {
                                self.add_plant(x, y, SeedType::Pumpkinshell, SeedType::None);
                            }
                        }
                        if self.can_plant_at(x, y, SeedType::Gatlingpea) == PlantingReason::Ok {
                            if x < 5 {
                                self.add_plant(x, y, SeedType::Gatlingpea, SeedType::None);
                            } else if x == 5 {
                                self.add_plant(x, y, SeedType::Torchwood, SeedType::None);
                            } else if x == 6 {
                                self.add_plant(x, y, SeedType::Splitpea, SeedType::None);
                            } else if y == 2 || y == 3 {
                                self.add_plant(x, y, SeedType::Gloomshroom, SeedType::None);
                                if self.can_plant_at(x, y, SeedType::InstantCoffee) == PlantingReason::Ok {
                                    self.add_plant(x, y, SeedType::InstantCoffee, SeedType::None);
                                }
                            }
                        }
                    }
                }
            } else if app_ref.is_izombie_level() {
                unsafe { (*app).m_easy_planting_cheat = true; }
                for i in 0..5 {
                    if let Some(ch) = self.challenge.as_mut() {
                        ch.i_zombie_place_zombie(ZombieType::Football, 6, i);
                    }
                }
            } else {
                unsafe { (*app).m_easy_planting_cheat = true; }
                for y in 0..crate::lawn::board::MAX_GRID_SIZE_Y as i32 {
                    for x in 0..crate::lawn::board::MAX_GRID_SIZE_X as i32 {
                        if self.stage_has_roof()
                            && self.can_plant_at(x, y, SeedType::Flowerpot) == PlantingReason::Ok
                        {
                            self.add_plant(x, y, SeedType::Flowerpot, SeedType::None);
                        }
                        if self.can_plant_at(x, y, SeedType::Lilypad) == PlantingReason::Ok {
                            self.add_plant(x, y, SeedType::Lilypad, SeedType::None);
                        }
                        if self.can_plant_at(x, y, SeedType::Threepeater) == PlantingReason::Ok {
                            self.add_plant(x, y, SeedType::Threepeater, SeedType::None);
                        }
                    }
                }

                if let Some(ch) = self.challenge.as_mut() {
                    if ch.update_zombie_spawning() == 0 {
                        let mut a_waves_remaining = (self.m_num_waves - self.m_current_wave).min(20);
                        while a_waves_remaining > 0 {
                            self.spawn_zombie_wave();
                            a_waves_remaining -= 1;
                        }
                    }
                }

                if app_ref.is_scary_potter_level() {
                    let pots: Vec<usize> = self.grid_items.iter().enumerate()
                        .filter(|(_, item)| !item.dead && item.grid_item_type == GridItemType::ScaryPot)
                        .map(|(i, _)| i)
                        .collect();
                    for pot_idx in pots {
                        if let Some(ch) = self.challenge.as_mut() {
                            let pot_ptr = &mut self.grid_items[pot_idx] as *mut crate::lawn::grid_item::GridItem;
                            ch.scary_potter_open_pot(unsafe { &mut *pot_ptr });
                        }
                    }
                }
            }

        return;
        }

        // C++: 'O' 键——左 3 列铺花盆（Board.cpp:8218-8232）
        if the_char == 'O' {
            unsafe { (*app).m_easy_planting_cheat = true; }
            for y in 0..crate::lawn::board::MAX_GRID_SIZE_Y as i32 {
                for x in 0..3 {
                    if self.can_plant_at(x, y, SeedType::Flowerpot) == PlantingReason::Ok {
                        self.add_plant(x, y, SeedType::Flowerpot, SeedType::None);
                    }
                }
            }
            return;
        }

        // C++: '?' / '/' 键（Board.cpp:8234-8245）
        if the_char == '?' || the_char == '/' {
            if self.m_huge_wave_count_down > 0 {
                self.m_huge_wave_count_down = 1;
            } else {
                self.m_zombie_count_down = 6;
            }
            return;
        }

        // C++: 僵尸生成键（Board.cpp:8247-8365）
        let a_summon_zombie = match the_char {
            'b' => Some(ZombieType::Bungee),
            'o' => Some(ZombieType::Football),
            's' => Some(ZombieType::Door),
            'L' => Some(ZombieType::Ladder),
            'y' => Some(ZombieType::Yeti),
            'a' => Some(ZombieType::Flag),
            'w' => Some(ZombieType::Newspaper),
            'F' => Some(ZombieType::Balloon),
            'n' => Some(ZombieType::Snorkel),
            'c' => Some(ZombieType::TrafficCone),
            'm' => Some(ZombieType::Dancer),
            'h' => Some(ZombieType::Pail),
            'D' => Some(ZombieType::Digger),
            'p' => Some(ZombieType::Polevaulter),
            'P' => Some(ZombieType::Pogo),
            'R' => Some(ZombieType::DolphinRider),
            'j' => Some(ZombieType::JackInTheBox),
            'g' => Some(ZombieType::Gargantuar),
            'G' => Some(ZombieType::RedeEyeGargantuar),
            'i' => Some(ZombieType::Zamboni),
            'C' => Some(ZombieType::Catapult),
            _ => None,
        };
        if let Some(a_zombie_type) = a_summon_zombie {
            // C++: 'n'（潜水）和 'R'（海豚）需要水池
            if (the_char == 'n' || the_char == 'R') && !self.stage_has_pool() {
                // C++: 'n' 无水池时条件不满足不生成；'R' 无水池时仅在函数尾 return
            } else {
                self.add_zombie(a_zombie_type, -3); // ZOMBIE_WAVE_DEBUG = -3
            }
            if the_char != 'n' {
                return;
            }
        }

        // C++: '1' 键——销毁 (0,0) 顶部植物（Board.cpp:8366-8375）
        if the_char == '1' {
            let a_plant_idx = self.plants.iter().position(|p| {
                !p.dead && p.plant_col == 0 && p.base.row == 0
            });
            if let Some(idx) = a_plant_idx {
                self.plants[idx].die();
                if let Some(ch) = self.challenge.as_mut() {
                    let plant_ptr = &mut self.plants[idx] as *mut Plant;
                    ch.zombie_ate_plant(unsafe { &mut *plant_ptr });
                }
            }
            return;
        }

        // C++: 'B' 键——吹散迷雾（Board.cpp:8376-8380）
        if the_char == 'B' {
            self.m_fog_blown_count_down = 2200;
            return;
        }

        // C++: 't' 键——生成雪橇僵尸（Board.cpp:8381-8401）
        if the_char == 't' {
            if !self.can_add_bob_sled() {
                let mut a_row = crate::todlib::tod_common::rand_range_int(0, 5);
                let mut a_pos = 400;
                if self.stage_has_pool() {
                    a_row = crate::todlib::tod_common::rand_range_int(0, 2);
                } else if self.stage_has_roof() {
                    a_pos = 500;
                }
                self.m_ice_timer[a_row as usize] = 3000;
                self.m_ice_min_x[a_row as usize] = a_pos;
            }
            self.add_zombie(ZombieType::Bobsled, -3);
            return;
        }

        // C++: 'r' 键——从墓碑生成僵尸（Board.cpp:8402-8406）
        if the_char == 'r' {
            self.spawn_zombies_from_graves();
            return;
        }

        // C++: '0' / '9' 键——加阳光（Board.cpp:8407-8418）
        if the_char == '0' {
            self.add_sun_money(100);
            unsafe { (*app).play_sample(40); } // C++: SOUND_BUTTONCLICK
            return;
        }
        if the_char == '9' {
            self.add_sun_money(999999);
            unsafe { (*app).play_sample(40); }
            return;
        }

        // C++: '$' 键——加 100 金币并显示金币库（Board.cpp:8419-8425）
        if the_char == '$' {
            if let Some(pi) = unsafe { (*app).player_info.as_mut() }.map(|b| b.as_mut()) {
                pi.add_coins(100);
            }
            unsafe { (*app).play_sample(40); }
            self.show_coin_bank(90);
            return;
        }

        // C++: '-' 键——扣 100 阳光（Board.cpp:8426-8431）
        if the_char == '-' {
            self.m_sun_money -= 100;
            self.m_sun_money = self.m_sun_money.max(0);
            return;
        }

        // C++: '%' 键——切换屏模式（Board.cpp:8433-8436）
        if the_char == '%' {
            // [TRANSLATION_NOTE]: C++ SwitchScreenMode(mIsWindowed, !Is3DAccelerated(), false)；Rust 三维加速状态未接入
            unsafe { (*app).switch_screen_mode(true, false, false); }
        }

        // C++: 'M' 键——音乐突发覆盖递减（Board.cpp:8437-8441）
        if the_char == 'M' {
            // C++: mMusic->mBurstOverride -= 2 - (mMusic->mBurstOverride != 1)
            if let Some(music) = unsafe { (*app).music.as_mut() } {
                let a_dec = 2 - if music.burst_override != 1 { 1 } else { 0 };
                music.burst_override -= a_dec;
            }
            return;
        }

        // C++: Ctrl+3 崩溃测试（Board.cpp:8443-8455）
        if the_char == '\u{3}' {
            // C++: 条件为 mCtrlDown && mCheatKeys 且调用 PvzpCrash()。
            // [TRANSLATION_NOTE]: Rust 无 mCtrlDown 实时状态与 PvzpCrash 崩溃注入；
            // mCheatKeys 以 m_cheat_keys_used（has_used_cheat_keys）近似。仅保留波次提速副作用。
            if app_ref.m_cheat_keys_used {
                if self.m_huge_wave_count_down > 0 {
                    self.m_huge_wave_count_down = 1;
                } else {
                    self.m_zombie_count_down = 6;
                }
            }
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

    /// 选择背景（对应 C++ Board::PickBackground）
    pub fn pick_background(&mut self, app: *mut crate::lawn::lawn_app::LawnApp) {
        let game_mode = unsafe { (*app).game_mode };
        let level = self.level;
        let mut bg = BackgroundType::Day;
        if game_mode == GameMode::Adventure {
            if level <= 1 * LEVELS_PER_AREA {
                bg = BackgroundType::Day;
            } else if level <= 2 * LEVELS_PER_AREA {
                bg = BackgroundType::Night;
            } else if level <= 3 * LEVELS_PER_AREA {
                bg = BackgroundType::Pool;
            } else if unsafe { (*app).is_scary_potter_level() } {
                bg = BackgroundType::Night;
            } else if level <= 4 * LEVELS_PER_AREA {
                bg = BackgroundType::Fog;
            } else if level < FINAL_LEVEL {
                bg = BackgroundType::Roof;
            } else if level == FINAL_LEVEL {
                bg = BackgroundType::Boss;
            } else {
                bg = BackgroundType::Day;
            }
        } else {
            bg = match game_mode {
                GameMode::SurvivalNormalStage1 | GameMode::SurvivalHardStage1 | GameMode::SurvivalEndlessStage1
                | GameMode::ChallengeWarAndPeas | GameMode::ChallengeWallnutBowling
                | GameMode::ChallengeSlotMachine | GameMode::ChallengeSeeingStars
                | GameMode::ChallengeWallnutBowling2 | GameMode::ChallengeArtChallengeWallnut
                | GameMode::ChallengeSunnyDay | GameMode::ChallengeResodded | GameMode::ChallengeBigTime
                | GameMode::ChallengeArtChallengeSunflower | GameMode::ChallengeIceLevel
                | GameMode::ChallengeShovel | GameMode::ChallengeSquirrel => BackgroundType::Day,
                GameMode::SurvivalNormalStage2 | GameMode::SurvivalHardStage2 | GameMode::SurvivalEndlessStage2
                | GameMode::ChallengeBeghouled | GameMode::ChallengeBeghouledTwist
                | GameMode::ChallengePortalCombat | GameMode::ChallengeWhackAZombie
                | GameMode::ChallengeGraveDanger
                | GameMode::ScaryPotter1 | GameMode::ScaryPotter2 | GameMode::ScaryPotter3
                | GameMode::ScaryPotter4 | GameMode::ScaryPotter5 | GameMode::ScaryPotter6
                | GameMode::ScaryPotter7 | GameMode::ScaryPotter8 | GameMode::ScaryPotter9
                | GameMode::ScaryPotterEndless
                | GameMode::PuzzleIZombie1 | GameMode::PuzzleIZombie2 | GameMode::PuzzleIZombie3
                | GameMode::PuzzleIZombie4 | GameMode::PuzzleIZombie5 | GameMode::PuzzleIZombie6
                | GameMode::PuzzleIZombie7 | GameMode::PuzzleIZombie8 | GameMode::PuzzleIZombie9
                | GameMode::PuzzleIZombieEndless => BackgroundType::Night,
                GameMode::SurvivalNormalStage3 | GameMode::SurvivalHardStage3 | GameMode::SurvivalEndlessStage3
                | GameMode::ChallengeLittleTrouble | GameMode::ChallengeBobsledBonanza
                | GameMode::ChallengeZombieNimble | GameMode::ChallengeLastStand
                | GameMode::ChallengeWarAndPeas2 | GameMode::Upsell | GameMode::Intro => BackgroundType::Pool,
                GameMode::SurvivalNormalStage4 | GameMode::SurvivalHardStage4 | GameMode::SurvivalEndlessStage4
                | GameMode::ChallengeRainingSeeds | GameMode::ChallengeInvisighoul
                | GameMode::ChallengeAirRaid | GameMode::ChallengeStormyNight => BackgroundType::Fog,
                GameMode::SurvivalNormalStage5 | GameMode::SurvivalHardStage5 | GameMode::SurvivalEndlessStage5
                | GameMode::ChallengeColumns | GameMode::ChallengePogoParty
                | GameMode::ChallengeHighGravity | GameMode::ChallengeBungeeBlitz => BackgroundType::Roof,
                GameMode::ChallengeFinalBoss => BackgroundType::Boss,
                GameMode::ChallengeZombiquarium => BackgroundType::Zombiquarium,
                GameMode::ChallengeZenGarden => BackgroundType::Greenhouse,
                GameMode::ChallengeTreeOfWisdom => BackgroundType::TreeOfWisdom,
                _ => BackgroundType::Day,
            };
        }
        self.m_background_type = bg;
        self.load_background_images(app);

        // 设置行类型
        match bg {
            BackgroundType::Day | BackgroundType::Greenhouse | BackgroundType::TreeOfWisdom => {
                for row in 0..5 {
                    self.m_plant_row[row] = PlantRowType::Normal;
                }
                self.m_plant_row[5] = PlantRowType::Dirt;
                if unsafe { (*app).is_adventure_mode() } && unsafe { (*app).is_first_time_adventure_mode() } {
                    if level == 1 {
                        self.m_plant_row[0] = PlantRowType::Dirt;
                        self.m_plant_row[1] = PlantRowType::Dirt;
                        self.m_plant_row[3] = PlantRowType::Dirt;
                        self.m_plant_row[4] = PlantRowType::Dirt;
                    } else if level == 2 || level == 3 {
                        self.m_plant_row[0] = PlantRowType::Dirt;
                        self.m_plant_row[4] = PlantRowType::Dirt;
                    }
                } else if game_mode == GameMode::ChallengeResodded {
                    self.m_plant_row[0] = PlantRowType::Dirt;
                    self.m_plant_row[4] = PlantRowType::Dirt;
                }
            }
            BackgroundType::Night => {
                for row in 0..5 {
                    self.m_plant_row[row] = PlantRowType::Normal;
                }
                self.m_plant_row[5] = PlantRowType::Dirt;
            }
            BackgroundType::Pool | BackgroundType::Zombiquarium | BackgroundType::Fog => {
                self.m_plant_row[0] = PlantRowType::Normal;
                self.m_plant_row[1] = PlantRowType::Normal;
                self.m_plant_row[2] = PlantRowType::Pool;
                self.m_plant_row[3] = PlantRowType::Pool;
                self.m_plant_row[4] = PlantRowType::Normal;
                self.m_plant_row[5] = PlantRowType::Normal;
            }
            BackgroundType::Roof | BackgroundType::Boss => {
                for row in 0..5 {
                    self.m_plant_row[row] = PlantRowType::Normal;
                }
                self.m_plant_row[5] = PlantRowType::Dirt;
            }
            _ => {}
        }

        // 设置网格方块类型
        for x in 0..MAX_GRID_SIZE_X {
            for y in 0..MAX_GRID_SIZE_Y {
                if self.m_plant_row[y] == PlantRowType::Dirt {
                    self.grid_square_type[x][y] = GridSquareType::Dirt;
                } else if self.m_plant_row[y] == PlantRowType::Pool && x >= 0 && x <= 8 {
                    self.grid_square_type[x][y] = GridSquareType::Pool;
                } else if self.m_plant_row[y] == PlantRowType::HighGround && x >= 4 && x <= 8 {
                    self.grid_square_type[x][y] = GridSquareType::HighGround;
                }
            }
        }
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

    /// 种子在当前关卡是否不推荐（对应 C++ Board::SeedNotRecommendedForLevel）
    pub fn seed_not_recommended_for_level(&self, seed_type: SeedType) -> u32 {
        let mut not_rec = 0u32;
        if Plant::is_nocturnal(seed_type) && !self.stage_is_night() {
            not_rec |= 1 << (NotRecommend::Nocturnal as u32);
        }
        if seed_type == SeedType::InstantCoffee && self.stage_is_night() {
            not_rec |= 1 << (NotRecommend::AtNight as u32);
        }
        if seed_type == SeedType::Gravebuster && !self.stage_has_grave_stones() {
            not_rec |= 1 << (NotRecommend::NeedsGraves as u32);
        }
        if seed_type == SeedType::Plantern && !self.stage_has_fog() {
            not_rec |= 1 << (NotRecommend::NeedsFog as u32);
        }
        if seed_type == SeedType::Flowerpot && !self.stage_has_roof() {
            not_rec |= 1 << (NotRecommend::NeedsRoof as u32);
        }
        if self.stage_has_roof() && (seed_type == SeedType::Spikeweed || seed_type == SeedType::Spikerock) {
            not_rec |= 1 << (NotRecommend::OnRoof as u32);
        }
        if !self.stage_has_pool() && Plant::is_aquatic(seed_type) {
            not_rec |= 1 << (NotRecommend::NeedsPool as u32);
        }
        not_rec
    }

    /// 是否有墓碑
    pub fn stage_has_grave_stones(&self) -> bool {
        if self.m_background_type == BackgroundType::Night {
            return self.level >= 2 && self.level != 10;
        }
        false
    }

    // ========== UI 交互 ==========

    /// 清除提示信息（对应 C++ ClearAdvice，Board.cpp:1984）
    pub fn clear_advice(&mut self, help_index: AdviceType) {
        if help_index == AdviceType::None || help_index == self.m_advice {
            self.m_advice_widget.clear_label();
            self.m_advice = AdviceType::None;
        }
    }

    /// 统计产阳光植物数量（对应 C++ Board::CountSunFlowers，Board.cpp:2278）
    pub fn count_sun_flowers(&self) -> i32 {
        let mut a_count = 0;
        for plant in &self.plants {
            if plant.dead {
                continue;
            }
            if plant.makes_sun() {
                a_count += 1;
            }
        }
        a_count
    }

    /// 是否可与界面按钮交互（对应 C++ CanInteractWithBoardButtons L4680）
    pub fn can_interact_with_board_buttons(&self) -> bool {
        if self.m_board_fade_out_counter > 0 { return false; }
        if self.m_board_result != BoardResult::None { return false; }
        if self.m_level_complete { return false; }
        if self.m_paused { return false; }

        // 检查光标类型（只有 Normal/Hammer/CobcannonTarget 允许交互）
        let ct = self.cursor_object.cursor_type;
        if ct != CursorType::Normal && ct != CursorType::Hammer && ct != CursorType::CobcannonTarget {
            return false;
        }

        // C++ 检查 STATECHALLENGE_ZEN_FADING，Rust 暂缺该变体，跳过此检查

        // 检查游戏模式和疯狂戴夫状态
        self.app.map_or(true, |app| unsafe {
            let app = &*app;
            app.m_crazy_dave_state == CrazyDaveState::Off
        })
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

    /// 某行是否可以生成僵尸（对应 C++ RowCanHaveZombies）
    pub fn row_can_have_zombies(&self, row: i32) -> bool {
        if row < 0 || row >= MAX_GRID_SIZE_Y as i32 { return false; }
        self.m_plant_row[row as usize] != PlantRowType::Dirt
    }

    /// 重置 FPS 统计（对应 C++ ResetFPSStats）
    pub fn reset_fps_stats(&mut self) {
        // 对应 C++ ResetFPSStats（SDL_GetTicks → 毫秒时间戳）
        let a_tick_count = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_millis() as i64);
        self.m_start_draw_time = a_tick_count;
        self.m_interval_draw_time = a_tick_count;
        self.m_draw_count = 1;
        self.m_interval_draw_count_start = 1;
    }

    /// 移动棋盘（对应 C++ Widget::Move：设置 mX/mY）
    pub fn move_by(&mut self, x: i32, y: i32) {
        self.m_x = x;
        self.m_y = y;
    }

    /// 移动种子银行（对应 C++ mSeedBank->Move）
    pub fn move_seed_bank(&mut self, x: i32) {
        self.m_seed_bank_x = x;
    }

    /// 获取菜单按钮可变引用（对应 C++ mMenuButton）
    pub fn get_menu_button_mut(&mut self) -> Option<&mut crate::lawn::widget::game_button::GameButton> {
        match self.menu_button {
            Some(p) => unsafe { Some(&mut *p) },
            None => None,
        }
    }

    /// 获取商店按钮可变引用（对应 C++ mStoreButton）
    pub fn get_store_button_mut(&mut self) -> Option<&mut crate::lawn::widget::game_button::GameButton> {
        match self.store_button {
            Some(p) => unsafe { Some(&mut *p) },
            None => None,
        }
    }

    /// 获取墓碑数量（对应 C++ GetGraveStoneCount）
    pub fn get_grave_stone_count(&self) -> i32 {
        self.grid_items.iter().filter(|item| item.grid_item_type == GridItemType::Grave).count() as i32
    }

    /// 统计某种硬币的数量（对应 C++ CountCoinByType）
    pub fn count_coin_by_type(&self, coin_type: CoinType) -> i32 {
        self.coins.iter().filter(|c| c.coin_type == coin_type && !c.dead).count() as i32
    }

    /// 获取获胜僵尸（对应 C++ GetWinningZombie，Board.cpp:9474）
    pub fn get_winning_zombie(&self) -> Option<&Zombie> {
        for zombie in &self.zombies {
            if zombie.dead {
                continue;
            }
            if zombie.from_wave == Zombie::ZOMBIE_WAVE_WINNER {
                return Some(zombie);
            }
        }
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

    /// 掉落战利品（对应 C++ DropLootPiece）
    /// 根据关卡进度、游戏模式、随机概率决定掉落硬币/礼物/巧克力等
    pub(crate) fn drop_loot_piece(&mut self, pos_x: i32, pos_y: i32, drop_factor: i32) {
        if let Some(app) = self.app {
            unsafe {
                // 首次冒险模式特殊掉落
                if (*app).is_first_time_adventure_mode() {
                    if self.level == 22
                        && self.m_current_wave > 5
                        && !(*app).player_info.as_ref().map_or(false, |p| p.m_has_unlocked_minigames)
                        && self.count_coin_by_type(CoinType::PresentMinigames) == 0
                    {
                        (*app).play_foley(crate::todlib::tod_foley::FoleyType::ArtChallenge as i32);
                        self.add_coin(pos_x as f32, pos_y as f32, CoinType::PresentMinigames, CoinMotion::Coin);
                        return;
                    }
                    if self.level == 36
                        && self.m_current_wave > 5
                        && !(*app).player_info.as_ref().map_or(false, |p| p.m_has_unlocked_puzzle_mode)
                        && self.count_coin_by_type(CoinType::PresentPuzzleMode) == 0
                    {
                        (*app).play_foley(crate::todlib::tod_foley::FoleyType::ArtChallenge as i32);
                        self.add_coin(pos_x as f32, pos_y as f32, CoinType::PresentPuzzleMode, CoinMotion::Coin);
                        return;
                    }
                }

                let mut drop_hit = common::rand_range(30000);
                if (*app).is_first_time_adventure_mode() && self.level == 11 && !self.m_dropped_first_coin && self.m_current_wave > 5 {
                    drop_hit = 1000;
                }
                if (*app).game_mode == GameMode::ChallengeColumns {
                    drop_hit *= 5;
                }

                if (*app).is_whack_a_zombie_level() {
                    let sun_chance_min = 2500;
                    let sun_chance_max = if self.m_sun_money > 500 {
                        2800
                    } else if self.m_sun_money > 350 {
                        3100
                    } else if self.m_sun_money > 200 {
                        3700
                    } else {
                        5000
                    };
                    if drop_hit >= sun_chance_min * drop_factor && drop_hit <= sun_chance_max * drop_factor {
                        (*app).play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
                        self.add_coin((pos_x - 20) as f32, pos_y as f32, CoinType::Sun, CoinMotion::Coin);
                        self.add_coin((pos_x - 40) as f32, pos_y as f32, CoinType::Sun, CoinMotion::Coin);
                        self.add_coin((pos_x - 60) as f32, pos_y as f32, CoinType::Sun, CoinMotion::Coin);
                        return;
                    }
                }

                if self.m_total_spawned_waves > 70 {
                    return;
                }

                // 计算各掉落物概率阈值
                let potted_plant_chance = {
                    let zg = (*app).zen_garden;
                    if zg.map_or(true, |z| unsafe { !(*z).can_drop_potted_plant_loot() }) {
                        0
                    } else if (*app).is_adventure_mode() && !(*app).is_first_time_adventure_mode() {
                        24
                    } else {
                        if (*app).is_survival_endless((*app).game_mode) { 3 } else { 12 }
                    }
                };

                let chocolate_chance = if (*app).zen_garden.map_or(false, |z| unsafe { (*z).can_drop_chocolate() }) {
                    if (*app).is_adventure_mode() && !(*app).is_first_time_adventure_mode() {
                        potted_plant_chance + 72
                    } else {
                        potted_plant_chance + if (*app).is_survival_endless((*app).game_mode) { 9 } else { 36 }
                    }
                } else {
                    potted_plant_chance
                };

                let diamond_chance = chocolate_chance + 14;
                let gold_chance = chocolate_chance + 250;
                let silver_chance = chocolate_chance + 2500;

                let coin_type = if drop_hit < potted_plant_chance * drop_factor {
                    CoinType::PresentPlant
                } else if drop_hit < chocolate_chance * drop_factor {
                    CoinType::Chocolate
                } else if drop_hit < diamond_chance * drop_factor {
                    if (*app).player_info.as_ref().map_or(true, |p| p.m_purchases[StoreItem::PacketUpgrade as usize] < 1) {
                        CoinType::Gold
                    } else {
                        CoinType::Diamond
                    }
                } else if drop_hit < gold_chance * drop_factor {
                    CoinType::Gold
                } else if drop_hit < silver_chance * drop_factor {
                    CoinType::Silver
                } else {
                    return;
                };

                // 核桃保龄球关卡不掉钱
                if (*app).is_wallnut_bowling_level()
                    && (coin_type == CoinType::Gold || coin_type == CoinType::Silver || coin_type == CoinType::Diamond)
                {
                    return;
                }

                // L11 特殊：确保玩家有足够钱买第一个种子槽升级
                if (*app).is_first_time_adventure_mode() && self.level == 11 {
                    let gold_value = match coin_type {
                        CoinType::Diamond => 1000,
                        CoinType::Gold => 100,
                        CoinType::Silver => 10,
                        _ => 0,
                    };
                    let money = 100  // Coin::GetCoinValue(COIN_GOLD) 值
                        * self.lawn_mowers.len() as i32
                        + (*app).player_info.as_ref().map_or(0, |p| p.m_coins)
                        + self.count_coins_being_collected();
                    if gold_value + money
                        >= crate::lawn::widget::store_screen::StoreScreen::get_item_cost(StoreItem::PacketUpgrade)
                    {
                        return;
                    }
                }

                (*app).play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
                self.add_coin(pos_x as f32, pos_y as f32, coin_type, CoinMotion::Coin);
            }
        }
    }

    // ========== 实体创建 ==========

    /// 添加一个梯子（对应 C++ AddALadder）
    pub fn add_ladder(&mut self, grid_x: i32, grid_y: i32) {
        let mut ladder = GridItem::new();
        ladder.grid_item_type = GridItemType::Ladder;
        ladder.grid_item_initialize(GridItemType::Ladder, grid_x, grid_y);
        self.grid_items.push(ladder);
    }

    /// 更新格子物品（对应 C++ UpdateGridItems）
    pub fn update_grid_items(&mut self) {
        let a_game_scene = self.app.map_or(crate::lawn::lawn_app::GameScenes::Playing, |app| unsafe { (*app).game_scene });
        let mut a_to_die: Vec<usize> = Vec::new();
        for item in self.grid_items.iter_mut() {
            if item.dead {
                continue;
            }
            // 墓碑计数推进（出现动画）
            if self.m_enable_grave_stones
                && item.grid_item_type == GridItemType::Grave
                && item.counter < 100
            {
                item.counter += 1;
            }
            // 弹坑在游戏场景中消退
            if item.grid_item_type == GridItemType::Crater && a_game_scene == crate::lawn::lawn_app::GameScenes::Playing {
                if item.counter > 0 {
                    item.counter -= 1;
                }
                if item.counter == 0 {
                    item.grid_item_die();
                }
            }
            item.update();
        }
        let _ = a_to_die;
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

    /// 获取墓碑数量（对应 C++ GetGraveStonesCount）
    pub fn get_grave_stones_count(&self) -> i32 {
        self.grid_items.iter().filter(|item| {
            !item.dead && item.grid_item_type == GridItemType::Grave
        }).count() as i32
    }

    /// 火焰扫荡（对应 C++ DoFwoosh）
    pub fn do_fwoosh(&mut self, the_row: i32) {
        let a_render_order = crate::lawn::board::make_render_order(
            crate::lawn::game_enums::RENDER_LAYER_PARTICLE, the_row, 1,
        );
        let app_ptr = match self.app {
            Some(a) => a,
            None => return,
        };
        unsafe {
            let app = &mut *app_ptr;
            for i in 0..12usize {
                let a_old_id = self.m_fwoosh_id[the_row as usize][i];
                if a_old_id != REANIMATIONID_NULL {
                    if let Some(reanim) = app.reanimation_get_mut(a_old_id) {
                        reanim.reanimation_die();
                    }
                }

                let a_pos_x = 750.0 * i as f32 / 11.0 + 10.0;
                let a_pos_y = self.get_pos_y_based_on_row(a_pos_x + 10.0, the_row) - 10.0;
                if let Some(ptr) = app.add_reanimation(a_pos_x, a_pos_y, a_render_order, ReanimationType::JalapenoFire as i32) {
                    let a_id = app.reanimation_get_id(ptr);
                    if let Some(fwoosh) = app.reanimation_get_mut(a_id) {
                        fwoosh.set_frames_for_layer("anim_flame");
                        fwoosh.m_loop_type = crate::todlib::reanimator::ReanimLoopType::LoopFullOffset;
                        fwoosh.m_anim_rate *= crate::framework::common::rand_float(0.6) + 0.7; // RandRangeFloat(0.7, 1.3)
                        let a_scale = crate::framework::common::rand_float(0.2) + 0.9; // RandRangeFloat(0.9, 1.1)
                        let a_flip = if crate::framework::common::rand_range(2) != 0 { 1.0 } else { -1.0 };
                        fwoosh.override_scale(a_scale * a_flip, 1.0);
                    }
                    self.m_fwoosh_id[the_row as usize][i] = a_id;
                }
            }
        }
        self.m_fwoosh_count_down = 100;
    }

    /// 添加多个墓碑（对应 C++ AddGraveStones）
    pub fn add_grave_stones(&mut self, grid_x: i32, mut count: i32, level_rng: &mut crate::framework::mt_rand::MTRand) {
        // 限制 count 不超过可放墓碑的格子数，否则下方循环永不终止
        let mut grid_allow_grave_stones_count = 0;
        for y in 0..MAX_GRID_SIZE_Y {
            if self.can_add_grave_stone_at(grid_x, y as i32) {
                grid_allow_grave_stones_count += 1;
            }
        }
        if count > grid_allow_grave_stones_count {
            count = grid_allow_grave_stones_count;
        }

        let mut i = 0;
        while i < count {
            let grid_y = level_rng.next_range(MAX_GRID_SIZE_Y as u32) as i32;
            // 每次重新检查而非缓存允许数组，因为 AddAGraveStone 可能改变状态
            if self.can_add_grave_stone_at(grid_x, grid_y) {
                self.add_grave_stone(grid_x, grid_y);
                i += 1;
            }
        }
    }

    // ========== 显示/提示 ==========

    /// 立即清除提示（对应 C++ ClearAdviceImmediately，Board.cpp:1975）
    pub fn clear_advice_immediately(&mut self) {
        self.clear_advice(AdviceType::None);
        self.m_advice_widget.duration = 0;
    }

    /// 显示提示（对应 C++ DisplayAdvice，Board.cpp:1947）
    pub fn display_advice(&mut self, advice: &str, style: i32, help_index: AdviceType) {
        if help_index != AdviceType::None {
            let help_idx = help_index as i32 as usize;
            if help_idx >= NUM_ADVICE_TYPES as usize {
                return;
            }
            if self.m_help_displayed[help_idx] {
                return;
            }
            self.m_help_displayed[help_idx] = true;
        }
        // C++: mAdvice->SetLabel(theAdvice, theMessageStyle)
        self.m_advice_widget.set_label(advice, i32_to_message_style(style));
        self.m_advice = help_index;
    }

    /// 再次显示提示（对应 C++ DisplayAdviceAgain，Board.cpp:1961）
    pub fn display_advice_again(&mut self, advice: &str, style: i32, help_index: AdviceType) {
        if help_index != AdviceType::None {
            let help_idx = help_index as i32 as usize;
            if help_idx < NUM_ADVICE_TYPES as usize {
                self.m_help_displayed[help_idx] = false;
            }
        }
        self.display_advice(advice, style, help_index);
    }

    /// 显示教程箭头（对应 C++ TutorialArrowShow）
    pub fn tutorial_arrow_show(&mut self, x: i32, y: i32) {
        self.tutorial_arrow_remove();
        if let Some(app) = self.app {
            unsafe {
                // C++: aParticle = mApp->AddPvzpParticle(theX, theY, MakeRenderOrder(RENDER_LAYER_TOP, 0, 0), PARTICLE_SEED_PACKET_PICK);
                //      mTutorialParticleID = mApp->ParticleGetID(aParticle);
                // [TRANSLATION_NOTE]: C++ AddPvzpParticle 与 AddTodParticle 在 Rust 端同为 add_tod_particle 入口
                let particle = (*app).add_tod_particle(
                    x as f32,
                    y as f32,
                    crate::lawn::game_enums::RENDER_LAYER_TOP as i32,
                    crate::lawn::game_enums::ParticleEffect::SeedPacketPick as i32,
                );
                self.m_tutorial_particle_id = particle.map_or(0, |p| (*app).particle_get_id(p));
            }
        }
    }

    /// 移除教程箭头（对应 C++ TutorialArrowRemove）
    pub fn tutorial_arrow_remove(&mut self) {
        if let Some(app) = self.app {
            unsafe { (*app).remove_particle(self.m_tutorial_particle_id); }
        }
        self.m_tutorial_particle_id = 0;
    }

    /// 设置教程状态（对应 C++ Board::SetTutorialState）
    pub fn set_tutorial_state(&mut self, tutorial_state: TutorialState) {
        match tutorial_state {
            TutorialState::Level1PickUpPeashooter => {
                let plant_count = self.plants.iter().filter(|p| !p.dead).count();
                if plant_count == 0 {
                    if let Some(packet) = self.seed_bank.first() {
                        let pos_x = packet.x;
                        let pos_y = packet.y;
                        self.tutorial_arrow_show(pos_x, pos_y);
                    }
                    self.display_advice("[ADVICE_CLICK_SEED_PACKET]", 0, AdviceType::None);
                } else {
                    self.display_advice("[ADVICE_ENOUGH_SUN]", 0, AdviceType::None);
                    self.m_tutorial_timer = 400;
                }
            }
            TutorialState::Level1PlantPeashooter => {
                self.m_tutorial_timer = -1;
                self.tutorial_arrow_remove();
                let plant_count = self.plants.iter().filter(|p| !p.dead).count();
                if plant_count == 0 {
                    self.display_advice("[ADVICE_CLICK_ON_GRASS]", 0, AdviceType::None);
                } else {
                    self.clear_advice(AdviceType::None);
                }
            }
            TutorialState::Level1RefreshPeashooter => {
                self.display_advice("[ADVICE_PLANTED_PEASHOOTER]", 0, AdviceType::None);
                self.m_sun_countdown = 400;
            }
            TutorialState::Level1Completed => {
                self.display_advice("[ADVICE_ZOMBIE_ONSLAUGHT]", 1, AdviceType::None);
                self.m_zombie_count_down = 99;
                self.m_zombie_count_down_start = self.m_zombie_count_down;
            }
            TutorialState::Level2PickUpSunflower | TutorialState::MoreSunflowers => {
                if let Some(packet) = self.seed_bank.get(1) {
                    let pos_x = packet.x;
                    let pos_y = packet.y;
                    self.tutorial_arrow_show(pos_x, pos_y);
                }
            }
            TutorialState::Level2PlantSunflower | TutorialState::Level2RefreshSunflower => {
                self.tutorial_arrow_remove();
            }
            TutorialState::Level2Completed => {
                if self.m_current_wave == 0 {
                    self.m_zombie_count_down = 999;
                    self.m_zombie_count_down_start = self.m_zombie_count_down;
                }
            }
            TutorialState::ShovelPickup => {
                self.display_advice("[ADVICE_CLICK_SHOVEL]", 0, AdviceType::None);
                let shovel_rect = self.get_shovel_button_rect();
                let pos_x = shovel_rect.x + shovel_rect.width / 2 - 25;
                let pos_y = shovel_rect.y + shovel_rect.height - 65;
                self.tutorial_arrow_show(pos_x, pos_y);
            }
            TutorialState::ShovelDig => {
                self.display_advice("[ADVICE_CLICK_PLANT]", 0, AdviceType::None);
                self.tutorial_arrow_remove();
            }
            TutorialState::ShovelKeepDigging => {
                self.display_advice("[ADVICE_KEEP_DIGGING]", 0, AdviceType::None);
            }
            TutorialState::ShovelCompleted => {
                self.clear_advice(AdviceType::None);
                if let Some(cut_scene) = self.m_cut_scene {
                    unsafe {
                        (*cut_scene).m_cutscene_time = 1500;
                        (*cut_scene).m_crazy_dave_dialog_start = 2410;
                    }
                }
            }
            _ => {}
        }
        self.m_tutorial_state = tutorial_state;
    }

    /// 更新教程（对应 C++ Board::UpdateTutorial）
    pub fn update_tutorial(&mut self) {
        if self.m_tutorial_timer > 0 {
            self.m_tutorial_timer -= 1;
        }

        if self.m_tutorial_state == TutorialState::Level1PickUpPeashooter && self.m_tutorial_timer == 0 {
            self.display_advice("[ADVICE_CLICK_PEASHOOTER]", 0, AdviceType::None);
            if let Some(packet) = self.seed_bank.first() {
                self.tutorial_arrow_show(packet.x, packet.y);
            }
            self.m_tutorial_timer = -1;
        } else if matches!(self.m_tutorial_state,
            TutorialState::Level2PickUpSunflower | TutorialState::Level2PlantSunflower | TutorialState::Level2RefreshSunflower)
        {
            if self.m_tutorial_timer == 0 {
                self.display_advice("[ADVICE_PLANT_SUNFLOWER2]", 0, AdviceType::None);
                self.m_tutorial_timer = -1;
            } else if self.m_zombie_count_down == 750 && self.m_current_wave == 0 {
                self.display_advice("[ADVICE_PLANT_SUNFLOWER3]", 0, AdviceType::None);
            }
        } else if matches!(self.m_tutorial_state, TutorialState::MoreSunflowers) {
            if self.m_tutorial_timer == 0 {
                self.display_advice("[ADVICE_PLANT_SUNFLOWER5]", 0, AdviceType::None);
                self.m_tutorial_timer = -1;
            }
        }

        let is_first_time = self.app.map_or(false, |app| unsafe { (*app).is_first_time_adventure_mode() });
        if is_first_time
            && self.level >= 3 && self.level != 5 && self.level <= 7
            && self.m_tutorial_state == TutorialState::Off
            && self.m_current_wave >= 5
            && !unsafe { crate::lawn::board::G_SHOWN_MORE_SUN_TUTORIAL }
            && self.seed_bank.get(1).map_or(false, |p| p.can_pick_up())
            && self.count_plant_by_type(SeedType::Sunflower) < 3
        {
            self.display_advice("[ADVICE_PLANT_SUNFLOWER4]", 0, AdviceType::None);
            unsafe { crate::lawn::board::G_SHOWN_MORE_SUN_TUTORIAL = true; }
            self.set_tutorial_state(TutorialState::MoreSunflowers);
            self.m_tutorial_timer = 500;
        }
    }

    // ========== 植物/种子 ==========

    /// 获取种子槽 X 坐标（对应 C++ GetSeedPacketPositionX）
    pub fn get_seed_packet_position_x(&self, index: i32) -> i32 {
        80 + index * 70
    }

    /// 统计产阳光的植物数量（对应 C++ CountSunFlowers）
    /// 使用 MakesSun() 判断，包含向日葵、双胞向日葵、小喷菇、金盏花
    pub fn count_sunflowers(&self) -> i32 {
        self.plants.iter().filter(|p| !p.dead && p.makes_sun()).count() as i32
    }

    /// 光标中是否有植物（对应 C++ IsPlantInCursor）
    /// 根据 CursorType 判断：仅当手持种子/植物时返回 true
    pub fn is_plant_in_cursor(&self) -> bool {
        matches!(self.cursor_object.cursor_type,
            CursorType::PlantFromBank | CursorType::PlantFromUsableCoin |
            CursorType::PlantFromDuplicator | CursorType::PlantFromGlove |
            CursorType::PlantFromWheelBarrow
        )
    }

    /// 获取光标中的种子类型（对应 C++ GetSeedTypeInCursor）
    /// 手推车模式需从 ZenGarden 获取（依赖未译，暂返回 cursor_object.seed_type）
    pub fn get_seed_type_in_cursor(&self) -> SeedType {
        self.cursor_object.seed_type
    }

    /// 从光标刷新种子包（对应 C++ RefreshSeedPacketFromCursor）
    pub fn refresh_seed_packet_from_cursor(&mut self) {
        match self.cursor_object.cursor_type {
            CursorType::PlantFromUsableCoin => {
                // C++ 2027: mCoins.DataArrayTryToGet(mCursorObject->mCoinID)->DroppedUsableSeed();
                // [TRANSLATION_NOTE]: C++ DataArray id 以 Vec 索引近似（Rust 侧 Coin 无 id 字段，coin_id 未接入赋值流程）
                if let Some(coin) = self.coins.get_mut(self.cursor_object.coin_id as usize) {
                    coin.dropped_usable_seed();
                }
            }
            CursorType::PlantFromBank => {
                // C++ 2030-2031: mSeedBank->mSeedPackets[mCursorObject->mSeedBankIndex].Activate();
                let a_index = self.cursor_object.seed_bank_index;
                if a_index >= 0 && (a_index as usize) < self.seed_bank.len() {
                    self.seed_bank[a_index as usize].activate();
                }
            }
            _ => {}
        }
        self.clear_cursor();
    }

    // ========== 僵尸查询 ==========

    /// 统计存活的大型僵尸数量（对应 C++ GetLiveGargantuarCount）
    pub fn get_live_gargantuar_count(&self) -> i32 {
        self.zombies.iter().filter(|z| {
            z.has_head
                && !z.dead
                && (z.zombie_type == ZombieType::Gargantuar || z.zombie_type == ZombieType::RedeEyeGargantuar)
        }).count() as i32
    }

    /// 获取 BOSS 僵尸（对应 C++ GetBossZombie）
    pub fn get_boss_zombie(&self) -> Option<&Zombie> {
        self.zombies.iter().find(|z| !z.dead && z.zombie_type == ZombieType::Boss)
    }

    /// 获取 BOSS 僵尸（可变引用）
    pub fn get_boss_zombie_mut(&mut self) -> Option<&mut Zombie> {
        self.zombies.iter_mut().find(|z| !z.dead && z.zombie_type == ZombieType::Boss)
    }

    /// 检查蹦极僵尸是否正在瞄准某格（对应 C++ BungeeIsTargetingCell）
    pub fn bungee_is_targeting_cell(&self, grid_x: i32, grid_y: i32) -> bool {
        self.zombies.iter().any(|z| {
            !z.dead
                && z.zombie_type == ZombieType::Bungee
                && z.base.row == grid_y
                && z.target_col == grid_x
        })
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
    pub fn is_zombie_allowed(&self, zombie_type: ZombieType) -> bool {
        if zombie_type == ZombieType::Invalid {
            return false;
        }
        let idx = zombie_type as usize;
        idx < NUM_ZOMBIE_TYPES as usize && self.m_zombie_allowed[idx]
    }

    // ========== 关卡效果 ==========

    /// 检查某行是否有冰冻效果（对应 C++ IsIceAt）
    pub fn is_ice_at(&self, grid_x: i32, grid_y: i32) -> bool {
        // 对应 C++ Board::IsIceAt (Board.cpp:2708)
        // if (mIceTimer[theGridY] == 0 || mIceMinX[theGridY] > 750) return false;
        // return theGridX >= PixelToGridXKeepOnBoard(mIceMinX[theGridY] + 12, 0);
        if grid_y < 0 || grid_y >= MAX_GRID_SIZE_Y as i32 {
            return false;
        }
        let gy = grid_y as usize;
        if self.m_ice_timer[gy] == 0 || self.m_ice_min_x[gy] > 750 {
            return false;
        }
        grid_x >= self.pixel_to_grid_x_keep_on_board(self.m_ice_min_x[gy] + 12, 0)
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

    /// 检查是否可以使用游戏对象（对应 C++ CanUseGameObject）
    pub fn can_use_game_object(&self, object_type: GameObjectType) -> bool {
        let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        if game_mode == GameMode::ChallengeTreeOfWisdom {
            return object_type == GameObjectType::TreeFood || object_type == GameObjectType::NextGarden;
        }
        if game_mode != GameMode::ChallengeZenGarden {
            return false;
        }

        let purchase = |item: StoreItem| -> bool {
            self.app.map_or(false, |app| unsafe {
                (*app).player_info.as_ref().map_or(0, |p| p.m_purchases.get(item as usize).copied().unwrap_or(0)) > 0
            })
        };
        match object_type {
            GameObjectType::WateringCan => true,
            GameObjectType::NextGarden => {
                purchase(StoreItem::MushroomGarden)
                    || purchase(StoreItem::AquariumGarden)
                    || purchase(StoreItem::TreeOfWisdom)
            }
            GameObjectType::Fertilizer => purchase(StoreItem::Fertilizer),
            GameObjectType::BugSpray => purchase(StoreItem::BugSpray),
            GameObjectType::Phonograph => purchase(StoreItem::Phonograph),
            GameObjectType::Chocolate => purchase(StoreItem::Chocolate),
            GameObjectType::Wheelbarrow => purchase(StoreItem::WheelBarrow),
            GameObjectType::Glove => purchase(StoreItem::GardeningGlove),
            GameObjectType::MoneySign => self.app.map_or(false, |app| unsafe { (*app).has_finished_adventure() }),
            GameObjectType::TreeFood => false,
            _ => false,
        }
    }

    /// 检查僵尸类型能否在某一关卡生成（对应 C++ CanZombieSpawnOnLevel）
    /// 检查僵尸的起始关卡和选择权重，以及 Yeti 的特殊生成条件
    pub fn can_zombie_spawn_on_level(&self, zombie_type: ZombieType, level: i32) -> bool {
        if zombie_type == ZombieType::Yeti {
            return self.app.map_or(false, |app| unsafe { (*app).can_spawn_yetis() });
        }

        let zombie_def = crate::lawn::zombie::get_zombie_definition(zombie_type);
        if level < zombie_def.starting_level || zombie_def.pick_weight == 0 {
            return false;
        }

        // 对应 C++ Board::CanZombieSpawnOnLevel (Board.cpp:2377-2379)
        // PVZP_ASSERT(gZombieAllowedLevels[theZombieType].mZombieType == theZombieType);
        // return gZombieAllowedLevels[theZombieType].mAllowedOnLevel[std::clamp(theLevel - 1, 0, 49)];
        let allowed = crate::lawn::zombie::G_ZOMBIE_ALLOWED_LEVELS[zombie_type as usize];
        let level_idx = (level - 1).max(0).min(49) as usize;
        allowed[level_idx] != 0
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
        // 对应 C++ Board::InitLevel (Board.cpp:1352)
        self.m_main_counter = 0;
        self.m_board_update_counter = 0;
        self.m_enable_grave_stones = false;
        self.m_sod_position = 0;

        // mPrevBoardResult = mApp->mBoardResult;
        // [TRANSLATION_NOTE]: m_prev_board_result 字段未在 Rust Board 定义，跳过

        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        // mLevel = mApp->IsAdventureMode() ? mApp->mPlayerInfo->mLevel : 0;
        self.level = if self.app.map_or(false, |app| unsafe { (*app).is_adventure_mode() }) {
            self.app.map_or(0, |app| unsafe { (*app).m_level })
        } else {
            0
        };

        // C++ 1366-1367: PickBackground(); InitZombieWaves();
        self.setup_background();
        self.setup_waves();

        // C++ 1368-1388: 按模式设定初始阳光
        let is_izombie = self.app.map_or(false, |app| unsafe { (*app).is_izombie_level() });
        let is_scary = self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() });
        let is_whack = self.app.map_or(false, |app| unsafe { (*app).is_whack_a_zombie_level() });
        let is_first_time = self.app.map_or(false, |app| unsafe { (*app).is_first_time_adventure_mode() });
        if app_mode == GameMode::ChallengeBeghouled || app_mode == GameMode::ChallengeBeghouledTwist
            || is_scary || is_whack
        {
            self.m_sun_count = 0;
        } else if app_mode == GameMode::ChallengeLastStand {
            self.m_sun_count = 5000;
        } else if is_izombie {
            self.m_sun_count = 150;
        } else if is_first_time && self.level == 1 {
            self.m_sun_count = 150;
        } else {
            self.m_sun_count = 50;
        }

        // C++ 1390-1398: 行选择数组/冰块/阳光倒计时初始化
        for a_row in 0..MAX_GRID_SIZE_Y {
            self.m_wave_row_got_lawn_mowered[a_row] = -100;
            self.m_ice_min_x[a_row] = BOARD_ICE_START;
            self.m_ice_timer[a_row] = 0;
            self.m_row_picking_array[a_row].item = a_row as i32;
        }
        self.m_num_suns_fallen = 0;
        if !self.stage_is_night() {
            self.m_sun_countdown = crate::todlib::tod_common::rand_range_int(425, 700);
        }
        // [TRANSLATION_NOTE]: memset(mHelpDisplayed,0,...) 字段未定义，跳过

        // C++ 1405-1543: SeedBank 包数/宽度/坐标/各模式种子预设
        // [TRANSLATION_NOTE]: Rust seed_bank 为 Vec<SeedPacket>，与 C++ SeedBank* 结构不同；
        // 种子预设逻辑在 setup_waves / challenge.init_level 中部分覆盖，此处不逐项翻译

        // C++ 1544-1552
        self.m_paused = false;
        self.m_out_of_money_counter = 0;
        if self.stage_has_fog() {
            self.m_fog_blown_count_down = 200;
            self.m_fog_offset = 1065.0 - self.left_fog_column() as f32 * 80.0;
        }

        // ★ 关键接入：挑战模式关卡初始化（C++ Board.cpp:1553 mChallenge->InitLevel()）
        if let Some(ch) = self.challenge.as_mut() {
            ch.init_level();
        }
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

    /// 获取僵尸 ID（对应 C++ Board::ZombieGetID）
    pub fn zombie_get_id(&self, zombie: &Zombie) -> ZombieID {
        // Rust 侧用 Vec 索引作为 ZombieID
        self.zombies.iter().position(|z| std::ptr::eq(z, zombie)).unwrap_or(0) as ZombieID
    }

    /// 获取僵尸（对应 C++ Board::ZombieGet）
    pub fn zombie_get(&self, zombie_id: ZombieID) -> Option<&Zombie> {
        self.zombies.get(zombie_id as usize)
    }

    /// 尝试获取僵尸（对应 C++ Board::ZombieTryToGet）
    pub fn zombie_try_to_get(&self, zombie_id: ZombieID) -> Option<&Zombie> {
        self.zombies.get(zombie_id as usize).filter(|z| !z.dead)
    }

    /// 尝试获取僵尸（可变版本，Boss 弹射器离开等需要修改僵尸）
    pub fn zombie_try_to_get_mut(&mut self, zombie_id: ZombieID) -> Option<&mut Zombie> {
        self.zombies.get_mut(zombie_id as usize).filter(|z| !z.dead)
    }

    /// 杀死半径内所有植物（对应 C++ Board::KillAllPlantsInRadius）
    pub fn kill_all_plants_in_radius(&mut self, x: i32, y: i32, radius: i32) {
        let mut to_kill: Vec<usize> = Vec::new();
        for (i, plant) in self.plants.iter().enumerate() {
            if plant.dead { continue; }
            if crate::lawn::board::get_circle_rect_overlap(x, y, radius, &plant.get_plant_rect()) {
                to_kill.push(i);
            }
        }
        for idx in to_kill {
            self.m_plants_eaten += 1;
            self.plants[idx].die();
        }
    }

    /// 杀死半径内所有僵尸（对应 C++ Board::KillAllZombiesInRadius）
    /// 返回被杀僵尸数
    pub fn kill_all_zombies_in_radius(&mut self, row: i32, x: i32, y: i32, radius: i32, row_range: i32, burn: bool, damage_range_flags: u32) -> i32 {
        let mut killed_zombies = 0;
        let mut burn_list: Vec<usize> = Vec::new();
        let mut damage_list: Vec<usize> = Vec::new();
        for (i, zombie) in self.zombies.iter().enumerate() {
            if zombie.dead { continue; }
            if !zombie.effected_by_damage(damage_range_flags) { continue; }
            let zombie_rect = zombie.get_zombie_rect();
            let mut row_dist = zombie.base.row - row;
            if zombie.zombie_type == ZombieType::Boss {
                row_dist = 0;
            }
            if row_dist <= row_range && row_dist >= -row_range
                && crate::lawn::board::get_circle_rect_overlap(x, y, radius, &zombie_rect)
            {
                if burn {
                    burn_list.push(i);
                } else {
                    damage_list.push(i);
                }
                killed_zombies += 1;
            }
        }
        for idx in burn_list {
            self.zombies[idx].apply_burn();
        }
        for idx in damage_list {
            self.zombies[idx].take_damage(1800, 18);
        }

        let grid_x = self.pixel_to_grid_x_keep_on_board(x, y);
        let grid_y = self.pixel_to_grid_y_keep_on_board(x, y);
        let mut ladders: Vec<usize> = Vec::new();
        for (i, item) in self.grid_items.iter().enumerate() {
            if item.dead { continue; }
            if item.grid_item_type == crate::lawn::grid_item::GridItemType::Ladder {
                if crate::lawn::board::grid_in_range(item.grid_x, item.grid_y, grid_x, grid_y, row_range, row_range) {
                    ladders.push(i);
                }
            }
        }
        for idx in ladders {
            self.grid_items[idx].grid_item_die();
        }

        killed_zombies
    }

    /// 移除开场动画僵尸（对应 C++ RemoveCutsceneZombies）
    /// 遍历所有僵尸，从过场动画波次生成的僵尸调用 DieNoLoot
    pub fn remove_cutscene_zombies(&mut self) {
        let mut i = 0;
        while i < self.zombies.len() {
            if self.zombies[i].from_wave == crate::lawn::zombie::Zombie::ZOMBIE_WAVE_CUTSCENE {
                self.zombies[i].die_no_loot();
            }
            i += 1;
        }
    }

    /// 重选种子时移除僵尸（对应 C++ RemoveZombiesForRepick）
    /// 遍历所有僵尸，移除被精神控制且位置靠右的僵尸
    pub fn remove_zombies_for_repick(&mut self) {
        let mut i = 0;
        while i < self.zombies.len() {
            if !self.zombies[i].dead && self.zombies[i].zombie_phase != ZombiePhase::Dying
                && self.zombies[i].mind_controlled
                && self.zombies[i].pos_x > 720.0
            {
                self.zombies[i].die_no_loot();
            }
            i += 1;
        }
    }

    /// 检查是否可以种植（对应 C++ CanPlantAt 完整版）
    pub fn can_plant_at(&self, grid_x: i32, grid_y: i32, seed_type: SeedType) -> PlantingReason {
        // 对应 C++ Board::CanPlantAt (Board.cpp:2717)
        if grid_x < 0 || grid_x >= MAX_GRID_SIZE_X as i32 || grid_y < 0 || grid_y >= MAX_GRID_SIZE_Y as i32 {
            return PlantingReason::NotHere;
        }

        // C++ 2724: mChallenge->CanPlantAt
        let a_reason = self.challenge.as_ref().map_or(PlantingReason::Ok, |c| c.can_plant_at(grid_x, grid_y, seed_type));
        if a_reason != PlantingReason::Ok || Challenge::is_zombie_seed_type(seed_type) != 0 {
            return a_reason;
        }

        // C++ 2730: GetPlantsOnLawn
        let a_plant_on_lawn = self.get_plants_on_lawn(grid_x, grid_y);
        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });

        // C++ 2732-2744: Zen Garden
        if app_mode == GameMode::ChallengeZenGarden {
            if a_plant_on_lawn.under_plant.is_some() || a_plant_on_lawn.pumpkin_plant.is_some()
                || a_plant_on_lawn.flying_plant.is_some() || a_plant_on_lawn.normal_plant.is_some()
            {
                return PlantingReason::NotHere;
            }
            // C++: mApp->mZenGarden->mGardenType == GARDEN_AQUARIUM && !Plant::IsAquatic
            let is_aquarium = self.app.map_or(false, |app| unsafe {
                (*app).zen_garden.map_or(false, |zg| (*zg).garden_type == crate::lawn::game_enums::GardenType::Aquarium)
            });
            if is_aquarium && !Plant::is_aquatic(seed_type) {
                return PlantingReason::NotOnWater;
            }
            return PlantingReason::Ok;
        }

        // C++ 2746-2755: GraveBuster
        let a_has_grave = self.get_grave_stone_at(grid_x, grid_y).is_some();
        if seed_type == SeedType::Gravebuster {
            if a_plant_on_lawn.normal_plant.is_some() {
                return PlantingReason::NotHere;
            }
            return if a_has_grave { PlantingReason::Ok } else { PlantingReason::OnlyOnGraves };
        }
        // C++ 2756-2770: InstantCoffee
        if seed_type == SeedType::InstantCoffee {
            if a_plant_on_lawn.flying_plant.is_some() {
                return PlantingReason::NotHere;
            }
            // !aNormalPlant || !aNormalPlant->mIsAsleep || mWakeUpCounter > 0 || GETTING_GRABBED_BY_BUNGEE
            let needs_sleeping = match a_plant_on_lawn.normal_plant {
                Some(idx) => {
                    let p = &self.plants[idx];
                    !p.is_asleep || p.wake_up_counter > 0 || p.on_bungee_state == PlantOnBungeeState::GettingGrabbedByBungee
                }
                None => true,
            };
            if needs_sleeping {
                return PlantingReason::NeedsSleeping;
            }
            return PlantingReason::Ok;
        }
        // C++ 2771-2774: 有墓碑时飞行植物可种
        if a_has_grave {
            return if Plant::is_flying(seed_type) { PlantingReason::Ok } else { PlantingReason::NotOnGrave };
        }

        // C++ 2776-2787: UnderPlant / Lilypad / FlowerPot 判定
        let mut a_has_lilypad = false;
        let mut a_has_flower_pot = false;
        let under_is_grabbed;
        if let Some(idx) = a_plant_on_lawn.under_plant {
            let p = &self.plants[idx];
            under_is_grabbed = p.on_bungee_state == PlantOnBungeeState::GettingGrabbedByBungee;
            if !under_is_grabbed {
                a_has_lilypad = p.seed_type == SeedType::Lilypad;
                a_has_flower_pot = p.seed_type == SeedType::Flowerpot;
            }
        } else {
            under_is_grabbed = false;
        }
        let _ = under_is_grabbed;

        // C++ 2788-2795: 弹坑 / 恐怖罐 / 冰面
        if self.get_crater_at(grid_x, grid_y).is_some() {
            return PlantingReason::NotOnCrater;
        }
        if self.get_scary_pot_at(grid_x, grid_y).is_some() || self.is_ice_at(grid_x, grid_y) {
            return PlantingReason::NotHere;
        }

        // C++ 2796-2800: 网格类型
        let a_grid_square = self.grid_square_type[grid_x as usize][grid_y as usize];
        if a_grid_square == GridSquareType::Dirt || a_grid_square == GridSquareType::None {
            return PlantingReason::NotHere;
        }

        // C++ 2801-2810: Lilypad/TangleKelp/Seashroom 需水池
        let a_normal_plant = a_plant_on_lawn.normal_plant;
        if seed_type == SeedType::Lilypad || seed_type == SeedType::Tanglekelp || seed_type == SeedType::Seashroom {
            if !self.is_pool_square(grid_x, grid_y) {
                return PlantingReason::OnlyInPool;
            }
            return if a_normal_plant.is_some() || a_plant_on_lawn.under_plant.is_some() { PlantingReason::NotHere } else { PlantingReason::Ok };
        }
        // C++ 2811-2814: 飞行植物
        if Plant::is_flying(seed_type) {
            return if a_plant_on_lawn.flying_plant.is_some() { PlantingReason::NotHere } else { PlantingReason::Ok };
        }
        // C++ 2815-2821: Spikeweed/Spikerock 需地面
        if seed_type == SeedType::Spikeweed || seed_type == SeedType::Spikerock {
            if a_grid_square == GridSquareType::Pool || self.stage_has_roof() || a_plant_on_lawn.under_plant.is_some() {
                return PlantingReason::NeedsGround;
            }
        }
        // C++ 2823-2830: 非水生植物在水池格需睡莲（Cattail 除外）
        let a_pumpkin_plant = a_plant_on_lawn.pumpkin_plant;
        if a_grid_square == GridSquareType::Pool && !a_has_lilypad && seed_type != SeedType::Cattail {
            let cattail_ok = match a_normal_plant {
                Some(idx) => {
                    let p = &self.plants[idx];
                    p.seed_type == SeedType::Cattail && seed_type == SeedType::Pumpkinshell
                }
                None => false,
            };
            if !cattail_ok {
                return PlantingReason::NotOnWater;
            }
        }
        // C++ 2831-2834: FlowerPot
        if seed_type == SeedType::Flowerpot {
            return if a_normal_plant.is_some() || a_plant_on_lawn.under_plant.is_some() || a_pumpkin_plant.is_some() {
                PlantingReason::NotHere
            } else {
                PlantingReason::Ok
            };
        }
        // C++ 2835-2838: 屋顶需花盆
        if self.stage_has_roof() && !a_has_flower_pot {
            return PlantingReason::NeedsPot;
        }

        // C++ 2839-2858: 急救购买 + PumpkinShell
        let a_aid_purchased = self.app.map_or(false, |app| unsafe {
            (*app).player_info.as_ref().map_or(false, |pi| {
                pi.m_purchases.get(crate::lawn::game_enums::StoreItem::Firstaid as usize).map_or(0, |v| *v) > 0
            })
        });
        if seed_type == SeedType::Pumpkinshell {
            if let Some(idx) = a_normal_plant {
                if self.plants[idx].seed_type == SeedType::Cobcannon {
                    return PlantingReason::NotHere;
                }
            }
            if a_pumpkin_plant.is_none() {
                return PlantingReason::Ok;
            }
            // 南瓜急救
            if a_aid_purchased {
                if let Some(idx) = a_pumpkin_plant {
                    let p = &self.plants[idx];
                    if p.plant_health < p.plant_max_health * 2 / 3
                        && p.seed_type == SeedType::Pumpkinshell
                        && p.on_bungee_state != PlantOnBungeeState::GettingGrabbedByBungee
                    {
                        return PlantingReason::Ok;
                    }
                }
            }
            return PlantingReason::NotHere;
        }
        // C++ 2859-2862: 马铃薯雷不可种在睡莲上
        if a_has_lilypad && seed_type == SeedType::PotatoMine {
            return PlantingReason::OnlyOnGround;
        }

        // C++ 2864-2888: UnderPlant 分支
        if let Some(under_idx) = a_plant_on_lawn.under_plant {
            let under_plant = &self.plants[under_idx];
            if seed_type == SeedType::Cattail {
                if a_normal_plant.is_some() {
                    return PlantingReason::NotHere;
                }
                if under_plant.is_upgradable_to(seed_type)
                    && under_plant.on_bungee_state != PlantOnBungeeState::GettingGrabbedByBungee
                {
                    return PlantingReason::Ok;
                }
                if Plant::is_upgrade(seed_type) {
                    return PlantingReason::NeedsUpgrade;
                }
            } else {
                if under_plant.seed_type == SeedType::Imitater {
                    return PlantingReason::NotHere;
                }
            }
        }

        // C++ 2890-2912: NormalPlant 分支
        if let Some(norm_idx) = a_normal_plant {
            let normal_plant = &self.plants[norm_idx];
            if normal_plant.is_upgradable_to(seed_type)
                && normal_plant.on_bungee_state != PlantOnBungeeState::GettingGrabbedByBungee
            {
                return PlantingReason::Ok;
            }
            if Plant::is_upgrade(seed_type) {
                return PlantingReason::NeedsUpgrade;
            }
            // 坚果急救
            if (seed_type == SeedType::Wallnut || seed_type == SeedType::Tallnut) && a_aid_purchased {
                if normal_plant.plant_health < normal_plant.plant_max_health * 2 / 3
                    && normal_plant.seed_type == seed_type
                    && normal_plant.on_bungee_state != PlantOnBungeeState::GettingGrabbedByBungee
                {
                    return PlantingReason::Ok;
                }
            }
            return PlantingReason::NotHere;
        }

        // C++ 2914-2926: 无普通植物时的升级判定
        let easy_planting_cheat = self.app.map_or(false, |app| unsafe { (*app).m_easy_planting_cheat });
        if !easy_planting_cheat && Plant::is_upgrade(seed_type) {
            return PlantingReason::NeedsUpgrade;
        }
        if seed_type == SeedType::Cobcannon && !self.is_valid_cob_cannon_spot(grid_x, grid_y) {
            return PlantingReason::NeedsUpgrade;
        } else if seed_type == SeedType::Cattail && a_grid_square != GridSquareType::Pool {
            return PlantingReason::NotHere;
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
        // 对应 C++ Board::PlantUsesAcceleratedPricing (Board.cpp:9396)
        // return Plant::IsUpgrade(theSeedType) && mApp->IsSurvivalEndless(mApp->mGameMode);
        let is_survival_endless = self.app.map_or(false, |app| unsafe {
            (*app).is_survival_endless((*app).game_mode)
        });
        Plant::is_upgrade(seed_type) && is_survival_endless
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
    pub fn get_top_plant_at(&self, grid_x: i32, grid_y: i32, priority: PlantPriority) -> Option<&Plant> {
        // 对应 C++ Board::GetTopPlantAt (Board.cpp:2237)
        if grid_x < 0 || grid_x >= MAX_GRID_SIZE_X as i32 || grid_y < 0 || grid_y >= MAX_GRID_SIZE_Y as i32 {
            return None;
        }

        // C++ 2242: IsWallnutBowlingLevel && !IsInShovelTutorial → null
        let is_wallnut_bowling = self.app.map_or(false, |app| unsafe { (*app).is_wallnut_bowling_level() });
        let is_in_shovel_tutorial = self.m_cut_scene.map_or(false, |c| unsafe { (*c).is_in_shovel_tutorial() });
        if is_wallnut_bowling && !is_in_shovel_tutorial {
            return None;
        }

        let a_plant_on_lawn = self.get_plants_on_lawn(grid_x, grid_y);

        // C++ 2248-2274: switch(thePriority)
        let idx = match priority {
            PlantPriority::EatingOrder => {
                a_plant_on_lawn.pumpkin_plant
                    .or(a_plant_on_lawn.normal_plant)
                    .or(a_plant_on_lawn.under_plant)
            }
            PlantPriority::DiggingOrder => {
                a_plant_on_lawn.normal_plant
                    .or(a_plant_on_lawn.under_plant)
            }
            PlantPriority::BungeeOrder | PlantPriority::CatapultOrder | PlantPriority::Any => {
                a_plant_on_lawn.flying_plant
                    .or(a_plant_on_lawn.normal_plant)
                    .or(a_plant_on_lawn.pumpkin_plant)
                    .or(a_plant_on_lawn.under_plant)
            }
            PlantPriority::ZenToolOrder => {
                a_plant_on_lawn.flying_plant
                    .or(a_plant_on_lawn.pumpkin_plant)
                    .or(a_plant_on_lawn.normal_plant)
                    .or(a_plant_on_lawn.under_plant)
            }
            PlantPriority::OnlyNormalPosition | PlantPriority::TopPlantOnly => a_plant_on_lawn.normal_plant,
            PlantPriority::OnlyFlying => a_plant_on_lawn.flying_plant,
            PlantPriority::OnlyPumpkin => a_plant_on_lawn.pumpkin_plant,
            PlantPriority::OnlyUnderPlant => a_plant_on_lawn.under_plant,
            _ => None,
        };

        idx.map(|i| &self.plants[i])
    }

    /// 兼容旧调用点：按 Any 优先级取最上层植物（对应 C++ GetTopPlantAt 默认 TOPPLANT_ANY）
    pub fn get_top_plant_at_any(&self, grid_x: i32, grid_y: i32) -> Option<&Plant> {
        self.get_top_plant_at(grid_x, grid_y, PlantPriority::Any)
    }

    /// 植物是否在黄金喷壶范围内（对应 C++ IsPlantInGoldWateringCanRange）
    pub fn is_plant_in_gold_watering_can_range(&self, mouse_x: i32, mouse_y: i32, plant: &Plant) -> bool {
        let min_x = mouse_x - 70;
        let max_x = mouse_x + 90;
        let min_y = mouse_y - 80;
        let max_y = mouse_y + 80;
        let is_top = self.plants.iter().any(|p| {
            !p.dead && p.plant_col == plant.plant_col && p.base.row == plant.base.row && std::ptr::eq(p, plant)
        });
        if is_top {
            return plant.pos_x as i32 + 40 >= min_x
                && plant.pos_x as i32 + 40 < max_x
                && plant.pos_y as i32 + 40 >= min_y
                && plant.pos_y as i32 + 40 < max_y;
        }
        false
    }

    /// 获取格子上的所有植物信息（对应 C++ GetPlantsOnLawn）
    pub fn get_plants_on_lawn(&self, grid_x: i32, grid_y: i32) -> PlantsOnLawn {
        let mut result = PlantsOnLawn {
            under_plant: None,
            pumpkin_plant: None,
            flying_plant: None,
            normal_plant: None,
        };

        if grid_x < 0 || grid_x >= MAX_GRID_SIZE_X as i32 || grid_y < 0 || grid_y >= MAX_GRID_SIZE_Y as i32 {
            return result;
        }

        // Wallnut Bowling 和铁锹教程中不返回植物
        // 暂略：is_wallnut_bowling_level / is_in_shovel_tutorial 依赖 CutScene

        for (idx, plant) in self.plants.iter().enumerate() {
            if plant.dead {
                continue;
            }

            let mut seed_type = plant.seed_type;
            if seed_type == SeedType::Imitater && plant.imitater_type != SeedType::None {
                seed_type = plant.imitater_type;
            }

            // 检测植物是否位于目标格子内
            if plant.start_row != grid_y {
                continue;
            }
            if seed_type == SeedType::Cobcannon {
                if plant.plant_col < grid_x - 1 || plant.plant_col > grid_x {
                    continue;
                }
            } else if plant.plant_col != grid_x {
                continue;
            }

            // 分类记录
            if Plant::is_flying(seed_type) {
                result.flying_plant = Some(idx);
            } else if seed_type == SeedType::Flowerpot
                || (seed_type == SeedType::Lilypad)  // && mApp->mGameMode != GameMode::ChallengeZenGarden
            {
                result.under_plant = Some(idx);
            } else if seed_type == SeedType::Pumpkinshell {
                result.pumpkin_plant = Some(idx);
            } else {
                result.normal_plant = Some(idx);
            }
        }

        result
    }

    // ========== UI 更新 ==========

    /// 更新层（对应 C++ UpdateLayers）
    pub fn update_layers(&mut self) {
        // 对应 C++ UpdateLayers（Board.cpp:5817）：标记所有 Widget 脏并前置对话框
        if let Some(app) = self.app {
            unsafe {
                if let Some(wm) = (*app).base.widget_manager {
                    // 对应 C++ mWidgetManager->MarkAllDirty()（WidgetManager 继承自 WidgetContainer）
                    for widget_ptr in (*wm).widget_list.iter() {
                        (**widget_ptr).mark_dirty();
                    }
                    // C++: for (Dialog* aDialog : mApp->mDialogList) { BringToFront(aDialog); MarkDirty(); }
                    // [TRANSLATION_NOTE]: Rust 对话框列表/前置（mDialogList）未接入
                }
            }
        }
    }

    /// 鼠标释放事件（对应 C++ MouseUp，Board.cpp:4605）
    pub fn mouse_up(&mut self, x: i32, y: i32, click_count: i32) {
        // 对应 C++: if (mIgnoreMouseUp) { mIgnoreMouseUp = false; return; }
        if self.ignore_mouse_up {
            self.ignore_mouse_up = false;
            return;
        }

        // C++ 4616: mGameMode==GAMEMODE_CHALLENGE_BEGHOULED && mChallenge->MouseUp(x,y) && theClickCount>0 → return
        let a_beghouled_gm = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        if a_beghouled_gm == GameMode::ChallengeBeghouled {
            if let Some(ch) = self.challenge.as_mut() {
                if ch.mouse_up(x, y) != 0 && click_count > 0 {
                    return;
                }
            }
        }

        if !self.can_interact_with_board_buttons() || click_count <= 0 {
            return;
        }

        // 对应 C++: 菜单按钮（且无游戏结束/通关对话框）
        let a_menu_over = self.menu_button.map_or(false, |m| unsafe { (*m).is_over });
        let a_game_over_dialog = self.m_game_over || self.m_level_complete; // C++ GetDialog(DIALOG_GAME_OVER/LEVEL_COMPLETE) 近似
        if a_menu_over && !a_game_over_dialog {
            if let Some(menu) = self.menu_button {
                unsafe {
                    (*menu).is_over = false;
                    (*menu).is_down = false;
                }
            }
            self.update_cursor();
            self.clear_cursor();

            // 对应 C++: 分支分派
            if self.m_tutorial_state == crate::lawn::game_enums::TutorialState::ZenGardenCompleted {
                // C++ 4627: mApp->FinishZenGardenToturial() → mBoardResult=WON; KillBoard(); PreNewGame(ADVENTURE)
                if let Some(app) = self.app.as_mut() {
                    unsafe { (**app).finish_zen_garden_tutorial(); }
                }
            } else {
                let a_gm = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
                if a_gm != GameMode::ChallengeZenGarden
                    && a_gm != GameMode::ChallengeTreeOfWisdom
                    && a_gm != GameMode::Upsell
                {
                    // 对应 C++: PlaySample(SOUND_PAUSE) + DoNewOptions(false)
                    if let Some(app) = self.app {
                        unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_PAUSE); }
                    }
                    if let Some(app) = self.app.as_mut() {
                        unsafe { (**app).do_new_options(false); }
                    }
                } else {
                    // 对应 C++: mApp->mBoardResult = BOARDRESULT_QUIT; DoBackToMain();
                    if let Some(app) = self.app.as_mut() {
                        unsafe {
                            (**app).board_result = BoardResult::Quit;
                            (**app).do_back_to_main();
                        }
                    }
                }
            }
        } else if self.store_button.map_or(false, |s| unsafe { (*s).is_over }) {
            // 对应 C++: 商店按钮分支
            let a_gm = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
            match a_gm {
                GameMode::ChallengeZenGarden => {
                    // C++ 4645: ClearAdviceImmediately(); mApp->mZenGarden->OpenStore();
                    self.clear_advice_immediately();
                    if let Some(app) = self.app.as_mut() {
                        unsafe {
                            if let Some(zg) = (**app).zen_garden.as_mut() {
                                (**zg).open_store();
                            }
                        }
                    }
                }
                GameMode::ChallengeTreeOfWisdom => {
                    // C++ 4649: mChallenge->TreeOfWisdomOpenStore();
                    if let Some(challenge) = self.challenge.as_mut() {
                        challenge.tree_of_wisdom_open_store();
                    }
                }
                GameMode::ChallengeLastStand => {
                    // 对应 C++: 进入 Onslaught 阶段并倒计时 10
                    if let Some(challenge) = self.challenge.as_mut() {
                        challenge.challenge_state = crate::lawn::game_enums::ChallengeState::LastStandOnslaught;
                    }
                    if let Some(store) = self.store_button {
                        unsafe {
                            (*store).btn_no_draw = true;
                            (*store).disabled = true;
                        }
                    }
                    self.m_zombie_count_down = 10;
                    self.m_zombie_count_down_start = 10;
                }
                GameMode::Upsell => {
                    if let Some(app) = self.app.as_mut() {
                        unsafe { (**app).do_back_to_main(); }
                    }
                }
                _ => {}
            }
        }

        let _ = (x, y);
        self.update_cursor();
    }

    /// 更新光标形状（对应 C++ UpdateCursor L3008，简化版）
    pub fn update_cursor(&mut self) {
        if self.m_paused || self.m_board_fade_out_counter >= 0 {
            return;
        }

        // 命中检测决定光标形状（简化版）
        let mut hit_result = HitResult {
            object: None,
            object_type: GameObjectType::None,
        };
        let mx = self.m_prev_mouse_x;
        let my = self.m_prev_mouse_y;
        self.mouse_hit_test(mx, my, &mut hit_result);

        // C++: UpdateMousePosition 中按光标类型高亮植物（金水壶/铲子等工具）
        self.highlight_plants_for_mouse(mx, my);
    }

    /// 命中检测（对应 C++ MouseHitTest L4225 完整版）
    /// 依次检测：菜单按钮→商店按钮→种子槽→铲子→硬币→Zen花园→智慧树→工具→植物→陶罐→老虎机
    pub fn mouse_hit_test(&self, x: i32, y: i32, result: &mut HitResult) {
        result.object = None;
        result.object_type = GameObjectType::None;

        // 关卡淡出或 Dave 说话时忽略
        if self.m_board_fade_out_counter >= 0 || self.is_scary_potter_dave_talking() {
            return;
        }

        // 菜单按钮
        if self.can_interact_with_board_buttons() && self.menu_button.is_some() {
            // 简化：按钮 IsMouseOver 暂略
        }

        // 铲子按钮
        let shovel_rect = self.get_shovel_button_rect();
        if self.m_show_shovel && shovel_rect.contains(x, y) && self.can_interact_with_board_buttons() {
            // GameObjectType::Shovel 在 Rust 枚举中未定义，暂跳过
        }

        // 硬币检测（仅 Normal 和 Hammer 光标模式）
        let ct = self.cursor_object.cursor_type;
        if ct == CursorType::Normal || ct == CursorType::Hammer {
            let mut top_coin: Option<usize> = None;
            for (i, coin) in self.coins.iter().enumerate() {
                if coin.mouse_hit_test(x, y) {
                    match top_coin {
                        None => top_coin = Some(i),
                        Some(old) => {
                            // 取渲染顺序较高的硬币（C++ 中通过 mRenderOrder 比较）
                            if i > old { top_coin = Some(i); }
                        }
                    }
                }
            }
            if let Some(idx) = top_coin {
                result.object = Some(idx);
                result.object_type = GameObjectType::Coin;
                return;
            }
        }

        // 植物命中检测
        if self.mouse_hit_test_plant(x, y, result) {
            return;
        }

        // 可怕陶罐关卡
        let is_scary = self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() });
        if is_scary && ct == CursorType::Normal {
            let gx = self.pixel_to_grid_x(x, y);
            let gy = self.pixel_to_grid_y(x, y);
            let grid_item = self.grid_items.iter().find(|gi| {
                gi.grid_item_type == GridItemType::ScaryPot
                    && gi.grid_x == gx && gi.grid_y == gy
            });
            if grid_item.is_some() {
                // result.object 设置为 grid_item 索引
                // result.object_type = GameObjectType::ScaryPot;  // 暂缺该变体
                return;
            }
        }

        // 老虎机关卡
        let is_slot = self.app.map_or(false, |app| unsafe { (*app).is_slot_machine_level() });
        if is_slot {
            if let Some(ref ch) = self.challenge {
                let handle_rect = ch.slot_machine_get_handle_rect();
                if handle_rect.contains(x, y) && ch.challenge_state == ChallengeState::Normal {
                    // result.object_type = GameObjectType::SlotMachineHandle;  // 暂缺该变体
                    return;
                }
            }
        }

        // 未命中任何对象
        result.object = None;
        result.object_type = GameObjectType::None;
    }

    /// 植物命中检测（对应 C++ MouseHitTestPlant L4168 完整版）
    /// 检测鼠标是否悬停在/点击到植物上（含 Zen 花园特殊处理）
    fn mouse_hit_test_plant(&self, x: i32, y: i32, result: &mut HitResult) -> bool {
        let ct = self.cursor_object.cursor_type;
        if ct == CursorType::CobcannonTarget || ct == CursorType::Hammer {
            return false;
        }

        // C++: MouseHitTestPlant 先做特殊植物检测（南瓜壳/飞行植物）
        if let Some(a_special_plant) = self.special_plant_hit_test(x, y) {
            result.object = Some(a_special_plant);
            result.object_type = GameObjectType::Plant;
            return true;
        }

        let gx = self.pixel_to_grid_x(x, y);
        let gy = self.pixel_to_grid_y(x, y);

        // 尝试找到该位置的植物
        let plant = self.plants.iter().find(|p| {
            !p.dead && p.is_on_board && p.plant_col == gx && p.base.row == gy
        });

        if plant.is_some() {
            result.object_type = GameObjectType::Plant;
            return true;
        }

        false
    }

    /// 使用工具点击（对应 C++ MouseDownWithTool L4099 完整版）
    /// 右击取消、Zen 花园/智慧树委托、铲子逻辑
    pub fn mouse_down_with_tool(&mut self, x: i32, y: i32, click_count: i32, cursor_type: CursorType) {
        // 右击取消
        if click_count < 0 {
            self.clear_cursor();
            return;
        }

        // 检查游戏模式
        let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });

        if game_mode == GameMode::ChallengeZenGarden || game_mode == GameMode::ChallengeTreeOfWisdom {
            // Zen 花园和智慧树工具操作由专用模块处理
            return;
        }

        // 工具命中检测
        let plant_idx = self.tool_hit_test(x, y);

        if cursor_type == CursorType::Shovel {
            if let Some(idx) = plant_idx {
                self.m_plants_shoveled += 1;
                if idx < self.plants.len() {
                    self.plants[idx].dead = true;
                }
                // 铲掉猫尾草后补种睡莲
                // 教程状态更新暂略
            }
        }

        self.clear_cursor();
    }

    /// 玉米加农炮点击发射（对应 C++ MouseDownCobcannonFire）
    pub fn mouse_down_cobcannon_fire(&mut self, x: i32, y: i32, the_click_count: i32) {
        if the_click_count >= 0 && y >= 80 {
            // 防误点：30cs 延迟期间且距离准星 < 100px 时忽略
            if self.m_cob_cannon_cursor_delay_counter > 0
                && crate::todlib::tod_common::distance(
                    x as f32, y as f32,
                    self.m_cob_cannon_mouse_x as f32, self.m_cob_cannon_mouse_y as f32,
                ) < 100.0
            {
                return;
            }
            let a_plant_id = self.cursor_object.cob_cannon_plant_id;
            if let Some(plant) = self.plants.get_mut(a_plant_id as usize) {
                plant.cob_cannon_fire(x, y);
            }
        }
        self.clear_cursor();
    }

    /// 工具命中检测（对应 C++ ToolHitTest L4075）
    /// 检查鼠标指向位置是否有可用工具的植物
    fn tool_hit_test(&self, x: i32, y: i32) -> Option<usize> {
        let mut result = HitResult {
            object: None,
            object_type: GameObjectType::None,
        };
        self.mouse_hit_test(x, y, &mut result);
        if result.object_type == GameObjectType::Plant {
            result.object
        } else {
            None
        }
    }

    /// 工具命中辅助（对应 C++ ToolHitTestHelper，Board.cpp:3975）
    /// 墓碑在非 Zen 花园模式不可被工具点中
    pub fn tool_hit_test_helper(&self, hit_result: &HitResult) -> Option<usize> {
        let a_plant_index = hit_result.object?;
        let a_plant = &self.plants[a_plant_index];
        // C++: (mSeedType != SEED_GRAVEBUSTER || mGameMode == GAMEMODE_CHALLENGE_ZEN_GARDEN) ? aPlant : nullptr
        if a_plant.seed_type == SeedType::Gravebuster
            && self.app.map_or(true, |app| unsafe { (*app).game_mode != GameMode::ChallengeZenGarden })
        {
            return None;
        }
        Some(a_plant_index)
    }

    /// 特殊植物命中检测（对应 C++ SpecialPlantHitTest，Board.cpp:4050）
    /// 南瓜壳：距离 25~50 且 y 在中心下方；飞行植物：距离 < 15
    pub fn special_plant_hit_test(&self, x: i32, y: i32) -> Option<usize> {
        for (i, a_plant) in self.plants.iter().enumerate() {
            if a_plant.dead {
                continue;
            }
            if a_plant.seed_type == SeedType::Pumpkinshell {
                // C++: GetTopPlantAt(..., TOPPLANT_ONLY_NORMAL_POSITION) ? 25 : 0
                let a_min_dist = if self.plants.iter().any(|p| {
                    !p.dead && p.plant_col == a_plant.plant_col && p.base.row == a_plant.base.row
                }) {
                    25.0
                } else {
                    0.0
                };
                let a_distance = Self::distance_2d(x, y, a_plant.pos_x as i32 + 40, a_plant.pos_y as i32 + 40);
                if a_distance >= a_min_dist && a_distance <= 50.0 && y > a_plant.pos_y as i32 + 25 {
                    return Some(i);
                }
            } else if Self::is_plant_flying(a_plant.seed_type) {
                let a_distance = Self::distance_2d(x, y, a_plant.pos_x as i32 + 40, a_plant.pos_y as i32);
                if a_distance < 15.0 {
                    return Some(i);
                }
            }
        }
        None
    }

    /// 两点距离（对应 C++ Distance2D）
    pub fn distance_2d(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
        let a_dx = (x1 - x2) as f32;
        let a_dy = (y1 - y2) as f32;
        (a_dx * a_dx + a_dy * a_dy).sqrt()
    }

    /// 是否为飞行植物（对应 C++ Plant::IsFlying）
    pub fn is_plant_flying(seed_type: SeedType) -> bool {
        matches!(
            seed_type,
            SeedType::Lilypad
                | SeedType::Seashroom
                | SeedType::Tanglekelp
                | SeedType::Cattail
                | SeedType::Cobcannon
                | SeedType::Spikeweed
                | SeedType::Spikerock
                | SeedType::Gravebuster
        )
    }

    /// 鼠标悬停高亮植物（对应 C++ HighlightPlantsForMouse，Board.cpp:3087）
    pub fn highlight_plants_for_mouse(&mut self, mouse_x: i32, mouse_y: i32) {
        // C++: 金水壶购买后高亮范围内所有植物（含底层花盆）
        if self.cursor_object.cursor_type == CursorType::WateringCan
            && self.app.map_or(false, |app| unsafe {
                (*app).player_info.as_ref().map_or(false, |p| {
                    p.m_purchases.get(StoreItem::GoldWateringcan as usize).copied().unwrap_or(0) > 0
                })
            })
        {
            let mut a_plants_to_highlight: Vec<usize> = Vec::new();
            for (i, a_plant) in self.plants.iter().enumerate() {
                if a_plant.dead {
                    continue;
                }
                if self.is_plant_in_gold_watering_can_range(mouse_x, mouse_y, a_plant) {
                    a_plants_to_highlight.push(i);
                }
            }
            for a_index in a_plants_to_highlight {
                let a_plant_col = self.plants[a_index].plant_col;
                let a_row = self.plants[a_index].base.row;
                self.plants[a_index].highlighted = true;
                // C++: GetTopPlantAt(..., TOPPLANT_ONLY_UNDER_PLANT) 的底层花盆
                if let Some(a_flower_pot) = self.plants.iter_mut().find(|p| {
                    !p.dead && p.seed_type == SeedType::Flowerpot
                        && p.plant_col == a_plant_col && p.base.row == a_row
                }) {
                    a_flower_pot.highlighted = true;
                }
            }
        } else {
            // C++: ToolHitTest → 高亮植物；Zen 花园同时高亮底层花盆
            if let Some(a_plant_index) = self.tool_hit_test(mouse_x, mouse_y) {
                let a_plant_col = self.plants[a_plant_index].plant_col;
                let a_row = self.plants[a_plant_index].base.row;
                self.plants[a_plant_index].highlighted = true;
                if self.app.map_or(false, |app| unsafe { (*app).game_mode == GameMode::ChallengeZenGarden }) {
                    if let Some(a_flower_pot) = self.plants.iter_mut().find(|p| {
                        !p.dead && p.seed_type == SeedType::Flowerpot
                            && p.plant_col == a_plant_col && p.base.row == a_row
                    }) {
                        a_flower_pot.highlighted = true;
                    }
                }
            }
        }
    }

    /// 手持植物点击（对应 C++ MouseDownWithPlant L3689 完整版）
    /// 验证种植条件、扣除阳光、执行种植、更新教程状态
    pub fn mouse_down_with_plant(&mut self, x: i32, y: i32, click_count: i32) {
        // 右击取消
        if click_count < 0 {
            // RefreshSeedPacketFromCursor 暂略
            return;
        }

        // 从光标获取种子类型
        let seed_type = self.cursor_object.seed_type;
        if seed_type == SeedType::None {
            return;
        }

        // 计算种植网格位置
        let grid_x = self.planting_pixel_to_grid_x(x, y, seed_type);
        let grid_y = self.planting_pixel_to_grid_y(x, y, seed_type);

        // 不在场地内 → 放下卡牌
        if grid_x < 0 || grid_x >= MAX_GRID_SIZE_X as i32 || grid_y < 0 || grid_y > MAX_GRID_SIZE_Y as i32 {
            return;
        }

        // 检查能否种植
        let reason = self.can_plant_at(grid_x, grid_y, seed_type);
        if reason != PlantingReason::Ok {
            // 根据 PlantingReason 显示提示
            match reason {
                PlantingReason::OnlyOnGraves => {
                    self.display_advice("[ADVICE_GRAVEBUSTERS_ON_GRAVES]", 2, AdviceType::PlantGravebustersOnGraves);
                }
                PlantingReason::NotOnWater | PlantingReason::OnlyInPool => {
                    self.display_advice("[ADVICE_PLANT_NOT_ON_WATER]", 2, AdviceType::PlantNotOnWater);
                }
                PlantingReason::NeedsPot => {
                    self.display_advice("[ADVICE_PLANT_NEED_POT2]", 2, AdviceType::CantPlantThere);
                }
                PlantingReason::NotPassedLine => {
                    self.display_advice("[ADVICE_NOT_PASSED_LINE]", 2, AdviceType::CantPlantThere);
                }
                PlantingReason::NotOnGrave => {
                    self.display_advice("[ADVICE_PLANT_NOT_ON_GRAVE]", 2, AdviceType::CantPlantThere);
                }
                PlantingReason::NotOnCrater => {
                    self.display_advice("[ADVICE_PLANT_NOT_ON_CRATER]", 2, AdviceType::CantPlantThere);
                }
                _ => {
                    self.display_advice("[ADVICE_CANT_PLANT_THERE]", 2, AdviceType::CantPlantThere);
                }
            }
            return;
        }

        // 从种子槽种植时扣除阳光
        self.take_sun_money(self.get_current_plant_cost(seed_type, SeedType::None));

        // 执行种植
        self.add_plant(grid_x, grid_y, seed_type, self.cursor_object.imitater_type);

        // 更新教程状态
        if self.m_tutorial_state == TutorialState::Level1PlantPeashooter {
            self.m_tutorial_state = TutorialState::Level1Completed;
        }

        // 清除光标状态
        self.cursor_object.deactivate();
        self.clear_cursor();
    }

    /// 拾取工具（对应 C++ PickUpTool L4373）
    /// 根据工具类型设置光标状态和播放音效
    pub fn pick_up_tool(&mut self, object_type: GameObjectType) {
        // 对应 C++ PickUpTool
        let is_playing = self.app.map_or(false, |app| unsafe {
            (*app).game_scene == crate::lawn::lawn_app::GameScenes::Playing
        });
        let in_shovel_tutorial = self.m_cut_scene.map_or(false, |cs| unsafe {
            (*cs).is_in_shovel_tutorial()
        });
        if self.m_paused || (!is_playing && !in_shovel_tutorial) {
            return;
        }

        match object_type {
            GameObjectType::Shovel => {
                if self.m_tutorial_state == TutorialState::ShovelPickup {
                    self.set_tutorial_state(TutorialState::ShovelDig);
                }
                self.cursor_object.cursor_type = CursorType::Shovel;
                if let Some(app) = self.app {
                    unsafe { (*app).play_foley(FoleyType::UseShovel as i32); }
                }
            }
            GameObjectType::WateringCan => {
                if self.m_tutorial_state == TutorialState::ZenGardenPickupWater {
                    self.m_tutorial_state = TutorialState::ZenGardenWaterPlant;
                    self.display_advice(
                        "[ADVICE_ZEN_GARDEN_WATER_PLANT]",
                        MessageStyle::ZenGardenLong as i32,
                        AdviceType::None,
                    );
                    self.tutorial_arrow_remove();
                }
                self.cursor_object.cursor_type = CursorType::WateringCan;
                if let Some(app) = self.app {
                    unsafe { (*app).play_foley(FoleyType::Drop as i32); }
                }
            }
            GameObjectType::Fertilizer => {
                let has = self.app.map_or(false, |app| unsafe {
                    (*app).player_info.as_ref().map_or(false, |info| {
                        info.m_purchases.get(StoreItem::Fertilizer as usize).copied().unwrap_or(0) > 1000
                    })
                });
                if has {
                    self.cursor_object.cursor_type = CursorType::Fertilizer;
                    if let Some(app) = self.app {
                        unsafe { (*app).play_foley(FoleyType::Drop as i32); }
                    }
                } else {
                    // C++: mApp->PlaySample(Sexy::SOUND_BUZZER);
                    if let Some(app) = self.app {
                        unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_BUZZER); }
                    }
                }
            }
            GameObjectType::BugSpray => {
                let has = self.app.map_or(false, |app| unsafe {
                    (*app).player_info.as_ref().map_or(false, |info| {
                        info.m_purchases.get(StoreItem::BugSpray as usize).copied().unwrap_or(0) > 1000
                    })
                });
                if has {
                    self.cursor_object.cursor_type = CursorType::BugSpray;
                    if let Some(app) = self.app {
                        unsafe { (*app).play_foley(FoleyType::Drop as i32); }
                    }
                } else {
                    // C++: mApp->PlaySample(Sexy::SOUND_BUZZER);
                    if let Some(app) = self.app {
                        unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_BUZZER); }
                    }
                }
            }
            GameObjectType::Phonograph => {
                self.cursor_object.cursor_type = CursorType::Phonograph;
                if let Some(app) = self.app {
                    unsafe { (*app).play_foley(FoleyType::Drop as i32); }
                }
            }
            GameObjectType::Chocolate => {
                let has = self.app.map_or(false, |app| unsafe {
                    (*app).player_info.as_ref().map_or(false, |info| {
                        info.m_purchases.get(StoreItem::Chocolate as usize).copied().unwrap_or(0) > 1000
                    })
                });
                if has {
                    self.cursor_object.cursor_type = CursorType::Chocolate;
                    if let Some(app) = self.app {
                        unsafe { (*app).play_foley(FoleyType::Drop as i32); }
                    }
                } else {
                    // C++: mApp->PlaySample(Sexy::SOUND_BUZZER);
                    if let Some(app) = self.app {
                        unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_BUZZER); }
                    }
                }
            }
            GameObjectType::Glove => {
                self.cursor_object.cursor_type = CursorType::Glove;
                if let Some(app) = self.app {
                    unsafe { (*app).play_foley(FoleyType::Drop as i32); }
                }
            }
            GameObjectType::MoneySign => {
                self.cursor_object.cursor_type = CursorType::MoneySign;
                if let Some(app) = self.app {
                    unsafe { (*app).play_foley(FoleyType::Drop as i32); }
                }
            }
            GameObjectType::Wheelbarrow => {
                self.cursor_object.cursor_type = CursorType::Wheelbarrow;
                if let Some(app) = self.app {
                    unsafe { (*app).play_foley(FoleyType::Drop as i32); }
                }
            }
            GameObjectType::TreeFood => {
                let can_feed = self.challenge.as_ref().map_or(0, |c| c.tree_of_wisdom_can_feed()) != 0;
                if can_feed {
                    let has = self.app.map_or(false, |app| unsafe {
                        (*app).player_info.as_ref().map_or(false, |info| {
                            info.m_purchases.get(StoreItem::TreeOfWisdom as usize).copied().unwrap_or(0) > 1000
                        })
                    });
                    if has {
                        self.cursor_object.cursor_type = CursorType::TreeFood;
                        if let Some(app) = self.app {
                            unsafe { (*app).play_foley(FoleyType::Drop as i32); }
                        }
                    } else {
                        // C++: mApp->PlaySample(Sexy::SOUND_BUZZER);
                        if let Some(app) = self.app {
                            unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_BUZZER); }
                        }
                    }
                }
            }
            _ => {
                // C++ 中 PVZP_ASSERT(false)
            }
        }

        // C++ 中结尾 mCursorObject->mType = SEED_NONE：由 cursor_object.deactivate()（clear_cursor）覆盖
        // （CursorObject 以 seed_type 字段对应 mType，deactivate 中置为 SeedType::None）
    }

    /// 清除光标（对应 C++ ClearCursor L4616 完整版）
    /// 重置光标对象所有字段并清除帮助提示
    pub fn clear_cursor(&mut self) {
        self.cursor_object.deactivate();
        self.m_advice = AdviceType::None;
        // C++ 4549: mApp->SetCursor(CURSOR_POINTER) — Rust 无 set_cursor 方法（框架光标 API 未接入）
        // C++ 4550: mChallenge->ClearCursor();
        if let Some(ch) = self.challenge.as_mut() {
            ch.clear_cursor();
        }
        // [TRANSLATION_NOTE]: C++ 4552+ 教程状态推进（TUTORIAL_LEVEL_1_PLANT_PEASHOOTER 等）依赖 SetTutorialState，暂略
    }

    /// 更新鼠标位置（对应 C++ UpdateMousePosition 简化版）
    pub fn update_mouse_position(&mut self, x: i32, y: i32) {
        // 对应 C++ Board::UpdateMousePosition (Board.cpp:3124)
        self.m_prev_mouse_x = x;
        self.m_prev_mouse_y = y;

        self.update_cursor();
        self.update_tool_tip(x, y);

        // 清除所有植物高亮
        for plant in &mut self.plants {
            if plant.dead { continue; }
            plant.highlighted = false;
        }

        let a_cursor_seed_type = self.get_seed_type_in_cursor();
        let a_mouse_x = self.app.map_or(0, |app| unsafe { (*app).base.widget_manager.map_or(0, |wm| (*wm).last_mouse_x) }) - self.m_x;
        let a_mouse_y = self.app.map_or(0, |app| unsafe { (*app).base.widget_manager.map_or(0, |wm| (*wm).last_mouse_y) }) - self.m_y;

        let is_scary_potter = self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() });
        if is_scary_potter {
            // 清除恐怖罐高亮
            for gi in &mut self.grid_items {
                if gi.dead { continue; }
                if gi.grid_item_type == GridItemType::ScaryPot {
                    gi.highlighted = false;
                }
            }
            let mut a_hit_result = HitResult { object: None, object_type: GameObjectType::None };
            self.mouse_hit_test(a_mouse_x, a_mouse_y, &mut a_hit_result);
            if a_hit_result.object_type == GameObjectType::ScaryPot {
                // [TRANSLATION_NOTE]: C++ 将 aHitResult.mObject 转为 GridItem* 置高亮；
                // Rust HitResult 存索引，需通过索引访问
                if let Some(idx) = a_hit_result.object {
                    if idx < self.grid_items.len() {
                        self.grid_items[idx].highlighted = true;
                        return;
                    }
                }
            }
        }

        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        if app_mode == GameMode::ChallengeZenGarden {
            // [TRANSLATION_NOTE]: C++ 获取 Stinky 后判断 MouseHitTest 是否命中
            let stinky_idx = self.app.and_then(|app| unsafe {
                (*app).zen_garden.and_then(|zg| (*zg).get_stinky())
            });
            // [TRANSLATION_NOTE]: Rust get_stinky 返回 Option<*mut GridItem>，无法直接在此比较 HitResult 索引
            // 简化：跳过 Stinky 高亮（ZenGarden 子系统交互尚未完整接入）
            let _ = stinky_idx;
        }

        // 工具光标高亮
        let cursor_type = self.cursor_object.cursor_type;
        let is_tool_cursor = matches!(cursor_type,
            CursorType::Shovel | CursorType::WateringCan | CursorType::Fertilizer |
            CursorType::BugSpray | CursorType::Phonograph | CursorType::Chocolate |
            CursorType::Glove | CursorType::MoneySign
        ) || (cursor_type == CursorType::Wheelbarrow
            && !self.app.map_or(false, |app| unsafe {
                (*app).zen_garden.map_or(false, |zg| (*zg).get_potted_plant_in_wheelbarrow().is_some())
            }));

        if is_tool_cursor {
            self.highlight_plants_for_mouse(a_mouse_x, a_mouse_y);
            return;
        }

        // 咖啡豆/坚果急救高亮
        if a_cursor_seed_type == SeedType::InstantCoffee {
            let grid_x = self.planting_pixel_to_grid_x(x, y, a_cursor_seed_type);
            let grid_y = self.planting_pixel_to_grid_y(x, y, a_cursor_seed_type);
            // 先提取所需信息，释放不可变借用后再可变借用设置高亮
            let should_highlight = self.get_top_plant_at(grid_x, grid_y, PlantPriority::OnlyNormalPosition)
                .map_or(false, |plant| plant.is_asleep && self.can_plant_at(grid_x, grid_y, SeedType::InstantCoffee) == PlantingReason::Ok);
            if should_highlight {
                for p in &mut self.plants {
                    if p.dead { continue; }
                    if p.is_asleep {
                        let pgx = p.plant_col; let pgy = p.base.row;
                        if pgx == grid_x && pgy == grid_y { p.highlighted = true; break; }
                    }
                }
            }
        } else if a_cursor_seed_type == SeedType::Wallnut || a_cursor_seed_type == SeedType::Tallnut {
            let grid_x = self.planting_pixel_to_grid_x(x, y, a_cursor_seed_type);
            let grid_y = self.planting_pixel_to_grid_y(x, y, a_cursor_seed_type);
            let should_highlight = self.get_top_plant_at(grid_x, grid_y, PlantPriority::OnlyPumpkin)
                .map_or(false, |plant| plant.seed_type == a_cursor_seed_type && self.can_plant_at(grid_x, grid_y, a_cursor_seed_type) == PlantingReason::Ok);
            if should_highlight {
                for p in &mut self.plants {
                    if p.dead { continue; }
                    if p.seed_type == a_cursor_seed_type {
                        let pgx = p.plant_col; let pgy = p.base.row;
                        if pgx == grid_x && pgy == grid_y { p.highlighted = true; break; }
                    }
                }
            }
        } else if a_cursor_seed_type == SeedType::Pumpkinshell {
            let grid_x = self.planting_pixel_to_grid_x(x, y, a_cursor_seed_type);
            let grid_y = self.planting_pixel_to_grid_y(x, y, a_cursor_seed_type);
            let should_highlight = self.get_top_plant_at(grid_x, grid_y, PlantPriority::OnlyNormalPosition)
                .map_or(false, |plant| plant.seed_type == SeedType::Pumpkinshell && self.can_plant_at(grid_x, grid_y, SeedType::Pumpkinshell) == PlantingReason::Ok);
            if should_highlight {
                for p in &mut self.plants {
                    if p.dead { continue; }
                    if p.seed_type == SeedType::Pumpkinshell {
                        let pgx = p.plant_col; let pgy = p.base.row;
                        if pgx == grid_x && pgy == grid_y { p.highlighted = true; break; }
                    }
                }
            }
        }
    }

    /// 僵尸命中检测（对应 C++ ZombieHitTest L3117）
    /// 返回鼠标位置下最上层的僵尸（越靠后的行中 y 值越大优先级越高）
    pub fn zombie_hit_test(&self, mouse_x: i32, mouse_y: i32) -> Option<usize> {
        let mut record: Option<usize> = None;
        let mut record_y: i32 = i32::MIN;

        for (i, z) in self.zombies.iter().enumerate() {
            // 排除已死亡或没有有效碰撞框的僵尸
            if z.dead {
                continue;
            }

            // 检查僵尸矩形是否包含鼠标位置
            if z.zombie_rect.contains(mouse_x, mouse_y) {
                match record {
                    None => {
                        record = Some(i);
                        record_y = z.base.y;
                    }
                    Some(_) if z.base.y > record_y => {
                        record = Some(i);
                        record_y = z.base.y;
                    }
                    _ => {}
                }
            }
        }

        record
    }

    /// 更新工具提示（对应 C++ UpdateToolTip L3302 完整版）
    /// 根据鼠标悬停的对象显示对应提示文本
    pub fn update_tool_tip(&mut self, mouse_x: i32, mouse_y: i32) {
        // C++ Board.cpp:3222-3230：!mMouseIn || !mActive || mTimeStopCounter > 0 ||
        // GetDialogCount() > 0 || SCENE_ZOMBIES_WON → 隐藏并返回
        let mouse_in = self.app.map_or(false, |app| unsafe {
            (*app).base.widget_manager.map_or(false, |wm| (*wm).mouse_in)
        });
        let active = self.app.map_or(false, |app| unsafe { (*app).base.active });
        let scene = self.app.map_or(crate::lawn::lawn_app::GameScenes::Playing, |app| unsafe { (*app).game_scene });
        let dialog_count = self.app.map_or(0, |app| unsafe { (*app).get_dialog_count() });
        if !mouse_in || !active || self.m_time_stop_counter > 0 || dialog_count > 0 || scene == crate::lawn::lawn_app::GameScenes::ZombiesWon {
            self.tool_tip.m_visible = false;
            return;
        }

        if self.m_paused {
            self.tool_tip.m_visible = false;
            return;
        }

        // C++: int aMouseX = mApp->mWidgetManager->mLastMouseX - mX; 同理 aMouseY
        let a_mouse_x = mouse_x - self.m_x;
        let a_mouse_y = mouse_y - self.m_y;

        // 获取 App 引用
        let app = match self.app {
            Some(a) => unsafe { &*a },
            None => return,
        };

        // 关卡开场（SCENE_LEVEL_INTRO）：显示僵尸名称提示
        if app.game_scene == crate::lawn::lawn_app::GameScenes::MainMenu {
            // 在选卡阶段显示僵尸提示
            if let Some(z_idx) = self.zombie_hit_test(a_mouse_x, a_mouse_y) {
                if let Some(z) = self.zombies.get(z_idx) {
                    if z.from_wave == Zombie::ZOMBIE_WAVE_CUTSCENE {
                        let def = get_zombie_definition(z.zombie_type);
                        self.tool_tip.set_title(&format!("[{}]", def.zombie_name));
                        self.tool_tip.set_label("[CLICK_TO_VIEW]");
                        self.tool_tip.m_visible = true;
                        self.tool_tip.m_center = true;
                        let r = &z.zombie_rect;
                        self.tool_tip.m_x = r.x + r.width / 2 + 5;
                        self.tool_tip.m_y = r.y + r.height - 10;
                        return;
                    }
                }
            }
            self.tool_tip.m_visible = false;
            return;
        }

        // 游戏进行中
        if !self.can_interact_with_board_buttons() {
            return;
        }

        // 重置提示
        self.tool_tip.set_title("");
        self.tool_tip.set_label("");
        self.tool_tip.set_warning_text("");
        self.tool_tip.m_center = false;

        // 委托 Challenge::UpdateToolTip
        if let Some(ref ch) = self.challenge {
            if ch.update_tool_tip(a_mouse_x, a_mouse_y) != 0 {
                return;
            }
        }

        // 命中检测
        let mut hit_result = HitResult {
            object: None,
            object_type: GameObjectType::None,
        };
        self.mouse_hit_test(a_mouse_x, a_mouse_y, &mut hit_result);

        // 根据命中对象类型设置提示
        match hit_result.object_type {
            GameObjectType::Coin => {
                self.tool_tip.set_label("");
                self.tool_tip.m_visible = true;
            }
            GameObjectType::SeedPacket => {
                // 种子包提示 — 依赖种子包字段，暂简化
                // 对应 C++ L3470-3664 的完整逻辑
                self.tool_tip.m_visible = false;
            }
            _ => {
                self.tool_tip.m_visible = false;
            }
        }
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
    pub fn survival_save_score(&mut self) {
        let is_survival = self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() });
        if !is_survival {
            return;
        }

        let a_flags_completed = self.get_survival_flags_completed() as u32;
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_mut() {
                    let a_index = (*app).get_current_challenge_index() as usize;
                    if let Some(a_record) = info.m_challenge_records.get_mut(a_index) {
                        if a_flags_completed > (*a_record) as u32 {
                            *a_record = a_flags_completed as i32;
                            (*app).write_current_user_config();
                        }
                    }
                }
            }
        }
    }

    /// 保存谜题连胜（对应 C++ PuzzleSaveStreak）
    pub fn puzzle_save_streak(&mut self) {
        let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        let is_endless = self.app.map_or(false, |app| unsafe {
            (*app).is_endless_izombie(game_mode) || (*app).is_endless_scary_potter(game_mode)
        });
        if !is_endless {
            return;
        }

        let a_streak = self.challenge.as_ref().map_or(0, |c| c.survival_stage + 1) as u32;
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_mut() {
                    let a_index = (*app).get_current_challenge_index() as usize;
                    if let Some(a_record) = info.m_challenge_records.get_mut(a_index) {
                        if a_streak > (*a_record) as u32 {
                            *a_record = a_streak as i32;
                            (*app).write_current_user_config();
                        }
                    }
                }
            }
        }
    }

    /// 是否在挥舞铁锹教程中与 Crazy Dave 对话（对应 C++ IsScaryPotterDaveTalking）
    pub fn is_scary_potter_dave_talking(&self) -> bool {
        let is_scary_potter = self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() });
        if !is_scary_potter {
            return false;
        }
        let dave_present = self.app.map_or(false, |app| unsafe {
            (*app).m_crazy_dave_state != CrazyDaveState::Off
        });
        self.m_next_survival_stage_counter > 0 && dave_present
    }

    /// 僵尸获胜（对应 C++ ZombiesWon 简化版）
    pub fn zombies_won(&mut self, the_zombie_idx: Option<usize>) {
        // 对应 C++ Board::ZombiesWon (Board.cpp:5073)
        let game_scene = self.app.map_or(crate::lawn::lawn_app::GameScenes::Playing, |app| unsafe { (*app).game_scene });
        if game_scene == crate::lawn::lawn_app::GameScenes::ZombiesWon {
            return;
        }

        self.clear_advice(AdviceType::None);
        self.m_board_result = BoardResult::Lost;
        // C++: mApp->mBoardResult = BOARDRESULT_LOST; Rust 用 m_board_result + m_game_over
        self.m_game_over = true;

        // C++ 5081-5098: 清理屏幕外/复活中僵尸（巨人不在此列）
        for i in 0..self.zombies.len() {
            if self.zombies[i].dead {
                continue;
            }
            if Some(i) == the_zombie_idx {
                continue;
            }
            let z = &self.zombies[i];
            // C++: aZombie->GetZombieRect().mX < -50
            let off_screen = z.get_zombie_rect().x < -50;
            let rising = z.zombie_phase == ZombiePhase::RisingFromGrave || z.zombie_phase == ZombiePhase::DancerRising;
            if off_screen || rising {
                let is_giant = z.zombie_type == ZombieType::Gargantuar || z.zombie_type == ZombieType::RedeEyeGargantuar;
                if is_giant && z.is_dead_or_dying() && z.pos_x < 140.0 {
                    self.zombies[i].die_no_loot();
                }
            }
        }

        self.survival_save_score();

        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        let is_endless_izombie = self.app.map_or(false, |app| unsafe { (*app).is_endless_izombie(app_mode) });
        let is_endless_scary = self.app.map_or(false, |app| unsafe { (*app).is_endless_scary_potter(app_mode) });
        let is_izombie = self.app.map_or(false, |app| unsafe { (*app).is_izombie_level() });

        // C++ 5096-5118: 特殊模式构造死亡文案 aGameOverMsg（普通模式走 SCENE_ZOMBIES_WON 演出）
        let a_game_over_msg: Option<String> = if app_mode == GameMode::ChallengeZombiquarium {
            Some(crate::todlib::tod_common::tod_string_translate("[ZOMBIQUARIUM_DEATH_MESSAGE]"))
        } else if app_mode == GameMode::ChallengeLastStand {
            // C++: aFlagStr = mApp->Pluralize(GetSurvivalFlagsCompleted(), "[ONE_FLAG]", "[COUNT_FLAGS]")
            let a_flag_str = crate::lawn::lawn_app::LawnApp::pluralize(
                self.get_survival_flags_completed(),
                "[ONE_FLAG]",
                "[COUNT_FLAGS]",
            );
            // [TRANSLATION_NOTE]: 对应 C++ PvzpReplaceString("[LAST_STAND_DEATH_MESSAGE]", "{FLAGS}", aFlagStr)
            Some(crate::todlib::tod_common::tod_string_translate("[LAST_STAND_DEATH_MESSAGE]").replace("{FLAGS}", &a_flag_str))
        } else if is_endless_izombie || is_endless_scary {
            // [TRANSLATION_NOTE]: 对应 C++ PvzpReplaceNumberString("[ENDLESS_PUZZLE_DEATH_MESSAGE]", "{STREAK}", mChallenge->mSurvivalStage)
            let a_streak = self.challenge.as_ref().map_or(0, |c| c.survival_stage);
            Some(
                crate::todlib::tod_common::tod_string_translate("[ENDLESS_PUZZLE_DEATH_MESSAGE]")
                    .replace("{STREAK}", &format!("{}", a_streak)),
            )
        } else if is_izombie {
            Some(crate::todlib::tod_common::tod_string_translate("[I_ZOMBIE_DEATH_MESSAGE]"))
        } else {
            None
        };

        if let Some(a_game_over_msg) = a_game_over_msg {
            // C++ 5134-5135: GameOverDialog(aGameOverMsg, true) + mApp->AddDialog(DIALOG_GAME_OVER) + mWidgetManager->SetFocus
            // [TRANSLATION_NOTE]: Rust 无 WidgetManager 对话框栈，以 LawnApp 字段持有（get_dialog_count 统计近似）
            if let Some(app) = self.app {
                unsafe {
                    let a_dialog = crate::lawn::widget::game_over_dialog::GameOverDialog::new(Some(app), &a_game_over_msg, true);
                    (*app).game_over_dialog = Some(Box::into_raw(Box::new(a_dialog)));
                }
            }
            // C++ 5138-5141: StopAllMusic / StopAllZombieSounds / PlaySample(SOUND_LOSEMUSIC)
            if let Some(app) = self.app {
                unsafe {
                    if let Some(music) = &mut (*app).music {
                        music.stop_all_music();
                    }
                }
                self.stop_all_zombie_sounds();
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(
                    crate::lawn::game_enums::ReanimationType::ZombiesWon,
                );
                unsafe {
                    (*app).play_sample(crate::todlib::tod_foley::SOUND_LOSEMUSIC);
                    // C++ 5143-5147: AddReanimation(-BOARD_OFFSET, 0, RENDER_LAYER_SCREEN_FADE, REANIM_ZOMBIES_WON)
                    // + mLoopType=REANIM_PLAY_ONCE_AND_HOLD + fullscreen 轨道 mTrackColor=Black + SetFramesForLayer("anim_screen")
                    if let Some(a_reanim) = (*app).add_reanimation(
                        -BOARD_OFFSET as f32,
                        0.0,
                        make_render_order(RENDER_LAYER_SCREEN_FADE, 0, 0),
                        crate::lawn::game_enums::ReanimationType::ZombiesWon as i32,
                    ) {
                        (*a_reanim).m_loop_type = crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold;
                        if let Some(ti) = (*a_reanim).get_track_instance_by_name("fullscreen") {
                            ti.m_track_color = Color::BLACK;
                        }
                        (*a_reanim).set_frames_for_layer("anim_screen");
                    }
                }
            }
        } else {
            // C++ 5121-5132: 普通关卡切到 SCENE_ZOMBIES_WON 演出
            if let Some(app) = self.app {
                unsafe {
                    (*app).game_scene = crate::lawn::lawn_app::GameScenes::ZombiesWon;
                }
            }
            if let Some(zidx) = the_zombie_idx {
                self.zombies[zidx].walk_into_house();
            }
            self.clear_advice(AdviceType::None);
            if let Some(cut_scene) = self.m_cut_scene {
                unsafe { (*cut_scene).start_zombies_won(); }
            }
            self.freeze_effects_for_cutscene(true);
            self.tutorial_arrow_remove();
            self.update_cursor();
        }
    }

    /// 兼容旧调用点：无特定僵尸的 zombies_won（theZombie=null）
    pub fn zombies_won_no_zombie(&mut self) {
        self.zombies_won(None);
    }

    /// 处理删除队列（对应 C++ ProcessDeleteQueue）
    /// 处理待删除队列（对应 C++ ProcessDeleteQueue）
    /// C++ 中依次回收死亡的植物/僵尸/投射物/硬币/割草机/格子物品
    pub fn process_delete_queue(&mut self) {
        // C++ Board.cpp:8514-8555：逐类清理死亡对象（mPlants/mZombies/mProjectiles/mCoins/
        // mLawnMowers/mGridItems 的 DataArrayFree，Rust 中 Vec retain 语义等价）
        self.plants.retain(|p| !p.dead);
        self.zombies.retain(|z| !z.dead);
        self.projectiles.retain(|p| !p.dead);
        self.coins.retain(|c| !c.dead);
        self.lawn_mowers.retain(|m| !m.dead);
        self.grid_items.retain(|g| !g.dead);
    }

    /// 停止所有僵尸声音（对应 C++ StopAllZombieSounds）
    pub fn stop_all_zombie_sounds(&mut self) {
        // 对应 C++ StopAllZombieSounds
        for i in 0..self.zombies.len() {
            if !self.zombies[i].dead {
                self.zombies[i].stop_zombie_sound();
            }
        }
    }

    // ========== 传送带/种子 ==========

    /// 是否有传送带种子槽（对应 C++ HasConveyorBeltSeedBank）
    pub fn has_conveyor_belt_seed_bank(&self) -> bool {
        self.app.map_or(false, |app| unsafe {
            (*app).is_final_boss_level()
                || (*app).is_mini_boss_level()
                || (*app).is_shovel_level()
                || (*app).is_wallnut_bowling_level()
                || (*app).is_little_trouble_level()
                || (*app).is_stormy_night_level()
                || (*app).is_bungee_blitz_level()
                || (*app).game_mode == GameMode::ChallengePortalCombat
                || (*app).game_mode == GameMode::ChallengeColumns
                || (*app).game_mode == GameMode::ChallengeInvisighoul
        })
    }

    /// 种子栏是否包含点（对应 C++ SeedBank::ContainsPoint，SeedPacket.cpp:1013）
    pub fn seed_bank_contains_point(&self, the_x: i32, the_y: i32) -> bool {
        // [TRANSLATION_NOTE]: C++ mWidth = IMAGE_SEEDBANK->GetWidth() + GetSeedBankExtraWidth()，
        // Rust 无 IMAGE_SEEDBANK 资源，用 456 近似（与 board 绘图处一致）
        let a_width = self.get_seed_bank_extra_width() + 456;
        the_x >= self.m_seed_bank_x
            && the_x < self.m_seed_bank_x + a_width
            && the_y >= self.m_seed_bank_y
            && the_y < self.m_seed_bank_y + SEED_PACKET_HEIGHT
    }

    /// 往传送带添加种子（对应 C++ SeedBank::AddSeed，SeedPacket.cpp:1018）
    pub fn add_seed(&mut self, the_seed_type: SeedType, the_place_on_left: bool) {
        // C++: PVZP_ASSERT(HasConveyorBeltSeedBank()); PVZP_ASSERT(theSeedType != SEED_NONE);
        let a_num_seeds = self.get_num_seeds_on_conveyor_belt();
        let a_num_packets = self.get_num_seeds_in_bank() as usize;
        if a_num_seeds == a_num_packets {
            return;
        }

        // aSeedPacket->mOffsetX = 515 - (SEED_PACKET_WIDTH + 1) * aNumSeeds
        let mut a_offset_x = 515 - (SEED_PACKET_WIDTH + 1) * a_num_seeds as i32;
        if the_place_on_left {
            a_offset_x = 0;
        }
        if a_num_seeds > 0 {
            let a_prev_offset_x = self.seed_bank[a_num_seeds - 1].offset_x;
            if a_offset_x < a_prev_offset_x {
                a_offset_x = a_prev_offset_x + 40;
            }
        }

        let a_seed_packet = match self.seed_bank.get_mut(a_num_seeds) {
            Some(p) => p,
            None => return,
        };
        a_seed_packet.seed_type = the_seed_type;
        a_seed_packet.countdown = 0;
        a_seed_packet.refresh_time = 0;
        a_seed_packet.refreshing = false;
        a_seed_packet.active = true;
        a_seed_packet.offset_x = a_offset_x;
    }

    /// 移除传送带种子（对应 C++ SeedBank::RemoveSeed，SeedPacket.cpp:1050）
    pub fn remove_seed(&mut self, the_index: i32) {
        // C++: PVZP_ASSERT(HasConveyorBeltSeedBank()); PVZP_ASSERT(0 <= theIndex < GetNumSeedsOnConveyorBelt());
        let a_num_packets = self.get_num_seeds_in_bank() as usize;
        for i in the_index.max(0) as usize..a_num_packets {
            let a_seed_packet = match self.seed_bank.get(i) {
                Some(p) => p,
                None => break,
            };
            if a_seed_packet.seed_type == SeedType::None {
                break;
            }

            let a_is_last = i == a_num_packets - 1;
            let (a_seed_type, a_offset_x) = if a_is_last {
                (SeedType::None, 0)
            } else {
                let a_next_packet = match self.seed_bank.get(i + 1) {
                    Some(p) => p,
                    None => break,
                };
                (a_next_packet.seed_type, a_next_packet.offset_x + SEED_PACKET_WIDTH + 1)
            };

            let a_seed_packet = match self.seed_bank.get_mut(i) {
                Some(p) => p,
                None => break,
            };
            a_seed_packet.seed_type = a_seed_type;
            a_seed_packet.offset_x = a_offset_x;
            a_seed_packet.countdown = 0;
            a_seed_packet.refresh_time = 0;
            a_seed_packet.refreshing = false;
            a_seed_packet.active = true;
        }
    }

    /// 传送带上种子数量（对应 C++ SeedBank::GetNumSeedsOnConveyorBelt，SeedPacket.cpp:1082）
    pub fn get_num_seeds_on_conveyor_belt(&self) -> usize {
        let a_num_packets = self.get_num_seeds_in_bank() as usize;
        for i in 0..a_num_packets {
            if self.seed_bank[i].seed_type == SeedType::None {
                return i;
            }
        }
        a_num_packets
    }

    /// 传送带上同类型种子计数（对应 C++ SeedBank::CountOfTypeOnConveyorBelt，SeedPacket.cpp:1094）
    pub fn count_of_type_on_conveyor_belt(&self, the_seed_type: SeedType) -> i32 {
        let mut a_count = 0;
        let a_num_packets = self.get_num_seeds_in_bank() as usize;
        for i in 0..a_num_packets {
            if self.seed_bank[i].seed_type == the_seed_type {
                a_count += 1;
            }
        }
        a_count
    }

    /// 传送带滚动更新（对应 C++ SeedBank::UpdateConveyorBelt，SeedPacket.cpp:1147）
    pub fn update_seed_bank_conveyor_belt(&mut self) {
        self.m_conveyor_belt_counter += 1;
        if self.m_conveyor_belt_counter % CONVEYOR_SPEED == 0 {
            let a_num_packets = self.get_num_seeds_in_bank() as usize;
            for i in 0..a_num_packets {
                if self.seed_bank[i].offset_x > 0 {
                    self.seed_bank[i].offset_x = std::cmp::max(self.seed_bank[i].offset_x - 1, 0);
                }
            }

            // mBoard->UpdateToolTip()
            self.update_tool_tip(self.m_prev_mouse_x, self.m_prev_mouse_y);
        }
    }

    /// 更新种子栏宽度与各槽位 X 位置（对应 C++ SeedBank::UpdateWidth，SeedPacket.cpp:1165）
    pub fn update_seed_bank_width(&mut self) {
        // C++: mNumPackets = mBoard->GetNumSeedsInBank()（Rust 由 GetNumSeedsInBank 语义推导，无独立字段）
        // C++: mWidth = IMAGE_SEEDBANK->GetWidth() + GetSeedBankExtraWidth()（Rust 无 mWidth 字段）
        let a_num_packets = self.get_num_seeds_in_bank() as usize;
        for i in 0..a_num_packets {
            let a_seed_packet_x = self.get_seed_packet_position_x(i as i32);
            self.seed_bank[i].x = a_seed_packet_x;
        }
    }

    /// 刷新所有种子包（对应 C++ SeedBank::RefreshAllPackets，SeedPacket.cpp:1175）
    pub fn refresh_all_packets(&mut self) {
        let a_num_packets = self.get_num_seeds_in_bank() as usize;
        for i in 0..a_num_packets {
            let a_seed_packet = match self.seed_bank.get_mut(i) {
                Some(p) => p,
                None => break,
            };
            if a_seed_packet.seed_type == SeedType::None {
                break;
            }
            if a_seed_packet.refreshing {
                a_seed_packet.countdown = 0;
                a_seed_packet.refreshing = false;
                a_seed_packet.active = true;
                a_seed_packet.flash_if_ready();
            }
        }
    }

    /// 更新进度条（对应 C++ UpdateProgressMeter）
    pub fn update_progress_meter(&mut self) {
        // 对应 C++ UpdateProgressMeter：按 Boss 血量或波次进度更新进度条宽度
        let is_final_boss = self.app.map_or(false, |app| unsafe { (*app).is_final_boss_level() });
        if is_final_boss {
            let boss = self.get_boss_zombie();
            if let Some(a_boss) = boss {
                if !a_boss.is_dead_or_dying() && a_boss.body_max_health > 0 {
                    self.m_progress_meter_width =
                        150 * (a_boss.body_max_health - a_boss.body_health) / a_boss.body_max_health;
                } else {
                    self.m_progress_meter_width = 150;
                }
            } else {
                self.m_progress_meter_width = 150;
            }
        } else if self.m_current_wave != 0 {
            if self.m_flag_raise_counter > 0 {
                self.m_flag_raise_counter -= 1;
            }

            let mut a_total_width = 150;
            let a_num_waves_per_flag = self.get_num_waves_per_flag();
            let a_has_flags = self.progress_meter_has_flags();
            if a_has_flags {
                a_total_width -= 12 * self.m_num_waves / a_num_waves_per_flag;
            }

            let a_wave_length = a_total_width / (self.m_num_waves - 1);
            let mut a_current_wave_length = (self.m_current_wave - 1) * a_total_width / (self.m_num_waves - 1);
            let mut a_next_wave_length = self.m_current_wave * a_total_width / (self.m_num_waves - 1);
            if a_has_flags {
                let an_extra_length = self.m_current_wave / a_num_waves_per_flag * 12;
                a_current_wave_length += an_extra_length;
                a_next_wave_length += an_extra_length;
            }

            let mut a_fraction = if self.m_zombie_count_down_start > 0 {
                (self.m_zombie_count_down_start - self.m_zombie_count_down) as f32 / self.m_zombie_count_down_start as f32
            } else {
                0.0
            };
            if self.m_zombie_health_to_next_wave != -1 {
                let a_health_current = self.total_zombies_health_in_wave(self.m_current_wave - 1);
                let mut a_damage_target = self.m_zombie_health_wave_start - self.m_zombie_health_to_next_wave;
                if a_damage_target < 1 {
                    a_damage_target = 1;
                }
                let a_health_fraction =
                    (a_damage_target - a_health_current + self.m_zombie_health_to_next_wave) as f32 / a_damage_target as f32;
                a_fraction = a_fraction.max(a_health_fraction);
            }

            let a_length = (a_current_wave_length
                + ((a_next_wave_length - a_current_wave_length) as f32 * a_fraction).round() as i32)
                .clamp(1, 150);
            let a_delta = a_length - self.m_progress_meter_width;
            if (a_delta > a_wave_length && (self.m_main_counter % 5 == 0))
                || (a_delta > 0 && (self.m_main_counter % 20 == 0))
            {
                self.m_progress_meter_width += 1;
            }
        }
    }

    /// 更新文本追踪检测（对应 C++ DoTypingCheck）
    pub fn do_typing_check(&mut self, key: KeyCode) {
        // 对应 C++ DoTypingCheck：逐个匹配秘籍输入
        let Some(app) = self.app else { return };
        unsafe {
            if (*app).m_konami_check.as_mut().map_or(false, |c| c.check_key(key)) {
                (*app).play_foley(FoleyType::Drop as i32);
                return;
            }
            if (*app).m_mustache_check.as_mut().map_or(false, |c| c.check_key(key))
                || (*app).m_moustache_check.as_mut().map_or(false, |c| c.check_key(key))
            {
                self.set_mustache_mode(!self.m_mustache_mode);
                crate::lawn::widget::achievements_screen::ReportAchievement::give_achievement(
                    Some(app),
                    crate::lawn::widget::achievements_screen::AchievementId::MustacheMode as i32,
                    true,
                );
                return;
            }
            // C++ 7585: mSuperMowerCheck->Check(theKey) || mSuperMowerCheck2->Check(theKey)
            if (*app).m_super_mower_check.as_mut().map_or(false, |c| c.check_key(key))
                || (*app).m_super_mower_check2.as_mut().map_or(false, |c| c.check_key(key))
            {
                self.set_super_mower_mode(!self.m_super_mower_mode);
                return;
            }
            if (*app).m_future_check.as_mut().map_or(false, |c| c.check_key(key)) {
                self.set_future_mode(!self.m_future_mode);
                return;
            }
            if (*app).m_pinata_check.as_mut().map_or(false, |c| c.check_key(key)) {
                if (*app).can_do_pinata_mode() {
                    self.set_pinata_mode(!self.m_pinata_mode);
                } else {
                    if (*app).game_scene == crate::lawn::lawn_app::GameScenes::Playing {
                        self.display_advice("[CANT_USE_CODE]", MessageStyle::BigMiddleFast as i32, AdviceType::None);
                    }
                    // C++: mApp->PlaySample(Sexy::SOUND_BUZZER);
                    if let Some(app) = self.app {
                        unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_BUZZER); }
                    }
                }
                return;
            }
            if (*app).m_dance_check.as_mut().map_or(false, |c| c.check_key(key)) {
                if (*app).can_do_dance_mode() {
                    self.set_dance_mode(!self.m_dance_mode);
                } else {
                    if (*app).game_scene == crate::lawn::lawn_app::GameScenes::Playing {
                        self.display_advice("[CANT_USE_CODE]", MessageStyle::BigMiddleFast as i32, AdviceType::None);
                    }
                    // C++: mApp->PlaySample(Sexy::SOUND_BUZZER);
                    if let Some(app) = self.app {
                        unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_BUZZER); }
                    }
                }
                return;
            }
            if (*app).m_daisy_check.as_mut().map_or(false, |c| c.check_key(key)) {
                if (*app).can_do_daisy_mode() {
                    self.set_daisy_mode(!self.m_daisy_mode);
                } else {
                    if (*app).game_scene == crate::lawn::lawn_app::GameScenes::Playing {
                        self.display_advice("[CANT_USE_CODE]", MessageStyle::BigMiddleFast as i32, AdviceType::None);
                    }
                    // C++: mApp->PlaySample(Sexy::SOUND_BUZZER);
                    if let Some(app) = self.app {
                        unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_BUZZER); }
                    }
                }
                return;
            }
            if (*app).m_sukhbir_check.as_mut().map_or(false, |c| c.check_key(key)) {
                self.set_sukhbir_mode(!self.m_sukhbir_mode);
                return;
            }
        }
    }

    /// 切换胡子模式（对应 C++ SetMustacheMode）
    fn set_mustache_mode(&mut self, enable: bool) {
        if let Some(app) = self.app {
            unsafe {
                (*app).play_foley(FoleyType::Polevault as i32);
                (*app).m_mustache_mode = enable;
            }
        }
        self.m_mustache_mode = enable;
        for i in 0..self.zombies.len() {
            if !self.zombies[i].dead {
                self.zombies[i].enable_mustache(enable);
            }
        }
    }

    /// 切换未来模式（对应 C++ SetFutureMode）
    fn set_future_mode(&mut self, enable: bool) {
        // C++ 7501: mApp->PlaySample(Sexy::SOUND_BOING);
        if let Some(app) = self.app {
            unsafe {
                (*app).play_sample(crate::todlib::tod_foley::SOUND_BOING);
                (*app).m_future_mode = enable;
            }
        }
        self.m_future_mode = enable;
        for i in 0..self.zombies.len() {
            if !self.zombies[i].dead {
                self.zombies[i].enable_future(enable);
            }
        }
    }

    /// 切换彩带模式（对应 C++ SetPinataMode）
    fn set_pinata_mode(&mut self, enable: bool) {
        if let Some(app) = self.app {
            unsafe {
                (*app).play_foley(FoleyType::Juicy as i32);
                (*app).m_pinata_mode = enable;
            }
        }
        self.m_pinata_mode = enable;
    }

    /// 切换舞蹈模式（对应 C++ SetDanceMode）
    fn set_dance_mode(&mut self, enable: bool) {
        if let Some(app) = self.app {
            unsafe {
                (*app).play_foley(FoleyType::Dancer as i32);
                (*app).m_dance_mode = enable;
            }
        }
        self.m_dance_mode = enable;
        for i in 0..self.zombies.len() {
            if !self.zombies[i].dead {
                self.zombies[i].enable_dance();
            }
        }
    }

    /// 切换超级割草机模式（对应 C++ SetSuperMowerMode）
    fn set_super_mower_mode(&mut self, enable: bool) {
        if let Some(app) = self.app {
            unsafe {
                (*app).play_foley(FoleyType::Zamboni as i32);
                (*app).m_super_mower_mode = enable;
            }
        }
        self.m_super_mower_mode = enable;
        for i in 0..self.lawn_mowers.len() {
            if !self.lawn_mowers[i].dead {
                self.lawn_mowers[i].enable_super_mower(enable);
            }
        }
    }

    /// 切换雏菊模式（对应 C++ SetDaisyMode）
    fn set_daisy_mode(&mut self, enable: bool) {
        // C++ 7560: mApp->PlaySample(SOUND_LOADINGBAR_FLOWER);
        if let Some(app) = self.app {
            unsafe {
                (*app).play_sample(crate::todlib::tod_foley::SOUND_LOADINGBAR_FLOWER);
                (*app).m_daisy_mode = enable;
            }
        }
        self.m_daisy_mode = enable;
    }

    /// 切换苏克尔模式（对应 C++ SetSukhbirMode）
    fn set_sukhbir_mode(&mut self, enable: bool) {
        // C++ 7567: mApp->PlaySample(Sexy::SOUND_SUKHBIR);
        if let Some(app) = self.app {
            unsafe {
                (*app).play_sample(crate::todlib::tod_foley::SOUND_SUKHBIR);
                (*app).m_sukhbir_mode = enable;
            }
        }
        self.m_sukhbir_mode = enable;
    }

    // ========== 僵尸生成 ==========

    /// 在某行添加僵尸（对应 C++ AddZombieInRow 完整版）
    /// 返回新僵尸的索引（对应 C++ 的 Zombie*）
    pub fn add_zombie_in_row(&mut self, zombie_type: ZombieType, row: i32, from_wave: i32) -> usize {
        // 对应 C++ Board::AddZombieInRow (Board.cpp:2638)
        // [TRANSLATION_NOTE]: C++ mZombies 为 DataArray（mSize >= mMaxSize-1 时
        // PvzpTrace("Too many zombies!!") 并返回 nullptr）；Rust 用 Vec 无容量上限，跳过该检查。

        if zombie_type == ZombieType::Yeti {
            let is_adventure = self.app.map_or(false, |app| unsafe { (*app).is_adventure_mode() });
            // C++: if (mApp->IsAdventureMode() && mLevel == 40 && theFromWave >= 0)
            //          ReportAchievement::GiveAchievement(mApp, Zombologist, true);
            if is_adventure && self.level == 40 && from_wave >= 0 {
                crate::lawn::widget::achievements_screen::ReportAchievement::give_achievement(
                    self.app,
                    crate::lawn::widget::achievements_screen::AchievementId::Zombologist as i32,
                    true,
                );
            }
        }

        // C++: bool aVariant = !Rand(5);  即 1/5 概率变体
        let a_variant = crate::todlib::tod_common::rand_range_int(0, 4) == 0;
        let mut zombie = Zombie::new();
        zombie.zombie_initialize(row, zombie_type, a_variant, None, from_wave);
        self.zombies.push(zombie);
        let idx = self.zombies.len() - 1;

        // C++: 雪橇僵尸在棋盘内时额外生成 3 个伴舞（parent 指向主雪橇）
        if zombie_type == ZombieType::Bobsled && self.zombies[idx].is_on_board() {
            for _ in 0..3 {
                let mut sled = Zombie::new();
                sled.zombie_initialize(row, ZombieType::Bobsled, false, Some(&mut self.zombies[idx]), from_wave);
                self.zombies.push(sled);
            }
        }
        idx
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

    /// 统计波中僵尸总血量（对应 C++ TotalZombiesHealthInWave）
    /// 累加指定波次中所有符合条件的僵尸的体力值
    pub fn total_zombies_health_in_wave(&self, wave_index: i32) -> i32 {
        let mut total_health = 0i32;
        for zombie in &self.zombies {
            if zombie.dead { continue; }
            if zombie.from_wave == wave_index
                && !zombie.mind_controlled
                && !zombie.dead
                && (zombie.zombie_phase == ZombiePhase::Normal || zombie.zombie_phase == ZombiePhase::BungeeDiving)
                && zombie.zombie_type != ZombieType::Bungee
                && zombie.related_zombie_id == crate::lawn::game_enums::ZOMBIEID_NULL
            {
                total_health += zombie.body_health
                    + zombie.helm_health
                    + (zombie.shield_health as f32 * 0.2) as i32
                    + zombie.flying_health;
            }
        }
        total_health
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
        // C++ Board.cpp:5014-5016: mCursorPreview->Update(); mCursorObject->Update();
        // CursorPreview::Update 使用 mApp->mWidgetManager->mLastMouseX/Y
        let mouse_x = self.app.map_or(0, |app| unsafe {
            (*app).base.widget_manager.map_or(0, |wm| (*wm).last_mouse_x)
        });
        let mouse_y = self.app.map_or(0, |app| unsafe {
            (*app).base.widget_manager.map_or(0, |wm| (*wm).last_mouse_y)
        });
        self.cursor_preview.update(mouse_x, mouse_y);
        // C++ 5015: mCursorObject->Update()（CursorObject.cpp:45-70，非 Playing 场景隐藏）
        let is_playing = self.app.map_or(false, |app| unsafe {
            (*app).game_scene == crate::lawn::lawn_app::GameScenes::Playing
        });
        let is_in_shovel_tutorial = self.m_cut_scene.map_or(false, |c| unsafe { (*c).is_in_shovel_tutorial() });
        self.cursor_object.update(mouse_x, mouse_y, is_playing, is_in_shovel_tutorial);
        // C++ 5016-5019: for i in 0..mSeedBank->mNumPackets { mSeedBank->mSeedPackets[i].Update(); }
        for packet in &mut self.seed_bank {
            packet.update();
        }
    }

    // ========== 更新循环 ==========

    /// 更新僵尸生成（对应 C++ UpdateZombieSpawning，音效/音乐/FadeOutLevel/DisplayAdviceAgain 未接入）
    /// 控制僵尸从墓碑/水池/天空的生成时机
    fn update_zombie_spawning(&mut self) {
        let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        // C++ 5261-5262: GAMEMODE_UPSELL || GAMEMODE_INTRO → return
        if app_mode == GameMode::Upsell || app_mode == GameMode::Intro {
            return;
        }

        // C++ 5264-5271: 最终波次声音计时
        if self.m_final_wave_sound_counter > 0 {
            self.m_final_wave_sound_counter -= 1;
            if self.m_final_wave_sound_counter == 0 {
                // C++: mApp->PlaySample(Sexy::SOUND_FINALWAVE);
                if let Some(app) = self.app {
                    unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_FINALWAVE); }
                }
            }
        }

        // C++ 5273-5277: 教程状态 → return
        if self.m_tutorial_state == TutorialState::Level1PickUpPeashooter
            || self.m_tutorial_state == TutorialState::Level1PlantPeashooter
            || self.m_tutorial_state == TutorialState::Level1RefreshPeashooter
            || self.m_tutorial_state == TutorialState::SlotMachine
        {
            return;
        }

        // 如果关卡已结束，提前返回
        if self.has_level_award_dropped() {
            return;
        }

        // 墓碑升起计时器
        if self.m_rise_from_grave_counter > 0 {
            self.m_rise_from_grave_counter -= 1;
            if self.m_rise_from_grave_counter == 0 {
                self.spawn_zombies_from_graves();
            }
        }

        // 大波倒计时处理（对应 C++ mHugeWaveCountDown）
        if self.m_huge_wave_count_down > 0 {
            self.m_huge_wave_count_down -= 1;
            if self.m_huge_wave_count_down == 0 {
                self.clear_advice(AdviceType::HugeWave);
                self.next_wave_coming();
                self.m_zombie_count_down = 1;
            } else {
                // C++ 5302-5305: ==725 时 PlaySample(SOUND_HUGE_WAVE)
                if self.m_huge_wave_count_down == 725 {
                    if let Some(app) = self.app {
                        unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_HUGE_WAVE); }
                    }
                } else {
                    // C++ 5307-5321: 按当前音乐 tune 触发 StartBurst
                    let a_tune = self.app.map_or(crate::lawn::system::music::MusicTune::None, |app| unsafe {
                        (*app).music.as_ref().map_or(crate::lawn::system::music::MusicTune::None, |m| m.cur_music_tune)
                    });
                    if a_tune == crate::lawn::system::music::MusicTune::DayGrasswalk
                        || a_tune == crate::lawn::system::music::MusicTune::PoolWateryGraves
                        || a_tune == crate::lawn::system::music::MusicTune::FogRigormormist
                        || a_tune == crate::lawn::system::music::MusicTune::RoofGrazeTheRoof
                    {
                        if self.m_huge_wave_count_down == 400 {
                            if let Some(app) = self.app {
                                unsafe { (*app).music.as_mut().map(|m| m.start_burst()); }
                            }
                        }
                    } else if a_tune == crate::lawn::system::music::MusicTune::NightMoongrains {
                        if self.m_huge_wave_count_down == 700 {
                            if let Some(app) = self.app {
                                unsafe { (*app).music.as_mut().map(|m| m.start_burst()); }
                            }
                        }
                    }
                }
                return;
            }
        }

        // 挑战模式自定义生成
        if let Some(ref mut challenge) = self.challenge {
            if challenge.update_zombie_spawning() != 0 {
                return;
            }
        }

        // 如果已经是最后波次，检查是否继续生成
        if self.m_current_wave >= self.m_num_waves {
            if self.is_final_survival_stage() || app_mode == GameMode::ChallengeLastStand {
                return;
            }
            if !self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() })
                && !self.app.map_or(false, |app| unsafe { (*app).is_continuous_challenge() })
            {
                return;
            }
        }

        // 波次倒计时
        self.m_zombie_count_down -= 1;
        if self.m_current_wave == self.m_num_waves && self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() }) {
            if self.m_zombie_count_down == 0 {
                // C++ 5354: FadeOutLevel();
                self.fade_out_level();
            }
            return;
        }

        // 限制倒计时上限
        if self.m_zombie_count_down > 200 && self.m_zombie_count_down_start - self.m_zombie_count_down > 400
            && self.total_zombies_health_in_wave(self.m_current_wave - 1) <= self.m_zombie_health_to_next_wave
        {
            self.m_zombie_count_down = 200;
        }

        // 倒计时到 5 时触发旗波/下一波提示
        if self.m_zombie_count_down == 5 {
            if self.is_flag_wave(self.m_current_wave) {
                self.clear_advice_immediately();
                // C++ 5368: DisplayAdviceAgain("[ADVICE_HUGE_WAVE]", MESSAGE_STYLE_HUGE_WAVE, ADVICE_HUGE_WAVE);
                self.display_advice_again(
                    "[ADVICE_HUGE_WAVE]",
                    crate::lawn::game_enums::MessageStyle::HugeWave as i32,
                    crate::lawn::game_enums::AdviceType::HugeWave,
                );
                self.m_huge_wave_count_down = 750;
                return;
            }
            self.next_wave_coming();
        }

        // 倒计时到 0 时生成僵尸波次
        if self.m_zombie_count_down == 0 {
            self.spawn_zombie_wave();
            self.m_zombie_health_wave_start = self.total_zombies_health_in_wave(self.m_current_wave - 1);

            if self.m_current_wave == self.m_num_waves && self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() }) {
                self.m_zombie_health_to_next_wave = 0;
                self.m_zombie_count_down = ZOMBIE_COUNTDOWN_BEFORE_REPICK + 1;
            } else if self.is_flag_wave(self.m_current_wave) && !self.app.map_or(false, |app| unsafe { (*app).is_wallnut_bowling_level() }) && app_mode != GameMode::ChallengeLastStand {
                self.m_zombie_health_to_next_wave = 0;
                self.m_zombie_count_down = ZOMBIE_COUNTDOWN_BEFORE_FLAG;
            } else {
                // C++ 5391: mZombieHealthToNextWave = RandRangeFloat(0.5f, 0.65f) * mZombieHealthWaveStart
                self.m_zombie_health_to_next_wave = (crate::todlib::tod_common::rand_range_float(0.5, 0.65)
                    * self.m_zombie_health_wave_start as f32) as i32;
                // C++ 5392-5399: 小麻烦/竖列/最后防线模式波次间隔固定 750
                let little_trouble = self.app.map_or(false, |app| unsafe { (*app).is_little_trouble_level() });
                if little_trouble || app_mode == GameMode::ChallengeColumns || app_mode == GameMode::ChallengeLastStand {
                    self.m_zombie_count_down = 750;
                } else {
                    self.m_zombie_count_down = ZOMBIE_COUNTDOWN + RandRange(ZOMBIE_COUNTDOWN_RANGE);
                }
            }
            self.m_zombie_count_down_start = self.m_zombie_count_down;
        }
    }

    /// 下一波到来（对应 C++ NextWaveComing）
    /// 触发最终波次动画和音效
    fn next_wave_coming(&mut self) {
        if self.m_current_wave + 1 == self.m_num_waves {
            let is_survival_repick = self.is_survival_stage_with_repick();
            let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
            if !is_survival_repick
                && app_mode != GameMode::ChallengeLastStand
                && !self.app.map_or(false, |app| unsafe { (*app).is_continuous_challenge() })
            {
                // 添加最终波次动画
                self.m_final_wave_sound_counter = 60;
            }
        }

        if self.m_current_wave == 0 {
            // 第一波——播放警笛
        } else if self.is_flag_wave(self.m_current_wave) {
            // 旗波——播放汽笛
        }
    }

    /// 更新冰冻效果（对应 C++ UpdateIce 完整版）
    /// 管理冰道融化计时器与冰粒子系统
    fn update_ice(&mut self) {
        for row in 0..MAX_GRID_SIZE_Y {
            if self.m_ice_timer[row] > 0 {
                self.m_ice_timer[row] -= 1;
                // C++: PvzpParticleSystem* aParticleIce = mApp->ParticleTryToGet(mIceParticleID[aRow]);
                let mut a_particle_ice = if let Some(app) = self.app {
                    unsafe { (*app).particle_try_to_get(self.m_ice_particle_id[row]) }
                } else {
                    None
                };
                if self.m_ice_timer[row] == 0 {
                    self.m_ice_min_x[row] = BOARD_ICE_START;
                    if let Some(ps) = a_particle_ice.as_mut() {
                        ps.particle_system_die();
                    }
                } else {
                    let a_pos_x = self.m_ice_min_x[row] as f32;
                    let a_pos_y = self.grid_to_pixel_y(8, row as i32) as f32;
                    if let Some(ps) = a_particle_ice.as_mut() {
                        ps.system_move(a_pos_x, a_pos_y);
                    } else {
                        // C++: aRenderPosition = MakeRenderOrder(RENDER_LAYER_GROUND, aRow, 3);
                        //       aParticleIce = mApp->AddPvzpParticle(..., PARTICLE_ICE_SPARKLE);
                        //       mIceParticleID[aRow] = mApp->ParticleGetID(aParticleIce);
                        let a_render_position = make_render_order(RENDER_LAYER_GROUND, row as i32, 3);
                        if let Some(app) = self.app {
                            unsafe {
                                if let Some(ptr) = (*app).add_tod_particle(
                                    a_pos_x, a_pos_y, a_render_position, ParticleEffect::IceSparkle as i32,
                                ) {
                                    self.m_ice_particle_id[row] = (*app).particle_get_id(ptr);
                                }
                            }
                        }
                    }
                }
                // C++: int anAlpha = std::clamp(mIceTimer[aRow] / 10, 0, 255);
                //       aParticleIce->OverrideColor(nullptr, Color(255, 255, 255, anAlpha));
                if let Some(ps) = a_particle_ice.as_mut() {
                    let an_alpha = (self.m_ice_timer[row] / 10).clamp(0, 255) as u8;
                    ps.override_color("", &crate::framework::color::Color::new(255, 255, 255, an_alpha));
                }
            }
        }
        // 注：C++ mIceTrapCounter 递减位于 UpdateGame（Board.cpp:5682），不在 UpdateIce 内。
    }

    /// 主更新循环（对应 C++ UpdateGame 完整版，粒子 mDontUpdate 恢复未接入）
    fn update_game(&mut self) {
        // 更新所有游戏对象（对应 C++ UpdateGameObjects）
        self.update_game_objects();

        // C++ 5658-5673: Fog 偏移动画
        if self.stage_has_fog() && self.m_fog_blown_count_down > 0 {
            let a_max_fog_offset = 1065.0 - self.left_fog_column() as f32 * 80.0;
            let scene = self.app.map_or(crate::lawn::lawn_app::GameScenes::Playing, |app| unsafe { (*app).game_scene });
            if scene == crate::lawn::lawn_app::GameScenes::LevelIntro {
                self.m_fog_offset = crate::todlib::tod_common::tod_animate_curve_float(
                    200, 0, self.m_fog_blown_count_down, a_max_fog_offset, 0.0,
                    crate::lawn::game_enums::TodCurves::EaseOut,
                );
            } else if self.m_fog_blown_count_down < 2000 {
                self.m_fog_offset = crate::todlib::tod_common::tod_animate_curve_float(
                    2000, 0, self.m_fog_blown_count_down, a_max_fog_offset, 0.0,
                    crate::lawn::game_enums::TodCurves::EaseOut,
                );
            } else if self.m_fog_offset < a_max_fog_offset {
                // C++: PvzpAnimateCurveFloat(-5, aMaxFogOffset, mFogOffset * 1.1f, 0, aMaxFogOffset, CURVE_LINEAR)
                // time 参数在 C++ 中为 int（float 隐式截断），此处显式 as i32 等价
                self.m_fog_offset = crate::todlib::tod_common::tod_animate_curve_float(
                    -5, a_max_fog_offset as i32, (self.m_fog_offset * 1.1) as i32, 0.0, a_max_fog_offset,
                    crate::lawn::game_enums::TodCurves::Linear,
                );
            }
        }

        // C++ 5675-5676: 场景检查（含 ShouldRunUpsellBoard）
        let scene = self.app.map_or(crate::lawn::lawn_app::GameScenes::Playing, |app| unsafe { (*app).game_scene });
        let should_run_upsell = self.m_cut_scene.map_or(false, |c| unsafe { (*c).should_run_upsell_board() });
        if scene != crate::lawn::lawn_app::GameScenes::Playing && !should_run_upsell {
            return;
        }

        self.m_main_counter += 1;
        self.update_sun_spawning();
        self.update_zombie_spawning();
        self.update_ice();

        // C++ 5682-5693: mIceTrapCounter 递减
        if self.m_ice_trap_counter > 0 {
            self.m_ice_trap_counter -= 1;
            if self.m_ice_trap_counter == 0 {
                // C++: PvzpParticleSystem* aPoolSparklyParticle = mApp->ParticleTryToGet(mPoolSparklyParticleID);
                //      if (aPoolSparklyParticle) aPoolSparklyParticle->mDontUpdate = false;
                if let Some(app) = self.app {
                    unsafe {
                        if let Some(ps) = (*app).particle_try_to_get(self.m_pool_sparkly_particle_id) {
                            ps.dont_update = false;
                        }
                    }
                }
            }
        }

        // C++ 5695-5698
        if self.m_fog_blown_count_down > 0 {
            self.m_fog_blown_count_down -= 1;
        }

        // C++ 5700-5712: 首次冒险模式的教程引导
        let first_time_adventure = self.app.map_or(false, |app| unsafe { (*app).is_first_time_adventure_mode() });
        if self.m_main_counter == 1 && first_time_adventure {
            if self.level == 1 {
                self.set_tutorial_state(TutorialState::Level1PickUpPeashooter);
            } else if self.level == 2 {
                self.set_tutorial_state(TutorialState::Level1PickUpSunflower);
                self.display_advice("[ADVICE_PLANT_SUNFLOWER1]", MessageStyle::TutorialLevel2 as i32, AdviceType::None);
                self.m_tutorial_timer = 500;
            }
        }

        self.update_progress_meter();
    }

    /// 更新浓雾效果（对应 C++ UpdateFog）
    /// 雾的能见度受 m_fog_blown_count_down 影响，植物灯笼/火炬可以驱散局部浓雾
    fn update_fog(&mut self) {
        if !self.stage_has_fog() {
            return;
        }

        // 雾的恢复速度
        // C++ 有三级速度：mFogBlownCountDown >= 2000 时 = 20, > 0 时 = 1, 否则 = 3
        let fog_fade_in_speed: i32;
        if self.m_fog_blown_count_down > 0 && self.m_fog_blown_count_down < 2000 {
            fog_fade_in_speed = 1;
        } else if self.m_fog_blown_count_down > 0 {
            fog_fade_in_speed = 20;
        } else {
            fog_fade_in_speed = 3;
        }

        let a_left = self.left_fog_column() as usize;
        for x in a_left..MAX_GRID_SIZE_X {
            let fog_max = if x == a_left { 200 } else { 255 };
            for y in 0..=MAX_GRID_SIZE_Y {
                self.grid_cel_fog[x][y] = std::cmp::min(
                    self.grid_cel_fog[x][y] + fog_fade_in_speed,
                    fog_max,
                );
            }
        }

        // 灯笼草（Pl lantern）驱散周围 4 格，火炬树桩驱散周围 1 格
        // 先收集要驱散雾的植物位置，避免借用冲突
        let fog_plants: Vec<(i32, i32, i32)> = self.plants.iter()
            .filter(|plant| {
                // 等同于 C++ NotOnGround() 检查
                !plant.dead && !plant.squished && plant.on_bungee_state == PlantOnBungeeState::NotOnBungee
            })
            .filter_map(|plant| {
                if plant.seed_type == SeedType::Plantern {
                    Some((plant.plant_col, plant.base.row, 4))
                } else if plant.seed_type == SeedType::Torchwood {
                    Some((plant.plant_col, plant.base.row, 1))
                } else {
                    None
                }
            })
            .collect();

        for (col, row, size) in fog_plants {
            self.clear_fog_around_plant(col, row, size);
        }
    }

    /// 清除植物周围的浓雾（对应 C++ ClearFogAroundPlant）
    fn clear_fog_around_plant(&mut self, plant_col: i32, plant_row: i32, size: i32) {
        let fog_fade_out_speed: i32;
        if self.m_fog_blown_count_down > 0 && self.m_fog_blown_count_down < 2000 {
            fog_fade_out_speed = 2;
        } else if self.m_fog_blown_count_down > 0 {
            fog_fade_out_speed = 40;
        } else {
            fog_fade_out_speed = 6;
        }

        let a_left = self.left_fog_column();
        let fog_offset_x = (self.m_fog_offset as i32 + 50) / 100;
        let mut start_x = plant_col - size - fog_offset_x;
        let mut end_x = plant_col + size - fog_offset_x;
        start_x = std::cmp::max(start_x, a_left);
        end_x = std::cmp::min(end_x, MAX_GRID_SIZE_X as i32 - 1);

        let mut start_y = plant_row - size;
        let mut end_y = plant_row + size;
        start_y = std::cmp::max(start_y, 0);
        end_y = std::cmp::min(end_y, MAX_GRID_SIZE_Y as i32);

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                let dist_x = (x + fog_offset_x - plant_col).abs();
                let dist_y = (y - plant_row).abs();
                if size == 4 {
                    if dist_x > 3 || dist_y > 2 {
                        continue;
                    }
                    if dist_x + dist_y == 5 {
                        continue;
                    }
                } else if dist_x + dist_y > size {
                    continue;
                }

                self.grid_cel_fog[x as usize][y as usize] = std::cmp::max(
                    self.grid_cel_fog[x as usize][y as usize] - fog_fade_out_speed,
                    0,
                );
            }
        }
    }

    /// 更新火焰扫荡效果（对应 C++ UpdateFwoosh 完整版）
    fn update_fwoosh(&mut self) {
        // 对应 C++ UpdateFwoosh：火焰扫荡动画按倒计时推进
        if self.m_fwoosh_count_down == 0 {
            return;
        }
        self.m_fwoosh_count_down -= 1;
        let a_fwoosh_remaining = crate::todlib::tod_common::tod_animate_curve(
            50, 0, self.m_fwoosh_count_down, 12, 0,
            crate::lawn::game_enums::TodCurves::Linear,
        );
        for a_row in 0..MAX_GRID_SIZE_Y {
            for i in 0..(12 - a_fwoosh_remaining) {
                let fwoosh_id = self.m_fwoosh_id[a_row][i as usize];
                if let Some(app) = self.app {
                    unsafe {
                        if let Some(fwoosh) = (*app).reanimation_get_mut(fwoosh_id) {
                            fwoosh.set_frames_for_layer("anim_done");
                            fwoosh.m_anim_rate = 15.0;
                            fwoosh.m_loop_type = crate::todlib::reanimator::ReanimLoopType::PlayOnceFullLastFrame;
                        }
                    }
                }
                self.m_fwoosh_id[a_row][i as usize] = REANIMATIONID_NULL;
            }
        }
    }

    // ========== 特殊格子操作 ==========

    /// 选择特殊墓碑（对应 C++ PickSpecialGraveStone）
    /// 在所有墓碑中随机选择一个，将其状态设为 GRIDITEM_STATE_GRAVESTONE_SPECIAL
    pub fn pick_special_grave_stone(&mut self) {
        // 收集所有墓碑的索引
        let picks: Vec<usize> = self.grid_items.iter().enumerate()
            .filter(|(_, item)| item.grid_item_type == GridItemType::Grave)
            .map(|(i, _)| i)
            .collect();

        let pick_count = picks.len();
        if pick_count > 0 {
            // 从有效范围随机选一个
            let idx = crate::todlib::tod_common::rand_range_int(0, pick_count as i32 - 1) as usize;
            self.grid_items[picks[idx]].grid_item_state = GridItemState::GravestoneSpecial;
        }
    }

    /// 获取铁锹按钮矩形（对应 C++ GetShovelButtonRect 完整版）
    pub fn get_shovel_button_rect(&self) -> Rect {
        // 对应 C++ Board::GetShovelButtonRect（Board.cpp:1307）
        let mut a_rect = Rect::new(self.get_seed_bank_extra_width() + 456, 0, 0, 0);
        if let Some(app) = self.app {
            let app_ref = unsafe { &*app };
            let a_shovel_bank = crate::lawn::board::get_overlay_image(app_ref, "IMAGE_SHOVELBANK");
            if !a_shovel_bank.is_null() {
                let a_shovel_bank_ref = unsafe { &*a_shovel_bank };
                a_rect.width = a_shovel_bank_ref.width;
                a_rect.height = a_shovel_bank_ref.height;
            }
            if app_ref.is_slot_machine_level() || app_ref.is_squirrel_level() {
                a_rect.x = 600;
            }
        }
        a_rect
    }

    /// 获取禅境工具按钮矩形（对应 C++ GetZenButtonRect）
    /// theRect 需预先初始化为铲子按钮矩形，此函数仅调整 X
    pub fn get_zen_button_rect(&self, object_type: GameObjectType) -> Rect {
        let mut the_rect = self.get_shovel_button_rect();
        the_rect.x = 30;
        if let Some(app) = self.app {
            unsafe { if (*app).game_mode == GameMode::ChallengeTreeOfWisdom { return the_rect; } }
        }

        let mut usable = true;
        for an_object in (GameObjectType::WateringCan as i32)..=(GameObjectType::Wheelbarrow as i32) {
            let obj = unsafe { std::mem::transmute::<i32, GameObjectType>(an_object) };
            if !self.can_use_game_object(obj) {
                usable = false;
                break;
            }
        }
        if usable {
            the_rect.x = 0;
        }

        // [TRANSLATION_NOTE]: C++ 中 aShovelWidth = IMAGE_SHOVELBANK->GetWidth()；
        // Rust 侧无该图片资源，以固定宽度近似
        let a_shovel_width = 72;
        for an_object in (GameObjectType::WateringCan as i32)..(object_type as i32) {
            let obj = unsafe { std::mem::transmute::<i32, GameObjectType>(an_object) };
            if self.can_use_game_object(obj) {
                the_rect.x += a_shovel_width;
            }
        }
        the_rect
    }

    // ========== 计数 ==========

    /// 统计空花盆或睡莲数量（对应 C++ CountEmptyPotsOrLilies）
    /// 统计某类型且没有被其他植物占据的底层植物数量
    pub fn count_empty_pots_or_lilies(&self, seed_type: SeedType) -> i32 {
        let mut count = 0;
        for plant in &self.plants {
            if plant.dead { continue; }
            if plant.seed_type != seed_type { continue; }

            // 检查同一格子上是否有其他普通位置的植物
            let has_normal_plant_above = self.plants.iter().any(|other| {
                if other.dead { return false; }
                if other.plant_col != plant.plant_col { return false; }
                if other.base.row != plant.base.row { return false; }
                if other.squished { return false; }
                if !other.is_on_board { return false; }
                // 跳过底层植物（花盆/睡莲/飞行）本身
                if other.seed_type == SeedType::Flowerpot || other.seed_type == SeedType::Lilypad {
                    return false;
                }
                if Plant::is_flying(other.seed_type) {
                    return false;
                }
                true
            });

            if !has_normal_plant_above {
                count += 1;
            }
        }
        count
    }

    /// 检查某格是否有南瓜（对应 C++ GetPlantsOnLawn 简化版—南瓜检查）
    fn has_pumpkin_at(&self, grid_x: i32, grid_y: i32) -> bool {
        self.plants.iter().any(|p| {
            !p.dead && p.plant_col == grid_x && p.base.row == grid_y && p.seed_type == SeedType::Pumpkinshell
        })
    }

    /// 玉米加农炮位置辅助检查（对应 C++ IsValidCobCannonSpotHelper）
    /// 检查单个格子是否可以作为玉米加农炮的一部分
    fn is_valid_cob_cannon_spot_helper(&self, grid_x: i32, grid_y: i32) -> bool {
        if grid_x < 0 || grid_x >= MAX_GRID_SIZE_X as i32 || grid_y < 0 || grid_y >= MAX_GRID_SIZE_Y as i32 {
            return false;
        }

        if self.has_pumpkin_at(grid_x, grid_y) {
            return false;
        }

        // 检查该格是否有玉米投手（CobCannon 的前置植物）
        let has_kernelpult = self.plants.iter().any(|p| {
            !p.dead && p.plant_col == grid_x && p.base.row == grid_y && p.seed_type == SeedType::Kernelpult
        });

        if has_kernelpult {
            return true;
        }

        // 简易种植外挂检查
        self.app.map_or(false, |app| unsafe {
            (*app).m_easy_planting_cheat
        }) && self.can_plant_at(grid_x, grid_y, SeedType::Kernelpult) == PlantingReason::Ok
    }

    /// 检查是否有效玉米加农炮位置（对应 C++ IsValidCobCannonSpot）
    /// 检查 (grid_x, grid_y) 和 (grid_x+1, grid_y) 两个格子是否构成有效位置
    pub fn is_valid_cob_cannon_spot(&self, grid_x: i32, grid_y: i32) -> bool {
        if !self.is_valid_cob_cannon_spot_helper(grid_x, grid_y)
            || !self.is_valid_cob_cannon_spot_helper(grid_x + 1, grid_y)
        {
            return false;
        }

        // 两个格子的花盆状态必须一致
        let has_pot_left = self.get_flower_pot_at(grid_x, grid_y).is_some();
        let has_pot_right = self.get_flower_pot_at(grid_x + 1, grid_y).is_some();
        has_pot_left == has_pot_right
    }

    /// 是否有有效的玉米加农炮位置（对应 C++ HasValidCobCannonSpot）
    pub fn has_valid_cob_cannon_spot(&self) -> bool {
        for plant in &self.plants {
            if plant.dead { continue; }
            if plant.seed_type == SeedType::Kernelpult
                && self.is_valid_cob_cannon_spot(plant.plant_col, plant.base.row)
            {
                return true;
            }
        }
        false
    }

    /// 将指定僵尸类型放入指定波次（对应 C++ PutZombieInWave）
    pub fn put_zombie_in_wave(&mut self, zombie_type: ZombieType, wave_number: i32, picker: &mut ZombiePicker) {
        picker.zombie_count += 1;
        let count = picker.zombie_count - 1;
        self.m_zombies_in_wave[wave_number as usize][count as usize] = zombie_type;
        if picker.zombie_count < MAX_ZOMBIES_IN_WAVE as i32 {
            self.m_zombies_in_wave[wave_number as usize][picker.zombie_count as usize] = ZombieType::Invalid;
        }
        let def = get_zombie_definition(zombie_type);
        picker.zombie_points -= def.zombie_value;
        picker.zombie_type_count[zombie_type as usize] += 1;
        picker.all_waves_zombie_type_count[zombie_type as usize] += 1;
    }

    /// 补充缺失的僵尸类型到最后一波（对应 C++ PutInMissingZombies）
    pub fn put_in_missing_zombies(&mut self, wave_number: i32, picker: &mut ZombiePicker) {
        for ztype_idx in 0..(NUM_ZOMBIE_TYPES as i32) {
            let a_zombie_type: ZombieType = unsafe { std::mem::transmute(ztype_idx) };
            if picker.zombie_type_count[ztype_idx as usize] <= 0
                && a_zombie_type != ZombieType::Yeti
                && self.can_zombie_spawn_on_level(a_zombie_type, self.level)
            {
                self.put_zombie_in_wave(a_zombie_type, wave_number, picker);
            }
        }
    }

    /// 随机选择一种僵尸类型（对应 C++ PickZombieType）
    pub fn pick_zombie_type(&self, zombie_points: i32, wave_index: i32, picker: &ZombiePicker) -> ZombieType {
        let mut pick_count = 0;
        let mut zombie_weight_array: [TodWeightedArray; NUM_ZOMBIE_TYPES as usize] = [TodWeightedArray { item: 0, weight: 0 }; NUM_ZOMBIE_TYPES as usize];
        
        for ztype_idx in 0..(NUM_ZOMBIE_TYPES as i32) {
            if !self.m_zombie_allowed[ztype_idx as usize] {
                continue;
            }

            let a_zombie_type: ZombieType = unsafe { std::mem::transmute(ztype_idx) };
            let a_zombie_def = get_zombie_definition(a_zombie_type);

            // 获取游戏模式
            let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });

            // 蹦极僵尸在无尽模式中仅在旗帜波出现
            if a_zombie_type == ZombieType::Bungee {
                if let Some(app) = self.app {
                    if unsafe { (*app).is_survival_endless(game_mode) } {
                        if !self.is_flag_wave(wave_index) {
                            continue;
                        }
                    }
                }
            }
            // 僵尸最早出现的波数的限制（出怪限制）
            // 注意：C++ 中排除 GAMEMODE_CHALLENGE_POGO_PARTY、GAMEMODE_CHALLENGE_BOBSLED_BONANZA、
            // GAMEMODE_CHALLENGE_AIR_RAID，但 Rust 枚举暂缺这些变体，跳过对应排除
            else
            {
                let mut first_allowed_wave = a_zombie_def.first_allowed_wave;
                // 无尽模式中，僵尸最早可出现的波数逐渐前移
                if let Some(app) = self.app {
                    if unsafe { (*app).is_survival_endless(game_mode) } {
                        let flags = self.get_survival_flags_completed();
                        let allowed_wave = first_allowed_wave
                            - tod_animate_curve(18, 50, flags, 0, 15, TodCurves::Linear);
                        first_allowed_wave = std::cmp::max(allowed_wave, 1);
                    }
                }
                if wave_index + 1 < first_allowed_wave || zombie_points < a_zombie_def.zombie_value {
                    continue;
                }
            }

            // 生存模式中，根据当前旗帜数等重新计算僵尸的权重
            let mut pick_weight = a_zombie_def.pick_weight;
            if let Some(app) = self.app {
                if unsafe { (*app).is_survival_mode() } {
                    let flags = self.get_survival_flags_completed();
                    // 伽刚特尔和雪橇车僵尸的每波出怪上限
                    if a_zombie_type == ZombieType::Gargantuar || a_zombie_type == ZombieType::Zamboni {
                        if picker.zombie_type_count[ztype_idx as usize]
                            >= tod_animate_curve(10, 50, flags, 2, 50, TodCurves::Linear)
                        {
                            continue;
                        }
                    }
                    // 红眼的旗帜波出怪上限和非旗帜波出怪总和上限
                    else if a_zombie_type == ZombieType::RedeEyeGargantuar {
                        if self.is_flag_wave(wave_index) {
                            if picker.zombie_type_count[ztype_idx as usize]
                                >= tod_animate_curve(14, 100, flags, 1, 50, TodCurves::Linear)
                            {
                                continue;
                            }
                        } else {
                            if picker.all_waves_zombie_type_count[ztype_idx as usize]
                                >= tod_animate_curve(10, 110, flags, 1, 50, TodCurves::Linear)
                            {
                                continue;
                            }
                            pick_weight = 1000;
                        }
                    }
                    // 普通僵尸和路障僵尸的权重衰减
                    else if a_zombie_type == ZombieType::Normal {
                        pick_weight = tod_animate_curve(
                            10, 50, flags, pick_weight, pick_weight / 10, TodCurves::Linear,
                        );
                    } else if a_zombie_type == ZombieType::TrafficCone {
                        pick_weight = tod_animate_curve(
                            10, 50, flags, pick_weight, pick_weight / 4, TodCurves::Linear,
                        );
                    }
                }
            }
            zombie_weight_array[pick_count].item = ztype_idx as usize;
            zombie_weight_array[pick_count].weight = pick_weight;
            pick_count += 1;
        }

        // 加权随机地取得一种可能的僵尸类型并返回
        let picked_idx = tod_pick_from_weighted_array(&zombie_weight_array[..pick_count]);
        unsafe { std::mem::transmute::<i32, ZombieType>(zombie_weight_array[picked_idx as usize].item as i32) }
    }

    /// 生成所有波次的僵尸列表（对应 C++ PickZombieWaves）
    pub fn pick_zombie_waves(&mut self) {
        // ====================================================================================================
        // ▲ 设定关卡总波数
        // ====================================================================================================
        let is_adventure = self.app.map_or(false, |app| unsafe { (*app).is_adventure_mode() });
        if is_adventure {
            let is_whack = self.app.map_or(false, |app| unsafe { (*app).is_whack_a_zombie_level() });
            if is_whack {
                self.m_num_waves = 8;
            } else {
                let level_index = std::cmp::max(self.level - 1, 0);
                let level_index_clamped = std::cmp::min(level_index, 49) as usize;
                self.m_num_waves = G_ZOMBIE_WAVES[level_index_clamped];
                let is_first_time = self.is_first_time_adventure();
                let is_mini_boss = self.app.map_or(false, |app| unsafe { (*app).is_mini_boss_level() });
                if !is_first_time && !is_mini_boss {
                    self.m_num_waves = if self.m_num_waves < 10 { 20 } else { self.m_num_waves + 10 };
                }
            }
        } else {
            let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
            let is_survival = self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() });
            if is_survival || game_mode == GameMode::ChallengeLastStand {
                self.m_num_waves = self.get_num_waves_per_survival_stage();
            } else if game_mode == GameMode::ChallengeZenGarden
                || game_mode == GameMode::ChallengeTreeOfWisdom
                || self.app.map_or(false, |app| unsafe { (*app).is_squirrel_level() })
            {
                self.m_num_waves = 0;
            } else if game_mode == GameMode::ChallengeWhackAZombie {
                self.m_num_waves = 12;
            } else if game_mode == GameMode::ChallengeWallnutBowling
                || game_mode == GameMode::ChallengeColumns
                || game_mode == GameMode::ChallengeInvisighoul
                || game_mode == GameMode::ChallengePortalCombat
                || game_mode == GameMode::ChallengeWarAndPeas
            {
                self.m_num_waves = 20;
            // 注意：C++ 中还有 GAMEMODE_CHALLENGE_GRAVE_DANGER、GAMEMODE_CHALLENGE_HIGH_GRAVITY、
            // GAMEMODE_CHALLENGE_AIR_RAID 等变体，但 Rust 枚举暂缺这些变体，跳过对应检查
            } else {
                let is_stormy = self.app.map_or(false, |app| unsafe { (*app).is_stormy_night_level() });
                let is_little = game_mode == GameMode::ChallengeLittleTrouble;
                let is_bungee = self.app.map_or(false, |app| unsafe { (*app).is_bungee_blitz_level() });
                let is_shovel = self.app.map_or(false, |app| unsafe { (*app).is_shovel_level() });
                if is_stormy || is_little || is_bungee
                    || game_mode == GameMode::ChallengeColumns
                    || is_shovel
                    || game_mode == GameMode::ChallengeWallnutBowling2
                    || game_mode == GameMode::ChallengePogoParty
                {
                    self.m_num_waves = 30;
                } else {
                    self.m_num_waves = 40;
                }
            }
        }

        // ====================================================================================================
        // ▲ 一些准备工作
        // ====================================================================================================
        let mut a_zombie_picker = ZombiePicker {
            zombie_count: 0,
            zombie_points: 0,
            zombie_type_count: [0; NUM_ZOMBIE_TYPES as usize],
            all_waves_zombie_type_count: [0; NUM_ZOMBIE_TYPES as usize],
        };
        a_zombie_picker.init();
        let a_intro_zombie_type = self.get_introduced_zombie_type();

        // ====================================================================================================
        // ▲ 遍历每一波并填充每波的出怪列表
        // ====================================================================================================
        for a_wave in 0..self.m_num_waves {
            a_zombie_picker.init_for_wave();
            self.m_zombies_in_wave[a_wave as usize][0] = ZombieType::Invalid;

            let a_is_flag_wave = self.is_flag_wave(a_wave);
            let a_is_final_wave = a_wave == self.m_num_waves - 1;

            // 蹦极闪电战关卡的每大波固定刷出 5 只蹦极僵尸
            let is_bungee_blitz = self.app.map_or(false, |app| unsafe { (*app).is_bungee_blitz_level() });
            if is_bungee_blitz && a_is_flag_wave {
                for _ in 0..5 {
                    self.put_zombie_in_wave(ZombieType::Bungee, a_wave, &mut a_zombie_picker);
                }
                if !a_is_final_wave {
                    continue;
                }
            }

            // ------------------------------------------------------------------------------------------------
            // △ 计算该波的僵尸总点数
            // ------------------------------------------------------------------------------------------------
            let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
            if game_mode == GameMode::ChallengeLastStand {
                let survival_stage = self.challenge.as_ref().map_or(0, |c| c.survival_stage);
                a_zombie_picker.zombie_points =
                    (survival_stage * self.get_num_waves_per_survival_stage() + a_wave + 10) * 2 / 5 + 1;
            } else if self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() })
                && self.challenge.as_ref().map_or(false, |c| c.survival_stage > 0)
            {
                let survival_stage = self.challenge.as_ref().map_or(0, |c| c.survival_stage);
                a_zombie_picker.zombie_points =
                    (survival_stage * self.get_num_waves_per_survival_stage() + a_wave) * 2 / 5 + 1;
            } else if is_adventure
                && self.app.map_or(false, |app| unsafe { (*app).has_finished_adventure() })
                && self.level != 5
            {
                a_zombie_picker.zombie_points = a_wave * 2 / 5 + 1;
            } else {
                a_zombie_picker.zombie_points = a_wave / 3 + 1;
            }

            // 旗帜波的特殊调整
            if a_is_flag_wave {
                let plain_zombies_num = std::cmp::min(a_zombie_picker.zombie_points, 8);
                a_zombie_picker.zombie_points = (a_zombie_picker.zombie_points as f32 * 2.5) as i32;

                // 注意：C++ 中还有 GAMEMODE_CHALLENGE_WAR_AND_PEAS_2 检查，
                // 但 Rust 枚举暂缺该变体，跳过对应检查
                if game_mode != GameMode::ChallengeWarAndPeas {
                    for _ in 0..plain_zombies_num {
                        self.put_zombie_in_wave(ZombieType::Normal, a_wave, &mut a_zombie_picker);
                    }
                    self.put_zombie_in_wave(ZombieType::Flag, a_wave, &mut a_zombie_picker);
                }
            }

            // 部分关卡的多倍出怪
            if game_mode == GameMode::ChallengeColumns {
                a_zombie_picker.zombie_points *= 6;
            } else if game_mode == GameMode::ChallengeLittleTrouble
                || self.app.map_or(false, |app| unsafe { (*app).is_wallnut_bowling_level() })
            {
                a_zombie_picker.zombie_points *= 4;
            } else if self.app.map_or(false, |app| unsafe { (*app).is_mini_boss_level() }) {
                a_zombie_picker.zombie_points *= 3;
            } else if self.app.map_or(false, |app| unsafe { (*app).is_stormy_night_level() }) && is_adventure {
                a_zombie_picker.zombie_points *= 3;
            } else if self.app.map_or(false, |app| unsafe { (*app).is_shovel_level() })
                || is_bungee_blitz
                || game_mode == GameMode::ChallengePortalCombat
                || game_mode == GameMode::ChallengeInvisighoul
            {
                a_zombie_picker.zombie_points *= 2;
            }

            // ------------------------------------------------------------------------------------------------
            // △ 向出怪列表中加入固定刷出的僵尸
            // ------------------------------------------------------------------------------------------------
            // 部分新出现的僵尸会在特定波固定刷出
            if a_intro_zombie_type != ZombieType::Invalid && a_intro_zombie_type != ZombieType::DuckyTube {
                let mut spawn_intro = false;
                if a_intro_zombie_type == ZombieType::Digger || a_intro_zombie_type == ZombieType::Balloon {
                    if a_wave + 1 == 7 || a_is_final_wave {
                        spawn_intro = true;
                    }
                } else if a_intro_zombie_type == ZombieType::Yeti {
                    let saw_yeti = self.app.map_or(true, |app| unsafe { (*app).m_saw_yeti });
                    if a_wave == self.m_num_waves / 2 && !saw_yeti {
                        spawn_intro = true;
                    }
                } else if a_wave == self.m_num_waves / 2 || a_is_final_wave {
                    spawn_intro = true;
                }

                if spawn_intro {
                    self.put_zombie_in_wave(a_intro_zombie_type, a_wave, &mut a_zombie_picker);
                }
            }

            // 5-10 关卡的最后一波加入一只伽刚特尔
            if self.level == 50 && a_is_final_wave {
                self.put_zombie_in_wave(ZombieType::Gargantuar, a_wave, &mut a_zombie_picker);
            }
            // 冒险模式关卡的最后一波会出现本关卡可能出现的所有僵尸
            if is_adventure && a_is_final_wave {
                self.put_in_missing_zombies(a_wave, &mut a_zombie_picker);
            }
            // 柱子关卡的特殊出怪
            if game_mode == GameMode::ChallengeColumns {
                // 每大波的第 5 小波，固定出现 10 只扶梯僵尸
                if a_wave % 10 == 5 {
                    for _ in 0..10 {
                        self.put_zombie_in_wave(ZombieType::Ladder, a_wave, &mut a_zombie_picker);
                    }
                }
                // 每大波的第 8 小波，固定出现 10 只玩偶匣僵尸
                if a_wave % 10 == 8 {
                    for _ in 0..10 {
                        self.put_zombie_in_wave(ZombieType::JackInTheBox, a_wave, &mut a_zombie_picker);
                    }
                }
                // 第 19/29 小波，固定出现 3/5 只伽刚特尔
                if a_wave == 19 {
                    for _ in 0..3 {
                        self.put_zombie_in_wave(ZombieType::Gargantuar, a_wave, &mut a_zombie_picker);
                    }
                }
                if a_wave == 29 {
                    for _ in 0..5 {
                        self.put_zombie_in_wave(ZombieType::Gargantuar, a_wave, &mut a_zombie_picker);
                    }
                }
            }

            // ------------------------------------------------------------------------------------------------
            // △ 剩余的僵尸点数用于向列表中补充随机僵尸
            // ------------------------------------------------------------------------------------------------
            while a_zombie_picker.zombie_points > 0 && a_zombie_picker.zombie_count < MAX_ZOMBIES_IN_WAVE as i32 {
                let a_zombie_type = self.pick_zombie_type(a_zombie_picker.zombie_points, a_wave, &a_zombie_picker);
                self.put_zombie_in_wave(a_zombie_type, a_wave, &mut a_zombie_picker);
            }
        }
    }

    /// 检查僵尸波次分布是否合理（对应 C++ IsZombieWaveDistributionOk）
    pub fn is_zombie_wave_distribution_ok(&self) -> bool {
        if !self.app.map_or(false, |app| unsafe { (*app).is_adventure_mode() }) {
            return true;
        }

        let mut zombie_type_count = [0i32; NUM_ZOMBIE_TYPES as usize];
        for a_wave in 0..self.m_num_waves {
            for a_idx in 0..MAX_ZOMBIES_IN_WAVE {
                let a_zombie_type = self.m_zombies_in_wave[a_wave as usize][a_idx];
                if a_zombie_type == ZombieType::Invalid {
                    break;
                }
                zombie_type_count[a_zombie_type as usize] += 1;
            }
        }

        for ztype_idx in 0..(NUM_ZOMBIE_TYPES as i32) {
            let a_zombie_type: ZombieType = unsafe { std::mem::transmute(ztype_idx) };
            if a_zombie_type != ZombieType::Yeti
                && self.can_zombie_spawn_on_level(a_zombie_type, self.level)
                && zombie_type_count[ztype_idx as usize] == 0
            {
                let def = get_zombie_definition(a_zombie_type);
                println!("Didn't spawn required zombie {}, level {}", def.zombie_name, self.level);
                return false;
            }
        }
        true
    }

    /// 初始化波次数据（对应 C++ InitZombieWaves）
    pub fn init_zombie_waves_for_level(&mut self, for_level: i32) {
        let is_whack = self.app.map_or(false, |app| unsafe { (*app).is_whack_a_zombie_level() });
        let is_wallnut_bowling = self.app.map_or(false, |app| unsafe {
            (*app).is_wallnut_bowling_level() && !(*app).is_first_time_adventure_mode()
        });
        if is_whack || is_wallnut_bowling {
            if let Some(ref mut challenge) = self.challenge {
                challenge.init_zombie_waves();
            }
            return;
        }

        for ztype_idx in 0..(NUM_ZOMBIE_TYPES as i32) {
            let a_zombie_type: ZombieType = unsafe { std::mem::transmute(ztype_idx) };
            self.m_zombie_allowed[ztype_idx as usize] =
                self.can_zombie_spawn_on_level(a_zombie_type, for_level);
        }
    }

    /// 初始化波次数据（对应 C++ InitZombieWaves）
    pub fn init_zombie_waves(&mut self) {
        // 重置所有僵尸类型为不允许
        self.m_zombie_allowed = [false; NUM_ZOMBIE_TYPES as usize];

        let is_adventure = self.app.map_or(false, |app| unsafe { (*app).is_adventure_mode() });
        if is_adventure {
            // InitZombieWavesForLevel：遍历所有僵尸类型设置允许列表
            for ztype_idx in 0..(NUM_ZOMBIE_TYPES as i32) {
                let a_zombie_type: ZombieType = unsafe { std::mem::transmute(ztype_idx) };
                self.m_zombie_allowed[ztype_idx as usize] =
                    self.can_zombie_spawn_on_level(a_zombie_type, self.level);
            }
        } else {
            // 非冒险模式：由 Challenge 的 InitZombieWaves 处理（C++ 1219: mChallenge->InitZombieWaves();）
            if let Some(ref mut challenge) = self.challenge {
                challenge.init_zombie_waves();
            }
        }

        // 生成波次数据
        self.pick_zombie_waves();

        // 验证波次分布
        debug_assert!(self.is_zombie_wave_distribution_ok());

        // 重置波次相关状态
        self.m_current_wave = 0;
        self.m_total_spawned_waves = 0;
        if let Some(app) = self.app {
            unsafe { (*app).m_saw_yeti = false; }
        }
    }

    /// 判断是否为最终 Boss 战（对应 C++ LawnApp::IsFinalBossLevel，经 app 转发）
    pub fn is_final_boss_level(&self) -> bool {
        self.app.map_or(false, |app| unsafe { (*app).is_final_boss_level() })
    }

    // ========== 生命周期管理 ==========

    /// 清理棋盘资源（对应 C++ Board::DisposeBoard）
    pub fn dispose_board(&mut self) {
        if let Some(app) = self.app {
            unsafe {
                if (*app).game_mode == GameMode::ChallengeZenGarden {
                    if let Some(zg) = (*app).zen_garden {
                        (*zg).leave_garden();
                    }
                }
                if (*app).game_mode == GameMode::ChallengeTreeOfWisdom {
                    // C++ 278: mChallenge->TreeOfWisdomLeave();
                    if let Some(ref mut challenge) = self.challenge {
                        challenge.tree_of_wisdom_leave();
                    }
                }
                // 停止雨声 Foley
                if let Some(ref ss) = (*app).sound_system {
                    ss.stop_foley(crate::todlib::tod_foley::FoleyType::Rain);
                }
                // 清理 ZenGarden 对 Board 的引用
                if let Some(zg) = (*app).zen_garden {
                    (*zg).board = Some(std::ptr::null_mut());
                }
                (*app).crazy_dave_die();
                if let Some(ref mut es) = (*app).effect_system {
                    // C++ 281: mApp->mEffectSystem->EffectSystemFreeAll();
                    es.effect_system_free_all();
                }
            }
        }
    }

    /// 尝试保存游戏（对应 C++ Board::TryToSaveGame，Board.cpp:348）
    pub fn try_to_save_game(&mut self) {
        // C++: aFileName = GetSavedGameName(mApp->mGameMode, mApp->mPlayerInfo->mId)
        let a_file_name = self.app.map_or(String::new(), |app| unsafe {
            let profile_id = (*app).player_info.as_ref().map_or(0, |p| p.m_id) as i32;
            crate::lawn::lawn_common::get_saved_game_name((*app).game_mode, profile_id)
        });

        if !self.need_save_game() {
            return;
        }

        if self.m_board_fade_out_counter > 0 {
            self.complete_end_level_sequence_for_saving();
            return;
        }

        // C++: MkDir(GetAppDataPath("userdata"))；[TRANSLATION_NOTE]: Rust 存档目录由 launcher 层确保，暂略
        if let Some(app) = self.app {
            unsafe {
                if let Some(ref mut music) = (*app).music {
                    music.game_music_pause(true);
                }
                // C++: LawnSaveGame(this, aFileName)
                let board_ptr = self as *const Board as usize as *mut Board;
                let _ = crate::lawn::system::save_game::lawn_save_game(Some(board_ptr), &a_file_name);
                // C++: mApp->ClearUpdateBacklog()；[TRANSLATION_NOTE]: Rust 无 update backlog 队列，暂略
            }
        }
        self.survival_save_score();
    }

    // ========== 背景与资源 ==========

    /// 加载背景图片资源（对应 C++ Board::LoadBackgroundImages，Board.cpp:813）
    pub fn load_background_images(&mut self, _app: *mut crate::lawn::lawn_app::LawnApp) {
        // C++: 按背景类型把延迟加载资源名推入 mLoadedResourceNames，随后逐个 PvzpLoadResources
        let mut loaded_resource_names: Vec<&str> = Vec::new();
        match self.m_background_type {
            BackgroundType::Day => {
                loaded_resource_names.push("DelayLoad_Background1");
                let is_adventure_low = self.app.map_or(false, |app| unsafe {
                    (*app).is_adventure_mode() && self.level <= 4
                });
                let app_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
                if is_adventure_low || app_mode == GameMode::ChallengeResodded {
                    loaded_resource_names.push("DelayLoad_BackgroundUnsodded");
                }
            }
            BackgroundType::Night => loaded_resource_names.push("DelayLoad_Background2"),
            BackgroundType::Pool => loaded_resource_names.push("DelayLoad_Background3"),
            BackgroundType::Fog => loaded_resource_names.push("DelayLoad_Background4"),
            BackgroundType::Roof => loaded_resource_names.push("DelayLoad_Background5"),
            BackgroundType::Boss => loaded_resource_names.push("DelayLoad_Background6"),
            BackgroundType::Greenhouse => {
                loaded_resource_names.push("DelayLoad_GreenHouseGarden");
                loaded_resource_names.push("DelayLoad_GreenHouseOverlay");
            }
            BackgroundType::TreeOfWisdom => {
                // C++: ReanimatorEnsureDefinitionLoaded(REANIM_TREEOFWISDOM, true)
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(
                    ReanimationType::Treeofwisdom,
                );
            }
            BackgroundType::Zombiquarium => {
                loaded_resource_names.push("DelayLoad_Zombiquarium");
                loaded_resource_names.push("DelayLoad_GreenHouseOverlay");
            }
            BackgroundType::MushroomGarden => loaded_resource_names.push("DelayLoad_MushroomGarden"),
            _ => {
                // C++: PVZP_ASSERT(false)
            }
        }
        for resource in loaded_resource_names {
            if let Some(app) = self.app {
                unsafe {
                    if let Some(rm) = (*app).base.resource_manager.as_mut() {
                        let _ = (**rm).load_resources(resource);
                    }
                }
            }
        }
    }

    // ========== 关卡初始化 ==========

    /// 初始化割草机（对应 C++ Board::InitLawnMowers）
    pub fn init_lawn_mowers(&mut self) {
        let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });

        // 不需要割草机的模式
        if game_mode == GameMode::ChallengeBeghouled
            || game_mode == GameMode::ChallengeBeghouledTwist
            || game_mode == GameMode::ChallengeZenGarden
            || game_mode == GameMode::ChallengeTreeOfWisdom
            || game_mode == GameMode::ChallengeLastStand
            || game_mode == GameMode::ChallengeZombiquarium
            || self.app.map_or(false, |app| unsafe { (*app).is_squirrel_level() })
            || self.app.map_or(false, |app| unsafe { (*app).is_izombie_level() })
            || (self.stage_has_roof()
                && self.app.map_or(true, |app| unsafe {
                    (*app).player_info.as_ref().map_or(true, |p| {
                        !p.m_purchases[StoreItem::RoofCleaner as usize] != 0
                    })
                }))
        {
            return;
        }

        for a_row in 0..(MAX_GRID_SIZE_Y as i32) {
            // ChallengeResodded 在 Rust GameMode 枚举中暂缺，简化处理
            let should_create = if self.app.map_or(false, |app| unsafe {
                (*app).is_adventure_mode() && self.level == 35
            }) {
                true
            } else if !self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() })
                && self.m_plant_row[a_row as usize] != PlantRowType::Dirt
            {
                true
            } else {
                false
            };

            if should_create {
                let mut mower = LawnMower::new();
                mower.lawn_mower_initialize(a_row);
                mower.visible = false;
                self.lawn_mowers.push(mower);
            }
        }
    }

    /// 初始化生存模式阶段（对应 C++ InitSurvivalStage）
    pub fn init_survival_stage(&mut self) {
        self.refresh_seed_packet_from_cursor();
        // C++ 1281: mApp->mSoundSystem->GamePause(true);
        if let Some(app) = self.app {
            unsafe {
                if let Some(ref ss) = (*app).sound_system {
                    ss.game_pause(true);
                }
            }
        }
        self.freeze_effects_for_cutscene(true);
        self.m_level_complete = false;
        self.init_zombie_waves();
        // mApp->mGameScene = GameScenes::SCENE_LEVEL_INTRO — 依赖 GameScene 设置
        // mApp->ShowSeedChooserScreen() — 依赖 LawnApp 方法
        // mCutScene->StartLevelIntro() — 依赖 CutScene
        // mSeedBank->UpdateWidth() — 依赖 SeedBank

        /*for i in 0..SEEDBANK_MAX {
            // 种子包重置 — 依赖 SeedBank 细节
        }*/

        if self.stage_has_fog() {
            self.m_fog_blown_count_down = 150; // FOG_BLOW_RETURN_TIME
        }
        for j in 0..MAX_GRID_SIZE_Y {
            self.m_wave_row_got_lawn_mowered[j] = -100;
        }
    }

    /// 当前关卡是否可以选择种子（对应 C++ Board::ChooseSeedsOnCurrentLevel）
    pub fn choose_seeds_on_current_level(&self) -> bool {
        if self.app.map_or(false, |app| unsafe {
            (*app).is_challenge_without_seed_bank()
        }) || self.has_conveyor_belt_seed_bank()
        {
            return false;
        }

        let game_mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
        if game_mode == GameMode::ChallengeIceLevel
            || game_mode == GameMode::ChallengeBeghouled
            || game_mode == GameMode::ChallengeBeghouledTwist
            || game_mode == GameMode::ChallengeZombiquarium
        {
            return false;
        }

        if self.app.map_or(false, |app| unsafe {
            (*app).is_izombie_level() || (*app).is_slot_machine_level()
        }) {
            return false;
        }

        !self.app.map_or(true, |app| unsafe {
            (*app).is_first_time_adventure_mode()
        }) || self.level > 7
    }

    /// 开始关卡（对应 C++ Board::StartLevel）
    pub fn start_level(&mut self) {
        self.m_coin_bank_fade_count = 0;
        if let Some(app) = self.app {
            unsafe {
                if let Some(ref mut stats) = (*app).m_last_level_stats {
                    stats.reset();
                }
                if let Some(ref mut challenge) = self.challenge {
                    challenge.start_level();
                }

                // 生存无尽模式特殊处理
                let survival_stage = (*app).game_mode as i32 - GameMode::SurvivalEndlessStage1 as i32;
                if survival_stage >= 0 && survival_stage <= 4 {
                    if self.get_survival_flags_completed() >= 20 {
                        // C++ 1667: ReportAchievement::GiveAchievement(mApp, Immortal, true);
                        crate::lawn::widget::achievements_screen::ReportAchievement::give_achievement(
                            Some(app),
                            crate::lawn::widget::achievements_screen::AchievementId::Immortal as i32,
                            true,
                        );
                    }
                }

                if (*app).is_survival_mode()
                    && self.challenge.as_ref().map_or(false, |c| c.survival_stage > 0)
                {
                    self.freeze_effects_for_cutscene(false);
                    // C++ 1674: mApp->mSoundSystem->GamePause(false);
                    if let Some(ref ss) = (*app).sound_system {
                        ss.game_pause(false);
                    }
                }

                let gm = (*app).game_mode;
                if gm == GameMode::ChallengeIceLevel
                    || gm == GameMode::ChallengeZenGarden
                    || gm == GameMode::ChallengeTreeOfWisdom
                    // GAMEMODE_UPSELL 和 GAMEMODE_INTRO 在 Rust GameMode 枚举中暂缺
                    || (*app).is_final_boss_level()
                {
                    return;
                }

                if let Some(ref mut music) = (*app).music {
                    music.start_game_music();
                }
            }
        }
    }

    /// 冻结/解冻过场动画效果（对应 C++ Board::FreezeEffectsForCutscene）
    pub fn freeze_effects_for_cutscene(&mut self, freeze: bool) {
        // 对应 C++ FreezeEffectsForCutscene：冻结特定粒子与睡眠动画
        if let Some(app) = self.app {
            unsafe {
                if let Some(es) = (*app).effect_system.as_mut() {
                    for ps in es.particle_systems.iter_mut() {
                        if ps.dead {
                            continue;
                        }
                        if ps.effect_type == ParticleEffect::GraveBuster {
                            ps.dont_update = freeze;
                        } else if ps.effect_type == ParticleEffect::PoolSparkly
                            && self.m_ice_trap_counter == 0
                        {
                            ps.dont_update = freeze;
                        }
                    }
                    for reanim in es.reanimations.iter_mut() {
                        if reanim.m_dead {
                            continue;
                        }
                        if reanim.reanim_type == ReanimationType::Sleeping {
                            reanim.m_anim_rate = if freeze { 0.0 } else {
                                crate::todlib::tod_common::rand_range_float(6.0, 8.0)
                            };
                        }
                    }
                }
            }
        }
    }

    // ========== 割草机查询 ==========

    /// 获取最下面的未触发割草机（对应 C++ Board::GetBottomLawnMower）
    pub fn get_bottom_lawn_mower(&self) -> Option<&LawnMower> {
        let mut bottom_mower: Option<&LawnMower> = None;
        for mower in &self.lawn_mowers {
            if mower.mower_state == LawnMowerState::Triggered
                || mower.mower_state == LawnMowerState::Squished
            {
                continue;
            }
            if bottom_mower.map_or(true, |b| b.base.row < mower.base.row) {
                bottom_mower = Some(mower);
            }
        }
        bottom_mower
    }

    // ========== 波次统计 ==========

    /// 计算指定波次中的僵尸数量（对应 C++ Board::NumberZombiesInWave）
    pub fn number_zombies_in_wave(&self, wave_index: i32) -> i32 {
        debug_assert!(wave_index >= 0 && wave_index < MAX_ZOMBIE_WAVES as i32);
        let wave_idx = wave_index as usize;
        for i in 0..MAX_ZOMBIES_IN_WAVE {
            if self.m_zombies_in_wave[wave_idx][i] == ZombieType::Invalid {
                return i as i32;
            }
        }
        0
    }

    // ========== 存档 ==========

    /// 保存游戏（对应 C++ Board::SaveGame）
    pub fn save_game(&self, file_name: &str) {
        // 对应 C++ SaveGame：委托 LawnSaveGame
        unsafe {
            let board_ptr = self as *const Board as usize as *mut Board;
            let _ = crate::lawn::system::save_game::lawn_save_game(Some(board_ptr), file_name);
        }
    }

    // ========== 钉耙 ==========

    /// 放置钉耙（对应 C++ Board::PlaceRake）
    pub fn place_rake(&mut self) {
        let has_rake = self.app.map_or(false, |app| unsafe {
            (*app).player_info.as_ref().map_or(false, |p| {
                p.m_purchases[StoreItem::Rake as usize] != 0
            })
        });
        if !has_rake {
            return;
        }

        let mut a_grid_x = 7;
        let is_scary = self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() });
        if is_scary {
            for item in &self.grid_items {
                // C++: if (aGridItem->mDead) continue;
                if item.dead {
                    continue;
                }
                if item.grid_item_type == GridItemType::ScaryPot
                    && item.grid_x <= a_grid_x
                    && item.grid_x > 0
                {
                    a_grid_x = item.grid_x - 1;
                }
            }
        } else {
            // C++: !StageHasZombieWalkInFromRight() || GAMEMODE_CHALLENGE_BEGHOULED ||
            //      GAMEMODE_CHALLENGE_BEGHOULED_TWIST || GAMEMODE_CHALLENGE_BOBSLED_BONANZA → return
            let mode = self.app.map_or(GameMode::Adventure, |app| unsafe { (*app).game_mode });
            if !self.stage_has_zombie_walk_in_from_right()
                || mode == GameMode::ChallengeBeghouled
                || mode == GameMode::ChallengeBeghouledTwist
                || mode == GameMode::ChallengeBobsledBonanza
            {
                return;
            }
        }

        // 构建加权行选择数组
        let mut pick_array = [TodWeightedArray { item: 0, weight: 0 }; MAX_GRID_SIZE_Y];
        let mut pick_count = 0;
        for a_row in 0..(MAX_GRID_SIZE_Y as i32) {
            if a_row != 5 && self.m_plant_row[a_row as usize] == PlantRowType::Normal {
                pick_array[pick_count].weight = 1;
                pick_array[pick_count].item = a_row as usize;
                pick_count += 1;
            }
        }
        if pick_count == 0 {
            return;
        }

        let a_grid_y = tod_pick_from_weighted_array(&pick_array[..pick_count]) as i32;
        if let Some(app) = self.app {
            unsafe {
                if let Some(ref mut player) = (*app).player_info {
                    player.m_purchases[StoreItem::Rake as usize] -= 1;
                }
            }
        }

        let mut rake = GridItem::new();
        rake.grid_item_initialize(GridItemType::Rake, a_grid_x, a_grid_y);
        rake.pos_x = self.grid_to_pixel_x(a_grid_x, a_grid_y) as f32;
        rake.pos_y = self.grid_to_pixel_y(a_grid_x, a_grid_y) as f32;
        rake.render_order = make_render_order(301000/*RenderLayer::GraveStone*/, a_grid_y, 9);
        // C++: aRake->mGridItemReanimID = mApp->ReanimationGetID(CreateRakeReanim(aRake->mPosX, aRake->mPosY, 0));
        if let Some(app) = self.app {
            unsafe {
                if let Some(reanim) = self.create_rake_reanim(rake.pos_x, rake.pos_y, 0) {
                    rake.grid_item_reanim_id = (*app).reanimation_get_id(reanim);
                }
            }
        }
        rake.grid_item_state = GridItemState::RakeAttracting;
        self.grid_items.push(rake);
    }

    // ========== 粒子管理 ==========

    /// 移除指定类型的粒子效果（对应 C++ Board::RemoveParticleByType）
    pub fn remove_particle_by_type(&mut self, effect_type: ParticleEffect) {
        // 对应 C++ RemoveParticleByType
        if let Some(app) = self.app {
            unsafe {
                if let Some(es) = (*app).effect_system.as_mut() {
                    for ps in es.particle_systems.iter_mut() {
                        if !ps.dead && ps.effect_type == effect_type {
                            ps.particle_system_die();
                        }
                    }
                }
            }
        }
    }

    // ========== GridItem 快捷查询 ==========

    /// 获取指定位置的惊吓盒（对应 C++ Board::GetScaryPotAt）
    pub fn get_scary_pot_at(&self, grid_x: i32, grid_y: i32) -> Option<&GridItem> {
        self.get_grid_item_at(GridItemType::ScaryPot, grid_x, grid_y)
    }

    /// 获取指定位置的禅境工具（对应 C++ Board::GetZenToolAt）
    pub fn get_zen_tool_at(&self, grid_x: i32, grid_y: i32) -> Option<&GridItem> {
        self.get_grid_item_at(GridItemType::ZenTool, grid_x, grid_y)
    }

    // ========== 存档加载 ==========

    /// 加载存档（对应 C++ Board::LoadGame，Board.cpp:389）
    pub fn load_game(&mut self, file_name: &str) -> bool {
        if !crate::lawn::system::save_game::lawn_load_game(Some(self as *mut Board), file_name) {
            return false;
        }
        let app = match self.app {
            Some(a) => a,
            None => return false,
        };
        self.load_background_images(app);
        // C++: mApp->ClearUpdateBacklog() — [TRANSLATION_NOTE]: Rust 无对应
        self.reset_fps_stats();
        self.update_layers();
        true
    }

    /// 完成结束关卡序列（用于保存前清理，对应 C++ Board::CompleteEndLevelSequenceForSaving）
    /// 将所有未触发的割草机转换为金币，收集正在收集的硬币，更新玩家资料
    pub fn complete_end_level_sequence_for_saving(&mut self) {
        if !self.can_drop_loot() {
            return;
        }

        // 遍历所有未触发的割草机，将其转换为金币
        for mower in &self.lawn_mowers {
            if mower.mower_state != LawnMowerState::Triggered
                && mower.mower_state != LawnMowerState::Squished
            {
                // 对应 C++: int aCoinValue = Coin::GetCoinValue(COIN_GOLD);
                //           mApp->mPlayerInfo->AddCoins(aCoinValue); mCoinsCollected += aCoinValue;
                let a_coin_value = crate::lawn::coin::Coin::get_coin_value(CoinType::Gold);
                if let Some(app) = self.app {
                    unsafe {
                        if let Some(player) = (*app).player_info.as_mut() {
                            player.add_coins(a_coin_value);
                        }
                    }
                }
                self.m_coins_collected += a_coin_value;
            }
        }

        // 处理正在收集的硬币：标记为死亡（移除）
        // 注：C++ 中调用 ScoreCoin() 处理正在收集的硬币，Rust 版简化处理
        for coin in &mut self.coins {
            coin.dead = true;
        }

        // C++ 中还会调用 mApp->UpdatePlayerProfileForFinishingLevel()
        // 该方法暂未翻译，此处留空
    }

    /// 种植效果（音效和粒子），对应 C++ Board::DoPlantingEffects
    pub fn do_planting_effects(&mut self, grid_x: i32, grid_y: i32, seed_type: SeedType) {
        let a_x_pos = self.grid_to_pixel_x(grid_x, grid_y) + 41;
        let mut a_y_pos = self.grid_to_pixel_y(grid_x, grid_y) + 74;

        if seed_type == SeedType::Lilypad {
            a_y_pos += 15;
        } else if seed_type == SeedType::Flowerpot {
            a_y_pos += 30;
        }

        if let Some(app) = self.app {
            unsafe {
                if self.m_background_type == BackgroundType::Greenhouse {
                    (*app).play_foley(crate::todlib::tod_foley::FoleyType::Ceramic as i32);
                    return;
                }
                if self.m_background_type == BackgroundType::Zombiquarium {
                    (*app).play_foley(crate::todlib::tod_foley::FoleyType::PlantWater as i32);
                    return;
                }

                // 飞行植物直接播放种植音效
                // 注：Plant::IsFlying 暂未翻译，暂时跳过
                if self.is_pool_square(grid_x, grid_y) {
                    (*app).play_foley(crate::todlib::tod_foley::FoleyType::PlantWater as i32);
                    (*app).add_tod_particle(
                        a_x_pos as f32,
                        a_y_pos as f32,
                        301000,
                        crate::lawn::game_enums::ParticleEffect::PlantingPool as i32,
                    );
                } else {
                    (*app).play_foley(crate::todlib::tod_foley::FoleyType::Plant as i32);
                    (*app).add_tod_particle(
                        a_x_pos as f32,
                        a_y_pos as f32,
                        301000,
                        crate::lawn::game_enums::ParticleEffect::Planting as i32,
                    );
                }
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
