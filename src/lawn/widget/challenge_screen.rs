// PvZ Portable Rust 翻译 — ChallengeScreen（挑战模式选择界面）
// 对应 C++ src/Lawn/Widget/ChallengeScreen.h / ChallengeScreen.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::todlib::tod_foley::FoleyType;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::button_widget::ButtonWidget;
use crate::framework::widget::dialog_button::DialogButton;
use crate::lawn::game_enums::*;

/// 挑战模式数量（对应 C++ #define NUM_CHALLENGE_MODES）
pub const NUM_CHALLENGE_MODES: usize = 73; // 对应 C++ NUM_GAME_MODES - 1

/// 挑战模式选择界面（对应 C++ ChallengeScreen）
pub struct ChallengeScreen {
    pub back_button: Option<*mut DialogButton>,
    pub page_button: [Option<*mut ButtonWidget>; 4],  // MAX_CHALLANGE_PAGES
    pub challenge_buttons: Vec<Option<*mut ButtonWidget>>,
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub tool_tip: Option<*mut crate::lawn::tool_tip_widget::ToolTipWidget>,
    pub page_index: ChallengePage,
    pub cheat_enable_challenges: bool,
    pub unlock_state: UnlockingState,
    pub unlock_state_counter: i32,
    pub unlock_challenge_index: i32,
    pub lock_shake_x: f32,
    pub lock_shake_y: f32,
    pub limbo_page_unlocked: bool,
    pub click_count: i32,
    pub last_click_time: u32,
}

impl ChallengeScreen {
    pub fn new() -> Self {
        ChallengeScreen {
            back_button: None,
            page_button: [None, None, None, None],
            challenge_buttons: vec![None; 73],
            app: None,
            tool_tip: None,
            page_index: ChallengePage::Survival,
            cheat_enable_challenges: false,
            unlock_state: UnlockingState::Off,
            unlock_state_counter: 0,
            unlock_challenge_index: 0,
            lock_shake_x: 0.0,
            lock_shake_y: 0.0,
            limbo_page_unlocked: false,
            click_count: 0,
            last_click_time: 0,
        }
    }

    pub fn set_unlock_challenge_index(&mut self, page: ChallengePage, is_i_zombie: bool) {
        // 对应 C++ SetUnlockChallengeIndex：查找当前页未锁定且可解锁的最后一个挑战
        self.unlock_state = UnlockingState::Shaking;
        self.unlock_state_counter = 100;
        self.unlock_challenge_index = 0;
        for a_challenge_mode in 0..72 { // NUM_CHALLENGE_MODES
            let Some(a_def) = get_challenge_definition(a_challenge_mode) else { continue };
            if a_def.page == page {
                let page_ok = page != ChallengePage::Puzzle
                    || (!is_i_zombie && Self::is_scary_potter_level(a_def.challenge_mode))
                    || (is_i_zombie && Self::is_i_zombie_level(a_def.challenge_mode));
                if page_ok && self.accomplishments_needed(a_challenge_mode) <= 0 {
                    self.unlock_challenge_index = a_challenge_mode;
                }
            }
        }
    }

    pub fn more_trophies_needed(&self, challenge_index: i32) -> i32 {
        // 对应 C++ MoreTrophiesNeeded（简化版）
        // [TRANSLATION_NOTE]: 依赖 gChallengeDefs 静态定义表（mRow/mCol/mPage）与 GetNumTrophies，
        // Rust 侧暂无定义表，按模式区间近似
        let a_def = get_challenge_definition(challenge_index);
        let a_mode = a_def.map_or(GameMode::Adventure, |d| d.challenge_mode);

        if let Some(app) = self.app {
            unsafe {
                let app = &*app;
                if app.has_finished_adventure() {
                    // 冒险完成后：挑战页第 4 项起按奖杯数（简化）
                    if a_def.map_or(false, |d| d.page == ChallengePage::Survival) {
                        return 0;
                    }
                } else if Self::is_scary_potter_level(a_mode) || Self::is_i_zombie_level(a_mode) {
                    // 未通关冒险：解谜页按已过关数
                    let mut a_levels_completed = 0;
                    let base = if Self::is_scary_potter_level(a_mode) {
                        GameMode::ScaryPotter1 as i32
                    } else {
                        GameMode::PuzzleIZombie1 as i32
                    };
                    for offset in 0..9 {
                        let m = unsafe { std::mem::transmute::<i32, GameMode>(base + offset) };
                        if app.has_beaten_challenge(m) {
                            a_levels_completed += 1;
                        }
                    }
                    return (a_mode as i32 - base - a_levels_completed).clamp(0, 9);
                }
            }
        }
        0
    }

