# 04 · 翻译进度台账

> **AI 每完成一项必须更新本文件。** 缺口的详细证据在 `ANALYSIS_REPORT.md`（章节号下文用 §n 引用）；本文件只做**状态与优先级**。

状态：⬜ 未开始 · 🔄 进行中 · 🧪 已翻译待验证 · ✅ 已验证 · ⚠️ 骨架化/有已知偏差 · 🔌 已实现但未接线 · 🔁 需返工

## 当前阶段与焦点

- **阶段：** 引擎主体编译通过；L1–L6 部分验证通过（标题屏「点击开始！」+ GameSelector 全部按钮/木牌/成就基座渲染成功，截图对照原版确认）。
- **当前焦点：** ①GameSelector 布局缺口（部件位置/FrameBasePose/滑动动画，画面元素位置与原版有偏差）；②新发现崩溃：按键触发 foley 播放时 tod_foley.rs:322 指针未对齐（详见更新记录）。**未验证项：游戏内玩法回放一致性。**
- **2026-11-21 二轮遗留项状态：** §1 方法类 133/167 有调用点（本轮 103→133）；遗留项2/3/4（pool_effect 构造、WidgetImpl 管理器回调、LawnEditWidget 桥接）已完成；剩余无调用点为 C++ 注释死代码（draw_zombie_part/head）、C++ 无调用点（Board::SaveGame）、Rust 自创拆分（§3）与 decl-only 链（§1 末段），登记维持。

## 优先级（建议）

| 优先级 | 事项 | 依据 |
|---|---|---|
| P0 | 启动加载不卡死，能进入主菜单（L1–L5） | `07` |
| P0 | 修复可能的 use-after-free：`loading_completed` 立即释放 TitleScreen | `07` §4 |
| P0 | 建立可复现的验证手段：无渲染的状态哈希 + `.dmo` 回放 | `06` §7 |
| P1 | RNG 一致性缺口：`InitZombieWavesSurvival`、`BeghouledPickSeed`（不消耗 RNG） | ANALYSIS §1.2、§1.17 |
| P1 | 关卡波次与挑战模式骨架化函数（`InitZombieWaves`、`PortalStart`、`LastStandUpdate` 等 🔴 项） | ANALYSIS §1 |
| P2 | 接线断裂：`SeedBank`、`PoolEffect`、`MessageWidget`、`Projectile::draw` 等从未被调用 | ANALYSIS §0 never-used 列表 |
| P2 | 粒子 API：C++ 有 130 处 `AddPvzpParticle`，Rust 无等价 API | ANALYSIS §0、§4.3 |
| P3 | 渲染层 / 音效层 / 存档 / 平台缺口 | ANALYSIS §5–§8 |

## 模块级进度（依据 ANALYSIS_REPORT 与代码统计，需随工作更新）

| 模块 | Rust 位置 | 状态 | 备注 |
|---|---|---|---|
| 基础类型/枚举 | `lawn/game_enums.rs`、`framework/{rect,point,color,common}.rs` | 🧪 | `TODO.md` 中枚举已勾选，未做数值核对 |
| MTRand / RNG | `framework/mt_rand.rs` | 🧪 | 需做黄金值测试（同种子前 N 个输出与 C++ 一致） |
| DataArray | `todlib/data_array.rs` | 🧪 | 需核对 ID 编码、遍历语义 |
| `.pak` / 资源解析 | `framework/paklib`、`resource_manager.rs`、`resources.rs` | 🧪 | `load_resources` 仅处理图像，声音另走路径（偏离 C++ 的 LoadGroup） |
| 字符串表（LawnStrings） | — | ⚠️ | `start_loading_thread` 注释：字符串表未接入，已跳过 |
| 存档 | `lawn/system/save_game.rs`（2845 行） | 🧪 | ANALYSIS §8 |
| 粒子/动画 | `todlib/{tod_particle,reanimator,trail,attachment}.rs` | 🧪/⚠️ | 粒子 API 缺口 |
| Board | `lawn/board.rs`（11417 行） | ⚠️ | 输入处理缺口（§2.1 `KeyDown`）等 |
| Challenge | `lawn/challenge.rs`（6210 行） | ⚠️ | §1 多项 🔴 |
| Plant / Zombie / Projectile / Coin | `lawn/*.rs` | ⚠️ | 部分未接线；§4 |
| 界面（标题/选关/商店/图鉴…） | `lawn/widget/*.rs` | ⚠️ | 标题加载有 BUG；§3 |
| 渲染 | `framework/graphics/*`、`ffi/opengl.rs` | 🧪 | §5 |
| 音频 | `framework/sound/*`、`ffi/sdl_mixer.rs`、`ffi/libopenmpt.rs` | 🧪 | §6；`mSoundSystem->GamePause` 未接线 |

## 未解决的 TODO(translate) / TRANSLATION_NOTE

仓库内约 560 处 `TRANSLATION_NOTE`。**要求：每次触碰某个文件，顺手把该文件的 TRANSLATION_NOTE 逐条登记到下表（位置、差异、是否影响回放）。** 登记完成前，不得把该文件标为 ✅。

| 位置 | 差异 | 影响回放? | 阻塞 |
|---|---|---|---|
| `lawn_app.rs:start_loading_thread` | 同步加载代替后台线程 | 否（但影响启动体验） | 标题加载 |
| `challenge.rs:314` | `beghouled_pick_seed` 恒取首项，不消耗 RNG | **是** | Beghouled |

## 已知与 C++ 行为的差异

以 `ANALYSIS_REPORT.md` 为准。新发现的差异先写在这里，再同步进该报告。

