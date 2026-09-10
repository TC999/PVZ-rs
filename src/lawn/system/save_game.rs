// PvZ Portable Rust 翻译 — 游戏存档系统
// 对应 C++ src/Lawn/System/SaveGame.cpp（内部数据结构）

#![allow(dead_code)]

use crate::framework::buffer::Buffer;
use crate::framework::color::Color;
use crate::framework::common::SexyVector2;
use crate::framework::rect::Rect;
use crate::framework::sexy_matrix::SexyMatrix3;
use crate::lawn::board::Board;
use crate::lawn::board::{MAX_GRID_SIZE_X, MAX_GRID_SIZE_Y, MAX_ZOMBIES_IN_WAVE, MAX_ZOMBIE_WAVES};
use crate::lawn::challenge::Challenge;
use crate::lawn::coin::Coin;
use crate::lawn::cursor_object::{CursorObject, CursorPreview};
use crate::lawn::grid_item::{GridItem, NUM_MOTION_TRAIL_FRAMES};
use crate::lawn::lawn_mower::LawnMower;
use crate::lawn::plant::{Plant, MAX_MAGNET_ITEMS};
use crate::lawn::projectile::Projectile;
use crate::lawn::seed_packet::SeedPacket;
use crate::lawn::system::music::Music;
use crate::lawn::system::player_info::PottedPlant;
use crate::lawn::widget::message_widget::{MessageWidget, MAX_MESSAGE_LENGTH};
use crate::lawn::zombie::{Zombie, MAX_ZOMBIE_FOLLOWERS};
use crate::lawn::game_enums::*;
use crate::lawn::game_object::GameObject;
use crate::todlib::tod_common::TodSmoothArray;

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
    // 对应 C++ BoardBaseFieldId（SaveGame.cpp:1448），PAUSED=1 递增至 CHOCOLATE_COLLECTED=103
    Paused = 1,
    GridSquareType,
    GridCelLook,
    GridCelOffset,
    GridCelFog,
    EnableGravestones,
    SpecialGravestoneX,
    SpecialGravestoneY,
    FogOffset,
    FogBlownCountdown,
    PlantRow,
    WaveRowGotLawnMowered,
    BonusLawnMowersRemaining,
    IceMinX,
    IceTimer,
    IceParticleId,
    RowPickingArray,
    ZombiesInWave,
    ZombieAllowed,
    SunCountdown,
    NumSunsFallen,
    ShakeCounter,
    ShakeAmountX,
    ShakeAmountY,
    BackgroundType,
    Level,
    SodPosition,
    PrevMouseX,
    PrevMouseY,
    SunMoney,
    NumWaves,
    MainCounter,
    EffectCounter,
    DrawCount,
    RiseFromGraveCounter,
    OutOfMoneyCounter,
    CurrentWave,
    TotalSpawnedWaves,
    TutorialState,
    TutorialParticleId,
    TutorialTimer,
    LastBungeeWave,
    ZombieHealthToNextWave,
    ZombieHealthWaveStart,
    ZombieCountdown,
    ZombieCountdownStart,
    HugeWaveCountdown,
    HelpDisplayed,
    HelpIndex,
    FinalBossKilled,
    ShowShovel,
    CoinBankFadeCount,
    DebugTextMode,
    LevelComplete,
    BoardFadeOutCounter,
    NextSurvivalStageCounter,
    ScoreNextMowerCounter,
    LevelAwardSpawned,
    ProgressMeterWidth,
    FlagRaiseCounter,
    IceTrapCounter,
    BoardRandSeed,
    PoolSparklyParticleId,
    FwooshId,
    FwooshCountdown,
    TimeStopCounter,
    DroppedFirstCoin,
    FinalWaveSoundCounter,
    CobCannonCursorDelayCounter,
    CobCannonMouseX,
    CobCannonMouseY,
    KilledYeti,
    MustacheMode,
    SuperMowerMode,
    FutureMode,
    PinataMode,
    DanceMode,
    DaisyMode,
    SukhbirMode,
    PrevBoardResult,
    TriggeredLawnMowers,
    PlayTimeActiveLevel,
    PlayTimeInactiveLevel,
    MaxSunPlants,
    StartDrawTime,
    IntervalDrawTime,
    IntervalDrawCountStart,
    MinFps,
    PreloadTime,
    GameId,
    GravesCleared,
    PlantsEaten,
    PlantsShoveled,
    PeaShooterUsed,
    CatapultPlantsUsed,
    MushroomAndCoffeeBeansOnly,
    MushroomsUsed,
    LevelCoinsCollected,
    GargantuarsKillsByCornCob,
    CoinsCollected,
    DiamondsCollected,
    PottedPlantsCollected,
    ChocolateCollected,
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
    /// 仅支持 4 字节（#[repr(i32)]）枚举；经 i32 中转，读写方向均用 transmute_copy
    pub fn sync_enum<T: Copy + 'static>(&mut self, value: &mut T) {
        assert!(
            std::mem::size_of::<T>() == std::mem::size_of::<i32>(),
            "sync_enum 仅支持 4 字节（#[repr(i32)]）枚举"
        );
        let mut raw: i32 = unsafe { std::mem::transmute_copy(value) };
        self.sync_i32(&mut raw);
        if self.reading {
            *value = unsafe { std::mem::transmute_copy(&raw) };
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

/// 内部：读取并处理一个 V4 chunk（对应 C++ ReadChunkV4，SaveGame.cpp:2360）
/// TLV 循环查找 fieldId==1 的字段，以读取上下文应用 Board 同步
fn read_chunk_v4(chunk_type: u32, data: &[u8], board: &mut Board) -> bool {
    let chunk = match save_chunk_from_id(chunk_type) {
        Some(c) => c,
        None => return true, // C++ GetChunkSyncFn 无同步函数的 chunk 跳过
    };
    match chunk {
        SaveChunkTypeV4::BoardBase => {}
        SaveChunkTypeV4::Coins => {}
        SaveChunkTypeV4::Mowers => {}
        SaveChunkTypeV4::Projectiles => {}
        SaveChunkTypeV4::GridItems => {}
        SaveChunkTypeV4::Plants => {}
        SaveChunkTypeV4::Zombies => {}
        SaveChunkTypeV4::Cursor => {}
        SaveChunkTypeV4::CursorPreview => {}
        SaveChunkTypeV4::Advice => {}
        SaveChunkTypeV4::SeedBank => {}
        SaveChunkTypeV4::SeedPackets => {}
        SaveChunkTypeV4::Challenge => {}
        SaveChunkTypeV4::Music => {}
        _ => return true,
    }
    if data.len() < 4 {
        return false;
    }

    let mut a_reader = TLVReader::new(data);
    let a_chunk_version = match a_reader.read_u32() {
        Some(v) => v,
        None => return false,
    };
    if a_chunk_version != SAVE4_CHUNK_VERSION {
        return false;
    }

    let mut a_applied = false;
    while a_reader.is_ok() && a_reader.remaining() > 0 {
        let field_id = match a_reader.read_u32() {
            Some(v) => v,
            None => break,
        };
        let field_size = match a_reader.read_u32() {
            Some(v) => v,
            None => break,
        };
        let field_data = match a_reader.read_bytes(field_size as usize) {
            Some(d) => d,
            None => break,
        };

        if field_id == 1 {
            let buf = Buffer::from_bytes(field_data);
            let mut a_context = PortableSaveContext::new_reader(buf);
            match chunk {
                SaveChunkTypeV4::BoardBase => sync_board_base_portable(&mut a_context, board),
                SaveChunkTypeV4::Coins => sync_coins_portable(&mut a_context, board),
                SaveChunkTypeV4::Mowers => sync_mowers_portable(&mut a_context, board),
                SaveChunkTypeV4::Projectiles => sync_projectiles_portable(&mut a_context, board),
                SaveChunkTypeV4::GridItems => sync_grid_items_portable(&mut a_context, board),
                SaveChunkTypeV4::Plants => sync_plants_portable(&mut a_context, board),
                SaveChunkTypeV4::Zombies => sync_zombies_portable(&mut a_context, board),
                SaveChunkTypeV4::Cursor => sync_cursor_portable(&mut a_context, board),
                SaveChunkTypeV4::CursorPreview => sync_cursor_preview_portable(&mut a_context),
                SaveChunkTypeV4::Advice => sync_advice_portable(&mut a_context, board),
                SaveChunkTypeV4::SeedBank => sync_seed_bank_portable(&mut a_context, board),
                SaveChunkTypeV4::SeedPackets => sync_seed_packets_portable(&mut a_context, board),
                SaveChunkTypeV4::Challenge => sync_challenge_portable(&mut a_context, board),
                SaveChunkTypeV4::Music => sync_music_portable(&mut a_context, board),
                _ => {}
            }
            if a_context.failed {
                return false;
            }
            a_applied = true;
        }
    }

    a_applied
}

/// 保存游戏存档（对应 C++ LawnSaveGame）
/// 实现 V4 格式文件写入
pub fn lawn_save_game(board: Option<*mut Board>, file_path: &str) -> bool {
    // 构建所有 chunk 的 payload
    let mut payload: Vec<u8> = Vec::new();

    if let Some(b) = board {
        unsafe {
            let _board = &mut *b;
            // 对应 C++ LawnSaveGame（SaveGame.cpp:3117-3136）：依次写入全部 chunk
            // （当前仅 BoardBase 有同步实现，其余 chunk 经 GetChunkSyncFn 语义跳过）
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::BoardBase as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Zombies as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Plants as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Projectiles as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Coins as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Mowers as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::GridItems as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::ParticleEmitters as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::ParticleParticles as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::ParticleSystems as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Reanimations as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Trails as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Attachments as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Cursor as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::CursorPreview as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Advice as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::SeedBank as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::SeedPackets as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Challenge as u32, _board) {
                return false;
            }
            if !write_chunk_v4(&mut payload, SaveChunkTypeV4::Music as u32, _board) {
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

// ── 数组同步辅助（对应 C++ Sync*Array 模板，SaveGame.cpp:579-625） ──

fn sync_i32_array(ctx: &mut PortableSaveContext, data: &mut [i32]) {
    for v in data.iter_mut() {
        ctx.sync_i32(v);
    }
}

fn sync_bool_array(ctx: &mut PortableSaveContext, data: &mut [bool]) {
    for v in data.iter_mut() {
        ctx.sync_bool(v);
    }
}

fn sync_enum_array<T: Copy + 'static>(ctx: &mut PortableSaveContext, data: &mut [T]) {
    for v in data.iter_mut() {
        ctx.sync_enum(v);
    }
}

fn sync_u32_array(ctx: &mut PortableSaveContext, data: &mut [u32]) {
    for v in data.iter_mut() {
        ctx.sync_u32(v);
    }
}

/// 同步 PvzpSmoothArray（对应 C++ SyncPvzpSmoothArray，SaveGame.cpp:612）
fn sync_pvzp_smooth_array(ctx: &mut PortableSaveContext, arr: &mut TodSmoothArray) {
    ctx.sync_i32(&mut arr.item);
    ctx.sync_f32(&mut arr.weight);
    ctx.sync_f32(&mut arr.last_picked);
    ctx.sync_f32(&mut arr.second_last_picked);
}

/// 同步 PvzpSmoothArray 列表（对应 C++ SyncPvzpSmoothArrayList，SaveGame.cpp:618）
fn sync_pvzp_smooth_array_list(ctx: &mut PortableSaveContext, data: &mut [TodSmoothArray]) {
    for v in data.iter_mut() {
        sync_pvzp_smooth_array(ctx, v);
    }
}

// ── BoardBase chunk（对应 C++ SyncBoardBasePortable，SaveGame.cpp:1555） ──

const ZOMBIE_ALLOWED_COUNT: usize = 100; // C++ SyncBoolArray(&mZombieAllowed[0], 100)

/// 写入 BoardBase 字段 blob（对应 C++ SyncBoardBasePortable 写入分支的 AppendFieldWithSync 序列）
fn append_board_base_fields(a_blob: &mut Vec<u8>, board: &mut Board) {
    // 缺失字段占位变量（C++ 字段存在但 Rust Board 未翻译命名）：保持存档格式兼容
    let mut tmp_i32 = 0i32;
    let mut tmp_u32 = 0u32;
    let mut tmp_f32 = 0.0f32;
    let mut tmp_i64 = 0i64;

    append_field_with_sync(a_blob, BoardBaseFieldId::Paused as u32, |c| c.sync_bool(&mut board.m_paused));
    append_field_with_sync(a_blob, BoardBaseFieldId::GridSquareType as u32, |c| {
        for row in 0..MAX_GRID_SIZE_Y {
            for col in 0..MAX_GRID_SIZE_X {
                c.sync_enum(&mut board.grid_square_type[row][col]);
            }
        }
    });
    append_field_with_sync(a_blob, BoardBaseFieldId::GridCelLook as u32, |c| {
        for row in 0..MAX_GRID_SIZE_Y {
            for col in 0..MAX_GRID_SIZE_X {
                c.sync_i32(&mut board.grid_cel_look[row][col]);
            }
        }
    });
    append_field_with_sync(a_blob, BoardBaseFieldId::GridCelOffset as u32, |c| {
        for row in 0..MAX_GRID_SIZE_Y {
            for col in 0..MAX_GRID_SIZE_X {
                for sub in 0..2 {
                    c.sync_i32(&mut board.grid_cel_offset[row][col][sub]);
                }
            }
        }
    });
    append_field_with_sync(a_blob, BoardBaseFieldId::GridCelFog as u32, |c| {
        for col in 0..MAX_GRID_SIZE_X {
            for row in 0..(MAX_GRID_SIZE_Y + 1) {
                c.sync_i32(&mut board.grid_cel_fog[col][row]);
            }
        }
    });
    append_field_with_sync(a_blob, BoardBaseFieldId::EnableGravestones as u32, |c| c.sync_bool(&mut board.m_enable_grave_stones));
    append_field_with_sync(a_blob, BoardBaseFieldId::SpecialGravestoneX as u32, |c| c.sync_i32(&mut board.m_special_grave_stone_x));
    append_field_with_sync(a_blob, BoardBaseFieldId::SpecialGravestoneY as u32, |c| c.sync_i32(&mut board.m_special_grave_stone_y));
    append_field_with_sync(a_blob, BoardBaseFieldId::FogOffset as u32, |c| c.sync_f32(&mut board.m_fog_offset));
    append_field_with_sync(a_blob, BoardBaseFieldId::FogBlownCountdown as u32, |c| c.sync_i32(&mut board.m_fog_blown_count_down));
    append_field_with_sync(a_blob, BoardBaseFieldId::PlantRow as u32, |c| {
        for row in 0..MAX_GRID_SIZE_Y {
            c.sync_enum(&mut board.m_plant_row[row]);
        }
    });
    append_field_with_sync(a_blob, BoardBaseFieldId::WaveRowGotLawnMowered as u32, |c| {
        sync_i32_array(c, &mut board.m_wave_row_got_lawn_mowered)
    });
    // [TRANSLATION_NOTE]: mBonusLawnMowersRemaining 未翻译为 Board 字段，占位保持格式
    append_field_with_sync(a_blob, BoardBaseFieldId::BonusLawnMowersRemaining as u32, |c| c.sync_i32(&mut tmp_i32));
    append_field_with_sync(a_blob, BoardBaseFieldId::IceMinX as u32, |c| sync_i32_array(c, &mut board.m_ice_min_x));
    append_field_with_sync(a_blob, BoardBaseFieldId::IceTimer as u32, |c| sync_i32_array(c, &mut board.m_ice_timer));
    // [TRANSLATION_NOTE]: mIceParticleID[6] 未翻译（Rust 仅 m_ice_particle 单值），循环占位
    append_field_with_sync(a_blob, BoardBaseFieldId::IceParticleId as u32, |c| {
        for _ in 0..MAX_GRID_SIZE_Y {
            c.sync_u32(&mut tmp_u32);
        }
    });
    append_field_with_sync(a_blob, BoardBaseFieldId::RowPickingArray as u32, |c| {
        sync_pvzp_smooth_array_list(c, &mut board.m_row_picking_array)
    });
    append_field_with_sync(a_blob, BoardBaseFieldId::ZombiesInWave as u32, |c| {
        for wave in 0..MAX_ZOMBIE_WAVES {
            for slot in 0..MAX_ZOMBIES_IN_WAVE {
                c.sync_enum(&mut board.m_zombies_in_wave[wave][slot]);
            }
        }
    });
    // C++: SyncBoolArray(&mZombieAllowed[0], 100)；Rust 数组为 NUM_ZOMBIE_TYPES(33)，超出部分占位
    append_field_with_sync(a_blob, BoardBaseFieldId::ZombieAllowed as u32, |c| {
        for i in 0..ZOMBIE_ALLOWED_COUNT {
            let mut v = if i < board.m_zombie_allowed.len() { board.m_zombie_allowed[i] } else { false };
            c.sync_bool(&mut v);
            if i < board.m_zombie_allowed.len() {
                board.m_zombie_allowed[i] = v;
            }
        }
    });
    append_field_with_sync(a_blob, BoardBaseFieldId::SunCountdown as u32, |c| c.sync_i32(&mut board.m_sun_countdown));
    append_field_with_sync(a_blob, BoardBaseFieldId::NumSunsFallen as u32, |c| c.sync_i32(&mut board.m_num_suns_fallen));
    append_field_with_sync(a_blob, BoardBaseFieldId::ShakeCounter as u32, |c| c.sync_i32(&mut board.m_shake_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::ShakeAmountX as u32, |c| c.sync_i32(&mut board.m_shake_amount_x));
    append_field_with_sync(a_blob, BoardBaseFieldId::ShakeAmountY as u32, |c| c.sync_i32(&mut board.m_shake_amount_y));
    append_field_with_sync(a_blob, BoardBaseFieldId::BackgroundType as u32, |c| c.sync_enum(&mut board.m_background_type));
    append_field_with_sync(a_blob, BoardBaseFieldId::Level as u32, |c| c.sync_i32(&mut board.level));
    append_field_with_sync(a_blob, BoardBaseFieldId::SodPosition as u32, |c| c.sync_i32(&mut board.m_sod_position));
    append_field_with_sync(a_blob, BoardBaseFieldId::PrevMouseX as u32, |c| c.sync_i32(&mut board.m_prev_mouse_x));
    append_field_with_sync(a_blob, BoardBaseFieldId::PrevMouseY as u32, |c| c.sync_i32(&mut board.m_prev_mouse_y));
    append_field_with_sync(a_blob, BoardBaseFieldId::SunMoney as u32, |c| c.sync_i32(&mut board.m_sun_money));
    append_field_with_sync(a_blob, BoardBaseFieldId::NumWaves as u32, |c| c.sync_i32(&mut board.m_num_waves));
    append_field_with_sync(a_blob, BoardBaseFieldId::MainCounter as u32, |c| c.sync_u32(&mut board.m_main_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::EffectCounter as u32, |c| c.sync_u32(&mut board.m_effect_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::DrawCount as u32, |c| c.sync_u32(&mut board.m_draw_count));
    append_field_with_sync(a_blob, BoardBaseFieldId::RiseFromGraveCounter as u32, |c| c.sync_i32(&mut board.m_rise_from_grave_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::OutOfMoneyCounter as u32, |c| c.sync_i32(&mut board.m_out_of_money_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::CurrentWave as u32, |c| c.sync_i32(&mut board.m_current_wave));
    append_field_with_sync(a_blob, BoardBaseFieldId::TotalSpawnedWaves as u32, |c| c.sync_i32(&mut board.m_total_spawned_waves));
    append_field_with_sync(a_blob, BoardBaseFieldId::TutorialState as u32, |c| c.sync_enum(&mut board.m_tutorial_state));
    append_field_with_sync(a_blob, BoardBaseFieldId::TutorialParticleId as u32, |c| c.sync_u32(&mut board.m_tutorial_particle_id));
    append_field_with_sync(a_blob, BoardBaseFieldId::TutorialTimer as u32, |c| c.sync_i32(&mut board.m_tutorial_timer));
    // [TRANSLATION_NOTE]: mLastBungeeWave 未翻译为 Board 字段，占位保持格式
    append_field_with_sync(a_blob, BoardBaseFieldId::LastBungeeWave as u32, |c| c.sync_i32(&mut tmp_i32));
    append_field_with_sync(a_blob, BoardBaseFieldId::ZombieHealthToNextWave as u32, |c| c.sync_i32(&mut board.m_zombie_health_to_next_wave));
    append_field_with_sync(a_blob, BoardBaseFieldId::ZombieHealthWaveStart as u32, |c| c.sync_i32(&mut board.m_zombie_health_wave_start));
    append_field_with_sync(a_blob, BoardBaseFieldId::ZombieCountdown as u32, |c| c.sync_i32(&mut board.m_zombie_count_down));
    append_field_with_sync(a_blob, BoardBaseFieldId::ZombieCountdownStart as u32, |c| c.sync_i32(&mut board.m_zombie_count_down_start));
    append_field_with_sync(a_blob, BoardBaseFieldId::HugeWaveCountdown as u32, |c| c.sync_i32(&mut board.m_huge_wave_count_down));
    append_field_with_sync(a_blob, BoardBaseFieldId::HelpDisplayed as u32, |c| {
        sync_bool_array(c, &mut board.m_help_displayed)
    });
    append_field_with_sync(a_blob, BoardBaseFieldId::HelpIndex as u32, |c| c.sync_enum(&mut board.m_advice));
    append_field_with_sync(a_blob, BoardBaseFieldId::FinalBossKilled as u32, |c| c.sync_bool(&mut board.m_final_boss_killed));
    append_field_with_sync(a_blob, BoardBaseFieldId::ShowShovel as u32, |c| c.sync_bool(&mut board.m_show_shovel));
    append_field_with_sync(a_blob, BoardBaseFieldId::CoinBankFadeCount as u32, |c| c.sync_i32(&mut board.m_coin_bank_fade_count));
    append_field_with_sync(a_blob, BoardBaseFieldId::DebugTextMode as u32, |c| c.sync_enum(&mut board.m_debug_text_mode));
    append_field_with_sync(a_blob, BoardBaseFieldId::LevelComplete as u32, |c| c.sync_bool(&mut board.m_level_complete));
    append_field_with_sync(a_blob, BoardBaseFieldId::BoardFadeOutCounter as u32, |c| c.sync_i32(&mut board.m_board_fade_out_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::NextSurvivalStageCounter as u32, |c| c.sync_i32(&mut board.m_next_survival_stage_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::ScoreNextMowerCounter as u32, |c| c.sync_i32(&mut board.m_score_next_mower_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::LevelAwardSpawned as u32, |c| c.sync_bool(&mut board.m_level_award_spawned));
    append_field_with_sync(a_blob, BoardBaseFieldId::ProgressMeterWidth as u32, |c| c.sync_i32(&mut board.m_progress_meter_width));
    append_field_with_sync(a_blob, BoardBaseFieldId::FlagRaiseCounter as u32, |c| c.sync_i32(&mut board.m_flag_raise_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::IceTrapCounter as u32, |c| c.sync_i32(&mut board.m_ice_trap_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::BoardRandSeed as u32, |c| {
        let mut raw = board.m_board_rand_seed as i32;
        c.sync_i32(&mut raw);
        board.m_board_rand_seed = raw as u32;
    });
    // [TRANSLATION_NOTE]: mPoolSparklyParticleID 未翻译为 Board 字段，占位保持格式
    append_field_with_sync(a_blob, BoardBaseFieldId::PoolSparklyParticleId as u32, |c| c.sync_u32(&mut tmp_u32));
    append_field_with_sync(a_blob, BoardBaseFieldId::FwooshId as u32, |c| {
        for row in 0..MAX_GRID_SIZE_Y {
            for slot in 0..12 {
                c.sync_u32(&mut board.m_fwoosh_id[row][slot]);
            }
        }
    });
    append_field_with_sync(a_blob, BoardBaseFieldId::FwooshCountdown as u32, |c| c.sync_i32(&mut board.m_fwoosh_count_down));
    append_field_with_sync(a_blob, BoardBaseFieldId::TimeStopCounter as u32, |c| c.sync_i32(&mut board.m_time_stop_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::DroppedFirstCoin as u32, |c| c.sync_bool(&mut board.m_dropped_first_coin));
    append_field_with_sync(a_blob, BoardBaseFieldId::FinalWaveSoundCounter as u32, |c| c.sync_i32(&mut board.m_final_wave_sound_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::CobCannonCursorDelayCounter as u32, |c| c.sync_i32(&mut board.m_cob_cannon_cursor_delay_counter));
    append_field_with_sync(a_blob, BoardBaseFieldId::CobCannonMouseX as u32, |c| c.sync_i32(&mut board.m_cob_cannon_mouse_x));
    append_field_with_sync(a_blob, BoardBaseFieldId::CobCannonMouseY as u32, |c| c.sync_i32(&mut board.m_cob_cannon_mouse_y));
    append_field_with_sync(a_blob, BoardBaseFieldId::KilledYeti as u32, |c| c.sync_bool(&mut board.m_killed_yeti));
    append_field_with_sync(a_blob, BoardBaseFieldId::MustacheMode as u32, |c| c.sync_bool(&mut board.m_mustache_mode));
    append_field_with_sync(a_blob, BoardBaseFieldId::SuperMowerMode as u32, |c| c.sync_bool(&mut board.m_super_mower_mode));
    append_field_with_sync(a_blob, BoardBaseFieldId::FutureMode as u32, |c| c.sync_bool(&mut board.m_future_mode));
    append_field_with_sync(a_blob, BoardBaseFieldId::PinataMode as u32, |c| c.sync_bool(&mut board.m_pinata_mode));
    append_field_with_sync(a_blob, BoardBaseFieldId::DanceMode as u32, |c| c.sync_bool(&mut board.m_dance_mode));
    append_field_with_sync(a_blob, BoardBaseFieldId::DaisyMode as u32, |c| c.sync_bool(&mut board.m_daisy_mode));
    append_field_with_sync(a_blob, BoardBaseFieldId::SukhbirMode as u32, |c| c.sync_bool(&mut board.m_sukhbir_mode));
    // [TRANSLATION_NOTE]: mPrevBoardResult 未翻译为 Board 字段（Rust 有 m_board_result），占位保持格式
    append_field_with_sync(a_blob, BoardBaseFieldId::PrevBoardResult as u32, |c| c.sync_i32(&mut tmp_i32));
    append_field_with_sync(a_blob, BoardBaseFieldId::TriggeredLawnMowers as u32, |c| c.sync_i32(&mut board.m_triggered_lawn_mowers));
    // [TRANSLATION_NOTE]: mPlayTimeActiveLevel/mPlayTimeInactiveLevel/mMaxSunPlants 未翻译，占位保持格式
    append_field_with_sync(a_blob, BoardBaseFieldId::PlayTimeActiveLevel as u32, |c| c.sync_u32(&mut tmp_u32));
    append_field_with_sync(a_blob, BoardBaseFieldId::PlayTimeInactiveLevel as u32, |c| c.sync_u32(&mut tmp_u32));
    append_field_with_sync(a_blob, BoardBaseFieldId::MaxSunPlants as u32, |c| c.sync_i32(&mut tmp_i32));
    append_field_with_sync(a_blob, BoardBaseFieldId::StartDrawTime as u32, |c| c.sync_i64(&mut board.m_start_draw_time));
    append_field_with_sync(a_blob, BoardBaseFieldId::IntervalDrawTime as u32, |c| c.sync_i64(&mut board.m_interval_draw_time));
    append_field_with_sync(a_blob, BoardBaseFieldId::IntervalDrawCountStart as u32, |c| c.sync_u32(&mut board.m_interval_draw_count_start));
    // [TRANSLATION_NOTE]: mMinFPS/mPreloadTime/mGameID（intptr_t）未翻译，占位保持格式
    append_field_with_sync(a_blob, BoardBaseFieldId::MinFps as u32, |c| c.sync_f32(&mut tmp_f32));
    append_field_with_sync(a_blob, BoardBaseFieldId::PreloadTime as u32, |c| c.sync_i32(&mut tmp_i32));
    append_field_with_sync(a_blob, BoardBaseFieldId::GameId as u32, |c| c.sync_i64(&mut tmp_i64));
    append_field_with_sync(a_blob, BoardBaseFieldId::GravesCleared as u32, |c| c.sync_u32(&mut board.m_graves_cleared));
    append_field_with_sync(a_blob, BoardBaseFieldId::PlantsEaten as u32, |c| c.sync_u32(&mut board.m_plants_eaten));
    append_field_with_sync(a_blob, BoardBaseFieldId::PlantsShoveled as u32, |c| c.sync_u32(&mut board.m_plants_shoveled));
    append_field_with_sync(a_blob, BoardBaseFieldId::PeaShooterUsed as u32, |c| c.sync_bool(&mut board.m_pea_shooter_used));
    append_field_with_sync(a_blob, BoardBaseFieldId::CatapultPlantsUsed as u32, |c| c.sync_bool(&mut board.m_catapult_plants_used));
    append_field_with_sync(a_blob, BoardBaseFieldId::MushroomAndCoffeeBeansOnly as u32, |c| c.sync_bool(&mut board.m_mushroom_and_coffee_beans_only));
    append_field_with_sync(a_blob, BoardBaseFieldId::MushroomsUsed as u32, |c| c.sync_bool(&mut board.m_mushrooms_used));
    append_field_with_sync(a_blob, BoardBaseFieldId::LevelCoinsCollected as u32, |c| c.sync_i32(&mut board.m_level_coins_collected));
    // [TRANSLATION_NOTE]: mGargantuarsKillsByCornCob 未翻译为 Board 字段，占位保持格式
    append_field_with_sync(a_blob, BoardBaseFieldId::GargantuarsKillsByCornCob as u32, |c| c.sync_u32(&mut tmp_u32));
    append_field_with_sync(a_blob, BoardBaseFieldId::CoinsCollected as u32, |c| c.sync_i32(&mut board.m_coins_collected));
    append_field_with_sync(a_blob, BoardBaseFieldId::DiamondsCollected as u32, |c| c.sync_i32(&mut board.m_diamonds_collected));
    append_field_with_sync(a_blob, BoardBaseFieldId::PottedPlantsCollected as u32, |c| c.sync_i32(&mut board.m_potted_plants_collected));
    append_field_with_sync(a_blob, BoardBaseFieldId::ChocolateCollected as u32, |c| c.sync_i32(&mut board.m_chocolate_collected));
    let _ = tmp_i64;
}

/// 应用一个 BoardBase 字段（读取侧，对应 C++ SyncBoardBasePortable 读取分支的 switch case）
/// 将存档 field_id 映射为 BoardBaseFieldId（判别式连续 1..=103，范围内 transmute 合法）
fn board_base_field_from_id(id: u32) -> Option<BoardBaseFieldId> {
    if id >= 1 && id <= 103 {
        Some(unsafe { std::mem::transmute::<u32, BoardBaseFieldId>(id) })
    } else {
        None
    }
}

fn apply_board_base_field(field_id: u32, data: &[u8], board: &mut Board) {
    // 缺失字段占位变量（读取后丢弃，保持存档格式兼容）
    let mut tmp_i32 = 0i32;
    let mut tmp_u32 = 0u32;
    let mut tmp_f32 = 0.0f32;
    let mut tmp_i64 = 0i64;
    let field = match board_base_field_from_id(field_id) {
        Some(f) => f,
        None => return, // C++ default break
    };
    match field {
        BoardBaseFieldId::Paused => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_paused)); }
        BoardBaseFieldId::GridSquareType => {
            apply_field_with_sync(data, |c| for row in 0..MAX_GRID_SIZE_Y { for col in 0..MAX_GRID_SIZE_X { c.sync_enum(&mut board.grid_square_type[row][col]); } });
        }
        BoardBaseFieldId::GridCelLook => {
            apply_field_with_sync(data, |c| for row in 0..MAX_GRID_SIZE_Y { for col in 0..MAX_GRID_SIZE_X { c.sync_i32(&mut board.grid_cel_look[row][col]); } });
        }
        BoardBaseFieldId::GridCelOffset => {
            apply_field_with_sync(data, |c| for row in 0..MAX_GRID_SIZE_Y { for col in 0..MAX_GRID_SIZE_X { for sub in 0..2 { c.sync_i32(&mut board.grid_cel_offset[row][col][sub]); } } });
        }
        BoardBaseFieldId::GridCelFog => {
            apply_field_with_sync(data, |c| for col in 0..MAX_GRID_SIZE_X { for row in 0..(MAX_GRID_SIZE_Y + 1) { c.sync_i32(&mut board.grid_cel_fog[col][row]); } });
        }
        BoardBaseFieldId::EnableGravestones => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_enable_grave_stones)); }
        BoardBaseFieldId::SpecialGravestoneX => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_special_grave_stone_x)); }
        BoardBaseFieldId::SpecialGravestoneY => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_special_grave_stone_y)); }
        BoardBaseFieldId::FogOffset => { apply_field_with_sync(data, |c| c.sync_f32(&mut board.m_fog_offset)); }
        BoardBaseFieldId::FogBlownCountdown => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_fog_blown_count_down)); }
        BoardBaseFieldId::PlantRow => {
            apply_field_with_sync(data, |c| for row in 0..MAX_GRID_SIZE_Y { c.sync_enum(&mut board.m_plant_row[row]); });
        }
        BoardBaseFieldId::WaveRowGotLawnMowered => { apply_field_with_sync(data, |c| sync_i32_array(c, &mut board.m_wave_row_got_lawn_mowered)); }
        // [TRANSLATION_NOTE]: mBonusLawnMowersRemaining 未翻译，读取后丢弃
        BoardBaseFieldId::BonusLawnMowersRemaining => { apply_field_with_sync(data, |c| c.sync_i32(&mut tmp_i32)); }
        BoardBaseFieldId::IceMinX => { apply_field_with_sync(data, |c| sync_i32_array(c, &mut board.m_ice_min_x)); }
        BoardBaseFieldId::IceTimer => { apply_field_with_sync(data, |c| sync_i32_array(c, &mut board.m_ice_timer)); }
        // [TRANSLATION_NOTE]: mIceParticleID[6] 未翻译，读取后丢弃
        BoardBaseFieldId::IceParticleId => { apply_field_with_sync(data, |c| for _ in 0..MAX_GRID_SIZE_Y { c.sync_u32(&mut tmp_u32); }); }
        BoardBaseFieldId::RowPickingArray => { apply_field_with_sync(data, |c| sync_pvzp_smooth_array_list(c, &mut board.m_row_picking_array)); }
        BoardBaseFieldId::ZombiesInWave => {
            apply_field_with_sync(data, |c| for wave in 0..MAX_ZOMBIE_WAVES { for slot in 0..MAX_ZOMBIES_IN_WAVE { c.sync_enum(&mut board.m_zombies_in_wave[wave][slot]); } });
        }
        BoardBaseFieldId::ZombieAllowed => {
            apply_field_with_sync(data, |c| for i in 0..ZOMBIE_ALLOWED_COUNT {
                let mut v = if i < board.m_zombie_allowed.len() { board.m_zombie_allowed[i] } else { false };
                c.sync_bool(&mut v);
                if i < board.m_zombie_allowed.len() { board.m_zombie_allowed[i] = v; }
            });
        }
        BoardBaseFieldId::SunCountdown => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_sun_countdown)); }
        BoardBaseFieldId::NumSunsFallen => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_num_suns_fallen)); }
        BoardBaseFieldId::ShakeCounter => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_shake_counter)); }
        BoardBaseFieldId::ShakeAmountX => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_shake_amount_x)); }
        BoardBaseFieldId::ShakeAmountY => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_shake_amount_y)); }
        BoardBaseFieldId::BackgroundType => { apply_field_with_sync(data, |c| c.sync_enum(&mut board.m_background_type)); }
        BoardBaseFieldId::Level => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.level)); }
        BoardBaseFieldId::SodPosition => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_sod_position)); }
        BoardBaseFieldId::PrevMouseX => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_prev_mouse_x)); }
        BoardBaseFieldId::PrevMouseY => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_prev_mouse_y)); }
        BoardBaseFieldId::SunMoney => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_sun_money)); }
        BoardBaseFieldId::NumWaves => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_num_waves)); }
        BoardBaseFieldId::MainCounter => { apply_field_with_sync(data, |c| c.sync_u32(&mut board.m_main_counter)); }
        BoardBaseFieldId::EffectCounter => { apply_field_with_sync(data, |c| c.sync_u32(&mut board.m_effect_counter)); }
        BoardBaseFieldId::DrawCount => { apply_field_with_sync(data, |c| c.sync_u32(&mut board.m_draw_count)); }
        BoardBaseFieldId::RiseFromGraveCounter => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_rise_from_grave_counter)); }
        BoardBaseFieldId::OutOfMoneyCounter => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_out_of_money_counter)); }
        BoardBaseFieldId::CurrentWave => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_current_wave)); }
        BoardBaseFieldId::TotalSpawnedWaves => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_total_spawned_waves)); }
        BoardBaseFieldId::TutorialState => { apply_field_with_sync(data, |c| c.sync_enum(&mut board.m_tutorial_state)); }
        BoardBaseFieldId::TutorialParticleId => { apply_field_with_sync(data, |c| c.sync_u32(&mut board.m_tutorial_particle_id)); }
        BoardBaseFieldId::TutorialTimer => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_tutorial_timer)); }
        // [TRANSLATION_NOTE]: mLastBungeeWave 未翻译，读取后丢弃
        BoardBaseFieldId::LastBungeeWave => { apply_field_with_sync(data, |c| c.sync_i32(&mut tmp_i32)); }
        BoardBaseFieldId::ZombieHealthToNextWave => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_zombie_health_to_next_wave)); }
        BoardBaseFieldId::ZombieHealthWaveStart => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_zombie_health_wave_start)); }
        BoardBaseFieldId::ZombieCountdown => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_zombie_count_down)); }
        BoardBaseFieldId::ZombieCountdownStart => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_zombie_count_down_start)); }
        BoardBaseFieldId::HugeWaveCountdown => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_huge_wave_count_down)); }
        BoardBaseFieldId::HelpDisplayed => {
            apply_field_with_sync(data, |c| sync_bool_array(c, &mut board.m_help_displayed));
        }
        BoardBaseFieldId::HelpIndex => { apply_field_with_sync(data, |c| c.sync_enum(&mut board.m_advice)); }
        BoardBaseFieldId::FinalBossKilled => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_final_boss_killed)); }
        BoardBaseFieldId::ShowShovel => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_show_shovel)); }
        BoardBaseFieldId::CoinBankFadeCount => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_coin_bank_fade_count)); }
        BoardBaseFieldId::DebugTextMode => { apply_field_with_sync(data, |c| c.sync_enum(&mut board.m_debug_text_mode)); }
        BoardBaseFieldId::LevelComplete => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_level_complete)); }
        BoardBaseFieldId::BoardFadeOutCounter => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_board_fade_out_counter)); }
        BoardBaseFieldId::NextSurvivalStageCounter => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_next_survival_stage_counter)); }
        BoardBaseFieldId::ScoreNextMowerCounter => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_score_next_mower_counter)); }
        BoardBaseFieldId::LevelAwardSpawned => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_level_award_spawned)); }
        BoardBaseFieldId::ProgressMeterWidth => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_progress_meter_width)); }
        BoardBaseFieldId::FlagRaiseCounter => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_flag_raise_counter)); }
        BoardBaseFieldId::IceTrapCounter => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_ice_trap_counter)); }
        BoardBaseFieldId::BoardRandSeed => { apply_field_with_sync(data, |c| {
            let mut raw = board.m_board_rand_seed as i32;
            c.sync_i32(&mut raw);
            board.m_board_rand_seed = raw as u32;
        }); }
        // [TRANSLATION_NOTE]: mPoolSparklyParticleID 未翻译，读取后丢弃
        BoardBaseFieldId::PoolSparklyParticleId => { apply_field_with_sync(data, |c| c.sync_u32(&mut tmp_u32)); }
        BoardBaseFieldId::FwooshId => {
            apply_field_with_sync(data, |c| for row in 0..MAX_GRID_SIZE_Y { for slot in 0..12 { c.sync_u32(&mut board.m_fwoosh_id[row][slot]); } });
        }
        BoardBaseFieldId::FwooshCountdown => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_fwoosh_count_down)); }
        BoardBaseFieldId::TimeStopCounter => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_time_stop_counter)); }
        BoardBaseFieldId::DroppedFirstCoin => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_dropped_first_coin)); }
        BoardBaseFieldId::FinalWaveSoundCounter => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_final_wave_sound_counter)); }
        BoardBaseFieldId::CobCannonCursorDelayCounter => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_cob_cannon_cursor_delay_counter)); }
        BoardBaseFieldId::CobCannonMouseX => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_cob_cannon_mouse_x)); }
        BoardBaseFieldId::CobCannonMouseY => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_cob_cannon_mouse_y)); }
        BoardBaseFieldId::KilledYeti => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_killed_yeti)); }
        BoardBaseFieldId::MustacheMode => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_mustache_mode)); }
        BoardBaseFieldId::SuperMowerMode => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_super_mower_mode)); }
        BoardBaseFieldId::FutureMode => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_future_mode)); }
        BoardBaseFieldId::PinataMode => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_pinata_mode)); }
        BoardBaseFieldId::DanceMode => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_dance_mode)); }
        BoardBaseFieldId::DaisyMode => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_daisy_mode)); }
        BoardBaseFieldId::SukhbirMode => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_sukhbir_mode)); }
        // [TRANSLATION_NOTE]: mPrevBoardResult 未翻译，读取后丢弃
        BoardBaseFieldId::PrevBoardResult => { apply_field_with_sync(data, |c| c.sync_i32(&mut tmp_i32)); }
        BoardBaseFieldId::TriggeredLawnMowers => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_triggered_lawn_mowers)); }
        // [TRANSLATION_NOTE]: mPlayTimeActiveLevel/mPlayTimeInactiveLevel/mMaxSunPlants 未翻译，读取后丢弃
        BoardBaseFieldId::PlayTimeActiveLevel => { apply_field_with_sync(data, |c| c.sync_u32(&mut tmp_u32)); }
        BoardBaseFieldId::PlayTimeInactiveLevel => { apply_field_with_sync(data, |c| c.sync_u32(&mut tmp_u32)); }
        BoardBaseFieldId::MaxSunPlants => { apply_field_with_sync(data, |c| c.sync_i32(&mut tmp_i32)); }
        BoardBaseFieldId::StartDrawTime => { apply_field_with_sync(data, |c| c.sync_i64(&mut board.m_start_draw_time)); }
        BoardBaseFieldId::IntervalDrawTime => { apply_field_with_sync(data, |c| c.sync_i64(&mut board.m_interval_draw_time)); }
        BoardBaseFieldId::IntervalDrawCountStart => { apply_field_with_sync(data, |c| c.sync_u32(&mut board.m_interval_draw_count_start)); }
        // [TRANSLATION_NOTE]: mMinFPS/mPreloadTime/mGameID 未翻译，读取后丢弃
        BoardBaseFieldId::MinFps => { apply_field_with_sync(data, |c| c.sync_f32(&mut tmp_f32)); }
        BoardBaseFieldId::PreloadTime => { apply_field_with_sync(data, |c| c.sync_i32(&mut tmp_i32)); }
        BoardBaseFieldId::GameId => { apply_field_with_sync(data, |c| c.sync_i64(&mut tmp_i64)); }
        BoardBaseFieldId::GravesCleared => { apply_field_with_sync(data, |c| c.sync_u32(&mut board.m_graves_cleared)); }
        BoardBaseFieldId::PlantsEaten => { apply_field_with_sync(data, |c| c.sync_u32(&mut board.m_plants_eaten)); }
        BoardBaseFieldId::PlantsShoveled => { apply_field_with_sync(data, |c| c.sync_u32(&mut board.m_plants_shoveled)); }
        BoardBaseFieldId::PeaShooterUsed => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_pea_shooter_used)); }
        BoardBaseFieldId::CatapultPlantsUsed => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_catapult_plants_used)); }
        BoardBaseFieldId::MushroomAndCoffeeBeansOnly => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_mushroom_and_coffee_beans_only)); }
        BoardBaseFieldId::MushroomsUsed => { apply_field_with_sync(data, |c| c.sync_bool(&mut board.m_mushrooms_used)); }
        BoardBaseFieldId::LevelCoinsCollected => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_level_coins_collected)); }
        // [TRANSLATION_NOTE]: mGargantuarsKillsByCornCob 未翻译，读取后丢弃
        BoardBaseFieldId::GargantuarsKillsByCornCob => { apply_field_with_sync(data, |c| c.sync_u32(&mut tmp_u32)); }
        BoardBaseFieldId::CoinsCollected => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_coins_collected)); }
        BoardBaseFieldId::DiamondsCollected => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_diamonds_collected)); }
        BoardBaseFieldId::PottedPlantsCollected => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_potted_plants_collected)); }
        BoardBaseFieldId::ChocolateCollected => { apply_field_with_sync(data, |c| c.sync_i32(&mut board.m_chocolate_collected)); }
        _ => { /* C++: default break */ }
    }
}

