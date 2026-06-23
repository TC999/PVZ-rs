#![allow(dead_code)]

pub struct LawnDialog;

impl LawnDialog {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LawnDialog {
    fn default() -> Self {
        Self::new()
    }
}
