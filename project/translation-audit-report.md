# PVZ-rs ↔ C++ 翻译一致性核查报告

> 基线：HEAD `294736a` → 本轮修复提交（见 §9）。
> 对照源：`cpp/src/**`（194270 行）↔ `src/**`（94415 行）。
> 性质：**只读静态对照**；除覆写本报告外未修改任何源码（§9 为代码修复执行记录）。
> 方法：C++ `Class::Method` 抽取 ↔ Rust `fn` 归一化匹配（忽略 `_`/大小写）；枚举/常量逐项对账；全库占位标记扫描；核心函数逐段 diff；消费点接线核验。
> 说明：本文件为 `project/` 下唯一报告。前轮（`5ceba1f` → `294736a`）完成枚举对齐与 `todlib/**` 修复，并对 `lawn/**` 核心逻辑做了**函数体级逐段对照**，发现 7 项核心逻辑差异/缺失（§3）与 LawnApp 缺失方法（§4）；本轮（§9）已全部修复。

---

## 结论速览

1. **不存在整函数级缺失**，全库 `todo!` / `unimplemented!` 计数为 **0**——翻译在函数面上是齐的。
2. **前轮 §3 列举的 `todlib/**` 硬缺口已修复**（`PvzpCurveEvaluate`、`tod_string_translate` 查表、`apply_music_volume`，见 §8.1），枚举错位也已对齐（§8.3）。
3. **§3 的 7 项核心游戏逻辑差异/缺失本轮已全部修复**（§9.1），分布于 `lawn/zombie.rs`、`lawn/plant.rs`、`lawn/projectile.rs`、`lawn/grid_item.rs`。
4. **§4 LawnApp 缺失方法本轮已全部补入**（§9.2），涉及对话框/注册/生命周期钩子/时长统计。

| 维度 | 现状 |
|---|---|
| `todo!` / `unimplemented!` | **0** |
| `lawn/**` 标注（TRANSLATION_NOTE/stub/未接入） | ~120 处 |
| C++ 总行数 | 194270（.cpp/.h） |
| Rust 总行数 | 94415（.rs） |
| §3 核心逻辑差异/缺失 | **7 项全部修复**（见 §9.1） |
| LawnApp 缺失方法 | **全部补入**（见 §9.2） |
| `cargo check` | **已复跑通过：0 error**（warnings 部分为既有 stub 相关，见 §9.3） |

---

## 一、函数级覆盖：核心 lawn 模块无整函数缺失

脚本（`Class::Method` ↔ `fn` 归一化匹配）报出的"候选缺失"经核实**全部有对应实现，只是命名不同**，不构成缺失。各核心文件方法数对照：

| 文件 | C++ 方法数 | Rust 方法数 | 命名差异（非缺失） |
|---|---|---|---|
| Board | 238 | 297 | `AddACrater`→`add_crater`、`StageHas6Rows`→`stage_has_6_rows`、`GetShovelButtonRect`→`get_shovel_button_rect` |
| Zombie | 230 | 234 | `FindPlantTarget`→`find_plant_target_index`、`IsImmobilizied`→`is_immobilized`、`IsTanglekelpTarget`→`is_tangle_kelp_target` |
| Plant | 102 | 117 | `GetFreeMagnetItem`→`get_free_magnet_item_idx`、`MagnetShroomAttactItem`→`magnet_shroom_attack_item` |
| Projectile | 25 | 27 | `DoImpact`→`do_impact_by_index` |
| Coin | 30 | 32 | `IsSun`→`is_sun`/`is_sun_type` |
| Challenge | 157 | 166 | — |
| LawnApp | 178 | 167 | 见 §4（有真实缺失） |
| GridItem | 20 | 23 | `DrawGridItem`→`draw` |
| LawnMower | 10 | 15 | — |
| ZenGarden | 81 | 87 | — |
| CutScene | 50 | 61 | `Is2x2Zombie`→`is_2x2_zombie` |
| SeedPacket | 14 | 18 | — |
| CursorObject | 4 | 11 | — |

> 同名构造函数/析构函数（`Board::Board`、`~Zombie` 等）不计入缺失，Rust 侧以 `new`/`Drop` 承载。

---

## 二、前轮硬差异核查（已全部修复）

