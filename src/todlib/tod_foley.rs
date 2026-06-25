// PvZ Portable Rust 翻译 — TodFoley（音效系统）
// 对应 C++ src/Sexy.TodLib/TodFoley.h / TodFoley.cpp

#![allow(dead_code)]

use crate::framework::sound::sound_manager::SoundManager;
use crate::framework::sound::sound_instance::SoundInstance;

/// 最大拟音类型数
pub const MAX_FOLEY_TYPES: usize = 110;
/// 最大拟音实例数
pub const MAX_FOLEY_INSTANCES: usize = 8;

/// 拟音标志（对应 C++ FoleyFlags）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoleyFlags {
    Loop,              // 循环播放
    OneAtATime,        // 禁止叠加播放
    MuteOnPause,       // 暂停时静默
    UsesMusicVolume,   // 使用音乐音量
    DontRepeat,        // 禁止变式重复
}

/// 拟音类型枚举（对应 C++ FoleyType，105 个值）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FoleyType {
    Sun = 0,
    Splat,
    Lawnmower,
    Throw,
    SpawnSun,
    Chomp,
    ChompSoft,
    Plant,
    UseShovel,
    Drop,
    Beep,
    Groan,
    Brains,
    Sukhbir,
    JackInTheBox,
    ArtChallenge,
    Zamboni,
    Thunder,
    Frozen,
    ZombieSplash,
    BowlingImpact,
    Squish,
    TirePop,
    Explosion,
    Slurp,
    LimbsPop,
    PogoZombie,
    SnowPeaSparkles,
    ZombieFalling,
    Puff,
    Fume,
    Coin,
    KernelSplat,
    Digger,
    JackSurprise,
    VaseBreaking,
    PoolCleaner,
    Basketball,
    Ignite,
    FirePea,
    Thump,
    SquashHmm,
    Magnetshroom,
    Butter,
    BungeeScream,
    BossExplosionSmall,
    ShieldHit,
    Swing,
    Bonk,
    Rain,
    DolphinBeforeJumping,
    DolphinAppears,
    PlantWater,
    ZombieEnteringWater,
    GravebusterChomp,
    Cherrybomb,
    JalapenoIgnite,
    ReverseExplosion,
    PlasticHit,
    WinMusic,
    BalloonInflate,
    BigChomp,
    MelonImpact,
    PlantGrow,
    Shoop,
    Juicy,
    NewspaperRarrgh,
    NewspaperRip,
    Floop,
    Coffee,
    LowGroan,
    Prize,
    Yuck,
    Umbrella,
    GrassStep,
    Shovel,
    CobLaunch,
    Watering,
    Polevault,
    GraveStoneRumble,
    DirtRise,
    Fertilizer,
    Portal,
    WakeUp,
    BugSpray,
    Scream,
    Paper,
    MoneyFalls,
    Imp,
    HydraulicShort,
    Hydraulic,
    Gargantudeath,
    Ceramic,
    BossBoulderAttack,
    Chime,
    CrazyDaveShort,
    CrazyDaveLong,
    CrazyDaveExtraLong,
    CrazyDaveCrazy,
    Phonograph,
    Dancer,
    FinalFanfare,
    CrazyDaveScream,
    CrazyDaveScream2,
    NumFoley,
}

/// 拟音参数（对应 C++ FoleyParams）
pub struct FoleyParams {
    pub foley_type: FoleyType,
    pub pitch_range: f32,
    pub sfx_id: [Option<*mut usize>; 10],
    pub foley_flags: u32,
}

/// DirectSound 音效实例（对应 C++ TodDSoundInstance）
pub struct TodDSoundInstance {
    // 继承 SDLSoundInstance，简化处理
    pub sound_instance: Option<*mut crate::framework::sound::sdl_sound_instance::SDLSoundInstance>,
}

impl TodDSoundInstance {
    pub fn get_sound_position(&self) -> i32 {
        // TODO: 从 TodFoley.cpp 翻译
        0
    }

    pub fn set_sound_position(&self, _position: i32) {
        // TODO: 从 TodFoley.cpp 翻译
    }
}

