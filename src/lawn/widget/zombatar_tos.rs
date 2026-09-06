// PvZ Portable Rust 翻译 — ZombatarTOS（僵尸自定义条款对话框）
// 对应 C++ src/Lawn/Widget/ZombatarTOS.h / ZombatarTOS.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::{KEYCODE_ESCAPE, KeyCode};
use crate::framework::rect::Rect;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::*;
use crate::todlib::tod_common::tod_string_translate;

/// 对话框 ID（对应 C++ ConstEnums.h Dialogs::DIALOG_ZOMBATAR_TOS）
pub const DIALOG_ZOMBATAR_TOS: i32 = 51;
/// 控件 ID（对应 C++ ZombatarTOS.h 枚举）
pub const ZOMBATAR_TOS_CHECKBOX: i32 = 500;
pub const ZOMBATAR_TOS_ACCEPT: i32 = 501;
pub const ZOMBATAR_TOS_BACK: i32 = 502;
pub const ZOMBATAR_TOS_SLIDER: i32 = 503;
/// 布局常量（对应 C++ ZombatarTOS.cpp 顶部 constexpr）
const TOS_DIALOG_WIDTH: i32 = 600;
const TOS_DIALOG_HEIGHT: i32 = 450;
const TOS_SLIDER_X: i32 = 500;
const TOS_SLIDER_Y: i32 = 140;
const TOS_SLIDER_WIDTH: i32 = 29;
const TOS_SLIDER_HEIGHT: i32 = 135;
const TOS_BACK_X: i32 = 40;
const TOS_ACCEPT_X: i32 = 450;
const TOS_BUTTON_Y: i32 = 344;
const TOS_CHECK_X: i32 = 400;
const TOS_CHECK_Y: i32 = 340;
const TOS_TEXT_X: i32 = 50;
const TOS_TEXT_Y: i32 = 130;
const TOS_TEXT_WIDTH: i32 = 435;
const TOS_CLIP_HEIGHT: i32 = 160;
const TOS_ARROW_X: i32 = 420;
const TOS_ARROW_Y: i32 = 290;
const TOS_CHECK_WIDTH: i32 = 45;
const TOS_CHECK_HEIGHT: i32 = 45;

/// 僵尸自定义条款对话框（对应 C++ ZombatarTOS）
pub struct ZombatarTOS {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    /// 对应 C++ mBodyText / mTextHeight / mFlashArrow / mArrowAlpha / mArrowFadeDir
    pub body_text: String,
    pub text_height: i32,
    pub flash_arrow: bool,
    pub arrow_alpha: i32,
    pub arrow_fade_dir: i32,
    /// 对应 C++ mTOSSlider->mVal（滑块值 0..1；图片未接入，用 f64 简化）
    pub slider_val: f64,
    /// 对应 C++ mTOSCheckbox->mChecked
    pub checkbox_checked: bool,
}

impl ZombatarTOS {
    /// 构造函数（对应 C++ ZombatarTOS::ZombatarTOS）
    pub fn new(app: Option<*mut crate::lawn::lawn_app::LawnApp>) -> Self {
        let mut tos = ZombatarTOS {
            app,
            x: 0, y: 0, width: TOS_DIALOG_WIDTH, height: TOS_DIALOG_HEIGHT,
            body_text: String::new(),
            text_height: 0,
            flash_arrow: false,
            arrow_alpha: 0,
            arrow_fade_dir: 3,
            slider_val: 0.0,
            checkbox_checked: false,
        };
        // [TRANSLATION_NOTE]: C++ 中创建 mTOSSlider（Slider + 竖置 + SetValue(0)）、
        // mBackButton/mAcceptButton（MakeNewButton，依赖 IMAGE_ZOMBATAR_* 图）与 mTOSCheckbox
        // （MakeNewCheckbox）；图片资源未接入，以简化字段代替，控件注册链待回填。
        let _ = &mut tos;
        tos
    }

    /// 对应 C++ ZombatarTOS::AddedToManager
    pub fn added_to_manager(&mut self, _the_widget_manager: *mut WidgetManager) {
        // C++: Dialog::AddedToManager + AddWidget(mTOSSlider/mBackButton/mAcceptButton/mTOSCheckbox)
        // [TRANSLATION_NOTE]: 子控件未创建（见构造函数注释），注册链保留调用点
    }

    /// 对应 C++ ZombatarTOS::RemovedFromManager
    pub fn removed_from_manager(&mut self, _the_widget_manager: *mut WidgetManager) {
        // C++: Dialog::RemovedFromManager + RemoveWidget（同上 4 个子控件）
    }

