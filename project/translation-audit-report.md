# PVZ-rs ↔ C++ 翻译一致性核查报告

> 基线：HEAD `5ceba1f`（`DataArray ID 体系：Coin::coin_id 接线`，2026-09-15）。
> 对照源：`cpp/src/**`（194270 行）↔ `src/**`（93521 行）。
> 性质：**只读静态对照**；除覆写本报告外未修改任何源码。
> 方法：C++ `Class::Method` 抽取 ↔ Rust `fn` 归一化匹配（忽略 `_`/大小写）；枚举/常量逐项对账；全库占位标记扫描；行数比；消费点接线核验；`todlib/**`+`framework/**` 全量逐标注回查 C++。
> 说明：本文件为 `project/` 下唯一报告。本轮在 `d67d8fb` 基础上推进至 `5ceba1f`，并对 `todlib/**`（68 处标注）与 `framework/**`（29 处标注）做了**全量逐条回查**，修正了大量过时标注。

---

## 结论速览

1. **不存在整函数级缺失**，全库 `todo!` / `unimplemented!` 计数为 **0**——翻译在函数面上是齐的。
2. **旧报告 §2 列举的硬差异已全部修复**（详见 §2）：`NUM_SEED_TYPES`=53、`NUM_CACHED_ZOMBIE_TYPES`=35、`ZombieType::CachedPolevaulterWithPole` 已补入并接线、定义表项数一致。
3. **本轮全量回查 `todlib/**`+`framework/**` 后，原报告 §3.1/§3.5 列举的子系统级缺口大量已过时**：Reanimator 核心函数（SetFramesForLayer/StartBlend/SetShakeOverride/AssignRenderGroupToPrefix）、overlay 矩阵全分量、attachment Trail 分支、`DefinitionLoadXML`（trail/particle）、FilterEffect 字段、`mReanimAtlas`、Coin::coin_id（DataArray ID 体系）**均已实现并接线**。
4. **当前残留的真实硬逻辑缺口收敛为 3 项**（全部位于 `todlib/**`）+ 2 项枚举变体缺失，详见 §3。

| 维度 | 现状 |
|---|---|
| `todo!` / `unimplemented!` | **0** |
| `todlib/**` 标注 | 68 处（本轮全量回查：约 60% 已过时） |
| `framework/**` 标注 | 29 处（本轮全量回查：均为合理近似/说明性注释） |
| C++ 总行数 | 194270（.cpp/.h） |
| Rust 总行数 | 93521（.rs） |
| 已核实硬逻辑缺失 | **3 项**（均在 todlib，见 §3） |
| 已核实枚举变体缺失 | **2 项**（ReanimLoopType，见 §3） |

---

## 一、函数级覆盖：核心 lawn 模块无整函数缺失

脚本（`Class::Method` ↔ `fn` 归一化匹配）报出的"候选缺失"经核实**全部有对应实现，只是命名不同**，不构成缺失。典型映射：

| C++ | Rust 对应 |
|---|---|
| `Board::AddACrater` / `AddALadder` / `AddAGraveStone` | `src/lawn/board.rs` `add_crater` / `add_ladder` / `add_grave_stone` |
| `Zombie::FindPlantTarget` / `IsImmobilizied` | `src/lawn/zombie.rs` `find_plant_target_index` / `is_immobilized` |
| `Plant::GetFreeMagnetItem` / `MagnetShroomAttactItem` | `src/lawn/plant.rs` `get_free_magnet_item_idx` / `magnet_shroom_attack_item` |
| `Projectile::DoImpact` | `src/lawn/projectile.rs` `do_impact_by_index` |
| `Coin::IsSun` | `src/lawn/coin.rs` `is_sun()` |
| `GridItem::DrawGridItem` | `src/lawn/grid_item.rs` `draw` |

> 同名构造函数/析构函数（`Board::Board`、`~Zombie` 等）不计入缺失，Rust 侧以 `new`/`Drop` 承载。

---

## 二、旧报告硬差异核查（已全部修复）

> 旧报告（基线 `496dd4a`）声称 4 项硬差异。逐条核验**全部已修复**：

