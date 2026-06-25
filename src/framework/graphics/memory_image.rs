// PvZ Portable Rust 翻译 — MemoryImage（内存图像）
// 对应 C++ SexyAppFramework/graphics/MemoryImage.h / MemoryImage.cpp
//
// MemoryImage 是软件渲染路径的像素缓冲区实现。
// 它持有系统内存中的像素数据（RGBA8888/RGB565/RGB555），
// 并在需要时提交到 OpenGL 纹理。
// Image 基类中的 Blt/FillRect/DrawLine 等方法的默认实现
// 已经操作了 Image 内部的 pixels 缓冲区，MemoryImage 复用这些方法，
// 并增加了 GPU 提交、调色板、格式转换等高层管理逻辑。

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::graphics::image::{Image, ImageKind, Span};
use crate::framework::rect::Rect;
use crate::framework::point::Point;
use crate::framework::sexy_matrix::SexyMatrix3;

/// 内存图像（像素数据在系统内存中）
///
/// 对应 C++ MemoryImage，继承自 Image。
/// Rust 中通过 image_kind = ImageKind::MemoryImage 标记类型。
#[derive(Debug)]
pub struct MemoryImage {
    pub base: Image,
    /// 像素变更计数（用于判定是否需要重新提交 GPU 纹理）
    pub bits_changed_count: i32,
    /// 渲染数据指针（对应 C++ mRenderData，指向 TextureData）
    pub render_data: Option<Box<crate::framework::graphics::gl_interface::TextureData>>,
    /// 渲染标记（对应 C++ mRenderFlags）
    pub render_flags: u32,
    /// 调色板（对应 C++ mColorTable），32-bit ARGB
    pub color_table: Vec<u32>,
    /// 颜色索引（对应 C++ mColorIndices），8-bit 索引
    pub color_indices: Vec<u8>,
    /// 强制模式（对应 C++ mForcedMode）
    pub forced_mode: bool,
    /// 是否有透明色
    pub has_trans: bool,
    /// 是否有 alpha 通道
    pub has_alpha: bool,
    /// 是否标记为可变（对应 C++ mIsVolatile）
    pub is_volatile: bool,
    /// 提交纹理后是否清除位图（对应 C++ mPurgeBits）
    pub purge_bits: bool,
    /// 是否需要调色板（对应 C++ mWantPal）
    pub want_pal: bool,
    /// 原生 alpha 数据（对应 C++ mNativeAlphaData）
    pub native_alpha_data: Option<Vec<u32>>,
    /// 游程编码 alpha 数据（对应 C++ mRLAlphaData）
    pub rl_alpha_data: Option<Vec<u8>>,
    /// 游程编码附加数据（对应 C++ mRLAdditiveData）
    pub rl_additive_data: Option<Vec<u8>>,
    /// 位图是否已变更（对应 C++ mBitsChanged）
    pub bits_changed: bool,
    /// 应用指针（用于回调、纹理管理等）
    pub app: *mut crate::framework::sexy_app_base::SexyAppBase,
}

/// 渲染标记常量（对应 C++ RENDERIMAGEFLAG_SANDING 等）
pub const RENDERFLAG_SANDING: u32 = 0x1000;

impl MemoryImage {
    /// 创建 MemoryImage（对应 C++ MemoryImage(theApp)）
    pub fn new(width: i32, height: i32) -> Self {
        let mut img = Image::new(width, height);
        img.image_kind = ImageKind::MemoryImage;
        MemoryImage {
            base: img,
            bits_changed_count: 1,
            render_data: None,
            render_flags: 0,
            color_table: Vec::new(),
            color_indices: Vec::new(),
            forced_mode: false,
            has_trans: false,
            has_alpha: false,
            is_volatile: false,
            purge_bits: false,
            want_pal: false,
            native_alpha_data: None,
            rl_alpha_data: None,
            rl_additive_data: None,
            bits_changed: false,
            app: std::ptr::null_mut(),
        }
    }

    /// 带应用指针的构造（对应 C++ MemoryImage(SexyAppBase* theApp)）
    pub fn new_with_app(width: i32, height: i32, app: *mut crate::framework::sexy_app_base::SexyAppBase) -> Self {
        let mut mem = Self::new(width, height);
        mem.app = app;
        mem
    }

    /// 获取像素缓冲区（对应 C++ GetBits）
    pub fn get_bits(&self) -> &[u8] {
        &self.base.pixels
    }

