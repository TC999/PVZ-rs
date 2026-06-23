// PvZ Portable Rust 翻译 — WidgetContainer
// 对应 C++ SexyAppFramework/widget/WidgetContainer.h / WidgetContainer.cpp

#![allow(dead_code)]

use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_manager::WidgetManager;

/// 组件容器基类
pub struct WidgetContainer {
    // 位置和大小
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,

    // 父子关系
    pub parent: Option<*mut WidgetContainer>,
    pub widget_manager: Option<*mut WidgetManager>,

    // 子组件列表
    pub widgets: Vec<*mut WidgetContainer>,

    // 裁剪
    pub clip: bool,
    pub clip_rect: Rect,
}

impl WidgetContainer {
    pub fn new() -> Self {
        WidgetContainer {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            parent: None,
            widget_manager: None,
            widgets: Vec::new(),
            clip: false,
            clip_rect: Rect::ZERO,
        }
    }

    /// 添加子组件
    pub fn add_widget(&mut self, widget: *mut WidgetContainer) {
        unsafe {
            (*widget).parent = Some(self as *mut WidgetContainer);
        }
        self.widgets.push(widget);
    }

    /// 移除子组件
    pub fn remove_widget(&mut self, widget: *mut WidgetContainer) {
        self.widgets.retain(|w| *w != widget);
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics) {
        // 由子类实现
    }

    /// 更新
    pub fn update(&mut self) {
        for widget in &self.widgets {
            unsafe { (**widget).update(); }
        }
    }

    /// 最终矩形
    pub fn final_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}

impl Default for WidgetContainer {
    fn default() -> Self {
        WidgetContainer::new()
    }
}
