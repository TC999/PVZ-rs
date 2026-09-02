// PvZ Portable Rust 翻译 — NewUserDialog（新建用户对话框）
// 对应 C++ src/Lawn/Widget/NewUserDialog.h / NewUserDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::lawn_app::LawnApp;

/// 编辑框部件（简化版，对应 C++ EditWidget）
pub struct EditWidget {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

/// 新建用户对话框
pub struct NewUserDialog {
    pub app: Option<*mut LawnApp>,
    pub name_edit_widget: Option<*mut EditWidget>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

impl NewUserDialog {
    pub fn new(app: Option<*mut LawnApp>) -> Self {
        NewUserDialog {
            app,
            name_edit_widget: None,
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
    pub fn edit_widget_text(&mut self, _id: i32, _text: &str) {
        // [TRANSLATION_NOTE]: C++ 中 mApp->ButtonDepress(mId + 2000)；Rust 侧 LawnApp 无 button_depress
    }
    pub fn allow_char(&self, _id: i32, _ch: char) -> bool {
        // 对应 C++ AllowChar：仅允许字母数字与空格
        _ch.is_alphanumeric() || _ch == ' '
    }

    /// 获取净化后的用户名（对应 C++ GetName：压缩连续空格并去尾空格）
    pub fn get_name(&self) -> String {
        let the_string = self.name_edit_widget.map_or(String::new(), |pw| unsafe { (*pw).text.clone() });
        let mut a_string = String::new();
        let mut a_last_char = ' ';
        for a_char in the_string.chars() {
            if a_char != ' ' {
                a_string.push(a_char);
            } else if a_char != a_last_char {
                a_string.push(' ');
            }
            a_last_char = a_char;
        }
        if a_string.ends_with(' ') {
            a_string.pop();
        }
        a_string
    }

    pub fn set_name(&mut self, the_name: &str) {
        // 对应 C++ SetName
        if let Some(pw) = self.name_edit_widget {
            unsafe {
                (*pw).text = the_name.to_string();
            }
        }
    }
    pub fn get_preferred_height(&self, _width: i32) -> i32 { 0 }
}
