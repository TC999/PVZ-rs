// PvZ Portable Rust 翻译 — CheatDialog（作弊对话框）
// 对应 C++ src/Lawn/Widget/CheatDialog.h / CheatDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::game_enums::GameMode;

/// 作弊对话框 — 输入关卡编号直接跳关
pub struct CheatDialog {
    pub app: Option<*mut LawnApp>,
    pub level_edit_widget: Option<*mut EditWidget>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

/// 编辑框部件（简化版，对应 C++ EditWidget）
pub struct EditWidget {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

impl CheatDialog {
    pub fn new(app: Option<*mut LawnApp>) -> Self {
        CheatDialog {
            app,
            level_edit_widget: None,
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

    /// 对应 C++ CheatDialog::Draw（CheatDialog.cpp 87-89：LawnDialog::Draw + DrawEditBox）
    pub fn draw(&mut self, g: &mut Graphics) {
        // [TRANSLATION_NOTE]: C++ 中 LawnDialog::Draw 绘制背景/标题/按钮；Rust 端 CheatDialog
        // 无父类组合，仅补编辑框绘制（对应 DrawEditBox(mLevelEditWidget)）
        if let Some(w) = self.level_edit_widget {
            unsafe {
                let w = &*w;
                if w.visible {
                    g.set_color(&crate::framework::color::Color::WHITE);
                    g.fill_rect_xywh(w.x, w.y, w.width, w.height);
                    if !w.text.is_empty() {
                        let mut a_font = crate::framework::graphics::font::Font::new("Briannetod", 12);
                        a_font.ascent = 13;
                        a_font.font_height = 12;
                        g.set_font(&mut a_font as *mut crate::framework::graphics::font::Font);
                        g.set_color(&crate::framework::color::Color::BLACK);
                        g.draw_string(&w.text, w.x + 4, w.y + 4);
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {}

    pub fn key_down(&mut self, _key: i32) {}

    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32) {}

    pub fn added_to_manager(&mut self, _manager: &mut WidgetManager) {}

    pub fn removed_from_manager(&mut self, _manager: &mut WidgetManager) {}

    pub fn edit_widget_text(&mut self, _id: i32, _text: &str) {
        // [TRANSLATION_NOTE]: C++ 中 mApp->ButtonDepress(mId + 2000)；Rust 侧 LawnApp 无 button_depress
    }

    pub fn allow_char(&self, _id: i32, _ch: char) -> bool {
        // 对应 C++ AllowChar：仅允许数字、'-'、'c'、'f'（含大写）
        _ch.is_ascii_digit() || _ch == '-' || _ch == 'c' || _ch == 'C' || _ch == 'f' || _ch == 'F'
    }

    pub fn apply_cheat(&mut self) -> bool {
        // 对应 C++ ApplyCheat（sscanf 解析：c%d / f%d-%d / f%d / %d-%d / %d）
        let Some(app) = self.app else { return false };
        let input = self.level_edit_widget.map_or(String::new(), |pw| unsafe { (*pw).text.clone() });
        let input = input.trim();

        // "c%d" / "C%d"：挑战模式
        if let Some(nums) = input.strip_prefix('c').or_else(|| input.strip_prefix('C')) {
            if let Ok(a_challenge_index) = nums.trim().parse::<i32>() {
                unsafe {
                    // [TRANSLATION_NOTE]: NUM_CHALLENGE_MODES = NUM_GAME_MODES - 1（GameMode 共 73 种）
                    let clamped = a_challenge_index.clamp(0, 72);
                    (*app).game_mode = std::mem::transmute::<i32, GameMode>(clamped);
                }
                return true;
            }
        }

        let mut a_level = -1i32;
        let mut a_finished_adventure = 0i32;
        if let Some(rest) = input.strip_prefix('f').or_else(|| input.strip_prefix('F')) {
            if let Some((area_s, sub_s)) = rest.split_once('-') {
                if let (Ok(area), Ok(sub)) = (area_s.trim().parse::<i32>(), sub_s.trim().parse::<i32>()) {
                    a_level = (area - 1) * 10 + sub;
                    a_finished_adventure = 1;
                }
            } else if let Ok(level) = rest.trim().parse::<i32>() {
                a_level = level;
                a_finished_adventure = 1;
            }
        } else if let Some((area_s, sub_s)) = input.split_once('-') {
            if let (Ok(area), Ok(sub)) = (area_s.trim().parse::<i32>(), sub_s.trim().parse::<i32>()) {
                a_level = (area - 1) * 10 + sub;
            }
        } else if let Ok(level) = input.parse::<i32>() {
            a_level = level;
        }

        unsafe {
            if a_level <= 0 {
                (*app).do_dialog(
                    crate::lawn::game_enums::Dialogs::CheatError as i32,
                    true,
                    "Enter Level",
                    "Invalid Level. Do 'number' or 'area-subarea' or 'Cnumber' or 'Farea-subarea'.",
                    "[DIALOG_BUTTON_OK]",
                    crate::framework::widget::dialog::BUTTONS_FOOTER,
                );
                return false;
            }

            (*app).game_mode = GameMode::Adventure;
            if let Some(pi) = (*app).player_info.as_mut() {
                pi.set_level(a_level);
                pi.m_finished_adventure = a_finished_adventure;
            }
            (*app).write_current_user_config();
        }
        true
    }

    pub fn get_preferred_height(&self, _width: i32) -> i32 {
        0
    }
}
