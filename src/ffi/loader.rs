// PvZ Portable Rust 翻译 — FFI 辅助函数

use crate::ffi::opengl::GLADloadfunc;
use std::ffi::CStr;

/// OpenGL 函数加载器（对接 SDL_GL_GetProcAddress）
pub unsafe extern "C" fn opengl_load_proc(name: *const std::os::raw::c_char) -> *mut std::os::raw::c_void {
    let c_str = CStr::from_ptr(name);
    let func_name = c_str.to_str().unwrap_or("");
    // 使用 SDL 加载 OpenGL 函数指针
    // 注意：此处依赖 SDL2 的动态库
    unsafe extern "C" {
        fn SDL_GL_GetProcAddress(proc: *const std::os::raw::c_char) -> *mut std::os::raw::c_void;
    }
    SDL_GL_GetProcAddress(name)
}
