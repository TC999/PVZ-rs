# PVZ-rs：C++ → Rust 翻译差异报告

> **基线**：`dc0fb9f`（`§2.4 demo 录制/回放移植`）+ 当前**未提交工作区修改**（`src/lawn/zombie.rs` 等 18 个文件）
> **对照对象**：C++ `cpp/src/`（权威参考实现） ↔ Rust `src/`
> **本文覆写** 2026-09-17 的 `ANALYSIS_REPORT.md` 与 `.reasonix/audit-report.md`；旧版条目状态见 §9，**仓库中只保留本文件**。
> **局限**：全程只读静态分析，未运行 `cargo check`/`cargo build`，无运行期验证。本次核对**未修改任何源码文件**。

**检查方法**

1. 文件级映射：`cpp/src` ↔ `src`（归一化 basename）
2. 名字级差集：C++ `Class::Method(` 定义（2682）vs Rust 全标识符（2432 命中 / 250 未命中）
3. 函数体行数悬殊候选：C++ 体 ≥12 行且 Rust 体 ≤1/3，共 **139 条**，逐条双端比对
4. 接线扫描：Rust `pub fn` 全库仅出现 1 次（= 定义自身）且 C++ 同名高频，共 **144 条**
5. 关键项逐行复核（直接 `sed`/`rg` 读取两端实现）

**判定原则**：Rust 属"重命名 + 重写"式翻译（非逐行），函数名与结构不一致属预期；差异一律以**行为**为准；无法确证的一律标注"未确证"。

---

## 0. 总体状况（量化）

| 指标 | 数值 |
|---|---|
| `cpp/src` 总行数 / `src` 总行数 | 194,270 / 98,466 |
| C++ `Lawn` 层行数 / Rust `lawn`+`todlib` 行数 | 76,168 / 74,617 |
| Rust `fn` 总数 | 3,948 |
| `TRANSLATION_NOTE` | 485 处 |
| "未接入" / "依赖底层系统" / "未实现" / "暂未" / "简化" | 144 / 52 / 28 / 18 / 110 处 |
| `play_foley`+`play_sample` 调用点 | Rust 289 vs C++ 415 |
| C++ `AddPvzpParticle` 调用 | 130 处；Rust 侧**无任何对应 API** |

标记密度 top：`zombie.rs` 96、`plant.rs` 82、`lawn_app.rs` 63、`cutscene.rs` 33、`board.rs` 33、`zen_garden.rs` 25、`widget/credit_screen.rs` 25、`challenge.rs` 22。

**缺口分三层**：①整段逻辑被骨架化（函数存在但只剩状态置位/计数器）；②判定用错常量或漏条件（函数"看起来实现了"但语义不符）；③子系统未接线（实现了却无调用点，或底层 API 根本不存在）。

---

## 1. 玩法级差异（影响判定与流程）

### 1.1 `Coin::CoinInitialize` —— 硬币运动与视觉整体缺失 🔴

- **C++** `cpp/src/Lawn/Coin.cpp:50-411`（361 行）
- **Rust** `src/lawn/coin.rs:107-148`（42 行）

缺失内容：
- 全部 reanimation 创建与挂接（`REANIM_SUN` / `REANIM_COIN_SILVER` / `REANIM_COIN_GOLD` / `REANIM_DIAMOND`）及 `AttachReanim`
- 每种 `CoinType` 的 `mWidth/mHeight/mRenderOrder`（`COIN_FINAL_SEED_PACKET`、`COIN_TROPHY`、`COIN_AWARD_*`、`COIN_SHOVEL`、`COIN_CARKEYS`、`COIN_ALMANAC`、`COIN_VASE`、`COIN_WATERING_CAN`、`COIN_TACO`、`COIN_NOTE`、`COIN_USABLE_SEED_PACKET`、`COIN_PRESENT_PLANT`、`COIN_CHOCOLATE` 等）
- `COIN_PRESENT_PLANT` / `COIN_AWARD_PRESENT` 按背景 `PvzpPickFromArray` 选种子（5 组背景表 + 无尽模式 `PickRandomSeedType`）
- **整个 `switch (mCoinMotion)`**：`mVelX/mVelY/mGroundY/mScale`（`FROM_SKY` 0.67、`FROM_SKY_SLOW` 0.33、`FROM_PLANT`、`COIN`、`LAWNMOWER_COIN`、`FROM_PRESENT`、`FROM_BOSS`）+ `std::clamp` 边界；`coin.rs` 的 `coin_initialize` **从不赋值这些字段**，`Board::add_coin`（`board.rs:1343`）也未补 → 硬币初速恒为 0、不会落体
- `COIN_SMALLSUN` 0.5f / `COIN_LARGESUN` 2.0f 的 `mScale` 叠加
- `COIN_MOTION_LAWNMOWER_COIN` 的即时 `Collect()`
- `IsStormyNightLevel()` 时的 `mRenderOrder` 覆写
- `PlayLaunchSound()` 缺少 `mCoinMotion != COIN_MOTION_FROM_PRESENT` 过滤

### 1.2 `Board::FindLawnMowerInRow` / `.._mut` —— 行参数被忽略 🔴

- **C++** `cpp/src/Lawn/Board.cpp:9460-9472`：`!mDead && mRow == theRow`
- **Rust** `src/lawn/board.rs:6331` / `:6335`：形参写作 `_row` 且完全未使用，`find(|m| !m.dead && !m.mowing)` 返回**任意一行**的割草机

