// SDL 声音实例 — 对齐 C++ SexyAppFramework/sound/SDLSoundInstance.cpp（275 行）
// 含 SDL_mixer 变调播放回调（PitchHandlerFuncCallback）与 Mix_* 播放链

use crate::ffi::sdl_mixer;
use crate::framework::sound::sound_instance::SoundInstance;
use std::os::raw::{c_int, c_void};

/// 对应 C++ SoundManager.h:36 `#define MAX_CHANNELS 32`
pub const MAX_CHANNELS: i32 = 32;

/// 对应 C++ SDLSoundPitchHandler（SDLSoundInstance.h:38-51）
#[repr(C)]
pub struct SDLSoundPitchHandler {
    pub self_ptr: *mut SDLSoundInstance,
    /// C++: const Mix_Chunk*
    pub chunk: *const c_void,
    /// C++: const float* speed —— 指向所属实例的 mPitch
    pub speed: *const f32,
    /// 当前播放位置（毫秒）
    pub position: f32,
    /// 该次播放是否已进入变调例程（0/1）
    pub altered: i32,
    /// 对应 C++ 字段名 `loop`
    pub loop_flag: i32,
    /// 音效时长（毫秒）
    pub duration: i32,
    /// 样本索引数（bytes / sample size）
    pub chunk_size: i32,
    /// 播放结束时是否由回调自行 HaltChannel（0/1）
    pub self_halt: i32,
}

/// SDL 声音实例（对应 C++ SDLSoundInstance）
pub struct SDLSoundInstance {
    pub m_mix_chunk: *mut c_void,
    pub m_auto_release: bool,
    pub m_has_played: bool,
    pub m_released: bool,
    pub m_reserved_channel: i32,
    pub m_channel: i32,

    pub m_base_pan: i32,
    pub m_base_volume: f64,

    // NOT-WIRED: C++ 仅 SetPan（SDLSoundInstance.cpp:202）写入、RehupPan 为空实现，全仓无读取点（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    pub m_pan: i32,
    pub m_volume: f64,
    pub m_pitch: f32,
    pub m_pitch_handler: Box<SDLSoundPitchHandler>,

    // NOT-WIRED: C++ 仅构造时写 44100（SDLSoundInstance.cpp:48），全仓无读取点（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    pub m_default_frequency: u32,

    /// demo 模式下：视为播放结束的首次更新计数；0 表示未播放
    pub m_demo_end_update_count: u32,
    /// demo 模式下：当前播放的循环标志
    pub m_demo_looping: bool,

    // [TRANSLATION_NOTE]: C++ 通过 mSoundManagerP->mMixerFreq/mMixerChannels/mMixerFormat/
    // mMasterVolume 读取混音参数。Rust 侧 SoundManager 以 `*mut dyn SoundManager` 保存，
    // 无法上转为具体类型指针，故在创建实例时把这些参数拷贝进来（值在 Mix_OpenAudio 后不再变化）。
    pub mixer_freq: i32,
    pub mixer_channels: i32,
    pub mixer_format: u16,
    pub master_volume: f64,
}

