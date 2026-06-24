// PvZ Portable Rust 翻译 — Widget 控件基类
// 对应 C++ SexyAppFramework/widget/Widget.h / Widget.cpp
//
// 在 Rust 版本中 Widget 通过组合方式包含 WidgetContainer 的字段。
// 子类通过继承 Widget（即包含字段）来扩展。

#![allow(dead_code)]

use std::cmp;

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::point::Point;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::widget::widget_container::WidgetContainer;
use crate::framework::widget::insets::Insets;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::{KeyCode, KEYCODE_TAB, KEYCODE_SHIFT};

/// 布局标志
pub const LAY_SAME_WIDTH: i32 = 0x0001;
pub const LAY_SAME_HEIGHT: i32 = 0x0002;
pub const LAY_SET_LEFT: i32 = 0x0010;
pub const LAY_SET_TOP: i32 = 0x0020;
pub const LAY_SET_WIDTH: i32 = 0x0040;
pub const LAY_SET_HEIGHT: i32 = 0x0080;
pub const LAY_ABOVE: i32 = 0x0100;
pub const LAY_BELOW: i32 = 0x0200;
pub const LAY_RIGHT: i32 = 0x0400;
pub const LAY_LEFT: i32 = 0x0800;
pub const LAY_SAME_LEFT: i32 = 0x1000;
pub const LAY_SAME_RIGHT: i32 = 0x2000;
pub const LAY_SAME_TOP: i32 = 0x4000;
pub const LAY_SAME_BOTTOM: i32 = 0x8000;
pub const LAY_GROW_TO_RIGHT: i32 = 0x10000;
pub const LAY_GROW_TO_LEFT: i32 = 0x20000;
pub const LAY_GROW_TO_TOP: i32 = 0x40000;
pub const LAY_GROW_TO_BOTTOM: i32 = 0x80000;
pub const LAY_HCENTER: i32 = 0x100000;
pub const LAY_VCENTER: i32 = 0x200000;
pub const LAY_MAX: i32 = 0x400000;

// ============================================================
// WidgetImpl trait — 模拟 C++ 虚函数分派
// ============================================================

/// Widget 子类需实现此 trait 来覆盖绘制/更新/事件等行为。
/// 所有方法都有默认空实现，子类只覆盖需要的即可。
pub trait WidgetImpl {
    // ---- 绘制 ----
    fn draw(&mut self, widget: &Widget, g: &mut Graphics) {}
    fn draw_overlay(&mut self, widget: &Widget, g: &mut Graphics) {}

    // ---- 更新 ----
    fn update(&mut self, widget: &mut Widget) {}

    // ---- 键盘 ----
    fn key_down(&mut self, widget: &mut Widget, key: KeyCode, wm: &mut WidgetManager) {}
    fn key_up(&mut self, widget: &mut Widget, key: KeyCode) {}
    fn key_char(&mut self, widget: &mut Widget, c: u8) {}

    // ---- 鼠标 ----
    fn mouse_down_btn(&mut self, widget: &mut Widget, x: i32, y: i32, btn: i32, click: i32) {}
    fn mouse_up_btn(&mut self, widget: &mut Widget, x: i32, y: i32, btn: i32, click: i32) {}
    fn mouse_move(&mut self, widget: &mut Widget, x: i32, y: i32) {}
    fn mouse_drag(&mut self, widget: &mut Widget, x: i32, y: i32) {}
    fn mouse_wheel(&mut self, widget: &mut Widget, delta: i32) {}
    fn mouse_enter(&mut self, widget: &mut Widget) {}
    fn mouse_leave(&mut self, widget: &mut Widget) {}

    // ---- 焦点 ----
    fn got_focus(&mut self, widget: &mut Widget) {}
    fn lost_focus(&mut self, widget: &mut Widget) {}
}

/// 控件基类（对应 C++ Widget，继承自 WidgetContainer）
/// 在 Rust 中使用组合，字段直接内联（不再通过 container 字段访问）
pub struct Widget {
    // ---- 位置和大小（原本来自 WidgetContainer） ----
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,

