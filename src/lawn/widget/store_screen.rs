// PvZ Portable Rust 翻译 — StoreScreen（商店界面）
// 对应 C++ src/Lawn/Widget/StoreScreen.h / StoreScreen.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::widget::widget::Widget;
use crate::lawn::game_enums::*;
use crate::lawn::system::player_info::PottedPlant;

/// 页面插槽最大数（对应 C++ #define MAX_PAGE_SPOTS 8）
pub const MAX_PAGE_SPOTS: usize = 8;
/// 最大购买数（对应 C++ #define MAX_PURCHASES 80）
pub const MAX_PURCHASES: usize = 80;

/// 商店界面（对应 C++ StoreScreen）
pub struct StoreScreen {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub back_button: Option<*mut DialogButton>,
    pub prev_button: Option<*mut DialogButton>,
    pub next_button: Option<*mut DialogButton>,
    pub overlay_widget: Option<*mut Widget>,
    pub store_time: i32,
    pub bubble_text: String,
    pub bubble_count_down: i32,
    pub bubble_click_to_continue: bool,
    pub ambient_speech_count_down: i32,
    pub previous_ambient_speech_index: i32,
    pub page: StorePages,
    pub mouse_over_item: StoreItem,
    pub hatch_timer: i32,
    pub hatch_open: bool,
    pub shake_x: i32,
    pub shake_y: i32,
    pub start_dialog: i32,
    pub easy_buying_cheat: bool,
    pub wait_for_dialog: bool,
    pub potted_plant_specs: PottedPlant,
    pub coins: Vec<()>, // DataArray<Coin> 简化，待翻译
    pub drawn_once: bool,
    pub go_to_tree_now: bool,
    pub purchased_full_version: bool,
    pub trial_locked_when_store_opened: bool,
}

impl StoreScreen {
    pub fn new() -> Self {
        StoreScreen {
            app: None,
            back_button: None,
            prev_button: None,
            next_button: None,
            overlay_widget: None,
            store_time: 0,
            bubble_text: String::new(),
            bubble_count_down: 0,
            bubble_click_to_continue: false,
            ambient_speech_count_down: 0,
            previous_ambient_speech_index: 0,
            page: StorePages::SlotUpgrades,
            mouse_over_item: StoreItem::PlantGatlingpea,
            hatch_timer: 0,
            hatch_open: false,
            shake_x: 0,
            shake_y: 0,
            start_dialog: 0,
            easy_buying_cheat: false,
            wait_for_dialog: false,
            potted_plant_specs: PottedPlant::new(),
            coins: Vec::new(),
            drawn_once: false,
            go_to_tree_now: false,
            purchased_full_version: false,
            trial_locked_when_store_opened: false,
        }
    }

