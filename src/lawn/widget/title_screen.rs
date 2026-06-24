// PvZ Portable Rust 翻译 — TitleScreen（标题屏幕 WidgetImpl）
// 对应 C++ src/Lawn/Widget/TitleScreen.h / TitleScreen.cpp
//
// 作为 WidgetImpl 的实现，通过 Widget 的虚表分派机制被调用。

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::{KeyCode, KEYCODE_UNKNOWN};
use crate::framework::color::Color;
use crate::lawn::lawn_app::LawnApp;

/// 标题屏幕状态（对应 C++ TitleState）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TitleState {
    WaitingForFirstDraw = 0,
    PopCapLogo = 1,
    PartnerLogo = 2,
    Loading = 3,
    Running = 4,
}

/// C++ TitleScreen 的业务逻辑，实现 WidgetImpl trait
pub struct TitleScreenImpl {
    /// LawnApp 指针（用于访问游戏状态）
    pub app: *mut LawnApp,

    // ---- 状态机 ----
    pub title_state: TitleState,
    pub title_state_counter: i32,
    pub title_state_duration: i32,
    pub drawn_yet: bool,
    pub need_to_init: bool,
    pub loading_thread_complete: bool,
    pub loader_screen_is_loaded: bool,
    pub title_age: i32,

    // ---- 加载条 ----
    pub cur_bar_width: f32,
    pub total_bar_width: f32,
    pub bar_vel: f32,
    pub bar_start_progress: f32,
    pub prev_loading_percent: f32,

    // ---- 注册 ----
    pub need_register: bool,
    pub register_clicked: bool,
    pub need_show_register_box: bool,
    pub display_partner_logo: bool,

    // ---- 快速加载键 ----
    pub quick_load_key: KeyCode,
}

impl TitleScreenImpl {
    pub fn new(app: *mut LawnApp) -> Self {
        TitleScreenImpl {
            app,
            title_state: TitleState::WaitingForFirstDraw,
            title_state_counter: 0,
            title_state_duration: 0,
            drawn_yet: false,
            need_to_init: true,
            loading_thread_complete: false,
            loader_screen_is_loaded: false,
            title_age: 0,
            cur_bar_width: 0.0,
            total_bar_width: 314.0,
            bar_vel: 0.2,
            bar_start_progress: 0.0,
            prev_loading_percent: 0.0,
            need_register: false,
            register_clicked: false,
            need_show_register_box: false,
            display_partner_logo: false,
            quick_load_key: KEYCODE_UNKNOWN,
        }
    }
}

impl WidgetImpl for TitleScreenImpl {
    fn draw(&mut self, widget: &Widget, g: &mut Graphics) {
        g.set_linear_blend(true);

        if self.title_state == TitleState::WaitingForFirstDraw {
            g.set_color(&Color::BLACK);
            g.fill_rect_xywh(0, 0, widget.width, widget.height);

            if !self.drawn_yet {
                self.drawn_yet = true;
            }
            return;
        }

        if self.title_state == TitleState::PopCapLogo {
            g.set_color(&Color::BLACK);
            g.fill_rect_xywh(0, 0, widget.width, widget.height);

            // 从 ResourceManager 获取 popcap_logo
            let app = unsafe { &*self.app };
            if let Some(rm_ptr) = app.base.resource_manager {
                unsafe {
                    let rm = &*rm_ptr;
                    let shared_ref = rm.get_image("popcap_logo");
                    let img_ptr = shared_ref.as_image_ptr();
                    if !img_ptr.is_null() {
                        let img = &*img_ptr;
                        let x = (widget.width - img.get_width()) / 2;
                        let y = (widget.height - img.get_height()) / 2;
                        g.set_colorize_images(true);
                        g.set_color(&Color::new(255, 255, 255, 255));
                        g.draw_image_xy(img, x, y);
                        g.set_colorize_images(false);
                    }
                }
            }
            return;
        }

        // 其他状态（Loading/Running）同样先画黑底
        g.set_color(&Color::BLACK);
        g.fill_rect_xywh(0, 0, widget.width, widget.height);
    }

    fn update(&mut self, widget: &mut Widget) {
        widget.mark_dirty();

        // 检查 LawnApp 是否已关闭
        let app = unsafe { &mut *self.app };
        if app.m_close_request {
            return;
        }

        if !self.drawn_yet {
            return;
        }

        // === 状态机主逻辑（对应 C++ TitleScreen::Update） ===

        if self.title_state == TitleState::WaitingForFirstDraw {
            // 首次 Update：初始化音乐、启动加载线程
            if let Some(ref mut music) = app.music {
                music.music_title_screen_init();
            }
            app.start_loading_thread();

            // 切换到 PopCap Logo 状态
            self.title_state = TitleState::PopCapLogo;
            self.title_state_counter = 0;
            // C++ 中 mTitleStateDuration 在这里被设置为其他值，此处暂用默认
        }

        // 计数递增
        self.title_state_counter += 1;
        self.title_age += 1;

        // 状态间迁移
        match self.title_state {
            TitleState::PopCapLogo => {
                // 后续 Phase 在显示足够时间后迁移到 PartnerLogo 或 Loading
            }
            TitleState::PartnerLogo => {
                // 后续 Phase 迁移到 Loading
            }
            TitleState::Loading => {
                // 后续 Phase 加载资源、更新加载条
            }
            TitleState::Running => {
                // 显示完整标题屏幕，等待用户点击
            }
            _ => {}
        }
    }

    fn key_down(&mut self, widget: &mut Widget, key: KeyCode, wm: &mut WidgetManager) {
        // 快速加载键（C++ 中按 Enter/Space 跳过 Logo 动画）
        let _ = (widget, wm);
        self.quick_load_key = key;
    }

    fn mouse_down_btn(&mut self, widget: &mut Widget, x: i32, y: i32, btn: i32, click: i32) {
        // 开始按钮点击处理（后续 Phase 实现）
        let _ = (widget, x, y, btn, click);
    }
}
