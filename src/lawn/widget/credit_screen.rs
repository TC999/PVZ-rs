#![allow(dead_code)]

pub struct CreditScreen;

impl CreditScreen {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CreditScreen {
    fn default() -> Self {
        Self::new()
    }
}
