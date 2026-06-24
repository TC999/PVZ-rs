// SDL 音乐接口 — 完整实现
// 对应 C++ SDLMusicInterface.h / SDLMusicInterface.cpp

use std::collections::HashMap;
use std::cell::RefCell;
use crate::framework::sound::music_interface::MusicInterface;
use crate::framework::paklib::with_pak_interface;
use crate::ffi::sdl_mixer;

struct SDLMusicInfo {
    filename: String,
    h_music: *mut std::ffi::c_void,
    volume: f64,
    volume_cap: f64,
}

pub struct SDLMusicInterface {
    music_map: RefCell<HashMap<i32, SDLMusicInfo>>,
    global_volume: f64,
}

impl SDLMusicInterface {
    pub fn new() -> Self {
        SDLMusicInterface {
            music_map: RefCell::new(HashMap::new()),
            global_volume: 1.0,
        }
    }

    fn extract_to_temp(&self, filename: &str) -> Option<String> {
        let extensions = ["", ".ogg", ".OGG", ".mp3", ".MP3", ".wav", ".WAV", ".mid", ".MID", ".flac", ".FLAC"];
        for ext in &extensions {
            let path = format!("{}{}", filename, ext);
            if let Some(data) = with_pak_interface(|pak| pak.load_file(&path)) {
                return self.write_temp_file(&path, &data);
            }
        }
        None
    }

    fn write_temp_file(&self, name: &str, data: &[u8]) -> Option<String> {
        let dir = std::env::temp_dir();
        let file_name = std::path::Path::new(name)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| format!("music_{}", self.music_map.borrow().len()));
        let dest_path = dir.join(&file_name);
        std::fs::write(&dest_path, data).ok().map(|_| dest_path.to_string_lossy().to_string())
    }

    fn set_music_volume_internal(&self, vol: f64) {
        sdl_mixer::mix_volume_music((vol * 128.0).clamp(0.0, 128.0) as i32);
    }
}

impl MusicInterface for SDLMusicInterface {
    fn load_music(&mut self, song_id: i32, filename: &str) -> bool {
        self.unload_music(song_id);
        if !sdl_mixer::is_mixer_available() {
            self.music_map.borrow_mut().insert(song_id, SDLMusicInfo {
                filename: filename.to_string(),
                h_music: std::ptr::null_mut(),
                volume: 1.0,
                volume_cap: 1.0,
            });
            return true;
        }
        let temp_path = match self.extract_to_temp(filename) { Some(p) => p, None => return false };
        let c_path = std::ffi::CString::new(temp_path.as_str()).unwrap();
        let h_music = sdl_mixer::mix_load_mus(c_path.as_ptr());
        if h_music.is_null() { return false; }
        self.music_map.borrow_mut().insert(song_id, SDLMusicInfo {
            filename: filename.to_string(), h_music, volume: 1.0, volume_cap: 1.0,
        });
        true
    }

    fn play_music(&self, song_id: i32, _offset: i32, no_loop: bool) {
        let map = self.music_map.borrow();
        if let Some(info) = map.get(&song_id) {
            if info.h_music.is_null() { return; }
            sdl_mixer::mix_play_music(info.h_music, if no_loop { 0 } else { -1 });
            self.set_music_volume_internal(self.global_volume * info.volume);
        }
    }

    fn stop_music(&self, _song_id: i32) { sdl_mixer::mix_halt_music(); }
    fn pause_music(&self, _song_id: i32) { sdl_mixer::mix_pause_music(); }
    fn resume_music(&self, _song_id: i32) { sdl_mixer::mix_resume_music(); }
    fn stop_all_music(&self) { sdl_mixer::mix_halt_music(); }

    fn unload_music(&mut self, song_id: i32) {
        if let Some(info) = self.music_map.borrow_mut().remove(&song_id) {
            if !info.h_music.is_null() { sdl_mixer::mix_free_music(info.h_music); }
        }
    }

    fn unload_all_music(&mut self) {
        for (_, info) in self.music_map.borrow_mut().drain() {
            if !info.h_music.is_null() { sdl_mixer::mix_free_music(info.h_music); }
        }
    }

    fn pause_all_music(&self) { sdl_mixer::mix_pause_music(); }
    fn resume_all_music(&self) { sdl_mixer::mix_resume_music(); }

    fn fade_in(&self, song_id: i32, _offset: i32, _speed: f64, no_loop: bool) {
        self.play_music(song_id, 0, no_loop);
    }

    fn fade_out(&self, _song_id: i32, _stop_song: bool, _speed: f64) {
        sdl_mixer::mix_halt_music();
    }

    fn fade_out_all(&self, _stop_song: bool, _speed: f64) {
        sdl_mixer::mix_halt_music();
    }

    fn set_song_volume(&self, song_id: i32, volume: f64) {
        if let Some(info) = self.music_map.borrow().get(&song_id) {
            self.set_music_volume_internal(self.global_volume * volume * info.volume_cap);
        }
        if let Some(info) = self.music_map.borrow_mut().get_mut(&song_id) {
            info.volume = volume;
        }
    }

    fn set_song_max_volume(&self, song_id: i32, max_volume: f64) {
        if let Some(info) = self.music_map.borrow_mut().get_mut(&song_id) {
            info.volume_cap = max_volume;
        }
    }

    fn is_playing(&self, _song_id: i32) -> bool {
        sdl_mixer::mix_playing(-1) > 0
    }

    fn set_volume(&mut self, volume: f64) {
        self.global_volume = volume;
        self.set_music_volume_internal(volume);
    }

    fn update(&mut self) {}
}

impl Default for SDLMusicInterface {
    fn default() -> Self { SDLMusicInterface::new() }
}

impl Drop for SDLMusicInterface {
    fn drop(&mut self) {
        sdl_mixer::mix_halt_music();
        for (_, info) in self.music_map.borrow_mut().drain() {
            if !info.h_music.is_null() { sdl_mixer::mix_free_music(info.h_music); }
        }
    }
}