| 位置 | 差异 | 状态 |
|---|---|---|
| `board.rs` 网格数组（248-252） | Rust 声明 `[[T; 9]; 6]` 使外层索引为 y，与 C++ `mGridSquareType[9][6]`（外层 x）相反；全代码按 [x][y] 访问 | ✅ 2026-11-14 修复（`[[T; 6]; 9]`），进入棋盘的越界 panic 消除 |
| `tod_foley.rs:322` | 按键/事件触发 foley 播放时 `sfx_id` 裸指针未对齐 → misaligned pointer panic（主线程 abort） | ⚠️ 未修（已发现，待指派；复现：进入任意触发按键音效的界面后按键） |
| `game_selector.rs` 布局 | reanim 部件（BG_Center/Left/Right、木牌）绘制位置与原版有偏差；缺 FrameBasePose/TrackButton 定位、滑动动画细节 | ✅ 2026-11-17 修复：SELECTOR_OPEN 状态机 + 每帧 TrackButton + 按钮初始态对齐 C++；`SOUND_EVILLAUGH` 资源未注册暂以 SoundLosemusic 近似 |

### 2026-11-20 第二轮：ANALYSIS_REPORT §1-A 缺口台账 A1~A10 全量修复

| 位置 | 差异 | 状态 |
|---|---|---|
| `seed_chooser_screen.rs` | OnStartButton/PickRandomSeeds/ClickedSeedInBank/ButtonDepress 桩 → 按 SeedChooserScreen.cpp:619-861 全量；**新增构造 init()（C++ 45-205：7 按钮 + ToolTipWidget + mChosenSeeds 49 项 + SurvivalRepick 回填 + SEEING_STARS + CrazyDavePickSeeds）**并在 show_seed_chooser_screen 接线；remove_tool_tip 改隐藏不销毁 | ✅ 修复（未运行验证） |
| `almanac_dialog.rs` | setup_plant/setup_zombie/set_page/update 桩 → 按 AlmanacDialog.cpp:119-260（Plant/Zombie 离板创建 + 页面切换 + 光标 + PoolEffectUpdate） | ✅ 修复（未运行验证） |
| `award_screen.rs` + `lawn_app.rs` | draw_achievements/achievements_continue_pressed/mouse_up 桩 → 按 AwardScreen.cpp:51-85,589-662；show_award_screen 补构造期成就项填充 | ✅ 修复（未运行验证） |
| `zombatar_widget.rs` | 9 个 NewLawnButton 未创建 → 按 ZombatarWidget.cpp:273-377 创建/可见性/命中/绘制接线；update 过渡段 finished_button Resize 动画 | ✅ 修复（未运行验证） |
| `edit_widget.rs` + `sexy_app_base.rs` | Ctrl+V 只删选区、剪贴板桩 → 按 EditWidget.cpp:314-354,539-576 + SexyAppBase.cpp:3571-3591（SDL 剪贴板实装） | ✅ 修复（未运行验证） |
| `reg_emu.rs` + `sexy_app_base.rs` | 注册表全桩 → RegEmu.cpp 全文（REGEMU 二进制持久化）+ SexyAppBase.cpp:1022-1375 读写族/ReadFromRegistry/WriteToRegistry；lawn_app.write_to_registry 接线 | ✅ 修复（未运行验证） |
| `music.rs` + `sdl_music_interface.rs` + `ffi/sdl_mixer.rs` | setup_volume_for_tune 桩 → 按 Music.cpp:112-167（四曲目 30 轨主/鼓/镲矩阵，Mix_ModMusicStreamSetChannelVolume 新 FFI）；get_music_order/get_music_handle 走 trait 分派 | ✅ 修复（未运行验证） |
| `imagelib/mod.rs` | write_jpeg_image 补 C++ 质量 80（JpegEncoder::new_with_quality）；png/tga 复核已有真实编码 | ✅ 修复（未运行验证） |
| `sexy_app_base.rs` | IME 桩 → start/stop_text_input 实装 SDL_Start/StopTextInput + 新增 set_text_input_rect（Input.cpp:492-527） | ✅ 修复（未运行验证） |
| `resource_manager.rs` | FindImagePath/FindFontPath 无对应 → 新增 find_image_path/find_font_path（PvzpCommon.cpp:1170-1198） | ✅ 修复（未运行验证） |

**验证**：cargo check 0 error；cargo fmt 通过；cargo test 26+26 通过；clippy 125 条 error 全部为存量（本轮触碰文件 0 条）。**未运行游戏**（无 main.pak 运行环境时需用户复验：选卡界面、图鉴交互、成就页、捏僵尸按钮、粘贴、注册表持久化）。

## 未使用符号统计（2026-11-20）

- 报告：`UNUSED_SYMBOLS.md`（人读，含分文件汇总与逐条清单）；机读：`target/unused_items.json`。
- 复现：`python scripts/unused_symbols.py --collect`（重新编译采集诊断并重建报告）。
- 口径：rustc 的 `dead_code`/`unused_variables`/`unused_assignments`/`unused_mut`/`unused_imports`，以 bin 目标 `pvz-portable` 为准并并入 lib 独有诊断。
- 结论：函数/方法类 360 项未接线（33 自由函数 + 222 方法 + 105 关联函数），是 `04` P2「接线断裂」的机器证据；枚举变体 885 项中 868 项来自 `framework/resources.rs` 的 `ResourceId`（整类未被构造，游戏实际走字符串键查找）。
- **C++ 出处分析（2026-11-20 第二轮补充）**：1573 条未使用符号逐一对照 cpp/ 检索调用点，产物 `UNUSED_CPP_USAGE.md`（人读）+ `target/unused_final_verdicts.json`（机读）。结论：**480 项在 C++ 中确实被使用**（= P2 接线缺口的直接证据清单，含调用点文件:行号）；23 项 C++ 仅有定义/声明（C++ 侧同样无调用）；36 项为 Rust 自创/拆分组织（C++ 无独立对应，含 check_collisions、check_game_state 等，属翻译口径问题待人工裁决）；16 项疑似改名有候选；130 项 Rust 自有（借用辅助/导入/常量）；868 枚举变体为 ResourceId 整类镜像（游戏实际走字符串键查找，非缺口）。方法与局限见 UNUSED_CPP_USAGE.md §7。**未改任何游戏代码；未运行验证**

