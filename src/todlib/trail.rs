// PvZ Portable Rust 翻译 — Trail（轨迹特效系统）
// 对应 C++ src/Sexy.TodLib/Trail.h / Trail.cpp

#![allow(dead_code)]

use std::f32::consts::PI;

use crate::framework::color::Color;
use crate::framework::common::SexyVector2;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::todlib::data_array::DataArray;
use crate::todlib::tod_particle::FloatParameterTrack;
use crate::todlib::tod_common::{clamp_int, rand_range_float};

/// 最大轨迹三角形数量
const MAX_TRAIL_TRIANGLES: usize = 38;

/// 轨迹类型
pub type TrailType = i32;
pub const TRAIL_NONE: TrailType = -1;
pub const TRAIL_ICE: TrailType = 0;
pub const NUM_TRAILS: TrailType = 1;

/// 轨迹动画轨道类型
pub type TrailTracks = i32;
pub const TRACK_WIDTH_OVER_LENGTH: TrailTracks = 0;
pub const TRACK_WIDTH_OVER_TIME: TrailTracks = 1;
pub const TRACK_ALPHA_OVER_LENGTH: TrailTracks = 2;
pub const TRACK_ALPHA_OVER_TIME: TrailTracks = 3;
pub const NUM_TRAIL_TRACKS: TrailTracks = 4;

/// 轨迹标志
pub type TrailFlags = i32;
pub const TRAIL_FLAG_LOOPS: TrailFlags = 0;

/// 轨迹参数（定义文件）
#[derive(Debug)]
pub struct TrailParams {
    pub m_trail_type: TrailType,
    pub m_trail_file_name: &'static str,
}

impl TrailParams {
    pub const fn new(trail_type: TrailType, file_name: &'static str) -> Self {
        TrailParams {
            m_trail_type: trail_type,
            m_trail_file_name: file_name,
        }
    }
}

/// 轨迹定义
#[derive(Debug)]
pub struct TrailDefinition {
    pub m_image: *mut Image,
    pub m_max_points: i32,
    pub m_min_point_distance: f32,
    pub m_trail_flags: i32,
    pub m_trail_duration: FloatParameterTrack,
    pub m_width_over_length: FloatParameterTrack,
    pub m_width_over_time: FloatParameterTrack,
    pub m_alpha_over_length: FloatParameterTrack,
    pub m_alpha_over_time: FloatParameterTrack,
}

impl TrailDefinition {
    pub fn new() -> Self {
        TrailDefinition {
            m_image: std::ptr::null_mut(),
            m_max_points: 2,
            m_min_point_distance: 1.0,
            m_trail_flags: 0,
            m_trail_duration: FloatParameterTrack::new(),
            m_width_over_length: FloatParameterTrack::new(),
            m_width_over_time: FloatParameterTrack::new(),
            m_alpha_over_length: FloatParameterTrack::new(),
            m_alpha_over_time: FloatParameterTrack::new(),
        }
    }
}

impl Default for TrailDefinition {
    fn default() -> Self {
        Self::new()
    }
}

/// 轨迹点
#[derive(Debug, Clone, Copy)]
pub struct TrailPoint {
    pub a_pos: SexyVector2,
}

impl TrailPoint {
    pub fn new() -> Self {
        TrailPoint {
            a_pos: SexyVector2::ZERO,
        }
    }
}

impl Default for TrailPoint {
    fn default() -> Self {
        Self::new()
    }
}

/// 轨迹对象
#[derive(Debug)]
pub struct Trail {
    pub m_trail_points: [TrailPoint; 20],
    pub m_num_trail_points: i32,
    pub m_dead: bool,
    pub m_render_order: i32,
    pub m_trail_age: i32,
    pub m_trail_duration: i32,
    pub m_definition: *mut TrailDefinition,
    pub m_trail_holder: *mut TrailHolder,
    pub m_trail_interp: [f32; 4],
    pub m_trail_center: SexyVector2,
    pub m_is_attachment: bool,
    pub m_color_override: Color,
}

impl Trail {
    pub fn new() -> Self {
        Trail {
            m_trail_points: [TrailPoint::new(); 20],
            m_num_trail_points: 0,
            m_dead: false,
            m_render_order: 0,
            m_trail_age: 0,
            m_trail_duration: 0,
            m_definition: std::ptr::null_mut(),
            m_trail_holder: std::ptr::null_mut(),
            m_trail_interp: [
                rand_range_float(0.0, 1.0),
                rand_range_float(0.0, 1.0),
                rand_range_float(0.0, 1.0),
                rand_range_float(0.0, 1.0),
            ],
            m_trail_center: SexyVector2::ZERO,
            m_is_attachment: false,
            m_color_override: Color::new(255, 255, 255, 255),
        }
    }

