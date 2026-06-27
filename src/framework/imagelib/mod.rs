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

/// 全局标志（对应 C++ gAlphaComposeColor = 0xFFFFFF）
pub static mut G_ALPHA_COMPOSE_COLOR: u32 = 0x00FFFFFF;
pub static mut G_AUTO_LOAD_ALPHA: bool = true;
pub static mut G_IGNORE_JPEG2000_ALPHA: bool = true;

/// 加载图像文件（对应 C++ GetImage）
/// 支持 Alpha 通道自动查找和合成：
/// - 查找 `_filename.ext`（前缀下划线）
/// - 查找 `filename_.ext`（后缀下划线）
pub fn get_image(file_name: &str, look_for_alpha: bool) -> Option<Box<Image>> {
    // 对应 C++ 的 gAutoLoadAlpha 检查
    let mut look_for_alpha = look_for_alpha;
    // SAFETY: 单线程环境下读取全局标志
    if unsafe { !G_AUTO_LOAD_ALPHA } {
        look_for_alpha = false;
    }

    // 加载主图像
    let main_image = load_image_file(file_name);

    // 尝试查找并合成 Alpha 通道
    let alpha_image = if look_for_alpha {
        find_alpha_image(file_name)
    } else {
        None
    };

    match (main_image, alpha_image) {
        (Some(mut img), Some(alpha)) => {
            compose_alpha(&mut img, &alpha);
            Some(img)
        }
        (None, Some(mut alpha)) => {
            // SAFETY: 单线程环境下读取全局标志
            let base_color = unsafe { G_ALPHA_COMPOSE_COLOR };
            apply_alpha_as_image(&mut alpha, base_color);
            Some(alpha)
        }
        (img, None) => img,
    }
}

/// 从文件加载原始图像（内部函数）
fn load_image_file(file_name: &str) -> Option<Box<Image>> {
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
            // 存储为 ARGB (0xAARRGGBB)
            bits[idx] = (px[3] as u32) << 24 | (px[0] as u32) << 16 | (px[1] as u32) << 8 | px[2] as u32;
        }
    }

    Some(Box::new(Image { width: w, height: h, bits }))
}

/// 查找 Alpha 通道图像（对应 C++ 的 alpha 查找逻辑）
/// 搜索 `_filename.ext` 和 `filename_.ext` 两种模式
fn find_alpha_image(file_name: &str) -> Option<Box<Image>> {
    // 分离路径和文件名
    let last_slash = file_name.rfind('/');
    let last_dot = file_name.rfind('.');

    // 确定扩展名位置
    let has_ext = last_dot.is_some() && (last_slash.is_none() || last_dot.unwrap() > last_slash.unwrap());

    // 模式1：_filename.ext（前缀下划线，在目录名后插入）
    let slash_end = match last_slash {
        Some(pos) => pos + 1,
        None => 0,
    };
    let alpha_path1 = format!(
        "{}_{}",
        &file_name[..slash_end],
        &file_name[slash_end..]
    );
    if std::path::Path::new(&alpha_path1).exists() {
        if let Some(img) = load_image_file(&alpha_path1) {
            return Some(img);
        }
    }

    // 模式2：filename_.ext（后缀下划线，在扩展名前插入）
    if has_ext {
        let dot_pos = last_dot.unwrap();
        let alpha_path2 = format!("{}_{}", &file_name[..dot_pos], &file_name[dot_pos..]);
        if std::path::Path::new(&alpha_path2).exists() {
            if let Some(img) = load_image_file(&alpha_path2) {
                return Some(img);
            }
        }
    }

    None
}

/// 合成 Alpha 通道（对应 C++ ComposeAlpha）
/// 将 alpha 图像的蓝色通道值写入主图像的 Alpha 通道
fn compose_alpha(image: &mut Image, alpha_image: &Image) {
    if image.width != alpha_image.width || image.height != alpha_image.height {
        return;
    }
    let size = (image.width * image.height) as usize;
    for i in 0..size {
        // 取 alpha 图像的低字节（B 通道，灰度图中 R=G=B）作为 Alpha
        let alpha_byte = alpha_image.bits[i] & 0xFF;
        image.bits[i] = (image.bits[i] & 0x00FFFFFF) | (alpha_byte << 24);
    }
}

/// 将图像作为 Alpha 通道应用到纯色上（对应 C++ ApplyAlphaAsImage）
fn apply_alpha_as_image(image: &mut Image, base_color: u32) {
    let size = (image.width * image.height) as usize;
    for i in 0..size {
        let alpha_byte = image.bits[i] & 0xFF;
        image.bits[i] = base_color | (alpha_byte << 24);
    }
}

/// 将 ARGB 像素转换为 RGBA 字节序列（用于 image crate 编码）
fn argb_to_rgba_bytes(bits: &[u32]) -> Vec<u8> {
    let mut rgba = vec![0u8; bits.len() * 4];
    for (i, pixel) in bits.iter().enumerate() {
        let a = (pixel >> 24) & 0xFF;
        let r = (pixel >> 16) & 0xFF;
        let g = (pixel >> 8) & 0xFF;
        let b = *pixel & 0xFF;
        rgba[i * 4] = r as u8;
        rgba[i * 4 + 1] = g as u8;
        rgba[i * 4 + 2] = b as u8;
        rgba[i * 4 + 3] = a as u8;
    }
    rgba
}

/// 将 ARGB 像素转换为 RGB 字节序列（用于 JPEG 编码，丢弃 Alpha）
fn argb_to_rgb_bytes(bits: &[u32]) -> Vec<u8> {
    let mut rgb = vec![0u8; bits.len() * 3];
    for (i, pixel) in bits.iter().enumerate() {
        let r = (pixel >> 16) & 0xFF;
        let g = (pixel >> 8) & 0xFF;
        let b = *pixel & 0xFF;
        rgb[i * 3] = r as u8;
        rgb[i * 3 + 1] = g as u8;
        rgb[i * 3 + 2] = b as u8;
    }
    rgb
}

/// 写入 JPEG 图像（对应 C++ WriteJPEGImage）
pub fn write_jpeg_image(file_name: &str, image: &Image) -> bool {
    let w = image.width as u32;
    let h = image.height as u32;
    if w == 0 || h == 0 {
        return false;
    }
    let rgb = argb_to_rgb_bytes(&image.bits);
    image::save_buffer(file_name, &rgb, w, h, image::ColorType::Rgb8).is_ok()
}

/// 写入 PNG 图像（对应 C++ WritePNGImage）
pub fn write_png_image(file_name: &str, image: &Image) -> bool {
    let w = image.width as u32;
    let h = image.height as u32;
    if w == 0 || h == 0 {
        return false;
    }
    let rgba = argb_to_rgba_bytes(&image.bits);
    match image::RgbaImage::from_raw(w, h, rgba) {
        Some(img) => img.save(file_name).is_ok(),
        None => false,
    }
}

/// 写入 TGA 图像（对应 C++ WriteTGAImage）
pub fn write_tga_image(file_name: &str, image: &Image) -> bool {
    let w = image.width as u32;
    let h = image.height as u32;
    if w == 0 || h == 0 {
        return false;
    }
    let rgba = argb_to_rgba_bytes(&image.bits);
    match image::RgbaImage::from_raw(w, h, rgba) {
        Some(img) => img.save(file_name).is_ok(),
        None => false,
    }
}
