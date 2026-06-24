// PvZ Portable Rust 翻译 — WidgetContainer 控件容器基类
// 对应 C++ SexyAppFramework/widget/WidgetContainer.h / WidgetContainer.cpp

#![allow(dead_code)]

use std::cmp;
use std::collections::LinkedList;

use crate::framework::point::Point;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::{Graphics, ModalFlags};

/// 组件列表类型
pub type WidgetList = LinkedList<*mut Widget>;

// 前向声明
pub struct Widget;
pub struct WidgetManager;

/// 控件容器基类（对应 C++ WidgetContainer）
/// Widget 和 WidgetManager 的基类，管理位置、子控件和脏区域标记
#[derive(Debug)]
pub struct WidgetContainer {
    // ---- 子控件列表 ----
    pub widgets: WidgetList,
    pub widget_manager: Option<*mut WidgetManager>,
    pub parent: Option<*mut WidgetContainer>,

    // ---- 迭代器状态 ----
    pub update_iterator_modified: bool,
    pub last_wm_update_count: u32,
    pub update_cnt: i32,

    // ---- 脏标记 ----
    pub dirty: bool,

    // ---- 位置尺寸 ----
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,

    // ---- 渲染属性 ----
    pub has_alpha: bool,
    pub clip: bool,
    pub priority: i32,
    pub zorder: i32,
}

impl WidgetContainer {
    pub fn new() -> Self {
        WidgetContainer {
            widgets: LinkedList::new(),
            widget_manager: None,
            parent: None,
            update_iterator_modified: false,
            last_wm_update_count: 0,
            update_cnt: 0,
            dirty: false,
            x: 0, y: 0, width: 0, height: 0,
            has_alpha: false,
            clip: true,
            priority: 0,
            zorder: 0,
        }
    }

    /// 获取矩形（对应 C++ GetRect）
    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    /// 判断是否相交（对应 C++ Intersects）
    pub fn intersects(&self, other: &WidgetContainer) -> bool {
        self.get_rect().intersects(&other.get_rect())
    }

    /// 获取绝对位置（对应 C++ GetAbsPos）
    pub fn get_abs_pos(&self) -> Point {
        match self.parent {
            Some(parent) => unsafe {
                Point::new(self.x, self.y) + (*parent).get_abs_pos()
            },
            None => Point::new(self.x, self.y),
        }
    }

    /// 按 Z 顺序插入控件（对应 C++ InsertWidgetHelper）
    pub fn insert_widget_helper(widgets: &mut WidgetList, the_widget: *mut Widget) {
        let z = unsafe { (*the_widget).get_zorder() };
        
        // 前向搜索
        let mut after = widgets.len();
        for (i, w) in widgets.iter().enumerate() {
            let wz = unsafe { (**w).get_zorder() };
            if wz >= z {
                after = i;
                break;
            }
        }

        if after < widgets.len() {
            let mut split = widgets.split_off(after);
            widgets.push_back(the_widget);
            widgets.append(&mut split);
        } else {
            widgets.push_back(the_widget);
        }
    }

    /// 添加子控件（对应 C++ AddWidget）
    pub fn add_widget(&mut self, the_widget: *mut Widget) {
        // 检查是否已存在
        for w in &self.widgets {
            if *w == the_widget {
                return;
            }
        }
        unsafe {
            (*the_widget).set_widget_manager(self.widget_manager);
            (*the_widget).set_parent(self as *mut WidgetContainer);
        }

        Self::insert_widget_helper(&mut self.widgets, the_widget);

        if let Some(wm) = self.widget_manager {
            unsafe {
                (*the_widget).added_to_manager(wm);
                (*the_widget).mark_dirty_full();
                (*wm).rehup_mouse();
            }
        }
        self.mark_dirty();
    }

    /// 移除子控件（对应 C++ RemoveWidget）
    pub fn remove_widget(&mut self, the_widget: *mut Widget) {
        if let Some(pos) = self.widgets.iter().position(|w| *w == the_widget) {
            unsafe {
                (*the_widget).widget_removed_helper();
                (*the_widget).set_parent(std::ptr::null_mut());
            }
            // 从列表中移除
            let mut rest = self.widgets.split_off(pos);
            if !rest.is_empty() {
                rest.pop_front();
            }
            self.widgets.append(&mut rest);
        }
    }

