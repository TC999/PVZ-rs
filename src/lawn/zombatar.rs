// PvZ Portable Rust 翻译 — Zombatar（僵尸头像定制系统）
// 对应 C++ src/Lawn/System/Zombatar.h / Zombatar.cpp 与 Zombie.cpp 中 ZombatarTrackName

#![allow(dead_code)]

use crate::framework::color::Color;

// ── Zombatar 记录槽位（对应 C++ ZombatarRecordSlot）─────────────────
pub const ZOMBATAR_SLOT_SKIN_PART: i32 = 0;
pub const ZOMBATAR_SLOT_SKIN_COLOR: i32 = 1;
pub const ZOMBATAR_SLOT_CLOTHES: i32 = 2;
pub const ZOMBATAR_SLOT_CLOTHES_COLOR: i32 = 3;
pub const ZOMBATAR_SLOT_TIDBITS: i32 = 4;
pub const ZOMBATAR_SLOT_TIDBITS_COLOR: i32 = 5;
pub const ZOMBATAR_SLOT_ACCESSORY: i32 = 6;
pub const ZOMBATAR_SLOT_ACCESSORY_COLOR: i32 = 7;
pub const ZOMBATAR_SLOT_FACIAL_HAIR: i32 = 8;
pub const ZOMBATAR_SLOT_FACIAL_HAIR_COLOR: i32 = 9;
pub const ZOMBATAR_SLOT_HAIR: i32 = 10;
pub const ZOMBATAR_SLOT_HAIR_COLOR: i32 = 11;
pub const ZOMBATAR_SLOT_EYEWEAR: i32 = 12;
pub const ZOMBATAR_SLOT_EYEWEAR_COLOR: i32 = 13;
pub const ZOMBATAR_SLOT_HATS: i32 = 14;
pub const ZOMBATAR_SLOT_HATS_COLOR: i32 = 15;
pub const ZOMBATAR_SLOT_BACKGROUND: i32 = 16;
pub const ZOMBATAR_SLOT_BACKGROUND_COLOR: i32 = 17;

// ── Zombatar 页面（对应 C++ ZombatarPage）────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ZombatarPage {
    Skin,
    Hair,
    FacialHair,
    Tidbits,
    Eyewear,
    Clothes,
    Accessory,
    Hats,
    Backdrops,
}

/// 部件布局（对应 C++ ZombatarPartLayout）
#[derive(Clone, Copy)]
pub struct ZombatarPartLayout {
    pub m_offset_x: i32,
    pub m_offset_y: i32,
    pub m_color_offset_x: i32,
    pub m_color_offset_y: i32,
    pub m_draw_order: i32,
}

/// 读取记录槽位 LE u32（对应 C++ ZombatarReadRecordSlot）
pub fn zombatar_read_record_slot(the_record: &[u8], the_slot: i32) -> u32 {
    let an_offset = (the_slot * 4) as usize;
    // [TRANSLATION_NOTE]: C++ 用 memcpy 读取，无边界检查；record 由调用方保证 >= ZOMBATAR_RECORD_SIZE(0x48)
    u32::from_le_bytes(the_record[an_offset..an_offset + 4].try_into().unwrap())
}

/// 读取有符号记录槽位（对应 C++ ZombatarReadSignedRecordSlot）
pub fn zombatar_read_signed_record_slot(the_record: &[u8], the_slot: i32) -> i32 {
    let a_value = zombatar_read_record_slot(the_record, the_slot);
    if a_value > i32::MAX as u32 {
        return -1;
    }
    a_value as i32
}