    // ---- 父子关系 ----
    pub parent: Option<*mut Widget>,
    pub widget_manager: Option<*mut WidgetManager>,

    // ---- 渲染 ----
    pub has_alpha: bool,
    pub clip: bool,
    pub dirty: bool,
    pub zorder: i32,
    pub priority: i32,
    pub visible: bool,
    pub mouse_visible: bool,
    pub disabled: bool,
    pub has_focus: bool,
    pub is_down: bool,
    pub is_over: bool,
    pub has_transparencies: bool,

    // ---- 颜色 ----
    pub colors: Vec<Color>,

    // ---- 鼠标 ----
    pub mouse_insets: Insets,

    // ---- 状态 ----
    pub do_finger: bool,
    pub wants_focus: bool,
    pub tab_prev: Option<*mut Widget>,
    pub tab_next: Option<*mut Widget>,

    // ---- 资源 ----
    pub loaded_resource_names: Vec<String>,

    // ---- WidgetImpl 分派（模拟 C++ 虚表） ----
    pub impl_: Option<Box<dyn WidgetImpl>>,
}

/// 静态标志：是否启用带颜色标记的字符串绘制
pub static mut WRITE_COLORED_STRING: bool = true;

impl Widget {
    pub fn new() -> Self {
        Widget {
            x: 0, y: 0, width: 0, height: 0,
            parent: None,
            widget_manager: None,
            has_alpha: false,
            clip: true,
            dirty: false,
            zorder: 0,
            priority: 0,
            visible: true,
            mouse_visible: true,
            disabled: false,
            has_focus: false,
            is_down: false,
            is_over: false,
            has_transparencies: false,
            colors: Vec::new(),
            mouse_insets: Insets::new(0, 0, 0, 0),
            do_finger: false,
            wants_focus: false,
            tab_prev: None,
            tab_next: None,
            loaded_resource_names: Vec::new(),
            impl_: None,
        }
    }

    // ============================================================
    // 矩形操作
    // ============================================================

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    pub fn left(&self) -> i32 { self.x }
    pub fn top(&self) -> i32 { self.y }
    pub fn right(&self) -> i32 { self.x + self.width }
    pub fn bottom(&self) -> i32 { self.y + self.height }
    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { self.height }

    /// 判断点是否在控件内
    pub fn contains(&self, the_x: i32, the_y: i32) -> bool {
        the_x >= self.x && the_x < self.x + self.width
            && the_y >= self.y && the_y < self.y + self.height
    }

    /// 获取内嵌矩形
    pub fn get_inset_rect(&self) -> Rect {
        Rect::new(
            self.x + self.mouse_insets.left,
            self.y + self.mouse_insets.top,
            self.width - self.mouse_insets.left - self.mouse_insets.right,
            self.height - self.mouse_insets.top - self.mouse_insets.bottom,
        )
    }

    /// 判断点是否可见
    pub fn is_point_visible(&self, _x: i32, _y: i32) -> bool {
        true
    }

    // ============================================================
    // 可见性/启用
    // ============================================================

    pub fn set_visible(&mut self, is_visible: bool) {
        if self.visible == is_visible { return; }
        self.visible = is_visible;
        if self.visible { self.mark_dirty(); }
        else { self.mark_dirty_full(); }
        if let Some(wm) = self.widget_manager {
            unsafe { (*wm).rehup_mouse(); }
        }
    }

    pub fn set_disabled(&mut self, is_disabled: bool) {
        if self.disabled == is_disabled { return; }
        self.disabled = is_disabled;
        if is_disabled {
            if let Some(wm) = self.widget_manager {
                unsafe { (*wm).disable_widget(self as *mut Widget); }
            }
        }
        self.mark_dirty();
        if !is_disabled {
            if let Some(wm) = self.widget_manager {
                unsafe {
                    if self.contains((*wm).mouse_x, (*wm).mouse_y) {
                        (*wm).mouse_position((*wm).mouse_x, (*wm).mouse_y);
                    }
                }
            }
        }
    }

    // ============================================================
    // 颜色
    // ============================================================

