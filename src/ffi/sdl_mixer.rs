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

// Mix_Chunk 结构（用于直接操作）
#[repr(C)]
pub struct Mix_ChunkStruct {
    pub allocated: c_int,
    pub abuf: *mut u8,
    pub alen: u32,
    pub volume: u8,
}

// ============================================================
// 函数指针类型
// ============================================================

pub type Mix_OpenAudioFn = unsafe extern "C" fn(frequency: c_int, format: u16, channels: c_int, chunksize: c_int) -> c_int;
pub type Mix_CloseAudioFn = unsafe extern "C" fn();
pub type Mix_QuerySpecFn = unsafe extern "C" fn(frequency: *mut c_int, format: *mut u16, channels: *mut c_int) -> c_int;
pub type Mix_AllocateChannelsFn = unsafe extern "C" fn(numchannels: c_int) -> c_int;
pub type Mix_LoadWAV_RWFn = unsafe extern "C" fn(src: *mut c_void, freesrc: c_int) -> *mut Mix_Chunk;
pub type Mix_FreeChunkFn = unsafe extern "C" fn(chunk: *mut Mix_Chunk);
pub type Mix_PlayChannelTimedFn = unsafe extern "C" fn(channel: c_int, chunk: *mut Mix_Chunk, loops: c_int, ticks: c_int) -> c_int;
pub type Mix_HaltChannelFn = unsafe extern "C" fn(channel: c_int) -> c_int;
pub type Mix_VolumeChunkFn = unsafe extern "C" fn(chunk: *mut Mix_Chunk, volume: c_int) -> c_int;
pub type Mix_VolumeFn = unsafe extern "C" fn(channel: c_int, volume: c_int) -> c_int;
pub type Mix_SetPanningFn = unsafe extern "C" fn(channel: c_int, left: u8, right: u8) -> c_int;
pub type Mix_PlayingFn = unsafe extern "C" fn(channel: c_int) -> c_int;
pub type Mix_LoadMUSFn = unsafe extern "C" fn(file: *const c_char) -> *mut Mix_Music;
pub type Mix_FreeMusicFn = unsafe extern "C" fn(music: *mut Mix_Music);
pub type Mix_PlayMusicFn = unsafe extern "C" fn(music: *mut Mix_Music, loops: c_int) -> c_int;
pub type Mix_HaltMusicFn = unsafe extern "C" fn() -> c_int;
pub type Mix_VolumeMusicFn = unsafe extern "C" fn(volume: c_int) -> c_int;
pub type Mix_PausedMusicFn = unsafe extern "C" fn() -> c_int;
pub type Mix_ResumeMusicFn = unsafe extern "C" fn();
pub type Mix_PauseMusicFn = unsafe extern "C" fn();

// ============================================================
// 全局函数表（运行时加载）
// ============================================================

pub struct MixFunctions {
    pub open_audio: Mix_OpenAudioFn,
    pub close_audio: Mix_CloseAudioFn,
    pub query_spec: Mix_QuerySpecFn,
    pub allocate_channels: Mix_AllocateChannelsFn,
    pub load_wav_rw: Mix_LoadWAV_RWFn,
    pub free_chunk: Mix_FreeChunkFn,
    pub play_channel_timed: Mix_PlayChannelTimedFn,
    pub halt_channel: Mix_HaltChannelFn,
    pub volume_chunk: Mix_VolumeChunkFn,
    pub volume: Mix_VolumeFn,
    pub set_panning: Mix_SetPanningFn,
    pub playing: Mix_PlayingFn,
    pub load_mus: Mix_LoadMUSFn,
    pub free_music: Mix_FreeMusicFn,
    pub play_music: Mix_PlayMusicFn,
    pub halt_music: Mix_HaltMusicFn,
    pub volume_music: Mix_VolumeMusicFn,
    pub paused_music: Mix_PausedMusicFn,
    pub resume_music: Mix_ResumeMusicFn,
    pub pause_music: Mix_PauseMusicFn,
}

/// 全局 SDL_mixer 函数表
static mut G_MIX: Option<Box<MixFunctions>> = None;
static mut G_MIX_DLL: Option<*mut c_void> = None; // HMODULE
static mut G_MIX_INITIALIZED: bool = false;

