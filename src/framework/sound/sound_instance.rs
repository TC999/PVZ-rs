// PvZ Portable Rust 翻译 — SoundInstance 声音实例接口
// 对应 C++ SexyAppFramework/sound/SoundInstance.h

/// 声音实例 trait（对应 C++ SoundInstance 纯虚接口）
pub trait SoundInstance {
    /// 释放资源
    fn release(&mut self);
    /// 设置基础音量
    fn set_base_volume(&mut self, vol: f64);
    /// 设置基础声像
    fn set_base_pan(&mut self, pan: i32);
    /// 调整音高（半音数）
    fn adjust_pitch(&mut self, num_steps: f64);
    /// 设置音量
    fn set_volume(&mut self, vol: f64);
    /// 设置声像
    fn set_pan(&mut self, pos: i32);
    /// 播放（looping=循环, autoRelease=自动释放）
    fn play(&mut self, looping: bool, auto_release: bool) -> bool;
    /// 停止
    fn stop(&mut self);
    /// 是否播放中
    fn is_playing(&self) -> bool;
    /// 是否已释放
    fn is_released(&self) -> bool;
    /// 获取音量
    fn get_volume(&self) -> f64;
}
