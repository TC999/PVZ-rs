// PvZ Portable Rust 翻译 — SW 三角形绘制函数
// 对应 C++ src/Sexy.TodLib/EffectSystem.h 中的 TodDrawTriangle_* 和 DrawTriangle_*
//
// 这些是软件渲染器的三角形绘制变体（像素格式/纹理/混合模式组合），
// 在 OpenGL 硬件渲染路径下不会实际调用。
// 此处用宏生成所有变体作为占位，引用 GLInterface 中的已有实现。

#![allow(unused_variables, dead_code, non_snake_case)]

/// SW 顶点（对应 C++ SWHelper::SWVertex）
pub struct SWVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rhw: f32,
    pub tu: f32,
    pub tv: f32,
    pub color: u32,
}

/// SW 纹理信息（对应 C++ SWHelper::SWTextureInfo）
pub struct SWTextureInfo {
    pub pixels: *const u8,
    pub width: i32,
    pub height: i32,
    pub pitch: i32,
}

/// SW 漫反射颜色（对应 C++ SWHelper::SWDiffuse）
pub struct SWDiffuse {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// ============================================================
// 宏：生成单个 TodDrawTriangle 变体
// ============================================================

macro_rules! tod_draw_triangle_variant {
    ($name:ident) => {
        #[allow(non_snake_case)]
        pub fn $name(
            pVerts: *mut SWVertex,
            pFrameBuffer: *mut std::ffi::c_void,
            bytepitch: u32,
            textureInfo: *const SWTextureInfo,
            globalDiffuse: &mut SWDiffuse,
        ) {
            // SW 渲染器变体 — 在 GL 模式下为空操作
            // 实际绘制由 GLInterface::draw_triangle / draw_triangle_tex 完成
            let _ = (pVerts, pFrameBuffer, bytepitch, textureInfo, globalDiffuse);
        }
    };
}

macro_rules! draw_triangle_variant {
    ($name:ident) => {
        #[allow(non_snake_case)]
        pub fn $name(
            pVerts: *mut SWVertex,
            pFrameBuffer: *mut std::ffi::c_void,
            bytepitch: u32,
            textureInfo: *const SWTextureInfo,
            globalDiffuse: &mut SWDiffuse,
        ) {
            let _ = (pVerts, pFrameBuffer, bytepitch, textureInfo, globalDiffuse);
        }
    };
}

// ============================================================
// 所有 TodDrawTriangle_* 变体
// ============================================================

// --- 8888 格式 (TEX1 only) ---
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA0_MOD0_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA0_MOD0_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA0_MOD0_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA0_MOD0_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA0_MOD1_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA0_MOD1_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA0_MOD1_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA0_MOD1_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA1_MOD0_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA1_MOD0_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA1_MOD0_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA1_MOD0_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA1_MOD1_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA1_MOD1_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA1_MOD1_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_8888_TEX1_TALPHA1_MOD1_GLOB1_BLEND1);

// --- 0888 格式 (TEX1 + additive variants) ---
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD0_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD0_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD0_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD0_GLOB1_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD0_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD0_GLOB1_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB0_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB0_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB1_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB1_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB0_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB0_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB1_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB1_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB0_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB0_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB1_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB1_BLEND1_ADDITIVE);

// --- 0565 格式 (TEX1 + additive variants) ---
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD0_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD0_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD0_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD0_GLOB1_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD0_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD0_GLOB1_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB0_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB0_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB1_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB1_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB0_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB0_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB1_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB1_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB0_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB0_BLEND1_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB1_BLEND0_ADDITIVE);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB1_BLEND1_ADDITIVE);

// --- 0555 格式 (TEX1 only) ---
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA0_MOD0_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA0_MOD0_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA0_MOD0_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA0_MOD0_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA0_MOD1_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA0_MOD1_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA0_MOD1_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA0_MOD1_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA1_MOD0_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA1_MOD0_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA1_MOD0_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA1_MOD0_GLOB1_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA1_MOD1_GLOB0_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA1_MOD1_GLOB0_BLEND1);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA1_MOD1_GLOB1_BLEND0);
tod_draw_triangle_variant!(TodDrawTriangle_0555_TEX1_TALPHA1_MOD1_GLOB1_BLEND1);

// ============================================================
// 所有 DrawTriangle_* 变体（无 Tod 前缀，TEX0 和 TEX1）
// ============================================================

// Format: 8888
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA0_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA0_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA0_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA0_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA0_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA0_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA0_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA0_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA1_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA1_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA1_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA1_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA1_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA1_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA1_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX0_TALPHA1_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA0_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA0_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA0_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA0_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA0_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA0_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA0_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA0_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA1_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA1_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA1_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA1_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA1_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA1_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA1_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_8888_TEX1_TALPHA1_MOD1_GLOB1_BLEND1);

// Format: 0888
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA0_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA0_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA0_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA0_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA0_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA0_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA0_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA0_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA1_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA1_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA1_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA1_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA1_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA1_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA1_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX0_TALPHA1_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA0_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA0_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA0_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA0_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA0_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA1_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0888_TEX1_TALPHA1_MOD1_GLOB1_BLEND1);

// Format: 0565
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA0_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA0_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA0_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA0_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA0_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA0_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA0_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA0_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA1_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA1_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA1_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA1_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA1_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA1_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA1_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX0_TALPHA1_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA0_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA0_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA0_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA0_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA0_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA1_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0565_TEX1_TALPHA1_MOD1_GLOB1_BLEND1);

// Format: 0555
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA0_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA0_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA0_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA0_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA0_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA0_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA0_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA0_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA1_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA1_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA1_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA1_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA1_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA1_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA1_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX0_TALPHA1_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA0_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA0_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA0_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA0_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA0_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA0_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA0_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA0_MOD1_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA1_MOD0_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA1_MOD0_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA1_MOD0_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA1_MOD0_GLOB1_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA1_MOD1_GLOB0_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA1_MOD1_GLOB0_BLEND1);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA1_MOD1_GLOB1_BLEND0);
draw_triangle_variant!(DrawTriangle_0555_TEX1_TALPHA1_MOD1_GLOB1_BLEND1);
