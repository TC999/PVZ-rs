// PvZ Portable Rust 翻译 — PoolEffect（水池波光效果）
// 对应 C++ src/Lawn/System/PoolEffect.cpp / PoolEffect.h
//
// 管理水池表面的波光粼粼视觉效果，包含波纹动画与纹理渲染。

use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::gl_interface::TriVertex;
use crate::framework::graphics::memory_image::MemoryImage;

const CAUSTIC_IMAGE_WIDTH: i32 = 128;
const CAUSTIC_IMAGE_HEIGHT: i32 = 64;
/// 游戏资源 IMAGE_POOL 的实际尺寸（704x300），
/// 对应 C++ PoolEffectDraw 中 IMAGE_POOL->GetWidth()/GetHeight()
const POOL_IMAGE_WIDTH: f32 = 704.0;
const POOL_IMAGE_HEIGHT: f32 = 300.0;

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
    // [TRANSLATION_NOTE]: IMAGE_POOL / IMAGE_POOL_NIGHT / IMAGE_POOL_BASE / IMAGE_POOL_SHADING /
    /// IMAGE_POOL_BASE_NIGHT / IMAGE_POOL_SHADING_NIGHT 六个全局图片资源尚未接入 Rust 资源系统，
    /// 软件渲染分支与基底/遮罩层纹理选择保留 C++ 的分支结构与 is_night 区分逻辑，但以注释占位；
    /// caustic 层（mCausticImage，运行时内存生成）真实调用 Graphics::draw_triangles_tex 绘制。
    /// is_night 的实际生效点——caustic 顶点颜色（0x30FFFFFF vs 0xC0/0x80FFFFFF）——按 C++ 原样执行。
    ///
    /// POOL_IMAGE_WIDTH/HEIGHT 为游戏资源 IMAGE_POOL 的 704x300 实际尺寸，
    /// 对应 C++ 中 IMAGE_POOL->GetWidth()/GetHeight()。
    pub fn draw(&mut self, g: &mut Graphics, the_is_night: bool) {
        // C++: if (!mApp->Is3DAccelerated()) —— 本移植中 3D 加速恒启用，软件分支永不执行（C++ 源码注释原话）
        if false {
            // 软件渲染分支（保留结构）：夜晚用 IMAGE_POOL_NIGHT，白天用 IMAGE_POOL
            if the_is_night {
                // g->DrawImage(IMAGE_POOL_NIGHT, 34, 278); —— 资源 IMAGE_POOL_NIGHT 未接入
            } else {
                // g->DrawImage(IMAGE_POOL, 34, 278); —— 资源 IMAGE_POOL 未接入
            }
            return;
        }
        // pool background
        let a_grid_square_x = POOL_IMAGE_WIDTH / 15.0f32;
        let a_grid_square_y = POOL_IMAGE_HEIGHT / 5.0f32;
        // aOffsetArray[3][16][6][2]（[层][x][y][xy]），对应 C++ float aOffsetArray[3][16][6][2]
        let mut a_offset_array = [[[[0.0f32; 2]; 6]; 16]; 3];
        for x in 0..=15 {
            for y in 0..=5 {
                // handles the caustic effect
                a_offset_array[2][x][y][0] = x as f32 / 15.0f32;
                a_offset_array[2][x][y][1] = y as f32 / 5.0f32;
                if x != 0 && x != 15 && y != 0 && y != 5 {
                    // LCM of all sin wave effective periods (1600, 300, 1800, 220, 3200/3, 200, 720, 640, 88)
                    const POOL_PHASE_PERIOD: u32 = 316800u32;
                    let a_pool_phase = (self.pool_counter % POOL_PHASE_PERIOD) as f32 * std::f32::consts::PI; // speed, * 2 is default
                    let a_wave_time1 = a_pool_phase / 800.0f32;
                    let a_wave_time2 = a_pool_phase / 150.0f32;
                    let a_wave_time3 = a_pool_phase / 900.0f32;
                    let a_wave_time4 = a_pool_phase / 800.0f32;
                    let a_wave_time5 = a_pool_phase / 110.0f32;
                    let x_phase = x as f32 * 3.0f32 * 2.0 * std::f32::consts::PI / 15.0f32;
                    let y_phase = y as f32 * 3.0f32 * 2.0 * std::f32::consts::PI / 5.0f32;
                    // verticies for rendering, dividing by 1 gives interesting results
                    a_offset_array[0][x][y][0] = (y_phase + a_wave_time2).sin() * 0.002f32 + (y_phase + a_wave_time1).sin() * 0.005f32;
                    a_offset_array[0][x][y][1] = (x_phase + a_wave_time5).sin() * 0.01f32 + (x_phase + a_wave_time3).sin() * 0.015f32 + (x_phase + a_wave_time4).sin() * 0.005f32;
                    a_offset_array[1][x][y][0] = (y_phase * 0.2f32 + a_wave_time2).sin() * 0.015f32 + (y_phase * 0.2f32 + a_wave_time1).sin() * 0.012f32;
                    a_offset_array[1][x][y][1] = (x_phase * 0.2f32 + a_wave_time5).sin() * 0.005f32 + (x_phase * 0.2f32 + a_wave_time3).sin() * 0.015f32 + (x_phase * 0.2f32 + a_wave_time4).sin() * 0.02f32;
                    a_offset_array[2][x][y][0] += (y_phase + a_wave_time1 * 1.5f32).sin() * 0.004f32 + (y_phase + a_wave_time2 * 1.5f32).sin() * 0.005f32;
                    a_offset_array[2][x][y][1] += (x_phase * 4.0f32 + a_wave_time5 * 2.5f32).sin() * 0.005f32 + (x_phase * 2.0f32 + a_wave_time3 * 2.5f32).sin() * 0.04f32 + (x_phase * 3.0f32 + a_wave_time4 * 2.5f32).sin() * 0.02f32;
                } else {
                    // skip animation
                    a_offset_array[0][x][y][0] = 0.0f32;
                    a_offset_array[0][x][y][1] = 0.0f32;
                    a_offset_array[1][x][y][0] = 0.0f32;
                    a_offset_array[1][x][y][1] = 0.0f32;
                }
            }
        }

        let a_index_offset_x = [0, 0, 1, 0, 1, 1];
        let a_index_offset_y = [0, 1, 1, 0, 1, 0];
        let zero_vert = TriVertex { x: 0.0, y: 0.0, u: 0.0, v: 0.0, color: 0 };
        let mut a_vert_array = [[[zero_vert; 3]; 150]; 3];

        for x in 0..15 {
            for y in 0..5 {
                for a_layer in 0..3 {
                    // C++: TriVertex* pVert = &aVertArray[aLayer][x * 10 + y * 2][0];
                    // 连续填充 6 个顶点（2 个三角形，Rust 数组索引 [tri][k] 与 C++ 扁平内存布局一一对应）
                    let tri_index = x * 10 + y * 2;
                    for a_vert_index in 0..6 {
                        let a_index_x = x + a_index_offset_x[a_vert_index];
                        let a_index_y = y + a_index_offset_y[a_vert_index];
                        let p_vert = &mut a_vert_array[a_layer][tri_index + a_vert_index / 3][a_vert_index % 3];
                        if a_layer == 2 {
                            // caustic effect
                            p_vert.x = (704.0f32 / 15.0f32) * a_index_x as f32 + 45.0f32; // x-offset
                            p_vert.y = 30.0f32 * a_index_y as f32 + 288.0f32; // y-offset
                            p_vert.u = a_offset_array[2][a_index_x][a_index_y][0] + a_index_x as f32 / 15.0f32;
                            p_vert.v = a_offset_array[2][a_index_x][a_index_y][1] + a_index_y as f32 / 5.0f32;
                            // use correct colors depending on the scene
                            if !g.clip_rect.contains(p_vert.x as i32, p_vert.y as i32) {
                                p_vert.color = 0x00FFFFFF;
                            } else if a_index_x == 0 || a_index_x == 15 || a_index_y == 0 {
                                p_vert.color = 0x20FFFFFF;
                            } else if the_is_night {
                                p_vert.color = 0x30FFFFFF;
                            } else {
                                p_vert.color = if a_index_x <= 7 { 0xC0FFFFFF } else { 0x80FFFFFF };
                            }
                        } else {
                            // update water outlines
                            p_vert.color = 0xFFFFFFFF;
                            p_vert.x = a_index_x as f32 * a_grid_square_x + 35.0f32;
                            p_vert.y = a_index_y as f32 * a_grid_square_y + 279.0f32;
                            p_vert.u = a_offset_array[a_layer][a_index_x][a_index_y][0] + a_index_x as f32 / 15.0f32;
                            p_vert.v = a_offset_array[a_layer][a_index_x][a_index_y][1] + a_index_y as f32 / 5.0f32;
                            if !g.clip_rect.contains(p_vert.x as i32, p_vert.y as i32) {
                                p_vert.color = 0x00FFFFFF;
                            }
                        }
                    }
                }
            }
        }
        // draw correct shading type depending on area.
        if the_is_night {
            // g->DrawTrianglesTex(IMAGE_POOL_BASE_NIGHT, aVertArray[0], 150); —— 资源未接入
            // g->DrawTrianglesTex(IMAGE_POOL_SHADING_NIGHT, aVertArray[1], 150); —— 资源未接入
        } else {
            // g->DrawTrianglesTex(IMAGE_POOL_BASE, aVertArray[0], 150); —— 资源未接入
            // g->DrawTrianglesTex(IMAGE_POOL_SHADING, aVertArray[1], 150); —— 资源未接入
        }
        // update positions
        self.update_water_effect();
        // send caustic effect tris to OpenGL (tex, verts, tris)
        if let Some(caustic) = &self.caustic_image {
            g.draw_triangles_tex(&caustic.base, &a_vert_array[2], 150);
        }
    }

    /// 更新计数器（对应 C++ PoolEffectUpdate）
    pub fn update(&mut self) {
        self.pool_counter = self.pool_counter.wrapping_add(1);
    }
}
