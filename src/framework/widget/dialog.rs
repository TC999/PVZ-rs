// PvZ Portable Rust 翻译 — Dialog 对话框控件
// 对应 C++ SexyAppFramework/widget/Dialog.h / Dialog.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::graphics::image::Image;
use crate::framework::widget::insets::Insets;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::button_listener::ButtonListener;
use crate::framework::widget::dialog_listener::DialogListener;

pub const BUTTONS_NONE: i32 = 0;
pub const BUTTONS_YES_NO: i32 = 1;
pub const BUTTONS_OK_CANCEL: i32 = 2;
pub const BUTTONS_FOOTER: i32 = 3;
pub const ID_YES: i32 = 1000;
pub const ID_NO: i32 = 1001;
pub const ID_OK: i32 = 1000;
pub const ID_CANCEL: i32 = 1001;
pub const ID_FOOTER: i32 = 1000;
pub const COLOR_HEADER: usize = 0;
pub const COLOR_LINES: usize = 1;
pub const COLOR_FOOTER: usize = 2;
pub const COLOR_BUTTON_TEXT: usize = 3;
pub const COLOR_BUTTON_TEXT_HILITE: usize = 4;
pub const COLOR_BKG: usize = 5;
pub const COLOR_OUTLINE: usize = 6;
pub const NUM_COLORS: usize = 7;

const G_DIALOG_COLORS: [[u8; 3]; NUM_COLORS] = [
    [255, 255, 255], [255, 255, 0], [255, 255, 255],
    [255, 255, 255], [255, 255, 255], [80, 80, 80], [255, 255, 255],
];

pub struct Dialog {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub visible: bool, pub disabled: bool,
    pub has_alpha: bool, pub has_transparencies: bool,
    pub colors: Vec<Color>,
    pub priority: i32,

    pub dialog_listener: Option<Box<dyn DialogListener>>,
    pub component_image: *mut Image,
    pub id: i32,
    pub is_modal: bool,
    pub result: i32,
    pub button_mode: i32,
    pub button_height: i32,
    pub button_horz_spacing: i32,

    pub dialog_header: String,
    pub dialog_footer: String,
    pub dialog_lines: String,
    pub text_align: i32,
    pub line_spacing_offset: i32,
    pub space_after_header: i32,
    pub content_insets: Insets,
    pub background_insets: Insets,
    pub header_font: Option<Box<Font>>,
    pub lines_font: Option<Box<Font>>,
    pub dragging: bool,
    pub drag_mouse_x: i32,
    pub drag_mouse_y: i32,
}

impl Dialog {
    pub fn new(
        component_image: *mut Image, _button_component_image: *mut Image,
        the_id: i32, is_modal: bool, header: &str, lines: &str, footer: &str, button_mode: i32,
    ) -> Self {
        let mut d = Dialog {
            x: 0, y: 0, width: 0, height: 0,
            visible: true, disabled: false,
            has_alpha: true, has_transparencies: true,
            colors: Vec::new(),
            priority: 1,
            dialog_listener: None, component_image, id: the_id,
            is_modal, result: 0x7FFFFFFF, button_mode,
            button_height: if component_image.is_null() { 24 } else { unsafe { (*component_image).get_height() } },
            button_horz_spacing: 8,
            dialog_header: header.to_string(), dialog_footer: footer.to_string(),
            dialog_lines: lines.to_string(), text_align: 0,
            line_spacing_offset: 0, space_after_header: 10,
            content_insets: Insets::new(24, 24, 24, 24),
            background_insets: Insets::new(0, 0, 0, 0),
            header_font: None, lines_font: None,
            dragging: false, drag_mouse_x: 0, drag_mouse_y: 0,
        };
        let mut mc = G_DIALOG_COLORS;
        if component_image.is_null() { mc[COLOR_BUTTON_TEXT] = [0,0,0]; mc[COLOR_BUTTON_TEXT_HILITE] = [0,0,0]; }
        d.colors.clear();
        for c in &mc { d.colors.push(Color::from_rgb(c[0], c[1], c[2])); }
        d
    }

    fn ensure_fonts(&mut self) {
        if self.header_font.is_none() { self.header_font = Some(Box::new(Font::new("Header", 14))); }
        if self.lines_font.is_none() { self.lines_font = Some(Box::new(Font::new("Lines", 12))); }
    }

    fn get_color(&self, idx: usize) -> Color { self.colors.get(idx).copied().unwrap_or(Color::WHITE) }

