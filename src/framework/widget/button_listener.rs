#![allow(dead_code)]

pub struct ButtonListener {
    id: u32,
}

impl ButtonListener {
    pub fn new() -> Self {
        ButtonListener { id: 0 }
    }
}

impl Default for ButtonListener {
    fn default() -> Self {
        Self::new()
    }
}
