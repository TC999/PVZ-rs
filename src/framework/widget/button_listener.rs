// PvZ Portable Rust 翻译 — ButtonListener 按钮监听器
// 对应 C++ SexyAppFramework/widget/ButtonListener.h
//
// 按钮事件回调的 trait 接口。

/// 按钮监听器 trait（对应 C++ ButtonListener 虚基类）
pub trait ButtonListener {
    /// 按钮按下
    fn button_press(&mut self, the_id: i32);
    /// 按钮释放
    fn button_depress(&mut self, the_id: i32);
    /// 按钮按下期间逐帧回调
    fn button_down_tick(&mut self, the_id: i32);
    /// 鼠标进入按钮
    fn button_mouse_enter(&mut self, the_id: i32);
    /// 鼠标离开按钮
    fn button_mouse_leave(&mut self, the_id: i32);
    /// 鼠标在按钮上移动
    fn button_mouse_move(&mut self, the_id: i32, the_x: i32, the_y: i32);
}
