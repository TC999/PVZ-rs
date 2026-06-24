// PvZ Portable Rust 翻译 — PakLib（PAK 资源包读取模块）
// 对应 C++ src/SexyAppFramework/paklib/PakInterface.h / PakInterface.cpp
//
// 完整实现 PAK 资源包文件的读取：
// - main.pak 的解析与加载
// - 包内文件的虚拟文件操作（FOpen/FRead/FSeek/FTell/FClose/FEof/FGetC/UnGetC/FGetS）
// - 路径规范化（大小写不敏感、正斜杠统一、相对路径化）
// - 文件系统回退：如果在 PAK 中找不到，尝试读取真实文件
//
// PAK 二进制格式：
//   [4B Magic: 0xBAC04AC0 (LE)]
//   [4B Version: 0 (LE)]
//   [目录条目列表，每项格式：]
//     [1B flags (0x80 = 终止)] [1B nameWidth] [nameWidth B name] [4B srcSize LE] [8B fileTime LE]
//   [文件数据区]
//   整个文件（包括目录和数据）以 0xF7 XOR 加密

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::Mutex;

// ============================================================
// 常量
// ============================================================

/// PAK 魔数（小端序）
const PAK_MAGIC: u32 = 0xBAC04AC0;
/// 当前支持的 PAK 版本
const PAK_VERSION: u32 = 0;
/// 文件头标志：目录条目终止
const FILEFLAGS_END: u8 = 0x80;
/// 整个 PAK 文件的 XOR 解密密钥
const PAK_XOR_KEY: u8 = 0xF7;

// ============================================================
// 类型定义
// ============================================================

/// PAK 资源包中的一个资源文件记录
#[derive(Debug, Clone)]
pub struct PakRecord {
    /// 所属资源包在 PakInterface::collections 中的索引
    pub collection_index: usize,
    /// 资源文件的规范路径（大写、正斜杠、相对路径）
    pub file_name: String,
    /// 文件时间戳
    pub file_time: i64,
    /// 在所属资源包数据中的起始偏移量
    pub start_pos: i64,
    /// 文件大小（字节）
    pub size: i64,
}

/// PAK 资源包的内存映射数据
pub struct PakCollection {
    /// 解密后的原始数据
    pub data: Vec<u8>,
}

/// 虚拟文件句柄，对应 C++ 的 PFILE 结构体
///
/// 两种模式：
/// - PakFile：指向 PAK 包中的虚拟文件（data 直接读取自 PakCollection）
/// - RealFile：指向真实文件系统的文件（通过 std::fs::File 操作）
pub enum PakFileHandle {
    /// PAK 包内虚拟文件
    PakFile {
        /// 指向 PakRecord 的索引（在 PakInterface 的 records 中）
        record_key: String,
        /// 当前读取位置
        pos: i64,
    },
    /// 真实文件系统文件
    RealFile {
        /// 文件路径（供调试使用）
        path: String,
        /// 内部文件对象
        file: std::fs::File,
    },
}

// ============================================================
// 全局单例
// ============================================================

/// 全局 PakInterface 实例（线程安全）
static G_PAK_INTERFACE: Mutex<Option<PakInterface>> = Mutex::new(None);

/// 初始化全局 PAK 接口（应在程序启动时调用一次）
pub fn init_pak_interface() {
    let mut guard = G_PAK_INTERFACE.lock().unwrap();
    *guard = Some(PakInterface::new());
}

/// 获取全局 PAK 接口的引用（通过闭包访问）
pub fn with_pak_interface<F, R>(f: F) -> R
where
    F: FnOnce(&PakInterface) -> R,
{
    let guard = G_PAK_INTERFACE.lock().unwrap();
    let pak = guard.as_ref().expect("PakInterface 未初始化，请先调用 init_pak_interface()");
    f(pak)
}

/// 获取全局 PAK 接口的可变引用（通过闭包访问，线程安全）
pub fn with_pak_interface_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut PakInterface) -> R,
{
    let mut guard = G_PAK_INTERFACE.lock().unwrap();
    let pak = guard.as_mut().expect("PakInterface 未初始化，请先调用 init_pak_interface()");
    f(pak)
}

// ============================================================
// 工具函数
// ============================================================

