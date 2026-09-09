// PvZ Portable Rust 翻译 — GameButton 游戏按钮
// 对应 C++ src/Lawn/Widget/GameButton.h

#![allow(dead_code)]
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::graphics::font::Font;
use crate::framework::color::Color;
use crate::framework::widget::dialog_button::{DialogButton, DIALOG_BUTTON_COLOR_BKG, DIALOG_BUTTON_COLOR_LABEL, DIALOG_BUTTON_COLOR_LABEL_HILITE};

/// 设置 Graphics 当前字体（对应 C++ g->SetFont；字体可空）
fn set_button_font(g: &mut Graphics, font: Option<&Box<Font>>) {
    match font {
        Some(f) => g.set_font(f.as_ref() as *const Font as *mut Font),
        None => g.set_font(std::ptr::null_mut()),
    }
}

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
    // 对应 C++ mOverAlpha/mOverAlphaSpeed/mOverAlphaFadeInSpeed（悬停渐变）
    pub over_alpha: f32,
    pub over_alpha_speed: f32,
    pub over_alpha_fade_in_speed: f32,
    // 对应 C++ 皮肤链图片（图片资源未接入阶段均为 null，接入后回填）
    pub over_image: *mut Image,
    pub down_image: *mut Image,
    pub disabled_image: *mut Image,
    pub over_overlay_image: *mut Image,
    // 对应 C++ mNormalRect/mOverRect/mDownRect/mDisabledRect
    pub normal_rect: crate::framework::rect::Rect,
    pub over_rect: crate::framework::rect::Rect,
    pub down_rect: crate::framework::rect::Rect,
    pub disabled_rect: crate::framework::rect::Rect,
    // 对应 C++ mInverted / mDrawStoneButton
    pub inverted: bool,
    pub draw_stone_button: bool,
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
            over_alpha: 0.0,
            over_alpha_speed: 0.05,
            over_alpha_fade_in_speed: 0.15,
            over_image: std::ptr::null_mut(),
            down_image: std::ptr::null_mut(),
            disabled_image: std::ptr::null_mut(),
            over_overlay_image: std::ptr::null_mut(),
            normal_rect: crate::framework::rect::Rect::ZERO,
            over_rect: crate::framework::rect::Rect::ZERO,
            down_rect: crate::framework::rect::Rect::ZERO,
            disabled_rect: crate::framework::rect::Rect::ZERO,
            inverted: false,
            draw_stone_button: false,
        }
    }
    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) { self.x = x; self.y = y; self.width = w; self.height = h; }

    /// 皮肤图是否有效（对应 C++ HaveButtonImage：图非空且区域有效）
    fn have_button_image(&self, image: *mut Image, rect: &crate::framework::rect::Rect) -> bool {
        !image.is_null() && rect.width > 0 && rect.height > 0
    }

    /// 绘制按钮图（对应 C++ DrawButtonImage；图片未接入时为 null，跳过绘制）
    fn draw_button_image(&self, g: &mut Graphics, image: *mut Image, rect: &crate::framework::rect::Rect, offset_x: i32, offset_y: i32) {
        if image.is_null() {
            return;
        }
        // [TRANSLATION_NOTE]: C++ 中按 normal/over/down/disabled 四矩形取九宫格资源并绘制；
        // Rust 侧简化为整图绘制（图片资源接入后回填九宫格逻辑）
        unsafe {
            g.draw_image_xy(&*image, rect.x + offset_x, rect.y + offset_y);
        }
    }

    /// 绘制按钮（对应 C++ GameButton::Draw，GameButton.cpp:128-209）
    pub fn draw(&mut self, g: &mut Graphics) {
        if self.btn_no_draw { return; }

        // C++: bool isDown = IsButtonDown() ^ mInverted; bool isHighLighted = IsMouseOver();
        let is_down = (self.is_down && self.is_over) ^ self.inverted;
        let is_highlighted = self.is_over;

        if self.draw_stone_button {
            // 对应 C++: DrawStoneButton(g, mX, mY, mWidth, mHeight, isDown, isHighLighted, mLabel)
            let get_image = |a_key: &str| -> *mut crate::framework::graphics::image::Image {
                crate::lawn::lawn_app::LawnApp::instance().map_or(std::ptr::null_mut(), |app| {
                    let a_rm = match app.base.resource_manager {
                        Some(r) => r,
                        None => return std::ptr::null_mut(),
                    };
                    unsafe { (*a_rm).get_image(a_key).as_image_ptr() }
                })
            };
            draw_stone_button(
                g,
                self.x, self.y, self.width, self.height,
                is_down, is_highlighted, &self.label,
                get_image("IMAGE_BUTTON_LEFT"), get_image("IMAGE_BUTTON_MIDDLE"), get_image("IMAGE_BUTTON_RIGHT"),
                get_image("IMAGE_BUTTON_DOWN_LEFT"), get_image("IMAGE_BUTTON_DOWN_MIDDLE"), get_image("IMAGE_BUTTON_DOWN_RIGHT"),
            );
            return;
        }

        g.translate(self.x, self.y);

        // C++: if (!mFont && mLabel.size() > 0) mFont = FONT_PICO129->Duplicate();
        // [TRANSLATION_NOTE]: Rust 无全局 FONT_PICO129 字体；未设字体时不再提前 return
        //（皮肤链图片仍绘制，文字按无字体跳过）

        let mut a_font_x = self.text_offset_x;
        let mut a_font_y = self.text_offset_y;
        if let Some(ref font) = self.font {
            if self.label_justify == GameButton::BUTTON_LABEL_CENTER {
                a_font_x += (self.width - font.string_width(&self.label)) / 2;
            } else if self.label_justify == GameButton::BUTTON_LABEL_RIGHT {
                a_font_x += self.width - font.string_width(&self.label);
            }
            a_font_y += (self.height - font.get_ascent() / 6 + font.get_ascent() - 1) / 2;
        }
        set_button_font(g, self.font.as_ref());

        if !is_down {
            // C++: 未按下分支：disabled → over 渐变混合 → highlighted over → normal
            if self.disabled && self.have_button_image(self.disabled_image, &self.disabled_rect) {
                self.draw_button_image(g, self.disabled_image, &self.disabled_rect, 0, 0);
            } else if self.over_alpha > 0.0 && self.have_button_image(self.over_image, &self.over_rect) {
                // C++: 渐变过渡未完成时先画 normal，再叠加 over（带 alpha）
                if self.have_button_image(self.button_image, &self.normal_rect) && self.over_alpha < 1.0 {
                    self.draw_button_image(g, self.button_image, &self.normal_rect, 0, 0);
                }
                g.set_colorize_images(true);
                g.set_color(&Color::new(255, 255, 255, (self.over_alpha * 255.0) as u8));
                self.draw_button_image(g, self.over_image, &self.over_rect, 0, 0);
                g.set_colorize_images(false);
            } else if is_highlighted && self.have_button_image(self.over_image, &self.over_rect) {
                self.draw_button_image(g, self.over_image, &self.over_rect, 0, 0);
            } else if self.have_button_image(self.button_image, &self.normal_rect) {
                self.draw_button_image(g, self.button_image, &self.normal_rect, 0, 0);
            }

            g.set_color(&self.colors[if is_highlighted {
                GameButton::COLOR_LABEL_HILITE
            } else {
                GameButton::COLOR_LABEL
            }]);
            g.draw_string(&self.label, a_font_x, a_font_y);

            if is_highlighted && !self.over_overlay_image.is_null() {
                g.set_draw_mode(1); // Graphics::DRAWMODE_ADDITIVE
                self.draw_button_image(g, self.over_overlay_image, &self.normal_rect, 0, 0);
                g.set_draw_mode(0); // Graphics::DRAWMODE_NORMAL
            }
        } else {
            // C++: 按下分支：down 图（或 over/normal + 1,1 位移）+ 文字 +1/+1 + HILITE 色
            if self.have_button_image(self.down_image, &self.down_rect) {
                self.draw_button_image(g, self.down_image, &self.down_rect, 0, 0);
            } else if self.have_button_image(self.over_image, &self.over_rect) {
                self.draw_button_image(g, self.over_image, &self.over_rect, 1, 1);
            } else {
                self.draw_button_image(g, self.button_image, &self.normal_rect, 1, 1);
            }

            g.set_color(&self.colors[GameButton::COLOR_LABEL_HILITE]);
            g.draw_string(&self.label, a_font_x + 1, a_font_y + 1);

            if is_highlighted && !self.over_overlay_image.is_null() {
                g.set_draw_mode(1); // DRAWMODE_ADDITIVE
                self.draw_button_image(g, self.over_overlay_image, &self.normal_rect, 0, 0);
                g.set_draw_mode(0); // DRAWMODE_NORMAL
            }
        }

        g.translate(-self.x, -self.y);
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

    /// 设置标签（对应 C++ SetLabel）
    pub fn set_label(&mut self, the_label: &str) {
        self.label = crate::todlib::tod_common::tod_string_translate(the_label);
    }

    /// 设置禁用（对应 C++ SetDisabled）
    pub fn set_disabled(&mut self, the_disabled: bool) {
        self.disabled = the_disabled;
    }

    /// 更新按钮（对应 C++ Update：悬停渐变）
    pub fn update(&mut self) {
        // [TRANSLATION_NOTE]: C++ 中 mIsOver = IsMouseOver() 且 mIsDown 由
        // WidgetManager::mDownButtons 驱动；Rust 侧 is_over 由 mouse_enter/leave 维护。
        if !self.is_down && !self.is_over && self.over_alpha > 0.0 {
            if self.over_alpha_speed < 0.0 {
                self.over_alpha = 0.0;
                return;
            }
            self.over_alpha -= self.over_alpha_speed;
            if self.over_alpha < 0.0 {
                self.over_alpha = 0.0;
            }
        } else if self.is_over && self.over_alpha_fade_in_speed > 0.0 && self.over_alpha < 1.0 {
            if self.over_alpha_fade_in_speed > 0.0 {
                self.over_alpha += self.over_alpha_fade_in_speed;
                if self.over_alpha > 1.0 {
                    self.over_alpha = 1.0;
                }
            } else {
                self.over_alpha = 1.0;
            }
        }
    }
}

