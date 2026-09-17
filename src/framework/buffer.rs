// PvZ Portable Rust 翻译 — Buffer 类型（Sexy Framework 位流缓冲区）
// 对应 C++ SexyAppFramework/misc/Buffer.h / Buffer.cpp

#![allow(dead_code)]

/// 对应 C++ WEB_ENCODE_MAP（64 个可打印字符，索引即 6 bit 值）
const WEB_ENCODE_MAP: &[u8; 64] =
    b".-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// 对应 C++ WEB_DECODE_MAP：编码字符 -> 0..63，非编码字符为 -1
fn web_decode(c: u8) -> i32 {
    match c {
        b'.' | b'+' => 0,
        b'-' => 1,
        b'0'..=b'9' => (c - b'0') as i32 + 2,
        b'A'..=b'Z' => (c - b'A') as i32 + 12,
        b'a'..=b'z' => (c - b'a') as i32 + 38,
        _ => -1,
    }
}

/// 对应 C++ WIN1252_TO_UNICODE（0x80-0x9F 段；SDL_iconv 在 Wine 下处理有误故手工映射）
const WIN1252_TO_UNICODE: [u32; 32] = [
    0x20AC, 0x0081, 0x201A, 0x0192, 0x201E, 0x2026, 0x2020, 0x2021,
    0x02C6, 0x2030, 0x0160, 0x2039, 0x0152, 0x008D, 0x017D, 0x008F,
    0x0090, 0x2018, 0x2019, 0x201C, 0x201D, 0x2022, 0x2013, 0x2014,
    0x02DC, 0x2122, 0x0161, 0x203A, 0x0153, 0x009D, 0x017E, 0x0178,
];

/// 对应 C++ GenerateCRCTable()（POLYNOMIAL = 0x04c11db7）
const fn generate_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut crc_accum = (i as u32) << 24;
        let mut j = 0;
        while j < 8 {
            if crc_accum & 0x8000_0000 != 0 {
                crc_accum = (crc_accum << 1) ^ 0x04c1_1db7;
            } else {
                crc_accum <<= 1;
            }
            j += 1;
        }
        table[i] = crc_accum;
        i += 1;
    }
    table
}

const CRC_TABLE: [u32; 256] = generate_crc_table();

/// 二进制缓冲区，对应 C++ 的 `Sexy::Buffer`。
///
/// C++ 侧是**位流**缓冲区：写入按 `mWriteBitPos` 逐位拼接（非对齐时并入当前字节并补出高位），
/// 读取按 `mReadBitPos` 按位前进，`mDataBitSize` 记录已写入的位数（`GetDataLen()` 由它向上取整）。
/// 本实现保留同样的三个游标，因此在位对齐（8 的倍数）时行为与顺序字节流一致——
/// `save_game.rs` 的 TLV 读写全部是字节对齐操作，故不受影响（C++ `SaveGame.cpp` 亦然，
/// 其 `PortableSaveContext` 同样以 `Sexy::Buffer` 的 ReadInt32/WriteInt32/ReadBytes/WriteBytes 承载 TLV）。
#[derive(Clone)]
pub struct Buffer {
    /// 对应 C++ `Buffer::mData`
    pub data: Vec<u8>,
    /// 对应 C++ `Buffer::mDataBitSize`
    pub data_bit_size: i32,
    /// 对应 C++ `Buffer::mReadBitPos`（C++ 中为 mutable）
    pub read_bit_pos: i32,
    /// 对应 C++ `Buffer::mWriteBitPos`
    pub write_bit_pos: i32,
}

impl Buffer {
    // ---- 构造 / 视图 ----

