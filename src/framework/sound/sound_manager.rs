// 声音管理器 trait
pub trait SoundManager {
    fn initialized(&self) -> bool;
    fn load_sound_by_id(&mut self, sfx_id: i32, filename: &str) -> bool;
    fn load_sound(&mut self, filename: &str) -> i32;
    fn release_sound(&mut self, sfx_id: i32);
    fn set_volume(&mut self, volume: f64);
    fn set_base_volume(&mut self, sfx_id: i32, base_volume: f64) -> bool;
    fn set_base_pan(&mut self, sfx_id: i32, base_pan: i32) -> bool;
    fn play_sound(&self, sfx_id: i32) -> bool;
    fn stop_sound(&self, sfx_id: i32);
    fn stop_all_sounds(&self);
    fn set_master_volume(&mut self, volume: f64);
    fn get_master_volume(&self) -> f64;
    fn flush(&mut self);
    fn get_free_sound_id(&self) -> i32;
    fn num_sounds(&self) -> i32;

    /// 获取音效实例（对应 C++ `virtual SoundInstance* GetSoundInstance(intptr_t theSfxID)`）
    /// [TRANSLATION_NOTE]: C++ 返回 SoundInstance*；Rust 返回具体类型指针，
    /// 调用方在需要时再 unsize 为 `*mut dyn SoundInstance`（见 tod_foley 的播放路径）。
    fn get_sound_instance(
        &mut self,
        sfx_id: i32,
    ) -> Option<*mut crate::framework::sound::sdl_sound_instance::SDLSoundInstance>;

    /// 释放全部音效源（对应 C++ ReleaseSounds）
    fn release_sounds(&mut self);
    /// 释放全部播放声道（对应 C++ ReleaseChannels）
    fn release_channels(&mut self);
}
