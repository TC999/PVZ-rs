// PvZ Portable Rust 翻译 — GLInterface（OpenGL 渲染管线）
// 对应 C++ SexyAppFramework/graphics/GLInterface.h / GLInterface.cpp
// 完整实现：着色器编译、顶点批处理、纹理分片、Blt/FillRect 等

#![allow(dead_code, non_camel_case_types, non_snake_case)]

/// 未初始化图元模式的标记值（对应 C++ 的 (GLenum)-1）
const GFX_MODE_NONE: u32 = u32::MAX;

use std::ptr;
use std::sync::OnceLock;

use crate::ffi::opengl::*;
use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::image::Image;
use crate::framework::graphics::gl_image::GLImage;
use crate::framework::graphics::memory_image::MemoryImage;
use crate::framework::sexy_matrix::SexyMatrix3;
use crate::framework::sexy_app_base::SexyAppBase;

// ============================================================
// 常量
// ============================================================

const MAX_VERTICES: usize = 16384;

// ============================================================
// 类型定义（匹配 C++ GLVertex / VertexList / TextureData 等）
// ============================================================

/// OpenGL 顶点（对应 C++ GLVertex）
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GLVertex {
    pub sx: f32,
    pub sy: f32,
    pub sz: f32,
    pub color: u32,
    pub tu: f32,
    pub tv: f32,
}

/// 三角顶点（对应 C++ TriVertex，从 TodLib 引入）
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TriVertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub color: u32,
}

/// 渲染图像标记（对应 C++ RenderImageFlags）
pub const RENDER_IMAGE_FLAG_MINIMIZE_NUM_SUBDIVISIONS: u32 = 0x0001;
pub const RENDER_IMAGE_FLAG_USE64_BY64_SUBDIVISIONS: u32 = 0x0002;
pub const RENDER_IMAGE_FLAG_USE_A4R4G4B4: u32 = 0x0004;
pub const RENDER_IMAGE_FLAG_USE_A8R8G8B8: u32 = 0x0008;
pub const RENDER_IMAGE_FLAG_REPEAT: u32 = 0x0010;
pub const RENDER_IMAGE_FLAG_TEXTURE_MASK: u32 = 0x001F;

/// 像素格式（对应 C++ PixelFormat）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PixelFormat {
    Unknown = 0x0000,
    A8R8G8B8 = 0x0001,
    A4R4G4B4 = 0x0002,
    R5G6B5 = 0x0004,
    Palette8 = 0x0008,
}

/// 纹理数据片（对应 C++ TextureDataPiece）
#[derive(Debug, Clone)]
#[repr(C)]
pub struct TextureDataPiece {
    pub texture: GLuint,
    pub width: i32,
    pub height: i32,
}

impl TextureDataPiece {
    pub const fn new() -> Self {
        TextureDataPiece { texture: 0, width: 0, height: 0 }
    }
}

// ============================================================
// 全局 GL 状态（对应 C++ 文件作用域静态变量）
// 注意：游戏循环是单线程的，使用 static mut 是安全的。
// 所有访问通过 unsafe 包装函数进行。
// ============================================================

static mut G_VERTICES: Vec<GLVertex> = Vec::new();
static mut G_NUM_VERTICES: usize = 0;
static mut G_VERTEX_MODE: u32 = GFX_MODE_NONE;
static mut G_PROGRAM: GLuint = 0;
static mut G_VBO: GLuint = 0;
static mut G_UF_VIEW_PROJ_MTX: GLint = -1;
static mut G_UF_TEXTURE: GLint = -1;
static mut G_UF_USE_TEXTURE: GLint = -1;
static mut G_UF_UV_BOUNDS: GLint = -1;
static mut G_UF_CLAMP_UV_ENABLED: GLint = -1;
static mut G_LINEAR_FILTER: bool = false;

// 初始化标记
static GL_INITED: OnceLock<bool> = OnceLock::new();

// 纹理尺寸全局参数
static mut G_MIN_TEXTURE_WIDTH: i32 = 8;
static mut G_MIN_TEXTURE_HEIGHT: i32 = 8;
static mut G_MAX_TEXTURE_WIDTH: i32 = 1024;
static mut G_MAX_TEXTURE_HEIGHT: i32 = 1024;
static mut G_SUPPORTED_PIXEL_FORMATS: i32 = 0;
static mut G_MAX_TEXTURE_SIZE: i32 = 1024;
static mut G_DESKTOP_GL_FALLBACK: bool = false;

// 默认 UV 边界
static DEFAULT_UV_BOUNDS: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

// ============================================================
// 全局状态访问辅助函数（unsafe 封装）
// ============================================================

#[inline]
unsafe fn gfx_set_vertex_mode(mode: GLenum) {
    G_VERTEX_MODE = mode;
}

#[inline]
unsafe fn gfx_vertex_mode() -> GLenum {
    G_VERTEX_MODE
}

#[inline]
unsafe fn gfx_set_num_vertices(n: usize) {
    G_NUM_VERTICES = n;
}

#[inline]
unsafe fn gfx_num_vertices() -> usize {
    G_NUM_VERTICES
}

#[inline]
unsafe fn gfx_vertices_mut() -> &'static mut Vec<GLVertex> {
    &mut G_VERTICES
}

#[inline]
unsafe fn gfx_linear_filter() -> bool {
    G_LINEAR_FILTER
}

#[inline]
unsafe fn gfx_set_linear_filter(v: bool) {
    G_LINEAR_FILTER = v;
}

// ============================================================
// 颜色转换辅助函数（对应 C++ ArgbToRgba / RgbaToArgb）
// ============================================================

/// ARGB → RGBA（OpenGL 使用的小端 RGBA）
#[inline]
fn argb_to_rgba(argb: u32) -> u32 {
    let abgr = (argb & 0xFF00FF00u32)
        | ((argb >> 16) & 0x000000FFu32)
        | ((argb << 16) & 0x00FF0000u32);
    abgr.to_le()
}

/// RGBA → ARGB（从 OpenGL 读回）
#[inline]
fn rgba_to_argb(rgba: u32) -> u32 {
    let abgr = u32::from_le(rgba);
    (abgr & 0xFF00FF00u32)
        | ((abgr >> 16) & 0x000000FFu32)
        | ((abgr << 16) & 0x00FF0000u32)
}

/// Color → GL 颜色（RGBA）
#[inline]
fn color_to_gl_color(c: &Color) -> u32 {
    argb_to_rgba(c.to_argb())
}

/// 从顶点颜色或 fallback 颜色获取 GL 颜色
#[inline]
fn vertex_color(tri_vertex_color: u32, fallback: u32) -> u32 {
    if tri_vertex_color != 0 { argb_to_rgba(tri_vertex_color) } else { fallback }
}

// ============================================================
// 顶点批处理系统（对应 C++ GfxBegin / GfxEnd / GfxAddVertices）
// ============================================================

/// 开始图元批处理
unsafe fn gfx_begin(vertex_mode: GLenum) {
    if gfx_vertex_mode() != GFX_MODE_NONE {
        return; // 已经开始了
    }
    gfx_set_vertex_mode(vertex_mode);
}

/// 结束图元批处理 — 提交所有顶点到 GPU
unsafe fn gfx_end() {
    if gfx_vertex_mode() == GFX_MODE_NONE {
        return;
    }

    let num_verts = gfx_num_vertices();
    if num_verts > 0 {
        let verts = gfx_vertices_mut();
        glBindBuffer(GL_ARRAY_BUFFER, G_VBO);
        glBufferData(
            GL_ARRAY_BUFFER,
            (size_of::<GLVertex>() * num_verts) as GLsizeiptr,
            verts.as_ptr() as *const std::ffi::c_void,
            GL_DYNAMIC_DRAW,
        );

        // 顶点属性：位置 (3 floats)
        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, size_of::<GLVertex>() as GLsizei, ptr::null());
        glEnableVertexAttribArray(0);
        // 颜色 (4 bytes, normalized)
        glVertexAttribPointer(
            1, 4, GL_UNSIGNED_BYTE, GL_TRUE, size_of::<GLVertex>() as GLsizei,
            (size_of::<f32>() * 3) as *const std::ffi::c_void,
        );
        glEnableVertexAttribArray(1);
        // UV (2 floats)
        glVertexAttribPointer(
            2, 2, GL_FLOAT, GL_FALSE, size_of::<GLVertex>() as GLsizei,
            (size_of::<f32>() * 3 + size_of::<u32>()) as *const std::ffi::c_void,
        );
        glEnableVertexAttribArray(2);

        glDrawArrays(gfx_vertex_mode(), 0, num_verts as GLsizei);
    }

    gfx_set_vertex_mode(GFX_MODE_NONE);
    gfx_set_num_vertices(0);
    // 不清空 Vec 以便复用容量，但重置逻辑长度
    gfx_vertices_mut().clear();
}

/// 如果超过预算则刷新
unsafe fn gfx_flush_if_over_budget() {
    if gfx_vertex_mode() == GFX_MODE_NONE || gfx_num_vertices() < MAX_VERTICES {
        return;
    }
    let old_mode = gfx_vertex_mode();
    gfx_end();
    gfx_begin(old_mode);
}

/// 添加顶点数组到批处理
unsafe fn gfx_add_vertices(arr: *const GLVertex, arr_count: i32) {
    if gfx_vertex_mode() == GFX_MODE_NONE || arr_count <= 0 {
        return;
    }
    let old_count = gfx_num_vertices();
    let new_count = old_count + arr_count as usize;
    gfx_set_num_vertices(new_count);
    let verts = gfx_vertices_mut();
    verts.reserve(arr_count as usize);
    // 从 arr 复制到 verts 尾部
    let src = arr as *const u8;
    let dst = verts.as_mut_ptr().add(old_count) as *mut u8;
    let byte_len = (arr_count as usize) * size_of::<GLVertex>();
    ptr::copy_nonoverlapping(src, dst, byte_len);
    // 更新 Vec 长度
    verts.set_len(new_count);
    gfx_flush_if_over_budget();
}

/// 从 VertexList 添加顶点
unsafe fn gfx_add_vertices_from_list(list: &VertexList) {
    gfx_add_vertices(list.mverts.as_ptr(), list.msize as i32);
}

