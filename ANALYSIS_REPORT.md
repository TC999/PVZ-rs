# PVZ-rs：C++ → Rust 翻译差异报告

> **基线**：`git HEAD bfa5d31`（工作区干净；无 main.pak 之外的未跟踪源码变更）
> **对照对象**：C++ `cpp/src/`（权威参考实现，排除 SDL-Mixer-X / glad / pxtone / timidity / fcaseopen 等第三方）↔ Rust `src/`
> **编译状态（本次实测）**：`cargo check --message-format short` → **0 error / 754 warnings**；全库 `todo!()/unimplemented!` **0 处**
> **局限**：全部结论为静态比对，**未运行游戏**；§1-B 的 7 项「疑似等价」需逐行复核后方可结案。
> **版本沿革**：本文件上一版基于 `2f8ee56`。此后 **44 个提交**（142 文件，+36,876/−9,163）对旧台账 §1~§8 做了批量修复；本次（2026-11-20）全量复核确认绝大部分为**真实修复**（§2，逐项抽查实现体，未发现假完成回归），旧台账已大面积过时，本版按当前真实状态覆写重组。

**第二轮（2026-11-20 同日）：§1-A 的 10 项缺口（A1~A10）已全部按 cpp/ 权威源码补全为行为等价实现**：修复明细见 §2 末「第二轮修复」表。编译验证 cargo check 0 error、cargo test 26+26 通过、cargo fmt 通过；clippy 125 条 error 全部为存量（unsafe_op_in_unsafe_fn 类，本轮触碰文件 0 条）。**未运行游戏**。

**检查方法（本次全量）**

1. 文件级映射：cpp 自有源码 **243 文件 / 122,600 行** ↔ src **135 文件 / 130,590 行**（口径：排除第三方；旧版 194,270/98,466 的口径含第三方头文件，行数不可直接对比）
2. 名字级差集：C++ 定义体方法 **3,006** vs Rust 全 fn **3,886** → 名字命中 **2,579（85.8%）**；未命中（体 ≥8 行）**177** 条 → 归一化模糊匹配分类：30 条疑似改名命中、147 条无名字命中（逐条判定见 §3）
3. 函数体规模悬殊候选（C++ 体 ≥15 行且 Rust 同名 ≤1/8）**31** 条，逐条读两端源码复核：多数为转发层误报或已修复，结论并入 §1/§2/§3
4. 编译验证：`cargo check` → 0 error / 754 warnings（本次未再以 never-used 告警作接线断裂证据）
5. 关键项抽查：挑战模式 18 个函数体量、Board::KeyDown、SoundInstance::play、sync_magnet_item_portable、draw_render_group、widget 层命中/绘制链等

---

## 0. 总体状况（量化）

| 指标 | 数值 |
|---|---|
| cpp 自有源码 / src 行数 | 122,600 / 130,590 |
| C++ 定义体方法 / Rust fn / 名字命中 | 3,006 / 3,886 / 2,579（85.8%） |
| 名字未命中（体 ≥8 行） | 177 → 30 疑似改名、147 无命中（判定见 §3） |
| 规模悬殊候选（复核前 / 复核后成立） | 31 / 见 §1-A |
| `cargo check` | **0 error**，754 warnings |
| `TRANSLATION_NOTE` | 586 处（密度 top：lawn_app 53、plant 40、board 35、zombie 32、challenge 31、cutscene 30） |
| `todo!()/unimplemented!` | **0** |

**缺口现状分三层**：①界面交互层占位（图鉴/成就/选种/捏僵尸，§1-A1~A4）②输入与环境集成（剪贴板/注册表/IME/资源查找，§1-A5~A10）③疑似等价待复核（§1-B）。

---

## 1. 现存缺口

### A. 行为级缺口（2026-11-20 第二轮：A1~A10 已按 cpp/ 权威源码补全为行为等价实现，修复证据见 §2 末「第二轮修复」；保留编号供追溯）

