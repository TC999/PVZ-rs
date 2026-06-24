// PvZ Portable Rust 翻译 — Slider 滑动条控件
// 对应 C++ SexyAppFramework/widget/Slider.h / Slider.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::widget::insets::Insets;
use crate::framework::widget::widget_manager::WidgetManager;

/// 滑动条监听器 trait（对应 C++ SliderListener）
pub trait SliderListener {
    fn slider_val(&mut self, id: i32, val: f64);
}

/// 滑动条控件（对应 C++ Slider）
pub struct Slider {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub widget_manager: Option<*mut WidgetManager>,
    pub visible: bool, pub disabled: bool,
    pub has_focus: bool, pub is_down: bool, pub is_over: bool,
    pub has_alpha: bool, pub has_transparencies: bool,
    pub colors: Vec<Color>,
    pub mouse_insets: Insets,
    pub do_finger: bool,

    pub listener: Option<Box<dyn SliderListener>>,
    pub val: f64,
    pub id: i32,
    pub track_image: *mut Image,
    pub thumb_image: *mut Image,
    pub dragging: bool,
    pub rel_x: i32,
    pub rel_y: i32,
    pub horizontal: bool,
}

impl Slider {
    pub fn new(track: *mut Image, thumb: *mut Image, the_id: i32, the_listener: Option<Box<dyn SliderListener>>) -> Self {
        Slider {
            x: 0, y: 0, width: 0, height: 0,
            widget_manager: None,
            visible: true, disabled: false,
            has_focus: false, is_down: false, is_over: false,
            has_alpha: true, has_transparencies: true,
            colors: Vec::new(),
            mouse_insets: Insets::new(0, 0, 0, 0),
            do_finger: false,
            listener: the_listener,
            val: 0.0,
            id: the_id,
            track_image: track,
            thumb_image: thumb,
            dragging: false,
            rel_x: 0, rel_y: 0,
            horizontal: true,
        }
    }

    /// 设置值（对应 C++ SetValue）
    pub fn set_value(&mut self, the_val: f64) {
        self.val = the_val;
        if self.val < 0.0 { self.val = 0.0; }
        else if self.val > 1.0 { self.val = 1.0; }
        self.mark_dirty_full();
    }

    /// 绘制（对应 C++ Draw）
    pub fn draw(&self, g: &mut Graphics) {
        if !self.track_image.is_null() {
            unsafe {
                let cw = if self.horizontal { (*self.track_image).get_width() / 3 } else { (*self.track_image).get_width() };
                let ch = if self.horizontal { (*self.track_image).get_height() } else { (*self.track_image).get_height() / 3 };

                if self.horizontal {
                    let ty = (self.height - ch) / 2;
                    // 左端
                    g.draw_image_src(&*self.track_image, 0, ty, &Rect::new(0, 0, cw, ch));
                    // 中间平铺
                    let mut clip_g = g.create();
                    clip_g.clip_rect(&Rect::new(cw, ty, self.width - cw * 2, ch));
                    let tile_count = (self.width - cw * 2 + cw - 1) / cw;
                    for i in 0..tile_count {
                        clip_g.draw_image_src(&*self.track_image, cw + i * cw, ty, &Rect::new(cw, 0, cw, ch));
                    }
                    // 右端
                    g.draw_image_src(&*self.track_image, self.width - cw, ty, &Rect::new(cw * 2, 0, cw, ch));
                } else {
                    // 上端
                    g.draw_image_src(&*self.track_image, 0, 0, &Rect::new(0, 0, cw, ch));
                    // 中间平铺
                    let mut clip_g = g.create();
                    clip_g.clip_rect(&Rect::new(0, ch, cw, self.height - ch * 2));
                    let tile_count = (self.height - ch * 2 + ch - 1) / ch;
                    for i in 0..tile_count {
                        clip_g.draw_image_src(&*self.track_image, 0, ch + i * ch, &Rect::new(0, ch, cw, ch));
                    }
                    // 下端
                    g.draw_image_src(&*self.track_image, 0, self.height - ch, &Rect::new(0, ch * 2, cw, ch));
                }
            }
        }

        // 绘制滑块
        if !self.thumb_image.is_null() {
            unsafe {
                if self.horizontal {
                    let tx = (self.val * (self.width - (*self.thumb_image).get_width()) as f64) as i32;
                    let ty = (self.height - (*self.thumb_image).get_height()) / 2;
                    g.draw_image_xy(&*self.thumb_image, tx, ty);
                } else {
                    let tx = (self.width - (*self.thumb_image).get_width()) / 2;
                    let ty = (self.val * (self.height - (*self.thumb_image).get_height()) as f64) as i32;
                    g.draw_image_xy(&*self.thumb_image, tx, ty);
                }
            }
        }
    }

    /// 鼠标移动（对应 C++ MouseMove）
    pub fn mouse_move(&mut self, x: i32, y: i32, _wm: &mut WidgetManager) {
        // 简化版：光标变化逻辑暂略
    }

    /// 鼠标按下（对应 C++ MouseDown）
    pub fn mouse_down(&mut self, x: i32, y: i32, _click: i32) {
        if !self.thumb_image.is_null() {
            unsafe {
                if self.horizontal {
                    let thumb_x = (self.val * (self.width - (*self.thumb_image).get_width()) as f64) as i32;
                    if x >= thumb_x && x < thumb_x + (*self.thumb_image).get_width() {
                        self.dragging = true;
                        self.rel_x = x - thumb_x;
                        return;
                    }
                } else {
                    let thumb_y = (self.val * (self.height - (*self.thumb_image).get_height()) as f64) as i32;
                    if y >= thumb_y && y < thumb_y + (*self.thumb_image).get_height() {
                        self.dragging = true;
                        self.rel_y = y - thumb_y;
                        return;
                    }
                }
            }
        }
        // 点击轨道，直接跳转
        let pos = if self.horizontal { x as f64 / self.width as f64 } else { y as f64 / self.height as f64 };
        self.set_value(pos);
    }

    /// 鼠标拖拽（对应 C++ MouseDrag）
    pub fn mouse_drag(&mut self, x: i32, y: i32) {
        if !self.dragging || self.thumb_image.is_null() { return; }
        let old_val = self.val;
        unsafe {
            if self.horizontal {
                self.val = (x - self.rel_x) as f64 / (self.width - (*self.thumb_image).get_width()) as f64;
            } else {
                self.val = (y - self.rel_y) as f64 / (self.height - (*self.thumb_image).get_height()) as f64;
            }
        }
        if self.val < 0.0 { self.val = 0.0; }
        if self.val > 1.0 { self.val = 1.0; }
        if (self.val - old_val).abs() > f64::EPSILON {
            if let Some(ref mut l) = self.listener {
                l.slider_val(self.id, self.val);
            }
            self.mark_dirty_full();
        }
    }

    /// 鼠标释放（对应 C++ MouseUp）
    pub fn mouse_up(&mut self, _x: i32, _y: i32) {
        self.dragging = false;
        if let Some(ref mut l) = self.listener {
            l.slider_val(self.id, self.val);
        }
    }

    /// 鼠标离开（对应 C++ MouseLeave）
    pub fn mouse_leave(&mut self) {}

    fn mark_dirty_full(&mut self) {}
}
