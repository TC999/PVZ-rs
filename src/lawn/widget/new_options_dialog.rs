// PvZ Portable Rust 翻译 — NewOptionsDialog
// 对应 C++ src/Lawn/Widget/NewOptionsDialog.h / NewOptionsDialog.cpp

#![allow(dead_code)]
use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::dialog::{BUTTONS_YES_NO, ID_YES};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::*;

/// 按钮 ID（对应 C++ NewOptionsDialog.h NewOptionsDialog_ 枚举）
const NEWOPTIONS_ALMANAC: i32 = 0;
const NEWOPTIONS_MAIN_MENU: i32 = 1;
const NEWOPTIONS_RESTART: i32 = 2;
const NEWOPTIONS_UPDATE: i32 = 3;
/// 对应 C++ ConstEnums.h Dialogs::DIALOG_CONFIRM_RESTART
const DIALOG_CONFIRM_RESTART: i32 = 23;

pub struct NewOptionsDialog {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub visible: bool,
    /// 对应 C++ mApp（用于读取/设置音量等）
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    /// 对应 C++ mMusicVolumeSlider / mSfxVolumeSlider（简化：记录值 + 拖动标记）
    pub music_volume: f64,
    pub sfx_volume: f64,
    pub sfx_slider_dragging: bool,
    /// 对应 C++ mFullscreenCheckbox / mHardwareAccelerationCheckbox（简化布尔态）
    pub fullscreen_checked: bool,
    pub hardware_acceleration_checked: bool,
    /// 对应 C++ mFromGameSelector
    pub from_game_selector: bool,
}

