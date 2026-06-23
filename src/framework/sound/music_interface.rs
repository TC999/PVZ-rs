// 音乐接口 trait
pub trait MusicInterface {
    fn init(&mut self) -> bool;
    fn load_music(&mut self, music_id: u32, file_path: &str) -> bool;
    fn play_music(&self, music_id: u32) -> bool;
    fn stop_music(&self);
    fn set_volume(&mut self, volume: f64);
}
