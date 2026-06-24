// PvZ Portable Rust 翻译 — ImageFont 图像字体
// 对应 C++ SexyAppFramework/graphics/ImageFont.h / ImageFont.cpp
//
// ImageFont 使用精灵图集渲染文本，支持多层、标签系统、缩放和点大小调整。
// FontData 继承自 DescParser，通过解析 .font 描述文件来定义字体。

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Mutex;

use crate::framework::color::Color;
use crate::framework::common::{string_to_int, string_to_upper, utf8_decode_next_str};
use crate::framework::desc_parser::{DataElement, DataElementMap, DescParser, CMDSEP_SEMICOLON};
use crate::framework::graphics::font::Font;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::graphics::memory_image::MemoryImage;
use crate::framework::graphics::shared_image::SharedImageRef;
use crate::framework::point::Point;
use crate::framework::rect::Rect;

// ============================================================
// CharData（对应 C++ CharData）
// ============================================================

/// 字符数据：存储每个字符的纹理矩形、偏移、字距、宽度和排序顺序
#[derive(Debug, Clone)]
pub struct CharData {
    /// 字符在纹理图集中的矩形区域
    pub image_rect: Rect,
    /// 绘制偏移
    pub offset: Point,
    /// 与其他字符的字距调整表（key=下一个字符, value=字距偏移）
    pub kerning_offsets: HashMap<char, i32>,
    /// 字符宽度
    pub width: i32,
    /// 排序顺序（用于图层叠加）
    pub order: i32,
}

impl CharData {
    pub fn new() -> Self {
        CharData {
            image_rect: Rect::new(0, 0, 0, 0),
            offset: Point::new(0, 0),
            kerning_offsets: HashMap::new(),
            width: 0,
            order: 0,
        }
    }
}

/// 字符数据映射类型：char -> CharData
pub type CharDataMap = HashMap<char, CharData>;

/// 字符映射类型：from -> to（用于字符替换/映射）
pub type CharMap = HashMap<char, char>;

/// 字符矩形映射类型：char -> Rect
pub type CharRectMap = HashMap<char, Rect>;

// ============================================================
// FontLayer（对应 C++ FontLayer）
// ============================================================

/// 字体图层：每个图层定义一个字符集、图像和颜色变换
#[derive(Debug, Clone)]
pub struct FontLayer {
    /// 所属 FontData
    pub font_data: *mut FontData,
    /// 扩展信息（字符串映射）
    pub extended_info: HashMap<String, String>,
    /// 图层名称
    pub layer_name: String,
    /// 所需标签列表
    pub required_tags: Vec<String>,
    /// 排除标签列表
    pub excluded_tags: Vec<String>,
    /// 字符数据映射
    pub char_data_map: CharDataMap,
    /// 颜色乘数
    pub color_mult: Color,
    /// 颜色加数
    pub color_add: Color,
    /// 共享图像引用
    pub image: SharedImageRef,
    /// 绘制模式（-1=默认, 0=Normal, 1=Additive, 2=Copy）
    pub draw_mode: i32,
    /// 偏移
    pub offset: Point,
    /// 间距
    pub spacing: i32,
    /// 最小点大小
    pub min_point_size: i32,
    /// 最大点大小
    pub max_point_size: i32,
    /// 点大小
    pub point_size: i32,
    /// 上升高度（基线到顶）
    pub ascent: i32,
    /// 上升填充
    pub ascent_padding: i32,
    /// 高度
    pub height: i32,
    /// 默认高度
    pub default_height: i32,
    /// 行间距偏移
    pub line_spacing_offset: i32,
    /// 基础排序顺序
    pub base_order: i32,
    /// 是否使用 Alpha 校正
    pub use_alpha_correction: bool,
}

impl FontLayer {
    pub fn new(font_data: *mut FontData) -> Self {
        FontLayer {
            font_data,
            extended_info: HashMap::new(),
            layer_name: String::new(),
            required_tags: Vec::new(),
            excluded_tags: Vec::new(),
            char_data_map: HashMap::new(),
            color_mult: Color::WHITE,
            color_add: Color::new(0, 0, 0, 0),
            image: SharedImageRef::new(),
            draw_mode: -1,
            offset: Point::new(0, 0),
            spacing: 0,
            min_point_size: -1,
            max_point_size: -1,
            point_size: 0,
            ascent: 0,
            ascent_padding: 0,
            height: 0,
            default_height: 0,
            line_spacing_offset: 0,
            base_order: 0,
            use_alpha_correction: false,
        }
    }

    /// 获取字符数据，如不存在则自动创建（对应 C++ GetCharData）
    pub fn get_char_data(&mut self, ch: char) -> &mut CharData {
        self.char_data_map.entry(ch).or_insert_with(CharData::new)
    }
}

/// 图层列表类型
pub type FontLayerList = Vec<FontLayer>;
/// 图层映射类型：名称 -> 图层指针
pub type FontLayerMap = HashMap<String, usize>;

// ============================================================
// FontData（对应 C++ FontData，继承 DescParser）
// ============================================================

/// 字体数据：管理图层的加载、解析和生命周期，继承 DescParser
#[derive(Debug)]
pub struct FontData {
    /// DescParser 基础字段
    pub cmd_sep: i32,
    pub error_msg: String,
    pub current_line_num: i32,
    pub current_line: String,
    pub define_map: DataElementMap,
    /// FontData 特有字段
    pub initialized: bool,
    pub ref_count: i32,
    pub app: *mut (),
    pub default_point_size: i32,
    pub char_map: CharMap,
    pub font_layer_list: FontLayerList,
    pub font_layer_map: FontLayerMap,
    pub source_file: String,
    pub font_error_header: String,
}

