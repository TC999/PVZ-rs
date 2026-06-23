#![allow(dead_code)]

pub struct ListWidget {
    items: Vec<String>,
}

impl ListWidget {
    pub fn new() -> Self {
        ListWidget { items: Vec::new() }
    }
}

impl Default for ListWidget {
    fn default() -> Self {
        Self::new()
    }
}
