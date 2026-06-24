// PvZ Portable Rust 翻译 — ModVal 动态常量修改系统
// 对应 C++ SexyAppFramework/misc/ModVal.h / ModVal.cpp
//
// 在 C++ 中通过 M() 宏允许运行时修改常量值。
// Rust 版本提供 `mod_val` 函数作为占位，在 RELEASE 模式下直接返回原值。

#![allow(dead_code)]

/// 运行时可修改的整数值（RELEASE 模式直接返回）
pub fn mod_val_int(_file: &str, val: i32) -> i32 { val }

/// 运行时可修改的双精度浮点值
pub fn mod_val_double(_file: &str, val: f64) -> f64 { val }

/// 运行时可修改的单精度浮点值
pub fn mod_val_float(_file: &str, val: f32) -> f32 { val }

/// 运行时可修改的字符串值
pub fn mod_val_str<'a>(_file: &str, val: &'a str) -> &'a str { val }

/// 重新解析 ModVal 源文件
pub fn reparse_mod_values() -> bool { false }

/// 添加 ModVal 枚举值
pub fn add_mod_val_enum(_enum_name: &str, _val: i32) {}
