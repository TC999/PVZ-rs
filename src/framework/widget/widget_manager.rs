// PvZ Portable Rust 翻译 — WidgetManager 控件管理器
// 对应 C++ SexyAppFramework/widget/WidgetManager.h / WidgetManager.cpp

#![allow(dead_code)]

use crate::framework::widget::widget::Widget;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::KeyCode;
use crate::framework::rect::Rect;

/// 模态控件信息
#[derive(Debug, Clone, Copy)]
pub struct PreModalInfo {
    pub prev_base_modal_widget: Option<*mut Widget>,
    pub prev_focus_widget: Option<*mut Widget>,
    pub prev_flags_mod: i32,
    pub prev_is_over: bool,
}

/// Widget 管理器
pub struct WidgetManager {
    /// 根 Widget 列表
    pub widget_list: Vec<*mut Widget>,

    // 鼠标
    pub mouse_x: i32, pub mouse_y: i32,
    pub last_mouse_x: i32, pub last_mouse_y: i32,
    pub mouse_in: bool,
    pub mouse_flags: i32,
    pub last_down_widget: Option<*mut Widget>,
    pub last_down_btn_id: i32,
    pub last_down_click_count: i32,
    pub over_widget: Option<*mut Widget>,

    // 键盘
    pub key_down: Vec<bool>,

    // 焦点
    pub focus_widget: Option<*mut Widget>,

    // 模态
    pub base_modal_widget: Option<*mut Widget>,
    pub pre_modal_info_list: Vec<PreModalInfo>,
    pub below_modal_flags_mod: i32,

    // 叠加层
    pub deferred_overlay_list: Vec<(*mut Widget, i32)>,
    pub min_deferred_overlay_priority: i32,

    // 弹窗
    pub popup_command_widget: Option<*mut Widget>,

    // 图形
    pub graphics: Graphics,

    // 应用
    pub app: Option<*mut crate::framework::sexy_app_base::SexyAppBase>,

    // 矩形
    pub widget_manager_rect: Rect,
    pub mouse_dest_rect: Rect,
    pub mouse_src_rect: Rect,

    // 更新计数
    pub update_cnt: i32,
}

impl WidgetManager {
    pub fn new() -> Self {
        WidgetManager {
            widget_list: Vec::new(),
            mouse_x: 0, mouse_y: 0,
            last_mouse_x: 0, last_mouse_y: 0,
            mouse_in: false,
            mouse_flags: 0,
            last_down_widget: None,
            last_down_btn_id: 0,
            last_down_click_count: 0,
            over_widget: None,
            key_down: vec![false; 512],
            focus_widget: None,
            base_modal_widget: None,
            pre_modal_info_list: Vec::new(),
            below_modal_flags_mod: 0,
            deferred_overlay_list: Vec::new(),
            min_deferred_overlay_priority: 0,
            popup_command_widget: None,
            graphics: Graphics::new(),
            app: None,
            widget_manager_rect: Rect::ZERO,
            mouse_dest_rect: Rect::ZERO,
            mouse_src_rect: Rect::ZERO,
            update_cnt: 0,
        }
    }

    /// 添加 Widget
    pub fn add_widget(&mut self, widget: *mut Widget) {
        unsafe {
            (*widget).widget_manager = Some(self as *mut WidgetManager);
        }
        self.widget_list.push(widget);
    }

    /// 移除 Widget
    pub fn remove_widget(&mut self, widget: *mut Widget) {
        self.widget_list.retain(|w| *w != widget);
        self.disable_widget(widget);
    }

    /// 禁用控件
    pub fn disable_widget(&mut self, w: *mut Widget) {
        if Some(w) == self.focus_widget {
            let mut new_focus: Option<*mut Widget> = None;
            for &w2 in &self.widget_list {
                unsafe {
                    if w2 != w && (*w2).wants_focus() {
                        new_focus = Some(w2); break;
                    }
                }
            }
            self.set_focus(new_focus);
        }
        if Some(w) == self.over_widget { self.over_widget = None; }
        if Some(w) == self.last_down_widget { self.last_down_widget = None; }
    }

    /// 重新计算鼠标
    pub fn rehup_mouse(&mut self) {
        if self.mouse_in { self.mouse_position(self.mouse_x, self.mouse_y); }
    }

    /// 设置焦点
    pub fn set_focus(&mut self, widget: Option<*mut Widget>) {
        if self.focus_widget != widget {
            if let Some(old) = self.focus_widget { unsafe { (*old).lost_focus(); } }
            self.focus_widget = widget;
            if let Some(new) = widget { unsafe { (*new).got_focus(); } }
        }
    }