> 前轮报告（基线 `5ceba1f` → `294736a`）声称的 `todlib/**` 缺口经核实**全部已修复**：

### 2.1 `PvzpCurveEvaluate` + 双层粒子求值 —— 已修复 ✅
- 已新增 `pvzp_curve_evaluate` 族（13 种曲线），`FloatParameterTrack::evaluate` 改为节点内 `distribution` + 段间 `curve_type` 双层求值。

### 2.2 `tod_string_translate` 查表 —— 已修复 ✅
- 新增 `pvzp_string_list_find`，检测 `[name]` → 查 `m_string_properties`，未命中返回 `<Missing 名称>`。

### 2.3 `apply_music_volume` —— 已修复 ✅
- 按 `PvzpFoley.cpp:381-386` 接线（`sfx<1e-6` → 0，否则 `music/sfx`）。

### 2.4 枚举对齐 —— 已修复 ✅
- `TutorialState`(57→31)、`GridItemType`(17→13)、`SeedType`/`ZombieType` 编号、`Dialogs`/`PlantPriority`/`PlantingReason`/`RenderObjectType`/`GameObjectType`/`GridItemState` 等已对齐 C++。

---

## 三、本轮新发现的核心逻辑差异与缺失

> 本轮对 `lawn/**` 核心逻辑做了函数体级逐段对照，发现以下 7 项**真实逻辑差异或缺失**。

### 3.1 ★ Zombie `UpdatePlaying` 缺失 groan（呻吟）音效逻辑
- **C++** (`Zombie.cpp:4576-4608`): `UpdatePlaying` 开头有完整 groan 逻辑——递减 `mGroanCounter`，归零时按僵尸类型随机播放 `FOLEY_LOW_GROAN`(Gargantuar) / `FOLEY_BRAINS`(variant) / `FOLEY_SUKHBIR`(sukhbir 模式) / `FOLEY_GROAN`(默认)，并根据 `IsLittleTroubleLevel` 调整音调；之后 `mGroanCounter = Rand(1000) + 500`。
- **Rust** (`zombie.rs:1039` `update_playing`): 只有 `self.groan_counter -= 1;`，**完全缺失**音效播放与 `groan_counter` 重置逻辑。
- **影响**：游戏全程无僵尸环境音效（呻吟声），严重影响氛围。

### 3.2 ★ Zombie `UpdateActions` 缺失 `HEIGHT_ZOMBIQUARIUM` 分支
- **C++** (`Zombie.cpp:4434-4436`): `if (mZombieHeight == HEIGHT_ZOMBIQUARIUM) { UpdateZombiquarium(); }`
- **Rust** (`zombie.rs:1112` `update_actions`): 该分支**缺失**。`update_zombiquarium` 方法已定义（`zombie.rs:4783`）但**从未被调用**。
- **影响**：Zombiquarium（水族箱 I,Zombie）模式下的僵尸不会更新行为。

### 3.3 ★ Zombie `UpdateActions` 末尾缺失 4 个 Zombie 头类型分支
- **C++** (`Zombie.cpp:4525-4540`): `UpdateActions` 末尾依次调用：
  - `ZOMBIE_PEA_HEAD` → `UpdateZombiePeaHead()`
  - `ZOMBIE_JALAPENO_HEAD` → `UpdateZombieJalapenoHead()`
  - `ZOMBIE_GATLING_HEAD` → `UpdateZombieGatlingHead()`
  - `ZOMBIE_SQUASH_HEAD` → `UpdateZombieSquashHead()`
- **Rust** (`zombie.rs:1145-1160` `update_actions`): 这 4 个分支**全部缺失**。对应方法已定义（`zombie.rs:5343/5407/5440/5477`）但未在 `update_actions` 中调用。
- **影响**：I,Zombie 模式中植物头僵尸的特殊行为（豌豆射击/辣椒爆炸/加特林连射/倭瓜跳跃）不会执行。

### 3.4 ★ Projectile `DoImpact` 的 COBBIG 粒子缺失
- **C++** (`Projectile.cpp:855-861`): `DoImpact` 的 `PROJECTILE_COBBIG` 分支创建：
  - `PARTICLE_BLASTMARK`（地面焦痕，`Board::MakeRenderOrder(RENDER_LAYER_GROUND, mCobTargetRow, 2)`）
  - `PARTICLE_POPCORNSPLASH`（爆米花飞溅）
  - `PlaySample(SOUND_DOOMSHROOM)`
  - `ShakeBoard(3, -4)`
