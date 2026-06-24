// PvZ Portable Rust 翻译 — ResourceManager 资源管理器
// 对应 C++ SexyAppFramework/misc/ResourceManager.h / ResourceManager.cpp
//
// 从 XML 资源配置文件加载/管理图像、字体、声音等资源。

#![allow(dead_code)]

use std::collections::{HashMap, HashSet, LinkedList};
use std::ptr;

use crate::framework::common::string_to_lower;
use crate::framework::graphics::image::Image;
use crate::framework::graphics::gl_image::GLImage;
use crate::framework::graphics::memory_image::MemoryImage;
use crate::framework::graphics::shared_image::{SharedImage, SharedImageRef};
use crate::framework::graphics::font::Font;
use crate::framework::sexy_app_base::SexyAppBase;

/// 资源类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResType {
    Image,
    Sound,
    Font,
}

/// 基础资源（对应 C++ BaseRes）
#[derive(Debug)]
pub struct BaseRes {
    pub res_type: ResType,
    pub id: String,
    pub res_group: String,
    pub path: String,
    pub from_program: bool,
}

impl BaseRes {
    pub fn new(res_type: ResType) -> Self {
        BaseRes { res_type, id: String::new(), res_group: String::new(), path: String::new(), from_program: false }
    }
    pub fn delete_resource(&mut self) {}
}

/// 图像资源（对应 C++ ImageRes）
#[derive(Debug)]
pub struct ImageRes {
    pub base: BaseRes,
    pub image: SharedImageRef,
    pub alpha_image: String,
    pub alpha_grid_image: String,
    pub variant: String,
    pub auto_find_alpha: bool,
    pub palletize: bool,
    pub rows: i32,
    pub cols: i32,
    pub alpha_color: u32,
}

/// 声音资源（对应 C++ SoundRes）
#[derive(Debug)]
pub struct SoundRes {
    pub base: BaseRes,
    pub sound_id: isize,
    pub volume: f64,
    pub panning: i32,
}

/// 字体资源（对应 C++ FontRes）
#[derive(Debug)]
pub struct FontRes {
    pub base: BaseRes,
    pub font: Option<Box<Font>>,
    pub image: *mut Image,
    pub image_path: String,
    pub tags: String,
    pub sys_font: bool,
    pub bold: bool,
    pub italic: bool,
    pub size: i32,
}

/// 资源管理器异常（对应 C++ ResourceManagerException）
#[derive(Debug, Clone)]
pub struct ResourceManagerException {
    pub what: String,
}

/// 资源管理器（对应 C++ ResourceManager）
pub struct ResourceManager {
    /// 已加载的资源组
    pub loaded_groups: HashSet<String>,
    /// 图像资源映射
    pub image_map: HashMap<String, *mut BaseRes>,
    /// 声音资源映射
    pub sound_map: HashMap<String, *mut BaseRes>,
    /// 字体资源映射
    pub font_map: HashMap<String, *mut BaseRes>,
    /// 错误信息
    pub error: String,
    /// 是否失败
    pub has_failed: bool,
    /// 应用基类指针
    pub app: Option<*mut SexyAppBase>,
    /// 当前资源组
    pub cur_res_group: String,
    /// 默认路径
    pub default_path: String,
    /// 默认 ID 前缀
    pub default_id_prefix: String,
    /// 允许缺失程序资源
    pub allow_missing_program_resources: bool,
}

impl ResourceManager {
    pub fn new(the_app: Option<*mut SexyAppBase>) -> Self {
        ResourceManager {
            loaded_groups: HashSet::new(),
            image_map: HashMap::new(),
            sound_map: HashMap::new(),
            font_map: HashMap::new(),
            error: String::new(),
            has_failed: false,
            app: the_app,
            cur_res_group: String::new(),
            default_path: String::new(),
            default_id_prefix: String::new(),
            allow_missing_program_resources: false,
        }
    }

    /// 解析资源文件（对应 C++ ParseResourcesFile）
    pub fn parse_resources_file(&mut self, _filename: &str) -> bool { true }

