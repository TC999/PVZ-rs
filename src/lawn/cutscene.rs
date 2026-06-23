#![allow(dead_code)]

pub struct Cutscene {
    frame: u32,
}

impl Cutscene {
    pub fn new() -> Self {
        Cutscene { frame: 0 }
    }
}

impl Default for Cutscene {
    fn default() -> Self {
        Self::new()
    }
}