/// 判断按钮是否有可用图片（对应 C++ ButtonWidget::HaveButtonImage）
fn have_button_image(the_image: *mut Image, the_rect: &crate::framework::rect::Rect) -> bool {
    !the_image.is_null() || the_rect.width != 0
}

/// 绘制按钮图片（对应 C++ ButtonWidget::DrawButtonImage：rect.width != 0 时用 mButtonImage）
fn draw_button_image(
    g: &mut Graphics,
    button_image: *mut Image,
    the_image: *mut Image,
    the_rect: &crate::framework::rect::Rect,
    x: i32,
    y: i32,
) {
    if the_rect.width != 0 {
        if !button_image.is_null() {
            unsafe {
                g.draw_image_src(&*button_image, x, y, the_rect);
            }
        }
    } else if !the_image.is_null() {
        unsafe {
            g.draw_image_xy(&*the_image, x, y);
        }
    }
}

/// 石头按钮绘制（对应 C++ DrawStoneButton，GameButton.cpp:34）
/// 按状态选左/中/右三段图片重复铺满，再居中绘制标签文字
fn draw_stone_button(
    g: &mut Graphics,
    x: i32, y: i32, the_width: i32, the_height: i32,
    is_down: bool, _is_high_lighted: bool, the_label: &str,
    a_left_img: *mut Image, a_middle_img: *mut Image, a_right_img: *mut Image,
    a_down_left_img: *mut Image, a_down_middle_img: *mut Image, a_down_right_img: *mut Image,
) {
    // [TRANSLATION_NOTE]: C++ 用 isHighLighted 选择 FONT_DWARVENTODCRAFT18BRIGHTGREENINSET，
    // Rust 无该字体资源，故参数保留但未使用
    let _ = _is_high_lighted;
    // [TRANSLATION_NOTE]: C++ 使用全局图片 Sexy::IMAGE_BUTTON_*；Rust 图片资源未接入时传空指针跳过图片绘制
    let (mut a_left, mut a_middle, mut a_right) = (a_left_img, a_middle_img, a_right_img);
    let mut a_font_x = x;
    let mut a_font_y = y;
    let mut a_image_x = x;
    if is_down {
        a_left = a_down_left_img;
        a_middle = a_down_middle_img;
        a_right = a_down_right_img;
        a_font_x += 1;
        a_font_y += 1;
        a_image_x += 1;
    }

    if !a_left.is_null() && !a_middle.is_null() && !a_right.is_null() {
        unsafe {
            let a_left_img = &*a_left;
            let a_middle_img = &*a_middle;
            let a_right_img = &*a_right;
            let a_left_width = a_left_img.width;
            let a_middle_width = a_middle_img.width;
            let a_right_width = a_right_img.width;

            let mut a_repeat = (the_width - a_left_width - a_right_width) / a_middle_width.max(1);
            g.draw_image_xy(a_left_img, a_image_x, y);
            a_image_x += a_left_width;
            while a_repeat > 0 {
                g.draw_image_xy(a_middle_img, a_image_x, y);
                a_image_x += a_middle_width;
                a_repeat -= 1;
            }
            g.draw_image_xy(a_right_img, a_image_x, y);
        }
    }

    // C++: g->SetFont(isHighLighted ? FONT_DWARVENTODCRAFT18BRIGHTGREENINSET : FONT_DWARVENTODCRAFT18GREENINSET);
    // [TRANSLATION_NOTE]: Rust 无对应字体资源，使用 Graphics 当前字体近似
    let font = g.get_font();
    if !font.is_null() && !the_label.is_empty() {
        unsafe {
            let a_font = &*font;
            let a_str_width = a_font.string_width(the_label);
            let a_ascent = a_font.get_ascent();
            a_font_x += (the_width - a_str_width) / 2 + 1;
            a_font_y += (the_height - a_ascent / 6 - 1 + a_ascent) / 2 - 4;
        }
    }
    g.set_color(&Color::new(255, 255, 255, 255));
    g.draw_string(the_label, a_font_x, a_font_y);
}

