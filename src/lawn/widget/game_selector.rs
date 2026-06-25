// PvZ Portable Rust 翻译 — GameSelector
// 对应 C++ src/Lawn/Widget/GameSelector.h / GameSelector.cpp
//
// 游戏选择界面（主菜单）

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::color::Color;
use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::KeyCode;
use crate::lawn::lawn_app::LawnApp;

pub struct GameSelectorImpl {
    pub app: *mut LawnApp,
}

impl GameSelectorImpl {
    pub fn new(app: *mut LawnApp) -> Self {
        GameSelectorImpl { app }
    }

    /// 获取或创建绘图用的字体
    fn ensure_font(&self) -> Font {
        let mut f = Font::new("Default", 24);
        f.font_height = 24;
        f.ascent = 20;
        f
    }
}

impl WidgetImpl for GameSelectorImpl {
    fn draw(&mut self, widget: &Widget, g: &mut Graphics) {
        let app = unsafe { &*self.app };

        // 画背景
        if let Some(rm_ptr) = app.base.resource_manager {
            unsafe {
                let rm = &*rm_ptr;
                let shared_ref = rm.get_image("titlescreen");
                let img_ptr = shared_ref.as_image_ptr();
                if !img_ptr.is_null() {
                    g.draw_image_xy(&*img_ptr, 0, 0);
                }
            }
        }

        // 画 PvZ Logo
        if let Some(rm_ptr) = app.base.resource_manager {
            unsafe {
                let rm = &*rm_ptr;
                let shared_ref = rm.get_image("pvz_logo");
                let img_ptr = shared_ref.as_image_ptr();
                if !img_ptr.is_null() {
                    let img = &*img_ptr;
                    let lx = (widget.width - img.get_width()) / 2;
                    let ly = 10;
                    g.draw_image_xy(img, lx, ly);
                }
            }
        }

        // 绘制提示文字（简化版）
        let mut tmp_font = Font::new("Default", 24);
        tmp_font.font_height = 24;
        tmp_font.ascent = 20;
        let fptr = &mut tmp_font as *mut Font;

        g.set_font(fptr);
        g.set_color(&Color::new(218, 184, 33, 255));
        g.draw_string("Plants vs. Zombies", 250, 350);
        g.set_color(&Color::new(255, 255, 255, 200));
        g.draw_string("PvZ Portable Rust", 270, 400);
    }

    fn update(&mut self, _widget: &mut Widget) {
        // 暂时不做更新
    }

    fn key_down(&mut self, _widget: &mut Widget, _key: KeyCode, _wm: &mut WidgetManager) {
    }

    fn mouse_down_btn(&mut self, _widget: &mut Widget, _x: i32, _y: i32, _btn: i32, _click: i32) {
    }
}