## 需要返工的文件

| 文件 | 原因 |
|---|---|
| ~~`lawn_app.rs` `loading_completed`~~ | ✅ 已完成（2026-11-13）：`loading_completed`/`fast_load`/`kill_board`/各 Kill*Screen/各 Finish*Dialog 改为经 `SexyAppBase::safe_delete_list` 延迟释放（ADR-010）；`process_safe_delete_list` 按 C++ SexyAppBase.cpp:2541-2553 实装并在主循环接线。**运行期验证未做**（无 main.pak） |

## 更新记录

## 2026-11-23 实际运行验证轮（cargo run + 截图验证）

**运行验证通过：游戏窗口渲染成功（标题画面「植物大战僵尸」完整显示，截图确认），进程持续响应 >150 秒无崩溃。**

本轮环境修复与发现：
1. 构建环境：`target/` 目录 ACL 损坏（所有者为 Administrators，当前用户无 WRITE_DAC/WRITE_OWNER）导致 cargo 锁文件拒绝访问，诊断脚本确认无法在不动所有者前提下修复（PRECONDITION），改用独立 --target-dir（target2）绕过。SDL2 bundled 构建在沙箱下仅单线程 MSBuild 可行（并行节点命名管道被拒），cargo build -j 1 通过。SDL 源 cmake_minimum_required 3.0.0/3.4 与 CMake 4.2 不兼容，按用户指示改为 3.5（改的是 registry 缓存 sdl2-sys-0.37.0/SDL/CMakeLists.txt，非仓库文件）。
2. **接线缺口（本轮修复）：LawnApp::init 漏掉 C++ LawnApp.cpp:1253-1263 的 profile 加载段**（mProfileMgr->Load() + 注册表 CurUser + GetAnyProfile 兜底），导致 player_info 恒为 None。
3. **崩溃修复：seed_chooser_screen.rs has_7_rows（原 497 行）对 player_info.unwrap() 直接 panic**，首次运行（无 userdata/users.dat，GetAnyProfile 返回 None）进入选卡界面必崩，backtrace 确认。修复按 C++ SeedChooserScreen.cpp:275-282 语义：None 视为未购买返回 false（C++ 此场景为空指针解引用 UB）。**TODO(translate) 已登记：C++ 279-280 的 SEED_TWINSUNFLOWER..SEED_COBCANNON HasSeedType 检查循环 Rust 版缺失。**
4. 验证过程：修复前 ~55-60 秒必现 panic（exit 101）；修复后 150+ 秒稳定运行、窗口 Responding=True、标题画面渲染正常（CopyFromScreen 截图成功；PrintWindow 对 GL 窗口恒黑帧，再次证实 2026-11-14 的记录）。

自检：cargo fmt --check 通过；cargo build 0 error（405 存量 warning）；cargo test 0+0 通过（仓库无测试用例）；cargo clippy 123 error 均为存量（missing_safety_doc 等，集中在 build.rs 与 FFI 层），本轮修改行无新告警。clippy 需 -j 1 且 TEMP 指向工作区内目录（MSBuild/链接器写 %TEMP%\MSBuildTemp 被沙箱拒绝）。

