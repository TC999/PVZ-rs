# PVZ-rs：C++ → Rust 翻译差异报告

> **基线**：`git HEAD 2f8ee56`（工作区仅 `reasonix.toml` 未跟踪）
> **对照对象**：C++ `cpp/src/`（权威参考实现）↔ Rust `src/`
> **本文覆写** 此前所有版本的 `ANALYSIS_REPORT.md`；已删除的 `.reasonix/audit-report.md`、`project/translation-audit-report.md` 不再维护，仓库中只保留本文件。
> **编译状态（本次实测）**：`cargo check --message-format short` → **0 error / 750 warnings**（3.23s）；`cargo build`（debug）→ **构建与链接成功**，`Finished dev profile [unoptimized + debuginfo] target(s) in 19.72s`，产物 `target/debug/pvz-portable.exe`（7,041,536 字节）
> **局限**：已通过编译与链接验证，但**未运行游戏**（无运行期行为验证）；C++ 有而 Rust 连名字都不存在的低价值项未穷尽核对；未确证项一律标注「未确证」。

**检查方法**

1. 文件级映射：`cpp/src` ↔ `src`（排除 `SDL-Mixer-X`/`glad`/`fcaseopen`/`pxtone`/`timidity` 等第三方）
2. 名字级差集：C++ `Class::Method(` 定义（2156）vs Rust 全 `fn`（2829）→ 未命中 **356**
3. 函数体规模悬殊候选：C++ 体 ≥15 行且 Rust 同名实现 ≤1/8，共 **259** 条，逐条**读两端源码**复核
4. 编译验证：`cargo check` 的 `never used / never constructed` 告警作为「接线断裂」的机器证据
5. 关键项逐行复核（直接读取两端实现）

**判定原则**：Rust 属「重命名 + 重写」式翻译，函数名与文件布局不一致属预期；**差异一律以行为为准**；行数悬殊 ≠ 缺失（已排除大量改名与块注释误报，例：`PoolEffect::PoolEffectDraw`→`draw`、`SeedBank::UpdateWidth`→`board.rs:9259`、`Zombie::DrawZombie` 实为 C++ 注释块内的死代码）。

---

## 0. 总体状况（量化）

| 指标 | 数值 |
|---|---|
| `cpp/src` 总行数 / `src` 总行数 | 194,270 / 98,466 |
| C++ 方法名 / Rust `fn` 名 / 名字未命中 | 2156 / 2829 / 356 |
| 规模悬殊候选（双端复核前） | 259 |
| `cargo check` | **0 error**，750 warnings（645 duplicates） |
| `TRANSLATION_NOTE` | 507 处 |
| C++ `AddPvzpParticle` 调用 | 130 处；Rust 侧**无任何等价 API** |

标记密度 top：`lawn_app.rs` 52、`plant.rs` 40、`board.rs` 34、`zombie.rs` 33、`cutscene.rs` 30、`challenge.rs` 20、`credit_screen.rs` 19、`zen_garden.rs` 19。

**缺口分三层**：①整段逻辑被骨架化（函数存在但只剩状态置位/计数器）；②判定用错常量或漏条件（"看起来实现了"但语义不符）；③子系统未接线（实现了却无调用点，或底层 API 根本不存在）。

**接线断裂的机器证据**（`cargo check` 的 never-used 告警）：`challenge.rs:53` `ZombieAllowedLevels` 未构造、`seed_packet.rs:556` `SeedBank` 未构造（`:565` `new/draw/mouse_hit_test` 未用）、`system/pool_effect.rs:28` 全部方法未用、`widget/message_widget.rs:104`（`get_font/draw/layout_reanim_text/draw_reanimated_text`）、`widget/award_screen.rs:471`（`draw_achievements/achievements_continue_pressed`）、`projectile.rs:808`（`draw/draw_shadow/is_zombie_hit_by_splash/get_projectile_def/get_damage_flags/check_for_high_ground`）、`widget/achievements_screen.rs:70` `AchievementsWidget` 未构造、`lawn_common.rs:80/135/155`（`LawnEditWidget`/`create_edit_widget`/`make_new_checkbox`）、`cursor_object.rs:65`、`game_object.rs:39`、`coin.rs:1457`、`title_screen.rs:140`、`cutscene.rs:104/1059`、`lawn_mower.rs:153`、`grid_item.rs:359`、`board.rs:1220`、`plant.rs:1286`、`zombie.rs:826`、`lawn_app.rs:449`。

---

## 1. 关卡波次与挑战模式（玩法核心，最严重）

