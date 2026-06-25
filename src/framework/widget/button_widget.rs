// PvZ Portable Rust 翻译 — ButtonWidget 按钮控件
// 对应 C++ SexyAppFramework/widget/ButtonWidget.h / ButtonWidget.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::graphics::image::Image;
use crate::framework::widget::widget::Widget;
use crate::framework::widget::button_listener::ButtonListener;

/// 按钮标签对齐方式
pub const BUTTON_LABEL_LEFT: i32 = -1;
pub const BUTTON_LABEL_CENTER: i32 = 0;
pub const BUTTON_LABEL_RIGHT: i32 = 1;

/// 按钮颜色索引
pub const COLOR_LABEL: usize = 0;
pub const COLOR_LABEL_HILITE: usize = 1;
pub const COLOR_DARK_OUTLINE: usize = 2;
pub const COLOR_LIGHT_OUTLINE: usize = 3;
pub const COLOR_MEDIUM_OUTLINE: usize = 4;
pub const COLOR_BKG: usize = 5;
pub const NUM_COLORS: usize = 6;

/// 按钮控件（对应 C++ ButtonWidget，继承 Widget）
pub struct ButtonWidget {
    // ---- Widget 基础字段 ----
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub parent: Option<*mut Widget>,
    pub widget_manager: Option<*mut crate::framework::widget::widget_manager::WidgetManager>,
    pub has_alpha: bool, pub clip: bool, pub dirty: bool,
    pub zorder: i32, pub priority: i32,
    pub visible: bool, pub mouse_visible: bool, pub disabled: bool,
    pub has_focus: bool, pub is_down: bool, pub is_over: bool,
    pub has_transparencies: bool,
    pub colors: Vec<Color>,
    pub mouse_insets: crate::framework::widget::insets::Insets,
    pub do_finger: bool, pub wants_focus: bool,
    pub tab_prev: Option<*mut Widget>, pub tab_next: Option<*mut Widget>,
    pub loaded_resource_names: Vec<String>,

    // ---- ButtonWidget 特有字段 ----
    /// 按钮 ID
    pub id: i32,
    /// 标签文字
    pub label: String,
    /// 标签对齐方式
    pub label_justify: i32,
    /// 字体
    pub font: Option<Box<Font>>,
    /// 各状态的图片
    pub button_image: *mut Image,
    pub over_image: *mut Image,
    pub down_image: *mut Image,
    pub disabled_image: *mut Image,
    /// 各状态的纹理矩形
    pub normal_rect: Rect,
    pub over_rect: Rect,
    pub down_rect: Rect,
    pub disabled_rect: Rect,
    /// 是否反转按下状态
    pub inverted: bool,
    /// 是否不绘制按钮（隐藏绘制）
    pub btn_no_draw: bool,
    /// 是否不绘制边框
    pub frame_no_draw: bool,
    /// 按钮监听器
    pub button_listener: Option<Box<dyn ButtonListener>>,
    /// 悬停透明度动画
    pub over_alpha: f64,
    pub over_alpha_speed: f64,
    pub over_alpha_fade_in_speed: f64,
}

/// 默认按钮颜色表（对应 C++ gButtonWidgetColors）
const DEFAULT_BUTTON_COLORS: [[u8; 3]; NUM_COLORS] = [
    [0, 0, 0],       // COLOR_LABEL
    [0, 0, 0],       // COLOR_LABEL_HILITE
    [0, 0, 0],       // COLOR_DARK_OUTLINE
    [255, 255, 255], // COLOR_LIGHT_OUTLINE
    [132, 132, 132], // COLOR_MEDIUM_OUTLINE
    [212, 212, 212], // COLOR_BKG
];

