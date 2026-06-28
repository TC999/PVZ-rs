// PvZ Portable Rust 翻译 — AlmanacDialog（图鉴对话框）
// 对应 C++ src/Lawn/Widget/AlmanacDialog.h / AlmanacDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::*;
use crate::lawn::widget::game_button::GameButton;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::todlib::reanimator::Reanimation;

pub const NUM_ALMANAC_SEEDS: i32 = 49;
pub const NUM_ALMANAC_ZOMBIES: i32 = 26;
pub const ALMANAC_PLANT_POSITION_X: f32 = 578.0;
pub const ALMANAC_PLANT_POSITION_Y: f32 = 140.0;
pub const ALMANAC_ZOMBIE_POSITION_X: f32 = 559.0;
pub const ALMANAC_ZOMBIE_POSITION_Y: f32 = 175.0;
pub const ALMANAC_INDEXPLANT_POSITION_X: i32 = 167;
pub const ALMANAC_INDEXPLANT_POSITION_Y: i32 = 255;
pub const ALMANAC_INDEXZOMBIE_POSITION_X: f32 = 535.0;
pub const ALMANAC_INDEXZOMBIE_POSITION_Y: f32 = 215.0;

/// 图鉴对话框（对应 C++ AlmanacDialog）
pub struct AlmanacDialog {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub close_button: Option<*mut GameButton>,
    pub index_button: Option<*mut GameButton>,
    pub plant_button: Option<*mut GameButton>,
    pub zombie_button: Option<*mut GameButton>,
    pub open_page: AlmanacPage,
    pub reanim: [Option<*mut Reanimation>; 4],
    pub selected_seed: SeedType,
    pub selected_zombie: ZombieType,
    pub plant: Option<*mut crate::lawn::plant::Plant>,
    pub zombie: Option<*mut crate::lawn::zombie::Zombie>,
}

impl AlmanacDialog {
    pub fn new() -> Self {
        AlmanacDialog {
            app: None,
            close_button: None,
            index_button: None,
            plant_button: None,
            zombie_button: None,
            open_page: AlmanacPage::Index,
            reanim: [None, None, None, None],
            selected_seed: SeedType::None,
            selected_zombie: ZombieType::Invalid,
            plant: None,
            zombie: None,
        }
    }

    pub fn clear_plants_and_zombies(&mut self) { /* TODO */ }
    pub fn removed_from_manager(&mut self, _mgr: &mut WidgetManager) { /* TODO */ }
    pub fn setup_plant(&mut self) { /* TODO */ }
    pub fn setup_zombie(&mut self) { /* TODO */ }
    pub fn set_page(&mut self, _page: AlmanacPage) { /* TODO */ }
    pub fn update(&mut self) { /* TODO */ }
    pub fn draw_index(&self, _g: &mut Graphics) { /* TODO */ }
    pub fn draw_plants(&self, _g: &mut Graphics) { /* TODO */ }
    pub fn draw_zombies(&self, _g: &mut Graphics) { /* TODO */ }
    pub fn draw(&self, _g: &mut Graphics) { /* TODO */ }
    pub fn get_seed_position(&self, _t: SeedType, _x: &mut i32, _y: &mut i32) { /* TODO */ }
    pub fn seed_hit_test(&self, _x: i32, _y: i32) -> SeedType { SeedType::None /* TODO */ }
    pub fn zombie_has_silhouette(&self, _t: ZombieType) -> bool { false /* TODO */ }
    pub fn zombie_is_shown(&self, _t: ZombieType) -> bool { false /* TODO */ }
    pub fn zombie_has_description(&self, _t: ZombieType) -> bool { false /* TODO */ }
    pub fn get_zombie_position(&self, _t: ZombieType, _x: &mut i32, _y: &mut i32) { /* TODO */ }
    pub fn zombie_hit_test(&self, _x: i32, _y: i32) -> ZombieType { ZombieType::Invalid /* TODO */ }
    pub fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) { /* TODO */ }
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) { /* TODO */ }
    pub fn get_zombie_type(index: i32) -> ZombieType { ZombieType::Invalid /* TODO */ }
    pub fn show_plant(&mut self, _t: SeedType) { /* TODO */ }
    pub fn show_zombie(&mut self, _t: ZombieType) { /* TODO */ }
}

impl Default for AlmanacDialog {
    fn default() -> Self { AlmanacDialog::new() }
}

/// 全局僵尸击败标记数组（对应 C++ gZombieDefeated）
pub static mut G_ZOMBIE_DEFEATED: [bool; NUM_ZOMBIE_TYPES as usize] = [false; NUM_ZOMBIE_TYPES as usize];

/// 初始化玩家图鉴数据（对应 C++ AlmanacInitForPlayer）
/// 重置所有僵尸的已击败标记
pub fn almanac_init_for_player() {
    unsafe {
        for i in 0..NUM_ZOMBIE_TYPES as usize {
            G_ZOMBIE_DEFEATED[i] = false;
        }
    }
}