| # | 项 | C++ | Rust | 缺失内容 |
|---|---|---|---|---|
| 1.1 | `Challenge::InitZombieWaves` 🔴 | `Challenge.cpp:2540-2741`（201 行） | `challenge.rs:1199-1202`（4 行） | 整个按 `GameMode` 的 `mZombieAllowed` 表（Survival 各阶段 / Speed / PogoParty / BungeeBlitz / SunnyDay / ArtChallenge…）全缺，Rust 仅 `Normal = true` |
| 1.2 | `Challenge::InitZombieWavesSurvival` 🔴 | `Challenge.cpp:2499-2530` | `challenge.rs:1188-1192` | 基于 `GetLevelRandSeed()` 的 `MTRand` 随机填充（池/屋顶/墓碑/无尽/生存阶段过滤）全缺，**且不消耗 RNG** → 随机序列与 C++ 不一致 |
| 1.3 | `Challenge::PortalStart` 🔴 | `Challenge.cpp:3121-3157` | `challenge.rs:2022-2028` | 创建 2×`GRIDITEM_PORTAL_SQUARE` + 2×`PORTAL_CIRCLE`、`MakeRenderOrder`、`OpenPortal()`；Rust 只设计数器 → 传送门关卡无门 |
| 1.4 | `Challenge::UpdateRain` | `Challenge.cpp:5076+` | `challenge.rs:4118-4123` | 随机生成 `REANIM_RAIN_SPLASH`/`RAIN_CIRCLE`（坐标/缩放/`ColorOverride`）；Rust 仅 `rain_counter -= 1` |
| 1.5 | `Challenge::UpdateRainingSeeds` | `Challenge.cpp:1932-1960+` | `challenge.rs:1804-1808` | `AddCoin(COIN_USABLE_SEED_PACKET, FROM_SKY_SLOW)` + 种子类型过滤链（Lilypad 曲线）全缺 |
| 1.6 | `Challenge::UpdateStormyNight` | `Challenge.cpp:1961-2010+` | `challenge.rs:1642-1650` | `FOLEY_THUNDER`、`STORM_FLASH_TIME + RandRangeInt(...)`、Yeti 时 350~450、`SCENE_ZOMBIES_WON` 分支；Rust 固定 `150+300` |
| 1.7 | `Challenge::PuzzleNextStageClear` | `Challenge.cpp:4193-4220+` | `challenge.rs:3247-3252` | `SOUND_HUGE_WAVE`、场上僵尸 `DieNoLoot()`、植物 `Die()`、`mNextSurvivalStageCounter/mProgressMeterWidth` 复位 |
| 1.8 | `Challenge::GraveDangerSpawnRandomGrave` | `Challenge.cpp:2942-2967` | `challenge.rs:1721-1723` | `PvzpPickFromWeightedGridArray` 加权选格（有植物权重 1、无植物 100000）；Rust 硬编码 `(5, 2)` |
| 1.9 | `Challenge::TreeOfWisdomInit` / `Fertilize` / `Update` | `Challenge.cpp:5273`(49) / `5344`(19) / `5450`(58) | `challenge.rs:4653`(4) / `4647`(5) / `4631`(15) | 智慧树成长/施肥/对话内容（仅保留状态置位） |
| 1.10 | `Challenge::LastStandUpdate` 🔴 | `Challenge.cpp:5108-5130` | `challenge.rs:4308-4312` | C++ 在 `mNextSurvivalStageCounter == 0 && NORMAL && mBtnNoDraw` 时恢复商店按钮（`mBtnNoDraw=false; mDisabled=false`）并按 `mSurvivalStage` 设 `[START_ONSLAUGHT]`(`Resize(300,559,210,46)`) / `[CONTINUE_ONSLAUGHT]`(`Resize(270,559,257,46)`)；Rust 只有 `counter += 1`。关联证据：C++ `Board.cpp:224-225` 构造期置 `mBtnNoDraw=true/mDisabled=true`，而 Rust `widget/game_button.rs:65` 默认 `btn_no_draw: false`，仅在 `board.rs:6972-6985`（进入 onslaught）置 `true` → 按钮显示时机与文案均不一致 |
| 1.11 | `Challenge::ScaryPotterUpdate` 🔴 | `Challenge.cpp:4031-4051` | `challenge.rs:2547-2551` | C++ 判锤子 reanim `mLoopCount > 0` → `GetGridItemAt(SCARY_POT)` → `ScaryPotterOpenPot()` → 复位 `mChallengeGridX/Y`、`ReanimationDie()`、`mReanimChallenge=NULL`；Rust 仅状态置位。且 `scary_potter_mallet_pot`（`challenge.rs:3254-3259`）**不创建 `REANIM_HAMMER` 动画/`FOLEY_SWING`**，而是点击即 `open_pot` → 动画与音效缺失、开罐时机提前 |
| 1.12 | `Challenge::BeghouledMakeStartBoard` 🔴 | `Challenge.cpp:1069-1083` | `challenge.rs:4522-4526` | C++ 在 `BeghouledFillHoles` 后校验 `!BeghouledBoardHasMatch` 并调用 **`BeghouledCreatePlants(aEmptyBoardState, aBoardState)`**；Rust 未调用 `beghouled_create_plants`（该函数存在于 `:4528`）也不做校验 → 开局盘面无植物 |
| 1.13 | `Challenge::DrawStormNight` 🔴 | `Challenge.cpp:3085-3114` | `challenge.rs:1633-1640` | C++ 四分支（`FLASH_1` 再按 `counter > STORM_FLASH_TIME` 分 255/92；`FLASH_2` 用 `counter/2`；`FLASH_3 < 150`；否则全黑）并收尾调 `mBoard->DrawUIBottom` + `DrawTopRightUI`；Rust 仅单分支，且**确证不重绘 UI** —— `draw_weather`（`challenge.rs:3897-3904`）只调 `draw_rain`/`draw_storm_night`，而 `board.rs` 的调用顺序是 `:3369 draw_ui_bottom` → `:3372 draw_ui_top` → `:3379 draw_weather` → 风暴之夜的 else 分支全屏黑矩形（`Rect(-1000,-1000,2800,2600)`）会**盖住底部 UI（种子栏/铲子/小推车栏）与右上 UI** |
| 1.14 | `Challenge::UpdateZombieSpawning` | `Challenge.cpp` 各模式分支 | `challenge.rs:1577-1583` | Rust 仅处理 `WhackAZombie`，其余模式一律返回 `0` |
| 1.15 | 其余挑战函数 | `UpdateBeghouled:1547-1627`、`UpdateBeghouledPlant:1384`、`UpdateSlotMachine:2013`、`DrawWeather:3026`、`DrawRain:3035-3083` | `challenge.rs:573` / `716` / `1208` / `3897` / `3824` | ✅ **已逐行核对为已实现**（Rust 内部普遍带 C++ 行号对照注释）。唯一残留：`UpdateBeghouled` 中的 `AddPvzpParticle(BOARD_WIDTH/2, BOARD_HEIGHT/2, RENDER_LAYER_TOP, PARTICLE_SCREEN_FLASH)`（`Challenge.cpp:1622` ↔ `challenge.rs:705-708` 注释略过，归入 §4.3 粒子 API 缺失）与 `DrawRain` 的 `!mApp->Is3DAccelerated()` 早退（`Challenge.cpp:3037` ↔ `challenge.rs:3829-3830`，GL 加速标志未接入） |
| 1.16 | Beghouled 系列其余函数 | `LoadBeghouledBoardState:361`、`BeghouledTwist*:578/590/615/628`、`BeghouledIsValidMove:673`、`BeghouledDrag*:696/1135`、`BeghouledRemove*Match:765/780`、`BeghouledFallIntoSquare:794`、`BeghouledMakePlantsFall:814`、`BeghouledUpdateCraters:828`、`BeghouledClearCrater:838`、`BeghouledScore:860`、`BeghouledRemoveMatches:929`、`BeghouledPickSeed:990`、`BeghouledFillHoles:1037`、`BeghouledCheckForPossibleMoves:1103`、`BeghouledFlash*:1426/1444/1460/1505/1516`、`BeghouledCheckStuckState:2105`、`DrawBeghouled:2336`、`BeghouledShuffle:3528`、`BeghouledCanClearCrater:3542` | `challenge.rs:284/4356/4373/1133/351/357/449/459/782/795/1585/1593/1773/469/298/4512/1152/4196/4241/4216/4493/1174/1101/2376/2387` | 函数名与结构一一对应（Rust 全部存在）；`beghouled_make_start_board` 的缺口见 §1.12。**已核对**：网格边界常量一致（C++ `BEGHOULED_MAX_GRIDSIZEX=8`/`Y=5`（`Challenge.h:31/32`）↔ Rust `challenge.rs:318-319`、`1154-1155`、`4513-4514` 的 `0..8`/`0..5`），`beghouled_fill_holes:4512-4520` 逐行对应 `Challenge.cpp:1037-1048`。**新确证缺口见 §1.17** |
| 1.17 | `Challenge::BeghouledPickSeed` 非随机 🔴 | `Challenge.cpp:1034` `return PvzpPickFromArray(aPickArray, aCount);` | `challenge.rs:314` `if count > 0 { pick_array[0] } else { SeedType::Peashooter }` | **恒取候选数组首项**，而非等概率随机挑 → ① Beghouled 补种图案确定化；② **不消耗 RNG**，破坏与 C++ 的随机序列一致性。根因：Rust **无 `PvzpPickFromArray` 等价 API**（`todlib/tod_common.rs` 只有 `tod_pick_from_weighted_array:355`/`tod_pick_from_weighted_grid_array:361`），其它调用点以 `RandRange` 手工替代（`zen_garden.rs:2585` 注释、`zombie.rs:3521`、`tod_foley.rs:311` 注释），唯独此处遗漏 |
| 1.18 | `Challenge::BeghouledScore` 缺首得分分支与阳光加成 🔴 | `Challenge.cpp:860-927`（68 行） | `challenge.rs:1773-1797`（25 行） | 缺失 5 项：① `mApp->PlayFoley(FOLEY_ART_CHALLENGE)`（:862）；② **`mSeedBank->mNumPackets == 0` 首得分分支**（:876-891）—— 设置 4 个卡槽为 `SEED_REPEATER`/`SEED_FUMESHROOM`/`SEED_TALLNUT`/`SEED_BEGHOULED_BUTTON_SHUFFLE` 并 `mNumPackets=4`、`DisplayAdvice("[ADVICE_BEGHOULED_SAVE_SUN]")`，若 `BeghouledCanClearCrater()` 再补 `SEED_BEGHOULED_BUTTON_CRATER` 且 `mNumPackets=5` → **Rust 完全无此分支**，首次得分后玩家拿不到升级/洗牌/清弹坑按钮；③ else 分支的 `[ADVICE_BEGHOULED_MATCH_3]`（`{SCORE}` 数字替换）与 `[ADVICE_BEGHOULED_ALMOST_THERE]`（`score >= WINNING-5`）两条提示（:894-902）；④ `theNumPlants >= 5` 时 `aNumSuns += 2`（:914-917）—— Rust 的 `num_suns = (num_plants - 2 + matches).clamp(1,5)` 无此加成；⑤ `mChallengeScore = BEGHOULED_WINNING_SCORE` 钳位（:907） |
| 1.19 | `Challenge::DrawStormFlash` 无渐变/抖动 | `Challenge.cpp:3015-3024`（`PvzpAnimateCurve` 双层 + `MTRand(mMainCounter/6).NextNoAssert(64)-32` 抖动） | `challenge.rs:1799-1802`（4 行，恒 `Color(0,0,0,128)` 全屏填充，`_time`/`_max_amount` 弃用） | 闪电亮度渐变与随机抖动缺失（§1.13 的补充证据） |
| 1.20 | `Challenge::BeghouledCheckStuckState` 计数器与提示 API 不一致 | `Challenge.cpp:2105-2118` | `challenge.rs:1174-1186` | 两处差异：① `mChallengeStateCounter = **500**`（:2115）↔ Rust `challenge_state_counter = **1500**`（:1184）→ 无解棋盘时洗牌延迟相差 3 倍；② C++ 用 **`DisplayAdviceAgain(...)`**（:2116，同一提示可重复显示）↔ Rust 用 `display_advice(...)`（:1182）→ 已显示过该 advice 时 Rust 不再显示 |