/// C++: SDLSoundInstance::PitchHandlerFuncCallback（SDLSoundInstance.cpp:97-172）
///
/// # Safety
/// `user_data` 必须是指向仍然存活的 `SDLSoundPitchHandler` 的指针（由 SDLSoundInstance 持有），
/// 且该 handler 的 `self_ptr` 必须指向仍然存活的 SDLSoundInstance。
pub unsafe extern "C" fn pitch_handler_func_callback(
    mix_channel: c_int,
    stream: *mut c_void,
    length: c_int,
    user_data: *mut c_void,
) {
    if user_data.is_null() || stream.is_null() {
        return;
    }
    let handler = &mut *(user_data as *mut SDLSoundPitchHandler);
    if handler.self_ptr.is_null() {
        return;
    }
    let p_this = &mut *handler.self_ptr;

    let chunk_data = sdl_mixer::chunk_abuf(handler.chunk as *mut c_void) as *const i16;
    let buffer = stream as *mut i16;
    // buffer size（以 i16 元素计）
    let buffer_size = length as usize / std::mem::size_of::<i16>();
    // 取一次 speed 的“快照”
    let speed_factor = *handler.speed;

    let mixer_channels = p_this.mixer_channels;

    if handler.position < handler.duration as f32 || handler.loop_flag != 0 {
        // 每个样本的正常时长 / 按 speedFactor 拉伸后的虚拟时长
        let delta = 1000.0f32 / p_this.mixer_freq as f32;
        let vdelta = delta * speed_factor;

        // 首次需要变调（speed 不为 1）时置位
        if handler.altered == 0 && speed_factor != 1.0 {
            handler.altered = 1;
        }

        if handler.altered != 0 {
            let mut i: usize = 0;
            while i < buffer_size {
                let j = (i / mixer_channels as usize) as f32;
                let x = handler.position + j * vdelta;
                let k = (x / delta).floor() as i32;
                let prop = (x / delta) - k as f32;

                let mut c: i32 = 0;
                while c < mixer_channels {
                    // k 是否越界（C++: k * channels + channels - 1 < chunk_size || loop）
                    if k * mixer_channels + mixer_channels - 1 < handler.chunk_size
                        || handler.loop_flag != 0
                    {
                        let chunk_size = handler.chunk_size;
                        let v0 = *chunk_data
                            .offset(((k * mixer_channels + c) % chunk_size) as isize)
                            as f32;
                        let v1 = *chunk_data
                            .offset((((k + 1) * mixer_channels + c) % chunk_size) as isize)
                            as f32;
                        // C++: buffer[i + c] = v0 + prop * (v1 - v0)
                        *buffer.offset((i + c as usize) as isize) = (v0 + prop * (v1 - v0)) as i16;
                    } else {
                        *buffer.offset((i + c as usize) as isize) = 0;
                    }
                    c += 1;
                }
                i += mixer_channels as usize;
            }
        }

        // 更新位置
        handler.position += (buffer_size as f32 / mixer_channels as f32) * vdelta;

        // 循环播放时回绕
        if handler.loop_flag != 0 {
            while handler.position > handler.duration as f32 {
                handler.position -= handler.duration as f32;
            }
        }
    } else {
        // 已播完但比 SDL_mixer 预期提前结束（播放速度更快）→ 填静音
        let mut i: usize = 0;
        while i < buffer_size {
            *buffer.offset(i as isize) = 0;
            i += 1;
        }

        if handler.self_halt != 0 {
            sdl_mixer::halt_channel(mix_channel);
        }
    }
}

impl SDLSoundInstance {
    pub fn new() -> Self {
        SDLSoundInstance {
            m_mix_chunk: std::ptr::null_mut(),
            m_auto_release: false,
            m_has_played: false,
            m_released: false,
            m_reserved_channel: -1,
            m_channel: -1,

            m_base_pan: 0,
            m_base_volume: 1.0,

            m_pan: 0,
            m_volume: 1.0,
            m_pitch: 1.0,
            m_pitch_handler: Box::new(SDLSoundPitchHandler {
                self_ptr: std::ptr::null_mut(),
                chunk: std::ptr::null(),
                speed: std::ptr::null(),
                position: 0.0,
                altered: 0,
                loop_flag: 0,
                duration: 0,
                chunk_size: 0,
                self_halt: 0,
            }),

            m_default_frequency: 44100,

            m_demo_end_update_count: 0,
            m_demo_looping: false,

            // 与 sexy_app_base.rs 中 Mix_OpenAudio(44100, 0x8010, 2, 2048) 一致
            mixer_freq: 44100,
            mixer_channels: 2,
            mixer_format: 0x8010,
            master_volume: 1.0,
        }
    }

    /// 对应 C++ SDLSoundInstance(SDLSoundManager*, Mix_Chunk*, int)
    pub fn new_with_chunk(m_mix_chunk: *mut c_void, reserved_channel: i32) -> Self {
        let mut a_instance = SDLSoundInstance::new();
        a_instance.m_mix_chunk = m_mix_chunk;
        a_instance.m_reserved_channel = reserved_channel;
        a_instance.rehup_volume();
        a_instance
    }

