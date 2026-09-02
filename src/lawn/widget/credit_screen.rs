// PvZ Portable Rust 翻译 — CreditScreen（制作人员列表）
// 对应 C++ src/Lawn/Widget/CreditScreen.h / CreditScreen.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::lawn::system::music::MusicTune;
use crate::todlib::tod_foley::FoleyType;
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

    pub fn update(&mut self) {
        // 对应 C++ Update：片尾阶段推进与 reanim 同步
        if !self.credits_paused {
            let menu_over = self.main_menu_button.map_or(false, |p| unsafe { (*p).is_over });
            let replay_over = self.replay_button.map_or(false, |p| unsafe { (*p).is_over });
            if !menu_over && !replay_over {
                // [TRANSLATION_NOTE]: C++ 中 SetCursor(CURSOR_POINTER)
            }
        }
        // [TRANSLATION_NOTE]: C++ 中 !IsInDemoMode() && mDrawCount == 0 时暂停；Rust 侧无 demo 模式
        if self.credits_paused {
            return;
        }

        self.update_count += 1;
        if self.update_count == 1 {
            // C++ 中 PreLoadCredits() + PlayReanim(1) + 播放片尾音乐
            let _ = self.play_reanim(1);
            if let Some(app) = self.app {
                unsafe {
                    if let Some(music) = (*app).music.as_mut() {
                        music.make_sure_music_is_playing(MusicTune::CreditsZombiesOnYourLawn);
                    }
                }
            }
        } else if self.dont_sync || self.credits_phase == CreditsPhase::End {
            self.update_movie();
        } else if self.update_count > 1 {
            // [TRANSLATION_NOTE]: C++ 中按 reanim 定义时长与计时器差值调用
            // JumpToFrame(phase+1, 0) 推进阶段或补帧 UpdateMovie()；Rust 侧
            // reanim 轨道计数/计时器未接入，简化直接推进阶段
            if self.credits_phase == CreditsPhase::Main1 {
                self.jump_to_frame(CreditsPhase::Main2, 0.0);
            } else if self.credits_phase == CreditsPhase::Main2 {
                self.jump_to_frame(CreditsPhase::Main3, 0.0);
            } else if self.credits_phase == CreditsPhase::Main3 {
                self.jump_to_frame(CreditsPhase::End, 0.0);
            }
        }

        self.last_draw_count = self.draw_count;
    }
    pub fn draw(&self, _g: &mut Graphics) { /* TODO: CreditScreen.cpp */ }
    pub fn key_char(&mut self, c: char) {
        // 对应 C++ KeyChar：调试键跳帧
        if self.credits_paused {
            return;
        }
        let debug_enabled = self.app.map_or(false, |app| unsafe { (*app).m_debug_keys_enabled });
        if !debug_enabled {
            return;
        }
        match c {
            '1' => self.jump_to_frame(CreditsPhase::Main1, 0.0),
            '2' => self.jump_to_frame(CreditsPhase::Main1, 128.0),
            '3' => self.jump_to_frame(CreditsPhase::Main1, 144.0),
            '4' => self.jump_to_frame(CreditsPhase::Main1, 272.0),
            '5' => self.jump_to_frame(CreditsPhase::Main1, 304.0),
            '6' => self.jump_to_frame(CreditsPhase::Main1, 340.0),
            '7' => self.jump_to_frame(CreditsPhase::Main1, 368.0),
            'q' => self.jump_to_frame(CreditsPhase::Main2, 0.0),
            'w' => self.jump_to_frame(CreditsPhase::Main2, 124.0),
            'e' => self.jump_to_frame(CreditsPhase::Main2, 188.0),
            'r' => self.jump_to_frame(CreditsPhase::Main2, 248.0),
            't' => self.jump_to_frame(CreditsPhase::Main2, 320.0),
            'a' => self.jump_to_frame(CreditsPhase::Main3, 0.0),
            's' => self.jump_to_frame(CreditsPhase::Main3, 124.0),
            'd' => self.jump_to_frame(CreditsPhase::Main3, 216.0),
            'f' => self.jump_to_frame(CreditsPhase::Main3, 240.0),
            'g' => self.jump_to_frame(CreditsPhase::Main3, 324.0),
            'n' => { self.dont_sync = !self.dont_sync; }
            _ => {}
        }
    }
    pub fn key_down(&mut self, key: KeyCode) {
        // 对应 C++ KeyDown：空格/回车/ESC 暂停片尾
        if key == crate::framework::key_codes::KEYCODE_SPACE
            || key == crate::framework::key_codes::KEYCODE_RETURN
            || key == crate::framework::key_codes::KEYCODE_ESCAPE
        {
            self.pause_credits();
        }
    }
    pub fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // C++ 中为空实现
    }
    pub fn button_press(&mut self, _id: i32) {
        // [TRANSLATION_NOTE]: C++ 中 PlaySample(SOUND_GRAVEBUTTON)
    }
    pub fn button_depress(&mut self, the_id: i32) {
        // 对应 C++ ButtonDepress
        const CREDITS_BUTTON_REPLAY: i32 = 0;
        const CREDITS_BUTTON_MAIN_MENU: i32 = 1;
        let Some(app) = self.app else { return };
        unsafe {
            if the_id == CREDITS_BUTTON_MAIN_MENU {
                (*app).kill_credit_screen();
                (*app).do_back_to_main();
            } else if the_id == CREDITS_BUTTON_REPLAY {
                (*app).kill_credit_screen();
                (*app).show_credit_screen();
            }
        }
    }
    pub fn play_reanim(&self, _index: i32) -> Option<*mut Reanimation> { None /* TODO */ }
    pub fn jump_to_frame(&self, _phase: CreditsPhase, _frame: f32) { /* TODO: CreditScreen.cpp */ }
    pub fn draw_fog_effect(&self, _g: &mut Graphics, _time: f32) { /* TODO */ }
    pub fn update_blink(&mut self) { /* TODO */ }
    pub fn draw_final_credits(&self, _g: &mut Graphics) { /* TODO */ }
    pub fn draw_overlay(&self, _g: &mut Graphics) { /* TODO */ }
    pub fn update_movie(&mut self) { /* TODO */ }
    pub fn pause_credits(&mut self) {
        // 对应 C++ PauseCredits：停止音效/音乐并弹出暂停菜单
        if self.credits_paused {
            return;
        }
        if let Some(app) = self.app {
            unsafe {
                if let Some(ss) = (*app).sound_system.as_ref() {
                    ss.stop_foley(FoleyType::Scream);
                }
                // [TRANSLATION_NOTE]: C++ 中 PlaySample(SOUND_PAUSE)
                if let Some(music) = (*app).music.as_mut() {
                    music.game_music_pause(true);
                }
            }
        }
        self.credits_paused = true;
        // [TRANSLATION_NOTE]: C++ 中 LawnMessageBox(DIALOG_MESSAGE, ...) 暂停菜单与恢复流程
        // 未接入，此处以 do_dialog 近似提示
        if let Some(app) = self.app {
            unsafe {
                let _ = (*app).do_dialog(
                    crate::lawn::game_enums::Dialogs::Message as i32,
                    true,
                    "[CREDITS_PAUSE_HEADER]",
                    "[CREDITS_PAUSE_BODY]",
                    "[DIALOG_BUTTON_RESUME]",
                    crate::framework::widget::dialog::BUTTONS_FOOTER,
                );
            }
        }
    }
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