/// 从 TriVertex 数组添加顶点（纹理三角形）
unsafe fn gfx_add_vertices_tri(
    arr: &[[TriVertex; 3]],
    arr_count: i32,
    the_color: u32,
    tx: f32,
    ty: f32,
    max_total_u: f32,
    max_total_v: f32,
) {
    if gfx_vertex_mode() == GFX_MODE_NONE || arr_count <= 0 {
        return;
    }
    let old_count = gfx_num_vertices();
    let new_count = old_count + (arr_count as usize) * 3;
    gfx_set_num_vertices(new_count);
    let verts = gfx_vertices_mut();
    verts.reserve((arr_count as usize) * 3);
    verts.set_len(new_count);

    let dst_ptr = verts.as_mut_ptr().add(old_count) as *mut GLVertex;
    for tri in 0..arr_count as usize {
        let v = &arr[tri];
        for i in 0..3 {
            let d = dst_ptr.add(tri * 3 + i);
            (*d).sx = v[i].x + tx;
            (*d).sy = v[i].y + ty;
            (*d).sz = 0.0;
            (*d).color = vertex_color(v[i].color, the_color);
            (*d).tu = v[i].u * max_total_u;
            (*d).tv = v[i].v * max_total_v;
        }
    }
    gfx_flush_if_over_budget();
}

// ============================================================
// 着色器系统（对应 C++ shaderCompile / shaderLoad / SHADER_CODE）
// ============================================================

/// 统一着色器代码（对应 C++ SHADER_CODE 宏）
/// Vertex: VERT_IN/V2F 宏由 GLPlatform 提供，这里使用标准 GLSL
const SHADER_CODE: &str = r"#ifdef VERTEX
	uniform mat4 u_viewProj;
	attribute vec3 a_position;
	attribute vec4 a_color;
	attribute vec2 a_uv;
	varying vec4 v_color;
	varying vec2 v_uv;
	void main() {
		v_color = a_color;
		v_uv = a_uv;
		gl_Position = u_viewProj * vec4(a_position, 1.0);
	}
#endif
#ifdef FRAGMENT
	uniform sampler2D u_texture;
	uniform int u_useTexture;
	uniform vec4 u_uvBounds;
	uniform int u_clampUvEnabled;
	varying vec4 v_color;
	varying vec2 v_uv;
	void main() {
		if (u_useTexture != 0) {
			vec2 uv = (u_clampUvEnabled != 0) ? clamp(v_uv, u_uvBounds.xy, u_uvBounds.zw) : v_uv;
			gl_FragColor = texture2D(u_texture, uv) * v_color;
		} else {
			gl_FragColor = v_color;
		}
	}
#endif
";

/// 编译单个着色器
unsafe fn shader_compile(src: &str, shader_type: GLenum) -> GLuint {
    let is_desktop = G_DESKTOP_GL_FALLBACK;
    let version_line = if is_desktop {
        "#version 120\n\0"
    } else {
        "#version 100\nprecision mediump float;\n\0"
    };

    let macros = if shader_type == GL_VERTEX_SHADER {
        "#define VERTEX\n\0"
    } else {
        "#define FRAGMENT\n\0"
    };

    let src_bytes = src.as_bytes();
    let strings: [*const GLchar; 3] = [
        version_line.as_ptr() as *const GLchar,
        macros.as_ptr() as *const GLchar,
        src_bytes.as_ptr() as *const GLchar,
    ];
    let lengths: [GLint; 3] = [
        version_line.len() as GLint - 1, // 减掉 null
        macros.len() as GLint - 1,
        src_bytes.len() as GLint,
    ];

    let shader = glCreateShader(shader_type);
    glShaderSource(shader, 3, strings.as_ptr(), lengths.as_ptr());
    glCompileShader(shader);

    let mut ok: GLint = 0;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &mut ok);
    if ok == 0 {
        let mut log_len: GLint = 0;
        glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut log_len);
        let mut log_buf: Vec<u8> = vec![0u8; log_len as usize];
        glGetShaderInfoLog(shader, log_len, &mut log_len, log_buf.as_mut_ptr() as *mut GLchar);
        let log_str = std::str::from_utf8(&log_buf[..log_len as usize]).unwrap_or("?");
        // 使用 eprintln 避免依赖 SexyAppBase
        eprintln!(
            "Shader error: {}\nsource:\n{}\n",
            log_str,
            src,
        );
        glDeleteShader(shader);
        return 0;
    }
    shader
}

/// 加载并链接着色器程序
unsafe fn shader_load(src: &str) -> GLuint {
    let vert = shader_compile(src, GL_VERTEX_SHADER);
    let frag = shader_compile(src, GL_FRAGMENT_SHADER);
    if vert == 0 || frag == 0 {
        if vert != 0 {
            glDeleteShader(vert);
        }
        if frag != 0 {
            glDeleteShader(frag);
        }
        return 0;
    }

    let prog = glCreateProgram();
    glAttachShader(prog, vert);
    glAttachShader(prog, frag);

    let attribs: [*const GLchar; 3] = [
        "a_position\0".as_ptr() as *const GLchar,
        "a_color\0".as_ptr() as *const GLchar,
        "a_uv\0".as_ptr() as *const GLchar,
    ];
    for i in 0..3 {
        glBindAttribLocation(prog, i as GLuint, attribs[i]);
    }

    glLinkProgram(prog);
    glDeleteShader(vert);
    glDeleteShader(frag);
    prog
}

/// 构建正交投影矩阵（对应 C++ MakeOrthoMatrix）
fn make_ortho_matrix(l: f32, r: f32, b: f32, t: f32, n: f32, f_: f32) -> [f32; 16] {
    [
        2.0 / (r - l), 0.0, 0.0, 0.0,
        0.0, 2.0 / (t - b), 0.0, 0.0,
        0.0, 0.0, -2.0 / (f_ - n), 0.0,
        -(r + l) / (r - l), -(t + b) / (t - b), -(f_ + n) / (f_ - n), 1.0,
    ]
}

// ============================================================
// 纹理辅助函数（对应 C++ CopyImageToTexture 系列）
// ============================================================

/// 设置纹理过滤模式
unsafe fn set_linear_filter(linear: bool) {
    gfx_set_linear_filter(linear);
}

/// 绑定纹理并设置参数
unsafe fn gfx_bind_texture(tex: GLuint, uv_bounds: &[f32; 4], clamp_uv: bool) {
    glBindTexture(GL_TEXTURE_2D, tex);
    let f = if gfx_linear_filter() { GL_LINEAR } else { GL_NEAREST };
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, f as GLint);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, f as GLint);
    glUniform1i(G_UF_CLAMP_UV_ENABLED, if clamp_uv { 1 } else { 0 });
    glUniform4fv(G_UF_UV_BOUNDS, 1, uv_bounds.as_ptr());
}

/// 获取最近的 2 的幂
fn closest_power_of_2_above(n: i32) -> i32 {
    let mut p = 1;
    while p < n {
        p <<= 1;
    }
    p
}

