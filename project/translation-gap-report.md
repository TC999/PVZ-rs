# PVZ-rs ↔ C++ 翻译差异核查报告（2026-09-04 三轮重核版）

> 基线：HEAD `59aa4d1`（2026-09-04 00:20）+ **未提交工作区改动**（`src/lawn/zombie.rs` +422、`src/todlib/definition.rs` +9；git status：M zombie.rs / M definition.rs / M 本报告）。
> 对比 Rust 版（`src/`）与 C++ 版（`cpp/src/Lawn/`、`cpp/src/ConstEnums.h`）的翻译不一致。
> 方法：只读扫描——词干级函数匹配（camelCase↔snake，含 `BossRVAttack`/`WhackAZombie`/C++ 拼写错误 `Attact`/`IsImmobilizied` 等边界）+ 空函数体/占位扫描（EMPTY / STUB / TODO / TRANSLATION_NOTE）+ 关键逻辑抽查。**未修改任何源码**（仅覆写本报告）。
> 编译基线：`cargo build` 成功（677 warnings，517 duplicates，含未提交改动）。

---

## 一、本版新增修复（未提交工作区，抽查验证为真实现）

**zombie.rs 12 个空函数已实质翻译**（对照 C++ Zombie.cpp，非占位）：
- `preload_zombie_resources(zombie_type)`：按类型加载 reanim 定义（Digger→DiggerDirt/ZombieCharredDigger；Boss→BossDriver/Fireball/Iceball+12 随从；Dancer→BackupDancer；Gargantuar→Imp/CharredImp/CharredGargantuar；Zamboni→CharredZamboni；Catapult→CharredCatapult；末尾统一 Puff/ZombieCharred/LawnMoweredZombie）
- `update_mowered()`：碾压 reanim 完成（`m_loop_count > 0` 或缺失）→ `drop_head`+`drop_arm`+`die_with_loot`（对应 C++ UpdateMowered）
- `draw_zombie_part(g, image, frame, row, draw_pos)`：图片 cel 裁剪/偏移（PolevaulterInVault −120/−120、DiggerTunneling +50、Zamboni −19）、`clip_height` 上限 CLIP_HEIGHT_LIMIT=−100、alpha/镜像（C++ 注"normally never called"）
- `stop_zombie_sound()`、`reanim_show_prefix`/`reanim_show_track`（按轨道名前缀/名称设 render group）、`setup_water_track`、`reanim_ignore_clip_rect`/`reanim_reenable_clipping`（strcasecmp 匹配轨道设 `m_ignore_clip_rect`）、`start_walk_anim`（LadderCarrying/NewspaperMad 等分支 + `pick_random_speed`）、`enable_mustache`/`enable_future`（UI 波次/zombotany 守卫、轨道存在性检查）

**definition.rs**：`ReanimatorTrackInstance` 增 `m_ignore_clip_rect` / `m_ignore_extra_additive_color` / `m_ignore_color_override` 三字段（C++ 对应成员），支撑裁剪/颜色覆盖接口。

上一版（59aa4d1 版）"已修复"清单（drop_loot 链、PoolEffect is_night、GetToolTip、challenge 分派）在本版仍成立，不再重复。

---

## 二、仍完全缺失的 C++ 函数（39 候选，人工筛后）

**真缺失（约 10）**
- **plant.rs**：`AnimateGarlic`、`AnimateNuts`、`AnimatePumpkin`、`DrawMagnetItems`/`DrawMagnetItemsOnTop`、`UpdateBowling`、`UpdateReanimColor`
- **board.rs**：`AddBossRenderItem`、`CreateRakeReanim`、`DrawGameObjects`、`DrawHouseDoorBottom`/`DrawHouseDoorTop`、`DrawDebugText`/`DrawDebugObjectRects`、`SpecialPlantHitTest`/`ToolHitTestHelper`/`HighlightPlantsForMouse`
  - 注：`DrawUITop/UIBottom/UICoinBank/Shovel/ProgressMeter/ZenButtons/ZenWheelBarrowButton/TopRightUI` 部分并入 `board.rs::draw_ui`(1545)，不按完全缺失计

**改名/等价（勿再报缺失）**
- `IsImmobilizied`→`is_immobilized`；`MagnetShroomAttactItem`→`magnet_shroom_attack_item`；`StarFruitFire`→`launch_star_fruit`；`DrawZombie` 由 `draw()`+`draw_reanim` 承担；`draw_zombie_head`/`draw_zombie_with_parts` 为 C++ 死代码（保留签名）
- DataSync `Sync*`→`read_u*/write_u*`；`Music::PvzpLoadMusic`→`tod_load_music`；PoolEffect 系列→`draw/initialize/update`
- board `AddACrater/AddAGraveStone/AddALadder`→`add_crater/add_grave_stone/add_ladder` 等（详见前版，均已有）

