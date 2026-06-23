#![allow(dead_code)]

pub struct ScrollbarWidget {
    position: u32,
}

impl ScrollbarWidget {
    pub fn new() -> Self {
        ScrollbarWidget { position: 0 }
    }
}

impl Default for ScrollbarWidget {
    fn default() -> Self {
        Self::new()
    }
}
