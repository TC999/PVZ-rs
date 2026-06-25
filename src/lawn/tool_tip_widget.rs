// PvZ Portable Rust 翻译 — ToolTipWidget（工具提示控件）
// 对应 C++ src/Lawn/ToolTipWidget.h / ToolTipWidget.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::graphics::font::Font;
use crate::framework::graphics::graphics::Graphics;
use crate::lawn::game_enums::{BOARD_WIDTH, BOARD_HEIGHT};

/// 工具提示小部件（鼠标悬停时显示的文字提示框）
#[derive(Debug)]
pub struct ToolTipWidget {
    pub m_title: String,
    pub m_label: String,
    pub m_warning_text: String,
    pub m_x: i32,
    pub m_y: i32,
    pub m_width: i32,
    pub m_height: i32,
    pub m_visible: bool,
    pub m_center: bool,
    pub m_min_left: i32,
    pub m_max_bottom: i32,
    pub m_gets_lines_width: i32,
    pub m_warning_flash_counter: i32,
    pub m_max_lines_width: i32,
}

impl ToolTipWidget {
    pub fn new() -> Self {
        ToolTipWidget {
            m_title: String::new(),
            m_label: String::new(),
            m_warning_text: String::new(),
            m_x: 0,
            m_y: 0,
            m_width: 0,
            m_height: 0,
            m_visible: true,
            m_center: false,
            m_min_left: 0,
            m_max_bottom: BOARD_HEIGHT,
            m_gets_lines_width: 0,
            m_warning_flash_counter: 0,
            m_max_lines_width: 0,
        }
    }

    /// 设置标签文本（自动本地化并重新计算大小）
    pub fn set_label(&mut self, label: &str) {
        self.m_label = label.to_string();
        self.calculate_size();
    }

    /// 设置标题文本（自动本地化并重新计算大小）
    pub fn set_title(&mut self, title: &str) {
        self.m_title = title.to_string();
        self.calculate_size();
    }

    /// 设置警告文本（自动本地化并重新计算大小）
    pub fn set_warning_text(&mut self, warning: &str) {
        self.m_warning_text = warning.to_string();
        self.calculate_size();
    }

    /// 闪烁警告
    pub fn flash_warning(&mut self) {
        self.m_warning_flash_counter = 70;
    }

    /// 每帧更新
    pub fn update(&mut self) {
        if self.m_warning_flash_counter > 0 {
            self.m_warning_flash_counter -= 1;
        }
    }

    /// 设置位置
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.m_x = x;
        self.m_y = y;
    }

    /// 计算工具提示框尺寸
    pub fn calculate_size(&mut self) {
        // TODO: 使用 FONT_TINYBOLD / FONT_PICO129 计算实际尺寸
        // 对应 C++ 中的字体宽度计算
        let a_max_width = 200; // 占位值，需结合字体系统实现

        self.m_gets_lines_width = std::cmp::max(a_max_width - 30, 100);
        if self.m_max_lines_width > 0 {
            self.m_gets_lines_width = std::cmp::min(self.m_gets_lines_width, self.m_max_lines_width);
        }

        let mut a_height = 6;
        if !self.m_title.is_empty() {
            a_height += 16; // FONT_TINYBOLD->GetAscent() + 8
        }
        if !self.m_warning_text.is_empty() {
            a_height += 16; // FONT_TINYBOLD->GetAscent() + 2
        }
        // 每行文本高度
        let lines_count = self.m_label.lines().count().max(1) as i32;
        a_height += lines_count * 10; // FONT_PICO129->GetAscent()

        self.m_width = a_max_width + 10;
        self.m_height = a_height + lines_count * 2 - 2;
    }

    /// 获取按行分割的文本（支持自动换行）
    pub fn get_lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        // 简化实现：按换行符分割
        for line in self.m_label.lines() {
            if self.m_gets_lines_width > 0 && line.len() > self.m_gets_lines_width as usize / 8 {
                // 粗略换行：每大约 12 个字符换行
                let max_chars = (self.m_gets_lines_width / 8) as usize;
                let mut start = 0;
                while start < line.len() {
                    let end = (start + max_chars).min(line.len());
                    // 尝试在空格处断开
                    if end < line.len() {
                        if let Some(space_pos) = line[start..end].rfind(' ') {
                            lines.push(line[start..start + space_pos].to_string());
                            start = start + space_pos + 1;
                            continue;
                        }
                    }
                    lines.push(line[start..end].to_string());
                    start = end;
                }
            } else {
                lines.push(line.to_string());
            }
        }
        lines
    }

    /// 绘制工具提示
    pub fn draw(&self, g: &mut Graphics) {
        if !self.m_visible {
            return;
        }

        let mut pos_x = self.m_x;
        if self.m_center {
            pos_x -= self.m_width / 2;
        }
        if self.m_min_left as f64 - g.trans_x > pos_x as f64 {
            pos_x = self.m_min_left - g.trans_x as i32;
        } else if (pos_x + self.m_width) as f64 + g.trans_x > BOARD_WIDTH as f64 {
            pos_x = (BOARD_WIDTH as f64 - g.trans_x - self.m_width as f64) as i32;
        }

        let mut pos_y = self.m_y;
        if -g.trans_y > pos_y as f64 {
            pos_y = -g.trans_y as i32;
        } else if (self.m_max_bottom as f64) < (self.m_y + self.m_height) as f64 + g.trans_y {
            pos_y = (self.m_max_bottom as f64 - g.trans_y - self.m_height as f64) as i32;
        }

        // 绘制背景
        g.set_color(&Color::new(255, 255, 200, 255));
        g.fill_rect_xywh(pos_x, pos_y, self.m_width, self.m_height);
        g.set_color(&Color::BLACK);
        g.draw_rect_xywh(pos_x, pos_y, self.m_width - 1, self.m_height - 1);
        pos_y += 1;

        // 绘制标题
        if !self.m_title.is_empty() {
            // TODO: g->SetFont(FONT_TINYBOLD);
            let title_x = pos_x + (self.m_width - self.m_title.len() as i32 * 8) / 2;
            g.draw_string(&self.m_title, title_x, pos_y + 14);
            pos_y += 16;
        }

        // 绘制警告文本
        if !self.m_warning_text.is_empty() {
            // TODO: g->SetFont(FONT_PICO129);
            let warn_x = pos_x + (self.m_width - self.m_warning_text.len() as i32 * 6) / 2;

            let mut warn_color = Color::new(255, 0, 0, 255);
            if self.m_warning_flash_counter > 0 && self.m_warning_flash_counter % 20 < 10 {
                warn_color = Color::BLACK;
            }

            g.set_color(&warn_color);
            g.draw_string(&self.m_warning_text, warn_x, pos_y + 10);
            g.set_color(&Color::BLACK);
            pos_y += 12;
        }

        // 绘制主文本行
        // TODO: g->SetFont(FONT_PICO129);
        let lines = self.get_lines();
        for line in &lines {
            let line_x = pos_x + (self.m_width - line.len() as i32 * 6) / 2;
            g.draw_string(line, line_x, pos_y + 10);
            pos_y += 12;
        }
    }
}

impl Default for ToolTipWidget {
    fn default() -> Self {
        Self::new()
    }
}
