// PvZ Portable Rust 翻译 — TitleScreen（标题屏幕 WidgetImpl）
// 对应 C++ src/Lawn/Widget/TitleScreen.h / TitleScreen.cpp
//
// 作为 WidgetImpl 的实现，通过 Widget 的虚表分派机制被调用。

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::{
    KeyCode, KEYCODE_UNKNOWN, KEYCODE_Z, KEYCODE_M, KEYCODE_S, KEYCODE_C,
    KEYCODE_U, KEYCODE_I, KEYCODE_P, KEYCODE_R, KEYCODE_T,
};
use crate::framework::color::Color;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::game_enums::{TodCurves, GameMode};
use crate::todlib::tod_common::{
    tod_animate_curve, tod_animate_curve_float, tod_animate_curve_float_time, clamp_float,
};

/// 标题屏幕状态（对应 C++ TitleState）
/// 注意：C++ 中 PopCapLogo→PartnerLogo→Screen，Rust 映射为
/// WaitingForFirstDraw→PopCapLogo→PartnerLogo→Loading→Running
/// C++ 中 Screen 状态包含加载条动画 + 标题菜单，我们拆成 Loading + Running
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
    /// Rust 递增计数器（从 0 向上计数到 duration，每次 update+=1）
    pub title_state_counter: i32,
    /// 当前状态的持续时间（帧数）
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

    // ---- 启动按钮文本 ----
    pub start_button_label: String,
    pub start_button_font: Option<Box<Font>>,

    // ---- 快速加载键 ----
    pub quick_load_key: KeyCode,

    // ---- 启动按钮状态 ----
    pub start_button_visible: bool,
    pub start_button_disabled: bool,
    pub start_button_x: i32,
    pub start_button_y: i32,
}

impl TitleScreenImpl {
    pub fn new(app: *mut LawnApp) -> Self {
        let app_ref = unsafe { &*app };
        let display_partner = app_ref.get_boolean("DisplayPartnerLogo", false);

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
            display_partner_logo: display_partner,
            quick_load_key: KEYCODE_UNKNOWN,
            start_button_visible: false,
            start_button_disabled: true,
            start_button_x: 0,
            start_button_y: 0,
            start_button_label: String::new(),
            start_button_font: None,
        }
    }

    /// 将 Rust 递增 counter 转换为 C++ 递减模式的值，以便直接使用 C++ 公式
    /// C++ 中 counter 从 duration 递减到 0，Rust 中从 0 递增到 duration
    fn cc(&self) -> i32 {
        self.title_state_duration - self.title_state_counter
    }
}