    /// 鼠标位置更新
    pub fn mouse_position(&mut self, x: i32, y: i32) {
        let mut hit: Option<*mut Widget> = None;
        for &w in self.widget_list.iter().rev() {
            unsafe {
                if (*w).visible && !(*w).disabled && (*w).contains(x, y) {
                    hit = Some(w); break;
                }
            }
        }
        if hit != self.over_widget {
            if let Some(old) = self.over_widget { unsafe { (*old).mouse_leave(); } }
            self.over_widget = hit;
            if let Some(new) = hit { unsafe { (*new).mouse_enter(); } }
        }
        if let Some(w) = hit {
            unsafe { (*w).mouse_move(x - (*w).x, y - (*w).y); }
        }
    }

    /// 延迟叠加层
    pub fn defer_overlay(&mut self, w: *mut Widget, prio: i32) {
        self.deferred_overlay_list.push((w, prio));
    }

    /// 刷新延迟叠加层
    pub fn flush_deferred_overlay_widgets(&mut self, max_prio: i32) {
        self.deferred_overlay_list.sort_by_key(|&(_, p)| p);
        let remaining: Vec<_> = self.deferred_overlay_list.drain(..)
            .filter(|&(_, p)| p > max_prio).collect();
        for &(w, _) in &self.deferred_overlay_list {
            if unsafe { (*w).widget_manager.is_some() } {
                let mut g = Graphics::new();
                unsafe { (*w).draw_overlay(&mut g); }
            }
        }
        self.deferred_overlay_list = remaining;
    }

    /// 绘制所有 Widget
    pub fn draw(&mut self, g: &mut Graphics) {
        for &w in &self.widget_list {
            unsafe {
                if (*w).visible { (*w).draw(g); }
            }
        }
    }

    /// 更新所有 Widget
    pub fn update(&mut self) {
        self.update_cnt += 1;
        let list: Vec<*mut Widget> = self.widget_list.clone();
        for &w in &list {
            unsafe { (*w).update(); }
        }
    }

    /// 鼠标按下
    pub fn mouse_down(&mut self, x: i32, y: i32, click_count: i32) -> bool {
        for &w in self.widget_list.iter().rev() {
            unsafe {
                if (*w).visible && !(*w).disabled && (*w).contains(x, y) {
                    self.last_down_widget = Some(w);
                    self.last_down_click_count = click_count;
                    (*w).mouse_down(x - (*w).x, y - (*w).y, click_count);
                    return true;
                }
            }
        }
        false
    }

    /// 鼠标释放
    pub fn mouse_up(&mut self, x: i32, y: i32, _click_count: i32) -> bool {
        for &w in self.widget_list.iter().rev() {
            unsafe {
                if (*w).visible && (*w).contains(x, y) {
                    (*w).mouse_up(x - (*w).x, y - (*w).y);
                    return true;
                }
            }
        }
        self.last_down_widget = None;
        false
    }

    /// 鼠标移动
    pub fn mouse_move(&mut self, x: i32, y: i32) -> bool {
        self.last_mouse_x = self.mouse_x;
        self.last_mouse_y = self.mouse_y;
        self.mouse_x = x; self.mouse_y = y;
        self.mouse_position(x, y);
        true
    }

    /// 鼠标拖拽
    pub fn mouse_drag(&mut self, x: i32, y: i32) -> bool {
        if let Some(w) = self.last_down_widget {
            unsafe { (*w).mouse_drag(x - (*w).x, y - (*w).y); }
            return true;
        }
        false
    }

    /// 滚轮
    pub fn mouse_wheel(&mut self, delta: i32) {
        if let Some(w) = self.over_widget { unsafe { (*w).mouse_wheel(delta); } }
    }

    /// 按键按下
    pub fn key_down(&mut self, key: KeyCode) -> bool {
        if (key as usize) < self.key_down.len() { self.key_down[key as usize] = true; }
        if let Some(w) = self.focus_widget { unsafe { (*w).key_down(key, self); } }
        true
    }

    /// 按键释放
    pub fn key_up(&mut self, key: KeyCode) -> bool {
        if (key as usize) < self.key_down.len() { self.key_down[key as usize] = false; }
        if let Some(w) = self.focus_widget { unsafe { (*w).key_up(key); } }
        true
    }

    /// 字符输入
    pub fn key_char(&mut self, c: u8) -> bool {
        if let Some(w) = self.focus_widget { unsafe { (*w).key_char(c); } return true; }
        false
    }

    /// 调整大小
    pub fn resize(&mut self, mouse_dest: Rect, _mouse_src: Rect) {
        self.mouse_dest_rect = mouse_dest;
    }
}

impl Default for WidgetManager {
    fn default() -> Self { WidgetManager::new() }
}
