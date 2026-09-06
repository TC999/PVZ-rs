# PVZ-rs ↔ C++ 翻译差异核查报告（2026-09-05 五轮重核版）

> 基线：HEAD `2321871` + **未提交工作区改动（再次更新后）**：25 文件约 +4683 行（相对四轮版 +2900 行），新增 `src/lawn/widget/zombatar_widget.rs`(671 行)、`src/lawn/widget/zombatar_tos.rs`(210 行)，`src/lawn/zombatar.rs` 扩展至 278 行，`src/todlib/tod_particle.rs` +132、`src/todlib/attachment.rs` +150。
> 方法：C++ 成员函数 ↔ Rust `fn` 词干匹配 + 缺失二次全文 grep + 对疑似 stub 逐段对照 C++ 源（含 EatPlant/ApplyChill/Draw 系列等）。只读扫描，未改动源码。
> 说明：四轮版所列多数"缺失"现已以同名/改名函数补齐，本版改为"已修复清单 + 剩余缺失 + 行为级 stub"三层。

---

## 一、四轮版遗留已修复并抽查通过（勿再报缺失）

- **plant.rs**：`Plant::Update` 补全 doUpdate 四分支场景判定（plant.rs 435-452 ↔ Plant.cpp 2856-2865）；`Animate()`（plant.rs 476+）完整复刻——cherry/jalapeno 抖动、三倒计时递减、`mSquished→mFrame=0;return` 短路、坚果/蒜/南瓜受损分支、`UpdateBlink()`（2134 已挂回调用点）、`mAnimPing/mAnimCounter/mFrame` 帧推进（3461-3482）。
- **zombie.rs + attachment.rs + tod_particle.rs**：`override_color/override_extra_additive_draw/override_scale`（tod_particle.rs 582-660）已实现为遍历 emitter_list 匹配空名=全部 emitter；`add_attached_reanim`（zombie.rs 5068）现为真实现（`AddReanimation` + `attach_reanim`）；`find_reanim_attachment`（attachment.rs 300）遍历 effect_array 找 `EffectType::Reanim`——上版"粒子/附着底层空体"链已接通。
- **game_button.rs**：`GameButton::draw` 已补全皮肤链（disabled 图 → mOverAlpha 渐变混合 → highlighted over → normal，`DRAWMODE_ADDITIVE` overlay、按下分支），与 GameButton.cpp 128-209 一致；`font=None` 不再提前 return。
- **store_screen.rs**：补齐至 32 fn（0 函数缺失）——`PurchaseItem`(896)（货币检查→Not enough money 对话框→扣费）与 C++ 一致；mouse_down/update/button_press 已存在。
- **board.rs**：`AddBossRenderItem`(1362)、`CreateRakeReanim`(1211)、`DrawDebugText`(1232)、`DrawProgressMeter`(1407) 均已补同名函数。
- **credit_screen.rs / new_options_dialog.rs / zombatar_tos.rs**：0 函数缺失（zombatar_tos 13 fn ↔ C++ 11）。
- **zombie.rs**：`is_immobilized`(3921)、`find_plant_target_index`(2554)、`is_tangle_kelp_target`(3514)、`count_bungees_targeting_sunflowers`(7296)、`update_zombie_pea_head`（发射 ZombiePea 投射物已实现）等均存在。

---

## 二、剩余函数缺失（已 grep 复核）

- **lawn_app.rs**（~56）：`DoCheatDialog`/`DoUserDialog`/`DoCreateUserDialog`/`DoRenameUserDialog`/`DoConfirmDeleteUserDialog`/`FinishCheatDialog`/`FinishTimesUpDialog`/`FinishCreateUserDialog`/`DoNewOptions`/`URLOpenFailed`/`UpdateApp`/`UpdatePlayTimeStats`/`UpdateRegisterInfo`/`ParticleTryToGet`/`ReanimationTryToGet`/`GetCrazyDaveText` 等对话框/回调链仍整块缺失（`ButtonPress/ButtonDepress/ButtonMouse*` 回调在 Rust 架构中分散于各 widget，未按 LawnApp 集中实现）。
- **title_screen.rs**（仅 6 fn ↔ C++ 10）：`Draw`、`Update`、`MouseDown`、`KeyDown`、`ButtonPress`、`ButtonDepress`、`DrawToPreload`、`AddedToManager`、`RemovedFromManager`、`Resize` 全缺。
- **zombatar_widget.rs**：**绘制整块缺失**——C++ `Draw/DrawAvatar/DrawAvatarBox/DrawColorSwatches/DrawConfirm/DrawCreate/DrawDraftAvatar/DrawImageColorized/DrawList/DrawMainBackground/DrawPartImage/DrawTransition/MouseMove/MouseUp/Update/CreatePreviewZombie/DestroyPreviewZombie/AddedToManager/RemovedFromManager/ButtonPress`（20 个）在 Rust 中无对应；Rust 现实现的是逻辑侧（handle_grid_click/handle_color_click/encode_record/decode_record/save_draft/delete_current/change_page 等），另有空体 `update_button_state`(332)/`show_max_heads_message`(335)/`get_item_hit_rect`(650)/`get_color_rect`(655)。
- **game_selector.rs**：`MouseDown` 无同名实现（疑并入事件分派，待核）；**seed_chooser_screen.rs**：`UpdateAfterPurchase`、`UpdateImitaterButton` 无对应。
- board/plant/zombie 中残余"缺失"均为已知改名/内联项（`AddACrater→add_crater`、`DrawUITop→draw_ui`、`StarFruitFire→launch_star_fruit`、`MagnetShroomAttactItem→magnet_shroom_attack_item`(2825)、`DrawZombie→draw()`、`IsTanglekelpTarget→is_tangle_kelp_target` 等），不再列入。

