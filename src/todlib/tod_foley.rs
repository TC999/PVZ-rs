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