### 2.1 `NUM_SEED_TYPES` —— 已对齐 ✅
- Rust：`src/lawn/game_enums.rs:1972` → `pub const NUM_SEED_TYPES: usize = 53`
- C++：`cpp/src/ConstEnums.h:1085-1090` → `SEED_IMITATER = 48`、`SEED_LEFTPEATER` → `NUM_SEED_TYPES`（= 53）

### 2.2 `NUM_CACHED_ZOMBIE_TYPES` —— 已对齐 ✅
- Rust：`src/lawn/game_enums.rs:1975` → `pub const NUM_CACHED_ZOMBIE_TYPES: usize = 35`
- C++：`cpp/src/ConstEnums.h:1376-1378` → `NUM_ZOMBIE_TYPES(33)` → `ZOMBIE_CACHED_POLEVAULTER_WITH_POLE(34)` → `NUM_CACHED_ZOMBIE_TYPES(35)`

### 2.3 `ZombieType::CachedPolevaulterWithPole` —— 已补入并接线 ✅
- Rust `src/lawn/game_enums.rs:400` 已含该变体。消费点已全部接线：`zombie.rs:773`、`reanimation_lawn.rs:311/200`、`almanac_dialog.rs:229`。

### 2.4 定义表项数 —— 一致 ✅
- `gZombieDefs`：Rust 33 项，与 C++ `NUM_ZOMBIE_TYPES(33)` 一致。
- `gPlantDefs`：Rust 53 项，与 C++ `NUM_SEED_TYPES(53)` 一致。

---

## 三、当前残留硬逻辑缺口（本轮全量回查结论）

> 本轮对 `todlib/**`（68 处标注）与 `framework/**`（29 处标注）做了**逐条回查 C++ 源码**。原报告 §3.1/§3.5 列举的子系统级缺口中，**约 60% 已过时**（对应函数已实现但标注/报告未更新）。真实残留缺口收敛如下。

### 3.1 ★ 粒子轨道求值曲线丢失（最高优先级）— `tod_particle.rs:216-243`

`FloatParameterTrack::evaluate` 是 C++ `FloatTrackEvaluate`（`Definition.cpp:1336-1359`）的对应物，但实现严重简化，**丢失双层曲线求值**：

| 维度 | C++ `FloatTrackEvaluate` | Rust `evaluate` |
|---|---|---|
| 段间插值 | 双层曲线：左右节点各经 `PvzpCurveEvaluate(interp, low, high, distribution)` 求值得 `aLeftValue`/`aRightValue`，再 `PvzpCurveEvaluate(fraction, left, right, curveType)` | 纯线性 `low + (next.low - low)*t` |
| `curve_type` 字段 | 用于段间过渡（EASE_IN/OUT/BOUNCE/SIN_WAVE 等 13 种） | **完全忽略**（字段存在于 `FloatParameterTrackNode:173`，XML 已解析至 `curve_type`，但 `evaluate` 未使用） |
| `distribution` 字段 | 用于节点内 min/max 范围采样 | **完全忽略**（同上） |
| `interp`/`_rand` 参数 | C++ 传入粒子插值随机数 | Rust 形参 `_rand` 被丢弃 |
| 单节点分支 | `PvzpCurveEvaluate(interp, low, high, distribution)` | 直接返回 `low_value` |
| 超出末节点 | 对末节点 distribution 求值 | 落入 `idx=len-2` 分支近似 |

**根因**：C++ `PvzpCurveEvaluate`（`PvzpCommon.cpp:336-357`，13 种曲线：CONSTANT/LINEAR/EASE_IN/EASE_OUT/EASE_IN_OUT/EASE_IN_OUT_WEAK/FAST_IN_OUT/FAST_IN_OUT_WEAK/BOUNCE/BOUNCE_FAST_MIDDLE/BOUNCE_SLOW_MIDDLE/SIN_WAVE/EASE_SIN_WAVE）在 **Rust 全库无对应实现**（`rg "curve_evaluate|CurveEvaluate|PvzpCurve" src/` 无命中）。

