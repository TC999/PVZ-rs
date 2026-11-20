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
    // NOT-WIRED: C++ 调用点 SexyAppBase.cpp:4082 PlaySample；Rust play_sample 走 play_sound 捷径、实例路径未接线（UNUSED_CPP_USAGE.md §1）
    #[allow(dead_code)]
    fn set_pan(&mut self, pos: i32);
    /// 播放（looping=循环, autoRelease=自动释放）
    fn play(&mut self, looping: bool, auto_release: bool) -> bool;
    /// 停止
    fn stop(&mut self);
    /// 是否播放中
    fn is_playing(&self) -> bool;
    /// 是否已释放（对应 C++ `virtual bool IsReleased()`，C++ 为非 const：查询时可能触发 Release）
    fn is_released(&mut self) -> bool;
    /// 获取音量
    // NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
    #[allow(dead_code)]
    fn get_volume(&self) -> f64;
}
