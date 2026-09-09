// TitleScreen 完整翻译 — 对应 C++ src/Lawn/Widget/TitleScreen.cpp / TitleScreen.h
//
// [TRANSLATION_NOTE]:
// - mStartButton（C++ HyperlinkWidget*）在 Rust 架构中内化为 start_button_* 字段，
//   按钮文字由 draw 末尾自行绘制，故 AddedToManager/RemovedFromManager 为
//   no-op（保留 C++ 的挂接语义注释）；ButtonPress/ButtonDepress 仍按 C++ 事件语义实现。
// - mTitleStateCounter 与 C++ 一致为递减计数器（从 duration 递减到 0），
//   动画曲线直接以该递减值为 time_age，逐参数对应 PvzpAnimateCurve。
// - 声音沿用既有 Foley 近似（Beep 对应 SOUND_BUTTONCLICK，Plant/Groan 对应
//   SOUND_LOADINGBAR_FLOWER/ZOMBIE）；图像访问带 get_image 缺省守卫。

use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::{
    KeyCode, KEYCODE_UNKNOWN,
    KEYCODE_M, KEYCODE_S, KEYCODE_C, KEYCODE_U, KEYCODE_I, KEYCODE_P, KEYCODE_R, KEYCODE_T,
};
use crate::framework::color::Color;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::game_enums::{TodCurves, GameMode, ChallengePage, ReanimationType};
use crate::todlib::reanimator::ReanimLoopType;
use crate::todlib::tod_common::{
    tod_animate_curve, tod_animate_curve_float_time, clamp_float,
};
use crate::todlib::tod_foley::FoleyType;

/// C++ KeyCodes.h: KEYCODE_ASCIIEND = 0x5A (90)。注意不是导航键 End(35)，
/// Rust key_codes 未导出该名字，此处按 C++ 值定义。
pub const KEYCODE_ASCIIEND: KeyCode = 90;

/// C++ TitleState 枚举（TitleScreen.h）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TitleState {
    WaitingForFirstDraw = 0,
    PopCapLogo = 1,
    PartnerLogo = 2,
    Screen = 3,
}

/// C++ TitleScreen 匿名枚举成员（TitleScreen_Start / TitleScreen_Register）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TitleScreenButtonId {
    Start = 0,
    Register = 1,
}

pub struct TitleScreenImpl {
    pub app: *mut LawnApp,

    // ---- 状态机（对应 C++ mTitleState 等） ----
    pub title_state: TitleState,
    pub title_state_counter: i32,
    pub title_state_duration: i32,
    pub title_age: i32,
    pub drawn_yet: bool,
    pub need_to_init: bool,
    pub loading_thread_complete: bool,
    pub loader_screen_is_loaded: bool,

    // ---- 加载条（对应 C++ mCurBarWidth 等） ----
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

    // ---- 启动按钮（C++ HyperlinkWidget* mStartButton 内化） ----
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
    /// 对应 C++ TitleScreen::TitleScreen(LawnApp* theApp)
    pub fn new(app: *mut LawnApp) -> Self {
        let app_ref = unsafe { &*app };
        let display_partner = app_ref.get_boolean("DisplayPartnerLogo", false);

        TitleScreenImpl {
            app,
            title_state: TitleState::WaitingForFirstDraw,
            title_state_counter: 0,
            title_state_duration: 0,
            title_age: 0,
            drawn_yet: false,
            need_to_init: true,
            loading_thread_complete: false,
            loader_screen_is_loaded: false,
            cur_bar_width: 0.0,
            total_bar_width: 314.0,
            bar_vel: 0.2,
            bar_start_progress: 0.0,
            prev_loading_percent: 0.0,
            need_register: false,
            register_clicked: false,
            need_show_register_box: false,
            display_partner_logo: display_partner,
            start_button_label: String::new(),
            start_button_font: None,
            start_button_visible: false,
            start_button_disabled: true,
            start_button_x: 0,
            start_button_y: 0,
            quick_load_key: KEYCODE_UNKNOWN,
        }
    }