    /// 检查是否持有控件（对应 C++ HasWidget）
    pub fn has_widget(&self, the_widget: *mut Widget) -> bool {
        self.widgets.iter().any(|w| *w == the_widget)
    }

    /// 查找指定位置的控件（对应 C++ GetWidgetAtHelper）
    pub fn get_widget_at_helper(
        &self,
        x: i32,
        y: i32,
        the_flags: i32,
        found: &mut bool,
        widget_x: Option<&mut i32>,
        widget_y: Option<&mut i32>,
        wm: &WidgetManager,
    ) -> Option<*mut Widget> {
        let mut below_modal = false;

        for w in self.widgets.iter().rev() {
            let a_widget = *w;

            unsafe {
                let w_flags = the_flags; // 简化：跳过 ModFlags
                let mut child_found = false;
                let child = (*a_widget).get_widget_at_helper(
                    x - (*a_widget).get_x(),
                    y - (*a_widget).get_y(),
                    w_flags,
                    &mut child_found,
                    None, None,
                    wm,
                );

                if child.is_some() || child_found {
                    *found = true;
                    return child;
                }

                if (*a_widget).get_mouse_visible()
                    && (*a_widget).get_inset_rect().contains(x, y)
                {
                    *found = true;
                    if (*a_widget).is_point_visible(
                        x - (*a_widget).get_x(),
                        y - (*a_widget).get_y(),
                    ) {
                        if let Some(wx) = widget_x {
                            *wx = x - (*a_widget).get_x();
                        }
                        if let Some(wy) = widget_y {
                            *wy = y - (*a_widget).get_y();
                        }
                        return Some(a_widget);
                    }
                }

                below_modal |= a_widget == wm.base_modal_widget;
            }
        }

        *found = false;
        None
    }

    /// 删除所有子控件（对应 C++ RemoveAllWidgets）
    pub fn remove_all_widgets(&mut self, do_delete: bool, recursive: bool) {
        while let Some(a_widget) = self.widgets.pop_front() {
            if recursive {
                unsafe {
                    (*a_widget).remove_all_widgets(do_delete, true);
                }
            }
            if do_delete {
                // 在 Rust 中删除由调用方管理
            }
        }
    }

    /// 标记全部为脏（对应 C++ MarkAllDirty）
    pub fn mark_all_dirty(&mut self) {
        self.mark_dirty();
        for w in &self.widgets {
            unsafe {
                (**w).set_dirty(true);
                (**w).mark_all_dirty();
            }
        }
    }

    /// 置顶（对应 C++ BringToFront）
    pub fn bring_to_front(&mut self, the_widget: *mut Widget) {
        if let Some(pos) = self.widgets.iter().position(|w| *w == the_widget) {
            let mut rest = self.widgets.split_off(pos);
            if !rest.is_empty() {
                rest.pop_front();
            }
            self.widgets.append(&mut rest);
            Self::insert_widget_helper(&mut self.widgets, the_widget);
            unsafe {
                (*the_widget).order_in_manager_changed();
            }
        }
    }

    /// 置底（对应 C++ BringToBack）
    pub fn bring_to_back(&mut self, the_widget: *mut Widget) {
        if let Some(pos) = self.widgets.iter().position(|w| *w == the_widget) {
            let mut rest = self.widgets.split_off(pos);
            if !rest.is_empty() {
                rest.pop_front();
            }
            self.widgets.append(&mut rest);
            Self::insert_widget_helper(&mut self.widgets, the_widget);
            // 移到开头
            if let Some(w) = self.widgets.pop_back() {
                self.widgets.push_front(w);
            }
            unsafe {
                (*the_widget).order_in_manager_changed();
            }
        }
    }

