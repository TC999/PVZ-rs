// PvZ Portable Rust 翻译 — DialogButton 对话框按钮
// 对应 C++ SexyAppFramework/widget/DialogButton.h / DialogButton.cpp
//
// DialogButton 继承 ButtonWidget，使用组件图片绘制带边框的对话框按钮。

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::graphics::image::Image;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::button_listener::ButtonListener;
use crate::framework::widget::insets::Insets;

/// 对话框按钮颜色索引（与 ButtonWidget 共用）
pub const DIALOG_BUTTON_COLOR_LABEL: usize = 0;
pub const DIALOG_BUTTON_COLOR_LABEL_HILITE: usize = 1;
pub const DIALOG_BUTTON_COLOR_DARK_OUTLINE: usize = 2;
pub const DIALOG_BUTTON_COLOR_LIGHT_OUTLINE: usize = 3;
pub const DIALOG_BUTTON_COLOR_MEDIUM_OUTLINE: usize = 4;
pub const DIALOG_BUTTON_COLOR_BKG: usize = 5;
pub const DIALOG_BUTTON_NUM_COLORS: usize = 6;

/// 默认对话框按钮颜色表（对应 C++ gDialogButtonColors）
const G_DIALOG_BUTTON_COLORS: [[u8; 3]; DIALOG_BUTTON_NUM_COLORS] = [
    [255, 255, 255], [255, 255, 255], [0, 0, 0],
    [255, 255, 255], [132, 132, 132], [212, 212, 212],
];

/// 对话框按钮（对应 C++ DialogButton）
/// 使用组件图片绘制，支持按下和悬停状态
pub struct DialogButton {
    // ---- Widget 基础字段 ----
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub widget_manager: Option<*mut WidgetManager>,
    pub has_alpha: bool, pub clip: bool, pub dirty: bool,
    pub zorder: i32, pub priority: i32,
    pub visible: bool, pub disabled: bool,
    pub has_focus: bool, pub is_down: bool, pub is_over: bool,
    pub has_transparencies: bool,
    pub colors: Vec<Color>,
    pub mouse_insets: Insets,
    pub do_finger: bool,
    pub label: String,
    pub label_justify: i32,
    pub font: Option<Box<Font>>,
    pub button_image: *mut Image,
    pub normal_rect: Rect,
    pub over_rect: Rect,
    pub down_rect: Rect,
    pub disabled_rect: Rect,
    pub over_alpha: f64,
    pub inverted: bool,
    pub btn_no_draw: bool,

    // ---- DialogButton 特有字段 ----
    /// 组件图片（背景图）
    pub component_image: *mut Image,
    /// 按下时的平移偏移
    pub translate_x: i32,
    pub translate_y: i32,
    /// 文字偏移
    pub text_offset_x: i32,
    pub text_offset_y: i32,
}

impl DialogButton {
    /// 创建对话框按钮（对应 C++ DialogButton 构造函数）
    pub fn new(
        component_image: *mut Image,
        the_id: i32,
        the_listener: Option<Box<dyn ButtonListener>>,
    ) -> Self {
        let mut db = DialogButton {
            x: 0, y: 0, width: 0, height: 0,
            widget_manager: None,
            has_alpha: true, clip: true, dirty: false,
            zorder: 0, priority: 0,
            visible: true, disabled: false,
            has_focus: false, is_down: false, is_over: false,
            has_transparencies: false,
            colors: Vec::new(),
            mouse_insets: Insets::new(0, 0, 0, 0),
            do_finger: true,
            label: String::new(),
            label_justify: 0, // BUTTON_LABEL_CENTER
            font: None,
            button_image: component_image,
            normal_rect: Rect::ZERO,
            over_rect: Rect::ZERO,
            down_rect: Rect::ZERO,
            disabled_rect: Rect::ZERO,
            over_alpha: 0.0,
            inverted: false,
            btn_no_draw: false,
            component_image,
            translate_x: 1,
            translate_y: 1,
            text_offset_x: 0,
            text_offset_y: 0,
        };
        // 设置默认颜色
        db.colors.clear();
        for c in &G_DIALOG_BUTTON_COLORS {
            db.colors.push(Color::from_rgb(c[0], c[1], c[2]));
        }
        let _ = the_id;
        let _ = the_listener;
        db
    }