    /// 对应 C++ TitleScreen::DrawToPreload — 预加载绘制（绘制植物阴影）
    pub fn draw_to_preload(&self, g: &mut Graphics) {
        let app = unsafe { &*self.app };
        if let Some(rm_ptr) = app.base.resource_manager {
            unsafe {
                let rm = &*rm_ptr;
                let shared_ref = rm.get_image("plantshadow");
                let img_ptr = shared_ref.as_image_ptr();
                if !img_ptr.is_null() {
                    g.draw_image_f_xy(&*img_ptr, 1000.0, 0.0);
                }
            }
        }
    }

    /// 对应 C++ TitleScreen::Resize — 仅调用基类 Widget::Resize
    pub fn resize(&mut self, widget: &mut Widget, x: i32, y: i32, w: i32, h: i32) {
        widget.resize(x, y, w, h);
    }

    /// 对应 C++ TitleScreen::AddedToManager
    /// C++ 中：Widget::AddedToManager + theWidgetManager->AddWidget(mStartButton)。
    /// [TRANSLATION_NOTE]: mStartButton 已内化为字段（无真实子控件可挂接），故为 no-op。
    pub fn added_to_manager(&mut self, _widget: &mut Widget, _wm: *mut WidgetManager) {
        // (void)theWidgetManager;
        // theWidgetManager->AddWidget(mStartButton);
    }

    /// 对应 C++ TitleScreen::RemovedFromManager
    /// C++ 中：Widget::RemovedFromManager + theWidgetManager->RemoveWidget(mStartButton)。
    /// [TRANSLATION_NOTE]: mStartButton 已内化，故为 no-op。
    pub fn removed_from_manager(&mut self, _widget: &mut Widget) {
        // (void)theWidgetManager;
        // theWidgetManager->RemoveWidget(mStartButton);
    }

    /// 对应 C++ TitleScreen::ButtonPress — 按钮按下播放点击音
    pub fn button_press(&mut self, _the_id: i32) {
        // (void)theId;
        let app = unsafe { &*self.app };
        app.play_foley(FoleyType::Beep as i32); // C++: PlaySample(SOUND_BUTTONCLICK)
    }

    /// 对应 C++ TitleScreen::ButtonDepress — 按钮释放分发
    pub fn button_depress(&mut self, the_id: i32) {
        let id = the_id as i32;
        if id == TitleScreenButtonId::Start as i32 {
            let app = unsafe { &mut *self.app };
            app.loading_completed();
        } else if id == TitleScreenButtonId::Register as i32 {
            self.register_clicked = true;
        }
    }

    /// 对应 C++ TitleScreen::SetRegistered（头文件声明，原 cpp 未提供定义，
    /// 此处提供与 mNeedRegister 语义一致的空实现）
    pub fn set_registered(&mut self) {
        // C++ 无实现体；mNeedRegister 由外部注册流程驱动
    }

    // ============================================================
    // 以下为 WidgetImpl 接口实现（对应 C++ 虚函数分派）
    // ============================================================

