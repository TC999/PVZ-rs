// PvZ Portable Rust 翻译 — TodFoley（音效系统）
// 对应 C++ src/Sexy.TodLib/TodFoley.h / TodFoley.cpp

#![allow(dead_code)]

/// Foley 音效类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FoleyType {
    None = -1,
    // 这些值从资源定义中获取
    // 常见值：Peashooter, Cherrybomb, ZombieEating, etc.
}

/// 音效管理器
pub struct FoleyManager {
    pub audio_enabled: bool,
}

impl FoleyManager {
    pub fn new() -> Self {
        FoleyManager {
            audio_enabled: true,
        }
    }

    /// 播放音效
    pub fn play_foley(&self, _foley_type: FoleyType) {
        // 委托给 SoundManager
    }

    /// 停止音效
    pub fn stop_foley(&self, _foley_type: FoleyType) {
        // 停止特定音效
    }

    /// 设置是否静音
    pub fn set_audio_enabled(&mut self, enabled: bool) {
        self.audio_enabled = enabled;
    }
}

impl Default for FoleyManager {
    fn default() -> Self {
        FoleyManager::new()
    }
}
