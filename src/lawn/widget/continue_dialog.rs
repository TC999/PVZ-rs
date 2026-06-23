#![allow(dead_code)]

pub struct ContinueDialog;

impl ContinueDialog {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ContinueDialog {
    fn default() -> Self {
        Self::new()
    }
}