/// 石头风格按钮（对应 C++ LawnStoneButton : DialogButton）
pub struct LawnStoneButton {
    pub dialog_button: DialogButton,
}

impl LawnStoneButton {
    pub fn draw(&self, g: &mut Graphics) {
        // 对应 C++ LawnStoneButton::Draw（GameButton.cpp:289）
        if self.dialog_button.btn_no_draw {
            return;
        }
        let is_down = (self.dialog_button.is_down && self.dialog_button.is_over && !self.dialog_button.disabled)
            ^ self.dialog_button.inverted;
        // [TRANSLATION_NOTE]: C++ 在此使用全局 IMAGE_BUTTON_* 图片；Rust 图片资源未接入，传空指针
        draw_stone_button(
            g,
            0, 0, self.dialog_button.width, self.dialog_button.height,
            is_down, self.dialog_button.is_over, &self.dialog_button.label,
            std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
            std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
        );
    }

    pub fn set_label(&mut self, the_label: &str) {
        // 对应 C++ LawnStoneButton::SetLabel（GameButton.cpp:284）
        self.dialog_button.label = crate::todlib::tod_common::tod_string_translate(the_label);
    }
}

/// 创建新型按钮（对应 C++ MakeNewButton）
pub fn make_new_button(_id: i32, _listener: Option<Box<dyn crate::framework::widget::button_listener::ButtonListener>>, text: &str) -> NewLawnButton {
    let mut btn = NewLawnButton::new();
    btn.set_label(text);
    btn
}

