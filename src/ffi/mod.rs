// PvZ Portable Rust 翻译 — FFI 模块
//
// 使用 extern "C" 直接声明 SDL2 和 OpenGL 的 C API。
// 不引入任何第三方 crate，完全通过标准 FFI 调用。
//
// 安全抽象层在上级模块中构建，此模块只做 C 接口声明。

#![allow(non_camel_case_types, dead_code, unused)]

pub mod sdl2;
pub mod opengl;
pub mod libc;
pub mod loader;

// 重新导出常用类型
pub use sdl2::*;
pub use opengl::*;
pub use loader::*;