### 1.3 `Board::GetNumSeedsInBank` —— 卡槽数错 🔴

- **C++** `cpp/src/Lawn/Board.cpp:8587-8647`
- **Rust** `src/lawn/board.rs:4934`：`if is_first_time_adventure() && level < 5 { 3 } else { 6 }`

缺全部模式分支：ScaryPotter=1、WhackAZombie=3、ChallengeWithoutSeedBank=0、ConveyorBelt=10、SlotMachine=3、ICE=6、Beghouled/BeghouledTwist=0、Zombiquarium=2、`I_ZOMBIE_1..4`=3 / `5..7`=4 / `8`=6 / `9`=8、ENDLESS=9，以及默认 `min(PACKET_UPGRADE + 6, GetSeedsAvailable())`。

### 1.4 `Projectile::GetProjectileRect` —— 碰撞矩形恒定 🔴

- **C++** `cpp/src/Lawn/Projectile.cpp:1167-1195`
- **Rust** `src/lawn/projectile.rs:1037-1044`：恒为 `Rect(pos_x-5, pos_y-5, 10, 10)`

缺按类型矩形：Pea/SnowPea/ZombiePea（`x-15, w+15`）、Cobbig（230×230 居中）、Melon/Wintermelon（`x+20, 60`）、Fireball（`w-10`）、Spike（`x-25, w+25`）、默认（`mX/mY/mWidth/mHeight`）。
**该函数在 `src/` 中有 9 个调用点**（`board.rs:1225/1652`、`projectile.rs:527/1016/1154/1173`、`plant.rs:3420`、`challenge.rs:1726`）——全部碰撞判定受影响。

### 1.5 `Board` 谓词与查询类

| 项目 | C++ | Rust | 差异 |
|---|---|---|---|
| `CountZombiesOnScreen` / `AreEnemyZombiesOnScreen` | `Board.cpp:286-313` | `board.rs:2634-2641` | 缺 `mHasHead`、`IsDeadOrDying()`、`IsOnBoard()`（三谓词均已存在；`zombie.rs:7335` 另有完整写法） |
| `GetLevelRandSeed` | `Board.cpp:799-811` | `board.rs:4929` | 缺 `playerInfo->mId`、冒险 `finishedAdventure*101+mLevel`、挑战 `mSurvivalStage*101+gameMode` |
| `StageHasZombieWalkInFromRight` | `Board.cpp:8689-8703` | `board.rs:5572` | 判据换成 `!stage_has_roof() && !stage_has_pool()`，与 C++ 的模式枚举（WhackAZombie/ICE/ZenGarden/TreeOfWisdom/Zombiquarium/FinalBoss/IZombie/Squirrel/ScaryPotter 均为 false）无关，多模式下结果相反 |
| `LeftFogColumn` | `Board.cpp:8718-8728` | `board.rs:6252` | 恒 `5`；缺 AIR_RAID=6、level31=6、32–36=5、37–40=4 |
| `PlantingPixelToGridY` | `Board.cpp:8769-8805` | `board.rs:6274` | 缺 `SEED_INSTANT_COFFEE` 对上(-50)/下(+30)格睡眠植物的智能选行 |
| `CountUntriggerLawnMowers` | `Board.cpp:329-342` | `board.rs:2644`（另 `:6228` 重复实现） | 缺 `state != SQUISHED` 排除 |
| `GetGraveStoneCount` / `GetRake` | `Board.cpp:9044` / `:415` | `board.rs:4984` / `:1453` | 缺 `!mDead` 过滤（同类 `GetGraveStonesCount:5214` 却带了，前后不一致） |
| `CountCoinsBeingCollected` | `Board.cpp:8481-8494` | `board.rs:6233` | 用 `lifetime` 区间替代 `mIsBeingCollected && IsMoney()` + `GetCoinValue()` 累加 → 影响 `CanTakeSunMoney` |

### 1.6 GridItem 生成与回收

- `AddACrater` / `AddAGraveStone`（`Board.cpp:491-510`）经 `board.rs:5200-5211` 落到 `grid_item.rs:123-129`：只设 type/坐标，**缺 `mRenderOrder`**（保持 0）与墓碑 `mGridItemCounter = -Rand(50)`（且不耗 RNG）
- `GridItem::GridItemDie`（`GridItem.cpp:65-81` ↔ `grid_item.rs:767`）：仅 `dead = true`，缺 `ReanimationDie()` + id 复位、`ParticleSystemDie()`

### 1.7 `ZenGarden::PickRandomSeedType`

- **C++** `cpp/src/Lawn/ZenGarden.cpp:2410-2424`：收集 0..40 排除 `MARIGOLD/FLOWERPOT` 后 `PvzpPickFromArray`
- **Rust** `src/lawn/zen_garden.rs:2573`：恒返回 `Peashooter`，且**不消耗 RNG** → 破坏与 C++ 一致的随机序列

### 1.8 其它常量桩

