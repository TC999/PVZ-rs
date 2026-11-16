// PvZ Portable Rust 翻译 — ListWidget 列表控件
// 对应 C++ SexyAppFramework/widget/ListWidget.h / ListWidget.cpp（527 行）
//
// [TRANSLATION_NOTE]: 本文件是**未接线的桩组件**（报告 §7.6 称其"极简"）：
//   - 全库无使用者（`rg "list_widget::ListWidget"` 无命中）；
//   - C++ 的 ListWidget 有 17+ 个方法（AddLine / Sort / RemoveLine / RemoveAll /
//     GetLineCount / GetLineIdx / SetColor / RemovedFromManager …），此处仅 new/
//     set_line_color/set_hilite，缺排序、增删行、事件回调等；
//   - **实际在用的列表组件是 `lawn/widget/user_dialog.rs:51` 的本地 ListWidget**
//     （自带 layout，用于 UserDialog 的存档槽列表）。
// 因此补齐本文件对运行行为没有影响；若将来要统一到 framework 层，需同时把调用方迁过来。

#![allow(dead_code)]

use crate::framework::color::Color;

pub struct ListWidget {
    pub items: Vec<String>,
    pub hilite_idx: i32,
    pub line_colors: Vec<Color>,
    pub list_listener: Option<Box<dyn ListListener>>,
    pub id: i32,
}

/// 列表控件监听器（对应 C++ ListListener）
pub trait ListListener {
    fn list_hilite_changed(&mut self, id: i32, old_idx: i32, new_idx: i32);
}

impl ListWidget {
    pub fn new() -> Self {
        ListWidget {
            items: Vec::new(),
            hilite_idx: -1,
            line_colors: Vec::new(),
            list_listener: None,
            id: 0,
        }
    }

    /// 设置行颜色（对应 C++ SetLineColor）
    pub fn set_line_color(&mut self, idx: i32, color: Color) {
        if idx >= 0 && (idx as usize) < self.items.len() {
            // 确保 line_colors 足够大
            while (self.line_colors.len() as i32) <= idx {
                self.line_colors.push(Color::WHITE);
            }
            self.line_colors[idx as usize] = color;
        }
    }

    /// 设置高亮行（对应 C++ SetHilite）
    pub fn set_hilite(&mut self, hilite_idx: i32, notify_listener: bool) {
        let old_idx = self.hilite_idx;
        self.hilite_idx = hilite_idx;
        if old_idx != hilite_idx && notify_listener {
            if let Some(ref mut listener) = self.list_listener {
                listener.list_hilite_changed(self.id, old_idx, hilite_idx);
            }
        }
    }
}

impl Default for ListWidget {
    fn default() -> Self {
        Self::new()
    }
}
