#![allow(dead_code)]

pub struct ReanimAtlas {
    entries: Vec<()>,
}

impl ReanimAtlas {
    pub fn new() -> Self {
        ReanimAtlas { entries: Vec::new() }
    }
}

impl Default for ReanimAtlas {
    fn default() -> Self {
        Self::new()
    }
}
