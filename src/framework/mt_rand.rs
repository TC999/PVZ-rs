// PvZ Portable Rust 翻译 — MTRand（梅森旋转随机数生成器）
// 对应 C++ SexyAppFramework/misc/MTRand.h / MTRand.cpp
//
// 移植自 Makoto Matsumoto 和 Takuji Nishimura 的 MT19937 算法。
// 必须确保种子序列与 C++ 实现完全一致，因为游戏需要确定性的关卡生成。

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

const MTRAND_N: usize = 624;
const MTRAND_M: usize = 397;
const MATRIX_A: u32 = 0x9908b0df;
const UPPER_MASK: u32 = 0x80000000;
const LOWER_MASK: u32 = 0x7fffffff;
const TEMPERING_MASK_B: u32 = 0x9d2c5680;
const TEMPERING_MASK_C: u32 = 0xefc60000;

/// 梅森旋转随机数生成器（对应 C++ MTRand）
pub struct MTRand {
    /// 状态向量
    mt: [u32; MTRAND_N],
    /// 当前索引（mti == MTRAND_N+1 表示未初始化）
    mti: usize,
}

/// 全局随机允许标志
/// 对应 C++ 中的 gRandAllowed（用于调试断言）
static mut G_RAND_ALLOWED: i32 = 0;

impl MTRand {
    /// 从字符串数据构造（对应 C++ MTRand(const std::string&)）
    pub fn from_serial(data: &[u8]) -> Self {
        let mut rng = MTRand {
            mt: [0u32; MTRAND_N],
            mti: MTRAND_N + 1,
        };
        rng.srand_serial(data);
        rng
    }

    /// 从种子构造（对应 C++ MTRand(unsigned long seed)）
    pub fn from_seed(seed: u32) -> Self {
        let mut rng = MTRand {
            mt: [0u32; MTRAND_N],
            mti: MTRAND_N + 1,
        };
        rng.srand(seed);
        rng
    }

    /// 默认构造（对应 C++ MTRand()，种子 4357）
    pub fn new() -> Self {
        let mut rng = MTRand {
            mt: [0u32; MTRAND_N],
            mti: MTRAND_N + 1,
        };
        rng.srand(4357);
        rng
    }

    /// 设置随机允许标志（对应 C++ SetRandAllowed）
    pub fn set_rand_allowed(allowed: bool) {
        unsafe {
            if allowed {
                if G_RAND_ALLOWED > 0 {
                    G_RAND_ALLOWED -= 1;
                }
            } else {
                G_RAND_ALLOWED += 1;
            }
        }
    }

    /// 从字符串数据设置种子（对应 C++ SRand(const std::string&)）
    pub fn srand_serial(&mut self, data: &[u8]) {
        if data.len() == MTRAND_N * 4 {
            // 从字节数据中恢复状态
            for i in 0..MTRAND_N {
                let base = i * 4;
                if base + 3 < data.len() {
                    self.mt[i] = u32::from_le_bytes([
                        data[base],
                        data[base + 1],
                        data[base + 2],
                        data[base + 3],
                    ]);
                }
            }
        } else {
            self.srand(4357);
        }
    }

    /// 从种子设置状态（对应 C++ SRand(unsigned long seed)）
    /// 使用 Knuth TAOCP Vol2 的初始化算法
    pub fn srand(&mut self, seed: u32) {
        let mut s = seed;
        if s == 0 {
            s = 4357;
        }
        self.mt[0] = s & 0xffffffff;
        for i in 1..MTRAND_N {
            // 注意：Rust 中乘法可能溢出，需要 wrapping 行为
            let prev = self.mt[i - 1];
            self.mt[i] = 1812433253u32.wrapping_mul(prev ^ (prev >> 30)).wrapping_add(i as u32);
            self.mt[i] &= 0xffffffff;
        }
        self.mti = MTRAND_N; // 设为 N 以在首次 Next 时触发重新生成
    }

    /// 获取下一个随机数（带断言检查，对应 C++ Next）
    pub fn next(&mut self) -> u32 {
        // DBG_ASSERT(gRandAllowed == 0)
        self.next_no_assert()
    }

    /// 获取下一个随机数（无断言，对应 C++ NextNoAssert）
    pub fn next_no_assert(&mut self) -> u32 {
        let mag01: [u32; 2] = [0x0, MATRIX_A];

        if self.mti >= MTRAND_N {
            for kk in 0..(MTRAND_N - MTRAND_M) {
                let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
                self.mt[kk] = self.mt[kk + MTRAND_M] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];
            }
            for kk in (MTRAND_N - MTRAND_M)..(MTRAND_N - 1) {
                let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
                self.mt[kk] = self.mt[kk + (MTRAND_M - MTRAND_N)] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];
            }
            let y = (self.mt[MTRAND_N - 1] & UPPER_MASK) | (self.mt[0] & LOWER_MASK);
            self.mt[MTRAND_N - 1] = self.mt[MTRAND_M - 1] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];
            self.mti = 0;
        }

        let mut y = self.mt[self.mti];
        self.mti += 1;

        // 调温操作（对应 C++ 宏）
        y ^= y >> 11;
        y ^= (y << 7) & TEMPERING_MASK_B;
        y ^= (y << 15) & TEMPERING_MASK_C;
        y ^= y >> 18;

        y &= 0x7FFFFFFF;
        y
    }

    /// 获取 [0, range) 范围内的随机整数（无断言，对应 C++ NextNoAssert(unsigned long)）
    pub fn next_no_assert_range(&mut self, range: u32) -> u32 {
        if range == 0 {
            return 0;
        }
        self.next_no_assert() % range
    }

    /// 获取 [0, range) 范围内的随机整数（带断言，对应 C++ Next(unsigned long)）
    pub fn next_range(&mut self, range: u32) -> u32 {
        self.next_no_assert_range(range)
    }

    /// 获取 [0, range) 范围内的随机浮点数（无断言，对应 C++ NextNoAssert(float)）
    pub fn next_no_assert_float(&mut self, range: f32) -> f32 {
        (self.next_no_assert() as f64 / 0x7FFFFFFF as f64) as f32 * range
    }

    /// 获取 [0, range) 范围内的随机浮点数（带断言，对应 C++ Next(float)）
    pub fn next_float(&mut self, range: f32) -> f32 {
        self.next_no_assert_float(range)
    }

    /// 序列化状态（对应 C++ Serialize）
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(MTRAND_N * 4);
        for &val in self.mt.iter() {
            data.extend_from_slice(&val.to_le_bytes());
        }
        data
    }
}

impl Default for MTRand {
    fn default() -> Self {
        MTRand::new()
    }
}

/// MTAutoDisallowRand（对应 C++ MTAutoDisallowRand）
/// 在作用域内禁止随机数生成
pub struct MTAutoDisallowRand;

impl MTAutoDisallowRand {
    pub fn new() -> Self {
        MTRand::set_rand_allowed(false);
        MTAutoDisallowRand
    }
}

impl Drop for MTAutoDisallowRand {
    fn drop(&mut self) {
        MTRand::set_rand_allowed(true);
    }
}
