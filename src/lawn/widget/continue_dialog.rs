// PvZ Portable Rust 翻译 — ContinueDialog（继续游戏对话框）
// 对应 C++ src/Lawn/Widget/ContinueDialog.h / ContinueDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::game_enums::{BoardResult, ChallengePage, Dialogs, GameMode};
use crate::todlib::tod_foley::FoleyType;

pub const CONTINUE_DIALOG_CONTINUE: i32 = 0;
pub const CONTINUE_DIALOG_NEW_GAME: i32 = 1;

/// 继续游戏对话框 — 选择继续或开新游戏
pub struct ContinueDialog {
    pub app: Option<*mut LawnApp>,
    pub continue_button: Option<*mut DialogButton>,
    pub new_game_button: Option<*mut DialogButton>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

impl ContinueDialog {
    pub fn new(app: Option<*mut LawnApp>) -> Self {
        ContinueDialog {
            app,
            continue_button: None,
            new_game_button: None,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            visible: true,
        }
    }

    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.x = x;
        self.y = y;
        self.width = w;
        self.height = h;
    }

    pub fn draw(&self, _g: &mut Graphics) {}
    pub fn update(&mut self) {}

    /// 按键处理（对应 C++ KeyDown）
    pub fn key_down(&mut self, key: i32) {
        if key == crate::framework::key_codes::KEYCODE_ESCAPE {
            self.button_depress(crate::framework::widget::dialog::ID_FOOTER);
            return;
        }
        if key == crate::framework::key_codes::KEYCODE_RETURN
            || key == crate::framework::key_codes::KEYCODE_SPACE
        {
            self.button_depress(CONTINUE_DIALOG_CONTINUE);
            return;
        }
        // C++ 中转发 LawnDialog::KeyDown
    }

    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32) {}
    pub fn added_to_manager(&mut self, _manager: &mut WidgetManager) {
        // C++ 中 AddWidget(mContinueButton/mNewGameButton)
    }
    pub fn removed_from_manager(&mut self, _manager: &mut WidgetManager) {
        // C++ 中 RemoveWidget(mContinueButton/mNewGameButton)
    }

    /// 按钮点击（对应 C++ ButtonDepress）
    pub fn button_depress(&mut self, the_id: i32) {
        let Some(app) = self.app else { return };
        unsafe {
            if the_id == CONTINUE_DIALOG_CONTINUE {
                self.restart_looping_sounds();
                (*app).kill_dialog(Dialogs::Continue);
            } else if the_id == CONTINUE_DIALOG_NEW_GAME {
                if (*app).is_adventure_mode() {
                    let _ = (*app).do_dialog(
                        Dialogs::RestartConfirm as i32,
                        true,
                        "[RESTART_LEVEL_HEADER]",
                        "[RESTART_LEVEL_BODY]",
                        "",
                        crate::framework::widget::dialog::BUTTONS_OK_CANCEL,
                    );
                } else {
                    let _ = (*app).do_dialog(
                        Dialogs::RestartConfirm as i32,
                        true,
                        "New Game?",
                        "Are you sure that you want to start a new game?",
                        "",
                        crate::framework::widget::dialog::BUTTONS_OK_CANCEL,
                    );
                }
            } else {
                (*app).kill_dialog(Dialogs::Continue);
                (*app).board_result = BoardResult::Quit;
                if (*app).is_adventure_mode() {
                    (*app).show_game_selector();
                } else if (*app).is_survival_mode() {
                    (*app).kill_board();
                    (*app).show_challenge_screen(ChallengePage::Survival as i32);
                } else if (*app).is_puzzle_mode() {
                    (*app).kill_board();
                    (*app).show_challenge_screen(ChallengePage::Puzzle as i32);
                } else {
                    (*app).kill_board();
                    (*app).show_challenge_screen(ChallengePage::Challenge as i32);
                }
            }
        }
    }

    pub fn get_preferred_height(&self, _width: i32) -> i32 { 0 }

    /// 重新启动循环音效（对应 C++ RestartLoopingSounds）
    /// 在继续游戏时恢复雨声和僵尸音乐
    pub fn restart_looping_sounds(&mut self) {
        if let Some(app) = self.app {
            unsafe {
                let app = &mut *app;
                if app.game_mode == GameMode::ChallengeRainingSeeds || app.is_stormy_night_level() {
                    app.play_foley(FoleyType::Rain as i32);
                }
                if let Some(board) = app.board.as_mut() {
                    for zombie in &mut (**board).zombies {
                        if zombie.playing_song {
                            zombie.start_zombie_sound();
                        }
                    }
                }
            }
        }
    }
}
