// SDL_mixer FFI 绑定 — 通过 sdl2::sys::mixer 模块编译时链接
//
// Windows: SDL2_mixer.lib 由 build.rs 从 dev 包提供
// Linux: 通过 pkg-config / -lSDL2_mixer 链接系统库
//
// 保持与原有 sdl_mixer.rs 相同的函数签名（参数为 *mut c_void），
// 内部转换为 sdl2-sys 的具体类型，避免调用方大量修改

#![allow(non_camel_case_types, dead_code)]

use std::os::raw::{c_int, c_uchar, c_void, c_char};

// 为向后兼容保留的类型别名（调用方以 *mut c_void 方式使用）
// sdl2-sys 中的实际类型是 mixer::Mix_Chunk (struct) 和 _Mix_Music (opaque)

pub const MAX_VOLUME: i32 = 128;

// ============ 编译时链接的 SDL2_mixer 函数 ============
// 所有内部转换均基于 sdl2::sys::mixer 模块

pub fn open_audio(freq: c_int, fmt: u16, ch: c_int, size: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_OpenAudio(freq, fmt, ch, size) }
}

pub fn close_audio() {
    unsafe { sdl2::sys::mixer::Mix_CloseAudio() }
}

pub fn allocate_channels(n: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_AllocateChannels(n) }
}

pub fn load_wav_rw(rw: *mut c_void, freesrc: c_int) -> *mut c_void {
    unsafe {
        sdl2::sys::mixer::Mix_LoadWAV_RW(rw as *mut sdl2::sys::SDL_RWops, freesrc) as *mut c_void
    }
}

pub fn free_chunk(c: *mut c_void) {
    unsafe { sdl2::sys::mixer::Mix_FreeChunk(c as *mut sdl2::sys::mixer::Mix_Chunk) }
}

pub fn play_channel(ch: c_int, ck: *mut c_void, loops: c_int) -> c_int {
    unsafe {
        sdl2::sys::mixer::Mix_PlayChannelTimed(ch, ck as *mut sdl2::sys::mixer::Mix_Chunk, loops, -1)
    }
}

pub fn halt_channel(ch: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_HaltChannel(ch) }
}

pub fn volume(ch: c_int, vol: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_Volume(ch, vol) }
}

pub fn set_panning(ch: c_int, l: u8, r: u8) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_SetPanning(ch, l, r) }
}

pub fn load_mus(path: *const c_char) -> *mut c_void {
    unsafe { sdl2::sys::mixer::Mix_LoadMUS(path) as *mut c_void }
}

pub fn free_music(m: *mut c_void) {
    unsafe { sdl2::sys::mixer::Mix_FreeMusic(m as *mut sdl2::sys::mixer::Mix_Music) }
}

pub fn play_music(m: *mut c_void, loops: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_PlayMusic(m as *mut sdl2::sys::mixer::Mix_Music, loops) }
}

pub fn halt_music() -> c_int {
    unsafe { sdl2::sys::mixer::Mix_HaltMusic() }
}

pub fn volume_music(v: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_VolumeMusic(v) }
}

pub fn pause_music() {
    unsafe { sdl2::sys::mixer::Mix_PauseMusic() }
}

pub fn resume_music() {
    unsafe { sdl2::sys::mixer::Mix_ResumeMusic() }
}

/// SDL_RWFromConstMem — 通过 sdl2-sys 编译时链接
pub unsafe fn rw_from_constmem(data: *const c_void, size: c_int) -> *mut c_void {
    sdl2::sys::SDL_RWFromConstMem(data, size) as *mut c_void
}

/// 总是可用（编译时链接不存在运行时加载问题）
pub fn is_available() -> bool {
    true
}

/// 不再需要运行时加载
pub fn load_library() -> bool {
    true
}