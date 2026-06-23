#![allow(dead_code)]

pub struct DataArray<T> {
    items: Vec<T>,
}

impl<T> DataArray<T> {
    pub fn new() -> Self {
        DataArray { items: Vec::new() }
    }
}

impl<T> Default for DataArray<T> {
    fn default() -> Self {
        Self::new()
    }
}