    /// 对应 C++ TitleScreen::Draw（由 WidgetImpl::draw 转调）
    fn draw_impl(&mut self, widget: &Widget, g: &mut Graphics) {
        g.set_linear_blend(true);

        let app = unsafe { &*self.app };

        // ---- TITLESTATE_WAITING_FOR_FIRST_DRAW：全黑，标记首帧 ----
        if self.title_state == TitleState::WaitingForFirstDraw {
            g.set_color(&Color::BLACK);
            g.fill_rect_xywh(0, 0, widget.width, widget.height);
            if !self.drawn_yet {
                // C++: PvzpTraceAndLogLn("First Draw Time: %d ms", ...) + PvzpHesitationTrace
                self.drawn_yet = true;
            }
            return;
        }

        // ---- TITLESTATE_POPCAP_LOGO ----
        if self.title_state == TitleState::PopCapLogo {
            g.set_color(&Color::BLACK);
            g.fill_rect_xywh(0, 0, widget.width, widget.height);

            let mut an_alpha: i32 = 255;
            if self.title_state_counter < self.title_state_duration - 50 {
                if !self.display_partner_logo {
                    an_alpha = tod_animate_curve(
                        50, 0, self.title_state_counter, 255, 0, TodCurves::Linear,
                    );
                }
            } else {
                an_alpha = tod_animate_curve(
                    self.title_state_duration, self.title_state_duration - 50,
                    self.title_state_counter, 0, 255, TodCurves::Linear,
                );
            }

            g.set_colorize_images(true);
            g.set_color(&Color::new(255, 255, 255, an_alpha as u8));
            if let Some(rm_ptr) = app.base.resource_manager {
                unsafe {
                    let rm = &*rm_ptr;
                    let shared_ref = rm.get_image("popcap_logo");
                    let img_ptr = shared_ref.as_image_ptr();
                    if !img_ptr.is_null() {
                        let img = &*img_ptr;
                        let x = (widget.width - img.get_width()) / 2;
                        let y = (widget.height - img.get_height()) / 2;
                        g.draw_image_xy(img, x, y);
                    }
                }
            }
            g.set_colorize_images(false);
            return;
        }

        // ---- TITLESTATE_PARTNER_LOGO ----
        if self.title_state == TitleState::PartnerLogo {
            g.set_color(&Color::BLACK);
            g.fill_rect_xywh(0, 0, widget.width, widget.height);

            g.set_colorize_images(true);
            let mut an_alpha: i32 = 255;
            if self.title_state_counter >= self.title_state_duration - 35 {
                an_alpha = tod_animate_curve(
                    self.title_state_duration, self.title_state_duration - 35,
                    self.title_state_counter, 0, 255, TodCurves::Linear,
                );
                // 淡出期间的 PopCap logo（255 - anAlpha）
                g.set_color(&Color::new(255, 255, 255, 255 - an_alpha as u8));
                if let Some(rm_ptr) = app.base.resource_manager {
                    unsafe {
                        let rm = &*rm_ptr;
                        let shared_ref = rm.get_image("popcap_logo");
                        let img_ptr = shared_ref.as_image_ptr();
                        if !img_ptr.is_null() {
                            let img = &*img_ptr;
                            let x = (widget.width - img.get_width()) / 2;
                            let y = (widget.height - img.get_height()) / 2;
                            g.draw_image_xy(img, x, y);
                        }
                    }
                }
            } else {
                an_alpha = tod_animate_curve(
                    35, 0, self.title_state_counter, 255, 0, TodCurves::Linear,
                );
            }
            g.set_color(&Color::new(255, 255, 255, an_alpha as u8));
            if let Some(rm_ptr) = app.base.resource_manager {
                unsafe {
                    let rm = &*rm_ptr;
                    let shared_ref = rm.get_image("partner_logo");
                    let img_ptr = shared_ref.as_image_ptr();
                    if !img_ptr.is_null() {
                        let img = &*img_ptr;
                        let x = (widget.width - img.get_width()) / 2;
                        let y = (widget.height - img.get_height()) / 2;
                        g.draw_image_xy(img, x, y);
                    }
                }
            }
            g.set_colorize_images(false);
            return;
        }

        // ---- 加载屏资源未就绪：全黑 ----
        if !self.loader_screen_is_loaded {
            g.set_color(&Color::BLACK);
            g.fill_rect_xywh(0, 0, widget.width, widget.height);
            return;
        }

        // ---- TITLESTATE_SCREEN ----
        // C++: g->DrawImage(IMAGE_TITLESCREEN, 0, 0);
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

        // PvZ Logo：C++ aLogoY 两条曲线
        let a_logo_y = if self.title_state_counter > 60 {
            tod_animate_curve(100, 60, self.title_state_counter, -150, 10, TodCurves::EaseIn)
        } else {
            tod_animate_curve(60, 50, self.title_state_counter, 10, 15, TodCurves::Bounce)
        };
        if let Some(rm_ptr) = app.base.resource_manager {
            unsafe {
                let rm = &*rm_ptr;
                let shared_ref = rm.get_image("pvz_logo");
                let img_ptr = shared_ref.as_image_ptr();
                if !img_ptr.is_null() {
                    let img = &*img_ptr;
                    g.draw_image_xy(img, widget.width / 2 - img.get_width() / 2, a_logo_y);
                }
            }
        }

        // 加载条：aGrassX = mStartButton->mX；aGrassY = mStartButton->mY - 17
        let a_grass_x = self.start_button_x;
        let a_grass_y = self.start_button_y - 17;

        // C++: g->DrawImage(IMAGE_LOADBAR_DIRT, aGrassX, aGrassY + 18);
        if let Some(rm_ptr) = app.base.resource_manager {
            unsafe {
                let rm = &*rm_ptr;
                let shared_ref = rm.get_image("loadbar_dirt");
                let img_ptr = shared_ref.as_image_ptr();
                if !img_ptr.is_null() {
                    g.draw_image_xy(&*img_ptr, a_grass_x, a_grass_y + 18);
                }
            }
        }

        if self.cur_bar_width >= self.total_bar_width {
            // C++: 满宽度时完整绘制草皮；完全加载后绘制 DrawToPreload
            if let Some(rm_ptr) = app.base.resource_manager {
                unsafe {
                    let rm = &*rm_ptr;
                    let shared_ref = rm.get_image("loadbar_grass");
                    let img_ptr = shared_ref.as_image_ptr();
                    if !img_ptr.is_null() {
                        g.draw_image_xy(&*img_ptr, a_grass_x, a_grass_y);
                    }
                }
            }
            if self.loading_thread_complete {
                self.draw_to_preload(g);
            }
        } else {
            // C++: Graphics aClipG(*g); aClipG.ClipRect(240, aGrassY, mCurBarWidth,
            //       IMAGE_LOADBAR_GRASS->mHeight); aClipG.DrawImage(IMAGE_LOADBAR_GRASS, ...)
            let mut clip_g = g.create();
            clip_g.set_clip_rect_xywh(240, a_grass_y, self.cur_bar_width as i32, 50);
            if let Some(rm_ptr) = app.base.resource_manager {
                unsafe {
                    let rm = &*rm_ptr;
                    let shared_ref = rm.get_image("loadbar_grass");
                    let img_ptr = shared_ref.as_image_ptr();
                    if !img_ptr.is_null() {
                        clip_g.draw_image_xy(&*img_ptr, a_grass_x, a_grass_y);
                    }
                }
            }

            // 滚动草皮卷（C++ PvzpBltMatrix + PvzpScaleRotateTransformMatrix）
            let a_roll_len = self.cur_bar_width * 0.94;
            let a_rotation = -a_roll_len / 180.0 * std::f32::consts::PI * 2.0;
            let a_scale = tod_animate_curve_float_time(
                0.0, self.total_bar_width, self.cur_bar_width, 1.0, 0.5, TodCurves::Linear,
            );
            let cap_x = a_grass_x as f32 + 11.0 + a_roll_len;
            let cap_y = a_grass_y as f32 - 3.0 - 35.0 * a_scale + 35.0;
            if let Some(rm_ptr) = app.base.resource_manager {
                unsafe {
                    let rm = &*rm_ptr;
                    let shared_ref = rm.get_image("sodrollcap");
                    let img_ptr = shared_ref.as_image_ptr();
                    if !img_ptr.is_null() {
                        g.draw_image_rotated_f_simple(
                            &*img_ptr, cap_x, cap_y, a_rotation as f64, None,
                        );
                    }
                }
            }
        }

        // C++: 遍历 mApp->mEffectSystem->mReanimationHolder->mReanimations 逐个 Draw
        if let Some(es) = &app.effect_system {
            for reanim in &es.reanimations {
                reanim.draw(g);
            }
        }

        // 启动按钮文字（C++ 由 HyperlinkWidget 自行绘制；此处内化绘制）
        self.draw_start_button_text(widget, g);
    }

