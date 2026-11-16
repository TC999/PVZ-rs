# 00 · 项目总纲

## 1. 目标

将 PvZ-Portable（C++20，基于 SexyAppFramework）完整翻译为 Rust，**行为等价**：

- 相同输入序列 + 相同随机数种子 → 相同的每帧游戏状态。
- 能读写与 C++ 版互通的存档（`user*.dat`、`*.v4`）。
- 能加载与 C++ 版相同的原版资源（`main.pak`、`properties/`）。

**非目标：** 重新设计架构、引入 ECS、改善玩法、修复"bug"、增加新功能、提高可读性而改变行为。

## 2. 上游

| 项目 | 地址 | 许可 |
|---|---|---|
| C++ 参照 | https://github.com/wszqkzqk/PvZ-Portable | LGPL-3.0-or-later；SexyAppFramework 部分另含 PopCap Games Framework License |
| Rust 翻译 | https://github.com/TC999/PVZ-rs | GPL-3.0（README 声明） |

本仓库为上游的衍生作品：必须保留版权声明、使用兼容许可证（LGPL-3.0-or-later 或兼容）、在翻译文件头标注来源。

## 3. 翻译依据（合规）

- **唯一依据：** 本仓库 `cpp/` 目录下的 C++ 源码（只读，不得修改）。
- 上游作者明确**不接受通过逆向工程得到的代码**。因此：不得引入内存地址表、反汇编结果、其他逆向项目的代码作为翻译依据。
- **不得提交任何原版素材**（`main.pak`、图片、音频、字体等）。用户需自备合法副本。

## 4. 范围

| 范围 | 内容 |
|---|---|
| ✅ 翻译 | `cpp/src/` 下的 `Lawn/`、`PvzpLib/`、`SexyAppFramework/`、`LawnApp.*`、`ConstEnums.h`、`Resources.*`、`main.cpp` |
| ✅ 翻译 | 存档读写、资源加载、`LawnStrings` / `default.xml` / `Layout.xml` 加载、录制回放（`.dmo`） |
| 🔶 另行决策 | 渲染（GLES2/GL2.1）、音频（含 MO3）、窗口输入：见 `05_DECISIONS.md` |
| ❌ 不翻译 | `android/ ios/ wasm/ archlinux/ CMake/ .github/` 及各平台打包脚本 |
| ❌ 不翻译 | `scripts/pvzp-v4-converter.py` 等 Python 工具：保留原样，用作验证工具 |

## 4.1 项目现状

引擎主体已翻译并能编译（`cargo check` 0 error / ~750 warnings），但**未做运行期验证**，且存在骨架化函数与未接线代码（详见 `ANALYSIS_REPORT.md`）。因此里程碑不再从零开始，而是"**先让现有代码可运行、可验证，再补缺口**"。

## 5. 里程碑与验收

| 阶段 | 内容 | 验收方式 |
|---|---|---|
| M0 | 基础库核对：数学、矩形/点、MTRand、`DataArray`（已翻译，需补测试） | 黄金值测试 + 与 C++ 输出对照 |
| M1 | 资源：`.pak` 读取、XML/properties 解析、`LawnStrings` | 资源条目数、字符串条目数与 C++ 一致 |
| M2 | 存档：`user*.dat`、`.v4` | 与 `pvzp-v4-converter.py` 导出的 YAML 逐项一致 |
| M3 | 动画系统：Reanimation / Trail / Particle | 固定输入下的状态哈希对照 |
| M4 | 游戏核心（无渲染）：Board / Plant / Zombie / Projectile / Coin / 各模式 | `.dmo` 回放：每 N 帧状态哈希与 C++ 一致 |
| M5 | UI 与屏幕：标题、菜单、图鉴、禅境花园 | 手工 + 截图对照 |
| M6 | 渲染、音频、输入接入 | 可实际游玩，`.dmo` 回放画面一致 |

**不允许跳级：** 上一层未通过验收，不得在其上继续翻译。

## 6. 术语

- **等价**：见 `06_BEHAVIOR_SPEC.md`。
- **回放哈希**：`.dmo` 回放过程中每 N 帧导出的游戏状态摘要（含 RNG 状态）。
- **fix_bugs**：对应 C++ 的 `DO_FIX_BUGS`；**debug**：对应 `PVZ_DEBUG`。
