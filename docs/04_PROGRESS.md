# 04 · 翻译进度台账

> **AI 每完成一项必须更新本文件。** 缺口的详细证据在 `ANALYSIS_REPORT.md`（章节号下文用 §n 引用）；本文件只做**状态与优先级**。

状态：⬜ 未开始 · 🔄 进行中 · 🧪 已翻译待验证 · ✅ 已验证 · ⚠️ 骨架化/有已知偏差 · 🔌 已实现但未接线 · 🔁 需返工

## 当前阶段与焦点

- **阶段：** 引擎主体编译通过；L1–L6 部分验证通过（GameSelector 真实画面渲染成功——atlas 修复后 BG_Center/木牌/云全部可见，截图确认）。
- **当前焦点：** ①GameSelector 布局缺口（部件位置/FrameBasePose/滑动动画，画面元素位置与原版有偏差）；②新发现崩溃：按键触发 foley 播放时 tod_foley.rs:322 指针未对齐（详见更新记录）。**未验证项：游戏内玩法回放一致性。**

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

## 需要返工的文件

| 文件 | 原因 |
|---|---|
| ~~`lawn_app.rs` `loading_completed`~~ | ✅ 已完成（2026-11-13）：`loading_completed`/`fast_load`/`kill_board`/各 Kill*Screen/各 Finish*Dialog 改为经 `SexyAppBase::safe_delete_list` 延迟释放（ADR-010）；`process_safe_delete_list` 按 C++ SexyAppBase.cpp:2541-2553 实装并在主循环接线。**运行期验证未做**（无 main.pak） |

## 更新记录

| 日期 | 变更 | 操作者 |
|---|---|---|
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
