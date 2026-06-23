// SDL 声音实例

pub struct SDLSoundInstance {
    pub sound_id: u32,
    pub playing: bool,
    pub volume: f64,
    pub pan: i32,
}

impl SDLSoundInstance {
    pub fn new() -> Self {
        SDLSoundInstance {
            sound_id: 0,
            playing: false,
            volume: 1.0,
            pan: 0,
        }
    }

    pub fn play(&mut self) { self.playing = true; }
    pub fn stop(&mut self) { self.playing = false; }
    pub fn is_playing(&self) -> bool { self.playing }
    pub fn set_volume(&mut self, volume: f64) { self.volume = volume; }
    pub fn set_pan(&mut self, pan: i32) { self.pan = pan; }
}

impl Default for SDLSoundInstance {
    fn default() -> Self {
        SDLSoundInstance::new()
    }
}