| 项目 | C++ | Rust |
|---|---|---|
| `LawnApp::CanPauseNow` | `LawnApp.cpp:367-385`（6 项判定） | `lawn_app.rs:1744` → `true` |
| `LawnApp::NeedPauseGame` | `LawnApp.cpp:1146-1161` | `lawn_app.rs:2360` → `false` |
| `LawnApp::GetNumTrophies` | `LawnApp.cpp:3151-3165`（遍历 `NUM_CHALLENGE_MODES`） | `lawn_app.rs:1831` → `0` |
| `LawnApp::OpenURL` | `LawnApp.cpp:1823-1837` | `lawn_app.rs:2936` → `false` |
| `ReportAchievement::GiveAchievement` | `AchievementsScreen.cpp:225-246` | `achievements_screen.rs:146` 仅越界检查；**全库无任何写入 `mEarnedAchievements` 的位置** |
| `Music::SetupVolumeForTune` / `UpdateMusicBurst` | `Music.cpp:112` / `:399` | `music.rs:439`（零调用） / `:367` |
| `SeedPacket::CanPickUp` | `SeedPacket.cpp:685-723` | `seed_packet.rs:100`：仅 `active && countdown<=0 && seed_type!=None`；缺暂停/场景、imitater 替换、SlotMachine、`mEasyPlantingCheat`、`GetCurrentPlantCost`/`CanTakeSunMoney`、`PlantingRequirementsMet` |
| `SeedChooserScreen::CloseSeedChooser` | `SeedChooserScreen.cpp:1099-1117` | `seed_chooser_screen.rs:879`：仅置 `choose_state=Normal`；缺遍历 bank `SetPacketType` + 刷新状态写入 + `EndSeedChooser()` → **选好的种子不写入卡槽** |
| `SeedChooserScreen::PickedPlantType` | `SeedChooserScreen.cpp:1083-1097` | `seed_chooser_screen.rs:882`：状态判据**取反**（C++ 要求 `== SEED_IN_BANK`，Rust 写 `!= InBank`），且未处理 `imitater_type` |

### 1.9 `Zombie::TakeBodyDamage` 的 Boss 钳位仍缺

- **C++** `cpp/src/Lawn/Zombie.cpp:7821` 起的 `TakeBodyDamage`，其 `ZOMBIE_BOSS` 分支结尾：
  ```cpp
  if (mBodyHealth <= 0) { mBodyHealth = 1; }   // Zombie.cpp:7952-7955
  ```
- **Rust** `src/lawn/zombie.rs:5342` 起的 `take_body_damage`：函数已重写为含 Zamboni / Catapult 类型分支的完整实现，但**函数体内无 `ZombieType::Boss` 分支**，全库亦无 `body_health = 1` 之类的钳位 → Boss 仍会被普通伤害击杀。

---

## 2. 僵尸行为差异（`cpp/src/Lawn/Zombie.cpp` ↔ `src/lawn/zombie.rs`）

| # | 函数 | C++ | Rust | 缺失内容 |
|---|---|---|---|---|
| 2.1 | `DropShield` 🔴 | `:7539-7598` | `:6087`（仅 `shield_type=None`） | DOOR / NEWSPAPER / LADDER 三分支全缺：`DetachShield`、报纸进入 `PHASE_NEWSPAPER_MADDENING`、`StopEating`、鬼脸清理、`PlayZombieReanim("anim_gasp")`、三种掉落粒子、`FOLEY_NEWSPAPER_RIP`、`AddAttachedReanim(REANIM_ZOMBIE_SURPRISE)` |
| 2.2 | `DropHelm` | `:7666-7714` | `:6077`（仅 `helm_type=None`） | 缺 `GetDrawPos`/`GetTrackPosition` 定位、`ReanimShowPrefix` 隐藏/恢复、CONE/PAIL/HELMET/HEADLIGHT 四种粒子、`BobsledCrash` |
| 2.3 | `TakeHelmDamage` | `:7717-7800` | `:5306` | 缺 damage index 变化时的 12 处 `SetImageOverride`（cone2/3、bucket2/3、hardhat2/3、helmet2/3、wallnut/tallnut cracked1/2）→ 头盔/坚果头受击无裂纹变化 |
| 2.4 | `DropHead` | `:3508-3693` | `:6124` | 缺 Zombotany 分支、全部头/护目镜/胡须/皮纳塔粒子、`PogoBreak/DropPole/DropFlag`、`FOLEY_LIMBS_POP` |
| 2.5 | `SetupReanimForLostArm` | `:3695-3876` | `:6150` | 缺定位计算、`IsWalkingBackwards()` 的 `+36` 偏移、全部 `SetImageOverride(*_UPPER2)`、手臂粒子 |
| 2.6 | `PlayDeathAnim` | `:8899-9028` | `:7067` | 轨道恒 `"anim_death"`；缺 `anim_waterdeath/superlongdeath/death2` 选择、`anim_death` 不存在→`DieNoLoot`、冰陷阱/黄油/鬼脸清理、`DAMAGE_DOESNT_LEAVE_BODY` 分支、`AttachmentReanimTypeDie`、squash head、`HEIGHT_ZOMBIQUARIUM`、`HEIGHT_UP_LADDER→FALLING`、`FOLEY_GARGANTUDEATH`、`BossDie` |
| 2.7 | `DieNoLoot` | `:7419-7442` | `:7396` | 缺 `AttachmentDie`、4×`RemoveReanimation`（body/mowered/specialHead/zombatarHead）、`TrySpawnLevelAward`、`BobsledDie/BungeeDie/BossDie` |
| 2.8 | `RiseFromGrave` | `:8184-8264` | `:4397` | 缺泳池 `ReanimIgnoreClipRect`×4、`PARTICLE_ZOMBIE_SEAWEED`×3 + `AttachParticleToTrack`、`PoolSplash`；陆地 `DIRT_RISE`/`GRAVESTONE_RUMBLE` 音效与两类出土粒子 |
| 2.9 | `BungeeLiftTarget` | `:1246-1284` | `:9388` | 仅播 `anim_raise`；缺 `aPlant->mOnBungeeState=RISING_WITH_BUNGEE`、`FOLEY_FLOOP`、植物 `mAnimRate=0.1`、猫尾草补睡莲、I-Zombie 掉阳光、`DO_FIX_BUGS` |
| 2.10 | `IsMovingAtChilledSpeed` | `:6589-6625` | `:4970` | 仅 `chilled_counter>0`；缺舞者/后备舞者的 leader + `NUM_BACKUP_DANCERS` follower 传染性减速判定 |
| 2.11 | `DrawBobsledReanim` 🔴 | `:5386-5534` | `:7827` | 函数体为空注释 → **雪橇完全不渲染**（前后图、`GetHelmDamageIndex` 分档贴图、`DrawImageF(BOBSLED_INSIDE)`、`mJustGotShotCounter` 加色） |
| 2.12 | `DrawBungeeReanim` | `:5536-5579` | `:7833` | 缺被提僵尸/植物的偏移绘制、`DrawRenderGroup(RENDER_GROUP_ARMS)` |
| 2.13 | `ShowYuckyFace` | `:4706-4732` | `:6197` | 函数体为空 → "嫌恶脸"从不显示 |
| 2.14 | `DrawZombieHead` / `DrawZombieWithParts` | `:5211` / `:5248` | `:7040` / `:7045` | ✅ 等价：C++ 侧处于 `/* … */` 注释块内，属死代码 |