/// 判断是否为 2 的幂
fn is_power_of_2(n: i32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

// ============================================================
// TextureData — 纹理数据管理（对应 C++ TextureData）
// ============================================================

/// 纹理数据（对应 C++ TextureData）
#[derive(Debug)]
pub struct TextureData {
    pub textures: Vec<TextureDataPiece>,
    pub width: i32,
    pub height: i32,
    pub tex_vec_width: i32,
    pub tex_vec_height: i32,
    pub tex_piece_width: i32,
    pub tex_piece_height: i32,
    pub bits_changed_count: i32,
    pub tex_mem_size: i32,
    pub max_total_u: f32,
    pub max_total_v: f32,
    pub pixel_format: PixelFormat,
    pub image_flags: u32,
    // 最佳尺寸缓存（对应 C++ 的 sGoodSizes）
    good_sizes: Option<Vec<i32>>,
}

impl TextureData {
    pub fn new() -> Self {
        TextureData {
            textures: Vec::new(),
            width: 0,
            height: 0,
            tex_vec_width: 0,
            tex_vec_height: 0,
            tex_piece_width: 64,
            tex_piece_height: 64,
            bits_changed_count: 0,
            tex_mem_size: 0,
            max_total_u: 0.0,
            max_total_v: 0.0,
            pixel_format: PixelFormat::Unknown,
            image_flags: 0,
            good_sizes: None,
        }
    }

    /// 释放所有纹理（对应 C++ ReleaseTextures）
    pub fn release_textures(&mut self) {
        unsafe {
            for piece in &self.textures {
                if piece.texture != 0 {
                    glDeleteTextures(1, &piece.texture as *const GLuint);
                }
            }
        }
        self.textures.clear();
        self.tex_mem_size = 0;
    }

    /// 获取最佳纹理尺寸（对应 C++ GetBestTextureDimensions）
    fn get_best_texture_dimensions(&mut self, w: &mut i32, h: &mut i32, is_edge: bool, use_pow2: bool, flags: u32) {
        if flags & RENDER_IMAGE_FLAG_USE64_BY64_SUBDIVISIONS != 0 {
            *w = 64;
            *h = 64;
            return;
        }

        let max_size = unsafe { G_MAX_TEXTURE_SIZE };
        if self.good_sizes.is_none() {
            let mut sizes = vec![0i32; max_size as usize];
            let mut pow2 = 1;
            for i in 0..max_size as usize {
                if i > pow2 as usize {
                    pow2 <<= 1;
                }
                let mut good = pow2;
                if good - i as i32 > 64 {
                    good >>= 1;
                    loop {
                        let leftover = i as i32 % good;
                        if leftover < 64 || is_power_of_2(leftover) {
                            break;
                        }
                        good >>= 1;
                    }
                }
                sizes[i] = good;
            }
            self.good_sizes = Some(sizes);
        }
        let good_sizes = self.good_sizes.as_ref().unwrap();

        let max_tex_w = unsafe { G_MAX_TEXTURE_WIDTH };
        let max_tex_h = unsafe { G_MAX_TEXTURE_HEIGHT };

        let mut aw = *w;
        let mut ah = *h;
        if use_pow2 {
            if is_edge || (flags & RENDER_IMAGE_FLAG_MINIMIZE_NUM_SUBDIVISIONS != 0) {
                aw = if aw >= max_tex_w { max_tex_w } else { closest_power_of_2_above(aw) };
                ah = if ah >= max_tex_h { max_tex_h } else { closest_power_of_2_above(ah) };
            } else {
                aw = if aw >= max_tex_w { max_tex_w } else { good_sizes[aw as usize] };
                ah = if ah >= max_tex_h { max_tex_h } else { good_sizes[ah as usize] };
            }
        }
        let min_w = unsafe { G_MIN_TEXTURE_WIDTH };
        let min_h = unsafe { G_MIN_TEXTURE_HEIGHT };
        if aw < min_w { aw = min_w; }
        if ah < min_h { ah = min_h; }
        *w = aw;
        *h = ah;
    }

    /// 创建纹理尺寸（对应 C++ CreateTextureDimensions）
    fn create_texture_dimensions(&mut self, image: &MemoryImage) {
        let a_width = image.base.width;
        let a_height = image.base.height;

        let mut tpw = a_width;
        let mut tph = a_height;
        self.get_best_texture_dimensions(&mut tpw, &mut tph, false, true, self.image_flags);
        self.tex_piece_width = tpw;
        self.tex_piece_height = tph;

        let mut a_right_width = a_width % self.tex_piece_width;
        let mut a_right_height = self.tex_piece_height;
        if a_right_width > 0 {
            self.get_best_texture_dimensions(&mut a_right_width, &mut a_right_height, true, true, self.image_flags);
        } else {
            a_right_width = self.tex_piece_width;
        }

        let mut a_bottom_width = self.tex_piece_width;
        let mut a_bottom_height = a_height % self.tex_piece_height;
        if a_bottom_height > 0 {
            self.get_best_texture_dimensions(&mut a_bottom_width, &mut a_bottom_height, true, true, self.image_flags);
        } else {
            a_bottom_height = self.tex_piece_height;
        }

        let mut a_corner_width = a_right_width;
        let mut a_corner_height = a_bottom_height;
        self.get_best_texture_dimensions(&mut a_corner_width, &mut a_corner_height, true, true, self.image_flags);

        self.tex_vec_width = (a_width + self.tex_piece_width - 1) / self.tex_piece_width;
        self.tex_vec_height = (a_height + self.tex_piece_height - 1) / self.tex_piece_height;
        let total = (self.tex_vec_width * self.tex_vec_height) as usize;
        self.textures = vec![TextureDataPiece::new(); total];

        // 设置每个 piece 的默认尺寸
        for piece in &mut self.textures {
            piece.width = self.tex_piece_width;
            piece.height = self.tex_piece_height;
        }
        // 右侧列
        let vw = self.tex_vec_width as usize;
        let vh = self.tex_vec_height as usize;
        for i in (vw - 1..vw * vh).step_by(vw) {
            if i < total {
                self.textures[i].width = a_right_width;
                self.textures[i].height = a_right_height;
            }
        }
        // 底行
        let bottom_start = vw * (vh - 1);
        for i in bottom_start as usize..total {
            self.textures[i].width = a_bottom_width;
            self.textures[i].height = a_bottom_height;
        }
        // 右下角
        if let Some(last) = self.textures.last_mut() {
            last.width = a_corner_width;
            last.height = a_corner_height;
        }

        self.max_total_u = a_width as f32 / self.tex_piece_width as f32;
        self.max_total_v = a_height as f32 / self.tex_piece_height as f32;
    }

    /// 复制图像到纹理（对应 C++ CopyImageToTexture 系列）
    unsafe fn copy_image_to_texture(
        image: &MemoryImage,
        offx: i32,
        offy: i32,
        tex_w: i32,
        tex_h: i32,
        fmt: PixelFormat,
        create: bool,
    ) {
        let wrap = if image.render_flags & RENDER_IMAGE_FLAG_REPEAT != 0 {
            GL_REPEAT
        } else {
            GL_CLAMP_TO_EDGE
        };
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, wrap as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, wrap as GLint);

        let w = tex_w.min(image.base.width - offx);
        let h = tex_h.min(image.base.height - offy);
        let pad_r = w < tex_w;
        let pad_b = h < tex_h;

        if w > 0 && h > 0 {
            match fmt {
                PixelFormat::A8R8G8B8 => {
                    Self::copy_to_texture_8888(image, offx, offy, w, h, tex_w, tex_h, pad_r, pad_b, create);
                }
                PixelFormat::A4R4G4B4 => {
                    Self::copy_to_texture_4444(image, offx, offy, w, h, tex_w, tex_h, pad_r, pad_b, create);
                }
                PixelFormat::R5G6B5 => {
                    Self::copy_to_texture_565(image, offx, offy, w, h, tex_w, tex_h, pad_r, pad_b, create);
                }
                PixelFormat::Palette8 => {
                    Self::copy_to_texture_palette8(image, offx, offy, w, h, tex_w, tex_h, pad_r, pad_b, create);
                }
                PixelFormat::Unknown => {}
            }
        }
    }

    /// 从 Vec<u8> 像素缓冲区读取 RGBA 像素（4字节 → u32 RGBA）
    unsafe fn read_pixel_rgba(pixels: &[u8], idx: usize) -> u32 {
        let base = idx * 4;
        if base + 3 < pixels.len() {
            (pixels[base] as u32) << 24
                | (pixels[base + 1] as u32) << 16
                | (pixels[base + 2] as u32) << 8
                | pixels[base + 3] as u32
        } else {
            0
        }
    }

    /// CopyImageToTexture8888
    unsafe fn copy_to_texture_8888(
        image: &MemoryImage,
        offx: i32,
        offy: i32,
        w: i32,
        h: i32,
        pitch: i32,
        dst_h: i32,
        pad_r: bool,
        pad_b: bool,
        create: bool,
    ) {
        let dst_size = (pitch * dst_h) as usize;
        let mut a_dst: Vec<u32> = vec![0u32; dst_size];
        let dst = a_dst.as_mut_ptr();
        let img_w = image.base.width;

        if image.color_table.is_empty() {
            for y in 0..h {
                let d = dst.add((y * pitch) as usize);
                for x in 0..w {
                    let px_idx = ((offy + y) * img_w + (offx + x)) as usize;
                    *d.add(x as usize) = Self::read_pixel_rgba(&image.base.pixels, px_idx);
                }
                if pad_r {
                    *d.add(w as usize) = *d.add(w as usize - 1);
                }
            }
        } else {
            // 调色板模式
            let src_row = image.color_indices.as_ptr().add((offy * img_w + offx) as usize);
            for y in 0..h {
                let s = src_row.add((y * img_w) as usize);
                let d = dst.add((y * pitch) as usize);
                for x in 0..w {
                    let idx = *s.add(x as usize) as usize;
                    *d.add(x as usize) = argb_to_rgba(if idx < image.color_table.len() { image.color_table[idx] } else { 0 });
                }
                if pad_r {
                    *d.add(w as usize) = *d.add(w as usize - 1);
                }
            }
        }

        if pad_b {
            let last_row = dst.add((h - 1) as usize * pitch as usize);
            for y in h..dst_h {
                ptr::copy_nonoverlapping(
                    last_row,
                    dst.add(y as usize * pitch as usize),
                    pitch as usize,
                );
            }
        }

        if create {
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, pitch, dst_h, 0, GL_RGBA, GL_UNSIGNED_BYTE, a_dst.as_ptr() as *const std::ffi::c_void);
        } else {
            glTexSubImage2D(GL_TEXTURE_2D, 0, 0, 0, pitch, dst_h, GL_RGBA, GL_UNSIGNED_BYTE, a_dst.as_ptr() as *const std::ffi::c_void);
        }
    }

    /// CopyImageToTexture4444
    unsafe fn copy_to_texture_4444(
        image: &MemoryImage,
        offx: i32,
        offy: i32,
        w: i32,
        h: i32,
        pitch: i32,
        dst_h: i32,
        pad_r: bool,
        pad_b: bool,
        create: bool,
    ) {
        let dst_size = (pitch * dst_h) as usize;
        let mut a_dst: Vec<u16> = vec![0u16; dst_size];
        let dst = a_dst.as_mut_ptr();
        let img_w = image.base.width;

        let argb_to_4444 = |p: u32| -> u16 {
            (((p >> 8) & 0xF000) | ((p >> 4) & 0x0F00) | (p & 0x00F0) | ((p >> 28) & 0x000F)) as u16
        };

        if image.color_table.is_empty() {
            for y in 0..h {
                let d = dst.add((y * pitch) as usize);
                for x in 0..w {
                    let px_idx = ((offy + y) * img_w + (offx + x)) as usize;
                    let rgba = Self::read_pixel_rgba(&image.base.pixels, px_idx);
                    *d.add(x as usize) = argb_to_4444(rgba);
                }
                if pad_r {
                    *d.add(w as usize) = *d.add(w as usize - 1);
                }
            }
        } else {
            let src_row = image.color_indices.as_ptr().add((offy * img_w + offx) as usize);
            for y in 0..h {
                let s = src_row.add((y * img_w) as usize);
                let d = dst.add((y * pitch) as usize);
                for x in 0..w {
                    let idx = *s.add(x as usize) as usize;
                    let pal_color = if idx < image.color_table.len() { image.color_table[idx] } else { 0 };
                    *d.add(x as usize) = argb_to_4444(pal_color);
                }
                if pad_r {
                    *d.add(w as usize) = *d.add(w as usize - 1);
                }
            }
        }

        if pad_b {
            let last_row = dst.add((h - 1) as usize * pitch as usize);
            for y in h..dst_h {
                ptr::copy_nonoverlapping(last_row, dst.add(y as usize * pitch as usize), pitch as usize);
            }
        }

        if create {
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, pitch, dst_h, 0, GL_RGBA, GL_UNSIGNED_SHORT_4_4_4_4, a_dst.as_ptr() as *const std::ffi::c_void);
        } else {
            glTexSubImage2D(GL_TEXTURE_2D, 0, 0, 0, pitch, dst_h, GL_RGBA, GL_UNSIGNED_SHORT_4_4_4_4, a_dst.as_ptr() as *const std::ffi::c_void);
        }
    }

    /// CopyImageToTexture565
    unsafe fn copy_to_texture_565(
        image: &MemoryImage,
        offx: i32,
        offy: i32,
        w: i32,
        h: i32,
        pitch: i32,
        dst_h: i32,
        pad_r: bool,
        pad_b: bool,
        create: bool,
    ) {
        let dst_size = (pitch * dst_h) as usize;
        let mut a_dst: Vec<u16> = vec![0u16; dst_size];
        let dst = a_dst.as_mut_ptr();
        let img_w = image.base.width;

        let argb_to_565 = |p: u32| -> u16 {
            (((p >> 8) & 0xF800) | ((p >> 5) & 0x07E0) | ((p >> 3) & 0x001F)) as u16
        };

        if image.color_table.is_empty() {
            for y in 0..h {
                let d = dst.add((y * pitch) as usize);
                for x in 0..w {
                    let px_idx = ((offy + y) * img_w + (offx + x)) as usize;
                    let rgba = Self::read_pixel_rgba(&image.base.pixels, px_idx);
                    *d.add(x as usize) = argb_to_565(rgba);
                }
                if pad_r {
                    *d.add(w as usize) = *d.add(w as usize - 1);
                }
            }
        } else {
            let src_row = image.color_indices.as_ptr().add((offy * img_w + offx) as usize);
            for y in 0..h {
                let s = src_row.add((y * img_w) as usize);
                let d = dst.add((y * pitch) as usize);
                for x in 0..w {
                    let idx = *s.add(x as usize) as usize;
                    let pal_color = if idx < image.color_table.len() { image.color_table[idx] } else { 0 };
                    *d.add(x as usize) = argb_to_565(pal_color);
                }
                if pad_r {
                    *d.add(w as usize) = *d.add(w as usize - 1);
                }
            }
        }

        if pad_b {
            let last_row = dst.add((h - 1) as usize * pitch as usize);
            for y in h..dst_h {
                ptr::copy_nonoverlapping(last_row, dst.add(y as usize * pitch as usize), pitch as usize);
            }
        }

        if create {
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB as GLint, pitch, dst_h, 0, GL_RGB, GL_UNSIGNED_SHORT_5_6_5, a_dst.as_ptr() as *const std::ffi::c_void);
        } else {
            glTexSubImage2D(GL_TEXTURE_2D, 0, 0, 0, pitch, dst_h, GL_RGB, GL_UNSIGNED_SHORT_5_6_5, a_dst.as_ptr() as *const std::ffi::c_void);
        }
    }

    /// CopyImageToTexturePalette8
    unsafe fn copy_to_texture_palette8(
        image: &MemoryImage,
        offx: i32,
        offy: i32,
        w: i32,
        h: i32,
        pitch: i32,
        dst_h: i32,
        pad_r: bool,
        pad_b: bool,
        create: bool,
    ) {
        let dst_size = (pitch * dst_h) as usize;
        let mut a_dst: Vec<u32> = vec![0u32; dst_size];
        let dst = a_dst.as_mut_ptr();
        let img_w = image.base.width;
        let src_row = image.color_indices.as_ptr().add((offy * img_w + offx) as usize);

        for y in 0..h {
            let s = src_row.add((y * img_w) as usize);
            let d = dst.add((y * pitch) as usize);
            for x in 0..w {
                let idx = *s.add(x as usize) as usize;
                *d.add(x as usize) = argb_to_rgba(if idx < image.color_table.len() { image.color_table[idx] } else { 0 });
            }
            if pad_r {
                *d.add(w as usize) = *d.add(w as usize - 1);
            }
        }

        if pad_b {
            let last_row = dst.add((h - 1) as usize * pitch as usize);
            for y in h..dst_h {
                ptr::copy_nonoverlapping(last_row, dst.add(y as usize * pitch as usize), pitch as usize);
            }
        }

        if create {
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, pitch, dst_h, 0, GL_RGBA, GL_UNSIGNED_BYTE, a_dst.as_ptr() as *const std::ffi::c_void);
        } else {
            glTexSubImage2D(GL_TEXTURE_2D, 0, 0, 0, pitch, dst_h, GL_RGBA, GL_UNSIGNED_BYTE, a_dst.as_ptr() as *const std::ffi::c_void);
        }
    }

    /// 创建纹理（对应 C++ CreateTextures）
    pub fn create_textures(&mut self, image: &mut MemoryImage) {
        let a_format = {
            // 确定像素格式
            let mut fmt = PixelFormat::A8R8G8B8;
            if !image.has_alpha && !image.has_trans
                && (unsafe { G_SUPPORTED_PIXEL_FORMATS } & (PixelFormat::R5G6B5 as i32)) != 0
                && (image.render_flags & RENDER_IMAGE_FLAG_USE_A8R8G8B8) == 0
            {
                fmt = PixelFormat::R5G6B5;
            }
            if (image.render_flags & RENDER_IMAGE_FLAG_USE_A4R4G4B4) != 0
                && fmt == PixelFormat::A8R8G8B8
                && (unsafe { G_SUPPORTED_PIXEL_FORMATS } & (PixelFormat::A4R4G4B4 as i32)) != 0
            {
                fmt = PixelFormat::A4R4G4B4;
            }
            if fmt == PixelFormat::A8R8G8B8 && (unsafe { G_SUPPORTED_PIXEL_FORMATS } & (PixelFormat::A8R8G8B8 as i32)) == 0 {
                fmt = PixelFormat::A4R4G4B4;
            }
            fmt
        };

        let mut create_textures = false;
        if self.width != image.base.width
            || self.height != image.base.height
            || a_format != self.pixel_format
            || (image.render_flags & RENDER_IMAGE_FLAG_TEXTURE_MASK) != self.image_flags
        {
            self.release_textures();
            self.pixel_format = a_format;
            self.image_flags = image.render_flags & RENDER_IMAGE_FLAG_TEXTURE_MASK;
            self.create_texture_dimensions(image);
            create_textures = true;
        }

        let a_height = image.base.height;
        let a_width = image.base.width;
        let fmt_size = if a_format == PixelFormat::R5G6B5 || a_format == PixelFormat::A4R4G4B4 {
            2
        } else {
            4
        };

        unsafe {
            let mut idx: usize = 0;
            let mut y = 0;
            while y < a_height {
                let mut x = 0;
                while x < a_width {
                    let piece = &mut self.textures[idx];
                    if create_textures {
                        glGenTextures(1, &mut piece.texture as *mut GLuint);
                        self.tex_mem_size += piece.width * piece.height * fmt_size;
                    }
                    glBindTexture(GL_TEXTURE_2D, piece.texture);
                    Self::copy_image_to_texture(image, x, y, piece.width, piece.height, a_format, create_textures);
                    x += self.tex_piece_width;
                    idx += 1;
                }
                y += self.tex_piece_height;
            }
        }

        self.width = image.base.width;
        self.height = image.base.height;
        self.bits_changed_count = image.bits_changed_count;
        self.pixel_format = a_format;
    }

    /// 检查并创建纹理（对应 C++ CheckCreateTextures）
    pub fn check_create_textures(&mut self, image: &mut MemoryImage) {
        if self.pixel_format == PixelFormat::Unknown
            || image.base.width != self.width
            || image.base.height != self.height
            || image.bits_changed_count != self.bits_changed_count
            || (image.render_flags & RENDER_IMAGE_FLAG_TEXTURE_MASK) != self.image_flags
        {
            self.create_textures(image);
        }
    }

    /// 获取纹理（对应 C++ GetTexture）
    pub fn get_texture(&self, x: i32, y: i32, w: &mut i32, h: &mut i32,
                       u1: &mut f32, v1: &mut f32, u2: &mut f32, v2: &mut f32,
                       uv_bounds: &mut [f32; 4]) -> GLuint {
        let tx = x / self.tex_piece_width;
        let ty = y / self.tex_piece_height;
        let p = &self.textures[(ty * self.tex_vec_width + tx) as usize];

        let left = x % self.tex_piece_width;
        let top = y % self.tex_piece_height;
        let right = (left + *w).min(p.width);
        let bottom = (top + *h).min(p.height);
        *w = right - left;
        *h = bottom - top;

        *u1 = left as f32 / p.width as f32;
        *v1 = top as f32 / p.height as f32;
        *u2 = right as f32 / p.width as f32;
        *v2 = bottom as f32 / p.height as f32;

        // 半像素内缩（用于着色器 UV 夹紧）
        let half_u = 0.5 / p.width as f32;
        let half_v = 0.5 / p.height as f32;
        let mid_u = (*u1 + *u2) * 0.5;
        let mid_v = (*v1 + *v2) * 0.5;
        uv_bounds[0] = (*u1 + half_u).min(mid_u);
        uv_bounds[1] = (*v1 + half_v).min(mid_v);
        uv_bounds[2] = (*u2 - half_u).max(mid_u);
        uv_bounds[3] = (*v2 - half_v).max(mid_v);

        p.texture
    }

    /// Blt — 纹理块绘制核心（对应 C++ TextureData::Blt）
    pub fn blt(&self, the_x: f32, the_y: f32, src_rect: &Rect, the_color: &Color) {
        let src_left = src_rect.x;
        let src_top = src_rect.y;
        let src_right = src_left + src_rect.width;
        let src_bottom = src_top + src_rect.height;
        if src_left >= src_right || src_top >= src_bottom {
            return;
        }

        let a_color = color_to_gl_color(the_color);
        unsafe {
            glActiveTexture(GL_TEXTURE0);
            glUniform1i(G_UF_USE_TEXTURE, 1);

            let mut src_y = src_top;
            let mut dst_y = the_y;
            while src_y < src_bottom {
                let mut src_x = src_left;
                let mut dst_x = the_x;
                let mut w;
                let mut h = 0;
                while src_x < src_right {
                    w = src_right - src_x;
                    h = src_bottom - src_y;
                    let mut u1: f32 = 0.0;
                    let mut v1: f32 = 0.0;
                    let mut u2: f32 = 0.0;
                    let mut v2: f32 = 0.0;
                    let mut uvb: [f32; 4] = [0.0; 4];
                    let tex = self.get_texture(src_x, src_y, &mut w, &mut h, &mut u1, &mut v1, &mut u2, &mut v2, &mut uvb);

                    let vtx: [GLVertex; 4] = [
                        GLVertex { sx: dst_x, sy: dst_y, sz: 0.0, color: a_color, tu: u1, tv: v1 },
                        GLVertex { sx: dst_x, sy: dst_y + h as f32, sz: 0.0, color: a_color, tu: u1, tv: v2 },
                        GLVertex { sx: dst_x + w as f32, sy: dst_y, sz: 0.0, color: a_color, tu: u2, tv: v1 },
                        GLVertex { sx: dst_x + w as f32, sy: dst_y + h as f32, sz: 0.0, color: a_color, tu: u2, tv: v2 },
                    ];
                    gfx_bind_texture(tex, &uvb, true);
                    gfx_begin(GL_TRIANGLE_STRIP);
                    gfx_add_vertices(vtx.as_ptr(), 4);
                    gfx_end();

                    src_x += w;
                    dst_x += w as f32;
                }
                src_y += h;
                dst_y += h as f32;
            }
        }
    }

    /// BltTransformed — 变换后纹理块绘制（对应 C++ TextureData::BltTransformed）
    pub fn blt_transformed(
        &self,
        the_trans: &SexyMatrix3,
        src_rect: &Rect,
        the_color: &Color,
        clip_rect: Option<&Rect>,
        the_x: f32,
        the_y: f32,
        center: bool,
    ) {
        let src_left = src_rect.x;
        let src_top = src_rect.y;
        let src_right = src_left + src_rect.width;
        let src_bottom = src_top + src_rect.height;
        if src_left >= src_right || src_top >= src_bottom {
            return;
        }

        let mut start_x = 0.0f32;
        let mut start_y = 0.0f32;
        if center {
            start_x = -src_rect.width as f32 / 2.0;
            start_y = -src_rect.height as f32 / 2.0;
        }

        let a_color = color_to_gl_color(the_color);
        unsafe {
            glActiveTexture(GL_TEXTURE0);
            glUniform1i(G_UF_USE_TEXTURE, 1);

            let mut src_y = src_top;
            let mut dst_y = start_y;
            while src_y < src_bottom {
                let mut src_x = src_left;
                let mut dst_x = start_x;
                let mut w;
                let mut h = 0;
                while src_x < src_right {
                    w = src_right - src_x;
                    h = src_bottom - src_y;
                    let mut u1: f32 = 0.0;
                    let mut v1: f32 = 0.0;
                    let mut u2: f32 = 0.0;
                    let mut v2: f32 = 0.0;
                    let mut uvb: [f32; 4] = [0.0; 4];
                    let tex = self.get_texture(src_x, src_y, &mut w, &mut h, &mut u1, &mut v1, &mut u2, &mut v2, &mut uvb);

                    let x = dst_x;
                    let y = dst_y;
                    let pts = [(x, y), (x, y + h as f32), (x + w as f32, y), (x + w as f32, y + h as f32)];
                    let mut tp = [(0.0f32, 0.0f32); 4];
                    for i in 0..4 {
                        let (px, py) = the_trans.transform(pts[i].0, pts[i].1);
                        tp[i] = (px + the_x, py + the_y);
                    }

                    let mut clipped = false;
                    if let Some(cr) = clip_rect {
                        let cl = cr.x as f32;
                        let ct = cr.y as f32;
                        let cr_r = (cr.x + cr.width) as f32;
                        let cr_b = (cr.y + cr.height) as f32;
                        for i in 0..4 {
                            let (px, py) = tp[i];
                            if px < cl || px >= cr_r || py < ct || py >= cr_b {
                                clipped = true;
                                break;
                            }
                        }
                    }

                    let vtx: [GLVertex; 4] = [
                        GLVertex { sx: tp[0].0, sy: tp[0].1, sz: 0.0, color: a_color, tu: u1, tv: v1 },
                        GLVertex { sx: tp[1].0, sy: tp[1].1, sz: 0.0, color: a_color, tu: u1, tv: v2 },
                        GLVertex { sx: tp[2].0, sy: tp[2].1, sz: 0.0, color: a_color, tu: u2, tv: v1 },
                        GLVertex { sx: tp[3].0, sy: tp[3].1, sz: 0.0, color: a_color, tu: u2, tv: v2 },
                    ];
                    gfx_bind_texture(tex, &uvb, true);

                    if !clipped {
                        gfx_begin(GL_TRIANGLE_STRIP);
                        gfx_add_vertices(vtx.as_ptr(), 4);
                        gfx_end();
                    } else {
                        // 使用裁剪多边形
                        let mut vl = VertexList::new();
                        vl.push_back(vtx[0]);
                        vl.push_back(vtx[1]);
                        vl.push_back(vtx[3]);
                        vl.push_back(vtx[2]);
                        draw_poly_clipped(clip_rect.unwrap(), &vl);
                    }

                    src_x += w;
                    dst_x += w as f32;
                }
                src_y += h;
                dst_y += h as f32;
            }
        }
    }

    /// BltTriangles — 三角纹理绘制（对应 C++ TextureData::BltTriangles）
    pub fn blt_triangles(
        &self,
        vertices: &[[TriVertex; 3]],
        num_triangles: i32,
        the_color: u32,
        tx: f32,
        ty: f32,
        clamp_uv: bool,
    ) {
        if self.max_total_u <= 1.0 && self.max_total_v <= 1.0 {
            // 单纹理快速路径
            let piece = &self.textures[0];
            let half_u = 0.5 / piece.width as f32;
            let half_v = 0.5 / piece.height as f32;
            let mid_u = self.max_total_u * 0.5;
            let mid_v = self.max_total_v * 0.5;
            let uvb: [f32; 4] = [
                half_u.min(mid_u),
                half_v.min(mid_v),
                (self.max_total_u - half_u).max(mid_u),
                (self.max_total_v - half_v).max(mid_v),
            ];
            unsafe {
                glActiveTexture(GL_TEXTURE0);
                gfx_bind_texture(piece.texture, &uvb, clamp_uv);
                glUniform1i(G_UF_USE_TEXTURE, 1);
                gfx_begin(GL_TRIANGLES);
                gfx_add_vertices_tri(vertices, num_triangles, the_color, tx, ty, self.max_total_u, self.max_total_v);
                gfx_end();
            }
            return;
        }

        // 多纹理路径
        for tri in 0..num_triangles as usize {
            let tv = &vertices[tri];
            let vtx: [GLVertex; 3] = [
                GLVertex { sx: tv[0].x + tx, sy: tv[0].y + ty, sz: 0.0, color: vertex_color(tv[0].color, the_color), tu: tv[0].u * self.max_total_u, tv: tv[0].v * self.max_total_v },
                GLVertex { sx: tv[1].x + tx, sy: tv[1].y + ty, sz: 0.0, color: vertex_color(tv[1].color, the_color), tu: tv[1].u * self.max_total_u, tv: tv[1].v * self.max_total_v },
                GLVertex { sx: tv[2].x + tx, sy: tv[2].y + ty, sz: 0.0, color: vertex_color(tv[2].color, the_color), tu: tv[2].u * self.max_total_u, tv: tv[2].v * self.max_total_v },
            ];

            let mut min_u = self.max_total_u;
            let mut min_v = self.max_total_v;
            let mut max_u = 0.0f32;
            let mut max_v = 0.0f32;
            for i in 0..3 {
                min_u = min_u.min(vtx[i].tu);
                max_u = max_u.max(vtx[i].tu);
                min_v = min_v.min(vtx[i].tv);
                max_v = max_v.max(vtx[i].tv);
            }

            let mut master = VertexList::new();
            master.push_back(vtx[0]);
            master.push_back(vtx[1]);
            master.push_back(vtx[2]);

            let a_left = (0i32).max(min_u.floor() as i32);
            let a_top = (0i32).max(min_v.floor() as i32);
            let a_right = self.tex_vec_width.min(max_u.ceil() as i32);
            let a_bottom = self.tex_vec_height.min(max_v.ceil() as i32);

            let std0 = &self.textures[0];
            for i in a_top..a_bottom {
                for j in a_left..a_right {
                    let piece = &self.textures[(i * self.tex_vec_width + j) as usize];
                    let half_u = 0.5 / piece.width as f32;
                    let half_v = 0.5 / piece.height as f32;
                    let uvb: [f32; 4] = [
                        half_u.min(0.5),
                        half_v.min(0.5),
                        (1.0 - half_u).max(0.5),
                        (1.0 - half_v).max(0.5),
                    ];

                    let mut vl = master.clone();
                    for k in 0..3 {
                        vl[k].tu -= j as f32;
                        vl[k].tv -= i as f32;
                        if i == self.tex_vec_height - 1 {
                            vl[k].tv *= std0.height as f32 / piece.height as f32;
                        }
                        if j == self.tex_vec_width - 1 {
                            vl[k].tu *= std0.width as f32 / piece.width as f32;
                        }
                    }
                    do_poly_texture_clip(&mut vl);
                    if vl.msize >= 3 {
                        unsafe {
                            gfx_bind_texture(piece.texture, &uvb, clamp_uv);
                            gfx_begin(GL_TRIANGLE_FAN);
                            gfx_add_vertices_from_list(&vl);
                            gfx_end();
                        }
                    }
                }
            }
        }
    }
}

