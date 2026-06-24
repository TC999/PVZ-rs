// PvZ Portable Rust 翻译 — 2D 图形上下文
// 对应 C++ SexyAppFramework/graphics/Graphics.h / Graphics.cpp
//
// Graphics 是整个框架的核心绘制类，负责任何 2D 绘制操作。
// 它处理坐标变换、裁剪、状态栈管理，并将实际绘制委托给目标 Image。
//
// 在 C++ 中 Graphics 同时继承 GraphicsState（状态数据）并添加了多边形填充
// 相关的扫描线算法字段。Rust 版直接在 Graphics 结构体中包含所有字段。

#![allow(dead_code)]

use std::cmp;

use crate::framework::color::Color;
use crate::framework::point::Point;
use crate::framework::rect::Rect;
use crate::framework::graphics::image::{Image, ImageKind, Span};
use crate::framework::graphics::font::Font;
use crate::framework::sexy_matrix::{SexyMatrix3, Transform};

/// 最大临时扫描线数量（对应 C++ MAX_TEMP_SPANS = 8192）
const MAX_TEMP_SPANS: usize = 8192;

/// 边缘结构（用于多边形填充扫描线算法）
#[derive(Debug, Clone, Copy)]
struct Edge {
    x: f64,
    dx: f64,
    i: i32,
    b: f64,
}

/// 绘制模式（对应 C++ Graphics::DRAWMODE_*）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DrawMode {
    Normal = 0,
    Additive = 1,
    Copy = 2,
}

/// 2D 图形上下文
///
/// 对应 C++ 的 GraphicsState + Graphics 合并。
/// 字段命名遵循 C++ 命名风格以便对照。
#[derive(Debug)]
pub struct Graphics {
    // ---- GraphicsState 字段 ----
    /// 目标绘制图像
    pub dest_image: *mut Image,
    /// 平移 X
    pub trans_x: f64,
    /// 平移 Y
    pub trans_y: f64,
    /// 缩放 X
    pub scale_x: f64,
    /// 缩放 Y
    pub scale_y: f64,
    /// 缩放原点 X
    pub scale_orig_x: f64,
    /// 缩放原点 Y
    pub scale_orig_y: f64,
    /// 裁剪矩形
    pub clip_rect: Rect,
    /// 当前颜色
    pub color: Color,
    /// 当前字体（裸指针，因 Font 在运行时动态确定）
    pub font: *mut Font,
    /// 绘制模式
    pub draw_mode: i32,
    /// 是否对图像染色
    pub colorize_images: bool,
    /// 是否使用快速拉伸
    pub fast_stretch: bool,
    /// 是否写入着色字符串（^RRGGBB^ 标签）
    pub write_colored_string: bool,
    /// 是否使用线性混合
    pub linear_blend: bool,
    /// 是否为 3D 模式
    pub is_3d: bool,

    // ---- Graphics 独有字段 ----
    /// 多边形填充：激活边缘列表
    active_edge_list: Vec<Edge>,
    /// 多边形填充：激活边缘数量
    num_active_edges: i32,
    /// 多边形填充：顶点指针（临时引用外部数组）
    pf_points: *const Point,
    /// 多边形填充：顶点数量
    num_vertices: i32,

    /// 状态栈（对应 C++ mStateStack）
    state_stack: Vec<GraphicsState>,
}

/// 可复制的图形状态快照（对应 C++ GraphicsState 结构体）
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct GraphicsState {
    dest_image: *mut Image,
    trans_x: f64,
    trans_y: f64,
    scale_x: f64,
    scale_y: f64,
    scale_orig_x: f64,
    scale_orig_y: f64,
    clip_rect: Rect,
    color: Color,
    font: *mut Font,
    draw_mode: i32,
    colorize_images: bool,
    fast_stretch: bool,
    write_colored_string: bool,
    linear_blend: bool,
    is_3d: bool,
}

impl Graphics {
    /// 创建 Graphics 实例（对应 C++ Graphics::Graphics(Image* theDestImage)）
    pub fn new_with_image(dest_image: *mut Image) -> Self {
        let mut g = Graphics {
            dest_image,
            trans_x: 0.0,
            trans_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            scale_orig_x: 0.0,
            scale_orig_y: 0.0,
            clip_rect: Rect::ZERO,
            color: Color::WHITE,
            font: std::ptr::null_mut(),
            draw_mode: 0, // DRAWMODE_NORMAL
            colorize_images: false,
            fast_stretch: false,
            write_colored_string: true,
            linear_blend: true,
            is_3d: false,
            active_edge_list: Vec::new(),
            num_active_edges: 0,
            pf_points: std::ptr::null(),
            num_vertices: 0,
            state_stack: Vec::new(),
        };

        // 初始化裁剪矩形
        g.update_clip_rect_from_dest();
        g
    }

    /// 创建 Graphics 实例（无目标图像，对应 C++ 默认构造函数 Graphics()）
    pub fn new() -> Self {
        Self::new_with_image(std::ptr::null_mut())
    }

    /// 从目标图像更新裁剪矩形
    fn update_clip_rect_from_dest(&mut self) {
        unsafe {
            if !self.dest_image.is_null() {
                let img = &*self.dest_image;
                self.clip_rect = Rect::new(0, 0, img.width, img.height);
                self.is_3d = img.image_kind == ImageKind::GLImage;
            } else {
                self.clip_rect = Rect::ZERO;
                self.is_3d = false;
            }
        }
    }

    // ====================================================================
    // 状态栈管理
    // ====================================================================

    /// 将当前状态压入栈（对应 C++ PushState）
    pub fn push_state(&mut self) {
        let state = self.save_state();
        self.state_stack.push(state);
    }

    /// 从栈中恢复状态（对应 C++ PopState）
    pub fn pop_state(&mut self) {
        if let Some(state) = self.state_stack.pop() {
            self.restore_state(&state);
        }
    }

    /// 创建当前状态的快照
    fn save_state(&self) -> GraphicsState {
        GraphicsState {
            dest_image: self.dest_image,
            trans_x: self.trans_x,
            trans_y: self.trans_y,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            scale_orig_x: self.scale_orig_x,
            scale_orig_y: self.scale_orig_y,
            clip_rect: self.clip_rect,
            color: self.color,
            font: self.font,
            draw_mode: self.draw_mode,
            colorize_images: self.colorize_images,
            fast_stretch: self.fast_stretch,
            write_colored_string: self.write_colored_string,
            linear_blend: self.linear_blend,
            is_3d: self.is_3d,
        }
    }

    /// 从快照恢复状态
    fn restore_state(&mut self, state: &GraphicsState) {
        self.dest_image = state.dest_image;
        self.trans_x = state.trans_x;
        self.trans_y = state.trans_y;
        self.scale_x = state.scale_x;
        self.scale_y = state.scale_y;
        self.scale_orig_x = state.scale_orig_x;
        self.scale_orig_y = state.scale_orig_y;
        self.clip_rect = state.clip_rect;
        self.color = state.color;
        self.font = state.font;
        self.draw_mode = state.draw_mode;
        self.colorize_images = state.colorize_images;
        self.fast_stretch = state.fast_stretch;
        self.write_colored_string = state.write_colored_string;
        self.linear_blend = state.linear_blend;
        self.is_3d = state.is_3d;
    }

    /// 创建当前 Graphics 副本（对应 C++ Create）
    pub fn create(&self) -> Self {
        Graphics {
            dest_image: self.dest_image,
            trans_x: self.trans_x,
            trans_y: self.trans_y,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            scale_orig_x: self.scale_orig_x,
            scale_orig_y: self.scale_orig_y,
            clip_rect: self.clip_rect,
            color: self.color,
            font: self.font,
            draw_mode: self.draw_mode,
            colorize_images: self.colorize_images,
            fast_stretch: self.fast_stretch,
            write_colored_string: self.write_colored_string,
            linear_blend: self.linear_blend,
            is_3d: self.is_3d,
            active_edge_list: Vec::new(),
            num_active_edges: 0,
            pf_points: std::ptr::null(),
            num_vertices: 0,
            state_stack: Vec::new(),
        }
    }


