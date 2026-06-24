// PvZ Portable Rust 翻译 — ScrollbuttonWidget 滚动按钮
// 对应 C++ SexyAppFramework/widget/ScrollbuttonWidget.h / ScrollbuttonWidget.cpp
//
// 绘制带箭头的滚动按钮（上/下/左/右方向），使用 FillRect 手工绘制三角形箭头。

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::insets::Insets;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::button_listener::ButtonListener;

/// 滚动按钮（对应 C++ ScrollbuttonWidget）
pub struct ScrollbuttonWidget {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub widget_manager: Option<*mut WidgetManager>,
    pub visible: bool, pub disabled: bool,
    pub has_focus: bool, pub is_down: bool, pub is_over: bool,
    pub has_alpha: bool,
    pub colors: Vec<Color>,
    pub mouse_insets: Insets,
    pub do_finger: bool,
    pub label: String,
    pub button_image: *mut crate::framework::graphics::image::Image,
    /// 是否水平方向（false=垂直）
    pub horizontal: bool,
    /// 类型：1=上, 2=下, 3=左, 4=右（覆盖 horizontal）
    pub the_type: i32,
    /// 按钮ID
    pub id: i32,
}

impl ScrollbuttonWidget {
    pub fn new(the_id: i32, _listener: Option<Box<dyn ButtonListener>>, the_type: i32) -> Self {
        ScrollbuttonWidget {
            x: 0, y: 0, width: 0, height: 0,
            widget_manager: None,
            visible: true, disabled: false,
            has_focus: false, is_down: false, is_over: false,
            has_alpha: true,
            colors: Vec::new(),
            mouse_insets: Insets::new(0, 0, 0, 0),
            do_finger: false,
            label: String::new(),
            button_image: std::ptr::null_mut(),
            horizontal: false,
            the_type,
            id: the_id,
        }
    }

    /// 绘制箭头按钮（对应 C++ Draw）
    pub fn draw(&self, g: &mut Graphics) {
        let mut arrow_offset = 0i32;

        // 背景色
        g.set_color(&Color::new(212, 212, 212, 255));
        g.fill_rect_xywh(0, 0, self.width, self.height);

        if self.is_down && self.is_over && !self.disabled {
            arrow_offset = 1;
            g.set_color(&Color::new(132, 132, 132, 255));
            g.draw_rect(&Rect::new(0, 0, self.width - 1, self.height - 1));
        } else {
            // 上边/左边高光线
            g.set_color(&Color::new(255, 255, 255, 255));
            g.fill_rect_xywh(1, 1, self.width - 2, 1);
            g.fill_rect_xywh(1, 1, 1, self.height - 2);

            // 下边/右边暗影线
            g.set_color(&Color::BLACK);
            g.fill_rect_xywh(0, self.height - 1, self.width, 1);
            g.fill_rect_xywh(self.width - 1, 0, 1, self.height);

            // 中间暗影
            g.set_color(&Color::new(132, 132, 132, 255));
            g.fill_rect_xywh(1, self.height - 2, self.width - 2, 1);
            g.fill_rect_xywh(self.width - 2, 1, 1, self.height - 2);
        }

        // 箭头颜色
        if !self.disabled {
            g.set_color(&Color::BLACK);
        } else {
            g.set_color(&Color::new(132, 132, 132, 255));
        }

        // 绘制箭头（用 FillRect 逐像素画三角形）
        if self.horizontal || self.the_type == 3 || self.the_type == 4 {
            // 水平箭头（左/右）
            for i in 0..4 {
                if self.id == 0 || self.the_type == 3 {
                    // 左箭头
                    g.fill_rect_xywh(
                        i + (self.width - 4) / 2 + arrow_offset,
                        self.height / 2 - i - 1 + arrow_offset,
                        1, 1 + i * 2,
                    );
                } else {
                    // 右箭头
                    g.fill_rect_xywh(
                        (3 - i) + (self.width - 4) / 2 + arrow_offset,
                        self.height / 2 - i - 1 + arrow_offset,
                        1, 1 + i * 2,
                    );
                }
            }
        } else {
            // 垂直箭头（上/下）
            for i in 0..4 {
                if self.id == 0 || self.the_type == 1 {
                    // 上箭头
                    g.fill_rect_xywh(
                        self.width / 2 - i - 1 + arrow_offset,
                        i + (self.height - 4) / 2 + arrow_offset,
                        1 + i * 2, 1,
                    );
                } else {
                    // 下箭头
                    g.fill_rect_xywh(
                        self.width / 2 - i - 1 + arrow_offset,
                        (3 - i) + (self.height - 4) / 2 + arrow_offset,
                        1 + i * 2, 1,
                    );
                }
            }
        }
    }
}
