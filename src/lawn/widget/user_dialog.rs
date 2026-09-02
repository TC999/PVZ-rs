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
    /// 列表行文本（对应 C++ 行数据）
    pub lines: Vec<String>,
    /// 当前选中行索引（对应 mSelectIdx）
    pub select_index: i32,
}

impl ListWidget {
    pub fn new() -> Self {
        ListWidget {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            visible: true,
            lines: Vec::new(),
            select_index: 0,
        }
    }

    /// 移除一行（对应 C++ RemoveLine）
    pub fn remove_line(&mut self, index: i32) {
        if index >= 0 && (index as usize) < self.lines.len() {
            self.lines.remove(index as usize);
        }
    }

    /// 设置选中行（对应 C++ SetSelect）
    pub fn set_select(&mut self, index: i32) {
        self.select_index = index.clamp(0, (self.lines.len() as i32).saturating_sub(1));
    }

    /// 获取行数（对应 C++ GetLineCount）
    pub fn get_line_count(&self) -> i32 {
        self.lines.len() as i32
    }

    /// 添加一行（对应 C++ AddLine）
    pub fn add_line(&mut self, text: &str) {
        self.lines.push(text.to_string());
    }

    /// 设置行文本（对应 C++ SetLine）
    pub fn set_line(&mut self, index: i32, text: &str) {
        if index >= 0 && (index as usize) < self.lines.len() {
            self.lines[index as usize] = text.to_string();
        }
    }
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
    pub fn finish_delete_user(&mut self) {
        // 对应 C++ FinishDeleteUser：删除选中用户行并调整选中
        if let Some(list) = self.user_list {
            unsafe {
                let a_sel_idx = (*list).select_index;
                (*list).remove_line(a_sel_idx);

                let a_sel_idx = (a_sel_idx - 1).max(0);
                if (*list).get_line_count() > 0 {
                    (*list).set_select(a_sel_idx);
                }

                self.num_users -= 1;
                if self.num_users == 7 {
                    (*list).add_line("(Create a New User)");
                }
            }
        }
    }

    pub fn finish_rename_user(&mut self, new_name: &str) {
        // 对应 C++ FinishRenameUser：重命名选中用户行
        if let Some(list) = self.user_list {
            unsafe {
                if (*list).select_index < self.num_users {
                    (*list).set_line((*list).select_index, new_name);
                }
            }
        }
    }
    pub fn get_sel_name(&self) -> String { String::new() }
    pub fn get_preferred_height(&self, _width: i32) -> i32 { 0 }
}