---

## 2. Board 交互与输入

| # | 项 | C++ | Rust | 缺失内容 |
|---|---|---|---|---|
| 2.1 | `Board::KeyDown` | `Board.cpp:7653-7690` | `board.rs:3994-4002` | C++ 含 `DoTypingCheck`、LEVEL_INTRO 转发 `mCutScene->KeyDown`、`RETURN/SPACE`（推进 Crazy Dave 对话 / `SOUND_PAUSE`+`DoPauseDialog`）、`ESCAPE`（取消光标 / `DoNewOptions`）；Rust **只把可打印 ASCII 转发给 `key_char`**（调试键）→ 空格暂停、ESC 取消光标等全部缺失 |

> **已核对为等价（勿重复上报）**：`Board::Pause`（`board.rs:6423-6462` ↔ `Board.cpp:4672-4693`，唯一残留是 `mSoundSystem->GamePause` 未接线，代码内已 `TRANSLATION_NOTE` 标注）、`Board::UpdateToolTip`（`board.rs:8449-8826`，工具文案与 LEVEL_INTRO 分支齐备）、`Board::MouseDownWithPlant`（`:7598-8106`，IZombie 分派已接线于 `:7614`；**升级/替换已确证实现** —— `:7879-7934` 逐行对应 `Board.cpp:3805-3853`：`is_upgradable_to`（`plant.rs:2939`）、Gloomshroom 的 `is_asleep`/`wake_up_counter` 传递、Wallnut/Tallnut 同类替换、Pumpkinshell、Cobcannon 右侧格、Cattail 下层+普通植物）、`Board::UpdateCursor`（`:7000-7157`，5 处 `set_cursor`：`7046/7131/7137/7143/7148`）、`Board::ProcessDeleteQueue`（`Vec::retain` 等价于 `DataArrayFree`）、`Board::GetProjectileRect`（`projectile.rs:1037` 已按类型分派）、`Board::FindLawnMowerInRow`、`Board::GetNumSeedsInBank`、`Board::GetLevelRandSeed`。

---

## 3. 界面层

| # | 项 | C++ | Rust | 缺失内容 |
|---|---|---|---|---|
| 3.1 | `ChallengeScreen::DrawButton` 🔴 | `Widget/ChallengeScreen.cpp:401`（172 行） | `challenge_screen.rs:170` | 源码内 `TRANSLATION_NOTE` 明示跳过缩略图、窗口框、挑战名、锁/奖杯/记录文本（`:188`「Rust 侧图片资源未接入，暂略」） |
| 3.2 | `SeedChooserScreen::OnStartButton` / `PickRandomSeeds` / `ClickedSeedInBank` | `SeedChooserScreen.cpp:619`(90) / `710` / `810` | `seed_chooser_screen.rs:479` / `483` / `782` | 前两者只调 `close_seed_chooser()`（"随机"不选种子、开战无校验）；第三只改 `seed_state`，缺 `mSeedsInBank--` 与重排 |
| 3.3 | `AlmanacDialog::SetupPlant` / `SetupZombie` | `Widget/AlmanacDialog.cpp:149-167` / `169-178` | `almanac_dialog.rs:64-71` | C++ 会 `new Plant/Zombie` + `PlantInitialize/ZombieInitialize` + 设定图鉴坐标；Rust 仅 `clear_plants_and_zombies()` + `None` → 图鉴不显示模型 |
| 3.4 | `AlmanacDialog::SetPage` | `AlmanacDialog.cpp:180`(37) | `almanac_dialog.rs:72-75` | 只置 `open_page` |
| 3.5 | `AlmanacDialog::Update` 🔴 | `AlmanacDialog.cpp` | `almanac_dialog.rs:94-99` | Rust 为占位：鼠标坐标硬编码 `let mx = 0; let my = 0;`，且 `if self.seed_hit_test(mx,my) != SeedType::None \|\| self.zombie_hit_test(mx,my) != ZombieType::Invalid {}` **空 if 体** → 命中结果被丢弃；而 `draw_index:101-108` 已实现绘制 → **绘制有、交互无** |
| 3.6 | `AwardScreen::DrawAchievements` / `AchievementsContinuePressed` | `Widget/AwardScreen.cpp:606`(26) / `633`(31) | `award_screen.rs:471-473` / `475-477` | 前者函数体为空（「依赖图片资源，暂用占位」），后者仅 `showing_achievements = false`；奖励页链式跳转缺失；`:404`「交互未接入，仅创建商店」 |
| 3.7 | `MessageWidget::GetFont` / `SetLabel` | `Lawn/MessageWidget.cpp:329`(32) / `103`(75) | `message_widget.rs:104-106` / `64` | `GetFont` 恒 `None`；`SetLabel` 缺 `PvzpStringTranslate`、截断、重叠消息排队 `mLabelNext`；`:245`「`[FLAGS_COMPLETED]` 文案拼接…暂未接入」；`:269`「字体系统未接入，按每字符 10 像素近似 `StringWidth`」 |
| 3.8 | `GameSelector::ClickedAdventure` | `Widget/GameSelector.cpp:1136`(44) | `game_selector.rs:556-561` | 仅 `starting_game = true` + 禁用按钮；缺重玩确认/存档重置（`EraseFile`、`mLevel=24`）、`StopAllMusic`、`SOUND_LOSEMUSIC`。`:1105/1172/1205` 另有 zombatar/achievements widget 未接入、`ShowZombatarTOS` 未接入、`mAchievementsWidget` `SetFocus` 省略 |
| 3.9 | `CreditScreen::UpdateMovie` / `PreLoadCredits` / `DrawFogEffect` | `Widget/CreditScreen.cpp:1197`(303) / `397`(122) / `681`(41) | `credit_screen.rs:689`(44) / `765`(29) / `610`(6) | `:426`「reanim 轨道计数/计时器未接入，简化直接推进阶段」、`:441` 文字 `IMAGE_*` 未接入、`:613` 雾效图片未接入、`:731` reanim 事件系统未接入、`:751` 以 `do_dialog` 近似提示、`:936` 叠加效果未接入 |
| 3.10 | 全局 `IMAGE_*` 资源表机制缺失 🔴 | C++ `Sexy::IMAGE_*` 全局常量（`Resources.cpp` 注册） | 多处同一模式注释：`从 ResourceManager 按 key 取图（对应 C++ IMAGE_* 全局资源；未接入资源表时返回 null）` —— `award_screen.rs:58`、`store_screen.rs:195`、`zombatar_widget.rs:894`、`zombatar_tos.rs:97`、`new_options_dialog.rs:195` | 取不到即静默跳过绘制；直接后果：`store_screen.rs:177`（`IMAGE_STORE_*` 未接入，**不创建控件**，含 `mOverlayWidget`）、`imitater_dialog.rs:106`（跳过背景） |
| 3.11 | `ZombatarWidget` 面板不可交互 | `Widget/ZombatarWidget.cpp` | `zombatar_widget.rs:236` / `:673` / `:711` / `:726` / `:732` / `:1095` | 构造函数**未创建 9 个 `NewLawnButton`**（`IMAGE_ZOMBATAR_*` 皮肤未接入）；`button_press(&mut self, _the_id: i32) {}` 空实现。对比 `update_button_state:961` 有实质状态机 → **状态在、控件不在**。布局已核对：11 个常量（`PANEL_X/Y=25`、`PANEL_WIDTH=560`、`GRID_COLS=6`、`GRID_PAGE=17`、`GRID_GAP=-4`、`GRID_BIAS_X=50`、`COLOR_COLS=9`、`COLOR_GAP=4`、`COLOR_X=238`、`COLOR_Y=367`）与 `GetItemRect`/`GetColorRect` 公式**逐行一致**（`zombatar_widget.rs:712-741` ↔ `ZombatarWidget.cpp:628-652`），仅 3 处图片尺寸用占位值（67 / 39 / 21×20），见 §10-5 |
| 3.12 | `ZombatarTOS` | `Widget/ZombatarTOS.cpp` | `zombatar_tos.rs:52` / `:74` / `:94` / `:164` / `:170` / `:188` | 滑条值用 `f64` 简化、`MakeNewCheckbox` 未创建、子控件定位暂略、`button_press` 空、`KillDialog` 链占位 |
| 3.13 | 字符串本地化系统未接入 | `PvzpStringTranslate` + `[KEY]` 字符串表 | `seed_chooser_screen.rs:471`（「字符串翻译系统未接入，以键名作为原始文本」）、`store_screen.rs:499`（`format!("Page {} / {}", …)`）、`zombatar_widget.rs:1467/1692`（同） | 硬编码英文替代 `[PAGE]` 等键 |
| 3.14 | `UserDialog` 列表控件 | `widget/ListWidget.cpp`（348 行） | `framework/widget/list_widget.rs`（53 行）+ `user_dialog.rs:51` 自带简化 `ListWidget` | `user_dialog.rs:50` 自述「未实现全部接口（`SetColors`/`Justify`/`ItemHeight`/`Layout` 等）」；`:136-137`「`Layout` 简化版，仅记录 flags」；`:476`「不含 Widget 基座，无法 add_widget」；`:592`「简化版无 draw 方法」 |
| 3.15 | `CheatDialog` | `Widget/CheatDialog.cpp` | `cheat_dialog.rs:136/141` | 关卡/模式显示退化为 `format!("C{}", …)` / `format!("F{}", …)` |

