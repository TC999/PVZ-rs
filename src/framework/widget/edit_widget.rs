// PvZ Portable Rust 翻译 — EditWidget 文本输入框
// 对应 C++ SexyAppFramework/widget/EditWidget.h / EditWidget.cpp

#![allow(dead_code)]

use std::cmp;
use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::point::Point;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::widget::insets::Insets;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::{KeyCode, KEYCODE_SHIFT, KEYCODE_CONTROL, KEYCODE_LEFT, KEYCODE_RIGHT, KEYCODE_BACKSPACE, KEYCODE_DELETE, KEYCODE_HOME, KEYCODE_END, KEYCODE_RETURN, KEYCODE_UNKNOWN, KEYCODE_DOWN};

/// 编辑框监听器 trait（对应 C++ EditListener）
pub trait EditListener {
    fn edit_widget_text(&mut self, id: i32, text: &str);
    fn allow_char(&mut self, id: i32, c: u8) -> bool;
}

pub const EDIT_COLOR_BKG: usize = 0;
pub const EDIT_COLOR_OUTLINE: usize = 1;
pub const EDIT_COLOR_TEXT: usize = 2;
pub const EDIT_COLOR_HILITE: usize = 3;
pub const EDIT_COLOR_HILITE_TEXT: usize = 4;
pub const EDIT_NUM_COLORS: usize = 5;

const G_EDIT_COLORS: [[u8; 3]; EDIT_NUM_COLORS] = [
    [255,255,255], [0,0,0], [0,0,0], [0,0,0], [255,255,255],
];

/// 宽度检查结构
pub(crate) struct WidthCheck {
    font: Font,
    max_width: i32,
}

/// 编辑框控件（对应 C++ EditWidget）
pub struct EditWidget {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub widget_manager: Option<*mut WidgetManager>,
    pub visible: bool, pub disabled: bool,
    pub has_focus: bool, pub is_down: bool, pub is_over: bool,
    pub has_alpha: bool, pub has_transparencies: bool,
    pub colors: Vec<Color>,
    pub mouse_insets: Insets,
    pub do_finger: bool,
    pub wants_focus: bool,

    pub id: i32,
    pub text: String,
    pub password_display: String,
    pub font: Option<Box<Font>>,
    pub(crate) width_check_list: Vec<WidthCheck>,
    pub listener: Option<Box<dyn EditListener>>,
    pub showing_cursor: bool,
    pub draw_sel_override: bool,
    pub had_double_click: bool,
    pub cursor_pos: i32,
    pub hilite_pos: i32,
    pub blink_acc: i32,
    pub blink_delay: i32,
    pub left_pos: i32,
    pub max_chars: i32,
    pub max_pixels: i32,
    pub password_char: u8,
    pub undo_string: String,
    pub undo_cursor: i32,
    pub undo_hilite_pos: i32,
    pub last_modify_idx: i32,
}

impl EditWidget {
    pub fn new(the_id: i32, the_listener: Option<Box<dyn EditListener>>) -> Self {
        let mut ew = EditWidget {
            x: 0, y: 0, width: 0, height: 0,
            widget_manager: None,
            visible: true, disabled: false,
            has_focus: false, is_down: false, is_over: false,
            has_alpha: true, has_transparencies: false,
            colors: Vec::new(),
            mouse_insets: Insets::new(0,0,0,0),
            do_finger: false, wants_focus: true,
            id: the_id,
            text: String::new(),
            password_display: String::new(),
            font: None,
            width_check_list: Vec::new(),
            listener: the_listener,
            showing_cursor: false,
            draw_sel_override: false,
            had_double_click: false,
            cursor_pos: 0,
            hilite_pos: -1,
            blink_acc: 0,
            blink_delay: 40,
            left_pos: 0,
            max_chars: -1,
            max_pixels: -1,
            password_char: 0,
            undo_string: String::new(),
            undo_cursor: 0,
            undo_hilite_pos: 0,
            last_modify_idx: -1,
        };
        ew.colors.clear();
        for c in &G_EDIT_COLORS { ew.colors.push(Color::from_rgb(c[0], c[1], c[2])); }
        ew
    }

    pub fn get_display_string(&mut self) -> &str {
        if self.password_char == 0 { return &self.text; }
        if self.password_display.len() != self.text.len() {
            self.password_display = vec![self.password_char as char; self.text.len()].into_iter().collect();
        }
        &self.password_display
    }

