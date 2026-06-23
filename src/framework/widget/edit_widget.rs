#![allow(dead_code)]

pub struct EditWidget {
    text: String,
}

impl EditWidget {
    pub fn new() -> Self {
        EditWidget { text: String::new() }
    }
}

impl Default for EditWidget {
    fn default() -> Self {
        Self::new()
    }
}
