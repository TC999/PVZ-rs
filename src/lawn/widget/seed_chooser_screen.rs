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
            seed_state: ChosenSeedState::Hidden,
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
    pub fn has_7_rows(&self) -> bool {
        if let Some(app) = self.app { unsafe {
            (*app).has_finished_adventure() || (*app).player_info.as_ref().unwrap().m_purchases[0] != 0
        } } else { false }
    }
    pub fn get_seed_position_in_chooser(&self, idx: i32, x: &mut i32, y: &mut i32) {
        let row = idx / 8;
        let col = idx % 8;
        *x = col * 53 + 22;
        *y = if self.has_7_rows() { row * 70 + 123 } else { row * 73 + 128 };
    }
    pub fn get_seed_position_in_bank(&self, idx: i32, x: &mut i32, y: &mut i32) {
        if let Some(board) = self.board { unsafe {
            *x = 0 /* seed_bank.x */ + (*board).get_seed_packet_position_x(idx);
            *y = 0 /* seed_bank.y */ + 8;
        } }
    }
    pub fn seed_not_recommended_to_pick(&self, t: SeedType) -> u32 { 0 }
    pub fn seed_not_allowed_to_pick(&self, _t: SeedType) -> bool { false }
    pub fn seed_not_allowed_during_trial(&self, _t: SeedType) -> bool { false }
    pub fn draw(&self, _g: &mut Graphics) { /* TODO */ }
    pub fn update_view_lawn(&mut self) {
        if self.choose_state != SeedChooserState::ViewLawn { return; }
        self.view_lawn_time += 1;
        if self.view_lawn_time >= 251 { self.view_lawn_time = 250; }
    }
    pub fn land_flying_seed(&mut self, seed: &mut ChosenSeed) {
        if seed.seed_state == ChosenSeedState::FlyingToBank {
            seed.x = seed.end_x; seed.y = seed.end_y;
            seed.seed_state = ChosenSeedState::InBank;
            self.seeds_in_flight -= 1;
        } else if seed.seed_state == ChosenSeedState::FlyingToChooser {
            seed.x = seed.end_x; seed.y = seed.end_y;
            seed.seed_state = ChosenSeedState::InChooser;
            self.seeds_in_flight -= 1;
        }
    }
    pub fn update_cursor(&mut self) {
        // 简化版：不处理光标变化
    }
    pub fn update(&mut self) {
        self.seed_chooser_age += 1;
        if let Some(app) = self.app { unsafe {
            self.last_mouse_x = 0 /* widget_manager */;
            self.last_mouse_y = 0 /* widget_manager */;
        } }
        self.show_tool_tip();
        if self.choose_state == SeedChooserState::ViewLawn { self.update_view_lawn(); }
    }
    pub fn display_repick_warning_dialog(&self, _msg: &str) -> bool { true }
    pub fn flyers_are_coming(&self) -> bool {
        if let Some(board) = self.board { unsafe {
            for wave in 0..(*board).m_num_waves {
                for idx in 0..crate::lawn::board::MAX_ZOMBIES_IN_WAVE {
                    let ztype = (*board).m_zombies_in_wave[wave as usize][idx as usize];
                    if ztype == ZombieType::Invalid { break; }
                    if ztype == ZombieType::Balloon { return true; }
                }
            }
        } }
        false
    }
    pub fn fly_protection_currently_planted(&self) -> bool {
        if let Some(board) = self.board { unsafe {
            for plant in &(*board).plants {
                if !plant.dead && (plant.seed_type == SeedType::Cattail || plant.seed_type == SeedType::Cactus) {
                    return true;
                }
            }
        } }
        false
    }
    pub fn check_seed_upgrade(&self, to: SeedType, from: SeedType) -> bool {
        // 对应 C++ SeedChooserScreen::CheckSeedUpgrade
        let a_survival = self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() });
        if a_survival || !self.picked_plant_type(to) || self.picked_plant_type(from) {
            return true;
        }

        // [TRANSLATION_NOTE]: C++ 用 Plant::GetNameString 构建 [SEED_CHOOSER_UPGRADE_WARNING] 文本后弹窗；
        // Rust 侧字符串翻译与弹窗未完全移植，直接调用 display_repick_warning_dialog
        self.display_repick_warning_dialog("")
    }
    pub fn on_start_button(&mut self) {
        // OnStartButton — 简化版
        self.close_seed_chooser();
    }
    pub fn pick_random_seeds(&mut self) {
        // PickRandomSeeds — 简化版
        self.close_seed_chooser();
    }
    pub fn button_depress(&mut self, id: i32) {
        if self.seeds_in_flight > 0 || self.choose_state == SeedChooserState::ViewLawn { return; }
        if id == 102 { /* ViewLawn */ self.choose_state = SeedChooserState::ViewLawn; }
        else if id == 100 { /* Start */ self.on_start_button(); }
        else if id == 101 { /* Random */ self.pick_random_seeds(); }
        else if id == 103 { /* Almanac */ }
        else if id == 104 { /* Menu */ }
        else if id == 105 { /* Store */ }
        else if id == 106 { /* Imitater */ }
    }
    pub fn seed_hit_test(&self, x: i32, y: i32) -> SeedType {
        for seed in &self.chosen_seeds {
            if seed.seed_state == ChosenSeedState::Hidden { continue; }
            let rect = crate::framework::rect::Rect::new(seed.x, seed.y, 50, 70);
            if rect.contains(x, y) { return seed.seed_type; }
        }
        SeedType::None
    }
    pub fn find_seed_in_bank(&self, idx: i32) -> SeedType {
        for seed in &self.chosen_seeds {
            if seed.seed_state == ChosenSeedState::InBank && seed.seed_index_in_bank == idx {
                return seed.seed_type;
            }
        }
        SeedType::None
    }
    pub fn enable_start_button(&self, _enabled: bool) {
        // EnableStartButton — 简化版
    }
    pub fn clicked_seed_in_bank(&mut self, seed: &mut ChosenSeed) {
        // ClickedSeedInBank — 简化版
        seed.seed_state = ChosenSeedState::FlyingToChooser;
        self.seeds_in_flight += 1;
    }
    pub fn clicked_seed_in_chooser(&mut self, seed: &mut ChosenSeed) {
        // ClickedSeedInChooser — 简化版
        if self.seeds_in_bank >= 10 { return; }
        seed.seed_state = ChosenSeedState::FlyingToBank;
        seed.seed_index_in_bank = self.seeds_in_bank;
        self.seeds_in_flight += 1;
        self.seeds_in_bank += 1;
        self.remove_tool_tip();
    }
    pub fn show_tool_tip(&mut self) {
        // ShowToolTip — 简化版
    }
    pub fn cancel_lawn_view(&mut self) {
        if self.choose_state == SeedChooserState::ViewLawn && self.view_lawn_time > 100 && self.view_lawn_time <= 250 {
            self.view_lawn_time = 251;
        }
    }
    pub fn mouse_up(&mut self, x: i32, y: i32, click_count: i32) {
        if click_count == 1 {
            if let Some(btn) = self.menu_button { unsafe { if true /* is_mouse_over */ { self.button_depress(104); } } }
            else if let Some(btn) = self.start_button { unsafe { if true /* is_mouse_over */ { self.button_depress(100); } } }
        }
    }
    pub fn remove_tool_tip(&mut self) { self.tool_tip = None; }
    pub fn close_seed_chooser(&mut self) {
        if let Some(app) = self.app { unsafe { /* kill_seed_chooser */ self.choose_state = SeedChooserState::Normal; } }
    }
    pub fn picked_plant_type(&self, t: SeedType) -> bool {
        self.chosen_seeds.iter().any(|s| s.seed_type == t && s.seed_state != ChosenSeedState::InBank)
    }
}

impl Default for SeedChooserScreen {
    fn default() -> Self { SeedChooserScreen::new() }
}