    pub fn set_text(&mut self, the_text: &str, left_pos_to_zero: bool) {
        self.text = the_text.to_string();
        self.cursor_pos = self.text.len() as i32;
        self.hilite_pos = 0;
        if left_pos_to_zero { self.left_pos = 0; }
        else { self.focus_cursor(true); }
        self.mark_dirty_internal();
    }

    pub fn resize(&mut self, the_x: i32, the_y: i32, the_w: i32, the_h: i32) {
        self.x = the_x; self.y = the_y; self.width = the_w; self.height = the_h;
        self.focus_cursor(false);
    }

    pub fn set_font(&mut self, the_font: &Font) {
        self.font = Some(Box::new(Font::new(&the_font.name, the_font.size)));
        self.width_check_list.clear();
    }

    /// 清除宽度检查列表（对应 C++ ClearWidthCheckFonts）
    pub fn clear_width_check_fonts(&mut self) {
        self.width_check_list.clear();
    }

    /// 更新光标位置（对应 C++ UpdateCaretPos）
    /// 基于控件绝对位置约束光标在屏幕范围内
    pub fn update_caret_pos(&mut self) {
        if let Some(wm) = self.widget_manager {
            unsafe {
                if let Some(app_ptr) = (*wm).app {
                    let app = &*app_ptr;
                    let _clamp_x = if self.x < 10 { 10 } else if self.x > app.width - 10 { app.width - 10 } else { self.x };
                    let _clamp_y = if self.y < 10 { 10 } else if self.y > app.height - 10 { app.height - 10 } else { self.y };
                    // C++ 原版：SetCaretPos(clamp_x, clamp_y)
                }
            }
        }
    }

    pub fn draw(&mut self, g: &mut Graphics) {
        if self.font.is_none() {
            self.font = Some(Box::new(Font::new("DefaultInput", 10)));
        }

        let display = self.get_display_string().to_string();
        let font = self.font.as_ref().unwrap();

        // 背景
        g.set_color(&self.get_color(EDIT_COLOR_BKG));
        g.fill_rect_xywh(0, 0, self.width, self.height);

        // 两次绘制：第一次绘制普通文本，第二次绘制选中高亮
        for pass in 0..2 {
            let mut clip_g = g.create();

            if pass == 1 {
                let cursor_x = font.string_width(&display[..self.cursor_pos as usize])
                    - font.string_width(&display[..self.left_pos as usize]);
                let mut hilite_x = cursor_x + 2;
                if self.hilite_pos != -1 && self.cursor_pos != self.hilite_pos {
                    hilite_x = font.string_width(&display[..self.hilite_pos as usize])
                        - font.string_width(&display[..self.left_pos as usize]);
                }
                let c_x = if !self.showing_cursor { cursor_x + 2 } else { cursor_x };
                let cx = cmp::max(0, cmp::min(c_x, self.width - 8));
                let hx = cmp::max(0, cmp::min(hilite_x, self.width - 8));
                let sel_start = 4 + cmp::min(cx, hx);
                let sel_w = (hx - cx).abs();
                clip_g.clip_rect(&Rect::new(sel_start, (self.height - font.font_height)/2, sel_w, font.font_height));
            } else {
                clip_g.clip_rect(&Rect::new(4, 0, self.width - 8, self.height));
            }

            let has_focus = self.has_focus || self.draw_sel_override;
            if pass == 1 && has_focus {
                clip_g.set_color(&self.get_color(EDIT_COLOR_HILITE));
                clip_g.fill_rect_xywh(0, 0, self.width, self.height);
            }

            if pass == 0 || !has_focus {
                clip_g.set_color(&self.get_color(EDIT_COLOR_TEXT));
            } else {
                clip_g.set_color(&self.get_color(EDIT_COLOR_HILITE_TEXT));
            }
            clip_g.draw_string(
                &display[self.left_pos as usize..],
                4,
                (self.height - font.font_height) / 2 + font.get_ascent(),
            );
        }

        // 边框
        g.set_color(&self.get_color(EDIT_COLOR_OUTLINE));
        g.draw_rect(&Rect::new(0, 0, self.width - 1, self.height - 1));
    }

    pub fn got_focus(&mut self) {
        self.has_focus = true;
        self.showing_cursor = true;
        self.blink_acc = 0;
        self.mark_dirty_internal();
    }

    pub fn lost_focus(&mut self) {
        self.has_focus = false;
        self.showing_cursor = false;
        self.mark_dirty_internal();
    }