/// 读取小端序 u32（从字节流中）
fn read_le_u32(data: &[u8], offset: &mut usize) -> u32 {
    let bytes = &data[*offset..*offset + 4];
    *offset += 4;
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// 读取小端序 i32（从字节流中）
fn read_le_i32(data: &[u8], offset: &mut usize) -> i32 {
    let bytes = &data[*offset..*offset + 4];
    *offset += 4;
    i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// 读取小端序 i64（从字节流中）
fn read_le_i64(data: &[u8], offset: &mut usize) -> i64 {
    let bytes = &data[*offset..*offset + 8];
    *offset += 8;
    i64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
}

/// 对字节缓冲区进行 XOR 解密/加密（与 0xF7 异或，可逆操作）
fn xor_decrypt(data: &mut [u8]) {
    for byte in data.iter_mut() {
        *byte ^= PAK_XOR_KEY;
    }
}

/// 规范化 PAK 路径，对应 C++ PakInterface::NormalizePakPath
///
/// 规则：
/// 1. 如果是绝对路径，尝试去掉资源文件夹前缀
/// 2. 词规范化（lexically_normal）
/// 3. 去掉开头的 "./"
/// 4. 统一转换为大写（PAK 内部路径不区分大小写）
/// 5. 反斜杠统一为斜杠
pub fn normalize_pak_path(the_file_name: &str) -> String {
    // 将反斜杠转换为正斜杠
    let mut normalized = the_file_name.replace('\\', "/");

    // 如果是绝对路径，尝试去掉资源文件夹前缀以变为相对路径
    let resource_folder = get_global_resource_folder();
    if !resource_folder.is_empty() && Path::new(&normalized).is_absolute() {
        let res_path = resource_folder.replace('\\', "/");
        // 检查是否以资源文件夹开头
        if let Some(relative) = normalized.strip_prefix(&res_path) {
            // 去掉开头的 "/"
            normalized = relative.trim_start_matches('/').to_string();
        } else if let Some(relative) = normalized.strip_prefix(&res_path.to_uppercase()) {
            normalized = relative.trim_start_matches('/').to_string();
        }
    }

    // 使用 std::path 做规范化处理
    // 注意：C++ lexically_normal() 会解析 "." 和 ".."
    // 我们手动处理简单的路径规范化
    let path = Path::new(&normalized);
    let components: Vec<_> = path.components().collect();
    let mut clean_components: Vec<_> = Vec::new();
    for comp in components {
        match comp {
            std::path::Component::CurDir => { /* 跳过 "." */ }
            std::path::Component::ParentDir => {
                // 如果是 ".."，弹出上一级（如果有）
                clean_components.pop();
            }
            other => {
                clean_components.push(other);
            }
        }
    }
    let mut result: String = clean_components
        .iter()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("/");

    // 去除开头的 "./"
    if result.starts_with("./") {
        result = result[2..].to_string();
    }
    if result.starts_with('.') && result.len() == 1 {
        result.clear();
    }

    // 统一转换为大写（PAK 内部键值不区分大小写）
    result = result.to_uppercase();

    result
}

/// 获取全局资源文件夹路径（通过环境变量或当前目录）
fn get_global_resource_folder() -> String {
    // 尝试从环境变量获取资源路径
    if let Ok(path) = std::env::var("PVZ_RESOURCE_FOLDER") {
        return path.replace('\\', "/");
    }
    // 默认为当前目录
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string().replace('\\', "/"))
        .unwrap_or_default()
}

/// 设置 PAK 系统的资源文件夹根路径
pub fn set_resource_folder(path: &str) {
    let resource_dir = path.replace('\\', "/");
    // set_var 在 Rust 2024 中是不安全函数（可能造成数据竞争）
    unsafe { std::env::set_var("PVZ_RESOURCE_FOLDER", &resource_dir); }
}

// ============================================================
// PakInterface 主结构体
// ============================================================

/// PAK 资源包接口，对应 C++ 的 PakInterface
///
/// 负责：
/// - 管理多个资源包（PakCollection）
/// - 维护文件名到记录的映射缓存
/// - 提供类文件操作接口读取包内文件或回退到真实文件系统
pub struct PakInterface {
    /// 已加载的资源包列表
    pub collections: Vec<PakCollection>,
    /// 所有已注册资源文件的映射（文件名 -> 记录）
    /// 文件名已规范化（大写、正斜杠）
    pub records: HashMap<String, PakRecord>,
    /// 资源文件夹根路径（用于回退到真实文件系统）
    pub resource_folder: String,
}

impl PakInterface {
    /// 创建一个新的 PAK 接口
    pub fn new() -> Self {
        PakInterface {
            collections: Vec::new(),
            records: HashMap::new(),
            resource_folder: get_global_resource_folder(),
        }
    }

    /// 设置资源文件夹路径
    pub fn set_resource_folder(&mut self, path: &str) {
        self.resource_folder = path.replace('\\', "/");
        set_resource_folder(&self.resource_folder);
    }

    // ============================================================
    // 添加 PAK 文件（核心解析逻辑）
    // ============================================================

    /// 加载并解析一个 PAK 文件，将其中所有资源注册到 records 映射中
    ///
    /// 对应 C++ PakInterface::AddPakFile()
    ///
    /// 流程：
    /// 1. 打开文件，读取全部内容
    /// 2. 用 0xF7 XOR 解密
    /// 3. 验证魔数（0xBAC04AC0）和版本号
    /// 4. 扫描目录条目，构建 PakRecord
    /// 5. 修正各记录的文件数据起始位置
    pub fn add_pak_file(&mut self, the_file_name: &str) -> bool {
        // 打开文件
        let file_path = if Path::new(the_file_name).is_absolute() {
            the_file_name.to_string()
        } else if !self.resource_folder.is_empty() {
            format!("{}/{}", self.resource_folder.trim_end_matches('/'), the_file_name)
        } else {
            the_file_name.to_string()
        };

        let mut file = match std::fs::File::open(&file_path) {
            Ok(f) => f,
            Err(_) => return false,
        };

        // 读取文件大小
        let file_size = match file.seek(SeekFrom::End(0)) {
            Ok(s) => s as usize,
            Err(_) => return false,
        };
        if file_size == 0 {
            return false;
        }
        if file.seek(SeekFrom::Start(0)).is_err() {
            return false;
        }

        // 读取整个文件内容
        let mut encrypted_data: Vec<u8> = Vec::with_capacity(file_size);
        if file.read_to_end(&mut encrypted_data).is_err() {
            return false;
        }
        if encrypted_data.len() != file_size {
            return false;
        }

        // 用 0xF7 XOR 解密整块数据
        xor_decrypt(&mut encrypted_data);

        // 添加到 collections 列表
        let collection_index = self.collections.len();
        self.collections.push(PakCollection {
            data: encrypted_data,
        });

        // 获取已解密的数据引用
        let data = &self.collections[collection_index].data;
        let mut offset: usize = 0;
        let data_len = data.len();

        // 读取魔数
        if offset + 4 > data_len {
            self.collections.pop();
            return false;
        }
        let magic = read_le_u32(data, &mut offset);
        if magic != PAK_MAGIC {
            self.collections.pop();
            return false;
        }

        // 读取版本号
        if offset + 4 > data_len {
            self.collections.pop();
            return false;
        }
        let version = read_le_u32(data, &mut offset);
        if version > PAK_VERSION {
            self.collections.pop();
            return false;
        }

        // 为当前资源包的文件名创建规范化键
        let pak_key = normalize_pak_path(the_file_name);

        // 将 PAK 文件自身也注册为一个记录
        let pak_record = PakRecord {
            collection_index,
            file_name: pak_key,
            file_time: 0,
            start_pos: 0,
            size: data_len as i64,
        };
        self.records.insert(pak_record.file_name.clone(), pak_record);

        // 解析目录条目
        let mut current_pos: i64 = 0;
        loop {
            // 检测结束标志
            if offset >= data_len {
                break;
            }
            let flags = data[offset];
            offset += 1;
            if (flags & FILEFLAGS_END) != 0 {
                break;
            }

            // 读取文件名宽度
            if offset >= data_len {
                break;
            }
            let name_width = data[offset] as usize;
            offset += 1;

            // 读取文件名
            if offset + name_width > data_len {
                break;
            }
            let name_bytes = &data[offset..offset + name_width];
            offset += name_width;

            // 读取文件大小
            if offset + 4 > data_len {
                break;
            }
            let src_size = read_le_i32(data, &mut offset) as i64;

            // 读取文件时间
            if offset + 8 > data_len {
                break;
            }
            let file_time = read_le_i64(data, &mut offset);

            // 将文件名中的反斜杠替换为正斜杠
            let mut name = String::from_utf8_lossy(name_bytes).to_string();
            name = name.replace('\\', "/");

            // 规范化路径（大写）
            let key = normalize_pak_path(&name);

            // 检查是否已存在（保留第一个遇到的文件）
            if !self.records.contains_key(&key) {
                let record = PakRecord {
                    collection_index,
                    file_name: key.clone(),
                    file_time,
                    start_pos: current_pos,
                    size: src_size,
                };
                self.records.insert(key, record);
            }

            current_pos += src_size;
        }

        // 修正当前资源包中所有记录的起始位置
        // 目录条目已经读完，offset 指向文件数据区开始
        let an_offset = offset as i64;
        for record in self.records.values_mut() {
            if record.collection_index == collection_index {
                record.start_pos += an_offset;
            }
        }

        true
    }

    // ============================================================
    // 虚拟文件操作
    // ============================================================

    /// 打开一个文件（优先从 PAK 包查找，失败则尝试真实文件系统）
    ///
    /// 对应 C++ PakInterface::FOpen()
    pub fn f_open(&self, the_file_name: &str, an_access: &str) -> Option<PakFileHandle> {
        // 仅支持读取模式
        let is_read = an_access == "r" || an_access == "rb" || an_access == "rt";
        if !is_read {
            return None;
        }

        // 1. 先从 PAK 包中查找
        let key = normalize_pak_path(the_file_name);
        if let Some(record) = self.records.get(&key) {
            return Some(PakFileHandle::PakFile {
                record_key: key,
                pos: 0,
            });
        }

        // 2. 尝试从真实文件系统打开
        // 先尝试相对于资源文件夹
        let file_path = if !self.resource_folder.is_empty() && !Path::new(the_file_name).is_absolute() {
            format!("{}/{}", self.resource_folder.trim_end_matches('/'), the_file_name)
        } else {
            the_file_name.to_string()
        };

        match std::fs::File::open(&file_path) {
            Ok(file) => Some(PakFileHandle::RealFile {
                path: file_path,
                file,
            }),
            Err(_) => None,
        }
    }

    /// 关闭一个文件句柄
    ///
    /// 对应 C++ PakInterface::FClose()
    /// 对于 PAK 虚拟文件，只是释放 PakFileHandle（无需额外操作）
    /// 对于真实文件，std::fs::File 的 Drop 会自动关闭
    pub fn f_close(_file: PakFileHandle) {
        // PakFileHandle 的析构会自动处理
    }

    /// 跳转到文件指定位置
    ///
    /// 对应 C++ PakInterface::FSeek()
    pub fn f_seek(file: &mut PakFileHandle, the_offset: i64, the_origin: i32) -> i32 {
        match file {
            PakFileHandle::PakFile { record_key, pos } => {
                // 只有通过 global interface 才能访问 records，所以需要获取记录信息
                // 这里使用一个内部辅助函数
                let result = with_pak_interface(|pak| {
                    let record = pak.records.get(record_key)?;
                    let new_pos = match the_origin {
                        0 => the_offset,                          // SEEK_SET
                        1 => *pos + the_offset,                   // SEEK_CUR
                        2 => record.size - the_offset,            // SEEK_END
                        _ => return None,
                    };
                    // 限制在 [0, record.size] 范围内
                    let clamped = new_pos.clamp(0, record.size);
                    Some(clamped)
                });

                match result {
                    Some(new_pos) => {
                        *pos = new_pos;
                        0
                    }
                    None => -1,
                }
            }
            PakFileHandle::RealFile { file, .. } => {
                let seek_pos = match the_origin {
                    0 => SeekFrom::Start(the_offset as u64),   // SEEK_SET
                    1 => SeekFrom::Current(the_offset),         // SEEK_CUR
                    2 => SeekFrom::End(the_offset),             // SEEK_END
                    _ => return -1,
                };
                match file.seek(seek_pos) {
                    Ok(_) => 0,
                    Err(_) => -1,
                }
            }
        }
    }

    /// 获取文件当前读取位置
    ///
    /// 对应 C++ PakInterface::FTell()
    pub fn f_tell(file: &mut PakFileHandle) -> i64 {
        match file {
            PakFileHandle::PakFile { pos, .. } => *pos,
            PakFileHandle::RealFile { file: f, .. } => {
                f.stream_position().unwrap_or(0) as i64
            }
        }
    }

    /// 从文件读取数据
    ///
    /// 对应 C++ PakInterface::FRead()
    pub fn f_read(file: &mut PakFileHandle, buffer: &mut [u8], elem_size: i32, count: i32) -> usize {
        let total_bytes = (elem_size * count) as usize;
        if buffer.len() < total_bytes {
            // buffer 太小，只读取能容纳的大小
            return 0;
        }

        match file {
            PakFileHandle::PakFile { record_key, pos } => {
                with_pak_interface(|pak| {
                    let record = match pak.records.get(record_key) {
                        Some(r) => r,
                        None => return 0,
                    };
                    let collection = match pak.collections.get(record.collection_index) {
                        Some(c) => c,
                        None => return 0,
                    };

                    // 实际可读取的字节数，不超过文件剩余大小
                    let remaining = record.size - *pos;
                    let read_bytes = (total_bytes as i64).min(remaining);
                    if read_bytes <= 0 {
                        return 0;
                    }

                    // 计算源数据位置
                    let src_start = (record.start_pos + *pos) as usize;
                    let src_end = src_start + read_bytes as usize;

                    // 边界检查
                    if src_end > collection.data.len() {
                        return 0;
                    }

                    // 拷贝数据
                    let src = &collection.data[src_start..src_end];
                    buffer[..read_bytes as usize].copy_from_slice(src);
                    *pos += read_bytes;

                    (read_bytes / elem_size as i64) as usize
                })
            }
            PakFileHandle::RealFile { file, .. } => {
                // 每次读取 elem_size * count 字节
                let read_size = total_bytes.min(buffer.len());
                match file.read(&mut buffer[..read_size]) {
                    Ok(n) => n / elem_size as usize,
                    Err(_) => 0,
                }
            }
        }
    }

    /// 读取一个字符
    ///
    /// 对应 C++ PakInterface::FGetC()
    /// 跳过 '\r'（换行符归一化）
    pub fn f_getc(file: &mut PakFileHandle) -> i32 {
        match file {
            PakFileHandle::PakFile { record_key, pos } => {
                with_pak_interface(|pak| {
                    let record = match pak.records.get(record_key) {
                        Some(r) => r,
                        None => return -1, // EOF
                    };
                    let collection = match pak.collections.get(record.collection_index) {
                        Some(c) => c,
                        None => return -1,
                    };

                    loop {
                        if *pos >= record.size {
                            return -1; // EOF
                        }
                        let src_pos = (record.start_pos + *pos) as usize;
                        let c = collection.data[src_pos] as char;
                        *pos += 1;
                        if c != '\r' {
                        }
                    }
                })
            }
            PakFileHandle::RealFile { file, .. } => {
                let mut buf = [0u8; 1];
                loop {
                    match file.read(&mut buf) {
                        Ok(0) => return -1, // EOF
                        Ok(_) => {
                            if buf[0] != b'\r' {
                                return buf[0] as i32;
                            }
                            // 跳过 '\r'，继续读取
                        }
                        Err(_) => return -1,
                    }
                }
            }
        }
    }

    /// 退回一个字符到流中
    ///
    /// 对应 C++ PakInterface::UnGetC()
    /// 注意：这只在按顺序回退时才正确工作
    pub fn ungetc(the_char: i32, file: &mut PakFileHandle) -> i32 {
        match file {
            PakFileHandle::PakFile { pos, .. } => {
                *pos = (*pos - 1).max(0);
                the_char
            }
            PakFileHandle::RealFile { file, .. } => {
                if the_char == -1 {
                    return -1;
                }
                let c = the_char as u8;
                // 尝试用 seek 回退一个字节
                match file.seek(SeekFrom::Current(-1)) {
                    Ok(_) => the_char,
                    Err(_) => -1,
                }
            }
        }
    }

    /// 读取一行字符串
    ///
    /// 对应 C++ PakInterface::FGetS()
    /// 读取直到遇到换行符或达到 size 限制，跳过 '\r'
    pub fn f_gets<'a>(file: &'a mut PakFileHandle, buffer: &'a mut [u8]) -> Option<&'a [u8]> {
        match file {
            PakFileHandle::PakFile { record_key, pos } => {
                with_pak_interface(|pak| {
                    let record = match pak.records.get(record_key) {
                        Some(r) => r,
                        None => return None,
                    };
                    let collection = match pak.collections.get(record.collection_index) {
                        Some(c) => c,
                        None => return None,
                    };

                    let mut idx: usize = 0;
                    let buf_len = buffer.len();
                    if buf_len == 0 {
                        return None;
                    }

                    if *pos >= record.size {
                        return None; // 开头即 EOF，返回 nullptr
                    }

                    while idx < buf_len - 1 {
                        if *pos >= record.size {
                            break;
                        }
                        let src_pos = (record.start_pos + *pos) as usize;
                        let c = collection.data[src_pos] as char;
                        *pos += 1;

                        if c == '\r' {
                            continue; // 跳过 '\r'
                        }
                        buffer[idx] = c as u8;
                        idx += 1;
                        if c == '\n' {
                            break;
                        }
                    }
                    buffer[idx] = 0; // null 终止
                    Some(&buffer[..=idx])
                })
            }
            PakFileHandle::RealFile { file, .. } => {
                let mut idx: usize = 0;
                let buf_len = buffer.len();
                if buf_len == 0 {
                    return None;
                }

                loop {
                    let mut byte = [0u8; 1];
                    match file.read(&mut byte) {
                        Ok(0) => {
                            if idx == 0 {
                                return None;
                            }
                            break;
                        }
                        Ok(_) => {
                            if byte[0] == b'\r' {
                                continue;
                            }
                            buffer[idx] = byte[0];
                            idx += 1;
                            if byte[0] == b'\n' || idx >= buf_len - 1 {
                                break;
                            }
                        }
                        Err(_) => {
                            if idx == 0 {
                                return None;
                            }
                            break;
                        }
                    }
                }
                buffer[idx] = 0;
                Some(&buffer[..=idx])
            }
        }
    }

    /// 判断是否到达文件末尾
    ///
    /// 对应 C++ PakInterface::FEof()
    pub fn f_eof(file: &mut PakFileHandle) -> bool {
        match file {
            PakFileHandle::PakFile { record_key, pos } => {
                with_pak_interface(|pak| {
                    let record = pak.records.get(record_key)?;
                    Some(*pos >= record.size)
                })
                .unwrap_or(true)
            }
            PakFileHandle::RealFile { file: f, .. } => {
                match f.stream_position() {
                    Ok(pos) => {
                        if let Ok(_) = f.seek(SeekFrom::End(0)) {
                            let len = f.stream_position().unwrap_or(0);
                            let _ = f.seek(SeekFrom::Start(pos));
                            return pos >= len;
                        }
                        false
                    }
                    Err(_) => true,
                }
            }
        }
    }

    // ============================================================
    // 高层便捷接口
    // ============================================================

    /// 从 PAK 包（或真实文件系统）加载一个文件的完整内容
    ///
    /// 注意：此方法可直接从 &self 读取，不通过 f_read（避免嵌套 Mutex 锁）
    pub fn load_file(&self, path: &str) -> Option<Vec<u8>> {
        let key = normalize_pak_path(path);
        
        // 先查 PAK 记录
        if let Some(record) = self.records.get(&key) {
            if let Some(collection) = self.collections.get(record.collection_index) {
                let start = record.start_pos as usize;
                let end = start + record.size as usize;
                if end <= collection.data.len() {
                    return Some(collection.data[start..end].to_vec());
                }
            }
            return None;
        }

        // 尝试从真实文件系统读取
        let file_path = if !self.resource_folder.is_empty() && !std::path::Path::new(path).is_absolute() {
            format!("{}/{}", self.resource_folder.trim_end_matches('/'), path)
        } else {
            path.to_string()
        };
        std::fs::read(&file_path).ok()
    }

    /// 获取文件大小
    pub fn f_get_size(file: &mut PakFileHandle) -> Option<i64> {
        match file {
            PakFileHandle::PakFile { record_key, .. } => {
                with_pak_interface(|pak| {
                    let record = pak.records.get(record_key)?;
                    Some(record.size)
                })
            }
            PakFileHandle::RealFile { file, .. } => {
                let current_pos = file.stream_position().ok()?;
                let size = file.seek(SeekFrom::End(0)).ok()?;
                file.seek(SeekFrom::Start(current_pos)).ok()?;
                Some(size as i64)
            }
        }
    }

    /// 检查文件是否存在（在 PAK 包或真实文件系统中）
    pub fn file_exists(&self, path: &str) -> bool {
        let key = normalize_pak_path(path);
        if self.records.contains_key(&key) {
            return true;
        }

        // 检查真实文件系统
        let file_path = if !self.resource_folder.is_empty() && !Path::new(path).is_absolute() {
            format!("{}/{}", self.resource_folder.trim_end_matches('/'), path)
        } else {
            path.to_string()
        };
        Path::new(&file_path).exists()
    }

    /// 获取当前已加载资源包中的所有文件列表
    pub fn list_files(&self) -> Vec<&str> {
        self.records.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for PakInterface {
    fn default() -> Self {
        PakInterface::new()
    }
}

// ============================================================
// 静态辅助函数（对应 C++ 的 p_fopen / p_fclose 等）
// ============================================================

/// 全局 p_fopen 等价函数
pub fn p_fopen(the_file_name: &str, an_access: &str) -> Option<PakFileHandle> {
    with_pak_interface(|pak| pak.f_open(the_file_name, an_access))
}

/// 全局 p_fclose 等价函数
pub fn p_fclose(file: PakFileHandle) {
    PakInterface::f_close(file);
}

/// 全局 p_fread 等价函数
pub fn p_fread(file: &mut PakFileHandle, buffer: &mut [u8], elem_size: i32, count: i32) -> usize {
    PakInterface::f_read(file, buffer, elem_size, count)
}

/// 全局 p_fseek 等价函数
pub fn p_fseek(file: &mut PakFileHandle, offset: i64, origin: i32) -> i32 {
    PakInterface::f_seek(file, offset, origin)
}

/// 全局 p_ftell 等价函数
pub fn p_ftell(file: &mut PakFileHandle) -> i64 {
    PakInterface::f_tell(file)
}

/// 全局 p_fgetc 等价函数
pub fn p_fgetc(file: &mut PakFileHandle) -> i32 {
    PakInterface::f_getc(file)
}

/// 全局 p_ungetc 等价函数
pub fn p_ungetc(the_char: i32, file: &mut PakFileHandle) -> i32 {
    PakInterface::ungetc(the_char, file)
}

/// 全局 p_feof 等价函数
pub fn p_feof(file: &mut PakFileHandle) -> bool {
    PakInterface::f_eof(file)
}

// ============================================================
// 测试模块
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path_backslash_to_slash() {
        let result = normalize_pak_path("sounds\\zombie_falling_1.ogg");
        assert_eq!(result, "SOUNDS/ZOMBIE_FALLING_1.OGG");
    }

    #[test]
    fn test_normalize_path_trailing_dot_slash() {
        let result = normalize_pak_path("./sounds/zombie.ogg");
        assert_eq!(result, "SOUNDS/ZOMBIE.OGG");
    }

    #[test]
    fn test_normalize_path_relative() {
        let result = normalize_pak_path("sounds/zombie.ogg");
        assert_eq!(result, "SOUNDS/ZOMBIE.OGG");
    }

    #[test]
    fn test_xor_decrypt_roundtrip() {
        let mut data = vec![0x00, 0xFF, 0xAB, 0xCD];
        xor_decrypt(&mut data);
        assert_eq!(data, vec![0xF7, 0x08, 0x5C, 0x3A]);
        xor_decrypt(&mut data); // XOR 是自反的
        assert_eq!(data, vec![0x00, 0xFF, 0xAB, 0xCD]);
    }

    #[test]
    fn test_pak_interface_new() {
        let pak = PakInterface::new();
        assert!(pak.collections.is_empty());
        assert!(pak.records.is_empty());
    }

    #[test]
    fn test_add_pak_invalid_file() {
        let mut pak = PakInterface::new();
        assert!(!pak.add_pak_file("nonexistent.pak"));
    }

    #[test]
    fn test_f_open_nonexistent() {
        let pak = PakInterface::new();
        assert!(pak.f_open("nonexistent.txt", "rb").is_none());
    }
}
