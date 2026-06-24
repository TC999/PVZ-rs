// SDL 音乐接口
// 流程：PAK→临时文件→Mix_LoadMUS，失败时输出日志
// 如果 SDL2_mixer_ext.dll 可用，支持 OGG/MP3/WAV/FLAC/MO3（需 libopenmpt）
// 如果只有 SDL2_mixer.dll，支持 OGG/WAV

#![allow(dead_code, invalid_reference_casting)]

use std::collections::HashMap;
use crate::framework::sound::music_interface::MusicInterface;
use crate::framework::paklib::with_pak_interface;
use crate::ffi::sdl_mixer;

struct MusicInfo {
    filename: String,
    handle: *mut std::ffi::c_void,
    volume: f64,
    volume_cap: f64,
}

pub struct SDLMusicInterface {
    music_map: HashMap<i32, MusicInfo>,
    global_volume: f64,
}

impl SDLMusicInterface {
    pub fn new() -> Self {
        SDLMusicInterface {
            music_map: HashMap::new(),
            global_volume: 1.0,
        }
    }

    fn write_temp_file(&self, name: &str, data: &[u8]) -> Option<String> {
        let dir = std::env::temp_dir();
        let fname = std::path::Path::new(name)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| format!("pvz_{}", self.music_map.len()));
        let dest = dir.join(fname);
        std::fs::write(&dest, data).ok().map(|_| dest.to_string_lossy().to_string())
    }

    /// 从 PAK 加载并尝试用 Mix_LoadMUS 播放
    fn try_pak_music(&self, song_id: i32, filename: &str) -> bool {
        let exts = ["", ".mo3", ".MO3", ".ogg", ".OGG", ".mp3", ".MP3", ".wav", ".WAV"];
        for ext in &exts {
            let path = format!("{}{}", filename, ext);
            if let Some(data) = with_pak_interface(|pak| pak.load_file(&path)) {
                if let Some(temp) = self.write_temp_file(&path, &data) {
                    let c_path = std::ffi::CString::new(temp).unwrap();
                    let h = sdl_mixer::load_mus(c_path.as_ptr());
                    if !h.is_null() {
                        unsafe {
                            let sm = &mut *(self as *const Self as *mut Self);
                            sm.music_map.insert(song_id, MusicInfo {
                                filename: path, handle: h, volume: 1.0, volume_cap: 1.0,
                            });
                        }
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl MusicInterface for SDLMusicInterface {
    fn load_music(&mut self, song_id: i32, filename: &str) -> bool {
        self.unload_music(song_id);

        // 1. 直接文件路径
        if !filename.is_empty() {
            let c_path = std::ffi::CString::new(filename).unwrap();
            let h = sdl_mixer::load_mus(c_path.as_ptr());
            if !h.is_null() {
                self.music_map.insert(song_id, MusicInfo {
                    filename: filename.to_string(), handle: h, volume: 1.0, volume_cap: 1.0,
                });
                return true;
            }
        }
        if !sdl_mixer::is_available() { return false; }

        // 2. PAK → 临时文件 → Mix_LoadMUS
        if self.try_pak_music(song_id, filename) { return true; }

        eprintln!("  [SDLMusicInterface] 加载失败 '{}': 格式不受支持或文件不存在", filename);
        false
    }

    fn play_music(&self, song_id: i32, _offset: i32, no_loop: bool) {
        if let Some(info) = self.music_map.get(&song_id) {
            if info.handle.is_null() { return; }
            sdl_mixer::play_music(info.handle, if no_loop { 0 } else { -1 });
            let v = (self.global_volume * info.volume * 128.0) as i32;
            sdl_mixer::volume_music(v.clamp(0, 128));
        }
    }

    fn stop_music(&self, _song_id: i32) { sdl_mixer::halt_music(); }
    fn pause_music(&self, _song_id: i32) { sdl_mixer::pause_music(); }
    fn resume_music(&self, _song_id: i32) { sdl_mixer::resume_music(); }
    fn stop_all_music(&self) { sdl_mixer::halt_music(); }

    fn unload_music(&mut self, song_id: i32) {
        if let Some(info) = self.music_map.remove(&song_id) {
            if !info.handle.is_null() { sdl_mixer::free_music(info.handle); }
        }
    }
    fn unload_all_music(&mut self) {
        for (_, info) in self.music_map.drain() {
            if !info.handle.is_null() { sdl_mixer::free_music(info.handle); }
        }
    }
    fn pause_all_music(&self) { sdl_mixer::pause_music(); }
    fn resume_all_music(&self) { sdl_mixer::resume_music(); }

    fn fade_in(&self, song_id: i32, _offset: i32, _speed: f64, no_loop: bool) {
        self.play_music(song_id, 0, no_loop);
    }
    fn fade_out(&self, _song_id: i32, _stop_song: bool, _speed: f64) { sdl_mixer::halt_music(); }
    fn fade_out_all(&self, _stop_song: bool, _speed: f64) { sdl_mixer::halt_music(); }

    fn set_song_volume(&self, song_id: i32, volume: f64) {
        if let Some(info) = self.music_map.get(&song_id) {
            let v = (volume * info.volume_cap * 128.0) as i32;
            sdl_mixer::volume_music(v.clamp(0, 128));
        }
    }
    fn set_song_max_volume(&self, song_id: i32, max_volume: f64) {
        unsafe {
            let sm = &mut *(self as *const Self as *mut Self);
            if let Some(info) = sm.music_map.get_mut(&song_id) { info.volume_cap = max_volume; }
        }
    }
    fn is_playing(&self, _song_id: i32) -> bool { true }
    fn set_volume(&mut self, volume: f64) {
        self.global_volume = volume;
        sdl_mixer::volume_music((volume * 128.0) as i32);
    }
    fn update(&mut self) {}
}

impl Default for SDLMusicInterface { fn default() -> Self { SDLMusicInterface::new() } }

impl Drop for SDLMusicInterface {
    fn drop(&mut self) {
        sdl_mixer::halt_music();
        for (_, info) in self.music_map.drain() {
            if !info.handle.is_null() { sdl_mixer::free_music(info.handle); }
        }
    }
}