/// 同步棋盘基础 chunk（对应 C++ SyncBoardBasePortable，SaveGame.cpp:1555）
/// 读取分支：TLV blob 循环应用各字段；写入分支：AppendFieldWithSync 序列 + WriteTLVBlob
fn sync_board_base_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    if ctx.reading {
        let a_blob = match read_tlv_blob(ctx) {
            Some(b) => b,
            None => return,
        };
        let mut a_reader = TLVReader::new(&a_blob);
        while a_reader.is_ok() && a_reader.remaining() > 0 {
            let field_id = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_size = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_data = match a_reader.read_bytes(field_size as usize) {
                Some(d) => d,
                None => break,
            };
            apply_board_base_field(field_id, field_data, board);
        }
    } else {
        let mut a_blob: Vec<u8> = Vec::new();
        append_board_base_fields(&mut a_blob, board);
        write_tlv_blob(ctx, &a_blob);
    }
}

/// 写入一个 V4 chunk（对应 C++ WriteChunkV4，SaveGame.cpp:2335）
/// 结构：字段 blob → chunk 包装（version + TLV(fieldId=1, size, data)）→ AppendChunk(type, size, data)
fn write_chunk_v4(payload: &mut Vec<u8>, chunk_type: u32, board: &mut Board) -> bool {
    // C++ GetChunkSyncFn：无同步函数的 chunk 返回 true 跳过
    let chunk = match save_chunk_from_id(chunk_type) {
        Some(c) => c,
        None => return true,
    };
    match chunk {
        SaveChunkTypeV4::BoardBase => {}
        SaveChunkTypeV4::Coins => {}
        SaveChunkTypeV4::Mowers => {}
        SaveChunkTypeV4::Projectiles => {}
        SaveChunkTypeV4::GridItems => {}
        SaveChunkTypeV4::Plants => {}
        SaveChunkTypeV4::Zombies => {}
        SaveChunkTypeV4::Cursor => {}
        SaveChunkTypeV4::CursorPreview => {}
        SaveChunkTypeV4::Advice => {}
        SaveChunkTypeV4::SeedBank => {}
        SaveChunkTypeV4::SeedPackets => {}
        SaveChunkTypeV4::Challenge => {}
        SaveChunkTypeV4::Music => {}
        _ => return true,
    }

    // C++ aFieldWriter：字段上下文
    let mut field_data: Vec<u8> = Vec::new();
    {
        let buf = Buffer::new();
        let mut field_ctx = PortableSaveContext::new_writer(buf);
        match chunk {
            SaveChunkTypeV4::BoardBase => sync_board_base_portable(&mut field_ctx, board),
            SaveChunkTypeV4::Coins => sync_coins_portable(&mut field_ctx, board),
            SaveChunkTypeV4::Mowers => sync_mowers_portable(&mut field_ctx, board),
            SaveChunkTypeV4::Projectiles => sync_projectiles_portable(&mut field_ctx, board),
            SaveChunkTypeV4::GridItems => sync_grid_items_portable(&mut field_ctx, board),
            SaveChunkTypeV4::Plants => sync_plants_portable(&mut field_ctx, board),
            SaveChunkTypeV4::Zombies => sync_zombies_portable(&mut field_ctx, board),
            SaveChunkTypeV4::Cursor => sync_cursor_portable(&mut field_ctx, board),
            SaveChunkTypeV4::CursorPreview => sync_cursor_preview_portable(&mut field_ctx),
            SaveChunkTypeV4::Advice => sync_advice_portable(&mut field_ctx, board),
            SaveChunkTypeV4::SeedBank => sync_seed_bank_portable(&mut field_ctx, board),
            SaveChunkTypeV4::SeedPackets => sync_seed_packets_portable(&mut field_ctx, board),
            SaveChunkTypeV4::Challenge => sync_challenge_portable(&mut field_ctx, board),
            SaveChunkTypeV4::Music => sync_music_portable(&mut field_ctx, board),
            _ => return true,
        }
        if field_ctx.failed {
            return false;
        }
        field_data = field_ctx
            .buffer
            .as_ref()
            .map(|b| b.data().to_vec())
            .unwrap_or_default();
    }

    // C++ aChunkWriter：SAVE4_CHUNK_VERSION + fieldId=1 + 长度 + 字段数据
    let mut chunk_data: Vec<u8> = Vec::new();
    append_u32_le(&mut chunk_data, SAVE4_CHUNK_VERSION);
    append_u32_le(&mut chunk_data, 1u32);
    append_u32_le(&mut chunk_data, field_data.len() as u32);
    append_bytes(&mut chunk_data, &field_data);

    // C++ AppendChunk：chunkType + size + data
    append_u32_le(payload, chunk_type);
    append_u32_le(payload, chunk_data.len() as u32);
    append_bytes(payload, &chunk_data);
    true
}

