#![allow(dead_code)]

pub struct Music {
    volume: f64,
}

impl Music {
    pub fn new() -> Self {
        Music { volume: 0.0 }
    }

    /// 初始化标题屏幕音乐（对应 C++ MusicTitleScreenInit）
    pub fn music_title_screen_init(&mut self) {
        // 暂为 stub，后续 Phase 实现
    }
}

impl Default for Music {
    fn default() -> Self {
        Self::new()
    }
}
