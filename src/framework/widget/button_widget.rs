#![allow(dead_code)]

pub struct ButtonWidget {
    label: String,
}

impl ButtonWidget {
    pub fn new() -> Self {
        ButtonWidget { label: String::new() }
    }
}

impl Default for ButtonWidget {
    fn default() -> Self {
        Self::new()
    }
}
