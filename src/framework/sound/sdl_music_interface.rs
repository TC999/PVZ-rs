// SDL 音乐接口 — 完整实现
// 对应 C++ SDLMusicInterface.h / SDLMusicInterface.cpp
// 游戏为单线程，&self 转换到 &mut self 是安全的

#![allow(dead_code, invalid_reference_casting)]

use std::collections::HashMap;
use crate::framework::sound::music_interface::MusicInterface;
use crate::framework::paklib::with_pak_interface;
use crate::ffi::sdl_mixer;

struct SDLMusicInfo {
    filename: String,
    /// Mix_Music* 指针
    h_music: *mut std::ffi::c_void,
    volume: f64,
    volume_cap: f64,
}

pub struct SDLMusicInterface {
    music_map: HashMap<i32, SDLMusicInfo>,
    global_volume: f64,
}

impl SDLMusicInterface {
    pub fn new() -> Self {
        SDLMusicInterface {
            music_map: HashMap::new(),
            global_volume: 1.0,
        }
    }

    fn extract_to_temp(&self, filename: &str) -> Option<String> {
        let extensions = ["", ".ogg", ".OGG", ".mp3", ".MP3", ".wav", ".WAV", ".mid", ".MID", ".flac", ".FLAC", ".mo3", ".MO3"];
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
            .unwrap_or_else(|| format!("music_{}", self.music_map.len()));
        let dest_path = dir.join(&file_name);
        std::fs::write(&dest_path, data).ok().map(|_| dest_path.to_string_lossy().to_string())
    }
}

impl MusicInterface for SDLMusicInterface {
    fn load_music(&mut self, song_id: i32, filename: &str) -> bool {
        self.unload_music(song_id);

        eprintln!("  [SDLMusicInterface] load_music({}): 尝试加载 '{}'", song_id, filename);

        if !sdl_mixer::is_mixer_available() {
            eprintln!("  [SDLMusicInterface] SDL_mixer 不可用，仅记录文件名");
            self.music_map.insert(song_id, SDLMusicInfo {
                filename: filename.to_string(), h_music: std::ptr::null_mut(), volume: 1.0, volume_cap: 1.0,
            });
            return true;
        }

        // 从 PAK 读取数据并写入临时文件
        let temp_path = match self.extract_to_temp(filename) { Some(p) => p, None => {
            eprintln!("  [SDLMusicInterface] load_music({}): '{}' 未找到", song_id, filename);
            return false;
        }};

        eprintln!("  [SDLMusicInterface] 写入临时文件: {}", temp_path);

        let c_path = std::ffi::CString::new(temp_path.as_str()).unwrap();
        let h_music = sdl_mixer::mix_load_mus(c_path.as_ptr());
        if h_music.is_null() {
            eprintln!("  [SDLMusicInterface] mix_load_mus 返回 null，格式可能不支持");
            return false;
        }
        eprintln!("  [SDLMusicInterface] mix_load_mus 成功");

        let c_path = std::ffi::CString::new(temp_path.as_str()).unwrap();
        let h_music = sdl_mixer::mix_load_mus(c_path.as_ptr());
        if h_music.is_null() {
            eprintln!("  [SDLMusicInterface] mix_load_mus 返回 null，格式可能不支持");
            return false;
        }
        eprintln!("  [SDLMusicInterface] mix_load_mus 成功");

        self.music_map.insert(song_id, SDLMusicInfo {
            filename: filename.to_string(), h_music, volume: 1.0, volume_cap: 1.0,
        });
        true
    }

    fn play_music(&self, song_id: i32, _offset: i32, no_loop: bool) {
        eprintln!("  [SDLMusicInterface] play_music({}, no_loop={})", song_id, no_loop);
        if let Some(info) = self.music_map.get(&song_id) {
            if info.h_music.is_null() {
                eprintln!("  [SDLMusicInterface] h_music 为 null，跳过播放");
                return;
            }
            let result = sdl_mixer::mix_play_music(info.h_music, if no_loop { 0 } else { -1 });
            eprintln!("  [SDLMusicInterface] mix_play_music 返回: {}", result);
            let vol = (self.global_volume * info.volume * 128.0) as i32;
            sdl_mixer::mix_volume_music(vol.clamp(0, 128));
            eprintln!("  [SDLMusicInterface] 设置音乐音量: {}", vol.clamp(0, 128));
        } else {
            eprintln!("  [SDLMusicInterface] song_id={} 不在 map 中", song_id);
        }
    }

    fn stop_music(&self, _song_id: i32) {
        eprintln!("  [SDLMusicInterface] stop_music");
        sdl_mixer::mix_halt_music();
    }
    fn pause_music(&self, _song_id: i32) { sdl_mixer::mix_pause_music(); }
    fn resume_music(&self, _song_id: i32) { sdl_mixer::mix_resume_music(); }
    fn stop_all_music(&self) { sdl_mixer::mix_halt_music(); }

    fn unload_music(&mut self, song_id: i32) {
        if let Some(info) = self.music_map.remove(&song_id) {
            if !info.h_music.is_null() { sdl_mixer::mix_free_music(info.h_music); }
        }
    }
    fn unload_all_music(&mut self) {
        for (_, info) in self.music_map.drain() {
            if !info.h_music.is_null() { sdl_mixer::mix_free_music(info.h_music); }
        }
    }
    fn pause_all_music(&self) { sdl_mixer::mix_pause_music(); }
    fn resume_all_music(&self) { sdl_mixer::mix_resume_music(); }

    fn fade_in(&self, song_id: i32, _offset: i32, _speed: f64, no_loop: bool) {
        self.play_music(song_id, 0, no_loop);
    }
    fn fade_out(&self, _song_id: i32, _stop_song: bool, _speed: f64) { sdl_mixer::mix_halt_music(); }
    fn fade_out_all(&self, _stop_song: bool, _speed: f64) { sdl_mixer::mix_halt_music(); }

    fn set_song_volume(&self, song_id: i32, volume: f64) {
        if let Some(info) = self.music_map.get(&song_id) {
            sdl_mixer::mix_volume_music((volume * info.volume_cap * 128.0) as i32);
        }
    }
    fn set_song_max_volume(&self, song_id: i32, max_volume: f64) {
        unsafe {
            let self_mut = &mut *(self as *const Self as *mut Self);
            if let Some(info) = self_mut.music_map.get_mut(&song_id) {
                info.volume_cap = max_volume;
            }
        }
    }
    fn is_playing(&self, _song_id: i32) -> bool { true }
    fn set_volume(&mut self, volume: f64) { self.global_volume = volume; }
    fn update(&mut self) {}
}

impl Default for SDLMusicInterface { fn default() -> Self { SDLMusicInterface::new() } }

impl Drop for SDLMusicInterface {
    fn drop(&mut self) {
        sdl_mixer::mix_halt_music();
        for (_, info) in self.music_map.drain() {
            if !info.h_music.is_null() { sdl_mixer::mix_free_music(info.h_music); }
        }
    }
}
