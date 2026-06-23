#![allow(dead_code)]

pub struct Checkbox {
    checked: bool,
}

impl Checkbox {
    pub fn new() -> Self {
        Checkbox { checked: false }
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}
