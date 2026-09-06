// PvZ Portable Rust 翻译 — AlmanacDialog（图鉴对话框）
// 对应 C++ src/Lawn/Widget/AlmanacDialog.h / AlmanacDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::{KEYCODE_ESCAPE, KeyCode};
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

    pub fn clear_plants_and_zombies(&mut self) {
        self.plant = None;
        self.zombie = None;
    }
    pub fn removed_from_manager(&mut self, _mgr: &mut WidgetManager) {
        self.clear_plants_and_zombies();
    }
    pub fn setup_plant(&mut self) {
        self.clear_plants_and_zombies();
        self.plant = None;
    }
    pub fn setup_zombie(&mut self) {
        self.clear_plants_and_zombies();
        self.zombie = None;
    }
    pub fn set_page(&mut self, page: AlmanacPage) {
        self.open_page = page;
        self.clear_plants_and_zombies();
    }

    /// 对应 C++ AlmanacDialog::KeyDown（AlmanacDialog.cpp）
    pub fn key_down(&mut self, key: KeyCode) {
        if key == KEYCODE_ESCAPE {
            if self.open_page == AlmanacPage::Index {
                if let Some(app) = self.app {
                    unsafe {
                        (*app).kill_almanac_dialog();
                    }
                }
            } else {
                self.set_page(AlmanacPage::Index);
            }
            return;
        }

        // C++: LawnDialog::KeyDown(theKey) —— 基类按键处理，Rust 侧无对应实现
    }
    pub fn update(&mut self) {
        if let Some(app) = self.app { unsafe {
            let mx = 0; let my = 0;
            if self.seed_hit_test(mx, my) != SeedType::None || self.zombie_hit_test(mx, my) != ZombieType::Invalid {}
        } }
    }
    pub fn draw_index(&self, _g: &mut Graphics) { /* \u4f9d\u8d56\u56fe\u7247\u8d44\u6e90 */ }
    pub fn draw_plants(&self, _g: &mut Graphics) { /* \u4f9d\u8d56\u56fe\u7247\u8d44\u6e90 */ }
    pub fn draw_zombies(&self, _g: &mut Graphics) { /* \u4f9d\u8d56\u56fe\u7247\u8d44\u6e90 */ }
    pub fn draw(&self, _g: &mut Graphics) { /* \u4f9d\u8d56\u56fe\u7247\u8d44\u6e90 */ }
    pub fn get_seed_position(&self, t: SeedType, x: &mut i32, y: &mut i32) {
        if t == SeedType::Imitater { *x = 20; *y = 23; }
        else { *x = (t as i32) % 8 * 52 + 26; *y = (t as i32) / 8 * 78 + 92; }
    }
    pub fn seed_hit_test(&self, x: i32, y: i32) -> SeedType {
        if self.open_page == AlmanacPage::Plants {
            for st in 0..NUM_ALMANAC_SEEDS {
                let seed_type = unsafe { std::mem::transmute::<i32, SeedType>(st) };
                if let Some(app) = self.app { unsafe {
                    if (*app).has_seed_type(seed_type) {
                        let (sx, sy) = (0, 0);
                        let rect = crate::framework::rect::Rect::new(sx, sy, 50, 70);
                        if rect.contains(x, y) { return seed_type; }
                    }
                } }
            }
        }
        SeedType::None
    }
    pub fn zombie_has_silhouette(&self, t: ZombieType) -> bool {
        if t == ZombieType::Yeti { if let Some(app) = self.app { unsafe { return !(*app).can_spawn_yetis(); } } }
        false
    }
    pub fn zombie_is_shown(&self, t: ZombieType) -> bool {
        if let Some(app) = self.app { unsafe {
            if (*app).is_trial_stage_locked() && (t as i32) > (ZombieType::Snorkel as i32) { return false; }
            if t == ZombieType::Yeti { return (*app).can_spawn_yetis() || self.zombie_has_silhouette(t); }
            if (t as i32) <= (ZombieType::Boss as i32) { return true; }
        } }
        false
    }
    pub fn zombie_has_description(&self, t: ZombieType) -> bool {
        if (t as i32) <= (ZombieType::Boss as i32) { return true; }
        if let Some(app) = self.app { unsafe { (*app).has_finished_adventure() } } else { false }
    }
    pub fn get_zombie_position(&self, t: ZombieType, x: &mut i32, y: &mut i32) {
        *x = (t as i32) % 5 * 53 + 10;
        *y = (t as i32) / 5 * 80 + 140;
    }
    pub fn zombie_hit_test(&self, x: i32, y: i32) -> ZombieType {
        if self.open_page == AlmanacPage::Zombies {
            for zt in 0..NUM_ALMANAC_ZOMBIES {
                let ztype = unsafe { std::mem::transmute::<i32, ZombieType>(zt) };
                let (zx, zy) = (0, 0);
                let rect = crate::framework::rect::Rect::new(zx, zy, 50, 70);
                if rect.contains(x, y) { return ztype; }
            }
        }
        ZombieType::Invalid
    }
    pub fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // 对应 C++ MouseUp：按钮悬停时切换页面/关闭
        if self.plant_button.map_or(false, |p| unsafe { (*p).is_over }) {
            self.set_page(AlmanacPage::Plants);
        } else if self.zombie_button.map_or(false, |p| unsafe { (*p).is_over }) {
            self.set_page(AlmanacPage::Zombies);
        } else if self.close_button.map_or(false, |p| unsafe { (*p).is_over }) {
            if let Some(app) = self.app {
                unsafe { (*app).kill_almanac_dialog(); }
            }
        } else if self.index_button.map_or(false, |p| unsafe { (*p).is_over }) {
            self.set_page(AlmanacPage::Index);
        }
    }
    pub fn mouse_down(&mut self, x: i32, y: i32, _click_count: i32) {
        let seed = self.seed_hit_test(x, y);
        if seed != SeedType::None { self.show_plant(seed); return; }
        let zombie = self.zombie_hit_test(x, y);
        if zombie != ZombieType::Invalid { self.show_zombie(zombie); return; }
    }
    pub fn get_zombie_type(index: i32) -> ZombieType { ZombieType::Invalid }
    pub fn show_plant(&mut self, t: SeedType) {
        self.selected_seed = t;
        self.set_page(AlmanacPage::Plants);
    }
    pub fn show_zombie(&mut self, t: ZombieType) {
        self.selected_zombie = t;
        self.set_page(AlmanacPage::Zombies);
    }
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

/// 记录玩家击败的僵尸类型（对应 C++ AlmanacPlayerDefeatedZombie）
/// 在 Zombie::DropLoot 中调用，用于图鉴的已击败标记
pub fn almanac_player_defeated_zombie(zombie_type: ZombieType) {
    unsafe {
        let idx = zombie_type as usize;
        if idx < G_ZOMBIE_DEFEATED.len() {
            G_ZOMBIE_DEFEATED[idx] = true;
        }
    }
}