| # | 项 | C++ | Rust | 状态 |
|---|---|---|---|---|
| A1 | SeedChooserScreen::OnStartButton / PickRandomSeeds / ClickedSeedInBank | SeedChooserScreen.cpp:619 / 710 / 810 | seed_chooser_screen.rs | ✅ 已修复（§2-第二轮 #1） |
| A2 | AlmanacDialog 交互 | AlmanacDialog.cpp:149-216 | almanac_dialog.rs | ✅ 已修复（§2-第二轮 #2） |
| A3 | AwardScreen 成就链 | AwardScreen.cpp:606 / 633 | award_screen.rs | ✅ 已修复（§2-第二轮 #3） |
| A4 | ZombatarWidget 控件 | Widget/ZombatarWidget.cpp | zombatar_widget.rs | ✅ 已修复（§2-第二轮 #4） |
| A5 | EditWidget 剪贴板粘贴 | EditWidget.cpp:539 | edit_widget.rs | ✅ 已修复（§2-第二轮 #5） |
| A6 | 注册表仿真 | RegEmu.cpp + SexyAppBase.cpp:1022-1433 | reg_emu.rs | ✅ 已修复（§2-第二轮 #6） |
| A7 | Music 逐轨音量 | Music.cpp:112 | music.rs | ✅ 已修复（§2-第二轮 #7） |
| A8 | ImageLib 写图 | imagelib/ImageLib.cpp:759 等 | imagelib/mod.rs | ✅ 已修复（§2-第二轮 #8） |
| A9 | IME 文本输入 | platform/default/Input.cpp:492-527 | sexy_app_base.rs | ✅ 已修复（§2-第二轮 #9） |
| A10 | 资源反查路径 | PvzpCommon.cpp:1170/1185 | resource_manager.rs | ✅ 已修复（§2-第二轮 #10） |

**未运行验证声明**：以上修复仅通过 cargo check（0 error）/ cargo test（26+26 通过）/ cargo fmt 静态验证，**未运行游戏**；运行期交互（选卡、图鉴、成就页、捏僵尸、粘贴、注册表持久化、MO3 逐轨音量）需按 §5 复现方法人工验证。

### B. 疑似等价，待逐行复核（均为本次新发现）

| # | 项 | C++ | Rust | 待核内容 |
|---|---|---|---|---|
| B1 | `AdditiveDrawLine` | `MemoryImage.cpp:404`(177) | 疑似并入 `image.rs` `draw_line` 的 `draw_mode` 参数 | 加色线段像素语义是否逐行等价 |
| B2 | `Reanimation::AttacherSynchWalkSpeed` | `Reanimator.cpp:1362`(50) | 疑似内联于 `update_attacher_track`（`reanimator.rs:1612`，120 行） | 走速同步公式 |
| B3 | `Reanimation::GetTrackMatrix` | `Reanimator.cpp:837`(54) | 疑似由 `get_track_base_pose_matrix`(`:1422`)/`get_attachment_overlay_matrix`(`:1451`) 覆盖 | 调用点语义 |
| B4 | `PvzpTriangleGroup` 批处理渲染 | `EffectSystem.cpp:394`(135)：`AddTriangle` 收集 + `DrawGroup` flush | `render_particle`（`tod_particle.rs:1413`）即时绘制 | 任意矩阵变换下裁剪/采样路径差异（输出大体等价） |
| B5 | `ScrollbarWidget::ResizeScrollbar` | `ScrollbarWidget.cpp:84`(14) | 疑似内联于 `get_thumb_size/get_thumb_position`（`scrollbar_widget.rs:142/150`） | 外部 resize 触发链 |
| B6 | `GameButton::IsMouseOver` | `GameButton.cpp:219`(17) | 改为 `(*button).is_over` 字段直读（`board.rs:4609/8292/8570/10044`） | C++ 方法内额外 hit-test 逻辑是否丢失 |
| B7 | `XMLEncodeString` | `Common.cpp:504`(50) | `xml_encode_string`（`common.rs:601`，6 行仅替换 5 实体） | C++ 逐字符版对非 ASCII/控制字符的处理是否等价 |

### C. 维持桩，已有判定依据（不再视为缺口）

- **C1** `LoadAUSound`（`sdl_sound_manager.rs:367`）：兜底路径，是否需要待运行期「尝试加载」日志确证（提交 ee8908b）
- **C2** 旧格式存档（`SaveGameContext` 族）：C++ 为整块内存镜像（含 padding/裸地址），本质不可移植（提交 f831276）
- **C3** `ClearUpdateBacklog`：Rust 事件循环每轮一帧、不累积时间，无积压概念（提交 6816f04）

---

## 2. 本次核实为已修复（旧台账条目 → 现状）