## 3. 植物行为差异（`cpp/src/Lawn/Plant.cpp` ↔ `src/lawn/plant.rs`）

| # | 函数 | C++ | Rust | 缺失内容 |
|---|---|---|---|---|
| 3.1 | `FindTargetZombie` 🔴 | `:4769-4904` | `:1133` | 仅 `row_dev==0` + 重叠 + `weight=-z_rect.x`；缺 `GetDamageRangeFlags`/`EffectedByDamage`、Portal 检查、Gloomshroom `±1` 行、Chomper/PotatoMine/Explode-o-nut/Tanglekelp 特判、`aExtraRange`、Cattail 2D 距离权重与飞行 `+10000` |
| 3.2 | `AttachBlinkAnim` | `:2890-3012` | `:2969` | 函数体返回 `None` → 全部眨眼动画（wallnut twitch/twice/thrice、三线射手、分裂豌豆、双子葵、豌豆系列等）未创建 |

---

## 4. Board 交互层（`cpp/src/Lawn/Board.cpp` ↔ `src/lawn/board.rs`）

| # | 函数 | C++ | Rust | 缺失内容 |
|---|---|---|---|---|
| 4.1 | `MouseDownWithPlant` 🔴 | `:3608-3973`（365 行） | `:7002-7112` | 缺 IZombie 分派（`IZombieMouseDownWithZombie`）；右键/出界 `FOLEY_DROP`；`CanPlantAt` 失败提示细分（升级种子 `[ADVICE_ONLY_ON_*]`/`NOT_ON_ART`/`NEEDS_POT`+level41/`NOT_ON_CRATER`+`IsPoolSquare`/Zen 水族馆/POTATOMINE/`NEEDS_GROUND`/`SLEEPING`）；`ClearAdvice` 系列；**扣阳光前置条件**（C++ 要求 `!easyCheat && cursor==PLANT_FROM_BANK && !HasConveyorBeltSeedBank()`，Rust 对非 BANK 光标也扣钱）；升级/替换 `Die()`（升级品/墙坚果/南瓜/玉米炮右侧/香蒲）；光标分派 GLOVE `MovePlant`、WHEEL_BARROW、USABLE_COIN、BANK `WasPlanted()+SetSleeping`；Column 整列种植 |
| 4.2 | `UpdateToolTip` | `:3222-3585`（363 行） | `:7458-7549` | 缺 LEVEL_INTRO 分支、铲子与全部工具（水壶/肥料/杀虫剂/留声机/巧克力/手套/独轮车/树肥）tooltip、种子包 tooltip 全部内容与阳光/升级警告；Rust 侧 `m_visible=false` 直接略过 |
| 4.3 | `UpdateCursor` | `:2931-3026` | `:6685` | 缺 seedchooser 早退、全部 `SetCursor`（DRAGGING/HAND/NONE/POINTER）、命中类型→光标形状、`mBeghouledMouseCapture` |
| 4.4 | `Pause` | `:4672-4693` | `:6221` | 仅 `m_paused = pause`；缺早退判定、`ShowCoinBank()`、`GamePause`/`GameMusicPause`、`TryToSaveGame()`（三者库中均已存在但未被调用） |
| 4.5 | `GetSeedTypeInCursor` | `:2006-2022` | `:5497` | 缺 WHEELBARROW 取盆栽种子、非植物光标返回 `SEED_NONE`、imitater 取 `mImitaterType` |
| 4.6 | `LawnMowerInitialize` | `LawnMower.cpp:30-85` | `lawn_mower.rs:59` | 仅 `row/state/visible`；缺初始坐标 `(-160, GetPosYBasedOnRow+23)`、`mRenderOrder`、各计数器、`StageHasRoof/PLANTROW_POOL` 类型判定、全部 `AddReanimation`（`anim_normal`/`anim_land`）、`EnableSuperMower` |

