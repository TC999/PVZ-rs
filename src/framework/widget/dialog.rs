#![allow(dead_code)]

pub struct Dialog {
    is_open: bool,
}

impl Dialog {
    pub fn new() -> Self {
        Dialog { is_open: false }
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}