    pub fn show_page_buttons(&self) -> bool {
        // 对应 C++ ShowPageButtons
        let cheat_keys = self.app.map_or(false, |app| unsafe { (*app).m_tod_cheat_keys });
        cheat_keys
            && self.page_index != ChallengePage::Survival
            && self.page_index != ChallengePage::Puzzle
    }

    pub fn update_buttons(&mut self) {
        // 对应 C++ UpdateButtons
        for a_challenge_mode in 0..72 { // NUM_CHALLENGE_MODES
            let visible = get_challenge_definition(a_challenge_mode)
                .map_or(false, |def| def.page == self.page_index);
            if let Some(btn) = self.challenge_buttons.get(a_challenge_mode as usize).copied().flatten() {
                unsafe { (*btn).visible = visible; }
            }
        }
        for a_page in 0..4 { // MAX_CHALLANGE_PAGES
            let Some(btn) = self.page_button[a_page] else { continue };
            unsafe {
                let b = &mut *btn;
                if a_page as i32 == ChallengePage::Limbo as i32 && self.limbo_page_unlocked {
                    b.visible = true;
                }
                if a_page as i32 == self.page_index as i32 {
                    if !b.colors.is_empty() {
                        b.colors[crate::framework::widget::button_widget::COLOR_LABEL] = crate::framework::color::Color::from_rgb(64, 64, 64);
                    }
                    b.disabled = true;
                } else {
                    if !b.colors.is_empty() {
                        b.colors[crate::framework::widget::button_widget::COLOR_LABEL] = crate::framework::color::Color::from_rgb(255, 240, 0);
                    }
                    b.disabled = false;
                }
            }
        }
    }

    pub fn accomplishments_needed(&self, challenge_index: i32) -> i32 {
        // 对应 C++ AccomplishmentsNeeded
        let mut a_trophies_needed = self.more_trophies_needed(challenge_index);
        let a_game_mode = get_challenge_definition(challenge_index).map_or(GameMode::Adventure, |d| d.challenge_mode);
        if let Some(app) = self.app {
            unsafe {
                if (*app).is_survival_endless(a_game_mode)
                    && a_trophies_needed <= 3
                    && crate::lawn::lawn_app::LawnApp::get_num_trophies(ChallengePage::Survival as i32) < 10
                    && (*app).has_finished_adventure()
                    && !(*app).is_trial_stage_locked()
                {
                    a_trophies_needed = 1;
                }
            }
        }
        if self.cheat_enable_challenges { 0 } else { a_trophies_needed }
    }

    pub fn draw_button(&self, g: &mut Graphics, challenge_index: i32) {
        // 对应 C++ DrawButton：绘制挑战按钮（图标/窗口框/名称/锁）
        let Some(btn) = self.challenge_buttons.get(challenge_index as usize).copied().flatten() else { return };
        unsafe {
            if !(*btn).visible {
                return;
            }
            let Some(a_def) = get_challenge_definition(challenge_index) else { return };
            let mut a_pos_x = (*btn).x;
            let mut a_pos_y = (*btn).y;
            if (*btn).is_down {
                a_pos_x += 1;
                a_pos_y += 1;
            }

            if self.accomplishments_needed(challenge_index) <= 1 {
                // [TRANSLATION_NOTE]: C++ 中绘制缩略图（IMAGE_SURVIVAL_THUMBNAILS/
                // IMAGE_CHALLENGE_THUMBNAILS）与窗口框（IMAGE_CHALLENGE_WINDOW[HIGHLIGHT]）；
                // Rust 侧图片资源未接入，暂略

                // 解锁中处理（C++ 中设置颜色化与摇动偏移绘制锁图标）
                if challenge_index == self.unlock_challenge_index {
                    let _shake_x = self.lock_shake_x;
                    let _shake_y = self.lock_shake_y;
                }

                let a_record = self.app.map_or(0, |app| unsafe {
                    (*app).player_info.as_ref().map_or(0, |info| {
                        info.m_challenge_records.get(challenge_index as usize).copied().unwrap_or(0)
                    })
                });
                if a_record > 0 {
                    // [TRANSLATION_NOTE]: C++ 中已通关绘制奖杯，Endless 挑战绘制旗数/最长连击文本
                    let _ = a_record;
                }
            } else {
                // [TRANSLATION_NOTE]: C++ 中绘制 IMAGE_CHALLENGE_BLANK 空按钮
            }
            let _ = (g, a_pos_x, a_pos_y);
        }
    }

