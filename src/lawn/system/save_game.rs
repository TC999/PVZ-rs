// PvZ Portable Rust 翻译 — 游戏存档系统
// 对应 C++ src/Lawn/System/SaveGame.cpp（内部数据结构）

#![allow(dead_code)]

use crate::lawn::board::Board;
use crate::framework::buffer::Buffer;

// ── 常量 ──────────────────────────────────────────────

const SAVE_FILE_MAGIC_NUMBER: u32 = 0xFEEDDEAD;
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

    // SyncReanimationDef, SyncParticleDef, SyncTrailDef, SyncImage
    // 依赖 ReanimatorDefinition, TodParticleDefinition, TrailDefinition, Image 类型
    // 这些类型的 Rust 翻译尚未完成，待后续处理
}

// ── 顶层函数 ──────────────────────────────────────────

/// 加载游戏存档（对应 C++ LawnLoadGame）
pub fn lawn_load_game(_board: Option<*mut Board>, _file_path: &str) -> bool {
    false
}

/// 保存游戏存档（对应 C++ LawnSaveGame）
pub fn lawn_save_game(_board: Option<*mut Board>, _file_path: &str) -> bool {
    false
}
