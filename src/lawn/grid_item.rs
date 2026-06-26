// PvZ Portable Rust 翻译 — GridItem（格子物品：墓碑/卵石/梯子/传送门等）
// 对应 C++ src/Lawn/GridItem.h / GridItem.cpp

use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;

/// 运动轨迹帧数（对应 C++ #define NUM_MOTION_TRAIL_FRAMES 12）
pub const NUM_MOTION_TRAIL_FRAMES: usize = 12;

/// 运动轨迹帧（对应 C++ MotionTrailFrame）
/// 用于记录物体运动的历史位置，实现残影效果
#[derive(Debug, Clone, Copy)]
pub struct MotionTrailFrame {
    pub pos_x: f32,
    pub pos_y: f32,
    pub anim_time: f32,
}

impl MotionTrailFrame {
    pub fn new() -> Self {
        MotionTrailFrame {
            pos_x: 0.0,
            pos_y: 0.0,
            anim_time: 0.0,
        }
    }
}

impl Default for MotionTrailFrame {
    fn default() -> Self {
        MotionTrailFrame::new()
    }
}

/// 格子物品类型（本地扩展版本，与 game_enums 中的 GridItemType 不同）
/// 用于 board.rs 中的实际游戏逻辑
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GridItemType {
    None = 0,
    Grave,
    Crater,
    Ladder,
    PortalCrystalBall,
    PortalSquare,
    PlantStinky,
    ScaryPot,
    DanceEggplant,
    Stone,
    WaterPlant,
    TreeOfWisdom,
    ZenTool,
    Cart,
    Rake,
}

/// 格子物品（地形元素等）
/// 对应 C++ class GridItem（独立类，不继承 GameObject）
pub struct GridItem {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub board: Option<*mut crate::lawn::board::Board>,

    pub grid_item_type: GridItemType,
    pub grid_item_state: GridItemState,
    pub grid_x: i32,
    pub grid_y: i32,
    pub counter: i32,
    pub render_order: i32,
    pub dead: bool,
    pub pos_x: f32,
    pub pos_y: f32,
    pub goal_x: f32,
    pub goal_y: f32,
    pub grid_item_reanim_id: ReanimationID,
    pub grid_item_particle_id: ParticleSystemID,
    pub zombie_type: ZombieType,
    pub seed_type: SeedType,
    pub scary_pot_type: ScaryPotType,
    pub highlighted: bool,
    pub transparent_counter: i32,
    pub sun_count: i32,
    pub motion_trail_frames: [MotionTrailFrame; NUM_MOTION_TRAIL_FRAMES],
    pub motion_trail_count: i32,
}

impl GridItem {
    pub fn new() -> Self {
        GridItem {
            app: None,
            board: None,
            grid_item_type: GridItemType::None,
            grid_item_state: GridItemState::Normal,
            grid_x: 0,
            grid_y: 0,
            counter: 0,
            render_order: 0,
            dead: false,
            pos_x: 0.0,
            pos_y: 0.0,
            goal_x: 0.0,
            goal_y: 0.0,
            grid_item_reanim_id: REANIMATIONID_NULL,
            grid_item_particle_id: PARTICLESYSTEMID_NULL,
            zombie_type: ZombieType::Invalid,
            seed_type: SeedType::Peashooter,
            scary_pot_type: ScaryPotType::None,
            highlighted: false,
            transparent_counter: 0,
            sun_count: 0,
            motion_trail_frames: [MotionTrailFrame::new(); NUM_MOTION_TRAIL_FRAMES],
            motion_trail_count: 0,
        }
    }

    /// 初始化格子物品（对应 C++ GridItemCreate 逻辑）
    pub fn grid_item_initialize(&mut self, item_type: GridItemType, grid_x: i32, grid_y: i32) {
        self.grid_item_type = item_type;
        self.grid_x = grid_x;
        self.grid_y = grid_y;
        self.pos_x = (40 + grid_x * 80) as f32;
        self.pos_y = (80 + grid_y * 100) as f32;
    }

    /// 更新
    pub fn update(&mut self) {
        if self.dead { return; }
        self.counter += 1;
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics) {}

    // 以下方法签名对应 C++ GridItem 的公开方法，方法体待从 GridItem.cpp 翻译
    /*
    pub fn draw_ladder(&self, g: &mut Graphics) {}
    pub fn draw_crater(&self, g: &mut Graphics) {}
    pub fn draw_grave_stone(&self, g: &mut Graphics) {}
    pub fn grid_item_die(&mut self) {}
    pub fn add_grave_stone_particles(&mut self) {}
    pub fn draw_grid_item(&self, g: &mut Graphics) {}
    pub fn draw_grid_item_overlay(&self, g: &mut Graphics) {}
    pub fn open_portal(&mut self) {}
    pub fn close_portal(&mut self) {}
    pub fn draw_scary_pot(&self, g: &mut Graphics) {}
    pub fn update_scary_pot(&mut self) {}
    pub fn update_portal(&mut self) {}
    pub fn draw_squirrel(&self, g: &mut Graphics) {}
    pub fn update_rake(&mut self) {}
    pub fn rake_find_zombie(&self) -> Option<*mut crate::lawn::zombie::Zombie> { None }
    pub fn draw_i_zombie_brain(&self, g: &mut Graphics) {}
    pub fn update_brain(&mut self) {}
    pub fn draw_stinky(&self, g: &mut Graphics) {}
    pub fn is_open_portal(&self) -> bool { false }
    */
}

impl Default for GridItem {
    fn default() -> Self {
        GridItem::new()
    }
}
