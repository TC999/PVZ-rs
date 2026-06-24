// 游戏音乐管理器 — 对应 C++ src/Lawn/System/Music.cpp
// 使用 MusicFile 枚举映射到具体文件路径

#![allow(dead_code)]

use crate::lawn::lawn_app::LawnApp;
use crate::framework::sound::music_interface::MusicInterface;

/// C++ MusicFile 枚举中的关键值
const MUSIC_FILE_MAIN_MUSIC: i32 = 0;

pub struct Music {
    pub app: *mut LawnApp,
}

impl Music {
    pub fn new(app: *mut LawnApp) -> Self {
        Music { app }
    }

    /// 对应 C++ Music::LoadSong
    pub fn load_song(&mut self, song_id: i32, filename: &str) {
        let app = unsafe { &mut *self.app };
        if let Some(mi_ptr) = app.base.music_interface {
            unsafe {
                // C++: mApp->mMusicInterface->LoadMusic(theSongId, theFileName)
                if !(*mi_ptr).load_music(song_id, filename) {
                    eprintln!("[Music] 加载失败: song={} file='{}'", song_id, filename);
                }
            }
        }
    }

    /// 对应 C++ Music::MusicTitleScreenInit
    pub fn music_title_screen_init(&mut self) {
        // C++: LoadSong(MUSIC_FILE_MAIN_MUSIC, "sounds/mainmusic.mo3");
        self.load_song(MUSIC_FILE_MAIN_MUSIC, "sounds/mainmusic.mo3");
        // C++: MakeSureMusicIsPlaying(MUSIC_TUNE_TITLE_CRAZY_DAVE_MAIN_THEME);
        self.make_sure_music_playing(0);
    }

    /// 对应 C++ Music::MakeSureMusicIsPlaying（简化版）
    fn make_sure_music_playing(&mut self, _tune: i32) {
        let app = unsafe { &*self.app };
        if let Some(mi_ptr) = app.base.music_interface {
            unsafe {
                // 如果没在播放则开始
                (*mi_ptr).play_music(MUSIC_FILE_MAIN_MUSIC, 0, false);
            }
        }
    }
}

impl Default for Music {
    fn default() -> Self {
        panic!("Music::default 无法在没有 app 指针时创建");
    }
}
