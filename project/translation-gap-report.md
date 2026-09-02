# PVZ-rs 翻译缺口分析报告（当前代码重核版）

> 对比 Rust 版 (PvZ Portable) 与 C++ 版 (PvPZ) 之间的翻译不一致
> 旧版报告（09-01）已作废：09-02 大批提交后其结论多处过时，本报告基于当时最新 `src/lawn/` 与 `cpp/src/Lawn/` 逐函数核对重新生成。
> 核对方式：只读扫描（函数名归一化对比 + 空函数体扫描 + 关键函数抽查），未修改任何源码。

**编译状态**：`cargo build` 成功（665 warnings）。

---

## 〇、相对旧报告的修复确认（旧版"严重问题"多数已消除）

- **GameMode 枚举**：已补齐 73 个值并显式对齐 C++ 数值（`game_enums.rs` 171 行起注释），ScaryPotter / I_Zombie 已拆分到各关卡，存档文件名错位问题消除。
- **HelmType 枚举**：已补齐 `Redeyes/Headband/Bobsled/Wallnut/Tallnut`（值 5–9）。
- **cutscene.rs 判定类函数**：`is_showing_crazy_dave`、`is_in_shovel_tutorial`、`is_survival_repick`、`is_scrolled_left_at_start`、`can_get_packet_upgrade`、`show_zombie_walking`、`should_run_upsell_board`、`is_non_scrolling_cutscene` 等已从硬编码 `false` 改为真实逻辑。
- **zombie.rs Boss 系列**：`update_boss` + `boss_play_idle/boss_rv_attack/boss_rv_landing/boss_spawn_attack/boss_spawn_contact/boss_stomp_attack/boss_stomp_contact/boss_bungee_attack/boss_bungee_spawn/boss_bungee_leave/boss_head_attack/boss_head_spit/boss_destroy_iceball_in_row/boss_destroy_fireball/boss_head_spit_effect/boss_head_spit_contact/update_boss_fireball/boss_start_death/boss_die` 均已翻译；`draw()`/`draw_reanim`/`draw_bobsled_reanim`/`draw_bungee_reanim`/`draw_bungee_target`/`draw_dancer_reanim`/`draw_bungee_cord`/`draw_ice_trap`/`draw_butter` 已有实体。
- **challenge.rs**：Beghouled / BeghouledTwist / ScaryPotter / Portal / Squirrel / Zombiquarium / 打地鼠等系列均已实现；`can_plant_at` 已含完整种植限制逻辑（墙果保龄球线 / I,Zombie 半场 / 艺术挑战 / Boss 行限制），不再恒返回 `Ok`。

---

## 一、仍缺失/不一致的差异总览（按文件）

### 1. 完全缺失的函数（C++ 有实现、全仓库无对应 `fn`）

| 文件 | 缺失函数 |
|---|---|
| **board.rs** | `AddACrater`、`AddAGraveStone`、`AddALadder`、`DrawBackdrop`、`DrawGameObjects`、`DrawHouseDoorBottom/Top`、`DrawUITop/Bottom/TopRightUI/UICoinBank/Shovel/ZenButtons/ProgressMeter`、board 层 `KeyChar`、`SpecialPlantHitTest`、`ToolHitTestHelper`、`HighlightPlantsForMouse`、`GetZenButtonRect`、`AddBossRenderItem`、`CreateRakeReanim`、`CountSunflowers`、`DrawDebugText/DrawDebugObjectRects` |
| **plant.rs** | `Animate/AnimateGarlic/AnimateNuts/AnimatePumpkin`、`UpdateReanim(+Color)`、`UpdateBowling`、`DrawMagnetItems(OnTop)`、`GetFreeMagnetItem`、`MagnetShroomAttactItem`、`StarFruitFire`、`MakeRenderOrder`、`GetImage`、`GetToolTip`、`PreloadPlantResources`、`AddAttachedParticle`、`DrawSeedType` |
| **zombie.rs** | `DrawZombieWithParts/DrawZombiePart/DrawZombieHead`、`FindPlantTarget`、`SetupReanimForLostArm/Head`、`UpdateZombieGatling/Pea/Jalapeno/SquashHead`、`ApplyZombatarHead`、`ShowYuckyFace/HasYuckyFaceImage`、`BobsledBurn/BobsledDie`、`BungeeDie/BungeeDropPlant`、`StartMindControlled`、`PreloadZombieResources`、`OverrideParticleColor/Scale`、`BalloonPropellerHatSpin`、`AddAttachedReanim`、`StrFormat`、`IsImmobilizied` |
| **coin.rs** | `IsSun`、`IsLevelAward`、`IsPresentWithAdvice`、`CoinGetsBouncyArrow`、`GetDisappearTime`、`PlayCollectSound`、`PlayGroundSound`、`PlayLaunchSound`、`MouseDown`、`DroppedUsableSeed`、`TryAutoCollectAfterLevelAward`（11 个） |
| **seed_packet.rs** | 传送带系列 `AddSeed/RemoveSeed/RefreshAllPackets/GetNumSeedsOnConveyorBelt/CountOfTypeOnConveyorBelt/UpdateWidth/ContainsPoint`（注：`update_conveyor_belt` 在 challenge.rs:924，不在 seed_packet） |
| **projectile.rs** | `DoImpact`、`FindCollisionTarget`、`PlayImpactSound` |
| 其他 | `lawn_common.rs::KeyText`；`grid_item.rs` 无独立 `DrawGridItem` |