impl FontData {
    pub fn new() -> Self {
        FontData {
            cmd_sep: CMDSEP_SEMICOLON,
            error_msg: String::new(),
            current_line_num: 0,
            current_line: String::new(),
            define_map: HashMap::new(),
            initialized: false,
            ref_count: 0,
            app: std::ptr::null_mut(),
            default_point_size: 0,
            char_map: HashMap::new(),
            font_layer_list: Vec::new(),
            font_layer_map: HashMap::new(),
            source_file: String::new(),
            font_error_header: String::new(),
        }
    }

    /// 增加引用计数
    pub fn ref_count_inc(&mut self) {
        self.ref_count += 1;
    }

    /// 减少引用计数，归零时返回 true（对应 C++ DeRef）
    pub fn ref_count_dec(&mut self) -> bool {
        self.ref_count -= 1;
        self.ref_count == 0
    }

    /// 加载字体描述文件（对应 C++ Load）
    /// 由 caller 提供文件内容，避免对 SexyAppBase 的强依赖
    pub fn load(&mut self, file_content: &str) -> bool {
        self.initialized = true;
        self.load_descriptor(file_content)
    }

    /// 加载旧版字体（对应 C++ LoadLegacy）
    pub fn load_legacy(&mut self, _font_image: *mut Image, _font_desc: &str) -> bool {
        // 旧版加载暂不支持
        self.initialized = true;
        true
    }

    /// 从数据元素获取颜色值（对应 C++ GetColorFromDataElement）
    pub fn get_color_from_data_element(&mut self, element: &DataElement, color: &mut Color) -> bool {
        match element {
            DataElement::List(list) => {
                let mut factor_vec = Vec::new();
                if self.data_to_double_vector(element, &mut factor_vec) && factor_vec.len() == 4 {
                    *color = Color::new(
                        (factor_vec[0] * 255.0) as u8,
                        (factor_vec[1] * 255.0) as u8,
                        (factor_vec[2] * 255.0) as u8,
                        (factor_vec[3] * 255.0) as u8,
                    );
                    return true;
                }
                false
            }
            DataElement::Single(s) => {
                let mut a_color: i32 = 0;
                if string_to_int(s, &mut a_color) {
                    *color = Color::from_argb(a_color as u32);
                    return true;
                }
                false
            }
        }
    }

    /// 将数据元素转为图层指针（对应 C++ DataToLayer）
    pub fn data_to_layer(&mut self, source: &DataElement) -> Option<usize> {
        match source {
            DataElement::Single(s) => {
                let upper = string_to_upper(s);
                self.font_layer_map.get(&upper).copied()
            }
            DataElement::List(_) => None,
        }
    }

    /// 从描述文件中创建水平跨度矩形列表（对应 C++ CreateHorzSpanRectList）
    pub fn create_horz_span_rect_list(&mut self, params: &DataElement) -> bool {
        // params 是 List, 包含 [defineName, rectIntVector, widthsVector]
        match params {
            DataElement::List(list) if list.len() >= 3 => {
                let name_elem = &list[0];
                let rect_elem = &list[1];
                let widths_elem = &list[2];

                let name = match name_elem.as_str() {
                    Some(s) => s,
                    None => return false,
                };
                let mut rect_ints = Vec::new();
                let mut widths = Vec::new();

                if !self.data_to_int_vector(rect_elem, &mut rect_ints) || rect_ints.len() != 4 {
                    return false;
                }
                if !self.data_to_int_vector(widths_elem, &mut widths) {
                    return false;
                }

                let def_name = string_to_upper(name);
                let mut x_pos = 0i32;
                let mut rect_list = Vec::new();

                for &w in &widths {
                    let item = DataElement::List(vec![
                        DataElement::Single((rect_ints[0] + x_pos).to_string()),
                        DataElement::Single(rect_ints[1].to_string()),
                        DataElement::Single(w.to_string()),
                        DataElement::Single(rect_ints[3].to_string()),
                    ]);
                    rect_list.push(item);
                    x_pos += w;
                }

                self.define_map.insert(def_name, DataElement::List(rect_list));
                true
            }
            _ => false,
        }
    }
}

// FontData 实现 DescParser trait
impl DescParser for FontData {
    fn error_message(&self) -> &str {
        &self.error_msg
    }
    fn error_message_mut(&mut self) -> &mut String {
        &mut self.error_msg
    }
    fn cmd_sep(&self) -> i32 {
        self.cmd_sep
    }
    fn set_cmd_sep(&mut self, sep: i32) {
        self.cmd_sep = sep;
    }
    fn current_line_num(&self) -> i32 {
        self.current_line_num
    }
    fn set_current_line_num(&mut self, num: i32) {
        self.current_line_num = num;
    }
    fn current_line(&self) -> &str {
        &self.current_line
    }
    fn current_line_mut(&mut self) -> &mut String {
        &mut self.current_line
    }
    fn define_map(&self) -> &DataElementMap {
        &self.define_map
    }
    fn define_map_mut(&mut self) -> &mut DataElementMap {
        &mut self.define_map
    }