    pub fn draw(&self, g: &mut Graphics) {
        // 对应 C++ Draw：背景/标题/奖杯计数 + 各挑战按钮
        // [TRANSLATION_NOTE]: C++ 中绘制 IMAGE_CHALLENGE_BACKGROUND 背景与标题文字
        let a_title = match self.page_index {
            ChallengePage::Survival => "[PICK_AREA]",
            ChallengePage::Puzzle => "[SCARY_POTTER]",
            _ => "[PICK_CHALLENGE]",
        };
        let _ = a_title;

        // C++ 中奖杯计数（mApp->GetNumTrophies(mPageIndex)）
        let a_trophies_total = match self.page_index {
            ChallengePage::Survival => 10,
            ChallengePage::Challenge => 20,
            ChallengePage::Puzzle => 18,
            _ => 0,
        };
        if a_trophies_total > 0 {
            // [TRANSLATION_NOTE]: C++ 中绘制 "x/y" 奖杯字符串与 IMAGE_TROPHY 图标
            let _ = a_trophies_total;
        }

        for a_challenge_mode in 0..72 {
            self.draw_button(g, a_challenge_mode);
        }
    }

    pub fn update(&mut self) {
        // 对应 C++ Update：解锁状态机（Shaking → Fading → Off）
        self.update_tool_tip();
        if self.unlock_state_counter > 0 {
            self.unlock_state_counter -= 1;
        }
        if self.unlock_state == UnlockingState::Shaking {
            if self.unlock_state_counter == 0 {
                if let Some(app) = self.app {
                    unsafe { (*app).play_foley(FoleyType::Paper as i32); }
                }
                self.unlock_state = UnlockingState::Fading;
                self.unlock_state_counter = 50;
                self.lock_shake_x = 0.0;
                self.lock_shake_y = 0.0;
            } else {
                self.lock_shake_x = crate::todlib::tod_common::rand_range_float(-2.0, 2.0);
                self.lock_shake_y = crate::todlib::tod_common::rand_range_float(-2.0, 2.0);
            }
        } else if self.unlock_state == UnlockingState::Fading && self.unlock_state_counter == 0 {
            self.unlock_state = UnlockingState::Off;
            self.unlock_state_counter = 0;
            self.unlock_challenge_index = -1;
        }
    }

    pub fn added_to_manager(&mut self, _manager: &mut WidgetManager) {
        // C++ 中 AddWidget(mBackButton) + 所有页签/挑战按钮
        // [TRANSLATION_NOTE]: Rust 侧 ButtonWidget 未接入 WidgetManager::add_widget，暂略
    }

    pub fn removed_from_manager(&mut self, _manager: &mut WidgetManager) {
        // C++ 中 RemoveWidget(mBackButton) + 所有页签/挑战按钮
        // [TRANSLATION_NOTE]: Rust 侧 ButtonWidget 未接入 WidgetManager，暂略
    }

    pub fn button_press(&mut self, _id: i32) {
        // [TRANSLATION_NOTE]: C++ 中 PlaySample(SOUND_ALMANAC_BUTTON)
    }

