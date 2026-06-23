#![allow(dead_code)]

pub struct ToolTipWidget {
    text: String,
}

impl ToolTipWidget {
    pub fn new() -> Self {
        ToolTipWidget { text: String::new() }
    }
}

impl Default for ToolTipWidget {
    fn default() -> Self {
        Self::new()
    }
}
