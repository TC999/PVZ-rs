// PvZ Portable Rust 翻译 — TodFoley（音效系统）
// 对应 C++ src/Sexy.TodLib/TodFoley.h / TodFoley.cpp

#![allow(dead_code)]

use crate::framework::sound::sound_manager::SoundManager;
use crate::framework::sound::sound_instance::SoundInstance;
use crate::framework::common::RandFloat;
use crate::framework::common::RandRange;

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
#[derive(Debug, Clone)]
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


    pub fn play_foley(&self, foley_type: FoleyType) {
        // 对应 C++ PvzpFoley::PlayFoley（PvzpFoley.cpp:300）
        let a_pitch = lookup_foley(foley_type).map_or(0.0, |p| {
            if p.pitch_range != 0.0 { RandFloat(p.pitch_range) } else { 0.0 }
        });
        self.play_foley_pitch(foley_type, a_pitch);
    }

    /// 停止拟音（对应 C++ PvzpFoley::StopFoley，:309）
    pub fn stop_foley(&self, foley_type: FoleyType) {
        sound_system_release_finished_instances(self as *const TodFoley as *mut TodFoley);
        let a_foley_instance = match sound_system_find_instance(self, foley_type) {
            Some(inst) => inst,
            None => return,
        };
        unsafe {
            if (*a_foley_instance).ref_count > 0 {
                (*a_foley_instance).ref_count -= 1;
                if (*a_foley_instance).ref_count == 0 {
                    // 对应 C++: mInstance->Release(); mInstance = nullptr;
                    if let Some(inst) = (*a_foley_instance).instance {
                        (*inst).release();
                    }
                    (*a_foley_instance).instance = None;
                }
            }
        }
    }

    /// 拟音是否在播放（对应 C++ PvzpFoley::IsFoleyPlaying，:409）
    pub fn is_foley_playing(&self, foley_type: FoleyType) -> bool {
        sound_system_release_finished_instances(self as *const TodFoley as *mut TodFoley);
        sound_system_find_instance(self, foley_type).is_some()
    }

    /// 暂停/恢复拟音（对应 C++ PvzpFoley::GamePause，:326）
    pub fn game_pause(&self, entering_pause: bool) {
        sound_system_game_pause(self as *const TodFoley as *mut TodFoley, entering_pause);
    }

    /// 带音调播放拟音（对应 C++ PvzpFoley::PlayFoleyPitch，:247）
    pub fn play_foley_pitch(&self, foley_type: FoleyType, pitch: f32) {
        let a_foley_params = match lookup_foley(foley_type) {
            Some(p) => p,
            None => return,
        };
        sound_system_release_finished_instances(self as *const TodFoley as *mut TodFoley);
        // 对应 C++: 非循环音效 10 厘秒内不得重叠
        if sound_system_has_foley_played_too_recently(self, foley_type)
            && !crate::lawn::zombie::test_bit(a_foley_params.foley_flags, FoleyFlags::Loop as u32)
        {
            return;
        }

        // 对应 C++: ONE_AT_A_TIME——已存在实例则引用计数 +1
        if crate::lawn::zombie::test_bit(a_foley_params.foley_flags, FoleyFlags::OneAtATime as u32) {
            if let Some(a_foley_instance) = sound_system_find_instance(self, foley_type) {
                unsafe {
                    (*a_foley_instance).ref_count += 1;
                    (*a_foley_instance).start_time = current_update_count();
                }
                return;
            }
        }
        let a_foley_instance = match sound_system_get_free_instance_index(self as *const TodFoley as *mut TodFoley, foley_type) {
            Some(inst) => inst,
            None => return, // 对应 C++: 全部实例占用
        };

        // 对应 C++: DONT_REPEAT 时排除上次播放的变式
        let mut a_variations: i32 = 0;
        let mut a_variations_array = [0i32; 10];
        let a_dont_repeat =
            crate::lawn::zombie::test_bit(a_foley_params.foley_flags, FoleyFlags::DontRepeat as u32);
        let a_last_variation = self.foley_type_data[foley_type as usize].last_variation_played;
        for i in 0..10 {
            if !a_dont_repeat || a_last_variation != i as i32 {
                if a_foley_params.sfx_id[i].is_none() {
                    break;
                }
                a_variations_array[a_variations as usize] = i as i32;
                a_variations += 1;
            }
        }
        if a_variations <= 0 {
            return;
        }
        // 对应 C++: PvzpPickFromArray 随机挑选变式
        let a_variation = a_variations_array[RandRange(a_variations) as usize];
        unsafe {
            (&mut (*(self as *const TodFoley as *mut TodFoley)).foley_type_data)[foley_type as usize].last_variation_played = a_variation;
        }

        // [TRANSLATION_NOTE]: C++ 中经 gSexyAppBase->mSoundManager->GetSoundInstance(id)
        // 创建 SoundInstance；Rust SoundManager trait 无该方法（sdl_sound_manager::get_sound_instance
        // 为桩），此处以 SoundManager::PlaySound 直接播放近似，实例引用计数状态机保持结构。
        let a_sfx_id = match a_foley_params.sfx_id[a_variation as usize] {
            Some(p) => unsafe { *p as i32 },
            None => 0,
        };
        if let Some(app) = crate::lawn::lawn_app::LawnApp::instance() {
            if let Some(sm) = app.base.sound_manager {
                unsafe {
                    (*sm).play_sound(a_sfx_id);
                }
            }
        }

        unsafe {
            (*a_foley_instance).instance = None; // 实例句柄待音频层补全
            (*a_foley_instance).ref_count = 1;
            (*a_foley_instance).start_time = current_update_count();
        }
        // 对应 C++: USES_MUSIC_VOLUME 时按音乐音量播放
        if crate::lawn::zombie::test_bit(a_foley_params.foley_flags, FoleyFlags::UsesMusicVolume as u32) {
            let a_inst = unsafe { &*a_foley_instance };
            self.apply_music_volume(a_inst);
        }
    }

    /// 取消暂停中的拟音（对应 C++ PvzpFoley::CancelPausedFoley，:357）
    pub fn cancel_paused_foley(&self) {
        sound_system_cancel_paused_foley(self as *const TodFoley as *mut TodFoley);
    }

    /// 应用音乐音量（对应 C++ PvzpFoley::ApplyMusicVolume，:386）
    pub fn apply_music_volume(&self, foley_instance: &FoleyInstance) {
        // [TRANSLATION_NOTE]: C++ 中 SetVolume(mMusicVolume / mSfxVolume) 使音量与音乐一致；
        // Rust 音频层暂无实例级音量，具体等声音系统恢复
        let _ = foley_instance;
    }

    /// 重挂钩音乐音量的拟音（对应 C++ PvzpFoley::RehookupSoundWithMusicVolume，:395）
    pub fn rehookup_sound_with_music_volume(&self) {
        sound_system_release_finished_instances(self as *const TodFoley as *mut TodFoley);
        let a_param_count = foley_param_count();
        for a_foley_type in 0..a_param_count {
            let a_params = match lookup_foley_index(a_foley_type) {
                Some(p) => p,
                None => continue,
            };
            if crate::lawn::zombie::test_bit(a_params.foley_flags, FoleyFlags::UsesMusicVolume as u32) {
                for i in 0..MAX_FOLEY_INSTANCES {
                    let a_foley_instance = &self.foley_type_data[a_foley_type].foley_instances[i];
                    if a_foley_instance.ref_count != 0 {
                        self.apply_music_volume(a_foley_instance);
                    }
                }
            }
        }
    }
}