    /// 放到后面（对应 C++ PutBehind）
    pub fn put_behind(&mut self, the_widget: *mut Widget, ref_widget: *mut Widget) {
        if let Some(pos) = self.widgets.iter().position(|w| *w == the_widget) {
            let mut rest = self.widgets.split_off(pos);
            if !rest.is_empty() {
                rest.pop_front();
            }
            self.widgets.append(&mut rest);

            if let Some(ref_pos) = self.widgets.iter().position(|w| *w == ref_widget) {
                let rest2 = self.widgets.split_off(ref_pos);
                self.widgets.push_back(the_widget);
                self.widgets.extend(rest2);
            }
            unsafe {
                (*the_widget).order_in_manager_changed();
            }
        }
    }

    /// 放到前面（对应 C++ PutInfront）
    pub fn put_infront(&mut self, the_widget: *mut Widget, ref_widget: *mut Widget) {
        if let Some(pos) = self.widgets.iter().position(|w| *w == the_widget) {
            let mut rest = self.widgets.split_off(pos);
            if !rest.is_empty() {
                rest.pop_front();
            }
            self.widgets.append(&mut rest);

            if let Some(ref_pos) = self.widgets.iter().position(|w| *w == ref_widget) {
                let mut rest2 = self.widgets.split_off(ref_pos + 1);
                self.widgets.push_back(the_widget);
                self.widgets.append(&mut rest2);
            }
            unsafe {
                (*the_widget).order_in_manager_changed();
            }
        }
    }

    /// 标记脏（对应 C++ MarkDirty）
    pub fn mark_dirty(&mut self) {
        match self.parent {
            Some(parent) => unsafe {
                (*parent).mark_dirty_widget(self as *mut WidgetContainer);
            },
            None => self.dirty = true,
        }
    }

    /// 标记全脏（对应 C++ MarkDirtyFull）
    pub fn mark_dirty_full(&mut self) {
        match self.parent {
            Some(parent) => unsafe {
                (*parent).mark_dirty_full_widget(self as *mut WidgetContainer);
            },
            None => self.dirty = true,
        }
    }

    /// 标记指定控件脏（对应 C++ MarkDirty(WidgetContainer*)）
    pub fn mark_dirty_widget(&mut self, the_widget: *mut WidgetContainer) {
        unsafe {
            if (*the_widget).dirty {
                return;
            }
        }
        self.mark_dirty();
        unsafe {
            (*the_widget).dirty = true;
        }

        if self.parent.is_some() {
            return;
        }

        unsafe {
            if (*the_widget).has_alpha {
                self.mark_dirty_full_widget(the_widget);
            } else {
                let mut found = false;
                for w in &self.widgets {
                    if *w as *mut WidgetContainer == the_widget {
                        found = true;
                    } else if found {
                        let wc = *w as *mut WidgetContainer;
                        if (**w).get_visible() && (*wc).intersects(&*the_widget) {
                            self.mark_dirty_widget(wc);
                        }
                    }
                }
            }
        }
    }

    /// 标记指定控件全脏（对应 C++ MarkDirtyFull(WidgetContainer*)）
    pub fn mark_dirty_full_widget(&mut self, the_widget: *mut WidgetContainer) {
        self.mark_dirty_full();
        unsafe {
            (*the_widget).dirty = true;
        }
        if self.parent.is_some() {
            return;
        }

        unsafe {
            let tw = &*the_widget;
            // 检查在 the_widget 之下的控件
            let mut before: Vec<*mut Widget> = Vec::new();
            let mut after: Vec<*mut Widget> = Vec::new();
            let mut found = false;
            for w in &self.widgets {
                if *w as *mut WidgetContainer == the_widget {
                    found = true;
                } else if !found {
                    before.push(*w);
                } else {
                    after.push(*w);
                }
            }

            for w in before.iter().rev() {
                let wc = **w as *mut WidgetContainer;
                if (**w).get_visible() && (*wc).intersects(tw) {
                    // 如果下面的控件不透明且完全覆盖，则停止
                    if !(**w).get_has_transparencies() && !(*wc).has_alpha {
                        let a_rect = Rect::new(
                            tw.x, tw.y, tw.width, tw.height,
                        ).intersection(&Rect::new(0, 0, self.width, self.height));

                        if (**w).contains(a_rect.x, a_rect.y)
                            && (**w).contains(
                                a_rect.x + a_rect.width - 1,
                                a_rect.y + a_rect.height - 1,
                            )
                        {
                            (**w).mark_dirty();
                            break;
                        }
                    }
                    if (*wc).intersects(tw) {
                        self.mark_dirty_widget(wc);
                    }
                }
            }

            for w in &after {
                let wc = **w as *mut WidgetContainer;
                if (**w).get_visible() && (*wc).intersects(tw) {
                    self.mark_dirty_widget(wc);
                }
            }
        }
    }