### 2. 函数体为空的占位（C++ 对应函数有实质逻辑）

| 文件 | 空函数数 | 代表函数 |
|---|---|---|
| **lawn_app.rs** | 36 | `check_for_game_end`、`end_level`、`show/kill_award_screen`、`show_seed_chooser_screen`、`show/kill_store_screen`、`do_almanac_dialog`、`do_pause_dialog`、Crazy Dave 整套（`crazy_dave_enter/update/leave/draw/die/stop_talking/talk_index/talk_message`）、`play_foley/play_foley_pitch/play_sample`、`toggle_slow/fast_mo`、`write_to_registry`、`write_current_user_config`、`update_player_profile_for_finishing_level`、`confirm_quit`、`switch_screen_mode`、`do_high_score_dialog` 等（对应 `LawnApp.cpp` 181 个方法） |
| **zen_garden.rs** | 26 | `zen_garden_update/start/init_level`、`draw_backdrop/draw_potted_plant(+icon)`、`draw_plant_overlay`、`mouse_down_with_money_sign`、`mouse_down_with_full/empty_wheel_barrow`、`goto_next_garden`、`leave_garden`、`open_store`、`do_feeding_tool`、`feed_chocolate_to_plant`、`plant_update_production`、`add/remove_happy_effect`、`zen_tool_update`、`advance_crazy_dave_dialog`、`show_tutorial_arrow_on_watering_can`、`setup_for_zen_tutorial`、`update_plant_effect_state`、`potted_plant_update`、`set_plant_anim_speed`、`plant_set_launch_counter` |
| **cutscene.rs** | 13 | `load_intro_board`、`load_upsell_board_pool/fog/roof`、`load_upsell_challenge_screen`、`update_upsell`、`draw_upsell/draw_intro`、`add_upsell_zombie`、`cancel_intro`、`animate_board`、`preload_resources`、`clear_upsell_board` |
| **board.rs** | 14 | `save_game`、`setup_waves`、`update_fwoosh`、`update_layers`、`process_delete_queue`、`do_typing_check`、`pick_up_tool`、`survival_save_score`、`puzzle_save_streak`、`update_progress_meter`、`freeze_effects_for_cutscene`、`stop_all_zombie_sounds`、`reset_fps_stats`、`remove_particle_by_type` |
| **zombie.rs** | 21 | reanim/声音辅助：`update_reanim`、`load_plain_zombie_reanim`、`start_walk_anim`、`set/apply_anim_rate`、`reanim_show_prefix/show_track`、`reanim_ignore_clip_rect/reenable_clipping`、`drop_loot`、`update_mowered`、`attach/detach_shield`、`setup_door_arms`、`setup_reanim_layers`、`show_door_arms`、`setup_water_track`、`enable_mustache/enable_future`、`setup_zombatar_flag_reanim` |
| **plant.rs** | 11 | reanim 动画接口：`set_body_reanim_frame/rate/loop/base_pose`、`add_head_reanim(1/2/3)`、`play_body_reanim`、`play_idle_anim`、`update_blover`、`set_sleeping` |
| **lawn_mower.rs** | 4 | `draw`、`draw_shadow`、`update_pool`、`enable_super_mower` |
| **challenge.rs** | 9 | 纯绘制：`draw_backdrop`、`draw_rain`、`draw_art_challenge`、`draw_beghouled`、`draw_slot_machine`、`tree_of_wisdom_draw`、`i_zombie_draw_plant`、`i_zombie_setup_plant`、`i_zombie_set_plant_filter_effect` |
| **coin.rs** | 1 | `draw` |
| **projectile.rs** | 1 | `draw_shadow` |
| **seed_packet.rs** | 4 | `draw`、`mouse_down`、`flash_if_ready`、`pick_next_slot_machine_seed` |
| **cursor_object.rs** | 2 | `draw`、`plant_draw_seed_type` |

### 3. widget/ 子目录空函数（对话框/界面层）

