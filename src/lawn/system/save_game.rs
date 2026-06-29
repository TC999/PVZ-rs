// PvZ Portable Rust 翻译 — 游戏存档系统
// 对应 C++ src/Lawn/System/SaveGame.cpp（内部数据结构）

#![allow(dead_code)]

use crate::framework::buffer::Buffer;
use crate::framework::color::Color;
use crate::framework::common::SexyVector2;
use crate::framework::rect::Rect;
use crate::framework::sexy_matrix::SexyMatrix3;
use crate::lawn::board::Board;
use crate::lawn::game_object::GameObject;

// ── 常量 ──────────────────────────────────────────────

const SAVE_FILE_MAGIC_NUMBER: u32 = 0xFEEDDEAD;
const PORTABLE_FIELD_TAIL: u32 = 100;
// 便携字段尾部 ID
const SAVE_FILE_VERSION: u32 = 2;
const SAVE_FILE_MAGIC_V4: [u8; 12] = *b"PVZP_SAVE4\0\0";
const SAVE_FILE_V4_VERSION: u32 = 1;
const SAVE4_CHUNK_VERSION: u32 = 1;

// ── 文件头结构体 ──────────────────────────────────────

/// V4 存档文件头（对应 C++ SaveFileHeaderV4）
#[repr(C)]
pub struct SaveFileHeaderV4 {
    pub magic: [u8; 12],       // "PVZP_SAVE4"
    pub version: u32,
    pub payload_size: u32,
    pub payload_crc: u32,
}

/// 旧版存档文件头（对应 C++ SaveFileHeader）
#[repr(C)]
pub struct SaveFileHeader {
    pub magic_number: u32,
    pub build_version: u32,
    pub build_date: u32,
}

// ── 枚举 ──────────────────────────────────────────────

/// V4 存档块类型（对应 C++ SaveChunkTypeV4）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SaveChunkTypeV4 {
    BoardBase = 1,
    Zombies = 2,
    Plants = 3,
    Projectiles = 4,
    Coins = 5,
    Mowers = 6,
    GridItems = 7,
    ParticleEmitters = 8,
    ParticleParticles = 9,
    ParticleSystems = 10,
    Reanimations = 11,
    Trails = 12,
    Attachments = 13,
    Cursor = 14,
    CursorPreview = 15,
    Advice = 16,
    SeedBank = 17,
    SeedPackets = 18,
    Challenge = 19,
    Music = 20,
}

