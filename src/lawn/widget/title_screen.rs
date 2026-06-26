// 修正后的 TitleScreenImpl（基于版本 B 架构，逻辑对齐版本 A / 原始 C++）

use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::{
    KeyCode, KEYCODE_UNKNOWN, KEYCODE_END,
    KEYCODE_M, KEYCODE_S, KEYCODE_C, KEYCODE_U, KEYCODE_I, KEYCODE_P, KEYCODE_R, KEYCODE_T,
};
use crate::framework::color::Color;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::game_enums::{TodCurves, GameMode, ChallengePage, ReanimationType, ReanimLoopType};
use crate::todlib::tod_common::{
    tod_animate_curve, tod_animate_curve_float, tod_animate_curve_float_time, clamp_float,
};
use crate::todlib::tod_foley::FoleyType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TitleState {
    WaitingForFirstDraw = 0,
    PopCapLogo = 1,
    PartnerLogo = 2,
    Loading = 3,      // 加载条动画阶段
    Running = 4,      // 加载完成，等待用户点击
}

pub struct TitleScreenImpl {
    pub app: *mut LawnApp,

    // ---- 状态机 ----
    pub title_state: TitleState,
    pub title_state_counter: i32,   // 递增，从 0 到 duration
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

    // ---- 启动按钮 ----
    pub start_button_label: String,
    pub start_button_font: Option<Box<Font>>,
    pub start_button_visible: bool,
    pub start_button_disabled: bool,
    pub start_button_x: i32,
    pub start_button_y: i32,

    // ---- 快速加载键 ----
    pub quick_load_key: KeyCode,
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

    /// 将 Rust 递增 counter 转换为 C++ 递减模式的值，用于动画曲线参数
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

                let duration = self.title_state_duration;
                let counter = self.title_state_counter; // 递增，0..duration

                // C++ 原始逻辑使用递减计数器： mTitleStateCounter 从 duration 递减到 0
                // 条件：if (mTitleStateCounter < mTitleStateDuration - 50)
                // 递减时，counter < duration-50 意味着已经过了至少 50 帧（剩余 50 帧）
                // 翻译为递增：counter > 50
                let mut an_alpha = 255i32;
                if counter > 50 {
                    if !self.display_partner_logo {
                        // C++: TodAnimateCurve(50, 0, counter, 255, 0, CURVE_LINEAR)
                        // 递减 counter 从 50 到 0，对应递增 counter 从 duration-50 到 duration
                        // 我们需要将递增 counter 转换回递减值： dec_counter = duration - counter
                        let dec_counter = duration - counter;
                        an_alpha = tod_animate_curve(50, 0, dec_counter, 255, 0, TodCurves::Linear);
                    }
                } else {
                    // counter <= 50，即前 50 帧，淡入 0→255
                    // 原始： TodAnimateCurve(duration, duration-50, counter, 0, 255, CURVE_LINEAR)
                    // 递减 counter 从 duration-50 到 duration，即值从 50 到 0 递增？不对，
                    // 实际上递减模式：counter 从 200 减到 150（duration-50）时，开始淡入。
                    // 用递增 counter：前 50 帧淡入，直接使用 counter（0..50）
                    an_alpha = tod_animate_curve(
                        duration, duration - 50,
                        self.title_state_counter, // 0..50
                        0, 255, TodCurves::Linear,
                    );
                }

                // 绘制 PopCap Logo...
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
                let duration = self.title_state_duration;
                let counter = self.title_state_counter;