**非差异（勿上报）**：`continue_dialog.rs:572-577`、`game_over_dialog.rs:41-47`、`new_options_dialog.rs:571-574` 的空按钮回调、`new_options_dialog.rs:296 update() {}` —— C++ 侧同为构造期占位。

---

## 4. 伤害、表示与实体行为

| # | 项 | C++ | Rust | 差异 |
|---|---|---|---|---|
| 4.1 | `Projectile::GetDamageFlags` | `Lawn/Projectile.cpp:401`(24) | `projectile.rs:1267-1274` | 源码自述「溅射/抛物线/倒走/星星+倒走 的盾牌穿越标志**暂未实现**」，实际只置 `Snowpea` 的 `1<<2` → 影响盾牌穿透判定 |
| 4.2 | `Zombie::BobsledCrash` | `Lawn/Zombie.cpp:2611`(27) | `zombie.rs:2601-2609` | 注释「依赖底层系统」→ 缺跌倒音效与粒子 |
| 4.3 | 粒子接线 API 缺失 🔴 | `LawnApp::AddPvzpParticle`（130 处调用）；`Reanimation::AttachParticleToTrack`（`PvzpLib/Reanimator.cpp`） | `src/` 全库无等价入口（仅 `todlib/tod_particle.rs` 有 3 处 `spawn_particle` 类底层 API） | 掉头/断臂/掉盾/掉盔/出土/水花等表现无法接线 |

> **已核对为已实现（勿重复上报）**：`Zombie::MowDown`(`zombie.rs:4008`)、`ApplyBurn`(`4282`)、`PogoBreak`(`4572`)、`UpdateYuckyFace`(`4788`)、`ShowYuckyFace`(`7254`)、`UpdateZombieWalking`(`4909`)、`IsMovingAtChilledSpeed`(`5059`)、`UpdateDamageStates`(`5933`)、`RiseFromGrave`(`4398`)、`BungeeLiftTarget`(`10908`)、`PlayDeathAnim`(`8174`)、`DropShield`(`6445`)、`DropHelm`(`6361`)、`DropHead`(`6567`)、`TakeHelmDamage`(`5431`)、`DrawBobsledReanim`(`9120`)、`TakeBodyDamage` 的 Boss 血量钳位（`zombie.rs:5917-5920`）；`Plant::FindTargetZombie`(`plant.rs:1133`)、`AttachBlinkAnim`(`3100`)。

---

## 5. 渲染层