/// Zombatar 颜色表（对应 C++ gZombatarColors）
pub const G_ZOMBATAR_COLORS: [Color; 48] = [
    Color { r: 134, g: 147, b: 122, a: 255 }, Color { r: 79, g: 135, b: 94, a: 255 }, Color { r: 127, g: 135, b: 94, a: 255 }, Color { r: 120, g: 130, b: 50, a: 255 },
    Color { r: 156, g: 163, b: 105, a: 255 }, Color { r: 96, g: 151, b: 11, a: 255 }, Color { r: 147, g: 184, b: 77, a: 255 }, Color { r: 82, g: 143, b: 54, a: 255 },
    Color { r: 121, g: 168, b: 99, a: 255 }, Color { r: 65, g: 156, b: 74, a: 255 }, Color { r: 107, g: 178, b: 114, a: 255 }, Color { r: 104, g: 121, b: 90, a: 255 },
    Color { r: 151, g: 33, b: 33, a: 255 }, Color { r: 199, g: 53, b: 53, a: 255 }, Color { r: 220, g: 112, b: 47, a: 255 }, Color { r: 251, g: 251, b: 172, a: 255 },
    Color { r: 240, g: 210, b: 87, a: 255 }, Color { r: 165, g: 126, b: 65, a: 255 }, Color { r: 106, g: 72, b: 32, a: 255 }, Color { r: 72, g: 35, b: 5, a: 255 },
    Color { r: 50, g: 56, b: 61, a: 255 }, Color { r: 0, g: 0, b: 10, a: 255 }, Color { r: 197, g: 239, b: 239, a: 255 }, Color { r: 63, g: 109, b: 242, a: 255 },
    Color { r: 13, g: 202, b: 151, a: 255 }, Color { r: 158, g: 183, b: 19, a: 255 }, Color { r: 30, g: 210, b: 64, a: 255 }, Color { r: 225, g: 65, b: 230, a: 255 },
    Color { r: 128, g: 47, b: 204, a: 255 }, Color { r: 255, g: 255, b: 255, a: 255 }, Color { r: 238, g: 19, b: 24, a: 255 }, Color { r: 247, g: 89, b: 215, a: 255 },
    Color { r: 239, g: 198, b: 253, a: 255 }, Color { r: 160, g: 56, b: 241, a: 255 }, Color { r: 86, g: 74, b: 241, a: 255 }, Color { r: 74, g: 160, b: 241, a: 255 },
    Color { r: 199, g: 244, b: 251, a: 255 }, Color { r: 49, g: 238, b: 237, a: 255 }, Color { r: 16, g: 194, b: 66, a: 255 }, Color { r: 112, g: 192, b: 33, a: 255 },
    Color { r: 16, g: 145, b: 52, a: 255 }, Color { r: 248, g: 247, b: 41, a: 255 }, Color { r: 227, g: 180, b: 20, a: 255 }, Color { r: 241, g: 115, b: 25, a: 255 },
    Color { r: 248, g: 247, b: 175, a: 255 }, Color { r: 103, g: 85, b: 54, a: 255 }, Color { r: 159, g: 17, b: 20, a: 255 }, Color { r: 255, g: 255, b: 255, a: 255 },
];

/// 取 Zombatar 颜色（对应 C++ ZombatarGetColor）
pub fn zombatar_get_color(the_index: i32) -> Color {
    if the_index < 0 {
        return Color { r: 255, g: 255, b: 255, a: 255 };
    }
    let a_max_index = (G_ZOMBATAR_COLORS.len() - 1) as i32;
    G_ZOMBATAR_COLORS[the_index.min(a_max_index) as usize]
}

/// 配饰运行时重映射（对应 C++ ZombatarRemapAccessoryForRuntime）
pub fn zombatar_remap_accessory_for_runtime(the_index: i32) -> i32 {
    match the_index {
        5 => 14,
        6 => 5,
        7 => 6,
        8 => 12,
        9 => 7,
        10 => 9,
        11 => 10,
        12 => 11,
        14 => 8,
        _ => the_index,
    }
}

/// 轨道名拼接（对应 C++ ZombatarTrackName：StrFormat("%s%02d")）
pub fn zombatar_track_name(the_prefix: &str, the_index: i32) -> String {
    format!("{}{:02}", the_prefix, the_index)
}

// ── 部件布局表（对应 C++ Zombatar.cpp 各 constexpr 表）────────────────