**影响**：所有粒子动画的缓动曲线（弹跳、正弦、缓入缓出等）退化为线性；`high_value` 分布范围不生效。

### 3.2 字符串本地化占位符未替换 — `tod_common.rs:136-140`

- C++ `PvzpStringTranslate`（`PvzpStringFile.cpp:174-182`）：检测 `[name]` 格式 → `PvzpStringListFind` 查 `mStringProperties` 表。
- Rust `tod_string_translate`：直接返回原文，**未实现 `[...]` 查表逻辑**。
- **影响**：`[LEVEL]`、`[UPGRADE_DIALOG_BODY]`、`[ZOMBIQUARIUM_DEATH_MESSAGE]`、`[LAST_STAND_DEATH_MESSAGE]` 等本地化占位符（`cutscene.rs:1271`、`challenge.rs:3435`、`board.rs:3563/7621/7630/7635/7639`、`game_button.rs:228` 共 8+ 调用点）**不会替换为实际文案**，显示原始 `[...]` 字面量。
- 注：基础设施已就绪——`sexy_app_base.rs:116` 有 `m_string_properties: HashMap<String,String>` + `get_string`（行 1070），仅 `tod_string_translate` 未接线。

### 3.3 拟音实例级音量未应用 — `tod_foley.rs:350-354`

- C++ `ApplyMusicVolume`（`PvzpFoley.cpp:384-385`）：`mInstance->SetVolume(mMusicVolume / mSfxVolume)`。
- Rust `apply_music_volume`：空操作（`let _ = foley_instance;`）。
- **影响**：`FOLEYFLAGS_USES_MUSIC_VOLUME` 标记的拟音音量不随音乐音量调整。

### 3.4 枚举变体缺失 — `reanimator.rs`

- `reanimator.rs:155`：C++ `REANIM_PLAY_ONCE_AND_RETURN_TO_ZERO` 在 Rust `ReanimLoopType` 无对应。
- `reanimator.rs:950`：C++ `REANIM_LOOP_FULL_LAST_FRAME` 在 Rust `ReanimLoopType` 无对应。

> 注：这两项已由标注记录，消费点以默认行为处理，影响有限。

---

## 四、本轮修正的过时标注（原报告 §3.1/§3.5 已过时项）

> 以下原报告列为"缺失/未接入"，经全量回查 C++ 源码后确认**均已实现**，相应条目应删除。

### 4.1 Reanimator 系统（原报告 §3.1）—— 全部已实现 ✅

| 原报告声称 | 实际核查 | 证据 |
|---|---|---|
| `SetFramesForLayer`/`SetTruncateDisappearingFrames`/`AssignRenderGroupToPrefix`/`StartBlend`/`SetShakeOverride` 为 stub | **已实现**，非空 stub | `reanimator.rs:663 set_frames_for_layer`、`:770 start_blend`、`:833 assign_render_group_to_prefix`、`:1123 set_shake_override`、`:1130 set_truncate_disappearing_frames` 均有完整逻辑 |
| overlay 矩阵仅取平移分量 `m[0][2]`/`m[1][2]` | **完整 6 分量** | `reanimator.rs:1105-1119 matrix_from_transform`：m00/m01/m02/m10/m11/m12 含 skew×scale；`draw_render_group` 矩阵链 pivot→MatrixFromTransform→×mOverlayMatrix→平移(shake+g) 完整 |
| `font`/文本轨道未解析（`reanim_loader.rs:230`，暂存 `extra_int`） | **已解析** | `parse_transform_node:263-273` 解析 font（`resolve_reanim_font_name`）、text（存 `track_texts` 表存索引）；`fill_in_missing_data:300-302` 实现 font/text 继承 |
| `mReanimAtlas` 为 stub | **已实现** | `reanim_atlas.rs`：`get_encoded_reanim_atlas`/`pick_atlas_width`/`image_fits`/`image_find_place_on_side` 完整 |
| `m_filter_effect` 字段不存在（challenge.rs:2378） | **字段已存在** | `reanimator.rs:80 m_filter_effect: FilterEffectType`；draw 路径 `:407-477` 接线 `filter_effect_get_image`；`filter_effect.rs` 全实现（init/dispose/do_lum_sat/washed_out/white/create_image/get_image） |

