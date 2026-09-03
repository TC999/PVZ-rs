# PVZ-rs ↔ C++ 翻译差异核查报告（2026-09-03 重核版）

> 基线：HEAD `d6982dd`（2026-09-03 08:22，"绘制层: seed_packet draw_seed_packet 骨架"）。
> 对比 Rust 版（`src/`）与 C++ 版（`cpp/src/Lawn/`、`cpp/src/ConstEnums.h`）的翻译不一致。
> 方法：只读扫描——词干级函数匹配（camelCase↔snake，含 `BossRVAttack`/`WhackAZombie`/C++ 拼写错误 `Attact` 等特殊边界）+ 空函数体/占位扫描（EMPTY / STUB / TODO / TRANSLATION_NOTE）+ 关键逻辑抽查。**未修改任何源码**（git 工作区干净）。
> 编译基线：`cargo build` 成功（670 warnings，514 duplicates）。
>
> **本版相对 09-02 版及对话摘要的勘误**：`challenge.rs` 的 IZombie/Whack/Portal/Beghouled 系列、`cutscene.rs::PlaceAZombie`、`grid_item.rs::DrawIZombieBrain` 均有对应 `fn`（个别为空体 stub），**不是**"完全缺失"；此前逐行括号脚本因漏采多行签名 fn 造成误报，本版以词干匹配 + 人工复核为准。

---

## 一、高危逻辑不一致（函数存在但行为与 C++ 不符，优先修复）

1. **`board.rs::find_plant_at` 恒返回 `None`**（board.rs:1041，`find(|p| /* p.id == pid */ false)` 为占位）。
   后果：僵尸吃植物分支（board.rs:1004）恒走 `else` → 植物被啃两口即 `stop_eating()`，永远不触发植物的 `die()`/被吃动画与音效（C++ 走 `Board::ZombieEatPlant` + `Plant::Die`）。
2. **`board.rs::find_zombie_in_row` 忽略 `row`/`from_col`，恒返回僵尸列表第一只**（board.rs:1054）。
   植物开火主链 `update_abilities → update_shooter → find_target_and_fire → board.find_zombie_in_row`（plant.rs:568）使用该函数 → 射手可能隔行开火、无视目标列。plant.rs:647 `find_target_zombie`（按行+碰撞矩形加权，较接近 C++）未被主链采用。
3. **割草机逻辑两套脱节**：
   - `lawn_mower.rs::update`（77 行起）碰撞命中处为空操作（借用问题注释掉，未调 `mow_zombie`）；
   - 实际触发在 `board.rs` 碰撞循环中直接 `mower.start_mowing() + zombie.die_no_loot()`；
   - `lawn_mower.rs::mow_zombie`（153 行）**无任何调用者**。
   C++ 语义（`LawnMower::Update→MowZombie`：同机连续碾压多只、Bungee 僵尸跳伞躲避、碾压行推进、SuperMower 判定）丢失。
4. **特殊植物状态机大量"依赖底层系统"占位**（plant.rs 1539–2260，对照 C++ `UpdateXxx`）：
   - `update_doom_shroom`/`update_ice_shroom`：仅置状态，无爆炸范围伤害/冰冻/特效音效接入；
   - `update_chomper`：无啃咬目标判定，`Ready` 直接进 `Biting`；
   - `update_potato`：`PotatoArmed` 后无僵尸接近引爆判定；
   - `update_tanglekelp`：抓到后 `die()` 被注释；
   - `update_scaredy_shroom`：无僵尸靠近检测，仅状态空转；
   - `update_blover`：空体。
5. **`plant.rs::fire`/`find_target_and_fire` 简化**：`weapon` 参数被忽略；`Fumeshroom/Gloomshroom/Starfruit` 分支直接 `return`（C++ `Plant::Fire` 按 weapon 决定弹种/伤害/特效）。

---

## 二、完全缺失（C++ 有实现，Rust 全仓库无对应 fn；改名等价项已剔除）

