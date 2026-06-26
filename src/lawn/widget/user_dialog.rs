// PvZ Portable Rust 翻译 — UserDialog（用户管理对话框）
// 对应 C++ src/Lawn/Widget/UserDialog.h / UserDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::lawn_app::LawnApp;

pub const USER_DIALOG_RENAME_USER: i32 = 0;
pub const USER_DIALOG_DELETE_USER: i32 = 1;

/// 列表部件（简化版，对应 C++ ListWidget）
pub struct ListWidget {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

/// 用户管理对话框 — 重命名/删除用户
pub struct UserDialog {
    pub app: Option<*mut LawnApp>,
    pub user_list: Option<*mut ListWidget>,
    pub rename_button: Option<*mut DialogButton>,
    pub delete_button: Option<*mut DialogButton>,
    pub num_users: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

impl UserDialog {
    pub fn new(app: Option<*mut LawnApp>) -> Self {
        UserDialog {
            app,
            user_list: None,
            rename_button: None,
            delete_button: None,
            num_users: 0,
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
    pub fn list_clicked(&mut self, _id: i32, _idx: i32, _click_count: i32) {}
    pub fn button_depress(&mut self, _id: i32) {}
    pub fn edit_widget_text(&mut self, _id: i32, _text: &str) {}
    pub fn allow_char(&self, _id: i32, _ch: char) -> bool { true }
    pub fn finish_delete_user(&mut self) {}
    pub fn finish_rename_user(&mut self, _new_name: &str) {}
    pub fn get_sel_name(&self) -> String { String::new() }
    pub fn get_preferred_height(&self, _width: i32) -> i32 { 0 }
}
