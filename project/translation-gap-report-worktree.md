# PVZ-rs ↔ C++ 翻译差异核查报告（当前工作区，2026-09 重核）

> 对比 Rust 版 (`src/lawn/`) 与 C++ 版 (`cpp/src/Lawn/`, `cpp/src/ConstEnums.h`) 的翻译不一致。
> 本报告基于**当前工作区**（含未提交改动，git status 显示 14 文件 +4281/-340 行）重新生成，
> 以函数级脚本对比 + 枚举逐项比对为准，旧版报告 `translation-gap-report.md` 部分结论已过时（见第二节"已修复项"）。

## 一、检查方法

- 脚本正则提取 `cpp/src/Lawn/*.cpp` 全部类方法，与 `src/lawn/*.rs` 的 `fn` 做 camelCase→snake_case 匹配；
  跨文件引用（`Plant::Xxx`、`ReportAchievement::GiveAchievement`、`std::clamp/min/max`）已剔除，不计入缺失。
- 枚举与 `cpp/src/ConstEnums.h` 逐项比对（GameMode / HelmType / ZombieType / SeedType 等）。
- 函数体内占位标记（`TRANSLATION_NOTE` / `todo!` / `unimplemented!`）按文件统计。
- 本报告未修改任何源码文件。

## 二、✅ 已修复项（相对旧报告，工作区已改好）

1. **GameMode 枚举数值错位已修复**：Rust 现为 0–73 显式对齐 C++，补齐全部 24 个缺失模式
   （`ChallengeWarAndPeas2`=32、`ChallengeArtChallengeWallnut`=36、`ChallengeBigTime`=39、
   `ChallengeArtChallengeSunflower`=40、`ChallengeAirRaid`=41、`ChallengeHighGravity`=44、
   `ChallengeGraveDanger`=45、`ChallengeShovel`=46、`ChallengeStormyNight`=47、`ChallengeBungeeBlitz`=48、
   `ChallengeSquirrel`=49，及 `ScaryPotter2-9/Endless`=52–60、`PuzzleIZombie2-9/Endless`=62–70）。
   存档文件名 (`userdata/game{id}_{mode}.v4`) 错乱风险解除。
2. **HelmType 缺失 5 值已补齐**：`Redeyes`=5、`Headband`=6、`Bobsled`=7、`Wallnut`=8、`Tallnut`=9。
3. **ZombieType 与 C++ 一致**（含 `RedeEyeGargantuar` 末尾位）。
4. **硬编码绕过已修复**：`can_plant_at` 现含真实种植限制（保龄球线 / IZombie 行数限制）；
   `is_zombie_allowed`、`can_use_game_object` 均有真实判定，不再是恒 `true`。

## 三、❌ 当前仍完全缺失的函数（脚本实测，共约 126 个）

### zombie.rs — 41 个（Boss 战 / 绘制 / 头部特效全缺）

- **Boss 战斗**：`BossBungeeAttack` `BossBungeeLeave` `BossBungeeSpawn` `BossDestroyFireball`
  `BossDestroyIceballInRow` `BossDie` `BossHeadAttack` `BossHeadSpit` `BossHeadSpitContact`
  `BossHeadSpitEffect` `BossPlayIdle` `BossRVAttack` `BossRVLanding` `BossSpawnAttack`
  `BossSpawnContact` `BossStartDeath` `BossStompAttack` `BossStompContact` `UpdateBossFireball`
- **绘制**：`DrawZombie` `DrawZombieWithParts` `DrawZombiePart` `DrawReanim` `DrawBossPart`
  `DrawBossBackArm` `DrawBossFireBall` `DrawButter` `DrawBobsledReanim` `DrawBungeeCord`
  `DrawBungeeReanim` `DrawBungeeTarget` `DrawDancerReanim` `DrawIceTrap`
- **头部/部件**：`UpdateZombieGatlingHead` `UpdateZombieJalapenoHead` `UpdateZombiePeaHead`
  `UpdateZombieSquashHead` `ApplyZombatarHead` `BalloonPropellerHatSpin` `SetupReanimForLostArm`
  `SetupReanimForLostHead` `ShowYuckyFace` `HasYuckyFaceImage` `OverrideParticleColor` `OverrideParticleScale`
- **行为**：`MowDown` `BungeeDie` `BungeeDropPlant` `BobsledBurn` `BobsledDie` `StartMindControlled`
  `FindPlantTarget` `IsImmobilizied` `GetTrackPosition` `SetupDrawZombieWon` `AddAttachedReanim`
  `AnimateChewEffect` `AnimateChewSound` `CountBungeesTargetingSunFlowers` `IsTanglekelpTarget`
  `PreloadZombieResources`

### board.rs — 34 个（UI/场景绘制与若干流程）