/// 拟音实例（对应 C++ FoleyInstance）
pub struct FoleyInstance {
    pub instance: Option<*mut dyn SoundInstance>,
    pub ref_count: i32,
    pub paused: bool,
    pub start_time: i32,
    pub pause_offset: i32,
}

impl FoleyInstance {
    pub fn new() -> Self {
        FoleyInstance {
            instance: None,
            ref_count: 0,
            paused: false,
            start_time: 0,
            pause_offset: 0,
        }
    }
}

/// 拟音类型数据（对应 C++ FoleyTypeData）
pub struct FoleyTypeData {
    pub foley_instances: [FoleyInstance; MAX_FOLEY_INSTANCES],
    pub last_variation_played: i32,
}

impl FoleyTypeData {
    pub fn new() -> Self {
        let mut instances: [FoleyInstance; MAX_FOLEY_INSTANCES] = unsafe { std::mem::zeroed() };
        // 覆盖初始化每个元素
        for i in 0..MAX_FOLEY_INSTANCES {
            instances[i] = FoleyInstance::new();
        }
        FoleyTypeData {
            foley_instances: instances,
            last_variation_played: -1,
        }
    }
}

/// 拟音系统主类（对应 C++ TodFoley）
pub struct TodFoley {
    pub foley_type_data: Vec<FoleyTypeData>,
}

/// 兼容旧名称（FoleyManager → TodFoley）
pub type FoleyManager = TodFoley;

impl TodFoley {
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(MAX_FOLEY_TYPES);
        for _ in 0..MAX_FOLEY_TYPES {
            data.push(FoleyTypeData::new());
        }
        TodFoley {
            foley_type_data: data,
        }
    }

    pub fn play_foley(&self, _foley_type: FoleyType) {
        // TODO: 从 TodFoley.cpp 翻译
    }

    pub fn stop_foley(&self, _foley_type: FoleyType) {
        // TODO: 从 TodFoley.cpp 翻译
    }

    pub fn is_foley_playing(&self, _foley_type: FoleyType) -> bool {
        // TODO: 从 TodFoley.cpp 翻译
        false
    }

    pub fn game_pause(&self, _entering_pause: bool) {
        // TODO: 从 TodFoley.cpp 翻译
    }

    pub fn play_foley_pitch(&self, _foley_type: FoleyType, _pitch: f32) {
        // TODO: 从 TodFoley.cpp 翻译
    }

    pub fn cancel_paused_foley(&self) {
        // TODO: 从 TodFoley.cpp 翻译
    }

    pub fn apply_music_volume(&self, _foley_instance: &FoleyInstance) {
        // TODO: 从 TodFoley.cpp 翻译
    }

    pub fn rehookup_sound_with_music_volume(&self) {
        // TODO: 从 TodFoley.cpp 翻译
    }
}

impl Default for TodFoley {
    fn default() -> Self {
        TodFoley::new()
    }
}

// --- 自由函数（对应 C++ 自由函数） ---
pub fn sound_system_release_finished_instances(_sound_system: &mut TodFoley) {
    // TODO: 从 TodFoley.cpp 翻译
}

pub fn sound_system_has_foley_played_too_recently(_sound_system: &TodFoley, _foley_type: FoleyType) -> bool {
    // TODO: 从 TodFoley.cpp 翻译
    false
}

pub fn sound_system_find_instance(_sound_system: &TodFoley, _foley_type: FoleyType) -> Option<*mut FoleyInstance> {
    // TODO: 从 TodFoley.cpp 翻译
    None
}

pub fn sound_system_get_free_instance_index(_sound_system: &mut TodFoley, _foley_type: FoleyType) -> Option<*mut FoleyInstance> {
    // TODO: 从 TodFoley.cpp 翻译
    None
}

// --- 全局变量 ---
// 对应 C++: extern int gFoleyParamArraySize;
// 对应 C++: extern const FoleyParams* gFoleyParamArray;
// 对应 C++: extern const FoleyParams gLawnFoleyParamArray[NUM_FOLEY];