pub const G_CLOTHES_LAYOUT: [ZombatarPartLayout; 12] = [
    ZombatarPartLayout { m_offset_x: 88, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
    ZombatarPartLayout { m_offset_x: 75, m_offset_y: 100, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
    ZombatarPartLayout { m_offset_x: 85, m_offset_y: 112, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
    ZombatarPartLayout { m_offset_x: 76, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
    ZombatarPartLayout { m_offset_x: 89, m_offset_y: 115, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
    ZombatarPartLayout { m_offset_x: 93, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
    ZombatarPartLayout { m_offset_x: 78, m_offset_y: 105, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
    ZombatarPartLayout { m_offset_x: 88, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
    ZombatarPartLayout { m_offset_x: 88, m_offset_y: 102, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
    ZombatarPartLayout { m_offset_x: 85, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
    ZombatarPartLayout { m_offset_x: 85, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
    ZombatarPartLayout { m_offset_x: 79, m_offset_y: 112, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 0 },
];

pub const G_TIDBITS_LAYOUT: [ZombatarPartLayout; 14] = [
    ZombatarPartLayout { m_offset_x: 28, m_offset_y: 63, m_color_offset_x: 18, m_color_offset_y: 48, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 28, m_offset_y: 63, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 46, m_offset_y: 111, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 31, m_offset_y: 62, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 31, m_offset_y: 58, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 28, m_offset_y: 66, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 28, m_offset_y: 72, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 33, m_offset_y: 55, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 21, m_offset_y: 76, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 36, m_offset_y: 71, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 36, m_offset_y: 70, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 86, m_offset_y: 91, m_color_offset_x: 0, m_color_offset_y: 6, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 88, m_offset_y: 50, m_color_offset_x: 0, m_color_offset_y: 8, m_draw_order: 2 },
    ZombatarPartLayout { m_offset_x: 113, m_offset_y: 115, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 2 },
];

pub const G_ACCESSORY_LAYOUT: [ZombatarPartLayout; 15] = [
    ZombatarPartLayout { m_offset_x: 103, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 108, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 86, m_offset_y: 113, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 131, m_offset_y: 95, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 131, m_offset_y: 100, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 131, m_offset_y: 100, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 104, m_offset_y: 111, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 118, m_offset_y: 65, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 61, m_offset_y: 118, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 43, m_offset_y: 100, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 135, m_offset_y: 92, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 78, m_offset_y: 130, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 68, m_offset_y: 145, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 133, m_offset_y: 70, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 4 },
    ZombatarPartLayout { m_offset_x: 13, m_offset_y: 40, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 10 },
];

pub const G_FACIAL_HAIR_LAYOUT: [ZombatarPartLayout; 24] = [
    ZombatarPartLayout { m_offset_x: 35, m_offset_y: 107, m_color_offset_x: 1, m_color_offset_y: 0, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 51, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 45, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 38, m_offset_y: 105, m_color_offset_x: 3, m_color_offset_y: 2, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 69, m_offset_y: 145, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 48, m_offset_y: 112, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 13, m_offset_y: 107, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 45, m_offset_y: 105, m_color_offset_x: 1, m_color_offset_y: 1, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 41, m_offset_y: 105, m_color_offset_x: 1, m_color_offset_y: 1, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 44, m_offset_y: 112, m_color_offset_x: 1, m_color_offset_y: 2, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 43, m_offset_y: 88, m_color_offset_x: 1, m_color_offset_y: 4, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 28, m_offset_y: 105, m_color_offset_x: 8, m_color_offset_y: 1, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 45, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 18, m_offset_y: 103, m_color_offset_x: 1, m_color_offset_y: 1, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 63, m_offset_y: 145, m_color_offset_x: 2, m_color_offset_y: 1, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 63, m_offset_y: 140, m_color_offset_x: 1, m_color_offset_y: 1, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 43, m_offset_y: 110, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 58, m_offset_y: 96, m_color_offset_x: 1, m_color_offset_y: 3, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 46, m_offset_y: 92, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 114, m_offset_y: 80, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 118, m_offset_y: 83, m_color_offset_x: 1, m_color_offset_y: 1, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 13, m_offset_y: 87, m_color_offset_x: 3, m_color_offset_y: 4, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 58, m_offset_y: 145, m_color_offset_x: 1, m_color_offset_y: 1, m_draw_order: 6 },
    ZombatarPartLayout { m_offset_x: 38, m_offset_y: 108, m_color_offset_x: 4, m_color_offset_y: 2, m_draw_order: 6 },
];

pub const G_HAIR_LAYOUT: [ZombatarPartLayout; 16] = [
    ZombatarPartLayout { m_offset_x: 23, m_offset_y: 0, m_color_offset_x: 8, m_color_offset_y: 1, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 23, m_offset_y: 25, m_color_offset_x: 2, m_color_offset_y: 3, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 23, m_offset_y: 30, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 30, m_offset_y: 15, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 36, m_offset_y: 37, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 39, m_offset_y: 13, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 51, m_offset_y: 22, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 28, m_offset_y: 15, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 128, m_offset_y: 55, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 22, m_offset_y: 32, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 25, m_offset_y: 19, m_color_offset_x: 2, m_color_offset_y: 2, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 51, m_offset_y: -5, m_color_offset_x: 2, m_color_offset_y: 2, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 33, m_offset_y: 13, m_color_offset_x: 2, m_color_offset_y: 2, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 9, m_offset_y: -2, m_color_offset_x: 1, m_color_offset_y: 5, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 45, m_offset_y: 4, m_color_offset_x: 0, m_color_offset_y: -1, m_draw_order: 8 },
    ZombatarPartLayout { m_offset_x: 26, m_offset_y: 20, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 8 },
];

pub const G_EYEWEAR_LAYOUT: [ZombatarPartLayout; 16] = [
    ZombatarPartLayout { m_offset_x: 28, m_offset_y: 73, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 31, m_offset_y: 85, m_color_offset_x: 0, m_color_offset_y: -1, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 28, m_offset_y: 69, m_color_offset_x: 0, m_color_offset_y: 1, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 28, m_offset_y: 78, m_color_offset_x: 0, m_color_offset_y: -1, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 30, m_offset_y: 75, m_color_offset_x: -1, m_color_offset_y: -1, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 30, m_offset_y: 78, m_color_offset_x: -1, m_color_offset_y: -1, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 50, m_offset_y: 90, m_color_offset_x: -1, m_color_offset_y: -1, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 32, m_offset_y: 70, m_color_offset_x: -1, m_color_offset_y: -1, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 36, m_offset_y: 100, m_color_offset_x: -1, m_color_offset_y: -1, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 31, m_offset_y: 75, m_color_offset_x: -1, m_color_offset_y: -1, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 31, m_offset_y: 67, m_color_offset_x: -1, m_color_offset_y: -1, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 38, m_offset_y: 95, m_color_offset_x: -1, m_color_offset_y: -1, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 30, m_offset_y: 81, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 35, m_offset_y: 64, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 42, m_offset_y: 65, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 10 },
    ZombatarPartLayout { m_offset_x: 35, m_offset_y: 65, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 10 },
];

pub const G_HATS_LAYOUT: [ZombatarPartLayout; 14] = [
    ZombatarPartLayout { m_offset_x: 28, m_offset_y: 5, m_color_offset_x: 2, m_color_offset_y: 1, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 47, m_offset_y: 12, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 36, m_offset_y: 20, m_color_offset_x: 15, m_color_offset_y: -1, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 11, m_offset_y: 10, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 41, m_offset_y: 16, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 18, m_offset_y: 3, m_color_offset_x: 4, m_color_offset_y: -2, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 53, m_offset_y: 17, m_color_offset_x: 0, m_color_offset_y: 15, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 3, m_offset_y: 0, m_color_offset_x: 0, m_color_offset_y: -2, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 38, m_offset_y: 0, m_color_offset_x: -1, m_color_offset_y: -2, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 13, m_offset_y: 45, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 63, m_offset_y: 8, m_color_offset_x: 1, m_color_offset_y: 14, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 43, m_offset_y: 15, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 18, m_offset_y: 0, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 12 },
    ZombatarPartLayout { m_offset_x: 23, m_offset_y: 5, m_color_offset_x: 0, m_color_offset_y: 0, m_draw_order: 12 },
];

/// 取部件布局（对应 C++ GetPartLayout；C++ 返回指针，nullptr 用 Option 表示）
pub fn get_part_layout(the_page: ZombatarPage, the_index: i32) -> Option<&'static ZombatarPartLayout> {
    match the_page {
        ZombatarPage::Clothes => {
            if the_index < G_CLOTHES_LAYOUT.len() as i32 { Some(&G_CLOTHES_LAYOUT[the_index as usize]) } else { None }
        }
        ZombatarPage::Tidbits => {
            if the_index < G_TIDBITS_LAYOUT.len() as i32 { Some(&G_TIDBITS_LAYOUT[the_index as usize]) } else { None }
        }
        ZombatarPage::Accessory => {
            if the_index < G_ACCESSORY_LAYOUT.len() as i32 { Some(&G_ACCESSORY_LAYOUT[the_index as usize]) } else { None }
        }
        ZombatarPage::FacialHair => {
            // C++：aIdx > 16 时按 aIdx -= aIdx / 17 紧凑映射
            let mut a_idx = the_index;
            if a_idx > 16 {
                a_idx -= a_idx / 17;
            }
            if a_idx < G_FACIAL_HAIR_LAYOUT.len() as i32 { Some(&G_FACIAL_HAIR_LAYOUT[a_idx as usize]) } else { None }
        }
        ZombatarPage::Hair => {
            if the_index < G_HAIR_LAYOUT.len() as i32 { Some(&G_HAIR_LAYOUT[the_index as usize]) } else { None }
        }
        ZombatarPage::Eyewear => {
            if the_index < G_EYEWEAR_LAYOUT.len() as i32 { Some(&G_EYEWEAR_LAYOUT[the_index as usize]) } else { None }
        }
        ZombatarPage::Hats => {
            if the_index < G_HATS_LAYOUT.len() as i32 { Some(&G_HATS_LAYOUT[the_index as usize]) } else { None }
        }
        ZombatarPage::Skin | ZombatarPage::Backdrops => None,
    }
}