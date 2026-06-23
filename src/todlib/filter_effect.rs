#![allow(dead_code)]

pub struct FilterEffect {
    data: Vec<u8>,
}

impl FilterEffect {
    pub fn new() -> Self {
        FilterEffect { data: Vec::new() }
    }
}

impl Default for FilterEffect {
    fn default() -> Self {
        Self::new()
    }
}
