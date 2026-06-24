// PvZ Portable Rust 翻译 — ImageLib 图像加载库
// 对应 C++ SexyAppFramework/imagelib/ImageLib.h / ImageLib.cpp
//
// 使用纯 Rust image crate 加载常见图像格式，转换为像素缓冲区。
// 无 SDL2_image 依赖，跨平台兼容。

#![allow(dead_code)]

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
    // 使用纯 Rust image crate 加载
    let img = image::ImageReader::open(file_name).ok()?.decode().ok()?;
    let rgba = img.to_rgba8();

    let w = rgba.width() as i32;
    let h = rgba.height() as i32;
    let total = (w * h) as usize;
    let mut bits = vec![0u32; total];

    for y in 0..h {
        for x in 0..w {
            let px = rgba.get_pixel(x as u32, y as u32);
            let idx = (y * w + x) as usize;
            bits[idx] = (px[3] as u32) << 24 | (px[0] as u32) << 16 | (px[1] as u32) << 8 | px[2] as u32;
        }
    }

    Some(Box::new(Image { width: w, height: h, bits }))
}

/// 写入 JPEG 图像（对应 C++ WriteJPEGImage）
pub fn write_jpeg_image(_file_name: &str, _image: &Image) -> bool { false }

/// 写入 PNG 图像（对应 C++ WritePNGImage）
pub fn write_png_image(_file_name: &str, _image: &Image) -> bool { false }

/// 写入 TGA 图像（对应 C++ WriteTGAImage）
pub fn write_tga_image(_file_name: &str, _image: &Image) -> bool { false }
