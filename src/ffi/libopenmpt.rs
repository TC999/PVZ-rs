// libopenmpt raw FFI — 运行时动态加载 libopenmpt.dll
// 提供 MO3/IT/XM/S3M/MOD 等模块格式解码能力
// 和 SDL_mixer FFI 一样，DLL 不存在时静默降级

#![allow(non_camel_case_types, dead_code)]

use std::os::raw::{c_int, c_uint, c_void, c_char, c_float};

// ============================================================
// 类型定义
// ============================================================

pub type openmpt_module = c_void;

// ============================================================
// 函数指针表
// ============================================================

struct OpenMPTFuncs {
    create_from_memory: unsafe extern "C" fn(data: *const c_void, size: usize, log_func: Option<unsafe extern "C" fn(*const c_char, *mut c_void)>, log_user: *mut c_void, ctls: *const *const c_char) -> *mut openmpt_module,
    destroy: unsafe extern "C" fn(mod_: *mut openmpt_module),
    /// openmpt_module_read_interleaved_stereo(mod, samplerate, count, buf)
    read_interleaved_stereo: unsafe extern "C" fn(mod_: *mut openmpt_module, sample_rate: c_int, count: usize, buf: *mut i16) -> usize,
    get_duration_seconds: unsafe extern "C" fn(mod_: *mut openmpt_module) -> c_float,
    get_num_channels: unsafe extern "C" fn(mod_: *mut openmpt_module) -> c_int,
    set_repeat_count: unsafe extern "C" fn(mod_: *mut openmpt_module, count: c_int) -> c_int,
}

static mut OPENMPT: Option<OpenMPTFuncs> = None;
static mut OPENMPT_DLL: Option<*mut c_void> = None;

#[cfg(windows)]
unsafe extern "system" {
    fn LoadLibraryA(lpLibFileName: *const i8) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const i8) -> *mut c_void;
    fn FreeLibrary(hLibModule: *mut c_void) -> c_int;
}

pub fn is_available() -> bool {
    unsafe { OPENMPT.is_some() }
}

pub fn load_library() -> bool {
    unsafe {
        if OPENMPT.is_some() { return true; }

        let dll_name = "libopenmpt.dll\0".as_ptr() as *const i8;
        let hmod = LoadLibraryA(dll_name);
        if hmod.is_null() { return false; }

        macro_rules! get {
            ($n:literal) => {{
                let p = GetProcAddress(hmod, concat!($n, "\0").as_ptr() as *const i8);
                if p.is_null() { FreeLibrary(hmod); return false; }
                std::mem::transmute::<*mut c_void, _>(p)
            }};
        }

        OPENMPT = Some(OpenMPTFuncs {
            create_from_memory: get!("openmpt_module_create_from_memory"),
            destroy: get!("openmpt_module_destroy"),
            read_interleaved_stereo: get!("openmpt_module_read_interleaved_stereo"),
            get_duration_seconds: get!("openmpt_module_get_duration_seconds"),
            get_num_channels: get!("openmpt_module_get_num_channels"),
            set_repeat_count: get!("openmpt_module_set_repeat_count"),
        });
        OPENMPT_DLL = Some(hmod);
        true
    }
}

// ============================================================
// 安全包装
// ============================================================

pub fn create_from_memory(data: &[u8]) -> *mut openmpt_module {
    unsafe {
        match OPENMPT {
            Some(ref f) => (f.create_from_memory)(data.as_ptr() as *const c_void, data.len(), None, std::ptr::null_mut(), std::ptr::null()),
            None => std::ptr::null_mut(),
        }
    }
}

pub fn destroy(mod_: *mut openmpt_module) {
    unsafe { if let Some(ref f) = OPENMPT { (f.destroy)(mod_); } }
}

/// 返回实际读取的样本数（立体声交错，i16 对）
pub fn read_interleaved_stereo(mod_: *mut openmpt_module, sample_rate: c_int, buf: &mut [i16]) -> usize {
    unsafe {
        match OPENMPT {
            Some(ref f) => (f.read_interleaved_stereo)(mod_, sample_rate, buf.len(), buf.as_mut_ptr()),
            None => 0,
        }
    }
}

pub fn get_duration_seconds(mod_: *mut openmpt_module) -> f32 {
    unsafe { match OPENMPT { Some(ref f) => (f.get_duration_seconds)(mod_), None => 0.0 } }
}

pub fn get_num_channels(mod_: *mut openmpt_module) -> c_int {
    unsafe { match OPENMPT { Some(ref f) => (f.get_num_channels)(mod_), None => 0 } }
}

pub fn set_repeat_count(mod_: *mut openmpt_module, count: c_int) {
    unsafe { if let Some(ref f) = OPENMPT { (f.set_repeat_count)(mod_, count); } }
}