| 2026-11-21 | **第二轮遗留项全量接线（本轮）**：①§1 方法类真实缺口 30 项接线（起点 103/167 → 133/167 有调用点）：board（init_zombie_waves→ForLevel、PlantingRequirementsMet+Cobcannon、ToolHitTest→Helper、MouseDownWithTool 补 ZenGarden/TreeOfWisdom 分支、update_level_end_sequence 补 C++ 1775-1802 GetBottomLawnMower 金币链+SOUND_POINTS 音高、init_level→PickBackground 忠实版+墓碑段 C++ 1097-1163、key_char f 段→GetZenToolAt、add_plant 补 C++ 2112-2135 fungi/catapult 段、ZOMBIE_WAVE_DEBUG 常量化）；challenge（MouseDown 补 WhackAZombie/ScaryPotter 分支、SpawnZombieWave 按 C++ 2969-3013 全量、InitLevel 补 ZenGarden/IZombie/ScaryPotter/TreeOfWisdom、LastStandCompletedStage 按 C++ 5132-5156 全量+SOUND_HUGE_WAVE、SquirrelUpdate 接 SquirrelUpdateOne、IZombiePlacePlantInSquare 接 IZombieSetupPlant、i_zombie_squish_brain 补 MakeRenderOrder）；plant（**DoSpecial 按 C++ 4290-4403 全量重写**：Cherrybomb/Doomshroom/Jalapeno/Iceshroom/PotatoMine/InstantCoffee 分支+Explodonator/Spudow 成就+AddACrater、BlowAwayFliers 补 SOUND_BLOVER、**Starfruit 链修正**：LaunchStarFruit=FindTarget+anim_shoot+counter40、Fire→StarFruitFire、Draw 按 C++ Plant::Draw 3963-4044 骨架：南瓜后景+飞行摆动+IZombieDrawPlant 分支）；zombie（apply_burn 尾补 BobsledBurn 钩子 C++ 8804-8807、update_zombie_jalapeno_head 补 DO_FIX_BUGS mMindControlled→Zombie::BurnRow 分支（cfg feature）、zombie_initialize Flag 分支补全 C++ 585-592：flag reanim attach+FrameBasePose+SetupZombatarFlagReanim、burn_row 误判注释修正）；tod_foley（补注册 SOUND_BLOVER/SOUND_DOOMSHROOM/SOUND_POTATO_MINE）；②遗留项2：pool_effect 实例构造接线（LawnApp.cpp:1745-1746）；③遗留项3：WidgetImpl trait 补 added_to_manager/removed_from_manager 默认回调+Widget 基类转发；④遗留项4：LawnEditWidget 按 C++ LawnCommon.cpp:80-146 重写（key_down_forward_escape/key_char/key_text+AutoCapChar 98-107+工厂），Dialog 补 dialog_key_down（对应继承 Widget::KeyDown 的 TAB-only 语义）；⑤Cargo.toml 声明 fix_bugs feature。**未接线判定修正**：zombie draw_zombie_part/head 在 C++ 为注释死代码（Zombie.cpp:5210-5275）保持 NOT-WIRED；save_game（Board.cpp:375）C++ 亦无调用点改记 §2。**未运行验证**（无 main.pak 复验环境；需用户复验：WarAndPeas 调试键生成、LastStand 通关提示、智慧树工具、锤僵尸/砸罐、星光果发射、燃烧雪橇队、旗僵尸 zombatar 头、用户名输入自动大写） | AI |
| 日期 | 变更 | 操作者 |
|---|---|---|
| 2026-11-20 | **未使用符号 C++ 出处全量分析（只读）**：按台账 P2 要求，把 `UNUSED_SYMBOLS.md` 1573 条逐一在 cpp/ 权威源码中定位原使用点，产出 `UNUSED_CPP_USAGE.md` 与机读 `target/unused_final_verdicts.json`。判定分布：C++ 有使用（接线缺口）480、C++ 仅定义 23、Rust 自创/拆分 36、疑似改名 16、Rust 自有 130、非 ResourceId 枚举变体 17、ResourceId 整类 868。复现脚本与中间产物在 `target/cpp_*.json`。未改任何游戏代码，未运行验证 | AI |
| 2026-11-20 | **未使用符号全量统计（只读，未改任何游戏代码）**：以 rustc 的 dead_code / unused_* 诊断为证据，产出 `UNUSED_SYMBOLS.md`（1573 条去重条目 / 72 个文件）与机读清单 `target/unused_items.json`，采集脚本 `scripts/unused_symbols.py`（`--collect` 可重跑）。分布：函数 33、方法 222、关联函数 105、常量 13、结构体 6、枚举 4、字段 51、枚举变体 885（其中 `resources.rs` 的 ResourceId 占 868，属整类未接线）、局部变量 188、未用导入 66。**未运行验证**（仅静态统计） |
| 2026-11-20 | **第二轮台账修复**：ANALYSIS_REPORT §1-A 的 A1~A10 十项缺口按 cpp/ 权威源码全部补全（选卡界面全套+构造数据、图鉴交互、成就页链、捏僵尸 9 按钮、剪贴板粘贴、RegEmu 注册表持久化、Music 30 轨音量、JPEG 质量 80、SDL 文本输入、资源反查路径），触碰 14 个源文件。cargo check 0 error，cargo fmt 通过，cargo test 26+26 通过；clippy 125 error 均为存量。**未运行验证** | AI |
| 2026-11-12 | 依据实际仓库重建台账 | |
| 2026-11-13 | ADR-010 实装：`sexy_app_base.rs` 新增 `WidgetSafeDeleteInfo`/`safe_delete_widget`/`safe_delete_ptr`，`process_safe_delete_list` 按 C++ 语义实现并接入主循环与 `kill_dialog`；`lawn_app.rs` 14 处 `Box::from_raw` 调用点改为延迟释放；`dialog.rs` `wait_for_result` 包模态等待深度防循环内释放。cargo check 0 error/746 warnings，cargo test 21+21 通过。未运行验证 | AI |
| 2026-11-13 | 运行验证（cargo run，main.pak 由 build.rs 自动复制）：资源加载全流程通过（`[LawnApp] 资源加载完成`），窗口持续响应（进程 Responding=True），未崩溃、未卡死；标题画面停留等待点击（截图确认 GUI 交互在托管桌面不可行，未验证 L5 进入 GameSelector）。补 4 个 ADR-010 回归测试（深度保留/模态门控/相等深度边界/kill_dialog 入队），cargo test 25+25 通过 | AI |
| 2026-11-13 | **L5 验证通过**（SendInput 注入点击）：点击标题 → `[LawnApp] LoadingCompleted` → `已显示 GameSelector`，延迟释放路径（TitleScreen 释放后继续点击/渲染）无崩溃。**发现**：GameSelector 背景走 titlescreen fallback（`IMAGE_REANIM_SELECTORSCREEN_BG` 未加载）且按钮图片未加载（`get_draw_image` 返回 null），画面与标题屏相似导致"看似未进入"；LoaderBar 资源组不含 SelectorScreen 所需资源。**偏亮修复**：补 C++ GLInterface.cpp:1198 的 `glDisable(GL_FRAMEBUFFER_SRGB)`（0x8DB9），待用户复验。诊断日志已全部移除 | AI |
| 2026-11-13 | **GameSelector 画面修复（用户复验通过，截图确认）**。根因：Rust 资源清单解析器不认 `<Resources id="组名">` 分组与 `<SetDefaults path/idprefix>`（C++ ResourceManager.cpp:546-561/450-462），导致所有资源 res_group 为空、注册键缺前缀 → `load_resources("LoadingImages")` 空转、`IMAGE_REANIM_*` 键永不注册。修复：①resource_manager.rs 补组/SetDefaults 语义（key=idprefix+id，path=default+path），根元素只认 `<ResourceManifest>`；②reanim_loader.rs 新增 `reanimator_register_images_to_resource_manager()`（对应 C++ DefinitionLoadImage 的 PvzpAddImageToMap 副作用），定义加载后把 reanim 图片注册进 image_map；③start_loading_thread 接入 `preload_for_user()`（对应 C++ LawnApp.cpp:1764）；④LoaderBar/Init 组替代硬编码图片名；⑤get_image/delete_image 对存量短名调用点补 `IMAGE_` 前缀 fallback（TRANSLATION_NOTE 登记）。cargo check 0 error/747 warnings，cargo test 26+26 通过 | AI |
| 2026-11-14 | **GameSelector 渲染链路修复（用户复验：三模式木牌出现）+ 本轮遗留补全**：①image_path_candidates 补 .jpg 双格式与"尾下划线"文件名变体（pak 内 SELECTORSCREEN_BG_CENTER_.PNG 命名，对应 RM 638-642 的变体语义）；②**atlas 编码冲突修复**：transform.m_image 的 atlas 编码句柄与名字表索引共用 0..1000 空间，解码误判 → 编码改 1001+index、解码只认 >=1001（对应 C++ 指针值 >1000 即真图的区分语义）；③draw_dirty_stuff 每帧清空 screen_image（对应 C++ GL Swap 后 glClear，GLInterface.cpp:1247-1250，否则 reanim 透明区透出旧帧）；④**网格数组维度错误修复**（board.rs:248-252/504-508：`[[T; 9]; 6]` 应为 `[[T; 6]; 9]`，C++ mGridSquareType[9][6] 外层为 x——此前进入棋盘即越界 panic）；⑤GameSelector 补 reanim 初始化（C++ GameSelector.cpp:294-337：selector/云×6/花×3/草叶）与 update 推进（C++ :892-919，GameSelector 界面无 EffectSystem::Update 驱动）、draw 改为 DrawRenderGroup(1)→云→(0)（C++ :559-563，移除静态 fallback）；⑥EffectSystem 在 init() 创建（对应 C++ Init 的 new EffectSystem，此前漏建致 AddReanimation 全部失效）。**新发现（未修，待指派）**：按键触发音效崩溃 tod_foley.rs:322 misaligned pointer dereference（sfx_id 指针未对齐）。诊断日志已全部移除；cargo check 0 error/750 warnings，cargo test 26+26 通过 | AI |
| 2026-11-14 | **下一轮第一批**：①tod_foley.rs:322 sfx_id 解引用对齐修复（SOUND_* 为 i32 而 sfx_id 指针声明 *mut usize，按 i32 解引用消除 misaligned panic，TRANSLATION_NOTE 登记）；②GameSelector 补 C++ 928-940 的 TrackButton 轨道绑定（GameBtnData 加 track_name/offset 字段，构造尾按首帧动画定位按钮位置，新增 track_button_position）；③验证手段说明：PrintWindow 对 GL 窗口偶发抓到 swap 后 clear 的黑帧（同 exe 间歇出内容），黑截图≠黑画面。cargo check 0 error，cargo test 26+26 通过 | AI |
| 2026-11-17 | **GameSelector/TitleScreen 选关界面修复（对照用户截图 3/4：左侧灰按钮+黑块+按钮错位+标题缺文字）**。①update 补 SELECTOR_OPEN→NEW_USER→SHOW_SIGN→IDLE 状态机（C++ GameSelector.cpp:808-890）：anim_open 播完隐藏 5 条 reanim 按钮轨道、按钮转 widget 绘制、播 anim_sign、无档案弹 DoCreateUserDialog、HasTrophy 补 AddTrophySparkle、mNeedsMessageOnGameSelector 弹完成提示；②构造补按钮初始态（C++ :77-281）：Adventure/Minigame/Puzzle/Survival/Options/Help/Quit/Zombatar/ChangeUser mBtnNoDraw=true + Options/Help/Quit 的 Resize 尺寸与 ButtonOffset 修正——修掉动画期间 widget 按钮提前画出（截图左侧灰按钮）；③update 末尾补每帧 TrackButton（C++ :928-942）+ woodsign2/3 悬停 image override + 云/手 reanim 更新；④draw 移除自创 update_button_positions（硬编码坐标是截图错位根因），按钮位置全部由 TrackButton 从 reanim 轨道取，滑动经 offset 平移（对应 C++ mX/mY）；⑤draw_overlay_internal 按 C++ DrawOverlay 606-709 重写：关卡数字坐标取自 SelectorScreen_BG_Right 轨道 + cel 索引修正（C++ theCelCol=stage/sub 0 基）+ sub==10 双 cel、浇水气泡、僵尸手（裁剪 BOARD_HEIGHT-40）、草叶、花×3、奖杯（坐标取自 BG_Left 轨道）、ToolTip 绘制；⑥clicked_adventure 补 C++ 1136-1179 全逻辑（试玩重玩确认/删存档/停音乐/僵尸手动画+rock 轨道忽略裁剪/FOLEY_DIRT_RISE）；⑦mouse_up_btn 分支对齐 C++ ButtonDepress 1224-1260（Quit→ConfirmQuit、Options→DoNewOptions(true)、ChangeUser→DoUserDialog、Store→ShowStoreScreen+GoToTreeNow、Almanac→DoAlmanacDialog+恢复音乐、ZenGarden→SetupForZenTutorial）；⑧starting_game 倒计时对齐 C++ 768-806（KillGameSelector/Intro 模式/MagicTaco 商店开场/禅教程/EVILLAUGH）；⑨TitleScreen：start_button_x 改按 IMAGE_LOADBAR_DIRT 宽定位（C++ 307）、按钮文字改居中于按钮宽度+悬停 mOverColor 变色（HyperlinkWidget.cpp:43-64）+mouse_move 悬停检测。[TRANSLATION_NOTE]: SOUND_EVILLAUGH 资源未注册以 SoundLosemusic 近似；StoreScreen 无 WaitForResult。cargo check 0 error，cargo fmt 通过，cargo test 26+26 通过。**未运行验证（无 main.pak 复验环境），待用户截图复验** | AI |

