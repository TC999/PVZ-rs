// PvZ Portable Rust 翻译 — ScrollbarWidget 滚动条控件
// 对应 C++ SexyAppFramework/widget/ScrollbarWidget.h / ScrollbarWidget.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::insets::Insets;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::scrollbutton_widget::ScrollbuttonWidget;
use crate::framework::widget::button_listener::ButtonListener;

/// 滚动监听器 trait（对应 C++ ScrollListener）
pub trait ScrollListener {
    fn scroll_position(&mut self, id: i32, val: f64);
}

pub const UPDATE_MODE_IDLE: i32 = 0;
pub const UPDATE_MODE_PGUP: i32 = 1;
pub const UPDATE_MODE_PGDN: i32 = 2;

/// 滚动条控件（对应 C++ ScrollbarWidget）
pub struct ScrollbarWidget {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub widget_manager: Option<*mut WidgetManager>,
    pub visible: bool, pub disabled: bool,
    pub has_focus: bool, pub is_down: bool, pub is_over: bool,
    pub colors: Vec<Color>,
    pub mouse_insets: Insets,
    pub up_button: Option<Box<ScrollbuttonWidget>>,
    pub down_button: Option<Box<ScrollbuttonWidget>>,
    pub invis_if_no_scroll: bool,
    pub id: i32,
    pub value: f64,
    pub max_value: f64,
    pub page_size: f64,
    pub horizontal: bool,
    pub pressed_on_thumb: bool,
    pub mouse_down_thumb_pos: i32,
    pub mouse_down_x: i32, pub mouse_down_y: i32,
    pub update_mode: i32,
    pub update_acc: i32,
    pub button_acc: i32,
    pub last_mouse_x: i32, pub last_mouse_y: i32,
    pub scroll_listener: Option<Box<dyn ScrollListener>>,
}

impl ScrollbarWidget {
    pub fn new(the_id: i32, the_listener: Option<Box<dyn ScrollListener>>) -> Self {
        ScrollbarWidget {
            x: 0, y: 0, width: 0, height: 0,
            widget_manager: None,
            visible: true, disabled: true,
            has_focus: false, is_down: false, is_over: false,
            colors: Vec::new(),
            mouse_insets: Insets::new(0,0,0,0),
            up_button: None, down_button: None,
            invis_if_no_scroll: false,
            id: the_id,
            value: 0.0, max_value: 0.0, page_size: 0.0,
            horizontal: false,
            pressed_on_thumb: false,
            mouse_down_thumb_pos: 0,
            mouse_down_x: 0, mouse_down_y: 0,
            update_mode: 0, update_acc: 0, button_acc: 0,
            last_mouse_x: 0, last_mouse_y: 0,
            scroll_listener: the_listener,
        }
    }

    pub fn set_max_value(&mut self, new_val: f64) { self.max_value = new_val; self.clamp_value(); }
    pub fn set_page_size(&mut self, new_val: f64) { self.page_size = new_val; self.clamp_value(); }

    pub fn set_value(&mut self, new_val: f64) {
        self.value = new_val;
        self.clamp_value();
        if let Some(ref mut l) = self.scroll_listener { l.scroll_position(self.id, self.value); }
    }

    pub fn at_bottom(&self) -> bool { self.max_value - self.page_size - self.value <= 1.0 }
    pub fn go_to_bottom(&mut self) { self.value = self.max_value - self.page_size; self.clamp_value(); self.set_value(self.value); }

    pub fn draw_thumb(&self, g: &mut Graphics, tx: i32, ty: i32, tw: i32, th: i32) {
        g.set_color(&Color::new(212,212,212,255));
        g.fill_rect_xywh(tx, ty, tw, th);
        g.set_color(&Color::WHITE);
        g.fill_rect_xywh(tx+1, ty+1, tw-2, 1);
        g.fill_rect_xywh(tx+1, ty+1, 1, th-2);
        g.set_color(&Color::BLACK);
        g.fill_rect_xywh(tx, ty+th-1, tw, 1);
        g.fill_rect_xywh(tx+tw-1, ty, 1, th);
        g.set_color(&Color::new(132,132,132,255));
        g.fill_rect_xywh(tx+1, ty+th-2, tw-2, 1);
        g.fill_rect_xywh(tx+tw-2, ty+1, 1, th-2);
    }

    pub fn get_track_size(&self) -> i32 {
        let btn_w = 16; // 简化：假设按钮宽度
        if self.horizontal { self.width - 2 * btn_w } else { self.height - 2 * btn_w }
    }

    pub fn get_thumb_size(&self) -> i32 {
        if self.page_size > self.max_value { return 0; }
        let ts = ((self.get_track_size() as f64 * self.page_size / self.max_value) + 0.5) as i32;
        std::cmp::max(8, ts)
    }

    pub fn get_thumb_position(&self) -> i32 {
        let btn_w = 16;
        if self.page_size > self.max_value { return btn_w; }
        let ts = self.get_thumb_size();
        let track_sz = self.get_track_size();
        if track_sz == ts { return btn_w; }
        ((self.value * (track_sz - ts) as f64 / (self.max_value - self.page_size)) + 0.5) as i32 + btn_w
    }