    /// 对应 C++ ZombatarTOS::Resize
    pub fn resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.x = x; self.y = y; self.width = width; self.height = height;
        // C++: LawnDialog::Resize + 子控件定位（依赖图片尺寸，默认 98/26）；
        // [TRANSLATION_NOTE]: 图片未接入，子控件定位暂略
    }

    /// 从 ResourceManager 按 key 取图（对应 C++ IMAGE_* 全局资源；未接入资源表时返回 null）
    fn get_resource_image(&self, a_key: &str) -> *mut crate::framework::graphics::image::Image {
        let Some(app) = self.app else { return std::ptr::null_mut() };
        unsafe {
            let app_ref = &*app;
            let Some(rm) = app_ref.base.resource_manager else { return std::ptr::null_mut() };
            let rm_ref = &*rm;
            rm_ref.get_image(a_key).as_image_ptr()
        }
    }

    /// 对应 C++ ZombatarTOS::Draw（ZombatarTOS.cpp 127-177）
    pub fn draw(&mut self, g: &mut Graphics) {
        // C++: LawnDialog::Draw（背景/标题）——Rust 端独立类，无父类组合
        if self.body_text.is_empty() {
            self.body_text = tod_string_translate("[ZOMBATAR_TOS]");
        }
        if self.text_height <= 0 {
            // [TRANSLATION_NOTE]: C++ PvzpDrawStringWrappedHelper 测量（FONT_PICO129）；此处以裁剪高兜底
            self.text_height = TOS_CLIP_HEIGHT;
        }

        let a_max_scroll = (self.text_height - TOS_CLIP_HEIGHT).max(0);
        let a_offset = (self.slider_val * a_max_scroll as f64) as i32;

        // [TRANSLATION_NOTE]: C++ PvzpDrawStringWrapped（FONT_PICO129）按字符断行；Rust 端近似为按行绘制
        g.set_clip_rect(&Rect::new(TOS_TEXT_X, TOS_TEXT_Y, TOS_TEXT_WIDTH, TOS_CLIP_HEIGHT));
        let mut a_font = crate::framework::graphics::font::Font::new("Pico", 129);
        a_font.ascent = 13;
        a_font.font_height = 16;
        g.set_font(&mut a_font as *mut crate::framework::graphics::font::Font);
        g.set_color(&Color::WHITE);
        let a_lines: Vec<&str> = self.body_text.lines().collect();
        for (i, a_line) in a_lines.iter().enumerate() {
            g.draw_string(a_line, TOS_TEXT_X, TOS_TEXT_Y - a_offset + i as i32 * 16);
        }
        g.clear_clip_rect();

        if self.flash_arrow {
            let a_arrow = self.get_resource_image("IMAGE_ZOMBATAR_TOS_ARROW");
            if !a_arrow.is_null() {
                g.set_colorize_images(true);
                g.set_color(&Color::new(255, 255, 255, self.arrow_alpha as u8));
                g.draw_image_xy(unsafe { &*a_arrow }, TOS_ARROW_X, TOS_ARROW_Y);
                g.set_colorize_images(false);
                g.set_color(&Color::WHITE);
            }
        }
    }

    /// 对应 C++ ZombatarTOS::Update
    pub fn update(&mut self) {
        // C++: LawnDialog::Update()
        if self.flash_arrow {
            self.arrow_alpha += self.arrow_fade_dir;
            if self.arrow_alpha >= 255 {
                self.arrow_alpha = 255;
                self.arrow_fade_dir = -3;
            } else if self.arrow_alpha <= 0 {
                self.arrow_alpha = 0;
                self.arrow_fade_dir = 3;
            }
            // C++: MarkDirty() —— Rust 侧无脏区系统，等效省略
        }
    }

    /// 对应 C++ ZombatarTOS::ButtonPress（空实现）
    pub fn button_press(&mut self, _the_id: i32) {}

    /// 对应 C++ ZombatarTOS::ButtonDepress
    pub fn button_depress(&mut self, the_id: i32) {
        match the_id {
            ZOMBATAR_TOS_BACK => {
                // C++: mApp->KillDialog(mId) —— Rust 侧对话框销毁链未接入，占位
            }
            ZOMBATAR_TOS_ACCEPT => {
                if !self.checkbox_checked {
                    self.flash_arrow = true;
                    self.arrow_alpha = 0;
                    self.arrow_fade_dir = 3;
                    return;
                }
                if let Some(app) = self.app {
                    unsafe {
                        if let Some(pi) = (*app).player_info.as_mut() {
                            pi.m_zombatar_accepted = 1;
                            pi.save_details();
                        }
                    }
                }
                // C++: mApp->KillDialog(mId)；mApp->mGameSelector->mZombatarWidget->Open()
                // [TRANSLATION_NOTE]: KillDialog 与 game_selector 的 zombatar widget（未接入）占位
            }
            _ => {}
        }
    }

    /// 对应 C++ ZombatarTOS::KeyDown
    pub fn key_down(&mut self, the_key: KeyCode) {
        if the_key == KEYCODE_ESCAPE {
            self.button_depress(ZOMBATAR_TOS_BACK);
        }
    }

    /// 对应 C++ ZombatarTOS::MouseWheel
    pub fn mouse_wheel(&mut self, the_delta: i32) {
        if self.text_height <= TOS_CLIP_HEIGHT {
            return;
        }
        let a_max_scroll = (self.text_height - TOS_CLIP_HEIGHT).max(0);
        let mut a_offset = (self.slider_val * a_max_scroll as f64) as i32;
        a_offset -= the_delta * 12;
        let a_fraction = a_offset as f64 / a_max_scroll as f64;
        self.slider_val = a_fraction.max(0.0).min(1.0);
        // C++: mTOSSlider->SetValue(...)
    }

    /// 对应 C++ ZombatarTOS::CheckboxChecked
    pub fn checkbox_checked(&mut self, the_id: i32, checked: bool) {
        if let Some(app) = self.app {
            unsafe {
                (*app).play_sample(crate::framework::resources::ResourceId::SoundButtonclick as i32);
            }
        }
        if the_id == ZOMBATAR_TOS_CHECKBOX && checked {
            self.flash_arrow = false;
            self.arrow_alpha = 0;
        }
    }

    /// 对应 C++ ZombatarTOS::SliderVal
    pub fn slider_val(&mut self, _the_id: i32, the_val: f64) {
        self.slider_val = the_val;
        // C++: MarkDirty() —— Rust 侧无脏区系统，等效省略
    }
}

impl Default for ZombatarTOS {
    fn default() -> Self {
        ZombatarTOS::new(None)
    }
}