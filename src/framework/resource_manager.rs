// 资源管理器
use std::collections::HashMap;

pub struct ResourceManager {
    pub resources: HashMap<String, *mut std::ffi::c_void>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: HashMap::new(),
        }
    }

    pub fn load_resource(&mut self, _name: &str, _path: &str) -> bool { true }
    pub fn get_resource(&self, name: &str) -> Option<*mut std::ffi::c_void> {
        self.resources.get(name).copied()
    }
    pub fn has_resource(&self, name: &str) -> bool {
        self.resources.contains_key(name)
    }
}

impl Default for ResourceManager {
    fn default() -> Self { ResourceManager::new() }
}
