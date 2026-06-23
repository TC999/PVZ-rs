#![allow(dead_code)]

pub struct UserDialog;

impl UserDialog {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UserDialog {
    fn default() -> Self {
        Self::new()
    }
}
