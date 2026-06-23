#![allow(dead_code)]

pub struct ReanimationLawn {
    animations: Vec<()>,
}

impl ReanimationLawn {
    pub fn new() -> Self {
        ReanimationLawn { animations: Vec::new() }
    }
}

impl Default for ReanimationLawn {
    fn default() -> Self {
        Self::new()
    }
}