---

## 三、空体/stub/占位（工作区状态）

**总计 290 个**（上版 302，−12）。按文件分布：

| 文件 | 数量 | 文件 | 数量 |
|---|---|---|---|
| zombie.rs | 38（上版 50） | board.rs | 13 |
| lawn_app.rs | 29 | widget/credit_screen.rs | 13 |
| plant.rs | 29 | coin.rs | 10 |
| zen_garden.rs | 18 | challenge.rs | 9 |
| projectile.rs | 16 | system/music.rs | 9 |
| cutscene.rs | 15 | grid_item.rs / lawn_mower.rs | 8 / 8 |
| widget/cheat 8 · challenge_screen 7 · lawn_dialog 7 · award 6 · chooser 6 · store 6 … | ~70 | | |

**纯空体/STUB 共 68 个**（上版 80，−12）：
- **zombie.rs 6**：`balloon_propeller_hat_spin`(5029)、`override_particle_scale`(5034)/`override_particle_color`(5039)、`apply_zombatar_head`(5044)、`draw_zombie_head`(5155)/`draw_zombie_with_parts`(5160)（后两者 C++ 死代码）——均含 TRANSLATION_NOTE（reanim/粒子/Zombatar 系统未接入）
- **widget 系列 ~45**：award_screen `draw_bottom/draw_award_seed/draw/draw_achievements/mouse_down`、cheat_dialog `draw/update/key_down/mouse_down/added_to/removed_from_manager/edit_widget_text`、lawn_dialog `button_press/checkbox_checked/added_to/removed_from_manager`、store_screen `draw_item_icon/draw_item/draw/draw_overlay`、challenge_screen `added_to/removed_from_manager/button_press`、game_button `draw/set_label`、seed_chooser `draw/update_cursor/enable_start_button`、credit_screen `mouse_up/button_press`、imitater_dialog `calc_size/update_cursor/draw`、achievements `draw/mouse_down`、continue/new_user/new_options 各 1–2 等
- **其他 17**：lawn_mower `enable_super_mower`(179)/`update_pool`(183)/`draw_shadow`(190)/`draw`(195)、coin `draw`(247)、projectile `draw_shadow`(484)、board `update_layers`(3381)、challenge `draw_slot_machine`(1002)/`draw_rain`(2170)/`tree_of_wisdom_draw`(2960)、lawn_app `do_high_score_dialog`(1797)/`url_open_succeeded`(1944)、cursor_object `draw`(120)/`plant_draw_seed_type`(368)、music `music_dispose`(151)、zen_garden `zen_garden_start`(2020)、reanimation_lawn `reanimator_cache_initialize/dispose`(42/46)/`get_plant_image_size`(188)、plant `update_blover`(2020)

> 注：其余 222 个为含 TRANSLATION_NOTE/TODO 的部分实现。

---

## 四、遗留待办（按建议顺序）

1. **zombie 剩余 6 个空体**：reanim 附着/粒子/Zombatar 系统接入（`balloon_propeller_hat_spin`、`override_particle_scale/color`、`apply_zombatar_head`）——`AddAttachedReanim` 仍返回 None、plant reanim 动画接口（`set_body_reanim_*`/`play_body_reanim`/`play_idle_anim`）仍全空，是 reanim 接入层最后一公里。
2. **纯绘制空体**（widget ~45 + board/challenge 绘制拆分）：依赖图片资源接入（award/store/lawn_dialog 等 draw/mouse_down、board `DrawGameObjects`/`DrawHouseDoor*`、challenge `draw_slot_machine/draw_rain/tree_of_wisdom_draw`）。
3. **玩法级**：`UpdateBowling`（保龄球）、`AnimateGarlic/Nuts/Pumpkin`、磁铁吸附物绘制、鼠标工具命中链（`SpecialPlantHitTest`/`ToolHitTestHelper`/`HighlightPlantsForMouse`）、`UpdateReanimColor`。
4. **待核**：`update_mowered` 现于 reanim 完成后掉头/臂并 `die_with_loot`（与 board 侧 mow_zombie 的 `die_with_loot` 可能双触发？需核对 Mowered 路径是否重复掉落——`dropped_loot` 标志应已防重，建议实机验证）；challenge `start_level` 锤子 reanim / SeedBank y 坐标 NOTE。

---

## 五、勘误与版本记录

- v1（d6982dd 版）误报（challenge IZombie/Whack 系列、cutscene `PlaceAZombie`、grid_item `DrawIZombieBrain`）已在上上版勘误；v2（b5609a8 版）遗留待办四项已修复并验证。
- 词干匹配假缺失（缩写词/拼写错/a-an 省略）统一以 `→` 标注，不列入缺失。
- 历史提交经 rebase（hash 变化），本报告以"内容基线"为准：HEAD 59aa4d1 + 未提交工作区改动。