    /// 添加轨迹点
    pub fn add_point(&mut self, x: f32, y: f32) {
        let def = unsafe { &*self.m_definition };
        let max_points = clamp_int(def.m_max_points, 2, 20);

        if self.m_num_trail_points > 0 {
            let last = &self.m_trail_points[(self.m_num_trail_points - 1) as usize];
            let dx = x - last.a_pos.x;
            let dy = y - last.a_pos.y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < def.m_min_point_distance {
                return;
            }
        }

        if self.m_num_trail_points == max_points {
            // 左移丢弃最早的点
            for i in 1..max_points as usize {
                self.m_trail_points[i - 1] = self.m_trail_points[i];
            }
            self.m_num_trail_points -= 1;
        }

        let idx = self.m_num_trail_points as usize;
        self.m_trail_points[idx].a_pos.x = x;
        self.m_trail_points[idx].a_pos.y = y;
        self.m_num_trail_points += 1;
    }

    /// 更新轨迹
    pub fn update(&mut self) {
        self.m_trail_age += 1;
        if self.m_trail_age >= self.m_trail_duration {
            let def = unsafe { &*self.m_definition };
            if (def.m_trail_flags & (1 << TRAIL_FLAG_LOOPS)) != 0 {
                self.m_trail_age = 0;
            } else {
                self.m_dead = true;
            }
        }
    }

    /// 获取指定点的法线方向
    pub fn get_normal_at_point(&self, n_index: i32, the_normal: &mut SexyVector2) -> bool {
        let direction;
        if n_index == 0 {
            let to_next = self.m_trail_points[(n_index + 1) as usize].a_pos
                - self.m_trail_points[n_index as usize].a_pos;
            direction = SexyVector2::new(-to_next.y, to_next.x); // Perp()
        } else if n_index == self.m_num_trail_points - 1 {
            let from_prev = self.m_trail_points[n_index as usize].a_pos
                - self.m_trail_points[(n_index - 1) as usize].a_pos;
            direction = SexyVector2::new(-from_prev.y, from_prev.x); // Perp()
        } else {
            let to_next = self.m_trail_points[(n_index + 1) as usize].a_pos
                - self.m_trail_points[n_index as usize].a_pos;
            let to_prev = self.m_trail_points[(n_index - 1) as usize].a_pos
                - self.m_trail_points[n_index as usize].a_pos;
            let nn = normalize(to_next);
            let np = normalize(to_prev);
            direction = nn + np;
        }

        let mag = (direction.x * direction.x + direction.y * direction.y).sqrt();
        if approx_zero(mag) {
            return false;
        }

        the_normal.x = direction.x / mag;
        the_normal.y = direction.y / mag;
        true
    }

