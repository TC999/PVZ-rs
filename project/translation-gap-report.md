# PVZ-rs ↔ C++ 翻译差异核查报告（2026-09-04 重核版）

> 基线：HEAD `b5609a8`（2026-09-03 22:43，"board: 补齐 ProcessDeleteQueue"）。
> 对比 Rust 版（`src/`）与 C++ 版（`cpp/src/Lawn/`、`cpp/src/ConstEnums.h`）的翻译不一致。
> 方法：只读扫描——词干级函数匹配（camelCase↔snake，含 `BossRVAttack`/`WhackAZombie`/C++ 拼写错误 `Attact` 等边界）+ 空函数体/占位扫描（EMPTY / STUB / TODO / TRANSLATION_NOTE）+ 关键逻辑抽查。**未修改任何源码**。
> 编译基线：`cargo build` 成功（676 warnings，518 duplicates）。
>
> 本版相对上一版（基线 `d6982dd`，86 行）的更新：其间新增 13 个提交、+2990 行，逐条修复了上版报告的一、二节大部分内容，本版相应改写为"已实质修复"与"仍存在差异"两部分。

---

## 一、相对上版已实质修复（抽查验证为真实现，非骨架）

1. **`board.rs::find_plant_at` 恒 None** → 已改为 C++ `GetTopPlantAt` 语义（TOPPLANT_EATING_ORDER：南瓜 > 正常植物 > 底层植物，board.rs:1045）；僵尸吃植物路径正常扣 `DAMAGE_PER_EAT`、设 `recently_eaten_countdown`、死亡时 `die()`+`m_plants_eaten` 计数+`stop_eating()`。
2. **`board.rs::find_zombie_in_row` 忽略参数** → 已改为同行 + 植物右侧（`z_x >= plant_x`）+ 最靠左（board.rs:1061）。
3. **割草机两套逻辑脱节** → 已统一到 `Board::check_collisions`→`mower.mow_zombie()`，含 C++ 语义：Balloon 重叠阈值 20、Bungee/无头僵尸不触发、`effected_by_damage(127)`、Boss 跳过；`lawn_mower.rs::mow_zombie` 成为实际调用路径。`lawn_mower.rs::update` 内仅剩场景检查/水花音效占位（集中式设计，可接受）。
4. **特殊植物状态机** → chomper 完整 doBite/doMiss（Gargantuar/Redeye/Boss 咬 40 伤、跳跳杆/撑杆预跳 miss、`is_immobilized` 判定）；potato 僵尸接近引爆（`find_target_zombie`→`do_special`）；tanglekelp 抓取→`drag_under`+`pool_splash`；scaredy_shroom 圆形 120 半径僵尸检测状态机；doom/ice_shroom 爆炸动画+音效接入。
5. **`plant.rs::fire`** → 完整 weapon 分发：Fumeshroom/Gloomshroom `do_row_area_damage(20,2)`、Starfruit→`launch_star_fruit`、逐种子弹种映射（含 Kernelpult Secondary→Butter）、FOLEY 音效、各植物发射原点偏移表。
6. **zombie 22 个缺失函数** → 签名已补齐（多数为带 TRANSLATION_NOTE 的占位，见第三节；`draw_zombie_head`/`draw_zombie_with_parts` 标注为 C++ 死代码保留签名）。
7. **coin 9+1 个缺失** → 已补齐：`is_level_award`(333)、`coin_gets_bouncy_arrow`(344)、`get_disappear_time`(361)、`dropped_usable_seed`(384)、`try_auto_collect_after_level_award`(393)、`play_launch/ground/collect_sound`(415/430/439，collect 有 Foley 实体，`SOUND_*` 采样占位)、`is_present_with_advice`(327)。
8. **seed_packet** → `update`/`draw`（老虎机滚动叠画、模仿者/费用绘制）/`mouse_down`（老虎机模式）/`pick_next_slot_machine_seed`/`was_planted`/`flash_if_ready` 已补实体。
9. **challenge** → `draw_backdrop`/`draw_art_challenge`/`draw_beghouled`/`i_zombie_draw_plant`（多层描边）/`i_zombie_set_plant_filter_effect`/`i_zombie_init_level` 等已补。
10. **其他**：`board::process_delete_queue`、zen_garden `init_level`/`mouse_down_with_feeding_tool`、PlayerInfo `load/save_details`+`sync_summary/details`+`reset_challenge_record`+`delete_user_files`、ProfileMgr 真实文件 IO、MessageWidget 文字绘制布局、lawn_app 对话框/注册表/模式切换。

---

## 二、仍完全缺失的 C++ 函数（词干级扫描 39 候选，人工筛后）

**真缺失（约 10）**
- **plant.rs**：`AnimateGarlic`、`AnimateNuts`、`AnimatePumpkin`（坚果/大蒜/南瓜动画）、`DrawMagnetItems`/`DrawMagnetItemsOnTop`、`UpdateBowling`（保龄球玩法，全仓库无）、`UpdateReanimColor`；`GetToolTip`（返回植物图鉴文案，待进一步确认是否以其他形式实现）
- **board.rs**：`AddBossRenderItem`、`CreateRakeReanim`、`DrawGameObjects`、`DrawHouseDoorBottom`/`DrawHouseDoorTop`、`DrawDebugText`/`DrawDebugObjectRects`、`SpecialPlantHitTest`/`ToolHitTestHelper`/`HighlightPlantsForMouse`（鼠标工具命中检测链）
  - 注：`DrawUITop/UIBottom/UICoinBank/Shovel/ProgressMeter/ZenButtons/ZenWheelBarrowButton/TopRightUI` 部分并入 `board.rs::draw_ui`(1545)（种子槽/阳光/铲子已画），不按完全缺失计