impl Default for TodFoley {
    fn default() -> Self {
        TodFoley::new()
    }
}

// --- 全局参数表（对应 C++ gFoleyParamArray/gFoleyParamArraySize） ---
static mut G_FOLEY_PARAM_ARRAY: Vec<FoleyParams> = Vec::new();

/// 注册拟音参数表（对应 C++ PvzpFoleyInitialize，PvzpFoley.cpp:160）
pub fn foley_initialize(params: &[FoleyParams]) {
    unsafe {
        G_FOLEY_PARAM_ARRAY = params.to_vec();
    }
}

/// 释放拟音参数表（对应 C++ PvzpFoleyDispose）
pub fn foley_dispose() {
    unsafe {
        G_FOLEY_PARAM_ARRAY.clear();
    }
}

/// 拟音参数表大小（对应 C++ gFoleyParamArraySize）
pub fn foley_param_count() -> usize {
    unsafe { G_FOLEY_PARAM_ARRAY.len() }
}

/// 查找拟音参数（对应 C++ LookupFoley）
pub fn lookup_foley(foley_type: FoleyType) -> Option<&'static FoleyParams> {
    lookup_foley_index(foley_type as usize)
}

/// 按索引查找拟音参数
pub fn lookup_foley_index(foley_type: usize) -> Option<&'static FoleyParams> {
    unsafe {
        if foley_type >= G_FOLEY_PARAM_ARRAY.len() {
            return None;
        }
        Some(&*(G_FOLEY_PARAM_ARRAY.as_ptr().add(foley_type)))
    }
}

/// 当前更新帧计数（对应 C++ gSexyAppBase->mUpdateCount）
fn current_update_count() -> i32 {
    crate::lawn::lawn_app::LawnApp::instance()
        .map_or(0, |app| app.base.m_update_count)
}

// --- 自由函数（对应 C++ PvzpFoley.cpp 自由函数） ---
/// 暂停/恢复拟音（对应 C++ PvzpFoley::GamePause 主体）
/// [TRANSLATION_NOTE]: 以 *mut 接收（对应 C++ 指针参数），内部 `&mut *` 从 raw 指针创建引用
pub fn sound_system_game_pause(sound_system: *mut TodFoley, entering_pause: bool) {
    if sound_system.is_null() {
        return;
    }
    sound_system_release_finished_instances(sound_system);
    let ss = unsafe { &mut *sound_system };
    let a_param_count = foley_param_count();
    for a_foley_type in 0..a_param_count {
        let a_params = match lookup_foley_index(a_foley_type) {
            Some(p) => p,
            None => continue,
        };
        if crate::lawn::zombie::test_bit(a_params.foley_flags, FoleyFlags::MuteOnPause as u32) {
            let a_foley_data = &mut ss.foley_type_data[a_foley_type];
            for i in 0..MAX_FOLEY_INSTANCES {
                let a_foley_instance = &mut a_foley_data.foley_instances[i];
                if a_foley_instance.ref_count != 0 {
                    if entering_pause {
                        a_foley_instance.paused = true;
                        a_foley_instance.pause_offset = 0;
                        if let Some(inst) = a_foley_instance.instance {
                            unsafe { (*inst).stop(); }
                        }
                    } else if a_foley_instance.paused {
                        a_foley_instance.paused = false;
                        let a_is_looping = crate::lawn::zombie::test_bit(
                            a_params.foley_flags, FoleyFlags::Loop as u32);
                        if let Some(inst) = a_foley_instance.instance {
                            unsafe { (*inst).play(a_is_looping, false); }
                        }
                    }
                }
            }
        }
    }
}

/// 取消暂停中的拟音（对应 C++ PvzpFoley::CancelPausedFoley 主体）
pub fn sound_system_cancel_paused_foley(sound_system: *mut TodFoley) {
    if sound_system.is_null() {
        return;
    }
    sound_system_release_finished_instances(sound_system);
    let ss = unsafe { &mut *sound_system };
    let a_param_count = foley_param_count();
    for a_foley_type in 0..a_param_count {
        let a_foley_data = &mut ss.foley_type_data[a_foley_type];
        for i in 0..MAX_FOLEY_INSTANCES {
            let a_foley_instance = &mut a_foley_data.foley_instances[i];
            if a_foley_instance.ref_count != 0 && a_foley_instance.paused {
                a_foley_instance.ref_count = 0;
                if let Some(inst) = a_foley_instance.instance {
                    unsafe { (*inst).release(); }
                }
                a_foley_instance.instance = None;
            }
        }
    }
}