    /// 加入管理器（对应 C++ AddedToManager）
    pub fn added_to_manager(&mut self, the_wm: *mut WidgetManager) {
        for w in &self.widgets {
            unsafe {
                (**w).set_widget_manager(Some(the_wm));
                (**w).added_to_manager(the_wm);
                self.mark_dirty();
            }
        }
    }

    /// 从管理器移除（对应 C++ RemovedFromManager）
    pub fn removed_from_manager(&mut self, the_wm: *mut WidgetManager) {
        for w in &self.widgets {
            unsafe {
                (**w).removed_from_manager(the_wm);
                (**w).set_widget_manager(None);
            }
        }
    }

    /// 更新（对应 C++ Update）
    pub fn update(&mut self) {
        self.update_cnt += 1;
    }

    /// 更新全部（对应 C++ UpdateAll）
    pub fn update_all(&mut self, the_flags: &mut ModalFlags, wm: &mut WidgetManager) {
        if the_flags.flags & 1 != 0 {
            self.mark_dirty();
        }

        if the_flags.flags & 2 != 0 {
            if self.last_wm_update_count != wm.update_cnt as u32 {
                self.last_wm_update_count = wm.update_cnt as u32;
                self.update();
            }
        }

        // 遍历子控件
        let mut temp_list: Vec<*mut Widget> = self.widgets.iter().copied().collect();
        for &a_widget in &temp_list {
            unsafe {
                if a_widget == wm.base_modal_widget {
                    the_flags.is_over = true;
                }
                (*a_widget).update_all(the_flags, wm);
            }
        }
    }

    /// 绘制（对应 C++ Draw）
    pub fn draw(&self, _g: &mut Graphics) {}

    /// 绘制全部（对应 C++ DrawAll）
    pub fn draw_all(&self, the_flags: &mut ModalFlags, g: &mut Graphics) {
        if self.clip && (the_flags.flags & 4 /* CLIP */) != 0 {
            g.clip_rect_xywh(0, 0, self.width, self.height);
        }

        if self.widgets.is_empty() {
            if the_flags.flags & 8 /* DRAW */ != 0 {
                self.draw(g);
            }
            return;
        }

        if the_flags.flags & 8 != 0 {
            g.push_state();
            self.draw(g);
            g.pop_state();
        }

        let temp_list: Vec<*mut Widget> = self.widgets.iter().copied().collect();
        for &a_widget in &temp_list {
            unsafe {
                if (**a_widget).get_visible() {
                    if a_widget as *mut WidgetContainer == wm.base_modal_widget {
                        the_flags.is_over = true;
                    }

                    let mut clip_g = Graphics::new();
                    clip_g.copy_translate_from(g, (**a_widget).get_x(), (**a_widget).get_y());
                    (**a_widget).draw_all(the_flags, &mut clip_g);
                    (**a_widget).set_dirty(false);
                }
            }
        }
    }

    /// 系统颜色改变（对应 C++ SysColorChanged / SysColorChangedAll）
    pub fn sys_color_changed(&mut self) {}
    pub fn sys_color_changed_all(&mut self) {
        self.sys_color_changed();
        let temp_list: Vec<*mut Widget> = self.widgets.iter().copied().collect();
        for &w in &temp_list {
            unsafe {
                (**w).sys_color_changed_all();
            }
        }
    }

    /// 禁用控件（对应 C++ DisableWidget）
    pub fn disable_widget(&mut self, _the_widget: *mut Widget) {}

    /// 设置焦点（对应 C++ SetFocus）
    pub fn set_focus(&mut self, _the_widget: *mut Widget) {}

    /// 获取最终矩形
    pub fn final_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}

impl Default for WidgetContainer {
    fn default() -> Self {
        WidgetContainer::new()
    }
}