impl ButtonWidget {
    pub fn new(the_id: i32, the_listener: Option<Box<dyn ButtonListener>>) -> Self {
        let mut bw = ButtonWidget {
            x: 0, y: 0, width: 0, height: 0,
            parent: None, widget_manager: None,
            has_alpha: true, clip: true, dirty: false,
            zorder: 0, priority: 0,
            visible: true, mouse_visible: true, disabled: false,
            has_focus: false, is_down: false, is_over: false,
            has_transparencies: false,
            colors: Vec::new(),
            mouse_insets: crate::framework::widget::insets::Insets::new(0, 0, 0, 0),
            do_finger: false, wants_focus: false,
            tab_prev: None, tab_next: None,
            loaded_resource_names: Vec::new(),
            id: the_id,
            label: String::new(),
            label_justify: BUTTON_LABEL_CENTER,
            font: None,
            button_image: std::ptr::null_mut(),
            over_image: std::ptr::null_mut(),
            down_image: std::ptr::null_mut(),
            disabled_image: std::ptr::null_mut(),
            normal_rect: Rect::ZERO,
            over_rect: Rect::ZERO,
            down_rect: Rect::ZERO,
            disabled_rect: Rect::ZERO,
            inverted: false,
            btn_no_draw: false,
            frame_no_draw: false,
            button_listener: the_listener,
            over_alpha: 0.0,
            over_alpha_speed: 0.0,
            over_alpha_fade_in_speed: 0.0,
        };
        bw.set_colors_rgb(&DEFAULT_BUTTON_COLORS);
        bw
    }

    /// 设置字体
    pub fn set_font(&mut self, the_font: &Font) {
        self.font = Some(Box::new(Font::new(
            &the_font.name, the_font.size,
        )));
    }

    /// 判断按钮是否按下
    pub fn is_button_down(&self) -> bool {
        self.is_down && self.is_over && !self.disabled
    }

    /// 判断是否有按钮图片
    pub fn have_button_image(img: *mut Image, rect: &Rect) -> bool {
        !img.is_null() || rect.width != 0
    }

    /// 绘制按钮图片
    pub fn draw_button_image(&self, g: &mut Graphics, img: *mut Image, rect: &Rect, x: i32, y: i32) {
        if rect.width != 0 {
            unsafe {
                g.draw_image_src(&*img, x, y, rect);
            }
        } else if !img.is_null() {
            unsafe {
                g.draw_image_xy(&*img, x, y);
            }
        }
    }

