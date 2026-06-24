// PvZ Portable Rust 翻译 — 点类型
// 对应 C++ SexyAppFramework/misc/Point.h

#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Neg};

/// 二维整数点
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const ZERO: Point = Point { x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn offset(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, scalar: i32) -> Point {
        Point { x: self.x * scalar, y: self.y * scalar }
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point { x: -self.x, y: -self.y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::ZERO
    }
}

/// 二维浮点点
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct PointF {
    pub x: f64,
    pub y: f64,
}

impl PointF {
    pub const ZERO: PointF = PointF { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        PointF { x, y }
    }
}

impl From<Point> for PointF {
    fn from(p: Point) -> Self {
        PointF { x: p.x as f64, y: p.y as f64 }
    }
}

impl Default for PointF {
    fn default() -> Self {
        PointF::ZERO
    }
}
