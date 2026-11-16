# 02 · 类型与惯用法映射

> 规则：**同一种 C++ 写法，在整个项目中只允许一种 Rust 译法。** 表中没有的情况，先问人，再把答案补进本表。

## 1. 基本类型

| C++ | Rust | 说明 |
|---|---|---|
| `int` / `unsigned int` | `i32` / `u32` | 位宽不变。不要用 `usize` 代替，除非确实是容器索引 |
| `short` / `unsigned short` | `i16` / `u16` | |
| `char` / `unsigned char` | `i8`·`u8`（按语义） | 字节数据用 `u8` |
| `long` | 【待核对：以原代码在目标平台含义为准，通常按 `i32`】 | 遇到先问 |
| `size_t` | `usize` | 仅用于大小/索引 |
| `bool` | `bool` | |
| `float` | `f32` | **不得改为 `f64`** |
| `double` | `f64` | |
| 枚举（会被存档/比较数值） | `#[repr(i32)] enum` + 显式判别值 | 判别值必须与 C++ 一致；转换用 `TryFrom<i32>`，非法值处理见 `06` |
| 位标志 | `bitflags!` 或 `u32` + 常量 | 位值与 C++ 一致 |

## 2. 容器与字符串

| C++ | Rust | 说明 |
|---|---|---|
| `std::vector<T>` | `Vec<T>` | |
| `std::array<T,N>` / `T a[N]` | `[T; N]` | 下标越界：原代码会 UB 的，见 `06` |
| `std::list<T>` | `Vec<T>` 或 `VecDeque<T>` | 注意迭代时增删的语义；不确定先问 |
| `std::map<K,V>` | `BTreeMap<K,V>` | **保持有序遍历** |
| `std::unordered_map` | `HashMap`，**且遍历顺序影响逻辑时改用有序结构并登记** | 遍历顺序可能影响行为 |
| `std::string` / `SexyString` | `String`（UTF-8） | 内部统一 UTF-8；原始字节用 `Vec<u8>` |
| `const char*` | `&str` 或 `&[u8]` | 按语义区分 |
| `std::pair` / `std::tuple` | 元组 | |
| `std::optional<T>` | `Option<T>` | |

## 3. 对象与引用（以当前代码实际做法为准）

当前仓库采用"C++ 同构"风格，沿用以下约定，**新翻译的代码必须与已有代码保持一致，不得另起一套**：

| C++ | 当前 Rust 做法 | 说明 |
|---|---|---|
| 非拥有指针 `T*` | `*mut T` / `*const T`，使用处 `unsafe { &mut *p }` | 与现有代码一致；新增使用须带 `// SAFETY:` |
| `new T` / `delete` | `Box::into_raw` / `Box::from_raw` | **释放时机必须与 C++ 一致；Widget 的删除用延迟释放（见 `03` §3）** |
| 对象池 `DataArray<T>` | `todlib/data_array.rs` 统一实现 | 所有对象池复用；ID 编码规则与 C++ 一致 |
| 全局指针/单例 | 现有 `static mut`（存量） | **禁止新增** |
| 虚函数（Widget 等） | `Widget { impl_: Option<Box<dyn …>> }` 手写分派 | 见 `framework/widget/widget.rs`、`main.rs` 中的说明 |
| `shared_ptr<T>` | 先问人 | 默认不允许 |

> 长期方向（把裸指针改为 ID/借用）是**阶段 2** 的事，见 ADR-003；在"行为等价"达成前不做。

## 4. 继承与多态

| C++ | Rust |
|---|---|
| 封闭集合的继承（如固定种类） | `enum` + `match` |
| 开放集合的虚函数（如 `Widget` 体系） | `trait` + `Box<dyn Trait>`，具体方案见 ADR |
| 基类里的通用字段 | 组合：`struct Base { … }` 作为字段 |
| `virtual` 默认实现 | trait 默认方法 |
| `dynamic_cast` / `static_cast` 到子类 | `match` / 枚举判别；不得使用 `Any` 乱转 |

## 5. 命名转换（Sexy 风格 → Rust）

| C++ | Rust |
|---|---|
| 类名 `Plant` | `Plant` |
| 成员 `mSunMoney` | `sun_money`（去掉 `m` 前缀） |
| 参数 `theCount` | `count`（去掉 `the`） |
| 局部变量 `aIndex` | `index`（去掉 `a`） |
| 函数 `UpdateShooter` | `update_shooter` |
| 常量/枚举值 `PLANT_PEASHOOTER` | `PlantType::Peashooter`（枚举）/ `PLANT_PEASHOOTER`（纯常量） |
| 冲突（去前缀后与关键字或其他名重名） | 加后缀 `_`，并登记在 `modules/<module>.md` |

## 6. 数值与运算

| C++ | Rust |
|---|---|
| `(int)f`（float→int 截断） | `f as i32`（同样向零截断；NaN→0，溢出饱和，**与 C++ 溢出时 UB 不同，见 `06`**） |
| `a / b`、`a % b`（整数） | `a / b`、`a % b`（均向零截断；除零 Rust 会 panic，见 `06`） |
| 依赖回绕的 `int` 运算 | `wrapping_add` 等，**显式写出** |
| `a << n`（n 可能 ≥ 位宽） | 先问；Rust debug 下会 panic |
| `std::abs / min / max` | `abs` / `min` / `max`（注意 `f32::min/max` 对 NaN 的处理） |
| `sinf / cosf / sqrtf / atan2f` | 见 ADR-004（数学函数确定性） |

## 7. 示例（仅示意风格，函数为虚构）

C++：

```cpp
void Plant::UpdateShooter()
{
    mLaunchCounter--;
    if (mLaunchCounter <= 0)
    {
        mLaunchCounter = mLaunchRate;
        Fire(nullptr, mRow, PlantWeapon::WEAPON_PRIMARY);
    }
}
```

Rust（正确）：

```rust
// Ported from: src/Lawn/Plant.cpp:L123-131 （示例，行号为虚构）
pub fn update_shooter(&mut self, ctx: &mut Ctx) {
    self.launch_counter -= 1;
    if self.launch_counter <= 0 {
        self.launch_counter = self.launch_rate;
        self.fire(ctx, None, self.row, PlantWeapon::Primary);
    }
}
```

Rust（错误示范）：

```rust
// ❌ 合并了判断；❌ 用 saturating_sub 改变了原语义；❌ 没有来源标注
pub fn update_shooter(&mut self, ctx: &mut Ctx) {
    self.launch_counter = self.launch_counter.saturating_sub(1);
    if self.launch_counter == 0 { /* ... */ }
}
```
