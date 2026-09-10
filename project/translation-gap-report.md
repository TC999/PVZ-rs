# PVZ-rs ↔ C++ 翻译差异核查报告

> 基线：HEAD `3b573c0`，工作区源码无改动。
> 对照源：`cpp/src/**`。
> 性质：只读静态对照；未修改任何源码，未编译/运行游戏。
> 方法：全局空体函数/`TRANSLATION_NOTE`/`TODO` 扫描 + 关键接入点（`Board::update`、`Challenge::update`、数值表、绘制链）抽样核对 + C++↔Rust 行数比。

---

## 结论速览

旧基线 `053342b` 报告中的多项高危项已被后续提交修复（见下）。当前仍存在的差异集中在：

1. **对话框族整套空体**——新用户/继续游戏/用户选择/新选项/作弊码/成就等界面完全无功能。
2. **Board 运行链剩余未接入点**——PoolEffect 计数与泳池闪光粒子整段未接入。
3. **渲染/附着层 stub**——粒子/Reanim 附着、Attachment 绘制等仍以注释占位。

---

## 一、已修复项（旧报告结论已过时，勿再报为缺失）

| 旧报告结论（基线 053342b） | 当前状态（HEAD 3b573c0） |
|---|---|
| `ZOMBIE_DEFS` 仅 26 项 → `ztype>=26` 索引越界 panic | ✅ 已修复：`zombie.rs:7983` 现为 **33 项**（`Normal=0 … RedeEyeGargantuar=32`），补齐 `PeaHead/WallnutHead/JalapenoHead/GatlingHead/SquashHead/TallnutHead/RedeEyeGargantuar` 7 项；`get_zombie_definition`（`zombie.rs:8020`）不再越界 |
| `NUM_ZOMBIE_TYPES=34` 越界 | ✅ 已改为 `33`（`game_enums.rs:1967`） |
| `Projectile::draw` 空体 → 所有子弹不可见 | ✅ 已实现：`projectile.rs:678-793` 按 `ProjectileType` 选图 + 缩放/旋转矩阵 + 镜像绘制 |
| `Challenge::update` 全库无调用者 → 挑战状态机不推进 | ✅ 已接入：`board.rs:854-856`（正常路径）、`board.rs:775-777`（暂停路径）调用 `ch.update()` |
| `Board::update` 缺 mCutScene/ZenGarden/CrazyDave/菜单商店按钮/EffectSystem/Advice/Tutorial/震动等 | ✅ 已接入：`board.rs:739-857` 逐一对应 C++ `Board::Update`（5724-5811），含 `mCutScene->Update`、`ZenGardenUpdate`、`UpdateCrazyDave`、菜单/商店按钮 `Update`、`EffectSystem->Update`、`Advice->Update`、`UpdateTutorial`、震动、`mCoinBankFadeCount`、`UpdateLayers`、`UpdateGridItems`、`UpdateFwoosh`、`UpdateGame`、`UpdateLevelEndSequence` |
| todlib 粒子完全不绘制 | ✅ 已修复：粒子 `Draw→Emitter::Draw→DrawParticle→GetRenderParams` 链已实现（提交 `9f758b0`） |
| framework `mouse_drag` 拖动链断裂 | ✅ 已修复：`WidgetManager::MouseMove` 按下转发 `MouseDrag`（提交 `1e5835a`） |
| framework 属性系统缺失 | ✅ 已修复：`SexyAppBase` 属性表 + `Get/Set` API + `PropertiesParser`（提交 `43fc91f`） |

---

## 二、仍存在的差异 / 缺失逻辑

### 2.1 对话框族整套空体（已完成 ✅ 2026-09-11）

> **本轮翻译完成状态**：本段 8 个文件共 32 个真实空体函数已全部翻译，5 个 commit：
> - `31a184e` NewUserDialog 空体全译（8 函数）
> - `9ffbe45` ContinueDialog 空体全译（8 函数 + Drop 清理）
> - `abe729f` UserDialog 空体全译（9 函数 + ListWidget 结构扩展 + Drop 清理）
> - `6361df8` CheatDialog 空体全译（6 函数 + sscanf 解析工具 + Drop 清理）
> - `f736338` AwardScreen::mouse_down + ZombatarWidget::show_max_heads_message
>
> 3 处"误报"（C++ 无对应虚函数或 C++ 也是空函数）已在下方勘误标注。
> **真实剩余：0 项**。
>
> [TRANSLATION_NOTE]: `LawnApp::ButtonDepress` 未实现（SexyAppBase::button_depress 空体），
> `NewUserDialog::EditWidgetText` / `UserDialog::EditWidgetText` / `CheatDialog::EditWidgetText`
> 中的 `mApp->ButtonDepress(mId + 2000)` 语义保留为注释；待 `plan_step_11` 接入 `LawnApp::ButtonDepress`
> 后统一补完 2000 偏移事件路由。

C++ 有完整实现，Rust 对应函数全为空体（现已翻译完毕）：