| # | 项 | C++ | Rust | 差异 |
|---|---|---|---|---|
| 5.1 | **旋转 blt** 🔴 | `MemoryImage::BltRotated`（`MemoryImage.cpp:1470-1527`，58 行）+ `BltRotatedClipHelper`（`:1392`，43 行） | `image.rs:488-504` | 函数体注释自述 **「简化：忽略旋转，直接 blt」**，`_rot`/`_rot_center_x`/`_rot_center_y` 均弃用 → 软件路径下所有旋转绘制退化为普通 blt。调用链：`Graphics::draw_image_rotated_f`（`graphics.rs:786/790`）→ `dest.blt_rotated` |
| 5.2 | **GL 路径无软件回退** 🔴 | C++ `MemoryImage` 始终有软件实现 | `gl_image.rs:252-288` | `GLImage::blt_rotated`/`stretch_blt` 仅在 `gl_interface` 非空时绘制，为空则**静默丢弃** |
| 5.3 | **拉伸 blt** | `SlowStretchBlt:1528`(44) + `FastStretchBlt:1574`(65) + `StretchBltClipHelper:1436` + `StretchBltMirrorClipHelper:1451` | `image.rs:507-545` | `_clip_rect`/`_fast_stretch` 弃用（不裁剪、无快慢两路），采样为最近邻 |
| 5.4 | **双线性过滤/浮点裁剪** | `NormalBlt:1321`(42) / `AdditiveBlt:1281`(39) 走 `Blt(..., linearFilter)` | `image.rs:442-477`（`_linear_filter` 弃用）、`blt_f:480`（`_clip_rect` 弃用） | 编译告警互证：`memory_image.rs:160:114: unused variable: linear_filter`、`gl_image.rs:319:9: unused variable: clip_rect` |
| 5.5 | **抗锯齿线段** | `MemoryImage::NormalDrawLineAA:617`(79) + `DrawLineAA:697`；`NormalDrawLine:116`(287) / `AdditiveDrawLine:404`(179) | `image.rs:392-395` | `draw_line_aa` 直接转调 `draw_line`（注释「目前使用普通 Bresenham」）→ **AA 完全缺失**；`additive_draw_line` 无对应 |
| 5.6 | **SWTri 软件路径空壳** 🔴 | `SWTri.cpp`：`SWDrawShape:269`（约 197 行）、`SWTri_AddDrawTriFunc:466`、`SWTri_AddAllDrawTriFuncs:479`、`SWDrawTriangle:613` + 9 个裁剪辅助（`lClip/rClip/tClip/bClip:42/55/68/81`、`leClip/reClip/teClip/beClip:94/133/172/211`、`clipShape:250`） | `todlib/sw_tri.rs:319-355` | `Tod_SWTri_AddAllDrawTriFuncs`/`SWDrawTriangle`/`TodBltMatrix`/`TodSandImageIfNeeded`/`BltMatrixHelper`/`BltTrianglesTexHelper` 体仅注释，`NormalDrawLine() {}`/`NormalDrawLineAA() {}`/`Ref() {}` 空体 → 同文件 `:100-315` 由 `draw_triangle_variant!` 展开的像素格式变体（0555/565/888/8888 × TEX/TALPHA/MOD/GLOB/BLEND）**无注册入口 = 死代码** |
| 5.7 | `Quantize8Bit` | `Quantize.cpp:31` | `quantize.rs:22+` | **一致**（真实实现：首像素初始化 + 二分查找 + 翻译表） |
| 5.8 | `Graphics::DrawImageTransformHelper` 缺 3D 条件 | `Graphics.cpp:860` `if (mComplex \|\| (GLImage::Check3D(mDestImage) && useFloat))` → 走 `DrawImageMatrix` | `graphics.rs:833` 仅 `if transform.complex` | `GLImage::check_3d` 已存在于 `gl_image.rs:89-91` 但**未接线**：GL 3D 加速 + 浮点绘制时应走矩阵路径，Rust 仍走 trans/scale/rot 快速路径 |
| 5.9 | `Graphics::DrawImageTransformHelper` 的 rot 分支忽略 `use_float` | `Graphics.cpp:878-881`：`useFloat ? DrawImageRotatedF : DrawImageRotated` | `graphics.rs:848-852`：两个分支**都**调 `draw_image_rotated_f` | 整型路径（`draw_image_rotated`）从未被使用 → 亚像素定位与 C++ 整型路径不同（轻微）。其余环节（`w2/h2`、`rx/ry`、`trans_x2 - rx + 0.5`、scale 的 `-1/1` 判断、mirror 与 stretch 调用）已逐行核对一致；`DrawImageMirror(..., bool mirror = true)`（`Graphics.h:153-155`）的默认值也与 Rust 显式传 `true` 一致 |
| 5.10 | **运行时图像类型 = `MemoryImage`（软件路径）** | C++ `mDestImage` 可为 `GLImage`（3D 加速）或 `MemoryImage` | `resource_manager.rs:547-565`：`load_single_image` **只创建 `MemoryImage`** 并 `set_bits`，再（若 `app.gl_interface` 存在）调 `create_image_texture` 挂 GL 纹理；`GLImage` 类型**从未被实例化**（仅 `gl_image.rs` 内部定义） | 因此 `Image`/`MemoryImage` 的软件实现是**实际执行路径**：§5.1（旋转忽略）、§5.3（忽略裁剪 + 最近邻）、§5.4（`linear_filter` 无效）、§5.5（无 AA）**全部在运行中生效**；而 §5.2 的 GL 路径（`GLInterface::blt/blt_rotated/stretch_blt/blt_transformed`，`gl_interface.rs:1856-1967`，均为矩阵 + 纹理的真实现）**当前未被使用** |

> **已核对为一致/非缺口**：`MemoryImage` 已退化为转发层（`memory_image.rs:136-241` 全部 `self.base.*`，实现在 `image.rs`）；`Graphics::draw_image_*` 家族（`graphics.rs:534…1066`）比 C++ 更细分；`Graphics` 约 70 个方法在 `graphics.rs` 全部有对应（`push_state:174`/`pop_state:180`/`fill_rect:347`/`draw_rect:375`/`poly_fill:1554`/`poly_fill_aa:1644`/`draw_triangle_tex:887`/`draw_triangles_tex:906`/`clear_clip_rect:989`/`set_clip_rect:1011`/`clip_rect:1022`/`set_scale:1043`/`write_string:1119`/`draw_string:498`/`string_width:511` 等）；`GLInterface` 纹理链完整（`create_textures:954`/`check_create_textures:1024`/`get_texture:1036`/`copy_to_texture_8888:718`/`_4444:781`/`_565:845`/`_palette8:909`/`release_textures:529`）→ `TextureData::GetTextureF` 属命名差异。

---

## 6. 音效层

| # | 项 | C++ | Rust | 差异 |
|---|---|---|---|---|
| 6.1 | `SDLSoundManager::LoadAUSound` | `sound/SDLSoundManager.cpp:85`（161 行） | `sdl_sound_manager.rs:166` | 恒 `0` |
| 6.2 | `SDLSoundManager::GetSoundInstance` / `ReleaseSounds` | `SDLSoundManager.cpp:346` | `:168` / `:170` | 恒 `0` / 空 |
| 6.3 | `SDLSoundInstance::Play` 🔴 | `SDLSoundInstance.cpp:206`（29 行）+ `PitchHandlerFuncCallback:97`（73 行） | `sdl_sound_instance.rs:20-24` | `play` **仅 `self.playing = true`**（`stop`/`set_volume`/`set_pan` 同）→ 无任何 `Mix_*` 调用；`PitchHandlerFuncCallback` 无对应 → `GetSoundInstance()->Play()` 链路无声音（`load_sound:98`/`play_sound:122` 的直接播放路径已实现） |
| 6.4 | `SDLMusicInterface::FadeIn` / `FadeOut` / `FadeOutAll` | `SDLMusicInterface.cpp:177`(17) 等 | `sdl_music_interface.rs:216` / `219` / `220` | `fade_in` 忽略 `_offset`/`_speed` 直接 `play_music`；`fade_out`/`fade_out_all` 直接 `halt_music()` → **无渐变** |

---

## 7. 框架 / 平台 / 资源

