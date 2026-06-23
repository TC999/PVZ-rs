#![allow(dead_code)]

pub struct PlayerInfo {
    name: String,
}

impl PlayerInfo {
    pub fn new() -> Self {
        PlayerInfo { name: String::new() }
    }
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self::new()
    }
}