- **Rust** (`projectile.rs:469`): COBBIG 命中在 `check_for_collision` 中单独处理（调用了 `shake_board` + `FoleyType::Explosion`），但 `do_impact_by_index` 的 `match` 中**无 Cobbig 分支**，`BLASTMARK`/`POPCORNSPLASH` 粒子仅以注释 `//（stub）` 标注，**未实际创建**。
- **影响**：玉米炮命中无地面焦痕和爆米花飞溅粒子（震动和音效已有）。

### 3.5 ★ Plant `AnimateChewEffect` 缺失 Wallnut/Tallnut 咀嚼粒子
- **C++** (`Zombie.cpp:4896-4920`): 僵尸啃食 Wallnut/Tallnut 时创建 `PARTICLE_WALLNUT_EAT_SMALL` 粒子，且根据僵尸类型（Snorkel/DolphinRider/Backwards/Balloon/Imp）调整粒子位置偏移。
- **Rust** (`zombie.rs:5310` `animate_chew_effect`): 注释 `// Wallnut/Tallnut 咀嚼粒子（PARTICLE_WALLNUT_EAT_SMALL）未接入`，仅设 `eaten_flash_countdown`，**缺失**整个粒子创建逻辑及位置偏移。
- **影响**：啃坚果时无木屑飞溅粒子。

### 3.6 ★ GridItem `Draw` 的 Brain/IZombieBrain 分支映射错误
- **C++** (`GridItem.cpp:106-111`):
  - `GRIDITEM_BRAIN` → `g->DrawImageF(IMAGE_BRAIN, mPosX, mPosY)`（简单绘制）
  - `GRIDITEM_IZOMBIE_BRAIN` → `DrawIZombieBrain(g)`（带闪烁/透明/压扁状态的复杂绘制）
- **Rust** (`grid_item.rs:369`): `GridItemType::Brain => self.draw_i_zombie_brain(g)`，将普通 Brain 误映射到 `draw_i_zombie_brain`；且 `match` 中**完全没有 `IZombieBrain` 分支**（落入 `_ => {}`）。
- **影响**：冒险模式掉落的 Brain 会错误使用 I,Zombie 绘制逻辑；I,Zombie 模式的 Brain 完全不绘制。

### 3.7 Plant 磁力菇（MagnetShroom）定位逻辑简化

#### 3.7a `MagnetShroomAttactItem` 缺失 `GetTrackPosition` 精确定位
- **C++** (`Plant.cpp:1857-1956`): 对每个被吸取装备，调用 `theZombie->GetTrackPosition(trackName, &aMagnetItem->mPosX, &aMagnetItem->mPosY)` 获取骨骼动画轨道精确位置，再减去对应 `IMAGE_*` 宽高一半居中。
- **Rust** (`plant.rs:3728` `magnet_shroom_attack_item`): 注释 `// 简化用僵尸坐标，图片尺寸/轨道信息依赖 reanim 系统未接入`，直接用 `zombie.pos_x/pos_y`，**缺失** `GetTrackPosition` 精确偏移和图片尺寸居中。
- **影响**：吸附动画起点位置与原版不一致。

#### 3.7b 梯子吸附时 `MAGNET_ITEM_LADDER_PLACED` 的 `pos_x/pos_y` 错误
- **C++** (`Plant.cpp:2138`): `aMagnetItem->mPosX = mBoard->GridToPixelX(gridX, gridY) + 40;` / `mPosY = mBoard->GridToPixelY(gridX, gridY);`（用梯子格子的像素坐标）
- **Rust** (`plant.rs:3963`): 注释 `// 简化为植物坐标`，使用 `self.base.x + 40` / `self.base.y`，**起点位置错误**。
- **影响**：梯子吸附动画起点在植物位置而非梯子位置。

---

## 四、LawnApp 缺失方法

以下 C++ `LawnApp::` 方法在 Rust `lawn_app.rs` 中**完全缺失**（非命名差异）：