| # | 项 | C++ | Rust | 差异 |
|---|---|---|---|---|
| 7.1 | `ResourceManager::LoadNextResource` | `misc/ResourceManager.cpp:911` | `resource_manager.rs:315` | 恒 `false`（异步加载队列不推进；是否被同步路径替代未确证） |
| 7.2 | `ReplaceImage` / `ReplaceSound` / `ReplaceFont` | `ResourceManager.cpp:1140` 等 | `:569/572/575` | 恒 `false` |
| 7.3 | `LoadImage` / `GetSound` / `DeleteSound` / `DeleteFont` / `DeleteResources` | `ResourceManager.cpp:778` 等 | `:590`（空 `SharedImageRef`）/ `:617`（恒 0）/ `:593` / `:596` / `:640`（空） | 资源加载/替换/释放桩化 |
| 7.4 | `ImageLib` 写图与 libjpeg 回调 | `imagelib/ImageLib.cpp`（31 KB） | `imagelib/mod.rs:181/192/206`（`write_jpeg/png/tga_image`）与 `:222-235`（`my_error_exit`/`init_source`/`fill_input_buffer`/`skip_input_data`/`term_source`/`f_get_c`/`f_get_s`）全为空实现 | 图像导出路径失效 |
| 7.5 | `WidgetContainer::PutBehind` / `PutInfront` | `WidgetContainer.cpp:301/320`（调用点：`Dialog.cpp:303/305` 把 Yes/No 按钮置顶、`ListWidget.cpp:346/348` 把滚动条置顶） | `widget_container.rs:154/166` 只有 `bring_to_front`/`bring_to_back` | 对话框按钮与列表滚动条的 Z 序提升缺失 |
| 7.6 | `ListWidget` | `widget/ListWidget.cpp`（348 行：`Layout`/`Resize`/`Sort`/`RemoveAll`/`GetLineIdx`/`GetOptimalHeight`/`GetOptimalWidth`/`Draw`/滚动条集成） | `list_widget.rs`（53 行：仅 `new`/`set_line_color`/`set_hilite`） | 极简版 |
| 7.7 | `TextWidget` 文本选择 | `TextWidget::SelectionReversed`/`GetSelectedIndices`/`GetStringIndex`/`DrawColorStringHilited` | `text_widget.rs:221-223`（`mouse_down`/`mouse_drag`/`key_down` 空） | 长文本不可选中/高亮 |
| 7.8 | platform 输入/窗口层 | `platform/default/Input.cpp`（22 KB：`InitInput`/`SetTextInputRect`）、`platform/default/Window.cpp` | 全库无 `init_input`/`set_text_input_rect`/`process_events`/`create_window`/`handle_events`/`struct SexyApp`（重跑确认 `exit=1`） | 函数级对应缺失（是否有等价替代**未确证**） |
| 7.9 | 资源路径查找 | `PvzpResourceManager::FindImagePath`/`FindSoundPath`/`FindFontPath`；`Sexy::GetResourcePath`/`GetResourceFolder`/`GetPathFrom` | 全库无对应（`exit=1`）；加载仅走 `parse_resources_file:133` → `load_single_image:373`（依赖 XML 内路径） | **无多目录回退查找** |
| 7.10 | `SexyAppBase::DoMainLoop` / `Process`（及未移植的辅助函数） | `SexyAppBase.cpp` | ✅ **主循环已实现（改名/内联）**：`sexy_app_base.rs:706-709` `while !self.m_shutdown_flag { self.update_app(); }`（内联于 `start()`）；`update_app:716+` 依次 `process_demo()` → `process_deferred_messages(true)`（SDL 事件泵 ↔ C++ `ProcessDeferredMessages`）→ `do_update_frames()`（↔ C++ `Process → DoUpdateFrames → UpdateFrames`）→ `draw_dirty_stuff()`；`main.rs:53` 调 `app.start()` | 主循环语义对应完整，仅命名与组织不同 |
| 7.11 | `SexyAppBase::ClearUpdateBacklog` | `SexyAppBase.cpp:453`（调用点 `:1918` `EndPopup`、`:2607` 快进循环 `mFastForwardToUpdateNum/mFastForwardToMarker`） | 全库无对应（`src/` 仅 `todlib/tod_common.rs:430 rgb_to_hsl`，无 `hsl_to_rgb`/`flip_image`/`rotate_image_hue`/`crossfade`/`precache`/`backlog`） | ✅ **属未移植的小缺口**：Rust 无更新积压清理 → demo 快进与弹窗结束后的时间基准/追赶行为与 C++ 不同 |
| 7.12 | `SexyAppBase::FlipImage` / `RotateImageHue` / `CreateCrossfadeImage` / `PrecacheAdditive` / `PrecacheAlpha` / `HSLToRGB` | `SexyAppBase.cpp:3872/3899/3635/4045/4050/3969/4027` | 无对应实现 | ⚪ **非缺口**：这些函数在 `cpp/src`（排除第三方）**只有定义与头文件声明、无任何调用点** —— 属 SexyAppFramework 未使用的遗留 API（C++ 侧亦为死代码） |
| 7.13 | `Dialog::WaitForResult` | `widget/Dialog.cpp:397` | `framework/widget/dialog.rs:176` | 仅 `return self.result`，无模态等待语义 |
| 7.14 | `Attachment::Draw` / `OverrideScale` / `OverrideColor` | `PvzpLib/Attachment.cpp:458/339/238`（63/43/43 行） | `todlib/attachment.rs:816/752/737`（约 10 行） | 大幅简化 |
| 7.15 | `MTRand::Serialize` | `misc/MTRand.cpp:199-221`（`MTRAND_N*4 + 4` 字节：`mt[]` + `mti`） | `framework/mt_rand.rs:172-178`（仅 `MTRAND_N*4` 字节：`mt[]`，**少 4 字节 `mti`**） | ⚪ **非功能缺口**：`Serialize()` 在 `cpp/src`（排除第三方）**只有声明与定义、无任何调用点**；Rust 的 `serialize()` 同样无调用点 → 少 `mti` 仅是代码层面差异 |
| 7.16 | `XMLParser::NextElement` | `misc/XMLParser.cpp:335`（383 行） | `framework/xml_parser.rs:147`（87 行，已拆分为 `parse_end_tag`/`parse_start_or_element` 等） | 旧「缺 section 栈」判断已不适用，**逐行等价性未确证** |

**文件级缺口判定**（27 项「cpp basename 无 Rust 对应」）：

- **改名/合并（非缺口）**：`ConstEnums.h`→`game_enums.rs`、`CutScene.cpp`→`cutscene.rs`、`PvzpCommon/PvzpDebug/PvzpFoley/PvzpList/PvzpParticle/PvzpStringFile`→`todlib/tod_common|tod_debug|tod_foley|tod_list|tod_particle|tod_string_file.rs`、`PakInterface`→`framework/paklib/mod.rs`、`CheckboxListener`/`EditListener`/`ListListener`/`ScrollListener`/`SliderListener`/`DialogListener`→各 widget 文件内 trait（`checkbox.rs:14`/`edit_widget.rs:16`/`list_widget.rs:14`/`scrollbar_widget.rs:15`/`slider.rs:14`/`dialog_listener.rs:5`）、`TriVertex.h`→`gl_interface.rs:47`、`SexyVector.h`→`common.rs:235/313`、`BoardInclude.h`/`GameConstants.h`/`resource.h`/`memmgr.h`/`GLPlatform.h`（纯头文件/常量）
- **真缺口**：`platform/default/Input.cpp`、`platform/default/Window.cpp`（见 7.8）、`imagelib/ImageLib.cpp` 的写图路径（见 7.4）
- `PvzpCommon` 落点核查：`PvzpAnimateCurve`→`tod_animate_curve`(`tod_common.rs:219`)、`_float`(`207`)、`_float_time`(`280`)、`PvzpPickFromWeightedArray/GridArray`→`tod_pick_from_weighted_array`(`355`)/`tod_pick_from_weighted_grid_array`(`361`)、`PvzpCurveEvaluate`→`pvzp_curve_evaluate`(`120`)、`PvzpStringListFind`→`pvzp_string_list_find`(`228`)。未找到对应的：`FindImagePath`/`FindSoundPath`/`FindFontPath`/`AddImageToMap`/`PvzpLoadResources`（见 7.9）

---

## 8. 存档