/// 释放已播放完毕的实例（对应 C++ SoundSystemReleaseFinishedInstances，:173）
/// [TRANSLATION_NOTE]: 以 *mut 接收（对应 C++ 指针参数），内部 `&mut *` 从 raw 指针
/// 创建引用，规避新版 rustc 的 invalid_reference_casting deny。
pub fn sound_system_release_finished_instances(sound_system: *mut TodFoley) {
    if sound_system.is_null() {
        return;
    }
    let sound_system_ref = unsafe { &mut *sound_system };
    let a_param_count = foley_param_count();
    for a_foley_type in 0..a_param_count {
        for i in 0..MAX_FOLEY_INSTANCES {
            let a_foley_instance = &mut sound_system_ref.foley_type_data[a_foley_type].foley_instances[i];
            if a_foley_instance.ref_count == 0 {
                // 对应 C++: PVZP_ASSERT(mInstance == nullptr)
            } else if !a_foley_instance.paused {
                // 对应 C++: 实例不再播放则释放
                let a_finished = match a_foley_instance.instance {
                    Some(inst) => !unsafe { (*inst).is_playing() },
                    None => true,
                };
                if a_finished {
                    if let Some(inst) = a_foley_instance.instance {
                        unsafe { (*inst).release(); }
                    }
                    a_foley_instance.instance = None;
                    a_foley_instance.ref_count = 0;
                }
            }
        }
    }
}

/// 该拟音是否在最近 10 厘秒内播放过（对应 C++ SoundSystemHasFoleyPlayedTooRecently，:196）
pub fn sound_system_has_foley_played_too_recently(sound_system: &TodFoley, foley_type: FoleyType) -> bool {
    let a_foley_data = &sound_system.foley_type_data[foley_type as usize];
    let a_update_count = current_update_count();
    for i in 0..MAX_FOLEY_INSTANCES {
        let a_foley_instance = &a_foley_data.foley_instances[i];
        if a_foley_instance.ref_count != 0 && a_update_count - a_foley_instance.start_time < 10 {
            return true;
        }
    }
    false
}

/// 查找已激活实例（对应 C++ SoundSystemFindInstance，:217）
pub fn sound_system_find_instance(sound_system: &TodFoley, foley_type: FoleyType) -> Option<*mut FoleyInstance> {
    let a_foley_data = &sound_system.foley_type_data[foley_type as usize];
    for i in 0..MAX_FOLEY_INSTANCES {
        let a_foley_instance = &a_foley_data.foley_instances[i];
        if a_foley_instance.ref_count > 0 {
            return Some(&sound_system.foley_type_data[foley_type as usize].foley_instances[i] as *const FoleyInstance as *mut FoleyInstance);
        }
    }
    None
}

/// 获取空闲实例（对应 C++ SoundSystemGetFreeInstanceIndex，:232）
pub fn sound_system_get_free_instance_index(sound_system: *mut TodFoley, foley_type: FoleyType) -> Option<*mut FoleyInstance> {
    if sound_system.is_null() {
        return None;
    }
    let sound_system_ref = unsafe { &mut *sound_system };
    let a_foley_data = &mut sound_system_ref.foley_type_data[foley_type as usize];
    for i in 0..MAX_FOLEY_INSTANCES {
        if a_foley_data.foley_instances[i].ref_count == 0 {
            return Some(&mut a_foley_data.foley_instances[i] as *mut FoleyInstance);
        }
    }
    None
}

// --- 全局变量 ---
// 对应 C++: extern int gFoleyParamArraySize;
// 对应 C++: extern const FoleyParams* gFoleyParamArray;
// 对应 C++: extern const FoleyParams gLawnFoleyParamArray[NUM_FOLEY];