| 旧条目 | 修复证据（本次抽查） |
|---|---|
| §1.1~1.20 挑战模式 20 项全部 | `challenge.rs`：`init_zombie_waves:1450`(71行)、`portal_start:3260`(52)、`update_rain:2737`(68)、`puzzle_next_stage_clear:4926`(86)、`tree_of_wisdom_init:7025`(91)、`last_stand_update:6434`(32)、`scary_potter_update:3994`(49)、`beghouled_score:2583`(110)、`beghouled_pick_seed:335`(41，经 `tod_pick_from_array`@`tod_common.rs:600` 已随机)、`beghouled_check_stuck_state:1426`(23)、`draw_storm_night:2283`(40)、`draw_storm_flash:2694`(42) 等；主修复提交 c1e37a4 |
| §2.1 Board::KeyDown | `board.rs:4799` 起全量：DoTypingCheck(:4812)、LEVEL_INTRO 转发(:4825)、RETURN/SPACE(:4835)、ESCAPE 分支（提交 6023bbe） |
| §3.7 MessageWidget | `get_font:106` / `layout_reanim_text:314` / `draw_reanimated_text:398` 均有实质实现 |
| §3.8/3.9 GameSelector/CreditScreen | 提交 f6aa35f、e93efb1、51e0235、76c7e38（标题屏/选关屏修复，IMAGE_* 接线范围扩大） |
| §3.10 资源 id 体系 | `resource_id_map/resource_ptrs` 已建立（f615ee2），almanac 等已按 key 取图；**残留**：zombatar 皮肤图等仍未接入（见 A4） |
| §4.1/4.2 GetDamageFlags / BobsledCrash | 提交 d67b466：盾牌穿越 4 标志 + 冻结、三追随者同步 |
| §4.3 粒子 API | `tod_particle.rs` `draw_particle:1357`/`render_particle:1413` 渲染路径已接（批处理差异见 B4）；AddPvzpParticle 等价入口已建 |
| §5.1~5.5 渲染 | 提交 9455971/d3ad338：BltRotated（`image.rs:855`，双线性+定点）、Slow/FastStretchBlt、Wu AA（`image.rs:465`）、Check3D、整型 rot 分支；旧 5.2 判定为 C++ 同构非缺口 |
| §5.6 SWTri | 判定死代码，等价光栅化在 `image.rs:1612-1800`（blt_triangles_*） |
| §6.1~6.4 音效 | 提交 f6cbc60/9d41520/53a0d33：`SoundInstance::play` 真调 `play_channel`（`sdl_sound_instance.rs:309`，含 PitchHandler 变调）、GetSoundInstance/ReleaseSounds 槽位管理、FadeIn/Out 渐变、`load_resources` 补逐组声音加载 |
| §7.5 PutBehind/PutInfront | `widget_container.rs:227/244` + `insert_widget_helper_from:188`（a72c252） |
| §7.13 WaitForResult | dialog.rs 模态嵌套主循环（6e6f9d7） |
| §7.14 Attachment::Draw | `draw_all_render_groups`（`reanimator.rs:246`）补 1..17 组（4363992）；OverrideColor/OverrideScale 早已完整 |
| §7.8 输入层 | 事件分发/SDLKeyToKeyCode/demo 录制/窗口创建已收口（f9546cc）；IME 残留见 A9 |
| §8.3 旧格式存档 | 不可移植判定（f831276，见 C2） |
| §8.7 MagnetItem | `item_type` 第 5 字段已补（`save_game.rs:2223-2236`，19a435c） |
| §8.8 def/override 占位 | reanim/particle/trail 定义指针↔下标反查表已接线（f615ee2/19a435c） |

---

### 第二轮修复（2026-11-20，A1~A10 → 逐项证据）