| # | 项 | C++ | Rust | 差异 |
|---|---|---|---|---|
| 8.1 | 格式常量 | `SaveGame.cpp:48/49/52/53/94` | `save_game.rs:36/39/40/41/42` | **一致**：`0xFEEDDEAD` / `2U` / `"PVZP_SAVE4"` / `1U` / `SAVE4_CHUNK_VERSION=1U` |
| 8.2 | `PortableSaveContext` 段 | `SyncBoardBasePortable:1555` … `SyncBoardPortable:2259`（约 50 个） | `sync_game_object_portable:405` … `sync_projectiles_portable:2661`（约 50 个，逐名对应） | **一致**（`PVZP_SAVE4` 为 C++ 原版既有设计，非 Rust 自创） |
| 8.3 | **旧格式存档读取** 🔴 | 保留 `SaveGameContext` 一族：`SyncDataIDList:2851`、`SyncParticleEmitter:2891`、`SyncParticleSystem:2919`、`SyncReanimation:2935`、`SyncTrail:2973`、**`SyncBoard:3017`** | 仅 Portable 一族，**无旧格式对应实现**；`SAVE_FILE_MAGIC_NUMBER`(:36)/`SAVE_FILE_VERSION`(:39) 定义后**无任何引用** | 旧版（`0xFEEDDEAD`/v2）存档无法在 Rust 版加载 |
| 8.4 | `BoardBase` 字段集 | `case BOARD_FIELD_*` **103 个**（`SaveGame.cpp:1575-1674+`，`switch` + `ApplyFieldWithSync`） | `BoardBaseFieldId::` 全文件 **206 处引用** = 写侧 `append_board_base_fields:989-1169` 与读侧 `apply_board_base_field:1170-1310` 各 **103** 个 | ✅ **字段数完全对齐**（103 ↔ 103）；分发方式由 `switch` 改为按 field_id 的函数（语义等价） |
| 8.5 | `Zombie` 段字段级 | `SyncZombieTailPortable:786-864`（79 个 `Sync*` 调用） | `sync_zombie_tail_portable:1798-1879`（82 行） | ✅ **逐字段同序同宽**：`SyncEnum32(mZombieType/Phase/Height/HelmType/ShieldType/AttachmentID)`、`SyncFloat(posX/posY/velX/altitude/scaleZombie/velZ/originalAnimRate)`、`SyncRectPortable(zombieRect/zombieAttackRect)`、`SyncEnumU32(RelatedZombieID/BodyReanimID/TargetPlantID/BossFireBallReanimID/SpecialHeadReanimID/MoweredReanimID/ZombatarHeadReanimID)`、`SyncEnumU32Array(FollowerZombieID, MAX_ZOMBIE_FOLLOWERS)` 等全部一一对应；Rust 另以注释标注 enum→整型宽度的等价映射（如 `mAttachmentID`→i32、`mRelatedZombieID`→u32） |
| 8.6 | 其余 chunk 段字段级（**自动比对**） | `Sync*Portable` 共 62 个函数（`SaveGame.cpp`） | `sync_*` 共 48 个函数（`save_game.rs`） | ✅ **39 个段字段数完全一致**（`cppN == rsN`）：Zombie 76=76、Plant 47=47、Projectile 27=27、Coin 23=23、Challenge 22=22、GridItem 21=21、Music 19=19、LawnMower 16=16、PottedPlant 15=15、ReanimTrackInstance 14=14、SeedPacket 12=12、CursorObject 10=10、MessageWidget 9=9、ReanimTransform 8=8、SeedPackets 7=7、AttachEffect 5=5、SmoothArray 4=4、MotionTrailFrame 3=3、AttachmentTail 3=3、SeedBankTail 3=3、Zombies/Projectiles/Coins/Mowers/GridItems/ParticleSystems/Reanimations/Attachments 各 3=3、CursorPreviewTail/CursorPortable/CursorPreview/Advice/SeedBank/Challenge/Music 各 2=2、BoolArray/SmoothArrayList/ParticleEmitters/Particles 各 1=1。**字段序列自动比对**（归一化 `mXxx`→`xxx`）：**14 段序列完全相同**（`PlantTail` 47、`PottedPlant` 15、`ReanimTransform` 8、`GameObject` 7、`AttachEffect` 5、`Rect` 4、`SmoothArray` 4、`AttachmentTail` 3、`MotionTrailFrame` 3，及 Challenge/Cursor/CursorPreviewTail/Music/Vector2 各 2）；其余段的不匹配位置数远小于字段数（如 Zombie 76 中仅 1 处、Plant 0 处、GridItem/LawnMower/MessageWidget/Music 各 1 处），经抽查**均为命名/写法差异**（`mXxx` 对自定义 Rust 字段名、数组下标提取、`sync_matrix` 等辅助函数），**未发现顺序错位**。**人工抽样另核验两段**：`SeedPacket`（`SyncSeedPacketTailPortable:710-723` 12 项 ↔ `sync_seed_packet_tail_portable:1956-1969` 12 项，差异仅命名 —— `mRefreshCounter`→`countdown`、`mIndex`→`packet_index`、`mPacketType`→`seed_type`、`mSlotMachining*`→`slot_machine_*`）与 `Challenge`（`SyncChallengeTailPortable:726-744+` 22 项 ↔ `sync_challenge_tail_portable:1972-1996+`，差异仅命名 —— C++ `mBeghouledPurcasedUpgrade` 拼写 vs Rust `beghouled_purchased_upgrade`；`SyncInt32Array(mBeghouledEated, 9*6)` ↔ 逐行 `sync_i32_array` 等价），**结论均为同序对应** |
| 8.7 | `MagnetItem` 段缺字段 🔴 | `SyncMagnetItemPortable:644-651`（**5 个**：4×`SyncFloat` + `SyncEnum32(mItemType)`） | `sync_magnet_item_portable:1710-1715`（**4 个**：仅 `pos_x/pos_y/dest_offset_x/dest_offset_y`） | **缺 `item_type`（`MagnetItemType`）** → 磁力菇吸附物品的类型不入存档，读档后默认化。**另：`Coin` 段已人工逐字段核对为完全一致**（`SyncCoinTailPortable:949-974` 23 项 ↔ `sync_coin_tail_portable:1459-1487` 24 项），其中类型差异以中转过桥保持格式：C++ `mGroundY`(int32) ↔ Rust `ground_y as i32`（内部 f32）、`mAttachmentID`(enum) ↔ `attachment_id`(i32) |
| 8.8 | reanim/particle 的定义与 image override 用占位 | `SyncImagePortable:477`（调用点 `:1131`/`:1274`）、`SyncReanimationDefPortable:371`（`:1142`）、`SyncParticleDefPortable:407`（`:1308`）、`SyncTrailDefPortable:442`（`:1330`） | `save_game.rs` **无这些函数**；改以占位替代：`sync_reanim_track_instance_portable:2412-2416`（`image_override = 0` 写、读侧置 `null_mut()`）、`sync_reanimation_portable:2427-2435`（写 `reanim_type` 作 `def_index`、读侧 `let _ = def_index` 丢弃）、`sync_particle_system_portable:2597-2600`（写 `effect_type` 占位、读侧丢弃） | 代码内 `TRANSLATION_NOTE` 自述"Rust 无资源反查表，占位" → 存档中**定义引用与 image override 信息丢失**（架构性差异，非编译问题） |

> 注意：`save_game.rs` 中 `// C++: 2U/3U 为旧版字段（legacy）` 等注释（`:1618/1696/1785/1894/2066/2118/2158/2203/2260/2317/2366/2538/2673`）指的是 **Portable 格式内部的字段版本号**，与 8.3 的「旧文件格式」无关。

---

## 9. 本次推翻的旧报告条目（**已修复或仅改名，勿再跟踪**）