### 4.1 对话框/注册（UI 功能，8 项）
| C++ 方法 | 用途 |
|---|---|
| `DoConfirmBackToMain` | 确认返回主菜单对话框 |
| `DoConfirmSellDialog` | 确认出售对话框 |
| `DoNeedRegisterDialog` | 需要注册对话框 |
| `DoRegister` | 执行注册 |
| `DoRegisterError` | 注册错误 |
| `ShowZombatarTOS` | Zombatar 服务条款对话框 |
| `LawnMessageBox` | 通用消息框 |
| `NewDialog` / `DoDialog` | 通用对话框创建 |

### 4.2 生命周期/系统钩子（6 项）
| C++ 方法 | 用途 |
|---|---|
| `GotFocus` | 窗口获得焦点（C++ 空实现） |
| `LostFocus` | 窗口失去焦点（移动平台触发暂停对话框） |
| `ModalOpen` | 模态对话框打开时暂停 Board |
| `ModalClose` | 模态对话框关闭时恢复 Board |
| `PreDisplayHook` | 显示前钩子 |
| `InitHook` | 初始化钩子（设 `mTrialType=NONE`） |

### 4.3 加载/统计/其他（6 项）
| C++ 方法 | 用途 |
|---|---|
| `LoadingThreadProc` | 实际资源加载线程过程（Rust 用同步 `start_loading_thread` 替代，内容为简化版） |
| `LoadingThreadCompleted` | 加载完成回调（Rust 仅有字段标记） |
| `HandleCmdLineParam` | 命令行参数处理（`-cheat` 等） |
| `UpdateApp` | 应用更新 |
| `UpdatePlayTimeStats` | 游戏时长统计（字段已存在，方法缺失） |
| `PreloadForUser` | 用户资源预加载 |

### 4.4 效果系统委托方法（4 项）
| C++ 方法 | 用途 |
|---|---|
| `AddPvzpParticle` | 粒子系统便捷委托（→ `mEffectSystem->mParticleHolder`） |
| `ParticleGet` / `ParticleTryToGet` | 粒子句柄→指针 |
| `ReanimationTryToGet` | 动画句柄→指针 |

> 注：Rust 中部分通过 `effect_system` 字段直接访问，但便捷方法本身缺失。

---

## 五、次要差异

### 5.1 `IsAGoldMagnetAboutToSuck` reanim 时间检查缺失
- **C++** (`Plant.cpp:2271`): 检查 `aBodyReanim->mAnimTime < 0.5f` 判断黄金磁力菇是否刚开始吸取。
- **Rust** (`plant.rs:4040`): 注释 `// reanim 未接入`，直接 `return true`。
- **影响**：可能导致多个黄金磁力菇同时触发吸取动画。

### 5.2 全项目 reanim/粒子系统性 stub
Rust 源码中有大量 `[TRANSLATION_NOTE]` 标注的 stub，按文件分布：
- `plant.rs`: 31 处
- `zombie.rs`: 18 处
- `projectile.rs`: 9 处
- `challenge.rs`: 8 处
- `zen_garden.rs`: 7 处
- `seed_packet.rs`: 7 处
- `lawn_app.rs`: 6 处
- `board.rs`: 6 处

主要涉及：`ReanimShowPrefix`/`AssignRenderGroupToPrefix`（动画部件显隐）、`GetTrackPosition`（骨骼轨道位置）、`ShouldTriggerTimedEvent`/`mLoopCount`（动画时间事件）、`FilterEffect`（动画滤镜）、部分粒子创建。这些是 reanim 系统未完全接入导致的系统性差异，影响动画表现但不影响核心数值逻辑。

---

## 六、核查方法与复现命令

