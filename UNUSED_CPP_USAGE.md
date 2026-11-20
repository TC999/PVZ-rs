# 未使用符号 C++ 出处分析（2026-11-20）

> 分析对象：`UNUSED_SYMBOLS.md` 的 1573 条未使用符号（rustc dead_code / unused_* 诊断）。
> 问题：这些 Rust 符号**在原来（cpp/ 权威源码）何处被使用？**
> 方法：Rust snake_case → C++ CamelCase 归一化 + 词根模糊匹配 + 全语料（排除 glad / fcaseopen / SDL-Mixer-X 第三方）检索调用点；变量/字段类按 Rust 文件↔C++ 文件映射（`target/rust_cpp_file_map.json`）限定范围；检索管线与中间产物在 `target/cpp_*.json`。
> **本文只做静态定位，未改任何游戏代码，未运行验证。**

## 0. 结论总览

| 判定 | 条数 | 含义 |
|---|---:|---|
| C++ 有使用（接线缺口） | 480 | C++ 有调用点/读写点，Rust 无人调用 → **接线路径缺失**（P2 台账的直接证据清单） |
| C++ 仅定义/声明 | 23 | C++ 本身也没有调用点（虚函数占位/预留 API，翻译忠实的结果） |
| Rust 自创或逻辑拆分 | 36 | C++ 无独立对应函数（自创组织函数或内联拆分），属翻译口径问题待人工裁决 |
| 疑似改名（候选） | 16 | 有 C++ 候选名，需人工确认 |
| Rust 自有 | 130 | 借用辅助 / 未使用导入 / 自创常量等 |
| 枚举变体未接线（非 ResourceId） | 17 | 所属枚举在 Rust 未接线 |
| ResourceId 整类 | 868 | 游戏实际走字符串键查找（C++ 同样不按枚举取图），清单镜像未接线是**预期形态，非缺口** |

**核心答案：480 项在 C++ 中确实被使用**（接线缺口），23 项 C++ 自己也没有调用，36 项是 Rust 自创/拆分。


## 1. C++ 有使用 → Rust 接线缺口（P2 台账证据）

共 480 项。