| # | 修复内容（Rust 侧） | 对齐 C++ |
|---|---|---|
| #1 A1 | seed_chooser_screen.rs：on_start_button 按 619-708 全量（SEEING_STARS/首周目 1-1/11 夜阳/水池/屋顶/ART_WALLNUT/ART_SUNFLOWER/飞行僵尸 8 组警告 + 8 组升级 CheckSeedUpgrade）；pick_random_seeds 按 710-730（Rand 循环选种入槽 + LandFlyingSeed + CloseSeedChooser）；clicked_seed_in_bank 按 810-840（后续种子左移动画 + 飞回选卡区 + seeds_in_bank-- + SOUND_TAP）；clicked_seed_in_chooser 按 842-861（满槽 return + 飞入卡槽 + EnableStartButton）；button_depress 按 732-774（ViewLawn 禁菜单/Almanac/Store 改标签+GoToTreeNow/Menu/Start/Random 带卡槽数门控）；remove_tool_tip 改隐藏不销毁；**新增构造 init()（45-205）**：7 按钮 + ToolTipWidget + mChosenSeeds 全 49 项填充 + SurvivalRepick 回填/清槽 + SEEING_STARS 星光果预置 + CrazyDavePickSeeds，并在 lawn_app.rs show_seed_chooser_screen 接线（此前构造数据整体缺失，PickRandomSeeds 的 do-while 会死循环） | SeedChooserScreen.cpp 全函数 + 构造函数 |
| #2 A2 | almanac_dialog.rs：clear_plants_and_zombies 补 Die/DieNoLoot + 释放；setup_plant 按 149-167（TALLNUT/COBCANNON/FLOWERPOT/INSTANT_COFFEE/GRAVEBUSTER 位置偏移 + Plant::new + plant_initialize，board=None、is_on_board=false）；setup_zombie 按 169-178（Zombie::new + zombie_initialize(ZOMBIE_WAVE_UI)）；set_page 按 180-216（Index 页向日葵+普通僵尸预览 + 三按钮 BtnNoDraw 切换）；update 按 230-260（植物/僵尸 Update + 鼠标命中 CURSOR_HAND/POINTER + PoolEffectUpdate） | AlmanacDialog.cpp:119-260 |
| #3 A3 | award_screen.rs：draw_achievements 按 606-631（白底 colorize + CHALLENGE_BACKGROUND + 标题 + 图集 70×70 切片 draw_image_stretch + 名称/描述）；achievements_continue_pressed 按 633-662（ACHIEVEMENTONLY 跳转四分支 / 回显按钮 + 通关接 CreditsZombieNote）；mouse_up 按 589-604（三按钮分派，替换原「任何点击即 Kill」）；lawn_app.rs show_award_screen 补构造期成就项填充（57-85：mShownAchievements 置位 + StartAnimTime/DestY 布局 + WriteCurrentUserConfig） | AwardScreen.cpp:51-85, 589-662 |
| #4 A4 | zombatar_widget.rs：新增 9 个 GameButton 字段并在 new() 按 291-329 创建（MakeNewButton 语义：皮肤图经 ResourceManager 查找、BACK 高亮图缺省 98×26）；update_button_state 按 1270-1288（各按钮 SetVisible/SetDisabled + FINISHED 在 Confirm/Create 两位置 Resize）；update 过渡段补 finished_button Resize 动画（404-409）；draw 末尾按 AddWidget 顺序直绘 9 按钮；mouse_up 补按钮命中 → button_depress（顺序同 AddedToManager）；补缺失常量 BACK/VIEW/NEW/CONFIRM_BACK/PORTRAIT 等 | ZombatarWidget.cpp:91-104, 273-377, 404-409, 1270-1289 |
| #5 A5 | edit_widget.rs：新增 insert_text_at_cursor（539-576：字体可渲染过滤 + 选区替换/光标插入 + 字节前进）；Ctrl+V 分支改 GetClipboard（截断至首行）→ InsertTextAtCursor；Ctrl+C 补 CopyToClipboard；sexy_app_base.rs copy_to_clipboard/get_clipboard 经 sdl2::sys SDL_SetClipboardText/SDL_GetClipboardText 实装（含 mPlayingDemoBuffer 门控，对齐 SexyAppBase.cpp:3571-3591） | EditWidget.cpp:314-354, 539-576；SexyAppBase.cpp:3571-3591 |
| #6 A6 | reg_emu.rs 全量重写：两级 BTreeMap（键序同 std::map）+ REGEMU 二进制持久化（REGEMU 头 + u16 版本 + 键/值树，save_to_file 对齐 SaveToFile 48-92，set_reg_file 对齐 94-151）；registry_read/write/erase_key/erase_value 对齐 153-214；sexy_app_base.rs 新增 registry_write{,String,Integer,Boolean}/registry_read{,String,Integer,Boolean}/registry_erase_{key,value}（对齐 1022-1335，键名 SOFTWARE\ 前缀拆分）+ WriteToRegistry/ReadFromRegistry（1100-1111/1337-1375，MusicVolume/SfxVolume/Muted/ScreenMode/PreferredX/Y/InProgress/WaitForVSync）；新增 m_read_from_registry/m_wait_for_v_sync/m_last_shutdown_was_graceful 字段；lawn_app.rs write_to_registry 接 CurUser 写入 + base.write_to_registry() | RegEmu.cpp 全文；SexyAppBase.cpp:1022-1375 |
| #7 A7 | music.rs：setup_volume_for_tune 按 Music.cpp:112-167（DAY_GRASSWALK/POOL_WATERYGRAVES/FOG_RIGORMORMIST/ROOF_GRAZETHEROOF 四曲目 30 轨主/鼓/镲音量矩阵，重合轨 max，Mix_ModMusicStreamSetChannelVolume 逐轨）；get_music_order/get_music_handle 经 MusicInterface trait 新默认方法走 SDL 后端；sdl_music_interface.rs get_music_handle/get_music_order（Mix_ModMusicStreamGetOrder）/set_music_channel_volume；ffi/sdl_mixer.rs 补 Mix_ModMusicStreamSetChannelVolume/Mix_ModMusicStreamGetOrder FFI | Music.cpp:112-167, 228-234, 375-378；SDLMusicInterface.cpp:308-319 |
| #8 A8 | imagelib/mod.rs：write_jpeg_image 复核为已有真实编码（非空体），补 C++ jpeg_set_quality(80) 语义（JpegEncoder::new_with_quality 80）；write_png/write_tga 复核已有 RGBA 编码，与 C++ 8/32 位输出等价 | ImageLib.cpp:759-972 |
| #9 A9 | sexy_app_base.rs：start_text_input/stop_text_input 实装 SDL_StartTextInput/SDL_StopTextInput（Input.cpp:492-511，桌面恒返回 false）；新增 set_text_input_rect（513-526：MouseDestRect→MouseSourceRect 映射 + SDL_SetTextInputRect） | Input.cpp:492-527 |
| #10 A10 | resource_manager.rs：新增 find_image_path（遍历 image_map 指针比对，对齐 PvzpResourceManager::FindImagePath 1185-1198）与 find_font_path（Box<Font> 地址比对，对齐 FindFontPath 1170-1183）；供 Definition 加载时反查路径（C++ Definition.cpp:280/287 调用点语义） | PvzpCommon.cpp:1161-1198 |