- **绘制**：`DrawBackdrop` `DrawLevel` `DrawFog` `DrawIce` `DrawUIBottom` `DrawUITop` `DrawTopRightUI`
  `DrawUICoinBank` `DrawShovel` `DrawZenButtons` `DrawZenWheelBarrowButton` `DrawProgressMeter`
  `DrawHouseDoorBottom` `DrawHouseDoorTop` `DrawGameObjects` `DrawDebugText` `DrawDebugObjectRects`
- **流程/交互**：`AddACrater` `AddAGraveStone` `AddALadder` `GetGraveStonesCount` `CountSunFlowers`
  `CreateRakeReanim` `DoFwoosh` `UpdateLevelEndSequence` `HighlightPlantsForMouse` `SpecialPlantHitTest`
  `ToolHitTestHelper` `MouseDownCobcannonFire` `GetZenButtonRect` `StageHas6Rows` `UpdateGridItems`
  `AddBossRenderItem` `KeyChar`

### plant.rs — 19 个

`Animate` `AnimateGarlic` `AnimateNuts` `AnimatePumpkin` `UpdateReanim` `UpdateReanimColor`
`FindSquashTarget` `StarFruitFire` `UpdateBowling` `MagnetShroomAttactItem` `GetFreeMagnetItem`
`DrawMagnetItems` `DrawMagnetItemsOnTop` `DrawSeedType` `GetImage` `GetPeaHeadOffset` `GetToolTip`
`AddAttachedParticle` `PreloadPlantResources`

### coin.rs — 11 个

`IsSun` `IsLevelAward` `IsPresentWithAdvice` `CoinGetsBouncyArrow` `GetDisappearTime`
`PlayCollectSound` `PlayGroundSound` `PlayLaunchSound` `MouseDown` `DroppedUsableSeed`
`TryAutoCollectAfterLevelAward`

### grid_item.rs — 9 个（全部为 draw 系列）

`DrawGridItem` `DrawGridItemOverlay` `DrawGraveStone` `DrawCrater` `DrawLadder` `DrawScaryPot`
`DrawIZombieBrain` `DrawStinky` `AddGraveStoneParticles`

### seed_packet.rs — 8 个（传送带相关全缺）

`AddSeed` `RemoveSeed` `UpdateConveyorBelt` `RefreshAllPackets` `GetNumSeedsOnConveyorBelt`
`CountOfTypeOnConveyorBelt` `UpdateWidth` `ContainsPoint`

### projectile.rs — 3 个

`DoImpact` `FindCollisionTarget` `PlayImpactSound`

### 其余文件

- `challenge.rs`、`cursor_object.rs`、`lawn_mower.rs`、`tool_tip_widget.rs`、`game_object.rs`：函数名层面**无缺失**。
- `cutscene.rs`：仅 `Is2x2Zombie` 为命名差异（Rust 用 `is_2x2_zombie`，已实现，非缺失）。
- `lawn_common.rs`：缺 `LawnEditWidget::KeyText`（属另一类，非 LawnCommon）。

## 四、⚠️ 有函数但逻辑缺失/占位（`TRANSLATION_NOTE` 共 195 处，按文件分布）

| 文件 | 标记数 | 主要缺口 |
|---|---|---|
| zombie.rs | 74 | `update_boss` 内大段注释掉的原始逻辑、`apply_burn`/`apply_butter`、`attach_shield`、`setup_reanim_layers`、`drop_loot`、`start_walk_anim`、`update_mowered`、`enable_mustache`/`enable_future` 等 |
| plant.rs | 27 | `set_body_reanim_frame/loop/rate`、`play_body_reanim`、`update_torchwood`/`update_blover`、`cob_cannon_fire`、`magnet_shroom` 系列、`update_imitater` 等 |
| projectile.rs | 20 | `update_motion` 缺坡度/`PixelToGridYKeepOnBoard`/`AttachmentUpdateAndMove`、`update_lob_motion`、`do_splash_damage` 等 |
| lawn_app.rs | 12 | `is_squirrel_level`/`is_shovel_level` 等模式判定 |
| coin.rs | 9 | `update`/`collect` 中注释掉的音效与场景分支 |
| board.rs | 9 | `update` 缺 CutScene 更新/鼠标/按钮/震屏、`init_zombie_waves`、`update_zombie_spawning` |
| lawn_mower.rs | 9 | `draw`/`draw_shadow`/`update_pool`/`enable_super_mower` 空 |
| grid_item.rs | 7 | `update`/`update_portal`/`update_scary_pot`/`update_rake` 部分实现 |
| zen_garden.rs | 7 | 全项目最薄弱（见下） |
| seed_packet.rs | 6 | `update`/`was_planted` 部分实现 |
| cutscene.rs | 5 | `update`/`draw_intro` 等过场逻辑空 |
| challenge.rs | 5 | `beghouled_is_valid_move`/`update_stormy_night`/`update_rain` 等 |

补充：`todo!()`/`unimplemented!()` 共 227 处散落各文件。