                // 先绘制 PopCap Logo（在 partner logo 之上淡出，仅最后 35 帧）
                // C++: if (mTitleStateCounter >= mTitleStateDuration - 35)
                // 递减 counter 从 35 到 0，对应递增 counter：duration - 35 <= counter <= duration
                if counter >= duration - 35 {
                    let dec_counter = duration - counter; // 35..0
                    let popcap_alpha = tod_animate_curve(
                        duration, duration - 35,
                        dec_counter, // 0..35
                        0, 255, TodCurves::Linear,
                    );
                    // 绘制 popcap logo，alpha 为 255 - popcap_alpha
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
                                g.set_color(&Color::new(255, 255, 255, (255 - popcap_alpha as u8) as u8));
                                g.draw_image_xy(img, x, y);
                            }
                        }
                    }
                }

                // 再绘制 Partner Logo
                // C++: 首先 else 分支对应 counter < duration-35 (前165帧) 淡入，最后35帧全亮
                // 递增 counter < duration-35 对应前165帧
                let partner_alpha = if counter < duration - 35 {
                    // C++: TodAnimateCurve(35, 0, counter, 255, 0, CURVE_LINEAR)
                    // 递减 counter 从 35 到 0，对应递增 counter 从 duration-35 到 duration？错！
                    // 实际上这一段是前165帧，递减 counter 从 200 到 35，所以递增 counter 从 0 到 165
                    // 我们需要将递减 counter = duration - counter 带入
                    let dec_counter = duration - counter; // 200..35
                    // 原始：TodAnimateCurve(35, 0, counter, 255, 0, CURVE_LINEAR) 
                    // 当递减 counter 从 35→0，alpha 从 255→0（淡出），但原文注释为“前165帧淡入”，所以应该是取反
                    // 仔细分析 C++ 原文：
                    //   if (mTitleStateCounter >= mTitleStateDuration - 35) { ... } // 最后35帧
                    //   else {
                    //       anAlpha = TodAnimateCurve(35, 0, mTitleStateCounter, 255, 0, CURVE_LINEAR);
                    //   }
                    // 这里 anAlpha 是 partner logo 的 alpha，在前165帧从 255 逐渐降到 0？这会导致 partner logo 在前165帧逐渐消失，最后35帧突然全亮。
                    // 原文注释为“淡入”，实际上应是“淡出”？再核对原 C++ 版本 A 的翻译：
                    //   anAlpha = TodAnimateCurve(35, 0, mTitleStateCounter, 255, 0, TodCurves::CURVE_LINEAR);
                    // 然后 g->SetColor(Color(255, 255, 255, anAlpha));
                    // 这意味着在前165帧，alpha 从 255 降到 0，即 partner logo 从完全可见到完全不可见（淡出）。
                    // 最后35帧全亮（无淡出？）。这似乎不符合“淡入”，但原 C++ 确实这样写。
                    // 我们保留此行为，不做改动。
                    let fade = tod_animate_curve(35, 0, dec_counter, 255, 0, TodCurves::Linear);
                    fade
                } else {
                    255 // 最后35帧全亮
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

                if !self.loader_screen_is_loaded {
                    g.set_color(&Color::BLACK);
                    g.fill_rect_xywh(0, 0, widget.width, widget.height);
                    return;
                }

                // 背景图
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

                if self.need_to_init {
                    return;
                }

                // PvZ Logo 动画（原始 C++ 在 need_to_init 检查之后绘制）
                let cc = self.cc(); // 递减值，用于动画曲线
                let a_logo_y = if cc > 60 {
                    tod_animate_curve(100, 60, cc, -150, 10, TodCurves::EaseIn)
                } else {
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

                // 加载条绘制
                let grass_x = self.start_button_x;
                let grass_y = self.start_button_y - 17;

                // 泥土
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
                    // 草皮
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
                        // DrawToPreload 预加载绘制
                        if let Some(rm_ptr) = app.base.resource_manager {
                            unsafe {
                                let rm = &*rm_ptr;
                                let shadow_ref = rm.get_image("plantshadow");
                                let shadow_ptr = shadow_ref.as_image_ptr();
                                if !shadow_ptr.is_null() {
                                    g.draw_image_f_xy(&*shadow_ptr, 1000.0, 0.0);
                                }
                            }
                        }
                    }
                } else {
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

                    // 滚动草皮卷轴（带缩放旋转）
                    let roll_len = self.cur_bar_width * 0.94;
                    let rotation = -roll_len / 180.0 * std::f32::consts::PI * 2.0;
                    let scale = tod_animate_curve_float_time(0.0, self.total_bar_width, self.cur_bar_width, 1.0, 0.5, TodCurves::Linear);

                    if let Some(rm_ptr) = app.base.resource_manager {
                        unsafe {
                            let rm = &*rm_ptr;
                            let cap_ref = rm.get_image("sodrollcap");
                            let cap_ptr = cap_ref.as_image_ptr();
                            if !cap_ptr.is_null() {
                                let cap = &*cap_ptr;
                                // 近似应用缩放和旋转，如果引擎支持，使用矩阵变换；否则用简单旋转，缩放丢失。
                                // 这里保留版本 B 的简单旋转，但添加缩放注释，后续可优化。
                                let cap_x = (grass_x + 11) as f32 + roll_len;
                                let cap_y = (grass_y - 3) as f32 - 35.0 * scale + 35.0;
                                // 如果可以，使用 draw_image_rotated_f 并传入缩放
                                g.draw_image_rotated_f_simple(cap, cap_x, cap_y, rotation as f64, None);
                            }
                        }
                    }
                }

                // 启动按钮文字
                if !self.start_button_label.is_empty() && self.start_button_visible {
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

        let app = unsafe { &mut *self.app };
        if app.m_close_request {
            return;
        }

        if !self.drawn_yet {
            return;
        }

        // === 首次绘制后的初始化 ===
        if self.title_state == TitleState::WaitingForFirstDraw {
            if let Some(ref mut music) = app.music {
                music.music_title_screen_init();
            }
            app.start_loading_thread();
            self.loader_screen_is_loaded = true; // 同步加载完成后加载条资源可用

            self.title_state = TitleState::PopCapLogo;
            if self.display_partner_logo {
                self.title_state_duration = 150;
            } else {
                self.title_state_duration = 200;
            }
            self.title_state_counter = 0; // 递增从0开始
        }

        // 快速加载键跳过 Logo（作弊模式额外检查）
        if self.quick_load_key != KEYCODE_UNKNOWN
            && self.title_state != TitleState::Running
        {
            self.title_state = TitleState::Loading;
            self.title_state_duration = 100;
            self.title_state_counter = 0;
        }

        self.title_state_counter += 1;
        self.title_age += 1;

        // 状态迁移
        match self.title_state {
            TitleState::PopCapLogo => {
                if self.title_state_counter >= self.title_state_duration {
                    if self.display_partner_logo {
                        self.title_state = TitleState::PartnerLogo;
                        self.title_state_counter = 0;
                        self.title_state_duration = 200;
                    } else {
                        self.title_state = TitleState::Loading;
                        self.title_state_counter = 0;
                        self.title_state_duration = 100;
                    }
                }
            }
            TitleState::PartnerLogo => {
                if self.title_state_counter >= self.title_state_duration {
                    self.title_state = TitleState::Loading;
                    self.title_state_counter = 0;
                    self.title_state_duration = 100;
                }
            }
            TitleState::Loading => {
                if !self.loader_screen_is_loaded {
                    // 等待资源加载完成（假设外部设置此标志）
                    return;
                }

                if self.need_to_init {
                    self.need_to_init = false;
                    self.start_button_x = (widget.width - 314) / 2;
                    self.start_button_y = 650;
                    self.start_button_label = "LOADING".to_string();
                    self.start_button_visible = true;
                    self.start_button_disabled = true;

                    let current_progress = app.get_loading_thread_progress();
                    let estimated_total_load_time = if current_progress > 0.000001 {
                        self.title_age as f32 / current_progress
                    } else {
                        3000.0
                    };

                    let mut load_time = estimated_total_load_time * (1.0 - current_progress);
                    load_time = clamp_float(load_time, 100.0, 3000.0);
                    self.bar_vel = self.total_bar_width / load_time;
                    self.bar_start_progress = if current_progress < 0.9 {
                        current_progress
                    } else {
                        0.9
                    };
                }

                let current_progress = app.get_loading_thread_progress();
                let loading_percent = if (1.0 - self.bar_start_progress).abs() > 0.0001 {
                    (current_progress - self.bar_start_progress) / (1.0 - self.bar_start_progress)
                } else {
                    1.0
                };

                let cc = self.cc();
                let button_y = if cc > 10 {
                    tod_animate_curve(60, 10, cc, 650, 534, TodCurves::EaseIn)
                } else {
                    tod_animate_curve(10, 0, cc, 534, 529, TodCurves::Bounce)
                };
                self.start_button_y = button_y;

                if let Some(ref mut es) = app.effect_system {
                    es.update();
                }

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
                }

                if loading_percent > self.prev_loading_percent + 0.01 || self.loading_thread_complete {
                    let bar_width = tod_animate_curve_float_time(
                        0.0, 1.0, loading_percent,
                        0.0, self.total_bar_width, TodCurves::EaseIn,
                    );
                    let diff = bar_width - self.cur_bar_width;
                    let acceleration = if self.loading_thread_complete {
                        0.0001
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
                    let max_velocity = 2.0;

                    if self.bar_vel < min_velocity {
                        self.bar_vel = min_velocity;
                    } else if self.bar_vel > max_velocity {
                        self.bar_vel = max_velocity;
                    }

                    self.prev_loading_percent = loading_percent;
                }

                // 加载完成处理
                if !self.loading_thread_complete && app.m_loading_thread_completed {
                    self.loading_thread_complete = true;
                    self.start_button_disabled = false;

                    // 快速加载键映射（对应 C++ 原始键值）
                    if self.quick_load_key == KEYCODE_END {
                        app.fast_load(GameMode::ChallengeZenGarden);
                    } else if self.quick_load_key == KEYCODE_M {
                        app.loading_completed();
                    } else if self.quick_load_key == KEYCODE_S {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.show_challenge_screen(ChallengePage::Survival as i32);
                    } else if self.quick_load_key == KEYCODE_C {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.show_challenge_screen(ChallengePage::Challenge as i32);
                    } else if self.quick_load_key == KEYCODE_U {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.pre_new_game(GameMode::Upsell, false);
                    } else if self.quick_load_key == KEYCODE_I {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.pre_new_game(GameMode::Intro, false);
                    } else if self.quick_load_key == KEYCODE_P {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.show_challenge_screen(ChallengePage::Puzzle as i32);
                    } else if self.quick_load_key == KEYCODE_R {
                        app.loading_completed();
                        app.kill_game_selector();
                        app.show_credit_screen();
                    } else if (app.m_tod_cheat_keys && app.player_info.is_some())
                        && self.quick_load_key == KEYCODE_T
                    {
                        app.fast_load(GameMode::Adventure);
                    } else {
                        self.start_button_visible = true;
                        self.start_button_label = "CLICK TO START".to_string();
                    }
                }

                // 里程碑动画与音效
                let trigger_points = [
                    self.total_bar_width * 0.11,
                    self.total_bar_width * 0.32,
                    self.total_bar_width * 0.54,
                    self.total_bar_width * 0.72,
                    self.total_bar_width * 0.91,
                ];

                for (i, &trigger) in trigger_points.iter().enumerate() {
                    if prev_width < trigger && self.cur_bar_width >= trigger {
                        let reanim_type = if i == 4 {
                            crate::lawn::game_enums::ReanimationType::LoadbarZombiehead
                        } else {
                            crate::lawn::game_enums::ReanimationType::LoadbarSprout
                        };
                        let pos_x = trigger + 225.0;
                        let pos_y = 511.0;

                        if let Some(anim) = app.add_reanimation(pos_x, pos_y, 0, reanim_type as i32) {
                            unsafe {
                                (*anim).m_anim_time = 18.0;
                                (*anim).m_loop_type = crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold;
                            }
                            if i == 1 || i == 3 {
                                unsafe { (*anim).override_scale(-1.0, 1.0); }
                            } else if i == 2 {
                                unsafe {
                                    (*anim).set_position(pos_x, pos_y - 5.0);
                                    (*anim).override_scale(1.1, 1.3);
                                }
                            } else if i == 4 {
                                unsafe { (*anim).set_position(pos_x - 20.0, pos_y); }
                            }

                            if i == 4 {
                                app.play_foley(FoleyType::Plant as i32);
                                app.play_foley(FoleyType::Groan as i32);
                            } else {
                                app.play_foley(FoleyType::Plant as i32);
                            }
                        }
                    }
                }
            }
            TitleState::Running => {
                // 等待用户交互
            }
            _ => {}
        }
    }

    fn mouse_down_btn(&mut self, widget: &mut Widget, x: i32, y: i32, btn: i32, click: i32) {
        let _ = (widget, x, y, btn, click);
        let app = unsafe { &mut *self.app };
        // 原 C++：只要 loading_thread_complete 为 true 即可点击，不要求状态为 Running
        if self.loading_thread_complete {
            app.play_foley(FoleyType::Beep as i32);
            app.loading_completed();
        }
    }

    fn key_down(&mut self, widget: &mut Widget, key: KeyCode, wm: &mut WidgetManager) {
        let _ = (widget, wm);
        let app = unsafe { &mut *self.app };

        if self.loading_thread_complete {
            app.play_foley(FoleyType::Beep as i32);
            app.loading_completed();
            return;
        }

        // 作弊键保存（仅在作弊模式且玩家信息存在时）
        if app.m_tod_cheat_keys && app.player_info.is_some() {
            self.quick_load_key = key;
        }
    }
}