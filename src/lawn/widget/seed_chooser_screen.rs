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

    pub fn pick_from_weighted_array_using_special_rand_seed(arr: &[TodWeightedArray], count: i32, rng: &mut MTRand) -> usize {
        // 对应 C++ PickFromWeightedArrayUsingSpecialRandSeed
        let mut a_total_weight = 0i32;
        for i in 0..count {
            a_total_weight += arr[i as usize].weight;
        }
        let a_rnd_result = rng.next() % a_total_weight.max(1) as u32;
        let mut a_weight = 0i32;
        for j in 0..count {
            a_weight += arr[j as usize].weight;
            if a_weight as u32 > a_rnd_result {
                return arr[j as usize].item;
            }
        }
        // C++ 中 DBG_ASSERT(false) 后 unreachable
        0
    }
    pub fn crazy_dave_pick_seeds(&mut self) {
        // 对应 C++ CrazyDavePickSeeds：按权重挑选前 3 个种子放入种子槽
        let mut a_seed_array: Vec<TodWeightedArray> = Vec::with_capacity(crate::lawn::game_enums::NUM_SEED_TYPES);
        for seed_val in 0..crate::lawn::game_enums::NUM_SEEDS_IN_CHOOSER {
            let a_seed_type = unsafe { std::mem::transmute::<i32, SeedType>(seed_val) };
            let weight = if self.app.map_or(true, |app| unsafe {
                !(*app).has_seed_type(a_seed_type)
                    || self.seed_not_recommended_to_pick(a_seed_type) != 0
                    || self.seed_not_allowed_to_pick(a_seed_type)
                    || crate::lawn::plant::Plant::is_upgrade(a_seed_type)
                    || a_seed_type == SeedType::Imitater
                    || a_seed_type == SeedType::Umbrella
                    || a_seed_type == SeedType::Blover
            }) {
                0
            } else {
                1
            };
            a_seed_array.push(TodWeightedArray { item: a_seed_type as usize, weight });
        }

        if let Some(app) = self.app {
            unsafe {
                if let Some(board) = (*app).board {
                    // C++ 中 mZombieAllowed[ZOMBIE_BUNGEE/ZOMBIE_CATAPULT] 允许时解锁 Umbrella
                    let bungee_allowed = (*board).m_zombie_allowed[ZombieType::Bungee as usize];
                    let catapult_allowed = (*board).m_zombie_allowed[ZombieType::Catapult as usize];
                    let balloon_allowed = (*board).m_zombie_allowed[ZombieType::Balloon as usize];
                    if bungee_allowed || catapult_allowed {
                        a_seed_array[SeedType::Umbrella as usize].weight = 1;
                    }
                    if balloon_allowed || (*board).stage_has_fog() {
                        a_seed_array[SeedType::Blover as usize].weight = 1;
                    }
                    if (*board).stage_has_roof() {
                        a_seed_array[SeedType::Torchwood as usize].weight = 0;
                    }
                }

                let mut a_level_rng = crate::framework::mt_rand::MTRand::new();
                a_level_rng.srand((self.board.map_or(0, |b| unsafe { (*b).get_level_rand_seed() })) as u32);
                for i in 0..3 {
                    let a_picked_seed = Self::pick_from_weighted_array_using_special_rand_seed(&a_seed_array, crate::lawn::game_enums::NUM_SEEDS_IN_CHOOSER, &mut a_level_rng);
                    a_seed_array[a_picked_seed].weight = 0;

                    let a_pos_x = self.board.map_or(0, |b| unsafe { (*b).get_seed_packet_position_x(i) });
                    let mut a_chosen_seed = ChosenSeed::new();
                    a_chosen_seed.seed_type = unsafe { std::mem::transmute::<i32, SeedType>(a_picked_seed as i32) };
                    a_chosen_seed.x = a_pos_x;
                    a_chosen_seed.y = 8;
                    a_chosen_seed.start_x = a_pos_x;
                    a_chosen_seed.start_y = 8;
                    a_chosen_seed.end_x = a_pos_x;
                    a_chosen_seed.end_y = 8;
                    a_chosen_seed.seed_state = ChosenSeedState::InBank;
                    a_chosen_seed.seed_index_in_bank = i;
                    a_chosen_seed.crazy_dave_picked = true;
                    self.chosen_seeds.push(a_chosen_seed);
                    self.seeds_in_bank += 1;
                }
            }
        }
    }
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
    pub fn seed_not_recommended_to_pick(&self, t: SeedType) -> u32 {
        // 对应 C++ SeedNotRecommendedToPick
        let a_rec_flags = self.board.map_or(0, |b| unsafe { (*b).seed_not_recommended_for_level(t) });
        // [TRANSLATION_NOTE]: C++ 中若 NOCTURNAL 位已设且已选 InstantCoffee 则清除该位；
        // NotRecommend 位号 Rust 侧未定义，暂保留原标志
        a_rec_flags
    }
    pub fn seed_not_allowed_to_pick(&self, t: SeedType) -> bool {
        // 对应 C++ SeedNotAllowedToPick：坚守模式禁用产阳植物
        let is_last_stand = self.app.map_or(false, |app| unsafe { (*app).game_mode == GameMode::ChallengeLastStand });
        is_last_stand
            && matches!(
                t,
                SeedType::Sunflower | SeedType::Sunshroom | SeedType::Twinsunflower
                    | SeedType::Seashroom | SeedType::Puffshroom
            )
    }
    pub fn seed_not_allowed_during_trial(&self, t: SeedType) -> bool {
        // 对应 C++ SeedNotAllowedDuringTrial：试用锁定禁用 Squash/Threepeater
        self.app.map_or(false, |app| unsafe { (*app).is_trial_stage_locked() })
            && (t == SeedType::Squash || t == SeedType::Threepeater)
    }
    pub fn draw(&self, _g: &mut Graphics) {
        // 对应 C++ Draw：绘制种子选择器背景/按钮/飞入种子
        // [TRANSLATION_NOTE]: C++ 中绘制 IMAGE_SEEDCHOOSER_BACKGROUND 等图片资源；
        // Rust 侧图片资源未接入，暂略
    }
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
        // 对应 C++ SeedChooserScreen::ShowToolTip L863-L948
        // [TRANSLATION_NOTE]: C++ 最外层条件 !mWidgetManager->mMouseIn || !mApp->mActive ||
        // mApp->GetDialogCount() > 0 —— widget_manager 未接入，以 choose_state 判断为主保留结构；
        // mImitaterButton->IsMouseOver() 无对应谓词，该分支跳过（C++ L871-L879）。
        if self.choose_state == SeedChooserState::ViewLawn {
            self.remove_tool_tip();
        } else if self.seeds_in_flight <= 0 {
            // C++: SeedType aSeedType = SeedHitTest(mLastMouseX, mLastMouseY);
            let a_seed_type = self.seed_hit_test(self.last_mouse_x, self.last_mouse_y);
            if a_seed_type == SeedType::None {
                self.remove_tool_tip();
            } else if a_seed_type as i32 != self.tool_tip_seed {
                self.remove_tool_tip();
                // C++: ChosenSeed& aChosenSeed = mChosenSeeds[aSeedType];
                let a_chosen_seed = self.chosen_seeds.iter().find(|s| s.seed_type == a_seed_type);
                let a_rec_flags = self.seed_not_recommended_to_pick(a_seed_type);
                let a_not_allowed = self.seed_not_allowed_to_pick(a_seed_type);
                let a_not_during_trial = self.seed_not_allowed_during_trial(a_seed_type);
                let a_state_in_bank = a_chosen_seed.map_or(false, |s| s.seed_state == ChosenSeedState::InBank);
                let a_crazy_dave_picked = a_chosen_seed.map_or(false, |s| s.crazy_dave_picked);
                let a_imitater_type = a_chosen_seed.map_or(SeedType::None, |s| s.imitater_type);
                let a_seed_index_in_bank = a_chosen_seed.map_or(0, |s| s.seed_index_in_bank);

                let tip_ptr = match self.tool_tip {
                    Some(p) => p,
                    None => return,
                };
                unsafe {
                    let tip = &mut *tip_ptr;
                    if a_not_allowed {
                        tip.set_warning_text("[NOT_ALLOWED_ON_THIS_LEVEL]");
                    } else if a_not_during_trial {
                        tip.set_warning_text("[FULL_VERSION_ONLY]");
                    } else if a_state_in_bank && a_crazy_dave_picked {
                        tip.set_warning_text("[CRAZY_DAVE_WANTS]");
                    } else if a_rec_flags != 0 {
                        if crate::lawn::zombie::test_bit(a_rec_flags, crate::lawn::game_enums::NotRecommend::Nocturnal as u32) {
                            tip.set_warning_text("[NOCTURNAL_WARNING]");
                        } else {
                            tip.set_warning_text("[NOT_RECOMMENDED_FOR_LEVEL]");
                        }
                    } else {
                        tip.set_warning_text("");
                    }

                    if a_seed_type == SeedType::Imitater {
                        // C++: SetTitle(GetNameString(aSeedType, mImitaterType)); SetLabel(GetToolTip(mImitaterType));
                        tip.set_title(&crate::lawn::plant::Plant::get_name_string(a_seed_type, a_imitater_type));
                        tip.set_label(&crate::lawn::plant::Plant::get_tool_tip(a_imitater_type));
                    } else {
                        tip.set_title(&crate::lawn::plant::Plant::get_name_string(a_seed_type, SeedType::None));
                        tip.set_label(&crate::lawn::plant::Plant::get_tool_tip(a_seed_type));
                    }

                    let mut a_seed_x = 0;
                    let mut a_seed_y = 0;
                    if a_state_in_bank {
                        self.get_seed_position_in_bank(a_seed_index_in_bank, &mut a_seed_x, &mut a_seed_y);
                    } else {
                        self.get_seed_position_in_chooser(a_seed_type as i32, &mut a_seed_x, &mut a_seed_y);
                    }
                    // C++: std::clamp((SEED_PACKET_WIDTH - mToolTip->mWidth) / 2 + aSeedX, 0, BOARD_WIDTH - mToolTip->mWidth)
                    tip.m_x = ((SEED_PACKET_WIDTH - tip.m_width) / 2 + a_seed_x).clamp(0, BOARD_WIDTH - tip.m_width);
                    tip.m_y = a_seed_y + 70;
                    tip.m_visible = true;
                    self.tool_tip_seed = a_seed_type as i32;
                }
            }
        }
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
