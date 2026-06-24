// PvZ Portable Rust 翻译 — GLImage（OpenGL 纹理图像）
// 对应 C++ SexyAppFramework/graphics/GLImage.h / GLImage.cpp
//
// GLImage 继承自 MemoryImage，将所有绘制操作委托给 GLInterface。
// 像素数据不存储在系统内存中（mBits = nullptr），而是直接作为 OpenGL 纹理。
//
// 注意：在 C++ 中 GLImage : MemoryImage : Image，但在 Rust 中由于没有继承，
// 我们让 GLImage 包含一个 MemoryImage 作为 base，并将 image_kind 设为 GLImage，
// 使得 Graphics 和 GLInterface 可以识别。

#![allow(dead_code)]
#![allow(invalid_reference_casting)]

use crate::framework::color::Color;
use crate::framework::graphics::image::{Image, ImageKind, Span};
use crate::framework::graphics::memory_image::MemoryImage;
use crate::framework::graphics::gl_interface::{GLInterface, TextureDataPiece, PixelFormat, TriVertex};
use crate::framework::rect::Rect;
use crate::framework::point::Point;
use crate::framework::sexy_matrix::SexyMatrix3;
use crate::ffi::opengl::GLuint;

/// OpenGL 纹理图像（对应 C++ GLImage）
///
/// 在 C++ 中 GLImage : MemoryImage，像素数据通过 OpenGL 纹理渲染。
/// Rust 版本将 GLImage 作为独立结构体，通过 image_kind = GLImage 标识类型，
/// 并通过 app/gl_interface 指针委托 OpenGL 绘制调用。
#[derive(Debug)]
pub struct GLImage {
    /// 基类（MemoryImage）
    pub base: MemoryImage,
    /// GLInterface 指针（对应 C++ mGLInterface）
    pub gl_interface: *mut GLInterface,
}

impl GLImage {
    /// 创建 GLImage（对应 C++ GLImage(GLInterface*)）
    pub fn new(width: i32, height: i32) -> Self {
        let mut mem = MemoryImage::new(width, height);
        mem.base.image_kind = ImageKind::GLImage;
        // GLImage 不保留系统内存中的像素数据，由 GPU 纹理管理
        mem.base.pixels.clear();
        mem.purge_bits = true;

        GLImage {
            base: mem,
            gl_interface: std::ptr::null_mut(),
        }
    }

    /// 带 GLInterface 指针的构造（对应 C++ GLImage(theGLInterface)）
    pub fn new_with_gl(width: i32, height: i32, gl: *mut GLInterface) -> Self {
        let mut img = Self::new(width, height);
        img.gl_interface = gl;
        img
    }

    /// 初始化 GLImage（构造完成后调用，注册到 GLInterface）
    pub fn init(&mut self) {
        if !self.gl_interface.is_null() {
            unsafe {
                (*self.gl_interface).add_gl_image(self as *mut GLImage);
            }
        }
    }

    /// 从 GLInterface 注销
    pub fn destroy(&mut self) {
        if !self.gl_interface.is_null() {
            unsafe {
                (*self.gl_interface).remove_gl_image(self as *mut GLImage);
            }
        }
    }

    /// 获取第一个纹理 ID
    pub fn get_tex_id(&self) -> GLuint {
        if let Some(ref data) = self.base.render_data {
            if !data.textures.is_empty() {
                data.textures[0].texture
            } else {
                0
            }
        } else {
            0
        }
    }

    /// 检查是否为 3D 图像（对应 C++ Check3D 静态方法）
    /// 只要是 GLImage 就返回 true
    pub fn check_3d(image: &Image) -> bool {
        image.image_kind == ImageKind::GLImage
    }

    // ====================================================================
    // 覆盖方法（对应 C++ GLImage 虚方法覆盖）
    // ====================================================================

    /// 创建（对应 C++ Create）— GLImage 的 Create 不分配像素内存
    pub fn create(&mut self, width: i32, height: i32) {
        self.base.base.width = width;
        self.base.base.height = height;
        self.base.base.pixels.clear(); // GLImage 不保存像素
        self.base.bits_changed();
    }

