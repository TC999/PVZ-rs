// PvZ Portable Rust 翻译 — LawnDialog（草坪对话框）
// 对应 C++ src/Lawn/Widget/LawnDialog.h / LawnDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::dialog::Dialog;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::KeyCode;
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::color::Color;
use crate::lawn::game_enums::*;
use crate::todlib::reanimator::Reanimation;

/// 对话框头部偏移常量
pub const DIALOG_HEADER_OFFSET: i32 = 45;

/// 重动画小部件（对应 C++ ReanimationWidget）
pub struct ReanimationWidget {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub reanim: Option<*mut Reanimation>,
    pub lawn_dialog: Option<*mut LawnDialog>,
    pub pos_x: f32,
    pub pos_y: f32,
}

impl ReanimationWidget {
    pub fn new() -> Self {
        ReanimationWidget {
            app: None,
            reanim: None,
            lawn_dialog: None,
            pos_x: 0.0,
            pos_y: 0.0,
        }
    }

    pub fn dispose(&mut self) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn draw(&self, _g: &mut Graphics) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn update(&mut self) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn add_reanimation(&mut self, x: f32, y: f32, reanimation_type: ReanimationType) {
        // TODO: 从 LawnDialog.cpp 翻译
    }
}

impl Default for ReanimationWidget {
    fn default() -> Self {
        ReanimationWidget::new()
    }
}

/// 草坪对话框（对应 C++ LawnDialog，内嵌 Dialog 基类字段）
pub struct LawnDialog {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub button_delay: i32,
    pub reanimation: Option<*mut ReanimationWidget>,
    pub draw_standard_back: bool,
    pub lawn_yes_button: Option<*mut DialogButton>,
    pub lawn_no_button: Option<*mut DialogButton>,
    pub tall_bottom: bool,
    pub vertical_center_text: bool,
    // Dialog 基类字段（对应 C++ Dialog 成员）
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub colors: Vec<Color>,
    pub id: i32,
    pub is_modal: bool,
    pub result: i32,
    pub button_height: i32,
    pub button_horz_spacing: i32,
    pub dialog_header: String,
    pub dialog_footer: String,
    pub dialog_lines: String,
    pub text_align: i32,
    pub line_spacing_offset: i32,
    pub space_after_header: i32,
    pub content_insets: crate::framework::widget::insets::Insets,
    pub background_insets: crate::framework::widget::insets::Insets,
    pub header_font: Option<Box<crate::framework::graphics::font::Font>>,
    pub lines_font: Option<Box<crate::framework::graphics::font::Font>>,
}

impl LawnDialog {
    pub fn new() -> Self {
        LawnDialog {
            app: None,
            button_delay: -1,
            reanimation: None,
            draw_standard_back: true,
            lawn_yes_button: None,
            lawn_no_button: None,
            tall_bottom: false,
            vertical_center_text: true,
            x: 0, y: 0, width: 0, height: 0,
            colors: vec![crate::framework::color::Color::new(0xE0, 0xBB, 0x62, 255); 7],            id: 0,
            is_modal: false,
            result: 0,
            button_height: 40,
            button_horz_spacing: 0,
            dialog_header: String::new(),
            dialog_footer: String::new(),
            dialog_lines: String::new(),
            text_align: 0,
            line_spacing_offset: 0,
            space_after_header: 0,
            content_insets: crate::framework::widget::insets::Insets::new(36, 35, 46, 36),
            background_insets: crate::framework::widget::insets::Insets::new(30, 30, 30, 30),
            header_font: None,
            lines_font: None,
        }
    }

    /// 获取左侧内容起点（对应 C++ GetLeft）
    pub fn get_left(&self) -> i32 {
        self.content_insets.left + self.background_insets.left
    }

    /// 获取内容宽度（对应 C++ GetWidth）
    pub fn get_width(&self) -> i32 {
        self.width - self.content_insets.left - self.content_insets.right
            - self.background_insets.left - self.background_insets.right
    }

    /// 获取内容顶部起点（对应 C++ GetTop）
    pub fn get_top(&self) -> i32 {
        self.content_insets.top + self.background_insets.top + 99
    }

    /// 设置按钮延迟（对应 C++ SetButtonDelay）
    pub fn set_button_delay(&mut self, delay: i32) {
        self.button_delay = delay;
        if let Some(btn) = self.lawn_yes_button {
            unsafe { (*btn).disabled = true; }
        }
        if let Some(btn) = self.lawn_no_button {
            unsafe { (*btn).disabled = true; }
        }
    }

    pub fn update(&mut self) {
        // 对应 C++ Update：延迟结束后启用按钮
        if self.button_delay == 0 {
            if let Some(btn) = self.lawn_yes_button {
                unsafe { (*btn).disabled = false; }
            }
            if let Some(btn) = self.lawn_no_button {
                unsafe { (*btn).disabled = false; }
            }
        }
    }

    /// 按钮按下（对应 C++ ButtonPress）
    pub fn button_press(&mut self, _id: i32) {
        // [TRANSLATION_NOTE]: PlaySample(SOUND_GRAVEBUTTON) 依赖音效系统，暂不执行
    }

    pub fn button_depress(&mut self, _id: i32) {
        // 对应 C++ ButtonDepress：延迟结束前忽略
        if self.button_delay < 0 {
            // 无延迟则直接处理
        }
    }