// FoleyParams 含 SOUND_XXX 指针（*mut usize），程序内单线程使用，unsafe impl Sync 使 static 表合法
unsafe impl Sync for FoleyParams {}
// ── 草坪拟音 SOUND 变量（对应 C++ Resources.h intptr_t SOUND_XXX，PvzpFoley.cpp gLawnFoleyParamArray 引用） ──
pub static mut SOUND_POINTS: i32 = -1;
pub static mut SOUND_SPLAT: i32 = -1;
pub static mut SOUND_SPLAT2: i32 = -1;
pub static mut SOUND_SPLAT3: i32 = -1;
pub static mut SOUND_LAWNMOWER: i32 = -1;
pub static mut SOUND_THROW: i32 = -1;
pub static mut SOUND_THROW2: i32 = -1;
pub static mut SOUND_CHOMP: i32 = -1;
pub static mut SOUND_CHOMP2: i32 = -1;
pub static mut SOUND_CHOMPSOFT: i32 = -1;
pub static mut SOUND_PLANT: i32 = -1;
pub static mut SOUND_PLANT2: i32 = -1;
pub static mut SOUND_TAP2: i32 = -1;
pub static mut SOUND_BLEEP: i32 = -1;
pub static mut SOUND_GROAN: i32 = -1;
pub static mut SOUND_GROAN2: i32 = -1;
pub static mut SOUND_GROAN3: i32 = -1;
pub static mut SOUND_GROAN4: i32 = -1;
pub static mut SOUND_GROAN5: i32 = -1;
pub static mut SOUND_GROAN6: i32 = -1;
pub static mut SOUND_SUKHBIR4: i32 = -1;
pub static mut SOUND_SUKHBIR5: i32 = -1;
pub static mut SOUND_SUKHBIR6: i32 = -1;
pub static mut SOUND_SUKHBIR: i32 = -1;
pub static mut SOUND_SUKHBIR2: i32 = -1;
pub static mut SOUND_SUKHBIR3: i32 = -1;
pub static mut SOUND_JACKINTHEBOX: i32 = -1;
pub static mut SOUND_DIAMOND: i32 = -1;
pub static mut SOUND_ZAMBONI: i32 = -1;
pub static mut SOUND_THUNDER: i32 = -1;
pub static mut SOUND_FROZEN: i32 = -1;
pub static mut SOUND_PLANT_WATER: i32 = -1;
pub static mut SOUND_ZOMBIE_ENTERING_WATER: i32 = -1;
pub static mut SOUND_BOWLINGIMPACT: i32 = -1;
pub static mut SOUND_BALLOON_POP: i32 = -1;
pub static mut SOUND_EXPLOSION: i32 = -1;
pub static mut SOUND_SLURP: i32 = -1;
pub static mut SOUND_LIMBS_POP: i32 = -1;
pub static mut SOUND_POGO_ZOMBIE: i32 = -1;
pub static mut SOUND_SNOW_PEA_SPARKLES: i32 = -1;
pub static mut SOUND_ZOMBIE_FALLING_1: i32 = -1;
pub static mut SOUND_ZOMBIE_FALLING_2: i32 = -1;
pub static mut SOUND_PUFF: i32 = -1;
pub static mut SOUND_FUME: i32 = -1;
pub static mut SOUND_COIN: i32 = -1;
pub static mut SOUND_KERNELPULT: i32 = -1;
pub static mut SOUND_KERNELPULT2: i32 = -1;
pub static mut SOUND_DIGGER_ZOMBIE: i32 = -1;
pub static mut SOUND_JACK_SURPRISE: i32 = -1;
pub static mut SOUND_JACK_SURPRISE2: i32 = -1;
pub static mut SOUND_VASE_BREAKING: i32 = -1;
pub static mut SOUND_POOL_CLEANER: i32 = -1;
pub static mut SOUND_BASKETBALL: i32 = -1;
pub static mut SOUND_IGNITE: i32 = -1;
pub static mut SOUND_IGNITE2: i32 = -1;
pub static mut SOUND_FIREPEA: i32 = -1;
pub static mut SOUND_GARGANTUAR_THUMP: i32 = -1;
pub static mut SOUND_SQUASH_HMM: i32 = -1;
pub static mut SOUND_SQUASH_HMM2: i32 = -1;
pub static mut SOUND_MAGNETSHROOM: i32 = -1;
pub static mut SOUND_BUTTER: i32 = -1;
pub static mut SOUND_BUNGEE_SCREAM: i32 = -1;
pub static mut SOUND_BUNGEE_SCREAM2: i32 = -1;
pub static mut SOUND_BUNGEE_SCREAM3: i32 = -1;
pub static mut SOUND_SHIELDHIT: i32 = -1;
pub static mut SOUND_SHIELDHIT2: i32 = -1;
pub static mut SOUND_SWING: i32 = -1;
pub static mut SOUND_BONK: i32 = -1;
pub static mut SOUND_RAIN: i32 = -1;
pub static mut SOUND_DOLPHIN_BEFORE_JUMPING: i32 = -1;
pub static mut SOUND_DOLPHIN_APPEARS: i32 = -1;
pub static mut SOUND_GRAVEBUSTERCHOMP: i32 = -1;
pub static mut SOUND_CHERRYBOMB: i32 = -1;
pub static mut SOUND_JALAPENO: i32 = -1;
pub static mut SOUND_REVERSE_EXPLOSION: i32 = -1;
pub static mut SOUND_PLASTICHIT: i32 = -1;
pub static mut SOUND_PLASTICHIT2: i32 = -1;
pub static mut SOUND_WINMUSIC: i32 = -1;
pub static mut SOUND_BALLOONINFLATE: i32 = -1;
pub static mut SOUND_BIGCHOMP: i32 = -1;
pub static mut SOUND_MELONIMPACT: i32 = -1;
pub static mut SOUND_MELONIMPACT2: i32 = -1;
pub static mut SOUND_PLANTGROW: i32 = -1;
pub static mut SOUND_SHOOP: i32 = -1;
pub static mut SOUND_JUICY: i32 = -1;
pub static mut SOUND_NEWSPAPER_RARRGH: i32 = -1;
pub static mut SOUND_NEWSPAPER_RARRGH2: i32 = -1;
pub static mut SOUND_NEWSPAPER_RIP: i32 = -1;
pub static mut SOUND_FLOOP: i32 = -1;
pub static mut SOUND_COFFEE: i32 = -1;
pub static mut SOUND_LOWGROAN: i32 = -1;
pub static mut SOUND_LOWGROAN2: i32 = -1;
pub static mut SOUND_PRIZE: i32 = -1;
pub static mut SOUND_YUCK: i32 = -1;
pub static mut SOUND_YUCK2: i32 = -1;
pub static mut SOUND_GRASSSTEP: i32 = -1;
pub static mut SOUND_SHOVEL: i32 = -1;
pub static mut SOUND_COBLAUNCH: i32 = -1;
pub static mut SOUND_WATERING: i32 = -1;
pub static mut SOUND_POLEVAULT: i32 = -1;
pub static mut SOUND_GRAVESTONE_RUMBLE: i32 = -1;
pub static mut SOUND_DIRT_RISE: i32 = -1;
pub static mut SOUND_FERTILIZER: i32 = -1;
pub static mut SOUND_PORTAL: i32 = -1;
pub static mut SOUND_WAKEUP: i32 = -1;
pub static mut SOUND_BUGSPRAY: i32 = -1;
pub static mut SOUND_SCREAM: i32 = -1;
pub static mut SOUND_PAPER: i32 = -1;
pub static mut SOUND_MONEYFALLS: i32 = -1;
pub static mut SOUND_IMP: i32 = -1;
pub static mut SOUND_IMP2: i32 = -1;
pub static mut SOUND_HYDRAULIC_SHORT: i32 = -1;
pub static mut SOUND_HYDRAULIC: i32 = -1;
pub static mut SOUND_GARGANTUDEATH: i32 = -1;
pub static mut SOUND_CERAMIC: i32 = -1;
pub static mut SOUND_BOSSBOULDERATTACK: i32 = -1;
pub static mut SOUND_CHIME: i32 = -1;
pub static mut SOUND_CRAZYDAVESHORT1: i32 = -1;
pub static mut SOUND_CRAZYDAVESHORT2: i32 = -1;
pub static mut SOUND_CRAZYDAVESHORT3: i32 = -1;
pub static mut SOUND_CRAZYDAVELONG1: i32 = -1;
pub static mut SOUND_CRAZYDAVELONG2: i32 = -1;
pub static mut SOUND_CRAZYDAVELONG3: i32 = -1;
pub static mut SOUND_CRAZYDAVEEXTRALONG1: i32 = -1;
pub static mut SOUND_CRAZYDAVEEXTRALONG2: i32 = -1;
pub static mut SOUND_CRAZYDAVEEXTRALONG3: i32 = -1;
pub static mut SOUND_CRAZYDAVECRAZY: i32 = -1;
pub static mut SOUND_PHONOGRAPH: i32 = -1;
pub static mut SOUND_DANCER: i32 = -1;
pub static mut SOUND_FINALFANFARE: i32 = -1;
pub static mut SOUND_CRAZYDAVESCREAM: i32 = -1;
pub static mut SOUND_CRAZYDAVESCREAM2: i32 = -1;
pub static mut SOUND_BUZZER: i32 = -1;
pub static mut SOUND_TAP: i32 = -1;
pub static mut SOUND_FINALWAVE: i32 = -1;
pub static mut SOUND_HUGE_WAVE: i32 = -1;
pub static mut SOUND_LIGHTFILL: i32 = -1;
pub static mut SOUND_PAUSE: i32 = -1;
pub static mut SOUND_GRAVEBUTTON: i32 = -1;
pub static mut SOUND_BOING: i32 = -1;
pub static mut SOUND_SEEDLIFT: i32 = -1;
pub static mut SOUND_SIREN: i32 = -1;
pub static mut SOUND_TAPGLASS: i32 = -1;
pub static mut SOUND_LOSEMUSIC: i32 = -1;

