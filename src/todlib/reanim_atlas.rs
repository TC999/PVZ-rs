// PvZ Portable Rust 翻译 — ReanimAtlas 动画图集
// 对应 C++ src/Sexy.TodLib/ReanimAtlas.cpp（254 行）

#![allow(dead_code)]

use crate::framework::graphics::image::Image;
use crate::framework::graphics::memory_image::MemoryImage;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;
use crate::todlib::definition::ReanimatorDefinition;

/// 图集中的单张图像信息（对应 C++ ReanimAtlasImage）
#[derive(Debug, Clone, Copy)]
pub struct ReanimAtlasImage {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub original_image: *mut Image,
}

impl ReanimAtlasImage {
    pub fn new() -> Self { ReanimAtlasImage { x: 0, y: 0, width: 0, height: 0, original_image: std::ptr::null_mut() } }
}

/// 排序比较函数（对应 C++ sSortByNonIncreasingHeight：高降序 → 宽降序 → 指针降序）
pub fn sort_by_non_increasing_height(a: &ReanimAtlasImage, b: &ReanimAtlasImage) -> std::cmp::Ordering {
    if a.height != b.height {
        return b.height.cmp(&a.height);
    } else if a.width != b.width {
        return b.width.cmp(&a.width);
    }
    (b.original_image as usize).cmp(&(a.original_image as usize))
}

/// 动画图集（对应 C++ ReanimAtlas）
pub struct ReanimAtlas {
    pub image_array: Vec<ReanimAtlasImage>,
    pub memory_image: *mut MemoryImage,
}

impl ReanimAtlas {
    pub fn new() -> Self { ReanimAtlas { image_array: Vec::new(), memory_image: std::ptr::null_mut() } }

    /// 获取编码图集图像（对应 C++ GetEncodedReanimAtlas）
    /// [TRANSLATION_NOTE]: C++ 以 Image* 编码（index + 1，指针值 <= 1000）；Rust 以 i32 编码索引承载
    pub fn get_encoded_reanim_atlas(&self, encoded_index: i32) -> Option<&ReanimAtlasImage> {
        if encoded_index <= 0 || encoded_index > 1000 {
            return None;
        }
        let a_atlas_index = (encoded_index - 1) as usize;
        self.image_array.get(a_atlas_index)
    }

    /// 选择图集宽度（对应 C++ PickAtlasWidth：总面积平方根与最大宽取大，上限 2048，向上取 2 的幂）
    pub fn pick_atlas_width(&self) -> i32 {
        let mut total_area = 0;
        let mut a_max_width = 0;
        for a_image in &self.image_array {
            total_area += a_image.width * a_image.height;
            a_max_width = a_max_width.max(a_image.width + 2);
        }
        let a_width = (total_area as f32).sqrt() as i32;
        let a_capped = a_width.max(a_max_width).min(2048);
        // 对应 C++: GetClosestPowerOf2Above
        let mut a_power2 = 1;
        while a_power2 < a_capped {
            a_power2 <<= 1;
        }
        a_power2
    }

    /// 矩形是否可放置（对应 C++ ImageFits：与已放置图像（各膨胀 1 像素）不相交）
    pub fn image_fits(&self, image_count: i32, rect_test: &Rect, max_width: i32) -> bool {
        if rect_test.x + rect_test.width > max_width {
            return false;
        }
        for i in 0..image_count as usize {
            let a_image = &self.image_array[i];
            let mut a_rect = Rect::new(a_image.x, a_image.y, a_image.width, a_image.height);
            a_rect.inflate(1, 1);
            if a_rect.intersects(rect_test) {
                return false;
            }
        }
        true
    }

    /// 在已放置图像的右侧或下方找空位（对应 C++ ImageFindPlaceOnSide）
    pub fn image_find_place_on_side(
        &mut self,
        the_atlas_image_to_place: &mut ReanimAtlasImage,
        image_count: i32,
        max_width: i32,
        to_right: bool,
    ) -> bool {
        let mut rect_test = Rect::new(0, 0, the_atlas_image_to_place.width + 2, the_atlas_image_to_place.height + 2);
        for i in 0..image_count as usize {
            let a_image = &self.image_array[i];
            if to_right {
                // 对应 C++: 放在已放置图像右侧
                rect_test.x = a_image.x + a_image.width + 1;
                rect_test.y = a_image.y;
            } else {
                // 对应 C++: 放在已放置图像下方
                rect_test.x = a_image.x;
                rect_test.y = a_image.y + a_image.height + 1;
            }
            if self.image_fits(image_count, &rect_test, max_width) {
                the_atlas_image_to_place.x = rect_test.x;
                the_atlas_image_to_place.y = rect_test.y;
                if to_right {
                    the_atlas_image_to_place.x += 1;
                } else {
                    the_atlas_image_to_place.y += 1;
                }
                return true;
            }
        }
        false
    }

    /// 寻找放置位置（对应 C++ ImageFindPlace：先右后下）
    pub fn image_find_place(&mut self, the_atlas_image_to_place: &mut ReanimAtlasImage, image_count: i32, max_width: i32) -> bool {
        self.image_find_place_on_side(the_atlas_image_to_place, image_count, max_width, true)
            || self.image_find_place_on_side(the_atlas_image_to_place, image_count, max_width, false)
    }

    /// 放置单张图像（对应 C++ PlaceAtlasImage：首图固定 (1,1)）
    pub fn place_atlas_image(&mut self, the_atlas_image_to_place: &mut ReanimAtlasImage, image_count: i32, max_width: i32) -> bool {
        if image_count == 0 {
            the_atlas_image_to_place.x = 1;
            the_atlas_image_to_place.y = 1;
            return true;
        }
        self.image_find_place(the_atlas_image_to_place, image_count, max_width)
    }

