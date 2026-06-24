// SDL_mixer Ext FFI 绑定 — 运行时动态加载 SDL2_mixer_ext.dll（优先）或 SDL2_mixer.dll
// 完全对齐 C++ SDL_mixer/SDL_mixer_ext 调用约定
// SDL_mixer_ext 兼容所有标准 Mix_* API，并扩展了 Mix_PlayMusicStream 等增强函数

#![allow(non_camel_case_types, dead_code)]
use std::os::raw::{c_int, c_uchar, c_void, c_char};

pub type Mix_Chunk = c_void;
pub type Mix_Music = c_void;

pub const MAX_VOLUME: i32 = 128;

#[cfg(windows)]
unsafe extern "system" {
    fn LoadLibraryA(lpLibFileName: *const i8) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const i8) -> *mut c_void;
    fn FreeLibrary(hLibModule: *mut c_void) -> c_int;
    fn GetModuleHandleA(lpModuleName: *const i8) -> *mut c_void;
}

#[cfg(not(windows))]
#[allow(non_snake_case)]
unsafe fn LoadLibraryA(_lpLibFileName: *const i8) -> *mut c_void { std::ptr::null_mut() }
#[cfg(not(windows))]
#[allow(non_snake_case)]
unsafe fn GetProcAddress(_hModule: *mut c_void, _lpProcName: *const i8) -> *mut c_void { std::ptr::null_mut() }
#[cfg(not(windows))]
#[allow(non_snake_case)]
unsafe fn FreeLibrary(_hLibModule: *mut c_void) -> c_int { 0 }
#[cfg(not(windows))]
#[allow(non_snake_case)]
unsafe fn GetModuleHandleA(_lpModuleName: *const i8) -> *mut c_void { std::ptr::null_mut() }

struct MixFuncs {
    open_audio: unsafe extern "C" fn(c_int, u16, c_int, c_int) -> c_int,
    close_audio: unsafe extern "C" fn(),
    allocate_channels: unsafe extern "C" fn(c_int) -> c_int,
    load_wav_rw: unsafe extern "C" fn(*mut c_void, c_int) -> *mut Mix_Chunk,
    free_chunk: unsafe extern "C" fn(*mut Mix_Chunk),
    play_channel_timed: unsafe extern "C" fn(c_int, *mut Mix_Chunk, c_int, c_int) -> c_int,
    halt_channel: unsafe extern "C" fn(c_int) -> c_int,
    volume: unsafe extern "C" fn(c_int, c_int) -> c_int,
    set_panning: unsafe extern "C" fn(c_int, c_uchar, c_uchar) -> c_int,
    load_mus: unsafe extern "C" fn(*const c_char) -> *mut Mix_Music,
    free_music: unsafe extern "C" fn(*mut Mix_Music),
    /// Mix_PlayMusic（标准 API，所有 DLL 都提供）
    play_music: unsafe extern "C" fn(*mut Mix_Music, c_int) -> c_int,
    /// Mix_PlayMusicStream（SDL_mixer_ext 增强 API，优先使用）
    play_music_stream: Option<unsafe extern "C" fn(*mut Mix_Music, c_int) -> c_int>,
    halt_music: unsafe extern "C" fn() -> c_int,
    volume_music: unsafe extern "C" fn(c_int) -> c_int,
    pause_music: unsafe extern "C" fn(),
    resume_music: unsafe extern "C" fn(),
}

static mut MIX: Option<MixFuncs> = None;
/// 标记是否为 SDL_mixer_ext（支持扩展 API）
#[allow(dead_code)]
/// 标记是否为 SDL_mixer_ext（支持扩展 API）
static mut G_IS_EXT: bool = false;

pub fn is_available() -> bool { unsafe { MIX.is_some() } }

pub fn load_library() -> bool {
    unsafe {
        if MIX.is_some() { return true; }

        // 优先加载 SDL2_mixer_ext.dll，它兼容所有标准 API 且支持更多格式 (MO3/OGG 等)
        // 回退到 SDL2_mixer.dll（基本版，不支持 MO3）
        let names: &[*const i8] = &[
            "SDL2_mixer_ext.dll\0".as_ptr() as *const i8,
            "SDL2_mixer.dll\0".as_ptr() as *const i8,
        ];

        let mut mod_: *mut c_void = std::ptr::null_mut();
        let mut is_ext_dll = false;
        for (i, &n) in names.iter().enumerate() {
            mod_ = LoadLibraryA(n);
            if !mod_.is_null() {
                is_ext_dll = i == 0;
                break;
            }
        }
        if mod_.is_null() { return false; }
        G_IS_EXT = is_ext_dll;
        if G_IS_EXT {
            eprintln!("[sdl_mixer] 已加载 SDL2_mixer_ext.dll（支持增强格式）");
        } else {
            eprintln!("[sdl_mixer] 已加载 SDL2_mixer.dll（基础版，不支持 MO3）");
        }

        macro_rules! get {
            ($n:literal) => {{
                let p = GetProcAddress(mod_, concat!($n, "\0").as_ptr() as *const i8);
                if p.is_null() { FreeLibrary(mod_); return false; }
                std::mem::transmute::<*mut c_void, _>(p)
            }};
        }
        // 可选函数 — 找不到返回 None
        macro_rules! get_opt {
            ($n:literal) => {{
                let p = GetProcAddress(mod_, concat!($n, "\0").as_ptr() as *const i8);
                if p.is_null() { None } else { Some(std::mem::transmute::<*mut c_void, _>(p)) }
            }};
        }

        MIX = Some(MixFuncs {
            open_audio: get!("Mix_OpenAudio"),
            close_audio: get!("Mix_CloseAudio"),
            allocate_channels: get!("Mix_AllocateChannels"),
            load_wav_rw: get!("Mix_LoadWAV_RW"),
            free_chunk: get!("Mix_FreeChunk"),
            play_channel_timed: get!("Mix_PlayChannelTimed"),
            halt_channel: get!("Mix_HaltChannel"),
            volume: get!("Mix_Volume"),
            set_panning: get!("Mix_SetPanning"),
            load_mus: get!("Mix_LoadMUS"),
            free_music: get!("Mix_FreeMusic"),
            play_music: get!("Mix_PlayMusic"),
            play_music_stream: get_opt!("Mix_PlayMusicStream"),
            halt_music: get!("Mix_HaltMusic"),
            volume_music: get!("Mix_VolumeMusic"),
            pause_music: get!("Mix_PauseMusic"),
            resume_music: get!("Mix_ResumeMusic"),
        });
        true
    }
}