    /// 对应 C++ `Buffer::Buffer()`（三个游标归零）
    pub fn new() -> Self {
        Buffer {
            data: Vec::new(),
            data_bit_size: 0,
            read_bit_pos: 0,
            write_bit_pos: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Buffer {
            data: Vec::with_capacity(capacity),
            data_bit_size: 0,
            read_bit_pos: 0,
            write_bit_pos: 0,
        }
    }

    /// 用已有字节构造（对应 C++ `SetData`；写入游标置于末尾，便于在已有数据后追加）
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Buffer {
            data: bytes.to_vec(),
            data_bit_size: (bytes.len() * 8) as i32,
            read_bit_pos: 0,
            write_bit_pos: (bytes.len() * 8) as i32,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get_pos(&self) -> usize {
        (self.read_bit_pos / 8) as usize
    }

    pub fn set_pos(&mut self, pos: usize) {
        let a_pos = pos.min(self.data.len());
        self.read_bit_pos = (a_pos * 8) as i32;
    }

    pub fn seek_forward(&mut self, count: usize) {
        let a_pos = (self.get_pos() + count).min(self.data.len());
        self.read_bit_pos = (a_pos * 8) as i32;
    }

    /// 对应 C++ `Buffer::SeekFront()`
    pub fn seek_front(&mut self) {
        self.read_bit_pos = 0;
    }

    pub fn is_eof(&self) -> bool {
        (self.read_bit_pos / 8) as usize >= self.data.len()
    }

    /// 剩余可读**字节**数。
    ///
    /// 对应 C++ `PortableSaveContext::ByteLeftToRead()`：
    /// `(mBuffer.mDataBitSize - mBuffer.mReadBitPos + 7) / 8`（SaveGame.cpp:2625）
    pub fn remaining(&self) -> usize {
        let a_left = (self.data_bit_size - self.read_bit_pos + 7) / 8;
        if a_left < 0 {
            0
        } else {
            a_left as usize
        }
    }

    /// 对应 C++ `Buffer::GetDataPtr()`
    pub fn get_data_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    /// 对应 C++ `Buffer::GetDataLen()`（按位向上取整）
    pub fn get_data_len(&self) -> i32 {
        (self.data_bit_size + 7) / 8
    }

    /// 对应 C++ `Buffer::GetDataLenBits()`
    pub fn get_data_len_bits(&self) -> i32 {
        self.data_bit_size
    }

    /// 对应 C++ `Buffer::AtEnd()`
    pub fn at_end(&self) -> bool {
        self.read_bit_pos >= self.data_bit_size
    }

    /// 对应 C++ `Buffer::PastEnd()`
    pub fn past_end(&self) -> bool {
        self.read_bit_pos > self.data_bit_size
    }

    /// 对应 C++ `Buffer::Clear()`
    pub fn clear(&mut self) {
        self.read_bit_pos = 0;
        self.write_bit_pos = 0;
        self.data_bit_size = 0;
        self.data.clear();
    }

    /// 对应 C++ `Buffer::SetData(const ByteVector&)`（不重置读写游标）
    pub fn set_data(&mut self, bytes: &[u8]) {
        self.data = bytes.to_vec();
        self.data_bit_size = (self.data.len() * 8) as i32;
    }

    // ---- 写入 ----

    /// 对应 C++ `Buffer::WriteByte`
    pub fn write_byte(&mut self, the_byte: u8) {
        if self.write_bit_pos % 8 == 0 {
            self.data.push(the_byte);
        } else {
            let an_ofs = (self.write_bit_pos % 8) as u32;
            let idx = (self.write_bit_pos / 8) as usize;
            if idx < self.data.len() {
                self.data[idx] |= the_byte << an_ofs;
            }
            self.data.push(the_byte >> (8 - an_ofs));
        }

        self.write_bit_pos += 8;
        if self.write_bit_pos > self.data_bit_size {
            self.data_bit_size = self.write_bit_pos;
        }
    }

    /// 对应 C++ `Buffer::WriteNumBits`（逐位写入，需要时补齐新字节）
    pub fn write_num_bits(&mut self, the_num: i32, the_bits: i32) {
        for a_bit_num in 0..the_bits {
            if self.write_bit_pos % 8 == 0 {
                self.data.push(0);
            }
            if (the_num & (1 << a_bit_num)) != 0 {
                let idx = (self.write_bit_pos / 8) as usize;
                if idx < self.data.len() {
                    self.data[idx] |= 1u8 << (self.write_bit_pos % 8);
                }
            }
            self.write_bit_pos += 1;
        }

        if self.write_bit_pos > self.data_bit_size {
            self.data_bit_size = self.write_bit_pos;
        }
    }

    /// 对应 C++ `static Buffer::GetBitsRequired(int, bool)`（负数按二补数取位宽）
    pub fn get_bits_required(the_num: i32, is_signed: bool) -> i32 {
        // C++ 在 theNum == INT_MIN 时 `-theNum` 溢出；这里用 i64 承载以得到同样的位宽结果
        let mut a_num = the_num as i64;
        if a_num < 0 {
            a_num = -a_num - 1;
        }

        let mut a_num_bits: i32 = 0;
        while a_num >= (1i64 << a_num_bits) {
            a_num_bits += 1;
        }

        if is_signed {
            a_num_bits += 1;
        }

        a_num_bits
    }

    /// 对应 C++ `Buffer::WriteBoolean`
    pub fn write_boolean(&mut self, the_bool: bool) {
        self.write_byte(if the_bool { 1 } else { 0 });
    }

    /// 对应 C++ `Buffer::WriteShort`（2 字节 LE）
    pub fn write_short(&mut self, the_short: i16) {
        self.write_byte(the_short as u8);
        self.write_byte(((the_short >> 8) as u8) & 0xFF);
    }

    /// 对应 C++ `Buffer::WriteUInt32`（4 字节 LE）
    pub fn write_u32(&mut self, the_value: u32) {
        self.write_byte(the_value as u8);
        self.write_byte((the_value >> 8) as u8);
        self.write_byte((the_value >> 16) as u8);
        self.write_byte((the_value >> 24) as u8);
    }

    /// 对应 C++ `Buffer::WriteInt32`
    pub fn write_i32(&mut self, the_value: i32) {
        self.write_u32(the_value as u32);
    }

    /// 对应 C++ `Buffer::WriteString`（长度前缀是 **WriteShort**，2 字节）
    pub fn write_string(&mut self, the_string: &str) {
        let a_bytes = the_string.as_bytes();
        self.write_short(a_bytes.len() as i16);
        self.write_bytes(a_bytes);
    }

    /// 对应 C++ `Buffer::WriteLine`（字符串 + CRLF）
    pub fn write_line(&mut self, the_string: &str) {
        self.write_bytes(the_string.as_bytes());
        self.write_byte(b'\r');
        self.write_byte(b'\n');
    }

    /// 对应 C++ `Buffer::WriteBuffer`（UInt32 长度 + 字节）
    pub fn write_buffer(&mut self, the_buffer: &[u8]) {
        self.write_u32(the_buffer.len() as u32);
        self.write_bytes(the_buffer);
    }

    /// 对应 C++ `Buffer::WriteBytes`（逐字节 WriteByte）
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.write_byte(b);
        }
    }