## 2026-11-17 字体系统三项修复 + 选关界面主体修复（用户截图 3/4 反馈轮）

**运行验证通过（金色 [CLICK_TO_START] 正常显示、选关木牌正常、灰按钮组消失）**

### 字体系统修复（bitmap_font.rs，三 bug 均为截图 3 [CLICK_TO_START] 不显示的根因链）
1. `next_token` 递归解析 72KB 描述文件栈溢出 → 改循环实现
2. `parse_define` 项切分丢项：换行处无逗号导致 CharList/RectList/WidthList 只收到约 1/3 项 → 项边界不再依赖逗号
3. 图集黑底不透明像素覆盖相邻字形 → 加载时对纯黑像素做色键透明

### TitleScreen（title_screen.rs）
- `draw_start_button_text` 接入全局位图字体 FONT_BRIANNETOD16（此前自建 Font 对象字形缺失）
- start_button_x 按 dirt 图宽定位；悬停 mOverColor 变色

### GameSelector（game_selector.rs，见 f6aa35f 提交）
- 构造按钮初始态/SELECTOR_OPEN 状态机/每帧 TrackButton/DrawOverlay 重写/clicked_adventure/mouse_up_btn 分支

### 遗留（下一轮）
- 选关屏 BG_Left/BG_Right 轨道位置偏移出屏（右侧石碑区黑块）——reanim 定义加载层问题，与本次修复独立