| 文件 | 缺失函数 | 备注 |
|---|---|---|
| **zombie.rs**（22） | `AddAttachedReanim`、`AnimateChewEffect`、`AnimateChewSound`、`ApplyZombatarHead`、`BalloonPropellerHatSpin`、`BobsledBurn`、`BobsledDie`、`BungeeDie`、`BungeeDropPlant`、`DrawZombieHead`、`DrawZombiePart`、`DrawZombieWithParts`、`HasYuckyFaceImage`、`OverrideParticleColor`、`OverrideParticleScale`、`SetupReanimForLostArm`、`SetupReanimForLostHead`、`ShowYuckyFace`、`StartMindControlled`、`UpdateZombieGatlingHead`、`UpdateZombieJalapenoHead`、`UpdateZombiePeaHead`、`UpdateZombieSquashHead` | `drop_arm`/`drop_head`（4530/4536）只清字段，未做 reanim 头/臂掉落设置；`IsImmobilizied`→`is_immobilized`(3853)、`IsTanglekelpTarget`→`is_tangle_kelp_target`(3446)、`FindPlantTarget`→`find_plant_target_index`(2486)、`CountBungees…`→`count_bungees_targeting_sunflowers`(5914) 已改名存在；C++ `DrawZombie` 手工部件绘制由 `draw()`(4752)+`draw_reanim` 承担（reanim 驱动，非手工图层） |
| **plant.rs**（8） | `AnimateGarlic`、`AnimateNuts`、`AnimatePumpkin`、`DrawMagnetItems`、`DrawMagnetItemsOnTop`、`GetToolTip`、`UpdateBowling`、`UpdateReanimColor` | `StarFruitFire`→`launch_star_fruit`(594)、`MagnetShroomAttactItem`→`magnet_shroom_attack_item`(1806)、`GetFreeMagnetItem`→`get_free_magnet_item_idx`(1792)、`IsAGoldMagnetAboutToSuck`→`is_a_gold_magnet_about_to_suck`(2098) 已改名存在；`UpdateBowling` 全仓库无保龄球逻辑 |
| **board.rs**（11） | `AddBossRenderItem`、`CreateRakeReanim`、`DrawGameObjects`、`DrawDebugText`、`DrawDebugObjectRects`、`DrawHouseDoorBottom`、`DrawHouseDoorTop`、`SpecialPlantHitTest`、`ToolHitTestHelper`、`HighlightPlantsForMouse`、`ResetFPSStats` | 顶层 `draw()`(1426) 已驱动各实体绘制，但 C++ `DrawGameObjects/HouseDoor*/Debug*` 拆分未搬；`DrawUITop/UIBottom/UICoinBank/Shovel/ProgressMeter/ZenButtons/ZenWheelBarrowButton/TopRightUI` 部分并入 `draw_ui`(1545)（种子槽/阳光/铲子已画）；`AddACrater/AddAGraveStone/AddALadder`→`add_crater/add_grave_stone/add_ladder`、`CountSunFlowers`→`count_sunflowers`(2650)、`StageHas6Rows`→`stage_has_6_rows`(2204)、`GetIceZPos`→`get_ice_z_pos`(3249)、`PixelToGrid*KeepOnBoard`/`OffsetYForPlanting` 均已有 |
| **coin.rs**（9） | `CoinGetsBouncyArrow`、`DroppedUsableSeed`、`GetDisappearTime`、`IsLevelAward`、`IsPresentWithAdvice`、`PlayCollectSound`、`PlayGroundSound`、`PlayLaunchSound`、`TryAutoCollectAfterLevelAward` | `IsSun` 由初始化字段 `is_sun` + `is_sun_type()`(305) 等价；收集音效在 `collect`(239)/`score_coin`(310) 内以注释占位 |
| **projectile.rs**（1） | `PlayImpactSound` | `DoImpact`→`do_impact_by_index`(388)、碰撞→`check_for_collision`(333)/`find_collision_target_plant`(427) 已改名存在 |
| **message_widget.rs**（2） | `DrawReanimatedText`、`LayoutReanimText` | |
| **system/player_info.rs**（6） | `LoadDetails`、`SaveDetails`、`SyncDetails`、`SyncSummary`、`DeleteUserFiles`、`ResetChallengeRecord` | 存档读写只有基础容器方法 |
| **system/profile_mgr.rs**（1） | `SyncState` | |

**等价替代（不视为缺失）**：`DataSync::Sync*`→`read_u*/write_u*` 系列；`Music::PvzpLoadMusic`→`tod_load_music`；`PoolEffect::PoolEffectDraw/Initialize/Update`→`draw/initialize/update`（注意 `pool_effect.rs::draw`(167) 忽略 `is_night` 参数，夜晚水面绘制未区分）。

---

## 三、存在但为空体/stub/占位（扫描口径：EMPTY 空体 / STUB / TODO / 函数体含 TRANSLATION_NOTE，后者可能有部分实现）

扫描共标出 **329 个函数**，按文件分布（前 20）：