### zen_garden.rs（旧报告 77 空/占位，翻译率 ~5%，仍为最薄弱）

- **盆栽**：`add_potted_plant`/`remove_potted_plant`/`place_potted_plant`/`move_plant`、
  `get_potted_plant_in_wheelbarrow`→None、`potted_plant_from_index`→None、`find_open_zen_garden_spot`→false
- **坐标转换**：`pixel_to_grid_x/y`→0、`grid_to_pixel_x/y`→0、`get_special_grid_placements`→None、
  `zen_plant_offset_x`→0、`plant_potted_draw_height_offset`→0.0
- **鼠标交互**：`mouse_down_zen_garden`→false、`mouse_down_with_tool`/`feeding_tool`/`money_sign`/`wheel_barrow`
- **臭鼬系统**：`stinky_update`、`stinky_pick_goal`、`stinky_wake_up`、`stinky_start/finish_falling_asleep`、
  `add_stinky`、`get_stinky`→None、`wake_stinky`、`should_stinky_be_awake`、`is_stinky_sleeping`→false、
  `is_stinky_high_on_chocolate`→false、`update_stinky_motion_trail`、`stinky_anim_rate_update`
- **浇水/施肥/巧克力**：`plant_fulfill_need`、`plant_watered`、`plant_fertilized`、
  `was_plant_need_fulfilled_today`、`was_plant_fertilized_in_last_hour`、`plant_should_refresh_need`→false、
  `plant_can_be_watered`→false、`plant_can_have_chocolate`→false、`plant_high_on_chocolate`→false、
  `count_plants_needing_fertilizer`→0、`all_plants_have_been_fertilized`→false、`update_plant_needs`、
  `refresh_plant_needs`、`feed_chocolate_to_plant`、`set_plant_anim_speed`、`plant_set_launch_counter`、
  `plant_get_minutes_since_happy`→0、`plant_update_production`、`update_plant_effect_state`
- **绘图**：`draw_backdrop`、`draw_plant_overlay`、`draw_potted_plant`、`draw_potted_plant_icon`
- **其他**：`get_plant_sell_price`→0、`zen_garden_update`、`zen_garden_start`、`zen_garden_init_level`、
  `is_zen_garden_full`→false、`goto_next_garden`、`leave_garden`、`open_store`、`do_feeding_tool`、
  `zen_tool_update`、`setup_for_zen_tutorial`、`has_purchased_stinky`→false、
  `show_tutorial_arrow_on_watering_can`、`plants_need_water`、`add_happy_effect`、`remove_happy_effect`、
  `advance_crazy_dave_dialog`、`reset_plant_timers`、`reset_stinky_timers`

## 五、结论与优先级

1. **存档兼容性 bug（旧报告最高危项）已解除**，枚举层与 C++ 对齐完毕。
2. 当前主要差异集中在：
   - 🔴 全部实体绘制（Zombie/Plant/Board/GridItem 的 draw 系统近乎为 0，画面不完整）
   - 🔴 Boss 战全套逻辑（zombie.rs 41 缺失中近半是 Boss 相关）
   - 🔴 zen_garden（~5% 翻译率）
   - 🟠 cutscene（~29%）
   - 🟠 传送带/磁铁/保龄球等特殊植物能力（seed_packet/plant 缺失项）
3. 函数级缺失约 126 个、带注释缺口约 195 处——工作区近 4281 行新增改动主要修复了枚举与判定逻辑，
   尚未触及绘制与 Boss 两大块。

## 附：函数级对比统计（当前工作区实测）

| 文件 | C++ 函数 | Rust fn | 完全缺失（真缺失） |
|---|---|---|---|
| Board.cpp→board.rs | 243 | 251 | 34 |
| Zombie.cpp→zombie.rs | 227 | 196 | 41 |
| Challenge.cpp→challenge.rs | 159 | 162 | 0 |
| Plant.cpp→plant.rs | 102 | 100 | 19 |
| ZenGarden.cpp→zen_garden.rs | 82 | 83 | 0（函数名齐全，逻辑空） |
| CutScene.cpp→cutscene.rs | 51 | 58 | 0（1 命名差异） |
| Coin.cpp→coin.rs | 30 | 21 | 11 |
| Projectile.cpp→projectile.rs | 25 | 24 | 3 |
| SeedPacket.cpp→seed_packet.rs | 23 | 17 | 8 |
| GridItem.cpp→grid_item.rs | 19 | 14 | 9 |
| LawnMower.cpp→lawn_mower.rs | 10 | 15 | 0 |
| ToolTipWidget.cpp→tool_tip_widget.rs | 6 | 11 | 0 |
| CursorObject.cpp→cursor_object.rs | 4 | 11 | 0 |
| GameObject.cpp→game_object.rs | 3 | 9 | 0 |
| LawnCommon.cpp→lawn_common.rs | 3 | 13 | 0 |
| **合计** | **约 987** | **约 965** | **约 126** |
