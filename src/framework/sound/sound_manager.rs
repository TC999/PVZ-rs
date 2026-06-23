// 声音管理器 trait
pub trait SoundManager {
    fn init(&mut self) -> bool;
    fn load_sound(&mut self, sound_id: u32, file_path: &str) -> bool;
    fn play_sound(&self, sound_id: u32) -> bool;
    fn stop_sound(&self, sound_id: u32);
    fn set_volume(&mut self, volume: f64);
    fn get_volume(&self) -> f64;
}
