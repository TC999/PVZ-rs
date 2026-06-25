// PvZ Portable Rust 翻译 — SeedChooserScreen（选卡界面）
// 对应 C++ src/Lawn/Widget/SeedChooserScreen.h / SeedChooserScreen.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::KeyCode;
use crate::lawn::game_enums::*;
use crate::lawn::widget::game_button::GameButton;
use crate::todlib::tod_common::TodWeightedArray;
use crate::framework::mt_rand::MTRand;

/// 已选种子（对应 C++ ChosenSeed）
pub struct ChosenSeed {
    pub x: i32,
    pub y: i32,
    pub time_start_motion: i32,
    pub time_end_motion: i32,
    pub start_x: i32,
    pub start_y: i32,
    pub end_x: i32,
    pub end_y: i32,
    pub seed_type: SeedType,
    pub seed_state: ChosenSeedState,
    pub seed_index_in_bank: i32,
    pub refreshing: bool,
    pub refresh_counter: i32,
    pub imitater_type: SeedType,
    pub crazy_dave_picked: bool,
}

impl ChosenSeed {
    pub fn new() -> Self {
        ChosenSeed {
            x: 0, y: 0,
            time_start_motion: 0, time_end_motion: 0,
            start_x: 0, start_y: 0,
            end_x: 0, end_y: 0,
            seed_type: SeedType::None,
            seed_state: ChosenSeedState::Unselected,
            seed_index_in_bank: 0,
            refreshing: false,
            refresh_counter: 0,
            imitater_type: SeedType::None,
            crazy_dave_picked: false,
        }
    }
}

/// 选卡界面（对应 C++ SeedChooserScreen）
pub struct SeedChooserScreen {
    pub start_button: Option<*mut GameButton>,
    pub random_button: Option<*mut GameButton>,
    pub view_lawn_button: Option<*mut GameButton>,
    pub store_button: Option<*mut GameButton>,
    pub almanac_button: Option<*mut GameButton>,
    pub menu_button: Option<*mut GameButton>,
    pub imitater_button: Option<*mut GameButton>,
    pub chosen_seeds: Vec<ChosenSeed>,  // C++ 固定数组 [NUM_SEED_TYPES]
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub board: Option<*mut crate::lawn::board::Board>,
    pub num_seeds_to_choose: i32,
    pub seed_chooser_age: i32,
    pub seeds_in_flight: i32,
    pub seeds_in_bank: i32,
    pub tool_tip: Option<*mut crate::lawn::tool_tip_widget::ToolTipWidget>,
    pub tool_tip_seed: i32,
    pub last_mouse_x: i32,
    pub last_mouse_y: i32,
    pub choose_state: SeedChooserState,
    pub view_lawn_time: i32,
}

impl SeedChooserScreen {
    pub fn new() -> Self {
        SeedChooserScreen {
            start_button: None, random_button: None,
            view_lawn_button: None, store_button: None,
            almanac_button: None, menu_button: None,
            imitater_button: None,
            chosen_seeds: Vec::new(),
            app: None, board: None,
            num_seeds_to_choose: 0, seed_chooser_age: 0,
            seeds_in_flight: 0, seeds_in_bank: 0,
            tool_tip: None, tool_tip_seed: 0,
            last_mouse_x: 0, last_mouse_y: 0,
            choose_state: SeedChooserState::Normal,
            view_lawn_time: 0,
        }
    }

    pub fn pick_from_weighted_array_using_special_rand_seed(_arr: &[TodWeightedArray], _count: i32, _rng: &mut MTRand) -> usize { 0 /* TODO */ }
    pub fn crazy_dave_pick_seeds(&mut self) { /* TODO */ }
    pub fn has_7_rows(&self) -> bool { false /* TODO */ }
    pub fn get_seed_position_in_chooser(&self, _idx: i32, _x: &mut i32, _y: &mut i32) { /* TODO */ }
    pub fn get_seed_position_in_bank(&self, _idx: i32, _x: &mut i32, _y: &mut i32) { /* TODO */ }
    pub fn seed_not_recommended_to_pick(&self, _t: SeedType) -> u32 { 0 /* TODO */ }
    pub fn seed_not_allowed_to_pick(&self, _t: SeedType) -> bool { false /* TODO */ }
    pub fn seed_not_allowed_during_trial(&self, _t: SeedType) -> bool { false /* TODO */ }
    pub fn draw(&self, _g: &mut Graphics) { /* TODO */ }
    pub fn update_view_lawn(&mut self) { /* TODO */ }
    pub fn land_flying_seed(&mut self, _seed: &mut ChosenSeed) { /* TODO */ }
    pub fn update_cursor(&mut self) { /* TODO */ }
    pub fn update(&mut self) { /* TODO */ }
    pub fn display_repick_warning_dialog(&self, _msg: &str) -> bool { false /* TODO */ }
    pub fn flyers_are_coming(&self) -> bool { false /* TODO */ }
    pub fn fly_protection_currently_planted(&self) -> bool { false /* TODO */ }
    pub fn check_seed_upgrade(&self, _to: SeedType, _from: SeedType) -> bool { false /* TODO */ }
    pub fn on_start_button(&mut self) { /* TODO */ }
    pub fn pick_random_seeds(&mut self) { /* TODO */ }
    pub fn button_depress(&mut self, _id: i32) { /* TODO */ }
    pub fn seed_hit_test(&self, _x: i32, _y: i32) -> SeedType { SeedType::None /* TODO */ }
    pub fn find_seed_in_bank(&self, _idx: i32) -> SeedType { SeedType::None /* TODO */ }
    pub fn enable_start_button(&self, _enabled: bool) { /* TODO */ }
    pub fn clicked_seed_in_bank(&mut self, _seed: &mut ChosenSeed) { /* TODO */ }
    pub fn clicked_seed_in_chooser(&mut self, _seed: &mut ChosenSeed) { /* TODO */ }
    pub fn show_tool_tip(&mut self) { /* TODO */ }
    pub fn remove_tool_tip(&mut self) { /* TODO */ }
    pub fn cancel_lawn_view(&mut self) { /* TODO */ }
    pub fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) { /* TODO */ }
    pub fn update_imitater_button(&mut self) { /* TODO */ }
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) { /* TODO */ }
    pub fn picked_plant_type(&self, _t: SeedType) -> bool { false /* TODO */ }
    pub fn close_seed_chooser(&mut self) { /* TODO */ }
    pub fn key_down(&mut self, _key: KeyCode) { /* TODO */ }
    pub fn key_char(&mut self, _c: char) { /* TODO */ }
    pub fn update_after_purchase(&mut self) { /* TODO */ }
}

impl Default for SeedChooserScreen {
    fn default() -> Self { SeedChooserScreen::new() }
}
