// 音乐接口 trait
pub trait MusicInterface {
    fn load_music(&mut self, song_id: i32, filename: &str) -> bool;
    fn play_music(&self, song_id: i32, offset: i32, no_loop: bool);
    fn stop_music(&self, song_id: i32);
    fn pause_music(&self, song_id: i32);
    fn resume_music(&self, song_id: i32);
    fn stop_all_music(&self);
    fn unload_music(&mut self, song_id: i32);
    fn unload_all_music(&mut self);
    fn pause_all_music(&self);
    fn resume_all_music(&self);
    fn fade_in(&self, song_id: i32, offset: i32, speed: f64, no_loop: bool);
    fn fade_out(&self, song_id: i32, stop_song: bool, speed: f64);
    fn fade_out_all(&self, stop_song: bool, speed: f64);
    fn set_song_volume(&self, song_id: i32, volume: f64);
    fn set_song_max_volume(&self, song_id: i32, max_volume: f64);
    fn is_playing(&self, song_id: i32) -> bool;
    fn set_volume(&mut self, volume: f64);
    fn update(&mut self);

    /// 取音乐流句柄（对应 C++ SDLMusicInterface::mMusicMap 的 mHMusic；非 SDL 后端返回 null）
    fn get_music_handle(&self, song_id: i32) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }
    /// 查询 MOD 当前播放 order（对应 C++ SDLMusicInterface::GetMusicOrder；非 SDL 后端返回 -1）
    fn get_music_order(&self, song_id: i32) -> i32 {
        -1
    }
    /// 按句柄设置 MOD 逐轨音量（对应 C++ Mix_ModMusicStreamSetChannelVolume；非 SDL 后端空操作）
    fn set_music_channel_volume(&self, _music: *mut std::ffi::c_void, _channel: i32, _volume: i32) {
    }
}