/// 棋盘基础字段 ID（对应 C++ BoardBaseFieldId，103 个值）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BoardBaseFieldId {
    // 从 C++ 枚举 BoardBaseFieldId : uint32_t 翻译
    Paused = 0,
    GridSquareType = 1,
    GridCelLook = 2,
    GridCelOffset = 3,
    GridCelFog = 4,
    GameMode = 5,
    Level = 6,
    SunCount = 7,
    SunCountdown = 8,
    NumSunsFallen = 9,
    CoinBank = 10,
    WaveCount = 11,
    CurrentWave = 12,
    TotalWaves = 13,
    NumWaves = 14,
    ZombiesInWave = 15,
    ZombieAllowed = 16,
    WaveCountdown = 17,
    ZombieCountdown = 18,
    FlagCountdown = 19,
    LevelComplete = 20,
    GameOver = 21,
    GameOverCountdown = 22,
    BoardFadeOutCounter = 23,
    NextSurvivalStageCounter = 24,
    ScoreNextMowerCounter = 25,
    LevelAwardSpawned = 26,
    ProgressMeterWidth = 27,
    FlagRaiseCounter = 28,
    SodPosition = 29,
    PlantRow = 30,
    WaveRowGotLawnMowered = 31,
    RowPickingArray = 32,
    BackgroundType = 33,
    PoolOccupied = 34,
    Roof = 35,
    Fog = 36,
    FogOffset = 37,
    FogBlownCountDown = 38,
    EnableGraveStones = 39,
    SpecialGraveStoneX = 40,
    SpecialGraveStoneY = 41,
    FinalBossKilled = 42,
    ShowShovel = 43,
    KilledYeti = 44,
    Challenge = 45,
    TutorialState = 46,
    IceTimer = 47,
    IceMinX = 48,
    IceTrapCounter = 49,
    FwooshCountDown = 50,
    IceParticle = 51,
    ShakeCounter = 52,
    ShakeAmountX = 53,
    ShakeAmountY = 54,
    PrevMouseX = 55,
    PrevMouseY = 56,
    DebugTextMode = 57,
    GravesCleared = 58,
    PlantsEaten = 59,
    PlantLawnMowerX = 60,
    PoolLawnMowerX = 61,
    RoofLawnMowerX = 62,
    SuperMowerMode = 63,
    StageHas6Rows = 64,
    MainCounter = 65,
    UpdateCount = 66,
    DrawCount = 67,
    EffectCounter = 68,
    RiseFromGraveCounter = 69,
    BoardRandSeed = 70,
    TotalSpawnedWaves = 71,
    Background = 72,
    NeedSaveGame = 73,
    OutOfMoneyCounter = 74,
    LevelInt = 75,
    NumLevels = 76,
    PausedOld = 77,
    SunCountOld = 78,
    SunCountdownOld = 79,
    NumSunsFallenOld = 80,
    CoinBankOld = 81,
    WaveCountOld = 82,
    CurrentWaveOld = 83,
    TotalWavesOld = 84,
    NumWavesOld = 85,
    ZombieAllowedOld = 86,
    EnableBoosts = 87,
    GridCelOffsetFile = 88,
    GridCelLookFile = 89,
    GridCelFogFile = 90,
    PoolInfo = 91,
    PoolCount = 92,
    PoolRow = 93,
    PoolCol = 94,
    PoolType = 95,
    PoolPos = 96,
    PoolGoal = 97,
    PoolTimer = 98,
    PoolParticleID = 99,
    PoolParticle = 100,
    PoolParticleFile = 101,
    PoolHasSplash = 102,
}

// ── 小端序列化辅助函数 ────────────────────────────────

/// 将 u32 以小端格式追加到字节向量（对应 C++ AppendU32LE）
pub fn append_u32_le(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

/// 将数据追加到字节向量（对应 C++ AppendBytes）
pub fn append_bytes(out: &mut Vec<u8>, data: &[u8]) {
    out.extend_from_slice(data);
}

/// 以 TLV 格式追加一个数据块（对应 C++ AppendChunk）
pub fn append_chunk(out: &mut Vec<u8>, chunk_type: u32, chunk_data: &[u8]) {
    append_u32_le(out, chunk_type);
    append_u32_le(out, chunk_data.len() as u32);
    append_bytes(out, chunk_data);
}

// ── TLVReader ─────────────────────────────────────────

/// TLV 格式读取器（对应 C++ TLVReader）
pub struct TLVReader<'a> {
    data: &'a [u8],
    pos: usize,
    ok: bool,
}

impl<'a> TLVReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        TLVReader {
            data,
            pos: 0,
            ok: true,
        }
    }

    /// 读取一个 U32 小端值
    pub fn read_u32(&mut self) -> Option<u32> {
        if self.pos + 4 > self.data.len() {
            self.ok = false;
            return Some(0);
        }
        let value = (self.data[self.pos] as u32)
            | ((self.data[self.pos + 1] as u32) << 8)
            | ((self.data[self.pos + 2] as u32) << 16)
            | ((self.data[self.pos + 3] as u32) << 24);
        self.pos += 4;
        Some(value)
    }

    /// 读取指定长度的字节切片
    pub fn read_bytes(&mut self, len: usize) -> Option<&'a [u8]> {
        if self.pos + len > self.data.len() {
            self.ok = false;
            return None;
        }
        let ptr = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Some(ptr)
    }

    pub fn is_ok(&self) -> bool {
        self.ok
    }

    pub fn remaining(&self) -> usize {
        if self.pos < self.data.len() {
            self.data.len() - self.pos
        } else {
            0
        }
    }
}

// ── PortableSaveContext ───────────────────────────────

/// 可移植存档读写上下文（对应 C++ PortableSaveContext）
/// 封装读写操作，在读取时从 Reader 读取，写入时写入 Writer
pub struct PortableSaveContext {
    pub reading: bool,
    pub failed: bool,
    pub buffer: Option<Buffer>,
}

