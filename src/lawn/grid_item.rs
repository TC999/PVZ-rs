// PvZ Portable Rust 翻译 — GridItem（格子物品：墓碑/卵石/车等）
// 对应 C++ src/Lawn/GridItem.h / GridItem.cpp

use crate::lawn::game_object::GameObject;
use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;

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
}

/// 格子物品（地形元素等）
pub struct GridItem {
    pub base: GameObject,

    pub grid_item_type: GridItemType,
    pub grid_x: i32,
    pub grid_y: i32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub frame: i32,
    pub counter: i32,
    pub dead: bool,
}

impl GridItem {
    pub fn new() -> Self {
        GridItem {
            base: GameObject::new(),
            grid_item_type: GridItemType::None,
            grid_x: 0,
            grid_y: 0,
            pos_x: 0.0,
            pos_y: 0.0,
            frame: 0,
            counter: 0,
            dead: false,
        }
    }

    /// 初始化格子物品
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
}

impl Default for GridItem {
    fn default() -> Self {
        GridItem::new()
    }
}
