// PvZ Portable Rust 翻译 — RegEmu 注册表模拟
// 对应 C++ SexyAppFramework/misc/RegEmu.h / RegEmu.cpp
// C++ 为「简单 Windows 注册表模拟器」：内存两级 map（键名→值名→值），
// 以 REGEMU 二进制文件持久化（SaveToFile/SetRegFile，RegEmu.cpp:48-151）。

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::io::{Read as IoRead, Write as IoWrite};

/// 注册表值类型枚举（C++ regemu::REGEMU_*，RegEmu.h）
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

/// REGEMU_VERSION（C++ RegEmu.cpp:36）
const REGEMU_VERSION: u16 = 1;

/// 单个注册表值（对应 C++ struct RegValue，RegEmu.cpp:38-43）
#[derive(Default, Clone)]
struct RegValue {
    m_type: u32,
    m_length: u32,
    m_value: Vec<u8>,
}

/// 键名 →（值名 → 值）的有序映射
/// 对应 C++ typedef std::map<std::string, std::map<std::string, RegValue>> RegContents；
/// 用 BTreeMap 保持与 std::map 一致的键序（文件序列化顺序确定）。
type RegContents = BTreeMap<String, BTreeMap<String, RegValue>>;

static mut REGISTRY: Option<RegContents> = None;
static mut CURR_FILE: Option<String> = None;

fn registry() -> &'static mut RegContents {
    unsafe {
        if REGISTRY.is_none() {
            REGISTRY = Some(RegContents::new());
        }
        REGISTRY.as_mut().unwrap()
    }
}

fn curr_file() -> Option<String> {
    unsafe { CURR_FILE.clone() }
}

/// 写入持久化文件（对应 C++ SaveToFile，RegEmu.cpp:48-92）
/// 格式："REGEMU" + u16 版本 + u32 键数 +（每个键：u32 键名长（含 NUL）+ 键名 NUL 结尾 + u32 值数 +
/// （每个值：u32 值名长（含 NUL）+ 值名 NUL 结尾 + u32 类型 + u32 长度 + 数据））
fn save_to_file() {
    let Some(a_file_name) = curr_file() else {
        eprintln!("RegEmu: Filename not specified, can't save");
        return;
    };
    let Ok(mut f) = std::fs::File::create(&a_file_name) else {
        eprintln!("RegEmu: Couldn't open '{}' for writing", a_file_name);
        return;
    };

    let a_registry = registry();
    let _ = f.write_all(b"REGEMU");
    let _ = f.write_all(&REGEMU_VERSION.to_le_bytes());
    let _ = f.write_all(&(a_registry.len() as u32).to_le_bytes());

    for (a_key_name, a_values) in a_registry.iter() {
        // C++ 73-75: 键名长度含结尾 NUL
        let _ = f.write_all(&((a_key_name.len() + 1) as u32).to_le_bytes());
        let _ = f.write_all(a_key_name.as_bytes());
        let _ = f.write_all(&[0u8]);
        let _ = f.write_all(&(a_values.len() as u32).to_le_bytes());
        for (a_value_name, a_value) in a_values.iter() {
            let _ = f.write_all(&((a_value_name.len() + 1) as u32).to_le_bytes());
            let _ = f.write_all(a_value_name.as_bytes());
            let _ = f.write_all(&[0u8]);
            let _ = f.write_all(&a_value.m_type.to_le_bytes());
            let _ = f.write_all(&a_value.m_length.to_le_bytes());
            let _ = f.write_all(&a_value.m_value);
        }
    }
}