    /// 便捷包装：小端 u16（C++ 无此重载，Rust 侧沿用）
    pub fn write_u16(&mut self, value: u16) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// 便捷包装：小端 i16
    pub fn write_i16(&mut self, value: i16) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// 便捷包装：小端 f32
    pub fn write_f32(&mut self, value: f32) {
        self.write_u32(value.to_bits());
    }

    /// 便捷包装：小端 f64
    pub fn write_f64(&mut self, value: f64) {
        self.write_bytes(&value.to_le_bytes());
    }

    // ---- 读取 ----

    /// 对应 C++ `Buffer::ReadByte`：越界返回 0 且**不**推进读取游标
    pub fn read_byte(&mut self) -> u8 {
        if (self.read_bit_pos + 7) / 8 >= self.data.len() as i32 {
            return 0; // Underflow
        }

        if self.read_bit_pos % 8 == 0 {
            let b = self.data[(self.read_bit_pos / 8) as usize];
            self.read_bit_pos += 8;
            b
        } else {
            let an_ofs = (self.read_bit_pos % 8) as u32;
            let idx = (self.read_bit_pos / 8) as usize;

            let mut b = self.data[idx] >> an_ofs;
            if idx + 1 < self.data.len() {
                b |= self.data[idx + 1] << (8 - an_ofs);
            }

            self.read_bit_pos += 8;

            b
        }
    }

    /// 对应 C++ `Buffer::ReadNumBits`
    pub fn read_num_bits(&mut self, the_bits: i32, is_signed: bool) -> i32 {
        let a_byte_length = self.data.len() as i32;

        let mut the_num: i32 = 0;
        let mut bset = false;
        for a_bit_num in 0..the_bits {
            let a_byte_pos = self.read_bit_pos / 8;

            if a_byte_pos >= a_byte_length {
                break;
            }

            bset = (self.data[a_byte_pos as usize] & (1u8 << (self.read_bit_pos % 8))) != 0;
            if bset {
                the_num |= 1 << a_bit_num;
            }

            self.read_bit_pos += 1;
        }

        if is_signed && bset {
            // 符号扩展
            for a_bit_num in the_bits..32 {
                the_num |= 1 << a_bit_num;
            }
        }

        the_num
    }

