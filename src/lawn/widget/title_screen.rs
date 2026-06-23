#![allow(dead_code)]

pub struct TitleScreen;

impl TitleScreen {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TitleScreen {
    fn default() -> Self {
        Self::new()
    }
}
