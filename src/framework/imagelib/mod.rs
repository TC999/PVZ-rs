// PvZ Portable Rust 翻译 — ImageLib 图像加载库
// 对应 C++ SexyAppFramework/imagelib/ImageLib.h / ImageLib.cpp
//
// 使用 SDL2_image (IMG_Load) 加载常见图像格式，转换为像素缓冲区。

#![allow(dead_code)]

use std::ffi::CString;
use crate::ffi::sdl2::*;

/// 加载的图像信息（对应 C++ ImageLib::Image）
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub bits: Vec<u32>,
}

impl Image {
    pub fn new() -> Self { Image { width: 0, height: 0, bits: Vec::new() } }
    pub fn get_width(&self) -> i32 { self.width }
    pub fn get_height(&self) -> i32 { self.height }
    pub fn get_bits(&self) -> &[u32] { &self.bits }
}

/// 全局标志
pub static mut G_ALPHA_COMPOSE_COLOR: i32 = 0;
pub static mut G_AUTO_LOAD_ALPHA: bool = true;
pub static mut G_IGNORE_JPEG2000_ALPHA: bool = true;

/// 加载图像文件（对应 C++ GetImage）
pub fn get_image(file_name: &str, _look_for_alpha: bool) -> Option<Box<Image>> {
    let c_path = CString::new(file_name).ok()?;
    unsafe {
        let surface = IMG_Load(c_path.as_ptr());
        if surface.is_null() { return None; }

        let w = (*surface).w;
        let h = (*surface).h;
        let pitch = (*surface).pitch;
        let pixels = (*surface).pixels;

        if pixels.is_null() {
            SDL_FreeSurface(surface);
            return None;
        }

        // 将 SDL_Surface 像素转为 RGBA u32 数组
        let bpp = pitch / w;
        let total = (w * h) as usize;
        let mut bits = vec![0u32; total];

        let src = pixels as *const u8;
        for i in 0..total {
            let offset = (i as i32 / w) * pitch + (i as i32 % w) * bpp;
            let (r, g, b, a) = match bpp {
                4 => (
                    *src.add(offset as usize),
                    *src.add(offset as usize + 1),
                    *src.add(offset as usize + 2),
                    *src.add(offset as usize + 3),
                ),
                3 => (
                    *src.add(offset as usize),
                    *src.add(offset as usize + 1),
                    *src.add(offset as usize + 2),
                    255,
                ),
                _ => (0, 0, 0, 255),
            };
            bits[i] = (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32;
        }

        SDL_FreeSurface(surface);

        Some(Box::new(Image { width: w, height: h, bits }))
    }
}

/// 写入 JPEG 图像（对应 C++ WriteJPEGImage）
pub fn write_jpeg_image(_file_name: &str, _image: &Image) -> bool { false }

/// 写入 PNG 图像（对应 C++ WritePNGImage）
pub fn write_png_image(_file_name: &str, _image: &Image) -> bool { false }

/// 写入 TGA 图像（对应 C++ WriteTGAImage）
pub fn write_tga_image(_file_name: &str, _image: &Image) -> bool { false }
