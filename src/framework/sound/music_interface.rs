// 音乐接口 trait
pub trait MusicInterface {
    /// 对应 C++ `MusicInterface.h:39 LoadMusic(int, const std::string&)`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn load_music(&mut self, song_id: i32, filename: &str) -> bool;
    fn play_music(&self, song_id: i32, offset: i32, no_loop: bool);
    fn stop_music(&self, song_id: i32);
    fn pause_music(&self, song_id: i32);
    fn resume_music(&self, song_id: i32);
    /// 对应 C++ `MusicInterface.h:44 StopAllMusic()`
    // NOT-WIRED: C++ 调用点 SexyAppBase.cpp:1619（Shutdown）；Rust SexyAppBase::shutdown 未接线（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn stop_all_music(&self);
    /// 对应 C++ `MusicInterface.h:46 UnloadMusic(int)`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn unload_music(&mut self, song_id: i32);
    /// 对应 C++ `MusicInterface.h:47 UnloadAllMusic()`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn unload_all_music(&mut self);
    /// 对应 C++ `MusicInterface.h:48 PauseAllMusic()`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn pause_all_music(&self);
    /// 对应 C++ `MusicInterface.h:49 ResumeAllMusic()`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn resume_all_music(&self);
    /// 对应 C++ `MusicInterface.h:51 FadeIn(int, int, double, bool)`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn fade_in(&self, song_id: i32, offset: i32, speed: f64, no_loop: bool);
    /// 对应 C++ `MusicInterface.h:52 FadeOut(int, bool, double)`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn fade_out(&self, song_id: i32, stop_song: bool, speed: f64);
    /// 对应 C++ `MusicInterface.h:53 FadeOutAll(bool, double)`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn fade_out_all(&self, stop_song: bool, speed: f64);
    fn set_song_volume(&self, song_id: i32, volume: f64);
    /// 对应 C++ `MusicInterface.h:55 SetSongMaxVolume(int, double)`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn set_song_max_volume(&self, song_id: i32, max_volume: f64);
    /// 对应 C++ `MusicInterface.h:56 IsPlaying(int)`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn is_playing(&self, song_id: i32) -> bool;
    /// 对应 C++ `MusicInterface.h:58 SetVolume(double)`
    // NOT-WIRED: C++ 调用点 SexyAppBase.cpp:4126 SetMusicVolume；Rust set_music_volume 仅存字段、未接线（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn set_volume(&mut self, volume: f64);
    /// 对应 C++ `MusicInterface.h:60 Update()`
    // NOT-WIRED: C++ 调用点 SexyAppBase.cpp:1676 UpdateFrames；Rust update_frames 空实现未接线（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn update(&mut self);

    /// 取音乐流句柄（对应 C++ SDLMusicInterface::mMusicMap 的 mHMusic；非 SDL 后端返回 null）
    fn get_music_handle(&self, _song_id: i32) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }
    /// 查询 MOD 当前播放 order（对应 C++ SDLMusicInterface::GetMusicOrder；非 SDL 后端返回 -1）
    fn get_music_order(&self, _song_id: i32) -> i32 {
        -1
    }
    /// 按句柄设置 MOD 逐轨音量（对应 C++ Mix_ModMusicStreamSetChannelVolume；非 SDL 后端空操作）
    fn set_music_channel_volume(&self, _music: *mut std::ffi::c_void, _channel: i32, _volume: i32) {
    }
}