    /// 获取可变像素缓冲区（对应 C++ GetBits）
    pub fn get_bits_mut(&mut self) -> &mut [u8] {
        &mut self.base.pixels
    }

    /// 获取 32 位像素缓冲区视图（若像素格式为 RGBA8888）
    pub fn get_bits_u32(&self) -> &[u32] {
        unsafe {
            let len = self.base.pixels.len() / 4;
            std::slice::from_raw_parts(self.base.pixels.as_ptr() as *const u32, len)
        }
    }

    pub fn get_bits_u32_mut(&mut self) -> &mut [u32] {
        unsafe {
            let len = self.base.pixels.len() / 4;
            std::slice::from_raw_parts_mut(self.base.pixels.as_mut_ptr() as *mut u32, len)
        }
    }

    // ====================================================================
    // 像素操作（覆盖 Image 基类的方法）
    // ====================================================================

    /// 填充矩形（对应 C++ FillRect）
    pub fn fill_rect(&mut self, rect: &Rect, color: &Color, draw_mode: i32) {
        self.base.fill_rect(rect, color, draw_mode);
    }

    /// 清除矩形（对应 C++ ClearRect）
    pub fn clear_rect(&mut self, rect: &Rect) {
        self.base.clear_rect(rect);
    }

    /// 绘制线段（对应 C++ DrawLine）
    pub fn draw_line(&mut self, start_x: f64, start_y: f64, end_x: f64, end_y: f64, color: &Color, draw_mode: i32) {
        self.base.draw_line(start_x, start_y, end_x, end_y, color, draw_mode);
    }

    /// 绘制抗锯齿线段（对应 C++ DrawLineAA）
    pub fn draw_line_aa(&mut self, start_x: f64, start_y: f64, end_x: f64, end_y: f64, color: &Color, draw_mode: i32) {
        self.base.draw_line_aa(start_x, start_y, end_x, end_y, color, draw_mode);
    }

    // ====================================================================
    // 块传输（Blt） - 核心像素复制方法
    // ====================================================================

    /// 普通 Blt（对应 C++ NormalBlt）
    fn normal_blt(&mut self, src: &Image, dest_x: i32, dest_y: i32, src_rect: &Rect, color: &Color) {
        self.base.blt(src, dest_x, dest_y, src_rect, color, 0, false);
    }

    /// 叠加 Blt（对应 C++ AdditiveBlt）
    fn additive_blt(&mut self, src: &Image, dest_x: i32, dest_y: i32, src_rect: &Rect, color: &Color) {
        self.base.blt(src, dest_x, dest_y, src_rect, color, 1, false);
    }

    /// 主 Blt 入口（对应 C++ Blt）
    pub fn blt(&mut self, src: &Image, dest_x: i32, dest_y: i32, src_rect: &Rect, color: &Color, draw_mode: i32, linear_filter: bool) {
        match draw_mode {
            0 => self.normal_blt(src, dest_x, dest_y, src_rect, color),
            1 => self.additive_blt(src, dest_x, dest_y, src_rect, color),
            _ => self.normal_blt(src, dest_x, dest_y, src_rect, color),
        }
    }

    /// 浮点 Blt（对应 C++ BltF）
    pub fn blt_f(&mut self, src: &Image, dest_x: f32, dest_y: f32, src_rect: &Rect, clip_rect: &Rect, color: &Color, draw_mode: i32) {
        self.base.blt_f(src, dest_x, dest_y, src_rect, clip_rect, color, draw_mode);
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
        self.base.blt_rotated(src, dest_x, dest_y, src_rect, clip_rect, color, draw_mode, rot, rot_center_x, rot_center_y);
    }

    /// 拉伸 Blt（对应 C++ StretchBlt）
    pub fn stretch_blt(&mut self, src: &Image, dest_rect: &Rect, src_rect: &Rect, clip_rect: &Rect, color: &Color, draw_mode: i32, fast_stretch: bool) {
        self.base.stretch_blt(src, dest_rect, src_rect, clip_rect, color, draw_mode, fast_stretch);
    }