// ── 实体 chunk（对应 C++ SyncDataArrayPortableTLV 系列，SaveGame.cpp:1392 起） ──

/// 将存档 chunk 类型 id 映射为 SaveChunkTypeV4（判别式连续 1..=20，范围内 transmute 合法）
fn save_chunk_from_id(id: u32) -> Option<SaveChunkTypeV4> {
    if id >= 1 && id <= 20 {
        Some(unsafe { std::mem::transmute::<u32, SaveChunkTypeV4>(id) })
    } else {
        None
    }
}

/// C++ DataArray 的活跃 key 掩码（DataArray.h:34，DATA_ARRAY_KEY_MASK = -65536）
const DATA_ARRAY_KEY_MASK: u32 = 0xFFFF_0000;

/// 同步盆栽植物规格（对应 C++ SyncPottedPlantPortable，SaveGame.cpp:625）
fn sync_potted_plant_portable(ctx: &mut PortableSaveContext, plant: &mut PottedPlant) {
    ctx.sync_enum(&mut plant.seed_type);
    ctx.sync_enum(&mut plant.which_zen_garden);
    ctx.sync_i32(&mut plant.x);
    ctx.sync_i32(&mut plant.y);
    ctx.sync_enum(&mut plant.facing);
    ctx.sync_i64(&mut plant.last_watered_time);
    ctx.sync_enum(&mut plant.draw_variation);
    ctx.sync_enum(&mut plant.plant_age);
    ctx.sync_i32(&mut plant.times_fed);
    ctx.sync_i32(&mut plant.feedings_per_grow);
    ctx.sync_enum(&mut plant.plant_need);
    ctx.sync_i64(&mut plant.last_need_fulfilled_time);
    ctx.sync_i64(&mut plant.last_fertilized_time);
    ctx.sync_i64(&mut plant.last_chocolate_time);
    // [TRANSLATION_NOTE]: C++ mFutureAttribute[3] 仅同步 [0]，Rust PottedPlant 未翻译该字段，占位保持格式
    let mut tmp_i64 = 0i64;
    ctx.sync_i64(&mut tmp_i64);
}