/// 草坪拟音参数表（对应 C++ gLawnFoleyParamArray，PvzpFoley.cpp:30，104 项）
/// [TRANSLATION_NOTE]: 指针指向上方 SOUND_XXX 变量（C++ &Sexy::SOUND_XXX 地址语义），
/// 加载声音后变量值 = SoundManager 槽位 id（C++ GetSoundThrow 赋值）
pub static G_LAWN_FOLEY_PARAM_ARRAY: [FoleyParams; 104] = [
    FoleyParams { foley_type: FoleyType::Sun, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_POINTS) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Splat, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_SPLAT) as *mut usize), Some(std::ptr::addr_of!(SOUND_SPLAT2) as *mut usize), Some(std::ptr::addr_of!(SOUND_SPLAT3) as *mut usize), None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Lawnmower, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_LAWNMOWER) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Throw, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_THROW) as *mut usize), Some(std::ptr::addr_of!(SOUND_THROW) as *mut usize), Some(std::ptr::addr_of!(SOUND_THROW) as *mut usize), Some(std::ptr::addr_of!(SOUND_THROW2) as *mut usize), None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::SpawnSun, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_THROW) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Chomp, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CHOMP) as *mut usize), Some(std::ptr::addr_of!(SOUND_CHOMP2) as *mut usize), None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::ChompSoft, pitch_range: 4f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CHOMPSOFT) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Plant, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_PLANT) as *mut usize), Some(std::ptr::addr_of!(SOUND_PLANT2) as *mut usize), None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::UseShovel, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_PLANT2) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Drop, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_TAP2) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Beep, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_BLEEP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Groan, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_GROAN) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN2) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN3) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN4) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN5) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN6) as *mut usize), None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Brains, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_GROAN) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN2) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN3) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN4) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN5) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN6) as *mut usize), Some(std::ptr::addr_of!(SOUND_SUKHBIR4) as *mut usize), Some(std::ptr::addr_of!(SOUND_SUKHBIR5) as *mut usize), Some(std::ptr::addr_of!(SOUND_SUKHBIR6) as *mut usize), None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Sukhbir, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_GROAN) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN2) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN3) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN4) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN5) as *mut usize), Some(std::ptr::addr_of!(SOUND_GROAN6) as *mut usize), Some(std::ptr::addr_of!(SOUND_SUKHBIR) as *mut usize), Some(std::ptr::addr_of!(SOUND_SUKHBIR2) as *mut usize), Some(std::ptr::addr_of!(SOUND_SUKHBIR3) as *mut usize), None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::JackInTheBox, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_JACKINTHEBOX) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 7 },
    FoleyParams { foley_type: FoleyType::ArtChallenge, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_DIAMOND) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Zamboni, pitch_range: 5f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_ZAMBONI) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Thunder, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_THUNDER) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Frozen, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_FROZEN) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::ZombieSplash, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_PLANT_WATER) as *mut usize), Some(std::ptr::addr_of!(SOUND_ZOMBIE_ENTERING_WATER) as *mut usize), None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::BowlingImpact, pitch_range: -3f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_BOWLINGIMPACT) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Squish, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CHOMP) as *mut usize), Some(std::ptr::addr_of!(SOUND_CHOMP2) as *mut usize), None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::TirePop, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_BALLOON_POP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Explosion, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_EXPLOSION) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Slurp, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_SLURP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::LimbsPop, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_LIMBS_POP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::PogoZombie, pitch_range: 4f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_POGO_ZOMBIE) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::SnowPeaSparkles, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_SNOW_PEA_SPARKLES) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::ZombieFalling, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_ZOMBIE_FALLING_1) as *mut usize), Some(std::ptr::addr_of!(SOUND_ZOMBIE_FALLING_2) as *mut usize), None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Puff, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_PUFF) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Fume, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_FUME) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Coin, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_COIN) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::KernelSplat, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_KERNELPULT) as *mut usize), Some(std::ptr::addr_of!(SOUND_KERNELPULT2) as *mut usize), None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Digger, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_DIGGER_ZOMBIE) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 7 },
    FoleyParams { foley_type: FoleyType::JackSurprise, pitch_range: 1f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_JACK_SURPRISE) as *mut usize), Some(std::ptr::addr_of!(SOUND_JACK_SURPRISE) as *mut usize), Some(std::ptr::addr_of!(SOUND_JACK_SURPRISE2) as *mut usize), None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::VaseBreaking, pitch_range: -5f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_VASE_BREAKING) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::PoolCleaner, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_POOL_CLEANER) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Basketball, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_BASKETBALL) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Ignite, pitch_range: 5f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_IGNITE) as *mut usize), Some(std::ptr::addr_of!(SOUND_IGNITE) as *mut usize), Some(std::ptr::addr_of!(SOUND_IGNITE) as *mut usize), Some(std::ptr::addr_of!(SOUND_IGNITE2) as *mut usize), None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::FirePea, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_FIREPEA) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Thump, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_GARGANTUAR_THUMP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::SquashHmm, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_SQUASH_HMM) as *mut usize), Some(std::ptr::addr_of!(SOUND_SQUASH_HMM) as *mut usize), Some(std::ptr::addr_of!(SOUND_SQUASH_HMM2) as *mut usize), None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Magnetshroom, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_MAGNETSHROOM) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Butter, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_BUTTER) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::BungeeScream, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_BUNGEE_SCREAM) as *mut usize), Some(std::ptr::addr_of!(SOUND_BUNGEE_SCREAM2) as *mut usize), Some(std::ptr::addr_of!(SOUND_BUNGEE_SCREAM3) as *mut usize), None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::BossExplosionSmall, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_EXPLOSION) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::ShieldHit, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_SHIELDHIT) as *mut usize), Some(std::ptr::addr_of!(SOUND_SHIELDHIT2) as *mut usize), None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Swing, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_SWING) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Bonk, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_BONK) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Rain, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_RAIN) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 5 },
    FoleyParams { foley_type: FoleyType::DolphinBeforeJumping, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_DOLPHIN_BEFORE_JUMPING) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::DolphinAppears, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_DOLPHIN_APPEARS) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::PlantWater, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_PLANT_WATER) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::ZombieEnteringWater, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_ZOMBIE_ENTERING_WATER) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::GravebusterChomp, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_GRAVEBUSTERCHOMP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 4 },
    FoleyParams { foley_type: FoleyType::Cherrybomb, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CHERRYBOMB) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::JalapenoIgnite, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_JALAPENO) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::ReverseExplosion, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_REVERSE_EXPLOSION) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::PlasticHit, pitch_range: 5f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_PLASTICHIT) as *mut usize), Some(std::ptr::addr_of!(SOUND_PLASTICHIT2) as *mut usize), None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::WinMusic, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_WINMUSIC) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 8 },
    FoleyParams { foley_type: FoleyType::BalloonInflate, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_BALLOONINFLATE) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::BigChomp, pitch_range: -2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_BIGCHOMP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::MelonImpact, pitch_range: -5f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_MELONIMPACT) as *mut usize), Some(std::ptr::addr_of!(SOUND_MELONIMPACT2) as *mut usize), None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::PlantGrow, pitch_range: -2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_PLANTGROW) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Shoop, pitch_range: -5f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_SHOOP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Juicy, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_JUICY) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::NewspaperRarrgh, pitch_range: -2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_NEWSPAPER_RARRGH) as *mut usize), Some(std::ptr::addr_of!(SOUND_NEWSPAPER_RARRGH2) as *mut usize), Some(std::ptr::addr_of!(SOUND_NEWSPAPER_RARRGH2) as *mut usize), None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::NewspaperRip, pitch_range: -2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_NEWSPAPER_RIP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Floop, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_FLOOP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Coffee, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_COFFEE) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::LowGroan, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_LOWGROAN) as *mut usize), Some(std::ptr::addr_of!(SOUND_LOWGROAN2) as *mut usize), None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Prize, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_PRIZE) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Yuck, pitch_range: 1f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_YUCK) as *mut usize), Some(std::ptr::addr_of!(SOUND_YUCK) as *mut usize), Some(std::ptr::addr_of!(SOUND_YUCK2) as *mut usize), None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Umbrella, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_THROW2) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::GrassStep, pitch_range: 2f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_GRASSSTEP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Shovel, pitch_range: 5f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_SHOVEL) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::CobLaunch, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_COBLAUNCH) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Watering, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_WATERING) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Polevault, pitch_range: 5f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_POLEVAULT) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::GraveStoneRumble, pitch_range: 10f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_GRAVESTONE_RUMBLE) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::DirtRise, pitch_range: 5f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_DIRT_RISE) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Fertilizer, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_FERTILIZER) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Portal, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_PORTAL) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::WakeUp, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_WAKEUP) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::BugSpray, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_BUGSPRAY) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Scream, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_SCREAM) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Paper, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_PAPER) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::MoneyFalls, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_MONEYFALLS) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Imp, pitch_range: 5f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_IMP) as *mut usize), Some(std::ptr::addr_of!(SOUND_IMP2) as *mut usize), None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::HydraulicShort, pitch_range: 3f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_HYDRAULIC_SHORT) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Hydraulic, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_HYDRAULIC) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Gargantudeath, pitch_range: 3f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_GARGANTUDEATH) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Ceramic, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CERAMIC) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::BossBoulderAttack, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_BOSSBOULDERATTACK) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Chime, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CHIME) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::CrazyDaveShort, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CRAZYDAVESHORT1) as *mut usize), Some(std::ptr::addr_of!(SOUND_CRAZYDAVESHORT2) as *mut usize), Some(std::ptr::addr_of!(SOUND_CRAZYDAVESHORT3) as *mut usize), None, None, None, None, None, None, None], foley_flags: 16 },
    FoleyParams { foley_type: FoleyType::CrazyDaveLong, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CRAZYDAVELONG1) as *mut usize), Some(std::ptr::addr_of!(SOUND_CRAZYDAVELONG2) as *mut usize), Some(std::ptr::addr_of!(SOUND_CRAZYDAVELONG3) as *mut usize), None, None, None, None, None, None, None], foley_flags: 16 },
    FoleyParams { foley_type: FoleyType::CrazyDaveExtraLong, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CRAZYDAVEEXTRALONG1) as *mut usize), Some(std::ptr::addr_of!(SOUND_CRAZYDAVEEXTRALONG2) as *mut usize), Some(std::ptr::addr_of!(SOUND_CRAZYDAVEEXTRALONG3) as *mut usize), None, None, None, None, None, None, None], foley_flags: 16 },
    FoleyParams { foley_type: FoleyType::CrazyDaveCrazy, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CRAZYDAVECRAZY) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Phonograph, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_PHONOGRAPH) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::Dancer, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_DANCER) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 6 },
    FoleyParams { foley_type: FoleyType::FinalFanfare, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_FINALFANFARE) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::CrazyDaveScream, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CRAZYDAVESCREAM) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
    FoleyParams { foley_type: FoleyType::CrazyDaveScream2, pitch_range: 0f32, sfx_id: [Some(std::ptr::addr_of!(SOUND_CRAZYDAVESCREAM2) as *mut usize), None, None, None, None, None, None, None, None, None], foley_flags: 0 },
];

