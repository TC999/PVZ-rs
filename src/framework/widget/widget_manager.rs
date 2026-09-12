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
    /// 对应 C++ mDownButtons（当前按下的按钮掩码）
    pub m_down_buttons: i32,
    /// 对应 C++ mActualDownButtons（实际按下的按钮掩码，含禁用 widget 也记录）
    pub m_actual_down_buttons: i32,
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
            m_down_buttons: 0,
            m_actual_down_buttons: 0,
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

    /// 添加 Widget（对应 C++ AddWidget：设置所属 + 回调 AddedToManager）
    pub fn add_widget(&mut self, widget: *mut Widget) {
        unsafe {
            (*widget).widget_manager = Some(self as *mut WidgetManager);
            // 对应 C++ AddWidget: aWidget->AddedToManager(this)
            (*widget).added_to_manager(self as *mut WidgetManager);
        }
        self.widget_list.push(widget);
    }

    /// 移除 Widget
    pub fn remove_widget(&mut self, widget: *mut Widget) {        self.widget_list.retain(|w| *w != widget);
        self.disable_widget(widget);
        // 对应 C++ WidgetManager::RemoveWidget（WidgetManager.cpp:98-103）：
        // 移除的 widget 若是 base modal / over / focus 则清空对应状态
        if self.base_modal_widget == Some(widget) {
            self.base_modal_widget = None;
        }
        if self.over_widget == Some(widget) {
            self.over_widget = None;
        }
        if self.last_down_widget == Some(widget) {
            self.last_down_widget = None;
        }
    }

    /// 前置 Widget（对应 C++ WidgetContainer::BringToFront：从列表移除并插入末尾）
    pub fn bring_to_front(&mut self, widget: *mut Widget) {
        if let Some(idx) = self.widget_list.iter().position(|w| *w == widget) {
            self.widget_list.remove(idx);
            self.widget_list.push(widget);
            unsafe {
                (*widget).order_in_manager_changed();
            }
        }
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
                    // 对应 C++: 模态控件之下的 widget 不参与 over 判定
                    if self.is_modal_blocked(w) {
                        continue;
                    }
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

    /// 鼠标按下（对应 C++ WidgetManager::MouseDown，WidgetManager.cpp:640）
    pub fn mouse_down(&mut self, x: i32, y: i32, click_count: i32) -> bool {
        // 对应 C++: 掩码与 mLastDownButtonId 设置
        if click_count < 0 {
            self.m_actual_down_buttons |= 0x02;
            self.last_down_btn_id = -1;
            self.m_down_buttons |= 0x02;
        } else if click_count == 3 {
            self.m_actual_down_buttons |= 0x04;
            self.last_down_btn_id = 2;
            self.m_down_buttons |= 0x04;
        } else {
            self.m_actual_down_buttons |= 0x01;
            self.last_down_btn_id = 1;
            self.m_down_buttons |= 0x01;
        }

        self.mouse_position(x, y);

        // 对应 C++: GetWidgetAt；已有 last_down_widget 时全部按钮按发给它
        let mut a_widget = {
            let mut hit: Option<*mut Widget> = None;
            for &w in self.widget_list.iter().rev() {
                unsafe {
                    if (*w).visible && !(*w).disabled && (*w).contains(x, y) {
                        // 对应 C++: 模态控件之下的 widget 不接收按下
                        if self.is_modal_blocked(w) {
                            continue;
                        }
                        hit = Some(w);
                        break;
                    }
                }
            }
            hit
        };
        if self.last_down_widget.is_some() {
            a_widget = self.last_down_widget;
        }

        self.last_down_widget = a_widget;
        self.last_down_click_count = click_count;
        if let Some(w) = a_widget {
            unsafe {
                // 对应 C++: if (aWidget->WantsFocus()) SetFocus(aWidget);
                if (*w).wants_focus() {
                    self.set_focus(Some(w));
                }
                // 对应 C++: aWidget->mIsDown = true;
                (*w).is_down = true;
                (*w).mouse_down(x - (*w).x, y - (*w).y, click_count);
            }
        }
        true
    }

    /// 鼠标释放（对应 C++ WidgetManager::MouseUp，:581）
    pub fn mouse_up(&mut self, x: i32, y: i32, click_count: i32) -> bool {
        // 对应 C++: 掩码
        let a_mask = if click_count < 0 { 0x02 } else if click_count == 3 { 0x04 } else { 0x01 };

        // 对应 C++: mActualDownButtons &= ~aMask
        self.m_actual_down_buttons &= !a_mask;

        // 对应 C++: 仅当按下的按钮是发给 last_down_widget 时才派发释放
        let a_last_down_widget = self.last_down_widget;
        if let Some(w) = a_last_down_widget {
            if self.m_down_buttons & a_mask != 0 {
                self.m_down_buttons &= !a_mask;
                if self.m_down_buttons == 0 {
                    self.last_down_widget = None;
                }
                unsafe {
                    // 对应 C++: aLastDownWidget->mIsDown = false;
                    (*w).is_down = false;
                    // 对应 C++: aLastDownWidget->MouseUp(x-mx, y-my, theClickCount)
                    //（Rust mouse_up_ext 对应 C++ 三参 MouseUp：空 mouse_up + 按 id 派发 mouse_up_btn）
                    (*w).mouse_up_ext(x - (*w).x, y - (*w).y, self.last_down_btn_id);
                }
            } else {
                self.m_down_buttons &= !a_mask;
            }
        }

        self.mouse_position(x, y);
        true
    }

    /// 鼠标移动（对应 C++ WidgetManager::MouseMove，WidgetManager.cpp:665）
    /// 按钮按下时移动转发给 MouseDrag（Rust 拖动链入口）
    pub fn mouse_move(&mut self, x: i32, y: i32) -> bool {
        if self.m_down_buttons != 0 {
            return self.mouse_drag(x, y);
        }
        self.mouse_in = true;
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

    /// 滚轮（对应 C++ WidgetManager::MouseWheel：发 mFocusWidget 而非 mOverWidget）
    pub fn mouse_wheel(&mut self, delta: i32) {
        if let Some(w) = self.focus_widget { unsafe { (*w).mouse_wheel(delta); } }
    }

    /// widget1 是否绘制在 widget2 之下（对应 C++ WidgetContainer::IsBelow）
    /// [TRANSLATION_NOTE]: C++ 为嵌套容器递归（IsBelowHelper）；Rust 扁平 widget_list 以索引顺序近似
    pub fn is_below(&self, widget1: *mut Widget, widget2: *mut Widget) -> bool {
        if widget1 == widget2 {
            return false;
        }
        match (self.widget_list.iter().position(|&w| w == widget1),
               self.widget_list.iter().position(|&w| w == widget2)) {
            (Some(a), Some(b)) => a < b,
            _ => false,
        }
    }

    /// 设置基础模态控件（对应 C++ WidgetManager::SetBaseModal，:239）
    pub fn set_base_modal(&mut self, widget: Option<*mut Widget>, below_flags_mod: i32) {
        self.base_modal_widget = widget;
        self.below_modal_flags_mod = below_flags_mod;
        if let Some(base) = widget {
            // 对应 C++: 位于 base 之下的 over/last_down/focus 状态清除
            //（C++ 按 mBelowModalFlagsMod 的 ALLOW_MOUSE/ALLOW_FOCUS 移除，语义对等）
            if self.over_widget.map_or(false, |w| self.is_below(w, base)) {
                self.over_widget = None;
            }
            if self.last_down_widget.map_or(false, |w| self.is_below(w, base)) {
                self.last_down_widget = None;
            }
            if self.focus_widget.map_or(false, |w| self.is_below(w, base)) {
                self.focus_widget = None;
            }
        }
    }

    /// 添加基础模态控件并压入恢复信息（对应 C++ WidgetManager::AddBaseModal，:271）
    pub fn add_base_modal(&mut self, widget: Option<*mut Widget>, below_flags_mod: i32) {
        self.pre_modal_info_list.push(PreModalInfo {
            prev_base_modal_widget: self.base_modal_widget,
            prev_focus_widget: self.focus_widget,
            prev_flags_mod: self.below_modal_flags_mod,
            prev_is_over: self.base_modal_widget.is_none(),
        });
        self.set_base_modal(widget, below_flags_mod);
    }

    /// 添加基础模态控件（默认旗标；对应 C++ AddBaseModal(Widget*)，移除 ALLOW_MOUSE|ALLOW_FOCUS）
    pub fn add_base_modal_default(&mut self, widget: Option<*mut Widget>) {
        let a_default_below_flags_mod = 16 | 32; // WIDGETFLAGS_ALLOW_MOUSE | WIDGETFLAGS_ALLOW_FOCUS
        self.add_base_modal(widget, a_default_below_flags_mod);
    }

    /// 移除基础模态控件并恢复上一个（对应 C++ RemoveBaseModal）
    pub fn remove_base_modal(&mut self) {
        if let Some(info) = self.pre_modal_info_list.pop() {
            self.set_base_modal(info.prev_base_modal_widget, info.prev_flags_mod);
            if self.focus_widget.is_none() {
                self.focus_widget = info.prev_focus_widget;
            }
        } else {
            self.base_modal_widget = None;
        }
    }

    /// 命中检测是否被模态屏蔽（widget 在 base modal 之下且 base 存在）
    pub fn is_modal_blocked(&self, widget: *mut Widget) -> bool {
        match self.base_modal_widget {
            Some(base) => self.is_below(widget, base),
            None => false,
        }
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
