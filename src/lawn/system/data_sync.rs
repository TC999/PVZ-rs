// PvZ Portable Rust 翻译 — DataSync（二进制数据同步/序列化）
// 对应 C++ src/Lawn/System/DataSync.h / DataSync.cpp

#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

/// 数据读取器（对应 C++ DataReader）
pub struct DataReader {
    data: Vec<u8>,
    pos: usize,
}

impl DataReader {
    pub fn new() -> Self {
        DataReader {
            data: Vec::new(),
            pos: 0,
        }
    }

    /// 从文件打开
    pub fn open_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        let mut file = File::open(path).ok()?;
        let mut data = Vec::new();
        file.read_to_end(&mut data).ok()?;
        Some(DataReader { data, pos: 0 })
    }

    /// 从内存打开
    pub fn open_memory(data: Vec<u8>) -> Self {
        DataReader { data, pos: 0 }
    }

    /// 关闭
    pub fn close(&mut self) {
        self.data.clear();
        self.pos = 0;
    }

    /// 读取字节
    pub fn read_bytes(&mut self, mem: &mut [u8]) {
        let end = (self.pos + mem.len()).min(self.data.len());
        let len = end - self.pos;
        mem[..len].copy_from_slice(&self.data[self.pos..end]);
        self.pos = end;
    }

    /// 后退指定字节数
    pub fn rewind(&mut self, num_bytes: u32) {
        self.pos = self.pos.saturating_sub(num_bytes as usize);
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.read_bytes(&mut buf);
        u64::from_le_bytes(buf)
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.read_bytes(&mut buf);
        u32::from_le_bytes(buf)
    }

    pub fn read_u16(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        self.read_bytes(&mut buf);
        u16::from_le_bytes(buf)
    }

    pub fn read_u8(&mut self) -> u8 {
        if self.pos < self.data.len() {
            let val = self.data[self.pos];
            self.pos += 1;
            val
        } else {
            0
        }
    }

    pub fn read_bool(&mut self) -> bool {
        self.read_u8() != 0
    }

    pub fn read_f32(&mut self) -> f32 {
        f32::from_bits(self.read_u32())
    }

    pub fn read_f64(&mut self) -> f64 {
        f64::from_bits(self.read_u64())
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_u16() as usize;
        let mut buf = vec![0u8; len];
        self.read_bytes(&mut buf);
        String::from_utf8_lossy(&buf).to_string()
    }
}

impl Default for DataReader {
    fn default() -> Self {
        Self::new()
    }
}

/// 数据读取器异常（对应 C++ DataReaderException）
#[derive(Debug)]
pub struct DataReaderException;

impl std::fmt::Display for DataReaderException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataReaderException")
    }
}

impl std::error::Error for DataReaderException {}

/// 数据写入器（对应 C++ DataWriter）
pub struct DataWriter {
    data: Vec<u8>,
}

impl DataWriter {
    pub fn new() -> Self {
        DataWriter { data: Vec::new() }
    }

    /// 确保容量
    fn ensure_capacity(&mut self, num_bytes: u32) {
        let needed = self.data.len() + num_bytes as usize;
        if needed > self.data.capacity() {
            self.data.reserve(num_bytes as usize);
        }
    }

    /// 打开文件写入
    pub fn open_file<P: AsRef<Path>>(&self, path: P) -> bool {
        // 写入由 write_to_file 完成
        Path::new(path.as_ref()).parent().map(|p| std::fs::create_dir_all(p)).is_some()
    }

    /// 打开内存写入
    pub fn open_memory(reserve_amount: u32) -> Self {
        DataWriter {
            data: Vec::with_capacity(reserve_amount as usize),
        }
    }

    /// 关闭
    pub fn close(&mut self) {
        self.data.clear();
    }

    /// 写入文件
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Ok(mut file) = File::create(path) {
            file.write_all(&self.data).is_ok()
        } else {
            false
        }
    }

    /// 写入字节
    pub fn write_bytes(&mut self, data: &[u8]) {
        self.ensure_capacity(data.len() as u32);
        self.data.extend_from_slice(data);
    }

    pub fn write_u64(&mut self, val: u64) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_u32(&mut self, val: u32) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_u16(&mut self, val: u16) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_u8(&mut self, val: u8) {
        self.data.push(val);
    }

    pub fn write_bool(&mut self, val: bool) {
        self.write_u8(if val { 1 } else { 0 });
    }

    pub fn write_f32(&mut self, val: f32) {
        self.write_u32(val.to_bits());
    }

    pub fn write_f64(&mut self, val: f64) {
        self.write_u64(val.to_bits());
    }

    pub fn write_string(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.write_u16(bytes.len() as u16);
        self.write_bytes(bytes);
    }

    /// 获取当前位置
    pub fn get_pos(&self) -> u32 {
        self.data.len() as u32
    }

    /// 获取数据指针
    pub fn get_data_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    /// 获取数据长度
    pub fn get_data_len(&self) -> i32 {
        self.data.len() as i32
    }
}