### 4.2 Attachment 系统（原报告 §3.1）—— Trail 分支已全部接入 ✅

- 原报告"AttachmentUpdateAndMove/AttachmentDraw/AttachmentDie/AttachReanim 未实现"+"Trail 分支未接入（attachment.rs 9 处）" → **已全部接入**：
  - `attachment.rs:94` Trail Update 分支、`:164` SetPosition 追加轨迹点、`:365` Draw、`:398` Die、`:457` Detach、`:638 attach_trail` 均有实现。
  - 余下的 `EffectType::Trail | EffectType::Other => {}` 空分支是 C++ 原样语义（这些方法中本就无操作）。

### 4.3 粒子系统（原报告 §3.5）—— DefinitionLoadXML/cross_fade/override 已实现 ✅

| 原报告声称 | 实际核查 | 证据 |
|---|---|---|
| `DefinitionLoadXML` 未实现（tod_particle.rs:2071、trail.rs） | **已实现** | `trail.rs:405 trail_load_a_def`（XML 字段 Image/MaxPoints/WidthOverLength/AlphaOverTime 等全解析 + FloatTrackSetDefault）；`tod_particle.rs:2601` 粒子定义加载；`reanim_loader.rs:307 parse_reanim_xml` |
| `cross_fade` keep empty | **已完整实现** | `tod_particle.rs:2121-2176`：查找 emitter def → 校验 cross_fade_duration → 容量上限 → 遍历创建淡出 emitter → `cross_fade_emitter` |
| override_color/image/frame/scale | **均已实现** | `tod_particle.rs:2029-2117`，遍历 emitter_list 按名称匹配设置 override |
| 3D 加速位过滤跳过 | **合理近似** | `tod_particle.rs:869-882`：`a_hardware=true` 硬编码，PVZ 默认硬件加速，SOFTWARE_ONLY 跳过逻辑正确，仅缺动态 `Is3DAccelerated()` 检测 |

### 4.4 DataArray ID 体系（原报告 §3.1）—— Coin::coin_id 已接线 ✅

- 原报告"coin 无 id 字段" → **已接线**：
  - `coin.rs:50 pub coin_id: CoinID`，`board.rs:1336` 赋值 `coin_id = new_index as CoinID`。
  - 消费点 `coin.rs:995` `cursor_object.coin_id = self.coin_id`（对应 C++ Coin.cpp:1252）。

### 4.5 framework/**（29 处标注）—— 均为合理近似/说明性注释 ✅

全量回查后无真实缺失：
- `xml_parser.rs`（30→归入 framework 计数前）：均为解析逻辑的"跳过字符"注释。
- `image.rs:488-504 blt_rotated`：软件路径忽略旋转，注释说明完整旋转在 GLImage 用 OpenGL 完成（分层合理）。
- `memory_image.rs:488` 调色板化简化、`image.rs:391` 抗锯齿用 Bresenham 近似：合理。
- `gl_interface.rs:1579/1760`、`bitmap_font.rs:168/253`、`image_font.rs:874/880`：说明性注释。
- `font.rs:37/150`：无字库时矩形块近似文字位置（降级行为）。
- `paklib/mod.rs`、`resource_manager.rs`：字符跳过/元素跳过注释。

### 4.6 tod_foley.rs GetSoundPosition/SetSoundPosition —— 与 C++ 等价 ✅

- 原标注 `TODO: 从 TodFoley.cpp 翻译` → **过时**：C++ `PvzpDSoundInstance::GetSoundPosition`（PvzpFoley.cpp:151-158）本身返回 0 / 空操作，Rust 实现已与 C++ 等价。

---

## 五、尚未逐行回查的区域（后续工作）

