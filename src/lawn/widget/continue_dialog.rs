// PvZ Portable Rust 翻译 — ContinueDialog（继续游戏对话框）
// 对应 C++ src/Lawn/Widget/ContinueDialog.h / ContinueDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::game_enums::GameMode;
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
    pub fn key_down(&mut self, _key: i32) {}
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32) {}
    pub fn added_to_manager(&mut self, _manager: &mut WidgetManager) {}
    pub fn removed_from_manager(&mut self, _manager: &mut WidgetManager) {}
    pub fn button_depress(&mut self, _id: i32) {}
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