impl Drop for TextureData {
    fn drop(&mut self) {
        self.release_textures();
    }
}

// ============================================================
// 裁剪系统（对应 C++ PointClipper / DrawPolyClipped / DoPolyTextureClip）
// ============================================================

/// 顶点列表（对应 C++ VertexList，使用 Vec 简化）
#[derive(Debug, Clone)]
pub struct VertexList {
    pub mverts: Vec<GLVertex>,
    pub msize: i32,
}

impl VertexList {
    pub fn new() -> Self {
        VertexList {
            mverts: Vec::with_capacity(100),
            msize: 0,
        }
    }

    pub fn push_back(&mut self, v: GLVertex) {
        if self.msize as usize >= self.mverts.len() {
            self.mverts.push(v);
        } else {
            self.mverts[self.msize as usize] = v;
        }
        self.msize += 1;
    }

    pub fn size(&self) -> i32 {
        self.msize
    }

    pub fn clear(&mut self) {
        self.msize = 0;
    }

    pub fn reserve(&mut self, capacity: i32) {
        self.mverts.reserve(capacity as usize - self.mverts.len());
    }

    pub fn set_size(&mut self, n: i32) {
        // 确保容量
        if n as usize > self.mverts.len() {
            self.mverts.resize(n as usize, GLVertex {
                sx: 0.0, sy: 0.0, sz: 0.0, color: 0, tu: 0.0, tv: 0.0,
            });
        }
        self.msize = n;
    }
}

