// PvZ Portable Rust 翻译 — TodCommon（工具函数）
// 对应 C++ src/Sexy.TodLib/TodCommon.h / TodCommon.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::color::Color;
use crate::lawn::game_enums::*;

/// 通用工具类型 — 加权数组（对应 C++ TodWeightedArray）
#[derive(Debug, Clone, Copy)]
pub struct TodWeightedArray {
    pub item: usize,
    pub weight: i32,
}

/// 通用工具类型 — 网格加权数组（对应 C++ TodWeightedGridArray）
#[derive(Debug, Clone, Copy)]
pub struct TodWeightedGridArray {
    pub x: i32,
    pub y: i32,
    pub weight: i32,
}

/// 通用工具类型 — 平滑数组（对应 C++ TodSmoothArray）
#[derive(Debug, Clone, Copy)]
pub struct TodSmoothArray {
    pub item: i32,
    pub weight: f32,
    pub last_picked: f32,
    pub second_last_picked: f32,
}

/// 夹值
pub fn clamp_float(val: f32, min: f32, max: f32) -> f32 {
    if val < min { min } else if val > max { max } else { val }
}

pub fn clamp_int(val: i32, min: i32, max: i32) -> i32 {
    if val < min { min } else if val > max { max } else { val }
}

/// 线性插值
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// 曲线插值（对应 C++ TodCurve 评估）
pub fn evaluate_curve(curve: TodCurves, start: f32, end: f32, time: f32) -> f32 {
    let t = clamp_float(time, 0.0, 1.0);
    let v = match curve {
        TodCurves::Constant => 0.0,
        TodCurves::Linear => t,
        TodCurves::EaseIn => t * t,
        TodCurves::EaseOut => t * (2.0 - t),
        TodCurves::EaseInOut => {
            if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t }
        },
        TodCurves::EaseInOutWeak => {
            if t < 0.5 { 0.5 * t * t * 4.0 } else { 1.0 - 0.5 * (1.0 - t) * (1.0 - t) * 4.0 }
        },
        TodCurves::FastInOut => t * t * (3.0 - 2.0 * t), // smoothstep
        TodCurves::FastInOutWeak => t * t * (3.0 - 2.0 * t),
        TodCurves::Bounce => {
            let y = 1.0 - t;
            1.0 - (y * y * (3.0 - 2.0 * y))
        },
        TodCurves::SinWave => ((t * std::f32::consts::PI * 2.0).sin() + 1.0) * 0.5,
        TodCurves::EaseSinWave => {
            let s = (t * std::f32::consts::PI).sin();
            s * s
        },
        _ => t,
    };
    start + (end - start) * v
}

/// 计算角度向量
pub fn angle_to_vector(angle: f32) -> (f32, f32) {
    (angle.cos(), angle.sin())
}

/// 计算两点间距离
pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

/// 随机范围浮点
pub fn rand_range_float(min: f32, max: f32) -> f32 {
    if min >= max { return min; }
    let r = crate::framework::common::rand() as f32 / 0x7FFFFFFF as f32;
    min + r * (max - min)
}

/// 随机范围整数
pub fn rand_range_int(min: i32, max: i32) -> i32 {
    if min >= max { return min; }
    crate::framework::common::rand_range(max - min + 1) + min
}