> 第二轮修复共触碰 14 个源文件；全部改动带 C++ 行号标注；未运行游戏验证。

---

## 3. 判定为等价 / 改名 / 非缺口（本次新增 + 沿用旧判定）

**内联/重写等价（本次确证）**：
- `Reanimation::DrawTrack`(180 行) → 内联进 `draw_render_group`（`reanimator.rs:258-580+`，含 GetCurrentTransform 插值、track×override 颜色乘、atlas 解码、clip 恢复、shake 平移，逐段标注 C++ 行号）
- `Resources.cpp` Extract* 全家（约 1,200 行代码生成物）→ Rust 从 resources.xml 动态解析
- `WidgetManager::DrawScreen/DrawWidgetsTo` → 内联于 `draw_dirty_stuff`（`sexy_app_base.rs:1203`）与 `widget_manager.draw:255`
- `WidgetContainer::GetWidgetAtHelper/UpdateAll/DrawAll/IsBelowHelper` → widget_manager 命中遍历(`:296-311`)/`update:266`/`draw:255`/`is_below:407` 承担
- `MemoryImage::GetBits/NormalBlt/AdditiveBlt/DrawLine/DrawLineAA` → 转发层（`memory_image.rs:101-190` → image.rs 实现，规模悬殊误报）
- `GraphicsState::CopyStateFrom` → Graphics push/pop_state；`Transform::CalcMatrix` → `calc_matrix_internal`（`sexy_matrix.rs:299`）
- `DataSync::SyncUInt64/UInt32/UInt16/UInt8/Float/Double` → `sync_u32/sync_u16/sync_u8/sync_f32/sync_f64`（`data_sync.rs:348-388`）
- `XMLParser::GetUTF8/UTF16LE/UTF16BEChar` → `xml_parser.rs` 编码处理内部化；`KeyCodes::GetKeyCodeFromName/GetKeyNameFromCode` → `key_codes.rs` 常量表 + `sdl_keycode_to_keycode`
- `Sexy::VFormat` → `format!`；`CommaSeperate` → `comma_separate`(`common.rs:152`)；`Trim` → `s.trim()`；`ReadBufferFromFile/WriteBytesToFile` → `std::fs`
- `SexyAppBase::DoExit` → `shutdown`(`:1651`)；`LawnApp::NewDialog` → `do_dialog/do_dialog_delay`(`lawn_app.rs:1543/1577`)；`URLOpenFailed` → `url_open_succeeded`(`:3528`)

