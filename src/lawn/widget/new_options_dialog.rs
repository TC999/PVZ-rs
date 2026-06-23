#![allow(dead_code)]

pub struct NewOptionsDialog;

impl NewOptionsDialog {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NewOptionsDialog {
    fn default() -> Self {
        Self::new()
    }
}
