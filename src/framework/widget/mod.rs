// PvZ Portable Rust 翻译 — Widget 模块

pub mod widget;
pub mod widget_container;
pub mod widget_manager;
pub mod dialog;
pub mod button_widget;
pub mod button_listener;
pub mod dialog_listener;
pub mod edit_widget;
pub mod list_widget;
pub mod checkbox;
pub mod slider;
pub mod scrollbar_widget;
pub mod scrollbutton_widget;
pub mod text_widget;
pub mod hyperlink_widget;
pub mod insets;
pub mod dialog_button;

// ── 通用 Widget 辅助函数（对应 C++ Widget 相关函数） ──

/// 设置选中状态（对应 C++ SetSelect）
pub fn set_select() {}
/// 设置无滚动时不可见（对应 C++ SetInvisIfNoScroll）
pub fn set_invis_if_no_scroll() {}
/// 获取最后颜色（对应 C++ GetLastColor）
pub fn get_last_color() {}
/// 获取指定位置的文本索引（对应 C++ GetTextIndexAt）
pub fn get_text_index_at() -> i32 { 0 }
/// 获取选中范围（对应 C++ GetSelection）
pub fn get_selection() {}
/// 移除所有控件（对应 C++ RemoveAllWidgets）
pub fn remove_all_widgets() {}
/// 释放资源（对应 C++ FreeResources）
pub fn free_resources() {}
/// 获取控件标志（对应 C++ GetWidgetFlags）
pub fn get_widget_flags() -> i32 { 0 }
/// 获取排序键（对应 C++ GetSortKey）
pub fn get_sort_key() -> i32 { 0 }