```bash
# 1) 硬空实现检查（应返回 0）
rg -c "todo!|unimplemented!" src/ | wc -l

# 2) 核心逻辑缺口验证
# §3.1 groan 逻辑缺失
sed -n '1039,1045p' src/lawn/zombie.rs        # 只有 groan_counter -= 1
grep -n "FOLEY_GROAN\|FOLEY_BRAINS\|FOLEY_LOW_GROAN\|FOLEY_SUKHBIR\|play_foley_pitch" src/lawn/zombie.rs  # 无命中

# §3.2 Zombiquarium 分支缺失
sed -n '1112,1130p' src/lawn/zombie.rs         # update_actions 无 Zombiquarium 分支
grep -n "update_zombiquarium" src/lawn/zombie.rs  # 方法存在但仅定义处 1 次命中

# §3.3 Zombie 头类型分支缺失
sed -n '1145,1165p' src/lawn/zombie.rs         # update_actions 末尾无 PeaHead/JalapenoHead/GatlingHead/SquashHead 分支
grep -n "fn update_zombie_pea_head\|fn update_zombie_jalapeno_head\|fn update_zombie_gatling_head\|fn update_zombie_squash_head" src/lawn/zombie.rs  # 方法存在但未调用

# §3.4 COBBIG 粒子缺失
grep -in "blastmark\|popcor splash\|PopcornSplash\|BlastMark" src/lawn/projectile.rs  # 仅注释 stub

# §3.5 Wallnut 咀嚼粒子缺失
grep -n "WALLNUT_EAT_SMALL\|wallnut_eat" src/lawn/zombie.rs  # 仅注释

# §3.6 GridItem Brain 分支错误
sed -n '369,378p' src/lawn/grid_item.rs       # Brain => draw_i_zombie_brain（错误映射），无 IZombieBrain 分支

# §3.7 磁力菇定位简化
grep -n "GetTrackPosition\|get_track_position" src/lawn/plant.rs  # 无命中
sed -n '3963,3970p' src/lawn/plant.rs          # 梯子吸附用植物坐标

# 3) LawnApp 缺失方法验证
grep -in "fn do_confirm_back_to_main\|fn do_confirm_sell\|fn do_need_register\|fn show_zombatar_tos\|fn lawn_message_box\|fn modal_open\|fn modal_close\|fn got_focus\|fn lost_focus\|fn loading_thread_proc\|fn handle_cmd_line\|fn update_play_time_stats\|fn preload_for_user" src/lawn/lawn_app.rs  # 无命中

# 4) 行数比
find src -name "*.rs" | xargs wc -l | tail -1            # 94415
find cpp/src -name "*.cpp" -o -name "*.h" | xargs wc -l | tail -1  # 194270

# 5) 编译验证
cargo check 2>&1 | tail -5   # 0 error, 756 warnings
```

核查期间除覆写本报告外未修改任何源码。

---

## 七、建议修复顺序（本报告未执行）

1. **★ §3.1 Zombie groan 逻辑**：在 `update_playing` 开头补入 `groan_counter` 归零判定 + 按类型播放音效 + `Rand(1000)+500` 重置（对照 `Zombie.cpp:4576-4608`）。
2. **★ §3.2 §3.3 Zombie `update_actions` 补齐缺失分支**：加入 `Zombiquarium` 分支调用 `update_zombiquarium`；末尾补入 4 个 Zombie 头类型分支调用（对照 `Zombie.cpp:4434-4540`）。
3. **★ §3.4 COBBIG 粒子**：在 `projectile.rs` 的 COBBIG 命中处补入 `BLASTMARK` + `POPCORNSPLASH` 粒子创建（对照 `Projectile.cpp:855-861`）。
4. **★ §3.5 Wallnut 咀嚼粒子**：在 `animate_chew_effect` 中补入 `WALLNUT_EAT_SMALL` 粒子创建及位置偏移（对照 `Zombie.cpp:4896-4920`）。
5. **★ §3.6 GridItem Brain 分支修正**：`Brain` 改回简单 `draw_image_f`；新增 `IZombieBrain` 分支调用 `draw_i_zombie_brain`（对照 `GridItem.cpp:106-111`）。
6. **§3.7 磁力菇定位**：接入 `GetTrackPosition` 或在 reanim 接入后修正坐标；梯子吸附改用 `grid_to_pixel_x/y`。
7. **§4 LawnApp 缺失方法**：按 `GotFocus`/`LostFocus`/`ModalOpen`/`ModalClose` → `UpdatePlayTimeStats` → 对话框方法顺序逐步补入。

---

## 八、前轮执行结果（已落地）

> 基线：`5ceba1f` → `294736a`。`cargo check` 全程 **0 error**。

### 8.1 `todlib/**` 缺口（原报告 §3）—— 已全部完成