/// 同步硬币尾部字段（对应 C++ SyncCoinTailPortable，SaveGame.cpp:949）
fn sync_coin_tail_portable(ctx: &mut PortableSaveContext, coin: &mut Coin) {
    ctx.sync_f32(&mut coin.pos_x);
    ctx.sync_f32(&mut coin.pos_y);
    ctx.sync_f32(&mut coin.vel_x);
    ctx.sync_f32(&mut coin.vel_y);
    ctx.sync_f32(&mut coin.scale);
    ctx.sync_bool(&mut coin.dead);
    ctx.sync_i32(&mut coin.fade_count);
    ctx.sync_f32(&mut coin.collect_x);
    ctx.sync_f32(&mut coin.collect_y);
    // [TRANSLATION_NOTE]: C++ mGroundY 为 int32，Rust Coin.ground_y 为 f32，经 i32 中转保持存档格式
    let mut ground_y = coin.ground_y as i32;
    ctx.sync_i32(&mut ground_y);
    coin.ground_y = ground_y as f32;
    ctx.sync_i32(&mut coin.coin_age);
    ctx.sync_bool(&mut coin.is_being_collected);
    ctx.sync_i32(&mut coin.disappear_counter);
    ctx.sync_enum(&mut coin.coin_type);
    ctx.sync_enum(&mut coin.coin_motion);
    // C++ SyncEnum32(mAttachmentID)；Rust AttachmentID = i32
    ctx.sync_i32(&mut coin.attachment_id);
    ctx.sync_f32(&mut coin.collection_distance);
    ctx.sync_enum(&mut coin.usable_seed_type);
    sync_potted_plant_portable(ctx, &mut coin.potted_plant_spec);
    ctx.sync_bool(&mut coin.needs_bouncy_arrow);
    ctx.sync_bool(&mut coin.has_bouncy_arrow);
    ctx.sync_bool(&mut coin.hit_ground);
    ctx.sync_i32(&mut coin.times_dropped);
}