    /// 启动按钮文字绘制（C++ FONT_BRIANNETOD16 + mColor(218,184,33)）
    fn draw_start_button_text(&mut self, widget: &Widget, g: &mut Graphics) {
        if self.start_button_label.is_empty() || !self.start_button_visible {
            return;
        }
        if self.start_button_font.is_none() {
            let mut f = Font::new("Briannetod", 16);
            f.ascent = 13;
            f.font_height = 16;
            self.start_button_font = Some(Box::new(f));
        }
        if let Some(ref font) = self.start_button_font {
            let text_width = font.string_width(&self.start_button_label);
            let text_x = widget.width / 2 - text_width / 2;
            let text_y = self.start_button_y + (50 + font.get_ascent()) / 2 - 1;
            g.set_color(&Color::new(218, 184, 33, 255));
            let fptr = &**font as *const Font as *mut Font;
            g.set_font(fptr);
            g.draw_string(&self.start_button_label, text_x, text_y);
        }
    }
}

impl WidgetImpl for TitleScreenImpl {
    /// 对应 C++ TitleScreen::Draw — 转调 draw_impl（C++ Draw 全逻辑）
    fn draw(&mut self, widget: &Widget, g: &mut Graphics) {
        self.draw_impl(widget, g);
    }

    /// 对应 C++ TitleScreen::Update
    fn update(&mut self, widget: &mut Widget) {
        // C++: Widget::Update() —— Rust 由 Widget::update 分派，不重复调用

        let app = unsafe { &mut *self.app };
        // C++: if (mApp->mShutdown) return;
        if app.m_close_request {
            return;
        }

        // C++: MarkDirty();
        widget.mark_dirty();
        // C++: if (!mDrawnYet) return;
        if !self.drawn_yet {
            return;
        }

        // ---- TITLESTATE_WAITING_FOR_FIRST_DRAW：初始化音乐/加载线程，进入 POPCAP ----
        if self.title_state == TitleState::WaitingForFirstDraw {
            // C++: mApp->mMusic->MusicTitleScreenInit();
            if let Some(ref mut music) = app.music {
                music.music_title_screen_init();
            }
            // C++: mApp->StartLoadingThread();
            app.start_loading_thread();
            // [TRANSLATION_NOTE]: C++ 中 mLoaderScreenIsLoaded 由加载线程置位（原子），
            // Rust 侧同步加载，此处直接置位。
            self.loader_screen_is_loaded = true;

            self.title_state = TitleState::PopCapLogo;
            if self.display_partner_logo {
                self.title_state_duration = 150;
            } else {
                self.title_state_duration = 200;
            }
            self.title_state_counter = self.title_state_duration;
        }

        // ---- 快速加载键跳过 Logo ----
        // C++: if (mQuickLoadKey != KEYCODE_UNKNOWN && mTitleState != TITLESTATE_SCREEN)
        if self.quick_load_key != KEYCODE_UNKNOWN && self.title_state != TitleState::Screen {
            self.title_state = TitleState::Screen;
            self.title_state_duration = 0;
            self.title_state_counter = 100;
        }

        // C++: mTitleAge++; if (mTitleStateCounter > 0) mTitleStateCounter--;
        self.title_age += 1;
        if self.title_state_counter > 0 {
            self.title_state_counter -= 1;
        }

        // ---- TITLESTATE_POPCAP_LOGO：结束时进入 SCREEN ----
        if self.title_state == TitleState::PopCapLogo {
            if self.title_state_counter == 0 {
                if self.display_partner_logo {
                    self.title_state = TitleState::Screen;
                    self.title_state_duration = 200;
                    self.title_state_counter = 200;
                } else {
                    self.title_state = TitleState::Screen;
                    self.title_state_duration = 100;
                    self.title_state_counter = 100;
                }
            }
            return;
        }
        // ---- TITLESTATE_PARTNER_LOGO：结束时进入 SCREEN ----
        else if self.title_state == TitleState::PartnerLogo {
            if self.title_state_counter == 0 {
                self.title_state = TitleState::Screen;
                self.title_state_duration = 100;
                self.title_state_counter = 100;
            }
            return;
        }

        // ---- SCREEN：加载屏资源未就绪则等待 ----
        if !self.loader_screen_is_loaded {
            return;
        }

        // C++: float aCurrentProgress = mApp->GetLoadingThreadProgress();
        let a_current_progress = app.get_loading_thread_progress();

        // ---- 首次进入 SCREEN 的初始化（仅一次） ----
        if self.need_to_init {
            self.need_to_init = false;

            // C++: mStartButton->mLabel = PvzpStringTranslate("[LOADING]"); SetFont(FONT_BRIANNETOD16);
            self.start_button_label = "[LOADING]".to_string();
            self.start_button_font = None;
            // C++: Resize((mWidth - IMAGE_LOADBAR_DIRT->mWidth)/2, 650, mTotalBarWidth, 50)
            self.start_button_x = (widget.width - self.total_bar_width as i32) / 2;
            self.start_button_y = 650;
            self.start_button_visible = true;

            // C++: 估算总加载时间
            let a_estimated_total_load_time = if a_current_progress > 0.000001 {
                self.title_age as f32 / a_current_progress
            } else {
                3000.0
            };
            let mut a_load_time = a_estimated_total_load_time * (1.0 - a_current_progress);
            a_load_time = clamp_float(a_load_time, 100.0, 3000.0);
            self.bar_vel = self.total_bar_width / a_load_time;
            self.bar_start_progress = a_current_progress.min(0.9);
        }

        // C++: float aLoadingPercent = (aCurrentProgress - mBarStartProgress) / (1 - mBarStartProgress);
        let a_loading_percent = (a_current_progress - self.bar_start_progress)
            / (1.0 - self.bar_start_progress);

        // C++: aButtonY 两条曲线；mStartButton->Resize(mStartButton->mX, aButtonY, ...)
        let a_button_y = if self.title_state_counter > 10 {
            tod_animate_curve(60, 10, self.title_state_counter, 650, 534, TodCurves::EaseIn)
        } else {
            tod_animate_curve(10, 0, self.title_state_counter, 534, 529, TodCurves::Bounce)
        };
        self.start_button_y = a_button_y;

        // C++: if (mTitleStateCounter > 0) return;  —— 剩余帧数耗尽后才推进加载条
        if self.title_state_counter > 0 {
            return;
        }

        // C++: mApp->mEffectSystem->Update();
        if let Some(ref mut es) = app.effect_system {
            es.update();
        }

        // ---- 加载条推进 ----
        // C++: float aPrevWidth = mCurBarWidth; mCurBarWidth += mBarVel;
        let a_prev_width = self.cur_bar_width;
        self.cur_bar_width += self.bar_vel;
        if !self.loading_thread_complete {
            // C++: if (mCurBarWidth > mTotalBarWidth * 0.99f) clamp 到 0.99
            if self.cur_bar_width > self.total_bar_width * 0.99 {
                self.cur_bar_width = self.total_bar_width * 0.99;
            }
        } else if self.cur_bar_width > self.total_bar_width {
            // C++: mStartButton->mLabel = PvzpStringTranslate("[CLICK_TO_START]");
            self.start_button_label = "[CLICK_TO_START]".to_string();
            self.cur_bar_width = self.total_bar_width;
        }

        // C++: 加载百分比刷新时按曲线调速
        // if (aLoadingPercent > mPrevLoadingPercent + 0.01f || mLoadingThreadComplete)
        if a_loading_percent > self.prev_loading_percent + 0.01 || self.loading_thread_complete {
            let a_bar_width = tod_animate_curve_float_time(
                0.0, 1.0, a_loading_percent, 0.0, self.total_bar_width, TodCurves::EaseIn,
            );
            let a_diff = a_bar_width - self.cur_bar_width;
            let mut a_acceleration = tod_animate_curve_float_time(
                0.0, 1.0, a_loading_percent, 0.0001, 0.00001, TodCurves::Linear,
            );
            if self.loading_thread_complete {
                a_acceleration = 0.0001;
            }
            self.bar_vel += a_diff * a_diff.abs() * a_acceleration;

            // C++: 最小/最大速度（mCheatKeys 时放宽）
            let mut a_min_velocity = tod_animate_curve_float_time(
                0.0, 1.0, a_loading_percent, 0.2, 0.01, TodCurves::Linear,
            );
            let mut a_max_velocity = 2.0;
            if app.m_tod_cheat_keys {
                a_min_velocity = 0.0;
                a_max_velocity = 100.0;
            }

            if self.bar_vel < a_min_velocity {
                self.bar_vel = a_min_velocity;
            } else if self.bar_vel > a_max_velocity {
                self.bar_vel = a_max_velocity;
            }

            self.prev_loading_percent = a_loading_percent;
        }

        // ---- 加载线程完成 → 快速加载键映射 / 显示启动按钮 ----
        // C++: if (!mLoadingThreadComplete && (IsInDemoMode() ? mLoaded : mLoadingThreadCompleted.load()))
        // [TRANSLATION_NOTE]: Rust 无 demo 模式，等价于判断 mLoadingThreadCompleted
        if !self.loading_thread_complete && app.m_loading_thread_completed {
            self.loading_thread_complete = true;
            // C++: mStartButton->SetDisabled(false);
            self.start_button_disabled = false;

            if self.quick_load_key == KEYCODE_ASCIIEND {
                // C++: FastLoad(GAMEMODE_CHALLENGE_ZEN_GARDEN)
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
            } else if app.m_tod_cheat_keys && app.player_info.is_some() && self.quick_load_key == KEYCODE_T {
                // C++: FastLoad(GAMEMODE_ADVENTURE)
                app.fast_load(GameMode::Adventure);
            } else {
                // C++: mStartButton->SetVisible(true);
                self.start_button_visible = true;
            }
        }

        // ---- 加载条里程碑：触发 Sprout / ZombieHead 动画与音效 ----
        // C++: float aTriggerPoint[] = { 0.11, 0.32, 0.54, 0.72, 0.91 } * mTotalBarWidth
        let a_trigger_point = [
            self.total_bar_width * 0.11,
            self.total_bar_width * 0.32,
            self.total_bar_width * 0.54,
            self.total_bar_width * 0.72,
            self.total_bar_width * 0.91,
        ];

        for (i, &a_trigger) in a_trigger_point.iter().enumerate() {
            // C++: if (aPrevWidth < aTriggerPoint[i] && mCurBarWidth >= aTriggerPoint[i])
            if a_prev_width < a_trigger && self.cur_bar_width >= a_trigger {
                let a_reanim_type = if i == 4 {
                    ReanimationType::LoadbarZombiehead
                } else {
                    ReanimationType::LoadbarSprout
                };
                let a_pos_x = a_trigger + 225.0;
                let a_pos_y = 511.0;
                // C++: mApp->AddReanimation(aPosX, aPosY, 0, aReanimType)
                if let Some(a_sprout) = app.add_reanimation(a_pos_x, a_pos_y, 0, a_reanim_type as i32) {
                    unsafe {
                        // C++: aSproutReanim->mAnimRate = 18.0f; mLoopType = REANIM_PLAY_ONCE_AND_HOLD;
                        (*a_sprout).m_anim_rate = 18.0;
                        (*a_sprout).m_loop_type = ReanimLoopType::PlayOnceAndHold;
                    }
                    if i == 1 || i == 3 {
                        // C++: OverrideScale(-1.0f, 1.0f)
                        unsafe { (*a_sprout).override_scale(-1.0, 1.0); }
                    } else if i == 2 {
                        // C++: SetPosition + OverrideScale(1.1f, 1.3f)
                        unsafe {
                            (*a_sprout).set_position(a_pos_x, a_pos_y - 5.0);
                            (*a_sprout).override_scale(1.1, 1.3);
                        }
                    } else if i == 4 {
                        // C++: SetPosition(aPosX - 20.0f, aPosY)
                        unsafe { (*a_sprout).set_position(a_pos_x - 20.0, a_pos_y); }
                    }

                    if i == 4 {
                        // C++: PlaySample(SOUND_LOADINGBAR_FLOWER); PlaySample(SOUND_LOADINGBAR_ZOMBIE);
                        app.play_foley(FoleyType::Plant as i32);
                        app.play_foley(FoleyType::Groan as i32);
                    } else {
                        // C++: PlaySample(SOUND_LOADINGBAR_FLOWER);
                        app.play_foley(FoleyType::Plant as i32);
                    }
                }
            }
        }
    }