    // ====================================================================
    // 字体和颜色 Getter/Setter
    // ====================================================================

    pub fn set_font(&mut self, font: *mut Font) {
        self.font = font;
    }

    pub fn get_font(&self) -> *mut Font {
        self.font
    }

    pub fn set_color(&mut self, color: &Color) {
        self.color = *color;
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn set_draw_mode(&mut self, mode: i32) {
        self.draw_mode = mode;
    }

    pub fn get_draw_mode(&self) -> i32 {
        self.draw_mode
    }

    pub fn set_colorize_images(&mut self, colorize: bool) {
        self.colorize_images = colorize;
    }

    pub fn get_colorize_images(&self) -> bool {
        self.colorize_images
    }

    pub fn set_fast_stretch(&mut self, fast: bool) {
        self.fast_stretch = fast;
    }

    pub fn get_fast_stretch(&self) -> bool {
        self.fast_stretch
    }

    pub fn set_linear_blend(&mut self, linear: bool) {
        self.linear_blend = linear;
    }

    pub fn get_linear_blend(&self) -> bool {
        self.linear_blend
    }

    // ====================================================================
    // 矩形绘制
    // ====================================================================

    /// 清除矩形区域（对应 C++ ClearRect）
    pub fn clear_rect_xywh(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let dest_rect = Rect::new(x + self.trans_x as i32, y + self.trans_y as i32, width, height)
            .intersection(&self.clip_rect);
        if dest_rect.is_zero() {
            return;
        }
        unsafe {
            if !self.dest_image.is_null() {
                (*self.dest_image).clear_rect(&dest_rect);
            }
        }
    }

    pub fn clear_rect(&mut self, rect: &Rect) {
        self.clear_rect_xywh(rect.x, rect.y, rect.width, rect.height);
    }

    /// 填充矩形（对应 C++ FillRect）
    pub fn fill_rect_xywh(&mut self, x: i32, y: i32, width: i32, height: i32) {
        if self.color.a == 0 {
            return;
        }
        let dest_rect = Rect::new(x + self.trans_x as i32, y + self.trans_y as i32, width, height)
            .intersection(&self.clip_rect);
        if dest_rect.is_zero() {
            return;
        }
        unsafe {
            if !self.dest_image.is_null() {
                (*self.dest_image).fill_rect(&dest_rect, &self.color, self.draw_mode);
            }
        }
    }

    pub fn fill_rect(&mut self, rect: &Rect) {
        self.fill_rect_xywh(rect.x, rect.y, rect.width, rect.height);
    }

    /// 绘制矩形边框（对应 C++ DrawRect）
    pub fn draw_rect_xywh(&mut self, x: i32, y: i32, width: i32, height: i32) {
        if self.color.a == 0 {
            return;
        }
        let dest_rect = Rect::new(x + self.trans_x as i32, y + self.trans_y as i32, width, height);
        let full_dest = Rect::new(dest_rect.x, dest_rect.y, width + 1, height + 1);
        let clipped = full_dest.intersection(&self.clip_rect);

        if full_dest == clipped {
            unsafe {
                if !self.dest_image.is_null() {
                    (*self.dest_image).draw_rect(&dest_rect, &self.color, self.draw_mode);
                }
            }
        } else {
            // 当矩形被裁剪时，分别绘制四条边
            self.fill_rect_xywh(x, y, width + 1, 1);
            self.fill_rect_xywh(x, y + height, width + 1, 1);
            self.fill_rect_xywh(x, y + 1, 1, height - 1);
            self.fill_rect_xywh(x + width, y + 1, 1, height - 1);
        }
    }

    pub fn draw_rect(&mut self, rect: &Rect) {
        self.draw_rect_xywh(rect.x, rect.y, rect.width, rect.height);
    }

    // ====================================================================
    // 线段绘制
    // ====================================================================

    /// 线段裁剪辅助（对应 C++ DrawLineClipHelper）
    /// 使用 Cohen-Sutherland 风格裁剪
    fn draw_line_clip_helper(
        &self,
        start_x: &mut f64,
        start_y: &mut f64,
        end_x: &mut f64,
        end_y: &mut f64,
    ) -> bool {
        let mut sx = *start_x;
        let mut sy = *start_y;
        let mut ex = *end_x;
        let mut ey = *end_y;

        // 裁剪 X
        if sx > ex {
            std::mem::swap(&mut sx, &mut ex);
            std::mem::swap(&mut sy, &mut ey);
        }

        let clip_left = self.clip_rect.x as f64;
        let clip_right = (self.clip_rect.x + self.clip_rect.width) as f64;
        let clip_top = self.clip_rect.y as f64;
        let clip_bottom = (self.clip_rect.y + self.clip_rect.height) as f64;

        if sx < clip_left {
            if ex < clip_left {
                return false;
            }
            let slope = (ey - sy) / (ex - sx);
            sy += (clip_left - sx) * slope;
            sx = clip_left;
        }

        if ex >= clip_right {
            if sx >= clip_right {
                return false;
            }
            let slope = (ey - sy) / (ex - sx);
            ey += (clip_right - 1.0 - ex) * slope;
            ex = clip_right - 1.0;
        }

        // 裁剪 Y
        if sy > ey {
            std::mem::swap(&mut sx, &mut ex);
            std::mem::swap(&mut sy, &mut ey);
        }

        if sy < clip_top {
            if ey < clip_top {
                return false;
            }
            let slope = (ex - sx) / (ey - sy);
            sx += (clip_top - sy) * slope;
            sy = clip_top;
        }

        if ey >= clip_bottom {
            if sy >= clip_bottom {
                return false;
            }
            let slope = (ex - sx) / (ey - sy);
            ex += (clip_bottom - 1.0 - ey) * slope;
            ey = clip_bottom - 1.0;
        }

        *start_x = sx;
        *start_y = sy;
        *end_x = ex;
        *end_y = ey;
        true
    }

    /// 绘制线段（对应 C++ DrawLine）
    pub fn draw_line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
        let mut sx = start_x as f64 + self.trans_x;
        let mut sy = start_y as f64 + self.trans_y;
        let mut ex = end_x as f64 + self.trans_x;
        let mut ey = end_y as f64 + self.trans_y;

        if !self.draw_line_clip_helper(&mut sx, &mut sy, &mut ex, &mut ey) {
            return;
        }

        unsafe {
            if !self.dest_image.is_null() {
                (*self.dest_image).draw_line(sx, sy, ex, ey, &self.color, self.draw_mode);
            }
        }
    }

    /// 绘制抗锯齿线段（对应 C++ DrawLineAA）
    pub fn draw_line_aa(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
        let mut sx = start_x as f64 + self.trans_x;
        let mut sy = start_y as f64 + self.trans_y;
        let mut ex = end_x as f64 + self.trans_x;
        let mut ey = end_y as f64 + self.trans_y;

        if !self.draw_line_clip_helper(&mut sx, &mut sy, &mut ex, &mut ey) {
            return;
        }

        unsafe {
            if !self.dest_image.is_null() {
                (*self.dest_image).draw_line_aa(sx, sy, ex, ey, &self.color, self.draw_mode);
            }
        }
    }

    // ====================================================================
    // 字符串绘制
    // ====================================================================

    /// 绘制字符串（对应 C++ DrawString）
    pub fn draw_string(&mut self, text: &str, x: i32, y: i32) {
        let font_ptr = self.font;
        let color = self.color;
        let clip_rect = self.clip_rect;
        if !font_ptr.is_null() {
            unsafe {
                let font = &*font_ptr;
                font.draw_string(self, x, y, text, &color, &clip_rect);
            }
        }
    }