    /// 排列图像并确定图集尺寸（对应 C++ ArrangeImages）
    pub fn arrange_images(&mut self, atlas_width: &mut i32, atlas_height: &mut i32) {
        self.image_array.sort_by(sort_by_non_increasing_height);
        *atlas_width = self.pick_atlas_width();
        *atlas_height = 0;
        for i in 0..self.image_array.len() {
            // 借用规避：取副本放置后写回（ReanimAtlasImage 为 Copy）
            let mut a_image = self.image_array[i];
            self.place_atlas_image(&mut a_image, i as i32, *atlas_width);
            // 对应 C++: theAtlasHeight = max(GetClosestPowerOf2Above(mY + mHeight), ...)
            let a_bottom = a_image.y + a_image.height;
            let mut a_power2 = 1;
            while a_power2 < a_bottom {
                a_power2 <<= 1;
            }
            *atlas_height = a_power2.max(*atlas_height);
            self.image_array[i] = a_image;
        }
    }

    /// 添加图像（对应 C++ AddImage：仅单列单行）
    pub fn add_image(&mut self, the_image: *mut Image) {
        if the_image.is_null() {
            return;
        }
        unsafe {
            if (*the_image).num_cols == 1 && (*the_image).num_rows == 1 {
                self.image_array.push(ReanimAtlasImage {
                    x: 0,
                    y: 0,
                    width: (*the_image).width,
                    height: (*the_image).height,
                    original_image: the_image,
                });
            }
        }
    }

    /// 查找图像索引（对应 C++ FindImage，找不到返回 -1）
    pub fn find_image(&self, the_image: *mut Image) -> i32 {
        for i in 0..self.image_array.len() {
            if self.image_array[i].original_image == the_image {
                return i as i32;
            }
        }
        -1
    }

    /// 创建图集纹理（对应 C++ ReanimAtlasCreate 的纹理生成阶段）
    /// 由 create_from_definition 在收集/编码完成后调用：ArrangeImages + 空白纹理 + 绘制 + 边缘混合修复
    pub fn create(&mut self) {
        let mut a_atlas_width = 0;
        let mut a_atlas_height = 0;
        self.arrange_images(&mut a_atlas_width, &mut a_atlas_height);

        // 对应 C++: ReanimAtlasMakeBlankMemoryImage（ARGB 零填充 + 透明/alpha 标记）
        let mut a_memory_image = MemoryImage::new(a_atlas_width, a_atlas_height);
        a_memory_image.has_trans = true;
        a_memory_image.has_alpha = true;

        // 对应 C++: 逐图绘制到图集位置
        let a_dest_ptr = &mut a_memory_image.base as *mut Image;
        let mut a_memory_graphics = Graphics::new_with_image(a_dest_ptr);
        for a_image in &self.image_array {
            if !a_image.original_image.is_null() {
                unsafe {
                    a_memory_graphics.draw_image_xy(&*a_image.original_image, a_image.x, a_image.y);
                }
            }
        }
        // 对应 C++: FixPixelsOnAlphaEdgeForBlending（透明像素取邻色均值）
        a_memory_image.fix_pixels_on_alpha_edge_for_blending();

        self.memory_image = Box::into_raw(Box::new(a_memory_image));
    }

    /// 从定义收集图像并编码句柄（对应 C++ ReanimAtlasCreate 的收集+编码阶段，
    /// ReanimAtlas.cpp:10-37）：将 ≤254×254 的单列图加入图集，并把
    /// transform.m_image 回写为编码索引（index+1，1..1000 范围内）
    pub fn create_from_definition(def: &mut ReanimatorDefinition) -> Option<*mut ReanimAtlas> {
        use crate::todlib::reanim_loader::{get_reanim_image_name, reanimator_get_image};
        let mut a_atlas = ReanimAtlas::new();
        // 第一遍：收集
        for a_track in &def.m_tracks {
            for a_key in &a_track.m_transforms {
                let a_image_id = a_key.m_image;
                if a_image_id < 0 {
                    continue;
                }
                if let Some(a_image) = reanimator_get_image(a_image_id) {
                    unsafe {
                        if (*a_image).width <= 254
                            && (*a_image).height <= 254
                            && (*a_image).num_cols == 1
                            && (*a_image).num_rows == 1
                            && a_atlas.find_image(a_image) < 0
                        {
                            a_atlas.add_image(a_image);
                        }
                    }
                }
                let _ = get_reanim_image_name(a_image_id);
            }
        }
        if a_atlas.image_array.is_empty() {
            return None; // 无可用图集图（C++ 亦会创建空图集，此处等价跳过）
        }
        let mut a_atlas_width = 0;
        let mut a_atlas_height = 0;
        a_atlas.arrange_images(&mut a_atlas_width, &mut a_atlas_height);
        // 第二遍：编码句柄回写（对应 C++ aImage = (Image*)(aImageIndex + 1)）
        for a_track in &mut def.m_tracks {
            for a_key in &mut a_track.m_transforms {
                let a_image_id = a_key.m_image;
                if a_image_id < 0 {
                    continue;
                }
                if let Some(a_image) = reanimator_get_image(a_image_id) {
                    let a_index = a_atlas.find_image(a_image);
                    if a_index < 0 {
                        continue; // 未入图集（如多列图）
                    }
                    // 编码索引 = index + 1，对应 get_encoded_reanim_atlas 的解码
                    a_key.m_image = a_index + 1;
                }
            }
        }
        // 生成内存图集纹理
        a_atlas.create();
        Some(Box::into_raw(Box::new(a_atlas)))
    }
}
