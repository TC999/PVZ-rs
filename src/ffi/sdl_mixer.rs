// PvZ Portable Rust 翻译 — SDL_mixer FFI 绑定（Windows 动态加载）
// 对应 C++ 的 SDL_mixer_ext/SDL_mixer_ext.h
//
// 在 Windows 上通过 LoadLibrary/GetProcAddress 运行时加载 SDL2_mixer.dll。
// 如果 DLL 不存在，所有函数静默返回失败，游戏继续运行但不发声。

#![allow(non_camel_case_types, dead_code)]

use std::os::raw::{c_int, c_uint, c_void, c_char};

// ============================================================
// SDL_mixer 类型定义
// ============================================================

pub type Mix_Chunk = c_void;
pub type Mix_Music = c_void;

/// 全局函数表（运行时加载）
pub struct MixFunctions {
    pub open_audio: unsafe extern "C" fn(frequency: c_int, format: u16, channels: c_int, chunksize: c_int) -> c_int,
    pub close_audio: unsafe extern "C" fn(),
    pub allocate_channels: unsafe extern "C" fn(numchannels: c_int) -> c_int,
    pub load_wav_rw: unsafe extern "C" fn(src: *mut c_void, freesrc: c_int) -> *mut Mix_Chunk,
    pub free_chunk: unsafe extern "C" fn(chunk: *mut Mix_Chunk),
    pub play_channel_timed: unsafe extern "C" fn(channel: c_int, chunk: *mut Mix_Chunk, loops: c_int, ticks: c_int) -> c_int,
    pub halt_channel: unsafe extern "C" fn(channel: c_int) -> c_int,
    pub volume: unsafe extern "C" fn(channel: c_int, volume: c_int) -> c_int,
    pub set_panning: unsafe extern "C" fn(channel: c_int, left: u8, right: u8) -> c_int,
    pub load_mus: unsafe extern "C" fn(file: *const c_char) -> *mut Mix_Music,
    pub free_music: unsafe extern "C" fn(music: *mut Mix_Music),
    pub play_music: unsafe extern "C" fn(music: *mut Mix_Music, loops: c_int) -> c_int,
    pub halt_music: unsafe extern "C" fn() -> c_int,
    pub volume_music: unsafe extern "C" fn(volume: c_int) -> c_int,
    pub pause_music: unsafe extern "C" fn(),
    pub resume_music: unsafe extern "C" fn(),
}

static mut G_MIX: Option<Box<MixFunctions>> = None;
static mut G_MIX_DLL: Option<*mut c_void> = None;

/// 是否成功加载 SDL_mixer
pub fn is_mixer_available() -> bool {
    unsafe { G_MIX.is_some() }
}

#[cfg(windows)]
unsafe extern "system" {
    fn LoadLibraryA(lpLibFileName: *const i8) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const i8) -> *mut c_void;
    fn FreeLibrary(hLibModule: *mut c_void) -> c_int;
    fn GetModuleHandleA(lpModuleName: *const i8) -> *mut c_void;
}

/// 加载 SDL_mixer DLL 并获取所有函数指针
pub fn load_mixer_library() -> bool {
    unsafe {
        if G_MIX.is_some() { return true; }

        let dll_names = [
            "SDL2_mixer.dll\0".as_ptr() as *const i8,
            "SDL2_mixer_ext.dll\0".as_ptr() as *const i8,
        ];

        let mut hmod: *mut c_void = std::ptr::null_mut();
        for &name in &dll_names {
            hmod = LoadLibraryA(name);
            if !hmod.is_null() { break; }
        }
        if hmod.is_null() { return false; }

        macro_rules! load_fn {
            ($name:literal) => {{
                let ptr = GetProcAddress(hmod, concat!($name, "\0").as_ptr() as *const i8);
                if ptr.is_null() { FreeLibrary(hmod); return false; }
                std::mem::transmute::<*mut c_void, _>(ptr)
            }};
        }

        G_MIX = Some(Box::new(MixFunctions {
            open_audio: load_fn!("Mix_OpenAudio"),
            close_audio: load_fn!("Mix_CloseAudio"),
            allocate_channels: load_fn!("Mix_AllocateChannels"),
            load_wav_rw: load_fn!("Mix_LoadWAV_RW"),
            free_chunk: load_fn!("Mix_FreeChunk"),
            play_channel_timed: load_fn!("Mix_PlayChannelTimed"),
            halt_channel: load_fn!("Mix_HaltChannel"),
            volume: load_fn!("Mix_Volume"),
            set_panning: load_fn!("Mix_SetPanning"),
            load_mus: load_fn!("Mix_LoadMUS"),
            free_music: load_fn!("Mix_FreeMusic"),
            play_music: load_fn!("Mix_PlayMusic"),
            halt_music: load_fn!("Mix_HaltMusic"),
            volume_music: load_fn!("Mix_VolumeMusic"),
            pause_music: load_fn!("Mix_PauseMusic"),
            resume_music: load_fn!("Mix_ResumeMusic"),
        }));
        G_MIX_DLL = Some(hmod);
        true
    }
}