**改名/等价（勿再报缺失）**
- `IsImmobilizied`→`is_immobilized`（zombie.rs:3853，C++ 拼写错）；`MagnetShroomAttactItem`→`magnet_shroom_attack_item`（plant.rs，C++ 拼错 Attact）；`StarFruitFire`→`launch_star_fruit`；`DrawZombie` 由 `draw()`+`draw_reanim` 承担
- DataSync `SyncUInt8/16/32/64/Double/Float`→`read_u*/write_u*` 系列；`Music::PvzpLoadMusic`→`tod_load_music`；PoolEffect `PoolEffectDraw/Initialize/Update`→`draw/initialize/update`
- board `AddACrater/AddAGraveStone/AddALadder`→`add_crater/add_grave_stone/add_ladder`；`CountSunFlowers`→`count_sunflowers`；`StageHas6Rows`→`stage_has_6_rows`；`GetIceZPos`→`get_ice_z_pos`；`PixelToGrid*KeepOnBoard`/`OffsetYForPlanting` 均已有

---

## 三、空体/stub/占位（扫描口径：EMPTY / STUB / TODO / 函数体含 TRANSLATION_NOTE）

**总计 302 个**（上版 329，−27；zombie 因补签名反升 42→51）。按文件分布：

| 文件 | 数量 | 要点 |
|---|---|---|
| zombie.rs | 51 | 真空 EMPTY/STUB：`preload_zombie_resources`(191)、`update_mowered`(964)、`drop_loot`(5219)、`reanim_show_prefix`(6146)/`reanim_show_track`(6149)、`setup_water_track`(6202)、`reanim_ignore_clip_rect`(6458)/`reanim_reenable_clipping`(6461)、`start_walk_anim`(6464)、`enable_mustache`(6467)/`enable_future`(6470)；其余为 NOTE（新补函数占位：`add_attached_reanim`→None、`draw_zombie_part`、`override_particle_*`、`apply_zombatar_head`、`balloon_propeller_hat_spin`、`animate_chew_sound/effect`、`update_zombie_*_head` 等） |
| lawn_app.rs | 29 | Crazy Dave 整套、`show/kill_award_screen`、`do_almanac/pause_dialog`、`write_to_registry`、`switch_screen_mode`、`do_high_score_dialog`、`confirm_quit` |
| plant.rs | 28 | reanim 动画接口空：`set_body_reanim_frame/rate/loop/frame_base_pose`、`add_head_reanim/2/3`、`play_body_reanim`、`play_idle_anim`、`set_sleeping`、`attach_blink_anim`(→None)、`preload_plant_resources` |
| zen_garden.rs | 18 | `zen_garden_update/start`、`do_feeding_tool`、`potted_plant_update`、`plant_update_production`、`draw_potted_plant` 等 |
| projectile.rs | 16 | `update` 分支、impact 音效/特效占位 |
| cutscene.rs | 15 | `load_intro_board`、`load_upsell_*`、`update_upsell`/`draw_upsell`、`preload_resources` |
| board.rs | 13 | `save_game`、`setup_waves`、`update_fwoosh`、`do_typing_check`、`pick_up_tool`、`update_level_end_sequence`（简化） |
| widget/credit_screen.rs | 13 | 阶段机已补，资源占位为主 |
| coin.rs | 10 | update 场景检查、`SOUND_*` 采样、附件动画占位 |
| challenge.rs | 9 | `start_level`(102)、`update`(330)、`init_level`(810) NOTE；`draw_rain`(1788)、`tree_of_wisdom_draw`(2550) 等 |
| system/music.rs | 9 | openmpt 接入占位 |
| grid_item.rs / lawn_mower.rs / seed_packet.rs | 8 / 8 / 6 | `update_portal/rake/scary_pot`、`draw_*`；mower 场景检查/水花/绘制；packet 余项 |
| widget/*（cheat 8、challenge_screen 7、lawn_dialog 7、award 6、chooser 6、store 6 等） | ~70 | "依赖图片/3D 资源"注释占位为主 |

---

## 四、遗留待办（真实差异，建议按序处理）

1. **`drop_loot()` 空体**（zombie.rs:5219）而 `die_with_loot()`/多处死亡路径调用它；zombie.rs:5174 附近疑似内联 `add_coin` 掉落——**需逐个死亡调用点核对掉落是否覆盖所有僵尸类型与关卡奖励**（可能影响核心经济循环）。
2. **`pool_effect.rs::draw`(167) 忽略 `is_night`**：夜晚水面绘制与白天未区分（C++ `PoolEffectDraw(g, isNight)` 分支）。
3. **`GetToolTip` 待核**：植物图鉴/鼠标悬浮文案是否以其他形式实现。
4. **challenge `update`(330)/`start_level`(102) 完成度**：NOTE 类函数需逐个人工确认（部分实现 vs 实质缺失）。
5. **reanim/粒子/图片系统接入层**（第三节大部分占位的共同根因）：zombie/plant 动画接口全空、`AddAttachedReanim` 返回 None、`DrawZombiePart` 空——建议先于 UI 资源层推进。

---

## 五、方法说明与勘误记录

- 上版（d6982dd 版）误报项（`challenge.rs` IZombie/Whack 系列、`cutscene::PlaceAZombie`、`grid_item::DrawIZombieBrain` 为"完全缺失"）已在上版文本中勘误；本版扫描显示其已全部补齐。
- 词干匹配对"缩写词"（`BossRVAttack`）、"C++ 拼写错误"（`IsImmobilizied`/`Attact`）、"a/an 省略"（`AddACrater`）会产生假缺失，本版均经人工复核剔除，仅保留带 `→` 改名映射说明的等价项。
