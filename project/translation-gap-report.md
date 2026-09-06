# PVZ-rs ↔ C++ 翻译差异核查报告（2026-09-05 七轮重核版）

> 基线：HEAD `aa828f5`（六轮版基线）+ **未提交工作区改动**：12 文件约 +2300 行（zombie/board/lawn_app/projectile/player_info/award_screen/cheat_dialog/new_options_dialog/store_screen/zombatar_tos/zombatar_widget）。
> 方法：C++ 成员函数 ↔ Rust `fn` 词干匹配 + 缺失全文 grep 复核 + 行为级 stub/新实现逐段对照 C++ 源（附行号证据）。只读扫描，未改动源码。
> 编译基线：`cargo check` 通过（仅 warnings，无 error）。
> 本版核心：六轮版"三、遗留待办"几乎全部落地，逐项抽查通过；剩余缺口收敛为 title_screen 整块 + lawn_app 少量对话框 + widget 生命周期细节。

---

## 一、六轮版待办已修复并抽查通过（勿再报缺失）

1. **`Zombie::eat_plant`（zombie.rs 4056 起，现 ~90 行）实装**：dancer 入场短路、`mYuckyFace` 短路、**梯子判定**（Digger 忽略；否则 StopEating + `HEIGHT_UP_LADDER` 上梯）、致命植物判定（Jalapeno/Cherry/Doom/Ice/Hypno + Flowerpot/Lilypad/Squash 不可侵状态，asleep 例外）、PotatoMine 非 NotReady 不吃、Blover 触发等——与 Zombie.cpp 7026 EatPlant 逐行一致。
2. **`Zombie::apply_chill`（zombie.rs 5519）实装**：`CanBeChilled` 守卫 → `FOLEY_FROZEN`（仅 chilled_counter==0 时）→ chill_time 1000/2000（冰道）→ `max` → `UpdateAnimSpeed`——与 Zombie.cpp 7519 ApplyChill 一致。
3. **board.rs 绘制实装**（参数 `_g` 已启用）：`draw_debug_text`/`draw_debug_object_rects` 真输出；`draw_progress_meter`（1407 起）完整——IMAGE_FLAGMETER `DrawImageCel(600,575,0)` + `aSrcRect/aDstRect` 裁剪条（Board.cpp 6570-6620）+ Beghouled/Squirrel/SlotMachine/Zombiquarium/IZombie 模式文本，图片缺省时安全跳过。
4. **projectile.rs**：补 `MOTION_BACKWARDS`（ZombiePea 向左直飞）——六轮版二.1.5 缺口已闭。
5. **lawn_app.rs 对话框链（13 个实装，56 缺→43 缺）**：`do_new_options`/`do_user_dialog`/`finish_user_dialog`/`do_create_user_dialog`/`finish_create_user_dialog`/`do_confirm_delete_user_dialog`/`finish_confirm_delete_user_dialog`/`do_rename_user_dialog`/`finish_rename_user_dialog`/`finish_name_error`/`do_cheat_dialog`/`finish_cheat_dialog`/`finish_times_up_dialog` 与 LawnApp.cpp 对话框逻辑对应。另确认等价改名存在：`is_izombie_level`(1385)/`is_endless_izombie`(1370)/`get_crazy_dave_text`(1729)/`need_register`(2062 简化恒 false)/`update_app_step`(338)（C++ `UpdateApp`）——勿再按缺失报。
6. **store_screen.rs 绘制实装**（+314）：`draw`/`draw_item`/`draw_item_icon`/`draw_overlay` 完整——item icon 按 StoreItem 分派图片（PacketUpgrade 等）+ 高亮 `DRAWMODE_ADDITIVE(255,255,255,96)` + 槽位文本、hatch/背景昼夜分支，与 StoreScreen.cpp Draw/DrawItemIcon 对应。
7. **award_screen.rs（+207）/cheat_dialog/new_options_dialog/zombatar_tos（+47）**：`draw_bottom`/`draw_award_seed`/`draw` 等补齐，文本居中/右齐辅助函数齐备，与各 C++ 绘制对应。
8. **zombatar_widget.rs（+995，44→63 fn，缺 2）**：绘制整块补齐——`draw`/`draw_avatar`（record 解码 + 9 页 part/color 读取 + subpage>16 偏移 + 背景/部件上色）/`draw_avatar_box`/`draw_color_swatches`/`draw_list`/`draw_create`/`draw_transition`/`draw_confirm`/`draw_main_background`/`draw_image_colorized`/`draw_part_image` + `create_preview_zombie`/`destroy_preview_zombie`，与 ZombatarWidget.cpp Draw* 系列对应（含 fit_icon_rect/zombatar_grid_align 工具）。