    /// 绘制（对应 C++ Draw）
    pub fn draw(&mut self, g: &mut Graphics) {
        if self.btn_no_draw { return; }

        let is_down = (self.is_down && self.is_over && !self.disabled) ^ self.inverted;

        let mut font_x = 0i32; // BUTTON_LABEL_LEFT
        let mut font_y = 0i32;

        if let Some(ref font) = self.font {
            font_x = match self.label_justify {
                BUTTON_LABEL_CENTER => (self.width - font.string_width(&self.label)) / 2,
                BUTTON_LABEL_RIGHT => self.width - font.string_width(&self.label),
                _ => 0,
            };
            font_y = (self.height + font.get_ascent() - font.get_ascent() / 6 - 1) / 2;

            // 设置字体到 Graphics（对应 C++ g->SetFont(mFont)）
            let fptr = &**font as *const Font as *mut Font;
            g.set_font(fptr);
        }

        if self.button_image.is_null() && self.down_image.is_null() {
            // 无图片模式：绘制简单边框和文字
            if !self.frame_no_draw {
                g.set_color(&self.get_color(COLOR_BKG));
                g.fill_rect_xywh(0, 0, self.width, self.height);
            }

            if is_down {
                if !self.frame_no_draw {
                    g.set_color(&self.get_color(COLOR_DARK_OUTLINE));
                    g.fill_rect_xywh(0, 0, self.width - 1, 1);
                    g.fill_rect_xywh(0, 0, 1, self.height - 1);
                    g.set_color(&self.get_color(COLOR_LIGHT_OUTLINE));
                    g.fill_rect_xywh(0, self.height - 1, self.width, 1);
                    g.fill_rect_xywh(self.width - 1, 0, 1, self.height);
                    g.set_color(&self.get_color(COLOR_MEDIUM_OUTLINE));
                    g.fill_rect_xywh(1, 1, self.width - 3, 1);
                    g.fill_rect_xywh(1, 1, 1, self.height - 3);
                }
                if self.is_over {
                    g.set_color(&self.get_color(COLOR_LABEL_HILITE));
                } else {
                    g.set_color(&self.get_color(COLOR_LABEL));
                }
                g.draw_string(&self.label, font_x + 1, font_y + 1);
            } else {
                if !self.frame_no_draw {
                    g.set_color(&self.get_color(COLOR_LIGHT_OUTLINE));
                    g.fill_rect_xywh(0, 0, self.width - 1, 1);
                    g.fill_rect_xywh(0, 0, 1, self.height - 1);
                    g.set_color(&self.get_color(COLOR_DARK_OUTLINE));
                    g.fill_rect_xywh(0, self.height - 1, self.width, 1);
                    g.fill_rect_xywh(self.width - 1, 0, 1, self.height);
                    g.set_color(&self.get_color(COLOR_MEDIUM_OUTLINE));
                    g.fill_rect_xywh(1, self.height - 2, self.width - 2, 1);
                    g.fill_rect_xywh(self.width - 2, 1, 1, self.height - 2);
                }
                if self.is_over {
                    g.set_color(&self.get_color(COLOR_LABEL_HILITE));
                } else {
                    g.set_color(&self.get_color(COLOR_LABEL));
                }
                g.draw_string(&self.label, font_x, font_y);
            }
        } else {
            // 有图片模式
            if !is_down {
                if self.disabled && Self::have_button_image(self.disabled_image, &self.disabled_rect) {
                    self.draw_button_image(g, self.disabled_image, &self.disabled_rect, 0, 0);
                } else if self.over_alpha > 0.0 && Self::have_button_image(self.over_image, &self.over_rect) {
                    if Self::have_button_image(self.button_image, &self.normal_rect) && self.over_alpha < 1.0 {
                        self.draw_button_image(g, self.button_image, &self.normal_rect, 0, 0);
                    }
                    g.set_colorize_images(true);
                    g.set_color(&Color::new(255, 255, 255, (self.over_alpha * 255.0) as u8));
                    self.draw_button_image(g, self.over_image, &self.over_rect, 0, 0);
                    g.set_colorize_images(false);
                } else if (self.is_over || self.is_down) && Self::have_button_image(self.over_image, &self.over_rect) {
                    self.draw_button_image(g, self.over_image, &self.over_rect, 0, 0);
                } else if Self::have_button_image(self.button_image, &self.normal_rect) {
                    self.draw_button_image(g, self.button_image, &self.normal_rect, 0, 0);
                }

                if self.is_over {
                    g.set_color(&self.get_color(COLOR_LABEL_HILITE));
                } else {
                    g.set_color(&self.get_color(COLOR_LABEL));
                }
                g.draw_string(&self.label, font_x, font_y);
            } else {
                if Self::have_button_image(self.down_image, &self.down_rect) {
                    self.draw_button_image(g, self.down_image, &self.down_rect, 0, 0);
                } else if Self::have_button_image(self.over_image, &self.over_rect) {
                    self.draw_button_image(g, self.over_image, &self.over_rect, 1, 1);
                } else {
                    self.draw_button_image(g, self.button_image, &self.normal_rect, 1, 1);
                }
                g.set_color(&self.get_color(COLOR_LABEL_HILITE));
                g.draw_string(&self.label, font_x + 1, font_y + 1);
            }
        }
    }

    /// 禁用（对应 C++ SetDisabled）
    pub fn set_disabled(&mut self, is_disabled: bool) {
        self.set_disabled_widget(is_disabled);
        if Self::have_button_image(self.disabled_image, &self.disabled_rect) {
            self.mark_dirty();
        }
    }

    /// 鼠标进入（对应 C++ MouseEnter）
    pub fn mouse_enter(&mut self) {
        if self.over_alpha_fade_in_speed == 0.0 && self.over_alpha > 0.0 {
            self.over_alpha = 0.0;
        }
        if self.is_down
            || Self::have_button_image(self.over_image, &self.over_rect)
            || self.get_color(COLOR_LABEL_HILITE) != self.get_color(COLOR_LABEL)
        {
            self.mark_dirty();
        }
        if let Some(ref mut listener) = self.button_listener {
            listener.button_mouse_enter(self.id);
        }
    }