/// 设置注册表文件路径并读取已有内容（对应 C++ SetRegFile，RegEmu.cpp:94-151）
pub fn set_reg_file(file_name: &str) {
    unsafe {
        CURR_FILE = Some(file_name.to_string());
    }
    let a_registry = registry();
    a_registry.clear();

    let Ok(mut f) = std::fs::File::open(file_name) else {
        eprintln!("RegEmu: Can't read '{}': File does not exist", file_name);
        return;
    };
    let mut a_data: Vec<u8> = Vec::new();
    if f.read_to_end(&mut a_data).is_err() {
        return;
    }

    // C++ 106-111: 校验 "REGEMU" 头
    if a_data.len() < 8 || &a_data[0..6] != b"REGEMU" {
        eprintln!("RegEmu: Can't read '{}': Invalid header", file_name);
        return;
    }

    // 简单游标读取器（C++ 用 ifstream 逐字段读，失败即 return）
    let mut a_pos: usize = 6;
    macro_rules! read_u16 {
        () => {{
            if a_pos + 2 > a_data.len() {
                return;
            }
            let v = u16::from_le_bytes([a_data[a_pos], a_data[a_pos + 1]]);
            a_pos += 2;
            v
        }};
    }
    macro_rules! read_u32 {
        () => {{
            if a_pos + 4 > a_data.len() {
                return;
            }
            let v = u32::from_le_bytes([
                a_data[a_pos],
                a_data[a_pos + 1],
                a_data[a_pos + 2],
                a_data[a_pos + 3],
            ]);
            a_pos += 4;
            v
        }};
    }
    macro_rules! read_bytes {
        ($n: expr) => {{
            if a_pos + $n > a_data.len() {
                return;
            }
            let v = a_data[a_pos..a_pos + $n].to_vec();
            a_pos += $n;
            v
        }};
    }

    let _a_version = read_u16!();
    let a_num_keys = read_u32!();
    for _i in 0..a_num_keys {
        let a_key_name_len = read_u32!() as usize;
        let mut a_key_name = read_bytes!(a_key_name_len);
        // 去掉结尾 NUL（C++ 用 vector<char> 隐式以 NUL 截断）
        while a_key_name.last() == Some(&0) {
            a_key_name.pop();
        }
        let a_key = String::from_utf8_lossy(&a_key_name).into_owned();
        a_registry.insert(a_key.clone(), BTreeMap::new());

        let a_num_values = read_u32!();
        for _j in 0..a_num_values {
            let mut a_value: RegValue = RegValue::default();
            let a_value_name_len = read_u32!() as usize;
            let mut a_value_name = read_bytes!(a_value_name_len);
            while a_value_name.last() == Some(&0) {
                a_value_name.pop();
            }
            a_value.m_type = read_u32!();
            a_value.m_length = read_u32!();
            a_value.m_value = read_bytes!(a_value.m_length as usize);
            a_registry
                .get_mut(&a_key)
                .unwrap()
                .insert(String::from_utf8_lossy(&a_value_name).into_owned(), a_value);
        }
    }

    eprintln!(
        "RegEmu: Loaded from '{}': {} total key(s)",
        file_name,
        a_registry.len()
    );
}

/// 读取注册表值（对应 C++ RegistryRead，RegEmu.cpp:153-170）
pub fn registry_read(
    key: &str,
    value: &str,
    type_: &mut u32,
    value_out: &mut [u8],
    length: &mut u32,
) -> bool {
    let a_registry = registry();
    let Some(a_values) = a_registry.get(key) else {
        eprintln!("RegEmu: Key '{}' does not exist", key);
        return false;
    };
    let Some(a_reg_value) = a_values.get(value) else {
        eprintln!("RegEmu: Value '{}' does not exist", value);
        return false;
    };

    *type_ = a_reg_value.m_type;
    *length = a_reg_value.m_length;
    // C++ 168: memcpy(value, ..., mLength)（调用方保证缓冲区足够）
    let a_copy_len = (a_reg_value.m_length as usize).min(value_out.len());
    value_out[..a_copy_len].copy_from_slice(&a_reg_value.m_value[..a_copy_len]);
    true
}

/// 写入注册表值（对应 C++ RegistryWrite，RegEmu.cpp:172-188）
pub fn registry_write(key: &str, value: &str, type_: u32, data: &[u8], length: u32) -> bool {
    let a_registry = registry();
    let a_entry = a_registry.entry(key.to_string()).or_default();
    let mut a_reg_value = RegValue {
        m_type: type_,
        m_length: length,
        m_value: Vec::new(),
    };
    // C++ 180-181: resize(length) + memcpy（data 实际长度可能小于 length）
    let a_copy_len = (length as usize).min(data.len());
    a_reg_value.m_value.extend_from_slice(&data[..a_copy_len]);
    a_reg_value.m_value.resize(length as usize, 0);
    a_entry.insert(value.to_string(), a_reg_value);

    save_to_file();
    true
}

/// 删除注册表键（对应 C++ RegistryEraseKey，RegEmu.cpp:190-201）
pub fn registry_erase_key(key: &str) -> bool {
    let a_registry = registry();
    if a_registry.remove(key).is_none() {
        return false;
    }
    eprintln!("RegEmu: Erased key '{}'", key);
    save_to_file();
    true
}

/// 删除注册表值（对应 C++ RegistryEraseValue，RegEmu.cpp:203-214）
pub fn registry_erase_value(key: &str, value: &str) -> bool {
    let a_registry = registry();
    let Some(a_values) = a_registry.get_mut(key) else {
        return false;
    };
    if a_values.remove(value).is_none() {
        return false;
    }
    eprintln!("RegEmu: Erased value '{}' from key '{}'", value, key);
    save_to_file();
    true
}
