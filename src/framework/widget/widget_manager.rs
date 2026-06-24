// PvZ Portable Rust 翻译 — WidgetManager
// 对应 C++ SexyAppFramework/widget/WidgetManager.h / WidgetManager.cpp

#![allow(dead_code)]

use crate::framework::widget::widget::Widget;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::KeyCode;
use crate::framework::rect::Rect;

/// Widget 管理器（负责事件分发和绘制管理）
pub struct WidgetManager {
    // 根 Widget 列表
    pub widget_list: Vec<*mut Widget>,
    // 当前鼠标位置
    pub last_mouse_x: i32,
    pub last_mouse_y: i32,
    pub mouse_x: i32,
    pub mouse_y: i32,
    // 鼠标按钮状态
    pub mouse_flags: i32,
    // 键盘状态
    pub key_down: Vec<bool>,
    // 焦点
    pub focus_widget: Option<*mut Widget>,
    // 图形上下文
    pub graphics: Graphics,
    // 应用程序
    pub app: Option<*mut crate::framework::sexy_app_base::SexyAppBase>,
}

impl WidgetManager {
    pub fn new() -> Self {
        WidgetManager {
            widget_list: Vec::new(),
            last_mouse_x: 0,
            last_mouse_y: 0,
            mouse_x: 0,
            mouse_y: 0,
            mouse_flags: 0,
            key_down: vec![false; 512],
            focus_widget: None,
            graphics: Graphics::new(),
            app: None,
        }
    }

    /// 添加 Widget
    pub fn add_widget(&mut self, widget: *mut Widget) {
        self.widget_list.push(widget);
    }

    /// 移除 Widget
    pub fn remove_widget(&mut self, widget: *mut Widget) {
        self.widget_list.retain(|w| *w != widget);
    }

    /// 设置焦点
    pub fn set_focus(&mut self, widget: Option<*mut Widget>) {
        if self.focus_widget != widget {
            if let Some(old) = self.focus_widget {
                unsafe { (*old).lost_focus(); }
            }
            self.focus_widget = widget;
            if let Some(new) = widget {
                unsafe { (*new).got_focus(); }
            }
        }
    }

    /// 处理鼠标移动
    pub fn mouse_move(&mut self, x: i32, y: i32) {
        self.last_mouse_x = self.mouse_x;
        self.last_mouse_y = self.mouse_y;
        self.mouse_x = x;
        self.mouse_y = y;
    }

    /// 处理鼠标按键
    pub fn mouse_down(&mut self, x: i32, y: i32, btn: i32, click_count: i32) {
        for widget in &self.widget_list {
            unsafe {
                if (**widget).visible && !(**widget).disabled {
                    (**widget).mouse_down(x, y, click_count);
                }
            }
        }
    }

    pub fn mouse_up(&mut self, x: i32, y: i32) {
        for widget in &self.widget_list {
            unsafe {
                if (**widget).visible {
                    (**widget).mouse_up(x, y);
                }
            }
        }
    }

    /// 处理按键
    pub fn key_down(&mut self, key: KeyCode) {
        if (key as usize) < self.key_down.len() {
            self.key_down[key as usize] = true;
        }
        if let Some(w) = self.focus_widget {
            unsafe { (*w).key_down(key); }
        }
    }

    pub fn key_up(&mut self, key: KeyCode) {
        if (key as usize) < self.key_down.len() {
            self.key_down[key as usize] = false;
        }
        if let Some(w) = self.focus_widget {
            unsafe { (*w).key_up(key); }
        }
    }

    /// 绘制所有 Widget
    pub fn draw(&self, g: &mut Graphics) {
        for widget in &self.widget_list {
            unsafe {
                if (**widget).visible {
                    (**widget).draw(g);
                }
            }
        }
    }

    /// 更新所有 Widget
    pub fn update(&mut self) {
        let widgets = self.widget_list.clone();
        for widget in &widgets {
            unsafe {
                (**widget).update();
            }
        }
    }

    /// 调整大小（对应 C++ WidgetManager::Resize）
    pub fn resize(&mut self, _rect: Rect, _presentation_rect: Rect) {
        // 在完整实现中，此方法会重新计算布局
    }
}

impl Default for WidgetManager {
    fn default() -> Self {
        WidgetManager::new()
    }
}