impl WidgetImpl for TitleScreenImpl {
    fn draw(&mut self, widget: &Widget, g: &mut Graphics) {
        g.set_linear_blend(true);

        match self.title_state {
            TitleState::WaitingForFirstDraw => {
                g.set_color(&Color::BLACK);
                g.fill_rect_xywh(0, 0, widget.width, widget.height);

                if !self.drawn_yet {
                    self.drawn_yet = true;
                }
                return;
            }

            // ======================== PopCapLogo ========================
            TitleState::PopCapLogo => {
                g.set_color(&Color::BLACK);
                g.fill_rect_xywh(0, 0, widget.width, widget.height);

                // C++ TitleScreen::Draw lines 109-132:
                // C++ counter 从 duration 递减，所以用 cc()
                let cc = self.cc();
                let duration = self.title_state_duration;

                let mut an_alpha = 255i32;
                if cc < duration - 50 {
                    // 已过了至少50帧（C++: counter < duration-50）
                    if !self.display_partner_logo {
                        // C++: TodAnimateCurve(50, 0, counter, 255, 0, CURVE_LINEAR)
                        // 当 counter >= 50（即 cc >= 50）时 alpha=0；
                        // 当 counter < 50（即 cc < 50，最后50帧）时 alpha=255
                        an_alpha = tod_animate_curve(50, 0, cc, 255, 0, TodCurves::Linear);
                    }
                } else {
                    // 前50帧（C++: counter >= duration-50）：淡入 0→255
                    // C++: TodAnimateCurve(duration, duration-50, counter, 0, 255, CURVE_LINEAR)
                    an_alpha = tod_animate_curve(
                        duration, duration - 50, cc,
                        0, 255, TodCurves::Linear,
                    );
                }

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
                            g.set_color(&Color::new(255, 255, 255, an_alpha as u8));
                            g.draw_image_xy(img, x, y);
                            g.set_colorize_images(false);
                        }
                    }
                }
                return;
            }

            // ======================== PartnerLogo ========================
            TitleState::PartnerLogo => {
                g.set_color(&Color::BLACK);
                g.fill_rect_xywh(0, 0, widget.width, widget.height);

                g.set_colorize_images(true);
                let cc = self.cc();
                let duration = self.title_state_duration;

                // C++ TitleScreen::Draw lines 134-156
                // C++ 中 cc 从 200 递减到 0

                // 先绘制 PopCap Logo（在 partner logo 之上淡出，仅最后35帧）
                if cc >= duration - 35 {
                    let popcap_alpha = tod_animate_curve(
                        duration, duration - 35, cc,
                        0, 255, TodCurves::Linear,
                    );
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
                                // C++: SetColor(255,255,255,255 - anAlpha) 即 popcap 逐渐消失
                                g.set_color(&Color::new(255, 255, 255, (255 - popcap_alpha as u8) as u8));
                                g.draw_image_xy(img, x, y);
                            }
                        }
                    }
                }

                // 再绘制 Partner Logo（前165帧淡入，最后35帧全亮）
                let partner_alpha = if cc >= duration - 35 {
                    // 最后35帧：popcap 淡出的同时，partner 全亮
                    255
                } else {
                    // 前165帧：C++: TodAnimateCurve(35, 0, counter, 255, 0, CURVE_LINEAR)
                    // 即 cc > 165 时 alpha=0，cc ≤ 165 时从 255 递减到 0
                    // 但 partner logo 是淡入，所以取反
                    let fade_out = tod_animate_curve(35, 0, cc, 255, 0, TodCurves::Linear);
                    255 - fade_out
                };

                let app = unsafe { &*self.app };
                if let Some(rm_ptr) = app.base.resource_manager {
                    unsafe {
                        let rm = &*rm_ptr;
                        let shared_ref = rm.get_image("partner_logo");
                        let img_ptr = shared_ref.as_image_ptr();
                        if !img_ptr.is_null() {
                            let img = &*img_ptr;
                            let x = (widget.width - img.get_width()) / 2;
                            let y = (widget.height - img.get_height()) / 2;
                            g.set_color(&Color::new(255, 255, 255, partner_alpha as u8));
                            g.draw_image_xy(img, x, y);
                        }
                    }
                }
                g.set_colorize_images(false);
                return;
            }

            // ======================== Loading / Running ========================
            TitleState::Loading | TitleState::Running => {
                let app = unsafe { &*self.app };

                // 如果 LoaderBar 资源还没加载完，画黑屏
                if !self.loader_screen_is_loaded {
                    g.set_color(&Color::BLACK);
                    g.fill_rect_xywh(0, 0, widget.width, widget.height);
                    return;
                }

                // 画标题屏幕背景
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

                // 画 PvZ Logo（带弹跳动画）—— 在 need_to_init return 之前画，
                // 避免初始化帧 Logo 不画导致的闪烁
                let app = unsafe { &*self.app };
                let cc = self.cc(); // 转换为 C++ 递减 counter（duration 递减到 0）
                let a_logo_y = if cc > 60 {
                    // C++: TodAnimateCurve(100, 60, counter, -150, 10, CURVE_EASE_IN)
                    // Logo 从 -150 逐渐降到 10
                    tod_animate_curve(100, 60, cc, -150, 10, TodCurves::EaseIn)
                } else {
                    // C++: TodAnimateCurve(60, 50, counter, 10, 15, CURVE_BOUNCE)
                    // Logo 从 10 弹到 15，再回到 10
                    tod_animate_curve(60, 50, cc, 10, 15, TodCurves::Bounce)
                };

                if let Some(rm_ptr) = app.base.resource_manager {
                    unsafe {
                        let rm = &*rm_ptr;
                        let shared_ref = rm.get_image("pvz_logo");
                        let img_ptr = shared_ref.as_image_ptr();
                        if !img_ptr.is_null() {
                            let img = &*img_ptr;
                            let lx = widget.width / 2 - img.get_width() / 2;
                            g.draw_image_xy(img, lx, a_logo_y);
                        }
                    }
                }

                if self.need_to_init {
                    // need_to_init 时跳过加载条的初始化，但 Logo 已经画了
                    return;
                }

                // 画加载条的泥土背景
                let grass_x = self.start_button_x;
                let grass_y = self.start_button_y - 17;

                if let Some(rm_ptr) = app.base.resource_manager {
                    unsafe {
                        let rm = &*rm_ptr;
                        let dirt_ref = rm.get_image("loadbar_dirt");
                        let dirt_ptr = dirt_ref.as_image_ptr();
                        if !dirt_ptr.is_null() {
                            g.draw_image_xy(&*dirt_ptr, grass_x, grass_y + 18);
                        }
                    }
                }

                if self.cur_bar_width >= self.total_bar_width {
                    // 加载条满了，画完整的草皮
                    if let Some(rm_ptr) = app.base.resource_manager {
                        unsafe {
                            let rm = &*rm_ptr;
                            let grass_ref = rm.get_image("loadbar_grass");
                            let grass_ptr = grass_ref.as_image_ptr();
                            if !grass_ptr.is_null() {
                                g.draw_image_xy(&*grass_ptr, grass_x, grass_y);
                            }
                        }
                    }

                    if self.loading_thread_complete {
                        // C++: DrawToPreload (预加载绘制)
                    }
                } else {
                    // 加载条未满，裁剪绘制
                    let bar_w = self.cur_bar_width as i32;
                    let mut clip_g = Graphics::new_with_image(g.dest_image);
                    clip_g.set_clip_rect_xywh(240, grass_y, bar_w, 50);

                    if let Some(rm_ptr) = app.base.resource_manager {
                        unsafe {
                            let rm = &*rm_ptr;
                            let grass_ref = rm.get_image("loadbar_grass");
                            let grass_ptr = grass_ref.as_image_ptr();
                            if !grass_ptr.is_null() {
                                clip_g.draw_image_xy(&*grass_ptr, grass_x, grass_y);
                            }
                        }
                    }

                    // 画草皮卷滚轴
                    let roll_len = self.cur_bar_width * 0.94f32;
                    let rotation = -roll_len / 180.0 * std::f32::consts::PI * 2.0;

                    if let Some(rm_ptr) = app.base.resource_manager {
                        unsafe {
                            let rm = &*rm_ptr;
                            let cap_ref = rm.get_image("sodrollcap");
                            let cap_ptr = cap_ref.as_image_ptr();
                            if !cap_ptr.is_null() {
                                let cap = &*cap_ptr;
                                let cap_x = (grass_x + 11) as f32 + roll_len;
                                let cap_y = (grass_y - 3) as f32 - 35.0 + 35.0;
                                g.draw_image_rotated_f_simple(
                                    cap, cap_x, cap_y,
                                    rotation as f64, None,
                                );
                            }
                        }
                    }
                }

                // 绘制启动按钮文本（对应 C++ mStartButton，HyperlinkWidget）
                if !self.start_button_label.is_empty() && self.start_button_visible {
                    // 懒初始化默认字体
                    if self.start_button_font.is_none() {
                        let mut f = Font::new("Briannetod", 16);
                        f.ascent = 13;
                        f.font_height = 16;
                        self.start_button_font = Some(Box::new(f));
                    }

                    if let Some(ref font) = self.start_button_font {
                        let text_width = font.string_width(&self.start_button_label);
                        let text_x = self.start_button_x + (314 - text_width) / 2;
                        let text_y = self.start_button_y + (50 + font.get_ascent()) / 2 - 1;

                        g.set_color(&Color::new(218, 184, 33, 255));
                        let fptr = &**font as *const Font as *mut Font;
                        g.set_font(fptr);
                        g.draw_string(&self.start_button_label, text_x, text_y);
                    }
                }
            }
        }
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
            eprintln!("[TitleScreen] 首次 Update，初始化音乐并启动加载线程");
            if let Some(ref mut music) = app.music {
                music.music_title_screen_init();
            } else {
                eprintln!("[TitleScreen] music 为 None，跳过音乐初始化");
            }
            app.start_loading_thread();

            // C++: mTitleStateCounter 设为 mTitleStateDuration（从 duration 递减）
            self.title_state = TitleState::PopCapLogo;
            self.title_state_counter = 0;
            if self.display_partner_logo {
                self.title_state_duration = 150;
            } else {
                self.title_state_duration = 200;
            }
            eprintln!("[TitleScreen] → PopCapLogo 状态，持续 {} 帧", self.title_state_duration);
        }

        // C++: 快速加载键（在状态检查前处理）
        if self.quick_load_key != KEYCODE_UNKNOWN
            && self.title_state != TitleState::Running
        {
            eprintln!("[TitleScreen] 快速加载键按下，跳过 Logo");
            self.title_state = TitleState::Loading;
            self.title_state_duration = 100;
            self.title_state_counter = 0;
        }

        // Rust 递增计数器
        self.title_state_counter += 1;
        self.title_age += 1;

        // C++ 递减计数器（相当于 cc = duration - counter）
        let cc = self.cc();

        // 状态间迁移和更新
        match self.title_state {
            TitleState::PopCapLogo => {
                // C++: if (mTitleStateCounter == 0) 即 cc == 0
                if self.title_state_counter >= self.title_state_duration {
                    if self.display_partner_logo {
                        self.title_state = TitleState::PartnerLogo;
                        self.title_state_counter = 0;
                        self.title_state_duration = 200;
                        eprintln!("[TitleScreen] → PartnerLogo 状态");
                    } else {
                        self.title_state = TitleState::Loading;
                        self.title_state_counter = 0;
                        self.title_state_duration = 100;
                        eprintln!("[TitleScreen] → Loading 状态");
                    }
                }
            }

            TitleState::PartnerLogo => {
                if self.title_state_counter >= self.title_state_duration {
                    self.title_state = TitleState::Loading;
                    self.title_state_counter = 0;
                    self.title_state_duration = 100;
                    eprintln!("[TitleScreen] → Loading 状态");
                }
            }

            TitleState::Loading => {
                if !self.loader_screen_is_loaded {
                    if app.m_loading_thread_tasks_completed > 0 {
                        self.loader_screen_is_loaded = true;
                        eprintln!("[TitleScreen] LoaderBar 资源加载完成");
                    }
                    return;
                }

                if self.need_to_init {
                    self.need_to_init = false;

                    // C++: 初始化启动按钮位置
                    self.start_button_x = (widget.width - 314) / 2;
                    self.start_button_y = 650;

                    // C++: 设置启动按钮文字（对应 mStartButton->mLabel = "[LOADING]"）
                    self.start_button_label = "LOADING".to_string();
                    self.start_button_visible = true;

                    // C++: 计算加载进度估计
                    let current_progress = app.get_loading_thread_progress();
                    let estimated_total_load_time = if current_progress > 0.000001f32 {
                        self.title_age as f32 / current_progress
                    } else {
                        3000.0f32
                    };

                    let mut load_time = estimated_total_load_time * (1.0 - current_progress);
                    load_time = clamp_float(load_time, 100.0, 3000.0);
                    self.bar_vel = self.total_bar_width / load_time;
                    self.bar_start_progress = if current_progress < 0.9 {
                        current_progress
                    } else {
                        0.9
                    };

                    eprintln!("[TitleScreen] 初始化加载条：bar_vel={}, bar_start={}",
                        self.bar_vel, self.bar_start_progress);
                }

                // C++: 计算加载百分比
                let current_progress = app.get_loading_thread_progress();
                let loading_percent = if (1.0 - self.bar_start_progress).abs() > 0.0001 {
                    (current_progress - self.bar_start_progress) / (1.0 - self.bar_start_progress)
                } else {
                    1.0
                };

                // C++: 按钮弹跳动画 — 这里需要用 cc（递减 counter）
                let button_y = if cc > 10 {
                    // C++: TodAnimateCurve(60, 10, counter, 650, 534, CURVE_EASE_IN)
                    tod_animate_curve(60, 10, cc, 650, 534, TodCurves::EaseIn)
                } else {
                    // C++: TodAnimateCurve(10, 0, counter, 534, 529, CURVE_BOUNCE)
                    tod_animate_curve(10, 0, cc, 534, 529, TodCurves::Bounce)
                };
                self.start_button_y = button_y;

                // C++: 更新效果系统
                if let Some(ref mut es) = app.effect_system {
                    es.update();
                }

                // C++: 加载条增长
                let prev_width = self.cur_bar_width;
                self.cur_bar_width += self.bar_vel;

                if !self.loading_thread_complete {
                    if self.cur_bar_width > self.total_bar_width * 0.99 {
                        self.cur_bar_width = self.total_bar_width * 0.99;
                    }
                } else if self.cur_bar_width > self.total_bar_width {
                    self.cur_bar_width = self.total_bar_width;
                    self.title_state = TitleState::Running;
                    self.start_button_disabled = false;
                    eprintln!("[TitleScreen] → Running 状态，等待用户点击");
                }

                // C++: 加速度更新
                if loading_percent > self.prev_loading_percent + 0.01 || self.loading_thread_complete {
                    let bar_width = tod_animate_curve_float_time(
                        0.0, 1.0, loading_percent,
                        0.0, self.total_bar_width, TodCurves::EaseIn,
                    );
                    let diff = bar_width - self.cur_bar_width;
                    let acceleration = if self.loading_thread_complete {
                        0.0001f32
                    } else {
                        tod_animate_curve_float_time(
                            0.0, 1.0, loading_percent,
                            0.0001, 0.00001, TodCurves::Linear,
                        )
                    };
                    self.bar_vel += diff * diff.abs() * acceleration;

                    let min_velocity = tod_animate_curve_float_time(
                        0.0, 1.0, loading_percent,
                        0.2, 0.01, TodCurves::Linear,
                    );
                    let max_velocity = 2.0f32;

                    if self.bar_vel < min_velocity {
                        self.bar_vel = min_velocity;
                    } else if self.bar_vel > max_velocity {
                        self.bar_vel = max_velocity;
                    }

                    self.prev_loading_percent = loading_percent;
                }

                // C++: 加载完成回调处理
                if !self.loading_thread_complete && app.m_loading_thread_completed {
                    self.loading_thread_complete = true;
                    self.start_button_disabled = false;

                    if self.quick_load_key == KEYCODE_Z {
                        app.fast_load(GameMode::ChallengeZenGarden);
                    } else if self.quick_load_key == KEYCODE_M {
                        app.loading_completed();
                    } else if self.quick_load_key == KEYCODE_S {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.show_challenge_screen(2);
                    } else if self.quick_load_key == KEYCODE_C {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.show_challenge_screen(1);
                    } else if self.quick_load_key == KEYCODE_U {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.pre_new_game(GameMode::Adventure, false);
                    } else if self.quick_load_key == KEYCODE_I {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.pre_new_game(GameMode::Adventure, false);
                    } else if self.quick_load_key == KEYCODE_P {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.show_challenge_screen(0);
                    } else if self.quick_load_key == KEYCODE_R {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.show_credit_screen();
                    } else if self.quick_load_key == KEYCODE_T {
                        app.fast_load(GameMode::Adventure);
                    } else {
                        self.start_button_visible = true;
                        self.start_button_label = "CLICK TO START".to_string();
                    }

                    eprintln!("[TitleScreen] 加载线程完成");
                }

                // C++: 加载条触发点动画
                let trigger_points = [
                    self.total_bar_width * 0.11,
                    self.total_bar_width * 0.32,
                    self.total_bar_width * 0.54,
                    self.total_bar_width * 0.72,
                    self.total_bar_width * 0.91,
                ];

                for (i, &trigger) in trigger_points.iter().enumerate() {
                    if prev_width < trigger && self.cur_bar_width >= trigger {
                        eprintln!("[TitleScreen] 加载条触发点 {} 达到", i);
                    }
                }
            }

            TitleState::Running => {
                // Running 状态：等待用户点击或按键
            }

            _ => {}
        }
    }

    fn key_down(&mut self, widget: &mut Widget, key: KeyCode, wm: &mut WidgetManager) {
        let _ = (widget, wm);
        let app = unsafe { &mut *self.app };

        if self.loading_thread_complete {
            app.loading_completed();
            return;
        }

        self.quick_load_key = key;
    }

    fn mouse_down_btn(&mut self, widget: &mut Widget, x: i32, y: i32, btn: i32, click: i32) {
        let _ = (widget, x, y, btn, click);
        let app = unsafe { &mut *self.app };

        if self.loading_thread_complete && self.title_state == TitleState::Running {
            app.loading_completed();
        }
    }
}