/// 按资源名把 SoundManager 槽位 id 写入对应 SOUND 变量（对应 C++ GetSoundThrow 赋值）
pub fn assign_sound_id(name: &str, id: i32) {
    unsafe {
        match name {
            "sound_points" => SOUND_POINTS = id,
            "sound_splat" => SOUND_SPLAT = id,
            "sound_splat2" => SOUND_SPLAT2 = id,
            "sound_splat3" => SOUND_SPLAT3 = id,
            "sound_lawnmower" => SOUND_LAWNMOWER = id,
            "sound_throw" => SOUND_THROW = id,
            "sound_throw2" => SOUND_THROW2 = id,
            "sound_chomp" => SOUND_CHOMP = id,
            "sound_chomp2" => SOUND_CHOMP2 = id,
            "sound_chompsoft" => SOUND_CHOMPSOFT = id,
            "sound_plant" => SOUND_PLANT = id,
            "sound_plant2" => SOUND_PLANT2 = id,
            "sound_tap2" => SOUND_TAP2 = id,
            "sound_bleep" => SOUND_BLEEP = id,
            "sound_groan" => SOUND_GROAN = id,
            "sound_groan2" => SOUND_GROAN2 = id,
            "sound_groan3" => SOUND_GROAN3 = id,
            "sound_groan4" => SOUND_GROAN4 = id,
            "sound_groan5" => SOUND_GROAN5 = id,
            "sound_groan6" => SOUND_GROAN6 = id,
            "sound_sukhbir4" => SOUND_SUKHBIR4 = id,
            "sound_sukhbir5" => SOUND_SUKHBIR5 = id,
            "sound_sukhbir6" => SOUND_SUKHBIR6 = id,
            "sound_sukhbir" => SOUND_SUKHBIR = id,
            "sound_sukhbir2" => SOUND_SUKHBIR2 = id,
            "sound_sukhbir3" => SOUND_SUKHBIR3 = id,
            "sound_jackinthebox" => SOUND_JACKINTHEBOX = id,
            "sound_diamond" => SOUND_DIAMOND = id,
            "sound_zamboni" => SOUND_ZAMBONI = id,
            "sound_thunder" => SOUND_THUNDER = id,
            "sound_frozen" => SOUND_FROZEN = id,
            "sound_plant_water" => SOUND_PLANT_WATER = id,
            "sound_zombie_entering_water" => SOUND_ZOMBIE_ENTERING_WATER = id,
            "sound_bowlingimpact" => SOUND_BOWLINGIMPACT = id,
            "sound_balloon_pop" => SOUND_BALLOON_POP = id,
            "sound_explosion" => SOUND_EXPLOSION = id,
            "sound_slurp" => SOUND_SLURP = id,
            "sound_limbs_pop" => SOUND_LIMBS_POP = id,
            "sound_pogo_zombie" => SOUND_POGO_ZOMBIE = id,
            "sound_snow_pea_sparkles" => SOUND_SNOW_PEA_SPARKLES = id,
            "sound_zombie_falling_1" => SOUND_ZOMBIE_FALLING_1 = id,
            "sound_zombie_falling_2" => SOUND_ZOMBIE_FALLING_2 = id,
            "sound_puff" => SOUND_PUFF = id,
            "sound_fume" => SOUND_FUME = id,
            "sound_coin" => SOUND_COIN = id,
            "sound_kernelpult" => SOUND_KERNELPULT = id,
            "sound_kernelpult2" => SOUND_KERNELPULT2 = id,
            "sound_digger_zombie" => SOUND_DIGGER_ZOMBIE = id,
            "sound_jack_surprise" => SOUND_JACK_SURPRISE = id,
            "sound_jack_surprise2" => SOUND_JACK_SURPRISE2 = id,
            "sound_vase_breaking" => SOUND_VASE_BREAKING = id,
            "sound_pool_cleaner" => SOUND_POOL_CLEANER = id,
            "sound_basketball" => SOUND_BASKETBALL = id,
            "sound_ignite" => SOUND_IGNITE = id,
            "sound_ignite2" => SOUND_IGNITE2 = id,
            "sound_firepea" => SOUND_FIREPEA = id,
            "sound_gargantuar_thump" => SOUND_GARGANTUAR_THUMP = id,
            "sound_squash_hmm" => SOUND_SQUASH_HMM = id,
            "sound_squash_hmm2" => SOUND_SQUASH_HMM2 = id,
            "sound_magnetshroom" => SOUND_MAGNETSHROOM = id,
            "sound_butter" => SOUND_BUTTER = id,
            "sound_bungee_scream" => SOUND_BUNGEE_SCREAM = id,
            "sound_bungee_scream2" => SOUND_BUNGEE_SCREAM2 = id,
            "sound_bungee_scream3" => SOUND_BUNGEE_SCREAM3 = id,
            "sound_shieldhit" => SOUND_SHIELDHIT = id,
            "sound_shieldhit2" => SOUND_SHIELDHIT2 = id,
            "sound_swing" => SOUND_SWING = id,
            "sound_bonk" => SOUND_BONK = id,
            "sound_rain" => SOUND_RAIN = id,
            "sound_dolphin_before_jumping" => SOUND_DOLPHIN_BEFORE_JUMPING = id,
            "sound_dolphin_appears" => SOUND_DOLPHIN_APPEARS = id,
            "sound_gravebusterchomp" => SOUND_GRAVEBUSTERCHOMP = id,
            "sound_cherrybomb" => SOUND_CHERRYBOMB = id,
            "sound_jalapeno" => SOUND_JALAPENO = id,
            "sound_reverse_explosion" => SOUND_REVERSE_EXPLOSION = id,
            "sound_plastichit" => SOUND_PLASTICHIT = id,
            "sound_plastichit2" => SOUND_PLASTICHIT2 = id,
            "sound_winmusic" => SOUND_WINMUSIC = id,
            "sound_ballooninflate" => SOUND_BALLOONINFLATE = id,
            "sound_bigchomp" => SOUND_BIGCHOMP = id,
            "sound_melonimpact" => SOUND_MELONIMPACT = id,
            "sound_melonimpact2" => SOUND_MELONIMPACT2 = id,
            "sound_plantgrow" => SOUND_PLANTGROW = id,
            "sound_shoop" => SOUND_SHOOP = id,
            "sound_juicy" => SOUND_JUICY = id,
            "sound_newspaper_rarrgh" => SOUND_NEWSPAPER_RARRGH = id,
            "sound_newspaper_rarrgh2" => SOUND_NEWSPAPER_RARRGH2 = id,
            "sound_newspaper_rip" => SOUND_NEWSPAPER_RIP = id,
            "sound_floop" => SOUND_FLOOP = id,
            "sound_coffee" => SOUND_COFFEE = id,
            "sound_lowgroan" => SOUND_LOWGROAN = id,
            "sound_lowgroan2" => SOUND_LOWGROAN2 = id,
            "sound_prize" => SOUND_PRIZE = id,
            "sound_yuck" => SOUND_YUCK = id,
            "sound_yuck2" => SOUND_YUCK2 = id,
            "sound_grassstep" => SOUND_GRASSSTEP = id,
            "sound_shovel" => SOUND_SHOVEL = id,
            "sound_coblaunch" => SOUND_COBLAUNCH = id,
            "sound_watering" => SOUND_WATERING = id,
            "sound_polevault" => SOUND_POLEVAULT = id,
            "sound_gravestone_rumble" => SOUND_GRAVESTONE_RUMBLE = id,
            "sound_dirt_rise" => SOUND_DIRT_RISE = id,
            "sound_fertilizer" => SOUND_FERTILIZER = id,
            "sound_portal" => SOUND_PORTAL = id,
            "sound_wakeup" => SOUND_WAKEUP = id,
            "sound_bugspray" => SOUND_BUGSPRAY = id,
            "sound_scream" => SOUND_SCREAM = id,
            "sound_paper" => SOUND_PAPER = id,
            "sound_moneyfalls" => SOUND_MONEYFALLS = id,
            "sound_imp" => SOUND_IMP = id,
            "sound_imp2" => SOUND_IMP2 = id,
            "sound_hydraulic_short" => SOUND_HYDRAULIC_SHORT = id,
            "sound_hydraulic" => SOUND_HYDRAULIC = id,
            "sound_gargantudeath" => SOUND_GARGANTUDEATH = id,
            "sound_ceramic" => SOUND_CERAMIC = id,
            "sound_bossboulderattack" => SOUND_BOSSBOULDERATTACK = id,
            "sound_chime" => SOUND_CHIME = id,
            "sound_crazydaveshort1" => SOUND_CRAZYDAVESHORT1 = id,
            "sound_crazydaveshort2" => SOUND_CRAZYDAVESHORT2 = id,
            "sound_crazydaveshort3" => SOUND_CRAZYDAVESHORT3 = id,
            "sound_crazydavelong1" => SOUND_CRAZYDAVELONG1 = id,
            "sound_crazydavelong2" => SOUND_CRAZYDAVELONG2 = id,
            "sound_crazydavelong3" => SOUND_CRAZYDAVELONG3 = id,
            "sound_crazydaveextralong1" => SOUND_CRAZYDAVEEXTRALONG1 = id,
            "sound_crazydaveextralong2" => SOUND_CRAZYDAVEEXTRALONG2 = id,
            "sound_crazydaveextralong3" => SOUND_CRAZYDAVEEXTRALONG3 = id,
            "sound_crazydavecrazy" => SOUND_CRAZYDAVECRAZY = id,
            "sound_phonograph" => SOUND_PHONOGRAPH = id,
            "sound_dancer" => SOUND_DANCER = id,
            "sound_finalfanfare" => SOUND_FINALFANFARE = id,
            "sound_crazydavescream" => SOUND_CRAZYDAVESCREAM = id,
            "sound_crazydavescream2" => SOUND_CRAZYDAVESCREAM2 = id,
            "sound_buzzer" => SOUND_BUZZER = id,
            "sound_tap" => SOUND_TAP = id,
            "sound_finalwave" => SOUND_FINALWAVE = id,
            "sound_huge_wave" => SOUND_HUGE_WAVE = id,
            "sound_lightfill" => SOUND_LIGHTFILL = id,
            "sound_pause" => SOUND_PAUSE = id,
            "sound_gravebutton" => SOUND_GRAVEBUTTON = id,
            "sound_boing" => SOUND_BOING = id,
            "sound_seedlift" => SOUND_SEEDLIFT = id,
            "sound_siren" => SOUND_SIREN = id,
            "sound_tapglass" => SOUND_TAPGLASS = id,
            "sound_losemusic" => SOUND_LOSEMUSIC = id,
            _ => {}
        }
    }
}