| 文件 | 数量 | 代表（C++ 均有实体逻辑） |
|---|---|---|
| zombie.rs | 42 | `load_plain_zombie_reanim`(5633)、`update_reanim`(5687)、`attach/detach_shield`(5666/…)、`setup_door_arms`(5740)、`setup_reanim_layers`(5743)、`show_door_arms`(5746)、`set_anim_rate`(5660)/`apply_anim_rate`、`reanim_show_prefix/show_track`(5649/5652)、`add_attached_particle`(5672→None)、`setup_zombatar_flag_reanim`(5737)、`drop_shield/drop_arm/drop_head`（简化） |
| lawn_app.rs | 29 | Crazy Dave 整套（`crazy_dave_enter/update/draw/talk_message/…`）、`show/kill_award_screen`、`do_almanac_dialog`、`do_pause_dialog`、`write_to_registry`、`switch_screen_mode`(1763 空)、`do_high_score_dialog`(1765 空)、`confirm_quit`（C++ LawnApp 共 181 方法，界面流程类为主） |
| plant.rs | 29 | reanim 动画接口全空：`set_body_reanim_frame/rate/loop/frame_base_pose`(1378–1387)、`add_head_reanim/2/3`(1390–1396)、`play_body_reanim`(1399)、`play_idle_anim`(1404)、`set_sleeping`(1286)、`preload_plant_resources`(1120)、`update_blover` |
| zen_garden.rs | 19 | `zen_garden_update/start/init_level`、`do_feeding_tool`、`potted_plant_update`、`plant_update_production`、`draw_potted_plant(+icon)` 等 |
| projectile.rs | 16 | `update` 分支、`do_impact_by_index` 内音效/特效占位 |
| cutscene.rs | 15 | `load_intro_board`、`load_upsell_board_pool/fog/roof`、`update_upsell`/`draw_upsell`、`preload_resources`、`clear_upsell_board` |
| board.rs | 14 | `save_game`、`setup_waves`、`update_fwoosh`、`process_delete_queue`(4047 空)、`do_typing_check`、`pick_up_tool`、`update_level_end_sequence`（简化）、`tutorial_arrow_show` |
| widget/credit_screen.rs | 13 | 09-02/09-03 已补 update/阶段推进/播放 reanim，仍以资源占位为主 |
| challenge.rs | 12 | **EMPTY 空体 9 个**：`draw_backdrop`(425)、`draw_art_challenge`(429)、`draw_beghouled`(519)、`draw_slot_machine`(683)、`draw_rain`(1788)、`tree_of_wisdom_draw`(2550)、`i_zombie_draw_plant`(1750)、`i_zombie_set_plant_filter_effect`(1754)、`i_zombie_setup_plant`(1985)；NOTE 3 个：`start_level`(102)、`update`(330)、`init_level`(810)。其余 IZombie/Whack/Portal/Beghouled 系列（`i_zombie_eat_brain` 1996、`i_zombie_get_brain_target` 2011、`i_zombie_place_*`、`i_zombie_score_brain` 2170、`whack_a_zombie_*` 等）已有实体 |
| system/music.rs | 9 | `music_init/dispose/update` 部分占位、`tod_load_music`/`load_song` 主体（openmpt 接入） |
| grid_item.rs | 8 | `update_portal`/`update_rake`/`update_scary_pot`、`draw_stinky`/`draw_scary_pot` 等 |
| widget/*（cheat_dialog 8、challenge_screen 7、lawn_dialog 7、award_screen 6、seed_chooser_screen 6、store_screen 6、game_button 5…） | ~70 | 对话框/商店/选卡界面，多为"依赖图片/3D 资源"注释占位 |
| coin.rs / lawn_mower.rs / seed_packet.rs / cursor_object.rs | 7 / 7 / 7 / 2 | `coin::draw`(230)、`lawn_mower::draw`(209)/`draw_shadow`(204)/`update_pool`(197)/`enable_super_mower`(193 空)、`seed_packet::draw`(153 空)/`mouse_down`/`flash_if_ready`/`pick_next_slot_machine_seed`、`cursor_object::draw`(120 空)/`plant_draw_seed_type`(368 空) |

> 注：09-02 版报告所列 widget 空函数（credit/challenge_screen 等）在 09-02/09-03 的"绘制层"提交中已有部分补齐（如 `CreditScreen update/update_blink/play_reanim`、`ChallengeScreen` 挑战定义表与解锁状态机、`seed_packet draw_seed_packet` 骨架）。

---

## 四、相对 09-02 版报告的进展（已修复/新增）

- 09-02 之后新增提交均集中于**绘制/UI 层**：CreditScreen 阶段机、ChallengeScreen 解锁与 72 项挑战表、seed_packet 绘制骨架、plant `draw_seed_type`/zen_garden 盆栽绘制接入。
- 09-02 版确认已修复项（GameMode/HelmType 枚举对齐、Boss 战斗系列、`can_plant_at` 种植限制、`is_zombie_allowed` 等）经复核在当前 HEAD 依然成立。
- zombie/plant 的伤害链（`take_damage` 4017 → 飞行物/盾牌/头盔/身体）与 Board 主循环（`update` 658 → update_game/碰撞/清理/状态/火焰）已较完整。

## 五、建议后续顺序

1. 高危项（第一节 1–3）：修复 `find_plant_at`/`find_zombie_in_row`，统一割草机触发路径——直接影响核心玩法可玩性；
2. coin/seed_packet/特殊植物效果接入（第一节 4–5 与第三节）；
3. 绘制/UI 资源层（第三节 widget/board/cutscene 占位）；
4. reanim/音效接口层（zombie/plant 动画接口全空，为绘制与特效的共同依赖，建议优先于 3）。