/// 创建石头风格按钮（对应 C++ MakeButton）
pub fn make_button(id: i32, listener: Option<Box<dyn crate::framework::widget::button_listener::ButtonListener>>, text: &str) -> LawnStoneButton {
    let mut btn = LawnStoneButton {
        dialog_button: DialogButton::new(std::ptr::null_mut(), id, listener),
    };
    btn.set_label(text);
    btn.dialog_button.has_alpha = true;
    btn.dialog_button.has_transparencies = true;
    btn.dialog_button.height = 33;
    btn
}

/// 新型按钮（对应 C++ NewLawnButton : DialogButton）
pub struct NewLawnButton {
    pub dialog_button: DialogButton,
    pub hilite_font: Option<Box<Font>>,
    pub text_down_offset_x: i32,
    pub text_down_offset_y: i32,
    pub button_offset_x: i32,
    pub button_offset_y: i32,
    pub use_polygon_shape: bool,
    pub polygon_shape: [(f64, f64); 4],
}

impl NewLawnButton {
    pub fn new() -> Self {
        NewLawnButton {
            dialog_button: DialogButton::new(std::ptr::null_mut(), 0, None),
            hilite_font: None,
            text_down_offset_x: 0,
            text_down_offset_y: 0,
            button_offset_x: 0,
            button_offset_y: 0,
            use_polygon_shape: false,
            polygon_shape: [(0.0, 0.0); 4],
        }
    }

