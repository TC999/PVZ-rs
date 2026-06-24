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
}
