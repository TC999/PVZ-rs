// PvZ Portable Rust 翻译 — LawnDialog（草坪对话框）
// 对应 C++ src/Lawn/Widget/LawnDialog.h / LawnDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::dialog::Dialog;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::KeyCode;
use crate::framework::widget::dialog_button::DialogButton;
use crate::lawn::game_enums::*;
use crate::todlib::reanimator::Reanimation;

/// 对话框头部偏移常量
pub const DIALOG_HEADER_OFFSET: i32 = 45;

/// 重动画小部件（对应 C++ ReanimationWidget）
pub struct ReanimationWidget {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub reanim: Option<*mut Reanimation>,
    pub lawn_dialog: Option<*mut LawnDialog>,
    pub pos_x: f32,
    pub pos_y: f32,
}

impl ReanimationWidget {
    pub fn new() -> Self {
        ReanimationWidget {
            app: None,
            reanim: None,
            lawn_dialog: None,
            pos_x: 0.0,
            pos_y: 0.0,
        }
    }

    pub fn dispose(&mut self) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn draw(&self, _g: &mut Graphics) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn update(&mut self) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn add_reanimation(&mut self, x: f32, y: f32, reanimation_type: ReanimationType) {
        // TODO: 从 LawnDialog.cpp 翻译
    }
}

impl Default for ReanimationWidget {
    fn default() -> Self {
        ReanimationWidget::new()
    }
}

/// 草坪对话框（对应 C++ LawnDialog）
pub struct LawnDialog {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub button_delay: i32,
    pub reanimation: Option<*mut ReanimationWidget>,
    pub draw_standard_back: bool,
    pub lawn_yes_button: Option<*mut DialogButton>,
    pub lawn_no_button: Option<*mut DialogButton>,
    pub tall_bottom: bool,
    pub vertical_center_text: bool,
}

impl LawnDialog {
    pub fn new() -> Self {
        LawnDialog {
            app: None,
            button_delay: 0,
            reanimation: None,
            draw_standard_back: true,
            lawn_yes_button: None,
            lawn_no_button: None,
            tall_bottom: false,
            vertical_center_text: false,
        }
    }

    pub fn get_left(&self) -> i32 {
        // TODO: 从 LawnDialog.cpp 翻译
        0
    }

    pub fn get_width(&self) -> i32 {
        // TODO: 从 LawnDialog.cpp 翻译
        0
    }

    pub fn get_top(&self) -> i32 {
        // TODO: 从 LawnDialog.cpp 翻译
        0
    }

    pub fn set_button_delay(&mut self, delay: i32) {
        self.button_delay = delay;
    }

    pub fn update(&mut self) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn button_press(&mut self, _id: i32) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn button_depress(&mut self, _id: i32) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn checkbox_checked(&mut self) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn key_down(&mut self, _key: KeyCode) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn added_to_manager(&mut self, _manager: &mut WidgetManager) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn removed_from_manager(&mut self, _manager: &mut WidgetManager) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn resize(&mut self, _x: i32, _y: i32, _width: i32, _height: i32) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn draw(&self, _g: &mut Graphics) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn calc_size(&mut self, _extra_x: i32, _extra_y: i32) {
        // TODO: 从 LawnDialog.cpp 翻译
    }
}

impl Default for LawnDialog {
    fn default() -> Self {
        LawnDialog::new()
    }
}

/// 游戏结束对话框（对应 C++ GameOverDialog）
pub struct GameOverDialog {
    pub menu_button: Option<*mut DialogButton>,
}

impl GameOverDialog {
    pub fn new() -> Self {
        GameOverDialog {
            menu_button: None,
        }
    }

    pub fn button_depress(&mut self, _id: i32) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn added_to_manager(&mut self, _manager: &mut WidgetManager) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn removed_from_manager(&mut self, _manager: &mut WidgetManager) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn mouse_drag(&mut self, _x: i32, _y: i32) {
        // TODO: 从 LawnDialog.cpp 翻译
    }
}

impl Default for GameOverDialog {
    fn default() -> Self {
        GameOverDialog::new()
    }
}