---

## 5. Challenge（小游戏/解谜模式）—— 整体骨架化

Rust 侧普遍只保留计数器递减与状态置位，关卡布局、`AddZombie`/`AddCoin`/`AddReanimation`、概率表、状态机分支基本未移植（相关调用已全库 `rg` 确认未搬到别处）。

| 函数 | C++ `cpp/src/Lawn/Challenge.cpp` | Rust `src/lawn/challenge.rs` |
|---|---|---|
| `ScaryPotterPopulate` 🔴 | `:3785`（221 行） | `:2277`（**1 行**：`scary_potter_pots = scary_potter_count_pots()`） |
| `IZombieInitLevel` 🔴 | `:4426`（226 行） | `:2531`（仅放 5 个 brain grid item） |
| `WhackAZombieSpawning` | `:2785`（115 行） | `:1180`（仅计数器） |
| `UpdateBeghouled` / `UpdateBeghouledPlant` | `:1547` / `:1384` | `:572` / `:582` |
| `IZombieUpdate` | `:4659` | `:2451` |
| `PuzzleNextStageClear` | `:4193` | `:2298` |
| `TreeOfWisdomInit` / `TreeOfWisdomUpdate` / `TreeOfWisdomFertilize` | `:5273` / `:5450` / `:5344` | `:3377` / `:3355` / `:3371` |
| `UpdateStormyNight` / `DrawStormNight` / `UpdateRain` / `UpdateRainingSeeds` | `:1961` / `:3085` / `:5076` / `:1932` | `:1257` / `:1248` / `:2842` / `:1419` |
| `PortalStart` / `GraveDangerSpawnRandomGrave` | `:3121` / `:2942` | `:1637` / `:1336` |
| `ScaryPotterUpdate` / `LastStandUpdate` / `LastStandCompletedStage` | `:4031` / `:5108` / `:5132` | `:2162` / `:3032` / `:3348` |
| `InitZombieWavesSurvival` / `GetArtChallengeSeed` | `:2499` / `:2239` | `:998` / `:763`（后者判定为等价重写，数据表未逐格核对） |

---

## 6. 界面层

| # | 项目 | Rust | 状态 |
|---|---|---|---|
| 6.1 | `ChallengeScreen::DrawButton` | `challenge_screen.rs:170` | 无任何 `g.*` 调用（仅 `TRANSLATION_NOTE` + `let _ = (g, …)`）→ 挑战按钮空白：缩略图、窗口框、挑战名、锁/奖杯/记录文本全缺 |
| 6.2 | `AwardScreen::DrawAchievements` / `AchievementsContinuePressed` | `award_screen.rs:467` / `:471` | 空实现 / 仅 `showing_achievements=false`；缺全部绘制与奖励页链式跳转 |
| 6.3 | `CreditScreen::DrawFogEffect` / `PreLoadCredits` / `UpdateMovie` | `credit_screen.rs:610` / `:765` / `:689` | 雾效空实现；预载缺 30+ reanim 与字体覆盖；`UpdateMovie` 缺全部 timed-event 粒子/音效、brain 位置、向日葵口型 |
| 6.4 | `MessageWidget::GetFont` / `SetLabel` | `message_widget.rs:104` / `:64` | 恒 `None`；缺 `PvzpStringTranslate`、`TruncateLabel`、重叠消息排队 `mLabelNext`、`LayoutReanimText` 与多个 style 时长 |
| 6.5 | `AlmanacDialog::SetPage` / `SetupPlant` / `ZombieIsShown` / `ZombieHasDescription` | `almanac_dialog.rs:72` / `:64` / `:378` / `:386` | 翻页无模型、植物页无植物、僵尸图鉴过早全显示、未通关也显示描述 |
| 6.6 | `SeedChooserScreen::OnStartButton` / `PickRandomSeeds` / `GetSeedPositionInChooser` / `ClickedSeedInBank` / `UpdateViewLawn` | `seed_chooser_screen.rs:479` / `:483` / `:185` / `:782` / `:378` | 前两者只会 `close_seed_chooser()`（"随机"不选种子、开战无校验）；缺 Imitater 按钮分支、`mSeedsInBank--`/重排、ViewLawn 位移动画与状态复位 |
| 6.7 | `GameSelector::ClickedAdventure` | `game_selector.rs:556` | 仅 `starting_game=true`；缺重玩确认/存档重置（`mLevel=24`、`EraseFile`）、`StopAllMusic`、`SOUND_LOSEMUSIC`、`REANIM_ZOMBIE_HAND` |

---

## 7. 接线断裂与基础设施

### 7.1 Rust 已实现但零调用（144 条中的重点，C++ 侧同名高频使用）