/// 同步割草机尾部字段（对应 C++ SyncLawnMowerTailPortable，SaveGame.cpp:972）
fn sync_lawn_mower_tail_portable(ctx: &mut PortableSaveContext, mower: &mut LawnMower) {
    ctx.sync_f32(&mut mower.pos_x);
    ctx.sync_f32(&mut mower.pos_y);
    ctx.sync_i32(&mut mower.render_order);
    // C++ SyncInt32(mRow)：Rust 存于 base.row
    ctx.sync_i32(&mut mower.base.row);
    ctx.sync_i32(&mut mower.anim_ticks_per_frame);
    ctx.sync_u32(&mut mower.mower_anim_id);
    ctx.sync_i32(&mut mower.chomp_counter);
    ctx.sync_i32(&mut mower.rolling_in_counter);
    ctx.sync_i32(&mut mower.squished_counter);
    ctx.sync_enum(&mut mower.mower_state);
    ctx.sync_bool(&mut mower.dead);
    ctx.sync_bool(&mut mower.visible);
    ctx.sync_enum(&mut mower.mower_type);
    ctx.sync_f32(&mut mower.altitude);
    ctx.sync_enum(&mut mower.mower_height);
    ctx.sync_i32(&mut mower.last_portal_x);
}

/// 同步实体数据数组（对应 C++ SyncDataArrayPortableTLV，SaveGame.cpp:1392）
/// Rust 以 Vec 承载实体；写侧模拟 C++ DataArray 头部元数据与活跃 key，读侧按槽重建
fn sync_data_array_tlv<T: Default, W, R>(
    ctx: &mut PortableSaveContext,
    items: &mut Vec<T>,
    write_fn: W,
    read_fn: R,
) -> bool
where
    W: Fn(&mut Vec<u8>, &mut T),
    R: Fn(u32, &[u8], &mut T),
{
    if ctx.reading {
        // C++: SyncUInt32(mFreeListHead / mMaxUsedCount / mSize / mNextKey / mMaxSize)
        let mut free_list_head = 0u32;
        let mut max_used_count = 0u32;
        let mut size = 0u32;
        let mut next_key = 0u32;
        let mut max_size = 0u32;
        ctx.sync_u32(&mut free_list_head);
        ctx.sync_u32(&mut max_used_count);
        ctx.sync_u32(&mut size);
        ctx.sync_u32(&mut next_key);
        ctx.sync_u32(&mut max_size);
        if ctx.failed {
            return false;
        }
        items.clear();
        items.reserve(max_used_count as usize);
        for _ in 0..max_used_count {
            let mut item_id = 0u32;
            let mut item_size = 0u32;
            ctx.sync_u32(&mut item_id);
            ctx.sync_u32(&mut item_size);
            if ctx.failed {
                return false;
            }
            let mut item = T::default();
            if item_size > 0 {
                let mut item_data = vec![0u8; item_size as usize];
                ctx.sync_bytes(&mut item_data);
                if ctx.failed {
                    return false;
                }
                let mut a_reader = TLVReader::new(&item_data);
                while a_reader.is_ok() && a_reader.remaining() > 0 {
                    let field_id = match a_reader.read_u32() {
                        Some(v) => v,
                        None => break,
                    };
                    let field_size = match a_reader.read_u32() {
                        Some(v) => v,
                        None => break,
                    };
                    let field_data = match a_reader.read_bytes(field_size as usize) {
                        Some(d) => d,
                        None => break,
                    };
                    read_fn(field_id, field_data, &mut item);
                }
            }
            items.push(item);
            let _ = item_id;
        }
    } else {
        // C++: 头部元数据（Rust Vec 无自由列表，均以当前长度表示）
        let mut free_list_head = 0u32;
        let mut max_used_count = items.len() as u32;
        let mut size = items.len() as u32;
        let mut next_key = items.len() as u32;
        let mut max_size = items.len() as u32;
        ctx.sync_u32(&mut free_list_head);
        ctx.sync_u32(&mut max_used_count);
        ctx.sync_u32(&mut size);
        ctx.sync_u32(&mut next_key);
        ctx.sync_u32(&mut max_size);
        if ctx.failed {
            return false;
        }
        for (i, item) in items.iter_mut().enumerate() {
            // C++ DataArrayGetIDAt(i)：活跃 key = 0xFFFF0000 | 索引
            let mut item_id = DATA_ARRAY_KEY_MASK | (i as u32);
            ctx.sync_u32(&mut item_id);
            let mut item_data: Vec<u8> = Vec::new();
            write_fn(&mut item_data, item);
            let mut item_size = item_data.len() as u32;
            ctx.sync_u32(&mut item_size);
            if item_size > 0 {
                ctx.sync_bytes_const(&item_data);
            }
        }
    }
    !ctx.failed
}

/// 同步硬币 chunk（对应 C++ SyncCoinsPortable，SaveGame.cpp:1856）
fn sync_coins_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    sync_data_array_tlv(
        ctx,
        &mut board.coins,
        |out, coin| {
            write_game_object_field(out, 1, &mut coin.base);
            append_field_with_sync(out, PORTABLE_FIELD_TAIL, |c| sync_coin_tail_portable(c, coin));
        },
        |field_id, data, coin| match field_id {
            1 => {
                read_game_object_field(data, &mut coin.base);
            }
            // C++: 2U/3U 为旧版字段（legacy），读侧直接应用
            2 | 3 => {
                let _ = data;
            }
            PORTABLE_FIELD_TAIL => {
                apply_field_with_sync(data, |c| sync_coin_tail_portable(c, coin));
            }
            _ => {}
        },
    );
}

/// 同步割草机 chunk（对应 C++ SyncMowersPortable，SaveGame.cpp:1873）
fn sync_mowers_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    sync_data_array_tlv(
        ctx,
        &mut board.lawn_mowers,
        |out, mower| {
            write_game_object_field(out, 1, &mut mower.base);
            append_field_with_sync(out, PORTABLE_FIELD_TAIL, |c| sync_lawn_mower_tail_portable(c, mower));
        },
        |field_id, data, mower| match field_id {
            1 => {
                read_game_object_field(data, &mut mower.base);
            }
            PORTABLE_FIELD_TAIL => {
                apply_field_with_sync(data, |c| sync_lawn_mower_tail_portable(c, mower));
            }
            _ => {}
        },
    );
}

