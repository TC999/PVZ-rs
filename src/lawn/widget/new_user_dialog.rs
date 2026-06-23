#![allow(dead_code)]

pub struct NewUserDialog;

impl NewUserDialog {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NewUserDialog {
    fn default() -> Self {
        Self::new()
    }
}
