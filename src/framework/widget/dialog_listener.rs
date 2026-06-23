#![allow(dead_code)]

pub struct DialogListener {
    id: u32,
}

impl DialogListener {
    pub fn new() -> Self {
        DialogListener { id: 0 }
    }
}

impl Default for DialogListener {
    fn default() -> Self {
        Self::new()
    }
}
