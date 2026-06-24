// PvZ Portable Rust 翻译 — Buffer 类型
// 对应 C++ SexyAppFramework/misc/Buffer.h / Buffer.cpp

#![allow(dead_code)]

/// 二进制缓冲区，对应 C++ 的 Buffer 类
#[derive(Clone)]
pub struct Buffer {
    data: Vec<u8>,
    pos: usize,
    bit_pos: u8,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            data: Vec::new(),
            pos: 0,
            bit_pos: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Buffer {
            data: Vec::with_capacity(capacity),
            pos: 0,
            bit_pos: 0,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Buffer {
            data: bytes.to_vec(),
            pos: 0,
            bit_pos: 0,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get_pos(&self) -> usize {
        self.pos
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos.min(self.data.len());
    }

    pub fn seek_forward(&mut self, count: usize) {
        self.pos = (self.pos + count).min(self.data.len());
    }

    // ---- 写入 ----

    pub fn write_byte(&mut self, value: u8) {
        if self.pos >= self.data.len() {
            self.data.push(value);
        } else {
            self.data[self.pos] = value;
        }
        self.pos += 1;
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        let end = self.pos + bytes.len();
        if end > self.data.len() {
            self.data.resize(end, 0);
        }
        self.data[self.pos..end].copy_from_slice(bytes);
        self.pos = end;
    }

    pub fn write_u16(&mut self, value: u16) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_i16(&mut self, value: i16) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_u32(&mut self, value: u32) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_i32(&mut self, value: i32) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_f32(&mut self, value: f32) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_f64(&mut self, value: f64) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_string(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.write_u32(bytes.len() as u32);
        self.write_bytes(bytes);
    }

    // ---- 读取 ----

    pub fn read_byte(&mut self) -> u8 {
        if self.pos >= self.data.len() { return 0; }
        let val = self.data[self.pos];
        self.pos += 1;
        val
    }

    pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        let end = (self.pos + count).min(self.data.len());
        let bytes = self.data[self.pos..end].to_vec();
        self.pos = end;
        bytes
    }

    pub fn read_u16(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        let end = (self.pos + 2).min(self.data.len());
        buf.copy_from_slice(&self.data[self.pos..end]);
        self.pos = end;
        u16::from_le_bytes(buf)
    }

    pub fn read_i16(&mut self) -> i16 {
        let mut buf = [0u8; 2];
        let end = (self.pos + 2).min(self.data.len());
        buf.copy_from_slice(&self.data[self.pos..end]);
        self.pos = end;
        i16::from_le_bytes(buf)
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        let end = (self.pos + 4).min(self.data.len());
        buf.copy_from_slice(&self.data[self.pos..end]);
        self.pos = end;
        u32::from_le_bytes(buf)
    }

    pub fn read_i32(&mut self) -> i32 {
        let mut buf = [0u8; 4];
        let end = (self.pos + 4).min(self.data.len());
        buf.copy_from_slice(&self.data[self.pos..end]);
        self.pos = end;
        i32::from_le_bytes(buf)
    }

    pub fn read_f32(&mut self) -> f32 {
        let mut buf = [0u8; 4];
        let end = (self.pos + 4).min(self.data.len());
        buf.copy_from_slice(&self.data[self.pos..end]);
        self.pos = end;
        f32::from_le_bytes(buf)
    }

    pub fn read_f64(&mut self) -> f64 {
        let mut buf = [0u8; 8];
        let end = (self.pos + 8).min(self.data.len());
        buf.copy_from_slice(&self.data[self.pos..end]);
        self.pos = end;
        f64::from_le_bytes(buf)
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_u32() as usize;
        let bytes = self.read_bytes(len);
        String::from_utf8_lossy(&bytes).to_string()
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.pos = 0;
        self.bit_pos = 0;
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.data.len()
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer::new()
    }
}