impl PortableSaveContext {
    pub fn new_reader(buffer: Buffer) -> Self {
        PortableSaveContext {
            reading: true,
            failed: false,
            buffer: Some(buffer),
        }
    }

    pub fn new_writer(buffer: Buffer) -> Self {
        PortableSaveContext {
            reading: false,
            failed: false,
            buffer: Some(buffer),
        }
    }

    pub fn sync_bytes(&mut self, data: &mut [u8]) {
        if let Some(ref mut buf) = self.buffer {
            if self.reading {
                let bytes = buf.read_bytes(data.len());
                if bytes.len() == data.len() {
                    data.copy_from_slice(&bytes);
                } else {
                    self.failed = true;
                    data.fill(0);
                }
            } else {
                buf.write_bytes(data);
            }
        }
    }

    pub fn sync_bytes_const(&mut self, data: &[u8]) {
        if let Some(ref mut buf) = self.buffer {
            if self.reading {
                self.failed = true;
            } else {
                buf.write_bytes(data);
            }
        }
    }

    pub fn sync_bool(&mut self, value: &mut bool) {
        if let Some(ref mut buf) = self.buffer {
            if self.reading {
                *value = buf.read_byte() != 0;
            } else {
                buf.write_byte(if *value { 1 } else { 0 });
            }
        }
    }

    pub fn sync_u32(&mut self, value: &mut u32) {
        if let Some(ref mut buf) = self.buffer {
            if self.reading {
                *value = buf.read_u32();
            } else {
                buf.write_u32(*value);
            }
        }
    }

    pub fn sync_i32(&mut self, value: &mut i32) {
        if let Some(ref mut buf) = self.buffer {
            if self.reading {
                *value = buf.read_i32();
            } else {
                buf.write_i32(*value);
            }
        }
    }

    pub fn sync_f32(&mut self, value: &mut f32) {
        if let Some(ref mut buf) = self.buffer {
            if self.reading {
                *value = buf.read_f32();
            } else {
                buf.write_f32(*value);
            }
        }
    }

    pub fn sync_u64(&mut self, value: &mut u64) {
        let mut low = (*value & 0xFFFFFFFF) as u32;
        let mut high = ((*value >> 32) & 0xFFFFFFFF) as u32;
        self.sync_u32(&mut low);
        self.sync_u32(&mut high);
        if self.reading {
            *value = (high as u64) << 32 | low as u64;
        }
    }

    pub fn sync_i64(&mut self, value: &mut i64) {
        let mut uval = *value as u64;
        self.sync_u64(&mut uval);
        if self.reading {
            *value = uval as i64;
        }
    }

    /// 同步枚举值（对应 C++ 模板 SyncEnum<TEnum>）
    pub fn sync_enum<T: Into<i32> + From<i32> + Copy>(&mut self, value: &mut T) {
        let mut raw = (*value).into();
        self.sync_i32(&mut raw);
        if self.reading {
            *value = T::from(raw);
        }
    }
}

// ── 可移植同步自由函数 ──────────────────────────

/// 同步 GameObject 基类字段（对应 C++ SyncGameObjectPortable）
pub fn sync_game_object_portable(ctx: &mut PortableSaveContext, obj: &mut GameObject) {
    ctx.sync_i32(&mut obj.x);
    ctx.sync_i32(&mut obj.y);
    ctx.sync_i32(&mut obj.width);
    ctx.sync_i32(&mut obj.height);
    ctx.sync_bool(&mut obj.visible);
    ctx.sync_i32(&mut obj.row);
    ctx.sync_i32(&mut obj.render_order);
}

/// 同步 Color（对应 C++ SyncColorPortable）
/// C++ 以 int32 存储每个分量，Rust 以 u8 存储，需转换以保持兼容
pub fn sync_color_portable(ctx: &mut PortableSaveContext, color: &mut Color) {
    let mut r = color.r as i32;
    let mut g = color.g as i32;
    let mut b = color.b as i32;
    let mut a = color.a as i32;
    ctx.sync_i32(&mut r);
    ctx.sync_i32(&mut g);
    ctx.sync_i32(&mut b);
    ctx.sync_i32(&mut a);
    if ctx.reading {
        color.r = r as u8;
        color.g = g as u8;
        color.b = b as u8;
        color.a = a as u8;
    }
}

