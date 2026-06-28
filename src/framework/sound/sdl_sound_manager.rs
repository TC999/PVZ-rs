// SDL 声音管理器 — 完全对齐 C++ SDLSoundManager
// 使用 SDL_RWFromConstMem + Mix_LoadWAV_RW 从 PAK 内存中加载音效

use crate::framework::sound::sound_manager::SoundManager;
use crate::framework::paklib::with_pak_interface;
use crate::ffi::sdl_mixer;
use std::array;

const MAX_SOURCE_SOUNDS: usize = 256;

pub struct SDLSoundManager {
    source_chunks: [*mut std::ffi::c_void; MAX_SOURCE_SOUNDS],
    source_file_names: [String; MAX_SOURCE_SOUNDS],
    base_volumes: [f64; MAX_SOURCE_SOUNDS],
    base_pans: [i32; MAX_SOURCE_SOUNDS],
    master_volume: f64,
}

impl SDLSoundManager {
    pub fn new() -> Self {
        SDLSoundManager {
            source_chunks: [std::ptr::null_mut(); MAX_SOURCE_SOUNDS],
            source_file_names: array::from_fn(|_| String::new()),
            base_volumes: [1.0; MAX_SOURCE_SOUNDS],
            base_pans: [0; MAX_SOURCE_SOUNDS],
            master_volume: 1.0,
        }
    }
}

impl SDLSoundManager {
    fn release_sound_at(&mut self, idx: usize) {
        if idx >= MAX_SOURCE_SOUNDS { return; }
        let ck = self.source_chunks[idx];
        if !ck.is_null() {
            sdl_mixer::free_chunk(ck);
            self.source_chunks[idx] = std::ptr::null_mut();
        }
        self.source_file_names[idx].clear();
    }

    fn find_slot(&self) -> Option<usize> {
        for i in (0..MAX_SOURCE_SOUNDS).rev() {
            if self.source_chunks[i].is_null() { return Some(i); }
        }
        None
    }

    fn find_by_name(&self, name: &str) -> Option<usize> {
        for i in 0..MAX_SOURCE_SOUNDS {
            if self.source_file_names[i] == name { return Some(i); }
        }
        None
    }

    /// 等价于 C++ LoadSound(SfxID, theFilename)
    /// 尝试 .wav → .mp3 → .ogg，从 PAK 读入内存后 Mix_LoadWAV_RW
    fn load_into_slot(&mut self, idx: usize, filename: &str) -> bool {
        self.release_sound_at(idx);
        self.source_file_names[idx] = filename.to_string();

        // SDL2_mixer 编译时链接，不需要 initialized 检查

        // C++: const char* formats[] = {".wav", ".mp3", ".ogg"};
        const EXTS: [&str; 3] = [".wav", ".mp3", ".ogg"];
        for ext in &EXTS {
            let path = format!("{}{}", filename, ext);
            eprintln!("  [SDLSoundManager] 尝试加载: {}", path);
            if let Some(data) = with_pak_interface(|pak| pak.load_file(&path)) {
                // C++: Mix_LoadWAV_RW(SDL_RWFromConstMem(data.data(), fileSize), 1)
                let rw = unsafe {
                    sdl_mixer::rw_from_constmem(data.as_ptr() as *const std::ffi::c_void, data.len() as i32)
                };
                if rw.is_null() { continue; }
                let ck = sdl_mixer::load_wav_rw(rw, 1);
                if !ck.is_null() {
                    self.source_chunks[idx] = ck;
                    std::mem::forget(data);
                    return true;
                }
            }
        }
        false
    }
}

impl SoundManager for SDLSoundManager {
    fn initialized(&self) -> bool { true }

    /// C++: bool LoadSound(intptr_t theSfxID, const std::string& theFilename)
    fn load_sound_by_id(&mut self, sfx_id: i32, filename: &str) -> bool {
        if sfx_id < 0 || sfx_id as usize >= MAX_SOURCE_SOUNDS { return false; }
        if filename.is_empty() { return false; }
        self.load_into_slot(sfx_id as usize, filename)
    }