---

## 三、行为级 stub / 逻辑不一致（本版重点：同名函数存在但被重度简化）

1. **`Zombie::eat_plant`（zombie.rs）→ 2 行 stub**：仅 `is_eating=true`。C++ EatPlant（Zombie.cpp 7026，70+ 行）完整实现：dancer 入场短路、yucky face 短路、**梯子判定**（有梯子则 StopEating/上梯）、致命植物判定（Jalapeno/Cherry/Doom/Ice/Hypno + Flowerpot/Lilypad/Squash 不可侵状态不吃）、PotatoMine 提前引爆分支、啃咬音效/计帧等。**僵尸进食玩法核心逻辑缺失**，僵尸吃植物只剩状态标志，不吃任何判定。
2. **`Zombie::apply_chill`（zombie.rs）→ 1 行 stub**：仅 `chilled_counter=100`。C++ ApplyChill（Zombie.cpp 7519）含冰道 reanim、音效、chilled 计数与蓝色覆盖层。
3. **board.rs 绘制空壳**（参数 `_g` 未用，无任何图形输出）：`draw_debug_text`（只拼字符串）、`draw_debug_object_rects`（只 `get_plant_rect()` 后丢弃）、`draw_progress_meter`（`tod_animate_curve` 算宽度后 `let _ =` 丢弃、进度条/旗/文案均未画）。
4. **widget 纯绘制空壳**（图片未接入，同四轮版，数量未减）：store_screen `draw`(仅空商品循环)/`draw_item`/`draw_item_icon`/`draw_overlay`、award_screen `draw*` 系列、cheat_dialog/lawn_dialog/new_options_dialog/title_screen 的 draw/mouse 系列、zombatar_tos 的 `draw/added_to_manager/...` 空体、game_button `draw_stone_button` 占位。
5. **zombie 特殊头部残余注记**：`update_zombie_pea_head` 的 `anim_shooting` 播放、`MOTION_BACKWARDS`（ZombiePea 向左飞行）仍未接入——发射已实现但方向/动画轨道缺。
6. **`prune_dead_effects`（attachment.rs 330）简化**：以 `effect_id != 0` 判存活，与 C++ 按效果生命周期判定不等价。

---

## 四、遗留待办（按影响排序）

1. **`EatPlant`/`ApplyChill` 等核心行为 stub 实装**（三.1/三.2）——直接影响僵尸对植物的交互（吃植物判定、冰道减速）。
2. **widget 纯绘制与生命周期补齐**（三.4 + 二.1/二.2）——title/store/award/cheat/zombatar_tos 的 draw 与鼠标键盘，lawn_app 对话框链。
3. **zombatar_widget 绘制整块**（二）——游戏内"造僵尸"界面尚无可视部分。
4. **board 调试/进度绘制实装**（三.3）；`MOTION_BACKWARDS`/头部动画轨道（三.5）。

---

## 五、版本记录

- 四轮版（本日早间）遗留：Plant::Update 帧动画（已修复，见一）、GameButton 皮肤链（已修复）、粒子/附着底层空体（已修复）、zombatar_widget/zombatar_tos 未建（已建，见二/三）。
- 四轮版"三.逻辑不一致"项 1-4 现已全部抽查通过（修复），本版新增行为级 stub 项（三.1/三.2/三.3/三.6）为**新发现**。
- 已知改名/内联项统一以 `→` 标注排除，不再重复列入。