| 文件 | 空函数数 | 文件 | 空函数数 |
|---|---|---|---|
| credit_screen.rs | 18 | lawn_dialog.rs | 12 |
| challenge_screen.rs | 11 | user_dialog.rs | 11 |
| new_user_dialog.rs | 8 | store_screen.rs | 7 |
| cheat_dialog.rs | 7 | continue_dialog.rs | 7 |
| almanac_dialog.rs | 5 | award_screen.rs | 5 |
| seed_chooser_screen.rs | 5 | game_button.rs | 4 |
| new_options_dialog.rs | 4 | imitater_dialog.rs | 3 |
| game_selector.rs | 3 | achievements_screen.rs | 2 |

### 4. system/ 子目录空函数（少量）

`music.rs` 1、`profile_mgr.rs` 1、`reanimation_lawn.rs` 3。`save_game.rs`(43)、`player_info.rs`、`data_sync.rs`(50)、`typing_check.rs`、`pool_effect.rs` 无空函数。

---

## 二、行为简化 / 注释保留的差异（`TRANSLATION_NOTE` 残留）

这些函数有实体代码，但与 C++ 不一致：相关逻辑被注释保留原文或以简化写法替代。

- **plant.rs**（约 27 处）：reanim 动画链路未接入 —— `update()` 仅 `update_abilities()`+血量检查，C++ 的 `Animate/UpdateReanim/DoBlink` 流程缺失；玉米炮 / 模仿者 / 磁力菇 / 大蒜等处的轨道触发、粒子、音效多为注释（`ShouldTriggerTimedEvent`、`FOLEY_*`、`ReanimShowPrefix`、`AttachEffect` 等）。
- **board.rs**（约 12 处 + 空函数）：`update()`（628 行）注释声明"完整 C++ 实现还包含 CutScene 更新、鼠标位置更新、按钮更新、震动等"；波次音效、`FadeOutLevel()`、粒子 ID 系统、大波提示、特殊模式波次间隔等未接入；`draw()` 相关缺第一节 UI/背景函数。
- **zombie.rs**（约 36 处）：reanim/particle/音效轨道大量以注释保留 C++ 原文；另有"comment garbled in local file"（822/3319/3385/3406/3445–3538 行区域）与 `boss_die` 内 `AttachEffect(PARTICLE_ZAMBONI_SMOKE)` 等未接入。
- **lawn_app.rs**（13 处）：部分模式判定为硬编码/近似（如挑战定义缺失导致的值近似）。

**共性根因**：reanim（动画）、particle（粒子）、音效三大子系统在游戏对象上的接入不完整，是"有骨架无血肉"差异的主来源。

---

## 三、差异本质归类

1. **绘制/UI 层整体缺失**：Board/Plant/Zombie 的绘制、全部背景/UI 绘制函数，及 widget 对话框、Crazy Dave、商店、奖杯/奖励屏幕多空 —— 画面与交互不完整。
2. **reanim / particle / 音效未接入游戏对象**：大量 `TRANSLATION_NOTE` 注释原文代替实际调用。
3. **应用层流程空壳**：`lawn_app.rs` 的界面切换/对话框/Crazy Dave/注册表写入为空；`board::save_game` 等存档入口为空（`system/save_game.rs` 本体无空，但调用链未核对）。
4. **已消除项**：枚举数值错位、Boss 战、Beghouled 等挑战模式逻辑、cutscene 状态判定（见第〇节）。

---

## 四、优先级建议（按严重程度）

| 优先级 | 模块 | 主要问题 | 影响 |
|---|---|---|---|
| 🔴 最高 | lawn_app.rs + widget/ | 36 + 约 115 个空函数 | 主界面流程/对话框/商店/奖励不可用 |
| 🔴 最高 | zen_garden.rs | 26 空 | 禅境花园交互/喂养/商店不可用 |
| 🔴 最高 | reanim/particle/音效接入 | plant/zombie/board 大量 `TRANSLATION_NOTE` | 动画特效音效缺失 |
| 🟠 高 | 绘制层 | board/plant/zombie/coin/seed_packet 的 draw 及 DrawBackdrop 等 | 画面不完整 |
| 🟠 高 | cutscene.rs | 13 空（过场/UPSELL 板） | 过场与导入流程缺 |
| 🟡 中 | board.rs | 14 空 + update() 简化 | 存档/波次/打字/进度条缺失 |
| 🟡 中 | coin/seed_packet/projectile/zombie/plant | 缺失辅助函数（音效、传送带、碰撞目标等） | 局部逻辑不完整 |
| 🟢 低 | lawn_mower/grid_item/system | 少量空函数 | 局部不完整 |

---

*本报告为只读核对产物；`src/todlib`、`src/framework` 及 widget 各对话框内部逻辑未逐函数比对，可按需继续深入。*