/// 是否成功加载 SDL_mixer
pub fn is_mixer_available() -> bool {
    unsafe { G_MIX.is_some() }
}

/// 加载 SDL_mixer DLL 并获取所有函数指针
/// 返回 true 表示成功加载
pub fn load_mixer_library() -> bool {
    unsafe {
        if G_MIX.is_some() {
            return true;
        }

        // 尝试加载 SDL2_mixer.dll（或 SDL2_mixer_ext.dll）
        let dll_names = [
            "SDL2_mixer.dll\0".as_ptr() as *const i8,
            "SDL2_mixer_ext.dll\0".as_ptr() as *const i8,
        ];

        let mut hmod: *mut c_void = std::ptr::null_mut();
        for &name in &dll_names {
            hmod = LoadLibraryA(name);
            if !hmod.is_null() {
                break;
            }
        }

        if hmod.is_null() {
            return false;
        }

        // 获取所有函数指针
        macro_rules! load_fn {
            ($name:literal) => {{
                let ptr = GetProcAddress(hmod, concat!($name, "\0").as_ptr() as *const i8);
                if ptr.is_null() {
                    FreeLibrary(hmod);
                    return false;
                }
                std::mem::transmute::<*mut c_void, _>(ptr)
            }};
        }

        G_MIX = Some(Box::new(MixFunctions {
            open_audio: load_fn!("Mix_OpenAudio"),
            close_audio: load_fn!("Mix_CloseAudio"),
            query_spec: load_fn!("Mix_QuerySpec"),
            allocate_channels: load_fn!("Mix_AllocateChannels"),
            load_wav_rw: load_fn!("Mix_LoadWAV_RW"),
            free_chunk: load_fn!("Mix_FreeChunk"),
            play_channel_timed: load_fn!("Mix_PlayChannelTimed"),
            halt_channel: load_fn!("Mix_HaltChannel"),
            volume_chunk: load_fn!("Mix_VolumeChunk"),
            volume: load_fn!("Mix_Volume"),
            set_panning: load_fn!("Mix_SetPanning"),
            playing: load_fn!("Mix_Playing"),
            load_mus: load_fn!("Mix_LoadMUS"),
            free_music: load_fn!("Mix_FreeMusic"),
            play_music: load_fn!("Mix_PlayMusic"),
            halt_music: load_fn!("Mix_HaltMusic"),
            volume_music: load_fn!("Mix_VolumeMusic"),
            paused_music: load_fn!("Mix_PausedMusic"),
            resume_music: load_fn!("Mix_ResumeMusic"),
            pause_music: load_fn!("Mix_PauseMusic"),
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
        G_MIX_INITIALIZED = false;
    }
}

// ============================================================
// Win32 动态加载辅助函数
// ============================================================

#[cfg(windows)]
unsafe extern "system" {
    fn LoadLibraryA(lpLibFileName: *const i8) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const i8) -> *mut c_void;
    fn FreeLibrary(hLibModule: *mut c_void) -> c_int;
    fn GetModuleHandleA(lpModuleName: *const i8) -> *mut c_void;
}

// ============================================================
// 安全包装函数
// ============================================================

/// 初始化 SDL_mixer 音频子系统
pub fn mix_init(frequency: c_int, format: u16, channels: c_int, chunksize: c_int) -> c_int {
    unsafe {
        match G_MIX.as_ref() {
            Some(mix) => {
                let result = (mix.open_audio)(frequency, format, channels, chunksize);
                if result == 0 {
                    G_MIX_INITIALIZED = true;
                }
                result
            }
            None => -1,
        }
    }
}

/// 关闭音频
pub fn mix_close_audio() {
    unsafe {
        if G_MIX_INITIALIZED {
            if let Some(ref mix) = G_MIX {
                (mix.close_audio)();
            }
            G_MIX_INITIALIZED = false;
        }
    }
}

/// 分配声道数
pub fn mix_allocate_channels(num: c_int) -> c_int {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.allocate_channels)(num)
        } else {
            -1
        }
    }
}

