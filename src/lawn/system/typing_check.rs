#![allow(dead_code)]

pub struct TypingCheck {
    buffer: String,
}

impl TypingCheck {
    pub fn new() -> Self {
        TypingCheck { buffer: String::new() }
    }
}

impl Default for TypingCheck {
    fn default() -> Self {
        Self::new()
    }
}
