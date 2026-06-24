// PvZ Portable Rust 翻译 — 框架公共模块
// 对应 C++ SexyAppFramework/Common.h

#![allow(dead_code, non_camel_case_types)]

use std::collections::HashMap;
use std::sync::atomic::AtomicBool;

// ============================================================
// 类型别名
// ============================================================

pub type uchar = u8;
pub type ushort = u16;
pub type uint = u32;
pub type ulong = u32;
pub type int64 = i64;

pub type DefinesMap = HashMap<String, String>;
pub type CharVector = Vec<u8>;

// ============================================================
// 全局标志
// ============================================================

pub static mut gDebug: AtomicBool = AtomicBool::new(false);

// ============================================================
// 随机数（使用 MTRand 梅森旋转算法，对应 C++ Common 中的 Rand/SRand）
// ============================================================

pub const SEXY_RAND_MAX: ulong = 0x7FFFFFFF;

use crate::framework::mt_rand::MTRand;

/// 全局随机数生成器（对应 C++ 中的 gMTRand）
static mut G_MT_RAND: Option<MTRand> = None;

/// 获取全局 MTRand 实例
fn get_global_mt_rand() -> &'static mut MTRand {
    unsafe {
        if G_MT_RAND.is_none() {
            G_MT_RAND = Some(MTRand::from_seed(4357));
        }
        G_MT_RAND.as_mut().unwrap()
    }
}

/// 返回 [0, SEXY_RAND_MAX] 范围内的随机整数（对应 C++ Sexy::Rand()）
pub fn Rand() -> i32 {
    get_global_mt_rand().next() as i32
}

/// 返回 [0, range) 范围内的随机整数（对应 C++ Sexy::Rand(int)）
pub fn RandRange(range: i32) -> i32 {
    if range <= 0 {
        return 0;
    }
    (get_global_mt_rand().next() % range as u32) as i32
}

/// 返回 [0, range) 范围内的随机浮点数（对应 C++ Sexy::Rand(float)）
pub fn RandFloat(range: f32) -> f32 {
    get_global_mt_rand().next_float(range)
}

/// 设置种子（对应 C++ Sexy::SRand(ulong)）
pub fn SRand(seed: u32) {
    let s = if seed == 0 { 4357 } else { seed };
    get_global_mt_rand().srand(s);
}

static mut gRandSeed: u32 = 0;

pub fn rand() -> i32 {
    unsafe {
        gRandSeed = gRandSeed.wrapping_mul(0x5D588B65).wrapping_add(0x269EC3) & 0x7FFFFFFF;
        gRandSeed as i32
    }
}

pub fn rand_range(range: i32) -> i32 {
    if range <= 0 { return 0; }
    rand() % range
}

pub fn rand_float(range: f32) -> f32 {
    if range <= 0.0 { return 0.0; }
    (rand() as f32 / SEXY_RAND_MAX as f32) * range
}

pub fn srand(seed: u32) {
    unsafe {
        gRandSeed = seed;
    }
}

// ============================================================
// 字符串工具函数
// ============================================================

/// 去除字符串两端空白
pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

/// 左侧去除空白（原地修改）
pub fn inline_ltrim(s: &mut String, chars: &str) {
    let trim_len = s.len() - s.trim_start_matches(|c: char| chars.contains(c)).len();
    s.drain(..trim_len);
}

/// 右侧去除空白（原地修改）
pub fn inline_rtrim(s: &mut String, chars: &str) {
    let trimmed = s.trim_end_matches(|c: char| chars.contains(c)).len();
    s.truncate(trimmed);
}

/// 两侧去除空白（原地修改）
pub fn inline_trim(s: &mut String, chars: &str) {
    inline_rtrim(s, chars);
    inline_ltrim(s, chars);
}

pub fn string_to_upper(s: &str) -> String {
    s.to_uppercase()
}

pub fn string_to_lower(s: &str) -> String {
    s.to_lowercase()
}

pub fn comma_separate(value: i32) -> String {
    let s = value.to_string();
    let mut result = String::new();
    let mut count = 0;
    for c in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }
    result.chars().rev().collect()
}

// ============================================================
// 文件系统工具
// ============================================================

/// 检查文件是否存在
pub fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

/// 获取文件名（不含目录）
pub fn get_file_name(path: &str, no_extension: bool) -> String {
    let p = std::path::Path::new(path);
    if no_extension {
        p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default()
    } else {
        p.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default()
    }
}

/// 获取文件所在目录
pub fn get_file_dir(path: &str, with_slash: bool) -> String {
    let p = std::path::Path::new(path);
    let parent = p.parent().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    if with_slash && !parent.is_empty() {
        parent + "/"
    } else {
        parent
    }
}

/// 获取当前工作目录
pub fn get_cur_dir() -> String {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}

// ============================================================
// 字节序工具
// ============================================================

#[inline]
pub const fn byte_swap16(v: u16) -> u16 {
    (v >> 8) | (v << 8)
}

#[inline]
pub const fn byte_swap32(v: u32) -> u32 {
    ((v & 0x000000FF) << 24) |
    ((v & 0x0000FF00) << 8) |
    ((v & 0x00FF0000) >> 8) |
    ((v & 0xFF000000) >> 24)
}

#[inline]
pub const fn byte_swap64(v: u64) -> u64 {
    ((v & 0x00000000000000FF) << 56) |
    ((v & 0x000000000000FF00) << 40) |
    ((v & 0x0000000000FF0000) << 24) |
    ((v & 0x00000000FF000000) << 8) |
    ((v & 0x000000FF00000000) >> 8) |
    ((v & 0x0000FF0000000000) >> 24) |
    ((v & 0x00FF000000000000) >> 40) |
    ((v & 0xFF00000000000000) >> 56)
}

// ============================================================
// 2D 向量类型（SexyVector2，用于矩阵变换）
// ============================================================

/// 2D 浮点向量（对应 C++ SexyVector2）
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct SexyVector2 {
    pub x: f32,
    pub y: f32,
}

impl SexyVector2 {
    pub const ZERO: SexyVector2 = SexyVector2 { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        SexyVector2 { x, y }
    }
}