    pub fn draw(&self, g: &mut Graphics) {
        // 对应 C++ NewLawnButton::Draw（GameButton.cpp:328）
        if self.dialog_button.btn_no_draw {
            return;
        }

        // C++: bool isDown = (mIsDown && mIsOver && !mDisabled) ^ mInverted;
        let is_down = (self.dialog_button.is_down && self.dialog_button.is_over && !self.dialog_button.disabled)
            ^ self.dialog_button.inverted;

        // C++: aFontX/Y = mTextOffsetX/Y + mTranslateX/Y
        let mut a_font_x = self.dialog_button.text_offset_x + self.dialog_button.translate_x;
        let mut a_font_y = self.dialog_button.text_offset_y + self.dialog_button.translate_y;
        if let Some(ref font) = self.dialog_button.font {
            // C++: BUTTON_LABEL_CENTER=0, BUTTON_LABEL_RIGHT=1
            if self.dialog_button.label_justify == 0 {
                a_font_x += (self.dialog_button.width - font.string_width(&self.dialog_button.label)) / 2;
            } else if self.dialog_button.label_justify == 1 {
                a_font_x += self.dialog_button.width - font.string_width(&self.dialog_button.label);
            }
            a_font_y += (self.dialog_button.height - font.get_ascent() / 6 + font.get_ascent() - 1) / 2;
        }

        g.set_colorize_images(true);
        if !is_down {
            g.set_color(&self.dialog_button.colors[DIALOG_BUTTON_COLOR_BKG]);
            if self.dialog_button.disabled && have_button_image(self.dialog_button.disabled_image, &self.dialog_button.disabled_rect) {
                draw_button_image(
                    g, self.dialog_button.button_image, self.dialog_button.disabled_image,
                    &self.dialog_button.disabled_rect, self.button_offset_x, self.button_offset_y,
                );
            } else if self.dialog_button.over_alpha > 0.0 && have_button_image(self.dialog_button.over_image, &self.dialog_button.over_rect) {
                // C++: 淡入过渡尚未完成时先绘制正常图片
                if have_button_image(self.dialog_button.button_image, &self.dialog_button.normal_rect) && self.dialog_button.over_alpha < 1.0 {
                    draw_button_image(
                        g, self.dialog_button.button_image, self.dialog_button.button_image,
                        &self.dialog_button.normal_rect, self.button_offset_x, self.button_offset_y,
                    );
                }
                // C++: g->mColor.mAlpha = mOverAlpha * 255
                let mut a_color = *g.get_color();
                a_color.a = (self.dialog_button.over_alpha * 255.0) as u8;
                g.set_color(&a_color);
                draw_button_image(
                    g, self.dialog_button.button_image, self.dialog_button.over_image,
                    &self.dialog_button.over_rect, self.button_offset_x, self.button_offset_y,
                );
            } else if (self.dialog_button.is_over || self.dialog_button.is_down)
                && have_button_image(self.dialog_button.over_image, &self.dialog_button.over_rect)
            {
                draw_button_image(
                    g, self.dialog_button.button_image, self.dialog_button.over_image,
                    &self.dialog_button.over_rect, self.button_offset_x, self.button_offset_y,
                );
            } else if have_button_image(self.dialog_button.button_image, &self.dialog_button.normal_rect) {
                draw_button_image(
                    g, self.dialog_button.button_image, self.dialog_button.button_image,
                    &self.dialog_button.normal_rect, self.button_offset_x, self.button_offset_y,
                );
            }

            g.set_colorize_images(false);
            // C++: 悬停用高亮字体与高亮颜色，否则用普通字体与标签颜色
            if self.dialog_button.is_over {
                set_button_font(g, self.hilite_font.as_ref().or(self.dialog_button.font.as_ref()));
                g.set_color(&self.dialog_button.colors[DIALOG_BUTTON_COLOR_LABEL_HILITE]);
            } else {
                set_button_font(g, self.dialog_button.font.as_ref());
                g.set_color(&self.dialog_button.colors[DIALOG_BUTTON_COLOR_LABEL]);
            }
            g.draw_string(&self.dialog_button.label, a_font_x, a_font_y);
        } else {
            g.set_color(&self.dialog_button.colors[DIALOG_BUTTON_COLOR_BKG]);
            if have_button_image(self.dialog_button.down_image, &self.dialog_button.down_rect) {
                draw_button_image(
                    g, self.dialog_button.button_image, self.dialog_button.down_image,
                    &self.dialog_button.down_rect,
                    self.button_offset_x + self.dialog_button.translate_x,
                    self.button_offset_y + self.dialog_button.translate_y,
                );
            } else if have_button_image(self.dialog_button.over_image, &self.dialog_button.over_rect) {
                draw_button_image(
                    g, self.dialog_button.button_image, self.dialog_button.over_image,
                    &self.dialog_button.over_rect,
                    self.button_offset_x + self.dialog_button.translate_x,
                    self.button_offset_y + self.dialog_button.translate_y,
                );
            } else {
                draw_button_image(
                    g, self.dialog_button.button_image, self.dialog_button.button_image,
                    &self.dialog_button.normal_rect,
                    self.button_offset_x + self.dialog_button.translate_x,
                    self.button_offset_y + self.dialog_button.translate_y,
                );
            }

            g.set_colorize_images(false);
            set_button_font(g, self.hilite_font.as_ref().or(self.dialog_button.font.as_ref()));
            g.set_color(&self.dialog_button.colors[DIALOG_BUTTON_COLOR_LABEL_HILITE]);
            g.draw_string(&self.dialog_button.label, a_font_x + self.text_down_offset_x, a_font_y + self.text_down_offset_y);
        }
    }

    pub fn is_point_visible(&self, _x: i32, _y: i32) -> bool {
        true
    }

    pub fn set_label(&mut self, the_label: &str) {
        // 对应 C++ NewLawnButton::SetLabel（GameButton.cpp:279）
        self.dialog_button.label = crate::todlib::tod_common::tod_string_translate(the_label);
    }

    pub fn set_offset(&mut self, x: i32, y: i32) {
        self.button_offset_x = x;
        self.button_offset_y = y;
    }
}