impl std::ops::Index<usize> for VertexList {
    type Output = GLVertex;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.mverts[idx]
    }
}

impl std::ops::IndexMut<usize> for VertexList {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.mverts[idx]
    }
}

/// 获取顶点坐标分量（对应 C++ GetCoord）
#[inline]
fn get_coord(v: &GLVertex, n: i32) -> f32 {
    match n {
        0 => v.sx,
        1 => v.sy,
        2 => v.sz,
        3 => v.tu,
        4 => v.tv,
        _ => 0.0,
    }
}

/// 插值两个顶点（对应 C++ Interpolate）
fn interpolate(a: &GLVertex, b: &GLVertex, t: f32) -> GLVertex {
    let mut r = *a;
    r.sx = a.sx + t * (b.sx - a.sx);
    r.sy = a.sy + t * (b.sy - a.sy);
    r.tu = a.tu + t * (b.tu - a.tu);
    r.tv = a.tv + t * (b.tv - a.tv);
    if a.color != b.color {
        let lerp_component = |shift: i32| -> u32 {
            let a_comp = (a.color >> shift) & 0xFF;
            let b_comp = (b.color >> shift) & 0xFF;
            let v = a_comp as f32 + t * (b_comp as f32 - a_comp as f32);
            (v as u32) & 0xFF
        };
        r.color = lerp_component(0) | (lerp_component(8) << 8) | (lerp_component(16) << 16) | (lerp_component(24) << 24);
    }
    r
}