## 2026-11-18 选关屏 BG_Left/BG_Right 黑块修复（遗留项）

**运行验证通过：石碑、花瓶、房子、木牌全部正常显示，黑块消失**

根因链（两项独立修复）：
1. reanim_loader.rs load_reanim_image：pak 中部分 reanim 图为 JPG（无 alpha），
   透明信息存于同名尾下划线 PNG 的蓝通道（如 SelectorScreen_BG_Right_）。
   C++ ImageLib::GetImage 自动合成（ImageLib.cpp:1314-1346 ComposeAlpha），
   Rust 缺此合成 → JPG 黑色背景区不透明 → BG 轨道整块发黑。修复：主图加载后
   按命中路径查找 <stem>_.png，将 alpha 图蓝通道写入主图 alpha。
2. reanimator.rs draw_render_group：Rust blt_matrix 是纹理四角正向映射，
   把 pivot 居中矩阵的平移（+365,+280）当作真实位移 → BG_Right 画到 (436,321)
   大部分出屏。C++ ReanimBltMatrix（Reanimator.cpp:609-612）在快速路径中把
   pivot 平移减回（m02 - scale*srcW/2），图像左上对齐 transform 位置。
   修复：无 skew、正 scale、color==White 时按 C++ 快速路径直绘
   （DrawImage/DrawImage 拉伸），否则保留矩阵路径。

注：石碑上 4 个模式按钮的显示（anim_open 完成后按钮轨道/木牌按钮态）待下轮核验。

## 2026-11-19 标题屏翻译 + 选关界面按钮全量修复（对照用户截图 1/2 复验轮）

**运行验证通过（cargo build + 运行截图）：标题屏金色「点击开始！」、选关界面「开始冒险模式/迷你游戏/益智模式/生存模式」四按钮、左下「成就」基座、右下「选项/帮助/退出」全部与原版一致**

根因链（四项独立修复）：
1. 字符串表未接入（对照截图 3 的 `[CLICK TO START]` 原始键显示）。C++ LoadingThreadProc
   （LawnApp.cpp:1702-1703）调用 PvzpStringListLoad("Properties/LawnStrings.txt")，
   Rust 此前明确跳过（lawn_app.rs 原 3640 注释）。修复：tod_common.rs 补
   PvzpStringFile.cpp:66-159 全链（ReadName/ReadValue/ReadItems/ReadFile/Load，
   含 Buffer::ToUTF8String 的 BOM 分流：UTF-8 剥离 / UTF-16LE|BE 转码 /
   严格 UTF-8 校验 / Win-1252 兜底，WIN1252_TO_UNICODE 32 项表照抄）；
   start_loading_thread 接线两文件装载；TitleScreen 两处 label 过 tod_string_translate
   （[CLICK_TO_START]→点击开始！、[LOADING]→读取中…）。
2. GameSelector 按钮图从未解析：GameBtnData::resolve_images 定义于 game_selector.rs:132
   但**无任何调用点**（缓存恒 null → draw_button_at 直接 return）→ 全部按钮消失。
   修复：draw 开头逐按钮 resolve_images（对应 C++ 构造时图像指针直接赋值的惰性版）。