/// 卸载 SDL_mixer DLL
pub fn unload_mixer_library() {
    unsafe {
        G_MIX = None;
        if let Some(hmod) = G_MIX_DLL.take() {
            FreeLibrary(hmod);
        }
    }
}

// ============================================================
// 安全包装函数
// ============================================================

pub fn mix_open_audio(frequency: c_int, format: u16, channels: c_int, chunksize: c_int) -> c_int {
    unsafe {
        match G_MIX.as_ref() { Some(m) => (m.open_audio)(frequency, format, channels, chunksize), None => -1 }
    }
}

pub fn mix_close_audio() {
    unsafe { if let Some(ref m) = G_MIX { (m.close_audio)(); } }
}

pub fn mix_allocate_channels(num: c_int) -> c_int {
    unsafe { match G_MIX.as_ref() { Some(m) => (m.allocate_channels)(num), None => -1 } }
}

/// 从 RWops 加载 WAV。SDL_RWFromMem 从 SDL2.dll 动态获取
pub fn mix_load_wav_rw(src: *mut c_void, freesrc: c_int) -> *mut Mix_Chunk {
    unsafe { match G_MIX.as_ref() { Some(m) => (m.load_wav_rw)(src, freesrc), None => std::ptr::null_mut() } }
}

pub fn mix_free_chunk(chunk: *mut Mix_Chunk) {
    unsafe { if let Some(ref m) = G_MIX { (m.free_chunk)(chunk); } }
}

pub fn mix_play_channel_timed(channel: c_int, chunk: *mut Mix_Chunk, loops: c_int, ticks: c_int) -> c_int {
    unsafe { match G_MIX.as_ref() { Some(m) => (m.play_channel_timed)(channel, chunk, loops, ticks), None => -1 } }
}

pub fn mix_halt_channel(channel: c_int) -> c_int {
    unsafe { match G_MIX.as_ref() { Some(m) => (m.halt_channel)(channel), None => -1 } }
}

pub fn mix_volume(channel: c_int, volume: c_int) -> c_int {
    unsafe { match G_MIX.as_ref() { Some(m) => (m.volume)(channel, volume), None => 0 } }
}

pub fn mix_set_panning(channel: c_int, left: u8, right: u8) -> c_int {
    unsafe { match G_MIX.as_ref() { Some(m) => (m.set_panning)(channel, left, right), None => -1 } }
}

pub fn mix_load_mus(file: *const c_char) -> *mut Mix_Music {
    unsafe { match G_MIX.as_ref() { Some(m) => (m.load_mus)(file), None => std::ptr::null_mut() } }
}

pub fn mix_free_music(music: *mut Mix_Music) {
    unsafe { if let Some(ref m) = G_MIX { (m.free_music)(music); } }
}

pub fn mix_play_music(music: *mut Mix_Music, loops: c_int) -> c_int {
    unsafe { match G_MIX.as_ref() { Some(m) => (m.play_music)(music, loops), None => -1 } }
}

pub fn mix_halt_music() -> c_int {
    unsafe { match G_MIX.as_ref() { Some(m) => (m.halt_music)(), None => -1 } }
}

pub fn mix_volume_music(volume: c_int) -> c_int {
    unsafe { match G_MIX.as_ref() { Some(m) => (m.volume_music)(volume), None => 0 } }
}

pub fn mix_pause_music() {
    unsafe { if let Some(ref m) = G_MIX { (m.pause_music)(); } }
}

pub fn mix_resume_music() {
    unsafe { if let Some(ref m) = G_MIX { (m.resume_music)(); } }
}

/// 从内存创建 SDL_RWops（从 SDL2.dll 动态获取）
pub unsafe fn rw_from_mem(mem: *mut c_void, size: c_int) -> *mut c_void {
    let sdl2_name = "SDL2.dll\0".as_ptr() as *const i8;
    let handle = GetModuleHandleA(sdl2_name);
    if handle.is_null() { return std::ptr::null_mut(); }
    let ptr = GetProcAddress(handle, "SDL_RWFromMem\0".as_ptr() as *const i8);
    if ptr.is_null() { return std::ptr::null_mut(); }
    type FnType = unsafe extern "C" fn(*mut c_void, c_int) -> *mut c_void;
    let func: FnType = std::mem::transmute(ptr);
    func(mem, size)
}