`Coin::CoinInitialize` 运动/视觉、`Board::FindLawnMowerInRow`、`Board::GetNumSeedsInBank`、`Board::GetProjectileRect`（现按类型分派，`projectile.rs:1037`）、`Board::GetLevelRandSeed`、`ZenGarden::PickRandomSeedType`、`Zombie::TakeBodyDamage` 的 Boss 血量钳位（`zombie.rs:5917-5920`）、`Zombie::DropShield`/`DropHelm`/`DropHead`/`TakeHelmDamage`、`Zombie::DrawBobsledReanim`、`Zombie::MowDown`/`ApplyBurn`/`UpdateZombieWalking`/`PogoBreak`/`UpdateDamageStates`/`UpdateYuckyFace`/`ShowYuckyFace`/`IsMovingAtChilledSpeed`/`RiseFromGrave`/`BungeeLiftTarget`、`Plant::AttachBlinkAnim`、`Plant::FindTargetZombie`、`Board::ProcessDeleteQueue`、`PoolEffect::*`、`SeedBank::UpdateWidth`/`RefreshAllPackets`、`XMLParser::NextElement`、`LawnApp::CanPauseNow`/`NeedPauseGame`/`GetNumTrophies`/`OpenURL`、`Challenge::ScaryPotterPopulate`/`IZombieInitLevel`/`WhackAZombieSpawning`、`Board::UpdateToolTip`/`MouseDownWithPlant`/`UpdateCursor`/`Pause`、`SeedBank::UpdateWidth`、`GridItem::OpenPortal`、`Music::StartGameMusic` 夜/池/屋顶分支、`Coin::GetSunValue`、`Plant::get_cost`、`RenderLayer` 常量表。

**改名（非缺失）**：`Plant::MagnetShroomAttactItem`→`magnet_shroom_attack_item`、`GetFreeMagnetItem`→`get_free_magnet_item_idx`、`Coin::IsSun`→`is_sun_type`、`Board::StageHas6Rows`→`stage_has_6_rows`、`Zombie::IsImmobilizied`→`is_immobilized`、`Zombie::CountBungeesTargetingSunFlowers`→`count_bungees_targeting_sunflowers`、`Board::UpdateWidth`→`board.rs:9259`（`update_seed_bank_width`）、`Reanimation::GetFrameTime`（`reanimator.rs:930`）、`DataReader::OpenMemory`（`data_sync.rs:41`）。

**死代码等价**：`Zombie::DrawZombie`/`DrawZombieHead`/`DrawZombieWithParts` 在 C++ 处于 `/* … */` 注释块内（`Zombie.cpp:5072` 收尾 `*/`），Rust 的空实现不算缺口。

---

## 10. 覆盖范围与未验证项

**已覆盖**：`cpp/src/Lawn`（含 `System`/`Widget`）、`cpp/src/LawnApp.cpp`、`cpp/src/SexyAppFramework`（含 `graphics`/`widget`/`sound`/`paklib`/`imagelib`）、`cpp/src/PvzpLib`（≈`todlib`）、存档系统；259 条规模悬殊候选逐条双端复核 + 356 条名字级未命中筛查 + 一次真实 `cargo check`。

**未覆盖 / 未确证**：

1. 已通过 `cargo check`（0 error）与 `cargo build`（debug，链接成功，产物 `target/debug/pvz-portable.exe` 7,041,536 字节），但**未运行游戏** → 无运行期行为验证（渲染/音效/输入的实际表现未实测）。
2. C++ 有而 Rust 连名字都不存在的低价值项未穷尽核对。
3. `SexyAppBase`/`Graphics`/`GLInterface` 个别函数的逐行语义（仅做存在性与实现质量抽查）。
4. 存档：39 个段的字段数已**自动比对为完全一致**，其中 14 段**字段序列完全相同**、其余段的不匹配位置经抽查均为命名/写法差异（§8.6）；**逐项人工核验**仅完成 `BoardBase`（§8.4，103↔103）与 `Zombie`（§8.5）两段；`MagnetItem` 缺字段已确证（§8.7）、reanim/particle 的 def 与 image override 为占位（§8.8）；`MTRand::Serialize` 的 `mti` 缺失是否影响存档互操作未确证。
5. `ZombatarWidget` 的 11 个布局常量与 `GetItemRect`/`GetColorRect` 公式已确证逐行一致（见 §3.11）；仅 3 处**图片尺寸占位值**（`aCellW/H=67`、`aTabH=39`、`aSwatchW/H=21×20`）无法从源码确证 —— C++ 从 `IMAGE_ZOMBATAR_ACCESSORY_BG`/`IMAGE_ZOMBATAR_COLORPICKER`/`GetCategoryImage(...)->mWidth/Height` 读取，而 `properties/resources.xml` 只注册 `id → path`（如 `<Image id="ZOMBATAR_ACCESSORY_BG" path="zombatar_accessory_bg" />`），真实尺寸在 `main.pak` 内。
6. 排除项：`SDL-Mixer-X`、`glad`、`fcaseopen`、`platform/emscripten`、`platform/switch`、`MusPlayer_Qt`、`EchoTune`、`pxtn*`、`PGE_*` 等第三方/调试工具不属游戏逻辑，未计入缺口。

> Beghouled 系列（`Challenge.cpp` 中 30+ 个函数）已**全部核对完毕**：缺口见 §1.17（`BeghouledPickSeed` 非随机）、§1.18（`BeghouledScore` 5 项缺失）、§1.19（`DrawStormFlash`）、§1.20（`BeghouledCheckStuckState` 计数器/提示 API）；一致或等价项见 §1.16 及本节历史结论（`FillHoles`、`MakePlantsFall`、`UpdateCraters`、`ClearCrater`、`RemoveMatches`、`Horizontal/VerticalMatchLength`、`BoardHasMatch`、`Shuffle`）。

> 已消除的未确证项：`Challenge::DrawStormNight` 的 UI 覆盖问题已**确证**（见 §1.13）；`Board::MouseDownWithPlant` 的升级/替换逻辑已**确证实现**（见 §2 等价清单）；`Challenge::{UpdateBeghouled,UpdateBeghouledPlant,UpdateSlotMachine,DrawWeather,DrawRain}` 已**逐行核对为已实现**（见 §1.15）。

---

## 11. 复现方法

```bash
# ① 文件级映射：cpp/src 与 src 的 basename 归一化比对
# ② 名字级差集：C++ "Class::Method(" 定义 vs Rust 全 fn（snake 归一化）
# ③ 函数体规模悬殊候选：brace 平衡扫描 → C++ 体 ≥15 行且 Rust 同名 ≤1/8
# ④ 编译验证：never-used 告警即接线断裂证据
cargo check --message-format short

# 关键项复核
rg -n "fn init_zombie_waves" src/lawn/challenge.rs
rg -n -A 8 "pub fn find_lawn_mower_in_row|pub fn get_num_seeds_in_bank" src/lawn/board.rs
rg -n -A 6 "pub fn get_projectile_rect" src/lawn/projectile.rs
rg -n "self.body_health = 1;" src/lawn/zombie.rs
rg -n -A 4 "fn load_au_sound|fn play\b" src/framework/sound/
rg -n "SAVE_FILE_MAGIC_NUMBER|SAVE_FILE_VERSION" src/lawn/system/save_game.rs
rg -n "fn find_image_path|fn init_input|fn create_window" src/   # 期望无匹配
```

> 本次审计全程只读；写操作仅有经用户授权的 `cargo check` 与 `cargo build`（仅生成 `target/` 构建产物，源码未动）。本文件的覆写由用户明确要求。
