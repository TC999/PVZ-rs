// PvZ Portable Rust 翻译 — WidgetContainer 控件容器基类
// 对应 C++ SexyAppFramework/widget/WidgetContainer.h / WidgetContainer.cpp
//
// 注意：在 C++ 中 Widget 继承自 WidgetContainer，但 Rust 使用组合方式。
// WidgetContainer 只管理基本的位置/尺寸和子容器列表，不直接引用 Widget 类型。

#![allow(dead_code)]

use std::collections::LinkedList;

use crate::framework::graphics::graphics::Graphics;
use crate::framework::point::Point;
use crate::framework::rect::Rect;

/// 通用容器指针（用于子容器列表）
pub type WidgetContainerPtr = *mut WidgetContainer;

/// 组件列表类型
pub type ContainerList = LinkedList<WidgetContainerPtr>;

/// 控件容器基类（对应 C++ WidgetContainer）
/// 管理子控件列表、位置尺寸和脏区域标记
#[derive(Debug)]
pub struct WidgetContainer {
    /// 子容器列表（由 WidgetManager 遍历时转换为具体 Widget 类型）
    pub widgets: ContainerList,
    /// 父容器指针
    pub parent: Option<WidgetContainerPtr>,
    /// 更新计数
    pub update_cnt: i32,

    /// 脏标记
    pub dirty: bool,

    // 位置尺寸
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,

    // 渲染属性
    pub has_alpha: bool,
    pub clip: bool,
    pub priority: i32,
    pub zorder: i32,
}

impl WidgetContainer {
    pub fn new() -> Self {
        WidgetContainer {
            widgets: LinkedList::new(),
            parent: None,
            update_cnt: 0,
            dirty: false,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            has_alpha: false,
            clip: true,
            priority: 0,
            zorder: 0,
        }
    }

    /// 获取矩形
    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    /// 判断是否相交
    pub fn intersects(&self, other: &WidgetContainer) -> bool {
        self.get_rect().intersects(&other.get_rect())
    }

    /// 获取绝对位置
    pub fn get_abs_pos(&self) -> Point {
        match self.parent {
            Some(parent) => unsafe { Point::new(self.x, self.y) + (*parent).get_abs_pos() },
            None => Point::new(self.x, self.y),
        }
    }

    /// 按 Z 顺序插入子容器
    pub fn insert_widget_helper(widgets: &mut ContainerList, new_widget: WidgetContainerPtr) {
        let z = unsafe { (*new_widget).zorder };
        let mut pos = 0usize;
        for (i, w) in widgets.iter().enumerate() {
            if unsafe { (**w).zorder } >= z {
                pos = i;
                break;
            }
            pos = i + 1;
        }
        if pos < widgets.len() {
            let mut rest = widgets.split_off(pos);
            widgets.push_back(new_widget);
            widgets.append(&mut rest);
        } else {
            widgets.push_back(new_widget);
        }
    }

    /// 添加子容器
    pub fn add_child(&mut self, child: WidgetContainerPtr) {
        unsafe {
            (*child).parent = Some(self as WidgetContainerPtr);
        }
        Self::insert_widget_helper(&mut self.widgets, child);
    }

    /// 移除子容器
    pub fn remove_child(&mut self, child: WidgetContainerPtr) {
        // LinkedList::retain 尚不稳定，使用 split_off 方式
        if let Some(pos) = self.widgets.iter().position(|w| *w == child) {
            let mut rest = self.widgets.split_off(pos);
            if !rest.is_empty() {
                rest.pop_front();
            }
            self.widgets.append(&mut rest);
        }
        unsafe {
            (*child).parent = None;
        }
    }

    /// 标记脏
    pub fn mark_dirty(&mut self) {
        match self.parent {
            Some(parent) => unsafe {
                (*parent).mark_dirty();
            },
            None => self.dirty = true,
        }
    }

    /// 标记全脏
    pub fn mark_dirty_full(&mut self) {
        match self.parent {
            Some(parent) => unsafe {
                (*parent).mark_dirty_full();
            },
            None => self.dirty = true,
        }
    }

    /// 标记全部为脏
    pub fn mark_all_dirty(&mut self) {
        self.dirty = true;
        for w in &self.widgets {
            unsafe {
                (**w).mark_all_dirty();
            }
        }
    }

