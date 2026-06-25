// PvZ Portable Rust 翻译 — PoolEffect（水池波光效果）
// 对应 C++ src/Lawn/System/PoolEffect.cpp / PoolEffect.h
//
// 管理水池表面的波光粼粼视觉效果，包含波纹动画与纹理渲染。

use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::memory_image::MemoryImage;

const CAUSTIC_IMAGE_WIDTH: i32 = 128;
const CAUSTIC_IMAGE_HEIGHT: i32 = 64;

/// 水池波光效果（对应 C++ PoolEffect）
pub struct PoolEffect {
    /// 灰度波光图像数据（256x256）
    pub grayscale_image: Vec<u8>,
    /// 最终渲染用的波光纹理
    pub caustic_image: Option<Box<MemoryImage>>,
    /// 计数器，驱动动画
    pub pool_counter: u32,
}

impl PoolEffect {
    pub fn new() -> Self {
        PoolEffect {
            grayscale_image: Vec::new(),
            caustic_image: None,
            pool_counter: 0,
        }
    }

    /// 初始化波光效果（对应 C++ PoolEffectInitialize）
    pub fn initialize(&mut self) {
        self.pool_counter = 0;

        // 创建波光纹理 MemoryImage
        let mut caustic_image = Box::new(MemoryImage::new(CAUSTIC_IMAGE_WIDTH, CAUSTIC_IMAGE_HEIGHT));
        caustic_image.has_trans = true;
        caustic_image.has_alpha = true;
        caustic_image.render_flags |= 0x0010; // RenderImageFlag_Repeat
        // 初始化像素为全白不透明
        caustic_image.base.pixels.fill(0xFF);

        // 初始化灰度图像（256x256 全灰）
        self.grayscale_image = vec![128u8; 256 * 256];

        self.caustic_image = Some(caustic_image);
    }

    /// 双线性插值查找（对应 C++ BilinearLookupFixedPoint）
    fn bilinear_lookup_fixed_point(&self, u: u32, v: u32) -> u32 {
        let time_u = u & 0xFFFF0000;
        let time_v = v & 0xFFFF0000;
        let factor_u1 = ((u - time_u) & 0x0000FFFE) + 1;
        let factor_v1 = ((v - time_v) & 0x0000FFFE) + 1;
        let factor_u0 = 65536 - factor_u1;
        let factor_v0 = 65536 - factor_v1;
        let index_u0 = (time_u >> 16) % 256;
        let index_u1 = ((time_u >> 16) + 1) % 256;
        let index_v0 = (time_v >> 16) % 256;
        let index_v1 = ((time_v >> 16) + 1) % 256;

        let idx = |vi: u32, ui: u32| -> usize { (vi * 256 + ui) as usize };

        ((factor_u0 * factor_v1) / 65536)
            * self.grayscale_image[idx(index_v1, index_u0)] as u32 / 65536
            + ((factor_u1 * factor_v1) / 65536)
            * self.grayscale_image[idx(index_v1, index_u1)] as u32 / 65536
            + ((factor_u0 * factor_v0) / 65536)
            * self.grayscale_image[idx(index_v0, index_u0)] as u32 / 65536
            + ((factor_u1 * factor_v0) / 65536)
            * self.grayscale_image[idx(index_v0, index_u1)] as u32 / 65536
    }

