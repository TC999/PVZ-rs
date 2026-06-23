#![allow(dead_code)]

pub struct SaveGame {
    slot: u32,
}

impl SaveGame {
    pub fn new() -> Self {
        SaveGame { slot: 0 }
    }
}

impl Default for SaveGame {
    fn default() -> Self {
        Self::new()
    }
}