`alloc_reanimation`(`todlib/reanimator.rs:1557`)、`add_attached_reanim`(`zombie.rs:6721`)、`override_particle_scale`/`override_particle_color`(`zombie.rs:6773/6783`)、`i_zombie_place_plants`(`challenge.rs:2406`)、`scary_potter_place_pot`(`challenge.rs:2131`)、`scary_potter_change_pot_type`(`:2251`)、`whack_a_zombie_place_graves`(`:3038`)、`add_grave_stones`(`board.rs:5259`)、`check_seed_upgrade`(`widget/seed_chooser_screen.rs:460`)、`can_get_packet_upgrade`(`cutscene.rs:1364`)、`try_auto_collect_after_level_award`(`coin.rs:1219`)、`set_unlock_challenge_index`(`widget/challenge_screen.rs:58`)、`setup_plant`/`setup_zombie`(`widget/almanac_dialog.rs:64/68`)、`setup_volume_for_tune`(`system/music.rs:439`)、`lawn_message_box`/`center_dialog`/`write_to_registry`/`read_from_registry`/`show_resource_error`(`lawn_app.rs`)、`get_image_throw`/`get_sound_throw`/`get_font_throw`(`framework/resource_manager.rs:626/632/635`)、`process_safe_delete_list`(`framework/sexy_app_base.rs:939`)、`delete_resource`/`delete_resources`/`release_tracked_resources`(`framework/resource_manager.rs`)。

### 7.2 粒子系统 —— API 缺失 🔴

C++ `AddPvzpParticle` 共 **130 处调用**（`cpp/src/PvzpLib/PvzpParticle.h` 定义）；`src/` 全库 **零匹配**，也无等价入口（`tod_particle.rs` 仅有 `spawn_particle`/`alloc_particle_system` 等底层函数）。→ 所有依赖粒子的表现（掉头/断臂/掉盾/掉头盔/出土/水花/僵尸手臂/坚果阻挡等）无法接线。

### 7.3 其它基础设施差异

| 项目 | C++ | Rust | 差异 |
|---|---|---|---|
| `Attachment::Draw` / `OverrideScale` / `OverrideColor` | `PvzpLib/Attachment.cpp:458/339/238`（63/43/43 行） | `todlib/attachment.rs:816/752/737`（10 行） | 大幅简化 |
| `XMLParser::NextElement` | `misc/XMLParser.cpp:335`（383 行） | `framework/xml_parser.rs:147`（87 行） | 缺 section 栈与结束标签校验 |
| `MTRand::Serialize` | `misc/MTRand.cpp:199` | `framework/mt_rand.rs:172` | 缺 4 字节 `mti` → 与 C++ 存档/RNG 序列不互操作 |
| `Transform::Translate` | `misc/SexyMatrix.cpp:167` | `framework/sexy_matrix.rs:248` | 写错 `transX` 变量、漏 `mComplex` 分支 |
| `MemoryImage::GetBits` / `NormalBlt` / `AdditiveBlt` / `GetNativeAlphaData` / `GetRLAdditiveData` | `graphics/MemoryImage.cpp:1140/1321/1281/791/906` | `graphics/memory_image.rs:97/150/155/500/553` | 大幅简化（`NormalBlt`/`AdditiveBlt` 委托 `Image::blt`，等价性未确证） |
| `SDLSoundManager::GetSoundInstance` / `FindFreeChannel` / `GetNumSounds`、`SDLSoundInstance::Play`、`SDLMusicInterface::FadeIn` / `FadeOutAll` | `sound/*.cpp` | `framework/sound/*.rs` | 返回 `0` / 空实现 |
| `ResourceManager::LoadNextResource` / `LoadImage` / `LoadFont` / `ReplaceImage` / `Fail` | `misc/ResourceManager.cpp:911/778/887/1140/150` | `framework/resource_manager.rs:315/590/599/569/653` | 桩化或简化 |
| `regemu::SetRegFile` / `RegistryRead` | `misc/RegEmu.cpp:94/153` | `framework/reg_emu.rs:19/22` | 恒空实现 |
| `Dialog::WaitForResult` | `widget/Dialog.cpp:397` | `framework/widget/dialog.rs:176` | 空实现 → 模态等待语义缺失 |
| `FontData::LoadLegacy` | `graphics/ImageFont.cpp:1071`（67 行） | `graphics/image_font.rs:225`（5 行） | 简化 |
| `WidgetContainer::MarkDirty/Full`、`WidgetManager` 三函数、`Widget::WidgetRemovedHelper` | `widget/*.cpp` | `framework/widget/*.rs` | 简化重写 |
| `SexyAppBase::EnforceCursor` / `Popup` / `AddDialog`、`Sexy::XMLEncodeString` / `XMLDecodeString` | `SexyAppBase.cpp:2484/1945/900`、`Common.cpp:504/460` | `sexy_app_base.rs:2159/1319/1254`、`common.rs:503/494` | 部分简化 |

---

## 8. 已核对为一致（勿重复上报）