    pub fn update(&mut self) {
        if self.has_focus {
            self.blink_acc += 1;
            if self.blink_acc > self.blink_delay {
                self.mark_dirty_internal();
                self.blink_acc = 0;
                self.showing_cursor = !self.showing_cursor;
            }
        }
    }

    pub fn key_down(&mut self, the_key: KeyCode, wm: &WidgetManager) {
        let is_ctrl = wm.key_down.get(KEYCODE_CONTROL as usize).copied().unwrap_or(false);
        if the_key < KEYCODE_LEFT as KeyCode || the_key > KEYCODE_DOWN as KeyCode {
            self.process_key(the_key, 0, is_ctrl, wm);
        }
    }

    pub fn key_char(&mut self, the_char: u8, wm: &WidgetManager) {
        let is_ctrl = wm.key_down.get(KEYCODE_CONTROL as usize).copied().unwrap_or(false);
        self.process_key(KEYCODE_UNKNOWN, the_char, is_ctrl, wm);
    }

    fn process_key(&mut self, the_key: KeyCode, the_char: u8, control_down: bool, wm: &WidgetManager) {
        if the_key == KEYCODE_SHIFT as KeyCode || the_key == KEYCODE_CONTROL as KeyCode { return; }

        let shift_down = wm.key_down.get(KEYCODE_SHIFT as usize).copied().unwrap_or(false);
        let mut big_change = false;
        let mut remove_hilite = !shift_down;

        if shift_down && self.hilite_pos == -1 { self.hilite_pos = self.cursor_pos; }

        let old_text = self.text.clone();
        let old_cursor = self.cursor_pos;
        let old_hilite = self.hilite_pos;

        // Ctrl+C / Ctrl+X (copy/cut)
        if the_char == 3 || the_char == 24 {
            if self.hilite_pos != -1 && self.hilite_pos != self.cursor_pos {
                let (start, end) = if self.cursor_pos < self.hilite_pos { (self.cursor_pos, self.hilite_pos) } else { (self.hilite_pos, self.cursor_pos) };
                // copy handled outside
                if the_char == 24 { // cut
                    let range = start as usize..end as usize;
                    self.text.replace_range(range, "");
                    self.cursor_pos = start;
                    self.hilite_pos = -1;
                    big_change = true;
                } else {
                    remove_hilite = false;
                }
            }
        }
        // Ctrl+V (paste)
        else if the_char == 22 {
            // paste simplified
            if self.hilite_pos != -1 && self.hilite_pos != self.cursor_pos {
                let (start, end) = if self.cursor_pos < self.hilite_pos { (self.cursor_pos, self.hilite_pos) } else { (self.hilite_pos, self.cursor_pos) };
                let range = start as usize..end as usize;
                self.text.replace_range(range, "");
                self.cursor_pos = start;
                self.hilite_pos = -1;
            }
            big_change = true;
        }
        // Ctrl+Z (undo)
        else if the_char == 26 {
            self.last_modify_idx = -1;
            let swap = (self.text.clone(), self.cursor_pos, self.hilite_pos);
            self.text = self.undo_string.clone();
            self.cursor_pos = self.undo_cursor;
            self.hilite_pos = self.undo_hilite_pos;
            self.undo_string = swap.0;
            self.undo_cursor = swap.1;
            self.undo_hilite_pos = swap.2;
            remove_hilite = false;
        }
        else if the_key == KEYCODE_LEFT as KeyCode {
            if control_down {
                while self.cursor_pos > 0 && !self.is_part_of_word(self.text.as_bytes()[(self.cursor_pos-1) as usize]) { self.cursor_pos -= 1; }
                while self.cursor_pos > 0 && self.is_part_of_word(self.text.as_bytes()[(self.cursor_pos-1) as usize]) { self.cursor_pos -= 1; }
            } else if shift_down || self.hilite_pos == -1 {
                self.cursor_pos -= 1;
            } else {
                self.cursor_pos = cmp::min(self.cursor_pos, self.hilite_pos);
            }
        }
        else if the_key == KEYCODE_RIGHT as KeyCode {
            if control_down {
                let len = self.text.len() as i32;
                while self.cursor_pos < len-1 && self.is_part_of_word(self.text.as_bytes()[(self.cursor_pos+1) as usize]) { self.cursor_pos += 1; }
                while self.cursor_pos < len-1 && !self.is_part_of_word(self.text.as_bytes()[(self.cursor_pos+1) as usize]) { self.cursor_pos += 1; }
            }
            if shift_down || self.hilite_pos == -1 { self.cursor_pos += 1; }
            else { self.cursor_pos = cmp::max(self.cursor_pos, self.hilite_pos); }
        }
        else if the_key == KEYCODE_BACKSPACE as KeyCode {
            if !self.text.is_empty() {
                if self.hilite_pos != -1 && self.hilite_pos != self.cursor_pos {
                    let (s, e) = if self.cursor_pos < self.hilite_pos { (self.cursor_pos, self.hilite_pos) } else { (self.hilite_pos, self.cursor_pos) };
                    self.text.replace_range(s as usize..e as usize, "");
                    self.cursor_pos = s; self.hilite_pos = -1; big_change = true;
                } else if self.cursor_pos > 0 {
                    self.text.replace_range((self.cursor_pos-1) as usize..self.cursor_pos as usize, "");
                    self.cursor_pos -= 1; self.hilite_pos = -1;
                    if self.cursor_pos != self.last_modify_idx { big_change = true; }
                    self.last_modify_idx = self.cursor_pos - 1;
                }
            }
        }
        else if the_key == KEYCODE_DELETE as KeyCode {
            if !self.text.is_empty() {
                if self.hilite_pos != -1 && self.hilite_pos != self.cursor_pos {
                    let (s, e) = if self.cursor_pos < self.hilite_pos { (self.cursor_pos, self.hilite_pos) } else { (self.hilite_pos, self.cursor_pos) };
                    self.text.replace_range(s as usize..e as usize, "");
                    self.cursor_pos = s; self.hilite_pos = -1; big_change = true;
                } else if (self.cursor_pos as usize) < self.text.len() {
                    self.text.replace_range(self.cursor_pos as usize..(self.cursor_pos+1) as usize, "");
                    if self.cursor_pos != self.last_modify_idx { big_change = true; }
                    self.last_modify_idx = self.cursor_pos;
                }
            }
        }
        else if the_key == KEYCODE_HOME as KeyCode { self.cursor_pos = 0; }
        else if the_key == KEYCODE_END as KeyCode { self.cursor_pos = self.text.len() as i32; }
        else if the_key == KEYCODE_RETURN as KeyCode {
            if let Some(ref mut l) = self.listener { l.edit_widget_text(self.id, &self.text); }
        }
        else if (the_char as u32) >= 32 && (the_char as u32) <= 127 {
            let c = the_char as char;
            let s = c.to_string();
            if self.hilite_pos != -1 && self.hilite_pos != self.cursor_pos {
                let (start, end) = if self.cursor_pos < self.hilite_pos { (self.cursor_pos, self.hilite_pos) } else { (self.hilite_pos, self.cursor_pos) };
                self.text.replace_range(start as usize..end as usize, &s);
                self.cursor_pos = start; self.hilite_pos = -1; big_change = true;
            } else {
                self.text.insert(self.cursor_pos as usize, c);
                if self.cursor_pos != self.last_modify_idx + 1 { big_change = true; }
                self.last_modify_idx = self.cursor_pos; self.hilite_pos = -1;
            }
            self.cursor_pos += 1;
            self.focus_cursor(false);
        } else { remove_hilite = false; }

        // 约束长度
        if self.max_chars != -1 && (self.text.len() as i32) > self.max_chars {
            self.text.truncate(self.max_chars as usize);
        }
        self.enforce_max_pixels();

        if self.cursor_pos < 0 { self.cursor_pos = 0; }
        else if (self.cursor_pos as usize) > self.text.len() { self.cursor_pos = self.text.len() as i32; }

        if old_cursor != self.cursor_pos { self.blink_acc = 0; self.showing_cursor = true; }
        self.focus_cursor(true);
        if remove_hilite || self.hilite_pos == self.cursor_pos { self.hilite_pos = -1; }
        if big_change {
            self.undo_string = old_text;
            self.undo_cursor = old_cursor;
            self.undo_hilite_pos = old_hilite;
        }
        self.mark_dirty_internal();
    }

