// PvZ Portable Rust 翻译 — Ratio 比例/分数类型
// 对应 C++ SexyAppFramework/misc/Ratio.h / Ratio.cpp

#![allow(dead_code)]

use std::cmp::Ordering;

/// 比例/分数类型（对应 C++ Ratio）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ratio {
    pub numerator: i32,
    pub denominator: i32,
}

impl Ratio {
    pub fn new() -> Self { Ratio { numerator: 1, denominator: 1 } }
    pub fn from(n: i32, d: i32) -> Self {
        let mut r = Ratio { numerator: n, denominator: d };
        r.reduce();
        r
    }

    /// 约分（辗转相除法找最大公约数）
    pub fn set(&mut self, n: i32, d: i32) {
        self.numerator = n;
        self.denominator = d;
        self.reduce();
    }

    fn reduce(&mut self) {
        let (mut a, mut b) = (self.numerator.abs(), self.denominator.abs());
        while b != 0 { let t = b; b = a % b; a = t; }
        if a != 0 { self.numerator /= a; self.denominator /= a; }
        if self.denominator < 0 { self.numerator = -self.numerator; self.denominator = -self.denominator; }
    }
}

impl Default for Ratio { fn default() -> Self { Ratio::new() } }

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs = self.numerator as i64 * other.denominator as i64;
        let rhs = other.numerator as i64 * self.denominator as i64;
        lhs.partial_cmp(&rhs)
    }
}

impl std::ops::Mul<i32> for Ratio {
    type Output = i32;
    fn mul(self, rhs: i32) -> i32 { rhs * self.numerator / self.denominator }
}

impl std::ops::Div<i32> for Ratio {
    type Output = i32;
    fn div(self, rhs: i32) -> i32 { rhs * self.denominator / self.numerator }
}

impl std::ops::Mul<Ratio> for i32 {
    type Output = i32;
    fn mul(self, rhs: Ratio) -> i32 { self * rhs.numerator / rhs.denominator }
}

impl std::ops::Div<Ratio> for i32 {
    type Output = i32;
    fn div(self, rhs: Ratio) -> i32 { self * rhs.denominator / rhs.numerator }
}
