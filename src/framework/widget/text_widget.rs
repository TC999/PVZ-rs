// PvZ Portable Rust 翻译 — TextWidget 文本显示控件
// 对应 C++ SexyAppFramework/widget/TextWidget.h / TextWidget.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::insets::Insets;
use crate::framework::widget::scrollbar_widget::{ScrollbarWidget, ScrollListener};
use crate::framework::key_codes::KeyCode;

/// 文本控件（对应 C++ TextWidget，继承 Widget 和 ScrollListener）
pub struct TextWidget {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub widget_manager: Option<*mut WidgetManager>,
    pub visible: bool, pub disabled: bool,
    pub has_focus: bool, pub is_down: bool, pub is_over: bool,
    pub colors: Vec<Color>,
    pub mouse_insets: Insets,
    pub font: Option<Box<Font>>,
    pub scrollbar: Option<Box<ScrollbarWidget>>,
    pub logical_lines: Vec<String>,
    pub physical_lines: Vec<String>,
    pub line_map: Vec<i32>,
    pub position: f64,
    pub page_size: f64,
    pub stick_to_bottom: bool,
    pub hilite_area: [[i32; 2]; 2],
    pub max_lines: i32,
}

impl TextWidget {
    pub fn new() -> Self {
        TextWidget {
            x: 0, y: 0, width: 0, height: 0,
            widget_manager: None,
            visible: true, disabled: false,
            has_focus: false, is_down: false, is_over: false,
            colors: Vec::new(),
            mouse_insets: Insets::new(0,0,0,0),
            font: None,
            scrollbar: None,
            logical_lines: Vec::new(),
            physical_lines: Vec::new(),
            line_map: Vec::new(),
            position: 0.0,
            page_size: 0.0,
            stick_to_bottom: true,
            hilite_area: [[0, 0], [0, 0]],
            max_lines: 2048,
        }
    }

    pub fn get_lines(&self) -> &[String] { &self.logical_lines }
    pub fn set_lines(&mut self, lines: Vec<String>) { self.logical_lines = lines; }

    pub fn clear(&mut self) {
        self.logical_lines.clear();
        self.physical_lines.clear();
        self.position = 0.0;
        if let Some(ref mut sb) = self.scrollbar { sb.set_max_value(0.0); }
    }

    /// 绘制带颜色的字符串（0x00 后跟 RGB 作为颜色标记）
    pub fn draw_color_string(&self, g: &mut Graphics, s: &str, x: i32, y: i32, use_colors: bool) {
        let mut cur_x = x;
        let mut cur_str = String::new();
        let bytes = s.as_bytes();
        let mut i = 0;
        if use_colors { g.set_color(&Color::new(0, 0, 0, 255)); }

        while i < bytes.len() {
            if bytes[i] == 0x00 && i + 3 < bytes.len() {
                if !cur_str.is_empty() {
                    g.draw_string(&cur_str, cur_x, y);
                    if let Some(f) = &self.font { cur_x += f.string_width(&cur_str); }
                }
                cur_str.clear();
                if use_colors {
                    g.set_color(&Color::new(bytes[i+1], bytes[i+2], bytes[i+3], 255));
                }
                i += 4;
            } else {
                cur_str.push(bytes[i] as char);
                i += 1;
            }
        }
        if !cur_str.is_empty() { g.draw_string(&cur_str, cur_x, y); }
    }