    pub fn draw(&self, g: &mut Graphics) {
        let ts = self.get_thumb_size();
        let tp = self.get_thumb_position();

        if self.horizontal {
            g.set_color(&if self.update_mode == UPDATE_MODE_PGUP { Color::new(48,48,48,255) } else { Color::new(232,232,232,255) });
            g.fill_rect_xywh(0, 0, tp, self.height);
            if ts > 0 { self.draw_thumb(g, tp, 0, ts, self.height); }
            g.set_color(&if self.update_mode == UPDATE_MODE_PGDN { Color::new(48,48,48,255) } else { Color::new(232,232,232,255) });
            g.fill_rect_xywh(tp+ts, 0, self.width-tp-ts, self.height);
        } else {
            g.set_color(&if self.update_mode == UPDATE_MODE_PGUP { Color::new(48,48,48,255) } else { Color::new(232,232,232,255) });
            g.fill_rect_xywh(0, 0, self.width, tp);
            if ts > 0 { self.draw_thumb(g, 0, tp, self.width, ts); }
            g.set_color(&if self.update_mode == UPDATE_MODE_PGDN { Color::new(48,48,48,255) } else { Color::new(232,232,232,255) });
            g.fill_rect_xywh(0, tp+ts, self.width, self.height-tp-ts);
        }
    }

    fn clamp_value(&mut self) {
        let old = self.value;
        if self.value > self.max_value - self.page_size { self.value = self.max_value - self.page_size; }
        if self.value < 0.0 { self.value = 0.0; }
        let can_scroll = self.page_size < self.max_value;
        self.disabled = !can_scroll;
        if self.invis_if_no_scroll {
            self.visible = can_scroll;
        }
        if (self.value - old).abs() > f64::EPSILON {
            if let Some(ref mut l) = self.scroll_listener { l.scroll_position(self.id, self.value); }
        }
    }

    pub fn set_thumb_position(&mut self, pos: i32) {
        let btn_w = 16;
        let track_sz = self.get_track_size();
        let thumb_sz = self.get_thumb_size();
        if track_sz != thumb_sz {
            self.set_value(((pos - btn_w) as f64 * (self.max_value - self.page_size)) / (track_sz - thumb_sz) as f64);
        }
    }

    pub fn button_press(&mut self, the_id: i32) {
        self.button_acc = 0;
        if the_id == 0 { self.set_value(self.value - 1.0); }
        else { self.set_value(self.value + 1.0); }
    }

    pub fn button_down_tick(&mut self, the_id: i32) {
        self.button_acc += 1;
        if self.button_acc >= 25 {
            if the_id == 0 { self.set_value(self.value - 1.0); }
            else { self.set_value(self.value + 1.0); }
            self.button_acc = 24;
        }
    }

    pub fn update(&mut self) {
        match self.update_mode {
            UPDATE_MODE_PGUP => {
                if self.thumb_compare(self.last_mouse_x, self.last_mouse_y) != -1 {
                    self.update_mode = UPDATE_MODE_IDLE;
                } else {
                    self.update_acc += 1;
                    if self.update_acc >= 25 { self.set_value(self.value - self.page_size); self.update_acc = 20; }
                }
            }
            UPDATE_MODE_PGDN => {
                if self.thumb_compare(self.last_mouse_x, self.last_mouse_y) != 1 {
                    self.update_mode = UPDATE_MODE_IDLE;
                } else {
                    self.update_acc += 1;
                    if self.update_acc >= 25 { self.set_value(self.value + self.page_size); self.update_acc = 20; }
                }
            }
            _ => {}
        }
    }

    pub fn thumb_compare(&self, x: i32, y: i32) -> i32 {
        let pos = if self.horizontal { x } else { y };
        let tp = self.get_thumb_position();
        if pos < tp { return -1; }
        if pos >= tp + self.get_thumb_size() { return 1; }
        0
    }

    pub fn mouse_down_btn(&mut self, x: i32, y: i32, _b: i32, _c: i32) {
        if !self.disabled {
            match self.thumb_compare(x, y) {
                -1 => { self.set_value(self.value - self.page_size); self.update_mode = UPDATE_MODE_PGUP; self.update_acc = 0; }
                0 => { self.pressed_on_thumb = true; self.mouse_down_thumb_pos = self.get_thumb_position(); self.mouse_down_x = x; self.mouse_down_y = y; }
                1 => { self.set_value(self.value + self.page_size); self.update_mode = UPDATE_MODE_PGDN; self.update_acc = 0; }
                _ => {}
            }
        }
        self.last_mouse_x = x; self.last_mouse_y = y;
    }

    pub fn mouse_up_btn(&mut self, _x: i32, _y: i32, _b: i32, _c: i32) {
        self.update_mode = UPDATE_MODE_IDLE;
        self.pressed_on_thumb = false;
    }

    pub fn mouse_drag(&mut self, x: i32, y: i32) {
        if self.pressed_on_thumb {
            if self.horizontal { self.set_thumb_position(self.mouse_down_thumb_pos + x - self.mouse_down_x); }
            else { self.set_thumb_position(self.mouse_down_thumb_pos + y - self.mouse_down_y); }
        }
        self.last_mouse_x = x; self.last_mouse_y = y;
    }
}

impl ButtonListener for ScrollbarWidget {
    fn button_press(&mut self, the_id: i32) { self.button_press(the_id); }
    fn button_depress(&mut self, _id: i32) { }
    fn button_down_tick(&mut self, the_id: i32) { self.button_down_tick(the_id); }
    fn button_mouse_enter(&mut self, _id: i32) {}
    fn button_mouse_leave(&mut self, _id: i32) {}
    fn button_mouse_move(&mut self, _id: i32, _x: i32, _y: i32) {}
}
