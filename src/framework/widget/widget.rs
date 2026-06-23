// PvZ Portable Rust 翻译 — Widget
// 对应 C++ SexyAppFramework/widget/Widget.h / Widget.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::point::Point;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_container::WidgetContainer;
use crate::framework::widget::insets::Insets;
use crate::framework::key_codes::KeyCode;

/// 组件（Widget）
pub struct Widget {
    pub container: WidgetContainer,

    // 可见性
    pub visible: bool,
    pub mouse_visible: bool,
    pub disabled: bool,
    pub has_focus: bool,
    pub is_down: bool,
    pub is_over: bool,
    pub has_transparencies: bool,

    // 颜色
    pub colors: Vec<Color>,

    // 鼠标
    pub mouse_insets: Insets,

    // 焦点管理
    pub wants_focus: bool,

    // 资源
    pub loaded_resource_names: Vec<String>,
}

impl Widget {
    pub fn new() -> Self {
        Widget {
            container: WidgetContainer::new(),
            visible: true,
            mouse_visible: true,
            disabled: false,
            has_focus: false,
            is_down: false,
            is_over: false,
            has_transparencies: false,
            colors: Vec::new(),
            mouse_insets: Insets::new(0, 0, 0, 0),
            wants_focus: false,
            loaded_resource_names: Vec::new(),
        }
    }

    /// 设置可见性
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// 启用/禁用
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// 设置颜色
    pub fn set_color(&mut self, idx: usize, color: &Color) {
        if idx >= self.colors.len() {
            self.colors.resize(idx + 1, Color::WHITE);
        }
        self.colors[idx] = *color;
    }

    pub fn get_color(&self, idx: usize) -> Color {
        self.colors.get(idx).copied().unwrap_or(Color::WHITE)
    }

    /// 调整大小
    pub fn resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.container.x = x;
        self.container.y = y;
        self.container.width = width;
        self.container.height = height;
    }

    /// 绘制（已变换坐标）
    pub fn draw(&self, _g: &mut Graphics) {
        // 由子类实现
    }

    /// 叠加层绘制
    pub fn draw_overlay(&self, _g: &mut Graphics) {
        // 由子类实现
    }

    /// 更新
    pub fn update(&mut self) {
        // 由子类实现
    }

    /// 按键事件
    pub fn key_char(&mut self, _the_char: u8) {}
    pub fn key_down(&mut self, _the_key: KeyCode) {}
    pub fn key_up(&mut self, _the_key: KeyCode) {}

    /// 鼠标事件
    pub fn mouse_enter(&mut self) {}
    pub fn mouse_leave(&mut self) {}
    pub fn mouse_move(&mut self, _x: i32, _y: i32) {}
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {}
    pub fn mouse_up(&mut self, _x: i32, _y: i32) {}
    pub fn mouse_drag(&mut self, _x: i32, _y: i32) {}
    pub fn mouse_wheel(&mut self, _delta: i32) {}

    /// 焦点事件
    pub fn got_focus(&mut self) {}
    pub fn lost_focus(&mut self) {}

    /// 判断点是否在组件内
    pub fn contains(&self, x: i32, y: i32) -> bool {
        let rect = self.container.final_rect();
        rect.contains(x, y)
    }

    pub fn left(&self) -> i32 { self.container.x }
    pub fn top(&self) -> i32 { self.container.y }
    pub fn right(&self) -> i32 { self.container.x + self.container.width }
    pub fn bottom(&self) -> i32 { self.container.y + self.container.height }
    pub fn width(&self) -> i32 { self.container.width }
    pub fn height(&self) -> i32 { self.container.height }
}

impl Default for Widget {
    fn default() -> Self {
        Widget::new()
    }
}