    /// 带覆盖率的扫描线填充（对应 C++ FillScanLinesWithCoverage）
    /// 创建一个临时 MemoryImage 在上面绘制，然后 Blt 到自身
    pub fn fill_scan_lines_with_coverage(
        &mut self,
        spans: &[Span],
        span_count: i32,
        color: &Color,
        draw_mode: i32,
        coverage: &[u8],
        cover_x: i32,
        cover_y: i32,
        cover_width: i32,
        cover_height: i32,
    ) {
        if span_count <= 0 {
            return;
        }

        // 计算包含所有 span 的最小矩形
        let mut l = spans[0].x;
        let mut t = spans[0].y;
        let mut r = spans[0].x + spans[0].width - 1;
        let mut b = spans[0].y;

        for i in 1..span_count as usize {
            let span = &spans[i];
            l = l.min(span.x);
            r = r.max(span.x + span.width - 1);
            t = t.min(span.y);
            b = b.max(span.y);
        }

        // 将 span 坐标转为相对于临时图像的坐标
        let mut local_spans: Vec<Span> = Vec::with_capacity(span_count as usize);
        for i in 0..span_count as usize {
            local_spans.push(Span {
                x: spans[i].x - l,
                y: spans[i].y - t,
                width: spans[i].width,
            });
        }

        // 创建临时 MemoryImage
        let mut temp = MemoryImage::new(r - l + 1, b - t + 1);
        temp.base.fill_scan_lines_with_coverage(
            &local_spans,
            color,
            draw_mode,
            coverage,
            cover_x - l,
            cover_y - t,
            cover_width,
            cover_height,
        );

        // 将临时图像 Blt 到自身
        let src_rect = Rect::new(0, 0, r - l + 1, b - t + 1);
        self.blt(&temp.base, l, t, &src_rect, &Color::WHITE, draw_mode, false);
    }

    /// 3D 多边形填充（对应 C++ PolyFill3D）
    pub fn poly_fill_3d(
        &mut self,
        vertices: &[Point],
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        tx: i32,
        ty: i32,
    ) -> bool {
        if !self.gl_interface.is_null() {
            // 将 Point slice 转为 (i32,i32) slice
            let v: Vec<(i32, i32)> = vertices.iter().map(|p| (p.x, p.y)).collect();
            unsafe {
                (*self.gl_interface).fill_poly(&v, Some(clip_rect), color, draw_mode, tx, ty);
            }
            true
        } else {
            false
        }
    }

    /// 填充矩形（对应 C++ FillRect）
    pub fn fill_rect(&mut self, rect: &Rect, color: &Color, draw_mode: i32) {
        if !self.gl_interface.is_null() {
            unsafe {
                (*self.gl_interface).fill_rect(rect, color, draw_mode);
            }
        }
    }

    /// 绘制线段（对应 C++ DrawLine）
    pub fn draw_line(&mut self, start_x: f64, start_y: f64, end_x: f64, end_y: f64, color: &Color, draw_mode: i32) {
        if !self.gl_interface.is_null() {
            unsafe {
                (*self.gl_interface).draw_line(start_x, start_y, end_x, end_y, color, draw_mode);
            }
        }
    }

    /// 绘制抗锯齿线段（对应 C++ DrawLineAA）— GLImage 中与 DrawLine 相同
    pub fn draw_line_aa(&mut self, start_x: f64, start_y: f64, end_x: f64, end_y: f64, color: &Color, draw_mode: i32) {
        self.draw_line(start_x, start_y, end_x, end_y, color, draw_mode);
    }

    /// 块传输（对应 C++ Blt）
    pub fn blt(&mut self, src: &Image, dest_x: i32, dest_y: i32, src_rect: &Rect, color: &Color, draw_mode: i32, linear_filter: bool) {
        // 确保自身纹理已提交
        self.commit_bits_internal();
        if !self.gl_interface.is_null() {
            unsafe {
                (*self.gl_interface).blt(
                    &mut *(src as *const Image as *mut Image),
                    dest_x as f32, dest_y as f32,
                    src_rect, color, draw_mode, linear_filter,
                );
            }
        }
    }

    /// 浮点坐标 Blt（对应 C++ BltF）
    pub fn blt_f(&mut self, src: &Image, dest_x: f32, dest_y: f32, src_rect: &Rect, clip_rect: &Rect, color: &Color, draw_mode: i32) {
        if !self.gl_interface.is_null() {
            unsafe {
                let gl = &mut *self.gl_interface;
                if !gl.transform_stack.is_empty()
                    || clip_rect.width != src_rect.width
                    || clip_rect.height != src_rect.height
                {
                    gl.blt_clip_f(
                        &mut *(src as *const Image as *mut Image),
                        dest_x, dest_y, src_rect,
                        Some(clip_rect), color, draw_mode, true,
                    );
                } else {
                    gl.blt(
                        &mut *(src as *const Image as *mut Image),
                        dest_x, dest_y, src_rect, color, draw_mode, true,
                    );
                }
            }
        }
    }

