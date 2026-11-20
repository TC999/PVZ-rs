// PvZ Portable Rust 翻译 — 图像基类
// 对应 C++ SexyAppFramework/graphics/Image.h / Image.cpp
//
// 此文件包含 Image 基类的数据结构定义以及其虚方法的默认实现。
// MemoryImage 和 GLImage 会通过类型标记分发覆盖部分方法。

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::point::Point;
use crate::framework::rect::Rect;
use std::any::Any;

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
    pub frame_delay: i32, // 1/100s
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
    pub fn compute(
        &mut self,
        the_num_cels: i32,
        the_begin_frame_time: i32,
        the_end_frame_time: i32,
    ) {
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

    pub fn get_width(&self) -> i32 {
        self.width
    }
    pub fn get_height(&self) -> i32 {
        self.height
    }
    pub fn get_x(&self) -> f64 {
        self.x
    }
    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
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
        self.fill_rect_impl(
            &Rect::new(r.x, r.y + r.height, r.width + 1, 1),
            color,
            draw_mode,
        );
        // 左边
        self.fill_rect_impl(&Rect::new(r.x, r.y + 1, 1, r.height - 1), color, draw_mode);
        // 右边
        self.fill_rect_impl(
            &Rect::new(r.x + r.width, r.y + 1, 1, r.height - 1),
            color,
            draw_mode,
        );
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
                    self.pixels[idx] = 0; // R
                    self.pixels[idx + 1] = 0; // G
                    self.pixels[idx + 2] = 0; // B
                    self.pixels[idx + 3] = 0; // A
                }
            }
        }
    }

    /// 绘制线段
    pub fn draw_line(
        &mut self,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        color: &Color,
        draw_mode: i32,
    ) {
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

    /// AA 线段的单像素混合（对应 C++ GENERIC_DrawLineAA.inc 的 BLEND_PIXEL 宏）
    /// 返回更新后的 `a`（C++ 中 a 在宏内被改写）。
    #[inline]
    fn aa_blend_pixel(pixels: &mut [u8], idx: usize, color: u32, a_in: i32) -> i32 {
        let i = idx * 4;
        let dest = ((pixels[i + 3] as u32) << 24)
            | ((pixels[i] as u32) << 16)
            | ((pixels[i + 1] as u32) << 8)
            | (pixels[i + 2] as u32);

        // C++: aDestAlpha + ((255-aDestAlpha)*a)/255; a = (new == 0) ? 0 : 255*a/new
        let a_dest_alpha = (dest >> 24) as i32;
        let a_new_dest_alpha = a_dest_alpha + ((255 - a_dest_alpha) * a_in) / 255;
        let a = if a_new_dest_alpha == 0 {
            0
        } else {
            255 * a_in / a_new_dest_alpha
        };
        let oma = (256 - a) as u32;

        // C++: (color&m * a + dest&m * oma) >> 8 & m —— uint32 运算可能超上限（C++ 回绕），故 wrapping
        let r_sum = (color & 0xFF0000)
            .wrapping_mul(a as u32)
            .wrapping_add((dest & 0xFF0000).wrapping_mul(oma));
        let g_sum = (color & 0x00FF00)
            .wrapping_mul(a as u32)
            .wrapping_add((dest & 0x00FF00).wrapping_mul(oma));
        let b_sum = (color & 0x0000FF)
            .wrapping_mul(a as u32)
            .wrapping_add((dest & 0x0000FF).wrapping_mul(oma));

        let out = ((a_new_dest_alpha as u32) << 24)
            | ((r_sum >> 8) & 0xFF0000)
            | ((g_sum >> 8) & 0x00FF00)
            | ((b_sum >> 8) & 0x0000FF);
        Self::write_packed_pixel(pixels, idx, out);
        a
    }

    /// 抗锯齿线段（对应 C++ MemoryImage::NormalDrawLineAA，MemoryImage.cpp:617-695 +
    /// inc_routines/GENERIC_DrawLineAA.inc 的 Wu 风格误差累积算法）
    fn normal_draw_line_aa(
        &mut self,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        color: &Color,
    ) {
        let a_color = ((color.a as u32) << 24)
            | ((color.r as u32) << 16)
            | ((color.g as u32) << 8)
            | (color.b as u32);
        let a_alpha = color.a as i32;

        let mut a_x0 = start_x as i32;
        let mut a_x1 = end_x as i32;
        let mut a_y0 = start_y as i32;
        let mut a_y1 = end_y as i32;
        let mut a_x_inc = 1i32;

        let mut a_start_x = start_x;
        let mut a_start_y = start_y;
        let mut a_end_x = end_x;
        let mut a_end_y = end_y;

        // C++: if (aY0 > aY1) 交换两端（整数与 double 坐标一起换）
        if a_y0 > a_y1 {
            std::mem::swap(&mut a_x0, &mut a_x1);
            std::mem::swap(&mut a_y0, &mut a_y1);
            std::mem::swap(&mut a_start_x, &mut a_end_x);
            std::mem::swap(&mut a_start_y, &mut a_end_y);
        }

        let mut dx = a_x1 - a_x0;
        let mut dy = a_y1 - a_y0;
        let mut dxd = a_end_x - a_start_x;
        let dyd = a_end_y - a_start_y;
        if dx < 0 {
            dx = -dx;
            a_x_inc = -1;
            dxd = -dxd;
        }

        let stride = self.width as i32;
        let a_height = self.height as i32;

        // C++: CALC_WEIGHT_A(w) = (mAlpha != 255) ? ((w * (mAlpha+1)) >> 8) : w
        let calc_weight_a = |w: i32| -> i32 {
            if a_alpha != 255 {
                (w * (a_alpha + 1)) >> 8
            } else {
                w
            }
        };

        let mut an_err_acc = ((a_start_x - a_x0 as f64).abs()
            * (a_start_y - a_y0 as f64).abs()
            * 0x10000 as f64) as u32;

        let mut a_dest_x = a_x0;
        let mut a_dest_y = a_y0;
        let mut a_dest_idx = a_dest_y * stride + a_dest_x;

        // [TRANSLATION_NOTE]: C++ 靠调用方保证索引在范围内；Rust 侧对越界像素跳过（避免 panic）。
        let a_pixels_len = self.pixels.len();
        let in_range = |idx: i32| -> bool { idx >= 0 && ((idx as usize) * 4 + 3) < a_pixels_len };

        if dx >= dy {
            // 偏水平
            let an_err_adj = (dyd / dxd * 0x10000 as f64) as u32;

            let a_weight = 255 - ((an_err_acc >> 8) as i32);
            let a = calc_weight_a(a_weight);
            if in_range(a_dest_idx) {
                Self::aa_blend_pixel(&mut self.pixels, a_dest_idx as usize, a_color, a);
            }

            loop {
                dx -= 1;
                if dx <= 0 {
                    break;
                }

                an_err_acc = an_err_acc.wrapping_add(an_err_adj);
                if an_err_acc >= 0x10000 {
                    an_err_acc &= 0xFFFF;
                    a_dest_idx += stride;
                    a_dest_y += 1;
                }
                a_dest_idx += a_x_inc;

                let a_weight = 255 - ((an_err_acc >> 8) as i32);
                let a_weight2 = a_weight ^ 0xFF;

                let a = calc_weight_a(a_weight);
                if in_range(a_dest_idx) {
                    Self::aa_blend_pixel(&mut self.pixels, a_dest_idx as usize, a_color, a);
                }

                if a_dest_y < a_height - 1 {
                    let a2 = calc_weight_a(a_weight2);
                    let an_idx2 = a_dest_idx + stride;
                    if in_range(an_idx2) {
                        Self::aa_blend_pixel(&mut self.pixels, an_idx2 as usize, a_color, a2);
                    }
                }
            }
        } else {
            // 偏垂直
            let an_err_adj = (dxd / dyd * 0x10000 as f64) as u32;

            let a_weight = 255 - ((an_err_acc >> 8) as i32);
            let a = calc_weight_a(a_weight);
            if in_range(a_dest_idx) {
                Self::aa_blend_pixel(&mut self.pixels, a_dest_idx as usize, a_color, a);
            }

            loop {
                dy -= 1;
                if dy <= 0 {
                    break;
                }

                an_err_acc = an_err_acc.wrapping_add(an_err_adj);
                if an_err_acc >= 0x10000 {
                    an_err_acc &= 0xFFFF;
                    a_dest_idx += a_x_inc;
                    a_dest_x += a_x_inc;
                }
                a_dest_idx += stride;

                let a_weight = 255 - ((an_err_acc >> 8) as i32);
                let a_weight2 = a_weight ^ 0xFF;

                let a = calc_weight_a(a_weight);
                if in_range(a_dest_idx) {
                    Self::aa_blend_pixel(&mut self.pixels, a_dest_idx as usize, a_color, a);
                }

                // C++ 此处判据写作 `aX0 < STRIDE-1`（STRIDE == mWidth）
                if a_dest_x < stride - 1 {
                    let a2 = calc_weight_a(a_weight2);
                    let an_idx2 = a_dest_idx + a_x_inc;
                    if in_range(an_idx2) {
                        Self::aa_blend_pixel(&mut self.pixels, an_idx2 as usize, a_color, a2);
                    }
                }
            }
        }

        // 收尾像素（C++: aDestPixels = &aBits[aY1*STRIDE + aX1]）
        let a_end_idx = a_y1 * stride + a_x1;
        let a_weight = 255 - ((an_err_acc >> 8) as i32);
        let a = calc_weight_a(a_weight);
        if in_range(a_end_idx) {
            Self::aa_blend_pixel(&mut self.pixels, a_end_idx as usize, a_color, a);
        }
    }

    /// 绘制抗锯齿线段（对应 C++ MemoryImage::DrawLineAA，MemoryImage.cpp:697-719）
    pub fn draw_line_aa(
        &mut self,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        color: &Color,
        draw_mode: i32,
    ) {
        if start_y == end_y {
            let a_start_x = start_x.min(end_x) as i32;
            let a_end_x = start_x.max(end_x) as i32;
            self.fill_rect(
                &Rect::new(
                    a_start_x,
                    start_y as i32,
                    a_end_x - a_start_x + 1,
                    end_y as i32 - start_y as i32 + 1,
                ),
                color,
                draw_mode,
            );
            return;
        } else if start_x == end_x {
            let a_start_y = start_y.min(end_y) as i32;
            let a_end_y = start_y.max(end_y) as i32;
            self.fill_rect(
                &Rect::new(
                    start_x as i32,
                    a_start_y,
                    end_x as i32 - start_x as i32 + 1,
                    a_end_y - a_start_y + 1,
                ),
                color,
                draw_mode,
            );
            return;
        }

        self.normal_draw_line_aa(start_x, start_y, end_x, end_y, color);
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
    /// [TRANSLATION_NOTE]: `_linear_filter` 被忽略是**符合 C++** 的，不是缺口 ——
    /// C++ MemoryImage::Blt（MemoryImage.cpp:1364-1367）首行即
    /// `(void)linearFilter; // Software rendering does not use texture filtering`。
    /// 因此编译告警中的 "unused variable: linear_filter"（memory_image.rs:160）属预期行为。
    pub fn blt(
        &mut self,
        src: &Image,
        dest_x: i32,
        dest_y: i32,
        src_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        _linear_filter: bool,
    ) {
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
    /// 对应 C++ MemoryImage::BltF（MemoryImage.cpp:1385-1390）：
    /// `theImage->mDrawn = true; BltRotated(theImage,theX,theY,theSrcRect,theClipRect,theColor,theDrawMode,0,0,0);`
    /// —— 即"旋转角与旋转中心均为 0 的 BltRotated"，因此同样带 clip 裁剪与双线性采样。
    /// [TRANSLATION_NOTE]: 原实现是"四舍五入后转调整型 blt"，既丢掉了 theClipRect 裁剪，
    /// 也与 C++ 的采样方式（双线性）不同；现按 C++ 转调 blt_rotated。
    pub fn blt_f(
        &mut self,
        src: &Image,
        dest_x: f32,
        dest_y: f32,
        src_rect: &Rect,
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
    ) {
        self.blt_rotated(
            src, dest_x, dest_y, src_rect, clip_rect, color, draw_mode, 0.0, 0.0, 0.0,
        );
    }

    /// 旋转 Blt 的裁剪辅助（对应 C++ MemoryImage::BltRotatedClipHelper，MemoryImage.cpp:1392-1434）
    /// 返回裁剪后的目标矩形 (x, y, width, height)；无可见区域时返回 None。
    fn blt_rotated_clip_helper(
        the_x: f32,
        the_y: f32,
        src_rect: &Rect,
        clip_rect: &Rect,
        the_rot: f64,
        rot_center_x: f32,
        rot_center_y: f32,
    ) -> Option<(f32, f32, f32, f32)> {
        let a_cos = the_rot.cos() as f32;
        let a_sin = the_rot.sin() as f32;

        // C++: 把源矩形四角映射到目标空间，求包围盒
        let px = [0.0f32, src_rect.width as f32, src_rect.width as f32, 0.0];
        let py = [0.0f32, 0.0, src_rect.height as f32, src_rect.height as f32];
        let mut a_min_x = 10000000.0f32;
        let mut a_max_x = -10000000.0f32;
        let mut a_min_y = 10000000.0f32;
        let mut a_max_y = -10000000.0f32;

        for i in 0..4 {
            let ox = px[i] - rot_center_x;
            let oy = py[i] - rot_center_y;

            let nx = (rot_center_x + ox * a_cos + oy * a_sin) + the_x;
            let ny = (rot_center_y + oy * a_cos - ox * a_sin) + the_y;

            if nx < a_min_x {
                a_min_x = nx;
            }
            if nx > a_max_x {
                a_max_x = nx;
            }
            if ny < a_min_y {
                a_min_y = ny;
            }
            if ny > a_max_y {
                a_max_y = ny;
            }
        }

        // C++: FRect(aMinX, aMinY, aMaxX-aMinX, aMaxY-aMinY).Intersection(aClipRect)
        let a_clip_x = clip_rect.x as f32;
        let a_clip_y = clip_rect.y as f32;
        let a_clip_r = a_clip_x + clip_rect.width as f32;
        let a_clip_b = a_clip_y + clip_rect.height as f32;

        let a_dest_x = a_min_x.max(a_clip_x);
        let a_dest_y = a_min_y.max(a_clip_y);
        let a_dest_r = a_max_x.min(a_clip_r);
        let a_dest_b = a_max_y.min(a_clip_b);
        let a_dest_w = a_dest_r - a_dest_x;
        let a_dest_h = a_dest_b - a_dest_y;

        if a_dest_w <= 0.0 || a_dest_h <= 0.0 {
            return None;
        }
        Some((a_dest_x, a_dest_y, a_dest_w, a_dest_h))
    }

    /// 旋转 Blt（对应 C++ MemoryImage::BltRotated，MemoryImage.cpp:1470-1526 +
    /// inc_routines/MI_BltRotated.inc / BltRotatedHelper.inc 的 32 位路径）
    ///
    /// [TRANSLATION_NOTE]: C++ 通过 `#include` 宏模板同时生成 8/16/32 位与
    /// Normal/Additive 变体；Rust 侧 MemoryImage 恒为 32 位 RGBA，故只实现 32 位路径，
    /// 并保留 White 与 Colored（theColor != White）两个 WRITE_PIXEL 变体。
    /// 采样与混合的整数运算（0x10000 定点、0x27FFF 边距、finalAlpha 公式）
    /// 逐条照抄 C++，源/目标像素在内部按 C++ 的 0xAARRGGBB 打包后再运算，以保证数值等价。
    pub fn blt_rotated(
        &mut self,
        src: &Image,
        dest_x: f32,
        dest_y: f32,
        src_rect: &Rect,
        clip_rect: &Rect,
        color: &Color,
        _draw_mode: i32,
        rot: f64,
        rot_center_x: f32,
        rot_center_y: f32,
    ) {
        if src.pixels.is_empty() || self.pixels.is_empty() {
            return;
        }
        if src_rect.width <= 0 || src_rect.height <= 0 {
            return;
        }

        let a_dest_rect = match Self::blt_rotated_clip_helper(
            dest_x,
            dest_y,
            src_rect,
            clip_rect,
            rot,
            rot_center_x,
            rot_center_y,
        ) {
            Some(r) => r,
            None => return,
        };

        let a_cos = rot.cos();
        let a_sin = rot.sin();
        let a_cos_long = (a_cos * 0x10000 as f64) as i32;
        let a_sin_long = (a_sin * 0x10000 as f64) as i32;

        let a_dest_x = a_dest_rect.0 as i32;
        let a_dest_y = a_dest_rect.1 as i32;
        let a_dest_w = a_dest_rect.2 as i32;
        let a_dest_h = a_dest_rect.3 as i32;

        // C++: aSrcBits = theImage->GetBits() + theSrcRect.mX + theSrcRect.mY * theSrcRect.mWidth
        // （注意 C++ 此处用 theSrcRect.mWidth 作 pitch，之后逐像素再加 aVInt * theImage->mWidth）
        let a_src_base = (src_rect.x + src_rect.y * src_rect.width) as usize;
        let a_src_width = src.width as i32;
        let a_self_width = self.width as i32;

        let a_is_white = *color == Color::WHITE;
        let ca = color.a as i32;
        let cr = color.r as i32 + 1;
        let cg = color.g as i32 + 1;
        let cb = color.b as i32 + 1;

        for y in 0..a_dest_h {
            let a_base_x = a_dest_x as f32 + 0.0 - rot_center_x - dest_x;
            let a_base_y = a_dest_y as f32 + y as f32 - rot_center_y - dest_y;

            let mut a_u = ((a_cos * a_base_x as f64 - a_sin * a_base_y as f64
                + rot_center_x as f64)
                * 0x10000 as f64) as i32;
            let mut a_v =
                ((a_cos * a_base_y as f64 + a_sin * a_base_x as f64 + rot_center_y as f64)
                    * 0x10000 as f64) as i32;

            let mut a_left = 0i32;
            let mut a_right = a_dest_w - 1;

            let a_u_int = a_u >> 16;
            let a_v_int = a_v >> 16;

            // C++: 依据 cos/sin 的符号收缩扫描区间（0x27FFF/0x17FFF/0x07FFF 为边距常量）
            if a_cos_long <= -0x0001 {
                if a_u_int < 0 {
                    continue;
                }
                let an_edge_left = ((src_rect.width << 16) - 0x27FFF - a_u) / a_cos_long;
                if an_edge_left >= a_left {
                    a_left = an_edge_left;
                }
                let an_edge_right = (0x07FFF - a_u) / a_cos_long;
                if an_edge_right < a_right {
                    a_right = an_edge_right;
                }
            } else if a_cos_long >= 0x0001 {
                if a_u_int >= src_rect.width - 1 {
                    continue;
                }
                let an_edge_left = (0x10000 - a_u) / a_cos_long;
                if an_edge_left > a_left {
                    a_left = an_edge_left;
                }
                let an_edge_right = ((src_rect.width << 16) - 0x17FFF - a_u) / a_cos_long;
                if an_edge_right < a_right {
                    a_right = an_edge_right;
                }
            } else if a_u < 0 || a_u >= (src_rect.width - 1) << 16 {
                continue;
            }

            if a_sin_long <= -0x0001 {
                if a_v_int < 0 {
                    continue;
                }
                let an_edge_left = ((src_rect.height << 16) - 0x27FFF - a_v) / a_sin_long;
                if an_edge_left > a_left {
                    a_left = an_edge_left;
                }
                let an_edge_right = (0x0000 - a_v) / a_sin_long;
                if an_edge_right < a_right {
                    a_right = an_edge_right;
                }
            } else if a_sin_long >= 0x0001 {
                if a_v_int >= src_rect.height - 1 {
                    continue;
                }
                let an_edge_left = (0x10000 - a_v) / a_sin_long;
                if an_edge_left > a_left {
                    a_left = an_edge_left;
                }
                let an_edge_right = ((src_rect.height << 16) - 0x17FFF - a_v) / a_sin_long;
                if an_edge_right < a_right {
                    a_right = an_edge_right;
                }
            } else if a_v < 0 || a_v >= (src_rect.height - 1) << 16 {
                continue;
            }

            a_u += a_left * a_cos_long;
            a_v += a_left * a_sin_long;

            let a_row_off = (a_dest_y + y) * a_self_width + a_dest_x + a_left;

            let mut x = a_left;
            while x <= a_right {
                let a_ui = a_u >> 16;
                let a_vi = a_v >> 16;

                let a_u_factor = (a_u & 0xFFFE) + 1;
                let a_v_factor = (a_v & 0xFFFE) + 1;

                // 双线性取 4 个采样点；越界时跳过（C++ 靠上面的边距常量保证不越界）
                let a_idx = a_src_base as i32 + a_vi * a_src_width + a_ui;
                let a_idx2 = a_idx + 1;
                let a_idx3 = a_idx + a_src_width;
                let a_idx4 = a_idx + 1 + a_src_width;
                if a_idx < 0 || a_idx4 < 0 || ((a_idx4 as usize) * 4 + 3) >= src.pixels.len() {
                    a_u += a_cos_long;
                    a_v += a_sin_long;
                    x += 1;
                    continue;
                }

                let a_dest_idx = a_row_off + (x - a_left);
                if a_dest_idx < 0 || ((a_dest_idx as usize) * 4 + 3) >= self.pixels.len() {
                    a_u += a_cos_long;
                    a_v += a_sin_long;
                    x += 1;
                    continue;
                }

                let (src1, a_src1a) = Self::read_packed_pixel(&src.pixels, a_idx as usize);
                let (src2, a_src2a) = Self::read_packed_pixel(&src.pixels, a_idx2 as usize);
                let (src3, a_src3a) = Self::read_packed_pixel(&src.pixels, a_idx3 as usize);
                let (src4, a_src4a) = Self::read_packed_pixel(&src.pixels, a_idx4 as usize);

                // [TRANSLATION_NOTE]: C++ 这里先做 int 乘法再转 uint32（(0x10000-x)*(0x10000-y)
                // 最大约 0xFFFF^2，会越过 i32 上限，C++ 靠整型回绕）；Rust 用 wrapping_mul 保持同一语义。
                let mut a1 = ((a_src1a as u32)
                    * ((((0x10000 - a_u_factor).wrapping_mul(0x10000 - a_v_factor)) as u32) >> 16)
                    >> 16) as i32;
                let mut a2 = ((a_src2a as u32)
                    * (((a_u_factor.wrapping_mul(0x10000 - a_v_factor)) as u32) >> 16)
                    >> 16) as i32;
                let mut a3 = ((a_src3a as u32)
                    * ((((0x10000 - a_u_factor).wrapping_mul(a_v_factor)) as u32) >> 16)
                    >> 16) as i32;
                let mut a4 = ((a_src4a as u32)
                    * (((a_u_factor.wrapping_mul(a_v_factor)) as u32) >> 16)
                    >> 16) as i32;

                let a_dest_pixel = Self::read_packed_pixel_raw(&self.pixels, a_dest_idx as usize);

                let a_out: Option<u32> = if a_is_white {
                    // ---- C++ WRITE_PIXEL（theColor == Color::White）----
                    let a = a1 + a2 + a3 + a4;
                    if a == 0 {
                        None // 全透明
                    } else {
                        let mut r = ((((src1 & 0xFF0000) as u64 * a1 as u64)
                            + ((src2 & 0xFF0000) as u64 * a2 as u64)
                            + ((src3 & 0xFF0000) as u64 * a3 as u64)
                            + ((src4 & 0xFF0000) as u64 * a4 as u64))
                            & 0xFF000000) as u32;
                        let mut g = ((((src1 & 0x00FF00) as u64 * a1 as u64)
                            + ((src2 & 0x00FF00) as u64 * a2 as u64)
                            + ((src3 & 0x00FF00) as u64 * a3 as u64)
                            + ((src4 & 0x00FF00) as u64 * a4 as u64))
                            & 0x00FF0000) as u32;
                        let mut b = ((((src1 & 0x0000FF) as u64 * a1 as u64)
                            + ((src2 & 0x0000FF) as u64 * a2 as u64)
                            + ((src3 & 0x0000FF) as u64 * a3 as u64)
                            + ((src4 & 0x0000FF) as u64 * a4 as u64))
                            & 0x0000FF00) as u32;

                        if a > 250 {
                            Some(0xFF000000 | (r >> 8) | (g >> 8) | (b >> 8))
                        } else {
                            let a_da = a_dest_pixel >> 24;
                            let dr = (((a_dest_pixel & 0xFF0000) * a_da) >> 8) & 0xFF0000;
                            let dg = (((a_dest_pixel & 0x00FF00) * a_da) >> 8) & 0x00FF00;
                            let db = (((a_dest_pixel & 0x0000FF) * a_da) >> 8) & 0x0000FF;

                            let a_final_alpha = 256 - (((256 - a) * (256 - a_da as i32)) >> 8);

                            // [TRANSLATION_NOTE]: a（权重和）最大 1020 > 256，故 (256 - a) 可能为负；
                            // C++ 是 `uint32_t r + (int)(256-a)*dr`（int→uint32 回绕），
                            // 这里用 wrapping_mul/wrapping_add 保持同一数值语义（避免 debug 溢出 panic）。
                            r = r.wrapping_add(((256 - a).wrapping_mul(dr as i32)) as u32)
                                / a_final_alpha as u32
                                & 0xFF0000;
                            g = g.wrapping_add(((256 - a).wrapping_mul(dg as i32)) as u32)
                                / a_final_alpha as u32
                                & 0x00FF00;
                            b = b.wrapping_add(((256 - a).wrapping_mul(db as i32)) as u32)
                                / a_final_alpha as u32
                                & 0x0000FF;

                            Some((((a_final_alpha - 1) as u32) << 24) | r | g | b)
                        }
                    }
                } else {
                    // ---- C++ WRITE_PIXEL（theColor != Color::White）----
                    a1 = (a1 * ca) >> 8;
                    a2 = (a2 * ca) >> 8;
                    a3 = (a3 * ca) >> 8;
                    a4 = (a4 * ca) >> 8;
                    let a = a1 + a2 + a3 + a4;

                    if a == 0 {
                        None
                    } else {
                        let a_sum_r = ((src1 & 0xFF0000) as u64 * a1 as u64)
                            + ((src2 & 0xFF0000) as u64 * a2 as u64)
                            + ((src3 & 0xFF0000) as u64 * a3 as u64)
                            + ((src4 & 0xFF0000) as u64 * a4 as u64);
                        let a_sum_g = ((src1 & 0x00FF00) as u64 * a1 as u64)
                            + ((src2 & 0x00FF00) as u64 * a2 as u64)
                            + ((src3 & 0x00FF00) as u64 * a3 as u64)
                            + ((src4 & 0x00FF00) as u64 * a4 as u64);
                        let a_sum_b = ((src1 & 0x0000FF) as u64 * a1 as u64)
                            + ((src2 & 0x0000FF) as u64 * a2 as u64)
                            + ((src3 & 0x0000FF) as u64 * a3 as u64)
                            + ((src4 & 0x0000FF) as u64 * a4 as u64);

                        // C++: r = cr * (sum_r >> 8) & 0xFF000000
                        let mut r = (cr as u32).wrapping_mul((a_sum_r >> 8) as u32) & 0xFF000000;
                        // C++: g = ((sum_g & 0x00FF0000) * cg) & 0xFF000000
                        let mut g =
                            ((a_sum_g & 0x00FF0000) as u32).wrapping_mul(cg as u32) & 0xFF000000;
                        // C++: b = ((sum_b & 0x0000FF00) * cb) & 0x00FF0000
                        let mut b =
                            ((a_sum_b & 0x0000FF00) as u32).wrapping_mul(cb as u32) & 0x00FF0000;

                        if a > 250 {
                            Some(0xFF000000 | (r >> 8) | (g >> 16) | (b >> 16))
                        } else {
                            let a_da = a_dest_pixel >> 24;
                            let dr = (((a_dest_pixel & 0xFF0000) * a_da) >> 8) & 0xFF0000;
                            let dg = (((a_dest_pixel & 0x00FF00) * a_da) >> 8) & 0x00FF00;
                            let db = (((a_dest_pixel & 0x0000FF) * a_da) >> 8) & 0x0000FF;

                            let a_final_alpha = 256 - (((256 - a) * (256 - a_da as i32)) >> 8);

                            // [TRANSLATION_NOTE]: 同 White 分支，用 wrapping 保持 C++ 的 int→uint32 回绕语义。
                            r = r.wrapping_add(((256 - a).wrapping_mul(dr as i32)) as u32)
                                / a_final_alpha as u32
                                & 0xFF0000;
                            g = (g >> 8).wrapping_add(((256 - a).wrapping_mul(dg as i32)) as u32)
                                / a_final_alpha as u32
                                & 0x00FF00;
                            b = (b >> 8).wrapping_add(((256 - a).wrapping_mul(db as i32)) as u32)
                                / a_final_alpha as u32
                                & 0x0000FF;

                            Some((((a_final_alpha - 1) as u32) << 24) | r | g | b)
                        }
                    }
                };

                if let Some(a_packed) = a_out {
                    Self::write_packed_pixel(&mut self.pixels, a_dest_idx as usize, a_packed);
                }

                a_u += a_cos_long;
                a_v += a_sin_long;
                x += 1;
            }
        }
    }

    /// 读取像素并按 C++ 的 0xAARRGGBB 打包（返回 (packed, alpha)）
    #[inline]
    fn read_packed_pixel(pixels: &[u8], pixel_index: usize) -> (u32, u8) {
        let i = pixel_index * 4;
        let r = pixels[i] as u32;
        let g = pixels[i + 1] as u32;
        let b = pixels[i + 2] as u32;
        let a = pixels[i + 3] as u32;
        ((a << 24) | (r << 16) | (g << 8) | b, a as u8)
    }

    /// 读取像素并按 C++ 的 0xAARRGGBB 打包（不返回 alpha 分量）
    #[inline]
    fn read_packed_pixel_raw(pixels: &[u8], pixel_index: usize) -> u32 {
        let i = pixel_index * 4;
        ((pixels[i + 3] as u32) << 24)
            | ((pixels[i] as u32) << 16)
            | ((pixels[i + 1] as u32) << 8)
            | (pixels[i + 2] as u32)
    }

    /// 把 C++ 的 0xAARRGGBB 写回 RGBA 字节序
    #[inline]
    fn write_packed_pixel(pixels: &mut [u8], pixel_index: usize, packed: u32) {
        let i = pixel_index * 4;
        pixels[i] = ((packed >> 16) & 0xFF) as u8;
        pixels[i + 1] = ((packed >> 8) & 0xFF) as u8;
        pixels[i + 2] = (packed & 0xFF) as u8;
        pixels[i + 3] = ((packed >> 24) & 0xFF) as u8;
    }

    /// 拉伸 Blt 的裁剪辅助（对应 C++ MemoryImage::StretchBltClipHelper，MemoryImage.cpp:1436-1449；
    /// mirror=true 时对应 StretchBltMirrorClipHelper，MemoryImage.cpp:1451-1468）
    /// 返回 (裁剪后的目标矩形, 对应的源矩形 FRect)。
    fn stretch_blt_clip_helper(
        src_rect: &Rect,
        clip_rect: &Rect,
        dest_rect: &Rect,
        mirror: bool,
    ) -> Option<(Rect, (f64, f64, f64, f64))> {
        // C++: Rect(theDestRect).Intersection(theClipRect)
        let d_x = dest_rect.x.max(clip_rect.x);
        let d_y = dest_rect.y.max(clip_rect.y);
        let d_r = (dest_rect.x + dest_rect.width).min(clip_rect.x + clip_rect.width);
        let d_b = (dest_rect.y + dest_rect.height).min(clip_rect.y + clip_rect.height);
        let d_w = d_r - d_x;
        let d_h = d_b - d_y;
        if d_w <= 0 || d_h <= 0 {
            return None;
        }
        let a_dest_out = Rect::new(d_x, d_y, d_w, d_h);

        let a_x_factor = src_rect.width as f64 / dest_rect.width as f64;
        let a_y_factor = src_rect.height as f64 / dest_rect.height as f64;

        // 非镜像版用左侧被裁掉的量；镜像版改用右侧被裁掉的量（C++ 的 aRightClip）
        let a_h_clip = if mirror {
            let a_total_clip = dest_rect.width - d_w;
            let a_left_clip = d_x - dest_rect.x;
            a_total_clip - a_left_clip
        } else {
            d_x - dest_rect.x
        };

        let a_src_out = (
            src_rect.x as f64 + a_h_clip as f64 * a_x_factor,
            src_rect.y as f64 + (d_y - dest_rect.y) as f64 * a_y_factor,
            src_rect.width as f64 + (d_w - dest_rect.width) as f64 * a_x_factor,
            src_rect.height as f64 + (d_h - dest_rect.height) as f64 * a_y_factor,
        );
        if a_src_out.2 <= 0.0 || a_src_out.3 <= 0.0 {
            return None;
        }
        Some((a_dest_out, a_src_out))
    }

    /// 快速拉伸 Blt（对应 C++ MemoryImage::FastStretchBlt，MemoryImage.cpp:1574-1638）
    /// [TRANSLATION_NOTE]: C++ 的 FastStretchBlt 只实现了 theColor == Color::White 分支，
    /// 非 White 时函数体为空 —— 此处保持一致。
    fn fast_stretch_blt(
        &mut self,
        src: &Image,
        dest_rect: &Rect,
        src_rect: &(f64, f64, f64, f64),
        color: &Color,
    ) {
        if *color != Color::WHITE {
            return;
        }

        let a_src_w = src.width as i32;
        let a_self_w = self.width as i32;
        let (a_src_x0, a_src_y0, a_src_wf, a_src_hf) = *src_rect;

        let an_add_x = a_src_wf / dest_rect.width as f64;
        let an_add_y = a_src_hf / dest_rect.height as f64;

        let mut a_src_y = a_src_y0;
        for y in 0..dest_rect.height {
            let mut a_src_x = a_src_x0;
            for x in 0..dest_rect.width {
                // C++: aSrcX += anAddX;（先递增再取样）
                a_src_x += an_add_x;

                let a_src_idx_i = a_src_y as i32 * a_src_w + a_src_x as i32;
                let a_dest_idx_i = (dest_rect.y + y) * a_self_w + dest_rect.x + x;
                if a_src_idx_i < 0 || a_dest_idx_i < 0 {
                    continue;
                }
                let a_src_idx = a_src_idx_i as usize;
                let a_dest_idx = a_dest_idx_i as usize;
                if a_src_idx * 4 + 3 >= src.pixels.len() || a_dest_idx * 4 + 3 >= self.pixels.len()
                {
                    continue;
                }

                let a_src_packed = Self::read_packed_pixel_raw(&src.pixels, a_src_idx);
                let a_dest_packed = Self::read_packed_pixel_raw(&self.pixels, a_dest_idx);

                let mut a = (a_src_packed >> 24) as i32;
                if a != 0 {
                    let a_dest_alpha = (a_dest_packed >> 24) as i32;
                    let a_new_dest_alpha = a_dest_alpha + ((255 - a_dest_alpha) * a) / 255;
                    a = 255 * a / a_new_dest_alpha;
                    let oma = (256 - a) as u32;

                    // C++: (aNewDestAlpha << 24) | ((dest&c)*oma>>8 + (src&c)*a>>8 & mask) | ...
                    let a_out = ((a_new_dest_alpha as u32) << 24)
                        | ((((a_dest_packed & 0x0000FF) * oma) >> 8)
                            + (((a_src_packed & 0x0000FF) * a as u32) >> 8)
                            & 0x0000FF)
                        | ((((a_dest_packed & 0x00FF00) * oma) >> 8)
                            + (((a_src_packed & 0x00FF00) * a as u32) >> 8)
                            & 0x00FF00)
                        | ((((a_dest_packed & 0xFF0000) * oma) >> 8)
                            + (((a_src_packed & 0xFF0000) * a as u32) >> 8)
                            & 0xFF0000);
                    Self::write_packed_pixel(&mut self.pixels, a_dest_idx, a_out);
                }
            }
            a_src_y += an_add_y;
        }
    }

    /// 慢速拉伸 Blt（对应 C++ MemoryImage::SlowStretchBlt，MemoryImage.cpp:1528-1571 +
    /// inc_routines/MI_SlowStretchBlt.inc，278 行）
    ///
    /// 算法：先把源区域**水平**缩放进一张临时缓冲（每像素 4 个 u32 累加器，顺序 B,G,R,A），
    /// 再**垂直**缩放进第二张缓冲，最后以 (累加值 >> 16)、按 `(dest*oma>>8) + (src*a>>8)`
    /// 与目标混合。缩小与放大走两条不同分支（缩小用 257 因子把每个源列摊到目标列上，
    /// 放大用 257/256 因子在两列/两行间线性插值）。
    ///
    /// 注意 C++ 忽略了 theColor/theDrawMode（`(void)theColor;(void)theDrawMode;`），此处一致。
    fn slow_stretch_blt(
        &mut self,
        src: &Image,
        dest_rect: &Rect,
        src_rect: &(f64, f64, f64, f64),
        _color: &Color,
        _draw_mode: i32,
    ) {
        let (a_src_xf, a_src_yf, a_src_wf, a_src_hf) = *src_rect;
        let a_src_w = src.width;
        let a_src_h = src.height;

        // C++: floor/ceil 后按图像边界裁剪
        let mut a_src_xi = a_src_xf.floor() as i32;
        let mut a_src_yi = a_src_yf.floor() as i32;
        let mut a_src_wi = (a_src_wf + (a_src_xf - a_src_xi as f64)).ceil() as i32;
        let mut a_src_hi = (a_src_hf + (a_src_yf - a_src_yi as f64)).ceil() as i32;

        if a_src_xi < 0 {
            a_src_xi = 0;
        }
        if a_src_yi < 0 {
            a_src_yi = 0;
        }
        if a_src_xi + a_src_wi > a_src_w as i32 {
            a_src_wi = a_src_w as i32 - a_src_xi;
        }
        if a_src_yi + a_src_hi > a_src_h as i32 {
            a_src_hi = a_src_h as i32 - a_src_yi;
        }
        if a_src_wi <= 0 || a_src_hi <= 0 {
            return;
        }

        let a_temp_w = dest_rect.width + 4;
        let a_temp_h = dest_rect.height + 4;
        let a_stride4 = (a_temp_w * 4) as usize;
        let a_src_row_w = a_src_w as i32;

        // ---- 水平一趟 ----
        let mut a_horz = vec![0u32; (a_temp_w * a_src_hi * 4) as usize];

        if a_src_wf >= dest_rect.width as f64 {
            // 缩小：把每个源列按 257 因子分摊到它覆盖的目标列
            let a_dest_x_factor = dest_rect.width as f64 / a_src_wf;
            let a_dest_x_offset = 1.0 + (a_src_xi as f64 - a_src_xf) * a_dest_x_factor;

            for a_src_x in 0..a_src_wi {
                let a_dest_x1 = a_dest_x_factor * a_src_x as f64 + a_dest_x_offset;
                let a_dest_x2 = a_dest_x1 + a_dest_x_factor;
                let a_dest_xi1 = a_dest_x1 as i32;
                let a_dest_xi2 = a_dest_x2 as i32;

                let mut s1 = a_src_yi * a_src_row_w + a_src_xi + a_src_x;

                if a_dest_xi1 == a_dest_xi2 {
                    let mut d = (a_dest_xi1 * 4) as usize;
                    let a_factor = (257.0 * a_dest_x_factor) as i32;
                    for _ in 0..a_src_hi {
                        let a_pixel = Self::read_packed_pixel_raw(&src.pixels, s1 as usize);
                        a_horz[d] =
                            a_horz[d].wrapping_add((a_factor * ((a_pixel & 0xFF) as i32)) as u32);
                        a_horz[d + 1] = a_horz[d + 1]
                            .wrapping_add((a_factor * (((a_pixel >> 8) & 0xFF) as i32)) as u32);
                        a_horz[d + 2] = a_horz[d + 2]
                            .wrapping_add((a_factor * (((a_pixel >> 16) & 0xFF) as i32)) as u32);
                        a_horz[d + 3] = a_horz[d + 3]
                            .wrapping_add((a_factor * (((a_pixel >> 24) & 0xFF) as i32)) as u32);
                        d += a_stride4;
                        s1 += a_src_row_w;
                    }
                } else {
                    let a_factor1 = (257.0 * (a_dest_xi2 as f64 - a_dest_x1)) as i32;
                    let a_factor2 = (257.0 * (a_dest_x2 - a_dest_xi2 as f64)) as i32;
                    let mut d = (a_dest_xi1 * 4) as usize;
                    for _ in 0..a_src_hi {
                        let a_pixel = Self::read_packed_pixel_raw(&src.pixels, s1 as usize);

                        a_horz[d] =
                            a_horz[d].wrapping_add((a_factor1 * ((a_pixel & 0xFF) as i32)) as u32);
                        a_horz[d + 1] = a_horz[d + 1]
                            .wrapping_add((a_factor1 * (((a_pixel >> 8) & 0xFF) as i32)) as u32);
                        a_horz[d + 2] = a_horz[d + 2]
                            .wrapping_add((a_factor1 * (((a_pixel >> 16) & 0xFF) as i32)) as u32);
                        a_horz[d + 3] = a_horz[d + 3]
                            .wrapping_add((a_factor1 * (((a_pixel >> 24) & 0xFF) as i32)) as u32);

                        a_horz[d + 4] = a_horz[d + 4]
                            .wrapping_add((a_factor2 * ((a_pixel & 0xFF) as i32)) as u32);
                        a_horz[d + 5] = a_horz[d + 5]
                            .wrapping_add((a_factor2 * (((a_pixel >> 8) & 0xFF) as i32)) as u32);
                        a_horz[d + 6] = a_horz[d + 6]
                            .wrapping_add((a_factor2 * (((a_pixel >> 16) & 0xFF) as i32)) as u32);
                        a_horz[d + 7] = a_horz[d + 7]
                            .wrapping_add((a_factor2 * (((a_pixel >> 24) & 0xFF) as i32)) as u32);

                        d += a_stride4;
                        s1 += a_src_row_w;
                    }
                }
            }
        } else {
            // 放大：目标列在两相邻源列之间线性插值
            let a_src_x_factor = if dest_rect.width != 1 {
                (a_src_wf - 1.0) / (dest_rect.width as f64 - 1.0)
            } else {
                a_src_wf / dest_rect.width as f64
            };

            for a_dest_x in 1..(a_temp_w - 1) {
                let mut d = (a_dest_x * 4) as usize;
                let a_src_x = (a_dest_x - 1) as f64 * a_src_x_factor + a_src_xf;
                let a_src_xi_sub = a_src_x as i32;
                let a_factor1 = (257.0 * (1.0 - (a_src_x - a_src_xi_sub as f64))) as i32;
                let a_factor2 = 257 - a_factor1;

                let mut s = a_src_yi * a_src_row_w + a_src_xi_sub;
                for _ in 0..a_src_hi {
                    let a_pixel1 = Self::read_packed_pixel_raw(&src.pixels, s as usize);
                    s += 1;
                    let a_pixel2 = Self::read_packed_pixel_raw(&src.pixels, s as usize);

                    a_horz[d] = ((a_factor1 * ((a_pixel1 & 0xFF) as i32))
                        + (a_factor2 * ((a_pixel2 & 0xFF) as i32)))
                        as u32;
                    a_horz[d + 1] = ((a_factor1 * (((a_pixel1 >> 8) & 0xFF) as i32))
                        + (a_factor2 * (((a_pixel2 >> 8) & 0xFF) as i32)))
                        as u32;
                    a_horz[d + 2] = ((a_factor1 * (((a_pixel1 >> 16) & 0xFF) as i32))
                        + (a_factor2 * (((a_pixel2 >> 16) & 0xFF) as i32)))
                        as u32;
                    a_horz[d + 3] = ((a_factor1 * (((a_pixel1 >> 24) & 0xFF) as i32))
                        + (a_factor2 * (((a_pixel2 >> 24) & 0xFF) as i32)))
                        as u32;

                    d += a_stride4;
                    s += a_src_row_w - 1;
                }
            }
        }

        // ---- 垂直一趟 ----
        let mut a_new_pixels = vec![0u32; (a_temp_w * a_temp_h * 4) as usize];

        if a_src_hf >= dest_rect.height as f64 {
            let a_dest_y_factor = dest_rect.height as f64 / a_src_hf;
            let a_dest_y_offset = 1.0 + (a_src_yi as f64 - a_src_yf) * a_dest_y_factor;

            for a_src_y in 0..a_src_hi {
                let a_dest_y1 = a_dest_y_factor * a_src_y as f64 + a_dest_y_offset;
                let a_dest_y2 = a_dest_y1 + a_dest_y_factor;
                let a_dest_yi1 = a_dest_y1.floor() as i32;
                let a_dest_yi2 = a_dest_y2.floor() as i32;

                let mut s = (a_src_y * a_temp_w * 4) as usize;

                if a_dest_yi1 == a_dest_yi2 {
                    let mut d = (a_dest_yi1 * a_temp_w * 4) as usize;
                    let a_factor = (256.0 * a_dest_y_factor) as i32;
                    for _ in 0..a_temp_w {
                        for k in 0..4 {
                            a_new_pixels[d + k] = a_new_pixels[d + k]
                                .wrapping_add((a_factor * a_horz[s + k] as i32) as u32);
                        }
                        d += 4;
                        s += 4;
                    }
                } else {
                    let a_factor1 = (256.0 * (a_dest_yi2 as f64 - a_dest_y1)) as i32;
                    let a_factor2 = (256.0 * (a_dest_y2 - a_dest_yi2 as f64)) as i32;
                    let mut d1 = (a_dest_yi1 * a_temp_w * 4) as usize;
                    let mut d2 = (a_dest_yi2 * a_temp_w * 4) as usize;
                    for _ in 0..a_temp_w {
                        for k in 0..4 {
                            a_new_pixels[d1 + k] = a_new_pixels[d1 + k]
                                .wrapping_add((a_factor1 * a_horz[s + k] as i32) as u32);
                            a_new_pixels[d2 + k] = a_new_pixels[d2 + k]
                                .wrapping_add((a_factor2 * a_horz[s + k] as i32) as u32);
                        }
                        d1 += 4;
                        d2 += 4;
                        s += 4;
                    }
                }
            }
        } else {
            // 垂直放大：目标行在两相邻源行之间线性插值（C++ 的写入形式是 `=` 而非 `+=`）
            let a_src_y_factor = if dest_rect.height != 1 {
                (a_src_hf - 1.0) / (dest_rect.height as f64 - 1.0)
            } else {
                a_src_hf / dest_rect.height as f64
            };

            for a_dest_y in 1..(dest_rect.height + 1) {
                let mut d = ((a_dest_y * a_temp_w + 1) * 4) as usize;

                let a_src_y =
                    (a_dest_y - 1) as f64 * a_src_y_factor + (a_src_yf - (a_src_yf as i32) as f64);
                let a_src_yi_sub = a_src_y as i32;

                let a_factor1 = (256.0 * (1.0 - (a_src_y - a_src_yi_sub as f64))) as i32;
                let a_factor2 = 256 - a_factor1;

                let s1_base = (a_src_yi_sub * a_temp_w + 1) * 4;
                let mut s1 = s1_base as usize;
                let a_is_last_row = a_src_yi_sub == a_src_hi - 1;
                let mut s2 = if a_is_last_row {
                    s1_base as usize
                } else {
                    ((a_src_yi_sub + 1) * a_temp_w + 1) as usize * 4
                };

                for _ in 1..(a_temp_w - 1) {
                    for k in 0..4 {
                        a_new_pixels[d + k] = ((a_factor1 * a_horz[s1 + k] as i32)
                            + (a_factor2 * a_horz[s2 + k] as i32))
                            as u32;
                    }
                    d += 4;
                    s1 += 4;
                    s2 += 4;
                }
            }
        }

        // ---- 合成到目标 ----
        let a_self_width = self.width as i32;
        let a_dest_len = self.pixels.len();

        for y in 0..dest_rect.height {
            for x in 0..dest_rect.width {
                let base = (((y + 1) * a_temp_w + x + 1) * 4) as usize;
                // C++ 的 4 分量顺序为 B,G,R,A，且各自 >> 16 还原
                let b = (a_new_pixels[base] >> 16) as i32;
                let g = (a_new_pixels[base + 1] >> 16) as i32;
                let r = (a_new_pixels[base + 2] >> 16) as i32;
                let a = (a_new_pixels[base + 3] >> 16) as i32;

                if a == 0 {
                    continue;
                }

                let a_dest_idx_i = (dest_rect.y + y) * a_self_width + dest_rect.x + x;
                if a_dest_idx_i < 0 || ((a_dest_idx_i as usize) * 4 + 3) >= a_dest_len {
                    continue;
                }
                let a_dest_idx = a_dest_idx_i as usize;

                let dest = Self::read_packed_pixel_raw(&self.pixels, a_dest_idx);
                let a_dest_alpha = (dest >> 24) as i32;
                let a_new_dest_alpha = a_dest_alpha + ((255 - a_dest_alpha) * a) / 255;
                let oma = (256 - a) as u32;

                let a_out = ((a_new_dest_alpha as u32) << 24)
                    | (((((dest & 0x0000FF) * oma) >> 8) & 0x0000FF)
                        .wrapping_add(((b * a) >> 8) as u32))
                    | (((((dest & 0x00FF00) * oma) >> 8) & 0x00FF00)
                        .wrapping_add((((g * a) >> 8) as u32) << 8))
                    | (((((dest & 0xFF0000) * oma) >> 8) & 0xFF0000)
                        .wrapping_add((((r * a) >> 8) as u32) << 16));

                Self::write_packed_pixel(&mut self.pixels, a_dest_idx, a_out);
            }
        }
    }

    /// 拉伸 Blt（对应 C++ MemoryImage::StretchBlt，MemoryImage.cpp:1640-1654）
    pub fn stretch_blt(
        &mut self,
        src: &Image,
        dest_rect: &Rect,
        src_rect: &Rect,
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        fast_stretch: bool,
    ) {
        if src.pixels.is_empty() || self.pixels.is_empty() {
            return;
        }
        if dest_rect.width <= 0
            || dest_rect.height <= 0
            || src_rect.width <= 0
            || src_rect.height <= 0
        {
            return;
        }

        let (a_dest_rect, a_src_rect) =
            match Self::stretch_blt_clip_helper(src_rect, clip_rect, dest_rect, false) {
                Some(r) => r,
                None => return,
            };

        if fast_stretch {
            self.fast_stretch_blt(src, &a_dest_rect, &a_src_rect, color);
        } else {
            self.slow_stretch_blt(src, &a_dest_rect, &a_src_rect, color, draw_mode);
        }
    }

    /// 三角纹理 Blt
    pub fn blt_triangles_tex(
        &mut self,
        _texture: &Image,
        vertices: &[[crate::framework::graphics::gl_interface::TriVertex; 3]],
        num_triangles: i32,
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        tx: f32,
        ty: f32,
        _blend: bool,
    ) {
        // 软件光栅化（GLImage 子类覆写为 GL 路径）
        self.blt_triangles_impl(
            Some(_texture),
            vertices,
            num_triangles,
            clip_rect,
            color,
            draw_mode,
            tx,
            ty,
        );
    }

    /// 纯色三角形绘制（无纹理，扫描线光栅化）
    /// 对应 C++ PvzpTriangleGroup 的软件路径；顶点色为 ARGB
    pub fn blt_triangles_flat(
        &mut self,
        vertices: &[[crate::framework::graphics::gl_interface::TriVertex; 3]],
        num_triangles: i32,
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        tx: f32,
        ty: f32,
    ) {
        self.blt_triangles_impl(
            None,
            vertices,
            num_triangles,
            clip_rect,
            color,
            draw_mode,
            tx,
            ty,
        );
    }

    /// 三角形纹理光栅化（软件路径）
    fn blt_triangles_impl(
        &mut self,
        texture: Option<&Image>,
        vertices: &[[crate::framework::graphics::gl_interface::TriVertex; 3]],
        num_triangles: i32,
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        tx: f32,
        ty: f32,
    ) {
        if self.pixels.is_empty() {
            return;
        }
        for tri in vertices.iter().take(num_triangles.max(0) as usize) {
            self.blt_triangle_scanline(
                &tri[0], &tri[1], &tri[2], texture, clip_rect, color, draw_mode, tx, ty,
            );
        }
    }

    /// 单个三角形扫描线光栅化（顶点色 + u/v 线性插值；texture 非空时采样纹理）
    fn blt_triangle_scanline(
        &mut self,
        a: &crate::framework::graphics::gl_interface::TriVertex,
        b: &crate::framework::graphics::gl_interface::TriVertex,
        c: &crate::framework::graphics::gl_interface::TriVertex,
        texture: Option<&Image>,
        clip: &Rect,
        tint: &Color,
        draw_mode: i32,
        tx: f32,
        ty: f32,
    ) {
        let mut v = [
            (a.x + tx, a.y + ty, a.u, a.v, a.color),
            (b.x + tx, b.y + ty, b.u, b.v, b.color),
            (c.x + tx, c.y + ty, c.u, c.v, c.color),
        ];
        v.sort_by(|p, q| p.1.partial_cmp(&q.1).unwrap_or(std::cmp::Ordering::Equal));
        let (x0, y0, u0, v0, c0) = v[0];
        let (x1, y1, u1, v1, c1) = v[1];
        let (x2, y2, u2, v2, c2) = v[2];
        if (y2 - y0).abs() < 0.001 {
            return; // 零高退化三角形
        }
        let y_min = y0.ceil() as i32;
        let y_max = y2.floor() as i32;
        let sy_lo = clip.y.max(y_min);
        let sy_hi = (clip.y + clip.height).min(y_max + 1);
        let tex_w = texture.map_or(1, |t| t.width.max(1));
        let tex_h = texture.map_or(1, |t| t.height.max(1));
        for sy in sy_lo..sy_hi {
            let y = sy as f32;
            // 长边（v0→v2）与短边（v0→v1 / v1→v2）
            let t02 = ((y - y0) / (y2 - y0)).clamp(0.0, 1.0);
            let x_long = x0 + (x2 - x0) * t02;
            let u_long = u0 + (u2 - u0) * t02;
            let v_long = v0 + (v2 - v0) * t02;
            let c_long = Self::lerp_color(c0, c2, t02);
            let (x_short, u_short, v_short, c_short) = if y < y1 {
                let t01 = if (y1 - y0).abs() < 0.001 {
                    0.0
                } else {
                    ((y - y0) / (y1 - y0)).clamp(0.0, 1.0)
                };
                (
                    x0 + (x1 - x0) * t01,
                    u0 + (u1 - u0) * t01,
                    v0 + (v1 - v0) * t01,
                    Self::lerp_color(c0, c1, t01),
                )
            } else {
                let t12 = if (y2 - y1).abs() < 0.001 {
                    0.0
                } else {
                    ((y - y1) / (y2 - y1)).clamp(0.0, 1.0)
                };
                (
                    x1 + (x2 - x1) * t12,
                    u1 + (u2 - u1) * t12,
                    v1 + (v2 - v1) * t12,
                    Self::lerp_color(c1, c2, t12),
                )
            };
            let (x_l, x_r, u_l, u_r, v_l, v_r, c_l, c_r) = if x_long < x_short {
                (
                    x_long, x_short, u_long, u_short, v_long, v_short, c_long, c_short,
                )
            } else {
                (
                    x_short, x_long, u_short, u_long, v_short, v_long, c_short, c_long,
                )
            };
            let sx_lo = clip.x.max(x_l.ceil() as i32);
            let sx_hi = (clip.x + clip.width).min(x_r.floor() as i32 + 1);
            let span = (x_r - x_l).max(0.001);
            for sx in sx_lo..sx_hi {
                let t = ((sx as f32 - x_l) / span).clamp(0.0, 1.0);
                let px = Self::lerp_color(c_l, c_r, t);
                let vtx_color = Color::new(
                    ((px >> 16) & 0xFF) as u8,
                    ((px >> 8) & 0xFF) as u8,
                    (px & 0xFF) as u8,
                    ((px >> 24) & 0xFF) as u8,
                );
                // 纹理采样（u/v 全图坐标，顶点色调制）
                let mut src_color = vtx_color;
                if let Some(tex) = texture {
                    if !tex.pixels.is_empty() {
                        let u = (u_l + (u_r - u_l) * t).clamp(0.0, 1.0);
                        let v = (v_l + (v_r - v_l) * t).clamp(0.0, 1.0);
                        let tx_px = ((u * tex_w as f32) as i32).clamp(0, tex_w - 1);
                        let ty_px = ((v * tex_h as f32) as i32).clamp(0, tex_h - 1);
                        let tidx = ((ty_px * tex_w + tx_px) * 4) as usize;
                        if tidx + 3 < tex.pixels.len() {
                            let tex_color = Color::new(
                                tex.pixels[tidx],
                                tex.pixels[tidx + 1],
                                tex.pixels[tidx + 2],
                                tex.pixels[tidx + 3],
                            );
                            // 顶点色调制纹理色（分量相乘）
                            src_color = Color::new(
                                (vtx_color.r as u32 * tex_color.r as u32 / 255) as u8,
                                (vtx_color.g as u32 * tex_color.g as u32 / 255) as u8,
                                (vtx_color.b as u32 * tex_color.b as u32 / 255) as u8,
                                (vtx_color.a as u32 * tex_color.a as u32 / 255) as u8,
                            );
                        }
                    }
                }
                let final_color = self.blend_color(&src_color, tint, draw_mode);
                self.set_pixel(sx, sy, &final_color, draw_mode);
            }
        }
    }

    /// 顶点色线性插值（ARGB u32）
    fn lerp_color(c0: u32, c1: u32, t: f32) -> u32 {
        let t = t.clamp(0.0, 1.0);
        let lerp_ch = |a: u32, b: u32| (a as f32 + (b as f32 - a as f32) * t) as u32;
        (lerp_ch((c0 >> 24) & 0xFF, (c1 >> 24) & 0xFF) << 24)
            | (lerp_ch((c0 >> 16) & 0xFF, (c1 >> 16) & 0xFF) << 16)
            | (lerp_ch((c0 >> 8) & 0xFF, (c1 >> 8) & 0xFF) << 8)
            | lerp_ch(c0 & 0xFF, c1 & 0xFF)
    }

    /// 矩阵变换 blit（对应 C++ Image::BltMatrix，软件路径）
    /// 源矩形 4 角经 SexyMatrix3 变换为屏幕 4 角，切成两个纹理三角形
    pub fn blt_matrix(
        &mut self,
        src: &Image,
        x: f32,
        y: f32,
        matrix: &crate::framework::sexy_matrix::SexyMatrix3,
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        src_rect: &Rect,
        _blend: bool,
    ) {
        if src.pixels.is_empty() || self.pixels.is_empty() {
            return;
        }
        if src_rect.width <= 0 || src_rect.height <= 0 {
            return;
        }
        let sw = src_rect.width as f32;
        let sh = src_rect.height as f32;
        let u0 = src_rect.x as f32 / src.width.max(1) as f32;
        let v0 = src_rect.y as f32 / src.height.max(1) as f32;
        let u1 = (src_rect.x + src_rect.width) as f32 / src.width.max(1) as f32;
        let v1 = (src_rect.y + src_rect.height) as f32 / src.height.max(1) as f32;
        let tf = |px: f32, py: f32| -> (f32, f32) {
            let tx = matrix.m[0][0] * px + matrix.m[0][1] * py + matrix.m[0][2] + x;
            let ty = matrix.m[1][0] * px + matrix.m[1][1] * py + matrix.m[1][2] + y;
            (tx, ty)
        };
        let (ax, ay) = tf(0.0, 0.0);
        let (bx, by) = tf(sw, 0.0);
        let (cx, cy) = tf(0.0, sh);
        let (dx, dy) = tf(sw, sh);
        let white = 0xFFFFFFFFu32;
        let verts = [
            [
                crate::framework::graphics::gl_interface::TriVertex {
                    x: ax,
                    y: ay,
                    u: u0,
                    v: v0,
                    color: white,
                },
                crate::framework::graphics::gl_interface::TriVertex {
                    x: bx,
                    y: by,
                    u: u1,
                    v: v0,
                    color: white,
                },
                crate::framework::graphics::gl_interface::TriVertex {
                    x: cx,
                    y: cy,
                    u: u0,
                    v: v1,
                    color: white,
                },
            ],
            [
                crate::framework::graphics::gl_interface::TriVertex {
                    x: cx,
                    y: cy,
                    u: u0,
                    v: v1,
                    color: white,
                },
                crate::framework::graphics::gl_interface::TriVertex {
                    x: bx,
                    y: by,
                    u: u1,
                    v: v0,
                    color: white,
                },
                crate::framework::graphics::gl_interface::TriVertex {
                    x: dx,
                    y: dy,
                    u: u1,
                    v: v1,
                    color: white,
                },
            ],
        ];
        self.blt_triangles_impl(Some(src), &verts, 2, clip_rect, color, draw_mode, 0.0, 0.0);
    }

    /// 镜像 Blt
    pub fn blt_mirror(
        &mut self,
        src: &Image,
        dest_x: i32,
        dest_y: i32,
        src_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        _linear_filter: bool,
    ) {
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
    /// [TRANSLATION_NOTE]: C++ 的 MemoryImage **没有** StretchBltMirror（只有 StretchBltMirrorClipHelper；
    /// 基类 Image::StretchBltMirror 是空实现，真正实现仅在 GLImage）。Rust 侧 GLImage 从未被实例化
    /// （报告 §5.10），若照抄空实现会导致镜像拉伸完全不绘制，故保留 Rust 既有的软件绘制，
    /// 并按 C++ 的 StretchBltMirrorClipHelper 语义补上裁剪（mirror=true）。
    pub fn stretch_blt_mirror(
        &mut self,
        src: &Image,
        dest_rect: &Rect,
        src_rect: &Rect,
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        _fast_stretch: bool,
    ) {
        if src.pixels.is_empty() || self.pixels.is_empty() {
            return;
        }
        if dest_rect.width <= 0
            || dest_rect.height <= 0
            || src_rect.width <= 0
            || src_rect.height <= 0
        {
            return;
        }

        let (a_dest_rect, a_src_rect) =
            match Self::stretch_blt_clip_helper(src_rect, clip_rect, dest_rect, true) {
                Some(r) => r,
                None => return,
            };

        let (a_src_x0, a_src_y0, a_src_wf, a_src_hf) = a_src_rect;
        let a_src_w = src.width as i32;

        let scale_x = a_src_wf / a_dest_rect.width as f64;
        let scale_y = a_src_hf / a_dest_rect.height as f64;

        for dy in 0..a_dest_rect.height {
            for dx in 0..a_dest_rect.width {
                // 水平镜像：从裁剪后源矩形的右端起算
                let sx = a_src_x0 as i32 + a_src_wf as i32 - 1 - (dx as f64 * scale_x) as i32;
                let sy = a_src_y0 as i32 + (dy as f64 * scale_y) as i32;
                let src_idx_i = sy * a_src_w + sx;
                let dst_x = a_dest_rect.x + dx;
                let dst_y = a_dest_rect.y + dy;
                if src_idx_i < 0 {
                    continue;
                }
                if dst_x < 0 || dst_x >= self.width || dst_y < 0 || dst_y >= self.height {
                    continue;
                }
                let src_idx = src_idx_i as usize;
                if src_idx * 4 + 3 >= src.pixels.len() {
                    continue;
                }
                let src_color = Color::new(
                    src.pixels[src_idx * 4],
                    src.pixels[src_idx * 4 + 1],
                    src.pixels[src_idx * 4 + 2],
                    src.pixels[src_idx * 4 + 3],
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
                            let r = ((color.r as u32 * sa + self.pixels[idx] as u32 * (255 - sa))
                                / 255) as u8;
                            let g = ((color.g as u32 * sa
                                + self.pixels[idx + 1] as u32 * (255 - sa))
                                / 255) as u8;
                            let b = ((color.b as u32 * sa
                                + self.pixels[idx + 2] as u32 * (255 - sa))
                                / 255) as u8;
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