/// 从 RWops 加载 WAV 音效
pub fn mix_load_wav_rw(src: *mut c_void, freesrc: c_int) -> *mut Mix_Chunk {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.load_wav_rw)(src, freesrc)
        } else {
            std::ptr::null_mut()
        }
    }
}

/// 释放音效块
pub fn mix_free_chunk(chunk: *mut Mix_Chunk) {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.free_chunk)(chunk);
        }
    }
}

/// 在指定声道播放音效
pub fn mix_play_channel_timed(channel: c_int, chunk: *mut Mix_Chunk, loops: c_int, ticks: c_int) -> c_int {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.play_channel_timed)(channel, chunk, loops, ticks)
        } else {
            -1
        }
    }
}

/// 停止指定声道
pub fn mix_halt_channel(channel: c_int) -> c_int {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.halt_channel)(channel)
        } else {
            -1
        }
    }
}

/// 设置音效块音量 (0-128)
pub fn mix_volume_chunk(chunk: *mut Mix_Chunk, volume: c_int) -> c_int {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.volume_chunk)(chunk, volume)
        } else {
            0
        }
    }
}

/// 设置声道音量
pub fn mix_volume(channel: c_int, volume: c_int) -> c_int {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.volume)(channel, volume)
        } else {
            0
        }
    }
}

/// 设置声道平衡
pub fn mix_set_panning(channel: c_int, left: u8, right: u8) -> c_int {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.set_panning)(channel, left, right)
        } else {
            -1
        }
    }
}

/// 检查声道是否正在播放
pub fn mix_playing(channel: c_int) -> c_int {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.playing)(channel)
        } else {
            0
        }
    }
}

/// 从文件加载音乐
pub fn mix_load_mus(file: *const c_char) -> *mut Mix_Music {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.load_mus)(file)
        } else {
            std::ptr::null_mut()
        }
    }
}

/// 释放音乐
pub fn mix_free_music(music: *mut Mix_Music) {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.free_music)(music);
        }
    }
}

/// 播放音乐 (loops=-1 无限循环)
pub fn mix_play_music(music: *mut Mix_Music, loops: c_int) -> c_int {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.play_music)(music, loops)
        } else {
            -1
        }
    }
}

/// 停止音乐
pub fn mix_halt_music() -> c_int {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.halt_music)()
        } else {
            -1
        }
    }
}

/// 设置音乐音量 (0-128)
pub fn mix_volume_music(volume: c_int) -> c_int {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.volume_music)(volume)
        } else {
            0
        }
    }
}

/// 检查音乐是否暂停
pub fn mix_paused_music() -> c_int {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.paused_music)()
        } else {
            0
        }
    }
}

/// 恢复音乐
pub fn mix_resume_music() {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.resume_music)();
        }
    }
}

/// 暂停音乐
pub fn mix_pause_music() {
    unsafe {
        if let Some(ref mix) = G_MIX {
            (mix.pause_music)();
        }
    }
}

// ============================================================
// SDL RWops 辅助（用于从内存加载音频）
// ============================================================

/// 从内存缓冲区创建 SDL_RWops（用于 Mix_LoadWAV_RW）
pub unsafe fn rw_from_mem(mem: *mut c_void, size: c_int) -> *mut c_void {
    // 使用 SDL_RWFromMem 从内存加载
    // 通过 GetProcAddress 从 SDL2.dll 获取
    type FnType = unsafe extern "C" fn(*mut c_void, c_int) -> *mut c_void;
    let ptr = get_sdl_function("SDL_RWFromMem\0".as_ptr() as *const i8);
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    let func: FnType = std::mem::transmute(ptr);
    func(mem, size)
}

/// 从 SDL2.dll 获取函数指针
fn get_sdl_function(name: *const i8) -> *mut c_void {
    #[cfg(windows)]
    unsafe {
        let sdl2_name = "SDL2.dll\0".as_ptr() as *const i8;
        let handle = GetModuleHandleA(sdl2_name);
        if handle.is_null() {
            return std::ptr::null_mut();
        }
        GetProcAddress(handle, name)
    }

    #[cfg(not(windows))]
    {
        std::ptr::null_mut()
    }
}