    /// 对应 C++ FormatSampleSize
    pub fn format_sample_size(format: u16) -> u16 {
        (format & 0xFF) / 8
    }

    /// 对应 C++ GetChunkDurationMS
    pub fn get_chunk_duration_ms(&self, chunk_size: i32) -> i32 {
        // bytes / sample size == sample points
        let points = chunk_size as u32 / Self::format_sample_size(self.mixer_format) as u32;
        // sample points / channels == sample frames
        let frames = points / self.mixer_channels as u32;
        // (sample frames * 1000) / frequency == 时长（毫秒）
        ((frames * 1000) / self.mixer_freq as u32) as i32
    }

    /// 对应 C++ CreateSoundPitchHandler
    pub fn create_sound_pitch_handler(
        &mut self,
        chunk: *const c_void,
        speed: *const f32,
        loop_flag: i32,
        self_halt: i32,
    ) {
        let a_chunk_len = sdl_mixer::chunk_len(chunk as *mut c_void) as i32;
        let a_duration = self.get_chunk_duration_ms(a_chunk_len);
        let a_chunk_size = a_chunk_len / Self::format_sample_size(self.mixer_format) as i32;

        let a_self: *mut SDLSoundInstance = self;
        let a_handler = &mut *self.m_pitch_handler;
        a_handler.self_ptr = a_self;
        a_handler.chunk = chunk;
        a_handler.speed = speed;
        a_handler.position = 0.0;
        a_handler.altered = 0;
        a_handler.loop_flag = loop_flag;
        a_handler.duration = a_duration;
        a_handler.chunk_size = a_chunk_size;
        a_handler.self_halt = self_halt;
    }

    /// 对应 C++ RehupVolume
    pub fn rehup_volume(&mut self) {
        // C++: Mix_VolumeChunk(mMixChunk, (mBaseVolume * mVolume * mSoundManagerP->mMasterVolume) * 128)
        if self.m_mix_chunk.is_null() {
            return;
        }
        let a_vol = (self.m_base_volume * self.m_volume * self.master_volume * 128.0) as i32;
        sdl_mixer::volume_chunk(self.m_mix_chunk, a_vol);
    }

    /// 对应 C++ RehupPan（C++ 实现为空）
    pub fn rehup_pan(&mut self) {}

    /// 读取全局 app 的 mUpdateCount（对应 C++ gSexyAppBase->mUpdateCount）
    fn global_update_count() -> u32 {
        crate::lawn::lawn_app::LawnApp::instance()
            .map_or(0, |app| unsafe { (*app).base.m_update_count }) as u32
    }

    /// 对应 C++ gSexyAppBase->IsInDemoMode()
    fn global_is_in_demo_mode() -> bool {
        crate::lawn::lawn_app::LawnApp::instance()
            .map_or(false, |app| unsafe { (*app).base.is_in_demo_mode() })
    }
}

impl SoundInstance for SDLSoundInstance {
    fn release(&mut self) {
        // C++: Stop(); mReleased = true;
        self.stop();
        self.m_released = true;
    }

    fn set_base_volume(&mut self, the_base_volume: f64) {
        self.m_base_volume = the_base_volume;
        self.rehup_volume();
    }

    fn set_base_pan(&mut self, the_base_pan: i32) {
        self.m_base_pan = the_base_pan;
        self.rehup_pan();
    }

    fn adjust_pitch(&mut self, the_num_steps: f64) {
        // C++: mPitch = powf(1.0594630943592952645618252949463, theNumSteps);
        self.m_pitch = 1.0594630943592952645618252949463f32.powf(the_num_steps as f32);
    }

    fn set_volume(&mut self, the_volume: f64) {
        self.m_volume = the_volume;
        self.rehup_volume();
    }