    /// 对应 C++ TitleScreen::MouseDown — 加载完成后任意点击进入
    fn mouse_down_btn(&mut self, _widget: &mut Widget, _x: i32, _y: i32, _btn: i32, _click: i32) {
        // C++: (void)x; (void)y; (void)theClickCount;
        if self.loading_thread_complete {
            let app = unsafe { &mut *self.app };
            // C++: mApp->PlaySample(Sexy::SOUND_BUTTONCLICK);
            app.play_foley(FoleyType::Beep as i32);
            // C++: mApp->LoadingCompleted();
            app.loading_completed();
        }
    }

    /// 对应 C++ TitleScreen::KeyDown
    fn key_down(&mut self, _widget: &mut Widget, the_key: KeyCode, _wm: &mut WidgetManager) {
        let app = unsafe { &mut *self.app };
        // C++: if (mLoadingThreadComplete) { PlaySample; LoadingCompleted; }
        if self.loading_thread_complete {
            app.play_foley(FoleyType::Beep as i32);
            app.loading_completed();
        }

        // C++: if (mApp->mCheatKeys && mApp->mPlayerInfo) mQuickLoadKey = theKey;
        // 注意：C++ 在 LoadingCompleted 之后不 return，仍会记录快速加载键
        if app.m_tod_cheat_keys && app.player_info.is_some() {
            self.quick_load_key = the_key;
        }
    }
}