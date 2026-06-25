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

            // 使用 Graphics 绘制两个三角形
            g.set_color(&color_cur);
            g.fill_rect_xywh(
                (x0.min(x1).min(x2).min(x3)) as i32,
                (y0.min(y1).min(y2).min(y3)) as i32,
                (x0.max(x1).max(x2).max(x3) - x0.min(x1).min(x2).min(x3)) as i32,
                (y0.max(y1).max(y2).max(y3) - y0.min(y1).min(y2).min(y3)) as i32,
            );
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
pub static G_LAWN_TRAIL_ARRAY: [TrailParams; 1] = [TrailParams::new(TRAIL_ICE, "particles/IceTrail.trail")];

// ======== 自由函数 ========

/// 加载单条轨迹定义
pub fn trail_load_a_def(
    _trail_def: &mut TrailDefinition,
    _trail_file_name: &str,
) -> bool {
    // TODO: 使用 Definition 模块加载 XML
    true
}

/// 批量加载轨迹定义
pub fn trail_load_definitions(_params: &mut [TrailParams]) {
    // TODO: 实现全局数组的加载逻辑
}

/// 释放轨迹定义
pub fn trail_free_definitions() {
    // TODO: 释放全局轨迹定义数组
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