    fn set_pan(&mut self, the_position: i32) {
        self.m_pan = the_position;
        self.rehup_pan();
    }

    /// C++: SDLSoundInstance::Play（SDLSoundInstance.cpp:206-241）
    fn play(&mut self, looping: bool, auto_release: bool) -> bool {
        self.stop();

        self.m_has_played = true;
        self.m_auto_release = auto_release;

        // demo 会话：播放状态由游戏 tick 推导，而非音频时钟
        if Self::global_is_in_demo_mode() {
            self.m_demo_looping = looping;
            let a_chunk_alen = sdl_mixer::chunk_len(self.m_mix_chunk);
            let a_duration_ms = self.get_chunk_duration_ms(a_chunk_alen as i32) as f64;
            self.m_demo_end_update_count = Self::global_update_count()
                + (a_duration_ms / (10.0 * self.m_pitch as f64)) as u32
                + 1;
        }

        if self.m_mix_chunk.is_null() {
            return false;
        }

        let a_target_channel =
            if self.m_reserved_channel >= 0 && self.m_reserved_channel < MAX_CHANNELS {
                self.m_reserved_channel
            } else {
                -1
            };
        self.m_channel = sdl_mixer::play_channel(
            a_target_channel,
            self.m_mix_chunk,
            if looping { -1 } else { 0 },
        );
        if self.m_channel != -1 {
            let a_channel = self.m_channel;
            let a_pitch_ptr: *const f32 = &self.m_pitch;
            self.create_sound_pitch_handler(
                self.m_mix_chunk as *const c_void,
                a_pitch_ptr,
                if looping { 1 } else { 0 },
                0,
            );
            // C++: Mix_RegisterEffect(mChannel, PitchHandlerFuncCallback, 0, &mPitchHandler);
            let a_handler_ptr: *mut SDLSoundPitchHandler = &mut *self.m_pitch_handler;
            sdl_mixer::register_effect(
                a_channel,
                pitch_handler_func_callback,
                a_handler_ptr as *mut c_void,
            );
            return true;
        }

        self.m_channel = -1;
        false
    }

    /// C++: SDLSoundInstance::Stop（SDLSoundInstance.cpp:243-256）
    fn stop(&mut self) {
        self.m_demo_end_update_count = 0;
        if self.m_channel >= 0 && self.m_channel < MAX_CHANNELS {
            // 先解绑变调效果，避免回调访问正在销毁的对象
            sdl_mixer::unregister_all_effects(self.m_channel);
            sdl_mixer::halt_channel(self.m_channel);
            self.m_channel = -1;
        }
        self.m_auto_release = false;
    }

    /// C++: SDLSoundInstance::IsPlaying（SDLSoundInstance.cpp:258-271）
    fn is_playing(&self) -> bool {
        if Self::global_is_in_demo_mode() {
            if self.m_mix_chunk.is_null() || !self.m_has_played || self.m_demo_end_update_count == 0
            {
                return false;
            }
            if self.m_demo_looping {
                return true;
            }
            return Self::global_update_count() < self.m_demo_end_update_count;
        }
        if self.m_mix_chunk.is_null()
            || !self.m_has_played
            || self.m_channel < 0
            || self.m_channel >= MAX_CHANNELS
        {
            return false;
        }
        sdl_mixer::playing(self.m_channel) != 0
    }

    /// C++: SDLSoundInstance::IsReleased（非 const，可能在查询时触发 Release）
    /// [TRANSLATION_NOTE]: C++ 该函数声明为非 const 且内部调用 Release()，故 trait 接口同样取 `&mut self`，
    /// 以避免 `&T as *mut T` 这类 UB 转换（invalid_reference_casting）。
    fn is_released(&mut self) -> bool {
        if !self.m_released && self.m_auto_release && self.m_has_played && !self.is_playing() {
            self.release();
        }
        self.m_released
    }

    fn get_volume(&self) -> f64 {
        self.m_volume
    }
}

impl Default for SDLSoundInstance {
    fn default() -> Self {
        SDLSoundInstance::new()
    }
}
