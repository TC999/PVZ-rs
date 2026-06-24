// PvZ Portable Rust 翻译 — FFI 模块
//
// OpenGL 函数通过 sdl2 crate 中的 SDL_GL_GetProcAddress 运行时加载。

#![allow(non_camel_case_types, dead_code, unused)]

pub mod opengl;
pub mod libc;
pub mod sdl_mixer;
pub mod libopenmpt;

// 重新导出常用类型
pub use opengl::*;