    /// 绘制轨迹
    pub fn draw(&self, g: &mut Graphics) {
        if self.m_dead || self.m_num_trail_points < 2 {
            return;
        }

        let def = unsafe { &*self.m_definition };
        let time_value = self.m_trail_age as f32 / (self.m_trail_duration - 1).max(1) as f32;
        let triangle_count = (self.m_num_trail_points - 1) * 2;
        // assert!(triangle_count < MAX_TRAIL_TRIANGLES as i32);

        let mut have_prev = false;
        let mut normal_prev = SexyVector2::ZERO;

        // 简化的绘制：使用 Graphics 提供的线段绘制
        for i in 0..self.m_num_trail_points - 1 {
            if !have_prev {
                if !self.get_normal_at_point(i, &mut normal_prev) {
                    continue;
                }
                have_prev = true;
            }

            let normal_cur = normal_prev;
            let mut normal_next = SexyVector2::ZERO;
            if !self.get_normal_at_point(i + 1, &mut normal_next) {
                normal_next = normal_prev;
            } else {
                normal_prev = normal_next;
            }

            let u_cur = 1.0 - i as f32 / (self.m_num_trail_points - 1).max(1) as f32;
            let u_next = 1.0 - (i + 1) as f32 / (self.m_num_trail_points - 1).max(1) as f32;

            let width_len_cur = def.m_width_over_length.evaluate(u_cur, self.m_trail_interp[TRACK_WIDTH_OVER_LENGTH as usize]);
            let width_len_next = def.m_width_over_length.evaluate(u_next, self.m_trail_interp[TRACK_WIDTH_OVER_LENGTH as usize]);
            let width_time_cur = def.m_width_over_time.evaluate(time_value, self.m_trail_interp[TRACK_WIDTH_OVER_TIME as usize]);
            let width_time_next = def.m_width_over_time.evaluate(time_value, self.m_trail_interp[TRACK_WIDTH_OVER_TIME as usize]);
            let alpha_len_cur = def.m_alpha_over_length.evaluate(u_cur, self.m_trail_interp[TRACK_ALPHA_OVER_LENGTH as usize]);
            let alpha_len_next = def.m_alpha_over_length.evaluate(u_next, self.m_trail_interp[TRACK_ALPHA_OVER_LENGTH as usize]);
            let alpha_time_cur = def.m_alpha_over_time.evaluate(time_value, self.m_trail_interp[TRACK_ALPHA_OVER_TIME as usize]);
            let alpha_time_next = def.m_alpha_over_time.evaluate(time_value, self.m_trail_interp[TRACK_ALPHA_OVER_TIME as usize]);

            let alpha_cur = clamp_int(
                (alpha_len_cur * alpha_time_cur * self.m_color_override.a as f32) as i32,
                0, 255,
            );
            let alpha_next = clamp_int(
                (alpha_len_next * alpha_time_next * self.m_color_override.a as f32) as i32,
                0, 255,
            );

            let color_cur = Color::new(
                self.m_color_override.r,
                self.m_color_override.g,
                self.m_color_override.b,
                alpha_cur as u8,
            );
            let color_next = Color::new(
                self.m_color_override.r,
                self.m_color_override.g,
                self.m_color_override.b,
                alpha_next as u8,
            );

            let p_cur = &self.m_trail_points[i as usize];
            let p_next = self.m_trail_points[(i + 1) as usize];

            let w_cur = width_len_cur * width_time_cur;
            let w_next = width_len_next * width_time_next;

            let x0 = self.m_trail_center.x + p_cur.a_pos.x + normal_cur.x * w_cur;
            let y0 = self.m_trail_center.y + p_cur.a_pos.y + normal_cur.y * w_cur;
            let x1 = self.m_trail_center.x + p_cur.a_pos.x - normal_cur.x * w_cur;
            let y1 = self.m_trail_center.y + p_cur.a_pos.y - normal_cur.y * w_cur;
            let x2 = self.m_trail_center.x + p_next.a_pos.x + normal_next.x * w_next;
            let y2 = self.m_trail_center.y + p_next.a_pos.y + normal_next.y * w_next;
            let x3 = self.m_trail_center.x + p_next.a_pos.x - normal_next.x * w_next;
            let y3 = self.m_trail_center.y + p_next.a_pos.y - normal_next.y * w_next;

            // C++ Trail::Draw 的三角形带（Trail.cpp:216-262）：每段两个三角形，顶点色 alpha 渐变
            g.set_color(&Color::WHITE); // 顶点色已含 alpha，避免 Graphics 调制色叠加
            let color_cur_argb = ((color_cur.a as u32) << 24)
                | ((color_cur.r as u32) << 16)
                | ((color_cur.g as u32) << 8)
                | (color_cur.b as u32);
            let color_next_argb = ((color_next.a as u32) << 24)
                | ((color_next.r as u32) << 16)
                | ((color_next.g as u32) << 8)
                | (color_next.b as u32);
            let a_verts = [
                [
                    crate::framework::graphics::gl_interface::TriVertex { x: x0, y: y0, u: 0.0, v: 1.0, color: color_cur_argb },
                    crate::framework::graphics::gl_interface::TriVertex { x: x1, y: y1, u: 0.0, v: 0.0, color: color_cur_argb },
                    crate::framework::graphics::gl_interface::TriVertex { x: x2, y: y2, u: 0.0, v: 1.0, color: color_next_argb },
                ],
                [
                    crate::framework::graphics::gl_interface::TriVertex { x: x2, y: y2, u: 0.0, v: 1.0, color: color_next_argb },
                    crate::framework::graphics::gl_interface::TriVertex { x: x1, y: y1, u: 0.0, v: 0.0, color: color_cur_argb },
                    crate::framework::graphics::gl_interface::TriVertex { x: x3, y: y3, u: 0.0, v: 0.0, color: color_next_argb },
                ],
            ];
            g.draw_triangles_flat(&a_verts, 2);
        }
    }
}

impl Default for Trail {
    fn default() -> Self {
        Self::new()
    }
}

/// 轨迹持有者
pub struct TrailHolder {
    pub m_trails: DataArray<Trail>,
}