    pub fn set_colors_rgb(&mut self, the_colors: &[[u8; 3]]) {
        self.colors.clear();
        for c in the_colors {
            self.colors.push(Color::from_rgb(c[0], c[1], c[2]));
        }
        self.mark_dirty();
    }

    pub fn set_colors_rgba(&mut self, the_colors: &[[u8; 4]]) {
        self.colors.clear();
        for c in the_colors {
            self.colors.push(Color::new(c[0], c[1], c[2], c[3]));
        }
        self.mark_dirty();
    }

    pub fn set_color(&mut self, idx: usize, color: &Color) {
        if idx >= self.colors.len() {
            self.colors.resize(idx + 1, Color::WHITE);
        }
        self.colors[idx] = *color;
        self.mark_dirty();
    }

    pub fn get_color(&self, idx: usize) -> Color {
        self.colors.get(idx).copied().unwrap_or(Color::WHITE)
    }

    pub fn get_color_default(&self, idx: usize, default_color: &Color) -> Color {
        self.colors.get(idx).copied().unwrap_or(*default_color)
    }

    // ============================================================
    // 位置/大小
    // ============================================================

    pub fn resize(&mut self, the_x: i32, the_y: i32, the_w: i32, the_h: i32) {
        if self.x == the_x && self.y == the_y && self.width == the_w && self.height == the_h {
            return;
        }
        self.mark_dirty_full();
        self.x = the_x; self.y = the_y; self.width = the_w; self.height = the_h;
        self.mark_dirty();
        if let Some(wm) = self.widget_manager {
            unsafe { (*wm).rehup_mouse(); }
        }
    }

    pub fn resize_rect(&mut self, r: &Rect) {
        self.resize(r.x, r.y, r.width, r.height);
    }

    pub fn move_to(&mut self, nx: i32, ny: i32) {
        self.resize(nx, ny, self.width, self.height);
    }

    // ============================================================
    // 绘制
    // ============================================================

