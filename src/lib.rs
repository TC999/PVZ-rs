// PvZ Portable Rust 翻译
//
// 根模块 — 声明所有子模块

// 在游戏逻辑完全串联之前，允许以下常见警告
// 这些状态量在游戏完整运行时都会被使用
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(rust_2024_compatibility)]

pub mod ffi;
pub mod framework;
pub mod todlib;
pub mod lawn;