| Rust 文件 | 空体函数 | C++ 对照（行数） | 状态 |
|---|---|---|---|
| `widget/new_user_dialog.rs:51-56` | `draw` / `update` / `key_down` / `mouse_down` / `added_to_manager` / `removed_from_manager` | `NewUserDialog.cpp`（125） | ✅ commit `31a184e` |
| `widget/continue_dialog.rs:49-67` | `draw` / `update` / `mouse_down` | `ContinueDialog.cpp`（219） | ✅ commit `9ffbe45` |
| `widget/user_dialog.rs:107-115` | `draw` / `update` / `key_down` / `mouse_down` / `added_to_manager` / `removed_from_manager` / `list_clicked` / `button_depress` / `edit_widget_text` | `UserDialog.cpp`（203） | ✅ commit `abe729f` |
| `widget/new_options_dialog.rs:93,106` | ~~`update` / `mouse_down`~~ **误报：C++ NewOptionsDialog.h/cpp 无 Update/MouseDown 虚函数覆写**（继承自 Sexy::Dialog 基类默认实现）；Rust 侧空体正确。已确认全 10 个 C++ 虚函数（`GetPreferredHeight/AddedToManager/RemovedFromManager/Resize/Draw/SliderVal/CheckboxChecked/ButtonPress/ButtonDepress/KeyDown`）在 Rust 侧全部实现。 | `NewOptionsDialog.cpp`（402） | ✅ 无需翻译 |
| `widget/cheat_dialog.rs:75-83` | `update` / `key_down` / `mouse_down` / `added_to_manager` / `removed_from_manager` | `CheatDialog.cpp`（155） | ✅ commit `6361df8` |
| `widget/award_screen.rs:394` | `mouse_down` | `AwardScreen.cpp`（663） | ✅ commit `f736338` |
| `widget/zombatar_tos.rs:164` | ~~`button_press`~~ **误报：C++ ZombatarTOS::ButtonPress 也是空函数**（ZombatarTOS.cpp:175-178：`void ZombatarTOS::ButtonPress(int theId) { (void)theId; }`）；Rust 侧空体正确。 | `ZombatarTOS.cpp` | ✅ 无需翻译 |
| `widget/zombatar_widget.rs:363` | `show_max_heads_message`（C++ ZombatarWidget::ShowMaxHeadsMessage 调用 LawnMessageBox，Rust 需接入） | `ZombatarWidget.cpp` | ✅ commit `f736338` |
| `widget/zombatar_widget.rs:1072` | ~~`button_press`~~ **误报：C++ ZombatarWidget::ButtonPress 也是空函数**（`void ZombatarWidget::ButtonPress(int theId) { (void)theId; }`）；Rust 侧空体正确。 | `ZombatarWidget.cpp` | ✅ 无需翻译 |

### 2.2 Board 运行链剩余未接入点

- `board.rs:842-845`：PoolEffect 计数与泳池闪光粒子整段被注释（C++ `Board::Update` 中 `mPoolEffect->mPoolCounter++` + 闪光粒子逻辑未接入）。
- `board.rs:829-831`：`m_coin_bank_fade_count` 递减缺少 C++ 的"`DIALOG_PURCHASE_PACKET_SLOT` 未打开才递减"过滤条件（Rust `LawnApp` 无对话框管理，自注暂不过滤）。

### 2.3 渲染/附着层 stub（`TRANSLATION_NOTE` 自认）

- `zombie.rs:7389`：`AttachReanim` 恒 `None`，附着效果待 `AttachEffect` 系统接入。
- `zombie.rs:6248`：`AttachmentDraw` 为 stub，附着物不绘制。
- `projectile.rs:790-792`：子弹附件绘制（`AttachmentDraw`）暂略。
- `plant.rs` 多处：`StartBlend` / `SetFramesForLayer` / `SetTruncateDisappearingFrames` / `AssignRenderGroupToPrefix` 依赖 reanim 完整实现（当前 stub）。
- `reanimation_lawn.rs:179,184`：两处显式 `TODO: 从 ReanimationLawn.cpp 翻译`。
- `coin.rs:13`：`SOUND_*` 常量在 `game_enums` 未定义，用占位值。

---

## 三、未覆盖范围（未逐行核对，沿旧报告）

- Zombie Boss 段逐行逻辑、Gargantuar 定时事件、Bungee 投递链细节。
- Plant 各 `update_*` 状态机数值（potato/squash/chomper/magnet/cactus/cobcannon/cattail/torchwood）。
- Challenge 各模式（Beghouled/ScaryPotter/WhackAZombie/IZombie/老虎机等）实现完整度。
- 伤害/速度/冷却/掉落概率逐项对账。
- framework/todlib 层渲染链逐行。
- 运行期实测（未启动游戏；本次均为静态推断）。

---

## 版本记录

- HEAD `3b573c0` 版：基于最新提交重核。纠正旧报告（`053342b`）已过时结论；确证仍存差异为对话框族空体、PoolEffect 未接入、渲染/附着 stub。未修改任何源码。