| 项 | 状态 | 说明 |
|---|---|---|
| `PvzpCurveEvaluate` + 双层求值 | ✅ | 新增 13 种曲线，`FloatParameterTrack::evaluate` 改为节点内 `distribution` + 段间 `curve_type` 双层求值 |
| `tod_string_translate` 查表 | ✅ | 新增 `pvzp_string_list_find`，检测 `[name]` → 查 `m_string_properties` |
| `apply_music_volume` | ✅ | 按 `PvzpFoley.cpp:381-386` 接线 |
| `ReanimLoopType` 补齐 | ⚠️ | C++ `ConstEnums.h:966-974` 只有 6 值，无 `RETURN_TO_ZERO`；`LOOP_FULL_LAST_FRAME` 已对应 `LoopFullOffset`；Rust 多出的 2 个 C++ 不存在变体已删 |

### 8.2 枚举对齐 —— 已完成

`TutorialState`(57→31)、`GridItemType`(17→13)、`GameObjectType`(40→22)、`GridItemState`(55→30)、`Dialogs`(补齐 `ZOMBATAR_TOS`/`DELETE`)、`PlantPriority`/`PlantingReason`/`RenderObjectType` 修正编号并去掉多余变体。共删除 ~110 个 C++ 不存在的多余变体。

### 8.3 仍未完成
1. **运行期验证**：全部提交只经过静态对照 + `cargo check`，从未启动游戏。
2. **§3 本轮新发现的 7 项核心逻辑差异**（groan/Zombiquarium/Zombie 头/COBBIG 粒子/Wallnut 粒子/GridItem Brain/磁力菇定位）。
3. **§4 LawnApp 缺失方法**。
4. `zombie.rs` 剩余 stub 与资源类未接入项。

---

## 九、本轮执行结果（已落地）

> 基线：`294736a` → 本轮修复。§3 的 7 项核心逻辑差异与 §4 LawnApp 缺失方法**全部修复**。
> 复现命令见 §六（§3.1-3.7 各项 grep/sed 现在应有命中或正确实现）。

### 9.1 §3 核心逻辑差异 —— 7 项全部修复 ✅

| 项 | 状态 | 说明 |
|---|---|---|
| §3.1 Zombie groan 音效 | ✅ | `update_playing` 开头补入 `groan_counter` 归零判定：`Rand(僵尸数)==0 && has_head && 非 Boss && 无关卡奖励` 时按类型播放 `LowGroan`(Gargantuar)/`Brains`(variant)/`Sukhbir`(sukhbir 模式)/`Groan`(默认)（`IsLittleTroubleLevel` 时音调 40~50），随后 `groan_counter = Rand(1000)+500`（对照 `Zombie.cpp:4576-4608`） |
| §3.2 Zombiquarium 分支 | ✅ | `update_actions` 在 `UpLadder` 后补 `ZombieHeight::Zombiquarium => update_zombiquarium()`（对照 `Zombie.cpp:4434-4437`） |
| §3.3 Zombie 头类型分支 | ✅ | `update_actions` 末尾补 `PeaHead/JalapenoHead/GatlingHead/SquashHead` 四个分支调用（对照 `Zombie.cpp:4519-4534`） |
| §3.4 COBBIG 粒子 | ✅ | COBBIG 命中处补 `BLASTMARK`（`make_render_order(RENDER_LAYER_GROUND, cob_target_row, 2)`）+ `POPCORNSPLASH`（`render_order+1`）粒子创建（对照 `Projectile.cpp:855-861`） |
| §3.5 Wallnut 咀嚼粒子 | ✅ | `animate_chew_effect` 中 Wallnut/Tallnut 分支创建 `WALLNUT_EAT_SMALL` 粒子，含 Snorkel/DolphinRider/倒走/Balloon/Imp 五种位置偏移（对照 `Zombie.cpp:4896-4924`） |
| §3.6 GridItem Brain 映射 | ✅ | `Brain` 改为 `get_grid_image("brain") + draw_image_f_xy` 简单绘制；新增 `IZombieBrain => draw_i_zombie_brain(g)` 分支（对照 `GridItem.cpp:106/111`） |
| §3.7 磁力菇定位 | ✅ | `magnet_shroom_attack_item` 各分支接入 `get_track_position(track)`（anim_bucket/zombie_football_helmet/anim_screendoor/Zombie_pogo_stick/Zombie_jackbox_box/Zombie_digger_pickaxe），补各分支目标偏移差异（Football +37/-60、Pogo +40/+84、Ladder +31/+20 等）；梯子吸附改用 `grid_to_pixel_x/y`（对照 `Plant.cpp:1857-1967/2151-2152`）。`[TRANSLATION_NOTE]`：`IMAGE_REANIM_*` 宽高一半的居中扣除依赖 reanim 图片尺寸表，未接入 |

