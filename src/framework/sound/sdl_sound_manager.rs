// SDL 声音管理器 — 完整实现
// 对应 C++ SDLSoundManager.h / SDLSoundManager.cpp

use crate::framework::sound::sound_manager::SoundManager;
use crate::framework::paklib::with_pak_interface;
use crate::ffi::sdl_mixer;
use std::array;

const MAX_SOURCE_SOUNDS: usize = 256;

pub struct SDLSoundManager {
    /// Mix_Chunk* 数组，存储已加载的音效
    source_sounds: [Option<*mut std::ffi::c_void>; MAX_SOURCE_SOUNDS],
    /// 音效文件名
    source_file_names: [String; MAX_SOURCE_SOUNDS],
    /// 基础音量
    base_volumes: [f64; MAX_SOURCE_SOUNDS],
    /// 基础平衡
    base_pans: [i32; MAX_SOURCE_SOUNDS],
    /// 主音量
    master_volume: f64,
    /// 是否已初始化
    initialized: bool,
}

impl SDLSoundManager {
    pub fn new() -> Self {
        SDLSoundManager {
            source_sounds: [None; MAX_SOURCE_SOUNDS],
            source_file_names: array::from_fn(|_| String::new()),
            base_volumes: [1.0; MAX_SOURCE_SOUNDS],
            base_pans: [0; MAX_SOURCE_SOUNDS],
            master_volume: 1.0,
            initialized: false,
        }
    }

    fn release_sound_at(&mut self, idx: i32) {
        let idx = idx as usize;
        if idx >= MAX_SOURCE_SOUNDS { return; }
        if let Some(chunk) = self.source_sounds[idx].take() {
            sdl_mixer::mix_free_chunk(chunk);
        }
        self.source_file_names[idx].clear();
    }

    fn find_free_slot(&self) -> Option<i32> {
        for i in (0..MAX_SOURCE_SOUNDS).rev() {
            if self.source_sounds[i].is_none() { return Some(i as i32); }
        }
        None
    }

    fn find_by_filename(&self, filename: &str) -> Option<i32> {
        let upper = filename.to_uppercase();
        for i in 0..MAX_SOURCE_SOUNDS {
            if !self.source_file_names[i].is_empty() && self.source_file_names[i].to_uppercase() == upper {
                return Some(i as i32);
            }
        }
        None
    }

    fn load_audio_data(filename: &str) -> Option<Vec<u8>> {
        let extensions = ["", ".wav", ".WAV", ".mp3", ".MP3", ".ogg", ".OGG"];
        for ext in &extensions {
            let path = format!("{}{}", filename, ext);
            if let Some(d) = with_pak_interface(|pak| pak.load_file(&path)) { return Some(d); }
            if let Ok(d) = std::fs::read(&path) { return Some(d); }
        }
        None
    }
}

impl SoundManager for SDLSoundManager {
    fn initialized(&self) -> bool { self.initialized }

    fn load_sound_by_id(&mut self, sfx_id: i32, filename: &str) -> bool {
        if sfx_id < 0 || sfx_id as usize >= MAX_SOURCE_SOUNDS { return false; }
        self.release_sound_at(sfx_id);
        if !self.initialized {
            self.source_file_names[sfx_id as usize] = filename.to_string();
            return true;
        }
        let data = match Self::load_audio_data(filename) { Some(d) => d, None => return false };
        let rwops = unsafe { sdl_mixer::rw_from_mem(data.as_ptr() as *mut std::ffi::c_void, data.len() as i32) };
        if rwops.is_null() { return false; }
        let chunk = sdl_mixer::mix_load_wav_rw(rwops, 1);
        if chunk.is_null() { return false; }
        self.source_sounds[sfx_id as usize] = Some(chunk);
        self.source_file_names[sfx_id as usize] = filename.to_string();
        std::mem::forget(data); // SDL_RWFromMem 持有内存指针
        true
    }

    fn load_sound(&mut self, filename: &str) -> i32 {
        if let Some(id) = self.find_by_filename(filename) { return id; }
        let slot = match self.find_free_slot() { Some(s) => s, None => return -1 };
        if !self.load_sound_by_id(slot, filename) { return -1; }
        slot
    }

    fn release_sound(&mut self, sfx_id: i32) { self.release_sound_at(sfx_id); }

    fn set_volume(&mut self, volume: f64) { self.master_volume = volume; }

    fn set_base_volume(&mut self, sfx_id: i32, base_volume: f64) -> bool {
        if sfx_id < 0 || sfx_id as usize >= MAX_SOURCE_SOUNDS { return false; }
        self.base_volumes[sfx_id as usize] = base_volume;
        true
    }

    fn set_base_pan(&mut self, sfx_id: i32, base_pan: i32) -> bool {
        if sfx_id < 0 || sfx_id as usize >= MAX_SOURCE_SOUNDS { return false; }
        self.base_pans[sfx_id as usize] = base_pan;
        true
    }

    fn play_sound(&self, sfx_id: i32) -> bool {
        if sfx_id < 0 || sfx_id as usize >= MAX_SOURCE_SOUNDS { return false; }
        let chunk = match self.source_sounds[sfx_id as usize] { Some(c) => c, None => return false };
        let channel = sdl_mixer::mix_play_channel_timed(-1, chunk, 0, -1);
        if channel < 0 { return false; }
        let vol = (self.master_volume * self.base_volumes[sfx_id as usize] * 128.0) as i32;
        sdl_mixer::mix_volume(channel, vol.clamp(0, 128));
        let pan = self.base_pans[sfx_id as usize];
        if pan < 0 {
            sdl_mixer::mix_set_panning(channel, 128, (128 + pan).clamp(0, 128) as u8);
        } else if pan > 0 {
            sdl_mixer::mix_set_panning(channel, (128 - pan).clamp(0, 128) as u8, 128);
        }
        true
    }

    fn stop_sound(&self, _sfx_id: i32) {
        // 简化：无法精确停止特定音效（需声道跟踪）
    }

    fn stop_all_sounds(&self) { sdl_mixer::mix_halt_channel(-1); }

    fn set_master_volume(&mut self, volume: f64) { self.master_volume = volume; }

    fn get_master_volume(&self) -> f64 { self.master_volume }

    fn flush(&mut self) {}

    fn get_free_sound_id(&self) -> i32 { self.find_free_slot().unwrap_or(-1) }

    fn num_sounds(&self) -> i32 {
        self.source_sounds.iter().filter(|s| s.is_some()).count() as i32
    }
}

impl Default for SDLSoundManager {
    fn default() -> Self { SDLSoundManager::new() }
}

impl Drop for SDLSoundManager {
    fn drop(&mut self) {
        sdl_mixer::mix_halt_channel(-1);
        for i in 0..MAX_SOURCE_SOUNDS {
            if let Some(chunk) = self.source_sounds[i].take() {
                sdl_mixer::mix_free_chunk(chunk);
            }
        }
    }
}
