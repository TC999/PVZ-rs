#![allow(dead_code)]

pub struct GameSelector;

impl GameSelector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GameSelector {
    fn default() -> Self {
        Self::new()
    }
}