/// 同步运动轨迹帧（对应 C++ SyncMotionTrailFramePortable，SaveGame.cpp:637）
fn sync_motion_trail_frame_portable(ctx: &mut PortableSaveContext, frame: &mut crate::lawn::grid_item::MotionTrailFrame) {
    ctx.sync_f32(&mut frame.pos_x);
    ctx.sync_f32(&mut frame.pos_y);
    ctx.sync_f32(&mut frame.anim_time);
}

/// 同步网格物品尾部字段（对应 C++ SyncGridItemTailPortable，SaveGame.cpp:996）
fn sync_grid_item_tail_portable(ctx: &mut PortableSaveContext, item: &mut GridItem) {
    ctx.sync_enum(&mut item.grid_item_type);
    ctx.sync_enum(&mut item.grid_item_state);
    ctx.sync_i32(&mut item.grid_x);
    ctx.sync_i32(&mut item.grid_y);
    ctx.sync_i32(&mut item.counter);
    ctx.sync_i32(&mut item.render_order);
    ctx.sync_bool(&mut item.dead);
    ctx.sync_f32(&mut item.pos_x);
    ctx.sync_f32(&mut item.pos_y);
    ctx.sync_f32(&mut item.goal_x);
    ctx.sync_f32(&mut item.goal_y);
    // C++ SyncEnumU32(mGridItemReanimID/mGridItemParticleID)；Rust ReanimationID/ParticleSystemID = u32
    ctx.sync_u32(&mut item.grid_item_reanim_id);
    ctx.sync_u32(&mut item.grid_item_particle_id);
    ctx.sync_enum(&mut item.zombie_type);
    ctx.sync_enum(&mut item.seed_type);
    ctx.sync_enum(&mut item.scary_pot_type);
    ctx.sync_bool(&mut item.highlighted);
    ctx.sync_i32(&mut item.transparent_counter);
    ctx.sync_i32(&mut item.sun_count);
    for i in 0..NUM_MOTION_TRAIL_FRAMES {
        sync_motion_trail_frame_portable(ctx, &mut item.motion_trail_frames[i]);
    }
    ctx.sync_i32(&mut item.motion_trail_count);
}

/// 同步网格物品 chunk（对应 C++ SyncGridItemsPortable，SaveGame.cpp:1879）
/// 注意：GridItem 非 GameObject，无基类字段（C++ 仅写 PORTABLE_FIELD_TAIL 尾部）
fn sync_grid_items_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    sync_data_array_tlv(
        ctx,
        &mut board.grid_items,
        |out, item| {
            append_field_with_sync(out, PORTABLE_FIELD_TAIL, |c| sync_grid_item_tail_portable(c, item));
        },
        |field_id, data, item| match field_id {
            // C++: 1U 为旧版字段（legacy）
            1 => {
                let _ = data;
            }
            PORTABLE_FIELD_TAIL => {
                apply_field_with_sync(data, |c| sync_grid_item_tail_portable(c, item));
            }
            _ => {}
        },
    );
}

/// 同步磁铁吸附物（对应 C++ SyncMagnetItemPortable，SaveGame.cpp:643）
/// C++ 仅同步 4 个 f32（mItemType 不入档）
fn sync_magnet_item_portable(ctx: &mut PortableSaveContext, item: &mut crate::lawn::plant::MagnetItem) {
    ctx.sync_f32(&mut item.pos_x);
    ctx.sync_f32(&mut item.pos_y);
    ctx.sync_f32(&mut item.dest_offset_x);
    ctx.sync_f32(&mut item.dest_offset_y);
}

/// 同步植物尾部字段（对应 C++ SyncPlantTailPortable，SaveGame.cpp:866）
fn sync_plant_tail_portable(ctx: &mut PortableSaveContext, plant: &mut Plant) {
    ctx.sync_enum(&mut plant.seed_type);
    ctx.sync_i32(&mut plant.plant_col);
    ctx.sync_i32(&mut plant.anim_counter);
    ctx.sync_i32(&mut plant.frame);
    ctx.sync_i32(&mut plant.frame_length);
    ctx.sync_i32(&mut plant.num_frames);
    ctx.sync_enum(&mut plant.state);
    ctx.sync_i32(&mut plant.plant_health);
    ctx.sync_i32(&mut plant.plant_max_health);
    ctx.sync_i32(&mut plant.subclass);
    ctx.sync_i32(&mut plant.disappear_countdown);
    ctx.sync_i32(&mut plant.do_special_countdown);
    ctx.sync_i32(&mut plant.state_countdown);
    ctx.sync_i32(&mut plant.launch_counter);
    ctx.sync_i32(&mut plant.launch_rate);
    sync_rect_portable(ctx, &mut plant.plant_rect);
    sync_rect_portable(ctx, &mut plant.plant_attack_rect);
    ctx.sync_i32(&mut plant.target_x);
    ctx.sync_i32(&mut plant.target_y);
    ctx.sync_i32(&mut plant.start_row);
    // C++ SyncEnumU32(mParticleID)；Rust ParticleSystemID = u32
    ctx.sync_u32(&mut plant.particle_id);
    ctx.sync_i32(&mut plant.shooting_counter);
    ctx.sync_u32(&mut plant.body_reanim_id);
    ctx.sync_u32(&mut plant.head_reanim_id);
    // [TRANSLATION_NOTE]: C++ mHeadReanimID2/mHeadReanimID3 未翻译为 Plant 字段，占位保持格式
    let mut tmp_reanim = 0u32;
    ctx.sync_u32(&mut tmp_reanim);
    ctx.sync_u32(&mut tmp_reanim);
    ctx.sync_u32(&mut plant.blink_reanim_id);
    ctx.sync_u32(&mut plant.light_reanim_id);
    ctx.sync_u32(&mut plant.sleeping_reanim_id);
    ctx.sync_i32(&mut plant.blink_countdown);
    ctx.sync_i32(&mut plant.recently_eaten_countdown);
    ctx.sync_i32(&mut plant.eaten_flash_countdown);
    ctx.sync_i32(&mut plant.beghouled_flash_countdown);
    ctx.sync_f32(&mut plant.shake_offset_x);
    ctx.sync_f32(&mut plant.shake_offset_y);
    for i in 0..MAX_MAGNET_ITEMS {
        sync_magnet_item_portable(ctx, &mut plant.magnet_items[i]);
    }
    // C++ SyncEnumU32(mTargetZombieID)；Rust ZombieID = u32
    ctx.sync_u32(&mut plant.target_zombie_id);
    ctx.sync_i32(&mut plant.wake_up_counter);
    ctx.sync_enum(&mut plant.on_bungee_state);
    ctx.sync_enum(&mut plant.imitater_type);
    ctx.sync_i32(&mut plant.potted_plant_index);
    ctx.sync_bool(&mut plant.anim_ping);
    ctx.sync_bool(&mut plant.dead);
    ctx.sync_bool(&mut plant.squished);
    ctx.sync_bool(&mut plant.is_asleep);
    ctx.sync_bool(&mut plant.is_on_board);
    ctx.sync_bool(&mut plant.highlighted);
}

/// 同步植物 chunk（对应 C++ SyncPlantsPortable，SaveGame.cpp:1812）
fn sync_plants_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    sync_data_array_tlv(
        ctx,
        &mut board.plants,
        |out, plant| {
            write_game_object_field(out, 1, &mut plant.base);
            append_field_with_sync(out, PORTABLE_FIELD_TAIL, |c| sync_plant_tail_portable(c, plant));
        },
        |field_id, data, plant| match field_id {
            1 => {
                read_game_object_field(data, &mut plant.base);
            }
            // C++: 2U-5U 为旧版字段（legacy）
            2 | 3 | 4 | 5 => {
                let _ = data;
            }
            PORTABLE_FIELD_TAIL => {
                apply_field_with_sync(data, |c| sync_plant_tail_portable(c, plant));
            }
            _ => {}
        },
    );
}

/// 同步僵尸尾部字段（对应 C++ SyncZombieTailPortable，SaveGame.cpp:786）
fn sync_zombie_tail_portable(ctx: &mut PortableSaveContext, zombie: &mut Zombie) {
    ctx.sync_enum(&mut zombie.zombie_type);
    ctx.sync_enum(&mut zombie.zombie_phase);
    ctx.sync_f32(&mut zombie.pos_x);
    ctx.sync_f32(&mut zombie.pos_y);
    ctx.sync_f32(&mut zombie.vel_x);
    ctx.sync_i32(&mut zombie.anim_counter);
    ctx.sync_i32(&mut zombie.groan_counter);
    ctx.sync_i32(&mut zombie.anim_ticks_per_frame);
    ctx.sync_i32(&mut zombie.anim_frames);
    ctx.sync_i32(&mut zombie.frame);
    ctx.sync_i32(&mut zombie.prev_frame);
    ctx.sync_bool(&mut zombie.variant);
    ctx.sync_bool(&mut zombie.is_eating);
    ctx.sync_i32(&mut zombie.just_got_shot_counter);
    ctx.sync_i32(&mut zombie.shield_just_got_shot_counter);
    ctx.sync_i32(&mut zombie.shield_recoil_counter);
    ctx.sync_i32(&mut zombie.zombie_age);
    ctx.sync_enum(&mut zombie.zombie_height);
    ctx.sync_i32(&mut zombie.phase_counter);
    ctx.sync_i32(&mut zombie.from_wave);
    ctx.sync_bool(&mut zombie.dropped_loot);
    ctx.sync_i32(&mut zombie.zombie_fade);
    ctx.sync_bool(&mut zombie.flat_tires);
    ctx.sync_i32(&mut zombie.use_ladder_col);
    ctx.sync_i32(&mut zombie.target_col);
    ctx.sync_f32(&mut zombie.altitude);
    ctx.sync_bool(&mut zombie.hit_umbrella);
    sync_rect_portable(ctx, &mut zombie.zombie_rect);
    sync_rect_portable(ctx, &mut zombie.zombie_attack_rect);
    ctx.sync_i32(&mut zombie.chilled_counter);
    ctx.sync_i32(&mut zombie.buttered_counter);
    ctx.sync_i32(&mut zombie.ice_trap_counter);
    ctx.sync_bool(&mut zombie.mind_controlled);
    ctx.sync_bool(&mut zombie.blowing_away);
    ctx.sync_bool(&mut zombie.has_head);
    ctx.sync_bool(&mut zombie.has_arm);
    ctx.sync_bool(&mut zombie.has_object);
    ctx.sync_bool(&mut zombie.in_pool);
    ctx.sync_bool(&mut zombie.on_high_ground);
    ctx.sync_bool(&mut zombie.yucky_face);
    ctx.sync_i32(&mut zombie.yucky_face_counter);
    ctx.sync_enum(&mut zombie.helm_type);
    ctx.sync_i32(&mut zombie.body_health);
    ctx.sync_i32(&mut zombie.body_max_health);
    ctx.sync_i32(&mut zombie.helm_health);
    ctx.sync_i32(&mut zombie.helm_max_health);
    ctx.sync_enum(&mut zombie.shield_type);
    ctx.sync_i32(&mut zombie.shield_health);
    ctx.sync_i32(&mut zombie.shield_max_health);
    ctx.sync_i32(&mut zombie.flying_health);
    ctx.sync_i32(&mut zombie.flying_max_health);
    ctx.sync_bool(&mut zombie.dead);
    // C++ SyncEnumU32(mRelatedZombieID)；Rust ZombieID = u32
    ctx.sync_u32(&mut zombie.related_zombie_id);
    // C++ SyncEnumU32Array(mFollowerZombieID, MAX_ZOMBIE_FOLLOWERS)
    sync_u32_array(ctx, &mut zombie.follower_zombie_ids);
    ctx.sync_bool(&mut zombie.playing_song);
    ctx.sync_i32(&mut zombie.particle_offset_x);
    ctx.sync_i32(&mut zombie.particle_offset_y);
    // C++ SyncEnum32(mAttachmentID)；Rust AttachmentID = i32
    ctx.sync_i32(&mut zombie.attachment_id);
    ctx.sync_i32(&mut zombie.summon_counter);
    ctx.sync_u32(&mut zombie.body_reanim_id);
    ctx.sync_f32(&mut zombie.scale_zombie);
    ctx.sync_f32(&mut zombie.vel_z);
    ctx.sync_f32(&mut zombie.original_anim_rate);
    // C++ SyncEnumU32(mTargetPlantID)；Rust PlantID = u32
    ctx.sync_u32(&mut zombie.target_plant_id);
    ctx.sync_i32(&mut zombie.boss_mode);
    ctx.sync_i32(&mut zombie.target_row);
    ctx.sync_i32(&mut zombie.boss_bungee_counter);
    ctx.sync_i32(&mut zombie.boss_stomp_counter);
    ctx.sync_i32(&mut zombie.boss_head_counter);
    ctx.sync_u32(&mut zombie.boss_fire_ball_reanim_id);
    ctx.sync_u32(&mut zombie.special_head_reanim_id);
    ctx.sync_i32(&mut zombie.fireball_row);
    ctx.sync_bool(&mut zombie.is_fire_ball);
    ctx.sync_u32(&mut zombie.mowered_reanim_id);
    ctx.sync_i32(&mut zombie.last_portal_x);
    ctx.sync_u32(&mut zombie.zombatar_head_reanim_id);
}

