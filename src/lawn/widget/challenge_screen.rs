// PvZ Portable Rust 翻译 — ChallengeScreen（挑战模式选择界面）
// 对应 C++ src/Lawn/Widget/ChallengeScreen.h / ChallengeScreen.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
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

    pub fn draw_button(&self, _g: &mut Graphics, _challenge_index: i32) {
        // TODO: 从 ChallengeScreen.cpp 翻译
    }

    pub fn draw(&self, _g: &mut Graphics) {
        // TODO: 从 ChallengeScreen.cpp 翻译
    }

    pub fn update(&mut self) {
        // TODO: 从 ChallengeScreen.cpp 翻译
    }

    pub fn added_to_manager(&mut self, _manager: &mut WidgetManager) {
        // TODO: 从 ChallengeScreen.cpp 翻译
    }

    pub fn removed_from_manager(&mut self, _manager: &mut WidgetManager) {
        // TODO: 从 ChallengeScreen.cpp 翻译
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
        // TODO: 从 ChallengeScreen.cpp 翻译
    }

    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // TODO: 从 ChallengeScreen.cpp 翻译
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
    // TODO: 从 ChallengeScreen.cpp 翻译
    None
}
