// PvZ Portable Rust 翻译 — CreditScreen（制作人员列表）
// 对应 C++ src/Lawn/Widget/CreditScreen.h / CreditScreen.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::widget::Widget;
use crate::framework::key_codes::KeyCode;
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::color::Color;
use crate::lawn::game_enums::*;
use crate::lawn::widget::game_button::GameButton;
use crate::todlib::reanimator::Reanimation;

/// 制作人员阶段（对应 C++ CreditsPhase）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditsPhase {
    Main1,
    Main2,
    Main3,
    End,
}

/// 制作人员图层（对应 C++ CreditLayer）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditLayer {
    Background,
    Zombie,
    Top,
}

/// 文字类型（对应 C++ CreditWordType）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditWordType {
    Aa,
    Ee,
    Aw,
    Oh,
    Off,
}

/// 脑子动画类型（对应 C++ CreditBrainType）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditBrainType {
    FlyOn,
    FastOn,
    NextWord,
    FastOff,
    FlyOff,
    Off,
}

/// 制作人员时间控制（对应 C++ CreditsTiming）
pub struct CreditsTiming {
    pub frame: f32,
    pub word_type: CreditWordType,
    pub word_x: i32,
    pub brain_type: CreditBrainType,
}

/// 制作人员界面（对应 C++ CreditScreen）
pub struct CreditScreen {
    pub close_button: Option<*mut GameButton>,
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub credits_phase: CreditsPhase,
    pub credits_phase_counter: i32,
    pub credits_reanim_id: ReanimationID,
    pub fog_particle_id: ParticleSystemID,
    pub blink_countdown: i32,
    pub main_menu_button: Option<*mut DialogButton>,
    pub replay_button: Option<*mut DialogButton>,
    pub overlay_widget: Option<*mut Widget>,
    pub draw_brain: bool,
    pub brain_pos_x: f32,
    pub brain_pos_y: f32,
    pub update_count: i32,
    pub draw_count: i32,
    pub dont_sync: bool,
    pub credits_paused: bool,
    pub original_music_volume: f64,
    pub preloaded: bool,
    pub last_draw_count: i32,
}

impl CreditScreen {
    pub fn new() -> Self {
        CreditScreen {
            close_button: None,
            app: None,
            credits_phase: CreditsPhase::Main1,
            credits_phase_counter: 0,
            credits_reanim_id: REANIMATIONID_NULL,
            fog_particle_id: PARTICLESYSTEMID_NULL,
            blink_countdown: 0,
            main_menu_button: None,
            replay_button: None,
            overlay_widget: None,
            draw_brain: false,
            brain_pos_x: 0.0,
            brain_pos_y: 0.0,
            update_count: 0,
            draw_count: 0,
            dont_sync: false,
            credits_paused: false,
            original_music_volume: 0.0,
            preloaded: false,
            last_draw_count: 0,
        }
    }

    pub fn update(&mut self) { /* TODO: CreditScreen.cpp */ }
    pub fn draw(&self, _g: &mut Graphics) { /* TODO: CreditScreen.cpp */ }
    pub fn key_char(&mut self, _c: char) { /* TODO: CreditScreen.cpp */ }
    pub fn key_down(&mut self, _key: KeyCode) { /* TODO: CreditScreen.cpp */ }
    pub fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) { /* TODO: CreditScreen.cpp */ }
    pub fn button_press(&mut self, _id: i32) { /* TODO: CreditScreen.cpp */ }
    pub fn button_depress(&mut self, _id: i32) { /* TODO: CreditScreen.cpp */ }
    pub fn play_reanim(&self, _index: i32) -> Option<*mut Reanimation> { None /* TODO */ }
    pub fn jump_to_frame(&self, _phase: CreditsPhase, _frame: f32) { /* TODO: CreditScreen.cpp */ }
    pub fn draw_fog_effect(&self, _g: &mut Graphics, _time: f32) { /* TODO */ }
    pub fn update_blink(&mut self) { /* TODO */ }
    pub fn draw_final_credits(&self, _g: &mut Graphics) { /* TODO */ }
    pub fn draw_overlay(&self, _g: &mut Graphics) { /* TODO */ }
    pub fn update_movie(&mut self) { /* TODO */ }
    pub fn pause_credits(&mut self) { /* TODO */ }
    pub fn pre_load_credits(&mut self) { /* TODO */ }
}

/// 制作人员叠加 Widget（对应 C++ CreditsOverlay）
pub struct CreditsOverlay {
    pub parent: Option<*mut CreditScreen>,
}

impl CreditsOverlay {
    pub fn new() -> Self { CreditsOverlay { parent: None } }
    pub fn draw(&self, _g: &mut Graphics) { /* TODO */ }
}

// --- 自由函数 ---
pub fn draw_disco(_g: &mut Graphics, _center_x: f32, _center_y: f32, _time: f32) { /* TODO */ }
pub fn draw_reanim_to_preload(_g: &mut Graphics, _reanim_type: ReanimationType) { /* TODO */ }

/// 绘制制作人员名单内容（对应 C++ DrawCreditsContent）
pub fn draw_credits_content(g: &mut Graphics, y_pos: i32, do_draw: bool) -> i32 {
    let line_height = 20;
    let mut a_y = y_pos;
    if do_draw && a_y > -line_height && a_y < BOARD_HEIGHT + line_height {
        g.set_color(&Color::WHITE);
        g.draw_string("[CREDITS_GAMENAME]", BOARD_WIDTH / 2, a_y);
    }
    a_y += line_height + 20;
    a_y
}

/// 绘制预加载画面（对应 C++ DrawToPreload）
pub fn draw_to_preload(g: &mut Graphics) {
    g.set_color(&Color::BLACK);
    g.fill_rect_xywh(0, 0, BOARD_WIDTH, BOARD_HEIGHT);
    g.set_color(&Color::WHITE);
    g.draw_string("Loading...", BOARD_WIDTH / 2 - 30, BOARD_HEIGHT / 2);
}