    pub fn draw(&mut self, g: &mut Graphics) {
        self.ensure_fonts();
        let br = self.background_insets;
        let cr = self.content_insets;

        if !self.component_image.is_null() {
            unsafe { g.draw_image_box_dest(&Rect::new(br.left, br.top, self.width-br.left-br.right, self.height-br.top-br.bottom), &*self.component_image); }
        } else {
            g.set_color(&self.get_color(COLOR_OUTLINE));
            g.draw_rect(&Rect::new(12, 12, self.width-25, self.height-25));
            g.set_color(&self.get_color(COLOR_BKG));
            g.fill_rect_xywh(13, 13, self.width-26, self.height-26);
            g.set_color(&Color::new(0,0,0,128));
            g.fill_rect_xywh(self.width-12, 24, 12, self.height-36);
            g.fill_rect_xywh(24, self.height-12, self.width-24, 12);
        }

        let hf = self.header_font.as_ref().unwrap();
        let lf = self.lines_font.as_ref().unwrap();
        let hf_ref: &Font = &**hf;
        let hf_raw = hf_ref as *const Font as *mut Font;
        let lf_ref: &Font = &**lf;
        let lf_raw = lf_ref as *const Font as *mut Font;
        let mut cur_y = cr.top + br.top;

        if !self.dialog_header.is_empty() {
            cur_y += hf.get_ascent() - hf.ascent_padding;
            g.set_font(hf_raw);
            g.set_color(&self.get_color(COLOR_HEADER));
            let w = hf.string_width(&self.dialog_header);
            g.draw_string(&self.dialog_header, (self.width-w)/2, cur_y);
            cur_y += hf.font_height - hf.get_ascent() + self.space_after_header;
        }

        g.set_font(lf_raw);
        g.set_color(&self.get_color(COLOR_LINES));
        let text_r = Rect::new(br.left+cr.left+2, cur_y, self.width-cr.left-cr.right-br.left-br.right-4, 0);
        cur_y += g.write_word_wrapped(&text_r, &self.dialog_lines, lf.line_spacing+self.line_spacing_offset, self.text_align, None, -1, None);

        if !self.dialog_footer.is_empty() && self.button_mode != BUTTONS_FOOTER {
            cur_y += 8 + hf.line_spacing;
            g.set_font(hf_raw);
            g.set_color(&self.get_color(COLOR_FOOTER));
            let w2 = hf.string_width(&self.dialog_footer);
            g.draw_string(&self.dialog_footer, (self.width-w2)/2, cur_y);
        }
    }

    pub fn mouse_down_btn(&mut self, x: i32, y: i32, _b: i32, cc: i32) {
        if cc == 1 { self.dragging = true; self.drag_mouse_x = x; self.drag_mouse_y = y; }
    }

    pub fn mouse_drag(&mut self, x: i32, y: i32, wm: &WidgetManager) {
        if !self.dragging { return; }
        let mut nx = self.x + x - self.drag_mouse_x;
        let mut ny = self.y + y - self.drag_mouse_y;
        if nx < -8 { nx = -8; } else if nx+self.width > wm.mouse_dest_rect.width+8 { nx = wm.mouse_dest_rect.width-self.width+8; }
        if ny < -8 { ny = -8; } else if ny+self.height > wm.mouse_dest_rect.height+8 { ny = wm.mouse_dest_rect.height-self.height+8; }
        self.drag_mouse_x = self.x+x-nx; self.drag_mouse_y = self.y+y-ny;
        if self.drag_mouse_x < 8 { self.drag_mouse_x = 8; } else if self.drag_mouse_x > self.width-9 { self.drag_mouse_x = self.width-9; }
        if self.drag_mouse_y < 8 { self.drag_mouse_y = 8; } else if self.drag_mouse_y > self.height-9 { self.drag_mouse_y = self.height-9; }
        self.x = nx; self.y = ny;
    }

    pub fn mouse_up_btn(&mut self, _x: i32, _y: i32, _b: i32, _c: i32) { self.dragging = false; }
    pub fn update(&mut self) {}
    pub fn is_modal(&self) -> bool { self.is_modal }
    pub fn wait_for_result(&mut self, _auto_kill: bool) -> i32 { self.result }

    pub fn button_press(&mut self, the_id: i32) {
        if (the_id == ID_YES || the_id == ID_NO) && self.dialog_listener.is_some() {
            self.dialog_listener.as_mut().unwrap().dialog_button_press(self.id, the_id);
        }
    }

    pub fn button_depress(&mut self, the_id: i32) {
        if the_id == ID_YES || the_id == ID_NO {
            self.result = the_id;
            if let Some(ref mut l) = self.dialog_listener { l.dialog_button_depress(self.id, the_id); }
        }
    }
}
