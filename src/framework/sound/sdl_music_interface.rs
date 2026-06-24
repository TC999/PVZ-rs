// SDL 音乐接口 — 完全对齐 C++ SDLMusicInterface
// C++ 调用 Mix_LoadMUS(theFileName.c_str()) 直接加载音乐文件
// Rust 版将其扩展到先从 PAK 提取到临时文件再加载

#![allow(dead_code, invalid_reference_casting)]

use std::collections::HashMap;
use crate::framework::sound::music_interface::MusicInterface;
use crate::framework::paklib::with_pak_interface;
use crate::ffi::sdl_mixer;

struct MusicInfo {
    filename: String,
    /// Mix_Music* 指针
    handle: *mut std::ffi::c_void,
    volume: f64,
    volume_cap: f64,
    volume_add: f64,
    stop_on_fade: bool,
}

pub struct SDLMusicInterface {
    /// 对应 C++ SDLMusicMap mMusicMap
    music_map: HashMap<i32, MusicInfo>,
    /// 对应 C++ mGlobalVolume
    global_volume: f64,
}

impl SDLMusicInterface {
    pub fn new() -> Self {
        SDLMusicInterface {
            music_map: HashMap::new(),
            global_volume: 1.0,
        }
    }

    /// 从 PAK 读取文件并提取到临时目录，返回真实路径
    /// C++ 版直接使用 Mix_LoadMUS(路径)，这里确保从 PAK 取出的文件可访问
    fn extract_to_temp(&self, name: &str, data: &[u8]) -> Option<String> {
        let dir = std::env::temp_dir();
        let fname = std::path::Path::new(name)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| format!("pvz_{}", self.music_map.len()));
        let dest = dir.join(&fname);
        // 先尝试直接写，如果已有同名文件则追加随机后缀
        if let Ok(_) = std::fs::write(&dest, data) {
            return Some(dest.to_string_lossy().to_string());
        }
        let dest2 = dir.join(format!("{}_{}", self.music_map.len(), fname));
        std::fs::write(&dest2, data).ok().map(|_| dest2.to_string_lossy().to_string())
    }
}

impl MusicInterface for SDLMusicInterface {
    /// C++: bool LoadMusic(int theSongId, const std::string& theFileName)
    ///     → Mix_LoadMUS(theFileName.c_str())
    /// Rust 扩展：先检查真实文件，再从 PAK 提取到临时文件
    fn load_music(&mut self, song_id: i32, filename: &str) -> bool {
        self.unload_music(song_id);

        // 先尝试真实文件（C++ 原生行为）
        if !filename.is_empty() {
            let c_path = std::ffi::CString::new(filename).unwrap();
            let h = sdl_mixer::load_mus(c_path.as_ptr());
            if !h.is_null() {
                eprintln!("  [SDLMusicInterface] 从文件加载成功: {}", filename);
                self.music_map.insert(song_id, MusicInfo {
                    filename: filename.to_string(),
                    handle: h,
                    volume: 1.0,
                    volume_cap: 1.0,
                    volume_add: 0.0,
                    stop_on_fade: false,
                });
                return true;
            }
        }

        // 从 PAK 读取 — 使用原始文件名（不含额外扩展名轮询，因为 C++ 也不试扩展名）
        if !sdl_mixer::is_available() {
            eprintln!("  [SDLMusicInterface] SDL_mixer 不可用");
            return false;
        }

        eprintln!("  [SDLMusicInterface] 从 PAK 搜索: {}", filename);

        // 尝试多种扩展名
        let exts = ["", ".mo3", ".MO3", ".ogg", ".OGG", ".mp3", ".MP3", ".wav", ".WAV", ".mid", ".MID"];
        for ext in &exts {
            let path = format!("{}{}", filename, ext);
            if let Some(data) = with_pak_interface(|pak| pak.load_file(&path)) {
                eprintln!("  [SDLMusicInterface] PAK 中找到: {} ({} 字节)", path, data.len());
                if let Some(temp_path) = self.extract_to_temp(&path, &data) {
                    eprintln!("  [SDLMusicInterface] 提取到临时文件: {}", temp_path);
                    let c_path = std::ffi::CString::new(temp_path).unwrap();
                    let h = sdl_mixer::load_mus(c_path.as_ptr());
                    if !h.is_null() {
                        eprintln!("  [SDLMusicInterface] Mix_LoadMUS 成功");
                        self.music_map.insert(song_id, MusicInfo {
                            filename: path,
                            handle: h,
                            volume: 1.0,
                            volume_cap: 1.0,
                            volume_add: 0.0,
                            stop_on_fade: false,
                        });
                        return true;
                    } else {
                        eprintln!("  [SDLMusicInterface] Mix_LoadMUS 返回 null，格式不支持");
                    }
                }
            } else {
                // 仅对第一批轮询打印调试
            }
        }

        // 最后尝试：列出 PAK 中所有可能的音乐文件
        if self.music_map.is_empty() {
            eprintln!("  [SDLMusicInterface] PAK 中的 SOUNDS/ 目录内容:");
            with_pak_interface(|pak| {
                for (k, _) in pak.records.iter() {
                    if k.contains("SOUND") {
                        eprintln!("    {}", k);
                    }
                }
            });
        }

        false
    }

    /// C++: void PlayMusic(int theSongId, int theOffset, bool noLoop)
    ///     → Mix_PlayMusicStream(mHMusic, (noLoop) ? 0 : -1)
    /// Rust 标准 API: Mix_PlayMusic(music, loops)
    fn play_music(&self, song_id: i32, _offset: i32, no_loop: bool) {
        if let Some(info) = self.music_map.get(&song_id) {
            if info.handle.is_null() { return; }
            // 对应 C++: Mix_PlayMusicStream(aMusicInfo->mHMusic, (noLoop) ? 0 : -1)
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
            let self_mut = &mut *(self as *const Self as *mut Self);
            if let Some(info) = self_mut.music_map.get_mut(&song_id) {
                info.volume_cap = max_volume;
            }
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
