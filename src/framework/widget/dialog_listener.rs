// PvZ Portable Rust 翻译 — DialogListener 对话框监听器
// 对应 C++ SexyAppFramework/widget/DialogListener.h

/// 对话框监听器 trait（对应 C++ DialogListener 虚接口）
pub trait DialogListener {
    /// 对话框按钮按下
    fn dialog_button_press(&mut self, dialog_id: i32, button_id: i32);
    /// 对话框按钮释放
    fn dialog_button_depress(&mut self, dialog_id: i32, button_id: i32);
}