    /// 更新水效纹理（对应 C++ UpdateWaterEffect）
    ///
    /// 为避免借用冲突，将纹理图像的可变引用和 self 的不可变引用分离操作。
    pub fn update_water_effect(&mut self) {
        // 先解出 caustic_image 的引用（可变借用 self 的一部分）
        let caustic_image: &mut Box<MemoryImage> = match &mut self.caustic_image {
            Some(img) => img,
            None => return,
        };

        // 取出计算所需的数据（避免对 self 的不可变借用与 caustic_image 的可变借用冲突）
        let grayscale = &self.grayscale_image;
        let counter = self.pool_counter;

        let width = CAUSTIC_IMAGE_WIDTH as usize;
        let height = CAUSTIC_IMAGE_HEIGHT as usize;
        let pixels = &mut caustic_image.base.pixels;

        for y in 0..height {
            let time_v1 = ((256 - y) as u32) << 17;
            let time_v0 = (y as u32) << 17;

            for x in 0..width {
                let idx = y * width + x;
                let time_u = (x as u32) << 17;
                let time_pool0 = counter << 16;
                let time_pool1 = ((counter & 65535) + 1) << 16;

                let a1 = {
                    let u_val = time_u.wrapping_sub(time_pool1 / 6);
                    let v_val = time_v1.wrapping_add(time_pool0 / 8);
                    let u_t = u_val & 0xFFFF0000;
                    let v_t = v_val & 0xFFFF0000;
                    let fu1 = ((u_val - u_t) & 0x0000FFFE) + 1;
                    let fv1 = ((v_val - v_t) & 0x0000FFFE) + 1;
                    let fu0 = 65536 - fu1;
                    let fv0 = 65536 - fv1;
                    let iu0 = (u_t >> 16) % 256;
                    let iu1 = ((u_t >> 16) + 1) % 256;
                    let iv0 = (v_t >> 16) % 256;
                    let iv1 = ((v_t >> 16) + 1) % 256;

                    (((fu0 * fv1) / 65536) * grayscale[(iv1 * 256 + iu0) as usize] as u32 / 65536
                        + ((fu1 * fv1) / 65536) * grayscale[(iv1 * 256 + iu1) as usize] as u32 / 65536
                        + ((fu0 * fv0) / 65536) * grayscale[(iv0 * 256 + iu0) as usize] as u32 / 65536
                        + ((fu1 * fv0) / 65536) * grayscale[(iv0 * 256 + iu1) as usize] as u32 / 65536)
                        as u8
                };

                let a0 = {
                    let u_val = time_u.wrapping_add(time_pool0 / 10);
                    let v_val = time_v0;
                    let u_t = u_val & 0xFFFF0000;
                    let v_t = v_val & 0xFFFF0000;
                    let fu1 = ((u_val - u_t) & 0x0000FFFE) + 1;
                    let fv1 = ((v_val - v_t) & 0x0000FFFE) + 1;
                    let fu0 = 65536 - fu1;
                    let fv0 = 65536 - fv1;
                    let iu0 = (u_t >> 16) % 256;
                    let iu1 = ((u_t >> 16) + 1) % 256;
                    let iv0 = (v_t >> 16) % 256;
                    let iv1 = ((v_t >> 16) + 1) % 256;

                    (((fu0 * fv1) / 65536) * grayscale[(iv1 * 256 + iu0) as usize] as u32 / 65536
                        + ((fu1 * fv1) / 65536) * grayscale[(iv1 * 256 + iu1) as usize] as u32 / 65536
                        + ((fu0 * fv0) / 65536) * grayscale[(iv0 * 256 + iu0) as usize] as u32 / 65536
                        + ((fu1 * fv0) / 65536) * grayscale[(iv0 * 256 + iu1) as usize] as u32 / 65536)
                        as u8
                };

                let a = (a0 as u16 + a1 as u16) / 2;

                let alpha: u8 = if a >= 160 {
                    (255u16 - 2 * (a as u16 - 160)) as u8
                } else if a >= 128 {
                    (5 * (a as u16 - 128)) as u8
                } else {
                    0
                };

                // 修改 alpha 通道（像素是 RGBA 格式，第四字节是 alpha）
                let pixel_start = idx * 4;
                let alpha_scaled = (alpha as u32 / 3) as u8;
                if pixel_start + 3 < pixels.len() {
                    pixels[pixel_start + 3] = alpha_scaled;
                }
            }
        }

        caustic_image.bits_changed_count += 1;
    }

    /// 绘制水池效果（对应 C++ PoolEffectDraw）
    pub fn draw(&mut self, g: &mut Graphics, _is_night: bool) {
        self.update_water_effect();
        // C++ 版本中此函数构建三角网格并调用 g->DrawTrianglesTex()
        // Rust 版本中图形渲染管线尚未完全集成，此处为核心逻辑占位
        _ = g;
    }

    /// 更新计数器（对应 C++ PoolEffectUpdate）
    pub fn update(&mut self) {
        self.pool_counter = self.pool_counter.wrapping_add(1);
    }
}