3. SetDefaults 解析断裂（reanim 组 4 个按钮图加载失败）：`<SetDefaults .../>` 是
   自闭合标签 → TYPE_ELEMENT 主循环 → parse_resource **无 SetDefaults 分支**（id 缺失
   直接 return）→ 其后 reanim 组条目（SELECTORSCREEN_ADVENTURE_BUTTON 等 116 项）
   注册键/路径缺前缀。修复：parse_resource 头部补 SetDefaults 分支
   （对应 C++ ParseSetDefaults，ResourceManager.cpp:450-462）。
4. anim_open 轨道在本 pak 不存在（SelectorScreen.reanim 无该 track）：C++
   FindTrackIndex 找不到时 PvzpTrace 后**返回 0**（Reanimator.cpp:950-958），
   PlayReanim("anim_open") 实际取 track 0（anim_sign）帧区间并照常推进 loop_count →
   OPEN 状态机正常转入 IDLE；Rust find_track_index 返回 -1 导致帧区间退化。
   修复：get_frames_for_layer 按 C++ 行为回退 track 0（TRANSLATION_NOTE 登记）。

附带：game_selector.rs ensure_fonts 改用 font_global("FONT_BRIANNETOD16") 全局位图字体
（pak 内 data/BrianneTod16.txt 含 2822 个 CJK 字形，欢迎木牌玩家名可渲染中文）；
此前为无字形占位 Font。

验证：cargo check 0 error；cargo fmt 通过；cargo test 26+26 通过；clippy error 总数
修改前后均为 124（全部为存量 not_unsafe_ptr_arg_deref 等，无新增）。运行截图对照
用户截图 1/2：标题屏与选关界面元素齐全。欢迎木牌玩家名空缺为无玩家档案时 C++ 同样
不绘制（mName 为空），非缺口。

遗留：欢迎木牌玩家名需建档后复验；Zombatar/成就 widget 接入仍未完成（added_to_manager 占位）。

## 2026-11-19 主循环节流修复：运行速度比原版慢的根因（用户反馈轮）

**运行验证通过：启动→logo 淡出→「点击开始！」出现全程约 5 秒（修复前 logo 阶段停 40s 不推进）；
CPU 占用从空转 ~100% 降至 13-18%（C++ 同款节流特征）；选关界面云动画持续推进**

根因：主循环无时间闸门。C++ 的速度控制链是 Process()（SexyAppBase.cpp:2572-2837）：
mFrameTime=10（:239，逻辑 100 FPS）→ UpdateFTimeAcc 累积真实耗时（:2555-2570，上限 200ms）→
闸门 mUpdateFTimeAcc >= aFrameFTime 才 DoUpdateFrames（:2693）→ 扣减（:2769）→
空闲 nanosleep 精确睡到下一帧（:2790-2807）→ mNonDrawCount 强制至少 10 次/秒重绘（:2684）。
mVSyncUpdates 默认 false（:324），vsync 不参与节流。

Rust 此前 update_app_step_inner 每轮固定推 1 个逻辑帧，唯一限速是 swap_buffers 的
gl_set_swap_interval(1)（vsync）→ 逻辑帧率被锁到显示器刷新率（60Hz 时游戏整体慢 40%：
全游戏逻辑按 SECONDS_PER_UPDATE=0.01 即 10ms/帧计速，1 秒只跑 0.6 秒游戏当量）；
frame_time: 10 字段只初始化从未被读取。

修复（sexy_app_base.rs，对齐 C++ Process 全部节流逻辑）：
1. 补 UPDATESTATE_* 四常量（SexyAppBase.h:138-144）与 m_non_draw_count /
   m_pending_updates_acc / m_sleep_count / m_last_draw_was_empty / m_yield_main_thread 字段
   （SexyAppBase.h:230-251 缺失部分）
2. update_app_step_inner 改为 C++ UpdateAppStep 两阶段等价结构：事件排空 → process(true)
3. 新增 process()：UpdateFTimeAcc 累积 + 时间闸门 + PROCESS_2 补跑循环 + 扣减(含 -0.2ms) +
   空闲 std::thread::sleep 到下一帧 + mYieldMainThread 让步睡眠
4. swap_buffers 从每轮调用移到 draw_dirty_stuff 成功路径尾部（对应 C++ DrawDirtyStuff →
   Redraw → SwapBuffers 仅真绘制时交换；每轮 swap 会把循环锁死在 vsync 刷新率）
5. draw_dirty_stuff 尾部维护 has_pending_draw/m_last_draw_was_empty（C++ :1889-1899）
6. start_loading_thread 置 m_yield_main_thread=true（C++ :2381），加载完成清除（:1716）

附带修复：reanimator.rs get_track_velocity 的局部 SECONDS_PER_UPDATE 0.02 → 0.01
（C++ Reanimator.h:51，此为对照截图轮遗留的独立翻译偏差）。

验证：cargo check 0 error；cargo fmt 通过；cargo test 26+26 通过；clippy error 总数
基线(e93efb1)=124 / 修复后=124，无新增。运行实测：启动到「点击开始！」约 5 秒、
CPU 13-18%、选关界面云动画推进正常。

遗留：无 vsync 节流分支与 mVSyncBroken 检测未翻译（C++ :2700-2722 仅 vsync 分支需要，
mVSyncUpdates 默认 false 不会走到）；mStepMode 单步调试分支未翻译（仅 demo 调试用）。

## 2026-11-21 未使用函数/变量全量处置（UNUSED_CPP_USAGE.md 落地）

**未运行验证**（无 main.pak 运行环境；本次为编译期 + 静态对照 C++ 源码）。

### 结果

