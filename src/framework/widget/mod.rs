// PvZ Portable Rust 翻译 — Widget 模块

pub mod button_listener;
pub mod button_widget;
pub mod checkbox;
pub mod dialog;
pub mod dialog_button;
pub mod dialog_listener;
pub mod edit_widget;
pub mod hyperlink_widget;
pub mod insets;
pub mod list_widget;
pub mod scrollbar_widget;
pub mod scrollbutton_widget;
pub mod slider;
pub mod text_widget;
pub mod widget;
pub mod widget_container;
pub mod widget_manager;

// ── 通用 Widget 辅助函数（对应 C++ Widget 相关函数） ──
//
// [TRANSLATION_NOTE]: 以下符号在 C++ 中均为**类成员方法**而非自由函数：
//   SetSelect          → ListWidget::SetSelect（ListWidget.cpp:493）
//   SetInvisIfNoScroll → ScrollbarWidget::SetInvisIfNoScroll（ScrollbarWidget.cpp:64）
//   GetLastColor       → TextWidget::GetLastColor（TextWidget.cpp:194）
//   GetTextIndexAt     → TextWidget::GetTextIndexAt（TextWidget.cpp:366）
//   GetSelection       → TextWidget::GetSelection（TextWidget.cpp:407）
//   RemoveAllWidgets   → WidgetContainer::RemoveAllWidgets（WidgetContainer.cpp:60）
//   GetWidgetFlags     → WidgetManager::GetWidgetFlags（WidgetManager.cpp:105）
//   GetSortKey         → ListWidget::GetSortKey（ListWidget.cpp:74）
//   FreeResources      → Widget::FreeResources（Widget.cpp）
// 本文件把它们写成了自由函数占位，宿主类型的对应方法尚未翻译，故无调用点。
// 保留占位以维持符号对应（删除会丢翻译线索），待宿主类型方法翻译后迁入并接线。

/// 设置选中状态（对应 C++ ListWidget::SetSelect）
// NOT-WIRED: C++ 调用点 UserDialog.cpp:55；宿主 ListWidget::SetSelect 方法待翻译（UNUSED_CPP_USAGE.md §1）
#[allow(dead_code)]
pub fn set_select(_sel: i32) {}
/// 设置无滚动时不可见（对应 C++ ScrollbarWidget::SetInvisIfNoScroll）
// NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
#[allow(dead_code)]
pub fn set_invis_if_no_scroll(_invis: bool) {}
/// 获取最后颜色（对应 C++ TextWidget::GetLastColor）
// NOT-WIRED: C++ 调用点 TextWidget.cpp:230；宿主 TextWidget::GetLastColor 方法待翻译（UNUSED_CPP_USAGE.md §1）
#[allow(dead_code)]
pub fn get_last_color(_key: &str) {}
/// 获取指定位置的文本索引（对应 C++ TextWidget::GetTextIndexAt）
// NOT-WIRED: C++ 调用点 TextWidget.cpp:393；宿主 TextWidget::GetTextIndexAt 方法待翻译（UNUSED_CPP_USAGE.md §1）
#[allow(dead_code)]
pub fn get_text_index_at(_x: i32, _y: i32, _index: *mut i32) {}
/// 获取选中范围（对应 C++ TextWidget::GetSelection）
// NOT-WIRED: C++ 亦仅定义/声明无调用点（UNUSED_CPP_USAGE.md §2），保留以维持逐函数对应
#[allow(dead_code)]
pub fn get_selection() {}
/// 移除所有控件（对应 C++ WidgetContainer::RemoveAllWidgets）
// NOT-WIRED: C++ 调用点 GameSelector.cpp:372 / ScrollbarWidget.cpp:61；宿主类型方法待翻译（UNUSED_CPP_USAGE.md §1）
#[allow(dead_code)]
pub fn remove_all_widgets(_free_widgets: bool, _delete_widgets: bool) {}
/// 释放资源（对应 C++ Widget::FreeResources）
// NOT-WIRED: C++ 调用点 WidgetManager.cpp:69（析构）；Rust 以 Drop 释放、未接线（UNUSED_CPP_USAGE.md §1）
#[allow(dead_code)]
pub fn free_resources() {}
/// 获取控件标志（对应 C++ WidgetManager::GetWidgetFlags）
// NOT-WIRED: C++ 调用点 WidgetManager.cpp:113；宿主 WidgetManager::GetAnyWidgetAt 未接线（UNUSED_CPP_USAGE.md §1）
#[allow(dead_code)]
pub fn get_widget_flags() -> i32 {
    0
}
/// 获取排序键（对应 C++ ListWidget::GetSortKey）
// NOT-WIRED: C++ 调用点 ListWidget.cpp:82（递归）；宿主 ListWidget::GetSortKey 方法待翻译（UNUSED_CPP_USAGE.md §1）
#[allow(dead_code)]
pub fn get_sort_key(_index: i32) -> i32 {
    0
}