    pub fn button_depress(&mut self, the_id: i32) {
        // 对应 C++ ButtonDepress
        let Some(app) = self.app else { return };
        unsafe {
            if the_id == 100 { // ChallengeScreen_Back
                (*app).kill_challenge_screen();
                (*app).do_back_to_main();
            }

            let a_challenge_mode = the_id - 200; // ChallengeScreen_Mode
            if a_challenge_mode >= 0 && a_challenge_mode < 72 { // NUM_CHALLENGE_MODES
                (*app).kill_challenge_screen();
                (*app).pre_new_game(std::mem::transmute::<i32, GameMode>(a_challenge_mode + 1), true);
            }

            let a_page_index = the_id - 300; // ChallengeScreen_Page
            if a_page_index >= 0 && a_page_index < 4 {
                self.page_index = std::mem::transmute::<i32, ChallengePage>(a_page_index);
                self.update_buttons();
            }
        }
    }

    pub fn update_tool_tip(&mut self) {
        // 对应 C++ UpdateToolTip：锁定挑战悬停时显示解锁提示
        let Some(app) = self.app else { return };
        unsafe {
            // [TRANSLATION_NOTE]: C++ 中 mWidgetManager->mMouseIn/mActive 判定鼠标在窗内；
            // Rust 侧以 base.active 近似
            if !(*app).base.active {
                self.tool_tip.map_or((), |tip| unsafe { (*tip).m_visible = false; });
                return;
            }
        }
        let mouse_x = 0; // [TRANSLATION_NOTE]: Rust 侧 widget_manager 未跟踪鼠标位置
        let mouse_y = 0;
        for a_challenge_mode in 0..72 {
            let Some(a_def) = get_challenge_definition(a_challenge_mode) else { continue };
            let Some(btn) = self.challenge_buttons.get(a_challenge_mode as usize).copied().flatten() else { continue };
            unsafe {
                if !(*btn).visible || !(*btn).disabled {
                    continue;
                }
                let in_button = mouse_x >= (*btn).x && mouse_x < (*btn).x + (*btn).width
                    && mouse_y >= (*btn).y && mouse_y < (*btn).y + (*btn).height;
                if !in_button || self.accomplishments_needed(a_challenge_mode) > 1 {
                    continue;
                }
                let tip = self.tool_tip;
                if let Some(tip) = tip {
                    (*tip).m_x = (*btn).width / 2 + (*btn).x;
                    (*tip).m_y = (*btn).y;
                }
                if self.more_trophies_needed(a_challenge_mode) > 0 {
                    let mut a_label = "";
                    if self.page_index == ChallengePage::Puzzle {
                        if Self::is_scary_potter_level(a_def.challenge_mode) {
                            a_label = if !(*app).has_finished_adventure() && a_def.challenge_mode == GameMode::ScaryPotter4 {
                                "[FINISH_ADVENTURE_TOOLTIP]"
                            } else {
                                "[ONE_MORE_SCARY_POTTER_TOOLTIP]"
                            };
                        } else if Self::is_i_zombie_level(a_def.challenge_mode) {
                            a_label = if !(*app).has_finished_adventure() && a_def.challenge_mode == GameMode::PuzzleIZombie4 {
                                "[FINISH_ADVENTURE_TOOLTIP]"
                            } else {
                                "[ONE_MORE_IZOMBIE_TOOLTIP]"
                            };
                        }
                    } else if !(*app).has_finished_adventure() || (*app).is_trial_stage_locked() {
                        a_label = "[FINISH_ADVENTURE_TOOLTIP]";
                    } else if (*app).is_survival_endless(a_def.challenge_mode) {
                        a_label = "[10_SURVIVAL_TOOLTIP]";
                    } else if self.page_index == ChallengePage::Survival {
                        a_label = "[ONE_MORE_SURVIVAL_TOOLTIP]";
                    } else if self.page_index == ChallengePage::Challenge {
                        a_label = "[ONE_MORE_CHALLENGE_TOOLTIP]";
                    } else {
                        continue;
                    }
                    if let Some(tip) = self.tool_tip {
                        unsafe {
                            (*tip).set_label(a_label);
                            (*tip).m_visible = true;
                        }
                    }
                    return;
                }
            }
        }
        if let Some(tip) = self.tool_tip {
            unsafe { (*tip).m_visible = false; }
        }
    }

    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // 对应 C++ MouseDown：快速连点 5 次解锁 Limbo 页
        if self.limbo_page_unlocked {
            return;
        }
        const MAX_GAP_TICKS: u32 = 20;
        const CLICKS_NEEDED: i32 = 5;
        let a_now = self.app.map_or(0, |app| unsafe { (*app).m_app_counter });
        if a_now.saturating_sub(self.last_click_time) > MAX_GAP_TICKS {
            self.click_count = 0;
        }
        self.last_click_time = a_now;
        self.click_count += 1;
        if self.click_count >= CLICKS_NEEDED {
            self.limbo_page_unlocked = true;
            self.update_buttons();
        }
    }

    pub fn is_scary_potter_level(game_mode: GameMode) -> bool {
        // 对应 C++ IsScaryPotterLevel：GAMEMODE_SCARY_POTTER_1 .. ENDLESS
        let m = game_mode as i32;
        m >= GameMode::ScaryPotter1 as i32 && m <= GameMode::ScaryPotterEndless as i32
    }

    pub fn is_i_zombie_level(game_mode: GameMode) -> bool {
        // 对应 C++ IsIZombieLevel：GAMEMODE_PUZZLE_I_ZOMBIE_1 .. ENDLESS
        let m = game_mode as i32;
        m >= GameMode::PuzzleIZombie1 as i32 && m <= GameMode::PuzzleIZombieEndless as i32
    }
}

