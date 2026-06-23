// PvZ Portable Rust 翻译 — SexyMatrix 类型
// 对应 C++ SexyAppFramework/misc/SexyMatrix.h / SexyMatrix.cpp

#![allow(dead_code)]

/// 3x3 矩阵（用于 2D 变换）
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SexyMatrix3 {
    pub m: [[f32; 3]; 3],
}

impl SexyMatrix3 {
    pub fn identity() -> Self {
        SexyMatrix3 {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn new_from_values(
        m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32,
    ) -> Self {
        SexyMatrix3 {
            m: [
                [m00, m01, m02],
                [m10, m11, m12],
                [m20, m21, m22],
            ],
        }
    }

    /// 创建平移矩阵
    pub fn translation(tx: f32, ty: f32) -> Self {
        SexyMatrix3 {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [tx, ty, 1.0],
            ],
        }
    }

    /// 创建缩放矩阵
    pub fn scaling(sx: f32, sy: f32) -> Self {
        SexyMatrix3 {
            m: [
                [sx, 0.0, 0.0],
                [0.0, sy, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    /// 创建旋转矩阵（弧度）
    pub fn rotation(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        SexyMatrix3 {
            m: [
                [c, s, 0.0],
                [-s, c, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    /// 矩阵乘法
    pub fn multiply(&self, other: &SexyMatrix3) -> SexyMatrix3 {
        let mut result = SexyMatrix3::identity();
        for i in 0..3 {
            for j in 0..3 {
                result.m[i][j] = self.m[i][0] * other.m[0][j]
                    + self.m[i][1] * other.m[1][j]
                    + self.m[i][2] * other.m[2][j];
            }
        }
        result
    }

    /// 变换一个 2D 点
    pub fn transform(&self, x: f32, y: f32) -> (f32, f32) {
        let tx = x * self.m[0][0] + y * self.m[1][0] + self.m[2][0];
        let ty = x * self.m[0][1] + y * self.m[1][1] + self.m[2][1];
        (tx, ty)
    }

    /// 转置
    pub fn transpose(&self) -> SexyMatrix3 {
        let mut result = SexyMatrix3::identity();
        for i in 0..3 {
            for j in 0..3 {
                result.m[i][j] = self.m[j][i];
            }
        }
        result
    }

    /// 获取底层数组（用于传递给 OpenGL）
    pub fn as_float_array(&self) -> &[f32; 9] {
        unsafe { &*(self as *const SexyMatrix3 as *const [f32; 9]) }
    }
}

impl Default for SexyMatrix3 {
    fn default() -> Self {
        SexyMatrix3::identity()
    }
}

/// 4x4 矩阵（用于 3D/投影变换）
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SexyMatrix4 {
    pub m: [[f32; 4]; 4],
}

impl SexyMatrix4 {
    pub fn identity() -> Self {
        SexyMatrix4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// 正交投影矩阵（对应 GLInterface.cpp 中的 MakeOrthoMatrix）
    pub fn ortho(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Self {
        SexyMatrix4 {
            m: [
                [2.0 / (r - l), 0.0, 0.0, 0.0],
                [0.0, 2.0 / (t - b), 0.0, 0.0],
                [0.0, 0.0, -2.0 / (f - n), 0.0],
                [-(r + l) / (r - l), -(t + b) / (t - b), -(f + n) / (f - n), 1.0],
            ],
        }
    }

    /// 获取 OpenGL 列主序 float16 数组
    pub fn as_col_major_array(&self) -> [f32; 16] {
        let mut arr = [0.0f32; 16];
        for i in 0..4 {
            for j in 0..4 {
                arr[j * 4 + i] = self.m[i][j];
            }
        }
        arr
    }
}

impl Default for SexyMatrix4 {
    fn default() -> Self {
        SexyMatrix4::identity()
    }
}