    pub fn get_store_item_type(&self, spot_index: i32) -> StoreItem {
        if let Some(app) = self.app { unsafe {
            if (self.page as i32) < (StorePages::NumPages as i32) && (spot_index as usize) < MAX_PAGE_SPOTS {
                let page = self.page;
                if page == StorePages::SlotUpgrades && spot_index == 6 && (*app).is_trial_stage_locked() {
                    return StoreItem::Invalid;
                }
                return StoreItem::Invalid;
            }
        } }
        StoreItem::Invalid
    }
    pub fn draw_item_icon(&self, _g: &mut Graphics, _pos: i32, _item: StoreItem, _highlight: bool) {
        // 依赖图片资源，暂用占位
    }
    pub fn draw_item(&self, _g: &mut Graphics, _pos: i32, _item: StoreItem) {
        // 依赖图片资源，暂用占位
    }
    pub fn draw(&self, _g: &mut Graphics) {
        // 依赖图片资源，暂用占位
    }
    pub fn draw_overlay(&self, _g: &mut Graphics) {
        // 依赖图片资源，暂用占位
    }
    pub fn is_full_version_only(&self, item: StoreItem) -> bool {
        if let Some(app) = self.app { unsafe {
            if !(*app).is_trial_stage_locked() { return false; }
            if item == StoreItem::PacketUpgrade && (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::PacketUpgrade as usize] >= 2 { return true; }
            item == StoreItem::PlantTwinsunflower
        } } else { false }
    }
    pub fn is_potted_plant(item: StoreItem) -> bool {
        matches!(item, StoreItem::PottedMarigold1 | StoreItem::PottedMarigold2 | StoreItem::PottedMarigold3)
    }
    pub fn is_coming_soon(&self, item: StoreItem) -> bool {
        if self.is_full_version_only(item) { return true; }
        if let Some(app) = self.app { unsafe {
            if item == StoreItem::WheelBarrow { return (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::MushroomGarden as usize] == 0 && (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::AquariumGarden as usize] == 0; }
            if Self::is_potted_plant(item) { return !(*app).has_finished_adventure(); }
        } }
        false
    }
    pub fn is_item_sold_out(&self, item: StoreItem) -> bool {
        if let Some(app) = self.app { unsafe {
            if item == StoreItem::Invalid { return false; }
            if item == StoreItem::PacketUpgrade { return (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::PacketUpgrade as usize] >= 4; }
            if item == StoreItem::Fertilizer || item == StoreItem::BugSpray { return (*app).player_info.as_ref().unwrap().m_purchases[item as usize] > 15; }
            if item == StoreItem::TreeFood { return (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::TreeFood as usize] >= 10; }
            if item == StoreItem::BonusLawnMower { return (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::BonusLawnMower as usize] >= 2; }
            if Self::is_potted_plant(item) { return true; }
            (*app).player_info.as_ref().unwrap().m_purchases[item as usize] != 0
        } } else { false }
    }
    pub fn is_item_unavailable(&self, _item: StoreItem) -> bool { false }
    pub fn get_store_position(spot_index: i32, pos_x: &mut i32, pos_y: &mut i32) {
        let row = spot_index / 4;
        let col = spot_index % 4;
        *pos_x = 155 + col * 140;
        *pos_y = 160 + row * 100;
    }
    pub fn can_interact_with_buttons(&self) -> bool {
        self.store_time >= 120 && !self.bubble_click_to_continue && self.hatch_timer <= 0 && !self.wait_for_dialog
    }
    pub fn is_page_shown(&self, page: StorePages) -> bool {
        if let Some(app) = self.app { unsafe {
            if (*app).is_trial_stage_locked() { return page == StorePages::SlotUpgrades; }
            if (*app).has_finished_adventure() { return true; }
            if page == StorePages::PlantUpgrades { return (*app).player_info.as_ref().unwrap().m_level >= 42; }
            if page == StorePages::Zen1 { return (*app).player_info.as_ref().unwrap().m_level >= 45; }
            return page != StorePages::Zen2;
        } } else { false }
    }
    pub fn get_item_cost(item: StoreItem) -> i32 {
        match item {
            StoreItem::PlantGatlingpea => 500,
            StoreItem::PlantTwinsunflower => 500,
            StoreItem::PlantGloomshroom => 750,
            StoreItem::PlantCattail => 1000,
            StoreItem::PlantWintermelon => 1000,
            StoreItem::PlantGoldMagnet => 300,
            StoreItem::PlantSpikerock => 750,
            StoreItem::PlantCobcannon => 2000,
            StoreItem::PlantImitater => 3000,
            StoreItem::PottedMarigold1 | StoreItem::PottedMarigold2 | StoreItem::PottedMarigold3 => 250,
            StoreItem::GoldWateringcan => 1000,
            StoreItem::Fertilizer => 75,
            StoreItem::BugSpray => 100,
            StoreItem::Phonograph => 1500,
            StoreItem::GardeningGlove => 100,
            StoreItem::MushroomGarden => 3000,
            StoreItem::WheelBarrow => 20,
            StoreItem::StinkyTheSnail => 300,
            StoreItem::BonusLawnMower => 200,
            StoreItem::PoolCleaner => 100,
            StoreItem::RoofCleaner => 100,
            _ => 0,
        }
    }
    pub fn can_afford_item(&self, item: StoreItem) -> bool {
        if let Some(app) = self.app { unsafe {
            let cost = Self::get_item_cost(item);
            cost <= 0 || (*app).player_info.as_ref().unwrap().m_coins >= cost
        } } else { false }
    }
    pub fn enable_buttons(&self, _enable: bool) { /* TODO */ }
    pub fn setup_for_intro(&mut self, _dialog_index: i32) { /* TODO */ }
}

impl Default for StoreScreen {
    fn default() -> Self { StoreScreen::new() }
}

/// 商店界面叠加 Widget（对应 C++ StoreScreenOverlay）
pub struct StoreScreenOverlay {
    pub parent: Option<*mut StoreScreen>,
}

impl StoreScreenOverlay {
    pub fn new() -> Self { StoreScreenOverlay { parent: None } }
    pub fn draw(&self, _g: &mut Graphics) { /* TODO */ }
}
