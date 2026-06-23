#![allow(dead_code)]

pub struct TextWidget {
    text: String,
}

impl TextWidget {
    pub fn new() -> Self {
        TextWidget { text: String::new() }
    }
}

impl Default for TextWidget {
    fn default() -> Self {
        Self::new()
    }
}