/// 同步 SexyVector2（对应 C++ SyncVector2Portable）
pub fn sync_vector2_portable(ctx: &mut PortableSaveContext, v: &mut SexyVector2) {
    ctx.sync_f32(&mut v.x);
    ctx.sync_f32(&mut v.y);
}

/// 同步 SexyMatrix3（对应 C++ SyncMatrixPortable）
/// Rust 版 SexyMatrix3 使用 m: [[f32; 3]; 3]，C++ 有 m00…m22 命名成员
pub fn sync_matrix_portable(ctx: &mut PortableSaveContext, m: &mut SexyMatrix3) {
    for row in 0..3 {
        for col in 0..3 {
            ctx.sync_f32(&mut m.m[row][col]);
        }
    }
}

/// 同步 Rect（对应 C++ SyncRectPortable）
pub fn sync_rect_portable(ctx: &mut PortableSaveContext, rect: &mut Rect) {
    ctx.sync_i32(&mut rect.x);
    ctx.sync_i32(&mut rect.y);
    ctx.sync_i32(&mut rect.width);
    ctx.sync_i32(&mut rect.height);
}

/// 以 TLV 格式向输出追加一个字段（对应 C++ AppendFieldWithSync）
/// 创建一个临时写入上下文，执行 writer_fn，然后将结果写入 out
pub fn append_field_with_sync<F>(out: &mut Vec<u8>, field_id: u32, writer_fn: F)
where
    F: FnOnce(&mut PortableSaveContext),
{
    let buf = Buffer::new();
    let mut ctx = PortableSaveContext::new_writer(buf);
    writer_fn(&mut ctx);
    if !ctx.failed {
        let data = ctx
            .buffer
            .as_ref()
            .map(|b| b.data().to_vec())
            .unwrap_or_default();
        append_u32_le(out, field_id);
        append_u32_le(out, data.len() as u32);
        append_bytes(out, &data);
    }
}

/// 从数据中应用一个字段的同步读取（对应 C++ ApplyFieldWithSync）
/// 从 theData 创建读取上下文，调用 reader_fn
pub fn apply_field_with_sync<F>(data: &[u8], reader_fn: F) -> bool
where
    F: FnOnce(&mut PortableSaveContext),
{
    let buf = Buffer::from_bytes(data);
    let mut ctx = PortableSaveContext::new_reader(buf);
    reader_fn(&mut ctx);
    !ctx.failed
}

/// 写入 GameObject 字段（对应 C++ WriteGameObjectField）
pub fn write_game_object_field(out: &mut Vec<u8>, field_id: u32, obj: &mut GameObject) {
    append_field_with_sync(out, field_id, |ctx| {
        sync_game_object_portable(ctx, obj);
    });
}

/// 读取 GameObject 字段（对应 C++ ReadGameObjectField）
pub fn read_game_object_field(data: &[u8], obj: &mut GameObject) -> bool {
    apply_field_with_sync(data, |ctx| {
        sync_game_object_portable(ctx, obj);
    })
}

/// 写入 TLV Blob 到 PortableSaveContext（对应 C++ WriteTLVBlob）
pub fn write_tlv_blob(ctx: &mut PortableSaveContext, blob: &[u8]) {
    let mut size = blob.len() as u32;
    ctx.sync_u32(&mut size);
    if !blob.is_empty() {
        // C++ 用 SyncBytes(const void*, uint32_t)（只写），Rust 有 sync_bytes_const
        if !ctx.reading {
            ctx.sync_bytes_const(blob);
        }
    }
}

/// 从 PortableSaveContext 读取 TLV Blob（对应 C++ ReadTLVBlob）
pub fn read_tlv_blob(ctx: &mut PortableSaveContext) -> Option<Vec<u8>> {
    let mut size: u32 = 0;
    ctx.sync_u32(&mut size);
    if ctx.failed {
        return None;
    }
    if size == 0 {
        return Some(Vec::new());
    }
    let mut blob = vec![0u8; size as usize];
    ctx.sync_bytes(&mut blob);
    if ctx.failed {
        None
    } else {
        Some(blob)
    }
}

