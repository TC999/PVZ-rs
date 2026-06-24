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

/// 2D 仿射变换（对应 C++ Sexy::Transform）
/// 用于简化变换组合：平移+旋转+缩放，支持延迟计算矩阵
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Transform {
    /// 内部矩阵（延迟计算）
    matrix: SexyMatrix3,
    /// 是否需要重新计算矩阵
    need_calc_matrix: bool,
    /// 是否为复杂变换（无法分解为平移/旋转/缩放的组合）
    pub complex: bool,
    /// 是否有旋转
    pub have_rot: bool,
    /// 是否有缩放
    pub have_scale: bool,
    /// 第一次平移
    pub trans_x1: f32,
    pub trans_y1: f32,
    /// 第二次平移
    pub trans_x2: f32,
    pub trans_y2: f32,
    /// 缩放
    pub scale_x: f32,
    pub scale_y: f32,
    /// 旋转弧度
    pub rot: f32,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            matrix: SexyMatrix3::identity(),
            need_calc_matrix: false,
            complex: false,
            have_rot: false,
            have_scale: false,
            trans_x1: 0.0,
            trans_y1: 0.0,
            trans_x2: 0.0,
            trans_y2: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rot: 0.0,
        }
    }

    /// 重置为单位变换
    pub fn reset(&mut self) {
        self.complex = false;
        self.have_rot = false;
        self.have_scale = false;
        self.trans_x1 = 0.0;
        self.trans_y1 = 0.0;
        self.trans_x2 = 0.0;
        self.trans_y2 = 0.0;
        self.scale_x = 1.0;
        self.scale_y = 1.0;
        self.rot = 0.0;
        self.need_calc_matrix = false;
        self.matrix = SexyMatrix3::identity();
    }

    /// 平移
    pub fn translate(&mut self, tx: f32, ty: f32) {
        self.trans_x2 += tx;
        self.trans_y2 += ty;
        self.need_calc_matrix = true;
    }

    /// 旋转（弧度）
    pub fn rotate_rad(&mut self, rot: f32) {
        if rot != 0.0 {
            if self.have_rot || self.have_scale {
                self.make_complex();
            }
            self.have_rot = true;
            self.rot += rot;
            self.need_calc_matrix = true;
        }
    }

    /// 旋转（角度）
    pub fn rotate_deg(&mut self, rot: f32) {
        self.rotate_rad(rot * std::f32::consts::PI / 180.0);
    }

    /// 缩放
    pub fn scale(&mut self, sx: f32, sy: f32) {
        if sx != 1.0 || sy != 1.0 {
            if self.have_rot || self.have_scale {
                self.make_complex();
            }
            self.have_scale = true;
            self.scale_x *= sx;
            self.scale_y *= sy;
            self.need_calc_matrix = true;
        }
    }

    /// 标记为复杂变换（需要完整矩阵计算）
    fn make_complex(&mut self) {
        if self.complex {
            return;
        }
        self.complex = true;
        // 将当前变换合并到矩阵中
        self.calc_matrix_internal();
    }

    /// 计算最终矩阵
    pub fn get_matrix(&self) -> SexyMatrix3 {
        // 注意：由于 get_matrix 在理论上只需要不可变引用，
        // 但需要延迟计算矩阵，这里通过 unsafe 实现内部可变性
        // 这对应 C++ 中 mutable 关键字的行为
        if self.need_calc_matrix {
            let self_mut = self as *const Transform as *mut Transform;
            unsafe {
                (*self_mut).calc_matrix_internal();
            }
        }
        self.matrix
    }

    fn calc_matrix_internal(&mut self) {
        if self.complex {
            // 使用完整矩阵计算（平移 × 旋转 × 缩放 × 平移）
            let mut m = SexyMatrix3::translation(self.trans_x1, self.trans_y1);
            if self.have_rot {
                m = m.multiply(&SexyMatrix3::rotation(self.rot));
            }
            if self.have_scale {
                m = m.multiply(&SexyMatrix3::scaling(self.scale_x, self.scale_y));
            }
            let t2 = SexyMatrix3::translation(self.trans_x2, self.trans_y2);
            self.matrix = m.multiply(&t2);
        } else {
            // 简单变换不需要矩阵
            self.matrix = SexyMatrix3::identity();
        }
        self.need_calc_matrix = false;
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform::new()
    }
}
