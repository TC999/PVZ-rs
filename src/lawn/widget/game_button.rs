// PvZ Portable Rust 翻译 — GameButton 游戏按钮
// 对应 C++ src/Lawn/Widget/GameButton.h

#![allow(dead_code)]
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::graphics::font::Font;
use crate::framework::color::Color;

/// 游戏中的自定义按钮（对应 C++ GameButton）
pub struct GameButton {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub label: String,
    pub btn_no_draw: bool,
    pub is_over: bool, pub is_down: bool, pub disabled: bool,
    pub button_image: *mut Image,
    pub font: Option<Box<Font>>,
    pub label_justify: i32,
    pub colors: [Color; 6],
    pub text_offset_x: i32,
    pub text_offset_y: i32,
}

impl GameButton {
    pub const BUTTON_LABEL_LEFT: i32 = 0;
    pub const BUTTON_LABEL_CENTER: i32 = 1;
    pub const BUTTON_LABEL_RIGHT: i32 = 2;

    pub const COLOR_BKG: usize = 0;
    pub const COLOR_DARK_OUTLINE: usize = 1;
    pub const COLOR_LIGHT_OUTLINE: usize = 2;
    pub const COLOR_MEDIUM_OUTLINE: usize = 3;
    pub const COLOR_LABEL: usize = 4;
    pub const COLOR_LABEL_HILITE: usize = 5;

    pub fn new(_id: i32, _listener: Option<Box<dyn crate::framework::widget::button_listener::ButtonListener>>) -> Self {
        GameButton {
            x: 0, y: 0, width: 0, height: 0, label: String::new(),
            btn_no_draw: false, is_over: false, is_down: false, disabled: false,
            button_image: std::ptr::null_mut(),
            font: None,
            label_justify: GameButton::BUTTON_LABEL_CENTER,
            colors: [Color::new(0, 0, 0, 0); 6],
            text_offset_x: 0,
            text_offset_y: 0,
        }
    }
    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) { self.x = x; self.y = y; self.width = w; self.height = h; }

    pub fn draw(&self, g: &mut Graphics) {
        if self.btn_no_draw { return; }

        let font = match self.font {
            Some(ref f) => f,
            None => return,
        };

        let mut font_x = self.text_offset_x;
        let mut font_y = self.text_offset_y;

        if self.label_justify == GameButton::BUTTON_LABEL_CENTER {
            font_x += (self.width - font.string_width(&self.label)) / 2;
        } else if self.label_justify == GameButton::BUTTON_LABEL_RIGHT {
            font_x += self.width - font.string_width(&self.label);
        }

        font_y += (self.height - font.get_ascent() / 6 + font.get_ascent() - 1) / 2;

        let color = if self.is_over {
            self.colors[GameButton::COLOR_LABEL_HILITE]
        } else {
            self.colors[GameButton::COLOR_LABEL]
        };

        g.set_color(&color);
        g.draw_string(&self.label, self.x + font_x, self.y + font_y);
    }

    pub fn set_font(&mut self, the_font: &Font) {
        self.font = Some(Box::new(Font::new(&the_font.name, the_font.size)));
        if let Some(ref mut f) = self.font {
            f.bold = the_font.bold;
            f.italic = the_font.italic;
            f.line_spacing = the_font.line_spacing;
            f.char_spacing = the_font.char_spacing;
            f.font_height = the_font.font_height;
            f.ascent = the_font.ascent;
            f.descent = the_font.descent;
            f.ascent_padding = the_font.ascent_padding;
        }
    }

    pub fn mouse_down_btn(&mut self, _x: i32, _y: i32, _b: i32, _c: i32) { self.is_down = true; }
    pub fn mouse_up_btn(&mut self, _x: i32, _y: i32, _b: i32, _c: i32) { self.is_down = false; }
    pub fn mouse_enter(&mut self) { self.is_over = true; }
    pub fn mouse_leave(&mut self) { self.is_over = false; }
}
