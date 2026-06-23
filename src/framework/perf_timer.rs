// PvZ Portable Rust 翻译 — PerfTimer 类型
// 对应 C++ SexyAppFramework/misc/PerfTimer.h / PerfTimer.cpp

#![allow(dead_code)]

use std::time::{Instant, Duration};

/// 性能计时器
#[derive(Clone)]
pub struct PerfTimer {
    start: Instant,
}

impl PerfTimer {
    pub fn new() -> Self {
        PerfTimer {
            start: Instant::now(),
        }
    }

    /// 开始计时
    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    /// 停止计时并返回经过的微秒数
    pub fn stop(&self) -> f64 {
        self.elapsed().as_secs_f64() * 1_000_000.0
    }

    /// 获取经过的时间
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// 获取经过的毫秒数
    pub fn elapsed_ms(&self) -> f64 {
        self.elapsed().as_secs_f64() * 1000.0
    }

    /// 获取经过的微秒数
    pub fn elapsed_us(&self) -> f64 {
        self.elapsed().as_secs_f64() * 1_000_000.0
    }
}

impl Default for PerfTimer {
    fn default() -> Self {
        PerfTimer::new()
    }
}
