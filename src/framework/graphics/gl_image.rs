// PvZ Portable Rust 翻译 — GLImage（OpenGL 纹理图像）
// 对应 C++ SexyAppFramework/graphics/GLImage.h / GLImage.cpp

#![allow(dead_code)]

use crate::framework::graphics::image::Image;
use crate::ffi::opengl::GLuint;

/// OpenGL 纹理图像
#[derive(Debug)]
pub struct GLImage {
    pub base: Image,
    /// 纹理数据块
    pub textures: Vec<TextureDataPiece>,
    /// 纹理宽度
    pub tex_width: i32,
    /// 纹理高度
    pub tex_height: i32,
    /// 像素格式
    pub pixel_format: PixelFormat,
    /// 内存图像来源
    pub memory_image: Option<*mut MemoryImage>,
    /// OpenGL 纹理句柄（如果只有一个纹理片）
    pub tex_id: GLuint,
}

#[derive(Debug, Clone)]
pub struct TextureDataPiece {
    pub texture: GLuint,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PixelFormat {
    Unknown = 0x0000,
    A8R8G8B8 = 0x0001,
    A4R4G4B4 = 0x0002,
    R5G6B5 = 0x0004,
    Palette8 = 0x0008,
}

impl GLImage {
    pub fn new(width: i32, height: i32) -> Self {
        GLImage {
            base: Image::new(width, height),
            textures: Vec::new(),
            tex_width: 0,
            tex_height: 0,
            pixel_format: PixelFormat::Unknown,
            memory_image: None,
            tex_id: 0,
        }
    }

    pub fn get_tex_id(&self) -> GLuint {
        if !self.textures.is_empty() {
            self.textures[0].texture
        } else {
            self.tex_id
        }
    }
}

// 前向声明（用于 GLImage 引用 MemoryImage）
use crate::framework::graphics::memory_image::MemoryImage;