    /// 鼠标离开（对应 C++ MouseLeave）
    pub fn mouse_leave(&mut self) {
        if self.over_alpha_speed == 0.0 && self.over_alpha > 0.0 {
            self.over_alpha = 0.0;
        } else if self.over_alpha_speed > 0.0 && self.over_alpha == 0.0 {
            self.over_alpha = 1.0;
        }
        if self.is_down
            || Self::have_button_image(self.over_image, &self.over_rect)
            || self.get_color(COLOR_LABEL_HILITE) != self.get_color(COLOR_LABEL)
        {
            self.mark_dirty();
        }
        if let Some(ref mut listener) = self.button_listener {
            listener.button_mouse_leave(self.id);
        }
    }

    /// 鼠标移动（对应 C++ MouseMove）
    pub fn mouse_move(&mut self, the_x: i32, the_y: i32) {
        if let Some(ref mut listener) = self.button_listener {
            listener.button_mouse_move(self.id, the_x, the_y);
        }
    }

    /// 鼠标按下（带按钮号，对应 C++ MouseDown）
    pub fn mouse_down_btn(&mut self, the_x: i32, the_y: i32, the_btn: i32, the_click: i32) {
        self.mouse_down_widget(the_x, the_y, the_click);
        if let Some(ref mut listener) = self.button_listener {
            listener.button_press(self.id);
        }
        self.mark_dirty();
    }

    /// 鼠标释放（带按钮号，对应 C++ MouseUp）
    pub fn mouse_up_btn(&mut self, the_x: i32, the_y: i32, the_btn: i32, the_click: i32) {
        self.mouse_up_widget(the_x, the_y);
        if self.is_over {
            if let Some(ref mut listener) = self.button_listener {
                listener.button_depress(self.id);
            }
        }
        self.mark_dirty();
    }

    /// 更新（对应 C++ Update）
    pub fn update(&mut self) {
        self.update_widget();

        if self.is_down && self.is_over {
            if let Some(ref mut listener) = self.button_listener {
                listener.button_down_tick(self.id);
            }
        }

        if !self.is_down && !self.is_over && self.over_alpha > 0.0 {
            if self.over_alpha_speed > 0.0 {
                self.over_alpha -= self.over_alpha_speed;
                if self.over_alpha < 0.0 { self.over_alpha = 0.0; }
            } else {
                self.over_alpha = 0.0;
            }
            self.mark_dirty();
        } else if self.is_over && self.over_alpha_fade_in_speed > 0.0 && self.over_alpha < 1.0 {
            self.over_alpha += self.over_alpha_fade_in_speed;
            if self.over_alpha > 1.0 { self.over_alpha = 1.0; }
            self.mark_dirty();
        }
    }

    // ---- 辅助方法（桥接到 Widget 方法） ----
    fn get_color(&self, idx: usize) -> Color {
        self.colors.get(idx).copied().unwrap_or(Color::WHITE)
    }
    fn set_colors_rgb(&mut self, colors: &[[u8; 3]]) {
        self.colors.clear();
        for c in colors { self.colors.push(Color::from_rgb(c[0], c[1], c[2])); }
    }
    fn mark_dirty(&mut self) {
        if let Some(p) = self.parent { unsafe { (*p).mark_dirty(); } }
        else { self.dirty = true; }
    }
    fn set_disabled_widget(&mut self, d: bool) {
        if self.disabled == d { return; }
        self.disabled = d;
        if d { /* WidgetManager interaction deferred */ }
        self.mark_dirty();
    }
    fn mouse_down_widget(&mut self, _x: i32, _y: i32, _click: i32) {}
    fn mouse_up_widget(&mut self, _x: i32, _y: i32) {}
    fn update_widget(&mut self) {}
}
