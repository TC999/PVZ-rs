#![allow(dead_code)]

pub struct DataSync {
    pending: Vec<u8>,
}

impl DataSync {
    pub fn new() -> Self {
        DataSync { pending: Vec::new() }
    }
}

impl Default for DataSync {
    fn default() -> Self {
        Self::new()
    }
}
