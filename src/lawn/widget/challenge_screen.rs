#![allow(dead_code)]

pub struct ChallengeScreen;

impl ChallengeScreen {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ChallengeScreen {
    fn default() -> Self {
        Self::new()
    }
}
