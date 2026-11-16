# modules/ 目录说明

为每个体量大或逻辑复杂的模块建一个文件 `docs/modules/<module>.md`，建议包含：

1. 对应的 C++ 文件与行数规模
2. 公共接口与被谁调用
3. 函数级对照表（C++ 函数 → Rust 函数 → 状态）
4. 该模块特有的陷阱（RNG 消耗点、遍历顺序、UB 位置、命名冲突）
5. 该模块的验收方式（单元测试 / golden / 回放字段）

优先为这些模块建立专项文档：`Board`、`Zombie`、`Plant`、`Challenge`、`ZenGarden`、`Reanimation`、`DataArray`、`ResourceManager`、`Widget 体系`。
