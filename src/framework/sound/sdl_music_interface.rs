// SDL 音乐接口

use crate::framework::sound::music_interface::MusicInterface;

pub struct SDLMusicInterface {
    volume: f64,
}

impl SDLMusicInterface {
    pub fn new() -> Self {
        SDLMusicInterface { volume: 1.0 }
    }
}

impl MusicInterface for SDLMusicInterface {
    fn init(&mut self) -> bool { true }
    fn load_music(&mut self, _music_id: u32, _file_path: &str) -> bool { true }
    fn play_music(&self, _music_id: u32) -> bool { true }
    fn stop_music(&self) {}
    fn set_volume(&mut self, volume: f64) { self.volume = volume; }
}

impl Default for SDLMusicInterface {
    fn default() -> Self {
        SDLMusicInterface::new()
    }
}
