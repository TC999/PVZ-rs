// PvZ Portable Rust 翻译 — GLInterface（OpenGL 渲染接口）
// 对应 C++ SexyAppFramework/graphics/GLInterface.h / GLInterface.cpp

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::rect::Rect;
use crate::framework::graphics::image::Image;
use crate::framework::graphics::gl_image::GLImage;
use crate::framework::graphics::memory_image::MemoryImage;
use crate::framework::sexy_matrix::SexyMatrix3;
use crate::framework::sexy_app_base::SexyAppBase;

/// 三角顶点（对应 TodLib 中的 TriVertex）
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TriVertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub color: u32,
}

/// OpenGL 顶点（用于渲染）
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

/// GLInterface — 核心 OpenGL 渲染管理
pub struct GLInterface {
    pub app: Option<*mut SexyAppBase>,
    pub width: i32,
    pub height: i32,
    pub display_width: i32,
    pub display_height: i32,
    pub presentation_rect: Rect,
    pub refresh_rate: i32,
    pub screen_image: Option<*mut GLImage>,
    pub next_cursor_x: i32,
    pub next_cursor_y: i32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub image_set: Vec<*mut MemoryImage>,
    pub gl_image_set: Vec<*mut GLImage>,
}

impl GLInterface {
    pub fn new(app: Option<*mut SexyAppBase>) -> Self {
        GLInterface {
            app,
            width: 800,
            height: 600,
            display_width: 800,
            display_height: 600,
            presentation_rect: Rect::new(0, 0, 800, 600),
            refresh_rate: 60,
            screen_image: None,
            next_cursor_x: 0,
            next_cursor_y: 0,
            cursor_x: 0,
            cursor_y: 0,
            image_set: Vec::new(),
            gl_image_set: Vec::new(),
        }
    }

    /// 初始化 OpenGL
    pub fn init(&mut self, _is_windowed: bool) -> i32 {
        // glClear/glViewport 通过 opengl32.dll 的 raw-dylib 链接自动解析
        // 无需显式加载 glad
        1
    }

    /// 更新视口
    pub fn update_viewport(&self) {
        unsafe {
            crate::ffi::opengl::glViewport(0, 0, self.width, self.height);
        }
    }

    /// 设置光标位置
    pub fn set_cursor_pos(&mut self, x: i32, y: i32) {
        self.next_cursor_x = x;
        self.next_cursor_y = y;
    }

    /// 绘制前的准备
    pub fn pre_draw(&self) -> bool {
        unsafe {
            crate::ffi::opengl::glClear(crate::ffi::opengl::GL_COLOR_BUFFER_BIT);
        }
        true
    }

    /// 刷新绘制命令
    pub fn flush(&self) {
        // 刷新缓冲
    }

    /// 绘制图像
    pub fn blt(&self, _image: &Image, _x: f32, _y: f32, _src_rect: &Rect, _color: &Color, _draw_mode: i32, _linear_filter: bool) {
        // 对应原 C++ GLInterface::Blt
    }

    /// 绘制旋转图像
    pub fn blt_rotated(&self, _image: &Image, _x: f32, _y: f32, _clip: &Rect, _color: &Color, _draw_mode: i32, _rot: f64, _rot_cx: f32, _rot_cy: f32, _src_rect: &Rect) {
        // 对应原 C++ GLInterface::BltRotated
    }

    /// 拉伸绘制
    pub fn stretch_blt(&self, _image: &Image, _dest_rect: &Rect, _src_rect: &Rect, _clip_rect: &Rect, _color: &Color, _draw_mode: i32, _fast_stretch: bool, _mirror: bool) {
        // 对应原 C++ GLInterface::StretchBlt
    }

    /// 填充矩形
    pub fn fill_rect(&self, _rect: &Rect, _color: &Color, _draw_mode: i32) {
        // 对应原 C++ GLInterface::FillRect
    }

    /// 绘制线段
    pub fn draw_line(&self, _sx: f64, _sy: f64, _ex: f64, _ey: f64, _color: &Color, _draw_mode: i32) {
        // 对应原 C++ GLInterface::DrawLine
    }

    /// 变换后绘制
    pub fn blt_transformed(&self, _image: &Image, _clip: &Rect, _color: &Color, _draw_mode: i32, _src_rect: &Rect, _transform: &SexyMatrix3, _linear_filter: bool, _x: f32, _y: f32, _center: bool) {
        // 对应原 C++ GLInterface::BltTransformed
    }

    /// 绘制三角形
    pub fn draw_triangle(&self, _p1: &TriVertex, _p2: &TriVertex, _p3: &TriVertex, _color: &Color, _draw_mode: i32) {
        // 对应原 C++ GLInterface::DrawTriangle
    }

    /// 添加 GLImage
    pub fn add_gl_image(&mut self, _image: &GLImage) {
        // 管理 GLImage 生命周期
    }

    /// 移除 GLImage
    pub fn remove_gl_image(&mut self, _image: &GLImage) {
        // 管理 GLImage 生命周期
    }

    /// 创建图像纹理
    pub fn create_image_texture(&self, _image: &MemoryImage) -> bool {
        true
    }

    /// 获取屏幕图像
    pub fn get_screen_image(&mut self) -> Option<&mut GLImage> {
        unsafe { self.screen_image.as_mut().map(|p| &mut **p) }
    }
}