/// 裁剪一个点（对应 C++ PointClipper::ClipPoint）
fn clip_point<Pred: Fn(f32, f32) -> bool>(
    n: i32, clip_val: f32, v1: &GLVertex, v2: &GLVertex, out: &mut VertexList, pred: &Pred,
) {
    let c1 = pred(get_coord(v1, n), clip_val);
    let c2 = pred(get_coord(v2, n), clip_val);
    if !c1 {
        if !c2 {
            out.push_back(*v2);
        } else {
            let coord1 = get_coord(v1, n);
            let coord2 = get_coord(v2, n);
            let t = (clip_val - coord1) / (coord2 - coord1);
            out.push_back(interpolate(v1, v2, t));
        }
    } else if !c2 {
        let coord1 = get_coord(v1, n);
        let coord2 = get_coord(v2, n);
        let t = (clip_val - coord1) / (coord2 - coord1);
        out.push_back(interpolate(v1, v2, t));
        out.push_back(*v2);
    }
}

/// 裁剪多点（对应 C++ PointClipper::ClipPoints）
fn clip_points<Pred: Fn(f32, f32) -> bool>(n: i32, clip_val: f32, inp: &VertexList, out: &mut VertexList, pred: &Pred) {
    let s = inp.size();
    if s < 2 {
        return;
    }
    clip_point(n, clip_val, &inp[(s - 1) as usize], &inp[0], out, pred);
    for i in 0..(s - 1) as usize {
        clip_point(n, clip_val, &inp[i], &inp[i + 1], out, pred);
    }
}

/// 裁剪多边形（对应 C++ DrawPolyClipped）
fn draw_poly_clipped(clip: &Rect, list: &VertexList) {
    let mut l1 = list.clone();
    let mut l2 = VertexList::new();

    let left = clip.x as f32;
    let top = clip.y as f32;
    let right = (clip.x + clip.width) as f32;
    let bottom = (clip.y + clip.height) as f32;

    let lt_pred = |a: f32, b: f32| a < b;
    let ge_pred = |a: f32, b: f32| a >= b;

    clip_points(0, left, &l1, &mut l2, &lt_pred);
    std::mem::swap(&mut l1, &mut l2);
    l2.clear();

    clip_points(1, top, &l1, &mut l2, &lt_pred);
    std::mem::swap(&mut l1, &mut l2);
    l2.clear();

    clip_points(0, right, &l1, &mut l2, &ge_pred);
    std::mem::swap(&mut l1, &mut l2);
    l2.clear();

    clip_points(1, bottom, &l1, &mut l2, &ge_pred);

    if l2.size() >= 3 {
        unsafe {
            gfx_begin(GL_TRIANGLE_FAN);
            gfx_add_vertices_from_list(&l2);
            gfx_end();
        }
    }
}

/// 裁剪纹理多边形（对应 C++ DoPolyTextureClip）
fn do_poly_texture_clip(list: &mut VertexList) {
    let mut l2 = VertexList::new();

    let lt_pred = |a: f32, b: f32| a < b;
    let ge_pred = |a: f32, b: f32| a >= b;

    clip_points(3, 0.0, list, &mut l2, &lt_pred);
    std::mem::swap(list, &mut l2);
    l2.clear();

    clip_points(4, 0.0, list, &mut l2, &lt_pred);
    std::mem::swap(list, &mut l2);
    l2.clear();

    clip_points(3, 1.0, list, &mut l2, &ge_pred);
    std::mem::swap(list, &mut l2);
    l2.clear();

    clip_points(4, 1.0, list, &mut l2, &ge_pred);
    std::mem::swap(list, &mut l2);
}

// ============================================================
// GLInterface — 核心 OpenGL 渲染管理（对应 C++ GLInterface）
// ============================================================

/// GLInterface — OpenGL 渲染上下文管理
pub struct GLInterface {
    pub app: Option<*mut SexyAppBase>,
    pub width: i32,
    pub height: i32,
    pub display_width: i32,
    pub display_height: i32,
    pub presentation_rect: Rect,
    pub refresh_rate: i32,
    pub milliseconds_per_frame: i32,
    pub screen_image: Option<*mut GLImage>,
    pub next_cursor_x: i32,
    pub next_cursor_y: i32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    // 图像集合（用于生命周期管理）
    pub image_set: Vec<*mut MemoryImage>,
    pub gl_image_set: Vec<*mut GLImage>,
    // 变换栈
    pub transform_stack: Vec<SexyMatrix3>,
}

impl GLInterface {
    /// 构造函数（对应 C++ GLInterface::GLInterface）
    pub fn new(app: Option<*mut SexyAppBase>) -> Self {
        GLInterface {
            app,
            width: 800,
            height: 600,
            display_width: 800,
            display_height: 600,
            presentation_rect: Rect::new(0, 0, 800, 600),
            refresh_rate: 60,
            milliseconds_per_frame: 1000 / 60,
            screen_image: None,
            next_cursor_x: 0,
            next_cursor_y: 0,
            cursor_x: 0,
            cursor_y: 0,
            image_set: Vec::new(),
            gl_image_set: Vec::new(),
            transform_stack: Vec::new(),
        }
    }

    /// 获取屏幕图像（对应 C++ GetScreenImage）
    pub fn get_screen_image(&mut self) -> Option<&mut GLImage> {
        unsafe { self.screen_image.as_mut().map(|p| &mut **p) }
    }

    /// 更新视口（对应 C++ UpdateViewport）
    pub fn update_viewport(&self) {
        // 简化实现：使用 SDL_GL_GetDrawableSize 需要 FFI 绑定
        // 这里只做简单的 4:3 letterbox
        let vw = self.width;
        let vh = self.height;
        let mut vx = 0;
        let mut vy = 0;

        // Letterbox to 4:3
        if self.width as f64 * 3.0 > self.height as f64 * 4.0 {
            let nvw = (self.height as f64 * 4.0 / 3.0) as i32;
            vx = (self.width - nvw) / 2;
        } else if self.width as f64 * 3.0 < self.height as f64 * 4.0 {
            let nvh = (self.width as f64 * 3.0 / 4.0) as i32;
            vy = (self.height - nvh) / 2;
        }

        unsafe {
            glViewport(vx, vy, vw, vh);
        }
        // presentation_rect 在 Init 中设置
    }

    /// 初始化 OpenGL（对应 C++ GLInterface::Init）
    pub fn init(&mut self, _is_windowed: bool) -> i32 {
        let inited = GL_INITED.get_or_init(|| {
            unsafe {
                // 首先初始化 GL 函数指针表（运行时加载）
                if !crate::ffi::opengl::init_gl() {
                    eprintln!("错误：无法加载 OpenGL 函数指针");
                    return false;
                }

                // 编译着色器
                let prog = shader_load(SHADER_CODE);
                if prog == 0 {
                    return false;
                }
                G_PROGRAM = prog;
                G_UF_VIEW_PROJ_MTX = glGetUniformLocation(prog, "u_viewProj\0".as_ptr() as *const GLchar);
                G_UF_TEXTURE = glGetUniformLocation(prog, "u_texture\0".as_ptr() as *const GLchar);
                G_UF_USE_TEXTURE = glGetUniformLocation(prog, "u_useTexture\0".as_ptr() as *const GLchar);
                G_UF_UV_BOUNDS = glGetUniformLocation(prog, "u_uvBounds\0".as_ptr() as *const GLchar);
                G_UF_CLAMP_UV_ENABLED = glGetUniformLocation(prog, "u_clampUvEnabled\0".as_ptr() as *const GLchar);

                glGenBuffers(1, &mut G_VBO as *mut GLuint);
                glBindBuffer(GL_ARRAY_BUFFER, G_VBO);
                glBufferData(
                    GL_ARRAY_BUFFER,
                    (size_of::<GLVertex>() * MAX_VERTICES) as GLsizeiptr,
                    ptr::null(),
                    GL_DYNAMIC_DRAW,
                );

                // 初始化全局状态
                G_VERTICES = Vec::with_capacity(MAX_VERTICES);
                G_NUM_VERTICES = 0;
                G_VERTEX_MODE = GFX_MODE_NONE;
            }
            true
        });

        if !*inited {
            return 0;
        }

        unsafe {
            let mut a_max_size: GLint = 0;
            glGetIntegerv(GL_MAX_TEXTURE_SIZE, &mut a_max_size);
            G_MAX_TEXTURE_SIZE = a_max_size;

            glClearColor(0.0, 0.0, 0.0, 1.0);
            glClear(GL_COLOR_BUFFER_BIT);

            G_MIN_TEXTURE_WIDTH = 8;
            G_MIN_TEXTURE_HEIGHT = 8;
            G_MAX_TEXTURE_WIDTH = a_max_size;
            G_MAX_TEXTURE_HEIGHT = a_max_size;
            G_SUPPORTED_PIXEL_FORMATS =
                (PixelFormat::A8R8G8B8 as i32)
                | (PixelFormat::A4R4G4B4 as i32)
                | (PixelFormat::R5G6B5 as i32)
                | (PixelFormat::Palette8 as i32);
            gfx_set_linear_filter(false);

            glUseProgram(G_PROGRAM);
            let ortho = make_ortho_matrix(0.0, self.width as f32, self.height as f32, 0.0, -10.0, 10.0);
            glUniformMatrix4fv(G_UF_VIEW_PROJ_MTX, 1, GL_FALSE, ortho.as_ptr());
            glUniform1i(G_UF_TEXTURE, 0);
            glUniform1i(G_UF_CLAMP_UV_ENABLED, 1);

            glEnable(GL_BLEND);
            glDisable(GL_DITHER);
            glDisable(GL_DEPTH_TEST);
            glDisable(GL_CULL_FACE);
            glGetError(); // 清除可能的 GL_INVALID_ENUM
        }

        self.set_video_only_draw(false);
        1
    }

    /// 设置绘制模式（对应 C++ SetDrawMode）
    pub fn set_draw_mode(&self, the_draw_mode: i32) {
        unsafe {
            if the_draw_mode == 0 {
                // DRAWMODE_NORMAL
                glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
            } else {
                // DRAWMODE_ADDITIVE
                glBlendFunc(GL_SRC_ALPHA, GL_ONE);
            }
        }
    }