/// 同步僵尸 chunk（对应 C++ SyncZombiesPortable，SaveGame.cpp:1792）
fn sync_zombies_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    sync_data_array_tlv(
        ctx,
        &mut board.zombies,
        |out, zombie| {
            write_game_object_field(out, 1, &mut zombie.base);
            append_field_with_sync(out, PORTABLE_FIELD_TAIL, |c| sync_zombie_tail_portable(c, zombie));
        },
        |field_id, data, zombie| match field_id {
            1 => {
                read_game_object_field(data, &mut zombie.base);
            }
            // C++: 2U 为旧版字段（legacy）
            2 => {
                let _ = data;
            }
            PORTABLE_FIELD_TAIL => {
                apply_field_with_sync(data, |c| sync_zombie_tail_portable(c, zombie));
            }
            _ => {}
        },
    );
}

/// 同步光标尾部字段（对应 C++ SyncCursorObjectTailPortable，SaveGame.cpp:670）
fn sync_cursor_object_tail_portable(ctx: &mut PortableSaveContext, object: &mut CursorObject) {
    // C++ 顺序：mSeedBankIndex → mType(SeedType) → mImitaterType → mCursorType
    ctx.sync_i32(&mut object.seed_bank_index);
    ctx.sync_enum(&mut object.seed_type);
    ctx.sync_enum(&mut object.imitater_type);
    ctx.sync_enum(&mut object.cursor_type);
    // C++ SyncEnumU32；Rust CoinID/PlantID/ReanimationID = u32
    ctx.sync_u32(&mut object.coin_id);
    ctx.sync_u32(&mut object.glove_plant_id);
    ctx.sync_u32(&mut object.duplicator_plant_id);
    ctx.sync_u32(&mut object.cob_cannon_plant_id);
    ctx.sync_i32(&mut object.hammer_down_counter);
    ctx.sync_u32(&mut object.reanim_cursor_id);
}

/// 同步光标预览尾部字段（对应 C++ SyncCursorPreviewTailPortable，SaveGame.cpp:684）
fn sync_cursor_preview_tail_portable(ctx: &mut PortableSaveContext, preview: &mut CursorPreview) {
    ctx.sync_i32(&mut preview.grid_x);
    ctx.sync_i32(&mut preview.grid_y);
}

/// 同步消息控件尾部字段（对应 C++ SyncMessageWidgetTailPortable，SaveGame.cpp:690）
fn sync_message_widget_tail_portable(ctx: &mut PortableSaveContext, widget: &mut MessageWidget) {
    ctx.sync_bytes(&mut widget.label);
    ctx.sync_i32(&mut widget.display_time);
    ctx.sync_i32(&mut widget.duration);
    ctx.sync_enum(&mut widget.message_style);
    // C++ SyncEnumU32Array(mTextReanimID, MAX_MESSAGE_LENGTH)
    sync_u32_array(ctx, &mut widget.text_reanim_id);
    ctx.sync_enum(&mut widget.reanim_type);
    ctx.sync_i32(&mut widget.slide_off_time);
    ctx.sync_bytes(&mut widget.label_next);
    ctx.sync_enum(&mut widget.message_style_next);
}

/// 同步种子银行尾部字段（对应 C++ SyncSeedBankTailPortable，SaveGame.cpp:703）
/// [TRANSLATION_NOTE]: C++ mSeedBank 是对象含 mNumPackets/mCutSceneDarken/mConveyorBeltCounter；
/// Rust 拆为 seed_bank Vec + m_seed_bank_darken + m_conveyor_belt_counter，mNumPackets 以 len 中转
fn sync_seed_bank_tail_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    let mut num_packets = board.seed_bank.len() as i32;
    ctx.sync_i32(&mut num_packets);
    if ctx.reading {
        let _ = num_packets; // 读取时忽略：Vec 由 SeedPackets chunk 重建
    }
    ctx.sync_i32(&mut board.m_seed_bank_darken);
    ctx.sync_i32(&mut board.m_conveyor_belt_counter);
}

/// 同步种子槽尾部字段（对应 C++ SyncSeedPacketTailPortable，SaveGame.cpp:710）
fn sync_seed_packet_tail_portable(ctx: &mut PortableSaveContext, packet: &mut SeedPacket) {
    ctx.sync_i32(&mut packet.countdown);
    ctx.sync_i32(&mut packet.refresh_time);
    ctx.sync_i32(&mut packet.packet_index);
    ctx.sync_i32(&mut packet.offset_x);
    ctx.sync_enum(&mut packet.seed_type);
    ctx.sync_enum(&mut packet.imitater_type);
    ctx.sync_i32(&mut packet.slot_machine_countdown);
    ctx.sync_enum(&mut packet.slot_machine_next_seed);
    ctx.sync_f32(&mut packet.slot_machine_position);
    ctx.sync_bool(&mut packet.active);
    ctx.sync_bool(&mut packet.refreshing);
    ctx.sync_i32(&mut packet.times_used);
}

/// 同步挑战尾部字段（对应 C++ SyncChallengeTailPortable，SaveGame.cpp:726）
fn sync_challenge_tail_portable(ctx: &mut PortableSaveContext, challenge: &mut Challenge) {
    ctx.sync_i32(&mut challenge.beghouled_mouse_capture);
    ctx.sync_i32(&mut challenge.beghouled_mouse_down_x);
    ctx.sync_i32(&mut challenge.beghouled_mouse_down_y);
    // C++ SyncInt32Array(&mBeghouledEated[0][0], 9 * 6)；Rust [[i32;6];9] 逐元素同步
    for row in challenge.beghouled_eated.iter_mut() {
        sync_i32_array(ctx, row);
    }
    // C++ SyncInt32Array(mBeghouledPurcasedUpgrade, NUM_BEGHOULED_UPGRADES)（Rust 数组 [i32;4] 对齐）
    sync_i32_array(ctx, &mut challenge.beghouled_purchased_upgrade);
    ctx.sync_i32(&mut challenge.beghouled_matches_this_move);
    ctx.sync_enum(&mut challenge.challenge_state);
    ctx.sync_i32(&mut challenge.challenge_state_counter);
    ctx.sync_i32(&mut challenge.conveyor_belt_counter);
    ctx.sync_i32(&mut challenge.challenge_score);
    ctx.sync_i32(&mut challenge.show_bowling_line);
    ctx.sync_enum(&mut challenge.last_conveyor_seed_type);
    ctx.sync_i32(&mut challenge.survival_stage);
    ctx.sync_i32(&mut challenge.slot_machine_roll_count);
    // C++ SyncEnumU32(mReanimChallenge)；Rust ReanimationID = u32
    ctx.sync_u32(&mut challenge.reanim_challenge);
    sync_u32_array(ctx, &mut challenge.reanim_clouds);
    sync_i32_array(ctx, &mut challenge.clouds_counter);
    ctx.sync_i32(&mut challenge.challenge_grid_x);
    ctx.sync_i32(&mut challenge.challenge_grid_y);
    ctx.sync_i32(&mut challenge.scary_potter_pots);
    ctx.sync_i32(&mut challenge.rain_counter);
    ctx.sync_i32(&mut challenge.tree_of_wisdom_talk_index);
}

/// 同步音乐尾部字段（对应 C++ SyncMusicTailPortable，SaveGame.cpp:752）
fn sync_music_tail_portable(ctx: &mut PortableSaveContext, music: &mut Music) {
    ctx.sync_enum(&mut music.cur_music_tune);
    ctx.sync_enum(&mut music.cur_music_file_main);
    ctx.sync_enum(&mut music.cur_music_file_drums);
    ctx.sync_enum(&mut music.cur_music_file_hihats);
    ctx.sync_i32(&mut music.burst_override);
    ctx.sync_f32(&mut music.base_bpm);
    ctx.sync_f32(&mut music.base_mod_speed);
    ctx.sync_enum(&mut music.music_burst_state);
    ctx.sync_i32(&mut music.burst_state_counter);
    ctx.sync_enum(&mut music.music_drums_state);
    ctx.sync_i32(&mut music.queued_drum_track_packed_order);
    ctx.sync_i32(&mut music.drums_state_counter);
    ctx.sync_i32(&mut music.pause_offset);
    ctx.sync_i32(&mut music.pause_offset_drums);
    ctx.sync_bool(&mut music.paused);
    // C++ 读取时丢弃存档中的 mMusicDisabled（运行时能力标志），写侧才写
    if ctx.reading {
        let mut saved_music_disabled = false;
        ctx.sync_bool(&mut saved_music_disabled);
    } else {
        ctx.sync_bool(&mut music.music_disabled);
    }
    ctx.sync_i32(&mut music.fade_out_counter);
    ctx.sync_i32(&mut music.fade_out_duration);
}

/// 同步光标 chunk（对应 C++ SyncCursorPortable，SaveGame.cpp:2007）
/// [TRANSLATION_NOTE]: C++ CursorObject 继承 GameObject；Rust CursorObject 无基类字段，占位读写
fn sync_cursor_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    let mut tmp_base = GameObject::new();
    if ctx.reading {
        let a_blob = match read_tlv_blob(ctx) {
            Some(b) => b,
            None => return,
        };
        let mut a_reader = TLVReader::new(&a_blob);
        while a_reader.is_ok() && a_reader.remaining() > 0 {
            let field_id = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_size = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_data = match a_reader.read_bytes(field_size as usize) {
                Some(d) => d,
                None => break,
            };
            match field_id {
                1 => {
                    let _ = read_game_object_field(field_data, &mut tmp_base);
                }
                // C++: 2U 为旧版字段（legacy）
                2 => {
                    let _ = field_data;
                }
                PORTABLE_FIELD_TAIL => {
                    apply_field_with_sync(field_data, |c| sync_cursor_object_tail_portable(c, &mut board.cursor_object));
                }
                _ => {}
            }
        }
    } else {
        let mut a_blob: Vec<u8> = Vec::new();
        write_game_object_field(&mut a_blob, 1, &mut tmp_base);
        append_field_with_sync(&mut a_blob, PORTABLE_FIELD_TAIL, |c| sync_cursor_object_tail_portable(c, &mut board.cursor_object));
        write_tlv_blob(ctx, &a_blob);
    }
}