// ============ 安全包装 ============

pub fn open_audio(freq: c_int, fmt: u16, ch: c_int, size: c_int) -> c_int {
    unsafe { match MIX { Some(ref m) => (m.open_audio)(freq, fmt, ch, size), None => -1 } }
}
pub fn close_audio() { unsafe { if let Some(ref m) = MIX { (m.close_audio)(); } } }
pub fn allocate_channels(n: c_int) -> c_int {
    unsafe { match MIX { Some(ref m) => (m.allocate_channels)(n), None => -1 } }
}
pub fn load_wav_rw(rw: *mut c_void, freesrc: c_int) -> *mut Mix_Chunk {
    unsafe { match MIX { Some(ref m) => (m.load_wav_rw)(rw, freesrc), None => std::ptr::null_mut() } }
}
pub fn free_chunk(c: *mut Mix_Chunk) { unsafe { if let Some(ref m) = MIX { (m.free_chunk)(c); } } }
pub fn play_channel(ch: c_int, ck: *mut Mix_Chunk, loops: c_int) -> c_int {
    unsafe { match MIX { Some(ref m) => (m.play_channel_timed)(ch, ck, loops, -1), None => -1 } }
}
pub fn halt_channel(ch: c_int) -> c_int {
    unsafe { match MIX { Some(ref m) => (m.halt_channel)(ch), None => -1 } }
}
pub fn volume(ch: c_int, vol: c_int) -> c_int {
    unsafe { match MIX { Some(ref m) => (m.volume)(ch, vol), None => 0 } }
}
pub fn set_panning(ch: c_int, l: u8, r: u8) -> c_int {
    unsafe { match MIX { Some(ref m) => (m.set_panning)(ch, l, r), None => -1 } }
}
pub fn load_mus(path: *const c_char) -> *mut Mix_Music {
    unsafe { match MIX { Some(ref m) => (m.load_mus)(path), None => std::ptr::null_mut() } }
}
pub fn free_music(m: *mut Mix_Music) { unsafe { if let Some(ref m_) = MIX { (m_.free_music)(m); } } }

/// 播放音乐 — 优先使用 Mix_PlayMusicStream（SDL_mixer_ext），回退到 Mix_PlayMusic
/// 对应 C++ SDLMusicInterface::PlayMusic 中的 Mix_PlayMusicStream
pub fn play_music(m: *mut Mix_Music, loops: c_int) -> c_int {
    unsafe {
        match MIX {
            Some(ref m_) => {
                if let Some(stream) = m_.play_music_stream {
                    (stream)(m, loops)
                } else {
                    (m_.play_music)(m, loops)
                }
            }
            None => -1,
        }
    }
}

pub fn halt_music() -> c_int { unsafe { match MIX { Some(ref m) => (m.halt_music)(), None => -1 } } }
pub fn volume_music(v: c_int) -> c_int {
    unsafe { match MIX { Some(ref m) => (m.volume_music)(v), None => 0 } }
}
pub fn pause_music() { unsafe { if let Some(ref m) = MIX { (m.pause_music)(); } } }
pub fn resume_music() { unsafe { if let Some(ref m) = MIX { (m.resume_music)(); } } }

/// 等价于 C++: SDL_RWFromConstMem(data, size)
pub unsafe fn rw_from_constmem(data: *const c_void, size: c_int) -> *mut c_void {
    let h = GetModuleHandleA("SDL2.dll\0".as_ptr() as *const i8);
    if h.is_null() { return std::ptr::null_mut(); }
    let p = GetProcAddress(h, "SDL_RWFromConstMem\0".as_ptr() as *const i8);
    if p.is_null() { return std::ptr::null_mut(); }
    type F = unsafe extern "C" fn(*const c_void, c_int) -> *mut c_void;
    let f: F = std::mem::transmute(p);
    f(data, size)
}