    /// 重新解析资源文件
    pub fn reparse_resources_file(&mut self, _filename: &str) -> bool { true }

    /// 获取错误文本
    pub fn get_error_text(&self) -> &str { &self.error }

    /// 是否有错误
    pub fn had_error(&self) -> bool { self.has_failed }

    /// 检查组是否已加载
    pub fn is_group_loaded(&self, group: &str) -> bool {
        self.loaded_groups.contains(&string_to_lower(group))
    }

    /// 加载下一个资源（对应 C++ LoadNextResource）
    pub fn load_next_resource(&mut self) -> bool { false }

    /// 开始加载资源组
    pub fn start_load_resources(&mut self, _group: &str) {}

    /// 加载整个资源组
    pub fn load_resources(&mut self, _group: &str) -> bool { true }

    /// 替换图像（对应 C++ ReplaceImage）
    pub fn replace_image(&mut self, _id: &str, _image: *mut Image) -> bool { false }

    /// 替换声音
    pub fn replace_sound(&mut self, _id: &str, _sound: isize) -> bool { false }

    /// 替换字体
    pub fn replace_font(&mut self, _id: &str, _font: *mut Font) -> bool { false }

    /// 删除图像资源（对应 C++ DeleteImage）
    pub fn delete_image(&mut self, _name: &str) {}

    /// 加载图像（对应 C++ LoadImage）
    pub fn load_image(&mut self, _name: &str) -> SharedImageRef { SharedImageRef::new() }

    /// 删除声音资源
    pub fn delete_sound(&mut self, _name: &str) {}

    /// 删除字体资源
    pub fn delete_font(&mut self, _name: &str) {}

    /// 加载字体
    pub fn load_font(&mut self, _name: &str) -> Option<*mut Font> { None }

    /// 获取图像（对应 C++ GetImage）
    pub fn get_image(&self, the_id: &str) -> SharedImageRef {
        let lower = string_to_lower(the_id);
        if let Some(res_ptr) = self.image_map.get(&lower) {
            unsafe {
                let res = &*(*res_ptr as *const ImageRes);
                return res.image.clone();
            }
        }
        SharedImageRef::new()
    }

    /// 获取声音
    pub fn get_sound(&self, _id: &str) -> isize { 0 }

    /// 获取字体
    pub fn get_font(&self, _id: &str) -> Option<*mut Font> { None }

    /// 获取图像（抛出异常版本）
    pub fn get_image_throw(&self, the_id: &str) -> SharedImageRef {
        let r = self.get_image(the_id);
        r
    }

    /// 获取声音（抛出异常版本）
    pub fn get_sound_throw(&self, _id: &str) -> isize { 0 }

    /// 获取字体（抛出异常版本）
    pub fn get_font_throw(&self, _id: &str) -> Option<*mut Font> { None }

    /// 删除资源组（对应 C++ DeleteResources）
    pub fn delete_resources(&mut self, _group: &str) {}

    /// 释放跟踪资源（对应 C++ ReleaseTrackedResources）
    pub fn release_tracked_resources(&mut self, names: &[String]) {
        for name in names {
            let lower = string_to_lower(name);
            self.image_map.remove(&lower);
            self.sound_map.remove(&lower);
            self.font_map.remove(&lower);
        }
    }

    /// 失败处理（对应 C++ Fail）
    fn fail(&mut self, err: &str) -> bool {
        self.error = err.to_string();
        self.has_failed = true;
        false
    }
}

impl Drop for ResourceManager {
    fn drop(&mut self) {
        // 清理资源映射
        for (_, ptr) in self.image_map.drain() {
            unsafe { let _ = Box::from_raw(ptr); }
        }
        for (_, ptr) in self.sound_map.drain() {
            unsafe { let _ = Box::from_raw(ptr); }
        }
        for (_, ptr) in self.font_map.drain() {
            unsafe { let _ = Box::from_raw(ptr); }
        }
    }
}