**改名（沿用旧 §9，行号已按当前源码刷新）**：
`AddAGraveStone`→`add_grave_stone`(board.rs:6510)/`add_grave_stones`(:6579)；`FindPlantTarget`→`find_plant_target_index`(zombie.rs:3643)；`MagnetShroomAttactItem`→`magnet_shroom_attack_item`(plant.rs:4705)；`GetFreeMagnetItem`→`get_free_magnet_item_idx`(:4691)；`FlyersAreComming`→`flyers_are_comming`(seed_chooser_screen.rs:573)；`ReanimationInitialize`→`reanimation_initialize_type`(reanimator.rs:722)；`ReanimationDelete`→dispose 链；`ReanimAtlasCreate`→reanim_atlas.rs；`GetIdByVariable`→resource_id_map 反查（f615ee2）

**C++ 侧死代码 / 平台特有 / 非游戏逻辑（非缺口）**：
- `SWTri` 全家（SWDrawShape:269 等）、`SexyAppBase::MirrorImage/FlipImage/RotateImageHue/CreateCrossfadeImage/HSLToRGB/GetSharedImage/SetSharedImage/CleanSharedImages`：cpp 侧无调用点（沿用旧 §7.12）
- `EmscriptenMainLoopCallback/DoParseCmdLine/EnableCustomCursors`：平台特有；`MsgBox/Popup` → eprintln 桩（`sexy_app_base.rs:1872`）
- `PvzpAllocator` → Rust Vec/Box 所有权模型；`PerfTimer` 全家 → `perf_timer.rs`（仅统计用途）
- `LawnApp::LoadingThreadProc/LoadGroup` → 同步加载模型（`load_resources`@`resource_manager.rs:534`），异步分步接口桩已注明（`:506/512`）
- `ListWidget::Sort/RemoveAll` → ListWidget 为未接线桩组件（user_dialog.rs 用本地简化版，沿用旧 §7.6）
- Registry read 家族 → reg_emu 桩（缺口已列入 A6，不重复计非缺口）

---

## 4. 覆盖范围与未验证项

1. 已通过 `cargo check`（0 error / 754 warnings），但**未运行游戏** → 无运行期行为验证；754 条 warning 未逐条分类。
2. §1-B 七项疑似等价需逐行复核后方可结案。
3. ZombatarWidget 3 处图片尺寸占位值（67 / 39 / 21×20）无法从源码确证——真实尺寸在 main.pak 内（继承旧 §10-5）。
4. cpp 自有源码口径（243 文件 / 122,600 行）与旧版报告口径不同，行数不可直接对比。
5. 排除项不变：SDL-Mixer-X、glad、fcaseopen、platform/emscripten、platform/switch、MusPlayer_Qt、EchoTune、pxtn*、PGE_* 等第三方/调试工具不属游戏逻辑，未计入缺口。

---

## 5. 复现方法

```bash
# ① 文件级映射：cpp/src（排除第三方）与 src 的 basename 归一化比对
# ② 名字级差集：C++ "Class::Method(" 定义 vs Rust 全 fn（snake 归一化 + 模糊子串分类）
# ③ 函数体规模悬殊候选：brace 平衡扫描 → C++ 体 ≥15 行且 Rust 同名 ≤1/8
cargo check --message-format short

# 现存缺口抽查
rg -n -A 3 "fn on_start_button" src/lawn/widget/seed_chooser_screen.rs
rg -n -A 8 "pub fn update" src/lawn/widget/almanac_dialog.rs
rg -n -B 2 -A 14 "Ctrl\+V" src/framework/widget/edit_widget.rs
rg -n "fn registry_read|fn registry_write" src/framework/reg_emu.rs
rg -n "fn setup_volume_for_tune" src/lawn/system/music.rs
rg -n "fn start_text_input" src/framework/sexy_app_base.rs

# 已修复抽查
rg -n "fn init_zombie_waves" src/lawn/challenge.rs            # challenge.rs:1450
rg -n "fn tod_pick_from_array" src/todlib/tod_common.rs        # tod_common.rs:600
rg -n "fn play_channel" src/framework/sound/sdl_sound_instance.rs
rg -n "item_type" src/lawn/system/save_game.rs                 # sync_magnet_item_portable 第 5 字段
```

> 本次审计全程只读源码；写操作仅有 `cargo check`（仅生成 target/ 构建产物）与本文件的覆写（用户要求）。