1. `zombie.rs`(62 标注) / `plant.rs`(66 标注) / `challenge.rs` / `coin.rs` 的**函数体逐行数值与状态机**（最大不确定区，尤其 Boss 段与僵尸头部件状态机）。
2. `lawn_app.rs`(55) 与 `system/save_game.rs`(38) 标注未逐条回查 C++。
3. `board.rs`(43) 的标注（进度条/教程/对话框栈等）未逐条回查——原报告 §3.2 条目可能部分过时，需后续核实。
4. widget 层（seed_chooser_screen/store_screen/message_widget 等）标注未回查。
5. 未做运行期验证（纯静态对照，本轮未启动游戏）。

---

## 六、核查方法与复现命令

```bash
# 1) 硬空实现检查（应返回 0）
rg -c "todo!|unimplemented!" src/ | wc -l
# 2) todlib 标注扫描
rg -n "TRANSLATION_NOTE|暂略|未翻译|暂未|尚未|简化|占位|待实现|未实现|stub|近似|TODO|placeholder" src/todlib/ -i | wc -l   # 68
# 3) framework 标注扫描
rg -n "TRANSLATION_NOTE|暂略|未翻译|暂未|尚未|简化|占位|待实现|未实现|stub|近似|TODO|placeholder" src/framework/ -i | wc -l   # 29
# 4) 枚举/常量对账（应一致）
rg -n "NUM_SEED_TYPES" src/lawn/game_enums.rs            # 53
rg -n "NUM_CACHED_ZOMBIE_TYPES" src/lawn/game_enums.rs   # 35
rg -n "CachedPolevaulterWithPole" src/                   # 消费点已接线
# 5) §3.1 核心缺口验证：PvzpCurveEvaluate 在 Rust 全库无对应
rg -n "curve_evaluate|CurveEvaluate|PvzpCurve" src/ -i   # 无命中 → 确认缺失
# 6) §3.2 核心缺口验证：tod_string_translate 未查表
sed -n '136,140p' src/todlib/tod_common.rs               # 直接返回原文
rg -n "m_string_properties" src/framework/sexy_app_base.rs  # 基础设施已就绪
# 7) §3.3 核心缺口验证：apply_music_volume 空操作
sed -n '350,354p' src/todlib/tod_foley.rs                # let _ = foley_instance;
# 8) §4 过时标注验证（应均有实现）
sed -n '663,675p' src/todlib/reanimator.rs               # set_frames_for_layer 已实现
sed -n '1105,1120p' src/todlib/reanimator.rs             # matrix_from_transform 完整 6 分量
sed -n '405,469p' src/todlib/trail.rs                    # trail_load_a_def 已实现
sed -n '50,91p' src/lawn/coin.rs                        # coin_id 字段已存在
# 9) 行数比
find src -name "*.rs" | xargs wc -l | tail -1            # 93521
find cpp/src -name "*.cpp" -o -name "*.h" | xargs wc -l | tail -1  # 194270
```

核查期间除覆写本报告外未修改任何源码。

---

## 七、建议修复顺序（供后续参考，本报告未执行）

1. **★ `PvzpCurveEvaluate` 实现 + `FloatParameterTrack::evaluate` 接线**（最高优先级）：实现 `PvzpCommon.cpp:336-357` 的 13 种曲线求值，改造 `tod_particle.rs:216-243 evaluate` 使用双层曲线（`distribution` 求值 + `curve_type` 段间过渡 + `interp` 参数）。这是粒子系统的数学核心，影响所有粒子动画曲线。
2. **`tod_string_translate` 接线查表**：检测 `[name]` 格式 → 查 `sexy_app_base.m_string_properties`（`get_string`），基础设施已就绪。
3. **`apply_music_volume` 接线**：调用 `mInstance->SetVolume(mMusicVolume / mSfxVolume)` 的对应实现（依赖音频层实例级音量 API）。
4. **`ReanimLoopType` 补齐**：加入 `REANIM_PLAY_ONCE_AND_RETURN_TO_ZERO` / `REANIM_LOOP_FULL_LAST_FRAME` 两个变体并接线消费点。
5. **函数体内部简化回查**：按热点模块（plant → zombie → board/lawn_app → save_game）逐函数回查数值与状态机（§5 未覆盖区域）。
6. **widget 层标注回查**：seed_chooser_screen/store_screen/message_widget/new_options_dialog/user_dialog 等（§5 未覆盖）。