    /// 置顶
    pub fn bring_to_front(&mut self, the_widget: WidgetContainerPtr) {
        if let Some(pos) = self.widgets.iter().position(|w| *w == the_widget) {
            let mut rest = self.widgets.split_off(pos);
            if !rest.is_empty() {
                rest.pop_front();
            }
            self.widgets.append(&mut rest);
            Self::insert_widget_helper(&mut self.widgets, the_widget);
        }
    }

    /// 置底
    pub fn bring_to_back(&mut self, the_widget: WidgetContainerPtr) {
        if let Some(pos) = self.widgets.iter().position(|w| *w == the_widget) {
            let mut rest = self.widgets.split_off(pos);
            if !rest.is_empty() {
                rest.pop_front();
            }
            self.widgets.append(&mut rest);
            Self::insert_widget_helper(&mut self.widgets, the_widget);
            if let Some(w) = self.widgets.pop_back() {
                self.widgets.push_front(w);
            }
        }
    }

    /// 从指定位置起按 z-order 插入（对应 C++ `InsertWidgetHelper(where, theWidget)`）
    /// C++ 的循环条件是 `(*anItr)->mZOrder <= theWidget->mZOrder` —— 即**跳过** z 更小或相等的，
    /// 插入到第一个 z 更大的之前（注意与上面 insert_widget_helper 的 `>=` 停止条件不同）。
    fn insert_widget_helper_from(
        widgets: &mut ContainerList,
        from: usize,
        new_widget: WidgetContainerPtr,
    ) {
        let z = unsafe { (*new_widget).zorder };
        let mut a_target: Option<usize> = None;
        for (idx, a_w) in widgets.iter().enumerate() {
            if idx >= from && unsafe { (**a_w).zorder } > z {
                a_target = Some(idx);
                break;
            }
        }
        match a_target {
            Some(p) => {
                let mut tail = widgets.split_off(p);
                widgets.push_back(new_widget);
                widgets.append(&mut tail);
            }
            None => widgets.push_back(new_widget),
        }
    }

    /// 从列表中移除指定控件（按指针相等），返回是否命中
    /// [TRANSLATION_NOTE]: `ContainerList` 是 `LinkedList`，它没有稳定的"按值/按下标删除"API，
    /// 故用 `split_off` + `pop_front` 实现，语义等价于 C++ 的 `mWidgets.erase(anItr)`。
    fn remove_widget(widgets: &mut ContainerList, the_widget: WidgetContainerPtr) -> bool {
        if let Some(p) = widgets.iter().position(|w| *w == the_widget) {
            let mut tail = widgets.split_off(p);
            tail.pop_front();
            widgets.append(&mut tail);
            true
        } else {
            false
        }
    }

    /// 置于指定控件之后（对应 C++ WidgetContainer::PutBehind，WidgetContainer.cpp:301-318）
    /// C++ 中参数顺序为 (theWidget, theRefWidget)：把 theWidget 移到 theRefWidget 的**原位置之前**。
    pub fn put_behind(
        &mut self,
        the_widget: WidgetContainerPtr,
        the_ref_widget: WidgetContainerPtr,
    ) {
        if Self::remove_widget(&mut self.widgets, the_widget) {
            // C++: anItr = find(theRefWidget); InsertWidgetHelper(anItr, theWidget)
            // ref 不存在时 anItr == end() → 直接插到末尾
            let from = match self.widgets.iter().position(|w| *w == the_ref_widget) {
                Some(p) => p,
                None => self.widgets.len(),
            };
            Self::insert_widget_helper_from(&mut self.widgets, from, the_widget);
        }
    }

    /// 置于指定控件之前（对应 C++ WidgetContainer::PutInfront，WidgetContainer.cpp:320-339）
    pub fn put_infront(
        &mut self,
        the_widget: WidgetContainerPtr,
        the_ref_widget: WidgetContainerPtr,
    ) {
        if Self::remove_widget(&mut self.widgets, the_widget) {
            // C++: anItr = find(theRefWidget); if (anItr != end()) anItr++; InsertWidgetHelper(anItr, theWidget)
            let from = match self.widgets.iter().position(|w| *w == the_ref_widget) {
                Some(p) => p + 1,
                None => self.widgets.len(),
            };
            Self::insert_widget_helper_from(&mut self.widgets, from, the_widget);
        }
    }

    /// 删除所有子容器
    pub fn remove_all_children(&mut self) {
        self.widgets.clear();
    }

    /// 更新
    pub fn update(&mut self) {
        self.update_cnt += 1;
    }

    /// 最终矩形
    pub fn final_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics) {}
}

impl Default for WidgetContainer {
    fn default() -> Self {
        WidgetContainer::new()
    }
}
