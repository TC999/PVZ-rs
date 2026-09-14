// PvZ Portable Rust 翻译 — SeedChooserScreen（选卡界面）
// 对应 C++ src/Lawn/Widget/SeedChooserScreen.h / SeedChooserScreen.cpp

#![allow(dead_code)]

use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::{KEYCODE_ESCAPE, KEYCODE_RETURN, KEYCODE_SPACE, KeyCode};
use crate::framework::color::Color;
use crate::framework::widget::dialog::{BUTTONS_YES_NO, ID_YES};
use crate::lawn::game_enums::*;
use crate::lawn::widget::imitater_dialog::ImitaterDialog;
use crate::lawn::widget::game_button::GameButton;
use crate::todlib::tod_common::TodWeightedArray;
use crate::framework::mt_rand::MTRand;
use crate::lawn::system::music::MusicTune;

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
    /// 对应 C++ LawnApp 对话框系统中的 ImitaterDialog（Rust 对话框驱动持有）
    pub imitater_dialog: Option<*mut ImitaterDialog>,
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
            tool_tip: None, tool_tip_seed: 0, imitater_dialog: None,
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
    pub fn draw(&self, g: &mut Graphics) {
        // C++: ImitaterDialog 由 LawnApp 对话框系统绘制（Rust 对话框驱动转发）
        if let Some(d) = self.imitater_dialog {
            unsafe { (*d).draw(g); }
            return;
        }
        // 对应 C++ Draw（SeedChooserScreen.cpp:345）
        let Some(app) = self.app else { return };
        unsafe {
            // C++: if (mApp->GetDialog(DIALOG_STORE) || mApp->GetDialog(DIALOG_ALMANAC)) return;
            if (*app).base.dialog_map.contains_key(&(Dialogs::Store as i32))
                || (*app).base.dialog_map.contains_key(&(Dialogs::Almanac as i32))
            {
                return;
            }

            g.set_linear_blend(true);
            // C++: if (!mBoard->ChooseSeedsOnCurrentLevel() || (mBoard->mCutScene && mBoard->mCutScene->IsBeforePreloading())) return;
            let a_board = match self.board {
                Some(b) => &mut *b,
                None => return,
            };
            if !a_board.choose_seeds_on_current_level() {
                return;
            }
            if let Some(cs) = a_board.m_cut_scene {
                if (*cs).is_before_preloading() {
                    return;
                }
            }

            // C++: g->DrawImage(IMAGE_SEEDCHOOSER_BACKGROUND, 0, 87)
            let a_background = crate::lawn::board::get_overlay_image(&*app, "IMAGE_SEEDCHOOSER_BACKGROUND");
            if !a_background.is_null() {
                g.draw_image_xy(&*a_background, 0, 87);
            }
            if (*app).has_seed_type(SeedType::Imitater) {
                // C++: g->DrawImage(IMAGE_SEEDCHOOSER_IMITATERADDON, 459, 503)
                let a_imitater_addon = crate::lawn::board::get_overlay_image(&*app, "IMAGE_SEEDCHOOSER_IMITATERADDON");
                if !a_imitater_addon.is_null() {
                    g.draw_image_xy(&*a_imitater_addon, 459, 503);
                }
            }
            // C++: PvzpDrawString("[CHOOSE_YOUR_PLANTS]", 229, 110, FONT_DWARVENTODCRAFT18YELLOW, White, DS_ALIGN_CENTER)
            let mut a_font = crate::framework::graphics::font::Font::new("Dwarventodcraft", 18);
            a_font.ascent = 13;
            a_font.font_height = 18;
            g.set_font(&mut a_font as *mut crate::framework::graphics::font::Font);
            g.set_color(&crate::framework::color::Color::new(255, 255, 255, 255));
            let a_text_width = a_font.string_width("[CHOOSE_YOUR_PLANTS]");
            g.draw_string("[CHOOSE_YOUR_PLANTS]", 229 - a_text_width / 2, 110);

            // C++: 遍历选择器格（Has7Rows ? 48 : 40）：已拥有且不在选择器内画淡色包（55），未拥有画剪影
            let a_num_seeds = if self.has_7_rows() { 48 } else { 40 };
            for a_seed_shadow in 0..a_num_seeds {
                let a_seed = unsafe { std::mem::transmute::<i32, SeedType>(a_seed_shadow) };
                let (mut x, mut y) = (0, 0);
                self.get_seed_position_in_chooser(a_seed_shadow, &mut x, &mut y);
                if a_seed == SeedType::Imitater {
                    continue;
                }
                if (*app).has_seed_type(a_seed) {
                    // C++: ChosenSeed& aChosenSeed = mChosenSeeds[aSeedShadow];
                    //      if (aChosenSeed.mSeedState != SEED_IN_CHOOSER) DrawSeedPacket(x, y, 55)
                    // [TRANSLATION_NOTE]: Rust chosen_seeds 为动态 Vec（C++ 固定数组构造时填满），
                    // 缺失槽视为无已选种子跳过
                    if let Some(a_chosen) = self.chosen_seeds.get(a_seed_shadow as usize) {
                        if a_chosen.seed_state != ChosenSeedState::InChooser {
                            crate::lawn::seed_packet::draw_seed_packet(g, x as f32, y as f32, a_seed, SeedType::None, 0.0, 55, true, false);
                        }
                    }
                } else {
                    let a_silhouette = crate::lawn::board::get_overlay_image(&*app, "IMAGE_SEEDPACKETSILHOUETTE");
                    if !a_silhouette.is_null() {
                        g.draw_image_xy(&*a_silhouette, x, y);
                    }
                }
            }

            // C++: 遍历 bank 槽：空槽画剪影（FindSeedInBank == SEED_NONE）
            let a_num_seeds_in_bank = a_board.get_num_seeds_in_bank();
            for an_index in 0..a_num_seeds_in_bank {
                if self.find_seed_in_bank(an_index) == SeedType::None {
                    let (mut x, mut y) = (0, 0);
                    self.get_seed_position_in_bank(an_index, &mut x, &mut y);
                    let a_silhouette = crate::lawn::board::get_overlay_image(&*app, "IMAGE_SEEDPACKETSILHOUETTE");
                    if !a_silhouette.is_null() {
                        g.draw_image_xy(&*a_silhouette, x, y);
                    }
                }
            }

            // C++: 遍历已选种子：灰化判定 + DrawSeedPacket（mX/mY 恒 0 简化，ViewLawn 动画未翻译）
            let a_seed_choosing = a_board.m_cut_scene.map_or(false, |cs| unsafe { (*cs).m_seed_choosing });
            for a_seed_index in 0..NUM_SEEDS_IN_CHOOSER {
                let a_seed = unsafe { std::mem::transmute::<i32, SeedType>(a_seed_index) };
                if !(*app).has_seed_type(a_seed) {
                    continue;
                }
                let a_chosen = match self.chosen_seeds.get(a_seed_index as usize) {
                    Some(c) => c,
                    None => continue,
                };
                let a_state = a_chosen.seed_state;
                if a_state == ChosenSeedState::FlyingToBank
                    || a_state == ChosenSeedState::FlyingToChooser
                    || a_state == ChosenSeedState::Hidden
                {
                    continue;
                }
                if a_state != ChosenSeedState::InChooser && !a_seed_choosing {
                    continue;
                }
                // C++: aGrayed = ((SeedNotRecommendedToPick || SeedNotAllowedToPick) && IN_CHOOSER) || SeedNotAllowedDuringTrial
                let a_grayed = ((self.seed_not_recommended_to_pick(a_seed) != 0 || self.seed_not_allowed_to_pick(a_seed))
                    && a_state == ChosenSeedState::InChooser)
                    || self.seed_not_allowed_during_trial(a_seed);
                let mut a_pos_x = a_chosen.x;
                let mut a_pos_y = a_chosen.y;
                if a_state == ChosenSeedState::InBank {
                    // C++: aPosX -= mX; aPosY -= mY（mX/mY 随 ViewLawn 动画变化，Rust 恒 0）
                    a_pos_x -= 0;
                    a_pos_y -= 0;
                }
                crate::lawn::seed_packet::draw_seed_packet(
                    g, a_pos_x as f32, a_pos_y as f32, a_chosen.seed_type, a_chosen.imitater_type,
                    0.0, if a_grayed { 115 } else { 255 }, true, false,
                );
            }

            // C++: mImitaterButton->Draw(g)
            if let Some(btn) = self.imitater_button { unsafe { (&mut *btn).draw(g); } }
            // C++: 遍历飞行中的种子
            for a_seed_index in 0..NUM_SEEDS_IN_CHOOSER {
                let a_seed = unsafe { std::mem::transmute::<i32, SeedType>(a_seed_index) };
                if !(*app).has_seed_type(a_seed) {
                    continue;
                }
                if let Some(a_chosen) = self.chosen_seeds.get(a_seed_index as usize) {
                    let a_state = a_chosen.seed_state;
                    if a_state == ChosenSeedState::FlyingToBank || a_state == ChosenSeedState::FlyingToChooser {
                        crate::lawn::seed_packet::draw_seed_packet(
                            g, a_chosen.x as f32, a_chosen.y as f32, a_chosen.seed_type, a_chosen.imitater_type,
                            0.0, 255, true, false,
                        );
                    }
                }
            }

            // C++: 各按钮（mMenuButton 用副本 Graphics，trans -= mX/mY 恒 0 等价直绘）
            if let Some(btn) = self.start_button { unsafe { (&mut *btn).draw(g); } }
            if let Some(btn) = self.random_button { unsafe { (&mut *btn).draw(g); } }
            if let Some(btn) = self.view_lawn_button { unsafe { (&mut *btn).draw(g); } }
            if let Some(btn) = self.almanac_button { unsafe { (&mut *btn).draw(g); } }
            if let Some(btn) = self.store_button { unsafe { (&mut *btn).draw(g); } }
            if let Some(btn) = self.menu_button { unsafe { (&mut *btn).draw(g); } }
            if let Some(tt) = self.tool_tip { unsafe { (*tt).draw(g); } }
        }
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
        // C++: ImitaterDialog 由 LawnApp 对话框系统更新（Rust 对话框驱动转发）
        if let Some(d) = self.imitater_dialog {
            unsafe {
                (*d).update();
                if !(*d).visible {
                    // 对应 C++ KillDialog：选中后关闭并释放
                    let _ = Box::from_raw(d);
                    self.imitater_dialog = None;
                }
            }
            return;
        }
        self.seed_chooser_age += 1;
        if let Some(app) = self.app { unsafe {
            self.last_mouse_x = 0 /* widget_manager */;
            self.last_mouse_y = 0 /* widget_manager */;
        } }
        self.show_tool_tip();
        if self.choose_state == SeedChooserState::ViewLawn { self.update_view_lawn(); }
    }
    pub fn display_repick_warning_dialog(&self, msg: &str) -> bool {
        // 对应 C++ DisplayRepickWarningDialog：LawnMessageBox(DIALOG_CHOOSER_WARNING,
        // "[DIALOG_WARNING]", theMessage, "[DIALOG_BUTTON_YES]", "[REPICK_BUTTON]", BUTTONS_YES_NO)
        // == ID_YES
        // [TRANSLATION_NOTE]: C++ 为阻塞式（返回按钮 ID）；Rust do_dialog 非阻塞（结果经
        // dialog_listener 回调），此处弹出对话框并返回 true（无法同步判定 Yes，调用链按放行处理）
        if let Some(app) = self.app {
            unsafe {
                (*app).do_dialog(
                    Dialogs::ChooserWarning as i32,
                    true,
                    "[DIALOG_WARNING]",
                    msg,
                    "[REPICK_BUTTON]",
                    BUTTONS_YES_NO,
                );
            }
        }
        true
    }
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

        // C++: aWarning = PvzpStringTranslate("[SEED_CHOOSER_UPGRADE_WARNING]")
        //      → PvzpReplaceString(aWarning, "{UPGRADE_TO}", Plant::GetNameString(theSeedTypeTo))
        //      → PvzpReplaceString(aWarning, "{UPGRADE_FROM}", Plant::GetNameString(theSeedTypeFrom))
        //      → DisplayRepickWarningDialog(aWarning)
        //（字符串翻译系统未接入，以键名作为原始文本）
        let mut a_warning = "[SEED_CHOOSER_UPGRADE_WARNING]".to_string();
        let a_name_to = crate::lawn::plant::Plant::get_name_string(to, SeedType::None);
        let a_name_from = crate::lawn::plant::Plant::get_name_string(from, SeedType::None);
        a_warning = a_warning.replace("{UPGRADE_TO}", &a_name_to);
        a_warning = a_warning.replace("{UPGRADE_FROM}", &a_name_from);
        self.display_repick_warning_dialog(&a_warning)
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
    pub fn enable_start_button(&mut self, enabled: bool) {
        // 对应 C++ SeedChooserScreen::EnableStartButton（SeedChooserScreen.cpp:803）
        if let Some(btn) = self.start_button {
            unsafe {
                (*btn).disabled = !enabled;
                if enabled {
                    (*btn).colors[GameButton::COLOR_LABEL] = Color::new(255, 255, 255, 255);
                } else {
                    (*btn).colors[GameButton::COLOR_LABEL] = Color::new(64, 64, 64, 255);
                }
            }
        }
    }
    /// 对应 C++ SeedChooserScreen::UpdateAfterPurchase（SeedChooserScreen.cpp:1138）
    /// 商店购买后刷新已选种子的位置（含商店购买种子槽后卡槽数变化）。
    pub fn update_after_purchase(&mut self) {
        // [TRANSLATION_NOTE]: C++ 用固定数组 mChosenSeeds[SeedType] 按种子类型索引，Rust 侧为 Vec；
        // 先快照各槽状态计算新位置，再统一写回，避免借用冲突。
        let a_positions: Vec<(usize, i32, i32, bool)> = self
            .chosen_seeds
            .iter()
            .enumerate()
            .map(|(idx, a_chosen_seed)| {
                let mut a_new_x = 0;
                let mut a_new_y = 0;
                let a_updated = if a_chosen_seed.seed_state == ChosenSeedState::InBank {
                    self.get_seed_position_in_bank(a_chosen_seed.seed_index_in_bank, &mut a_new_x, &mut a_new_y);
                    true
                } else if a_chosen_seed.seed_state == ChosenSeedState::InChooser {
                    self.get_seed_position_in_chooser(a_chosen_seed.seed_type as i32, &mut a_new_x, &mut a_new_y);
                    true
                } else {
                    false
                };
                (idx, a_new_x, a_new_y, a_updated)
            })
            .collect();
        for (idx, a_new_x, a_new_y, a_updated) in a_positions {
            if a_updated {
                let a_chosen_seed = &mut self.chosen_seeds[idx];
                a_chosen_seed.x = a_new_x;
                a_chosen_seed.y = a_new_y;
                a_chosen_seed.start_x = a_new_x;
                a_chosen_seed.start_y = a_new_y;
                a_chosen_seed.end_x = a_new_x;
                a_chosen_seed.end_y = a_new_y;
            }
        }
        // C++: EnableStartButton(mSeedsInBank == mBoard->mSeedBank->mNumPackets)
        let a_in_bank_matches = self
            .board
            .map_or(false, |b| unsafe { (*b).get_num_seeds_in_bank() == self.seeds_in_bank });
        self.enable_start_button(a_in_bank_matches);
        // C++: UpdateImitaterButton()（Imitater 按钮状态刷新）
        self.update_imitater_button();
    }

    /// 对应 C++ SeedChooserScreen::UpdateImitaterButton（SeedChooserScreen.cpp:831）
    pub fn update_imitater_button(&mut self) {
        let a_has_imitater = self.app.map_or(false, |app| unsafe { (*app).has_seed_type(SeedType::Imitater) });
        if let Some(btn) = self.imitater_button {
            unsafe {
                if !a_has_imitater {
                    (*btn).btn_no_draw = true;
                    (*btn).disabled = true;
                } else {
                    (*btn).btn_no_draw = false;
                    let a_state = self
                        .chosen_seeds
                        .iter()
                        .find(|s| s.seed_type == SeedType::Imitater)
                        .map_or(ChosenSeedState::Hidden, |s| s.seed_state);
                    (*btn).disabled = a_state != ChosenSeedState::Hidden;
                }
            }
        }
    }

    /// 对应 C++ SeedChooserScreen::KeyDown（SeedChooserScreen.cpp:802）
    pub fn key_down(&mut self, key: KeyCode) {
        if let Some(board) = self.board {
            unsafe {
                (*board).do_typing_check(key);
            }
        }

        let a_view_lawn = self.choose_state == SeedChooserState::ViewLawn;
        if a_view_lawn && (key == KEYCODE_SPACE || key == KEYCODE_RETURN || key == KEYCODE_ESCAPE) {
            self.cancel_lawn_view();
        } else if key == KEYCODE_ESCAPE {
            // C++: ButtonDepress(SeedChooserScreen_Menu)（ID 104）
            self.button_depress(104);
        }
    }

    /// 对应 C++ SeedChooserScreen::KeyChar（SeedChooserScreen.cpp:816）
    pub fn key_char(&mut self, the_char: char) {
        // C++: mBoard->KeyChar(theChar)；Rust 侧 Board 未实现 KeyChar，保留调用点语义
        let _ = the_char;
    }

    /// 对应 C++ SeedChooserScreen::MouseDown（SeedChooserScreen.cpp:908）
    pub fn mouse_down(&mut self, x: i32, y: i32, _the_click_count: i32) {
        // C++: ImitaterDialog 由 LawnApp 对话框系统接收鼠标（Rust 对话框驱动转发）
        if let Some(d) = self.imitater_dialog {
            unsafe { (*d).mouse_down(x - (*d).x, y - (*d).y, _the_click_count); }
            return;
        }
        // C++: Widget::MouseDown(x, y, theClickCount) —— Rust 无 Widget 基类，等效空操作

        if self.seeds_in_flight > 0 {
            // C++: for (i = 0; i < NUM_SEEDS_IN_CHOOSER; i++) LandFlyingSeed(mChosenSeeds[i])
            // Rust chosen_seeds 为 Vec（仅含活跃种子），遍历等价。
            let a_count = self.chosen_seeds.len();
            for i in 0..a_count {
                let seed_ptr = &mut self.chosen_seeds[i] as *mut ChosenSeed;
                unsafe {
                    self.land_flying_seed(&mut *seed_ptr);
                }
            }
        }

        if self.choose_state == SeedChooserState::ViewLawn {
            self.cancel_lawn_view();
            return;
        }

        if self.random_button.map_or(false, |b| unsafe { (*b).is_over }) {
            if let Some(app) = self.app {
                unsafe {
                    (*app).play_sample(crate::framework::resources::ResourceId::SoundTap as i32);
                }
            }
            // C++: ButtonDepress(SeedChooserScreen_Random)（ID 101）
            self.button_depress(101);
        } else if self.view_lawn_button.map_or(false, |b| unsafe { (*b).is_over }) {
            if let Some(app) = self.app {
                unsafe {
                    (*app).play_sample(crate::framework::resources::ResourceId::SoundTap as i32);
                }
            }
            // C++: ButtonDepress(SeedChooserScreen_ViewLawn)（ID 102）
            self.button_depress(102);
        } else if self.menu_button.map_or(false, |b| unsafe { (*b).is_over }) {
            if let Some(app) = self.app {
                unsafe {
                    (*app).play_sample(crate::framework::resources::ResourceId::SoundGravebutton as i32);
                }
            }
        } else if self.start_button.map_or(false, |b| unsafe { (*b).is_over })
            || self.almanac_button.map_or(false, |b| unsafe { (*b).is_over })
            || self.store_button.map_or(false, |b| unsafe { (*b).is_over })
        {
            if let Some(app) = self.app {
                unsafe {
                    (*app).play_sample(crate::framework::resources::ResourceId::SoundTap as i32);
                }
            }
        } else if self.imitater_button.map_or(false, |b| unsafe { (*b).is_over }) {
            // C++: if (mSeedsInBank != mBoard->mSeedBank->mNumPackets)
            let a_seeds_available = self
                .board
                .map_or(false, |b| unsafe { (*b).get_num_seeds_in_bank() != self.seeds_in_bank });
            if a_seeds_available {
                if let Some(app) = self.app {
                    unsafe {
                        (*app).play_sample(crate::framework::resources::ResourceId::SoundTap as i32);
                        // C++: ImitaterDialog* aDialog = new ImitaterDialog();
                        //      mApp->AddDialog(aDialog->mId, aDialog);
                        //      aDialog->Resize(居中, mWidth, mHeight); SetFocus(aDialog)
                        // [TRANSLATION_NOTE]: Rust ImitaterDialog 为独立结构（非 framework Dialog），
                        // 由 SeedChooserScreen 对话框驱动持有（见 draw/update/mouse_down 转发）
                        if self.imitater_dialog.is_none() {
                            let mut a_dialog = Box::new(ImitaterDialog::new(app, BOARD_WIDTH, BOARD_HEIGHT));
                            let a_w = a_dialog.width;
                            let a_h = a_dialog.height;
                            a_dialog.x = (BOARD_WIDTH - a_w) / 2;
                            a_dialog.y = (BOARD_HEIGHT - a_h) / 2;
                            a_dialog.visible = true;
                            self.imitater_dialog = Some(Box::into_raw(a_dialog));
                        }
                    }
                }
            }
        } else {
            // C++: !mBoard->mSeedBank->ContainsPoint(x, y)（SeedBank::ContainsPoint，SeedPacket.cpp:1013）
            let a_seed_bank_contains = self.board.map_or(false, |b| unsafe { (*b).seed_bank_contains_point(x, y) });
            let a_almanac_over = self.almanac_button.map_or(false, |b| unsafe { (*b).is_over });
            let a_store_over = self.store_button.map_or(false, |b| unsafe { (*b).is_over });
            if !a_seed_bank_contains && !a_almanac_over && !a_store_over {
                if let Some(app) = self.app {
                    unsafe {
                        if (*app).can_show_almanac() {
                            if let Some(board) = self.board {
                                // C++: mBoard->ZombieHitTest(x - mBoard->mX, y - mBoard->mY)；Board.mX 恒 0
                                let a_zombie_idx = (*board).zombie_hit_test(x, y);
                                if let Some(idx) = a_zombie_idx {
                                    // [TRANSLATION_NOTE]: nightly 禁止裸指针隐式 autoref，先显式解引用
                                    let board_ref = unsafe { &*board };
                                    if idx < board_ref.zombies.len() {
                                        let a_zombie = &board_ref.zombies[idx];
                                        if a_zombie.from_wave == crate::lawn::zombie::Zombie::ZOMBIE_WAVE_CUTSCENE
                                            && a_zombie.zombie_type != ZombieType::RedeEyeGargantuar
                                        {
                                            (*app).play_sample(crate::framework::resources::ResourceId::SoundTap as i32);
                                            let a_zombie_type = a_zombie.zombie_type;
                                            // C++: DoAlmanacDialog(...)->WaitForResult(true)；Rust 版返回 ()，无模态等待
                                            (*app).do_almanac_dialog(SeedType::None, a_zombie_type);
                                            if let Some(music) = (*app).music.as_mut() {
                                                music.make_sure_music_is_playing(MusicTune::ChooseYourSeeds);
                                            }
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let a_seed_type = self.seed_hit_test(x, y);
            if a_seed_type != SeedType::None && !self.seed_not_allowed_to_pick(a_seed_type) {
                if self.seed_not_allowed_during_trial(a_seed_type) {
                    if let Some(app) = self.app {
                        unsafe {
                            (*app).play_sample(crate::framework::resources::ResourceId::SoundTap as i32);
                            // C++: LawnMessageBox(DIALOG_MESSAGE, [GET_FULL_VERSION_TITLE], [GET_FULL_VERSION_BODY],
                            //      [GET_FULL_VERSION_YES_BUTTON], [GET_FULL_VERSION_NO_BUTTON], BUTTONS_YES_NO)
                            let a_dialog = (*app).do_dialog(
                                48, // DIALOG_MESSAGE
                                true,
                                "[GET_FULL_VERSION_TITLE]",
                                "[GET_FULL_VERSION_BODY]",
                                "",
                                BUTTONS_YES_NO,
                            );
                            let a_result = a_dialog.map_or(0, |d| unsafe { (&mut *d).wait_for_result(true) });
                            if a_result == ID_YES {
                                (*app).do_back_to_main();
                            }
                        }
                    }
                } else {
                    let a_seed_idx = self.chosen_seeds.iter().position(|s| s.seed_type == a_seed_type);
                    if let Some(seed_idx) = a_seed_idx {
                        let seed_ptr = &mut self.chosen_seeds[seed_idx] as *mut ChosenSeed;
                        unsafe {
                            let a_chosen_seed = &mut *seed_ptr;
                            if a_chosen_seed.seed_state == ChosenSeedState::InBank {
                                if a_chosen_seed.crazy_dave_picked {
                                    if let Some(app) = self.app {
                                        (*app).play_sample(crate::framework::resources::ResourceId::SoundBuzzer as i32);
                                    }
                                    if let Some(tip) = self.tool_tip {
                                        (*tip).flash_warning();
                                    }
                                } else {
                                    self.clicked_seed_in_bank(a_chosen_seed);
                                }
                            } else if a_chosen_seed.seed_state == ChosenSeedState::InChooser {
                                self.clicked_seed_in_chooser(a_chosen_seed);
                            }
                        }
                    }
                }
            }
        }
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

/// WidgetManager 包装（对应 C++ SeedChooserScreen : Widget）
pub struct SeedChooserScreenImpl {
    pub screen: *mut SeedChooserScreen,
}

impl SeedChooserScreenImpl {
    pub fn new(screen: *mut SeedChooserScreen) -> Self {
        SeedChooserScreenImpl { screen }
    }
}

impl WidgetImpl for SeedChooserScreenImpl {
    fn update(&mut self, _widget: &mut Widget) {
        unsafe { (*self.screen).update(); }
    }
    fn draw(&mut self, _widget: &Widget, g: &mut Graphics) {
        unsafe { (*self.screen).draw(g); }
    }
    fn key_char(&mut self, _widget: &mut Widget, c: u8) {
        unsafe { (*self.screen).key_char(c as char); }
    }
    fn key_down(&mut self, _widget: &mut Widget, key: KeyCode, _wm: &mut WidgetManager) {
        unsafe { (*self.screen).key_down(key); }
    }
    fn mouse_down_btn(&mut self, _widget: &mut Widget, x: i32, y: i32, _btn: i32, click: i32) {
        unsafe { (*self.screen).mouse_down(x, y, click); }
    }
    fn mouse_up_btn(&mut self, _widget: &mut Widget, x: i32, y: i32, _btn: i32, click: i32) {
        unsafe { (*self.screen).mouse_up(x, y, click); }
    }
}

