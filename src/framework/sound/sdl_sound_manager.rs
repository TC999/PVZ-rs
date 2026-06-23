// SDL 声音管理器 - 包装 SDL_Mixer-X C API

use crate::framework::sound::sound_manager::SoundManager;

pub struct SDLSoundManager {
    volume: f64,
    initialized: bool,
}

impl SDLSoundManager {
    pub fn new() -> Self {
        SDLSoundManager {
            volume: 1.0,
            initialized: false,
        }
    }
}

impl SoundManager for SDLSoundManager {
    fn init(&mut self) -> bool {
        self.initialized = true;
        true
    }

    fn load_sound(&mut self, _sound_id: u32, _file_path: &str) -> bool {
        true
    }

    fn play_sound(&self, _sound_id: u32) -> bool {
        true
    }

    fn stop_sound(&self, _sound_id: u32) {}

    fn set_volume(&mut self, volume: f64) {
        self.volume = volume;
    }

    fn get_volume(&self) -> f64 {
        self.volume
    }
}

impl Default for SDLSoundManager {
    fn default() -> Self {
        SDLSoundManager::new()
    }
}