// ── SaveGameContext ──────────────────────────────────

/// 旧版存档读写上下文（对应 C++ SaveGameContext）
pub struct SaveGameContext {
    pub buffer: Buffer,
    pub failed: bool,
    pub reading: bool,
}

impl SaveGameContext {
    pub fn new(buffer: Buffer, reading: bool) -> Self {
        SaveGameContext {
            buffer,
            failed: false,
            reading,
        }
    }

    pub fn bytes_left_to_read(&self) -> i32 {
        self.buffer.remaining() as i32
    }

    pub fn sync_bytes(&mut self, dest: &mut [u8]) {
        let read_size = if self.reading {
            if self.bytes_left_to_read() < 4 {
                self.failed = true;
            }
            if self.failed {
                0
            } else {
                self.buffer.read_i32()
            }
        } else {
            self.buffer.write_i32(dest.len() as i32);
            dest.len() as i32
        };

        if self.reading {
            if read_size != dest.len() as i32 || self.bytes_left_to_read() < dest.len() as i32 {
                self.failed = true;
            }
            if self.failed {
                dest.fill(0);
            } else {
                let bytes = self.buffer.read_bytes(dest.len());
                dest.copy_from_slice(&bytes);
            }
        } else {
            self.buffer.write_bytes(dest);
        }
    }

    pub fn sync_int(&mut self, value: &mut i32) {
        if self.reading {
            if self.bytes_left_to_read() < 4 {
                self.failed = true;
            }
            *value = if self.failed { 0 } else { self.buffer.read_i32() };
        } else {
            self.buffer.write_i32(*value);
        }
    }

    pub fn sync_uint(&mut self, value: &mut u32) {
        let mut signed = *value as i32;
        self.sync_int(&mut signed);
        if self.reading {
            *value = signed as u32;
        }
    }

    /// 同步动画定义索引（对应 C++ SyncReanimationDef）
    /// C++ 使用全局数组指针，Rust 使用索引值以绕过类型依赖
    pub fn sync_reanimation_index(&mut self, index: &mut i32) {
        self.sync_int(index);
    }

    /// 同步粒子定义索引（对应 C++ SyncParticleDef）
    /// C++ 使用全局数组指针，Rust 使用索引值以绕过类型依赖
    pub fn sync_particle_index(&mut self, index: &mut i32) {
        self.sync_int(index);
    }

    /// 同步拖尾定义索引（对应 C++ SyncTrailDef）
    /// C++ 使用全局数组指针，Rust 使用索引值以绕过类型依赖
    pub fn sync_trail_index(&mut self, index: &mut i32) {
        self.sync_int(index);
    }

    /// 同步资源 ID（对应 C++ SyncImage）
    /// C++ 使用 ResourceId 枚举，Rust 使用 i32 索引以绕过类型依赖
    pub fn sync_resource_id(&mut self, id: &mut i32) {
        self.sync_int(id);
    }
}

// ── 顶层函数 ──────────────────────────────────────────

