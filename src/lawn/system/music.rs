// PvZ Portable Rust 翻译 — Music（游戏音乐管理器）
// 对应 C++ src/Lawn/System/Music.h / Music.cpp

#![allow(dead_code)]

use crate::lawn::lawn_app::LawnApp;
use crate::framework::sound::music_interface::MusicInterface;

pub struct Music {
    pub app: *mut LawnApp,
}

impl Music {
    pub fn new(app: *mut LawnApp) -> Self {
        Music { app }
    }

    /// 初始化标题屏幕音乐（对应 C++ MusicTitleScreenInit）
    pub fn music_title_screen_init(&mut self) {
        let app = unsafe { &*self.app };
        if let Some(mi_ptr) = app.base.music_interface {
            unsafe {
                let mi = &mut *mi_ptr;
                // 尝试加载主菜单音乐
                mi.load_music(0, "sounds/mainmusic.mo3");
                mi.load_music(0, "sounds/mainmusic.ogg");
                mi.play_music(0, 0, false);
            }
        }
    }
}

impl Default for Music {
    fn default() -> Self {
        panic!("Music::default called without app pointer");
    }
}