    /// 判断按钮是否按下（对应 C++ IsButtonDown）
    pub fn is_button_down(&self) -> bool {
        self.is_down && self.is_over && !self.disabled
    }

    /// 绘制（对应 C++ Draw）
    pub fn draw(&mut self, g: &mut Graphics) {
        if self.btn_no_draw { return; }

        // 无组件图片时回退到基本绘制
        if self.component_image.is_null() {
            // 简化：仅绘制标签
            if let Some(ref font) = self.font {
                let fw = font.string_width(&self.label);
                let fx = (self.width - fw) / 2;
                let fy = (self.height + font.get_ascent() - font.ascent_padding - font.get_ascent() / 6 - 1) / 2;
                g.set_color(&self.get_color(DIALOG_BUTTON_COLOR_LABEL));
                g.draw_string(&self.label, fx + self.text_offset_x, fy + self.text_offset_y);
            }
            return;
        }

        // 确保字体
        if self.font.is_none() && !self.label.is_empty() {
            self.font = Some(Box::new(Font::new("DefaultButton", 12)));
        }

        let do_translate = self.is_button_down();

        if self.normal_rect.width == 0 {
            // 无矩形分割：直接绘制整个图片
            if do_translate {
                g.translate(self.translate_x, self.translate_y);
            }
            unsafe {
                g.draw_image_box_dest(
                    &Rect::new(0, 0, self.width, self.height),
                    &*self.component_image,
                );
            }
            if do_translate {
                g.translate(-self.translate_x, -self.translate_y);
            }
        } else {
            // 有矩形分割：根据状态选择对应的子矩形
            if self.disabled && self.disabled_rect.width > 0 && self.disabled_rect.height > 0 {
                unsafe {
                    g.draw_image_box(
                        &self.disabled_rect,
                        &Rect::new(0, 0, self.width, self.height),
                        &*self.component_image,
                    );
                }
            } else if self.is_button_down() {
                unsafe {
                    g.draw_image_box(
                        &self.down_rect,
                        &Rect::new(0, 0, self.width, self.height),
                        &*self.component_image,
                    );
                }
            } else if self.over_alpha > 0.0 {
                if self.over_alpha < 1.0 {
                    unsafe {
                        g.draw_image_box(
                            &self.normal_rect,
                            &Rect::new(0, 0, self.width, self.height),
                            &*self.component_image,
                        );
                    }
                }
                g.set_colorize_images(true);
                g.set_color(&Color::new(255, 255, 255, (self.over_alpha * 255.0) as u8));
                unsafe {
                    g.draw_image_box(
                        &self.over_rect,
                        &Rect::new(0, 0, self.width, self.height),
                        &*self.component_image,
                    );
                }
                g.set_colorize_images(false);
            } else if self.is_over {
                unsafe {
                    g.draw_image_box(
                        &self.over_rect,
                        &Rect::new(0, 0, self.width, self.height),
                        &*self.component_image,
                    );
                }
            } else {
                unsafe {
                    g.draw_image_box(
                        &self.normal_rect,
                        &Rect::new(0, 0, self.width, self.height),
                        &*self.component_image,
                    );
                }
            }

            // 按下状态需要平移
            if do_translate {
                g.translate(self.translate_x, self.translate_y);
            }
        }

        // 绘制文字
        if let Some(ref font) = self.font {
            let fw = font.string_width(&self.label);
            let fx = (self.width - fw) / 2;
            let fy = (self.height + font.get_ascent() - font.ascent_padding - font.get_ascent() / 6 - 1) / 2;

            let font_ref: &Font = &**font;
            let fptr = font_ref as *const Font as *mut Font;
            g.set_font(fptr);

            if self.is_over {
                g.set_color(&self.get_color(DIALOG_BUTTON_COLOR_LABEL_HILITE));
            } else {
                g.set_color(&self.get_color(DIALOG_BUTTON_COLOR_LABEL));
            }
            g.draw_string(&self.label, fx + self.text_offset_x, fy + self.text_offset_y);
        }

        // 恢复平移
        if do_translate && self.normal_rect.width != 0 {
            g.translate(-self.translate_x, -self.translate_y);
        }
    }

    fn get_color(&self, idx: usize) -> Color {
        self.colors.get(idx).copied().unwrap_or(Color::WHITE)
    }
}