| Rust 位置 | 符号 | 类别 | C++ 使用处（原来在何处被使用） |
|---|---|---|---|
| src/framework/sound/sdl_sound_manager.rs:358 | set_music_amplify | 未使用函数 | cpp/src/SexyAppFramework/sound/DummyMusicInterface.h:54 in DummyMusicInterface |
| src/framework/sound/sdl_sound_manager.rs:367 | load_au_sound | 未使用函数 | SexyAppFramework/sound/SDLSoundManager.cpp:280 \| LoadAUSound(theSfxID, theFilename + ".au"); |
| src/framework/sound/sdl_sound_manager.rs:371 | get_num_sounds | 未使用函数 | cpp/src/SexyAppFramework/misc/ResourceManager.cpp:1052 in ResourceManager::GetNumResources |
| src/framework/widget/mod.rs:24 | set_select | 未使用函数 | cpp/src/Lawn/Widget/UserDialog.cpp:55 in UserDialog::UserDialog |
| src/framework/widget/mod.rs:28 | get_last_color | 未使用函数 | cpp/src/SexyAppFramework/widget/TextWidget.cpp:230 in TextWidget::AddToPhysicalLines |
| src/framework/widget/mod.rs:30 | get_text_index_at | 未使用函数 | cpp/src/SexyAppFramework/widget/TextWidget.cpp:393 in Widget::MouseDown |
| src/framework/widget/mod.rs:34 | remove_all_widgets | 未使用函数 | cpp/src/Lawn/Widget/GameSelector.cpp:372 in Sexy::StrFormat |
| src/framework/widget/mod.rs:36 | free_resources | 未使用函数 | cpp/src/SexyAppFramework/widget/WidgetManager.cpp:69 in WidgetManager::WidgetManager |
| src/framework/widget/mod.rs:38 | get_widget_flags | 未使用函数 | cpp/src/SexyAppFramework/widget/WidgetManager.cpp:113 in WidgetManager::GetAnyWidgetAt |
| src/framework/widget/mod.rs:42 | get_sort_key | 未使用函数 | cpp/src/SexyAppFramework/widget/ListWidget.cpp:82 in ListWidget::GetSortKey |
| src/lawn/board.rs:153 | render_item_sort_func | 未使用函数 | cpp/src/Lawn/Board.cpp:5950 in Board::DrawBackdrop |
| src/lawn/board.rs:240 | board_init_for_player | 未使用函数 | cpp/src/Lawn/Board.cpp:268 in Board::Board |
| src/lawn/challenge.rs:25 | get_image | 未使用函数 | cpp/src/SexyAppFramework/imagelib/ImageLib.cpp:1323 in std::string_view |
| src/lawn/lawn_app.rs:3957 | lawn_get_current_level_name | 未使用函数 | cpp/src/LawnApp.cpp:3214 in LawnApp::GetMoneyString |
| src/lawn/lawn_app.rs:3963 | lawn_get_close_request | 未使用函数 | cpp/src/LawnApp.cpp:98 in (文件级) |
| src/lawn/lawn_common.rs:30 | mod_in_range | 未使用函数 | cpp/src/Lawn/LawnCommon.cpp:44 in (文件级) |
| src/lawn/lawn_common.rs:41 | grid_in_range | 未使用函数 | cpp/src/Lawn/Board.cpp:9218 in Board::FindUmbrellaPlant |
| src/lawn/lawn_common.rs:60 | tile_image_horizontally | 未使用函数 | cpp/src/Lawn/LawnCommon.cpp:58 in (文件级) |
| src/lawn/lawn_common.rs:79 | tile_image_vertically | 未使用函数 | cpp/src/Lawn/LawnCommon.cpp:69 in std::min |
| src/lawn/lawn_common.rs:161 | create_edit_widget | 未使用函数 | cpp/src/Lawn/LawnCommon.cpp:138 in EditWidget::KeyText |
| src/lawn/lawn_common.rs:185 | make_new_checkbox | 未使用函数 | cpp/src/Lawn/LawnCommon.cpp:154 in EditWidget::KeyText |
| src/lawn/plant.rs:5804 | get_image | 未使用函数 | cpp/src/SexyAppFramework/imagelib/ImageLib.cpp:1323 in std::string_view |
| src/lawn/projectile.rs:13 | pvzp_scale_rotate_transform_matrix | 未使用函数 | cpp/src/Lawn/Challenge.cpp:2362 in Challenge::DrawBeghouled |
| src/framework/sound/music_interface.rs:3 | load_music | 未使用方法 | cpp/src/SexyAppFramework/sound/DummyMusicInterface.h:36 in DummyMusicInterface |
| src/framework/sound/music_interface.rs:8 | stop_all_music | 未使用方法 | cpp/src/Lawn/Board.cpp:1363 in Board::InitLevel |
| src/framework/sound/music_interface.rs:10 | unload_all_music | 未使用方法 | cpp/src/SexyAppFramework/sound/DummyMusicInterface.h:43 in DummyMusicInterface |
| src/framework/sound/music_interface.rs:11 | pause_all_music | 未使用方法 | cpp/src/SexyAppFramework/sound/DummyMusicInterface.h:44 in DummyMusicInterface |
| src/framework/sound/music_interface.rs:12 | resume_all_music | 未使用方法 | cpp/src/SexyAppFramework/sound/DummyMusicInterface.h:45 in DummyMusicInterface |
| src/framework/sound/music_interface.rs:13 | fade_in | 未使用方法 | cpp/src/SexyAppFramework/sound/DummyMusicInterface.h:46 in DummyMusicInterface |
| src/framework/sound/music_interface.rs:14 | fade_out | 未使用方法 | cpp/src/Lawn/Board.cpp:1936 in Board::FadeOutLevel |
| src/framework/sound/music_interface.rs:15 | fade_out_all | 未使用方法 | cpp/src/SexyAppFramework/sound/DummyMusicInterface.h:48 in DummyMusicInterface |
| src/framework/sound/music_interface.rs:17 | set_song_max_volume | 未使用方法 | cpp/src/SexyAppFramework/sound/DummyMusicInterface.h:50 in DummyMusicInterface |
| src/framework/sound/music_interface.rs:18 | is_playing | 未使用方法 | cpp/src/PvzpLib/PvzpFoley.cpp:186 in PvzpDSoundInstance::SetSoundPosition |
| src/framework/sound/music_interface.rs:19 | set_volume | 未使用方法 | cpp/src/PvzpLib/PvzpFoley.cpp:383 in PvzpFoley::ApplyMusicVolume |
| src/framework/sound/music_interface.rs:20 | update | 未使用方法 | cpp/src/Lawn/Board.cpp:4983 in Board::UpdateGameObjects |
| src/framework/sound/sound_instance.rs:17 | set_pan | 未使用方法 | cpp/src/SexyAppFramework/SexyAppBase.cpp:4082 in SexyAppBase::PlaySample |
| src/framework/sound/sound_instance.rs:27 | get_volume | 未使用方法 | cpp/src/SexyAppFramework/sound/SDL-Mixer-X/src/codecs/music_cmd.c:286 in if |
| src/framework/sound/sound_manager.rs:3 | initialized | 未使用方法 | cpp/src/SexyAppFramework/sound/SDL-Mixer-X/src/codecs/dr_libs/dr_flac.h:1781 in if |
| src/framework/sound/sound_manager.rs:6 | release_sound | 未使用方法 | cpp/src/SexyAppFramework/misc/ResourceManager.cpp:47 in SoundRes::DeleteResource |
| src/framework/sound/sound_manager.rs:7 | set_volume | 未使用方法 | cpp/src/PvzpLib/PvzpFoley.cpp:383 in PvzpFoley::ApplyMusicVolume |
| src/framework/sound/sound_manager.rs:8 | set_base_volume | 未使用方法 | cpp/src/SexyAppFramework/misc/ResourceManager.cpp:811 in ResourceManager::DoLoadSound |
| src/framework/sound/sound_manager.rs:9 | set_base_pan | 未使用方法 | cpp/src/SexyAppFramework/misc/ResourceManager.cpp:814 in ResourceManager::DoLoadSound |
| src/framework/sound/sound_manager.rs:12 | stop_all_sounds | 未使用方法 | cpp/src/SexyAppFramework/SexyAppBase.cpp:2659 in SexyAppBase::Process |
| src/framework/sound/sound_manager.rs:13 | set_master_volume | 未使用方法 | cpp/src/SexyAppFramework/SexyAppBase.cpp:4150 in SexyAppBase::SetMasterVolume |
| src/framework/sound/sound_manager.rs:14 | get_master_volume | 未使用方法 | cpp/src/SexyAppFramework/SexyAppBase.cpp:4144 in SexyAppBase::GetMasterVolume |
| src/framework/sound/sound_manager.rs:15 | flush | 未使用方法 | cpp/src/SexyAppFramework/graphics/GLInterface.cpp:1080 in GLInterface::GLInterface |
| src/framework/sound/sound_manager.rs:16 | get_free_sound_id | 未使用方法 | cpp/src/SexyAppFramework/misc/ResourceManager.cpp:802 in ResourceManager::DoLoadSound |
| src/lawn/board.rs:1689 | draw_debug_text | 未使用方法 | cpp/src/Lawn/Board.cpp:7459 in Board::DrawUITop |
| src/lawn/board.rs:1866 | draw_debug_object_rects | 未使用方法 | cpp/src/Lawn/Board.cpp:7460 in Board::DrawUITop |
| src/lawn/board.rs:1927 | add_boss_render_item | 未使用方法 | cpp/src/Lawn/Board.cpp:6149 in Board::DrawGameObjects |
| src/lawn/board.rs:1989 | draw_ui_coin_bank | 未使用方法 | Lawn/Board.cpp:6477 \| DrawUICoinBank(g); |
| src/lawn/board.rs:2047 | draw_zen_wheel_barrow_button | 未使用方法 | cpp/src/Lawn/Board.cpp:6854 in Board::DrawZenButtons |
| src/lawn/board.rs:2143 | draw_shovel | 未使用方法 | cpp/src/Lawn/Board.cpp:7216 in std::abs |
| src/lawn/board.rs:2318 | draw_ui_bottom | 未使用方法 | Lawn/Board.cpp:6489 \| DrawUIBottom(g); |
| src/lawn/board.rs:2395 | draw_top_right_ui | 未使用方法 | Lawn/Board.cpp:7219 \| DrawTopRightUI(g); |
| src/lawn/board.rs:2480 | draw_ui_top | 未使用方法 | Lawn/Board.cpp:6493 \| DrawUITop(g); |
| src/lawn/board.rs:2582 | draw_backdrop | 未使用方法 | cpp/src/Lawn/Board.cpp:5943 in Board::DrawBackdrop |
| src/lawn/board.rs:2766 | draw_zen_buttons | 未使用方法 | cpp/src/Lawn/Board.cpp:6908 in Board::DrawShovel |
| src/lawn/board.rs:3123 | draw_progress_meter | 未使用方法 | cpp/src/Lawn/Board.cpp:7413 in Board::DrawUITop |
| src/lawn/board.rs:3595 | game_over | 未使用方法 | Lawn/Widget/LawnDialog.cpp:455 \| "[GAME_OVER]", |
| src/lawn/board.rs:3631 | draw_game_objects | 未使用方法 | cpp/src/Lawn/Board.cpp:7492 in Board::Draw |
| src/lawn/board.rs:4106 | draw | 未使用方法 | cpp/src/Lawn/Board.cpp:6325 in std::sort |
| src/lawn/board.rs:4232 | draw_ice | 未使用方法 | cpp/src/Lawn/Board.cpp:6459 in std::sort |
| src/lawn/board.rs:4280 | draw_fog | 未使用方法 | cpp/src/Lawn/Board.cpp:6497 in std::sort |
| src/lawn/board.rs:4334 | draw_house_door_bottom | 未使用方法 | cpp/src/Lawn/Board.cpp:5929 in Board::DrawBackdrop |
| src/lawn/board.rs:4360 | draw_house_door_top | 未使用方法 | cpp/src/Lawn/Board.cpp:6485 in std::sort |
| src/lawn/board.rs:4389 | draw_level | 未使用方法 | cpp/src/Lawn/Board.cpp:7414 in Board::DrawUITop |
| src/lawn/board.rs:4548 | draw_fade_out | 未使用方法 | cpp/src/Lawn/Board.cpp:6505 in std::sort |
| src/lawn/board.rs:4577 | mouse_down | 未使用方法 | cpp/src/Lawn/Board.cpp:4397 in Widget::MouseDown |
| src/lawn/board.rs:4799 | key_down | 未使用方法 | cpp/src/Lawn/Board.cpp:7661 in Board::KeyDown |
| src/lawn/board.rs:4897 | key_char | 未使用方法 | cpp/src/Lawn/Widget/SeedChooserScreen.cpp:1135 in SeedChooserScreen::KeyChar |
| src/lawn/board.rs:5703 | pick_background | 未使用方法 | cpp/src/Lawn/Board.cpp:1366 in Board::InitLevel |
| src/lawn/board.rs:5929 | count_sun_flowers | 未使用方法 | cpp/src/Lawn/Board.cpp:3923 in Plant::GetNameString |
| src/lawn/board.rs:6047 | has_progress_meter | 未使用方法 | cpp/src/Lawn/Board.cpp:6558 in Board::DrawProgressMeter |
| src/lawn/board.rs:6579 | add_grave_stones | 未使用方法 | cpp/src/Lawn/Board.cpp:1102 in Board::PickBackground |
| src/lawn/board.rs:7693 | mouse_move | 未使用方法 | cpp/src/Lawn/Board.cpp:3031 in Widget::MouseMove |
| src/lawn/board.rs:7699 | mouse_drag | 未使用方法 | cpp/src/SexyAppFramework/widget/WidgetManager.cpp:668 in WidgetManager::MouseMove |
| src/lawn/board.rs:7720 | take_sun_money | 未使用方法 | cpp/src/Lawn/Board.cpp:3799 in Plant::GetNameString |
| src/lawn/board.rs:7984 | find_lawn_mower_in_row | 未使用方法 | cpp/src/Lawn/CutScene.cpp:1264 in Board::MakeRenderOrder |
| src/lawn/board.rs:8044 | get_ice_z_pos | 未使用方法 | cpp/src/Lawn/Board.cpp:6259 in Board::DrawGameObjects |
| src/lawn/board.rs:8051 | get_pumpkin_at | 未使用方法 | cpp/src/Lawn/Plant.cpp:3970 in Plant::GetImage |
| src/lawn/board.rs:8268 | mouse_up | 未使用方法 | cpp/src/Lawn/Board.cpp:4614 in Widget::MouseUp |
| src/lawn/board.rs:8813 | mouse_down_with_tool | 未使用方法 | cpp/src/Lawn/Board.cpp:4017 in Board::MouseDownWithTool |
| src/lawn/board.rs:8855 | mouse_down_cobcannon_fire | 未使用方法 | cpp/src/Lawn/Board.cpp:4444 in std::min |
| src/lawn/board.rs:8893 | tool_hit_test_helper | 未使用方法 | cpp/src/Lawn/Board.cpp:3988 in Board::ToolHitTest |
| src/lawn/board.rs:9021 | mouse_down_with_plant | 未使用方法 | cpp/src/Lawn/Board.cpp:4475 in std::min |
| src/lawn/board.rs:9533 | pick_up_tool | 未使用方法 | cpp/src/Lawn/Board.cpp:4508 in std::min |
| src/lawn/board.rs:10812 | remove_seed | 未使用方法 | cpp/src/Lawn/SeedPacket.cpp:878 in SeedPacket::WasPlanted |
| src/lawn/board.rs:10902 | refresh_all_packets | 未使用方法 | cpp/src/Lawn/Challenge.cpp:5136 in Challenge::LastStandCompletedStage |
| src/lawn/board.rs:11982 | pick_special_grave_stone | 未使用方法 | cpp/src/Lawn/Board.cpp:1163 in Board::PickBackground |
| src/lawn/board.rs:12174 | has_valid_cob_cannon_spot | 未使用方法 | cpp/src/Lawn/Board.cpp:9312 in Board::PlantingRequirementsMet |
| src/lawn/board.rs:12666 | init_zombie_waves_for_level | 未使用方法 | cpp/src/Lawn/Board.cpp:1217 in Board::InitZombieWaves |
| src/lawn/board.rs:12726 | is_final_boss_level | 未使用方法 | cpp/src/Lawn/Board.cpp:1682 in ReportAchievement::GiveAchievement |
| src/lawn/board.rs:13067 | get_bottom_lawn_mower | 未使用方法 | cpp/src/Lawn/Board.cpp:1787 in LawnApp::IsEndlessIZombie |
| src/lawn/board.rs:13099 | save_game | 未使用方法 | cpp/src/Lawn/Board.cpp:29 in (文件级) |
| src/lawn/board.rs:13223 | get_zen_tool_at | 未使用方法 | cpp/src/Lawn/Board.cpp:7747 in Board::KeyChar |
| src/lawn/challenge.rs:441 | beghouled_drag_start | 未使用方法 | cpp/src/Lawn/Challenge.cpp:1289 in Challenge::MouseDown |
| src/lawn/challenge.rs:447 | beghouled_drag_update | 未使用方法 | cpp/src/Lawn/Challenge.cpp:1151 in Challenge::MouseMove |
| src/lawn/challenge.rs:493 | mouse_move | 未使用方法 | cpp/src/Lawn/Board.cpp:3031 in Widget::MouseMove |
| src/lawn/challenge.rs:507 | mouse_down | 未使用方法 | cpp/src/Lawn/Board.cpp:4397 in Widget::MouseDown |
| src/lawn/challenge.rs:540 | mouse_up | 未使用方法 | cpp/src/Lawn/Board.cpp:4614 in Widget::MouseUp |
| src/lawn/challenge.rs:963 | zombie_ate_plant | 未使用方法 | cpp/src/Lawn/Board.cpp:8372 in std::min |
| src/lawn/challenge.rs:995 | draw_backdrop | 未使用方法 | cpp/src/Lawn/Board.cpp:5943 in Board::DrawBackdrop |
| src/lawn/challenge.rs:1071 | draw_art_challenge | 未使用方法 | cpp/src/Lawn/Challenge.cpp:2393 in Challenge::DrawBackdrop |
| src/lawn/challenge.rs:1332 | draw_beghouled | 未使用方法 | cpp/src/Lawn/Challenge.cpp:2401 in Challenge::DrawBackdrop |
| src/lawn/challenge.rs:1849 | draw_slot_machine | 未使用方法 | cpp/src/Lawn/Challenge.cpp:2425 in Challenge::DrawBackdrop |
| src/lawn/challenge.rs:2186 | beghouled_clear_crater | 未使用方法 | cpp/src/Lawn/Challenge.cpp:3504 in Challenge::BeghouledPacketClicked |
| src/lawn/challenge.rs:2207 | mouse_down_whack_a_zombie | 未使用方法 | cpp/src/Lawn/Challenge.cpp:1327 in Challenge::MouseDown |
| src/lawn/challenge.rs:2283 | draw_storm_night | 未使用方法 | cpp/src/Lawn/Challenge.cpp:3032 in Challenge::DrawWeather |
| src/lawn/challenge.rs:2466 | grave_danger_spawn_random_grave | 未使用方法 | cpp/src/Lawn/Challenge.cpp:2997 in Challenge::SpawnZombieWave |
| src/lawn/challenge.rs:2507 | grave_danger_spawn_grave_at | 未使用方法 | cpp/src/Lawn/Challenge.cpp:2965 in Challenge::GraveDangerSpawnRandomGrave |
| src/lawn/challenge.rs:2694 | draw_storm_flash | 未使用方法 | cpp/src/Lawn/Challenge.cpp:3091 in Challenge::DrawStormNight |
| src/lawn/challenge.rs:3693 | beghouled_packet_clicked | 未使用方法 | cpp/src/Lawn/SeedPacket.cpp:823 in std::min |
| src/lawn/challenge.rs:3793 | zombiquarium_packet_clicked | 未使用方法 | cpp/src/Lawn/SeedPacket.cpp:827 in std::min |
| src/lawn/challenge.rs:3841 | zombiquarium_drop_brain | 未使用方法 | cpp/src/Lawn/Challenge.cpp:3628 in Challenge::ZombiquariumMouseDown |
| src/lawn/challenge.rs:3861 | zombiquarium_mouse_down | 未使用方法 | cpp/src/Lawn/Challenge.cpp:1333 in if |
| src/lawn/challenge.rs:5013 | scary_potter_mallet_pot | 未使用方法 | cpp/src/Lawn/Challenge.cpp:1343 in if |
| src/lawn/challenge.rs:5099 | i_zombie_mouse_down_with_zombie | 未使用方法 | cpp/src/Lawn/Board.cpp:3619 in Board::MouseDownWithPlant |
| src/lawn/challenge.rs:5324 | i_zombie_draw_plant | 未使用方法 | cpp/src/Lawn/Plant.cpp:4027 in Plant::IsFlying |
| src/lawn/challenge.rs:5373 | i_zombie_set_plant_filter_effect | 未使用方法 | cpp/src/Lawn/Challenge.cpp:4761 in Challenge::IZombieDrawPlant |
| src/lawn/challenge.rs:5676 | draw_rain | 未使用方法 | cpp/src/Lawn/Challenge.cpp:3029 in Challenge::DrawWeather |
| src/lawn/challenge.rs:5783 | draw_weather | 未使用方法 | cpp/src/Lawn/Board.cpp:6501 in std::sort |
| src/lawn/challenge.rs:5876 | squirrel_found | 未使用方法 | cpp/src/Lawn/Challenge.cpp:5036 in Challenge::SquirrelUpdateOne |
| src/lawn/challenge.rs:5970 | squirrel_peek | 未使用方法 | cpp/src/Lawn/Challenge.cpp:5042 in Challenge::SquirrelUpdateOne |
| src/lawn/challenge.rs:5975 | squirrel_chew | 未使用方法 | cpp/src/Lawn/Challenge.cpp:5046 in Challenge::SquirrelUpdateOne |
| src/lawn/challenge.rs:6009 | squirrel_update_one | 未使用方法 | cpp/src/Lawn/Challenge.cpp:5068 in Challenge::SquirrelUpdate |
| src/lawn/challenge.rs:6057 | i_zombie_setup_plant | 未使用方法 | cpp/src/Lawn/Challenge.cpp:4363 in Challenge::IZombiePlacePlantInSquare |
| src/lawn/challenge.rs:6180 | i_zombie_eat_brain | 未使用方法 | cpp/src/Lawn/Zombie.cpp:6886 in Zombie::CheckIfPreyCaught |
| src/lawn/challenge.rs:6202 | i_zombie_get_brain_target | 未使用方法 | cpp/src/Lawn/Challenge.cpp:4839 in Challenge::IZombieEatBrain |
| src/lawn/challenge.rs:6248 | advance_crazy_dave_dialog | 未使用方法 | cpp/src/Lawn/Board.cpp:7667 in Board::KeyDown |
| src/lawn/challenge.rs:6418 | i_zombie_squish_brain | 未使用方法 | cpp/src/Lawn/Zombie.cpp:2132 in Zombie::UpdateZombieGargantuar |
| src/lawn/challenge.rs:6425 | i_zombie_score_brain | 未使用方法 | cpp/src/Lawn/Challenge.cpp:4849 in Challenge::IZombieEatBrain |
| src/lawn/challenge.rs:6564 | beghouled_twist_mouse_down | 未使用方法 | cpp/src/Lawn/Challenge.cpp:1299 in Challenge::MouseDown |
| src/lawn/challenge.rs:6862 | i_zombie_place_zombie | 未使用方法 | cpp/src/Lawn/Board.cpp:8166 in Board::KeyChar |
| src/lawn/challenge.rs:6981 | tree_of_wisdom_fertilize | 未使用方法 | cpp/src/Lawn/Board.cpp:7857 in Board::KeyChar |
| src/lawn/challenge.rs:7025 | tree_of_wisdom_init | 未使用方法 | cpp/src/Lawn/Challenge.cpp:435 in Challenge::InitLevel |
| src/lawn/challenge.rs:7117 | tree_of_wisdom_mouse_on | 未使用方法 | cpp/src/Lawn/Challenge.cpp:5190 in Challenge::TreeOfWisdomDraw |
| src/lawn/challenge.rs:7153 | tree_of_wisdom_draw | 未使用方法 | cpp/src/Lawn/Challenge.cpp:2397 in Challenge::DrawBackdrop |
| src/lawn/challenge.rs:7303 | tree_of_wisdom_next_garden | 未使用方法 | cpp/src/Lawn/Board.cpp:4494 in std::min |
| src/lawn/challenge.rs:7323 | tree_of_wisdom_open_store | 未使用方法 | cpp/src/Lawn/Board.cpp:4649 in Widget::MouseUp |
| src/lawn/challenge.rs:7366 | tree_of_wisdom_tool | 未使用方法 | cpp/src/Lawn/Board.cpp:4022 in Board::MouseDownWithTool |
| src/lawn/coin.rs:1593 | mouse_down | 未使用方法 | cpp/src/Lawn/Board.cpp:4397 in Widget::MouseDown |
| src/lawn/cursor_object.rs:65 | init | 未使用方法 | cpp/src/LawnApp.cpp:1245 in SexyApp::Init |
| src/lawn/cursor_object.rs:104 | die | 未使用方法 | cpp/src/Lawn/Board.cpp:1798 in std::clamp |
| src/lawn/cursor_object.rs:126 | draw | 未使用方法 | cpp/src/Lawn/Board.cpp:6325 in std::sort |
| src/lawn/cursor_object.rs:377 | draw | 未使用方法 | cpp/src/Lawn/Board.cpp:6325 in std::sort |
| src/lawn/cursor_object.rs:451 | plant_draw_height_offset | 未使用方法 | cpp/src/Lawn/CursorObject.cpp:191 in CursorObject::Draw |
| src/lawn/cutscene.rs:1324 | is_after_seed_chooser | 未使用方法 | cpp/src/Lawn/Board.cpp:6273 in Board::DrawGameObjects |
| src/lawn/cutscene.rs:1455 | show_zombie_walking | 未使用方法 | cpp/src/Lawn/Zombie.cpp:4347 in Zombie::Update |
| src/lawn/cutscene.rs:1461 | is_cut_scene_over | 未使用方法 | cpp/src/Lawn/CutScene.cpp:1539 in CutScene::ZombieWonClick |
| src/lawn/cutscene.rs:1467 | zombie_won_click | 未使用方法 | cpp/src/Lawn/Board.cpp:4422 in Widget::MouseDown |
| src/lawn/cutscene.rs:1477 | mouse_down | 未使用方法 | cpp/src/Lawn/Board.cpp:4397 in Widget::MouseDown |
| src/lawn/cutscene.rs:1486 | key_down | 未使用方法 | cpp/src/Lawn/Board.cpp:7661 in Board::KeyDown |
| src/lawn/cutscene.rs:1657 | can_get_packet_upgrade | 未使用方法 | cpp/src/Lawn/CutScene.cpp:771 in CutScene::StartLevelIntro |
| src/lawn/cutscene.rs:2124 | can_get_second_packet_upgrade | 未使用方法 | cpp/src/Lawn/CutScene.cpp:775 in CutScene::StartLevelIntro |
| src/lawn/cutscene.rs:2777 | draw_upsell | 未使用方法 | cpp/src/Lawn/Board.cpp:7429 in Board::DrawUITop |
| src/lawn/cutscene.rs:2856 | draw_intro | 未使用方法 | cpp/src/Lawn/Board.cpp:7433 in Board::DrawUITop |
| src/lawn/game_object.rs:39 | begin_draw | 未使用方法 | cpp/src/Lawn/Board.cpp:6323 in std::sort |
| src/lawn/game_object.rs:49 | end_draw | 未使用方法 | cpp/src/Lawn/Board.cpp:6326 in std::sort |
| src/lawn/game_object.rs:55 | make_parent_graphics_frame | 未使用方法 | cpp/src/Lawn/Coin.cpp:830 in Coin::Draw |
| src/lawn/grid_item.rs:367 | draw | 未使用方法 | cpp/src/Lawn/Board.cpp:6325 in std::sort |
| src/lawn/grid_item.rs:405 | draw_grid_item_overlay | 未使用方法 | cpp/src/Lawn/Board.cpp:6454 in std::sort |
| src/lawn/grid_item.rs:428 | draw_i_zombie_brain | 未使用方法 | cpp/src/Lawn/GridItem.cpp:111 in GridItem::DrawGridItem |
| src/lawn/grid_item.rs:470 | draw_grave_stone | 未使用方法 | cpp/src/Lawn/GridItem.cpp:99 in GridItem::DrawGridItem |
| src/lawn/grid_item.rs:586 | draw_stinky | 未使用方法 | cpp/src/Lawn/GridItem.cpp:110 in GridItem::DrawGridItem |
| src/lawn/grid_item.rs:648 | draw_crater | 未使用方法 | cpp/src/Lawn/GridItem.cpp:100 in GridItem::DrawGridItem |
| src/lawn/grid_item.rs:723 | draw_scary_pot | 未使用方法 | cpp/src/Lawn/GridItem.cpp:107 in GridItem::DrawGridItem |
| src/lawn/grid_item.rs:783 | draw_ladder | 未使用方法 | cpp/src/Lawn/GridItem.cpp:101 in GridItem::DrawGridItem |
| src/lawn/grid_item.rs:799 | add_grave_stone_particles | 未使用方法 | cpp/src/Lawn/Challenge.cpp:2938 in Challenge::GraveDangerSpawnGraveAt |
| src/lawn/lawn_mower.rs:147 | start_mower | 未使用方法 | cpp/src/Lawn/LawnMower.cpp:147 in LawnMower::MowZombie |
| src/lawn/lawn_mower.rs:247 | mow_zombie | 未使用方法 | cpp/src/Lawn/LawnMower.cpp:230 in LawnMower::Update |
| src/lawn/lawn_mower.rs:265 | get_lawn_mower_attack_rect | 未使用方法 | cpp/src/Lawn/Board.cpp:7103 in Board::DrawDebugObjectRects |
| src/lawn/lawn_mower.rs:403 | draw_shadow | 未使用方法 | cpp/src/Lawn/Board.cpp:6376 in std::sort |
| src/lawn/lawn_mower.rs:464 | draw | 未使用方法 | cpp/src/Lawn/Board.cpp:6325 in std::sort |
| src/lawn/projectile.rs:863 | draw | 未使用方法 | cpp/src/Lawn/Board.cpp:6325 in std::sort |
| src/lawn/projectile.rs:986 | draw_shadow | 未使用方法 | cpp/src/Lawn/Board.cpp:6376 in std::sort |
| src/lawn/projectile.rs:1206 | is_zombie_hit_by_splash | 未使用方法 | cpp/src/Lawn/Projectile.cpp:469 in Projectile::DoSplashDamage |
| src/lawn/projectile.rs:1320 | get_projectile_def | 未使用方法 | cpp/src/Lawn/Projectile.cpp:314 in Projectile::CheckForCollision |
| src/lawn/projectile.rs:1370 | get_damage_flags | 未使用方法 | cpp/src/Lawn/Projectile.cpp:496 in std::max |
| src/lawn/projectile.rs:1428 | check_for_high_ground | 未使用方法 | cpp/src/Lawn/Projectile.cpp:736 in std::min |
| src/lawn/seed_packet.rs:245 | slot_machine_start | 未使用方法 | cpp/src/Lawn/Challenge.cpp:1309 in Challenge::MouseDown |
| src/lawn/seed_packet.rs:307 | draw | 未使用方法 | cpp/src/Lawn/Board.cpp:6325 in std::sort |
| src/lawn/seed_packet.rs:406 | was_planted | 未使用方法 | cpp/src/Lawn/Board.cpp:3882 in Plant::GetNameString |
| src/lawn/seed_packet.rs:450 | mouse_down | 未使用方法 | cpp/src/Lawn/Board.cpp:4397 in Widget::MouseDown |
| src/lawn/system/pool_effect.rs:56 | bilinear_lookup_fixed_point | 未使用方法 | cpp/src/Lawn/System/PoolEffect.cpp:103 in PoolEffect::UpdateWaterEffect |
| src/lawn/widget/award_screen.rs:709 | draw_achievements | 未使用方法 | cpp/src/Lawn/Widget/AwardScreen.cpp:301 in AwardScreen::Draw |
| src/lawn/widget/message_widget.rs:106 | get_font | 未使用方法 | cpp/src/Lawn/MessageWidget.cpp:183 in MessageWidget::LayoutReanimText |
| src/lawn/widget/message_widget.rs:112 | draw | 未使用方法 | cpp/src/Lawn/Board.cpp:6325 in std::sort |
| src/lawn/widget/message_widget.rs:308 | is_being_displayed | 未使用方法 | cpp/src/Lawn/Challenge.cpp:894 in Challenge::BeghouledScore |
| src/lawn/widget/message_widget.rs:314 | layout_reanim_text | 未使用方法 | cpp/src/Lawn/MessageWidget.cpp:173 in MessageWidget::SetLabel |
| src/lawn/widget/message_widget.rs:398 | draw_reanimated_text | 未使用方法 | cpp/src/Lawn/MessageWidget.cpp:459 in MessageWidget::Draw |
| src/lawn/widget/title_screen.rs:141 | resize | 未使用方法 | cpp/src/Lawn/Board.cpp:205 in Board::Board |
| src/lawn/widget/title_screen.rs:148 | added_to_manager | 未使用方法 | cpp/src/SexyAppFramework/widget/WidgetContainer.cpp:94 in std::find |
| src/lawn/widget/title_screen.rs:156 | removed_from_manager | 未使用方法 | cpp/src/SexyAppFramework/widget/Widget.cpp:85 in Widget::WidgetRemovedHelper |
| src/lawn/widget/title_screen.rs:162 | button_press | 未使用方法 | cpp/src/Lawn/Board.h:291 in (文件级) |
| src/lawn/widget/title_screen.rs:169 | button_depress | 未使用方法 | cpp/src/Lawn/Board.h:292 in (文件级) |
| src/framework/resources.rs:894 | count | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:3016 in Sexy::PrintF |
| src/lawn/board.rs:82 | new | 未使用关联项 | cpp/src/Lawn/Widget/CheatDialog.cpp:32 in CheatDialog::CheatDialog |
| src/lawn/lawn_app.rs:52 | new | 未使用关联项 | cpp/src/Lawn/Widget/CheatDialog.cpp:32 in CheatDialog::CheatDialog |
| src/lawn/lawn_app.rs:569 | start_level | 未使用关联项 | cpp/src/Lawn/Board.cpp:1661 in Board::StartLevel |
| src/lawn/lawn_app.rs:978 | kill_seed_chooser_screen | 未使用关联项 | cpp/src/LawnApp.cpp:343 in LawnApp::KillBoard |
| src/lawn/lawn_app.rs:1180 | finish_modeless_dialogs | 未使用关联项 | cpp/src/LawnApp.cpp:342 in LawnApp::KillBoard |
| src/lawn/lawn_app.rs:1526 | do_cheat_dialog | 未使用关联项 | cpp/src/Lawn/Board.cpp:7922 in Board::KeyChar |
| src/lawn/lawn_app.rs:1567 | do_pause_dialog | 未使用关联项 | cpp/src/Lawn/Board.cpp:7676 in Board::KeyDown |
| src/lawn/lawn_app.rs:1650 | center_dialog | 未使用关联项 | cpp/src/Lawn/Widget/LawnDialog.cpp:472 in GameOverDialog::GameOverDialog |
| src/lawn/lawn_app.rs:2079 | can_pause_now | 未使用关联项 | cpp/src/Lawn/Board.cpp:7673 in Board::KeyDown |
| src/lawn/lawn_app.rs:2209 | get_current_challenge_def | 未使用关联项 | cpp/src/Lawn/Board.cpp:6670 in Board::DrawLevel |
| src/lawn/lawn_app.rs:2215 | get_current_level_name | 未使用关联项 | cpp/src/LawnApp.h:319 in GetCurrentLevelName |
| src/lawn/lawn_app.rs:2846 | toggle_slow_mo | 未使用关联项 | cpp/src/Lawn/Board.cpp:8031 in Board::KeyChar |
| src/lawn/lawn_app.rs:2855 | toggle_fast_mo | 未使用关联项 | cpp/src/Lawn/Board.cpp:8035 in Board::KeyChar |
| src/lawn/lawn_app.rs:2862 | got_focus | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:2033 in SexyAppBase::RehupFocus |
| src/lawn/lawn_app.rs:2866 | lost_focus | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:2041 in SexyAppBase::RehupFocus |
| src/lawn/lawn_app.rs:2872 | modal_open | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:918 in DialogMap::value_type |
| src/lawn/lawn_app.rs:2881 | init_hook | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:3556 in Sexy::LogError |
| src/lawn/lawn_app.rs:2886 | update_play_time_stats | 未使用关联项 | cpp/src/LawnApp.cpp:1621 in LawnApp::UpdateFrames |
| src/lawn/lawn_app.rs:3014 | modal_close | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:870 in std::find |
| src/lawn/lawn_app.rs:3023 | do_confirm_back_to_main | 未使用关联项 | cpp/src/Lawn/Widget/NewOptionsDialog.cpp:340 in Dialog::ButtonDepress |
| src/lawn/lawn_app.rs:3078 | do_register | 未使用关联项 | cpp/src/LawnApp.cpp:1876 in LawnApp::ButtonDepress |
| src/lawn/lawn_app.rs:3087 | show_zombatar_tos | 未使用关联项 | Lawn/Widget/GameSelector.cpp:1389 \| mApp->ShowZombatarTOS(); |
| src/lawn/lawn_app.rs:3101 | pre_display_hook | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:3515 in filesystem::path |
| src/lawn/lawn_app.rs:3106 | loading_thread_completed | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:1699 in SexyAppBase::DoUpdateFrames |
| src/lawn/lawn_app.rs:3125 | update_app | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:2890 in SexyAppBase::DoMainLoop |
| src/lawn/lawn_app.rs:3389 | write_to_registry | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:1624 in this_thread::get_id |
| src/lawn/lawn_app.rs:3401 | read_from_registry | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:3491 in filesystem::path |
| src/lawn/lawn_app.rs:3414 | switch_screen_mode | 未使用关联项 | cpp/src/Lawn/Board.cpp:8435 in std::max |
| src/lawn/lawn_app.rs:3426 | change_dir_hook | 未使用关联项 | cpp/src/SexyAppFramework/SexyAppBase.cpp:3422 in this_thread::get_id |
| src/lawn/lawn_app.rs:3582 | url_open_succeeded | 未使用关联项 | LawnApp.cpp:1819 \| SexyAppBase::URLOpenSucceeded(theURL); |
| src/lawn/lawn_app.rs:3586 | open_url | 未使用关联项 | LawnApp.cpp:1811 \| GetString("OPEN_URL", "Please open the following URL in your browser\n\n%s\n\nFo |
| src/lawn/lawn_app.rs:3918 | mouse_down | 未使用关联项 | cpp/src/Lawn/Board.cpp:4397 in Widget::MouseDown |
| src/lawn/lawn_app.rs:3926 | key_down | 未使用关联项 | cpp/src/Lawn/Board.cpp:7661 in Board::KeyDown |
| src/lawn/lawn_app.rs:3936 | debug_key_down | 未使用关联项 | LawnApp.cpp:1338 \| return SexyAppBase::DebugKeyDown(theKey); |
| src/lawn/lawn_app.rs:3939 | show_resource_error | 未使用关联项 | cpp/src/LawnApp.cpp:1241 in SexyApp::Init |
| src/lawn/lawn_common.rs:108 | new | 未使用关联项 | cpp/src/Lawn/Widget/CheatDialog.cpp:32 in CheatDialog::CheatDialog |
| src/lawn/lawn_common.rs:119 | key_down | 未使用关联项 | cpp/src/Lawn/Board.cpp:7661 in Board::KeyDown |
| src/lawn/lawn_common.rs:127 | key_char | 未使用关联项 | cpp/src/Lawn/Widget/SeedChooserScreen.cpp:1135 in SeedChooserScreen::KeyChar |
| src/lawn/lawn_common.rs:136 | key_text | 未使用关联项 | cpp/src/SexyAppFramework/platform/default/Input.cpp:481 in std::clamp |
| src/lawn/plant.rs:1433 | find_star_fruit_target | 未使用关联项 | cpp/src/Lawn/Plant.cpp:913 in Plant::LaunchStarFruit |
| src/lawn/plant.rs:1495 | mouse_down | 未使用关联项 | cpp/src/Lawn/Board.cpp:4397 in Widget::MouseDown |
| src/lawn/plant.rs:2099 | update_reanim | 未使用关联项 | cpp/src/Lawn/Challenge.cpp:4356 in Challenge::IZombieSetupPlant |
| src/lawn/plant.rs:2434 | draw_magnet_items_on_top | 未使用关联项 | cpp/src/Lawn/Board.cpp:6123 in Board::DrawGameObjects |
| src/lawn/plant.rs:2461 | draw_magnet_items | 未使用关联项 | cpp/src/Lawn/Board.cpp:6347 in std::sort |
| src/lawn/plant.rs:3165 | is_fungus | 未使用关联项 | Lawn/Board.cpp:2129 \| bool aIsFungi = Plant::IsFungus(theSeedType); |
| src/lawn/plant.rs:4346 | star_fruit_fire | 未使用关联项 | cpp/src/Lawn/Plant.cpp:4490 in Plant::Fire |
| src/lawn/plant.rs:4553 | ice_zombies | 未使用关联项 | cpp/src/Lawn/Plant.cpp:4364 in ReportAchievement::GiveAchievement |
| src/lawn/plant.rs:4570 | burn_row | 未使用关联项 | cpp/src/Lawn/Plant.cpp:4343 in ReportAchievement::GiveAchievement |
| src/lawn/plant.rs:5647 | blow_away_fliers | 未使用关联项 | cpp/src/Lawn/Plant.cpp:4303 in Plant::DoSpecial |
| src/lawn/plant.rs:5667 | kill_all_plants_near_doom | 未使用关联项 | cpp/src/Lawn/Plant.cpp:4326 in ReportAchievement::GiveAchievement |
| src/lawn/plant.rs:5731 | cob_cannon_fire | 未使用关联项 | cpp/src/Lawn/Board.cpp:3601 in Board::MouseDownCobcannonFire |
| src/lawn/seed_packet.rs:651 | new | 未使用关联项 | cpp/src/Lawn/Widget/CheatDialog.cpp:32 in CheatDialog::CheatDialog |
| src/lawn/seed_packet.rs:662 | draw | 未使用关联项 | cpp/src/Lawn/Board.cpp:6325 in std::sort |
| src/lawn/seed_packet.rs:670 | mouse_hit_test | 未使用关联项 | cpp/src/Lawn/Board.cpp:2952 in Board::UpdateCursor |
| src/lawn/system/pool_effect.rs:28 | new | 未使用关联项 | cpp/src/Lawn/Widget/CheatDialog.cpp:32 in CheatDialog::CheatDialog |
| src/lawn/system/pool_effect.rs:56 | bilinear_lookup_fixed_point | 未使用关联项 | cpp/src/Lawn/System/PoolEffect.cpp:103 in PoolEffect::UpdateWaterEffect |
| src/lawn/system/pool_effect.rs:86 | update_water_effect | 未使用关联项 | cpp/src/Lawn/System/PoolEffect.cpp:252 in PoolEffect::PoolEffectDraw |
| src/lawn/system/pool_effect.rs:188 | draw | 未使用关联项 | cpp/src/Lawn/Board.cpp:6325 in std::sort |
| src/lawn/widget/achievements_screen.rs:141 | new | 未使用关联项 | cpp/src/Lawn/Widget/CheatDialog.cpp:32 in CheatDialog::CheatDialog |
| src/lawn/widget/achievements_screen.rs:153 | update | 未使用关联项 | cpp/src/Lawn/Board.cpp:4983 in Board::UpdateGameObjects |
| src/lawn/widget/achievements_screen.rs:166 | draw | 未使用关联项 | cpp/src/Lawn/Board.cpp:6325 in std::sort |
| src/lawn/widget/achievements_screen.rs:170 | key_down | 未使用关联项 | cpp/src/Lawn/Board.cpp:7661 in Board::KeyDown |
| src/lawn/widget/achievements_screen.rs:184 | mouse_down | 未使用关联项 | cpp/src/Lawn/Board.cpp:4397 in Widget::MouseDown |
| src/lawn/widget/achievements_screen.rs:188 | mouse_up | 未使用关联项 | cpp/src/Lawn/Board.cpp:4614 in Widget::MouseUp |
| src/lawn/widget/achievements_screen.rs:196 | mouse_wheel | 未使用关联项 | cpp/src/SexyAppFramework/platform/default/Input.cpp:660 in SexyAppBase::ProcessDeferredMessages |
| src/lawn/widget/achievements_screen.rs:281 | achievement_init_for_player | 未使用关联项 | Lawn/Widget/GameSelector.cpp:550 \| ReportAchievement::AchievementInitForPlayer(mApp); |
| src/lawn/zombie.rs:3762 | burn_row | 未使用关联项 | cpp/src/Lawn/Plant.cpp:4343 in ReportAchievement::GiveAchievement |
| src/lawn/zombie.rs:4493 | mow_down | 未使用关联项 | cpp/src/Lawn/LawnMower.cpp:174 in LawnMower::MowZombie |
| src/lawn/zombie.rs:4560 | start_eating | 未使用关联项 | cpp/src/Lawn/Challenge.cpp:4843 in Challenge::IZombieEatBrain |
| src/lawn/zombie.rs:4583 | eat_zombie | 未使用关联项 | cpp/src/Lawn/Zombie.cpp:6872 in Zombie::CheckIfPreyCaught |
| src/lawn/zombie.rs:4854 | hit_ice_trap | 未使用关联项 | cpp/src/Lawn/Plant.cpp:4208 in Plant::IceZombies |
| src/lawn/zombie.rs:5755 | eat_plant | 未使用关联项 | cpp/src/Lawn/Zombie.cpp:6881 in Zombie::CheckIfPreyCaught |
| src/lawn/zombie.rs:6528 | find_zombie_target | 未使用关联项 | cpp/src/Lawn/Zombie.cpp:2089 in Zombie::UpdateZombieGargantuar |
| src/lawn/zombie.rs:6565 | check_if_prey_caught | 未使用关联项 | cpp/src/Lawn/Zombie.cpp:4646 in Zombie::UpdatePlaying |
| src/lawn/zombie.rs:6875 | has_shadow | 未使用关联项 | cpp/src/Lawn/Board.cpp:6155 in Board::DrawGameObjects |
| src/lawn/zombie.rs:8018 | bobsled_burn | 未使用关联项 | cpp/src/Lawn/Zombie.cpp:8806 in Zombie::IsZombotany |
| src/lawn/zombie.rs:8851 | draw_zombie_part | 未使用关联项 | cpp/src/Lawn/Zombie.cpp:5216 in Zombie::DrawZombieHead |
| src/lawn/zombie.rs:8989 | draw_zombie_head | 未使用关联项 | cpp/src/Lawn/Zombie.cpp:5271 in Zombie::DrawZombieWithParts |
| src/lawn/zombie.rs:10346 | draw_bungee_target | 未使用关联项 | cpp/src/Lawn/Board.cpp:6385 in std::sort |
| src/lawn/zombie.rs:10751 | draw_boss_part | 未使用关联项 | cpp/src/Lawn/Board.cpp:6394 in std::sort |
| src/lawn/zombie.rs:10765 | draw_boss_back_arm | 未使用关联项 | cpp/src/Lawn/Zombie.cpp:10535 in Zombie::DrawBossPart |
| src/lawn/zombie.rs:10801 | draw_boss_fire_ball | 未使用关联项 | cpp/src/Lawn/Zombie.cpp:10536 in Zombie::DrawBossPart |
| src/lawn/zombie.rs:11561 | setup_zombatar_flag_reanim | 未使用关联项 | cpp/src/Lawn/Zombie.cpp:592 in Zombie::ZombieInitialize |
| src/lawn/zombie.rs:12074 | bungee_drop_zombie | 未使用关联项 | cpp/src/Lawn/Board.cpp:4815 in Board::BungeeDropZombie |
| src/lawn/zombie.rs:12758 | ZOMBIE_WAVE_DEBUG | 未使用关联项 | Lawn/Board.cpp:8090 \| AddZombie(ZombieType::ZOMBIE_WALLNUT_HEAD, Zombie::ZOMBIE_WAVE_DEBUG); |
| src/lawn/board.rs:53 | MAX_RENDER_ITEMS | 未使用常量 / 静态 | Lawn/Board.cpp:5962 \| PVZP_ASSERT(theCurRenderItem < MAX_RENDER_ITEMS); |
| src/lawn/cutscene.rs:548 | TIME_SEED_CHOSER_SLIDE_ON_END | 未使用常量 / 静态 | Lawn/CutScene.cpp:59 \| static const int	TimeSeedChoserSlideOnEnd		= 4250; |
| src/lawn/cutscene.rs:549 | TIME_SEED_CHOSER_SLIDE_OFF_START | 未使用常量 / 静态 | Lawn/CutScene.cpp:60 \| static const int	TimeSeedChoserSlideOffStart		= 4500; |
| src/lawn/cutscene.rs:550 | TIME_SEED_CHOSER_SLIDE_OFF_END | 未使用常量 / 静态 | Lawn/CutScene.cpp:61 \| static const int	TimeSeedChoserSlideOffEnd		= 4750; |
| src/lawn/cutscene.rs:568 | SEED_CHOOSER_OFFSET_Y | 未使用常量 / 静态 | Lawn/CutScene.cpp:695 \| mApp->mSeedChooserScreen->Move(0, SEED_CHOOSER_OFFSET_Y); |
| src/lawn/widget/message_widget.rs:14 | MAX_REANIM_LINES | 未使用常量 / 静态 | Lawn/MessageWidget.cpp:92 \| if (aNext > MAX_MESSAGE_LENGTH - 1 \|\| (aChar == U'\n' && aLineCount == MAX_REANI |
| src/lawn/zombie.rs:31 | DAMAGE_PER_EAT | 未使用常量 / 静态 | Lawn/Zombie.cpp:67 \| constexpr const int DAMAGE_PER_EAT = TICKS_BETWEEN_EATS; |
| src/lawn/zombie.rs:10901 | CLIP_HEIGHT_LIMIT | 未使用常量 / 静态 | Lawn/Zombie.cpp:70 \| constexpr const float CLIP_HEIGHT_LIMIT = -100.0f; |
| src/lawn/zombie.rs:10904 | BUNGEE_ZOMBIE_HEIGHT | 未使用常量 / 静态 | Lawn/Zombie.cpp:48 \| constexpr const int BUNGEE_ZOMBIE_HEIGHT = 3000; |
| src/lawn/board.rs:65 | RenderItem | 未使用类型 | cpp/src/Lawn/Board.cpp:5950 in Board::DrawBackdrop |
| src/lawn/challenge.rs:17 | BeghouledUpgrade | 未使用类型 | cpp/src/Lawn/Challenge.cpp:347 in Challenge::Challenge |
| src/lawn/challenge.rs:56 | ZombieAllowedLevels | 未使用类型 | cpp/src/Lawn/Challenge.cpp:75 in (文件级) |
| src/lawn/lawn_common.rs:102 | LawnEditWidget | 未使用类型 | cpp/src/Lawn/LawnCommon.cpp:86 in LawnEditWidget::LawnEditWidget |
| src/lawn/projectile.rs:1482 | ProjectileDefinition | 未使用类型 | cpp/src/Lawn/Projectile.cpp:37 in (文件级) |
| src/lawn/seed_packet.rs:642 | SeedBank | 未使用类型 | cpp/src/Lawn/Board.cpp:96 in Board::Board |
| src/lawn/widget/achievements_screen.rs:130 | AchievementsWidget | 未使用类型 | cpp/src/Lawn/Widget/AchievementsScreen.cpp:78 in AchievementsWidget::AchievementsWidget |
| src/lawn/zombie.rs:103 | ZombieParts | 未使用类型 | cpp/src/Lawn/Zombie.cpp:5216 in Zombie::DrawZombieHead |
| src/framework/sound/sdl_sound_instance.rs:45 | m_pan | 未读取字段 | SexyAppFramework/sound/SDLSoundInstance.cpp:45 [mPan] mPan = 0; |
| src/framework/sound/sdl_sound_instance.rs:50 | m_default_frequency | 未读取字段 | SexyAppFramework/sound/SDLSoundInstance.cpp:48 [mDefaultFrequency] mDefaultFrequency = 44100; |
| src/lawn/board.rs:254 | m_width | 未读取字段 | GameConstants.h:28 [BOARD_WIDTH] constexpr const int BOARD_WIDTH = 800; |
| src/lawn/board.rs:255 | m_height | 未读取字段 | GameConstants.h:29 [BOARD_HEIGHT] constexpr const int BOARD_HEIGHT = 600; |
| src/lawn/board.rs:268 | grid_plants | 未读取字段 | Lawn/Board.cpp:79 [mPlants] mPlants.DataArrayInitialize(1024U, "plants"); |
| src/lawn/board.rs:307 | m_coin_bank | 未读取字段 | Lawn/Board.cpp:92 [mCoinBankFadeCount] mCoinBankFadeCount = 0; |
| src/lawn/board.rs:311 | m_wave_count | 未读取字段 | Lawn/Board.cpp:551 [mNumWaves] return (mApp->IsFirstTimeAdventureMode() && mNumWaves < 10) ? mNumWaves : 10; |
| src/lawn/board.rs:323 | m_game_over_countdown | 未读取字段 | Lawn/Board.cpp:1248 [mZombieHealthToNextWave] mZombieHealthToNextWave = -1; |
| src/lawn/board.rs:391 | m_ice_particle | 未读取字段 | Lawn/Board.cpp:1395 [mIceTimer] mIceTimer[aRow] = 0; |
| src/lawn/coin.rs:26 | destination_x | 未读取字段 | Lawn/Board.cpp:1293 [mX] aPacket->mX = GetSeedPacketPositionX(i); |
| src/lawn/coin.rs:27 | destination_y | 未读取字段 | Lawn/Board.cpp:1412 [mY] aPacket->mY = 8; |
| src/lawn/lawn_app.rs:110 | control_button_list | 未读取字段 | LawnApp.h:85 ButtonList						mControlButtonList; |
| src/lawn/lawn_app.rs:111 | created_image_list | 未读取字段 | LawnApp.h:86 ImageList						mCreatedImageList; |
| src/lawn/lawn_app.rs:159 | m_games_played | 未读取字段 | LawnApp.cpp:176 [mGamesPlayed] mGamesPlayed = 0; |
| src/lawn/lawn_app.rs:160 | m_max_executions | 未读取字段 | LawnApp.cpp:177 [mMaxExecutions] mMaxExecutions = 0; |
| src/lawn/lawn_app.rs:161 | m_max_plays | 未读取字段 | LawnApp.cpp:178 [mMaxPlays] mMaxPlays = 0; |
| src/lawn/lawn_app.rs:162 | m_max_time | 未读取字段 | LawnApp.cpp:179 [mMaxTime] mMaxTime = 0; |
| src/lawn/lawn_app.rs:163 | m_play_time_active_session | 未读取字段 | LawnApp.cpp:1220 [mPlayTimeActiveSession] mPlayTimeActiveSession = 0; |
| src/lawn/lawn_app.rs:164 | m_play_time_inactive_session | 未读取字段 | LawnApp.cpp:1221 [mPlayTimeInactiveSession] mPlayTimeInactiveSession = 0; |
| src/lawn/lawn_app.rs:169 | m_tutorial_state | 未读取字段 | ConstEnums.h:1187 [TutorialState] enum TutorialState : int32_t |
| src/lawn/lawn_app.rs:172 | m_refer_id | 未读取字段 | LawnApp.h:87 [mReferId] std::string						mReferId; |
| src/lawn/lawn_app.rs:173 | m_register_link | 未读取字段 | LawnApp.h:88 [mRegisterLink] std::string						mRegisterLink; |
| src/lawn/lawn_app.rs:174 | m_mod | 未读取字段 | LawnApp.h:89 [mMod] std::string						mMod; |
| src/lawn/lawn_app.rs:175 | m_register_resources_loaded | 未读取字段 | LawnApp.cpp:182 [mRegisterResourcesLoaded] mRegisterResourcesLoaded = false; |
| src/lawn/lawn_app.rs:183 | m_loading_zombies_thread_completed | 未读取字段 | LawnApp.cpp:175 [mLoadingZombiesThreadCompleted] mLoadingZombiesThreadCompleted = true; |
| src/lawn/lawn_app.rs:186 | m_session_id | 未读取字段 | LawnApp.cpp:1219 [mSessionID] mSessionID = time(0); |
| src/lawn/lawn_mower.rs:11 | app | 未读取字段 | Lawn/LawnMower.cpp:32 mApp = (LawnApp*)gSexyAppBase; |
| src/lawn/lawn_mower.rs:12 | board | 未读取字段 | Lawn/LawnMower.cpp:35 mBoard = mApp->mBoard; |
| src/lawn/lawn_mower.rs:26 | mowing | 未读取字段 | Lawn/Board.cpp:336 [mMowerState] if (aLawnMower->mMowerState != LawnMowerState::MOWER_TRIGGERED && aLawnMower->mMowerState  |
| src/lawn/plant.rs:5791 | seed_type | 未读取字段 | Lawn/Plant.cpp:47 constinit const PlantDefinition gPlantDefs[SeedType::NUM_SEED_TYPES] = { |
| src/lawn/plant.rs:5792 | plant_image | 未读取字段 | Lawn/Plant.cpp:48 { .mSeedType = SeedType::SEED_PEASHOOTER,        .mPlantImage = nullptr, .mReanimationType |
| src/lawn/plant.rs:5794 | packet_index | 未读取字段 | Lawn/Plant.cpp:48 { .mSeedType = SeedType::SEED_PEASHOOTER,        .mPlantImage = nullptr, .mReanimationType |
| src/lawn/projectile.rs:100 | target_x | 未读取字段 | Lawn/Plant.cpp:123 mTargetX = -1; |
| src/lawn/projectile.rs:101 | target_y | 未读取字段 | Lawn/Plant.cpp:124 mTargetY = -1; |
| src/lawn/projectile.rs:112 | shadow | 未读取字段 | SexyAppFramework/misc/ResourceManager.cpp:441 aRes->mShadow = theElement.mAttributes.find("shadow")!=theElement.mAttributes.end(); |
| src/lawn/seed_packet.rs:24 | can_afford | 未读取字段 | Lawn/Board.cpp:2013 [mSeedType] return aPottedPlant->mSeedType; |
| src/lawn/seed_packet.rs:30 | offset_y | 未读取字段 | Lawn/System/Zombatar.h:132 int mOffsetY; |
| src/lawn/widget/achievements_screen.rs:42 | description | 未读取字段 | Lawn/Widget/AchievementsScreen.cpp:161 std::string aDesc = mApp->GetString(gAchievementList[i].description, gAchievementList[i].d |
| src/lawn/widget/award_screen.rs:14 | id | 未读取字段 | Lawn/Widget/AwardScreen.cpp:62 aAchievementItem.mId = i; |
| src/lawn/widget/message_widget.rs:19 | app | 未读取字段 | Lawn/MessageWidget.cpp:35 mApp = theApp; |
| src/lawn/widget/message_widget.rs:29 | text_reanim_count | 未读取字段 | Lawn/MessageWidget.cpp:44 mTextReanimCount = 0; |
| src/lawn/widget/message_widget.rs:30 | text_reanim_byte_offset | 未读取字段 | Lawn/MessageWidget.cpp:48 mTextReanimByteOffset[i] = 0; |
| src/lawn/widget/title_screen.rs:70 | need_register | 未读取字段 | Lawn/Widget/TitleScreen.cpp:48 mNeedRegister = false; |
| src/lawn/widget/title_screen.rs:71 | register_clicked | 未读取字段 | Lawn/Widget/TitleScreen.cpp:49 mRegisterClicked = false; |
| src/lawn/widget/title_screen.rs:72 | need_show_register_box | 未读取字段 | Lawn/Widget/TitleScreen.cpp:50 mNeedShowRegisterBox = false; |
| src/lawn/zombie.rs:12295 | zombie_type | 未读取字段 | Lawn/Zombie.cpp:80 { .mZombieType = ZOMBIE_NORMAL, .mReanimationType = REANIM_ZOMBIE, .mZombieValue = 1, .mSt |
| src/framework/graphics/image_font.rs:238 | list | 未使用变量 | cpp/src/SexyAppFramework/graphics/ImageFont.h:92 in (文件级) |
| src/framework/graphics/image_font.rs:1446 | colorize_images | 未使用变量 | cpp/src/SexyAppFramework/graphics/ImageFont.cpp:1486 in ImageFont::DrawStringEx |
| src/framework/graphics/memory_image.rs:206 | linear_filter | 未使用变量 | cpp/src/SexyAppFramework/graphics/MemoryImage.cpp:1364 in MemoryImage::Blt |
| src/framework/paklib/mod.rs:470 | record | 未使用变量 | cpp/src/SexyAppFramework/paklib/PakInterface.cpp:180 in Sexy::FromLE64 |
| src/framework/paklib/mod.rs:684 | c | 未使用变量 | cpp/src/SexyAppFramework/paklib/PakInterface.cpp:3 in (文件级) |
| src/framework/widget/slider.rs:176 | x | 未使用变量 | cpp/src/SexyAppFramework/widget/Slider.cpp:103 in Slider::MouseDown |
| src/framework/widget/slider.rs:176 | y | 未使用变量 | cpp/src/SexyAppFramework/widget/Slider.cpp:103 in Slider::MouseDown |
| src/framework/widget/widget.rs:52 | g | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:121 in Widget::DrawOverlay |
| src/framework/widget/widget.rs:52 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:53 | g | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:121 in Widget::DrawOverlay |
| src/framework/widget/widget.rs:53 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:56 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:59 | key | 未使用变量 | cpp/src/SexyAppFramework/widget/WidgetManager.cpp#WidgetManager::KeyDown |
| src/framework/widget/widget.rs:59 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:60 | key | 未使用变量 | cpp/src/SexyAppFramework/widget/WidgetManager.cpp#WidgetManager::KeyUp |
| src/framework/widget/widget.rs:60 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:61 | c | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:3 in (文件级) |
| src/framework/widget/widget.rs:61 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:64 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:64 | x | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:95 in Widget::IsPointVisible |
| src/framework/widget/widget.rs:64 | y | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:95 in Widget::IsPointVisible |
| src/framework/widget/widget.rs:65 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:65 | x | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:95 in Widget::IsPointVisible |
| src/framework/widget/widget.rs:65 | y | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:95 in Widget::IsPointVisible |
| src/framework/widget/widget.rs:66 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:66 | x | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:95 in Widget::IsPointVisible |
| src/framework/widget/widget.rs:66 | y | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:95 in Widget::IsPointVisible |
| src/framework/widget/widget.rs:67 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:67 | x | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:95 in Widget::IsPointVisible |
| src/framework/widget/widget.rs:67 | y | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:95 in Widget::IsPointVisible |
| src/framework/widget/widget.rs:68 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:69 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:70 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:73 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/framework/widget/widget.rs:74 | widget | 未使用变量 | cpp/src/SexyAppFramework/widget/Widget.cpp:36 in (文件级) |
| src/lawn/challenge.rs:7274 | a_scale | 未使用变量 | cpp/src/Lawn/Challenge.cpp:5085 in Challenge::UpdateRain |
| src/lawn/coin.rs:1638 | a_angle | 未使用变量 | cpp/src/Lawn/Coin.cpp:1012 in Coin::FanOutCoins |
| src/lawn/cursor_object.rs:307 | seed_type | 未使用变量 | cpp/src/Lawn/CursorObject.cpp:33 in CursorObject::CursorObject |
| src/lawn/lawn_app.rs:2800 | a_pos_x | 未使用变量 | cpp/src/LawnApp.cpp:2966 in LawnApp::DrawCrazyDave |
| src/lawn/lawn_app.rs:2801 | a_pos_y | 未使用变量 | cpp/src/LawnApp.cpp:2967 in LawnApp::DrawCrazyDave |
| src/lawn/lawn_app.rs:2825 | a_message_name | 未使用变量 | cpp/src/LawnApp.cpp:2433 in LawnApp::AdvanceCrazyDaveText |
| src/lawn/plant.rs:1673 | a_damage_range_flags | 未使用变量 | cpp/src/Lawn/Plant.cpp:674 in Plant::DoRowAreaDamage |
| src/lawn/plant.rs:3309 | a_scale_y | 未使用变量 | cpp/src/Lawn/Plant.cpp:2749 in Plant::UpdateReanim |
| src/lawn/plant.rs:5407 | board | 未使用变量 | cpp/src/Lawn/Plant.cpp:528 in Board::MakeRenderOrder |
| src/lawn/plant.rs:5685 | a_pos_x | 未使用变量 | cpp/src/Lawn/Plant.cpp:539 in Plant::SetSleeping |
| src/lawn/plant.rs:5686 | a_pos_y | 未使用变量 | cpp/src/Lawn/Plant.cpp:540 in Plant::SetSleeping |
| src/lawn/projectile.rs:252 | a_old_row | 未使用变量 | cpp/src/Lawn/Projectile.cpp:747 in Projectile::UpdateMotion |
| src/lawn/projectile.rs:253 | a_old_y | 未使用变量 | cpp/src/Lawn/Projectile.cpp:748 in Projectile::UpdateMotion |
| src/lawn/seed_packet.rs:802 | a_scale | 未使用变量 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:803 | a_draw_seed_in_middle | 未使用变量 | cpp/src/Lawn/SeedPacket.cpp:324 in Plant::IsUpgrade |
| src/lawn/widget/challenge_screen.rs:270 | a_def | 未使用变量 | cpp/src/Lawn/Widget/ChallengeScreen.cpp:224 in ChallengeScreen::ChallengeScreen |
| src/lawn/widget/credit_screen.rs:1969 | a_frame_factor | 未使用变量 | cpp/src/Lawn/Widget/CreditScreen.cpp:854 in CreditScreen::Draw |
| src/lawn/widget/seed_chooser_screen.rs:1701 | x | 未使用变量 | cpp/src/Lawn/Widget/SeedChooserScreen.cpp:284 in SeedChooserScreen::GetSeedPositionInChooser |
| src/lawn/widget/seed_chooser_screen.rs:1701 | y | 未使用变量 | cpp/src/Lawn/Widget/SeedChooserScreen.cpp:284 in SeedChooserScreen::GetSeedPositionInChooser |
| src/lawn/zen_garden.rs:1486 | g | 未使用变量 | cpp/src/Lawn/ZenGarden.cpp:118 in ZenGarden::DrawPottedPlantIcon |
| src/todlib/reanimator.rs:1621 | app | 未使用变量 | cpp/src/PvzpLib/Reanimator.cpp:1172 in ReanimationHolder::AllocReanimation |
| src/todlib/tod_particle.rs:3595 | i | 未使用变量 | cpp/src/PvzpLib/PvzpParticle.cpp:157 in (文件级) |
| src/lawn/challenge.rs:7280 | a_scale | 未读赋值 | cpp/src/Lawn/Challenge.cpp:5085 in Challenge::UpdateRain |
| src/lawn/lawn_app.rs:2803 | a_pos_x | 未读赋值 | cpp/src/LawnApp.cpp:2966 in LawnApp::DrawCrazyDave |
| src/lawn/lawn_app.rs:2804 | a_pos_y | 未读赋值 | cpp/src/LawnApp.cpp:2967 in LawnApp::DrawCrazyDave |
| src/lawn/lawn_app.rs:2806 | a_pos_x | 未读赋值 | cpp/src/LawnApp.cpp:2966 in LawnApp::DrawCrazyDave |
| src/lawn/lawn_app.rs:2807 | a_pos_y | 未读赋值 | cpp/src/LawnApp.cpp:2967 in LawnApp::DrawCrazyDave |
| src/lawn/lawn_app.rs:2812 | a_bubble_text | 未读赋值 | cpp/src/LawnApp.cpp:2981 in LawnApp::DrawCrazyDave |
| src/lawn/plant.rs:2205 | a_offset_x_start | 未读赋值 | cpp/src/Lawn/Plant.cpp:2808 in Plant::UpdateReanim |
| src/lawn/plant.rs:2206 | a_offset_x_end | 未读赋值 | cpp/src/Lawn/Plant.cpp:2808 in Plant::UpdateReanim |
| src/lawn/plant.rs:2207 | a_offset_y_start | 未读赋值 | cpp/src/Lawn/Plant.cpp:2809 in Plant::UpdateReanim |
| src/lawn/plant.rs:2208 | a_offset_y_end | 未读赋值 | cpp/src/Lawn/Plant.cpp:2809 in Plant::UpdateReanim |
| src/lawn/plant.rs:2209 | a_scale_start | 未读赋值 | cpp/src/Lawn/Plant.cpp:2810 in Plant::UpdateReanim |
| src/lawn/plant.rs:2210 | a_scale_end | 未读赋值 | cpp/src/Lawn/Plant.cpp:2810 in Plant::UpdateReanim |
| src/lawn/plant.rs:3321 | a_scale_y | 未读赋值 | cpp/src/Lawn/Plant.cpp:2749 in Plant::UpdateReanim |
| src/lawn/plant.rs:3328 | a_scale_x | 未读赋值 | cpp/src/Lawn/Plant.cpp:2749 in Plant::UpdateReanim |
| src/lawn/plant.rs:3335 | a_scale_x | 未读赋值 | cpp/src/Lawn/Plant.cpp:2749 in Plant::UpdateReanim |
| src/lawn/plant.rs:3336 | a_scale_y | 未读赋值 | cpp/src/Lawn/Plant.cpp:2749 in Plant::UpdateReanim |
| src/lawn/seed_packet.rs:808 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:813 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:818 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:827 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:845 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:850 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:855 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:860 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:865 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:870 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:875 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:880 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:885 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:890 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:895 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:900 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:905 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:910 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:915 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:920 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:928 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:933 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:938 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:943 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:948 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:953 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:958 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:968 | a_draw_seed_in_middle | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:324 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:987 | a_scale | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:323 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:991 | a_offset_x | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:325 in Plant::IsUpgrade |
| src/lawn/seed_packet.rs:992 | a_offset_y | 未读赋值 | cpp/src/Lawn/SeedPacket.cpp:326 in Plant::IsUpgrade |
| src/lawn/widget/cheat_dialog.rs:207 | need_space | 未读赋值 | cpp/src/SexyAppFramework/widget/Dialog.cpp#Dialog::GetPreferredHeight |
| src/lawn/widget/continue_dialog.rs:252 | need_space | 未读赋值 | cpp/src/SexyAppFramework/widget/Dialog.cpp#Dialog::GetPreferredHeight |
| src/lawn/widget/message_widget.rs:132 | a_outline_color | 未读赋值 | cpp/src/Lawn/MessageWidget.cpp:375 in MessageWidget::Draw |
| src/lawn/widget/message_widget.rs:253 | a_outline_color | 未读赋值 | cpp/src/Lawn/MessageWidget.cpp:375 in MessageWidget::Draw |
| src/lawn/widget/message_widget.rs:258 | a_outline_color | 未读赋值 | cpp/src/Lawn/MessageWidget.cpp:375 in MessageWidget::Draw |
| src/lawn/widget/new_user_dialog.rs:236 | need_space | 未读赋值 | cpp/src/SexyAppFramework/widget/Dialog.cpp#Dialog::GetPreferredHeight |
| src/lawn/widget/title_screen.rs:259 | an_alpha | 未读赋值 | cpp/src/Lawn/Widget/TitleScreen.cpp:114 in TitleScreen::Draw |
| src/lawn/widget/user_dialog.rs:418 | need_space | 未读赋值 | cpp/src/SexyAppFramework/widget/Dialog.cpp#Dialog::GetPreferredHeight |
| src/lawn/zombie.rs:4310 | a_speed | 未读赋值 | cpp/src/Lawn/Zombie.cpp:3934 in Zombie::ZombieTargetLeadX |
| src/todlib/tod_particle.rs:2415 | a_pos_x | 未读赋值 | cpp/src/PvzpLib/PvzpParticle.cpp:381 in Sexy::Rand |
| src/todlib/tod_particle.rs:2416 | a_pos_y | 未读赋值 | cpp/src/PvzpLib/PvzpParticle.cpp:381 in Sexy::Rand |
| src/lawn/board.rs:3846 | a_z_pos | 多余 mut | cpp/src/Lawn/Board.cpp:6264 in Board::DrawGameObjects |
| src/lawn/board.rs:4296 | a_pos_x | 多余 mut | cpp/src/Lawn/Board.cpp:5423 in Board::UpdateIce |
| src/lawn/board.rs:4297 | a_pos_y | 多余 mut | cpp/src/Lawn/Board.cpp:5424 in Board::UpdateIce |
| src/lawn/board.rs:4404 | a_pos_y | 多余 mut | cpp/src/Lawn/Board.cpp:5424 in Board::UpdateIce |
| src/lawn/lawn_app.rs:1610 | a_dialog | 多余 mut | cpp/src/LawnApp.cpp:688 in LawnApp::DoConfirmBackToMain |
| src/lawn/lawn_mower.rs:211 | a_speed | 多余 mut | cpp/src/Lawn/LawnMower.cpp:241 in LawnMower::Update |
| src/lawn/lawn_mower.rs:413 | a_scale_x | 多余 mut | cpp/src/Lawn/LawnMower.cpp:286 in LawnMower::Draw |
| src/lawn/plant.rs:1099 | a_origin_x | 多余 mut | cpp/src/Lawn/Plant.cpp:4551 in Plant::Fire |
| src/lawn/plant.rs:1989 | a_color_override | 多余 mut | cpp/src/Lawn/Plant.cpp:2650 in Plant::UpdateReanimColor |
| src/lawn/plant.rs:2299 | a_pos_x | 多余 mut | cpp/src/Lawn/Plant.cpp:539 in Plant::SetSleeping |
| src/lawn/plant.rs:2475 | a_cel_row | 多余 mut | cpp/src/Lawn/Plant.cpp:3701 in Plant::DrawMagnetItems |
| src/lawn/plant.rs:2477 | a_image | 多余 mut | cpp/src/Lawn/Plant.cpp:3702 in Plant::DrawMagnetItems |
| src/lawn/system/reanimation_lawn.rs:290 | a_memory_image | 多余 mut | cpp/src/Lawn/System/ReanimationLawn.cpp:211 in ReanimatorCache::MakeCachedPlantFrame |
| src/lawn/system/reanimation_lawn.rs:402 | a_memory_image | 多余 mut | cpp/src/Lawn/System/ReanimationLawn.cpp:211 in ReanimatorCache::MakeCachedPlantFrame |
| src/lawn/system/reanimation_lawn.rs:465 | a_memory_image | 多余 mut | cpp/src/Lawn/System/ReanimationLawn.cpp:211 in ReanimatorCache::MakeCachedPlantFrame |
| src/lawn/system/reanimation_lawn.rs:483 | a_pos_x | 多余 mut | cpp/src/Lawn/System/ReanimationLawn.cpp:275 in ReanimatorCache::MakeCachedZombieFrame |
| src/lawn/zombie.rs:9648 | coin_type | 多余 mut | cpp/src/Lawn/Zombie.cpp:3245 in std::min |
| src/todlib/reanimator.rs:1644 | a_attach_reanim | 多余 mut | cpp/src/PvzpLib/Reanimator.cpp:1443 in Reanimation::UpdateAttacherTrack |
| src/todlib/reanimator.rs:1655 | app | 多余 mut | cpp/src/PvzpLib/Reanimator.cpp:1172 in ReanimationHolder::AllocReanimation |
| src/todlib/tod_particle.rs:1475 | a_src_rect | 多余 mut | cpp/src/PvzpLib/PvzpParticle.cpp:969 in std::min |

## 2. C++ 仅有定义/声明

共 23 项。

| Rust 位置 | 符号 | 类别 | C++ 使用处（原来在何处被使用） |
|---|---|---|---|
| src/framework/widget/mod.rs:26 | set_invis_if_no_scroll | 未使用函数 | SexyAppFramework/widget/ScrollbarWidget.cpp:64[def]; SexyAppFramework/widget/ScrollbarWidget.h:83[decl] |
| src/framework/widget/mod.rs:32 | get_selection | 未使用函数 | SexyAppFramework/widget/TextWidget.cpp:407[def]; SexyAppFramework/widget/TextWidget.h:73[decl] |
| src/framework/sound/sound_manager.rs:28 | release_sounds | 未使用方法 | SexyAppFramework/sound/SDLSoundManager.cpp:366[def]; SexyAppFramework/sound/SDLSoundManager.h:75[decl]; SexyAppFramework |
| src/framework/sound/sound_manager.rs:30 | release_channels | 未使用方法 | SexyAppFramework/sound/SDLSoundManager.cpp:378[def]; SexyAppFramework/sound/SDLSoundManager.h:76[decl]; SexyAppFramework |
| src/lawn/board.rs:6204 | get_grave_stone_count | 未使用方法 | Lawn/Board.cpp:9044[def]; Lawn/Board.h:334[decl] |
| src/lawn/board.rs:6226 | get_winning_zombie | 未使用方法 | Lawn/Board.cpp:9474[def]; Lawn/Board.h:469[decl] |
| src/lawn/board.rs:13085 | number_zombies_in_wave | 未使用方法 | Lawn/Board.cpp:9505[def]; Lawn/Board.h:342[decl] |
| src/lawn/challenge.rs:1522 | init_zombie_waves_from_list | 未使用方法 | Lawn/Challenge.cpp:2532[def]; Lawn/Challenge.h:126[decl] |
| src/lawn/challenge.rs:3658 | get_portal_left_right | 未使用方法 | Lawn/Challenge.cpp:3426[def]; Lawn/Challenge.h:248[decl] |
| src/lawn/challenge.rs:4912 | scary_potter_fill_column_with_plant | 未使用方法 | Lawn/Challenge.cpp:3718[def]; Lawn/Challenge.h:178[decl] |
| src/lawn/challenge.rs:6698 | beghouled_cancel_match_flashing | 未使用方法 | Lawn/Challenge.cpp:1505[def]; Lawn/Challenge.h:220[decl] |
| src/lawn/widget/title_screen.rs:181 | set_registered | 未使用方法 | Lawn/Widget/TitleScreen.h:95[decl] |
| src/lawn/lawn_app.rs:1629 | do_dialog_delay | 未使用关联项 | LawnApp.cpp:812[def]; LawnApp.h:205[decl] |
| src/lawn/lawn_app.rs:2968 | need_register | 未使用关联项 | LawnApp.cpp:3335[def]; LawnApp.h:216[decl] |
| src/lawn/lawn_app.rs:3038 | do_confirm_sell_dialog | 未使用关联项 | LawnApp.cpp:1098[def]; LawnApp.h:177[decl] |
| src/lawn/lawn_app.rs:3081 | do_register_error | 未使用关联项 | LawnApp.cpp:3310[def]; LawnApp.h:197[decl] |
| src/lawn/lawn_app.rs:3084 | do_need_register_dialog | 未使用关联项 | LawnApp.cpp:3320[def]; LawnApp.h:200[decl] |
| src/lawn/lawn_app.rs:3422 | do_high_score_dialog | 未使用关联项 | LawnApp.cpp:3300[def]; LawnApp.h:191[decl] |
| src/lawn/zombie.rs:4692 | is_standing_on_spikeweed | 未使用关联项 | Lawn/Zombie.cpp:4219[def]; Lawn/Zombie.h:215[decl] |
| src/lawn/zombie.rs:4730 | is_squash_target | 未使用关联项 | Lawn/Zombie.cpp:8441[def]; Lawn/Zombie.h:393[decl] |
| src/lawn/zombie.rs:4889 | convert_to_normal_zombie | 未使用关联项 | Lawn/Zombie.cpp:6697[def]; Lawn/Zombie.h:244[decl] |
| src/lawn/zombie.rs:6847 | zombie_type_can_go_on_high_ground | 未使用关联项 | Lawn/Board.cpp:2535[def]; Lawn/Zombie.cpp:8293[def]; Lawn/Zombie.h:387[decl] |
| src/lawn/zombie.rs:8999 | draw_zombie_with_parts | 未使用关联项 | Lawn/Zombie.cpp:5248[def] |

## 3. Rust 自创或拆分（C++ 无独立对应）

共 36 项。

| Rust 位置 | 符号 | 类别 | C++ 使用处（原来在何处被使用） |
|---|---|---|---|
| src/lawn/board.rs:138 | y_to_row | 未使用函数 | - |
| src/lawn/board.rs:143 | x_to_col | 未使用函数 | - |
| src/lawn/cursor_object.rs:476 | plant_draw_seed_type | 未使用函数 | - |
| src/lawn/widget/message_widget.rs:426 | message_label_to_str | 未使用函数 | - |
| src/lawn/widget/message_widget.rs:432 | setup_default_font | 未使用函数 | - |
| src/todlib/reanim_loader.rs:204 | get_reanim_font_name | 未使用函数 | - |
| src/todlib/reanim_loader.rs:217 | reanim_image_name_index | 未使用函数 | - |
| src/todlib/reanim_loader.rs:360 | parse_reanim_xml_for_test | 未使用函数 | - |
| src/framework/desc_parser.rs:41 | as_list | 未使用方法 | - |
| src/framework/desc_parser.rs:78 | set_cmd_sep | 未使用方法 | - |
| src/framework/sound/sdl_sound_manager.rs:162 | rehup_all_volumes | 未使用方法 | - |
| src/framework/sound/sound_manager.rs:4 | load_sound_by_id | 未使用方法 | - |
| src/lawn/board.rs:1328 | check_collisions | 未使用方法 | - |
| src/lawn/board.rs:1466 | find_zombie_in_row | 未使用方法 | - |
| src/lawn/board.rs:2202 | draw_seed_bank | 未使用方法 | - |
| src/lawn/board.rs:3567 | check_game_state | 未使用方法 | - |
| src/lawn/board.rs:4204 | draw_ui_text | 未使用方法 | - |
| src/lawn/board.rs:4421 | get_board_image | 未使用方法 | - |
| src/lawn/board.rs:6973 | get_row_picking_item | 未使用方法 | - |
| src/lawn/board.rs:7157 | get_zombie_points_for_wave | 未使用方法 | - |
| src/lawn/board.rs:7786 | count_lawn_mowers_remaining | 未使用方法 | - |
| src/lawn/cursor_object.rs:240 | get_cursor_type_for_app | 未使用方法 | - |
| src/lawn/cutscene.rs:1677 | can_get_packet_upgrade_index | 未使用方法 | - |
| src/lawn/grid_item.rs:822 | get_grid_image | 未使用方法 | - |
| src/lawn/lawn_mower.rs:174 | start_mowing | 未使用方法 | - |
| src/lawn/seed_packet.rs:401 | set_countdown | 未使用方法 | - |
| src/lawn/zombie.rs:868 | setup_health_for_type | 未使用方法 | - |
| src/lawn/lawn_app.rs:592 | finish_zen_garden_tutorial | 未使用关联项 | - |
| src/lawn/plant.rs:3425 | get_health_for_type | 未使用关联项 | - |
| src/lawn/plant.rs:3437 | get_launch_rate_for_type | 未使用关联项 | - |
| src/lawn/plant.rs:3449 | get_frame_length_for_type | 未使用关联项 | - |
| src/lawn/plant.rs:3452 | get_num_frames_for_type | 未使用关联项 | - |
| src/lawn/zombie.rs:868 | setup_health_for_type | 未使用关联项 | - |
| src/lawn/zombie.rs:2749 | plant_col_below | 未使用关联项 | - |
| src/lawn/projectile.rs:49 | ProjectileFlags | 未使用类型 | - |
| src/lawn/widget/title_screen.rs:44 | TitleScreenButtonId | 未使用类型 | - |

## 4. 疑似改名候选

共 16 项。

| Rust 位置 | 符号 | 类别 | C++ 使用处（原来在何处被使用） |
|---|---|---|---|
| src/framework/desc_parser.rs:28 | is_list | 未使用方法 | 候选: DialogList, mDialogList, theRenderList, list, List, LIST |
| src/framework/desc_parser.rs:72 | error_message | 未使用方法 | 候选: USER_ERROR_MESSAGE, PvzpErrorMessageBox, error_message, WavpackGetErrorMessage |
| src/framework/desc_parser.rs:80 | current_line_num | 未使用方法 | 候选: GetCurrentLineNum, mCurrentLineNum |
| src/framework/sound/sound_manager.rs:11 | stop_sound | 未使用方法 | 候选: StopZombieSound, CrazyDaveStopSound, aStopSound |
| src/framework/sound/sound_manager.rs:17 | num_sounds | 未使用方法 | 候选: GetNumSounds |
| src/lawn/board.rs:1445 | find_plant_at | 未使用方法 | 候选: FindUmbrellaPlant, FindCollisionTargetPlant, FindPlantTarget |
| src/lawn/board.rs:3623 | level_complete | 未使用方法 | 候选: DIALOG_LEVEL_COMPLETE, mLevelComplete, CompleteEndLevelSequenceForSaving, BOARD_FIELD_LEVEL_COMPLETE |
| src/lawn/board.rs:4147 | draw_background | 未使用方法 | 候选: DrawMainBackground |
| src/lawn/board.rs:6978 | is_zombie_allowed | 未使用方法 | 候选: mZombieAllowed, gZombieAllowedLevels, ZombieAllowedLevels, BOARD_FIELD_ZOMBIE_ALLOWED |
| src/lawn/board.rs:7818 | is_row_lawn_mowered | 未使用方法 | 候选: mWaveRowGotLawnMowered, BOARD_FIELD_WAVE_ROW_GOT_LAWN_MOWERED |
| src/lawn/cursor_object.rs:73 | update_position | 未使用方法 | 候选: UpdateMousePosition, UpdateZombiePosition, updatePositionSlider, updatePositionEffect, adjustPositionAndUpdate |
| src/lawn/cutscene.rs:103 | init_app | 未使用方法 | 候选: PvzpAssertInitForApp, FilterEffectInitForApp |
| src/lawn/lawn_mower.rs:270 | get_mower_rect | 未使用方法 | 候选: GetLawnMowerAttackRect |
| src/lawn/lawn_app.rs:2268 | get_close_request | 未使用关联项 | 候选: LawnGetCloseRequest |
| src/lawn/lawn_app.rs:2271 | has_used_cheat_keys | 未使用关联项 | 候选: mHasUsedCheatKeys, LawnHasUsedCheatKeys |
| src/lawn/lawn_common.rs:17 | LAWN_EDIT_WIDGET_COLORS | 未使用常量 / 静态 | 候选: gLawnEditWidgetColors |

## 5. 枚举变体未接线（非 ResourceId）

共 17 项。

| Rust 位置 | 符号 | 类别 | C++ 使用处（原来在何处被使用） |
|---|---|---|---|
| src/lawn/lawn_app.rs:34 | ZenGarden | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 ZenGarden 属 GameScenes，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/lawn_app.rs:35 | Store | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 Store 属 GameScenes，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/lawn_app.rs:36 | Credits | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 Credits 属 GameScenes，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/lawn_app.rs:39 | SeedChooser | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 SeedChooser 属 GameScenes，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/lawn_app.rs:40 | GameSelector | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 GameSelector 属 GameScenes，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/projectile.rs:38 | Bee | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 Bee 属 ProjectileMotion，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/projectile.rs:39 | BeeBackwards | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 BeeBackwards 属 ProjectileMotion，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/projectile.rs:70 | Stinky | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 Stinky 属 ProjectileType，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/projectile.rs:75 | Guod | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 Guod 属 ProjectileType，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/projectile.rs:76 | Cobcannon | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 Cobcannon 属 ProjectileType，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/widget/achievements_screen.rs:15 | NovelPeasPrize | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 NovelPeasPrize 属 AchievementId，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/widget/achievements_screen.rs:18 | Spudow | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 Spudow 属 AchievementId，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/widget/achievements_screen.rs:19 | Explodonator | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 Explodonator 属 AchievementId，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/widget/achievements_screen.rs:22 | RollSomeHeads | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 RollSomeHeads 属 AchievementId，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/widget/achievements_screen.rs:25 | PennyPincher | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 PennyPincher 属 AchievementId，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/widget/achievements_screen.rs:27 | PopcornParty | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 PopcornParty 属 AchievementId，该枚举在 Rust 尚未接线（游戏路径未触达） |
| src/lawn/widget/achievements_screen.rs:32 | ToweringWisdom | 未构造枚举变体 | C++ ConstEnums.h 有对应枚举；变体 ToweringWisdom 属 AchievementId，该枚举在 Rust 尚未接线（游戏路径未触达） |

## 6. Rust 自有符号

共 130 项。

| Rust 位置 | 符号 | 类别 | C++ 使用处（原来在何处被使用） |
|---|---|---|---|
| src/lawn/board.rs:319 | m_wave_countdown | 未读取字段 | - |
| src/lawn/board.rs:320 | m_flag_countdown | 未读取字段 | - |
| src/lawn/lawn_app.rs:118 | m_num_levels | 未读取字段 | - |
| src/lawn/lawn_app.rs:168 | m_dialog_id | 未读取字段 | - |
| src/lawn/projectile.rs:108 | projectile_flags | 未读取字段 | - |
| src/framework/desc_parser.rs:164 | child_list | 未使用变量 | - |
| src/framework/graphics/graphics.rs:1556 | mc | 未使用变量 | - |
| src/framework/graphics/image_font.rs:868 | layer | 未使用变量 | - |
| src/framework/graphics/image_font.rs:884 | layer | 未使用变量 | - |
| src/framework/graphics/image_font.rs:1082 | image_ref | 未使用变量 | - |
| src/framework/graphics/image_font.rs:1083 | offset_val | 未使用变量 | - |
| src/framework/graphics/image_font.rs:1084 | spacing_val | 未使用变量 | - |
| src/framework/resource_manager.rs:492 | section_name | 未使用变量 | - |
| src/framework/sound/music_interface.rs:23 | song_id | 未使用变量 | - |
| src/framework/sound/music_interface.rs:27 | song_id | 未使用变量 | - |
| src/framework/widget/button_widget.rs:352 | the_btn | 未使用变量 | - |
| src/framework/widget/button_widget.rs:361 | the_btn | 未使用变量 | - |
| src/framework/widget/button_widget.rs:361 | the_click | 未使用变量 | - |
| src/framework/widget/edit_widget.rs:630 | display_str | 未使用变量 | - |
| src/framework/widget/widget.rs:59 | wm | 未使用变量 | - |
| src/framework/widget/widget.rs:64 | click | 未使用变量 | - |
| src/framework/widget/widget.rs:65 | click | 未使用变量 | - |
| src/lawn/board.rs:1184 | grid_x | 未使用变量 | - |
| src/lawn/plant.rs:5054 | a_ladder_grid_x | 未使用变量 | - |
| src/lawn/plant.rs:5054 | a_ladder_grid_y | 未使用变量 | - |
| src/lawn/projectile.rs:1083 | proj_type | 未使用变量 | - |
| src/lawn/projectile.rs:1211 | my_width | 未使用变量 | - |
| src/lawn/seed_packet.rs:705 | use_current_cost | 未使用变量 | - |
| src/todlib/trail.rs:230 | triangle_count | 未使用变量 | - |
| src/lawn/system/save_game.rs:1877 | field_data | 未读赋值 | - |
| src/lawn/board.rs:6472 | a_to_die | 多余 mut | - |
| src/lawn/challenge.rs:5830 | picks_count | 多余 mut | - |
| src/lawn/projectile.rs:868 | get_projectile_image | 多余 mut | - |
| src/lawn/projectile.rs:1212 | splash_rect | 多余 mut | - |
| src/todlib/effect_system.rs:134 | a_system | 多余 mut | - |
| src/todlib/reanimator.rs:447 | a_atlas_memory | 多余 mut | - |
| src/framework/desc_parser.rs:10 | std::fs | 未使用导入 | - |
| src/framework/graphics/gl_image.rs:17 | PixelFormat | 未使用导入 | - |
| src/framework/graphics/gl_image.rs:17 | TextureDataPiece | 未使用导入 | - |
| src/framework/graphics/image.rs:10 | crate::framework::graphics::graphics::Graphics | 未使用导入 | - |
| src/framework/graphics/memory_image.rs:14 | Span | 未使用导入 | - |
| src/framework/graphics/memory_image.rs:15 | crate::framework::point::Point | 未使用导入 | - |
| src/framework/graphics/quantize.rs:9 | crate::framework::common::uchar | 未使用导入 | - |
| src/framework/paklib/mod.rs:22 | Write | 未使用导入 | - |
| src/framework/resource_manager.rs:8 | LinkedList | 未使用导入 | - |
| src/framework/resource_manager.rs:9 | std::ptr | 未使用导入 | - |
| src/framework/resource_manager.rs:13 | crate::framework::graphics::gl_image::GLImage | 未使用导入 | - |
| src/framework/resource_manager.rs:16 | SharedImage | 未使用导入 | - |
| src/framework/sexy_app_base.rs:11 | sdl2::keyboard::Keycode | 未使用导入 | - |
| src/framework/sexy_app_base.rs:20 | crate::framework::graphics::image::Image | 未使用导入 | - |
| src/framework/widget/dialog.rs:11 | crate::framework::widget::button_listener::ButtonListener | 未使用导入 | - |
| src/framework/widget/scrollbar_widget.rs:8 | crate::framework::rect::Rect | 未使用导入 | - |
| src/framework/widget/text_widget.rs:17 | crate::framework::rect::Rect | 未使用导入 | - |
| src/framework/widget/widget.rs:18 | crate::framework::widget::widget_container::WidgetContainer | 未使用导入 | - |
| src/framework/widget/widget_container.rs:9 | std::cmp | 未使用导入 | - |
| src/lawn/board.rs:8 | RandFloat | 未使用导入 | - |
| src/lawn/board.rs:25 | crate::todlib::reanimator::Reanimation | 未使用导入 | - |
| src/lawn/board.rs:31 | crate::todlib::tod_particle::ParticleSystem | 未使用导入 | - |
| src/lawn/cursor_object.rs:5 | HitResult | 未使用导入 | - |
| src/lawn/cutscene.rs:15 | crate::todlib::tod_common::rand_range_float | 未使用导入 | - |
| src/lawn/lawn_app.rs:12 | WidgetImpl | 未使用导入 | - |
| src/lawn/lawn_app.rs:13 | crate::framework::widget::widget_manager::WidgetManager | 未使用导入 | - |
| src/lawn/lawn_app.rs:20 | TitleState | 未使用导入 | - |
| src/lawn/plant.rs:5 | Rand | 未使用导入 | - |
| src/lawn/plant.rs:12 | crate::lawn::zombie::Zombie | 未使用导入 | - |
| src/lawn/projectile.rs:4 | Rand | 未使用导入 | - |
| src/lawn/projectile.rs:4 | RandFloat | 未使用导入 | - |
| src/lawn/projectile.rs:4 | RandRange | 未使用导入 | - |
| src/lawn/seed_packet.rs:5 | crate::framework::rect::Rect | 未使用导入 | - |
| src/lawn/system/data_sync.rs:8 | Seek | 未使用导入 | - |
| src/lawn/system/data_sync.rs:8 | SeekFrom | 未使用导入 | - |
| src/lawn/system/player_info.rs:4 | crate::lawn::system::data_sync::DataReaderException | 未使用导入 | - |
| src/lawn/system/save_game.rs:25 | MAX_MESSAGE_LENGTH | 未使用导入 | - |
| src/lawn/system/save_game.rs:26 | MAX_ZOMBIE_FOLLOWERS | 未使用导入 | - |
| src/lawn/system/save_game.rs:29 | crate::todlib::effect_system::EffectSystem | 未使用导入 | - |
| src/lawn/tool_tip_widget.rs:7 | crate::framework::graphics::font::Font | 未使用导入 | - |
| src/lawn/widget/almanac_dialog.rs:11 | crate::lawn::widget::lawn_dialog::LawnDialog | 未使用导入 | - |
| src/lawn/widget/challenge_screen.rs:17 | tod_animate_curve | 未使用导入 | - |
| src/lawn/widget/lawn_dialog.rs:9 | crate::framework::widget::dialog::Dialog | 未使用导入 | - |
| src/lawn/widget/message_widget.rs:7 | crate::framework::rect::Rect | 未使用导入 | - |
| src/lawn/widget/store_screen.rs:21 | tod_animate_curve | 未使用导入 | - |
| src/lawn/widget/zombatar_tos.rs:11 | crate::lawn::game_enums::* | 未使用导入 | - |
| src/lawn/widget/zombatar_widget.rs:13 | crate::framework::widget::widget_manager::WidgetManager | 未使用导入 | - |
| src/lawn/widget/zombatar_widget.rs:17 | zombatar_read_record_slot | 未使用导入 | - |
| src/lawn/zombie.rs:5 | Rand | 未使用导入 | - |
| src/lawn/zombie.rs:10 | crate::lawn::board::Board | 未使用导入 | - |
| src/lawn/zombie.rs:13 | crate::lawn::lawn_app::LawnApp | 未使用导入 | - |
| src/lawn/zombie.rs:16 | Attachment | 未使用导入 | - |
| src/todlib/definition.rs:7 | crate::framework::rect::Rect | 未使用导入 | - |
| src/todlib/definition.rs:8 | ReanimLoopType | 未使用导入 | - |
| src/todlib/definition.rs:8 | Reanimation | 未使用导入 | - |
| src/todlib/effect_system.rs:7 | ATTACHMENTID_NULL | 未使用导入 | - |
| src/todlib/effect_system.rs:7 | CoinID | 未使用导入 | - |
| src/todlib/mod.rs:23 | tod_string_file::TodStringFile | 未使用导入 | - |
| src/todlib/reanimator.rs:7 | crate::framework::graphics::gl_interface::TriVertex | 未使用导入 | - |
| src/todlib/reanimator.rs:10 | crate::framework::rect::Rect | 未使用导入 | - |
| src/todlib/reanimator.rs:11 | TodCurves | 未使用导入 | - |
| src/todlib/tod_foley.rs:9 | crate::framework::sound::sound_manager::SoundManager | 未使用导入 | - |
| src/todlib/tod_particle.rs:10 | std::collections::LinkedList | 未使用导入 | - |
| src/todlib/tod_particle.rs:19 | PARTICLESYSTEMID_NULL | 未使用导入 | - |
| src/todlib/tod_particle.rs:19 | ParticleSystemID | 未使用导入 | - |
| src/todlib/trail.rs:6 | std::f32::consts::PI | 未使用导入 | - |
| src/framework/desc_parser.rs:49 | as_list_mut | 未使用方法 | - |
| src/framework/desc_parser.rs:90 | define_map_mut | 未使用方法 | - |
| src/lawn/board.rs:6917 | get_boss_zombie_mut | 未使用方法 | - |
| src/lawn/board.rs:8060 | get_pumpkin_at_mut | 未使用方法 | - |
| src/lawn/system/pool_effect.rs:14 | POOL_IMAGE_WIDTH | 未使用常量 / 静态 | - |
| src/lawn/system/pool_effect.rs:15 | POOL_IMAGE_HEIGHT | 未使用常量 / 静态 | - |
| src/framework/graphics/gl_image.rs:405 | clip_rect | 未使用变量 | cpp/src/Lawn/Widget/CreditScreen.cpp#CreditScreen::Draw×7; cpp/src/Lawn/Zombie.cpp#Zombie::SetupDrawZombieWon×3 |
| src/framework/graphics/image_font.rs:1296 | bytes | 未使用变量 | cpp/src/SexyAppFramework/platform/default/Input.cpp#if×3; cpp/src/SexyAppFramework/imagelib/ImageLib.cpp#Sexy::FromLE16×2 |
| src/framework/graphics/memory_image.rs:379 | app | 未使用变量 | cpp/src/SexyAppFramework/SexyAppBase.cpp#SexyAppBase::EmscriptenMainLoopCallback×7; cpp/src/SexyAppFramework/SexyAppBase.cpp#if×5 |
| src/framework/graphics/memory_image.rs:396 | app | 未使用变量 | cpp/src/SexyAppFramework/SexyAppBase.cpp#SexyAppBase::EmscriptenMainLoopCallback×7; cpp/src/SexyAppFramework/SexyAppBase.cpp#if×5 |
| src/framework/widget/widget.rs:64 | btn | 未使用变量 | cpp/src/SexyAppFramework/platform/default/Input.cpp#SexyAppBase::ProcessDeferredMessages×5 |
| src/framework/widget/widget.rs:65 | btn | 未使用变量 | cpp/src/SexyAppFramework/platform/default/Input.cpp#SexyAppBase::ProcessDeferredMessages×5 |
| src/framework/widget/widget.rs:68 | delta | 未使用变量 | cpp/src/PvzpLib/EffectSystem.cpp#EffectSystem::Update×36; cpp/src/SexyAppFramework/graphics/SWTri.cpp#(文件级)×36 |
| src/lawn/challenge.rs:3564 | b | 未使用变量 | cpp/src/SexyAppFramework/SexyAppBase.cpp#std::max×18; cpp/src/PvzpLib/FilterEffect.cpp#std::clamp×10 |
| src/lawn/coin.rs:1392 | mode | 未使用变量 | cpp/src/ConstEnums.h#(文件级)×2; cpp/src/SexyAppFramework/graphics/inc_routines/MI_SlowStretchBlt.inc#(文件级)×2 |
| src/lawn/seed_packet.rs:664 | packet | 未使用变量 | cpp/src/SexyAppFramework/imagelib/ImageLib.cpp#Sexy::FromLE16×4 |
| src/lawn/widget/new_options_dialog.rs:574 | app | 未使用变量 | cpp/src/SexyAppFramework/SexyAppBase.cpp#SexyAppBase::EmscriptenMainLoopCallback×7; cpp/src/SexyAppFramework/SexyAppBase.cpp#if×5 |
| src/lawn/widget/seed_chooser_screen.rs:822 | app | 未使用变量 | cpp/src/SexyAppFramework/SexyAppBase.cpp#SexyAppBase::EmscriptenMainLoopCallback×7; cpp/src/SexyAppFramework/SexyAppBase.cpp#if×5 |
| src/lawn/widget/seed_chooser_screen.rs:1703 | btn | 未使用变量 | cpp/src/SexyAppFramework/platform/default/Input.cpp#SexyAppBase::ProcessDeferredMessages×5 |
| src/lawn/widget/seed_chooser_screen.rs:1711 | btn | 未使用变量 | cpp/src/SexyAppFramework/platform/default/Input.cpp#SexyAppBase::ProcessDeferredMessages×5 |
| src/todlib/data_array.rs:57 | i | 未使用变量 | cpp/src/Lawn/System/SaveGame.cpp#(文件级)×48; cpp/src/SexyAppFramework/graphics/inc_routines/DDI_AlphaBlt.inc#(文件级)×19 |
| src/framework/graphics/image.rs:555 | a_dest_x | 未读赋值 | cpp/src/Lawn/Coin.cpp#Coin::UpdateCollected×11; cpp/src/Lawn/Zombie.cpp#Zombie::UpdateZombieSquashHead×9 |
| src/framework/graphics/image.rs:596 | a_dest_y | 未读赋值 | cpp/src/Lawn/Coin.cpp#Coin::UpdateCollected×10; cpp/src/SexyAppFramework/graphics/inc_routines/MI_SlowStretchBlt.inc#(文件级)×4 |
| src/framework/sexy_app_base.rs:2832 | pos | 未读赋值 | cpp/src/SexyAppFramework/imagelib/ImageLib.cpp#Sexy::FromLE16×4; cpp/src/SexyAppFramework/widget/Slider.cpp#Slider::MouseDown×4 |
| src/lawn/board.rs:5706 | bg | 未读赋值 | cpp/src/Lawn/Widget/ZombatarWidget.cpp#(文件级)×1 |
| src/lawn/lawn_app.rs:1005 | screen | 多余 mut | cpp/src/Lawn/CutScene.cpp#CutScene::IsScrolledLeftAtStart×1; cpp/src/Lawn/Cutscene.h#(文件级)×1 |
| src/lawn/widget/cheat_dialog.rs:160 | h | 多余 mut | cpp/src/SexyAppFramework/graphics/GLInterface.cpp#Sexy::PrintF×19; cpp/src/SexyAppFramework/graphics/GLInterface.cpp#std::min×12 |
| src/lawn/widget/store_screen.rs:144 | screen | 多余 mut | cpp/src/Lawn/CutScene.cpp#CutScene::IsScrolledLeftAtStart×1; cpp/src/Lawn/Cutscene.h#(文件级)×1 |

## 7. 口径与局限

1. 证据全部为静态检索：C++ 调用点按「非定义/非声明行出现该标识符（排除注释与字符串字面量）」判定；成员变量按 mCamel / Camel / 别名形态检索；局部变量按所在函数体内出现判定。
2. 同名 C++ 函数（如 Board::MouseMove 与 Widget::MouseMove）按 Rust 文件↔C++ 文件映射限定；映射失败的进入宽扫，可能引入少量噪音（`cpp_ident_elsewhere` 类即此来源）。
3. `check_collisions`/`check_game_state`/`get_row_picking_item`/`y_to_row`/`x_to_col` 等为抽查函数体后的人工判定；其余 rust_org_or_split 按词根无匹配 + 无 C++ 注释自动判定，可能含少量误判（如 `load_sound_by_id` 实为 C++ LoadSound 重载的翻译）。
4. 修复时应以本清单为**线索**而非结论：接线前先读 cpp/ 对应调用点的完整上下文（铁律 1），逐项确认调用时机与线程归属。
5. 机读数据：`target/unused_final_verdicts.json`（1573 条全量，字段 file/line/name/kind/verdict/cppWhere/note）。