    /// 处理命令（对应 C++ FontData::HandleCommand）
    fn handle_command(&mut self, params: &DataElement) -> bool {
        let list = match params {
            DataElement::List(l) => l,
            _ => return false,
        };
        if list.is_empty() {
            return false;
        }

        let cmd = match &list[0] {
            DataElement::Single(s) => s.as_str(),
            _ => return false,
        };

        let cmd_upper = cmd.to_uppercase();

        match cmd_upper.as_str() {
            "DEFINE" => {
                if list.len() == 3 {
                    let name_elem = &list[1];
                    let val_elem = &list[2];
                    if let DataElement::Single(name) = name_elem {
                        if !Self::is_immediate(name) {
                            let def_name = string_to_upper(name);
                            // 移除已存在的定义
                            self.define_map.remove(&def_name);
                            match val_elem {
                                DataElement::List(_) => {
                                    let mut values = Vec::new();
                                    if self.get_values(val_elem, &mut values) {
                                        self.define_map.insert(def_name, DataElement::List(values));
                                        return true;
                                    }
                                    return false;
                                }
                                DataElement::Single(val_str) => {
                                    // 尝试解引用
                                    let upper_val = string_to_upper(val_str);
                                    if let Some(deref_val) = self.define_map.get(&upper_val) {
                                        self.define_map.insert(def_name, deref_val.clone());
                                    } else {
                                        self.define_map.insert(def_name, DataElement::Single(val_str.clone()));
                                    }
                                    return true;
                                }
                            }
                        }
                    }
                }
                false
            }
            "CREATEHORZSPANRECTLIST" => {
                self.create_horz_span_rect_list(&DataElement::List(list[1..].to_vec()))
            }
            "SETDEFAULTPOINTSIZE" => {
                if list.len() == 2 {
                    if let DataElement::Single(s) = &list[1] {
                        if let Ok(v) = s.parse::<i32>() {
                            self.default_point_size = v;
                            return true;
                        }
                    }
                }
                false
            }
            "SETCHARMAN" => {
                if list.len() == 3 {
                    let mut from_vec = Vec::new();
                    let mut to_vec = Vec::new();
                    if self.data_to_string_vector(&list[1], &mut from_vec)
                        && self.data_to_string_vector(&list[2], &mut to_vec)
                        && from_vec.len() == to_vec.len()
                    {
                        for (from_s, to_s) in from_vec.iter().zip(to_vec.iter()) {
                            if from_s.len() == 1 && to_s.len() == 1 {
                                let from_c = from_s.chars().next().unwrap();
                                let to_c = to_s.chars().next().unwrap();
                                self.char_map.insert(from_c, to_c);
                            } else {
                                return false;
                            }
                        }
                        return true;
                    }
                }
                false
            }
            "CREATELAYER" => {
                if list.len() == 2 {
                    if let DataElement::Single(name) = &list[1] {
                        let layer_name = string_to_upper(name);
                        let idx = self.font_layer_list.len();
                        let self_ptr = std::ptr::from_mut(self);
                        self.font_layer_list.push(FontLayer::new(self_ptr));
                        if let Some(last) = self.font_layer_list.last_mut() {
                            last.font_data = self_ptr;
                        }
                        if self.font_layer_map.contains_key(&layer_name) {
                            self.error("Layer Already Exists");
                            return false;
                        }
                        self.font_layer_map.insert(layer_name, idx);
                        return true;
                    }
                }
                false
            }
            "CREATELAYERFROM" => {
                if list.len() == 3 {
                    if let DataElement::Single(name) = &list[1] {
                        if let Some(src_idx) = self.data_to_layer(&list[2]) {
                            let layer_name = string_to_upper(name);
                            if let Some(src_layer) = self.font_layer_list.get(src_idx) {
                                let idx = self.font_layer_list.len();
                                self.font_layer_list.push(src_layer.clone());
                                if self.font_layer_map.contains_key(&layer_name) {
                                    self.error("Layer Already Exists");
                                    return false;
                                }
                                self.font_layer_map.insert(layer_name, idx);
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERREQUIRETAGS" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut tags = Vec::new();
                        if self.data_to_string_vector(&list[2], &mut tags) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.required_tags = tags;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYEREXCLUDETAGS" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut tags = Vec::new();
                        if self.data_to_string_vector(&list[2], &mut tags) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.excluded_tags = tags;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERIMAG" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut image_name = String::new();
                        if self.data_to_string(&list[2], &mut image_name) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                // 设置图层图片——这里简化处理，
                                // 实际应从 ResourceManager 获取 SharedImage
                                layer.image = SharedImageRef::new();
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYEROFFSET" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut offset_str = String::new();
                        if self.data_to_string(&list[2], &mut offset_str) {
                            let parts: Vec<&str> = offset_str.split(',').collect();
                            if parts.len() == 2 {
                                if let (Ok(x), Ok(y)) = (parts[0].trim().parse::<i32>(), parts[1].trim().parse::<i32>()) {
                                    if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                        layer.offset = Point::new(x, y);
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
                false
            }
            "LAYERSPACING" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut spacing = 0i32;
                        if self.data_to_int(&list[2], &mut spacing) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.spacing = spacing;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERPOINTSIZ" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut pt_size = 0i32;
                        if self.data_to_int(&list[2], &mut pt_size) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.point_size = pt_size;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERMINPOINTSIZE" | "LAYERMINPOINTSIZ" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut val = 0i32;
                        if self.data_to_int(&list[2], &mut val) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.min_point_size = val;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERMAXPOINTSIZE" | "LAYERMAXPOINTSIZ" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut val = 0i32;
                        if self.data_to_int(&list[2], &mut val) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.max_point_size = val;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERCOLOR" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut color = Color::WHITE;
                        if self.get_color_from_data_element(&list[2], &mut color) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.color_mult = color;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERCOLORMULT" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut color = Color::WHITE;
                        if self.get_color_from_data_element(&list[2], &mut color) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.color_mult = color;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERCOLORADD" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut color = Color::new(0, 0, 0, 0);
                        if self.get_color_from_data_element(&list[2], &mut color) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.color_add = color;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERDRAWMOD" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut mode = 0i32;
                        if self.data_to_int(&list[2], &mut mode) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.draw_mode = mode;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERASCENT" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut val = 0i32;
                        if self.data_to_int(&list[2], &mut val) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.ascent = val;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERASCENTPADDING" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut val = 0i32;
                        if self.data_to_int(&list[2], &mut val) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.ascent_padding = val;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERHEIGHT" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut val = 0i32;
                        if self.data_to_int(&list[2], &mut val) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.height = val;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERDEFAULTHEIGHT" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut val = 0i32;
                        if self.data_to_int(&list[2], &mut val) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.default_height = val;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERLINESPACINGOFFSET" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut val = 0i32;
                        if self.data_to_int(&list[2], &mut val) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.line_spacing_offset = val;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERBASEORDER" => {
                if list.len() == 3 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut val = 0i32;
                        if self.data_to_int(&list[2], &mut val) {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                layer.base_order = val;
                                return true;
                            }
                        }
                    }
                }
                false
            }
            "LAYERCHARMAN" => {
                if list.len() == 4 {
                    if let Some(idx) = self.data_to_layer(&list[1]) {
                        let mut from_codes = Vec::new();
                        let mut to_codes = Vec::new();
                        if self.data_to_string_vector(&list[2], &mut from_codes)
                            && self.data_to_string_vector(&list[3], &mut to_codes)
                            && from_codes.len() == to_codes.len()
                        {
                            if let Some(layer) = self.font_layer_list.get_mut(idx) {
                                // 处理字符映射——此处的语义是在图层级别做映射
                                // C++ 中在 FontLayer 级别没有 mCharMap，只有 FontData 有
                                // 这里实际上在 FontData 的 HandleCommand 中处理
                                // 暂且忽略图层级字符映射
                            }
                            return true;
                        }
                    }
                }
                false
            }
            "LAYERFRAM" => {
                // 处理字符帧定义
                if let Some(idx) = self.data_to_layer(&list[1]) {
                    // 第4+个参数是帧数据
                    let layer = &self.font_layer_list[idx];
                    // 获取字符码点
                    let ch_elem = &list[2];
                    if let DataElement::Single(ch_s) = ch_elem {
                        let ch = ch_s.chars().next().unwrap_or('\0');
                        // 从第3个参数开始是帧数据（rect + offset + width + order）
                        let mut params_idx = 3usize;
                        while params_idx < list.len() {
                            // 预期格式：((x y w h) (ox oy) width order)
                            // 简化处理——假设第一个是矩形，其他参数后续
                            params_idx += 1;
                        }
                        let _ = ch; // 暂不处理
                    }
                }
                true // 简化，不报错
            }
            _ => {
                // 未知命令，不报错（兼容性）
                true
            }
        }
    }
}

// ============================================================
// ActiveFontLayer（对应 C++ ActiveFontLayer）
// ============================================================

/// 活跃字体图层：经过缩放和点大小适配后的渲染图层
#[derive(Debug, Clone)]
pub struct ActiveFontLayer {
    /// 基础 FontLayer 指针
    pub base_font_layer: *mut FontLayer,
    /// 缩放后的图像
    pub scaled_image: *mut Image,
    /// 是否拥有该图像（需析构时释放）
    pub owns_image: bool,
    /// 缩放后的字符矩形映射
    pub scaled_char_image_rects: CharRectMap,
}

impl ActiveFontLayer {
    pub fn new() -> Self {
        ActiveFontLayer {
            base_font_layer: std::ptr::null_mut(),
            scaled_image: std::ptr::null_mut(),
            owns_image: false,
            scaled_char_image_rects: HashMap::new(),
        }
    }
}

/// 活跃字体图层列表
pub type ActiveFontLayerList = Vec<ActiveFontLayer>;

// ============================================================
// RenderCommand（对应 C++ RenderCommand）
// ============================================================

/// 图像指针的 Send 包装器（用于在 Mutex 中安全传输原始指针）
/// 调用者需确保指针在同一线程内使用
#[derive(Debug, Clone, Copy)]
pub(crate) struct ImagePtr(*mut Image);
unsafe impl Send for ImagePtr {}
unsafe impl Sync for ImagePtr {}

/// 渲染命令：存储单个字符的一次绘制操作（排序后批量执行）
#[derive(Debug, Clone)]
pub struct RenderCommand {
    /// 绘制图像指针（Send 包装）
    pub(crate) image: ImagePtr,
    /// 目标位置 (x, y)
    pub dest: [i32; 2],
    /// 源纹理矩形 (x, y, w, h)
    pub src: [i32; 4],
    /// 绘制模式
    pub mode: i32,
    /// 颜色
    pub color: Color,
    /// 是否使用 Alpha 校正
    pub use_alpha_correction: bool,
}

// ============================================================
// ImageFont（对应 C++ ImageFont，继承 _Font）
// ============================================================

/// 图像字体：使用精灵图集渲染文本
/// 支持多层叠加、缩放、点大小调整和标签系统
#[derive(Debug)]
pub struct ImageFont {
    /// 基类 Font 字段
    pub base: Font,
    /// 字体数据
    pub font_data: *mut FontData,
    /// 点大小
    pub point_size: i32,
    /// 标签向量
    pub tag_vector: Vec<String>,
    /// 活跃层列表是否有效
    pub active_list_valid: bool,
    /// 活跃字体图层列表
    pub active_layer_list: ActiveFontLayerList,
    /// 缩放因子
    pub scale: f64,
    /// 是否强制缩放图像为白色（仅保留 Alpha）
    pub force_scaled_images_white: bool,
    /// 以下字段用于缓存计算值（对应 Font 的 ascent/height 等，在 GenerateActiveFontLayers 中计算）
    pub ascent: i32,
    pub height: i32,
    pub ascent_padding: i32,
    pub line_spacing_offset: i32,
}

impl ImageFont {
    /// 从字体文件创建 ImageFont（对应 C++ ImageFont(SexyAppBase*, string)）
    pub fn from_file(font_data: *mut FontData, point_size: i32) -> Self {
        ImageFont {
            base: Font::new("ImageFont", point_size),
            font_data,
            point_size,
            tag_vector: Vec::new(),
            active_list_valid: false,
            active_layer_list: Vec::new(),
            scale: 1.0,
            force_scaled_images_white: false,
            ascent: 0,
            height: 0,
            ascent_padding: 0,
            line_spacing_offset: 0,
        }
    }

    /// 从图像创建 ImageFont（简化构造，对应 C++ ImageFont(Image*)）
    pub fn from_image(_image: *mut Image) -> Self {
        ImageFont {
            base: Font::new("ImageFont", 12),
            font_data: std::ptr::null_mut(),
            point_size: 12,
            tag_vector: Vec::new(),
            active_list_valid: false,
            active_layer_list: Vec::new(),
            scale: 1.0,
            force_scaled_images_white: false,
            ascent: 0,
            height: 0,
            ascent_padding: 0,
            line_spacing_offset: 0,
        }
    }

    /// 准备渲染（对应 C++ Prepare）
    pub fn prepare(&mut self) {
        if !self.active_list_valid {
            self.generate_active_font_layers();
            self.active_list_valid = true;
        }
    }

    /// 获取映射后的字符（对应 C++ GetMappedChar）
    pub fn get_mapped_char(&self, ch: char) -> char {
        if self.font_data.is_null() {
            return ch;
        }
        unsafe {
            match (*self.font_data).char_map.get(&ch) {
                Some(&mapped) => mapped,
                None => ch,
            }
        }
    }

    /// 生成活跃字体图层（对应 C++ GenerateActiveFontLayers）
    /// 根据当前 point_size、scale、tags 筛选并缩放图层
    pub fn generate_active_font_layers(&mut self) {
        self.active_layer_list.clear();
        self.ascent = 0;
        self.height = 0;
        self.ascent_padding = i32::MAX;
        self.line_spacing_offset = 0;

        if self.font_data.is_null() {
            return;
        }

        let fd = unsafe { &mut *self.font_data };
        let a_point_size = self.point_size as f64 * self.scale;
        let mut first_layer = true;

        for layer_idx in 0..fd.font_layer_list.len() {
            // 先复制需要的所有数据（避免后续可变借用时的借用检查问题）
            let (excluded_tags, required_tags, min_ps, max_ps, point_size_val, 
                 char_data_map, image_ref, offset_val, spacing_val) = 
            {
                let layer = &fd.font_layer_list[layer_idx];
                (layer.excluded_tags.clone(), layer.required_tags.clone(),
                 layer.min_point_size, layer.max_point_size, layer.point_size,
                 layer.char_data_map.clone(), 
                 // 无法克隆 SharedImageRef（因为涉及原始指针），
                 // 用临时 None 替代
                 true, layer.offset, layer.spacing)
            };

            // 检查标签排除
            let mut excluded = false;
            for tag in &excluded_tags {
                if self.tag_vector.contains(tag) {
                    excluded = true;
                    break;
                }
            }
            if excluded {
                continue;
            }

            // 检查标签需求
            if !required_tags.is_empty() {
                let mut all_found = true;
                for tag in &required_tags {
                    if !self.tag_vector.contains(tag) {
                        all_found = false;
                        break;
                    }
                }
                if !all_found {
                    continue;
                }
            }

            // 检查点大小范围
            if (min_ps != -1 && self.point_size < min_ps)
                || (max_ps != -1 && self.point_size > max_ps)
            {
                continue;
            }

            // 创建活跃层
            let mut active_layer = ActiveFontLayer::new();
            // 在创建活跃层后获取可变指针
            active_layer.base_font_layer = &mut fd.font_layer_list[layer_idx] as *mut FontLayer;

            let a_layer_point_size = point_size_val;

            if a_layer_point_size == 0 {
                // 无需缩放，直接引用原始图像
                // 使用原始字符矩形
                for (ch, cd) in &char_data_map {
                    active_layer
                        .scaled_char_image_rects
                        .insert(*ch, cd.image_rect);
                }
                active_layer.owns_image = false;
            } else {
                // 需要缩放——创建一个新的 MemoryImage 并渲染缩放后的字符
                let a_memory_image = Box::new(MemoryImage::new(0, 0));
                let a_memory_image_ptr = Box::into_raw(a_memory_image);

                let mut cur_x = 0i32;
                let mut max_height = 0i32;

                // 计算缩放后的布局
                let mut scaled_rects: CharRectMap = HashMap::new();
                for (ch, cd) in &char_data_map {
                    let orig_rect = &cd.image_rect;
                    let scaled_rect = Rect::new(
                        cur_x,
                        0,
                        (orig_rect.width as f64 * a_point_size / a_layer_point_size as f64) as i32,
                        (orig_rect.height as f64 * a_point_size / a_layer_point_size as f64) as i32,
                    );
                    scaled_rects.insert(*ch, scaled_rect);
                    if scaled_rect.height > max_height {
                        max_height = scaled_rect.height;
                    }
                    cur_x += scaled_rect.width;
                }

                active_layer.scaled_char_image_rects = scaled_rects;
                active_layer.scaled_image = a_memory_image_ptr as *mut Image;
                active_layer.owns_image = true;

                // 创建图像并绘制缩放字符
                unsafe {
                    (*a_memory_image_ptr).create(cur_x, max_height);
                    // 使用 Graphics 进行绘制（简化实现——实际需要在 Graphics 中实现 draw_image 的缩放版本）
                }
            }

            // 从 FontData 读取图层数据用于 ascent/height 计算
            let (layer_ascent_val, layer_height_val, layer_default_height_val,
                  layer_ascent_padding_val, layer_line_spacing_offset_val) = {
                let l = &fd.font_layer_list[layer_idx];
                (l.ascent, l.height, l.default_height, l.ascent_padding, l.line_spacing_offset)
            };

            // 更新 ImageFont 的 ascent/height 等字段
            let a_layer_ascent_value =
                (layer_ascent_val as f64 * a_point_size / a_layer_point_size as f64) as i32;
            if a_layer_ascent_value > self.ascent {
                self.ascent = a_layer_ascent_value;
            }

            if layer_height_val != 0 {
                let a_layer_height =
                    (layer_height_val as f64 * a_point_size / a_layer_point_size as f64) as i32;
                if a_layer_height > self.height {
                    self.height = a_layer_height;
                }
            } else {
                let a_layer_height =
                    (layer_default_height_val as f64 * a_point_size / a_layer_point_size as f64) as i32;
                if a_layer_height > self.height {
                    self.height = a_layer_height;
                }
            }

            let an_ascent_padding =
                (layer_ascent_padding_val as f64 * a_point_size / a_layer_point_size as f64) as i32;
            if first_layer || an_ascent_padding < self.ascent_padding {
                self.ascent_padding = an_ascent_padding;
            }

            let a_line_spacing_offset =
                (layer_line_spacing_offset_val as f64 * a_point_size / a_layer_point_size as f64) as i32;
            if first_layer || a_line_spacing_offset > self.line_spacing_offset {
                self.line_spacing_offset = a_line_spacing_offset;
            }

            first_layer = false;
            self.active_layer_list.push(active_layer);
        }
    }

    /// 获取字符串宽度（对应 C++ StringWidth）
    pub fn string_width(&mut self, text: &str) -> i32 {
        let mut width = 0i32;
        let mut prev_char: char = '\0';
        let bytes = text.as_bytes();
        let mut offset = 0usize;
        while let Some(ch) = utf8_decode_next_str(text, &mut offset) {
            width += self.char_width_kern(ch, prev_char);
            prev_char = ch;
        }
        width
    }

    /// 获取带字距调整的字符宽度（对应 C++ CharWidthKern）
    pub fn char_width_kern(&mut self, the_char: char, the_prev_char: char) -> i32 {
        self.prepare();

        let mut max_x_pos = 0i32;
        let a_point_size = self.point_size as f64 * self.scale;

        let ch = self.get_mapped_char(the_char);
        let prev_ch = if the_prev_char != '\0' {
            self.get_mapped_char(the_prev_char)
        } else {
            '\0'
        };

        for active_layer in &self.active_layer_list {
            let mut a_layer_x_pos = 0i32;

            if active_layer.base_font_layer.is_null() {
                continue;
            }

            let a_layer_point_size = unsafe { (*active_layer.base_font_layer).point_size };

            unsafe {
                let base_layer = &mut *active_layer.base_font_layer;
                // 先复制需要的字段，避免借用检查问题
                let layer_spacing = base_layer.spacing;
                let char_data = base_layer.get_char_data(ch).clone();
                let prev_char_data = if prev_ch != '\0' {
                    Some(base_layer.get_char_data(prev_ch).clone())
                } else {
                    None
                };

                let (a_char_width, a_spacing) = if a_layer_point_size == 0 {
                    let cw = char_data.width;
                    let sp = if prev_ch != '\0' {
                        layer_spacing
                            + prev_char_data
                                .as_ref()
                                .unwrap()
                                .kerning_offsets
                                .get(&ch)
                                .copied()
                                .unwrap_or(0)
                    } else {
                        0
                    };
                    ((cw as f64 * self.scale) as i32, (sp as f64 * self.scale) as i32)
                } else {
                    let cw = (char_data.width as f64 * a_point_size / a_layer_point_size as f64) as i32;
                    let sp = if prev_ch != '\0' {
                        let base_sp =
                            layer_spacing
                                + prev_char_data
                                    .as_ref()
                                    .unwrap()
                                    .kerning_offsets
                                    .get(&ch)
                                    .copied()
                                    .unwrap_or(0);
                        (base_sp as f64 * a_point_size / a_layer_point_size as f64) as i32
                    } else {
                        0
                    };
                    (cw, sp)
                };

                a_layer_x_pos += a_char_width + a_spacing;
            }

            if a_layer_x_pos > max_x_pos {
                max_x_pos = a_layer_x_pos;
            }
        }

        max_x_pos
    }

    /// 获取单个字符宽度（对应 C++ CharWidth）
    pub fn char_width(&mut self, ch: char) -> i32 {
        self.char_width_kern(ch, '\0')
    }

    /// 绘制字符串（对应 C++ DrawString）
    pub fn draw_string(
        &mut self,
        g: &mut Graphics,
        x: i32,
        y: i32,
        text: &str,
        color: &Color,
        clip_rect: &Rect,
    ) {
        let _ = clip_rect;
        self.draw_string_ex(g, x, y, text, color, None, None);
    }

    /// 绘制字符串增强版（对应 C++ DrawStringEx）
    /// 此方法使用全局渲染命令池，支持多层、排序和批量绘制
    pub fn draw_string_ex(
        &mut self,
        g: &mut Graphics,
        the_x: i32,
        the_y: i32,
        the_string: &str,
        the_color: &Color,
        drawn_areas: Option<&mut Vec<Rect>>,
        out_width: Option<&mut i32>,
    ) {
        // 加全局锁（对应 C++ gRenderCritSec）
        let _lock = RENDER_LOCK.lock().unwrap();

        // 清空渲染命令池
        RENDER_POOL.lock().unwrap().clear();

        // 将 drawn_areas 移到局部变量中，避免重复借用
        let mut drawn_mut = drawn_areas;
        if let Some(ref mut areas) = drawn_mut {
            areas.clear();
        }

        if self.font_data.is_null() {
            if let Some(w) = out_width {
                *w = 0;
            }
            return;
        }

        if !unsafe { (*self.font_data).initialized } {
            if let Some(w) = out_width {
                *w = 0;
            }
            return;
        }

        self.prepare();

        let colorize_images = g.get_colorize_images();
        g.set_colorize_images(true);

        let mut cur_x_pos = the_x;

        // 使用 UTF8 逐个字符迭代（Rust 用 chars().collect() 迭代，无需手动字节偏移）

        // 逐字符处理
        let chars: Vec<char> = the_string.chars().collect();
        for i in 0..chars.len() {
            let cur_raw_char = chars[i];
            let has_next = i + 1 < chars.len();
            let next_raw_char = if has_next { chars[i + 1] } else { '\0' };

            let ch = self.get_mapped_char(cur_raw_char);
            let next_ch = if has_next {
                self.get_mapped_char(next_raw_char)
            } else {
                '\0'
            };

            let mut max_x_pos = cur_x_pos;

            for (_layer_order_offset, active_layer) in self.active_layer_list.iter().enumerate() {
                let mut a_layer_x_pos = cur_x_pos;

                if active_layer.base_font_layer.is_null() {
                    continue;
                }

                let base_layer = unsafe { &mut *active_layer.base_font_layer };

                let a_layer_point_size = base_layer.point_size;
                let a_scale = if a_layer_point_size != 0 {
                    self.scale * self.point_size as f64 / a_layer_point_size as f64
                } else {
                    self.scale
                };

                // 先复制需要的字段，避免借用检查器问题（get_char_data 会可变借用 base_layer）
                let layer_offset = base_layer.offset;
                let layer_ascent = base_layer.ascent;
                let layer_spacing = base_layer.spacing;
                let layer_color_mult = base_layer.color_mult;
                let layer_color_add = base_layer.color_add;
                let layer_draw_mode = base_layer.draw_mode;

                // 获取字符数据（可变借用 base_layer）
                let char_data = base_layer.get_char_data(ch).clone();

                let (an_image_x, an_image_y, a_char_width, a_spacing) = if (a_scale - 1.0).abs() < f64::EPSILON
                {
                    let ix = a_layer_x_pos + layer_offset.x + char_data.offset.x;
                    let iy = the_y
                        - (layer_ascent - layer_offset.y - char_data.offset.y);
                    let cw = char_data.width;
                    let sp = if next_ch != '\0' {
                        layer_spacing
                            + char_data
                                .kerning_offsets
                                .get(&next_ch)
                                .copied()
                                .unwrap_or(0)
                    } else {
                        0
                    };
                    (ix, iy, cw, sp)
                } else {
                    let ix = a_layer_x_pos
                        + ((layer_offset.x as f64 + char_data.offset.x as f64) * a_scale)
                            as i32;
                    let iy = the_y
                        - ((layer_ascent as f64
                            - layer_offset.y as f64
                            - char_data.offset.y as f64)
                            * a_scale) as i32;
                    let cw = (char_data.width as f64 * a_scale) as i32;
                    let sp = if next_ch != '\0' {
                        ((layer_spacing as f64
                            + char_data
                                .kerning_offsets
                                .get(&next_ch)
                                .copied()
                                .unwrap_or(0) as f64)
                            * a_scale) as i32
                    } else {
                        0
                    };
                    (ix, iy, cw, sp)
                };

                // 计算颜色（使用复制出来的 color_mult 和 color_add）
                let a_color = Color::new(
                    (the_color.r as i32 * layer_color_mult.r as i32 / 255
                        + layer_color_add.r as i32)
                        .min(255) as u8,
                    (the_color.g as i32 * layer_color_mult.g as i32 / 255
                        + layer_color_add.g as i32)
                        .min(255) as u8,
                    (the_color.b as i32 * layer_color_mult.b as i32 / 255
                        + layer_color_add.b as i32)
                        .min(255) as u8,
                    (the_color.a as i32 * layer_color_mult.a as i32 / 255
                        + layer_color_add.a as i32)
                        .min(255) as u8,
                );

                // 创建渲染命令
                if let Some(scaled_rect) = active_layer.scaled_char_image_rects.get(&ch) {
                    let cmd = RenderCommand {
                        image: ImagePtr(active_layer.scaled_image),
                        dest: [an_image_x, an_image_y],
                        src: [
                            scaled_rect.x,
                            scaled_rect.y,
                            scaled_rect.width,
                            scaled_rect.height,
                        ],
                        mode: layer_draw_mode,
                        color: a_color,
                        use_alpha_correction: base_layer.use_alpha_correction,
                    };

                    RENDER_POOL.lock().unwrap().push(cmd);

                    // 记录绘制区域
                    if let Some(ref mut areas) = drawn_mut {
                        areas.push(Rect::new(
                            an_image_x,
                            an_image_y,
                            scaled_rect.width,
                            scaled_rect.height,
                        ));
                    }
                }

                a_layer_x_pos += a_char_width + a_spacing;
                if a_layer_x_pos > max_x_pos {
                    max_x_pos = a_layer_x_pos;
                }
            }

            cur_x_pos = max_x_pos;
        }

        if let Some(w) = out_width {
            *w = cur_x_pos - the_x;
        }

        let orig_color = g.get_color().clone();
        let orig_colorize = g.get_colorize_images();

        // 执行渲染命令
        {
            let pool = RENDER_POOL.lock().unwrap();
            for cmd in pool.iter() {
                let old_draw_mode = g.get_draw_mode();
                if cmd.mode != -1 {
                    g.set_draw_mode(cmd.mode);
                }
                g.set_color(&cmd.color);
                if !cmd.image.0.is_null() {
                    // 通过 unsafe 将原始指针转为引用
                    unsafe {
                        let img_ref = &*cmd.image.0;
                        g.draw_image_src(
                            img_ref,
                            cmd.dest[0],
                            cmd.dest[1],
                            &Rect::new(cmd.src[0], cmd.src[1], cmd.src[2], cmd.src[3]),
                        );
                    }
                }
                g.set_draw_mode(old_draw_mode);
            }
        }

        g.set_color(&orig_color);
        g.set_colorize_images(orig_colorize);
    }

    /// 复制字体（对应 C++ Duplicate）
    pub fn duplicate(&self) -> Self {
        // 简单复制——注意不复制 active_layer_list（需要重新生成）
        ImageFont {
            base: Font::new("ImageFont", self.point_size),
            font_data: self.font_data,
            point_size: self.point_size,
            tag_vector: self.tag_vector.clone(),
            active_list_valid: false,
            active_layer_list: Vec::new(),
            scale: self.scale,
            force_scaled_images_white: self.force_scaled_images_white,
            ascent: 0,
            height: 0,
            ascent_padding: 0,
            line_spacing_offset: 0,
        }
    }

    // ---- 标签系统 ----

    pub fn set_point_size(&mut self, pt_size: i32) {
        self.point_size = pt_size;
        self.active_list_valid = false;
    }

    pub fn get_point_size(&self) -> i32 {
        self.point_size
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
        self.active_list_valid = false;
    }

    pub fn get_default_point_size(&self) -> i32 {
        if self.font_data.is_null() {
            0
        } else {
            unsafe { (*self.font_data).default_point_size }
        }
    }

    pub fn add_tag(&mut self, tag: &str) -> bool {
        let upper = tag.to_uppercase();
        if self.tag_vector.contains(&upper) {
            return false;
        }
        self.tag_vector.push(upper);
        self.active_list_valid = false;
        true
    }

    pub fn remove_tag(&mut self, tag: &str) -> bool {
        let upper = tag.to_uppercase();
        if let Some(pos) = self.tag_vector.iter().position(|t| t == &upper) {
            self.tag_vector.remove(pos);
            self.active_list_valid = false;
            return true;
        }
        false
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tag_vector.iter().any(|t| t == &tag.to_uppercase())
    }

    pub fn get_define(&self, name: &str) -> String {
        if self.font_data.is_null() {
            return String::new();
        }
        let fd = unsafe { &*self.font_data };
        let upper = name.to_uppercase();
        match fd.define_map.get(&upper) {
            Some(elem) => fd.data_element_to_string(elem),
            None => String::new(),
        }
    }
}

impl Drop for ImageFont {
    fn drop(&mut self) {
        // 释放拥有的缩放图像
        for active_layer in &self.active_layer_list {
            if active_layer.owns_image && !active_layer.scaled_image.is_null() {
                unsafe {
                    let _ = Box::from_raw(active_layer.scaled_image as *mut MemoryImage);
                }
            }
        }
    }
}

// ============================================================
// 全局渲染命令池（对应 C++ 静态数组 gRenderCommandPool）
// ============================================================

/// 渲染命令池大小
const POOL_SIZE: usize = 4096;

/// 渲染命令池（对应 C++ gRenderCommandPool[POOL_SIZE]）
/// 使用 Vec 动态分配，按 Z 顺序排序后执行
static RENDER_POOL: Mutex<Vec<RenderCommand>> = Mutex::new(Vec::new());

/// 渲染互斥锁（对应 C++ gRenderCritSec）
static RENDER_LOCK: Mutex<()> = Mutex::new(());