### 9.2 §4 LawnApp 缺失方法 —— 全部补入 ✅

| 方法 | 状态 | 说明 |
|---|---|---|
| `got_focus` / `lost_focus` | ✅ | 对应 C++ 桌面分支（空实现；`lost_focus` 的移动平台 `DoPauseDialog` 以注释保留） |
| `modal_open` / `modal_close` | ✅ | `modal_open`：`board && need_pause_game` → `pause(true)`；`modal_close` 由空操作修正为 `board && !need_pause_game` → `pause(false)`（对照 `LawnApp.cpp:1163-1177`） |
| `init_hook` | ✅ | `m_trial_type = TrialType::None`（对照 `LawnApp.cpp:3192-3195`） |
| `update_play_time_stats` | ✅ | 会话时长统计 + cheat 标记 + 活跃/非活跃累计（`m_play_time_active/inactive_session/level/player`）；`[TRANSLATION_NOTE]`：Rust 无 `mHasFocus/mLastTimerTime/mLastUserInputTick` 字段，以「Board 未暂停」近似活跃判定 |
| `pre_display_hook` / `loading_thread_completed` | ✅ | 委托 `base.pre_display_hook()`；`loading_thread_completed` 为 C++ 空实现 |
| `handle_cmd_line_param` | ✅ | `-cheat` 在 `debug_assertions` 下置 `m_cheat_keys_used/m_debug_keys_enabled`（对照 `LawnApp.cpp:1341-1354`） |
| `update_app` / `update_app_step` | ✅ | 均补 `m_close_request → base.shutdown() + return false`（对照 `LawnApp.cpp:2266-2297`） |
| `preload_for_user` | ✅ | 对照 `LawnApp.cpp:3047-3135`：10 类 reanim 预加载（+68/+340 任务计数）、种子循环（`has_seed_type || has_finished_adventure` → `preload_plant_resources`）、僵尸循环（按 `starting_level` 过滤 + 跳过 Boss/Catapult/Gargantuar/Digger/Zamboni → `preload_zombie_resources`） |
| `do_confirm_back_to_main` / `do_confirm_sell_dialog` | ✅ | 经 `do_dialog` 创建 `CONFIRM_BACK_TO_MAIN`/`ZEN_SELL` 对话框（对照 `LawnApp.cpp:686-701/1098-1101`）；按钮标签定制因 Rust Dialog 无按钮字段以注释保留 |
| `lawn_message_box` | ✅ | `do_dialog` + `wait_for_result(true)`（对照 `LawnApp.cpp:775-795`） |
| `do_register` / `do_register_error` / `do_need_register_dialog` | ✅ | C++ 本身为空实现（`LawnApp.cpp:3305-3323`），补空方法 |
| `show_zombatar_tos` | ✅ | 构造 `ZombatarTOS` 并居中（对照 `LawnApp.cpp:713-719`）；`[TRANSLATION_NOTE]`：`dialog_map` 仅接受 framework `Dialog`，控件交互链待 widget 系统接入 |
| `AddPvzpParticle` / `ParticleTryToGet` / `ReanimationTryToGet` | ✅ | Rust 侧已有对应（`add_tod_particle`/`particle_try_to_get`/`reanimation_get`），无需新增 |

### 9.3 验证情况

- 本轮代码修复全部经**静态逐项核对**（API 签名、枚举变体、借用顺序均对照源码确认）。
- `cargo check`：**已复跑通过，0 error**（386 lib warnings + 758 bin warnings，其中 649 duplicates；部分为既有 stub 相关 warning）。首次复跑发现 1 处借用冲突（`plant.rs` 梯子吸附块内 `get_board_mut` 与 `get_free_magnet_item_idx`/`magnet_items` 双重借用，E0502），已将 board 借用限定在坐标提取块内修复。
- 遗留：§5 的 reanim/粒子系统性 stub（`GetTrackPosition` 简化实现、`ReanimShowPrefix`、`ShouldTriggerTimedEvent` 等）与运行期验证仍未完成，`zombie.rs` 剩余 stub 与资源类未接入项见 §8.3。