    pub fn checkbox_checked(&mut self) {
        // 对应 C++ CheckboxChecked
        // [TRANSLATION_NOTE]: PlaySample(SOUND_BUTTONCLICK) 依赖音效系统，暂不执行
    }

    pub fn key_down(&mut self, key: KeyCode) {
        // 对应 C++ KeyDown：空格/回车=Yes，Esc/N=No（非图鉴对话框）
        if self.id != crate::lawn::game_enums::Dialogs::Almanac as i32 {
            if key == crate::framework::key_codes::KEYCODE_SPACE
                || key == crate::framework::key_codes::KEYCODE_RETURN
                || key == b'y' as i32
                || key == b'Y' as i32
            {
                self.result = crate::framework::widget::dialog::ID_YES;
            } else if key == crate::framework::key_codes::KEYCODE_ESCAPE
                || key == b'n' as i32
                || key == b'N' as i32
            {
                if self.lawn_no_button.is_some() {
                    self.result = crate::framework::widget::dialog::ID_NO;
                } else if key == crate::framework::key_codes::KEYCODE_ESCAPE && self.lawn_yes_button.is_some() {
                    self.result = crate::framework::widget::dialog::ID_YES;
                }
            }
        }
    }

    pub fn added_to_manager(&mut self, _manager: &mut WidgetManager) {
        // 对应 C++ AddedToManager
    }

    pub fn removed_from_manager(&mut self, _manager: &mut WidgetManager) {
        // 对应 C++ RemovedFromManager
    }

    pub fn resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
        // 对应 C++ Resize
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }

    pub fn draw(&self, _g: &mut Graphics) {
        // [TRANSLATION_NOTE]: C++ 用 9 张对话框组件图（IMAGE_DIALOG_*）平铺绘制；
        // Rust 图片资源未接入，留待对话框绘图系统
    }

    /// 计算对话框尺寸（对应 C++ CalcSize）
    pub fn calc_size(&mut self, extra_x: i32, extra_y: i32) {
        let mut a_width = self.background_insets.left + self.background_insets.right
            + self.content_insets.left + self.content_insets.right + extra_x;
        if !self.dialog_header.is_empty() {
            if let Some(font) = &self.header_font {
                a_width += font.string_width(&self.dialog_header);
            }
        }
        // 最小宽度：对话框组件图（左上+右上+中上）
        const IMAGE_DIALOG_TOPLEFT_W: i32 = 36;
        const IMAGE_DIALOG_TOPRIGHT_W: i32 = 36;
        const IMAGE_DIALOG_TOPMIDDLE_W: i32 = 18;
        let a_image_width = IMAGE_DIALOG_TOPLEFT_W + IMAGE_DIALOG_TOPRIGHT_W + IMAGE_DIALOG_TOPMIDDLE_W;
        if a_width <= a_image_width {
            a_width = a_image_width;
        } else if IMAGE_DIALOG_TOPMIDDLE_W > 0 {
            let an_extra = (a_width - a_image_width) % IMAGE_DIALOG_TOPMIDDLE_W;
            if an_extra != 0 {
                a_width += IMAGE_DIALOG_TOPMIDDLE_W - an_extra;
            }
        }

        let mut a_height = self.background_insets.top + self.background_insets.bottom
            + self.content_insets.top + self.content_insets.bottom + extra_y + DIALOG_HEADER_OFFSET;
        if !self.dialog_header.is_empty() {
            if let Some(font) = &self.header_font {
                a_height += -font.get_ascent_padding() + font.get_height() + self.space_after_header;
            }
        }
        a_height += self.button_height;

        // 最小高度：对话框组件图
        const IMAGE_DIALOG_TOPLEFT_H: i32 = 36;
        const IMAGE_DIALOG_BOTTOMLEFT_H: i32 = 36;
        const IMAGE_DIALOG_CENTERLEFT_H: i32 = 24;
        let a_image_height = IMAGE_DIALOG_TOPLEFT_H + IMAGE_DIALOG_BOTTOMLEFT_H + DIALOG_HEADER_OFFSET;
        if a_height < a_image_height {
            a_height = a_image_height;
        } else {
            let an_extra = (a_height - a_image_height) % IMAGE_DIALOG_CENTERLEFT_H;
            if an_extra != 0 {
                a_height += IMAGE_DIALOG_CENTERLEFT_H - an_extra;
            }
        }

        self.resize(self.x, self.y, a_width, a_height);
    }
}

impl Default for LawnDialog {
    fn default() -> Self {
        LawnDialog::new()
    }
}

/// 游戏结束对话框（对应 C++ GameOverDialog）
pub struct GameOverDialog {
    pub menu_button: Option<*mut DialogButton>,
}

impl GameOverDialog {
    pub fn new() -> Self {
        GameOverDialog {
            menu_button: None,
        }
    }

    pub fn button_depress(&mut self, _id: i32) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn added_to_manager(&mut self, _manager: &mut WidgetManager) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn removed_from_manager(&mut self, _manager: &mut WidgetManager) {
        // TODO: 从 LawnDialog.cpp 翻译
    }

    pub fn mouse_drag(&mut self, _x: i32, _y: i32) {
        // TODO: 从 LawnDialog.cpp 翻译
    }
}

impl Default for GameOverDialog {
    fn default() -> Self {
        GameOverDialog::new()
    }
}