    /// 旋转 Blt（对应 C++ BltRotated）
    pub fn blt_rotated(
        &mut self,
        src: &Image,
        dest_x: f32,
        dest_y: f32,
        src_rect: &Rect,
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        rot: f64,
        rot_center_x: f32,
        rot_center_y: f32,
    ) {
        self.commit_bits_internal();
        if !self.gl_interface.is_null() {
            unsafe {
                (*self.gl_interface).blt_rotated(
                    &mut *(src as *const Image as *mut Image),
                    dest_x, dest_y, Some(clip_rect), color, draw_mode,
                    rot, rot_center_x, rot_center_y, src_rect,
                );
            }
        }
    }

    /// 拉伸 Blt（对应 C++ StretchBlt）
    pub fn stretch_blt(&mut self, src: &Image, dest_rect: &Rect, src_rect: &Rect, clip_rect: &Rect, color: &Color, draw_mode: i32, fast_stretch: bool) {
        self.commit_bits_internal();
        if !self.gl_interface.is_null() {
            unsafe {
                (*self.gl_interface).stretch_blt(
                    &mut *(src as *const Image as *mut Image),
                    dest_rect, src_rect, Some(clip_rect), color, draw_mode, fast_stretch, false,
                );
            }
        }
    }

    /// 矩阵变换 Blt（对应 C++ BltMatrix）
    pub fn blt_matrix(
        &mut self,
        src: &Image,
        x: f32,
        y: f32,
        matrix: &SexyMatrix3,
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        src_rect: &Rect,
        blend: bool,
    ) {
        if !self.gl_interface.is_null() {
            unsafe {
                (*self.gl_interface).blt_transformed(
                    &mut *(src as *const Image as *mut Image),
                    Some(clip_rect), color, draw_mode, src_rect, matrix, blend, x, y, true,
                );
            }
        }
    }

    /// 三角纹理绘制（对应 C++ BltTrianglesTex）
    pub fn blt_triangles_tex(
        &mut self,
        texture: &Image,
        vertices: &[[TriVertex; 3]],
        num_triangles: i32,
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        tx: f32,
        ty: f32,
        blend: bool,
    ) {
        if !self.gl_interface.is_null() {
            unsafe {
                (*self.gl_interface).draw_triangles_tex(
                    vertices, num_triangles, color, draw_mode,
                    &mut *(texture as *const Image as *mut Image), tx, ty, blend,
                );
            }
        }
    }

    /// 镜像 Blt（对应 C++ BltMirror）
    pub fn blt_mirror(&mut self, src: &Image, dest_x: i32, dest_y: i32, src_rect: &Rect, color: &Color, draw_mode: i32, linear_filter: bool) {
        self.commit_bits_internal();
        if !self.gl_interface.is_null() {
            unsafe {
                (*self.gl_interface).blt_mirror(
                    &mut *(src as *const Image as *mut Image),
                    dest_x as f32, dest_y as f32, src_rect, color, draw_mode, linear_filter,
                );
            }
        }
    }

    /// 拉伸镜像 Blt（对应 C++ StretchBltMirror）
    pub fn stretch_blt_mirror(&mut self, src: &Image, dest_rect: &Rect, src_rect: &Rect, clip_rect: &Rect, color: &Color, draw_mode: i32, fast_stretch: bool) {
        self.commit_bits_internal();
        if !self.gl_interface.is_null() {
            unsafe {
                (*self.gl_interface).stretch_blt(
                    &mut *(src as *const Image as *mut Image),
                    dest_rect, src_rect, Some(clip_rect), color, draw_mode, fast_stretch, true,
                );
            }
        }
    }

    /// 清除位图（对应 C++ PurgeBits）
    pub fn purge_bits(&mut self) {
        self.base.purge_bits = true;
        self.base.commit_bits();
        self.base.purge_bits_impl();
        // 基类 MemoryImage::PurgeBits 会清除像素缓冲区
    }

    /// 设置图像模式（对应 C++ SetImageMode）
    pub fn set_image_mode(&mut self, has_trans: bool, has_alpha: bool) {
        self.base.set_image_mode(has_trans, has_alpha);
    }

    // ====================================================================
    // 内部辅助
    // ====================================================================

    /// 提交位图变更到 GPU（在调用 GLInterface 绘制前调用）
    fn commit_bits_internal(&mut self) {
        if self.base.bits_changed {
            self.base.commit_bits();
        }
        // 若有 render_data 变更，更新计数
        if let Some(ref mut data) = self.base.render_data {
            if data.bits_changed_count != self.base.bits_changed_count {
                data.bits_changed_count = self.base.bits_changed_count;
            }
        }
    }
}

impl Drop for GLImage {
    fn drop(&mut self) {
        self.destroy();
    }
}