impl TrailHolder {
    pub fn new() -> Self {
        TrailHolder {
            m_trails: DataArray::new(),
        }
    }

    pub fn initialize_holder(&mut self) {
        self.m_trails.initialize(1024u32, "trails");
    }

    pub fn dispose_holder(&mut self) {
        self.m_trails.dispose();
    }

    pub fn alloc_trail(&mut self, _render_order: i32, trail_type: TrailType) -> *mut Trail {
        // TODO: 使用全局轨迹定义数组 gTrailDefArray
        self.alloc_trail_from_def(_render_order, std::ptr::null_mut())
    }

    pub fn alloc_trail_from_def(&mut self, _render_order: i32, _definition: *mut TrailDefinition) -> *mut Trail {
        if self.m_trails.len() == self.m_trails.max_size() {
            return std::ptr::null_mut();
        }

        let trail = self.m_trails.alloc();
        if !trail.is_null() {
            unsafe {
                (*trail).m_trail_holder = self;
                (*trail).m_definition = _definition;
                let duration_interp = rand_range_float(0.0, 1.0);
                if !_definition.is_null() {
                    (*trail).m_trail_duration =
                        (*_definition).m_trail_duration.evaluate(0.0, duration_interp) as i32;
                }
            }
        }
        trail
    }
}

impl Default for TrailHolder {
    fn default() -> Self {
        Self::new()
    }
}

// ======== 全局数据 ========

/// 全局轨迹定义数量
pub static mut G_TRAIL_DEF_COUNT: i32 = 0;
/// 全局轨迹定义数组
pub static mut G_TRAIL_DEF_ARRAY: *mut TrailDefinition = std::ptr::null_mut();
/// 全局轨迹参数数量
pub static mut G_TRAIL_PARAM_ARRAY_SIZE: i32 = 0;
/// 全局轨迹参数数组
pub static mut G_TRAIL_PARAM_ARRAY: *mut TrailParams = std::ptr::null_mut();

/// 草坪轨迹参数数组
pub static mut G_LAWN_TRAIL_ARRAY: [TrailParams; 1] = [TrailParams::new(TRAIL_ICE, "particles/IceTrail.trail")];

// ======== 自由函数 ========

/// 加载单条轨迹定义（对应 C++ TrailLoadADef，Trail.cpp:45：
/// DefinitionLoadXML + FloatTrackSetDefault 默认值阶段）
pub fn trail_load_a_def(
    trail_def: &mut TrailDefinition,
    trail_file_name: &str,
) -> bool {
    // 对应 C++ DefinitionLoadXML(theTrailFileName, &gTrailDefMap, theTrailDef)
    let xml_data = crate::framework::paklib::with_pak_interface(|pak| pak.load_file(trail_file_name));
    let xml = match xml_data {
        Some(data) => String::from_utf8_lossy(&data).to_string(),
        None => return false,
    };
    let nodes = crate::todlib::xml_parser::parse_fragment(&xml);
    // 对应 C++ gTrailDefFields（Definition.cpp:44-55）
    for node in &nodes {
        let field_name = node.name.as_str();
        let text = node.text.trim().to_string();
        match field_name {
            "Image" => {
                let a_idx = crate::todlib::reanim_loader::resolve_reanim_image_name(&text);
                trail_def.m_image = crate::todlib::reanim_loader::reanimator_get_image(a_idx)
                    .unwrap_or(std::ptr::null_mut());
            }
            "MaxPoints" => trail_def.m_max_points = text.parse().unwrap_or(2),
            "MinPointDistance" => trail_def.m_min_point_distance = text.parse().unwrap_or(0.0),
            "TrailFlags" => {
                // 对应 C++ DT_FLAGS（gTrailFlagDefSymbols：Loops=0）
                let a_flag: f32 = text.parse().unwrap_or(0.0);
                if a_flag != 0.0 {
                    trail_def.m_trail_flags |= 1 << 0;
                }
            }
            "WidthOverLength" => {
                if let Some(t) = crate::todlib::tod_particle::parse_float_track(&text) {
                    trail_def.m_width_over_length = t;
                }
            }
            "WidthOverTime" => {
                if let Some(t) = crate::todlib::tod_particle::parse_float_track(&text) {
                    trail_def.m_width_over_time = t;
                }
            }
            "AlphaOverLength" => {
                if let Some(t) = crate::todlib::tod_particle::parse_float_track(&text) {
                    trail_def.m_alpha_over_length = t;
                }
            }
            "AlphaOverTime" => {
                if let Some(t) = crate::todlib::tod_particle::parse_float_track(&text) {
                    trail_def.m_alpha_over_time = t;
                }
            }
            "TrailDuration" => {
                if let Some(t) = crate::todlib::tod_particle::parse_float_track(&text) {
                    trail_def.m_trail_duration = t;
                }
            }
            _ => {}
        }
    }
    // 对应 C++ FloatTrackSetDefault（Trail.cpp:48-53）
    trail_def.m_width_over_length.set_default(1.0);
    trail_def.m_width_over_time.set_default(1.0);
    trail_def.m_trail_duration.set_default(100.0);
    trail_def.m_alpha_over_length.set_default(1.0);
    trail_def.m_alpha_over_time.set_default(1.0);
    true
}

