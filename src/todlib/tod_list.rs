#![allow(dead_code)]

pub struct TodList<T> {
    items: Vec<T>,
}

impl<T> TodList<T> {
    pub fn new() -> Self {
        TodList { items: Vec::new() }
    }
}

impl<T> Default for TodList<T> {
    fn default() -> Self {
        Self::new()
    }
}