    pub fn draw(&mut self, g: &mut Graphics) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.draw(self, g);
            }
            self.impl_ = impl_;
        }
    }
    pub fn draw_overlay(&mut self, g: &mut Graphics) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.draw_overlay(self, g);
            }
            self.impl_ = impl_;
        }
    }
    pub fn draw_overlay_priority(&mut self, g: &mut Graphics, _priority: i32) {
        self.draw_overlay(g);
    }

    // ============================================================
    // 更新
    // ============================================================

    pub fn update(&mut self) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.update(self);
            }
            self.impl_ = impl_;
        }
    }

    // ============================================================
    // 焦点
    // ============================================================

    pub fn got_focus(&mut self) {
        self.has_focus = true;
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.got_focus(self);
            }
            self.impl_ = impl_;
        }
    }
    pub fn lost_focus(&mut self) {
        self.has_focus = false;
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.lost_focus(self);
            }
            self.impl_ = impl_;
        }
    }
    pub fn wants_focus(&self) -> bool { self.wants_focus }
    pub fn show_finger(&mut self, _on: bool) {}

    // ============================================================
    // 键盘
    // ============================================================

    pub fn key_char(&mut self, c: u8) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.key_char(self, c);
            }
            self.impl_ = impl_;
        }
    }

    pub fn key_down(&mut self, the_key: KeyCode, wm: &mut WidgetManager) {
        if the_key == KEYCODE_TAB {
            let shift_idx = KEYCODE_SHIFT as usize;
            let shift_down = wm.key_down.get(shift_idx).copied().unwrap_or(false);
            if shift_down {
                if let Some(prev) = self.tab_prev {
                    wm.set_focus(Some(prev));
                }
            } else {
                if let Some(next) = self.tab_next {
                    wm.set_focus(Some(next));
                }
            }
        }
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.key_down(self, the_key, wm);
            }
            self.impl_ = impl_;
        }
    }

    pub fn key_up(&mut self, key: KeyCode) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.key_up(self, key);
            }
            self.impl_ = impl_;
        }
    }

    // ============================================================
    // 鼠标
    // ============================================================

    pub fn mouse_enter(&mut self) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.mouse_enter(self);
            }
            self.impl_ = impl_;
        }
    }
    pub fn mouse_leave(&mut self) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.mouse_leave(self);
            }
            self.impl_ = impl_;
        }
    }
    pub fn mouse_move(&mut self, x: i32, y: i32) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.mouse_move(self, x, y);
            }
            self.impl_ = impl_;
        }
    }

    pub fn mouse_down(&mut self, x: i32, y: i32, click_count: i32) {
        if click_count == 3 {
            self.mouse_down_btn(x, y, 2, 1);
        } else if click_count >= 0 {
            self.mouse_down_btn(x, y, 0, click_count);
        } else {
            self.mouse_down_btn(x, y, 1, -click_count);
        }
    }

    pub fn mouse_down_btn(&mut self, x: i32, y: i32, btn: i32, click: i32) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.mouse_down_btn(self, x, y, btn, click);
            }
            self.impl_ = impl_;
        }
    }
    pub fn mouse_up(&mut self, _x: i32, _y: i32) {}

    pub fn mouse_up_ext(&mut self, x: i32, y: i32, last_btn_id: i32) {
        self.mouse_up(x, y);
        if last_btn_id == 3 {
            self.mouse_up_btn(x, y, 2, 1);
        } else if last_btn_id >= 0 {
            self.mouse_up_btn(x, y, 0, last_btn_id);
        } else {
            self.mouse_up_btn(x, y, 1, -last_btn_id);
        }
    }

    pub fn mouse_up_btn(&mut self, x: i32, y: i32, btn: i32, click: i32) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.mouse_up_btn(self, x, y, btn, click);
            }
            self.impl_ = impl_;
        }
    }
    pub fn mouse_drag(&mut self, x: i32, y: i32) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.mouse_drag(self, x, y);
            }
            self.impl_ = impl_;
        }
    }
    pub fn mouse_wheel(&mut self, delta: i32) {
        if self.impl_.is_some() {
            let mut impl_ = self.impl_.take();
            if let Some(ref mut imp) = impl_ {
                imp.mouse_wheel(self, delta);
            }
            self.impl_ = impl_;
        }
    }

    // ============================================================
    // 脏标记
    // ============================================================

    pub fn mark_dirty(&mut self) {
        if let Some(p) = self.parent {
            unsafe { (*p).mark_dirty(); }
        } else {
            self.dirty = true;
        }
    }

    pub fn mark_dirty_full(&mut self) {
        if let Some(p) = self.parent {
            unsafe { (*p).mark_dirty_full(); }
        } else {
            self.dirty = true;
        }
    }

    pub fn mark_all_dirty(&mut self) {
        self.dirty = true;
    }

    // ============================================================
    // 管理器回调
    // ============================================================

    pub fn order_in_manager_changed(&mut self) {}
    pub fn added_to_manager(&mut self, _wm: *mut WidgetManager) {}
    pub fn removed_from_manager(&mut self, _wm: Option<*mut WidgetManager>) {}

    pub fn widget_removed_helper(&mut self) {
        if self.widget_manager.is_none() { return; }
        if let Some(wm) = self.widget_manager {
            unsafe { (*wm).disable_widget(self as *mut Widget); }
        }
        self.removed_from_manager(self.widget_manager);
        self.mark_dirty_full();
        self.widget_manager = None;
    }

    // ============================================================
    // 帮助方法
    // ============================================================

    pub fn defer_overlay(&mut self, priority: i32) {
        if let Some(wm) = self.widget_manager {
            unsafe { (*wm).defer_overlay(self as *mut Widget, priority); }
        }
    }

    pub fn write_centered_line(&self, g: &mut Graphics, offset: i32, line: &str) -> Rect {
        let font = unsafe { g.font.as_ref().unwrap() };
        let w = font.string_width(line);
        let x = (self.width - w) / 2;
        g.draw_string(line, x, offset);
        Rect::new(x, offset - font.get_ascent(), w, font.font_height)
    }

    pub fn write_centered_line_shadow(
        &self, g: &mut Graphics, offset: i32, line: &str,
        color1: &Color, color2: &Color, shadow_offset: &Point,
    ) -> Rect {
        let font = unsafe { g.font.as_ref().unwrap() };
        let w = font.string_width(line);
        g.set_color(color2);
        g.draw_string(line, (self.width - w) / 2 + shadow_offset.x, offset + shadow_offset.y);
        g.set_color(color1);
        g.draw_string(line, (self.width - w) / 2, offset);
        Rect::new(
            (self.width - w) / 2 + cmp::min(0, shadow_offset.x),
            offset - font.get_ascent() + cmp::min(0, shadow_offset.y),
            w + shadow_offset.x.abs(),
            font.font_height + shadow_offset.y.abs(),
        )
    }

    pub fn get_num_digits(n: i32) -> i32 {
        let mut d = 10; let mut nd = 1;
        while n >= d { nd += 1; d *= 10; }
        nd
    }

    pub fn write_number_from_strip(
        &self, g: &mut Graphics, number: i32,
        the_x: i32, the_y: i32, strip: &Image, spacing: i32,
    ) {
        let mut divisor = 10;
        let mut num_digits = 1;
        while number >= divisor { num_digits += 1; divisor *= 10; }
        if number == 0 { divisor = 10; }

        let digit_len = strip.get_width() / 10;
        for di in 0..num_digits {
            divisor /= 10;
            let digit = (number / divisor) % 10;
            let mut cg = g.create();
            cg.clip_rect_xywh(the_x + di * (digit_len + spacing), the_y, digit_len, strip.get_height());
            cg.draw_image_xy(strip, the_x + di * (digit_len + spacing) - digit * digit_len, the_y);
        }
    }

    // ============================================================
    // 布局
    // ============================================================

    pub fn layout(
        &mut self, flags: i32, rel: Option<&Widget>,
        left_pad: i32, top_pad: i32, w_pad: i32, h_pad: i32,
    ) {
        let rel = match rel { Some(r) => r, None => return };

        let rl = if self.parent.is_some() && self.parent == rel.parent { 0 } else { rel.x };
        let rt = if self.parent.is_some() && self.parent == rel.parent { 0 } else { rel.y };
        let rw = rel.width; let rh = rel.height;
        let rr = rl + rw; let rb = rt + rh;

        let mut l = self.x; let mut t = self.y;
        let mut wd = self.width; let mut ht = self.height;
        let mut ft = 1;

        while ft < LAY_MAX {
            if (flags & ft) != 0 {
                match ft {
                    LAY_SAME_WIDTH => wd = rw + w_pad,
                    LAY_SAME_HEIGHT => ht = rh + h_pad,
                    LAY_ABOVE => t = rt - ht + top_pad,
                    LAY_BELOW => t = rb + top_pad,
                    LAY_RIGHT => l = rr + left_pad,
                    LAY_LEFT => l = rl - wd + left_pad,
                    LAY_SAME_LEFT => l = rl + left_pad,
                    LAY_SAME_RIGHT => l = rr - wd + left_pad,
                    LAY_SAME_TOP => t = rt + top_pad,
                    LAY_SAME_BOTTOM => t = rb - ht + top_pad,
                    LAY_GROW_TO_RIGHT => wd = rr - l + w_pad,
                    LAY_GROW_TO_LEFT => wd = rl - l + w_pad,
                    LAY_GROW_TO_TOP => ht = rt - t + h_pad,
                    LAY_GROW_TO_BOTTOM => ht = rb - t + h_pad,
                    LAY_SET_LEFT => l = left_pad,
                    LAY_SET_TOP => t = top_pad,
                    LAY_SET_WIDTH => wd = w_pad,
                    LAY_SET_HEIGHT => ht = h_pad,
                    LAY_HCENTER => l = rl + (rw - wd) / 2 + left_pad,
                    LAY_VCENTER => t = rt + (rh - ht) / 2 + top_pad,
                    _ => {}
                }
            }
            ft <<= 1;
        }
        self.resize(l, t, wd, ht);
    }
}

impl Default for Widget {
    fn default() -> Self { Widget::new() }
}