    /// 设置视频专用绘制模式（对应 C++ SetVideoOnlyDraw）
    pub fn set_video_only_draw(&mut self, _video_only: bool) {
        self.screen_image = {
            let mut img = Box::new(GLImage::new(self.width, self.height));
            img.base.base.width = self.width;
            img.base.base.height = self.height;
            Some(Box::into_raw(img))
        };
        if let Some(img) = self.screen_image {
            unsafe {
                (*img).set_image_mode(false, false);
            }
        }
    }

    /// 设置光标位置（对应 C++ SetCursorPos）
    pub fn set_cursor_pos(&mut self, x: i32, y: i32) {
        self.next_cursor_x = x;
        self.next_cursor_y = y;
    }

    /// 添加 GLImage（对应 C++ AddGLImage）
    pub fn add_gl_image(&mut self, image: *mut GLImage) {
        self.gl_image_set.push(image);
    }

    /// 移除 GLImage（对应 C++ RemoveGLImage）
    pub fn remove_gl_image(&mut self, image: *mut GLImage) {
        self.gl_image_set.retain(|&i| i != image);
    }

    /// 移除 3D 数据（对应 C++ Remove3DData）
    pub fn remove_3d_data(&mut self, image: &mut MemoryImage) {
        if image.render_data.is_some() {
            image.render_data = None;
            self.image_set.retain(|&i| i != image as *mut MemoryImage);
        }
    }

    /// 绘制前的准备（对应 C++ PreDraw）
    pub fn pre_draw(&self) -> bool {
        unsafe {
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        }
        true
    }

    /// 刷新绘制命令（对应 C++ Flush）
    pub fn flush(&self) {
        unsafe {
            gfx_set_num_vertices(0);
            // 使用 SDL_GL_SwapWindow 交换缓冲区（由 SexyAppBase 的主循环处理）
        }
    }

    /// 重绘（对应 C++ Redraw）
    pub fn redraw(&mut self, _clip_rect: Option<&Rect>) -> bool {
        self.flush();
        true
    }

    /// 创建图像纹理（对应 C++ CreateImageTexture）
    pub fn create_image_texture(&self, image: &mut MemoryImage) -> bool {
        if image.render_data.is_none() {
            image.render_data = Some(Box::new(TextureData::new()));
            image.purge_bits = false; // 简化：不处理 purge 逻辑
        }

        // 获取 TextureData 的原始指针以绕过借用检查
        let data_ptr: *mut TextureData = image.render_data.as_mut().map(|d| &mut **d as *mut TextureData).unwrap_or(std::ptr::null_mut());
        if data_ptr.is_null() {
            return false;
        }

        unsafe {
            (*data_ptr).check_create_textures(image);
            let pixel_format = (*data_ptr).pixel_format;
            pixel_format != PixelFormat::Unknown
        }
    }

