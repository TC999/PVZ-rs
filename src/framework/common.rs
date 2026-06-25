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

impl std::ops::Add for SexyVector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        SexyVector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Sub for SexyVector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        SexyVector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

// ============================================================
// UTF8 编解码（对应 C++ Common.h UTF8DecodeNext）
// ============================================================

/// 从 UTF-8 字符串中解码下一个码点（对应 C++ UTF8DecodeNext）
/// 成功返回 true 并更新 offset，失败返回 false
pub fn utf8_decode_next(s: &[u8], offset: &mut usize) -> Option<char> {
    if *offset >= s.len() {
        return None;
    }

    let first = s[*offset];
    if first < 0x80 {
        *offset += 1;
        return Some(first as char);
    }

    let length = if (first & 0xE0) == 0xC0 {
        2usize
    } else if (first & 0xF0) == 0xE0 {
        3
    } else if (first & 0xF8) == 0xF0 {
        4
    } else {
        return None; // 无效前缀字节
    };

    if *offset + length > s.len() {
        return None;
    }

    // 验证后续字节都是 10xxxxxx 格式
    for i in 1..length {
        if (s[*offset + i] & 0xC0) != 0x80 {
            return None;
        }
    }

    let value: u32 = match length {
        2 => ((first as u32 & 0x1F) << 6) | (s[*offset + 1] as u32 & 0x3F),
        3 => {
            ((first as u32 & 0x0F) << 12)
                | ((s[*offset + 1] as u32 & 0x3F) << 6)
                | (s[*offset + 2] as u32 & 0x3F)
        }
        4 => {
            ((first as u32 & 0x07) << 18)
                | ((s[*offset + 1] as u32 & 0x3F) << 12)
                | ((s[*offset + 2] as u32 & 0x3F) << 6)
                | (s[*offset + 3] as u32 & 0x3F)
        }
        _ => return None,
    };

    *offset += length;
    char::from_u32(value)
}

/// 从 UTF-8 字符串中解码下一个码点（std::string 版本，对应 C++ UTF8DecodeNext(string)）
pub fn utf8_decode_next_str(s: &str, offset: &mut usize) -> Option<char> {
    utf8_decode_next(s.as_bytes(), offset)
}

/// 判断是否为开头标点（对应 C++ IsOpeningPunctuation）
pub fn is_opening_punctuation(c: char) -> bool {
    matches!(
        c,
        '〈' | '《' | '「' | '『' | '【' | '〔' | '〖' | '〘' | '〚'
            | '（' | '［' | '｛'
            | '\u{2018}' | '\u{201A}' | '\u{201B}' | '\u{201C}'
    )
}

/// 判断是否为结尾标点（对应 C++ IsClosingPunctuation）
pub fn is_closing_punctuation(c: char) -> bool {
    matches!(
        c,
        '〉' | '》' | '」' | '』' | '】' | '〕' | '〗' | '〙' | '〛'
            | '）' | '］' | '｝'
            | '\u{2019}' | '\u{201D}'
            | '、' | '。'
            | '，' | '．'
            | '！' | '？'
            | '：' | '；'
    )
}

/// 判断是否可自动换行（对应 C++ IsAutoBreakChar）
pub fn is_auto_break_char(c: char) -> bool {
    let v = c as u32;
    if v < 0x80 {
        return false;
    }
    (v >= 0x2018 && v <= 0x201D)
        || (v >= 0x2600 && v <= 0x27BF)
        || (v >= 0x3000 && v <= 0x303F)
        || (v >= 0x3040 && v <= 0x309F)
        || (v >= 0x30A0 && v <= 0x30FF)
        || (v >= 0x3400 && v <= 0x4DBF)
        || (v >= 0x4E00 && v <= 0x9FFF)
        || (v >= 0xAC00 && v <= 0xD7AF)
        || (v >= 0xF900 && v <= 0xFAFF)
        || (v >= 0xFE30 && v <= 0xFE4F)
        || (v >= 0xFF01 && v <= 0xFF60)
        || (v >= 0x1F300 && v <= 0x1FAFF)
        || (v >= 0x20000 && v <= 0x2FA1F)
}

/// 将字符串转为整数（对应 C++ StringToInt）
pub fn string_to_int(s: &str, val: &mut i32) -> bool {
    if let Ok(v) = s.trim().parse::<i32>() {
        *val = v;
        true
    } else {
        false
    }
}

/// 将字符串转为浮点数（对应 C++ StringToDouble）
pub fn string_to_double(s: &str, val: &mut f64) -> bool {
    if let Ok(v) = s.trim().parse::<f64>() {
        *val = v;
        true
    } else {
        false
    }
}

/// XML 解码字符串（对应 C++ XMLDecodeString）——简单实现
pub fn xml_decode_string(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

/// XML 编码字符串（对应 C++ XMLEncodeString）——简单实现
pub fn xml_encode_string(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