/// 同步光标预览 chunk（对应 C++ SyncCursorPreviewPortable，SaveGame.cpp:2042）
/// [TRANSLATION_NOTE]: Rust Board 无 mCursorPreview 成员，C++ 侧字段读入丢弃（不改变 Board 结构）
fn sync_cursor_preview_portable(ctx: &mut PortableSaveContext) {
    let mut tmp_preview = CursorPreview::new();
    if ctx.reading {
        let a_blob = match read_tlv_blob(ctx) {
            Some(b) => b,
            None => return,
        };
        let mut a_reader = TLVReader::new(&a_blob);
        while a_reader.is_ok() && a_reader.remaining() > 0 {
            let field_id = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_size = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_data = match a_reader.read_bytes(field_size as usize) {
                Some(d) => d,
                None => break,
            };
            match field_id {
                1 => {
                    let _ = read_game_object_field(field_data, &mut tmp_preview.base);
                }
                // C++: 2U 为旧版字段（legacy）
                2 => {
                    let _ = field_data;
                }
                PORTABLE_FIELD_TAIL => {
                    apply_field_with_sync(field_data, |c| sync_cursor_preview_tail_portable(c, &mut tmp_preview));
                }
                _ => {}
            }
        }
    } else {
        let mut a_blob: Vec<u8> = Vec::new();
        write_game_object_field(&mut a_blob, 1, &mut tmp_preview.base);
        append_field_with_sync(&mut a_blob, PORTABLE_FIELD_TAIL, |c| sync_cursor_preview_tail_portable(c, &mut tmp_preview));
        write_tlv_blob(ctx, &a_blob);
    }
}

/// 同步提示 chunk（对应 C++ SyncAdvicePortable，SaveGame.cpp:2077）
fn sync_advice_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    if ctx.reading {
        let a_blob = match read_tlv_blob(ctx) {
            Some(b) => b,
            None => return,
        };
        let mut a_reader = TLVReader::new(&a_blob);
        while a_reader.is_ok() && a_reader.remaining() > 0 {
            let field_id = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_size = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_data = match a_reader.read_bytes(field_size as usize) {
                Some(d) => d,
                None => break,
            };
            match field_id {
                // C++: 1U 为旧版字段（legacy）
                1 => {
                    let _ = field_data;
                }
                PORTABLE_FIELD_TAIL => {
                    apply_field_with_sync(field_data, |c| sync_message_widget_tail_portable(c, &mut board.m_advice_widget));
                }
                _ => {}
            }
        }
    } else {
        let mut a_blob: Vec<u8> = Vec::new();
        append_field_with_sync(&mut a_blob, PORTABLE_FIELD_TAIL, |c| sync_message_widget_tail_portable(c, &mut board.m_advice_widget));
        write_tlv_blob(ctx, &a_blob);
    }
}

/// 同步种子银行 chunk（对应 C++ SyncSeedBankPortable，SaveGame.cpp:2110）
/// [TRANSLATION_NOTE]: C++ mSeedBank 为 GameObject 派生对象；Rust 无对应对象，
/// field 1U 的 GameObject 基类以占位写入/读出丢弃
fn sync_seed_bank_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    let mut tmp_base = GameObject::new();
    if ctx.reading {
        let a_blob = match read_tlv_blob(ctx) {
            Some(b) => b,
            None => return,
        };
        let mut a_reader = TLVReader::new(&a_blob);
        while a_reader.is_ok() && a_reader.remaining() > 0 {
            let field_id = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_size = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_data = match a_reader.read_bytes(field_size as usize) {
                Some(d) => d,
                None => break,
            };
            match field_id {
                1 => {
                    let _ = read_game_object_field(field_data, &mut tmp_base);
                }
                // C++: 2U-4U 为旧版字段（legacy）
                2 | 3 | 4 => {
                    let _ = field_data;
                }
                PORTABLE_FIELD_TAIL => {
                    apply_field_with_sync(field_data, |c| sync_seed_bank_tail_portable(c, board));
                }
                _ => {}
            }
        }
    } else {
        let mut a_blob: Vec<u8> = Vec::new();
        write_game_object_field(&mut a_blob, 1, &mut tmp_base);
        append_field_with_sync(&mut a_blob, PORTABLE_FIELD_TAIL, |c| sync_seed_bank_tail_portable(c, board));
        write_tlv_blob(ctx, &a_blob);
    }
}

/// 同步种子槽数组 chunk（对应 C++ SyncSeedPacketsPortable，SaveGame.cpp:2147）
/// 注意：非 TLV blob，格式为 SyncInt32(aCount=SEEDBANK_MAX) + 逐包 (size + bytes)
/// [TRANSLATION_NOTE]: C++ mSeedPackets 固定 SEEDBANK_MAX 个；Rust 为 Vec，写入前 min 个、读取重建 Vec
fn sync_seed_packets_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    let mut a_count = SEEDBANK_MAX;
    ctx.sync_i32(&mut a_count);
    let n = a_count.min(SEEDBANK_MAX) as usize;
    if ctx.reading {
        board.seed_bank.clear();
        for i in 0..n {
            if i >= SEEDBANK_MAX as usize {
                break;
            }
            let mut item_size = 0u32;
            ctx.sync_u32(&mut item_size);
            let mut item_data: Vec<u8> = vec![0; item_size as usize];
            if item_size > 0 {
                ctx.sync_bytes(&mut item_data);
            }
            let mut a_reader = TLVReader::new(&item_data);
            let mut packet = SeedPacket::new();
            let mut tmp_base = GameObject::new();
            while a_reader.is_ok() && a_reader.remaining() > 0 {
                let field_id = match a_reader.read_u32() {
                    Some(v) => v,
                    None => break,
                };
                let field_size = match a_reader.read_u32() {
                    Some(v) => v,
                    None => break,
                };
                let field_data = match a_reader.read_bytes(field_size as usize) {
                    Some(d) => d,
                    None => break,
                };
                match field_id {
                    1 => {
                        let _ = read_game_object_field(field_data, &mut tmp_base);
                    }
                    // C++: 2U 为旧版字段（legacy）
                    2 => {
                        let _ = field_data;
                    }
                    PORTABLE_FIELD_TAIL => {
                        apply_field_with_sync(field_data, |c| sync_seed_packet_tail_portable(c, &mut packet));
                    }
                    _ => {}
                }
            }
            board.seed_bank.push(packet);
        }
    } else {
        let write_n = board.seed_bank.len().min(SEEDBANK_MAX as usize);
        for i in 0..write_n {
            let mut item_data: Vec<u8> = Vec::new();
            {
                let packet = &mut board.seed_bank[i];
                let mut tmp_base = GameObject::new();
                write_game_object_field(&mut item_data, 1, &mut tmp_base);
                append_field_with_sync(&mut item_data, PORTABLE_FIELD_TAIL, |c| sync_seed_packet_tail_portable(c, packet));
            }
            let mut item_size = item_data.len() as u32;
            ctx.sync_u32(&mut item_size);
            if item_size > 0 {
                ctx.sync_bytes_const(&item_data);
            }
        }
    }
}

/// 同步挑战 chunk（对应 C++ SyncChallengePortable，SaveGame.cpp:2193）
fn sync_challenge_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    let challenge = match board.challenge.as_mut() {
        Some(c) => c,
        None => return,
    };
    if ctx.reading {
        let a_blob = match read_tlv_blob(ctx) {
            Some(b) => b,
            None => return,
        };
        let mut a_reader = TLVReader::new(&a_blob);
        while a_reader.is_ok() && a_reader.remaining() > 0 {
            let field_id = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_size = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_data = match a_reader.read_bytes(field_size as usize) {
                Some(d) => d,
                None => break,
            };
            match field_id {
                // C++: 1U 为旧版字段（legacy）
                1 => {
                    let _ = field_data;
                }
                PORTABLE_FIELD_TAIL => {
                    apply_field_with_sync(field_data, |c| sync_challenge_tail_portable(c, challenge));
                }
                _ => {}
            }
        }
    } else {
        let mut a_blob: Vec<u8> = Vec::new();
        append_field_with_sync(&mut a_blob, PORTABLE_FIELD_TAIL, |c| sync_challenge_tail_portable(c, challenge));
        write_tlv_blob(ctx, &a_blob);
    }
}

/// 同步音乐 chunk（对应 C++ SyncMusicPortable，SaveGame.cpp:2226）
fn sync_music_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    let app = match board.app {
        Some(ptr) => ptr,
        None => return,
    };
    // 注意：mMusic 属 LawnApp（C++ theBoard->mApp->mMusic），Rust 同路径
    let music = unsafe { &mut (*app).music };
    let music = match music.as_mut() {
        Some(m) => m,
        None => return,
    };
    if ctx.reading {
        let a_blob = match read_tlv_blob(ctx) {
            Some(b) => b,
            None => return,
        };
        let mut a_reader = TLVReader::new(&a_blob);
        while a_reader.is_ok() && a_reader.remaining() > 0 {
            let field_id = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_size = match a_reader.read_u32() {
                Some(v) => v,
                None => break,
            };
            let field_data = match a_reader.read_bytes(field_size as usize) {
                Some(d) => d,
                None => break,
            };
            match field_id {
                // C++: 1U 为旧版字段（legacy）
                1 => {
                    let _ = field_data;
                }
                PORTABLE_FIELD_TAIL => {
                    apply_field_with_sync(field_data, |c| sync_music_tail_portable(c, music));
                }
                _ => {}
            }
        }
    } else {
        let mut a_blob: Vec<u8> = Vec::new();
        append_field_with_sync(&mut a_blob, PORTABLE_FIELD_TAIL, |c| sync_music_tail_portable(c, music));
        write_tlv_blob(ctx, &a_blob);
    }
}

/// 同步子弹尾部字段（对应 C++ SyncProjectileTailPortable，SaveGame.cpp:918）
fn sync_projectile_tail_portable(ctx: &mut PortableSaveContext, projectile: &mut Projectile) {
    ctx.sync_i32(&mut projectile.frame);
    ctx.sync_i32(&mut projectile.num_frames);
    ctx.sync_i32(&mut projectile.anim_counter);
    ctx.sync_f32(&mut projectile.pos_x);
    ctx.sync_f32(&mut projectile.pos_y);
    ctx.sync_f32(&mut projectile.pos_z);
    ctx.sync_f32(&mut projectile.vel_x);
    ctx.sync_f32(&mut projectile.vel_y);
    ctx.sync_f32(&mut projectile.vel_z);
    ctx.sync_f32(&mut projectile.acc_z);
    ctx.sync_f32(&mut projectile.shadow_y);
    ctx.sync_bool(&mut projectile.dead);
    ctx.sync_i32(&mut projectile.anim_ticks_per_frame);
    ctx.sync_enum(&mut projectile.motion);
    ctx.sync_enum(&mut projectile.projectile_type);
    ctx.sync_i32(&mut projectile.projectile_age);
    ctx.sync_i32(&mut projectile.click_backoff_counter);
    ctx.sync_f32(&mut projectile.rotation);
    ctx.sync_f32(&mut projectile.rotation_speed);
    ctx.sync_bool(&mut projectile.on_high_ground);
    // [TRANSLATION_NOTE]: C++ SyncInt32(mDamageRangeFlags)，Rust 为 u32，经 i32 中转
    let mut damage_range_flags = projectile.damage_range_flags as i32;
    ctx.sync_i32(&mut damage_range_flags);
    projectile.damage_range_flags = damage_range_flags as u32;
    ctx.sync_i32(&mut projectile.hit_torchwood_grid_x);
    // C++ SyncEnum32(mAttachmentID)；Rust AttachmentID = i32
    ctx.sync_i32(&mut projectile.attachment_id);
    ctx.sync_f32(&mut projectile.cob_target_x);
    ctx.sync_i32(&mut projectile.cob_target_row);
    // C++ SyncEnumU32(mTargetZombieID)；Rust ZombieID = u32
    ctx.sync_u32(&mut projectile.target_zombie_id);
    ctx.sync_i32(&mut projectile.last_portal_x);
}

/// 同步子弹 chunk（对应 C++ SyncProjectilesPortable，SaveGame.cpp:1835）
fn sync_projectiles_portable(ctx: &mut PortableSaveContext, board: &mut Board) {
    sync_data_array_tlv(
        ctx,
        &mut board.projectiles,
        |out, projectile| {
            write_game_object_field(out, 1, &mut projectile.base);
            append_field_with_sync(out, PORTABLE_FIELD_TAIL, |c| sync_projectile_tail_portable(c, projectile));
        },
        |field_id, data, projectile| match field_id {
            1 => {
                read_game_object_field(data, &mut projectile.base);
            }
            // C++: 2U/3U 为旧版字段（legacy）
            2 | 3 => {
                let _ = data;
            }
            PORTABLE_FIELD_TAIL => {
                apply_field_with_sync(data, |c| sync_projectile_tail_portable(c, projectile));
            }
            _ => {}
        },
    );
}