    /// 对应 C++ `Buffer::ReadBoolean`
    pub fn read_boolean(&mut self) -> bool {
        self.read_byte() != 0
    }

    /// 对应 C++ `Buffer::ReadShort`
    pub fn read_short(&mut self) -> i16 {
        let mut a_short = self.read_byte() as i16;
        a_short |= (self.read_byte() as i16) << 8;
        a_short
    }

    /// 对应 C++ `Buffer::ReadUInt32`
    pub fn read_u32(&mut self) -> u32 {
        let mut a_value = self.read_byte() as u32;
        a_value |= (self.read_byte() as u32) << 8;
        a_value |= (self.read_byte() as u32) << 16;
        a_value |= (self.read_byte() as u32) << 24;
        a_value
    }

    /// 对应 C++ `Buffer::ReadInt32`
    pub fn read_i32(&mut self) -> i32 {
        self.read_u32() as i32
    }

    /// 对应 C++ `Buffer::ReadString`（长度前缀是 **ReadShort**，2 字节）
    pub fn read_string(&mut self) -> String {
        let a_len = self.read_short();
        let mut a_bytes: Vec<u8> = Vec::new();
        for _ in 0..a_len {
            a_bytes.push(self.read_byte());
        }
        String::from_utf8_lossy(&a_bytes).to_string()
    }

    /// 对应 C++ `Buffer::ReadLine`（读到 NUL 或 '\n'，丢弃 '\r'）
    pub fn read_line(&mut self) -> String {
        let mut a_bytes: Vec<u8> = Vec::new();
        loop {
            let c = self.read_byte();
            if c == 0 || c == b'\n' {
                break;
            }
            if c != b'\r' {
                a_bytes.push(c);
            }
        }
        String::from_utf8_lossy(&a_bytes).to_string()
    }

    /// 读取 `the_len` 个字节（对应 C++ `ReadBytes`）。
    ///
    /// C++ 的 ReadBytes 逐字节调用 ReadByte，越界部分由调用方缓冲区保持/填 0；
    /// 这里返回**实际可读**的字节（越界即截断，读取游标停在末尾），
    /// `save_game.rs` 依赖短返回值判定读取失败。
    pub fn read_bytes(&mut self, the_len: usize) -> Vec<u8> {
        let mut a_data: Vec<u8> = Vec::with_capacity(the_len);
        for _ in 0..the_len {
            if (self.read_bit_pos + 7) / 8 >= self.data.len() as i32 {
                break;
            }
            a_data.push(self.read_byte());
        }
        a_data
    }

    /// 对应 C++ `Buffer::ReadBuffer`（UInt32 长度 + 字节）
    pub fn read_buffer(&mut self) -> Vec<u8> {
        let a_length = self.read_u32() as usize;
        self.read_bytes(a_length)
    }

    /// 便捷包装：小端 u16
    pub fn read_u16(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        for b in buf.iter_mut() {
            *b = self.read_byte();
        }
        u16::from_le_bytes(buf)
    }

    /// 便捷包装：小端 i16
    pub fn read_i16(&mut self) -> i16 {
        let mut buf = [0u8; 2];
        for b in buf.iter_mut() {
            *b = self.read_byte();
        }
        i16::from_le_bytes(buf)
    }

    /// 便捷包装：小端 f32
    pub fn read_f32(&mut self) -> f32 {
        f32::from_bits(self.read_u32())
    }

    /// 便捷包装：小端 f64
    pub fn read_f64(&mut self) -> f64 {
        let mut buf = [0u8; 8];
        for b in buf.iter_mut() {
            *b = self.read_byte();
        }
        f64::from_le_bytes(buf)
    }

    // ---- 文本编码 / 校验 ----

    /// 对应 C++ `Buffer::ToWebString()`
    pub fn to_web_string(&mut self) -> String {
        let mut a_string = String::new();
        let a_size_bits = self.write_bit_pos;

        let an_old_read_bit_pos = self.read_bit_pos;
        self.read_bit_pos = 0;

        a_string.push_str(&format!("{:08X}", a_size_bits));

        let a_num_chars = (a_size_bits + 5) / 6;
        for _ in 0..a_num_chars {
            let a_val = self.read_num_bits(6, false);
            a_string.push(WEB_ENCODE_MAP[(a_val & 0x3F) as usize] as char);
        }

        self.read_bit_pos = an_old_read_bit_pos;

        a_string
    }