`RENDER_LAYER_*` 常量表（`game_enums.rs:630-646` 与 `ConstEnums.h:975-993` 逐值一致）、`MAX_POTTED_PLANTS = 200`、`Plant::get_cost`（Beghouled/I-Zombie/Imitater 分支齐全）、`PlayerInfo` 存档段（`LoadDetails`/逐项字节同步/`DataReader` 语义）、`Zombie::take_body_damage` 的类型分支主体、`Zombie::update_anim_speed`、`Zombie::get_zombie_attack_rect`、`Zombie::animate` 的咀嚼链（`animate_chew_sound`/`animate_chew_effect` 已接线）、`Zombie::update_zombie_polevaulter`、`Zombie::update_zombiquarium`、`Zombie::try_spawn_level_award` 的冒险奖励表、`Zombie::summon_backup_dancer`、`Zombie::hit_ice_trap`、`Board::add_sun_money`（9990 上限 + SunnyDays 成就）、`Board::ProcessDeleteQueue`、`Board::Count*ByType`、`Board::GetBossZombie`、`Board::GetGraveStonesCount`、`GridItem::open_portal`、`Music::StartGameMusic` 的夜/池/屋顶分支、`Coin::get_sun_value`（按类型返回 25/15/50/0）、`Sexy::Trim`、`Graphics::DrawImageMirror`、`ResourceManager::LoadFont`、`Zombie::DrawZombieHead`/`DrawZombieWithParts`（C++ 侧为注释掉的死代码）、`Buffer` 位流 API（已补齐）、demo 录制/回放（已移植）。`Reanimation::GetFrameTime`、`DataReader::OpenMemory` 为锚点错位（真实现分别在 `reanimator.rs:930`、`data_sync.rs:41`）。

**非差异（勿重复上报）**：`ContinueDialog::EmptyButtonListener` / `GameOverDialog` 空回调（构造期占位）、`NewOptionsDialog::update() {}`、`ZombatarTOS::button_press` 空、`CutScene::PurchasePacketSlotListener::dialog_button_press` 空、`DataWriter::open_file` 只建目录、`TLVReader::ReadU32` 固定 4 字节 LE。

---

## 9. 旧版（2026-09-17）报告条目状态

### 9.1 本轮复核为**已修复**（可不再跟踪）

| 旧条目 | 现状证据 |
|---|---|
| §1.2 `Zombie::UpdateAnimSpeed` | `zombie.rs:8699` 已重写（`is_on_board` 早退 + reanim 判定） |
| §1.3 `Plant::get_cost` | `plant.rs:2600+` 已补 Beghouled 特例 + I-Zombie 种子价格 + Imitater 分支 |
| §1.4 `Zombie::GetZombieAttackRect` | `zombie.rs:8279` 与 `Zombie.cpp` 逐项对齐（相位/倒走/`mClipHeight`） |
| §1.5 `Animate` 咀嚼链 | `animate_chew_sound`/`animate_chew_effect` 已在 `zombie.rs:4674/4675/4679/4682` 被调用 |
| §1.6 `TrySpawnLevelAward` 冒险奖励 | `zombie.rs:7466/7476/7480/7482` 已含 `CoinType::Note/Shovel/Carkeys/Taco` 等分支 |
| §1.7 `UpdateZombiquarium` | `zombie.rs:5900+` 已有 `pos_x += a_vel_x`、`summon_counter` 产阳光、`take_damage(10, 8)` |
| §1.8 `UpdateZombiePolevaulter` | `zombie.rs:1517` 已改用 `find_plant_target_index(ZombieAttackType::Vault)` |
| §1.13 `SummonBackupDancer` | `zombie.rs:2734` 已含 `row_can_have_zombie_type` 与 `mRelatedZombieID` 关联 |
| §1.15 `HitIceTrap` | `zombie.rs:4345` 已重写（`can_be_frozen`/`in_pool` 分支） |
| §1.16 `AddSunMoney` / `TakeSunMoney` | `board.rs:6186-6193` 已有 9990 上限与 `SunnyDays` 成就写入 |
| §1.17 `Coin::GetSunValue` | `coin.rs:1353` 已按类型返回 25/15/50/0 |
| §2.11 `GridItem::OpenPortal` | `grid_item.rs:772+` 已重写（位置计算 + `PortalSquare` 分支） |
| §2.13 `Music::StartGameMusic` | `music.rs:513` 已有 `stage_is_night` 分支 |
| §3.1 `RenderLayer` 常量 | `game_enums.rs:630-646` 与 C++ 逐值一致 |
| §3.4 `RandRangeInt` | 已提供 `todlib::tod_common::rand_range_int`（`challenge.rs:3657` 在用）；`rand_range`（半开区间）仍在别处使用，逐点误用未核 |
| §4.1 死代码 `animate_chew_*` | 见上，已有调用点 |

### 9.2 本轮复核为**仍存在**

| 旧条目 | 现状 |
|---|---|
| §1.1 `TakeBodyDamage` 的 Boss 钳位 | 函数主体已重写，但 **`mBodyHealth = 1` 钳位仍缺**（见本报告 §1.9） |
| §2.2 `DropHelm` | 仍为 4 行（本报告 §2.2） |
| §2.3 `DropShield` | 仍为 4 行（本报告 §2.1） |
| §2.7 `PlayDeathAnim` | 仍固定 `anim_death`（本报告 §2.6） |
| §2.8 `RiseFromGrave` | 泳池/陆地粒子与音效仍缺（本报告 §2.8） |

### 9.3 **未复核**（原结论保留，供后续复查）

