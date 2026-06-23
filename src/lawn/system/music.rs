#![allow(dead_code)]

pub struct Music {
    volume: f64,
}

impl Music {
    pub fn new() -> Self {
        Music { volume: 0.0 }
    }
}

impl Default for Music {
    fn default() -> Self {
        Self::new()
    }
}
