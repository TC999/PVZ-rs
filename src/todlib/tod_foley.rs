// PvZ Portable Rust 翻译 — TodFoley（音效系统）
// 对应 C++ src/Sexy.TodLib/TodFoley.h / TodFoley.cpp

#![allow(dead_code)]

use crate::framework::sound::sound_manager::SoundManager;

/// Foley 音效管理器
pub struct FoleyManager {
    pub audio_enabled: bool,
    pub app: *mut crate::lawn::lawn_app::LawnApp,
}

impl FoleyManager {
    pub fn new(app: *mut crate::lawn::lawn_app::LawnApp) -> Self {
        FoleyManager {
            audio_enabled: true,
            app,
        }
    }

    /// 播放音效
    pub fn play_foley(&self, foley_type: i32) {
        if !self.audio_enabled { return; }
        let app = unsafe { &*self.app };
        if let Some(sm_ptr) = app.base.sound_manager {
            unsafe {
                // 按 FoleyType 作为音效 ID 播放
                // C++ 版 FoleyType 值和 SoundManager 的 ID 是直接对应的
                (*sm_ptr).play_sound(foley_type);
            }
        }
    }

    /// 停止音效
    pub fn stop_foley(&self, foley_type: i32) {
        let app = unsafe { &*self.app };
        if let Some(sm_ptr) = app.base.sound_manager {
            unsafe { (*sm_ptr).stop_sound(foley_type); }
        }
    }

    /// 设置是否启用音频
    pub fn set_audio_enabled(&mut self, enabled: bool) {
        self.audio_enabled = enabled;
    }
}

impl Default for FoleyManager {
    fn default() -> Self {
        panic!("FoleyManager::default called without app pointer");
    }
}
