#![allow(dead_code)]

pub struct GameButton;

impl GameButton {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GameButton {
    fn default() -> Self {
        Self::new()
    }
}