impl NewOptionsDialog {
    pub fn new() -> Self {
        NewOptionsDialog {
            x: 0, y: 0, width: 0, height: 0, visible: true,
            app: None,
            music_volume: 0.85,
            sfx_volume: 1.0,
            sfx_slider_dragging: false,
            fullscreen_checked: true,
            hardware_acceleration_checked: false,
            from_game_selector: false,
        }
    }
    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) { self.x = x; self.y = y; self.width = w; self.height = h; }
    pub fn draw(&self, _g: &mut Graphics) {
        // [TRANSLATION_NOTE]: C++ 中绘制 IMAGE_OPTIONS_MENUBACK 背景与 Music/Sound FX/
        // 3D Acceleration/Full Screen 文字标签；Rust 侧图片资源未接入，暂略
    }
    pub fn update(&mut self) {}
    pub fn key_down(&mut self, key: i32) {
        // 对应 C++ KeyDown
        if let Some(app) = self.app {
            unsafe {
                if let Some(board) = (*app).board {
                    (*board).do_typing_check(key);
                }
            }
        }
        // C++ 中空格/回车 → ID_OK、ESC → ID_CANCEL（Dialogs 按钮处理）
        let _ = key;
    }
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32) {}

    /// 音量滑块变化（对应 C++ SliderVal）
    pub fn slider_val(&mut self, the_id: i32, the_val: f64) {
        let Some(app) = self.app else { return };
        unsafe {
            match the_id {
                4 => { // NewOptionsDialog_MusicVolume
                    self.music_volume = the_val;
                    (*app).base.set_music_volume(the_val);
                    if let Some(ss) = (*app).sound_system.as_ref() {
                        ss.rehookup_sound_with_music_volume();
                    }
                }
                5 => { // NewOptionsDialog_SoundVolume
                    self.sfx_volume = the_val * 0.65;
                    (*app).base.set_sfx_volume(the_val * 0.65);
                    if let Some(ss) = (*app).sound_system.as_ref() {
                        ss.rehookup_sound_with_music_volume();
                    }
                    if !self.sfx_slider_dragging {
                        // [TRANSLATION_NOTE]: C++ 中 PlaySample(SOUND_BUTTONCLICK)
                    }
                }
                _ => {}
            }
        }
    }

    /// 复选框变化（对应 C++ CheckboxChecked）
    pub fn checkbox_checked(&mut self, the_id: i32, checked: bool) {
        let Some(app) = self.app else { return };
        unsafe {
            match the_id {
                6 => { // NewOptionsDialog_Fullscreen
                    if !checked {
                        // [TRANSLATION_NOTE]: C++ 中 !checked && mApp->mForceFullscreen 时
                        // DoDialog(DIALOG_COLORDEPTH_EXP) 并还原复选框；Rust 侧 mForceFullscreen 缺
                        self.fullscreen_checked = true;
                    } else {
                        self.fullscreen_checked = true;
                    }
                }
                7 => { // NewOptionsDialog_HardwareAcceleration
                    if checked {
                        // [TRANSLATION_NOTE]: C++ 中 Is3DAccelerationSupported/Recommended 判定
                        // 并弹出 DIALOG_INFO 警告；Rust 侧无 3D 加速检测，直接启用
                        self.hardware_acceleration_checked = true;
                    } else {
                        self.hardware_acceleration_checked = false;
                    }
                }
                _ => {}
            }
        }
    }

    /// 对应 C++ NewOptionsDialog::AddedToManager
    pub fn added_to_manager(&mut self, _the_widget_manager: *mut WidgetManager) {
        // C++: Dialog::AddedToManager + AddWidget(mAlmanacButton/mRestartButton/mBackToMainButton/
        //      mMusicVolumeSlider/mSfxVolumeSlider/mHardwareAccelerationCheckbox/mFullscreenCheckbox/mBackToGameButton)
        // [TRANSLATION_NOTE]: Rust 侧这些子控件未创建（以简化布尔/数值态替代），注册链保留调用点
    }

    /// 对应 C++ NewOptionsDialog::RemovedFromManager
    pub fn removed_from_manager(&mut self, _the_widget_manager: *mut WidgetManager) {
        // C++: Dialog::RemovedFromManager + RemoveWidget（同上 8 个子控件）
        // [TRANSLATION_NOTE]: 同上，子控件未创建
    }

    /// 对应 C++ NewOptionsDialog::ButtonPress
    pub fn button_press(&mut self, _the_id: i32) {
        // C++: mApp->PlaySample(SOUND_GRAVEBUTTON)
        if let Some(app) = self.app {
            unsafe {
                (*app).play_sample(crate::framework::resources::ResourceId::SoundGravebutton as i32);
            }
        }
    }

    /// 对应 C++ NewOptionsDialog::GetPreferredHeight
    pub fn get_preferred_height(&self) -> i32 {
        // C++: return IMAGE_OPTIONS_MENUBACK->mWidth；图片资源未接入，返回 0
        0
    }

    /// 对应 C++ NewOptionsDialog::ButtonDepress
    pub fn button_depress(&mut self, the_id: i32) {
        // C++: Dialog::ButtonDepress(theId) —— 基类处理，Rust 无对应
        match the_id {
            NEWOPTIONS_ALMANAC => {
                let Some(app) = self.app else { return };
                unsafe {
                    // C++: DoAlmanacDialog(SEED_NONE, ZOMBIE_INVALID)->WaitForResult(true)；Rust 版返回 ()
                    (*app).do_almanac_dialog(SeedType::None, ZombieType::Invalid);
                }
            }
            NEWOPTIONS_MAIN_MENU => {
                let Some(app) = self.app else { return };
                unsafe {
                    if self.from_game_selector {
                        (*app).kill_new_options_dialog();
                        (*app).kill_game_selector();
                        (*app).show_award_screen(AwardType::CreditsZombieNote as i32, false);
                    } else if (*app).board.map_or(false, |b| unsafe { (*b).need_save_game() }) {
                        // C++: mApp->DoConfirmBackToMain() —— Rust 侧未实现，保留调用点
                    } else {
                        // C++: else if (mApp->mBoard && mApp->mBoard->mCutScene && mCutScene->IsSurvivalRepick())
                        //       DoConfirmBackToMain() —— Rust 侧 mCutScene 未接入，同上省略
                        if let Some(board) = (*app).board {
                            (*board).m_board_result = BoardResult::Quit;
                        }
                        (*app).do_back_to_main();
                    }
                }
            }
            NEWOPTIONS_RESTART => {
                let Some(app) = self.app else { return };
                unsafe {
                    if (*app).board.is_some() {
                        let a_dialog_title;
                        let a_dialog_message;
                        if (*app).is_puzzle_mode() {
                            a_dialog_title = "[RESTART_PUZZLE_HEADER]";
                            a_dialog_message = "[RESTART_PUZZLE_BODY]";
                        } else if (*app).is_challenge_mode() {
                            a_dialog_title = "[RESTART_CHALLENGE_HEADER]";
                            a_dialog_message = "[RESTART_CHALLENGE_BODY]";
                        } else if (*app).is_survival_mode() {
                            a_dialog_title = "[RESTART_SURVIVAL_HEADER]";
                            a_dialog_message = "[RESTART_SURVIVAL_BODY]";
                        } else {
                            a_dialog_title = "[RESTART_LEVEL_HEADER]";
                            a_dialog_message = "[RESTART_LEVEL_BODY]";
                        }

                        // C++: (LawnDialog*)DoDialog(DIALOG_CONFIRM_RESTART, ...) 并设置 Yes/No 标签；
                        // Rust do_dialog 返回 *mut Dialog，按钮标签跳过（同 StoreScreen PurchaseItem 注释）
                        let a_dialog = (*app).do_dialog(
                            DIALOG_CONFIRM_RESTART,
                            true,
                            a_dialog_title,
                            a_dialog_message,
                            "",
                            BUTTONS_YES_NO,
                        );
                        let a_result = a_dialog.map_or(0, |d| unsafe { (&mut *d).wait_for_result(true) });
                        if a_result == ID_YES {
                            if let Some(music) = (*app).music.as_mut() {
                                music.stop_all_music();
                            }
                            if let Some(ss) = (*app).sound_system.as_ref() {
                                ss.cancel_paused_foley();
                            }
                            (*app).kill_new_options_dialog();
                            if let Some(board) = (*app).board {
                                (&mut *board).m_board_result = BoardResult::Restart;
                                (*app).m_saw_yeti = (*board).m_killed_yeti;
                            }
                            (*app).pre_new_game((*app).game_mode, false);
                        }
                    }
                }
            }
            NEWOPTIONS_UPDATE => {
                // C++: mApp->CheckForUpdates() —— Rust 侧未实现，保留调用点
            }
            _ => {}
        }
    }
}