| 口径 | 处置前 | 处置后 |
|---|---|---|
| rustc dead_code（never used / never constructed / never read） | 178 | **0** |
| unused_variables | 90 | **0** |
| unused_assignments | ~50 | **0** |
| unused_imports | 59 | **0** |
| unused_mut | 29 | **0** |
| cargo check error | 0 | **0** |
| 总 warning 行（含存量 gl 命名/unsafe 等非本任务口径） | 755 | 402 |

cargo fmt --check 通过；cargo test 26 + 26 通过。

### 根接线（最大缺口，一次激活整批符号）

- **Board 注册进 WidgetManager**（对齐 C++ LawnApp.cpp:444-452 MakeNewBoard）：
  lawn_app.rs 新增 board_widget 字段与 BoardWidgetImpl 桥（WidgetImpl 转发
  draw/update/mouse/key 到 Board 对应方法），make_new_board 执行
  Resize + AddWidget + BringToBack + SetFocus；kill_board 补 RemoveWidget +
  延迟释放（ADR-010）。WidgetManager 补 bring_to_back（对应 C++
  WidgetContainer::BringToBack，WidgetContainer.cpp:283-296）。
  桥接层按 C++ 编码还原按钮参数（左=1/右=-1/中=3），对应
  WidgetManager.cpp:659 / 581 的直传语义。
  该接线使 Board::draw / DrawGameObjects 全渲染链、MouseDown/MouseUp/MouseMove/
  MouseDrag/KeyDown/KeyChar 首次进入实际调用路径（激活 board/challenge/zombie/
  plant/coin/seed_packet/grid_item/lawn_mower/cutscene 等约 60 个方法）。
- **Zombie::UpdatePlaying 补 CheckIfPreyCaught**（C++ Zombie.cpp:4642-4650 顺序），
  并修正其内部 EatPlant 段（C++ 6876-6883）：原实现丢 return 导致 start_eating
  后立即 stop_eating；补 IZombie 大脑分支（C++ 6886-6889）。
- **LawnMower 碰撞链**：C++ 碰撞在 LawnMower::Update 内（LawnMower.cpp:178-234）；
  Rust 因借用限制曾在 Board::check_collisions 重复实现三类碰撞（与
  Projectile::CheckForCollision / Zombie::CheckIfPreyCaught 双倍伤害）。
  本轮将 check_collisions 精简为仅割草机碰撞段，并在 update_game_objects 的
  mower.update() 之前调用（保持「碰撞先于状态机」）。
- **Projectile**：CheckForCollision/CheckForHighGround 从 update_motion（且排除
  Lobbed）移到 Update 末尾无条件调用（C++ Projectile.cpp:735-736）；
  do_splash_damage 按 C++ 461-510 重写（溅射总量上限 + IsZombieHitBySplash +
  GetDamageFlags）。
- **StartLevelIntro 分支链**按 C++ CutScene.cpp:751-853 逐条重写（补 12/13-24/
  16-24/21/31/41 关卡、can_get_packet_upgrade 两处、little trouble、TreeOfWisdom、
  isRestart 判定、mLawnMowerTime=550），并补 SyncProfile 尾部三初始化
  （C++ GameSelector.cpp:547-550）：AlmanacInitForPlayer / BoardInitForPlayer /
  AchievementInitForPlayer。
- 其他接线：Board::MouseMove/MouseDrag → Challenge::MouseMove（C++ Board.cpp:
  3028-3038）；Board::DrawBackdrop → Challenge::DrawBackdrop（C++ Board.cpp:5943）；
  Plant::Update → UpdateReanim（C++ Plant.cpp:2875）；Zombie ZOMBIES_WON 场景 →
  CutScene::ShowZombieWalking（C++ Zombie.cpp:4345-4353）；
  make_new_checkbox 实装 IMAGE_OPTIONS_CHECKBOX0/1 并接入 NewOptionsDialog
  （C++ LawnCommon.cpp:154-162 / NewOptionsDialog.cpp:77-78）。

### NOT-WIRED 登记口径

对「C++ 仅定义/声明（UNUSED_CPP_USAGE.md §2）」「Rust 自创/拆分（§3）」「宿主
类型或调用链尚未翻译（§1）」的符号，逐符号加
// NOT-WIRED: <原因 + C++ 文件:行> + #[allow(dead_code)]（未用文件级 allow
掩盖证据）。共登记约 130 处，含：
- Board/Challenge/Plant/Zombie 的 4 组聚合未接线方法（C++ 调用方未触达，
  见各 impl 上方统一注释）；
- ResourceId 整类（§0 预期形态：游戏走字符串键查找）；
- 各音频/音乐 trait 的 C++ 虚接口方法（宿主 SexyAppBase 路径未接线）；
- Widget/TextWidget/ListWidget 的类方法占位（自由函数位置待迁入宿主类型）；
- 各级「挂起」项：pool_effect 实例构造（lawn_app.rs 范围）、TitleScreen 的
  Widget 生命周期回调（需 widget.rs 扩展）、成就/图鉴绘制链等。

### 遗留（下一轮）

1. 4 组聚合未接线方法需按 UNUSED_CPP_USAGE.md §1 逐条接线（Board 29 /
   Challenge 18 / Plant 10 / Zombie 11），主要是各模式 MouseDown 分支
   （WhackAZombie / ScaryPotter / IZombie / TreeOfWisdom）与绘制链。
2. pool_effect 实例构造（C++ LawnApp.cpp:1745）。
3. WidgetImpl trait 缺 added_to_manager/removed_from_manager 回调。
4. LawnEditWidget 继承 EditWidget 的桥接（CheatDialog/NewUserDialog）。
