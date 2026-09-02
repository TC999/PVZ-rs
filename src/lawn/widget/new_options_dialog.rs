// PvZ Portable Rust 翻译 — NewOptionsDialog
// 对应 C++ src/Lawn/Widget/NewOptionsDialog.h / NewOptionsDialog.cpp

#![allow(dead_code)]
use crate::framework::graphics::graphics::Graphics;

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
}