    /// 对应 C++ `Buffer::FromWebString()`
    pub fn from_web_string(&mut self, the_string: &str) {
        self.clear();

        let the_bytes = the_string.as_bytes();
        if the_bytes.len() < 4 {
            return;
        }

        let mut a_size_bits: i32 = 0;
        for a_digit_num in 0..8i32 {
            let a_char = if (a_digit_num as usize) < the_bytes.len() {
                the_bytes[a_digit_num as usize]
            } else {
                0
            };

            let a_val = if (b'0'..=b'9').contains(&a_char) {
                (a_char - b'0') as i32
            } else if (b'A'..=b'F').contains(&a_char) {
                (a_char - b'A') as i32 + 10
            } else if (b'a'..=b'f').contains(&a_char) {
                (a_char - b'a') as i32 + 10
            } else {
                0
            };

            a_size_bits += a_val << ((7 - a_digit_num) * 4);
        }

        let mut a_char_idx = 8usize;
        let mut a_num_bits_left = a_size_bits;
        while a_num_bits_left > 0 {
            // C++ 对 string_view 不做边界检查（越界为 UB）；这里提前结束以保证安全
            if a_char_idx >= the_bytes.len() {
                break;
            }

            let a_char = the_bytes[a_char_idx];
            a_char_idx += 1;
            let a_val = web_decode(a_char);
            let a_num_bits = if a_num_bits_left < 6 { a_num_bits_left } else { 6 };

            self.write_num_bits(a_val, a_num_bits);
            a_num_bits_left -= a_num_bits;
        }

        self.seek_front();
    }

    /// 对应 C++ `Buffer::ToUTF8String()`（BOM 处理 + UTF-16 转码 + Windows-1252 回退）
    pub fn to_utf8_string(&self) -> Option<String> {
        let a_data: &[u8] = &self.data;
        let a_len = a_data.len();

        if a_len >= 3 && &a_data[0..3] == b"\xEF\xBB\xBF" {
            // UTF-8 BOM：剔除
            return Some(String::from_utf8_lossy(&a_data[3..]).to_string());
        }

        if a_len >= 2 && &a_data[0..2] == b"\xFF\xFE" {
            if (a_len - 2) % 2 != 0 {
                return None;
            }
            return utf16_to_utf8(&a_data[2..], true);
        }

        if a_len >= 2 && &a_data[0..2] == b"\xFE\xFF" {
            if (a_len - 2) % 2 != 0 {
                return None;
            }
            return utf16_to_utf8(&a_data[2..], false);
        }

        if std::str::from_utf8(a_data).is_ok() {
            return Some(String::from_utf8_lossy(a_data).to_string());
        }

        Some(win1252_to_utf8(a_data))
    }

    /// 对应 C++ `Buffer::GetCRC32()`
    pub fn get_crc32(&self, the_seed: u32) -> u32 {
        let mut a_crc = the_seed;
        for &b in self.data.iter() {
            let i = (((a_crc >> 24) ^ (b as u32)) & 0xFF) as usize;
            a_crc = (a_crc << 8) ^ CRC_TABLE[i];
        }
        a_crc
    }
}

/// 对应 C++ `Buffer::ToUTF8String` 中的 `SDL_iconv_string("UTF-8", "UTF-16LE/BE", ...)`
fn utf16_to_utf8(bytes: &[u8], little_endian: bool) -> Option<String> {
    let mut a_units: Vec<u16> = Vec::with_capacity(bytes.len() / 2);
    let mut i = 0usize;
    while i + 1 < bytes.len() {
        let a_unit = if little_endian {
            u16::from_le_bytes([bytes[i], bytes[i + 1]])
        } else {
            u16::from_be_bytes([bytes[i], bytes[i + 1]])
        };
        a_units.push(a_unit);
        i += 2;
    }
    String::from_utf16(&a_units).ok()
}

/// 对应 C++ `Win1252ToUTF8`
fn win1252_to_utf8(data: &[u8]) -> String {
    let mut a_result = String::with_capacity(data.len());
    for &b in data {
        let a_codepoint: u32 = if b < 0x80 {
            b as u32
        } else if b <= 0x9F {
            WIN1252_TO_UNICODE[(b - 0x80) as usize]
        } else {
            // 0xA0-0xFF 直接映射到 U+00A0-U+00FF
            b as u32
        };

        if let Some(c) = char::from_u32(a_codepoint) {
            a_result.push(c);
        }
    }
    a_result
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer::new()
    }
}