impl Default for ChallengeScreen {
    fn default() -> Self {
        ChallengeScreen::new()
    }
}

/// 挑战模式定义（对应 C++ ChallengeDefinition）
pub struct ChallengeDefinition {
    pub challenge_mode: GameMode,
    pub challenge_icon_index: i32,
    pub page: ChallengePage,
    pub row: i32,
    pub col: i32,
    pub challenge_name: Option<&'static str>,
}

/// 获取挑战模式定义（对应 C++ GetChallengeDefinition）
pub fn get_challenge_definition(challenge_mode: i32) -> Option<&'static ChallengeDefinition> {
    // 对应 C++ gChallengeDefs 静态定义表
    static G_CHALLENGE_DEFS: [ChallengeDefinition; 72] = [
        ChallengeDefinition { challenge_mode: GameMode::SurvivalNormalStage1, challenge_icon_index: 0, page: ChallengePage::Survival, row: 0, col: 0, challenge_name: Some("[SURVIVAL_DAY_NORMAL]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalNormalStage2, challenge_icon_index: 1, page: ChallengePage::Survival, row: 0, col: 1, challenge_name: Some("[SURVIVAL_NIGHT_NORMAL]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalNormalStage3, challenge_icon_index: 2, page: ChallengePage::Survival, row: 0, col: 2, challenge_name: Some("[SURVIVAL_POOL_NORMAL]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalNormalStage4, challenge_icon_index: 3, page: ChallengePage::Survival, row: 0, col: 3, challenge_name: Some("[SURVIVAL_FOG_NORMAL]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalNormalStage5, challenge_icon_index: 4, page: ChallengePage::Survival, row: 0, col: 4, challenge_name: Some("[SURVIVAL_ROOF_NORMAL]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalHardStage1, challenge_icon_index: 5, page: ChallengePage::Survival, row: 1, col: 0, challenge_name: Some("[SURVIVAL_DAY_HARD]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalHardStage2, challenge_icon_index: 6, page: ChallengePage::Survival, row: 1, col: 1, challenge_name: Some("[SURVIVAL_NIGHT_HARD]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalHardStage3, challenge_icon_index: 7, page: ChallengePage::Survival, row: 1, col: 2, challenge_name: Some("[SURVIVAL_POOL_HARD]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalHardStage4, challenge_icon_index: 8, page: ChallengePage::Survival, row: 1, col: 3, challenge_name: Some("[SURVIVAL_FOG_HARD]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalHardStage5, challenge_icon_index: 9, page: ChallengePage::Survival, row: 1, col: 4, challenge_name: Some("[SURVIVAL_ROOF_HARD]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalEndlessStage1, challenge_icon_index: 10, page: ChallengePage::Limbo, row: 3, col: 0, challenge_name: Some("[SURVIVAL_DAY_ENDLESS]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalEndlessStage2, challenge_icon_index: 10, page: ChallengePage::Limbo, row: 3, col: 1, challenge_name: Some("[SURVIVAL_NIGHT_ENDLESS]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalEndlessStage3, challenge_icon_index: 10, page: ChallengePage::Survival, row: 2, col: 2, challenge_name: Some("[SURVIVAL_POOL_ENDLESS]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalEndlessStage4, challenge_icon_index: 10, page: ChallengePage::Limbo, row: 3, col: 2, challenge_name: Some("[SURVIVAL_FOG_ENDLESS]") },
        ChallengeDefinition { challenge_mode: GameMode::SurvivalEndlessStage5, challenge_icon_index: 10, page: ChallengePage::Limbo, row: 3, col: 3, challenge_name: Some("[SURVIVAL_ROOF_ENDLESS]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeWarAndPeas, challenge_icon_index: 0, page: ChallengePage::Challenge, row: 0, col: 0, challenge_name: Some("[WAR_AND_PEAS]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeWallnutBowling, challenge_icon_index: 6, page: ChallengePage::Challenge, row: 0, col: 1, challenge_name: Some("[WALL_NUT_BOWLING]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeSlotMachine, challenge_icon_index: 2, page: ChallengePage::Challenge, row: 0, col: 2, challenge_name: Some("[SLOT_MACHINE]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeRainingSeeds, challenge_icon_index: 3, page: ChallengePage::Challenge, row: 0, col: 3, challenge_name: Some("[ITS_RAINING_SEEDS]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeBeghouled, challenge_icon_index: 1, page: ChallengePage::Challenge, row: 0, col: 4, challenge_name: Some("[BEGHOULED]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeInvisighoul, challenge_icon_index: 8, page: ChallengePage::Challenge, row: 1, col: 0, challenge_name: Some("[INVISIGHOUL]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeSeeingStars, challenge_icon_index: 5, page: ChallengePage::Challenge, row: 1, col: 1, challenge_name: Some("[SEEING_STARS]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeZombiquarium, challenge_icon_index: 7, page: ChallengePage::Challenge, row: 1, col: 2, challenge_name: Some("[ZOMBIQUARIUM]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeBeghouledTwist, challenge_icon_index: 20, page: ChallengePage::Challenge, row: 1, col: 3, challenge_name: Some("[BEGHOULED_TWIST]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeLittleTrouble, challenge_icon_index: 12, page: ChallengePage::Challenge, row: 1, col: 4, challenge_name: Some("[LITTLE_TROUBLE]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengePortalCombat, challenge_icon_index: 15, page: ChallengePage::Challenge, row: 2, col: 0, challenge_name: Some("[PORTAL_COMBAT]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeColumns, challenge_icon_index: 4, page: ChallengePage::Challenge, row: 2, col: 1, challenge_name: Some("[COLUMN_AS_YOU_SEE_EM]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeBobsledBonanza, challenge_icon_index: 17, page: ChallengePage::Challenge, row: 2, col: 2, challenge_name: Some("[BOBSLED_BONANZA]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeZombieNimble, challenge_icon_index: 18, page: ChallengePage::Challenge, row: 2, col: 3, challenge_name: Some("[ZOMBIES_ON_SPEED]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeWhackAZombie, challenge_icon_index: 16, page: ChallengePage::Challenge, row: 2, col: 4, challenge_name: Some("[WHACK_A_ZOMBIE]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeLastStand, challenge_icon_index: 21, page: ChallengePage::Challenge, row: 3, col: 0, challenge_name: Some("[LAST_STAND]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeWarAndPeas2, challenge_icon_index: 0, page: ChallengePage::Challenge, row: 3, col: 1, challenge_name: Some("[WAR_AND_PEAS_2]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeWallnutBowling2, challenge_icon_index: 6, page: ChallengePage::Challenge, row: 3, col: 2, challenge_name: Some("[WALL_NUT_BOWLING_EXTREME]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengePogoParty, challenge_icon_index: 14, page: ChallengePage::Challenge, row: 3, col: 3, challenge_name: Some("[POGO_PARTY]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeFinalBoss, challenge_icon_index: 19, page: ChallengePage::Challenge, row: 3, col: 4, challenge_name: Some("[FINAL_BOSS]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeArtChallengeWallnut, challenge_icon_index: 0, page: ChallengePage::Limbo, row: 0, col: 0, challenge_name: Some("[ART_CHALLENGE_WALL_NUT]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeSunnyDay, challenge_icon_index: 1, page: ChallengePage::Limbo, row: 0, col: 1, challenge_name: Some("[SUNNY_DAY]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeResodded, challenge_icon_index: 2, page: ChallengePage::Limbo, row: 0, col: 2, challenge_name: Some("[UNSODDED]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeBigTime, challenge_icon_index: 3, page: ChallengePage::Limbo, row: 0, col: 3, challenge_name: Some("[BIG_TIME]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeArtChallengeSunflower, challenge_icon_index: 4, page: ChallengePage::Limbo, row: 0, col: 4, challenge_name: Some("[ART_CHALLENGE_SUNFLOWER]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeAirRaid, challenge_icon_index: 5, page: ChallengePage::Limbo, row: 1, col: 0, challenge_name: Some("[AIR_RAID]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeIceLevel, challenge_icon_index: 6, page: ChallengePage::Limbo, row: 1, col: 1, challenge_name: Some("[ICE_LEVEL]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeZenGarden, challenge_icon_index: 7, page: ChallengePage::Limbo, row: 1, col: 2, challenge_name: Some("[ZEN_GARDEN]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeHighGravity, challenge_icon_index: 8, page: ChallengePage::Limbo, row: 1, col: 3, challenge_name: Some("[HIGH_GRAVITY]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeGraveDanger, challenge_icon_index: 11, page: ChallengePage::Limbo, row: 1, col: 4, challenge_name: Some("[GRAVE_DANGER]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeShovel, challenge_icon_index: 10, page: ChallengePage::Limbo, row: 2, col: 0, challenge_name: Some("[CAN_YOU_DIG_IT]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeStormyNight, challenge_icon_index: 13, page: ChallengePage::Limbo, row: 2, col: 1, challenge_name: Some("[DARK_STORMY_NIGHT]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeBungeeBlitz, challenge_icon_index: 9, page: ChallengePage::Limbo, row: 2, col: 2, challenge_name: Some("[BUNGEE_BLITZ]") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeSquirrel, challenge_icon_index: 10, page: ChallengePage::Limbo, row: 2, col: 3, challenge_name: Some("Squirrel") },
        ChallengeDefinition { challenge_mode: GameMode::ChallengeTreeOfWisdom, challenge_icon_index: 10, page: ChallengePage::Limbo, row: 2, col: 4, challenge_name: Some("Tree of Wisdom") },
        ChallengeDefinition { challenge_mode: GameMode::ScaryPotter1, challenge_icon_index: 10, page: ChallengePage::Puzzle, row: 0, col: 0, challenge_name: Some("[SCARY_POTTER_1]") },
        ChallengeDefinition { challenge_mode: GameMode::ScaryPotter2, challenge_icon_index: 10, page: ChallengePage::Puzzle, row: 0, col: 1, challenge_name: Some("[SCARY_POTTER_2]") },
        ChallengeDefinition { challenge_mode: GameMode::ScaryPotter3, challenge_icon_index: 10, page: ChallengePage::Puzzle, row: 0, col: 2, challenge_name: Some("[SCARY_POTTER_3]") },
        ChallengeDefinition { challenge_mode: GameMode::ScaryPotter4, challenge_icon_index: 10, page: ChallengePage::Puzzle, row: 0, col: 3, challenge_name: Some("[SCARY_POTTER_4]") },
        ChallengeDefinition { challenge_mode: GameMode::ScaryPotter5, challenge_icon_index: 10, page: ChallengePage::Puzzle, row: 0, col: 4, challenge_name: Some("[SCARY_POTTER_5]") },
        ChallengeDefinition { challenge_mode: GameMode::ScaryPotter6, challenge_icon_index: 10, page: ChallengePage::Puzzle, row: 1, col: 0, challenge_name: Some("[SCARY_POTTER_6]") },
        ChallengeDefinition { challenge_mode: GameMode::ScaryPotter7, challenge_icon_index: 10, page: ChallengePage::Puzzle, row: 1, col: 1, challenge_name: Some("[SCARY_POTTER_7]") },
        ChallengeDefinition { challenge_mode: GameMode::ScaryPotter8, challenge_icon_index: 10, page: ChallengePage::Puzzle, row: 1, col: 2, challenge_name: Some("[SCARY_POTTER_8]") },
        ChallengeDefinition { challenge_mode: GameMode::ScaryPotter9, challenge_icon_index: 10, page: ChallengePage::Puzzle, row: 1, col: 3, challenge_name: Some("[SCARY_POTTER_9]") },
        ChallengeDefinition { challenge_mode: GameMode::ScaryPotterEndless, challenge_icon_index: 10, page: ChallengePage::Puzzle, row: 1, col: 4, challenge_name: Some("[SCARY_POTTER_ENDLESS]") },
        ChallengeDefinition { challenge_mode: GameMode::PuzzleIZombie1, challenge_icon_index: 11, page: ChallengePage::Puzzle, row: 2, col: 0, challenge_name: Some("[I_ZOMBIE_1]") },
        ChallengeDefinition { challenge_mode: GameMode::PuzzleIZombie2, challenge_icon_index: 11, page: ChallengePage::Puzzle, row: 2, col: 1, challenge_name: Some("[I_ZOMBIE_2]") },
        ChallengeDefinition { challenge_mode: GameMode::PuzzleIZombie3, challenge_icon_index: 11, page: ChallengePage::Puzzle, row: 2, col: 2, challenge_name: Some("[I_ZOMBIE_3]") },
        ChallengeDefinition { challenge_mode: GameMode::PuzzleIZombie4, challenge_icon_index: 11, page: ChallengePage::Puzzle, row: 2, col: 3, challenge_name: Some("[I_ZOMBIE_4]") },
        ChallengeDefinition { challenge_mode: GameMode::PuzzleIZombie5, challenge_icon_index: 11, page: ChallengePage::Puzzle, row: 2, col: 4, challenge_name: Some("[I_ZOMBIE_5]") },
        ChallengeDefinition { challenge_mode: GameMode::PuzzleIZombie6, challenge_icon_index: 11, page: ChallengePage::Puzzle, row: 3, col: 0, challenge_name: Some("[I_ZOMBIE_6]") },
        ChallengeDefinition { challenge_mode: GameMode::PuzzleIZombie7, challenge_icon_index: 11, page: ChallengePage::Puzzle, row: 3, col: 1, challenge_name: Some("[I_ZOMBIE_7]") },
        ChallengeDefinition { challenge_mode: GameMode::PuzzleIZombie8, challenge_icon_index: 11, page: ChallengePage::Puzzle, row: 3, col: 2, challenge_name: Some("[I_ZOMBIE_8]") },
        ChallengeDefinition { challenge_mode: GameMode::PuzzleIZombie9, challenge_icon_index: 11, page: ChallengePage::Puzzle, row: 3, col: 3, challenge_name: Some("[I_ZOMBIE_9]") },
        ChallengeDefinition { challenge_mode: GameMode::PuzzleIZombieEndless, challenge_icon_index: 11, page: ChallengePage::Puzzle, row: 3, col: 4, challenge_name: Some("[I_ZOMBIE_ENDLESS]") },
        ChallengeDefinition { challenge_mode: GameMode::Upsell, challenge_icon_index: 10, page: ChallengePage::Limbo, row: 3, col: 4, challenge_name: Some("Upsell") },
        ChallengeDefinition { challenge_mode: GameMode::Intro, challenge_icon_index: 10, page: ChallengePage::Limbo, row: 2, col: 3, challenge_name: Some("Intro") },
    ];
    if challenge_mode < 0 || challenge_mode as usize >= G_CHALLENGE_DEFS.len() {
        return None;
    }
    Some(&G_CHALLENGE_DEFS[challenge_mode as usize])
}
