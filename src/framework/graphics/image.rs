// PvZ Portable Rust 翻译 — 图像基类
// 对应 C++ SexyAppFramework/graphics/Image.h / Image.cpp
// 
// 此文件包含 Image 基类的数据结构定义以及其虚方法的默认实现。
// MemoryImage 和 GLImage 会通过类型标记分发覆盖部分方法。

#![allow(dead_code)]

use std::any::Any;
use crate::framework::color::Color;
use crate::framework::point::Point;
use crate::framework::rect::Rect;
use crate::framework::graphics::graphics::Graphics;

/// 扫描线跨度（用于多边形填充）
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Span {
    pub y: i32,
    pub x: i32,
    pub width: i32,
}

/// 动画类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AnimType {
    None = 0,
    Once = 1,
    PingPong = 2,
    Loop = 3,
}

/// 动画信息
#[derive(Debug, Clone)]
pub struct AnimInfo {
    pub anim_type: AnimType,
    pub frame_delay: i32,      // 1/100s
    pub num_cels: i32,
    pub per_frame_delay: Vec<i32>,
    pub frame_map: Vec<i32>,
    pub total_anim_time: i32,
}

impl AnimInfo {
    pub fn new() -> Self {
        AnimInfo {
            anim_type: AnimType::None,
            frame_delay: 0,
            num_cels: 0,
            per_frame_delay: Vec::new(),
            frame_map: Vec::new(),
            total_anim_time: 0,
        }
    }

    pub fn set_per_frame_delay(&mut self, the_frame: i32, the_time: i32) {
        if the_frame >= self.per_frame_delay.len() as i32 {
            self.per_frame_delay.resize(the_frame as usize + 1, 0);
        }
        self.per_frame_delay[the_frame as usize] = the_time;
    }

    /// 计算动画总时间和帧映射
    pub fn compute(&mut self, the_num_cels: i32, the_begin_frame_time: i32, the_end_frame_time: i32) {
        self.num_cels = the_num_cels;
        let mut a_total = 0i32;
        self.frame_map.clear();

        for i in 0..the_num_cels {
            let a_frame_delay = if i < self.per_frame_delay.len() as i32 {
                self.per_frame_delay[i as usize]
            } else {
                self.frame_delay
            };
            // 跳过延迟为0的帧
            if a_frame_delay > 0 {
                a_total += a_frame_delay;
                self.frame_map.push(i);
            }
        }

        if a_total == 0 {
            // 没有有效的帧延迟，使用默认
            for i in 0..the_num_cels {
                a_total += self.frame_delay;
                self.frame_map.push(i);
            }
        }

        self.total_anim_time = a_total + the_begin_frame_time + the_end_frame_time;
    }

    /// 获取指定时间点应播放的帧
    pub fn get_per_frame_cel(&self, the_time: i32) -> i32 {
        if self.frame_map.is_empty() {
            return 0;
        }

        let mut a_time = the_time;
        for &frame in &self.frame_map {
            let delay = if frame < self.per_frame_delay.len() as i32 {
                self.per_frame_delay[frame as usize]
            } else {
                self.frame_delay
            };
            if a_time < delay {
                return frame;
            }
            a_time -= delay;
        }
        *self.frame_map.last().unwrap_or(&0)
    }

    /// 获取指定时间点的帧（考虑循环模式）
    pub fn get_cel(&self, the_time: i32) -> i32 {
        if self.total_anim_time == 0 || self.frame_map.is_empty() {
            return 0;
        }

        match self.anim_type {
            AnimType::Once => {
                let t = the_time.min(self.total_anim_time - 1);
                self.get_per_frame_cel(t)
            }
            AnimType::PingPong => {
                let t = the_time % (self.total_anim_time * 2);
                if t < self.total_anim_time {
                    self.get_per_frame_cel(t)
                } else {
                    self.get_per_frame_cel(self.total_anim_time * 2 - t - 1)
                }
            }
            AnimType::Loop | _ => {
                let t = if self.total_anim_time > 0 {
                    the_time % self.total_anim_time
                } else {
                    0
                };
                self.get_per_frame_cel(t)
            }
        }
    }
}

/// 图像类型标记，用于分发虚方法调用
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ImageKind {
    Image = 0,
    MemoryImage = 1,
    GLImage = 2,
}