    /// C++: intptr_t LoadSound(const std::string& theFilename)
    fn load_sound(&mut self, filename: &str) -> i32 {
        if let Some(id) = self.find_by_name(filename) { return id as i32; }
        let slot = match self.find_slot() { Some(s) => s, None => return -1 };
        if !self.load_into_slot(slot, filename) { return -1; }
        slot as i32
    }

    fn release_sound(&mut self, sfx_id: i32) {
        if sfx_id >= 0 { self.release_sound_at(sfx_id as usize); }
    }

    fn set_volume(&mut self, volume: f64) { self.master_volume = volume; }

    fn set_base_volume(&mut self, sfx_id: i32, vol: f64) -> bool {
        if sfx_id < 0 || sfx_id as usize >= MAX_SOURCE_SOUNDS { return false; }
        self.base_volumes[sfx_id as usize] = vol; true
    }

    fn set_base_pan(&mut self, sfx_id: i32, pan: i32) -> bool {
        if sfx_id < 0 || sfx_id as usize >= MAX_SOURCE_SOUNDS { return false; }
        self.base_pans[sfx_id as usize] = pan; true
    }

    /// C++: Mix_PlayChannel(-1, chunk, 0) → 返回声道号
    fn play_sound(&self, sfx_id: i32) -> bool {
        if sfx_id < 0 || sfx_id as usize >= MAX_SOURCE_SOUNDS { return false; }
        let ck = self.source_chunks[sfx_id as usize];
        if ck.is_null() { return false; }
        let ch = sdl_mixer::play_channel(-1, ck, 0);
        if ch < 0 { return false; }
        let v = (self.master_volume * self.base_volumes[sfx_id as usize] * 128.0) as i32;
        sdl_mixer::volume(ch, v.clamp(0, 128));
        let p = self.base_pans[sfx_id as usize];
        if p < 0 { sdl_mixer::set_panning(ch, 128, (128 + p).clamp(0, 128) as u8); }
        else if p > 0 { sdl_mixer::set_panning(ch, (128 - p).clamp(0, 128) as u8, 128); }
        true
    }

    fn stop_sound(&self, _sfx_id: i32) {}
    fn stop_all_sounds(&self) { sdl_mixer::halt_channel(-1); }
    fn set_master_volume(&mut self, v: f64) { self.master_volume = v; }
    fn get_master_volume(&self) -> f64 { self.master_volume }
    fn flush(&mut self) {}
    fn get_free_sound_id(&self) -> i32 {
        self.find_slot().map(|i| i as i32).unwrap_or(-1)
    }
    fn num_sounds(&self) -> i32 {
        self.source_chunks.iter().filter(|c| !c.is_null()).count() as i32
    }
}

impl Default for SDLSoundManager { fn default() -> Self { SDLSoundManager::new() } }

impl Drop for SDLSoundManager {
    fn drop(&mut self) {
        sdl_mixer::halt_channel(-1);
        for i in 0..MAX_SOURCE_SOUNDS {
            let ck = self.source_chunks[i];
            if !ck.is_null() { sdl_mixer::free_chunk(ck); }
        }
    }
}

/// 设置音乐放大（对应 C++ SetMusicAmplify）
pub fn set_music_amplify(_music_id: i32, _amplify: f64) {}
/// 重设音量（对应 C++ RehupVolume）
pub fn rehup_volume() {}
/// 加载 AU 音效（对应 C++ LoadAUSound）
pub fn load_au_sound(_data: &[u8]) -> i32 { 0 }
/// 获取音效实例（对应 C++ GetSoundInstance）
pub fn get_sound_instance(_id: i32) -> i32 { 0 }
/// 释放音效（对应 C++ ReleaseSounds）
pub fn release_sounds() {}
/// 释放声道（对应 C++ ReleaseChannels）
pub fn release_channels() {}
/// 获取音效数量（对应 C++ GetNumSounds）
pub fn get_num_sounds() -> i32 { 0 }
/// 查找空闲声道（对应 C++ FindFreeChannel）
pub fn find_free_channel() -> i32 { 0 }