    /// 恢复位图（对应 C++ RecoverBits）
    pub fn recover_bits(&self, image: &mut MemoryImage) -> bool {
        let data = match image.render_data {
            Some(ref d) => d,
            None => return false,
        };
        if data.bits_changed_count != image.bits_changed_count {
            return false;
        }

        unsafe {
            for row in 0..data.tex_vec_height {
                for col in 0..data.tex_vec_width {
                    let piece = &data.textures[(row * data.tex_vec_width + col) as usize];
                    let offx = col * data.tex_piece_width;
                    let offy = row * data.tex_piece_height;
                    let w = (image.base.width - offx).min(piece.width);
                    let h = (image.base.height - offy).min(piece.height);

                    glActiveTexture(GL_TEXTURE0);
                    glBindTexture(GL_TEXTURE_2D, piece.texture);

                    // FBO 回读
                    let mut fbo: GLuint = 0;
                    glGenFramebuffers(1, &mut fbo);
                    glBindFramebuffer(GL_FRAMEBUFFER, fbo);
                    glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, piece.texture, 0);

                    if glCheckFramebufferStatus(GL_FRAMEBUFFER) != GL_FRAMEBUFFER_COMPLETE {
                        glBindFramebuffer(GL_FRAMEBUFFER, 0);
                        glDeleteFramebuffers(1, &fbo);
                        return false;
                    }

                    let mut buf: Vec<u32> = vec![0u32; (w * h) as usize];
                    glReadPixels(0, 0, w, h, GL_RGBA, GL_UNSIGNED_BYTE, buf.as_mut_ptr() as *mut std::ffi::c_void);

                    glBindFramebuffer(GL_FRAMEBUFFER, 0);
                    glDeleteFramebuffers(1, &fbo);

                    // 复制回 bits（转换为 RGBA 字节）
                    for r in 0..h {
                        for c in 0..w {
                            let src_idx = (r * w + c) as usize;
                            if src_idx < buf.len() {
                                let rgba = buf[src_idx];
                                let dst_base = ((offy + r) * image.base.width + (offx + c)) as usize * 4;
                                if dst_base + 3 < image.base.pixels.len() {
                                    image.base.pixels[dst_base] = ((rgba >> 24) & 0xFF) as u8;     // R
                                    image.base.pixels[dst_base + 1] = ((rgba >> 16) & 0xFF) as u8;  // G
                                    image.base.pixels[dst_base + 2] = ((rgba >> 8) & 0xFF) as u8;   // B
                                    image.base.pixels[dst_base + 3] = (rgba & 0xFF) as u8;         // A
                                }
                            }
                        }
                    }
                }
            }
        }
        true
    }

    /// 推入变换（对应 C++ PushTransform）
    pub fn push_transform(&mut self, transform: &SexyMatrix3, concatenate: bool) {
        if self.transform_stack.is_empty() || !concatenate {
            self.transform_stack.push(*transform);
        } else {
            let combined = transform.multiply(self.transform_stack.last().unwrap());
            self.transform_stack.push(combined);
        }
    }

    /// 弹出变换（对应 C++ PopTransform）
    pub fn pop_transform(&mut self) {
        self.transform_stack.pop();
    }

    /// --- 绘制方法 ---

    /// Blt — 基本图像绘制（对应 C++ GLInterface::Blt）
    pub fn blt(
        &mut self,
        image: &mut Image,
        the_x: f32,
        the_y: f32,
        src_rect: &Rect,
        the_color: &Color,
        the_draw_mode: i32,
        linear_filter: bool,
    ) {
        if !self.transform_stack.is_empty() {
            self.blt_clip_f(image, the_x, the_y, src_rect, None, the_color, the_draw_mode, linear_filter);
            return;
        }
        if !self.pre_draw() { return; }

        let mem = match image.as_any_mut().downcast_mut::<MemoryImage>() {
            Some(m) => m,
            None => return,
        };
        if !self.create_image_texture(mem) { return; }

        self.set_draw_mode(the_draw_mode);
        unsafe { set_linear_filter(linear_filter); }

        if let Some(ref data) = mem.render_data {
            data.blt(the_x, the_y, src_rect, the_color);
        }
    }

    /// BltClipF — 带裁切的 Blt（对应 C++ BltClipF）
    pub fn blt_clip_f(
        &mut self,
        image: &mut Image,
        the_x: f32,
        the_y: f32,
        src_rect: &Rect,
        clip_rect: Option<&Rect>,
        the_color: &Color,
        the_draw_mode: i32,
        linear_filter: bool,
    ) {
        let t = SexyMatrix3::translation(the_x, the_y);
        self.blt_transformed(image, clip_rect, the_color, the_draw_mode, src_rect, &t, linear_filter, 0.0, 0.0, false);
    }

    /// BltMirror — 镜像绘制（对应 C++ BltMirror）
    pub fn blt_mirror(
        &mut self,
        image: &mut Image,
        the_x: f32,
        the_y: f32,
        src_rect: &Rect,
        the_color: &Color,
        the_draw_mode: i32,
        linear_filter: bool,
    ) {
        let mut t = SexyMatrix3::translation(-src_rect.width as f32, 0.0);
        let s = SexyMatrix3::scaling(-1.0, 1.0);
        t = s.multiply(&t);
        let trans = SexyMatrix3::translation(the_x, the_y);
        t = trans.multiply(&t);
        self.blt_transformed(image, None, the_color, the_draw_mode, src_rect, &t, linear_filter, 0.0, 0.0, false);
    }

    /// StretchBlt — 拉伸绘制（对应 C++ StretchBlt）
    pub fn stretch_blt(
        &mut self,
        image: &mut Image,
        dest_rect: &Rect,
        src_rect: &Rect,
        clip_rect: Option<&Rect>,
        the_color: &Color,
        the_draw_mode: i32,
        fast_stretch: bool,
        mirror: bool,
    ) {
        let xs = dest_rect.width as f32 / src_rect.width as f32;
        let ys = dest_rect.height as f32 / src_rect.height as f32;

        let t = if mirror {
            let mut t = SexyMatrix3::translation(-src_rect.width as f32, 0.0);
            let s = SexyMatrix3::scaling(-xs, ys);
            t = s.multiply(&t);
            SexyMatrix3::translation(dest_rect.x as f32, dest_rect.y as f32).multiply(&t)
        } else {
            let s = SexyMatrix3::scaling(xs, ys);
            SexyMatrix3::translation(dest_rect.x as f32, dest_rect.y as f32).multiply(&s)
        };
        self.blt_transformed(image, clip_rect, the_color, the_draw_mode, src_rect, &t, !fast_stretch, 0.0, 0.0, false);
    }

    /// BltRotated — 旋转绘制（对应 C++ BltRotated）
    pub fn blt_rotated(
        &mut self,
        image: &mut Image,
        the_x: f32,
        the_y: f32,
        clip_rect: Option<&Rect>,
        the_color: &Color,
        the_draw_mode: i32,
        the_rot: f64,
        rot_center_x: f32,
        rot_center_y: f32,
        src_rect: &Rect,
    ) {
        let mut t = SexyMatrix3::translation(-rot_center_x, -rot_center_y);
        let r = SexyMatrix3::rotation(the_rot as f32);
        t = r.multiply(&t);
        t = SexyMatrix3::translation(the_x + rot_center_x, the_y + rot_center_y).multiply(&t);
        self.blt_transformed(image, clip_rect, the_color, the_draw_mode, src_rect, &t, true, 0.0, 0.0, false);
    }

    /// BltTransformed — 变换后绘制（对应 C++ GLInterface::BltTransformed）
    pub fn blt_transformed(
        &mut self,
        image: &mut Image,
        clip_rect: Option<&Rect>,
        the_color: &Color,
        the_draw_mode: i32,
        src_rect: &Rect,
        the_transform: &SexyMatrix3,
        linear_filter: bool,
        the_x: f32,
        the_y: f32,
        center: bool,
    ) {
        if !self.pre_draw() { return; }

        let mem = match image.as_any_mut().downcast_mut::<MemoryImage>() {
            Some(m) => m,
            None => return,
        };
        if !self.create_image_texture(mem) { return; }

        self.set_draw_mode(the_draw_mode);
        unsafe { set_linear_filter(linear_filter); }

        let data_ptr: *mut TextureData = match mem.render_data {
            Some(ref mut d) => &mut **d as *mut TextureData,
            None => return,
        };

        unsafe {
            let data = &mut *data_ptr;
            if !self.transform_stack.is_empty() {
                let stack_top = &self.transform_stack[self.transform_stack.len() - 1];
                if the_x != 0.0 || the_y != 0.0 {
                    let mut t = SexyMatrix3::identity();
                    if center {
                        t = SexyMatrix3::translation(-src_rect.width as f32 / 2.0, -src_rect.height as f32 / 2.0);
                    }
                    t = the_transform.multiply(&t);
                    t = SexyMatrix3::translation(the_x, the_y).multiply(&t);
                    t = stack_top.multiply(&t);
                    data.blt_transformed(&t, src_rect, the_color, clip_rect, 0.0, 0.0, false);
                } else {
                    let t = stack_top.multiply(the_transform);
                    data.blt_transformed(&t, src_rect, the_color, clip_rect, the_x, the_y, center);
                }
            } else {
                data.blt_transformed(the_transform, src_rect, the_color, clip_rect, the_x, the_y, center);
            }
        }
    }

    /// DrawLine — 绘制线段（对应 C++ DrawLine）
    pub fn draw_line(
        &mut self,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        the_color: &Color,
        the_draw_mode: i32,
    ) {
        if !self.pre_draw() { return; }
        self.set_draw_mode(the_draw_mode);

        let (mut fx1, mut fy1, mut fx2, mut fy2) = (start_x as f32, start_y as f32, end_x as f32, end_y as f32);

        if !self.transform_stack.is_empty() {
            let stack_top = &self.transform_stack[self.transform_stack.len() - 1];
            let p1 = stack_top.transform(fx1, fy1);
            let p2 = stack_top.transform(fx2, fy2);
            fx1 = p1.0; fy1 = p1.1;
            fx2 = p2.0; fy2 = p2.1;
        }

        let c = color_to_gl_color(the_color);
        let vtx: [GLVertex; 3] = [
            GLVertex { sx: fx1, sy: fy1, sz: 0.0, color: c, tu: 0.0, tv: 0.0 },
            GLVertex { sx: fx2, sy: fy2, sz: 0.0, color: c, tu: 0.0, tv: 0.0 },
            GLVertex { sx: fx2 + 0.5, sy: fy2 + 0.5, sz: 0.0, color: c, tu: 0.0, tv: 0.0 },
        ];

        unsafe {
            glUniform1i(G_UF_USE_TEXTURE, 0);
            gfx_begin(GL_LINE_STRIP);
            gfx_add_vertices(vtx.as_ptr(), 3);
            gfx_end();
        }
    }

    /// FillRect — 填充矩形（对应 C++ GLInterface::FillRect）
    pub fn fill_rect(&mut self, the_rect: &Rect, the_color: &Color, the_draw_mode: i32) {
        if !self.pre_draw() { return; }
        self.set_draw_mode(the_draw_mode);

        let x = the_rect.x as f32;
        let y = the_rect.y as f32;
        let w = the_rect.width as f32;
        let h = the_rect.height as f32;
        let c = color_to_gl_color(the_color);

        let mut vtx: [GLVertex; 4] = [
            GLVertex { sx: x, sy: y, sz: 0.0, color: c, tu: 0.0, tv: 0.0 },
            GLVertex { sx: x, sy: y + h, sz: 0.0, color: c, tu: 0.0, tv: 0.0 },
            GLVertex { sx: x + w, sy: y, sz: 0.0, color: c, tu: 0.0, tv: 0.0 },
            GLVertex { sx: x + w, sy: y + h, sz: 0.0, color: c, tu: 0.0, tv: 0.0 },
        ];

        if !self.transform_stack.is_empty() {
            let stack_top = &self.transform_stack[self.transform_stack.len() - 1];
            let pts = [(x, y), (x, y + h), (x + w, y), (x + w, y + h)];
            for i in 0..4 {
                let p = stack_top.transform(pts[i].0, pts[i].1);
                vtx[i].sx = p.0;
                vtx[i].sy = p.1;
            }
        }

        unsafe {
            glUniform1i(G_UF_USE_TEXTURE, 0);
            gfx_begin(GL_TRIANGLE_STRIP);
            gfx_add_vertices(vtx.as_ptr(), 4);
            gfx_end();
        }
    }

    /// DrawTriangle — 绘制三角形（对应 C++ DrawTriangle）
    pub fn draw_triangle(
        &mut self,
        p1: &TriVertex,
        p2: &TriVertex,
        p3: &TriVertex,
        the_color: &Color,
        the_draw_mode: i32,
    ) {
        if !self.pre_draw() { return; }
        self.set_draw_mode(the_draw_mode);

        let c = color_to_gl_color(the_color);
        let vtx: [GLVertex; 3] = [
            GLVertex { sx: p1.x, sy: p1.y, sz: 0.0, color: vertex_color(p1.color, c), tu: 0.0, tv: 0.0 },
            GLVertex { sx: p2.x, sy: p2.y, sz: 0.0, color: vertex_color(p2.color, c), tu: 0.0, tv: 0.0 },
            GLVertex { sx: p3.x, sy: p3.y, sz: 0.0, color: vertex_color(p3.color, c), tu: 0.0, tv: 0.0 },
        ];

        unsafe {
            glUniform1i(G_UF_USE_TEXTURE, 0);
            gfx_begin(GL_TRIANGLE_STRIP);
            gfx_add_vertices(vtx.as_ptr(), 3);
            gfx_end();
        }
    }

    /// DrawTriangleTex — 带纹理的三角形绘制（对应 C++ DrawTriangleTex）
    pub fn draw_triangle_tex(
        &mut self,
        p1: &TriVertex,
        p2: &TriVertex,
        p3: &TriVertex,
        the_color: &Color,
        the_draw_mode: i32,
        texture: &mut Image,
        blend: bool,
    ) {
        let arr: [[TriVertex; 3]; 1] = [[*p1, *p2, *p3]];
        self.draw_triangles_tex(&arr, 1, the_color, the_draw_mode, texture, 0.0, 0.0, blend);
    }

    /// DrawTrianglesTex — 批量纹理三角形绘制（对应 C++ DrawTrianglesTex）
    pub fn draw_triangles_tex(
        &mut self,
        vertices: &[[TriVertex; 3]],
        num_triangles: i32,
        the_color: &Color,
        the_draw_mode: i32,
        texture: &mut Image,
        tx: f32,
        ty: f32,
        blend: bool,
    ) {
        if !self.pre_draw() { return; }

        let mem = match texture.as_any_mut().downcast_mut::<MemoryImage>() {
            Some(m) => m,
            None => return,
        };
        if !self.create_image_texture(mem) { return; }

        self.set_draw_mode(the_draw_mode);
        unsafe { set_linear_filter(blend); }

        let c = color_to_gl_color(the_color);
        let clamp_uv = (mem.render_flags & RENDER_IMAGE_FLAG_REPEAT) == 0;

        if let Some(ref data) = mem.render_data {
            data.blt_triangles(vertices, num_triangles, c, tx, ty, clamp_uv);
        }
    }

    /// DrawTrianglesTexStrip — 条带纹理三角形（对应 C++ DrawTrianglesTexStrip）
    pub fn draw_triangles_tex_strip(
        &mut self,
        vertices: &[TriVertex],
        num_triangles: i32,
        the_color: &Color,
        the_draw_mode: i32,
        texture: &mut Image,
        tx: f32,
        ty: f32,
        blend: bool,
    ) {
        let mut done = 0i32;
        while done < num_triangles {
            let n = (100).min(num_triangles - done);
            let mut batch = [[TriVertex { x: 0.0, y: 0.0, u: 0.0, v: 0.0, color: 0 }; 3]; 100];
            for i in 0..n as usize {
                let idx = done as usize + i;
                batch[i][0] = vertices[idx];
                batch[i][1] = vertices[idx + 1];
                batch[i][2] = vertices[idx + 2];
            }
            self.draw_triangles_tex(&batch[..n as usize], n, the_color, the_draw_mode, texture, tx, ty, blend);
            done += n;
        }
    }

    /// FillPoly — 填充多边形（对应 C++ FillPoly）
    pub fn fill_poly(
        &mut self,
        vertices: &[(i32, i32)],
        clip_rect: Option<&Rect>,
        the_color: &Color,
        the_draw_mode: i32,
        tx: i32,
        ty: i32,
    ) {
        if vertices.len() < 3 { return; }
        if !self.pre_draw() { return; }
        self.set_draw_mode(the_draw_mode);

        let c = color_to_gl_color(the_color);
        unsafe {
            glUniform1i(G_UF_USE_TEXTURE, 0);
        }

        let mut vl = VertexList::new();
        for &(vx, vy) in vertices {
            let mut glv = GLVertex {
                sx: vx as f32 + tx as f32,
                sy: vy as f32 + ty as f32,
                sz: 0.0,
                color: c,
                tu: 0.0,
                tv: 0.0,
            };
            if !self.transform_stack.is_empty() {
                let stack_top = &self.transform_stack[self.transform_stack.len() - 1];
                let p = stack_top.transform(glv.sx, glv.sy);
                glv.sx = p.0;
                glv.sy = p.1;
            }
            vl.push_back(glv);
        }

        if let Some(cr) = clip_rect {
            draw_poly_clipped(cr, &vl);
        } else {
            unsafe {
                gfx_begin(GL_TRIANGLE_FAN);
                gfx_add_vertices_from_list(&vl);
                gfx_end();
            }
        }
    }
}

impl Drop for GLInterface {
    fn drop(&mut self) {
        // 清理所有 TextureData
        for img_ptr in &self.image_set {
            unsafe {
                if let Some(img) = img_ptr.as_mut() {
                    img.render_data = None;
                }
            }
        }
    }
}

