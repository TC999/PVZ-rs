// PvZ Portable Rust 翻译 — Quantize 颜色量化
// 对应 C++ SexyAppFramework/graphics/Quantize.h / Quantize.cpp
//
// 中值切割颜色量化：将 32 位图像转为 256 色调色板图像。
// 使用二分查找维护排序的颜色表，去重并建立索引。

#![allow(dead_code)]

use crate::framework::common::uchar;

/// 将 32 位 RGBA 像素数据量化为 8 位索引色（对应 C++ Quantize8Bit）
///
/// # 参数
/// * `src_bits` - 输入像素数组（32 位 RGBA，每个像素 4 字节）
/// * `width` - 图像宽度
/// * `height` - 图像高度
/// * `dest_color_indices` - 输出颜色索引数组（每个像素 1 字节）
/// * `dest_color_table` - 输出调色板（最多 256 种颜色）
///
/// # 返回值
/// 成功返回 true，颜色数超过 256 返回 false
pub fn quantize_8bit(
    src_bits: &[u32],
    width: i32,
    height: i32,
    dest_color_indices: &mut [u8],
    dest_color_table: &mut [u32],
) -> bool {
    let size = (width * height) as usize;

    if size == 0 {
        return true;
    }

    let mut color_table_size = 0usize;
    let mut search_table = [0u32; 256];
    let mut translation_table = [0u8; 256]; // 从搜索表到颜色表的映射

    // 处理第一个像素
    search_table[0] = src_bits[0];
    dest_color_table[0] = src_bits[0];
    translation_table[0] = 0;
    dest_color_indices[0] = 0;
    color_table_size = 1;

    // 遍历剩余像素
    for idx in 1..size {
        let color = src_bits[idx];

        let mut left_pos = 0i32;
        let mut right_pos = (color_table_size - 1) as i32;
        let mut middle_pos = (left_pos + right_pos) / 2;

        loop {
            let check_color = search_table[middle_pos as usize];

            if color < check_color {
                right_pos = middle_pos - 1;
            } else if color > check_color {
                left_pos = middle_pos + 1;
            } else {
                // 找到匹配颜色，使用已有索引
                dest_color_indices[idx] = translation_table[middle_pos as usize];
                break;
            }

            if left_pos > right_pos {
                // 颜色不在表中，需要插入
                if color_table_size >= 256 {
                    return false; // 超过 256 色限制
                }

                let insert_pos = left_pos as usize;

                // 在排序的搜索表中插入新颜色
                if insert_pos + 1 < 256 {
                    search_table.copy_within(insert_pos..color_table_size, insert_pos + 1);
                    translation_table.copy_within(insert_pos..color_table_size, insert_pos + 1);
                }
                search_table[insert_pos] = color;
                translation_table[insert_pos] = color_table_size as u8;

                dest_color_table[color_table_size] = color;
                dest_color_indices[idx] = color_table_size as u8;
                color_table_size += 1;

                break;
            }

            middle_pos = (left_pos + right_pos) / 2;
        }
    }

    true
}
