#![allow(dead_code)]

pub struct Slider {
    value: f64,
}

impl Slider {
    pub fn new() -> Self {
        Slider { value: 0.0 }
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}