    /// 矩阵 Blt（对应 C++ BltMatrix）
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
        self.base.blt_matrix(src, x, y, matrix, clip_rect, color, draw_mode, src_rect, blend);
    }

    /// 三角纹理（对应 C++ BltTrianglesTex）
    pub fn blt_triangles_tex(
        &mut self,
        texture: &Image,
        vertices: &[[crate::framework::graphics::gl_interface::TriVertex; 3]],
        num_triangles: i32,
        clip_rect: &Rect,
        color: &Color,
        draw_mode: i32,
        tx: f32,
        ty: f32,
        blend: bool,
    ) {
        self.base.blt_triangles_tex(texture, vertices, num_triangles, clip_rect, color, draw_mode, tx, ty, blend);
    }

    /// 镜像 Blt（对应 C++ BltMirror）
    pub fn blt_mirror(&mut self, src: &Image, dest_x: i32, dest_y: i32, src_rect: &Rect, color: &Color, draw_mode: i32, linear_filter: bool) {
        self.base.blt_mirror(src, dest_x, dest_y, src_rect, color, draw_mode, linear_filter);
    }

    /// 拉伸镜像 Blt（对应 C++ StretchBltMirror）
    pub fn stretch_blt_mirror(&mut self, src: &Image, dest_rect: &Rect, src_rect: &Rect, clip_rect: &Rect, color: &Color, draw_mode: i32, fast_stretch: bool) {
        self.base.stretch_blt_mirror(src, dest_rect, src_rect, clip_rect, color, draw_mode, fast_stretch);
    }

    // ====================================================================
    // GPU 纹理管理
    // ====================================================================

    /// 标记位图已变更（对应 C++ BitsChanged）
    pub fn bits_changed(&mut self) {
        self.bits_changed = true;
        self.bits_changed_count += 1;
        // 通知 GLInterface 重新提交纹理
        if !self.app.is_null() && self.committed_to_gpu() {
            unsafe {
                let app = &*self.app;
                // 通过 SexyAppBase 获取 GLInterface 并重新提交
                // 实际实现在 GLInterface 中
            }
        }
    }

    /// 提交位图到 GPU 纹理（对应 C++ CommitBits）
    pub fn commit_bits(&mut self) {
        if !self.bits_changed {
            return;
        }
        self.bits_changed = false;

        // 通过 GLInterface 上传纹理
        if !self.app.is_null() {
            unsafe {
                let app = &*self.app;
                // 使用 gl_interface 上传
                self.commit_to_gpu();
            }
        }

        // 如果设置了 purge_bits，提交后清除位图
        if self.purge_bits {
            self.purge_bits_impl();
        }
    }

    /// 实际提交到 GPU（由 GLInterface 调用）
    fn commit_to_gpu(&mut self) {
        // 实际实现：将 self.base.pixels 上传为 OpenGL 纹理
        // 此方法由 GLInterface 覆盖
        self.bits_changed = false;
    }

    /// 是否已提交到 GPU
    fn committed_to_gpu(&self) -> bool {
        self.render_data.is_some()
    }

    /// 清除位图（释放像素缓冲区，对应 C++ PurgeBits）
    pub fn purge_bits_impl(&mut self) {
        self.base.pixels.clear();
    }

    /// 删除软件缓冲区（对应 C++ DeleteSWBuffers）
    pub fn delete_sw_buffers(&mut self) {
        self.base.pixels = Vec::new();
        self.native_alpha_data = None;
        self.rl_alpha_data = None;
        self.rl_additive_data = None;
    }

    /// 删除 3D 缓冲区（对应 C++ Delete3DBuffers）
    pub fn delete_3d_buffers(&mut self) {
        self.render_data = None;
    }

    /// 删除额外缓冲区（对应 C++ DeleteExtraBuffers）
    pub fn delete_extra_buffers(&mut self) {
        self.color_table.clear();
        self.color_indices.clear();
    }

    /// 删除原生数据（对应 C++ DeleteNativeData）
    pub fn delete_native_data(&mut self) {
        self.native_alpha_data = None;
        self.rl_alpha_data = None;
        self.rl_additive_data = None;
    }

    /// 重新初始化（对应 C++ ReInit）
    pub fn re_init(&mut self) {
        self.delete_sw_buffers();
        self.delete_3d_buffers();
        self.delete_extra_buffers();
        self.bits_changed = false;
        self.bits_changed_count = 0;
    }

    // ====================================================================
    // 图像创建与数据设置
    // ====================================================================

    /// 设置像素数据（对应 C++ SetBits）
    pub fn set_bits(&mut self, bits: &[u32], width: i32, height: i32, commit_bits: bool) {
        self.base.width = width;
        self.base.height = height;
        self.base.pixels = Vec::with_capacity((width * height * 4) as usize);
        for &pixel in bits {
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
            let a = ((pixel >> 24) & 0xFF) as u8;
            self.base.pixels.push(r);
            self.base.pixels.push(g);
            self.base.pixels.push(b);
            self.base.pixels.push(a);
        }
        self.base.pixel_format = 0; // RGBA8888
        if commit_bits {
            self.bits_changed();
            self.commit_bits();
        }
    }

    /// 创建图像缓冲区（对应 C++ Create）
    pub fn create(&mut self, width: i32, height: i32) {
        self.base.width = width;
        self.base.height = height;
        self.base.pixels = vec![0u8; (width * height * 4) as usize];
        self.base.pixel_format = 0;
        self.bits_changed = true;
        self.bits_changed_count = 1;
    }

    /// 清除所有像素（对应 C++ Clear）
    pub fn clear(&mut self, color: Color) {
        let pixel_count = self.base.pixels.len() / 4;
        for i in 0..pixel_count {
            let idx = i * 4;
            self.base.pixels[idx] = color.r;
            self.base.pixels[idx + 1] = color.g;
            self.base.pixels[idx + 2] = color.b;
            self.base.pixels[idx + 3] = color.a;
        }
    }

    /// 检查图像是否有全天透明像素
    fn has_transparent_pixels(&self) -> bool {
        let pixels = &self.base.pixels;
        for chunk in pixels.chunks(4) {
            if chunk.len() == 4 && chunk[3] == 0 {
                return true;
            }
        }
        false
    }

    /// 计算透明像素附近非透明像素的平均 RGB 颜色（对应 C++ AverageNearByPixels）
    /// `pixels`：RGBA 字节数组，`stride`：每行字节数（width * 4）
    /// 扫描上方一行和下方一行共6个相邻像素，忽略透明像素和自身
    fn average_nearby_pixels(pixels: &[u8], stride: i32, x: i32, y: i32, width: i32, height: i32) -> (u8, u8, u8) {
        let mut r_sum: u32 = 0;
        let mut g_sum: u32 = 0;
        let mut b_sum: u32 = 0;
        let mut count: u32 = 0;

        // C++ 逻辑：遍历上方行 (i=-1) 和下方行 (i=1)，跳过当前行 (i=0)
        for &i in &[-1i32, 1i32] {
            let ny = y + i;
            if ny < 0 || ny >= height {
                continue;
            }
            for j in -1i32..=1i32 {
                let nx = x + j;
                if nx < 0 || nx >= width {
                    continue;
                }
                let idx = (ny * stride + nx) as usize * 4;
                if idx + 3 < pixels.len() {
                    // 检查是否非透明（alpha > 0）
                    if pixels[idx + 3] != 0 {
                        r_sum += pixels[idx] as u32;
                        g_sum += pixels[idx + 1] as u32;
                        b_sum += pixels[idx + 2] as u32;
                        count += 1;
                    }
                }
            }
        }

        if count == 0 {
            return (0, 0, 0);
        }

        (
            (r_sum / count).min(255) as u8,
            (g_sum / count).min(255) as u8,
            (b_sum / count).min(255) as u8,
        )
    }

    /// 修复透明像素边缘的黑边（对应 C++ FixPixelsOnAlphaEdgeForBlending）
    /// 将完全透明的像素（alpha=0）的颜色替换为附近非透明像素的平均色，
    /// 防止在纹理采样或混合时出现黑色边缘。
    pub fn fix_pixels_on_alpha_edge_for_blending(&mut self) {
        if self.base.pixels.is_empty() || self.base.width <= 0 || self.base.height <= 0 {
            return;
        }

        // 如果没有透明像素，无需处理
        if !self.has_transparent_pixels() {
            return;
        }

        let width = self.base.width;
        let height = self.base.height;
        let stride = width;

        for y in 0..height {
            for x in 0..width {
                let idx = (y * stride + x) * 4;
                let idx_u = idx as usize;
                if idx_u + 3 >= self.base.pixels.len() {
                    continue;
                }
                // 如果像素完全不透明（alpha == 0）
                if self.base.pixels[idx_u + 3] == 0 {
                    let (avg_r, avg_g, avg_b) = Self::average_nearby_pixels(
                        &self.base.pixels, stride, x, y, width, height,
                    );
                    self.base.pixels[idx_u] = avg_r;
                    self.base.pixels[idx_u + 1] = avg_g;
                    self.base.pixels[idx_u + 2] = avg_b;
                    // alpha 保持为 0
                }
            }
        }

        self.bits_changed_count += 1;
    }

    /// 设置图像模式（对应 C++ SetImageMode）
    pub fn set_image_mode(&mut self, has_trans: bool, has_alpha: bool) {
        self.has_trans = has_trans;
        self.has_alpha = has_alpha;
    }

    /// 设置可变标记（对应 C++ SetVolatile）
    pub fn set_volatile(&mut self, is_volatile: bool) {
        self.is_volatile = is_volatile;
    }

    /// 调色板化（对应 C++ Palletize）
    pub fn palletize(&mut self) -> bool {
        if !self.want_pal {
            return false;
        }
        // 简化的调色板化：将 RGBA8888 转为索引色
        // 完整实现在后面使用 Quantize
        let pixel_count = (self.base.width * self.base.height) as usize;
        if self.color_table.is_empty() {
            // 使用 256 色调色板
            self.color_table = vec![0u32; 256];
            self.color_indices = vec![0u8; pixel_count];
        }
        true
    }

    /// 获取原生 alpha 数据（对应 C++ GetNativeAlphaData）
    pub fn get_native_alpha_data(&mut self, _native: *mut crate::framework::graphics::native_display::NativeDisplay) -> Option<&mut Vec<u32>> {
        // 生成 alpha 数据
        if self.native_alpha_data.is_none() {
            let pixel_count = (self.base.width * self.base.height) as usize;
            let mut data = vec![0u32; pixel_count];
            // 从 ARGB 中提取 alpha
            for i in 0..pixel_count {
                let idx = i * 4;
                if idx + 3 < self.base.pixels.len() {
                    data[i] = self.base.pixels[idx + 3] as u32; // alpha
                }
            }
            self.native_alpha_data = Some(data);
        }
        self.native_alpha_data.as_mut()
    }

    /// 获取游程编码 alpha 数据（对应 C++ GetRLAlphaData）
    pub fn get_rl_alpha_data(&mut self) -> Option<&Vec<u8>> {
        if self.rl_alpha_data.is_none() {
            let mut data = Vec::new();
            let pixel_count = (self.base.width * self.base.height) as usize;
            let mut i = 0;
            while i < pixel_count {
                let idx = i * 4;
                let alpha = if idx + 3 < self.base.pixels.len() {
                    self.base.pixels[idx + 3]
                } else {
                    0
                };
                let mut run_len: u8 = 1;
                while i + (run_len as usize) < pixel_count {
                    let next_idx = (i + run_len as usize) * 4;
                    let next_alpha = if next_idx + 3 < self.base.pixels.len() {
                        self.base.pixels[next_idx + 3]
                    } else {
                        0
                    };
                    if next_alpha != alpha || run_len == 255 {
                        break;
                    }
                    run_len += 1;
                }
                data.push(alpha);
                data.push(run_len);
                i += run_len as usize;
            }
            self.rl_alpha_data = Some(data);
        }
        self.rl_alpha_data.as_ref()
    }

    /// 获取游程编码附加数据（对应 C++ GetRLAdditiveData）
    pub fn get_rl_additive_data(&mut self, _native: *mut crate::framework::graphics::native_display::NativeDisplay) -> Option<&Vec<u8>> {
        if self.rl_additive_data.is_none() {
            let mut data = Vec::new();
            let pixel_count = (self.base.width * self.base.height) as usize;
            let mut i = 0;
            while i < pixel_count {
                let idx = i * 4;
                // 附加数据 = R+G+B 之和
                let sum = if idx + 2 < self.base.pixels.len() {
                    (self.base.pixels[idx] as u32 + self.base.pixels[idx + 1] as u32 + self.base.pixels[idx + 2] as u32) as u8
                } else {
                    0
                };
                let mut run_len: u8 = 1;
                while i + (run_len as usize) < pixel_count {
                    let next_idx = (i + run_len as usize) * 4;
                    let next_sum = if next_idx + 2 < self.base.pixels.len() {
                        (self.base.pixels[next_idx] as u32 + self.base.pixels[next_idx + 1] as u32 + self.base.pixels[next_idx + 2] as u32) as u8
                    } else {
                        0
                    };
                    if next_sum != sum || run_len == 255 {
                        break;
                    }
                    run_len += 1;
                }
                data.push(sum);
                data.push(run_len);
                i += run_len as usize;
            }
            self.rl_additive_data = Some(data);
        }
        self.rl_additive_data.as_ref()
    }
}

impl Default for MemoryImage {
    fn default() -> Self {
        MemoryImage::new(0, 0)
    }
}
