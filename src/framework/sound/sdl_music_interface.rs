// SDL 音乐接口 — 完整实现
// MO3 回退：SDL_mixer 不支持的格式通过 libopenmpt 解码为 WAV 再播放

#![allow(dead_code, invalid_reference_casting)]

use std::collections::HashMap;
use crate::framework::sound::music_interface::MusicInterface;
use crate::framework::paklib::with_pak_interface;
use crate::ffi::sdl_mixer;
use crate::ffi::libopenmpt;

const OPENMPT_SAMPLE_RATE: i32 = 48000;

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

    /// 尝试用 SDL_mixer 从 PAK 加载
    fn try_mixer_load(&self, song_id: i32, filename: &str) -> bool {
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

    /// 通过 libopenmpt 解码 MO3/IT/XM 为 WAV → Mix_LoadMUS
    fn try_libopenmpt_decode(&self, song_id: i32, filename: &str) -> bool {
        if !libopenmpt::is_available() { return false; }

        let data = match with_pak_interface(|pak| {
            let exts = [".mo3", ".MO3", ".it", ".IT", ".xm", ".XM", ".s3m", ".S3M", ".mod", ".MOD", ""];
            for ext in &exts {
                let path = format!("{}{}", filename, ext);
                if let Some(d) = pak.load_file(&path) { return Some((path, d)); }
                if let Ok(d) = std::fs::read(&path) { return Some((path, d)); }
            }
            None
        }) {
            Some((path, d)) => { eprintln!("  [libopenmpt] 从 '{}' 加载 {} 字节", path, d.len()); d }
            None => return false,
        };

        // 解码
        let mod_ = libopenmpt::create_from_memory(&data);
        if mod_.is_null() {
            eprintln!("  [libopenmpt] openmpt_module_create_from_memory 失败");
            return false;
        }

        libopenmpt::set_repeat_count(mod_, 0);

        let duration = libopenmpt::get_duration_seconds(mod_);
        eprintln!("  [libopenmpt] 时长: {:.2}s", duration);

        let num_channels = libopenmpt::get_num_channels(mod_);
        if duration <= 0.0 || num_channels <= 0 {
            libopenmpt::destroy(mod_);
            return false;
        }

        // 渲染 PCM
        let total_frames = (duration * OPENMPT_SAMPLE_RATE as f32) as usize;
        let mut pcm: Vec<i16> = vec![0i16; total_frames * 2]; // 立体声交错

        let mut written = 0usize;
        loop {
            let n = libopenmpt::read_interleaved_stereo(mod_, OPENMPT_SAMPLE_RATE, &mut pcm[written..]);
            if n == 0 { break; }
            written += n as usize;
        }
        libopenmpt::destroy(mod_);

        pcm.truncate(written);
        if pcm.len() < 2 { return false; }

        eprintln!("  [libopenmpt] 解码 {} 个 PCM 样本", pcm.len());

        // 写入 WAV
        let num_samples = pcm.len() as u32;
        let data_size = num_samples * 2;
        let file_size = 44 + data_size;
        let mut wav = Vec::with_capacity(file_size as usize);

        wav.extend_from_slice(b"RIFF");
        wav.extend(&(file_size - 8).to_le_bytes());
        wav.extend_from_slice(b"WAVE");
        wav.extend_from_slice(b"fmt ");
        wav.extend(&16u32.to_le_bytes());
        wav.extend(&1u16.to_le_bytes());          // PCM
        wav.extend(&2u16.to_le_bytes());          // stereo
        wav.extend(&(OPENMPT_SAMPLE_RATE as u32).to_le_bytes());
        wav.extend(&(OPENMPT_SAMPLE_RATE as u32 * 2 * 2).to_le_bytes());
        wav.extend(&4u16.to_le_bytes());          // block align
        wav.extend(&16u16.to_le_bytes());         // 16-bit
        wav.extend_from_slice(b"data");
        wav.extend(&data_size.to_le_bytes());
        for &sample in &pcm {
            wav.extend(&sample.to_le_bytes());
        }

        // 临时 WAV 文件 → Mix_LoadMUS
        if let Some(temp) = self.write_temp_file(&format!("{}.wav", filename), &wav) {
            let c_path = std::ffi::CString::new(temp).unwrap();
            let h = sdl_mixer::load_mus(c_path.as_ptr());
            if !h.is_null() {
                unsafe {
                    let sm = &mut *(self as *const Self as *mut Self);
                    sm.music_map.insert(song_id, MusicInfo {
                        filename: format!("{} (libopenmpt decoded)", filename),
                        handle: h, volume: 1.0, volume_cap: 1.0,
                    });
                }
                return true;
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
        if self.try_mixer_load(song_id, filename) { return true; }

        // 3. libopenmpt 解码 MO3/IT/XM → WAV → Mix_LoadMUS
        if self.try_libopenmpt_decode(song_id, filename) { return true; }

        eprintln!("  [SDLMusicInterface] 所有加载方式均失败: '{}'", filename);
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
