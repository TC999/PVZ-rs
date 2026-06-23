#![allow(dead_code)]

pub struct ProfileMgr {
    profiles: Vec<String>,
}

impl ProfileMgr {
    pub fn new() -> Self {
        ProfileMgr { profiles: Vec::new() }
    }
}

impl Default for ProfileMgr {
    fn default() -> Self {
        Self::new()
    }
}