/// 浮点数约等于
pub fn float_nearly_equal(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

/// 度数转弧度
pub fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

/// 动画曲线（时间范围为整数，值范围为浮点数，对应 C++ TodAnimateCurveFloat）
pub fn tod_animate_curve_float(
    time_start: i32, time_end: i32, time_age: i32,
    pos_start: f32, pos_end: f32, curve: TodCurves,
) -> f32 {
    if time_start == time_end {
        return pos_start;
    }
    let t = if time_age < time_start {
        0.0
    } else if time_age > time_end {
        1.0
    } else {
        (time_age - time_start) as f32 / (time_end - time_start) as f32
    };
    evaluate_curve(curve, pos_start, pos_end, t)
}

/// 动画曲线（全整数版本，对应 C++ TodAnimateCurve）
pub fn tod_animate_curve(
    time_start: i32, time_end: i32, time_age: i32,
    pos_start: i32, pos_end: i32, curve: TodCurves,
) -> i32 {
    tod_animate_curve_float(time_start, time_end, time_age, pos_start as f32, pos_end as f32, curve) as i32
}

/// 字符串翻译（简化版，对应 C++ TodStringTranslate）
/// 根据语言字符串查找翻译。如果未找到返回原文。
pub fn tod_string_translate(text: &str) -> String {
    // 从全局字符串文件查找翻译
    // 当前简化实现：直接返回原文
    text.to_string()
}

/// 绘制字符串（对应 C++ TodDrawString）
/// 使用指定字体和颜色在 (thePosX, thePosY) 处绘制文本，支持对齐方式
pub fn tod_draw_string(
    g: &mut Graphics,
    text: &str,
    the_pos_x: i32,
    the_pos_y: i32,
    the_font: &Font,
    the_color: &Color,
    the_justification: DrawStringJustification,
) {
    let final_text = tod_string_translate(text);

    let mut a_pos_x = the_pos_x;
    match the_justification {
        DrawStringJustification::DS_ALIGN_RIGHT
        | DrawStringJustification::DS_ALIGN_RIGHT_VERTICAL_MIDDLE => {
            a_pos_x -= the_font.string_width(&final_text);
        }
        DrawStringJustification::DS_ALIGN_CENTER
        | DrawStringJustification::DS_ALIGN_CENTER_VERTICAL_MIDDLE => {
            a_pos_x -= the_font.string_width(&final_text) / 2;
        }
        _ => {}
    }

    // 直接调用 Font::draw_string（传递 Graphics 当前的裁剪矩形）
    let clip_rect = g.clip_rect;
    the_font.draw_string(g, a_pos_x, the_pos_y, &final_text, the_color, &clip_rect);
}

/// 动画曲线（全浮点时间版本，对应 C++ TodAnimateCurveFloatTime）
pub fn tod_animate_curve_float_time(
    time_start: f32, time_end: f32, time_age: f32,
    pos_start: f32, pos_end: f32, curve: TodCurves,
) -> f32 {
    if (time_end - time_start).abs() < 0.0001 {
        return pos_start;
    }
    let t = if time_age < time_start {
        0.0
    } else if time_age > time_end {
        1.0
    } else {
        (time_age - time_start) / (time_end - time_start)
    };
    evaluate_curve(curve, pos_start, pos_end, t)
}

/// 平滑权重计算（对应 C++ TodCalcSmoothWeight）
/// 根据权重和上次/上上次被选中时间计算调整后的权重值
fn tod_calc_smooth_weight(weight: f32, last_picked: f32, second_last_picked: f32) -> f32 {
    if weight < 1e-6 {
        return 0.0;
    }

    let expected_length1 = 1.0 / weight;                          // last_picked 的期望值
    let expected_length2 = expected_length1 * 2.0;                 // second_last_picked 的期望值
    let advanced_length1 = last_picked + 1.0 - expected_length1;   // 相较于期望值的提前轮数
    let advanced_length2 = second_last_picked + 1.0 - expected_length2;
    let factor1 = 1.0 + advanced_length1 / expected_length1 * 2.0;
    let factor2 = 1.0 + advanced_length2 / expected_length2 * 2.0;
    let factor_final = clamp_float(factor1 * 0.75 + factor2 * 0.25, 0.01, 100.0);
    weight * factor_final
}

/// 从平滑数组中按权重选择一个项目（对应 C++ TodPickFromSmoothArray）
/// 返回被选中项目的索引
pub fn tod_pick_from_smooth_array(arr: &mut [TodSmoothArray]) -> i32 {
    let count = arr.len();
    if count == 0 {
        return -1;
    }

    // 计算总权重
    let total_weight: f32 = arr.iter().map(|a| a.weight).sum();
    debug_assert!(total_weight > 0.0);

    let normalize_factor = 1.0 / total_weight;

    // 计算调整后的总权重
    let total_adjusted_weight: f32 = arr.iter().map(|a| {
        tod_calc_smooth_weight(a.weight * normalize_factor, a.last_picked, a.second_last_picked)
    }).sum();
    debug_assert!(total_adjusted_weight > 0.0);

    // 随机选择
    let rand_weight = crate::framework::common::rand_float(total_adjusted_weight);
    let mut accumulated_weight = 0.0;
    let mut pick_idx = count - 1;
    for (i, a) in arr.iter().enumerate() {
        if i == count - 1 {
            break;
        }
        accumulated_weight += tod_calc_smooth_weight(a.weight * normalize_factor, a.last_picked, a.second_last_picked);
        if rand_weight <= accumulated_weight {
            pick_idx = i;
            break;
        }
    }

    // 更新选中记录
    tod_update_smooth_array_pick(arr, pick_idx);
    arr[pick_idx].item
}

/// 从加权数组中随机选择一个项目并返回其索引（对应 C++ TodPickFromWeightedArray）
pub fn tod_pick_from_weighted_array(arr: &[TodWeightedArray]) -> isize {
    tod_pick_array_item_from_weighted_array(arr).map_or(-1, |item| item.item as isize)
}

/// 从加权网格数组中随机选择一个项目（对应 C++ TodPickFromWeightedGridArray）
/// 返回被选中项目的索引，并将其权重清零。失败返回 None。
pub fn tod_pick_from_weighted_grid_array(arr: &mut [TodWeightedGridArray], count: usize) -> Option<usize> {
    if count == 0 {
        return None;
    }

    let total_weight: i32 = arr[..count].iter().map(|a| a.weight).sum();
    debug_assert!(total_weight > 0);
    if total_weight <= 0 {
        return None;
    }

    let mut rand_weight = crate::framework::common::rand_range(total_weight);
    for i in 0..count {
        rand_weight -= arr[i].weight;
        if rand_weight < 0 {
            return Some(i);
        }
    }

    debug_assert!(false, "TodPickFromWeightedGridArray: should not reach here");
    None
}

/// 从加权数组中随机选择一个项目并返回其引用（对应 C++ TodPickArrayItemFromWeightedArray）
pub fn tod_pick_array_item_from_weighted_array(arr: &[TodWeightedArray]) -> Option<&TodWeightedArray> {
    let count = arr.len();
    if count == 0 {
        return None;
    }

    let total_weight: i32 = arr.iter().map(|a| a.weight).sum();
    debug_assert!(total_weight > 0);

    let mut rand_weight = crate::framework::common::rand_range(total_weight);

    for item in arr.iter() {
        rand_weight -= item.weight;
        if rand_weight < 0 {
            return Some(item);
        }
    }

    debug_assert!(false, "TodPickArrayItemFromWeightedArray: should not reach here");
    None
}

/// 更新平滑数组的选中记录（对应 C++ TodUpdateSmoothArrayPick）
pub fn tod_update_smooth_array_pick(arr: &mut [TodSmoothArray], pick_idx: usize) {
    for a in arr.iter_mut() {
        if a.weight > 0.0 {
            a.last_picked += 1.0;
            a.second_last_picked += 1.0;
        }
    }
    arr[pick_idx].second_last_picked = arr[pick_idx].last_picked;
    arr[pick_idx].last_picked = 0.0;
}

/// 获取闪烁颜色（对应 C++ GetFlashingColor）
/// 根据计数器和闪烁时间产生周期性闪烁的灰度颜色
pub fn get_flashing_color(counter: u32, flash_time: i32) -> Color {
    let time_age = (counter % flash_time as u32) as i32;
    let time_inf = flash_time / 2;
    let grayness = clamp_int(200 * (time_inf - time_age).abs() / time_inf + 55, 0, 255) as u8;
    Color::new(grayness, grayness, grayness, 255)
}

/// RGB 转 HSL（对应 C++ RGB_to_HSL）
/// 输入 R/G/B 范围 [0.0, 1.0]，输出 H [0.0, 1.0), S [0.0, 1.0], L [0.0, 1.0]
pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let maxval = r.max(g).max(b);
    let minval = r.min(g).min(b);

    let l = (minval + maxval) / 2.0;
    if l <= 0.0 {
        return (0.0, 0.0, 0.0);
    }

    let delta = maxval - minval;
    let mut s = delta;
    if s <= 0.0 {
        return (0.0, 0.0, l);
    }
    s /= if l <= 0.5 { minval + maxval } else { 2.0 - minval - maxval };

    let r2 = (maxval - r) / delta;
    let g2 = (maxval - g) / delta;
    let b2 = (maxval - b) / delta;

    let h = if maxval == r {
        if g == minval { 5.0 + b2 } else { 1.0 - g2 }
    } else if maxval == g {
        if b == minval { 1.0 + r2 } else { 3.0 - b2 }
    } else {
        if r == minval { 3.0 + g2 } else { 5.0 - r2 }
    } / 6.0;

    (h, s, l)
}
