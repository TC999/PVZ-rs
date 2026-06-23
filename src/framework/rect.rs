// PvZ Portable Rust 翻译 — 矩形类型
// 对应 C++ SexyAppFramework/misc/Rect.h

#![allow(dead_code)]

use crate::framework::point::Point;

/// 整数矩形
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub const ZERO: Rect = Rect { x: 0, y: 0, width: 0, height: 0 };

    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect { x, y, width, height }
    }

    pub fn from_points(tl: Point, br: Point) -> Self {
        Rect {
            x: tl.x,
            y: tl.y,
            width: br.x - tl.x,
            height: br.y - tl.y,
        }
    }

    /// 矩形的左边界
    pub fn left(&self) -> i32 { self.x }

    /// 矩形的右边界
    pub fn right(&self) -> i32 { self.x + self.width }

    /// 矩形的上边界
    pub fn top(&self) -> i32 { self.y }

    /// 矩形的下边界
    pub fn bottom(&self) -> i32 { self.y + self.height }

    /// 矩形左上角
    pub fn top_left(&self) -> Point { Point::new(self.x, self.y) }

    /// 矩形右下角
    pub fn bottom_right(&self) -> Point { Point::new(self.x + self.width, self.y + self.height) }

    /// 矩形中心点
    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2, self.y + self.height / 2)
    }

    /// 判断点是否在矩形内
    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
    }

    /// 判断点（Point）是否在矩形内
    pub fn contains_point(&self, p: Point) -> bool {
        self.contains(p.x, p.y)
    }

    /// 判断另一个矩形是否与此矩形相交
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }

    /// 计算两个矩形的交集
    pub fn intersection(&self, other: &Rect) -> Rect {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let w = (self.x + self.width).min(other.x + other.width) - x;
        let h = (self.y + self.height).min(other.y + other.height) - y;
        if w > 0 && h > 0 {
            Rect::new(x, y, w, h)
        } else {
            Rect::ZERO
        }
    }

    /// 膨胀矩形（正数扩大，负数缩小）
    pub fn inflate(&mut self, dx: i32, dy: i32) {
        self.x -= dx;
        self.y -= dy;
        self.width += 2 * dx;
        self.height += 2 * dy;
    }

    /// 偏移矩形位置
    pub fn offset(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn is_zero(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn area(&self) -> i32 {
        self.width * self.height
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect::ZERO
    }
}

/// 浮点矩形
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct RectF {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl RectF {
    pub const ZERO: RectF = RectF { x: 0.0, y: 0.0, width: 0.0, height: 0.0 };

    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        RectF { x, y, width, height }
    }

    pub fn left(&self) -> f64 { self.x }
    pub fn right(&self) -> f64 { self.x + self.width }
    pub fn top(&self) -> f64 { self.y }
    pub fn bottom(&self) -> f64 { self.y + self.height }
}

impl From<Rect> for RectF {
    fn from(r: Rect) -> Self {
        RectF {
            x: r.x as f64,
            y: r.y as f64,
            width: r.width as f64,
            height: r.height as f64,
        }
    }
}

impl Default for RectF {
    fn default() -> Self {
        RectF::ZERO
    }
}
