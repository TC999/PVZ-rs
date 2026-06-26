// PvZ Portable Rust 翻译 — GameObject（游戏对象基类）
// 对应 C++ src/Lawn/GameObject.h / GameObject.cpp

use crate::lawn::lawn_app::LawnApp;
use crate::lawn::board::Board;
use crate::framework::graphics::graphics::Graphics;

/// 游戏对象基类（所有游戏实体的基类）
#[derive(Debug)]
pub struct GameObject {
    pub app: Option<*mut LawnApp>,
    pub board: Option<*mut Board>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
    pub row: i32,
    pub render_order: i32,
}

impl GameObject {
    pub fn new() -> Self {
        GameObject {
            app: None,
            board: None,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            visible: true,
            row: -1,
            render_order: 0,
        }
    }

    /// 开始绘制（设置裁剪区域）
    /// 如果对象不可见，返回 false；否则将坐标转换为 Graphics 上下文
    pub fn begin_draw(&self, g: &mut Graphics) -> bool {
        if !self.visible {
            return false;
        }
        g.translate(self.x, self.y);
        true
    }

    /// 结束绘制
    /// 恢复 Graphics 上下文的坐标转换
    pub fn end_draw(&self, g: &mut Graphics) {
        g.translate(-self.x, -self.y);
    }

    /// 创建父级图形帧
    /// 恢复 Graphics 上下文的坐标转换
    pub fn make_parent_graphics_frame(&self, g: &mut Graphics) {
        g.translate(-self.x, -self.y);
    }

    /// 获取 Board 引用
    pub fn get_board(&self) -> Option<&Board> {
        unsafe { self.board.map(|b| &*b) }
    }

    pub fn get_board_mut(&mut self) -> Option<&mut Board> {
        unsafe { self.board.map(|b| &mut *b) }
    }

    /// 获取 LawnApp 引用
    pub fn get_app(&self) -> Option<&LawnApp> {
        unsafe { self.app.map(|a| &*a) }
    }
}

impl Default for GameObject {
    fn default() -> Self {
        GameObject::new()
    }
}
