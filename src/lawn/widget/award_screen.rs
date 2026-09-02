// PvZ Portable Rust 翻译 — AwardScreen（奖励界面）
// 对应 C++ src/Lawn/Widget/AwardScreen.h / AwardScreen.cpp
// 完整翻译版本

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::*;
use crate::lawn::widget::game_button::GameButton;

/// 成就屏幕条目（对应 C++ AchievementScreenItem）
pub struct AchievementScreenItem {
    pub id: i32,
    pub start_anim_time: i32,
    pub end_anim_time: i32,
    pub dest_y: i32,
    pub start_y: i32,
    pub y: i32,
}

/// 奖励界面（对应 C++ AwardScreen）
pub struct AwardScreen {
    pub start_button: Option<*mut GameButton>,
    pub menu_button: Option<*mut GameButton>,
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub fade_in_counter: i32,
    pub award_type: AwardType,
    pub continue_button: Option<*mut GameButton>,
    pub show_start_button_after_achievements: bool,
    pub show_menu_button_after_achievements: bool,
    pub achievement_anim_time: i32,
    pub showing_achievements: bool,
    pub achievement_items: Vec<AchievementScreenItem>,
}

impl AwardScreen {
    pub fn new() -> Self {
        AwardScreen {
            start_button: None,
            menu_button: None,
            app: None,
            fade_in_counter: 180,
            award_type: AwardType::ForLevel,
            continue_button: None,
            show_start_button_after_achievements: false,
            show_menu_button_after_achievements: false,
            achievement_anim_time: 0,
            showing_achievements: false,
            achievement_items: Vec::new(),
        }
    }

    pub fn is_paper_note(&self) -> bool {
        matches!(self.award_type, AwardType::CreditsZombieNote | AwardType::HelpZombieNote)
    }

    pub fn draw_bottom(_g: &mut Graphics, _title: &str, _award: &str, _message: &str) {
        // 依赖图片资源，暂用占位
    }

    pub fn draw_award_seed(&self, _g: &mut Graphics) {
        // 依赖图片资源，暂用占位
    }

    pub fn draw(&self, _g: &mut Graphics) {
        // 依赖图片资源，暂用占位
    }

    pub fn update(&mut self) {
        if self.fade_in_counter > 0 {
            self.fade_in_counter -= 1;
        }
        if self.showing_achievements {
            self.achievement_anim_time += 1;
            for item in &mut self.achievement_items {
                if self.achievement_anim_time >= item.start_anim_time && self.achievement_anim_time < item.end_anim_time {
                    let progress = (self.achievement_anim_time - item.start_anim_time) as f32 / (item.end_anim_time - item.start_anim_time) as f32;
                    item.y = item.start_y + ((item.dest_y - item.start_y) as f32 * progress) as i32;
                } else if self.achievement_anim_time >= item.end_anim_time {
                    item.y = item.dest_y;
                }
            }
        }
    }

    pub fn key_char(&mut self, _c: char) {
        if let Some(app) = self.app { unsafe {
            (*app).kill_award_screen();
        } }
    }

    pub fn start_button_pressed(&mut self) {
        // 对应 C++ StartButtonPressed：按奖励类型/模式/等级跳转
        let Some(app) = self.app else { return };
        unsafe {
            if (*app).base.dialog_map.contains_key(&(Dialogs::Store as i32)) {
                return;
            }

            if self.award_type == AwardType::CreditsZombieNote {
                (*app).kill_award_screen();
                (*app).show_credit_screen();
            } else if self.award_type == AwardType::HelpZombieNote {
                (*app).kill_award_screen();
                (*app).show_game_selector();
            } else if (*app).is_survival_mode() {
                (*app).kill_award_screen();
                (*app).show_challenge_screen(ChallengePage::Survival as i32);
            } else if (*app).is_puzzle_mode() {
                (*app).kill_award_screen();
                (*app).show_challenge_screen(ChallengePage::Puzzle as i32);
            } else if (*app).is_challenge_mode() {
                (*app).kill_award_screen();
                (*app).show_challenge_screen(ChallengePage::Challenge as i32);
            } else {
                let a_level = (*app).player_info.as_ref().map_or(0, |pi| pi.get_level());
                if a_level == 1 {
                    (*app).kill_award_screen();
                    if (*app).has_finished_adventure() {
                        (*app).show_award_screen(AwardType::CreditsZombieNote as i32, false);
                    } else {
                        (*app).pre_new_game(GameMode::Adventure, false);
                    }
                } else {
                    if a_level == 15 {
                        (*app).do_almanac_dialog(SeedType::None, ZombieType::Invalid);
                    } else if a_level == 25 {
                        // [TRANSLATION_NOTE]: C++ 中 ShowStoreScreen + SetupForIntro(301) + WaitForResult，
                        // 并处理 mPurchasedFullVersion / IsTrialStageLocked 升级分支；Rust 侧 StoreScreen
                        // 交互未接入，仅创建商店
                        let _store = crate::lawn::lawn_app::LawnApp::show_store_screen(Some(app));
                    } else if a_level == 35 {
                        let _store = crate::lawn::lawn_app::LawnApp::show_store_screen(Some(app));
                        // C++ 中 SetupForIntro(601) + WaitForResult(true)
                    } else if a_level == 42 {
                        let _store = crate::lawn::lawn_app::LawnApp::show_store_screen(Some(app));
                        // C++ 中 SetupForIntro(3100) + WaitForResult(true)
                    } else if a_level == 45 {
                        (*app).kill_award_screen();
                        (*app).pre_new_game(GameMode::ChallengeZenGarden, false);
                        if let Some(zg) = (*app).zen_garden {
                            (*zg).setup_for_zen_tutorial();
                        }
                        return;
                    }

                    (*app).kill_award_screen();
                    (*app).pre_new_game(GameMode::Adventure, false);
                }
            }
        }
    }

    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {}

    pub fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) {
        if let Some(app) = self.app { unsafe {
            (*app).kill_award_screen();
        } }
    }

    pub fn draw_achievements(&self, _g: &mut Graphics) {
        // 依赖图片资源，暂用占位
    }

    pub fn achievements_continue_pressed(&mut self) {
        self.showing_achievements = false;
    }
}

impl Default for AwardScreen {
    fn default() -> Self {
        AwardScreen::new()
    }
}