---

## 二、剩余差异与缺失

### 2.1 函数级缺失
- **title_screen.rs（未改动，仍 6 fn ↔ C++ 10）——头号缺口**：`Draw`、`Update`、`MouseDown`、`KeyDown`、`ButtonPress`、`ButtonDepress`、`DrawToPreload`、`AddedToManager`、`RemovedFromManager`、`Resize` 全缺（主菜单界面渲染/交互未翻译）。
- **zombatar_widget.rs 仅余 2 个**：`AddedToManager`/`RemovedFromManager`（widget 生命周期挂接/清理）。
- **lawn_app.rs 剩余 43 个候选，人工归类后真正 gameplay 相关**：`DoConfirmBackToMain`、`DoConfirmSellDialog`、`FinishRestartConfirmDialog`（确认框，疑未用或待接线）；其余多为平台/注册类（`DoRegister*`/`CanDoRegisterDialog`、`URLOpenFailed`、`InitHook`/`ShutdownHook`/`PreDisplayHook`/`LoadingThreadProc`、`GotFocus`/`LostFocus`、`HandleCmdLineParam`、`ModalOpen`/`NewDialog`、`ParticleGet*`/`ReanimationTryToGet` 数据访问器）——在 Rust 架构中多数无对应机制或已内联，建议逐个标注用途后关闭而非照搬。
- 其余 board/plant/zombie/widget 的"缺失"均为已知改名/内联项（`AddACrater→add_crater`、`DrawUITop→draw_ui`、`StarFruitFire→launch_star_fruit`、`MagnetShroomAttactItem→magnet_shroom_attack_item`、`DrawZombie→draw()`、`IsImmobilizied→is_immobilized`、`IsButtonDown/IsMouseOver→is_down/is_over 字段`、`MouseDown(game_selector)→事件分派` 等），不再列入。

### 2.2 行为级 stub / 注记
- **zombie 特殊头部 reanim 轨道**：`update_zombie_pea_head` 的 `anim_shooting` 播放、`update_zombie_gatling_head`/`squash_head` 的头部 reanim 位置/轨道仍未接入（投射物与 MOTION_BACKWARDS 已补）。
- `prune_dead_effects`（attachment.rs ~330）仍以 `effect_id != 0` 简化判活（C++ 按效果生命周期）。
- `draw_stone_button`（game_button.rs）仍占位；`NeedRegister` 类注册功能 Rust 恒 false 简化。
- widget 纯图片绘制（lawn_dialog/cheat 等背景）仍有依赖资源接入的空壳；本轮新增绘制均带 `get_resource_image` 缺省守卫（图缺失时安全跳过），资源表接入后即生效。

---

## 三、遗留待办（按影响排序）

1. **title_screen.rs 全套翻译**（Draw/Update/MouseDown/KeyDown/Button*/生命周期/Resize/DrawToPreload）——主菜单目前无渲染。
2. **zombatar_widget 生命周期 2 项**（AddedToManager/RemovedFromManager）+ lawn_app 确认框 3 项与数据访问器归类关闭。
3. **reanim 头部轨道收尾**：pea/gatling/squash head 的 `anim_shooting`/头部 reanim 位置轨道接入。
4. 资源接入后回填带守卫的绘制空壳；`prune_dead_effects` 生命周期语义。

---

## 四、版本记录

- 六轮版"三、遗留待办"1-4 项已全部落地并抽查通过（见一）；本轮新增缺口收敛为 title_screen 整块 + 少量生命周期/确认框 + reanim 头部轨道注记。
- 编译基线：`cargo check` 通过（仅 warnings）。
- 已知改名/内联项统一以 `→` 标注排除（见二.1 末）。
