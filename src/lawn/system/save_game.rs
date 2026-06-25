// 游戏存档系统 — 对应 C++ src/Lawn/System/SaveGame.h / SaveGame.cpp

#![allow(dead_code)]

use crate::lawn::board::Board;

/// 加载游戏存档（对应 C++ LawnLoadGame）
pub fn lawn_load_game(board: Option<*mut Board>, file_path: &str) -> bool {
    // TODO: 从 SaveGame.cpp 翻译（3115行，TLV 序列化系统）
    false
}

/// 保存游戏存档（对应 C++ LawnSaveGame）
pub fn lawn_save_game(board: Option<*mut Board>, file_path: &str) -> bool {
    // TODO: 从 SaveGame.cpp 翻译
    false
}
