#![allow(dead_code)]

pub struct CheatDialog;

impl CheatDialog {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CheatDialog {
    fn default() -> Self {
        Self::new()
    }
}
