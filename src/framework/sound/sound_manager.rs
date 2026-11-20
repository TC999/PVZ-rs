// 声音管理器 trait
pub trait SoundManager {
    /// 对应 C++ `SoundManager.h:44 virtual bool Initialized() = 0`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn initialized(&self) -> bool;

    /// 对应 C++ `SoundManager.h:46 LoadSound(intptr_t, const std::string&)`
    // NOT-WIRED: C++ 调用点 ResourceManager.cpp:806；Rust 资源加载走 load_sound 单参简化路径（resource_manager.rs，不在本任务文件范围）（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn load_sound_by_id(&mut self, sfx_id: i32, filename: &str) -> bool;

    fn load_sound(&mut self, filename: &str) -> i32;

    /// 对应 C++ `SoundManager.h:48 ReleaseSound(intptr_t)`
    // NOT-WIRED: C++ 调用点 ResourceManager.cpp:47 SoundRes::DeleteResource；Rust DeleteResource 未接线（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn release_sound(&mut self, sfx_id: i32);

    /// 对应 C++ `SoundManager.h:50 SetVolume(double)`
    // NOT-WIRED: C++ 调用点 SexyAppBase.cpp:4139 SetSfxVolume；Rust set_sfx_volume 仅存字段、未接线（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn set_volume(&mut self, volume: f64);

    /// 对应 C++ `SoundManager.h:51 SetBaseVolume(intptr_t, double)`
    // NOT-WIRED: C++ 调用点 ResourceManager.cpp:811；Rust DoLoadSound 未接线（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn set_base_volume(&mut self, sfx_id: i32, base_volume: f64) -> bool;

    /// 对应 C++ `SoundManager.h:52 SetBasePan(intptr_t, int)`
    // NOT-WIRED: C++ 调用点 ResourceManager.cpp:814；Rust DoLoadSound 未接线（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn set_base_pan(&mut self, sfx_id: i32, base_pan: i32) -> bool;

    fn play_sound(&self, sfx_id: i32) -> bool;

    /// C++ SoundManager 接口无 StopSound（仅 SoundInstance::Stop）；Rust 自创
    // NOT-WIRED: C++ 无对应方法（UNUSED_CPP_USAGE.md §4 同名噪音：CrazyDaveStopSound 非本接口）
    #[allow(dead_code)]
    fn stop_sound(&self, sfx_id: i32);

    /// 对应 C++ `SoundManager.h:63 StopAllSounds()`
    // NOT-WIRED: C++ 调用点 SexyAppBase.cpp:2659（demoMute 分支）；Rust Process 未翻译该分支（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn stop_all_sounds(&self);

    /// 对应 C++ `SoundManager.h:60 SetMasterVolume(double)`
    // NOT-WIRED: C++ 调用点 SexyAppBase.cpp:4150；Rust SexyAppBase 无对应函数（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn set_master_volume(&mut self, volume: f64);

    /// 对应 C++ `SoundManager.h:59 GetMasterVolume()`
    // NOT-WIRED: C++ 调用点 SexyAppBase.cpp:4144；Rust SexyAppBase 无对应函数（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn get_master_volume(&self) -> f64;

    /// 对应 C++ `SoundManager.h:62 Flush()`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn flush(&mut self);

    /// 对应 C++ `SoundManager.h:64 GetFreeSoundId()`
    // NOT-WIRED: C++ 调用点 ResourceManager.cpp:802；Rust DoLoadSound 未接线（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn get_free_sound_id(&self) -> i32;

    /// 对应 C++ `SoundManager.h:65 GetNumSounds()`
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn num_sounds(&self) -> i32;

    /// 获取音效实例（对应 C++ `virtual SoundInstance* GetSoundInstance(intptr_t theSfxID)`）
    /// [TRANSLATION_NOTE]: C++ 返回 SoundInstance*；Rust 返回具体类型指针，
    /// 调用方在需要时再 unsize 为 `*mut dyn SoundInstance`（见 tod_foley 的播放路径）。
    fn get_sound_instance(
        &mut self,
        sfx_id: i32,
    ) -> Option<*mut crate::framework::sound::sdl_sound_instance::SDLSoundInstance>;

    /// 释放全部音效源（对应 C++ ReleaseSounds）
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn release_sounds(&mut self);
    /// 释放全部播放声道（对应 C++ ReleaseChannels）
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn release_channels(&mut self);
}