    /// 获取颜色字符串宽度
    pub fn get_color_string_width(&self, s: &str) -> i32 {
        let mut w = 0i32;
        let mut tmp = String::new();
        let bytes = s.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == 0x00 && i + 3 < bytes.len() { i += 4; continue; }
            tmp.push(bytes[i] as char);
            i += 1;
        }
        if let Some(f) = &self.font { w = f.string_width(&tmp); }
        w
    }

    pub fn resize(&mut self, the_x: i32, the_y: i32, the_w: i32, the_h: i32) {
        self.x = the_x; self.y = the_y; self.width = the_w; self.height = the_h;
        // 预先提取所需值，避免借用冲突
        let fh = self.font.as_ref().map(|f| f.font_height).unwrap_or(0);
        let sb_val = self.scrollbar.as_ref().map(|sb| sb.value).unwrap_or(0.0);
        let log_val = if !self.line_map.is_empty() && sb_val >= 0.0 {
            self.line_map.get(sb_val as usize).copied().unwrap_or(0)
        } else { 0 };
        let lines = self.logical_lines.clone();
        let mw = if self.width > 8 { self.width - 8 } else { 0 };

        self.line_map.clear(); self.physical_lines.clear();
        let mut new_phys = 0i32;
        if fh > 0 {
            let page = if self.height > fh + 16 { (self.height - 8) as f64 / fh as f64 } else { 1.0 };
            for i in 0..lines.len() {
                if i == log_val as usize { new_phys = self.physical_lines.len() as i32; }
                self.add_to_physical_lines(i as i32, &lines[i], mw);
            }
            let at_bottom = self.scrollbar.as_ref().map(|sb| sb.at_bottom()).unwrap_or(false);
            self.page_size = page;
            if let Some(ref mut sb) = self.scrollbar {
                sb.set_max_value(self.physical_lines.len() as f64);
                sb.set_page_size(page as i32 as f64);
                sb.set_value(new_phys as f64);
                if self.stick_to_bottom && at_bottom { sb.go_to_bottom(); }
            }
        }
    }

    fn add_to_physical_lines(&mut self, idx: i32, line: &str, mw: i32) {
        if self.get_color_string_width(line) <= mw {
            self.physical_lines.push(line.to_string());
            self.line_map.push(idx);
        } else {
            let mut cur_str = String::new();
            let mut cur_pos = 0usize;
            while cur_pos < line.len() {
                let mut next_check = cur_pos;
                while next_check < line.len() && line.as_bytes()[next_check] == b' ' { next_check += 1; }
                let space_pos = line[cur_pos..].find(' ').map(|p| cur_pos + p).unwrap_or(line.len());
                let new_str = format!("{}{}", cur_str, &line[cur_pos..space_pos]);
                if self.get_color_string_width(&new_str) > mw {
                    self.physical_lines.push(cur_str.clone());
                    self.line_map.push(idx);
                    cur_str = line[next_check..space_pos].to_string();
                } else {
                    cur_str = new_str;
                }
                cur_pos = space_pos;
            }
            if !cur_str.is_empty() || line.is_empty() {
                self.physical_lines.push(cur_str);
                self.line_map.push(idx);
            }
        }
    }

    pub fn add_line(&mut self, line: &str) {
        let l = if line.is_empty() { " " } else { line };
        let at_bottom = self.scrollbar.as_ref().map(|sb| sb.at_bottom()).unwrap_or(false);
        self.logical_lines.push(l.to_string());

        if self.logical_lines.len() as i32 > self.max_lines {
            let remove = self.logical_lines.len() as i32 - self.max_lines + 10;
            self.logical_lines.drain(..remove as usize);
            let phys_remove = self.line_map.iter().take_while(|&&v| v < remove).count();
            self.line_map.drain(..phys_remove);
            self.physical_lines.drain(..phys_remove);
            for v in &mut self.line_map { *v -= remove; }
            for i in 0..2 {
                self.hilite_area[i][1] -= remove;
                if self.hilite_area[i][1] < 0 { self.hilite_area[i][0] = 0; self.hilite_area[i][1] = 0; }
            }
            if let Some(ref mut sb) = self.scrollbar { sb.set_value(sb.value - remove as f64); }
        }

        let idx = self.logical_lines.len() as i32 - 1;
        let mw = if self.width > 8 { self.width - 8 } else { 0 };
        self.add_to_physical_lines(idx, l, mw);
        if let Some(ref mut sb) = self.scrollbar {
            sb.set_max_value(self.physical_lines.len() as f64);
            sb.set_page_size(self.page_size as i32 as f64);
            if self.stick_to_bottom && at_bottom { sb.go_to_bottom(); }
        }
    }

    pub fn draw(&self, g: &mut Graphics) {
        // 提取所需数据以避免借用冲突
        let font_data = self.font.as_ref().map(|f| {
            let font: &Font = &**f;
            let fptr = font as *const Font as *mut Font;
            (fptr, font.font_height)
        });
        if let Some((fptr, fh)) = font_data {
            g.set_font(fptr);
            let mut phys_idx = self.position as usize;
            let mut y_pos = -(self.position - phys_idx as f64) * fh as f64;
            let max_h = self.height;

            while (y_pos as i32) < max_h && phys_idx < self.physical_lines.len() {
                if y_pos + fh as f64 >= 0.0 {
                    // 简化绘制（不处理内联颜色标记）
                    g.draw_string(&self.physical_lines[phys_idx], 4, y_pos as i32 + fh - 2);
                }
                y_pos += fh as f64;
                phys_idx += 1;
            }
        }
    }

    pub fn scroll_position(&mut self, _id: i32, position: f64) { self.position = position; }
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _c: i32) {}
    pub fn mouse_drag(&mut self, _x: i32, _y: i32) {}
    pub fn key_down(&mut self, _key: KeyCode) {}
}

impl ScrollListener for TextWidget {
    fn scroll_position(&mut self, id: i32, val: f64) { self.scroll_position(id, val); }
}
