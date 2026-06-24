// PvZ Portable Rust 翻译 — HyperlinkWidget 超链接控件
// 对应 C++ SexyAppFramework/widget/HyperlinkWidget.h / HyperlinkWidget.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::widget::insets::Insets;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::button_listener::ButtonListener;

/// 超链接按钮（对应 C++ HyperlinkWidget，继承 ButtonWidget）
pub struct HyperlinkWidget {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub widget_manager: Option<*mut WidgetManager>,
    pub visible: bool, pub disabled: bool,
    pub has_focus: bool, pub is_down: bool, pub is_over: bool,
    pub colors: Vec<Color>,
    pub mouse_insets: Insets,
    pub do_finger: bool,
    pub label: String,
    pub font: Option<Box<Font>>,
    pub color: Color,
    pub over_color: Color,
    pub underline_size: i32,
    pub underline_offset: i32,
}

impl HyperlinkWidget {
    pub fn new(_id: i32, _listener: Option<Box<dyn ButtonListener>>) -> Self {
        HyperlinkWidget {
            x: 0, y: 0, width: 0, height: 0,
            widget_manager: None,
            visible: true, disabled: false,
            has_focus: false, is_down: false, is_over: false,
            colors: Vec::new(),
            mouse_insets: Insets::new(0, 0, 0, 0),
            do_finger: true,
            label: String::new(),
            font: None,
            color: Color::WHITE,
            over_color: Color::WHITE,
            underline_size: 1,
            underline_offset: 3,
        }
    }

    /// 绘制（对应 C++ Draw）
    pub fn draw(&mut self, g: &mut Graphics) {
        if self.font.is_none() && !self.label.is_empty() {
            self.font = Some(Box::new(Font::new("DefaultLink", 10)));
        }
        if let Some(ref font) = self.font {
            let fx = (self.width - font.string_width(&self.label)) / 2;
            let fy = (self.height + font.get_ascent()) / 2 - 1;

            if self.is_over {
                g.set_color(&self.over_color);
            } else {
                g.set_color(&self.color);
            }

            let font_ref: &Font = &**font;
            let fptr = font_ref as *const Font as *mut Font;
            g.set_font(fptr);
            g.draw_string(&self.label, fx, fy);

            // 绘制下划线
            for i in 0..self.underline_size {
                g.fill_rect_xywh(fx, fy + self.underline_offset + i, font.string_width(&self.label), 1);
            }
        }
    }

    /// 鼠标进入（对应 C++ MouseEnter）
    pub fn mouse_enter(&mut self) { self.mark_dirty_full(); }

    /// 鼠标离开（对应 C++ MouseLeave）
    pub fn mouse_leave(&mut self) { self.mark_dirty_full(); }

    fn mark_dirty_full(&self) {}
}