/// 批量加载轨迹定义（对应 C++ TrailLoadDefinitions，Trail.cpp:56）
pub fn trail_load_definitions(params: &mut [TrailParams]) {
    unsafe {
        // C++: PVZP_ASSERT(!gTrailParamArray && !gTrailDefArray)
        if !G_TRAIL_PARAM_ARRAY.is_null() || !G_TRAIL_DEF_ARRAY.is_null() {
            return;
        }
        G_TRAIL_PARAM_ARRAY_SIZE = params.len() as i32;
        G_TRAIL_PARAM_ARRAY = params.as_mut_ptr();
        G_TRAIL_DEF_COUNT = params.len() as i32;
        let mut a_def_array: Vec<TrailDefinition> = Vec::with_capacity(params.len());
        for _ in 0..params.len() {
            a_def_array.push(TrailDefinition::new());
        }
        for (i, a_param) in params.iter().enumerate() {
            // C++: PVZP_ASSERT(aTrailParams->mTrailType == static_cast<TrailType>(i))
            debug_assert_eq!(a_param.m_trail_type as i32, i as i32);
            if !trail_load_a_def(&mut a_def_array[i], a_param.m_trail_file_name) {
                // C++: PvzpErrorMessageBox——Rust 以 eprintln 近似
                eprintln!("Failed to load trail '{}'", a_param.m_trail_file_name);
            }
        }
        G_TRAIL_DEF_ARRAY = a_def_array.as_mut_ptr();
        // 防析构：所有权移交全局指针，由 trail_free_definitions 重建回收
        std::mem::forget(a_def_array);
    }
}

/// 释放轨迹定义（对应 C++ TrailFreeDefinitions，Trail.cpp:70）
pub fn trail_free_definitions() {
    unsafe {
        // 对应 C++ TrailFreeDefinitions：DefinitionFreeMap 释放 XML map；
        // Rust 侧回收原始数组（XML 定义由 trail_load_a_def 解析后随数组释放）
        if !G_TRAIL_DEF_ARRAY.is_null() {
            let count = G_TRAIL_DEF_COUNT as usize;
            let _ = Vec::from_raw_parts(G_TRAIL_DEF_ARRAY, count, count);
        }
        G_TRAIL_DEF_ARRAY = std::ptr::null_mut();
        G_TRAIL_DEF_COUNT = 0;
        G_TRAIL_PARAM_ARRAY = std::ptr::null_mut();
        G_TRAIL_PARAM_ARRAY_SIZE = 0;
    }
}

// ======== 内部辅助函数 ========

fn normalize(v: SexyVector2) -> SexyVector2 {
    let mag = (v.x * v.x + v.y * v.y).sqrt();
    if mag > 0.0 {
        SexyVector2::new(v.x / mag, v.y / mag)
    } else {
        SexyVector2::ZERO
    }
}

fn approx_zero(v: f32) -> bool {
    v.abs() < 0.0001
}

#[cfg(test)]
mod trail_tests {
    use super::*;

    #[test]
    fn test_trail_load_ice() {
        crate::framework::paklib::init_pak_interface();
        crate::framework::paklib::with_pak_interface_mut(|pak| { pak.add_pak_file("main.pak"); });
        let mut def = TrailDefinition::new();
        let ok = trail_load_a_def(&mut def, "particles/IceTrail.trail");
        assert!(ok, "IceTrail.trail 应加载成功");
        // XML 加载成功：WidthOverLength 由 XML 定义（节点数 >= 1）
        assert!(!def.m_width_over_length.nodes.is_empty());
        // 未显式定义的 track 得到默认值（m_trail_duration 默认 100.0）
        if !def.m_trail_duration.nodes.is_empty() {
            assert_eq!(def.m_trail_duration.nodes[0].low_value, 100.0);
        }
        println!("TRAIL OK: max_points={}", def.m_max_points);
    }
}