    fn is_part_of_word(&self, c: u8) -> bool {
        (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z') ||
        (c >= b'0' && c <= b'9') || c == b'_'
    }

    pub fn get_char_at(&self, x: i32, _y: i32) -> i32 {
        let display = if self.password_char == 0 { &self.text } else { &self.password_display };
        let font = match self.font.as_ref() { Some(f) => f, None => return 0 };
        let mut pos = 0i32;
        let s = &display[self.left_pos as usize..];
        for i in 0..s.len() {
            let low = font.string_width(&s[..i]);
            let high = font.string_width(&s[..=i]);
            if x >= (low + high) / 2 + 5 { pos = (i + 1) as i32; }
        }
        pos + self.left_pos
    }

    pub fn focus_cursor(&mut self, big_jump: bool) {
        while self.cursor_pos < self.left_pos {
            if big_jump { self.left_pos = cmp::max(0, self.left_pos - 10); }
            else { self.left_pos = cmp::max(0, self.left_pos - 1); }
            self.mark_dirty_internal();
        }
        let display_str = if self.password_char == 0 { self.text.clone() } else { self.password_display.clone() };
        // 克隆字体宽度函数所需数据，避免借用冲突
        let font_data = self.font.as_ref().map(|f| (f.name.clone(), f.size));
        if let Some((ref _fn, _fs)) = font_data {
            // 使用 Font::string_width 的简化替代：按字符数估算
            while self.width - 8 > 0 {
                let chars_before_cursor = self.cursor_pos as usize;
                let chars_before_left = self.left_pos as usize;
                // 使用字符数估算宽度（每个字符约 size * 0.6）
                let est_cw = ((chars_before_cursor - chars_before_left) as f64 * _fs as f64 * 0.6) as i32;
                if est_cw < self.width - 8 { break; }
                if big_jump { self.left_pos = cmp::min(self.left_pos + 10, self.text.len() as i32 - 1); }
                else { self.left_pos = cmp::min(self.left_pos + 1, self.text.len() as i32 - 1); }
                self.mark_dirty_internal();
            }
        }
    }

    pub fn mouse_down_btn(&mut self, x: i32, y: i32, _b: i32, click_count: i32) {
        self.hilite_pos = -1;
        self.cursor_pos = self.get_char_at(x, y);
        if click_count > 1 {
            self.had_double_click = true;
            self.hilite_word();
        }
        self.mark_dirty_internal();
        self.focus_cursor(false);
    }

    pub fn mouse_up_btn(&mut self, x: i32, y: i32, _b: i32, _c: i32) {
        if self.hilite_pos == self.cursor_pos { self.hilite_pos = -1; }
        if self.had_double_click {
            self.hilite_pos = -1;
            self.cursor_pos = self.get_char_at(x, y);
            self.had_double_click = false;
            self.hilite_word();
        }
        self.mark_dirty_internal();
    }

    pub fn mouse_drag(&mut self, x: i32, _y: i32) {
        if self.hilite_pos == -1 { self.hilite_pos = self.cursor_pos; }
        self.cursor_pos = self.get_char_at(x, 0);
        self.mark_dirty_internal();
        self.focus_cursor(false);
    }

    pub fn mouse_enter(&mut self) {}
    pub fn mouse_leave(&mut self) {}

    fn hilite_word(&mut self) {
        let display = if self.password_char == 0 { &self.text } else { &self.password_display };
        if (self.cursor_pos as usize) < display.len() {
            self.hilite_pos = self.cursor_pos;
            while self.hilite_pos > 0 && self.is_part_of_word(display.as_bytes()[(self.hilite_pos-1) as usize]) {
                self.hilite_pos -= 1;
            }
            while (self.cursor_pos as usize) < display.len() - 1 && self.is_part_of_word(display.as_bytes()[(self.cursor_pos+1) as usize]) {
                self.cursor_pos += 1;
            }
            if (self.cursor_pos as usize) < display.len() { self.cursor_pos += 1; }
        }
    }

    fn enforce_max_pixels(&mut self) {
        if self.max_pixels <= 0 && self.width_check_list.is_empty() { return; }
        if self.width_check_list.is_empty() {
            if let Some(ref font) = self.font {
                while font.string_width(&self.text) > self.max_pixels {
                    self.text.pop();
                }
            }
            return;
        }
        for check in &self.width_check_list {
            let mut w = check.max_width;
            if w <= 0 { w = self.max_pixels; if w <= 0 { continue; } }
            while check.font.string_width(&self.text) > w { self.text.pop(); }
        }
    }

    fn get_color(&self, idx: usize) -> Color { self.colors.get(idx).copied().unwrap_or(Color::WHITE) }
    fn mark_dirty_internal(&mut self) {}
}