/// 图像基类
/// 注意：C++ 中的虚方法通过 image_kind 字段和 match 分发模拟
#[derive(Debug)]
pub struct Image {
    pub width: i32,
    pub height: i32,
    /// 图像的宽度对齐（用于像素对齐）
    pub width_align: i32,
    pub x: f64,
    pub y: f64,
    /// 是否已提交到显卡
    pub committed_bits: bool,

    // 图像条带（用于精灵图集）
    pub num_rows: i32,
    pub num_cols: i32,

    // 动画信息
    pub anim_info: Option<Box<AnimInfo>>,

    // 文件路径
    pub file_path: String,

    // 类型标记
    pub(crate) image_kind: ImageKind,

    // 像素数据（仅 MemoryImage 使用）
    // 保存在这里以便基类方法可以操作
    pub(crate) pixels: Vec<u8>,
    /// 像素格式：0=RGBA8888, 1=RGB565, 2=RGB555
    pub(crate) pixel_format: i32,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        let pixel_count = (width * height * 4).max(0) as usize;
        Image {
            width,
            height,
            width_align: 4,
            x: 0.0,
            y: 0.0,
            committed_bits: false,
            num_rows: 1,
            num_cols: 1,
            anim_info: None,
            file_path: String::new(),
            image_kind: ImageKind::Image,
            pixels: vec![0u8; pixel_count],
            pixel_format: 0,
        }
    }

    pub fn get_width(&self) -> i32 { self.width }
    pub fn get_height(&self) -> i32 { self.height }
    pub fn get_x(&self) -> f64 { self.x }
    pub fn get_y(&self) -> f64 { self.y }

    pub fn set_x(&mut self, x: f64) { self.x = x; }
    pub fn set_y(&mut self, y: f64) { self.y = y; }
    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// 获取图像的单帧宽度
    pub fn get_cel_width(&self) -> i32 {
        self.width / self.num_cols
    }

    /// 获取图像的单帧高度
    pub fn get_cel_height(&self) -> i32 {
        self.height / self.num_rows
    }

    /// 根据动画信息获取当前帧
    pub fn get_anim_cel(&self, the_time: i32) -> i32 {
        match &self.anim_info {
            Some(info) => info.get_cel(the_time),
            None => 0,
        }
    }

    /// 获取动画当前帧的矩形区域
    pub fn get_anim_cel_rect(&self, the_time: i32) -> Rect {
        self.get_cel_rect_by_index(self.get_anim_cel(the_time))
    }

    /// 根据帧索引获取矩形区域
    pub fn get_cel_rect_by_index(&self, the_cel: i32) -> Rect {
        let col = the_cel % self.num_cols;
        let row = the_cel / self.num_cols;
        self.get_cel_rect(col, row)
    }

    /// 获取指定行列的帧矩形
    pub fn get_cel_rect(&self, col: i32, row: i32) -> Rect {
        let cw = self.get_cel_width();
        let ch = self.get_cel_height();
        Rect::new(col * cw, row * ch, cw, ch)
    }

    /// 复制属性
    pub fn copy_attributes(&mut self, from: &Image) {
        self.width = from.width;
        self.height = from.height;
        self.width_align = from.width_align;
        self.num_rows = from.num_rows;
        self.num_cols = from.num_cols;
        // 注意: anim_info 所有权
        self.x = from.x;
        self.y = from.y;
    }

    /// 获取图像的逻辑矩形区域
    pub fn get_rect(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    /// 向下转型为具体类型（用于 MemoryImage 等子类）
    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    // ====================================================================
    // 以下方法对应 C++ Image 的虚方法
    // 在 Rust 中通过 image_kind 分发到不同实现
    // ====================================================================

    /// 3D 多边形填充（由 GLImage 加速）
    pub fn poly_fill_3d(
        &self,
        _vertices: &[Point],
        _clip_rect: &Rect,
        _color: &Color,
        _draw_mode: i32,
        _tx: i32,
        _ty: i32,
    ) -> bool {
        false // 默认返回 false，表示未处理
    }

    /// 填充矩形
    pub fn fill_rect(&mut self, rect: &Rect, color: &Color, draw_mode: i32) {
        self.fill_rect_impl(rect, color, draw_mode);
    }

    fn fill_rect_impl(&mut self, rect: &Rect, color: &Color, draw_mode: i32) {
        if self.pixels.is_empty() || rect.width <= 0 || rect.height <= 0 {
            return;
        }
        let x_start = rect.x.max(0);
        let y_start = rect.y.max(0);
        let x_end = (rect.x + rect.width).min(self.width);
        let y_end = (rect.y + rect.height).min(self.height);

        if x_start >= x_end || y_start >= y_end {
            return;
        }

        for y in y_start..y_end {
            for x in x_start..x_end {
                self.set_pixel(x, y, color, draw_mode);
            }
        }
    }

    /// 绘制矩形边框
    pub fn draw_rect(&mut self, rect: &Rect, color: &Color, draw_mode: i32) {
        let r = *rect;
        // 上边
        self.fill_rect_impl(&Rect::new(r.x, r.y, r.width + 1, 1), color, draw_mode);
        // 下边
        self.fill_rect_impl(&Rect::new(r.x, r.y + r.height, r.width + 1, 1), color, draw_mode);
        // 左边
        self.fill_rect_impl(&Rect::new(r.x, r.y + 1, 1, r.height - 1), color, draw_mode);
        // 右边
        self.fill_rect_impl(&Rect::new(r.x + r.width, r.y + 1, 1, r.height - 1), color, draw_mode);
    }

    /// 清除矩形区域（设为全透明）
    pub fn clear_rect(&mut self, rect: &Rect) {
        if self.pixels.is_empty() || rect.width <= 0 || rect.height <= 0 {
            return;
        }
        let x_start = rect.x.max(0);
        let y_start = rect.y.max(0);
        let x_end = (rect.x + rect.width).min(self.width);
        let y_end = (rect.y + rect.height).min(self.height);

        for y in y_start..y_end {
            for x in x_start..x_end {
                let idx = ((y * self.width + x) * 4) as usize;
                if idx + 3 < self.pixels.len() {
                    self.pixels[idx] = 0;     // R
                    self.pixels[idx + 1] = 0; // G
                    self.pixels[idx + 2] = 0; // B
                    self.pixels[idx + 3] = 0; // A
                }
            }
        }
    }

    /// 绘制线段
    pub fn draw_line(&mut self, start_x: f64, start_y: f64, end_x: f64, end_y: f64, color: &Color, draw_mode: i32) {
        // Bresenham 线算法
        let mut x0 = start_x.round() as i32;
        let mut y0 = start_y.round() as i32;
        let x1 = end_x.round() as i32;
        let y1 = end_y.round() as i32;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && x0 < self.width && y0 >= 0 && y0 < self.height {
                self.set_pixel(x0, y0, color, draw_mode);
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// 绘制抗锯齿线段（简化版，使用 Bresenham）
    pub fn draw_line_aa(&mut self, start_x: f64, start_y: f64, end_x: f64, end_y: f64, color: &Color, draw_mode: i32) {
        // 使用 Wu 算法简化版——目前使用普通 Bresenham
        self.draw_line(start_x, start_y, end_x, end_y, color, draw_mode);
    }

    /// 填充扫描线
    pub fn fill_scan_lines(&mut self, spans: &[Span], color: &Color, draw_mode: i32) {
        for span in spans {
            let x = span.x;
            let y = span.y;
            let w = span.width;
            if w > 0 {
                self.fill_rect_impl(&Rect::new(x, y, w, 1), color, draw_mode);
            }
        }
    }

    /// 带覆盖率的扫描线填充（抗锯齿多边形）
    pub fn fill_scan_lines_with_coverage(
        &mut self,
        spans: &[Span],
        color: &Color,
        draw_mode: i32,
        coverage: &[u8],
        cover_x: i32,
        cover_y: i32,
        cover_width: i32,
        _cover_height: i32,
    ) {
        for span in spans {
            for x in span.x..span.x + span.width {
                let cx = x - cover_x;
                let cy = span.y - cover_y;
                if cx >= 0 && cy >= 0 && cx < cover_width {
                    let cov_idx = (cy * cover_width + cx) as usize;
                    if cov_idx < coverage.len() {
                        let cov = coverage[cov_idx];
                        if cov > 0 {
                            let mut blended = *color;
                            let a = (color.a as u32 * cov as u32 / 255) as u8;
                            blended.a = a;
                            self.set_pixel(x, span.y, &blended, draw_mode);
                        }
                    }
                }
            }
        }
    }

    /// 块传输（Blt）— 从源图像复制区域到目标
    pub fn blt(&mut self, src: &Image, dest_x: i32, dest_y: i32, src_rect: &Rect, color: &Color, draw_mode: i32, _linear_filter: bool) {
        if src.pixels.is_empty() || self.pixels.is_empty() {
            return;
        }
        let sx = src_rect.x;
        let sy = src_rect.y;
        let sw = src_rect.width;
        let sh = src_rect.height;

        for y in 0..sh {
            for x in 0..sw {
                let src_idx = (((sy + y) * src.width + (sx + x)) * 4) as usize;
                let dst_x = dest_x + x;
                let dst_y = dest_y + y;
                if dst_x < 0 || dst_x >= self.width || dst_y < 0 || dst_y >= self.height {
                    continue;
                }
                if src_idx + 3 >= src.pixels.len() {
                    continue;
                }
                let src_color = Color::new(
                    src.pixels[src_idx],
                    src.pixels[src_idx + 1],
                    src.pixels[src_idx + 2],
                    src.pixels[src_idx + 3],
                );
                // 用 color 参数调色
                let final_color = if *color != Color::WHITE || draw_mode != 0 {
                    self.blend_color(&src_color, color, draw_mode)
                } else {
                    src_color
                };
                self.set_pixel(dst_x, dst_y, &final_color, 0);
            }
        }
    }

    /// 浮点位置 Blt
    pub fn blt_f(&mut self, src: &Image, dest_x: f32, dest_y: f32, src_rect: &Rect, _clip_rect: &Rect, color: &Color, draw_mode: i32) {
        // 四舍五入到整数坐标后调用 blt
        let ix = dest_x.round() as i32;
        let iy = dest_y.round() as i32;
        self.blt(src, ix, iy, src_rect, color, draw_mode, false);
    }

    /// 旋转 Blt
    pub fn blt_rotated(
        &mut self,
        src: &Image,
        dest_x: f32,
        dest_y: f32,
        src_rect: &Rect,
        _clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        _rot: f64,
        _rot_center_x: f32,
        _rot_center_y: f32,
    ) {
        // 简化：忽略旋转，直接 blt
        // 完整的旋转实现在 GLImage 中使用 OpenGL 完成
        self.blt(src, dest_x as i32, dest_y as i32, src_rect, color, draw_mode, false);
    }

    /// 拉伸 Blt
    pub fn stretch_blt(&mut self, src: &Image, dest_rect: &Rect, src_rect: &Rect, _clip_rect: &Rect, color: &Color, draw_mode: i32, _fast_stretch: bool) {
        if src.pixels.is_empty() || self.pixels.is_empty() {
            return;
        }
        if dest_rect.width <= 0 || dest_rect.height <= 0 || src_rect.width <= 0 || src_rect.height <= 0 {
            return;
        }

        let scale_x = src_rect.width as f64 / dest_rect.width as f64;
        let scale_y = src_rect.height as f64 / dest_rect.height as f64;

        for dy in 0..dest_rect.height {
            for dx in 0..dest_rect.width {
                let sx = src_rect.x + (dx as f64 * scale_x) as i32;
                let sy = src_rect.y + (dy as f64 * scale_y) as i32;
                let src_idx = ((sy * src.width + sx) * 4) as usize;
                let dst_x = dest_rect.x + dx;
                let dst_y = dest_rect.y + dy;
                if dst_x < 0 || dst_x >= self.width || dst_y < 0 || dst_y >= self.height {
                    continue;
                }
                if src_idx + 3 >= src.pixels.len() {
                    continue;
                }
                let src_color = Color::new(
                    src.pixels[src_idx],
                    src.pixels[src_idx + 1],
                    src.pixels[src_idx + 2],
                    src.pixels[src_idx + 3],
                );
                let final_color = if *color != Color::WHITE || draw_mode != 0 {
                    self.blend_color(&src_color, color, draw_mode)
                } else {
                    src_color
                };
                self.set_pixel(dst_x, dst_y, &final_color, 0);
            }
        }
    }

    /// 矩阵变换 Blt
    pub fn blt_matrix(
        &mut self,
        _src: &Image,
        _x: f32,
        _y: f32,
        _matrix: &crate::framework::sexy_matrix::SexyMatrix3,
        _clip_rect: &Rect,
        _color: &Color,
        _draw_mode: i32,
        _src_rect: &Rect,
        _blend: bool,
    ) {
        // 完整实现在 GLImage 中
        // 软件渲染路径这里简化为不处理
    }

    /// 三角纹理 Blt
    pub fn blt_triangles_tex(
        &mut self,
        _texture: &Image,
        _vertices: &[[crate::framework::graphics::gl_interface::TriVertex; 3]],
        _num_triangles: i32,
        _clip_rect: &Rect,
        _color: &Color,
        _draw_mode: i32,
        _tx: f32,
        _ty: f32,
        _blend: bool,
    ) {
        // 完整实现在 GLImage 中
    }

    /// 镜像 Blt
    pub fn blt_mirror(&mut self, src: &Image, dest_x: i32, dest_y: i32, src_rect: &Rect, color: &Color, draw_mode: i32, _linear_filter: bool) {
        if src.pixels.is_empty() || self.pixels.is_empty() {
            return;
        }
        let sx = src_rect.x;
        let sy = src_rect.y;
        let sw = src_rect.width;
        let sh = src_rect.height;

        for y in 0..sh {
            for x in 0..sw {
                // 水平镜像：源 x 方向反转
                let src_x = sx + sw - 1 - x;
                let src_idx = (((sy + y) * src.width + src_x) * 4) as usize;
                let dst_x = dest_x + x;
                let dst_y = dest_y + y;
                if dst_x < 0 || dst_x >= self.width || dst_y < 0 || dst_y >= self.height {
                    continue;
                }
                if src_idx + 3 >= src.pixels.len() {
                    continue;
                }
                let src_color = Color::new(
                    src.pixels[src_idx],
                    src.pixels[src_idx + 1],
                    src.pixels[src_idx + 2],
                    src.pixels[src_idx + 3],
                );
                let final_color = if *color != Color::WHITE || draw_mode != 0 {
                    self.blend_color(&src_color, color, draw_mode)
                } else {
                    src_color
                };
                self.set_pixel(dst_x, dst_y, &final_color, 0);
            }
        }
    }

    /// 拉伸镜像 Blt
    pub fn stretch_blt_mirror(&mut self, src: &Image, dest_rect: &Rect, src_rect: &Rect, _clip_rect: &Rect, color: &Color, draw_mode: i32, _fast_stretch: bool) {
        if src.pixels.is_empty() || self.pixels.is_empty() {
            return;
        }
        if dest_rect.width <= 0 || dest_rect.height <= 0 || src_rect.width <= 0 || src_rect.height <= 0 {
            return;
        }

        let scale_x = src_rect.width as f64 / dest_rect.width as f64;
        let scale_y = src_rect.height as f64 / dest_rect.height as f64;

        for dy in 0..dest_rect.height {
            for dx in 0..dest_rect.width {
                // 水平镜像
                let sx = src_rect.x + src_rect.width - 1 - (dx as f64 * scale_x) as i32;
                let sy = src_rect.y + (dy as f64 * scale_y) as i32;
                let src_idx = ((sy * src.width + sx) * 4) as usize;
                let dst_x = dest_rect.x + dx;
                let dst_y = dest_rect.y + dy;
                if dst_x < 0 || dst_x >= self.width || dst_y < 0 || dst_y >= self.height {
                    continue;
                }
                if src_idx + 3 >= src.pixels.len() {
                    continue;
                }
                let src_color = Color::new(
                    src.pixels[src_idx],
                    src.pixels[src_idx + 1],
                    src.pixels[src_idx + 2],
                    src.pixels[src_idx + 3],
                );
                let final_color = if *color != Color::WHITE || draw_mode != 0 {
                    self.blend_color(&src_color, color, draw_mode)
                } else {
                    src_color
                };
                self.set_pixel(dst_x, dst_y, &final_color, 0);
            }
        }
    }

    // ====================================================================
    // 辅助方法
    // ====================================================================

    /// 设置单个像素（带混合模式处理）
    fn set_pixel(&mut self, x: i32, y: i32, color: &Color, draw_mode: i32) {
        let idx = ((y * self.width + x) * 4) as usize;
        if idx + 3 >= self.pixels.len() {
            return;
        }
        match draw_mode {
            0 => {
                // 普通模式（alpha 混合）
                let dst_a = self.pixels[idx + 3] as u32;
                if dst_a == 0 {
                    // 目标透明，直接写入
                    self.pixels[idx] = color.r;
                    self.pixels[idx + 1] = color.g;
                    self.pixels[idx + 2] = color.b;
                    self.pixels[idx + 3] = color.a;
                } else if color.a == 255 {
                    // 源不透明，直接覆盖
                    self.pixels[idx] = color.r;
                    self.pixels[idx + 1] = color.g;
                    self.pixels[idx + 2] = color.b;
                    self.pixels[idx + 3] = 255;
                } else if color.a > 0 {
                    if self.pixel_format == 0 {
                        // RGBA8888 alpha blend
                        let sa = color.a as u32;
                        let da = dst_a;
                        let out_a = sa + da - (sa * da / 255);
                        if out_a > 0 {
                            let r = ((color.r as u32 * sa + self.pixels[idx] as u32 * (255 - sa)) / 255) as u8;
                            let g = ((color.g as u32 * sa + self.pixels[idx + 1] as u32 * (255 - sa)) / 255) as u8;
                            let b = ((color.b as u32 * sa + self.pixels[idx + 2] as u32 * (255 - sa)) / 255) as u8;
                            self.pixels[idx] = r;
                            self.pixels[idx + 1] = g;
                            self.pixels[idx + 2] = b;
                            self.pixels[idx + 3] = out_a as u8;
                        }
                    }
                }
            }
            1 => {
                // 叠加模式（DRAWMODE_ADDITIVE）
                if self.pixel_format == 0 {
                    let r = (self.pixels[idx] as u32 + color.r as u32).min(255) as u8;
                    let g = (self.pixels[idx + 1] as u32 + color.g as u32).min(255) as u8;
                    let b = (self.pixels[idx + 2] as u32 + color.b as u32).min(255) as u8;
                    let a = (self.pixels[idx + 3] as u32 + color.a as u32).min(255) as u8;
                    self.pixels[idx] = r;
                    self.pixels[idx + 1] = g;
                    self.pixels[idx + 2] = b;
                    self.pixels[idx + 3] = a;
                }
            }
            _ => {
                // 默认：直接写入
                self.pixels[idx] = color.r;
                self.pixels[idx + 1] = color.g;
                self.pixels[idx + 2] = color.b;
                self.pixels[idx + 3] = color.a;
            }
        }
    }

    /// 颜色混合（用于 blt 调色）
    fn blend_color(&self, src: &Color, tint: &Color, _draw_mode: i32) -> Color {
        // 将色调颜色与源颜色相乘
        let r = (src.r as u32 * tint.r as u32 / 255) as u8;
        let g = (src.g as u32 * tint.g as u32 / 255) as u8;
        let b = (src.b as u32 * tint.b as u32 / 255) as u8;
        let a = (src.a as u32 * tint.a as u32 / 255) as u8;
        Color::new(r, g, b, a)
    }

    /// 获取指定位置的像素颜色
    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        let idx = ((y * self.width + x) * 4) as usize;
        if idx + 3 < self.pixels.len() {
            Color::new(
                self.pixels[idx],
                self.pixels[idx + 1],
                self.pixels[idx + 2],
                self.pixels[idx + 3],
            )
        } else {
            Color::TRANSPARENT
        }
    }

    /// 设置原始像素数据
    pub fn set_pixel_data(&mut self, data: &[u8], format: i32) {
        self.pixel_format = format;
        let expected = (self.width * self.height * 4) as usize;
        if data.len() >= expected {
            self.pixels[..expected].copy_from_slice(&data[..expected]);
        } else {
            self.pixels[..data.len()].copy_from_slice(data);
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Image::new(0, 0)
    }
}
