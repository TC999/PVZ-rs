#![allow(dead_code)]

pub struct TodDebug {
    enabled: bool,
}

impl TodDebug {
    pub fn new() -> Self {
        TodDebug { enabled: false }
    }
}

impl Default for TodDebug {
    fn default() -> Self {
        Self::new()
    }
}
