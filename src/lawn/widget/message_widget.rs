#![allow(dead_code)]

pub struct MessageWidget;

impl MessageWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MessageWidget {
    fn default() -> Self {
        Self::new()
    }
}
