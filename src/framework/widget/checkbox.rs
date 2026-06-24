// PvZ Portable Rust 翻译 — Checkbox 复选框控件
// 对应 C++ SexyAppFramework/widget/Checkbox.h / Checkbox.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::widget::insets::Insets;
use crate::framework::widget::widget_manager::WidgetManager;

/// 复选框监听器 trait（对应 C++ CheckboxListener）
pub trait CheckboxListener {
    fn checkbox_checked(&mut self, id: i32, checked: bool);
}

/// 复选框控件（对应 C++ Checkbox）
pub struct Checkbox {
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
    pub checked: bool,
    pub unchecked_image: *mut Image,
    pub checked_image: *mut Image,
    pub checked_rect: Rect,
    pub unchecked_rect: Rect,
    pub outline_color: Color,
    pub bkg_color: Color,
    pub check_color: Color,
    pub listener: Option<Box<dyn CheckboxListener>>,
}

impl Checkbox {
    pub fn new(
        unchecked_img: *mut Image, checked_img: *mut Image,
        the_id: i32, the_listener: Option<Box<dyn CheckboxListener>>,
    ) -> Self {
        Checkbox {
            x: 0, y: 0, width: 0, height: 0,
            widget_manager: None,
            visible: true, disabled: false,
            has_focus: false, is_down: false, is_over: false,
            has_alpha: true, has_transparencies: false,
            colors: Vec::new(),
            mouse_insets: Insets::new(0, 0, 0, 0),
            do_finger: true,
            wants_focus: false,
            id: the_id,
            checked: false,
            unchecked_image: unchecked_img,
            checked_image: checked_img,
            checked_rect: Rect::ZERO,
            unchecked_rect: Rect::ZERO,
            outline_color: Color::WHITE,
            bkg_color: Color::new(80, 80, 80, 255),
            check_color: Color::new(255, 255, 0, 255),
            listener: the_listener,
        }
    }

    /// 设置选中状态（对应 C++ SetChecked）
    pub fn set_checked(&mut self, checked: bool, tell_listener: bool) {
        self.checked = checked;
        if tell_listener {
            if let Some(ref mut l) = self.listener {
                l.checkbox_checked(self.id, self.checked);
            }
        }
        self.mark_dirty();
    }

    /// 是否选中（对应 C++ IsChecked）
    pub fn is_checked(&self) -> bool { self.checked }

    /// 绘制（对应 C++ Draw）
    pub fn draw(&self, g: &mut Graphics) {
        if self.checked_rect.width == 0 && !self.checked_image.is_null() && !self.unchecked_image.is_null() {
            // 直接绘制整张图片
            if self.checked {
                unsafe { g.draw_image_xy(&*self.checked_image, 0, 0); }
            } else {
                unsafe { g.draw_image_xy(&*self.unchecked_image, 0, 0); }
            }
        } else if self.checked_rect.width != 0 && !self.unchecked_image.is_null() {
            // 从图集中截取对应矩形
            if self.checked {
                unsafe { g.draw_image_src(&*self.unchecked_image, 0, 0, &self.checked_rect); }
            } else {
                unsafe { g.draw_image_src(&*self.unchecked_image, 0, 0, &self.unchecked_rect); }
            }
        } else if self.unchecked_image.is_null() && self.checked_image.is_null() {
            // 无图片：用颜色绘制
            g.set_color(&self.outline_color);
            g.fill_rect_xywh(0, 0, self.width, self.height);
            g.set_color(&self.bkg_color);
            g.fill_rect_xywh(1, 1, self.width - 2, self.height - 2);

            if self.checked {
                g.set_color(&self.check_color);
                g.draw_line(1, 1, self.width - 2, self.height - 2);
                g.draw_line(self.width - 1, 1, 1, self.height - 2);
            }
        }
    }

    /// 鼠标按下（对应 C++ MouseDown）
    pub fn mouse_down_btn(&mut self, _x: i32, _y: i32, _btn: i32, _click: i32) {
        self.checked = !self.checked;
        if let Some(ref mut l) = self.listener {
            l.checkbox_checked(self.id, self.checked);
        }
        self.mark_dirty();
    }

    fn mark_dirty(&mut self) {
        // 简单标记
        self.has_alpha = self.has_alpha;
    }
}