    /// 字符串宽度（对应 C++ StringWidth）
    pub fn string_width(&self, text: &str) -> i32 {
        unsafe {
            if !self.font.is_null() {
                (*self.font).string_width(text)
            } else {
                0
            }
        }
    }

    // ====================================================================
    // 图像绘制（核心）
    // ====================================================================

    fn get_image_color(&self) -> Color {
        if self.colorize_images {
            self.color
        } else {
            Color::WHITE
        }
    }

    /// DrawImage(image, x, y) — 简单绘制（对应 C++ DrawImage #1）
    pub fn draw_image_xy(&mut self, image: &Image, x: i32, y: i32) {
        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();

            // 检查缩放
            if (self.scale_x - 1.0).abs() > 0.001 || (self.scale_y - 1.0).abs() > 0.001 {
                let src_rect = image.get_rect();
                self.draw_image_scale_raw(image, x, y, &src_rect);
                return;
            }

            let tx = x + self.trans_x as i32;
            let ty = y + self.trans_y as i32;

            let dest_rect = Rect::new(tx, ty, image.width, image.height).intersection(&self.clip_rect);
            if dest_rect.is_zero() {
                return;
            }
            let src_rect = Rect::new(
                dest_rect.x - tx,
                dest_rect.y - ty,
                dest_rect.width,
                dest_rect.height,
            );

            if src_rect.width > 0 && src_rect.height > 0 {
                dest.blt(image, dest_rect.x, dest_rect.y, &src_rect, &draw_color, self.draw_mode, self.linear_blend);
            }
        }
    }

    /// DrawImage(image, x, y, srcRect)（对应 C++ DrawImage #2）
    pub fn draw_image_src(&mut self, image: &Image, x: i32, y: i32, src_rect: &Rect) {
        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();

            // 边界检查
            if src_rect.x + src_rect.width > image.width || src_rect.y + src_rect.height > image.height {
                return;
            }

            let tx = x + self.trans_x as i32;
            let ty = y + self.trans_y as i32;

            // 检查缩放
            if (self.scale_x - 1.0).abs() > 0.001 || (self.scale_y - 1.0).abs() > 0.001 {
                let dest_rect = Rect::new(
                    (self.scale_orig_x + (tx as f64 - self.scale_orig_x) * self.scale_x).floor() as i32,
                    (self.scale_orig_y + (ty as f64 - self.scale_orig_y) * self.scale_y).floor() as i32,
                    (src_rect.width as f64 * self.scale_x).ceil() as i32,
                    (src_rect.height as f64 * self.scale_y).ceil() as i32,
                );
                dest.stretch_blt(image, &dest_rect, src_rect, &self.clip_rect, &draw_color, self.draw_mode, self.fast_stretch);
                return;
            }

            let dest_rect = Rect::new(tx, ty, src_rect.width, src_rect.height).intersection(&self.clip_rect);
            if dest_rect.is_zero() {
                return;
            }
            let adjusted_src = Rect::new(
                src_rect.x + dest_rect.x - tx,
                src_rect.y + dest_rect.y - ty,
                dest_rect.width,
                dest_rect.height,
            );

            if adjusted_src.width > 0 && adjusted_src.height > 0 {
                dest.blt(image, dest_rect.x, dest_rect.y, &adjusted_src, &draw_color, self.draw_mode, self.linear_blend);
            }
        }
    }

    /// DrawImage(image, destRect, srcRect)（对应 C++ DrawImage #3 — StretchBlt）
    pub fn draw_image_stretch(&mut self, image: &Image, dest_rect: &Rect, src_rect: &Rect) {
        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();
            let adjusted_dest = Rect::new(
                dest_rect.x + self.trans_x as i32,
                dest_rect.y + self.trans_y as i32,
                dest_rect.width,
                dest_rect.height,
            );
            dest.stretch_blt(image, &adjusted_dest, src_rect, &self.clip_rect, &draw_color, self.draw_mode, self.fast_stretch);
        }
    }

    /// DrawImage(image, x, y, stretchedW, stretchedH)（对应 C++ DrawImage #4）
    pub fn draw_image_stretch_xy(&mut self, image: &Image, x: i32, y: i32, sw: i32, sh: i32) {
        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();
            let dest_rect = Rect::new(
                x + self.trans_x as i32,
                y + self.trans_y as i32,
                sw,
                sh,
            );
            let src_rect = Rect::new(0, 0, image.width, image.height);
            dest.stretch_blt(image, &dest_rect, &src_rect, &self.clip_rect, &draw_color, self.draw_mode, self.fast_stretch);
        }
    }

    /// 带缩放的内部辅助（用于 DrawImage 检查到 scale != 1 时）
    fn draw_image_scale_raw(&mut self, image: &Image, x: i32, y: i32, src_rect: &Rect) {
        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();
            let tx = x + self.trans_x as i32;
            let ty = y + self.trans_y as i32;
            let dest_rect = Rect::new(
                (self.scale_orig_x + (tx as f64 - self.scale_orig_x) * self.scale_x).floor() as i32,
                (self.scale_orig_y + (ty as f64 - self.scale_orig_y) * self.scale_y).floor() as i32,
                (src_rect.width as f64 * self.scale_x).ceil() as i32,
                (src_rect.height as f64 * self.scale_y).ceil() as i32,
            );
            dest.stretch_blt(image, &dest_rect, src_rect, &self.clip_rect, &draw_color, self.draw_mode, self.fast_stretch);
        }
    }

    /// DrawImageF（浮点坐标，对应 C++ DrawImageF #1）
    pub fn draw_image_f_xy(&mut self, image: &Image, x: f32, y: f32) {
        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();
            let tx = x + self.trans_x as f32;
            let ty = y + self.trans_y as f32;
            let src_rect = image.get_rect();
            dest.blt_f(image, tx, ty, &src_rect, &self.clip_rect, &draw_color, self.draw_mode);
        }
    }

    /// DrawImageF（浮点坐标 + 源矩形，对应 C++ DrawImageF #2）
    pub fn draw_image_f_src(&mut self, image: &Image, x: f32, y: f32, src_rect: &Rect) {
        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();
            let tx = x + self.trans_x as f32;
            let ty = y + self.trans_y as f32;
            dest.blt_f(image, tx, ty, src_rect, &self.clip_rect, &draw_color, self.draw_mode);
        }
    }

    // ====================================================================
    // 镜像绘制
    // ====================================================================

    /// DrawImageMirror(image, x, y, mirror)（对应 C++）
    pub fn draw_image_mirror(&mut self, image: &Image, x: i32, y: i32, mirror: bool) {
        let src_rect = image.get_rect();
        self.draw_image_mirror_src(image, x, y, &src_rect, mirror);
    }

    /// DrawImageMirror(image, x, y, srcRect, mirror)（对应 C++）
    pub fn draw_image_mirror_src(&mut self, image: &Image, x: i32, y: i32, src_rect: &Rect, mirror: bool) {
        if !mirror {
            self.draw_image_src(image, x, y, src_rect);
            return;
        }

        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();
            let tx = x + self.trans_x as i32;
            let ty = y + self.trans_y as i32;

            if src_rect.x + src_rect.width > image.width || src_rect.y + src_rect.height > image.height {
                return;
            }

            let dest_rect = Rect::new(tx, ty, src_rect.width, src_rect.height).intersection(&self.clip_rect);
            let total_clip = src_rect.width - dest_rect.width;
            let left_clip = dest_rect.x - tx;
            let right_clip = total_clip - left_clip;

            let adjusted_src = Rect::new(
                src_rect.x + right_clip,
                src_rect.y + dest_rect.y - ty,
                dest_rect.width,
                dest_rect.height,
            );

            if adjusted_src.width > 0 && adjusted_src.height > 0 {
                dest.blt_mirror(image, dest_rect.x, dest_rect.y, &adjusted_src, &draw_color, self.draw_mode, self.linear_blend);
            }
        }
    }

    /// DrawImageMirror(image, destRect, srcRect, mirror)（对应 C++）
    pub fn draw_image_mirror_stretch(&mut self, image: &Image, dest_rect: &Rect, src_rect: &Rect, mirror: bool) {
        if !mirror {
            self.draw_image_stretch(image, dest_rect, src_rect);
            return;
        }

        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();
            let adjusted_dest = Rect::new(
                dest_rect.x + self.trans_x as i32,
                dest_rect.y + self.trans_y as i32,
                dest_rect.width,
                dest_rect.height,
            );
            dest.stretch_blt_mirror(image, &adjusted_dest, src_rect, &self.clip_rect, &draw_color, self.draw_mode, self.fast_stretch);
        }
    }

    // ====================================================================
    // 旋转绘制
    // ====================================================================

    /// DrawImageRotated(image, x, y, rot, srcRect)（对应 C++）
    pub fn draw_image_rotated(&mut self, image: &Image, x: i32, y: i32, rot: f64, src_rect: Option<&Rect>) {
        let (cx, cy) = match src_rect {
            Some(r) => (r.width as f32 / 2.0, r.height as f32 / 2.0),
            None => (image.width as f32 / 2.0, image.height as f32 / 2.0),
        };
        self.draw_image_rotated_f(image, x as f32, y as f32, rot, cx, cy, src_rect);
    }

    /// DrawImageRotatedF(image, x, y, rot, srcRect)（对应 C++）
    pub fn draw_image_rotated_f_simple(&mut self, image: &Image, x: f32, y: f32, rot: f64, src_rect: Option<&Rect>) {
        let (cx, cy) = match src_rect {
            Some(r) => (r.width as f32 / 2.0, r.height as f32 / 2.0),
            None => (image.width as f32 / 2.0, image.height as f32 / 2.0),
        };
        self.draw_image_rotated_f(image, x, y, rot, cx, cy, src_rect);
    }

    /// DrawImageRotated / DrawImageRotatedF（完整参数）的统一实现
    pub fn draw_image_rotated_f(
        &mut self,
        image: &Image,
        x: f32,
        y: f32,
        rot: f64,
        rot_center_x: f32,
        rot_center_y: f32,
        src_rect: Option<&Rect>,
    ) {
        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();
            let tx = x + self.trans_x as f32;
            let ty = y + self.trans_y as f32;

            match src_rect {
                Some(r) => {
                    dest.blt_rotated(image, tx, ty, r, &self.clip_rect, &draw_color, self.draw_mode, rot, rot_center_x, rot_center_y);
                }
                None => {
                    let r = image.get_rect();
                    dest.blt_rotated(image, tx, ty, &r, &self.clip_rect, &draw_color, self.draw_mode, rot, rot_center_x, rot_center_y);
                }
            }
        }
    }

    // ====================================================================
    // 矩阵变换绘制
    // ====================================================================

    /// DrawImageMatrix(image, matrix, x, y)（对应 C++）
    pub fn draw_image_matrix(&mut self, image: &Image, matrix: &SexyMatrix3, x: f32, y: f32) {
        let src_rect = image.get_rect();
        self.draw_image_matrix_src(image, matrix, &src_rect, x, y);
    }

    /// DrawImageMatrix(image, matrix, srcRect, x, y)（对应 C++）
    pub fn draw_image_matrix_src(&mut self, image: &Image, matrix: &SexyMatrix3, src_rect: &Rect, x: f32, y: f32) {
        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();
            dest.blt_matrix(
                image, x + self.trans_x as f32, y + self.trans_y as f32,
                matrix, &self.clip_rect, &draw_color, self.draw_mode,
                src_rect, self.linear_blend,
            );
        }
    }

    // ====================================================================
    // Transform 辅助（对应 C++ DrawImageTransformHelper）
    // ====================================================================

    /// DrawImageTransformHelper — 将 Transform 分解为具体的绘制调用
    pub fn draw_image_transform_helper(
        &mut self,
        image: &Image,
        transform: &Transform,
        src_rect: &Rect,
        x: f32,
        y: f32,
        use_float: bool,
    ) {
        if transform.complex {
            // 复杂变换 → 使用矩阵方法
            self.draw_image_matrix_src(image, &transform.get_matrix(), src_rect, x, y);
            return;
        }

        let w2 = src_rect.width as f32 / 2.0;
        let h2 = src_rect.height as f32 / 2.0;

        if transform.have_rot {
            let rx = w2 - transform.trans_x1;
            let ry = h2 - transform.trans_y1;
            let nx = x + transform.trans_x2 - rx + 0.5;
            let ny = y + transform.trans_y2 - ry + 0.5;

            if use_float {
                self.draw_image_rotated_f(image, nx, ny, transform.rot as f64, rx, ry, Some(src_rect));
            } else {
                self.draw_image_rotated_f(image, nx, ny, transform.rot as f64, rx, ry, Some(src_rect));
            }
        } else if transform.have_scale {
            let mut mirror = false;
            if (transform.scale_x - (-1.0)).abs() < 0.001 {
                if (transform.scale_y - 1.0).abs() < 0.001 {
                    let nx = (x + transform.trans_x1 + transform.trans_x2 - w2 + 0.5) as i32;
                    let ny = (y + transform.trans_y1 + transform.trans_y2 - h2 + 0.5) as i32;
                    self.draw_image_mirror_src(image, nx, ny, src_rect, true);
                    return;
                }
                mirror = true;
            }

            let sw = w2 * transform.scale_x;
            let sh = h2 * transform.scale_y;
            let nx = x + transform.trans_x2 - sw;
            let ny = y + transform.trans_y2 - sh;
            let dest_rect = Rect::new(nx as i32, ny as i32, (sw * 2.0) as i32, (sh * 2.0) as i32);
            self.draw_image_mirror_stretch(image, &dest_rect, src_rect, mirror);
        } else {
            let nx = x + transform.trans_x1 + transform.trans_x2 - w2 + 0.5;
            let ny = y + transform.trans_y1 + transform.trans_y2 - h2 + 0.5;
            if use_float {
                self.draw_image_f_src(image, nx, ny, src_rect);
            } else {
                self.draw_image_src(image, nx as i32, ny as i32, src_rect);
            }
        }
    }

    // ====================================================================
    // 三角纹理绘制
    // ====================================================================

    /// DrawTriangleTex（对应 C++）
    pub fn draw_triangle_tex(
        &mut self,
        texture: &Image,
        v1: &crate::framework::graphics::gl_interface::TriVertex,
        v2: &crate::framework::graphics::gl_interface::TriVertex,
        v3: &crate::framework::graphics::gl_interface::TriVertex,
    ) {
        let vertices = [[v1.clone(), v2.clone(), v3.clone()]];
        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();
            dest.blt_triangles_tex(
                texture, &vertices, 1, &self.clip_rect, &draw_color,
                self.draw_mode, self.trans_x as f32, self.trans_y as f32, self.linear_blend,
            );
        }
    }

    /// DrawTrianglesTex（对应 C++）
    pub fn draw_triangles_tex(
        &mut self,
        texture: &Image,
        vertices: &[[crate::framework::graphics::gl_interface::TriVertex; 3]],
        num_triangles: i32,
    ) {
        unsafe {
            let dest = &mut *self.dest_image;
            let draw_color = self.get_image_color();
            dest.blt_triangles_tex(
                texture, vertices, num_triangles, &self.clip_rect, &draw_color,
                self.draw_mode, self.trans_x as f32, self.trans_y as f32, self.linear_blend,
            );
        }
    }

    // ====================================================================
    // 精灵图动画帧绘制
    // ====================================================================

    /// DrawImageCel(image, x, y, cel)（对应 C++）
    pub fn draw_image_cel(&mut self, image: &Image, x: i32, y: i32, cel: i32) {
        let col = cel % image.num_cols;
        let row = cel / image.num_cols;
        self.draw_image_cel_rc(image, x, y, col, row);
    }

    /// DrawImageCel(image, destRect, cel)（对应 C++）
    pub fn draw_image_cel_dest(&mut self, image: &Image, dest_rect: &Rect, cel: i32) {
        let col = cel % image.num_cols;
        let row = cel / image.num_cols;
        self.draw_image_cel_dest_rc(image, dest_rect, col, row);
    }

    /// DrawImageCel(image, x, y, celCol, celRow)
    pub fn draw_image_cel_rc(&mut self, image: &Image, x: i32, y: i32, col: i32, row: i32) {
        if row < 0 || col < 0 || row >= image.num_rows || col >= image.num_cols {
            return;
        }
        let cw = image.width / image.num_cols;
        let ch = image.height / image.num_rows;
        let src_rect = Rect::new(col * cw, row * ch, cw, ch);
        self.draw_image_src(image, x, y, &src_rect);
    }

    /// DrawImageCel(image, destRect, celCol, celRow)
    pub fn draw_image_cel_dest_rc(&mut self, image: &Image, dest_rect: &Rect, col: i32, row: i32) {
        if row < 0 || col < 0 || row >= image.num_rows || col >= image.num_cols {
            return;
        }
        let cw = image.width / image.num_cols;
        let ch = image.height / image.num_rows;
        let src_rect = Rect::new(col * cw, row * ch, cw, ch);
        self.draw_image_stretch(image, dest_rect, &src_rect);
    }

    /// DrawImageAnim(image, x, y, time)（对应 C++）
    pub fn draw_image_anim(&mut self, image: &Image, x: i32, y: i32, time: i32) {
        let cel = image.get_anim_cel(time);
        self.draw_image_cel(image, x, y, cel);
    }

    // ====================================================================
    // 裁剪管理
    // ====================================================================

    /// 清除裁剪矩形（对应 C++ ClearClipRect）
    pub fn clear_clip_rect(&mut self) {
        unsafe {
            if !self.dest_image.is_null() {
                let img = &*self.dest_image;
                self.clip_rect = Rect::new(0, 0, img.width, img.height);
            }
        }
    }

    /// 设置裁剪矩形（对应 C++ SetClipRect(x, y, w, h)）
    pub fn set_clip_rect_xywh(&mut self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            if !self.dest_image.is_null() {
                let img = &*self.dest_image;
                let full_rect = Rect::new(0, 0, img.width, img.height);
                let clip = Rect::new(x + self.trans_x as i32, y + self.trans_y as i32, width, height);
                self.clip_rect = full_rect.intersection(&clip);
            }
        }
    }

    /// 设置裁剪矩形（对应 C++ SetClipRect(rect)）
    pub fn set_clip_rect(&mut self, rect: &Rect) {
        self.set_clip_rect_xywh(rect.x, rect.y, rect.width, rect.height);
    }

    /// 追加裁剪（与当前裁剪取交集，对应 C++ ClipRect(x, y, w, h)）
    pub fn clip_rect_xywh(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let clip = Rect::new(x + self.trans_x as i32, y + self.trans_y as i32, width, height);
        self.clip_rect = self.clip_rect.intersection(&clip);
    }

    /// 追加裁剪（对应 C++ ClipRect(rect)）
    pub fn clip_rect(&mut self, rect: &Rect) {
        self.clip_rect_xywh(rect.x, rect.y, rect.width, rect.height);
    }

    // ====================================================================
    // 变换
    // ====================================================================

    /// 平移变换（整数，对应 C++ Translate）
    pub fn translate(&mut self, x: i32, y: i32) {
        self.trans_x += x as f64;
        self.trans_y += y as f64;
    }

    /// 平移变换（浮点，对应 C++ TranslateF）
    pub fn translate_f(&mut self, x: f32, y: f32) {
        self.trans_x += x as f64;
        self.trans_y += y as f64;
    }

    /// 设置缩放（对应 C++ SetScale）
    pub fn set_scale(&mut self, scale_x: f32, scale_y: f32, orig_x: f32, orig_y: f32) {
        self.scale_x = scale_x as f64;
        self.scale_y = scale_y as f64;
        self.scale_orig_x = orig_x as f64 + self.trans_x;
        self.scale_orig_y = orig_y as f64 + self.trans_y;
    }

    pub fn is_3d(&self) -> bool {
        self.is_3d
    }

    // ====================================================================
    // 九宫格图像框绘制
    // ====================================================================

    /// DrawImageBox(dest, componentImage)（对应 C++）
    pub fn draw_image_box_dest(&mut self, dest: &Rect, component: &Image) {
        let src = Rect::new(0, 0, component.width, component.height);
        self.draw_image_box(&src, dest, component);
    }

    /// DrawImageBox(src, dest, componentImage)（对应 C++）
    /// 将一个 3x3 分割的图像绘制为可伸缩的框
    pub fn draw_image_box(&mut self, src: &Rect, dest: &Rect, component: &Image) {
        if src.width <= 0 || src.height <= 0 {
            return;
        }

        let cw = src.width / 3;
        let ch = src.height / 3;
        let cx = src.x;
        let cy = src.y;
        let cmw = src.width - cw * 2;
        let cmh = src.height - ch * 2;

        // 绘制 4 个角
        self.draw_image_src(component, dest.x, dest.y, &Rect::new(cx, cy, cw, ch));
        self.draw_image_src(component, dest.x + dest.width - cw, dest.y, &Rect::new(cx + cw + cmw, cy, cw, ch));
        self.draw_image_src(component, dest.x, dest.y + dest.height - ch, &Rect::new(cx, cy + ch + cmh, cw, ch));
        self.draw_image_src(component, dest.x + dest.width - cw, dest.y + dest.height - ch, &Rect::new(cx + cw + cmw, cy + ch + cmh, cw, ch));

        // 创建带裁剪的 Graphics 副本来绘制边框
        let mut vert_clip = self.create();
        vert_clip.clip_rect_xywh(dest.x + cw, dest.y, dest.width - cw * 2, dest.height);

        let num_cols = (dest.width - cw * 2 + cmw - 1) / cmw;
        for col in 0..num_cols {
            vert_clip.draw_image_src(component, dest.x + cw + col * cmw, dest.y, &Rect::new(cx + cw, cy, cmw, ch));
            vert_clip.draw_image_src(component, dest.x + cw + col * cmw, dest.y + dest.height - ch, &Rect::new(cx + cw, cy + ch + cmh, cmw, ch));
        }

        let mut horz_clip = self.create();
        horz_clip.clip_rect_xywh(dest.x, dest.y + ch, dest.width, dest.height - ch * 2);

        let num_rows = (dest.height - ch * 2 + cmh - 1) / cmh;
        for row in 0..num_rows {
            horz_clip.draw_image_src(component, dest.x, dest.y + ch + row * cmh, &Rect::new(cx, cy + ch, cw, cmh));
            horz_clip.draw_image_src(component, dest.x + dest.width - cw, dest.y + ch + row * cmh, &Rect::new(cx + cw + cmw, cy + ch, cw, cmh));
        }

        // 绘制中间区域
        let mut mid_clip = self.create();
        mid_clip.clip_rect_xywh(dest.x + cw, dest.y + ch, dest.width - cw * 2, dest.height - ch * 2);

        for col in 0..num_cols {
            for row in 0..num_rows {
                mid_clip.draw_image_src(component, dest.x + cw + col * cmw, dest.y + ch + row * cmh, &Rect::new(cx + cw, cy + ch, cmw, cmh));
            }
        }
    }

    // ====================================================================
    // 文字排版（WriteString 等）
    // ====================================================================

    /// WriteString（对应 C++ WriteString）
    pub fn write_string(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        width: i32,
        justification: i32,
        draw_string: bool,
        offset: i32,
        length: i32,
        old_color: i32,
    ) -> i32 {
        // 处理 old_color 默认值
        let mut old_color = old_color;
        if old_color == -1 {
            old_color = self.color.to_argb() as i32;
        }

        // 处理对齐方式（通过递归测量宽度）
        if draw_string {
            let measured = self.write_string(text, x, y, width, -1, false, offset, length, old_color);
            match justification {
                0 => {
                    // 居中对齐
                    let nx = x + (width - measured) / 2;
                    return self.write_string(text, nx, y, width, -1, true, offset, length, old_color);
                }
                1 => {
                    // 右对齐
                    let nx = x + width - measured;
                    return self.write_string(text, nx, y, width, -1, true, offset, length, old_color);
                }
                _ => {}
            }
        }

        let text_len = text.len() as i32;
        let mut a_len = if length < 0 || offset + length > text_len {
            text_len
        } else {
            offset + length
        };
        // length 从偏移量算起
        if length >= 0 && offset + length <= text_len {
            a_len = offset + length;
        }

        let mut a_string = String::new();
        let mut x_offset = 0;

        let chars: Vec<char> = text.chars().collect();
        let char_count = chars.len();

        let mut i = offset.max(0) as usize;
        let end = a_len.min(text_len).min(char_count as i32) as usize;

        while i < end {
            if chars[i] == '^' && self.write_colored_string {
                if i + 1 < end && chars[i + 1] == '^' {
                    // 字面 '^'
                    a_string.push('^');
                    i += 2;
                    continue;
                }
                if i + 8 > end {
                    break; // 格式错误的颜色说明
                }
                // 解析颜色标签 ^RRGGBB^
                let mut a_color: u32 = 0;
                if chars[i + 1] == 'o' {
                    // ^oldclr^
                    let tag: String = chars[i + 1..(i + 8).min(end)].iter().collect();
                    if tag.starts_with("oldclr") {
                        a_color = old_color as u32;
                    }
                } else {
                    for digit_num in 0..6 {
                        let c = chars[i + 1 + digit_num];
                        let val = match c {
                            '0'..='9' => c as u32 - '0' as u32,
                            'A'..='F' => c as u32 - 'A' as u32 + 10,
                            'a'..='f' => c as u32 - 'a' as u32 + 10,
                            _ => 0,
                        };
                        a_color += val << ((5 - digit_num) * 4);
                    }
                }

                if draw_string {
                    // 绘制当前累积的字符串
                    self.draw_string(&a_string, x + x_offset, y);
                    // 设置新颜色
                    self.color = Color::new(
                        ((a_color >> 16) & 0xFF) as u8,
                        ((a_color >> 8) & 0xFF) as u8,
                        (a_color & 0xFF) as u8,
                        self.color.a,
                    );
                }

                i += 8; // 跳过颜色标签
                x_offset += self.string_width(&a_string);
                a_string = String::new();
            } else {
                a_string.push(chars[i]);
                i += 1;
            }
        }

        if draw_string {
            self.draw_string(&a_string, x + x_offset, y);
        }
        x_offset += self.string_width(&a_string);
        x_offset
    }

    /// DrawStringWordWrapped（对应 C++ DrawStringWordWrapped）
    pub fn draw_string_word_wrapped(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        wrap_width: i32,
        line_spacing: i32,
        justification: i32,
        max_width: Option<&mut i32>,
    ) -> i32 {
        unsafe {
            if self.font.is_null() {
                return 0;
            }
            let font = &*self.font;
            let ascent = font.ascent;
            let ascent_padding = 0; // Font 结构体暂未包含 ascent_padding
            let y_offset = ascent - ascent_padding;
            let rect = Rect::new(x, y - y_offset, wrap_width, 0);
            self.write_word_wrapped(&rect, text, line_spacing, justification, max_width, -1, None)
        }
    }

    /// WriteWordWrapped（对应 C++ WriteWordWrapped）
    pub fn write_word_wrapped(
        &mut self,
        rect: &Rect,
        line: &str,
        line_spacing: i32,
        justification: i32,
        mut max_width: Option<&mut i32>,
        max_chars: i32,
        mut last_width: Option<&mut i32>,
    ) -> i32 {
        let orig_color = self.color;
        let orig_color_int = orig_color.to_argb() as i32;
        let orig_color_int = if (orig_color_int as u32 & 0xFF000000) == 0xFF000000 {
            (orig_color_int as u32 & !0xFF000000) as i32
        } else {
            orig_color_int
        };

        let chars: Vec<char> = line.chars().collect();
        let num_chars = chars.len() as i32;
        let mc = if max_chars < 0 { num_chars } else { max_chars };

        unsafe {
            if self.font.is_null() {
                return 0;
            }
            let font = &*self.font;
            let ascent = font.ascent;
            let ascent_padding = 0;
            let mut y_offset = ascent - ascent_padding;
            let spacing = if line_spacing == -1 { font.line_spacing } else { line_spacing };

            let mut cur_pos: usize = 0;
            let mut line_start_pos: usize = 0;
            let mut cur_width: i32 = 0;
            let mut cur_char: char = '\0';
            let mut prev_char: char = '\0';
            let mut a_max_width: i32 = 0;
            let mut indent_x: i32 = 0;
            let mut break_draw_len: i32 = -1;
            let mut break_resume_pos: usize = 0;
            let mut break_skip_spaces: bool = false;

            if let Some(ref lw) = last_width {
                indent_x = **lw;
                cur_width = indent_x;
            }

            while cur_pos < chars.len() {
                let char_start = cur_pos;

                // 检查颜色标签
                if chars[cur_pos] == '^' && self.write_colored_string {
                    if cur_pos + 1 < chars.len() {
                        if chars[cur_pos + 1] == '^' {
                            cur_pos += 1; // 字面 '^'
                        } else {
                            cur_pos += 8;
                            continue; // 跳过颜色说明
                        }
                    }
                }

                if cur_pos >= chars.len() {
                    break;
                }
                cur_char = chars[cur_pos];
                cur_pos += 1;

                if cur_char == '\r' {
                    continue;
                }

                let char_end = cur_pos;
                let is_newline = cur_char == '\n';
                let is_space = !is_newline && cur_char == ' ';

                if is_space {
                    break_draw_len = (char_start - line_start_pos) as i32;
                    break_resume_pos = char_end;
                    break_skip_spaces = true;
                } else if is_newline {
                    cur_width = rect.width + 1;
                    break_draw_len = (char_start - line_start_pos) as i32;
                    break_resume_pos = char_end;
                    break_skip_spaces = false;
                }

                cur_width += font.char_width_kern(cur_char, prev_char);

                // 自动换行字符检查
                if !is_space && !is_newline && Self::is_auto_break_char(cur_char)
                    && !Self::is_closing_punctuation(cur_char)
                    && char_start > line_start_pos
                    && !Self::is_opening_punctuation(prev_char)
                {
                    break_draw_len = (char_start - line_start_pos) as i32;
                    break_resume_pos = char_start;
                    break_skip_spaces = false;
                }
                prev_char = cur_char;

                if cur_width > rect.width {
                    let written_width: i32;
                    if break_draw_len >= 0 {
                        let phys_y = rect.y + y_offset + self.trans_y as i32;
                        if phys_y >= self.clip_rect.y
                            && phys_y < self.clip_rect.y + self.clip_rect.height + spacing
                        {
                            self.write_string(
                                line,
                                rect.x + indent_x,
                                rect.y + y_offset,
                                rect.width,
                                justification,
                                true,
                                line_start_pos as i32,
                                break_draw_len,
                                orig_color_int,
                            );
                        }
                        written_width = cur_width + indent_x;
                        if written_width < 0 {
                            break;
                        }
                        cur_pos = break_resume_pos;
                        if break_skip_spaces {
                            while cur_pos < chars.len() && chars[cur_pos] == ' ' {
                                cur_pos += 1;
                            }
                        }
                        line_start_pos = cur_pos;
                    } else {
                        let mut draw_end = char_start;
                        if draw_end <= line_start_pos {
                            draw_end = char_end;
                        }
                        written_width = self.write_string(
                            line,
                            rect.x + indent_x,
                            rect.y + y_offset,
                            rect.width,
                            justification,
                            true,
                            line_start_pos as i32,
                            (draw_end - line_start_pos) as i32,
                            orig_color_int,
                        );
                        if written_width < 0 {
                            break;
                        }
                        if let Some(ref mut mw) = max_width {
                            if written_width > **mw {
                                **mw = written_width;
                            }
                        }
                        if let Some(ref mut lw) = last_width {
                            **lw = written_width;
                        }
                        cur_pos = draw_end;
                    }

                    if written_width > a_max_width {
                        a_max_width = written_width;
                    }
                    line_start_pos = cur_pos;
                    break_draw_len = -1;
                    cur_width = 0;
                    prev_char = '\0';
                    indent_x = 0;
                    y_offset += spacing;
                }
            }

            if line_start_pos < chars.len() {
                let written_width = self.write_string(
                    line,
                    rect.x + indent_x,
                    rect.y + y_offset,
                    rect.width,
                    justification,
                    true,
                    line_start_pos as i32,
                    (chars.len() - line_start_pos) as i32,
                    orig_color_int,
                );
                if written_width >= 0 {
                    if written_width > a_max_width {
                        a_max_width = written_width;
                    }
                    if let Some(ref mut mw) = max_width {
                        if written_width > **mw {
                            **mw = written_width;
                        }
                    }
                    if let Some(ref mut lw) = last_width {
                        **lw = written_width;
                    }
                    y_offset += spacing;
                }
            } else if cur_char == '\n' {
                y_offset += spacing;
                if let Some(ref mut lw) = last_width {
                    **lw = 0;
                }
            }

            self.color = orig_color;
            if let Some(mw) = max_width {
                *mw = a_max_width;
            }
            y_offset + 0 - spacing
        }
    }

    /// DrawStringColor（对应 C++ DrawStringColor）
    pub fn draw_string_color(&mut self, text: &str, x: i32, y: i32, old_color: i32) -> i32 {
        self.write_string(text, x, y, -1, -1, true, 0, -1, old_color)
    }

    /// GetWordWrappedHeight（对应 C++ GetWordWrappedHeight）
    pub fn get_word_wrapped_height(
        &self,
        width: i32,
        text: &str,
        line_spacing: i32,
        max_width: Option<&mut i32>,
    ) -> i32 {
        let mut test_g = self.create();
        test_g.font = self.font;
        test_g.write_word_wrapped(
            &Rect::new(0, 0, width, 0),
            text,
            line_spacing,
            -1,
            max_width,
            -1,
            None,
        )
    }

    // ====================================================================
    // 多边形填充
    // ====================================================================

    /// PFCompareInd — 比较顶点的 Y 坐标（用于排序）
    fn pf_compare_ind(points: &[Point], u: i32, v: i32) -> cmp::Ordering {
        let a = &points[u as usize];
        let b = &points[v as usize];
        a.y.cmp(&b.y)
    }

    /// PFCompareActive — 比较边缘的 X 坐标
    fn pf_compare_active(a: &Edge, b: &Edge) -> cmp::Ordering {
        a.x.partial_cmp(&b.x).unwrap_or(cmp::Ordering::Equal)
    }

    /// PFDelete — 从活动列表中移除边缘
    fn pf_delete(&mut self, i: i32) {
        let mut j = 0;
        while j < self.num_active_edges && self.active_edge_list[j as usize].i != i {
            j += 1;
        }
        if j >= self.num_active_edges {
            return;
        }
        self.num_active_edges -= 1;
        self.active_edge_list.remove(j as usize);
    }

    /// PFInsert — 向活动列表追加边缘
    fn pf_insert(&mut self, i: i32, y: i32, points: &[Point]) {
        let j = if i < self.num_vertices - 1 { i + 1 } else { 0 };
        let (p, q) = if points[i as usize].y < points[j as usize].y {
            (&points[i as usize], &points[j as usize])
        } else {
            (&points[j as usize], &points[i as usize])
        };

        let dx = (q.x - p.x) as f64 / (q.y - p.y) as f64;
        let edge = Edge {
            dx,
            x: dx * (y as f64 + 0.5 - p.y as f64 - self.trans_y) + p.x as f64 + self.trans_x,
            i,
            b: p.y as f64 - 1.0 / dx * p.x as f64,
        };
        self.active_edge_list.push(edge);
        self.num_active_edges += 1;
    }

    /// PolyFill（对应 C++ PolyFill）
    pub fn poly_fill(&mut self, vertices: &[Point], convex: bool) {
        unsafe {
            if !self.dest_image.is_null() {
                let dest = &*self.dest_image;
                if convex && dest.poly_fill_3d(vertices, &self.clip_rect, &self.color, self.draw_mode, self.trans_x as i32, self.trans_y as i32) {
                    return;
                }
            }
        }

        let num_vertices = vertices.len() as i32;
        if num_vertices <= 0 {
            return;
        }

        let mut spans: Vec<Span> = Vec::with_capacity(MAX_TEMP_SPANS);
        let mut span_pos = 0;

        let min_x = self.clip_rect.x;
        let max_x = self.clip_rect.x + self.clip_rect.width - 1;
        let min_y = self.clip_rect.y;
        let max_y = self.clip_rect.y + self.clip_rect.height - 1;

        self.num_vertices = num_vertices;

        // 创建索引并按 Y 排序
        let mut ind: Vec<i32> = (0..num_vertices).collect();
        ind.sort_by(|a, b| Self::pf_compare_ind(vertices, *a, *b));

        self.active_edge_list.clear();
        self.num_active_edges = 0;

        let y0 = cmp::max(min_y, (vertices[ind[0] as usize].y as f64 - 0.5 + self.trans_y).ceil() as i32);
        let y1 = cmp::min(max_y, (vertices[ind[(num_vertices - 1) as usize] as usize].y as f64 - 0.5 + self.trans_y).floor() as i32);

        let mut k: usize = 0;
        for y in y0..=y1 {
            // 处理从前一条扫描线到当前扫描线之间的顶点
            while k < ind.len() && vertices[ind[k] as usize].y as f64 + self.trans_y <= y as f64 + 0.5 {
                let i = ind[k];

                // 处理前一个和后一个边缘
                let j = if i > 0 { i - 1 } else { num_vertices - 1 };
                if vertices[j as usize].y as f64 + self.trans_y <= y as f64 - 0.5 {
                    self.pf_delete(j);
                } else if vertices[j as usize].y as f64 + self.trans_y > y as f64 + 0.5 {
                    self.pf_insert(j, y, vertices);
                }

                let j = if i < num_vertices - 1 { i + 1 } else { 0 };
                if vertices[j as usize].y as f64 + self.trans_y <= y as f64 - 0.5 {
                    self.pf_delete(i);
                } else if vertices[j as usize].y as f64 + self.trans_y > y as f64 + 0.5 {
                    self.pf_insert(i, y, vertices);
                }

                k += 1;
            }

            // 按 X 排序活动边缘列表
            self.active_edge_list.sort_by(|a, b| Self::pf_compare_active(a, b));

            // 绘制水平线段
            let mut j: usize = 0;
            while j + 1 < self.active_edge_list.len() {
                let xl = (self.active_edge_list[j].x - 0.5).ceil() as i32;
                let xl = if xl < min_x { min_x } else { xl };
                let xr = (self.active_edge_list[j + 1].x - 0.5).floor() as i32;
                let xr = if xr > max_x { max_x } else { xr };

                if xl <= xr && span_pos < MAX_TEMP_SPANS {
                    spans.push(Span { y, x: xl, width: xr - xl + 1 });
                    span_pos += 1;
                }

                self.active_edge_list[j].x += self.active_edge_list[j].dx;
                self.active_edge_list[j + 1].x += self.active_edge_list[j + 1].dx;
                j += 2;
            }
        }

        unsafe {
            if !self.dest_image.is_null() {
                let dest = &mut *self.dest_image;
                dest.fill_scan_lines(&spans, &self.color, self.draw_mode);
            }
        }
    }

    /// PolyFillAA（对应 C++ PolyFillAA — 抗锯齿多边形填充）
    pub fn poly_fill_aa(&mut self, vertices: &[Point], convex: bool) {
        unsafe {
            if !self.dest_image.is_null() {
                let dest = &*self.dest_image;
                if convex && dest.poly_fill_3d(vertices, &self.clip_rect, &self.color, self.draw_mode, self.trans_x as i32, self.trans_y as i32) {
                    return;
                }
            }
        }

        let num_vertices = vertices.len() as i32;
        if num_vertices <= 0 {
            return;
        }

        // 计算包围盒
        let mut cover_left = vertices[0].x;
        let mut cover_right = vertices[0].x;
        let mut cover_top = vertices[0].y;
        let mut cover_bottom = vertices[0].y;
        for v in vertices {
            cover_left = cover_left.min(v.x);
            cover_right = cover_right.max(v.x);
            cover_top = cover_top.min(v.y);
            cover_bottom = cover_bottom.max(v.y);
        }

        let cover_width = (cover_right - cover_left + 1).max(1);
        let cover_height = (cover_bottom - cover_top + 1).max(1);
        let mut coverage = vec![0u8; (cover_width * cover_height) as usize];

        let min_x = self.clip_rect.x;
        let max_x = self.clip_rect.x + self.clip_rect.width - 1;
        let min_y = self.clip_rect.y;
        let max_y = self.clip_rect.y + self.clip_rect.height - 1;

        self.num_vertices = num_vertices;

        let mut ind: Vec<i32> = (0..num_vertices).collect();
        ind.sort_by(|a, b| Self::pf_compare_ind(vertices, *a, *b));

        self.active_edge_list.clear();
        self.num_active_edges = 0;

        let y0 = cmp::max(min_y, (vertices[ind[0] as usize].y as f64 - 0.5 + self.trans_y).ceil() as i32);
        let y1 = cmp::min(max_y, (vertices[ind[(num_vertices - 1) as usize] as usize].y as f64 - 0.5 + self.trans_y).floor() as i32);

        let mut spans: Vec<Span> = Vec::with_capacity(MAX_TEMP_SPANS);
        let mut span_pos = 0;

        let mut k: usize = 0;
        for y in y0..=y1 {
            while k < ind.len() && vertices[ind[k] as usize].y as f64 + self.trans_y <= y as f64 + 0.5 {
                let i = ind[k];

                let j = if i > 0 { i - 1 } else { num_vertices - 1 };
                if vertices[j as usize].y as f64 + self.trans_y <= y as f64 - 0.5 {
                    self.pf_delete(j);
                } else if vertices[j as usize].y as f64 + self.trans_y > y as f64 + 0.5 {
                    self.pf_insert(j, y, vertices);
                }

                let j = if i < num_vertices - 1 { i + 1 } else { 0 };
                if vertices[j as usize].y as f64 + self.trans_y <= y as f64 - 0.5 {
                    self.pf_delete(i);
                } else if vertices[j as usize].y as f64 + self.trans_y > y as f64 + 0.5 {
                    self.pf_insert(i, y, vertices);
                }
                k += 1;
            }

            self.active_edge_list.sort_by(|a, b| Self::pf_compare_active(a, b));

            let mut j: usize = 0;
            while j + 1 < self.active_edge_list.len() {
                let mut xl = (self.active_edge_list[j].x - 0.5).ceil() as i32;
                let mut l_err = ((self.active_edge_list[j].x - 0.5 - xl as f64).abs() * 255.0) as i32;
                if xl < min_x {
                    xl = min_x;
                    l_err = 255;
                }
                let mut xr = (self.active_edge_list[j + 1].x - 0.5).floor() as i32;
                let mut r_err = ((self.active_edge_list[j + 1].x - 0.5 - xr as f64).abs() * 255.0) as i32;
                if xr > max_x {
                    xr = max_x;
                    r_err = 255;
                }

                if xl <= xr && span_pos < MAX_TEMP_SPANS {
                    spans.push(Span { y, x: xl, width: xr - xl + 1 });
                    span_pos += 1;

                    let cover_row_offset = ((y - cover_top) * cover_width) as usize;
                    if xr == xl {
                        let idx = (xl - cover_left) as usize;
                        if cover_row_offset + idx < coverage.len() {
                            coverage[cover_row_offset + idx] = coverage[cover_row_offset + idx]
                                .min(255 - ((255 - coverage[cover_row_offset + idx] as i32).min((l_err * r_err) >> 8) as u8));
                        }
                    } else {
                        // 左边缘
                        let idx_l = (xl - cover_left) as usize;
                        if cover_row_offset + idx_l < coverage.len() {
                            coverage[cover_row_offset + idx_l] =
                                coverage[cover_row_offset + idx_l].saturating_add(l_err as u8).min(255);
                        }
                        // 右边缘
                        let idx_r = (xr - cover_left) as usize;
                        if cover_row_offset + idx_r < coverage.len() {
                            coverage[cover_row_offset + idx_r] =
                                coverage[cover_row_offset + idx_r].saturating_add(r_err as u8).min(255);
                        }
                        // 中间区域完全覆盖
                        if xl + 1 <= xr - 1 {
                            for cx in (xl + 1)..=(xr - 1) {
                                let idx_m = (cx - cover_left) as usize;
                                if cover_row_offset + idx_m < coverage.len() {
                                    coverage[cover_row_offset + idx_m] = 255;
                                }
                            }
                        }
                    }
                }

                self.active_edge_list[j].x += self.active_edge_list[j].dx;
                self.active_edge_list[j + 1].x += self.active_edge_list[j + 1].dx;
                j += 2;
            }
        }

        unsafe {
            if !self.dest_image.is_null() {
                let dest = &mut *self.dest_image;
                dest.fill_scan_lines_with_coverage(
                    &spans, &self.color, self.draw_mode,
                    &coverage, cover_left, cover_top, cover_width, cover_height,
                );
            }
        }
    }

    // ====================================================================
    // 字符分类辅助（对应 C++ Sexy::IsAutoBreakChar 等）
    // ====================================================================

    fn is_auto_break_char(c: char) -> bool {
        match c {
            '-' | '?' | '!' | ':' | ';' | ')' | ']' | '}' | '>'
            | '\u{3001}' | '\u{3002}' | '\u{FF0C}' | '\u{FF0E}' | '\u{FF1F}' | '\u{FF01}' => true,
            _ => false,
        }
    }

    fn is_closing_punctuation(c: char) -> bool {
        match c {
            ')' | ']' | '}' | '>' | '\u{3001}' | '\u{3002}' | '\u{FF0C}' | '\u{FF0E}' | '\u{FF1F}' | '\u{FF01}' => true,
            _ => false,
        }
    }

    fn is_opening_punctuation(c: char) -> bool {
        match c {
            '(' | '[' | '{' | '<' | '\u{300C}' | '\u{300E}' | '\u{2018}' | '\u{201C}' => true,
            _ => false,
        }
    }
}

// ====================================================================
// GraphicsAutoState 辅助（对应 C++ GraphicsAutoState）
// 在作用域结束时自动恢复图形状态
// ====================================================================

pub struct GraphicsAutoState<'a> {
    g: &'a mut Graphics,
}

impl<'a> GraphicsAutoState<'a> {
    pub fn new(g: &'a mut Graphics) -> Self {
        g.push_state();
        GraphicsAutoState { g }
    }
}

impl<'a> Drop for GraphicsAutoState<'a> {
    fn drop(&mut self) {
        self.g.pop_state();
    }
}

/// 默认构造（无目标图像）
impl Default for Graphics {
    fn default() -> Self {
        Graphics::new_with_image(std::ptr::null_mut())
    }
}
