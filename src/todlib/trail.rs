#![allow(dead_code)]

pub struct Trail {
    points: Vec<()>,
}

impl Trail {
    pub fn new() -> Self {
        Trail { points: Vec::new() }
    }
}

impl Default for Trail {
    fn default() -> Self {
        Self::new()
    }
}
