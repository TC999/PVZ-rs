// PakLib — PAK 包接口
// 用于从 main.pak 资源包加载文件

pub struct PakInterface;

impl PakInterface {
    pub fn new() -> Self { PakInterface }
    pub fn open_pak(&mut self, _path: &str) -> bool { true }
    pub fn load_file(&self, _path: &str) -> Option<Vec<u8>> { None }
    pub fn file_exists(&self, _path: &str) -> bool { false }
}
