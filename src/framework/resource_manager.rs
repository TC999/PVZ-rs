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
use crate::framework::xml_parser::{XMLParser, XMLElement};

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
    /// 字体基类指针（指向泄露的 ImageFont::base）
    pub font_ptr: Option<*mut Font>,
    /// 泄露的 ImageFont 所有者指针（用于 Drop 清理）
    pub imagefont_owner: Option<*mut std::ffi::c_void>,
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
    pub fn parse_resources_file(&mut self, filename: &str) -> bool {
        let mut parser = XMLParser::new();
        eprintln!("  parse_resources_file: 打开 {}", filename);

        // 先检查 PAK 中是否有该文件
        crate::framework::paklib::with_pak_interface(|pak| {
            let key = crate::framework::paklib::normalize_pak_path(filename);
            eprintln!("  PAK 中查找: '{}' → exists={}", key, pak.file_exists(filename));
        });

        if !parser.open_file(filename) {
            return self.fail(&format!("无法打开资源文件 '{}': {}", filename, parser.get_error_text()));
        }
        eprintln!("  parse_resources_file: 文件打开成功");

        let mut element = XMLElement::new();

        // 跳过开头的注释和处理指令，找到 <Resources> 根元素
        eprintln!("  parse_resources_file: 查找 <Resources> 根元素...");
        loop {
            if !parser.next_element(&mut element) {
                return self.fail(&format!("资源文件 '{}' 中未找到 <Resources> 根元素", filename));
            }
            if element.elem_type == XMLElement::TYPE_START && (element.section == "Resources" || element.section == "ResourceManifest") {
                break; // 找到根元素
            }
            // 跳过 注释、处理指令 等非元素节点
            if element.elem_type == XMLElement::TYPE_ELEMENT || element.elem_type == XMLElement::TYPE_START {
            // 遇到了非 Resources/ResourceManifest 的元素 — 格式错误
                return self.fail(&format!("资源文件 '{}' 的根元素应为 <ResourceManifest>，但找到 <{}>", filename, element.section));
            }
        }
        eprintln!("  parse_resources_file: 找到 <Resources>");

        // 循环解析子元素
        loop {
            if !parser.next_element(&mut element) {
                break;
            }

            match element.elem_type {
                XMLElement::TYPE_START => {
                    // 遇到带子元素的标签（如 <Font> 可能有子标签）
                    self.parse_resource_container(&mut parser, &element);
                }
                XMLElement::TYPE_ELEMENT => {
                    // 自闭合标签 <Image ... /> 或 <Sound ... />
                    self.parse_resource(&element);
                }
                XMLElement::TYPE_END => {
                    // </ResourceManifest> 或 </Resources> 结束
                    if element.section == "Resources" || element.section == "ResourceManifest" {
                        return true;
                    }
                }
                _ => {} // 忽略其他类型
            }
        }

        true
    }

    /// 解析自闭合资源元素（Image / Sound / Font 等）
    fn parse_resource(&mut self, element: &XMLElement) {
        let id: &String = match element.attributes.get("id") {
            Some(id) => id,
            None => return, // 没有 id 属性的元素跳过
        };

        match element.section.as_str() {
            "Image" => {
                let path = element.attributes.get("path").cloned().unwrap_or_default();
                let rows: i32 = element.attributes.get("rows").and_then(|v| v.parse::<i32>().ok()).unwrap_or(1);
                let cols: i32 = element.attributes.get("cols").and_then(|v| v.parse::<i32>().ok()).unwrap_or(1);
                let alpha_color_str = element.attributes.get("alphaColor").cloned().unwrap_or_default();
                let alpha_color: u32 = if alpha_color_str.starts_with("0x") || alpha_color_str.starts_with("0X") {
                    u32::from_str_radix(&alpha_color_str[2..], 16).unwrap_or(0)
                } else {
                    0
                };

                let mut res = Box::new(ImageRes {
                    base: BaseRes::new(ResType::Image),
                    image: crate::framework::graphics::shared_image::SharedImageRef::new(),
                    alpha_image: element.attributes.get("alphaImage").cloned().unwrap_or_default(),
                    alpha_grid_image: element.attributes.get("alphaGridImage").cloned().unwrap_or_default(),
                    variant: String::new(),
                    auto_find_alpha: element.attributes.get("autoFindAlpha").map(|v| v == "true").unwrap_or(false),
                    palletize: element.attributes.get("palletize").map(|v| v == "true").unwrap_or(false),
                    rows,
                    cols,
                    alpha_color,
                });
                res.base.id = id.clone();
                res.base.path = path;
                res.base.res_group = self.cur_res_group.clone();
                let key = string_to_lower(id);
                self.image_map.insert(key, Box::into_raw(res) as *mut BaseRes);
            }
            "Sound" => {
                let path = element.attributes.get("path").cloned().unwrap_or_default();
                let volume: f64 = element.attributes.get("volume").and_then(|v| v.parse::<f64>().ok()).unwrap_or(1.0);
                let mut res = Box::new(SoundRes {
                    base: BaseRes::new(ResType::Sound),
                    sound_id: 0,
                    volume,
                    panning: 0,
                });
                res.base.id = id.clone();
                res.base.path = path;
                res.base.res_group = self.cur_res_group.clone();
                let key = string_to_lower(id);
                self.sound_map.insert(key, Box::into_raw(res) as *mut BaseRes);
            }
            "Font" => {
                let path = element.attributes.get("path").cloned().unwrap_or_default();
                let size: i32 = element.attributes.get("size").and_then(|v| v.parse::<i32>().ok()).unwrap_or(16);

                let mut res = Box::new(FontRes {
                    base: BaseRes::new(ResType::Font),
                    font_ptr: None,
                    imagefont_owner: None,
                    image: std::ptr::null_mut(),
                    image_path: element.attributes.get("image").cloned().unwrap_or_default(),
                    tags: element.attributes.get("tags").cloned().unwrap_or_default(),
                    sys_font: element.attributes.get("sysFont").map(|v| v == "true").unwrap_or(false),
                    bold: element.attributes.get("bold").map(|v| v == "true").unwrap_or(false),
                    italic: element.attributes.get("italic").map(|v| v == "true").unwrap_or(false),
                    size,
                });
                res.base.id = id.clone();
                res.base.path = path;
                res.base.res_group = self.cur_res_group.clone();
                let key = string_to_lower(id);
                self.font_map.insert(key, Box::into_raw(res) as *mut BaseRes);
            }
            _ => {} // 未知类型忽略
        }
    }

    /// 解析带子元素的资源容器（如 <Font ...>...</Font>）
    fn parse_resource_container(&mut self, parser: &mut XMLParser, start_element: &XMLElement) {
        // 原子地处理当前容器内的所有子元素
        let section_name = start_element.section.clone();
        let mut depth = 1;

        let mut element = XMLElement::new();
        loop {
            if !parser.next_element(&mut element) {
                break;
            }
            match element.elem_type {
                XMLElement::TYPE_START => depth += 1,
                XMLElement::TYPE_END => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                XMLElement::TYPE_ELEMENT => {
                    // 子资源元素
                    self.parse_resource(&element);
                }
                _ => {}
            }
        }
    }

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

    /// 加载整个资源组（对应 C++ LoadResources）
    pub fn load_resources(&mut self, group: &str) -> bool {
        self.cur_res_group = group.to_string();
        let lower_group = string_to_lower(group);

        // 依次加载所有属于该组的图像资源
        let image_keys: Vec<String> = self.image_map.keys()
            .filter(|k| {
                if let Some(ptr) = self.image_map.get(*k) {
                    unsafe {
                        let res = &*(*ptr as *const ImageRes);
                        string_to_lower(&res.base.res_group) == lower_group
                    }
                } else { false }
            })
            .cloned()
            .collect();

        for key in &image_keys {
            if let Some(&ptr) = self.image_map.get(key) {
                unsafe {
                    let res = &mut *(ptr as *mut ImageRes);
                    if !res.base.path.is_empty() && res.image.unshared_image.is_null() && res.image.shared_image.is_null() {
                        self.load_single_image(res);
                    }
                }
            }
        }

        self.loaded_groups.insert(lower_group);
        true
    }

    /// 加载单个图像资源（公开方法，可按需调用）
    pub fn load_single_image(&mut self, res: &mut ImageRes) {
        use crate::framework::paklib::with_pak_interface;

        if res.base.path.is_empty() {
            eprintln!("    load_image: 路径为空");
            return;
        }

        // 检查是否是程序内建资源
        if res.base.from_program {
            return;
        }

        eprintln!("    load_image: id='{}' path='{}'", res.base.id, res.base.path);

        // 从 PAK 读取图像文件（尝试常见路径模式）
        let base_name = &res.base.path;
        let try_paths = [
            base_name.clone(),
            format!("{}.png", base_name),
            format!("{}.PNG", base_name),
            format!("{}.jpg", base_name),
            format!("{}.JPG", base_name),
            format!("images/{}.png", base_name),
            format!("IMAGES/{}.PNG", base_name),
            format!("images/{}.jpg", base_name),
            format!("IMAGES/{}.JPG", base_name),
        ];

        eprintln!("    load_image: 尝试 {} 种路径变体", try_paths.len());

        // 检查 PAK 记录中是否有匹配的
        with_pak_interface(|pak| {
            let normalized = crate::framework::paklib::normalize_pak_path(&try_paths[0]);
            let exists_no_ext = pak.records.contains_key(&normalized);
            let normalized_png = crate::framework::paklib::normalize_pak_path(&try_paths[1]);
            let exists_png = pak.records.contains_key(&normalized_png);
            eprintln!("    load_image: PAK 中 '{}' → {}  '{}' → {}", 
                normalized, exists_no_ext, normalized_png, exists_png);
            
            // 搜索包含 POPCAP 或 LOGO 的记录
            if try_paths[0].to_uppercase().contains("POPCAP") || try_paths[0].to_uppercase().contains("LOGO") || try_paths[0].to_uppercase().contains("TITLE") {
                eprintln!("    load_image: PAK 记录匹配搜索:");
                for (k, _v) in pak.records.iter() {
                    let ku = k.to_uppercase();
                    if ku.contains("POPCAP") || ku.contains("LOGO") || ku.contains("TITLE") {
                        if ku.len() < 30 {
                            eprintln!("      '{}'", k);
                        }
                    }
                }
            }
        });

        let mut image_data: Option<Vec<u8>> = None;
        let mut loaded_path = String::new();
        for p in &try_paths {
            let data = with_pak_interface(|pak| pak.load_file(p));
            if data.is_some() {
                eprintln!("    load_image: 从 PAK 读取 '{}' 成功", p);
                loaded_path = p.clone();
                image_data = data;
                break;
            }
        }

        let image_data = match image_data {
            Some(d) => d,
            None => {
                // 也尝试从文件系统加载（尝试带 .png 扩展名）
                eprintln!("    load_image: PAK 中未找到，尝试文件系统");
                let mut fs_result = None;
                for p in &try_paths {
                    if let Ok(d) = std::fs::read(p) {
                        loaded_path = p.clone();
                        fs_result = Some(d);
                        break;
                    }
                }
                match fs_result {
                    Some(d) => d,
                    None => {
                        eprintln!("    load_image: 文件系统也找不到");
                        return;
                    }
                }
            }
        };

        eprintln!("    load_image: 读取到 {} 字节，开始解码", image_data.len());

        // 用 image crate 解码
        let decoded = match image::load_from_memory(&image_data) {
            Ok(img) => {
                img.to_rgba8()
            }
            Err(_) => {
                return;
            }
        };

        let width = decoded.width() as i32;
        let height = decoded.height() as i32;
        let raw = decoded.as_raw();

        // 将 RGBA 字节数据转换为 u32 像素 (ARGB 格式)
        let mut pixels: Vec<u32> = raw.chunks(4).map(|chunk| {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3] as u32;
            (a << 24) | (r << 16) | (g << 8) | b
        }).collect();

        // ===== 查找并合成独立的 Alpha 遮罩（对应 C++ ImageLib::GetImage 逻辑）=====
        // C++ GetImage 会查找 `_filename` 或 `filename_`（同级目录下）作为 alpha 遮罩
        // 从实际加载路径中提取目录前缀，构建对应的 alpha 遮罩路径
        let base = &res.base.path;
        let base_lower = base.to_lowercase();
        let dir_prefix = if let Some(pos) = loaded_path.rfind(['/', '\\']) {
            &loaded_path[..=pos]
        } else {
            ""
        };
        let alpha_paths = [
            format!("{}_{}", dir_prefix, base_lower),
            format!("{}_{}.png", dir_prefix, base_lower),
            format!("{}_{}.jpg", dir_prefix, base_lower),
            format!("{}{}_", dir_prefix, base_lower),
            format!("{}{}_.png", dir_prefix, base_lower),
            format!("{}{}_.jpg", dir_prefix, base_lower),
        ];
        let mut alpha_data = None;
        for ap in &alpha_paths {
            let d = with_pak_interface(|pak| pak.load_file(ap));
            if d.is_some() {
                eprintln!("    load_image: 发现 alpha 遮罩 '{}'", ap);
                alpha_data = d;
                break;
            }
        }
        // 也检查文件系统
        if alpha_data.is_none() {
            for ap in &alpha_paths {
                if let Ok(d) = std::fs::read(ap) {
                    eprintln!("    load_image: 文件系统发现 alpha 遮罩 '{}'", ap);
                    alpha_data = Some(d);
                    break;
                }
            }
        }
        if let Some(alpha_bytes) = alpha_data {
            if let Ok(alpha_img) = image::load_from_memory(&alpha_bytes) {
                let alpha_rgba = alpha_img.to_rgba8();
                let aw = alpha_rgba.width() as i32;
                let ah = alpha_rgba.height() as i32;
                let a_raw = alpha_rgba.as_raw();
                // ComposeAlpha（对应 C++）：将遮罩的蓝色通道(B)复制为主图的 alpha 通道
                let min_w = width.min(aw);
                let min_h = height.min(ah);
                for y in 0..min_h {
                    for x in 0..min_w {
                        let src_idx = (y * aw + x) as usize;
                        let dst_idx = (y * width + x) as usize;
                        if dst_idx < pixels.len() && (src_idx * 4 + 2) < a_raw.len() {
                            let mask_b = a_raw[src_idx * 4 + 2] as u32; // B channel
                            pixels[dst_idx] = (pixels[dst_idx] & 0x00FFFFFF) | (mask_b << 24);
                        }
                    }
                }
            }
        }

        // 创建 MemoryImage
        let mut mem_image = Box::new(MemoryImage::new(width, height));
        mem_image.set_bits(&pixels, width, height, false);

        // === 去除透明像素的黑边（对应 C++ TodMarkImageForSanding / FixPixelsOnAlphaEdgeForBlending）===
        mem_image.fix_pixels_on_alpha_edge_for_blending();

        // 通过 GLInterface 创建纹理
        if let Some(app_ptr) = self.app {
            unsafe {
                let app = &*app_ptr;
                if let Some(gl_ptr) = app.gl_interface {
                    (*gl_ptr).create_image_texture(&mut mem_image);
                }
            }
        }

        // 存入 SharedImageRef（传递所有权）
        let mi_ptr = Box::into_raw(mem_image);
        res.image.set_unshared(mi_ptr);
    }

    /// 替换图像（对应 C++ ReplaceImage）
    pub fn replace_image(&mut self, _id: &str, _image: *mut Image) -> bool { false }

    /// 替换声音
    pub fn replace_sound(&mut self, _id: &str, _sound: isize) -> bool { false }

    /// 替换字体
    pub fn replace_font(&mut self, _id: &str, _font: *mut Font) -> bool { false }

    /// 删除图像资源（对应 C++ DeleteImage）
    /// 删除图像资源（对应 C++ DeleteImage）
    pub fn delete_image(&mut self, name: &str) {
        let lower = string_to_lower(name);
        if let Some(res_ptr) = self.image_map.remove(&lower) {
            unsafe {
                // 释放资源（SharedImageRef 在 drop 时会自动释放图像内存）
                let _ = Box::from_raw(res_ptr);
            }
        }
    }

    /// 加载图像（对应 C++ LoadImage）
    pub fn load_image(&mut self, _name: &str) -> SharedImageRef { SharedImageRef::new() }

    /// 删除声音资源
    pub fn delete_sound(&mut self, _name: &str) {}

    /// 删除字体资源
    pub fn delete_font(&mut self, _name: &str) {}

    /// 加载字体
    pub fn load_font(&mut self, name: &str) -> Option<*mut Font> {
        use crate::framework::desc_parser::DescParser;
        use crate::framework::graphics::image_font::{FontData, ImageFont};

        let lower = string_to_lower(name);
        if let Some(&res_ptr) = self.font_map.get(&lower) {
            unsafe {
                let res = &mut *(res_ptr as *mut FontRes);
                // 已加载则直接返回
                if let Some(fp) = res.font_ptr {
                    return Some(fp);
                }

                let path = &res.base.path;
                eprintln!("    load_font: id='{}' path='{}'", name, path);

                // 从 PAK 加载字体描述文件
                let desc_content = crate::framework::paklib::with_pak_interface(|pak| {
                    let try_paths = [
                        path.clone(),
                        format!("{}.txt", path),
                        format!("{}.font", path),
                    ];
                    for p in &try_paths {
                        if let Some(data) = pak.load_file(p) {
                            if let Ok(s) = String::from_utf8(data) {
                                return Some(s);
                            }
                        }
                    }
                    None
                });

                if let Some(content) = desc_content {
                    // 解析 FontData
                    let mut font_data = Box::new(FontData::new());
                    font_data.load_descriptor(&content);
                    let fd_ptr = Box::into_raw(font_data);

                    // 创建 ImageFont 并泄露（由 imagefont_owner 管理生命周期）
                    let imf = Box::new(ImageFont::from_file(fd_ptr, res.size));
                    let owner_ptr = Box::into_raw(imf) as *mut std::ffi::c_void;
                    // 用 Box::into_raw 后的稳定堆地址注册到全局表
                    // 这样 Font::draw_string 才能正确分派到 ImageFont::draw_string
                    unsafe {
                        let imf_ref = &*(owner_ptr as *mut ImageFont);
                        let font_ptr: *const Font = &imf_ref.base;
                        let imf_vptr: *const std::ffi::c_void = owner_ptr as *const std::ffi::c_void;
                        crate::framework::graphics::font::register_imagefont(font_ptr, imf_vptr);
                    }
                    let base_ptr: *mut Font = unsafe { &mut (*(owner_ptr as *mut ImageFont)).base };

                    // 存入 FontRes
                    res.font_ptr = Some(base_ptr);
                    res.imagefont_owner = Some(owner_ptr);

                    eprintln!("    load_font: '{}' 加载成功", name);
                    return Some(base_ptr);
                } else {
                    eprintln!("    load_font: '{}' 描述文件未找到", name);
                }
            }
        }
        None
    }

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
    pub fn get_font(&self, id: &str) -> Option<*mut Font> {
        let lower = string_to_lower(id);
        if let Some(&res_ptr) = self.font_map.get(&lower) {
            unsafe {
                let res = &*(res_ptr as *const FontRes);
                return res.font_ptr;
            }
        }
        None
    }

    /// 获取图像（抛出异常版本）
    pub fn get_image_throw(&self, the_id: &str) -> SharedImageRef {
        let r = self.get_image(the_id);
        r
    }

    /// 获取声音（抛出异常版本）
    pub fn get_sound_throw(&self, _id: &str) -> isize { 0 }

    /// 获取字体（抛出异常版本）
    pub fn get_font_throw(&self, id: &str) -> Option<*mut Font> {
        self.get_font(id)
    }

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
            unsafe {
                // 先释放 imagefont_owner（如果有）
                let font_res = &mut *(ptr as *mut FontRes);
                if let Some(owner) = font_res.imagefont_owner.take() {
                    let _ = Box::from_raw(owner);
                }
                let _ = Box::from_raw(ptr);
            }
        }
    }
}

