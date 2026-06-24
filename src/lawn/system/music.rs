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
        eprintln!("[Music] music_title_screen_init 被调用");
        let app = unsafe { &*self.app };
        if let Some(mi_ptr) = app.base.music_interface {
            unsafe {
                let mi = &mut *mi_ptr;
                // 尝试加载主菜单音乐
                let paths = [
                    "sounds/mainmusic.mo3",
                    "sounds/mainmusic.ogg",
                    "sounds/main_music.ogg",
                ];
                for path in &paths {
                    eprintln!("[Music] 尝试加载: {}", path);
                    if mi.load_music(1, path) {
                        eprintln!("[Music] 加载成功: {}", path);
                        mi.play_music(1, 0, false);
                        eprintln!("[Music] 开始播放");
                        return;
                    }
                }
                eprintln!("[Music] 所有音乐文件均未找到");
            }
        } else {
            eprintln!("[Music] music_interface 为 None");
        }
    }
}

impl Default for Music {
    fn default() -> Self {
        panic!("Music::default called without app pointer");
    }
}
