// PvZ Portable Rust 翻译 — RegEmu 注册表模拟
// 对应 C++ SexyAppFramework/misc/RegEmu.h / RegEmu.cpp

#![allow(dead_code)]

/// 注册表值类型枚举
pub const REGEMU_NONE: u32 = 0;
pub const REGEMU_SZ: u32 = 1;
pub const REGEMU_EXPAND_SZ: u32 = 2;
pub const REGEMU_BINARY: u32 = 3;
pub const REGEMU_DWORD: u32 = 4;
pub const REGEMU_DWORD_LITTLE_ENDIAN: u32 = 4;
pub const REGEMU_DWORD_BIG_ENDIAN: u32 = 5;
pub const REGEMU_MULTI_SZ: u32 = 7;
pub const REGEMU_QWORD: u32 = 11;
pub const REGEMU_QWORD_LITTLE_ENDIAN: u32 = 11;

/// 设置注册表文件路径（对应 C++ SetRegFile）
pub fn set_reg_file(_file_name: &str) {}

/// 读取注册表值（对应 C++ RegistryRead）
pub fn registry_read(_key: &str, _value: &str, _type_: &mut u32, _value_out: &mut [u8], _length: &mut u32) -> bool { false }

/// 写入注册表值（对应 C++ RegistryWrite）
pub fn registry_write(_key: &str, _value: &str, _type_: u32, _data: &[u8], _length: u32) -> bool { false }

/// 删除注册表键（对应 C++ RegistryEraseKey）
pub fn registry_erase_key(_key: &str) -> bool { false }

/// 删除注册表值（对应 C++ RegistryEraseValue）
pub fn registry_erase_value(_key: &str, _value: &str) -> bool { false }