impl Default for DataWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// 数据同步器（对应 C++ DataSync）
/// 提供统一的序列化/反序列化接口
pub struct DataSync {
    reader: Option<DataReader>,
    writer: Option<DataWriter>,
    version: i32,
    pointer_to_int: HashMap<usize, i32>,
    int_to_pointer: HashMap<i32, usize>,
    pointer_sync_list: Vec<*mut *mut u8>,
    cur_pointer_index: i32,
}

impl DataSync {
    /// 从读取器创建（用于反序列化）
    pub fn from_reader(reader: DataReader) -> Self {
        DataSync {
            reader: Some(reader),
            writer: None,
            version: 0,
            pointer_to_int: HashMap::new(),
            int_to_pointer: HashMap::new(),
            pointer_sync_list: Vec::new(),
            cur_pointer_index: 0,
        }
    }

    /// 从写入器创建（用于序列化）
    pub fn from_writer(writer: DataWriter) -> Self {
        DataSync {
            reader: None,
            writer: Some(writer),
            version: 0,
            pointer_to_int: HashMap::new(),
            int_to_pointer: HashMap::new(),
            pointer_sync_list: Vec::new(),
            cur_pointer_index: 0,
        }
    }

    pub fn get_reader(&self) -> Option<&DataReader> {
        self.reader.as_ref()
    }

    pub fn get_writer(&self) -> Option<&DataWriter> {
        self.writer.as_ref()
    }

    pub fn set_version(&mut self, version: i32) {
        self.version = version;
    }

    pub fn get_version(&self) -> i32 {
        self.version
    }

    pub fn get_reader_mut(&mut self) -> Option<&mut DataReader> {
        self.reader.as_mut()
    }

    pub fn get_writer_mut(&mut self) -> Option<&mut DataWriter> {
        self.writer.as_mut()
    }

    /// 同步字节（读取或写入）
    pub fn sync_bytes(&mut self, data: &mut [u8]) {
        if let Some(ref mut r) = self.reader {
            r.read_bytes(data);
        } else if let Some(ref mut w) = self.writer {
            w.write_bytes(data);
        }
    }

    pub fn sync_u32(&mut self, num: &mut u32) {
        if let Some(ref mut r) = self.reader {
            *num = r.read_u32();
        } else if let Some(ref mut w) = self.writer {
            w.write_u32(*num);
        }
    }

    pub fn sync_u16(&mut self, num: &mut u16) {
        if let Some(ref mut r) = self.reader {
            *num = r.read_u16();
        } else if let Some(ref mut w) = self.writer {
            w.write_u16(*num);
        }
    }

    pub fn sync_u8(&mut self, num: &mut u8) {
        if let Some(ref mut r) = self.reader {
            *num = r.read_u8();
        } else if let Some(ref mut w) = self.writer {
            w.write_u8(*num);
        }
    }

    pub fn sync_bool(&mut self, b: &mut bool) {
        if let Some(ref mut r) = self.reader {
            *b = r.read_bool();
        } else if let Some(ref mut w) = self.writer {
            w.write_bool(*b);
        }
    }

    pub fn sync_f32(&mut self, f: &mut f32) {
        if let Some(ref mut r) = self.reader {
            *f = r.read_f32();
        } else if let Some(ref mut w) = self.writer {
            w.write_f32(*f);
        }
    }

    pub fn sync_f64(&mut self, f: &mut f64) {
        if let Some(ref mut r) = self.reader {
            *f = r.read_f64();
        } else if let Some(ref mut w) = self.writer {
            w.write_f64(*f);
        }
    }

    pub fn sync_string(&mut self, s: &mut String) {
        if let Some(ref mut r) = self.reader {
            *s = r.read_string();
        } else if let Some(ref mut w) = self.writer {
            w.write_string(s);
        }
    }

    fn reset_pointer_table(&mut self) {
        self.pointer_to_int.clear();
        self.int_to_pointer.clear();
    }

    fn reset(&mut self) {
        self.version = 0;
        self.reset_pointer_table();
        self.pointer_sync_list.clear();
        self.cur_pointer_index = 0;
    }
}