/// 使用 zlib 兼容的 CRC32 计算（纯 Rust 实现）
fn crc32_compute(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

/// 从字节数组安全解析 SaveFileHeaderV4
fn parse_header_v4(bytes: &[u8]) -> Option<SaveFileHeaderV4> {
    if bytes.len() < 24 {
        return None;
    }
    let mut magic = [0u8; 12];
    magic.copy_from_slice(&bytes[0..12]);
    let version = u32::from_le_bytes(bytes[12..16].try_into().ok()?);
    let payload_size = u32::from_le_bytes(bytes[16..20].try_into().ok()?);
    let payload_crc = u32::from_le_bytes(bytes[20..24].try_into().ok()?);
    Some(SaveFileHeaderV4 {
        magic,
        version,
        payload_size,
        payload_crc,
    })
}

/// 将 SaveFileHeaderV4 序列化为字节数组
fn header_v4_to_bytes(header: &SaveFileHeaderV4) -> [u8; 24] {
    let mut bytes = [0u8; 24];
    bytes[0..12].copy_from_slice(&header.magic);
    bytes[12..16].copy_from_slice(&header.version.to_le_bytes());
    bytes[16..20].copy_from_slice(&header.payload_size.to_le_bytes());
    bytes[20..24].copy_from_slice(&header.payload_crc.to_le_bytes());
    bytes
}

/// 加载游戏存档（对应 C++ LawnLoadGame）
/// 实现 V4 格式文件头验证和 chunk 分发
pub fn lawn_load_game(board: Option<*mut Board>, file_path: &str) -> bool {
    // 首先读取文件
    let file_data = match std::fs::read(file_path) {
        Ok(data) => data,
        Err(_) => return false,
    };

    // 尝试 V4 格式加载
    if file_data.len() < 24 {
        return false;
    }

    // 解析并验证 SaveFileHeaderV4
    let header = match parse_header_v4(&file_data[..24]) {
        Some(h) => h,
        None => return false,
    };

    // 验证 magic
    if &header.magic != b"PVZP_SAVE4\0\0" {
        return false;
    }
    if header.version != SAVE_FILE_V4_VERSION {
        return false;
    }
    let payload_size = header.payload_size as usize;
    if 24 + payload_size > file_data.len() {
        return false;
    }

    let payload = &file_data[24..][..payload_size];
    let actual_crc = crc32_compute(payload);
    if actual_crc != header.payload_crc {
        return false;
    }

    // 用 TLVReader 解析 payload
    let mut a_reader = TLVReader::new(payload);
    let mut base_loaded = false;

    while a_reader.is_ok() && a_reader.remaining() > 0 {
        let chunk_type = match a_reader.read_u32() {
            Some(t) => t,
            None => break,
        };
        let chunk_size = match a_reader.read_u32() {
            Some(s) => s,
            None => break,
        };
        let chunk_data = match a_reader.read_bytes(chunk_size as usize) {
            Some(d) => d,
            None => break,
        };

        // 分发到对应 chunk 处理器
        if let Some(b) = board {
            unsafe {
                let _board = &mut *b;
                if !read_chunk_v4(chunk_type, chunk_data, _board) {
                    return false;
                }
            }
        }
        if chunk_type == SaveChunkTypeV4::BoardBase as u32 {
            base_loaded = true;
        }
    }

    if !base_loaded {
        return false;
    }

    // V4 加载格式验证成功
    true
}

/// 内部：读取并处理一个 V4 chunk（对应 C++ ReadChunkV4）
/// 由于 Board 内部字段尚未完整翻译，当前仅做格式解析占位
fn read_chunk_v4(_chunk_type: u32, _data: &[u8], _board: &mut Board) -> bool {
    // 待 Board 翻译完成后实现各 chunk 的同步
    // 当前返回 true 以允许 V4 文件头验证通过
    true
}

/// 保存游戏存档（对应 C++ LawnSaveGame）
/// 实现 V4 格式文件写入
pub fn lawn_save_game(board: Option<*mut Board>, file_path: &str) -> bool {
    // 构建所有 chunk 的 payload
    let mut payload: Vec<u8> = Vec::new();

    if let Some(b) = board {
        unsafe {
            let _board = &mut *b;
            // 目前各 chunk 写入尚需 Board 类型翻译完成后实现
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::BoardBase as u32, _board) {
                return false;
            }
        }
    }

    // 构建文件头并写入文件
    let mut header = SaveFileHeaderV4 {
        magic: *b"PVZP_SAVE4\0\0",
        version: SAVE_FILE_V4_VERSION,
        payload_size: payload.len() as u32,
        payload_crc: 0,
    };

    // 计算 CRC
    header.payload_crc = crc32_compute(&payload);

    let header_bytes = header_v4_to_bytes(&header);
    let mut out_data = Vec::with_capacity(24 + payload.len());
    out_data.extend_from_slice(&header_bytes);
    out_data.extend_from_slice(&payload);

    std::fs::write(file_path, &out_data).is_ok()
}

/// 内部：写入一个 V4 chunk（对应 C++ WriteChunkV4）
/// 由于 Board 内部字段尚未完整翻译，当前仅做占位
fn write_chunk_v4(_payload: &mut Vec<u8>, _chunk_type: u32, _board: &mut Board) -> bool {
    // 待 Board 翻译完成后实现各 chunk 的写入
    true
}