§1.9 `UpdateZombieSnorkel`、§1.10 `UpdateZombieDolphinRider`、§1.11 `UpdateZombieBungee`、§1.12 `UpdateDamageStates`、§1.14 `UpdateZombieGargantuar`、§2.1 `DropHead`（本轮已独立确证仍缺）、§2.4 `UpdateYuckyFace`、§2.5 `MowDown`、§2.6 `UpdateZombieWalking`、§2.9 `ApplyBurn`、§2.10 `ZombieInitialize`（植物头 reanim / 电线杆变体 / 气球手臂渲染组）、§2.12 `PogoBreak`/`ZamboniDeath`/`UpdateZombieCatapult`/`UpdateZombieGatlingHead`/`UpdateZombieSquashHead`、§3.2 `DamageFlags` 死枚举、§3.3 `GameMode::ChallengeZombieNimble` 命名错位。

### 9.4 历史执行记录（原 `project/translation-audit-report.md` 摘要；该文件已删除）

该文件记录的前几轮落地结果如下（**历史记录，本轮未重新验证**，其声称的 `cargo check` 通过亦未复跑）：

- `todlib/**`：`PvzpCurveEvaluate` 双层求值、`pvzp_string_list_find` 查表、`apply_music_volume` 接线、`ReanimLoopType` 对齐
- 枚举对齐：`TutorialState`、`GridItemType`、`GameObjectType`、`GridItemState`、`Dialogs` 补齐、`PlantPriority`/`PlantingReason`/`RenderObjectType` 修正（共删 ~110 个 C++ 不存在的多余变体）
- 七项核心逻辑：`update_playing` groan 音效、`update_actions` 的 `HEIGHT_ZOMBIQUARIUM` 分支、`update_actions` 末尾 4 个僵尸头类型分支、`Projectile` COBBIG 的 `BLASTMARK`/`POPCORNSPLASH` 粒子、`animate_chew_effect` 的 Wallnut/Tallnut 粒子、`GridItem::draw` 的 `Brain`/`IZombieBrain` 映射、磁力菇 `get_track_position` 定位
- `LawnApp` 方法补入：`got_focus`/`lost_focus`、`modal_open`/`modal_close`、`init_hook`、`update_play_time_stats`、`pre_display_hook`/`loading_thread_completed`、`handle_cmd_line_param`、`update_app`/`update_app_step`、`preload_for_user`、`do_confirm_back_to_main`/`do_confirm_sell_dialog`、`lawn_message_box`、`do_register*`、`show_zombatar_tos`
- 该报告亦声明未完成：**运行期验证从未进行**；`GetTrackPosition`/`ReanimShowPrefix`/`ShouldTriggerTimedEvent` 等 reanim/粒子系统性 stub 仍在（与本报告 §7 结论一致）

---

## 10. 覆盖范围与未验证项

**已覆盖**：`cpp/src/Lawn`（含 `System`/`Widget`）、`cpp/src/LawnApp.cpp`、`cpp/src/SexyAppFramework`、`cpp/src/PvzpLib` 共 139 条行数悬殊候选逐条双端比对 + 144 条零调用点扫描 + 250 条名字级未命中项人工筛查。

**未覆盖 / 未确证**：

1. 未运行 `cargo check` / `cargo build`，无运行期验证。
2. C++ 有而 Rust 侧**连名字都不存在**的函数未做穷尽核对（仅筛查了 250 条名字级未命中的高价值项）。
3. `MemoryImage::NormalBlt` / `AdditiveBlt` 是否已被 `Image::blt` 等价实现未展开核验。
4. `Board::MouseDownWithPlant` 的升级/替换逻辑是否被 `Board::add_plant` 内部吸收未确证。
5. `Coin::CoinInitialize` 缺失的运动/视觉赋值是否被 `Board::add_coin` 之外路径补齐（已确认 `add_coin` 未补）。
6. `AlmanacDialog::ClearPlantsAndZombies` 中 `Die()`/`DieNoLoot()` 的副作用、Rust 是否存在 `mZombiePerfTest` 对应字段未确证。
7. `Challenge::GetArtChallengeSeed` 数据表内容未逐格核对。
8. §9.3 所列旧条目未复核。
9. 排除项：`SDL-Mixer-X`、`glad`、`fcaseopen`、`platform/emscripten`、`platform/switch`、`MusPlayer_Qt`、`EchoTune`、`pxtn*`、`PGE_*` 等第三方/调试工具不属游戏逻辑，未计入缺口。

---

## 11. 复现方法

```bash
# ① 文件级映射：cpp/src 与 src 的 basename 归一化比对
# ② 名字级差集：C++ "Class::Method(" 定义 vs Rust 全标识符（去下划线小写归一化）
# ③ 函数体行数悬殊候选：brace 平衡扫描 → C++ 体 ≥12 行且 Rust 体 ≤1/3
# ④ 零调用点：Rust pub fn 名在全库出现次数 == 1 且 C++ 同名出现 ≥4 次
# ⑤ 关键项复核
rg -n "fn find_lawn_mower_in_row" src/lawn/board.rs
rg -n "fn get_projectile_rect" -A8 src/lawn/projectile.rs
rg -c "AddPvzpParticle" cpp/src --glob '!SDL-Mixer-X'
rg -n "mBodyHealth = 1;" cpp/src/Lawn/Zombie.cpp
rg -n "fn (scary_potter_populate|whack_a_zombie_spawning|close_seed_chooser|give_achievement)" src/
```

> 本次核对脚本置于系统临时目录（未纳入仓库），源码未被修改。
