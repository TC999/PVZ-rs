// PvZ Portable Rust 翻译 — TodCommon（工具函数）
// 对应 C++ src/Sexy.TodLib/TodCommon.h / TodCommon.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::color::Color;
use crate::lawn::game_enums::{TodCurves, DrawStringJustification};

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
