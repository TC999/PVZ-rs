#![allow(dead_code)]

pub struct ZenGarden {
    plants: Vec<()>,
}

impl ZenGarden {
    pub fn new() -> Self {
        ZenGarden { plants: Vec::new() }
    }
}

impl Default for ZenGarden {
    fn default() -> Self {
        Self::new()
    }
}
