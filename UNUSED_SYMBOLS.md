# UNUSED_SYMBOLS · Rust 侧未使用符号统计

> 数据来源：rustc 的 can-never-be-used 诊断（dead_code / unused_variables / unused_assignments / unused_mut / unused_imports）。
> 语义：这些符号**在本 crate 内从未被使用**，是“翻译了但没有接线”或“多余定义”的机器证据。

| 项 | 值 |
|---|---|
| 生成时间 | 2026-11-20 23:32:26 |
| 工具链 | rustc 1.98.0-nightly (f46ec5218 2026-06-30) |
| 采集命令 | cargo build --all-targets --message-format=json |
| 统计范围 | src/ 全部模块；以 bin 目标 pvz-portable（src/main.rs 依赖链）为准，并入 lib 目标独有诊断 |
| 符号条目总数 | 1573（按 类别+名称+文件+行号 去重，lib/bin 重复编译不重复计数） |
| 涉及文件 | 72 |

## 1. 总览

| 类别 | 数量 | 说明 |
|---|---:|---|
| 函数 | 33 | 自由函数，无人调用 |
| 方法 | 222 | impl 方法，无人调用 |
| 关联项 | 101 | 关联函数，无人调用 |
| 关联函数 | 4 | 关联函数，无人调用 |
| 常量 | 12 | 常量，无人引用 |
| 关联常量 | 1 | 关联常量，无人引用 |
| 结构体 | 6 | 结构体，从未被构造 |
| 枚举 | 4 | 枚举类型，从未被使用 |
| 字段 | 51 | 结构体字段，从未被读取 |
| 枚举变体 | 885 | 枚举变体，从未被构造 |
| 变量 | 96 | 局部变量声明后未使用（unused_variables） |
| 赋值未使用 | 63 | 赋值结果从未被读（unused_assignments） |
| 多余的 mut | 29 | 不需要 mut 的变量（unused_mut） |
| 导入 | 66 | use 导入未使用（unused_imports） |

**归纳**：函数/方法类 360 项，变量类 188 项，枚举变体 885 项，其余（常量/类型/字段/导入）140 项。

## 2. 分文件汇总

| 文件 | 函数 | 方法 | 关联项 | 关联函数 | 常量 | 关联常量 | 结构体 | 枚举 | 字段 | 枚举变体 | 变量 | 赋值未使用 | 多余的 mut | 导入 | 合计 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| src/framework/resources.rs |  |  |  | 1 |  |  |  |  |  | 868 |  |  |  |  | 869 |
| src/lawn/board.rs | 4 | 71 |  | 1 | 1 |  | 1 |  | 9 |  | 1 | 1 | 5 | 3 | 97 |
| src/lawn/lawn_app.rs | 2 |  | 42 | 1 |  |  |  |  | 17 | 5 | 3 | 5 | 2 | 3 | 80 |
| src/lawn/challenge.rs | 1 | 49 |  |  |  |  | 1 | 1 |  |  | 2 | 1 | 1 |  | 56 |
| src/lawn/seed_packet.rs |  | 5 | 3 |  |  |  | 1 |  | 2 |  | 4 | 31 |  | 1 | 47 |
| src/lawn/plant.rs | 1 |  | 16 |  |  |  |  |  | 3 |  | 7 | 10 | 5 | 2 | 44 |
| src/lawn/zombie.rs |  | 1 | 25 |  | 3 | 1 |  | 1 | 1 |  |  | 1 | 1 | 4 | 38 |
| src/framework/widget/widget.rs |  |  |  |  |  |  |  |  |  |  | 34 |  |  | 1 | 35 |
| src/lawn/projectile.rs | 1 | 6 |  |  |  |  | 1 | 1 | 4 | 5 | 4 |  | 2 | 3 | 27 |
| src/lawn/cutscene.rs |  | 12 |  |  | 4 |  |  |  |  |  |  |  |  | 1 | 17 |
| src/lawn/widget/achievements_screen.rs |  |  | 7 | 1 |  |  | 1 |  | 1 | 7 |  |  |  |  | 17 |
| src/framework/sound/sound_manager.rs |  | 15 |  |  |  |  |  |  |  |  |  |  |  |  | 15 |
| src/lawn/widget/message_widget.rs | 2 | 5 |  |  | 1 |  |  |  | 3 |  |  | 3 |  | 1 | 15 |
| src/framework/sound/music_interface.rs |  | 12 |  |  |  |  |  |  |  |  | 2 |  |  |  | 14 |
| src/lawn/lawn_common.rs | 6 |  | 4 |  | 1 |  | 1 |  |  |  |  |  |  |  | 12 |
| src/lawn/lawn_mower.rs |  | 7 |  |  |  |  |  |  | 3 |  |  |  | 2 |  | 12 |
| src/lawn/widget/title_screen.rs |  | 6 |  |  |  |  |  | 1 | 3 |  |  | 1 |  |  | 11 |
| src/lawn/cursor_object.rs | 1 | 7 |  |  |  |  |  |  |  |  | 1 |  |  | 1 | 10 |
| src/lawn/grid_item.rs |  | 10 |  |  |  |  |  |  |  |  |  |  |  |  | 10 |
| src/framework/desc_parser.rs |  | 7 |  |  |  |  |  |  |  |  | 1 |  |  | 1 | 9 |
| src/framework/widget/mod.rs | 9 |  |  |  |  |  |  |  |  |  |  |  |  |  | 9 |
| src/framework/graphics/image_font.rs |  |  |  |  |  |  |  |  |  |  | 8 |  |  |  | 8 |
| src/lawn/system/pool_effect.rs |  | 1 | 4 |  | 2 |  |  |  |  |  |  |  |  |  | 7 |
| src/todlib/reanimator.rs |  |  |  |  |  |  |  |  |  |  | 1 |  | 3 | 3 | 7 |
| src/todlib/tod_particle.rs |  |  |  |  |  |  |  |  |  |  | 1 | 2 | 1 | 3 | 7 |
| src/framework/graphics/memory_image.rs |  |  |  |  |  |  |  |  |  |  | 3 |  |  | 2 | 5 |
| src/framework/resource_manager.rs |  |  |  |  |  |  |  |  |  |  | 1 |  |  | 4 | 5 |
| src/lawn/coin.rs |  | 1 |  |  |  |  |  |  | 2 |  | 2 |  |  |  | 5 |
| src/lawn/widget/seed_chooser_screen.rs |  |  |  |  |  |  |  |  |  |  | 5 |  |  |  | 5 |
| src/framework/sound/sdl_sound_manager.rs | 3 | 1 |  |  |  |  |  |  |  |  |  |  |  |  | 4 |
| src/lawn/system/reanimation_lawn.rs |  |  |  |  |  |  |  |  |  |  |  |  | 4 |  | 4 |
| src/lawn/system/save_game.rs |  |  |  |  |  |  |  |  |  |  |  | 1 |  | 3 | 4 |
| src/framework/graphics/gl_image.rs |  |  |  |  |  |  |  |  |  |  | 1 |  |  | 2 | 3 |
| src/framework/graphics/image.rs |  |  |  |  |  |  |  |  |  |  |  | 2 |  | 1 | 3 |
| src/framework/paklib/mod.rs |  |  |  |  |  |  |  |  |  |  | 2 |  |  | 1 | 3 |
| src/framework/sexy_app_base.rs |  |  |  |  |  |  |  |  |  |  |  | 1 |  | 2 | 3 |
| src/framework/widget/button_widget.rs |  |  |  |  |  |  |  |  |  |  | 3 |  |  |  | 3 |
| src/lawn/game_object.rs |  | 3 |  |  |  |  |  |  |  |  |  |  |  |  | 3 |
| src/todlib/definition.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 3 | 3 |
| src/todlib/effect_system.rs |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 2 | 3 |
| src/todlib/reanim_loader.rs | 3 |  |  |  |  |  |  |  |  |  |  |  |  |  | 3 |
| src/framework/sound/sdl_sound_instance.rs |  |  |  |  |  |  |  |  | 2 |  |  |  |  |  | 2 |
| src/framework/sound/sound_instance.rs |  | 2 |  |  |  |  |  |  |  |  |  |  |  |  | 2 |
| src/framework/widget/slider.rs |  |  |  |  |  |  |  |  |  |  | 2 |  |  |  | 2 |
| src/lawn/system/data_sync.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 2 | 2 |
| src/lawn/widget/award_screen.rs |  | 1 |  |  |  |  |  |  | 1 |  |  |  |  |  | 2 |
| src/lawn/widget/challenge_screen.rs |  |  |  |  |  |  |  |  |  |  | 1 |  |  | 1 | 2 |
| src/lawn/widget/cheat_dialog.rs |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |  | 2 |
| src/lawn/widget/store_screen.rs |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 | 2 |
| src/lawn/widget/zombatar_widget.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 2 | 2 |
| src/todlib/trail.rs |  |  |  |  |  |  |  |  |  |  | 1 |  |  | 1 | 2 |
| src/framework/graphics/graphics.rs |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  | 1 |
| src/framework/graphics/quantize.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| src/framework/widget/dialog.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| src/framework/widget/edit_widget.rs |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  | 1 |
| src/framework/widget/scrollbar_widget.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| src/framework/widget/text_widget.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| src/framework/widget/widget_container.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| src/lawn/system/player_info.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| src/lawn/tool_tip_widget.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| src/lawn/widget/almanac_dialog.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| src/lawn/widget/continue_dialog.rs |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  | 1 |
| src/lawn/widget/credit_screen.rs |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  | 1 |
| src/lawn/widget/lawn_dialog.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| src/lawn/widget/new_options_dialog.rs |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  | 1 |
| src/lawn/widget/new_user_dialog.rs |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  | 1 |
| src/lawn/widget/user_dialog.rs |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  | 1 |
| src/lawn/widget/zombatar_tos.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| src/lawn/zen_garden.rs |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  | 1 |
| src/todlib/data_array.rs |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  | 1 |
| src/todlib/mod.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| src/todlib/tod_foley.rs |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |
| **合计** | **33** | **222** | **101** | **4** | **12** | **1** | **6** | **4** | **51** | **885** | **96** | **63** | **29** | **66** | **1573** |

## 3. 详细清单

### 3.1 未使用函数（自由函数）

共 33 项。

| 位置 | 名称 | pub | C++ 出处 |
|---|---|---|---|
| src/framework/sound/sdl_sound_manager.rs:358 | set_music_amplify | pub |  |
| src/framework/sound/sdl_sound_manager.rs:367 | load_au_sound | pub |  |
| src/framework/sound/sdl_sound_manager.rs:371 | get_num_sounds | pub |  |
| src/framework/widget/mod.rs:24 | set_select | pub |  |
| src/framework/widget/mod.rs:26 | set_invis_if_no_scroll | pub |  |
| src/framework/widget/mod.rs:28 | get_last_color | pub |  |
| src/framework/widget/mod.rs:30 | get_text_index_at | pub |  |
| src/framework/widget/mod.rs:32 | get_selection | pub |  |
| src/framework/widget/mod.rs:34 | remove_all_widgets | pub |  |
| src/framework/widget/mod.rs:36 | free_resources | pub |  |
| src/framework/widget/mod.rs:38 | get_widget_flags | pub |  |
| src/framework/widget/mod.rs:42 | get_sort_key | pub |  |
| src/lawn/board.rs:138 | y_to_row | pub |  |
| src/lawn/board.rs:143 | x_to_col | pub |  |
| src/lawn/board.rs:153 | render_item_sort_func | pub |  |
| src/lawn/board.rs:240 | board_init_for_player | pub |  |
| src/lawn/challenge.rs:25 | get_image |  |  |
| src/lawn/cursor_object.rs:476 | plant_draw_seed_type |  |  |
| src/lawn/lawn_app.rs:3957 | lawn_get_current_level_name | pub |  |
| src/lawn/lawn_app.rs:3963 | lawn_get_close_request | pub |  |
| src/lawn/lawn_common.rs:30 | mod_in_range | pub |  |
| src/lawn/lawn_common.rs:41 | grid_in_range | pub |  |
| src/lawn/lawn_common.rs:60 | tile_image_horizontally | pub |  |
| src/lawn/lawn_common.rs:79 | tile_image_vertically | pub |  |
| src/lawn/lawn_common.rs:161 | create_edit_widget | pub |  |
| src/lawn/lawn_common.rs:185 | make_new_checkbox | pub |  |
| src/lawn/plant.rs:5804 | get_image | pub |  |
| src/lawn/projectile.rs:13 | pvzp_scale_rotate_transform_matrix |  |  |
| src/lawn/widget/message_widget.rs:426 | message_label_to_str |  |  |
| src/lawn/widget/message_widget.rs:432 | setup_default_font |  |  |
| src/todlib/reanim_loader.rs:204 | get_reanim_font_name | pub |  |
| src/todlib/reanim_loader.rs:217 | reanim_image_name_index | pub |  |
| src/todlib/reanim_loader.rs:360 | parse_reanim_xml_for_test | pub |  |

### 3.2 未使用方法（impl 方法）

共 222 项。

| 位置 | 名称 | pub | C++ 出处 |
|---|---|---|---|
| src/framework/desc_parser.rs:28 | is_list | pub |  |
| src/framework/desc_parser.rs:41 | as_list | pub |  |
| src/framework/desc_parser.rs:49 | as_list_mut | pub |  |
| src/framework/desc_parser.rs:72 | error_message |  |  |
| src/framework/desc_parser.rs:78 | set_cmd_sep |  |  |
| src/framework/desc_parser.rs:80 | current_line_num |  |  |
| src/framework/desc_parser.rs:90 | define_map_mut |  |  |
| src/framework/sound/music_interface.rs:3 | load_music |  |  |
| src/framework/sound/music_interface.rs:8 | stop_all_music |  |  |
| src/framework/sound/music_interface.rs:10 | unload_all_music |  |  |
| src/framework/sound/music_interface.rs:11 | pause_all_music |  |  |
| src/framework/sound/music_interface.rs:12 | resume_all_music |  |  |
| src/framework/sound/music_interface.rs:13 | fade_in |  |  |
| src/framework/sound/music_interface.rs:14 | fade_out |  |  |
| src/framework/sound/music_interface.rs:15 | fade_out_all |  |  |
| src/framework/sound/music_interface.rs:17 | set_song_max_volume |  |  |
| src/framework/sound/music_interface.rs:18 | is_playing |  |  |
| src/framework/sound/music_interface.rs:19 | set_volume |  |  |
| src/framework/sound/music_interface.rs:20 | update |  |  |
| src/framework/sound/sdl_sound_manager.rs:162 | rehup_all_volumes |  |  |
| src/framework/sound/sound_instance.rs:17 | set_pan |  |  |
| src/framework/sound/sound_instance.rs:27 | get_volume |  |  |
| src/framework/sound/sound_manager.rs:3 | initialized |  |  |
| src/framework/sound/sound_manager.rs:4 | load_sound_by_id |  |  |
| src/framework/sound/sound_manager.rs:6 | release_sound |  |  |
| src/framework/sound/sound_manager.rs:7 | set_volume |  |  |
| src/framework/sound/sound_manager.rs:8 | set_base_volume |  |  |
| src/framework/sound/sound_manager.rs:9 | set_base_pan |  |  |
| src/framework/sound/sound_manager.rs:11 | stop_sound |  |  |
| src/framework/sound/sound_manager.rs:12 | stop_all_sounds |  |  |
| src/framework/sound/sound_manager.rs:13 | set_master_volume |  |  |
| src/framework/sound/sound_manager.rs:14 | get_master_volume |  |  |
| src/framework/sound/sound_manager.rs:15 | flush |  |  |
| src/framework/sound/sound_manager.rs:16 | get_free_sound_id |  |  |
| src/framework/sound/sound_manager.rs:17 | num_sounds |  |  |
| src/framework/sound/sound_manager.rs:28 | release_sounds |  |  |
| src/framework/sound/sound_manager.rs:30 | release_channels |  |  |
| src/lawn/board.rs:1328 | check_collisions |  |  |
| src/lawn/board.rs:1445 | find_plant_at | pub |  |
| src/lawn/board.rs:1466 | find_zombie_in_row | pub |  |
| src/lawn/board.rs:1689 | draw_debug_text | pub |  |
| src/lawn/board.rs:1866 | draw_debug_object_rects | pub |  |
| src/lawn/board.rs:1927 | add_boss_render_item | pub |  |
| src/lawn/board.rs:1989 | draw_ui_coin_bank | pub |  |
| src/lawn/board.rs:2047 | draw_zen_wheel_barrow_button | pub |  |
| src/lawn/board.rs:2143 | draw_shovel | pub |  |
| src/lawn/board.rs:2202 | draw_seed_bank |  |  |
| src/lawn/board.rs:2318 | draw_ui_bottom | pub |  |
| src/lawn/board.rs:2395 | draw_top_right_ui | pub |  |
| src/lawn/board.rs:2480 | draw_ui_top | pub |  |
| src/lawn/board.rs:2582 | draw_backdrop | pub |  |
| src/lawn/board.rs:2766 | draw_zen_buttons | pub |  |
| src/lawn/board.rs:3123 | draw_progress_meter | pub |  |
| src/lawn/board.rs:3567 | check_game_state |  |  |
| src/lawn/board.rs:3595 | game_over | pub |  |
| src/lawn/board.rs:3623 | level_complete | pub |  |
| src/lawn/board.rs:3631 | draw_game_objects | pub |  |
| src/lawn/board.rs:4106 | draw | pub |  |
| src/lawn/board.rs:4147 | draw_background |  |  |
| src/lawn/board.rs:4204 | draw_ui_text |  |  |
| src/lawn/board.rs:4232 | draw_ice |  |  |
| src/lawn/board.rs:4280 | draw_fog |  |  |
| src/lawn/board.rs:4334 | draw_house_door_bottom | pub |  |
| src/lawn/board.rs:4360 | draw_house_door_top | pub |  |
| src/lawn/board.rs:4389 | draw_level |  |  |
| src/lawn/board.rs:4421 | get_board_image |  |  |
| src/lawn/board.rs:4548 | draw_fade_out |  |  |
| src/lawn/board.rs:4577 | mouse_down | pub |  |
| src/lawn/board.rs:4799 | key_down | pub |  |
| src/lawn/board.rs:4897 | key_char | pub |  |
| src/lawn/board.rs:5703 | pick_background | pub |  |
| src/lawn/board.rs:5929 | count_sun_flowers | pub |  |
| src/lawn/board.rs:6047 | has_progress_meter | pub |  |
| src/lawn/board.rs:6204 | get_grave_stone_count | pub |  |
| src/lawn/board.rs:6226 | get_winning_zombie | pub |  |
| src/lawn/board.rs:6501 | add_crater | pub |  |
| src/lawn/board.rs:6579 | add_grave_stones | pub |  |
| src/lawn/board.rs:6917 | get_boss_zombie_mut | pub |  |
| src/lawn/board.rs:6973 | get_row_picking_item | pub |  |
| src/lawn/board.rs:6978 | is_zombie_allowed | pub |  |
| src/lawn/board.rs:7157 | get_zombie_points_for_wave | pub |  |
| src/lawn/board.rs:7693 | mouse_move | pub |  |
| src/lawn/board.rs:7699 | mouse_drag | pub |  |
| src/lawn/board.rs:7720 | take_sun_money | pub |  |
| src/lawn/board.rs:7786 | count_lawn_mowers_remaining | pub |  |
| src/lawn/board.rs:7818 | is_row_lawn_mowered | pub |  |
| src/lawn/board.rs:7984 | find_lawn_mower_in_row | pub |  |
| src/lawn/board.rs:7991 | find_lawn_mower_in_row_mut | pub |  |
| src/lawn/board.rs:8044 | get_ice_z_pos | pub |  |
| src/lawn/board.rs:8051 | get_pumpkin_at | pub |  |
| src/lawn/board.rs:8060 | get_pumpkin_at_mut | pub |  |
| src/lawn/board.rs:8268 | mouse_up | pub |  |
| src/lawn/board.rs:8813 | mouse_down_with_tool | pub |  |
| src/lawn/board.rs:8855 | mouse_down_cobcannon_fire | pub |  |
| src/lawn/board.rs:8893 | tool_hit_test_helper | pub |  |
| src/lawn/board.rs:9021 | mouse_down_with_plant | pub |  |
| src/lawn/board.rs:9533 | pick_up_tool | pub |  |
| src/lawn/board.rs:10812 | remove_seed | pub |  |
| src/lawn/board.rs:10891 | update_seed_bank_width | pub |  |
| src/lawn/board.rs:10902 | refresh_all_packets | pub |  |
| src/lawn/board.rs:11982 | pick_special_grave_stone | pub |  |
| src/lawn/board.rs:12174 | has_valid_cob_cannon_spot | pub |  |
| src/lawn/board.rs:12666 | init_zombie_waves_for_level | pub |  |
| src/lawn/board.rs:12726 | is_final_boss_level | pub |  |
| src/lawn/board.rs:13067 | get_bottom_lawn_mower | pub |  |
| src/lawn/board.rs:13085 | number_zombies_in_wave | pub |  |
| src/lawn/board.rs:13099 | save_game | pub |  |
| src/lawn/board.rs:13223 | get_zen_tool_at | pub |  |
| src/lawn/challenge.rs:441 | beghouled_drag_start | pub |  |
| src/lawn/challenge.rs:447 | beghouled_drag_update | pub |  |
| src/lawn/challenge.rs:493 | mouse_move | pub |  |
| src/lawn/challenge.rs:507 | mouse_down | pub |  |
| src/lawn/challenge.rs:540 | mouse_up | pub |  |
| src/lawn/challenge.rs:963 | zombie_ate_plant | pub |  |
| src/lawn/challenge.rs:995 | draw_backdrop | pub |  |
| src/lawn/challenge.rs:1071 | draw_art_challenge | pub |  |
| src/lawn/challenge.rs:1332 | draw_beghouled | pub |  |
| src/lawn/challenge.rs:1522 | init_zombie_waves_from_list | pub |  |
| src/lawn/challenge.rs:1849 | draw_slot_machine | pub |  |
| src/lawn/challenge.rs:2186 | beghouled_clear_crater | pub |  |
| src/lawn/challenge.rs:2207 | mouse_down_whack_a_zombie | pub |  |
| src/lawn/challenge.rs:2283 | draw_storm_night | pub |  |
| src/lawn/challenge.rs:2466 | grave_danger_spawn_random_grave | pub |  |
| src/lawn/challenge.rs:2507 | grave_danger_spawn_grave_at | pub |  |
| src/lawn/challenge.rs:2694 | draw_storm_flash | pub |  |
| src/lawn/challenge.rs:3658 | get_portal_left_right | pub |  |
| src/lawn/challenge.rs:3693 | beghouled_packet_clicked | pub |  |
| src/lawn/challenge.rs:3793 | zombiquarium_packet_clicked | pub |  |
| src/lawn/challenge.rs:3841 | zombiquarium_drop_brain | pub |  |
| src/lawn/challenge.rs:3861 | zombiquarium_mouse_down | pub |  |
| src/lawn/challenge.rs:4912 | scary_potter_fill_column_with_plant | pub |  |
| src/lawn/challenge.rs:5013 | scary_potter_mallet_pot | pub |  |
| src/lawn/challenge.rs:5099 | i_zombie_mouse_down_with_zombie | pub |  |
| src/lawn/challenge.rs:5324 | i_zombie_draw_plant | pub |  |
| src/lawn/challenge.rs:5373 | i_zombie_set_plant_filter_effect | pub |  |
| src/lawn/challenge.rs:5676 | draw_rain | pub |  |
| src/lawn/challenge.rs:5783 | draw_weather | pub |  |
| src/lawn/challenge.rs:5876 | squirrel_found | pub |  |
| src/lawn/challenge.rs:5970 | squirrel_peek | pub |  |
| src/lawn/challenge.rs:5975 | squirrel_chew | pub |  |
| src/lawn/challenge.rs:6009 | squirrel_update_one | pub |  |
| src/lawn/challenge.rs:6057 | i_zombie_setup_plant | pub |  |
| src/lawn/challenge.rs:6180 | i_zombie_eat_brain | pub |  |
| src/lawn/challenge.rs:6202 | i_zombie_get_brain_target | pub |  |
| src/lawn/challenge.rs:6248 | advance_crazy_dave_dialog | pub |  |
| src/lawn/challenge.rs:6418 | i_zombie_squish_brain | pub |  |
| src/lawn/challenge.rs:6425 | i_zombie_score_brain | pub |  |
| src/lawn/challenge.rs:6564 | beghouled_twist_mouse_down | pub |  |
| src/lawn/challenge.rs:6698 | beghouled_cancel_match_flashing | pub |  |
| src/lawn/challenge.rs:6862 | i_zombie_place_zombie | pub |  |
| src/lawn/challenge.rs:6981 | tree_of_wisdom_fertilize | pub |  |
| src/lawn/challenge.rs:7025 | tree_of_wisdom_init | pub |  |
| src/lawn/challenge.rs:7117 | tree_of_wisdom_mouse_on | pub |  |
| src/lawn/challenge.rs:7153 | tree_of_wisdom_draw | pub |  |
| src/lawn/challenge.rs:7303 | tree_of_wisdom_next_garden | pub |  |
| src/lawn/challenge.rs:7323 | tree_of_wisdom_open_store | pub |  |
| src/lawn/challenge.rs:7366 | tree_of_wisdom_tool | pub |  |
| src/lawn/coin.rs:1593 | mouse_down | pub |  |
| src/lawn/cursor_object.rs:65 | init | pub |  |
| src/lawn/cursor_object.rs:73 | update_position | pub |  |
| src/lawn/cursor_object.rs:104 | die | pub |  |
| src/lawn/cursor_object.rs:126 | draw | pub |  |
| src/lawn/cursor_object.rs:240 | get_cursor_type_for_app | pub |  |
| src/lawn/cursor_object.rs:377 | draw | pub |  |
| src/lawn/cursor_object.rs:451 | plant_draw_height_offset |  |  |
| src/lawn/cutscene.rs:103 | init_app | pub |  |
| src/lawn/cutscene.rs:1324 | is_after_seed_chooser | pub |  |
| src/lawn/cutscene.rs:1455 | show_zombie_walking | pub |  |
| src/lawn/cutscene.rs:1461 | is_cut_scene_over | pub |  |
| src/lawn/cutscene.rs:1467 | zombie_won_click | pub |  |
| src/lawn/cutscene.rs:1477 | mouse_down | pub |  |
| src/lawn/cutscene.rs:1486 | key_down | pub |  |
| src/lawn/cutscene.rs:1657 | can_get_packet_upgrade | pub |  |
| src/lawn/cutscene.rs:1677 | can_get_packet_upgrade_index | pub |  |
| src/lawn/cutscene.rs:2124 | can_get_second_packet_upgrade | pub |  |
| src/lawn/cutscene.rs:2777 | draw_upsell | pub |  |
| src/lawn/cutscene.rs:2856 | draw_intro | pub |  |
| src/lawn/game_object.rs:39 | begin_draw | pub |  |
| src/lawn/game_object.rs:49 | end_draw | pub |  |
| src/lawn/game_object.rs:55 | make_parent_graphics_frame | pub |  |
| src/lawn/grid_item.rs:367 | draw | pub |  |
| src/lawn/grid_item.rs:405 | draw_grid_item_overlay | pub |  |
| src/lawn/grid_item.rs:428 | draw_i_zombie_brain | pub |  |
| src/lawn/grid_item.rs:470 | draw_grave_stone | pub |  |
| src/lawn/grid_item.rs:586 | draw_stinky | pub |  |
| src/lawn/grid_item.rs:648 | draw_crater | pub |  |
| src/lawn/grid_item.rs:723 | draw_scary_pot | pub |  |
| src/lawn/grid_item.rs:783 | draw_ladder | pub |  |
| src/lawn/grid_item.rs:799 | add_grave_stone_particles | pub |  |
| src/lawn/grid_item.rs:822 | get_grid_image |  |  |
| src/lawn/lawn_mower.rs:147 | start_mower | pub |  |
| src/lawn/lawn_mower.rs:174 | start_mowing | pub |  |
| src/lawn/lawn_mower.rs:247 | mow_zombie | pub |  |
| src/lawn/lawn_mower.rs:265 | get_lawn_mower_attack_rect | pub |  |
| src/lawn/lawn_mower.rs:270 | get_mower_rect | pub |  |
| src/lawn/lawn_mower.rs:403 | draw_shadow | pub |  |
| src/lawn/lawn_mower.rs:464 | draw | pub |  |
| src/lawn/projectile.rs:863 | draw | pub |  |
| src/lawn/projectile.rs:986 | draw_shadow | pub |  |
| src/lawn/projectile.rs:1206 | is_zombie_hit_by_splash | pub |  |
| src/lawn/projectile.rs:1320 | get_projectile_def | pub |  |
| src/lawn/projectile.rs:1370 | get_damage_flags | pub |  |
| src/lawn/projectile.rs:1428 | check_for_high_ground | pub |  |
| src/lawn/seed_packet.rs:245 | slot_machine_start | pub |  |
| src/lawn/seed_packet.rs:307 | draw | pub |  |
| src/lawn/seed_packet.rs:401 | set_countdown | pub |  |
| src/lawn/seed_packet.rs:406 | was_planted | pub |  |
| src/lawn/seed_packet.rs:450 | mouse_down | pub |  |
| src/lawn/system/pool_effect.rs:56 | bilinear_lookup_fixed_point |  |  |
| src/lawn/widget/award_screen.rs:709 | draw_achievements | pub |  |
| src/lawn/widget/message_widget.rs:106 | get_font | pub |  |
| src/lawn/widget/message_widget.rs:112 | draw | pub |  |
| src/lawn/widget/message_widget.rs:308 | is_being_displayed | pub |  |
| src/lawn/widget/message_widget.rs:314 | layout_reanim_text | pub |  |
| src/lawn/widget/message_widget.rs:398 | draw_reanimated_text | pub |  |
| src/lawn/widget/title_screen.rs:141 | resize | pub |  |
| src/lawn/widget/title_screen.rs:148 | added_to_manager | pub |  |
| src/lawn/widget/title_screen.rs:156 | removed_from_manager | pub |  |
| src/lawn/widget/title_screen.rs:162 | button_press | pub |  |
| src/lawn/widget/title_screen.rs:169 | button_depress | pub |  |
| src/lawn/widget/title_screen.rs:181 | set_registered | pub |  |
| src/lawn/zombie.rs:868 | setup_health_for_type |  |  |

### 3.3 未使用关联项（关联函数 / 关联常量）

共 106 项。

| 位置 | 名称 | pub | C++ 出处 |
|---|---|---|---|
| src/framework/resources.rs:894 | count | pub |  |
| src/lawn/board.rs:82 | new | pub |  |
| src/lawn/lawn_app.rs:52 | new | pub |  |
| src/lawn/lawn_app.rs:569 | start_level | pub |  |
| src/lawn/lawn_app.rs:592 | finish_zen_garden_tutorial | pub |  |
| src/lawn/lawn_app.rs:978 | kill_seed_chooser_screen | pub |  |
| src/lawn/lawn_app.rs:1180 | finish_modeless_dialogs | pub |  |
| src/lawn/lawn_app.rs:1526 | do_cheat_dialog | pub |  |
| src/lawn/lawn_app.rs:1567 | do_pause_dialog | pub |  |
| src/lawn/lawn_app.rs:1629 | do_dialog_delay | pub |  |
| src/lawn/lawn_app.rs:1650 | center_dialog | pub |  |
| src/lawn/lawn_app.rs:2079 | can_pause_now | pub |  |
| src/lawn/lawn_app.rs:2209 | get_current_challenge_def | pub |  |
| src/lawn/lawn_app.rs:2215 | get_current_level_name | pub |  |
| src/lawn/lawn_app.rs:2268 | get_close_request | pub |  |
| src/lawn/lawn_app.rs:2271 | has_used_cheat_keys | pub |  |
| src/lawn/lawn_app.rs:2846 | toggle_slow_mo | pub |  |
| src/lawn/lawn_app.rs:2855 | toggle_fast_mo | pub |  |
| src/lawn/lawn_app.rs:2862 | got_focus | pub |  |
| src/lawn/lawn_app.rs:2866 | lost_focus | pub |  |
| src/lawn/lawn_app.rs:2872 | modal_open | pub |  |
| src/lawn/lawn_app.rs:2881 | init_hook | pub |  |
| src/lawn/lawn_app.rs:2886 | update_play_time_stats | pub |  |
| src/lawn/lawn_app.rs:2968 | need_register | pub | C++: (mBoard == nullptr \|\| mGameMode != ZEN_GARDEN) && (mBoard == nullptr \|\| mGameMode != TREE_OF_WISDOM) |
| src/lawn/lawn_app.rs:3014 | modal_close | pub |  |
| src/lawn/lawn_app.rs:3023 | do_confirm_back_to_main | pub |  |
| src/lawn/lawn_app.rs:3038 | do_confirm_sell_dialog | pub |  |
| src/lawn/lawn_app.rs:3078 | do_register | pub |  |
| src/lawn/lawn_app.rs:3081 | do_register_error | pub |  |
| src/lawn/lawn_app.rs:3084 | do_need_register_dialog | pub |  |
| src/lawn/lawn_app.rs:3087 | show_zombatar_tos | pub |  |
| src/lawn/lawn_app.rs:3101 | pre_display_hook | pub |  |
| src/lawn/lawn_app.rs:3106 | loading_thread_completed | pub |  |
| src/lawn/lawn_app.rs:3125 | update_app | pub |  |
| src/lawn/lawn_app.rs:3389 | write_to_registry | pub |  |
| src/lawn/lawn_app.rs:3401 | read_from_registry | pub |  |
| src/lawn/lawn_app.rs:3414 | switch_screen_mode | pub |  |
| src/lawn/lawn_app.rs:3422 | do_high_score_dialog | pub | C++: 若 NewOptionsDialog 存在则 mFullscreenCheckbox->SetChecked(!mIsWindowed) |
| src/lawn/lawn_app.rs:3426 | change_dir_hook | pub |  |
| src/lawn/lawn_app.rs:3582 | url_open_succeeded | pub |  |
| src/lawn/lawn_app.rs:3586 | open_url | pub |  |
| src/lawn/lawn_app.rs:3918 | mouse_down | pub |  |
| src/lawn/lawn_app.rs:3926 | key_down | pub |  |
| src/lawn/lawn_app.rs:3936 | debug_key_down | pub |  |
| src/lawn/lawn_app.rs:3939 | show_resource_error | pub |  |
| src/lawn/lawn_common.rs:108 | new | pub |  |
| src/lawn/lawn_common.rs:119 | key_down | pub |  |
| src/lawn/lawn_common.rs:127 | key_char | pub |  |
| src/lawn/lawn_common.rs:136 | key_text | pub |  |
| src/lawn/plant.rs:1433 | find_star_fruit_target | pub |  |
| src/lawn/plant.rs:1495 | mouse_down | pub |  |
| src/lawn/plant.rs:2099 | update_reanim | pub |  |
| src/lawn/plant.rs:2434 | draw_magnet_items_on_top | pub |  |
| src/lawn/plant.rs:2461 | draw_magnet_items | pub |  |
| src/lawn/plant.rs:3165 | is_fungus | pub |  |
| src/lawn/plant.rs:3425 | get_health_for_type | pub |  |
| src/lawn/plant.rs:3437 | get_launch_rate_for_type | pub |  |
| src/lawn/plant.rs:3449 | get_frame_length_for_type | pub |  |
| src/lawn/plant.rs:3452 | get_num_frames_for_type | pub |  |
| src/lawn/plant.rs:4346 | star_fruit_fire | pub |  |
| src/lawn/plant.rs:4553 | ice_zombies | pub |  |
| src/lawn/plant.rs:4570 | burn_row | pub |  |
| src/lawn/plant.rs:5647 | blow_away_fliers | pub |  |
| src/lawn/plant.rs:5667 | kill_all_plants_near_doom | pub |  |
| src/lawn/plant.rs:5731 | cob_cannon_fire | pub |  |
| src/lawn/seed_packet.rs:651 | new | pub |  |
| src/lawn/seed_packet.rs:662 | draw | pub |  |
| src/lawn/seed_packet.rs:670 | mouse_hit_test | pub |  |
| src/lawn/system/pool_effect.rs:28 | new | pub |  |
| src/lawn/system/pool_effect.rs:56 | bilinear_lookup_fixed_point |  |  |
| src/lawn/system/pool_effect.rs:86 | update_water_effect | pub |  |
| src/lawn/system/pool_effect.rs:188 | draw | pub |  |
| src/lawn/widget/achievements_screen.rs:141 | new | pub |  |
| src/lawn/widget/achievements_screen.rs:153 | update | pub |  |
| src/lawn/widget/achievements_screen.rs:166 | draw | pub |  |
| src/lawn/widget/achievements_screen.rs:170 | key_down | pub |  |
| src/lawn/widget/achievements_screen.rs:184 | mouse_down | pub |  |
| src/lawn/widget/achievements_screen.rs:188 | mouse_up | pub |  |
| src/lawn/widget/achievements_screen.rs:196 | mouse_wheel | pub |  |
| src/lawn/widget/achievements_screen.rs:281 | achievement_init_for_player | pub |  |
| src/lawn/zombie.rs:868 | setup_health_for_type |  |  |
| src/lawn/zombie.rs:2749 | plant_col_below | pub |  |
| src/lawn/zombie.rs:3762 | burn_row | pub |  |
| src/lawn/zombie.rs:4493 | mow_down | pub |  |
| src/lawn/zombie.rs:4560 | start_eating | pub |  |
| src/lawn/zombie.rs:4583 | eat_zombie | pub |  |
| src/lawn/zombie.rs:4692 | is_standing_on_spikeweed | pub |  |
| src/lawn/zombie.rs:4730 | is_squash_target | pub |  |
| src/lawn/zombie.rs:4854 | hit_ice_trap | pub |  |
| src/lawn/zombie.rs:4889 | convert_to_normal_zombie | pub |  |
| src/lawn/zombie.rs:5755 | eat_plant | pub |  |
| src/lawn/zombie.rs:6528 | find_zombie_target | pub |  |
| src/lawn/zombie.rs:6565 | check_if_prey_caught | pub |  |
| src/lawn/zombie.rs:6847 | zombie_type_can_go_on_high_ground | pub |  |
| src/lawn/zombie.rs:6875 | has_shadow | pub |  |
| src/lawn/zombie.rs:8018 | bobsled_burn | pub |  |
| src/lawn/zombie.rs:8851 | draw_zombie_part | pub |  |
| src/lawn/zombie.rs:8989 | draw_zombie_head | pub |  |
| src/lawn/zombie.rs:8999 | draw_zombie_with_parts | pub |  |
| src/lawn/zombie.rs:10346 | draw_bungee_target | pub |  |
| src/lawn/zombie.rs:10751 | draw_boss_part | pub |  |
| src/lawn/zombie.rs:10765 | draw_boss_back_arm | pub |  |
| src/lawn/zombie.rs:10801 | draw_boss_fire_ball | pub |  |
| src/lawn/zombie.rs:11561 | setup_zombatar_flag_reanim | pub |  |
| src/lawn/zombie.rs:12074 | bungee_drop_zombie | pub |  |
| src/lawn/zombie.rs:12758 | ZOMBIE_WAVE_DEBUG | pub |  |

### 3.4 未使用常量 / 静态变量

共 12 项。

| 位置 | 名称 | pub | C++ 出处 |
|---|---|---|---|
| src/lawn/board.rs:53 | MAX_RENDER_ITEMS | pub |  |
| src/lawn/cutscene.rs:548 | TIME_SEED_CHOSER_SLIDE_ON_END |  |  |
| src/lawn/cutscene.rs:549 | TIME_SEED_CHOSER_SLIDE_OFF_START |  |  |
| src/lawn/cutscene.rs:550 | TIME_SEED_CHOSER_SLIDE_OFF_END |  |  |
| src/lawn/cutscene.rs:568 | SEED_CHOOSER_OFFSET_Y |  |  |
| src/lawn/lawn_common.rs:17 | LAWN_EDIT_WIDGET_COLORS | pub |  |
| src/lawn/system/pool_effect.rs:14 | POOL_IMAGE_WIDTH |  |  |
| src/lawn/system/pool_effect.rs:15 | POOL_IMAGE_HEIGHT |  |  |
| src/lawn/widget/message_widget.rs:14 | MAX_REANIM_LINES | pub |  |
| src/lawn/zombie.rs:31 | DAMAGE_PER_EAT | pub |  |
| src/lawn/zombie.rs:10901 | CLIP_HEIGHT_LIMIT |  |  |
| src/lawn/zombie.rs:10904 | BUNGEE_ZOMBIE_HEIGHT |  |  |

### 3.5 未使用类型（结构体 / 枚举 / trait / 别名）

共 10 项。

| 位置 | 名称 | pub | C++ 出处 |
|---|---|---|---|
| src/lawn/board.rs:65 | RenderItem | pub |  |
| src/lawn/challenge.rs:17 | BeghouledUpgrade | pub |  |
| src/lawn/challenge.rs:56 | ZombieAllowedLevels | pub |  |
| src/lawn/lawn_common.rs:102 | LawnEditWidget | pub |  |
| src/lawn/projectile.rs:49 | ProjectileFlags | pub |  |
| src/lawn/projectile.rs:1482 | ProjectileDefinition | pub |  |
| src/lawn/seed_packet.rs:642 | SeedBank | pub |  |
| src/lawn/widget/achievements_screen.rs:130 | AchievementsWidget | pub |  |
| src/lawn/widget/title_screen.rs:44 | TitleScreenButtonId | pub |  |
| src/lawn/zombie.rs:103 | ZombieParts | pub |  |

### 3.6 未读取字段

共 51 项。

| 位置 | 名称 | pub | C++ 出处 |
|---|---|---|---|
| src/framework/sound/sdl_sound_instance.rs:45 | m_pan | pub |  |
| src/framework/sound/sdl_sound_instance.rs:50 | m_default_frequency | pub |  |
| src/lawn/board.rs:254 | m_width | pub |  |
| src/lawn/board.rs:255 | m_height | pub |  |
| src/lawn/board.rs:268 | grid_plants | pub |  |
| src/lawn/board.rs:307 | m_coin_bank | pub |  |
| src/lawn/board.rs:311 | m_wave_count | pub |  |
| src/lawn/board.rs:319 | m_wave_countdown | pub |  |
| src/lawn/board.rs:320 | m_flag_countdown | pub |  |
| src/lawn/board.rs:323 | m_game_over_countdown | pub |  |
| src/lawn/board.rs:391 | m_ice_particle | pub |  |
| src/lawn/coin.rs:26 | destination_x | pub |  |
| src/lawn/coin.rs:27 | destination_y | pub |  |
| src/lawn/lawn_app.rs:110 | control_button_list | pub |  |
| src/lawn/lawn_app.rs:111 | created_image_list | pub |  |
| src/lawn/lawn_app.rs:118 | m_num_levels | pub |  |
| src/lawn/lawn_app.rs:159 | m_games_played | pub |  |
| src/lawn/lawn_app.rs:160 | m_max_executions | pub |  |
| src/lawn/lawn_app.rs:161 | m_max_plays | pub |  |
| src/lawn/lawn_app.rs:162 | m_max_time | pub |  |
| src/lawn/lawn_app.rs:163 | m_play_time_active_session | pub |  |
| src/lawn/lawn_app.rs:164 | m_play_time_inactive_session | pub |  |
| src/lawn/lawn_app.rs:168 | m_dialog_id | pub |  |
| src/lawn/lawn_app.rs:169 | m_tutorial_state | pub |  |
| src/lawn/lawn_app.rs:172 | m_refer_id | pub |  |
| src/lawn/lawn_app.rs:173 | m_register_link | pub |  |
| src/lawn/lawn_app.rs:174 | m_mod | pub |  |
| src/lawn/lawn_app.rs:175 | m_register_resources_loaded | pub |  |
| src/lawn/lawn_app.rs:183 | m_loading_zombies_thread_completed | pub |  |
| src/lawn/lawn_app.rs:186 | m_session_id | pub |  |
| src/lawn/lawn_mower.rs:11 | app | pub |  |
| src/lawn/lawn_mower.rs:12 | board | pub |  |
| src/lawn/lawn_mower.rs:26 | mowing | pub |  |
| src/lawn/plant.rs:5791 | seed_type | pub |  |
| src/lawn/plant.rs:5792 | plant_image | pub |  |
| src/lawn/plant.rs:5794 | packet_index | pub |  |
| src/lawn/projectile.rs:100 | target_x | pub |  |
| src/lawn/projectile.rs:101 | target_y | pub |  |
| src/lawn/projectile.rs:108 | projectile_flags | pub |  |
| src/lawn/projectile.rs:112 | shadow | pub |  |
| src/lawn/seed_packet.rs:24 | can_afford | pub |  |
| src/lawn/seed_packet.rs:30 | offset_y | pub |  |
| src/lawn/widget/achievements_screen.rs:42 | description | pub |  |
| src/lawn/widget/award_screen.rs:14 | id | pub |  |
| src/lawn/widget/message_widget.rs:19 | app | pub |  |
| src/lawn/widget/message_widget.rs:29 | text_reanim_count | pub |  |
| src/lawn/widget/message_widget.rs:30 | text_reanim_byte_offset | pub |  |
| src/lawn/widget/title_screen.rs:70 | need_register | pub |  |
| src/lawn/widget/title_screen.rs:71 | register_clicked | pub |  |
| src/lawn/widget/title_screen.rs:72 | need_show_register_box | pub |  |
| src/lawn/zombie.rs:12295 | zombie_type | pub |  |

### 3.7 未构造枚举变体

共 885 项。

| 位置 | 名称 | pub | C++ 出处 |
|---|---|---|---|
| src/framework/resources.rs:9 | ImageBlank |  |  |
| src/framework/resources.rs:10 | ImagePopcapLogo |  |  |
| src/framework/resources.rs:11 | ImagePartnerLogo |  |  |
| src/framework/resources.rs:12 | ImageTitlescreen |  |  |
| src/framework/resources.rs:13 | ImageLoadbarDirt |  |  |
| src/framework/resources.rs:14 | ImageLoadbarGrass |  |  |
| src/framework/resources.rs:15 | ImagePvzLogo |  |  |
| src/framework/resources.rs:16 | ImageReanimSodrollcap |  |  |
| src/framework/resources.rs:17 | FontBrannetod16 |  |  |
| src/framework/resources.rs:19 | SoundLoadingbarFlower |  |  |
| src/framework/resources.rs:20 | SoundLoadingbarZombie |  |  |
| src/framework/resources.rs:21 | FontImageHouseofterror28 |  |  |
| src/framework/resources.rs:22 | FontHouseofterror28 |  |  |
| src/framework/resources.rs:23 | FontHouseofterror20 |  |  |
| src/framework/resources.rs:24 | FontHouseofterror16 |  |  |
| src/framework/resources.rs:25 | FontTinybold |  |  |
| src/framework/resources.rs:26 | FontContinuumbold14 |  |  |
| src/framework/resources.rs:27 | FontContinuumbold14Outline |  |  |
| src/framework/resources.rs:28 | FontDwarventodcraft12 |  |  |
| src/framework/resources.rs:29 | FontDwarventodcraft15 |  |  |
| src/framework/resources.rs:30 | FontDwarventodcraft18 |  |  |
| src/framework/resources.rs:31 | FontDwarventodcraft18Brightgreeninset |  |  |
| src/framework/resources.rs:32 | FontDwarventodcraft18Greeninset |  |  |
| src/framework/resources.rs:33 | FontDwarventodcraft18Yellow |  |  |
| src/framework/resources.rs:34 | FontDwarventodcraft24 |  |  |
| src/framework/resources.rs:35 | FontDwarventodcraft36Brightgreeninset |  |  |
| src/framework/resources.rs:36 | FontDwarventodcraft36Greeninset |  |  |
| src/framework/resources.rs:37 | FontPico129 |  |  |
| src/framework/resources.rs:38 | FontBrannetod12 |  |  |
| src/framework/resources.rs:39 | ImageEditbox |  |  |
| src/framework/resources.rs:40 | ImageDialogTopleft |  |  |
| src/framework/resources.rs:41 | ImageDialogTopmiddle |  |  |
| src/framework/resources.rs:42 | ImageDialogTopright |  |  |
| src/framework/resources.rs:43 | ImageDialogCenterleft |  |  |
| src/framework/resources.rs:44 | ImageDialogCentermiddle |  |  |
| src/framework/resources.rs:45 | ImageDialogCenterright |  |  |
| src/framework/resources.rs:46 | ImageDialogBottomleft |  |  |
| src/framework/resources.rs:47 | ImageDialogBottommiddle |  |  |
| src/framework/resources.rs:48 | ImageDialogBottomright |  |  |
| src/framework/resources.rs:49 | ImageDialogBigbottomleft |  |  |
| src/framework/resources.rs:50 | ImageDialogBigbottommiddle |  |  |
| src/framework/resources.rs:51 | ImageDialogBigbottomright |  |  |
| src/framework/resources.rs:52 | ImageDialogHeader |  |  |
| src/framework/resources.rs:53 | ImageButtonLeft |  |  |
| src/framework/resources.rs:54 | ImageButtonMiddle |  |  |
| src/framework/resources.rs:55 | ImageButtonRight |  |  |
| src/framework/resources.rs:56 | ImageButtonDownLeft |  |  |
| src/framework/resources.rs:57 | ImageButtonDownMiddle |  |  |
| src/framework/resources.rs:58 | ImageButtonDownRight |  |  |
| src/framework/resources.rs:59 | ImageSeedchooserBackground |  |  |
| src/framework/resources.rs:60 | ImageSeedchooserButton |  |  |
| src/framework/resources.rs:61 | ImageSeedchooserButtonDisabled |  |  |
| src/framework/resources.rs:62 | ImageSeedchooserButtonGlow |  |  |
| src/framework/resources.rs:63 | ImageSeedchooserButton2 |  |  |
| src/framework/resources.rs:64 | ImageSeedchooserButton2Glow |  |  |
| src/framework/resources.rs:65 | ImageSeedchooserImitateraddon |  |  |
| src/framework/resources.rs:66 | ImageSeeds |  |  |
| src/framework/resources.rs:67 | ImageSeedpacketLarger |  |  |
| src/framework/resources.rs:68 | ImageSeedpacketsilhouette |  |  |
| src/framework/resources.rs:69 | ImageFlagmeter |  |  |
| src/framework/resources.rs:70 | ImageFlagmeterlevelprogress |  |  |
| src/framework/resources.rs:71 | ImageFlagmeterparts |  |  |
| src/framework/resources.rs:72 | ImageTrophy |  |  |
| src/framework/resources.rs:73 | ImageTrophyHiRes |  |  |
| src/framework/resources.rs:74 | ImageMinigameTrophy |  |  |
| src/framework/resources.rs:75 | ImageZombieNoteSmall |  |  |
| src/framework/resources.rs:76 | ImageTaco |  |  |
| src/framework/resources.rs:77 | ImageCarkeys |  |  |
| src/framework/resources.rs:78 | ImageAlmanac |  |  |
| src/framework/resources.rs:79 | ImageIconPoolcleaner |  |  |
| src/framework/resources.rs:80 | ImageIconRoofcleaner |  |  |
| src/framework/resources.rs:81 | ImageIconRake |  |  |
| src/framework/resources.rs:82 | ImageBrain |  |  |
| src/framework/resources.rs:83 | ImageMoneybag |  |  |
| src/framework/resources.rs:84 | ImageMoneybagHiRes |  |  |
| src/framework/resources.rs:85 | ImageChocolate |  |  |
| src/framework/resources.rs:86 | ImageOptionsBacktogamebutton0 |  |  |
| src/framework/resources.rs:87 | ImageOptionsBacktogamebutton2 |  |  |
| src/framework/resources.rs:88 | ImageOptionsCheckbox0 |  |  |
| src/framework/resources.rs:89 | ImageOptionsCheckbox1 |  |  |
| src/framework/resources.rs:90 | ImageOptionsMenuback |  |  |
| src/framework/resources.rs:91 | ImageOptionsSliderknob2 |  |  |
| src/framework/resources.rs:92 | ImageOptionsSliderslot |  |  |
| src/framework/resources.rs:93 | ImageSelectorscreenHelp1 |  |  |
| src/framework/resources.rs:94 | ImageSelectorscreenHelp2 |  |  |
| src/framework/resources.rs:95 | ImageSelectorscreenOptions1 |  |  |
| src/framework/resources.rs:96 | ImageSelectorscreenOptions2 |  |  |
| src/framework/resources.rs:97 | ImageSelectorscreenQuit1 |  |  |
| src/framework/resources.rs:98 | ImageSelectorscreenQuit2 |  |  |
| src/framework/resources.rs:99 | ImageSelectorscreenAlmanac |  |  |
| src/framework/resources.rs:100 | ImageSelectorscreenAlmanachighlight |  |  |
| src/framework/resources.rs:101 | ImageSelectorscreenStore |  |  |
| src/framework/resources.rs:102 | ImageSelectorscreenStorehighlight |  |  |
| src/framework/resources.rs:103 | ImageSelectorscreenLevelnumbers |  |  |
| src/framework/resources.rs:104 | ImageSelectorscreenZengarden |  |  |
| src/framework/resources.rs:105 | ImageSelectorscreenZengardenhighlight |  |  |
| src/framework/resources.rs:106 | ImageStoreSpeechbubble |  |  |
| src/framework/resources.rs:107 | ImageStoreSpeechbubble2 |  |  |
| src/framework/resources.rs:108 | ImageSelectorscreenAchievementsBg |  |  |
| src/framework/resources.rs:109 | ImageSelectorscreenMorewaystoplayBg |  |  |
| src/framework/resources.rs:110 | ImageSelectorscreenAchievementsPedestal |  |  |
| src/framework/resources.rs:111 | ImageSelectorscreenAchievementsPedestalPress |  |  |
| src/framework/resources.rs:112 | ImageAcheesementsBejeweled |  |  |
| src/framework/resources.rs:113 | ImageAcheesementsBookworm |  |  |
| src/framework/resources.rs:114 | ImageAcheesementsChina |  |  |
| src/framework/resources.rs:115 | ImageAcheesementsChuzzle |  |  |
| src/framework/resources.rs:116 | ImageAcheesementsHoleTile |  |  |
| src/framework/resources.rs:117 | ImageAcheesementsPeggle |  |  |
| src/framework/resources.rs:118 | ImageAcheesementsPipe |  |  |
| src/framework/resources.rs:119 | ImageAcheesementsZuma |  |  |
| src/framework/resources.rs:120 | ImageAcheesementsIcons |  |  |
| src/framework/resources.rs:121 | ImageAcheesementsBackHighlight |  |  |
| src/framework/resources.rs:122 | ImageQuickplayMinigamesButton |  |  |
| src/framework/resources.rs:123 | ImageQuickplayMinigamesButtonHighlight |  |  |
| src/framework/resources.rs:124 | ImageQuickplayMinigamesCloud |  |  |
| src/framework/resources.rs:125 | ImageQuickplayPuzzlesButton |  |  |
| src/framework/resources.rs:126 | ImageQuickplayPuzzlesButtonHighlight |  |  |
| src/framework/resources.rs:127 | ImageQuickplayPuzzlesCloud |  |  |
| src/framework/resources.rs:128 | ImageQuickplaySurvivalButton |  |  |
| src/framework/resources.rs:129 | ImageQuickplaySurvivalButtonHighlight |  |  |
| src/framework/resources.rs:130 | ImageQuickplaySurvivalCloud |  |  |
| src/framework/resources.rs:131 | ImageQuickplayBackButton |  |  |
| src/framework/resources.rs:132 | ImageQuickplayBackButtonHighlight |  |  |
| src/framework/resources.rs:133 | ImageAcheesementsMoreRock |  |  |
| src/framework/resources.rs:134 | ImageAcheesementsMoreButton |  |  |
| src/framework/resources.rs:135 | ImageAcheesementsMoreButtonHighlight |  |  |
| src/framework/resources.rs:136 | ImageAcheesementsTopButton |  |  |
| src/framework/resources.rs:137 | ImageAcheesementsTopButtonHighlight |  |  |
| src/framework/resources.rs:138 | ImageShovel |  |  |
| src/framework/resources.rs:139 | ImageShovelHiRes |  |  |
| src/framework/resources.rs:140 | ImageTombstones |  |  |
| src/framework/resources.rs:141 | ImageTombstoneMounds |  |  |
| src/framework/resources.rs:142 | ImageNightGraveGraphic |  |  |
| src/framework/resources.rs:143 | ImageCrater |  |  |
| src/framework/resources.rs:144 | ImageCraterFading |  |  |
| src/framework/resources.rs:145 | ImageCraterRoofCenter |  |  |
| src/framework/resources.rs:146 | ImageCraterRoofLeft |  |  |
| src/framework/resources.rs:147 | ImageCraterWaterDay |  |  |
| src/framework/resources.rs:148 | ImageCraterWaterNight |  |  |
| src/framework/resources.rs:149 | ImageCobcannonTarget |  |  |
| src/framework/resources.rs:150 | ImageCobcannonPopcorn |  |  |
| src/framework/resources.rs:151 | ImageFertilizer |  |  |
| src/framework/resources.rs:152 | ImageWateringcan |  |  |
| src/framework/resources.rs:153 | ImageWateringcangold |  |  |
| src/framework/resources.rs:154 | ImagePhonograph |  |  |
| src/framework/resources.rs:155 | ImageBeghouledTwistOverlay |  |  |
| src/framework/resources.rs:156 | ImageZenGoldtoolreticle |  |  |
| src/framework/resources.rs:157 | ImageZenGardenglove |  |  |
| src/framework/resources.rs:158 | ImageZenMoneysign |  |  |
| src/framework/resources.rs:159 | ImageZenNextgarden |  |  |
| src/framework/resources.rs:160 | ImageZenWheelbarrow |  |  |
| src/framework/resources.rs:161 | ImageZenshopbutton |  |  |
| src/framework/resources.rs:162 | ImageZenshopbuttonHighlight |  |  |
| src/framework/resources.rs:163 | ImagePresent |  |  |
| src/framework/resources.rs:164 | ImagePresentopen |  |  |
| src/framework/resources.rs:165 | ImageSunflowerTrophy |  |  |
| src/framework/resources.rs:166 | ImageSlotmachineOverlay |  |  |
| src/framework/resources.rs:167 | ImageScaryPot |  |  |
| src/framework/resources.rs:168 | ImageBugSpray |  |  |
| src/framework/resources.rs:169 | ImagePlantspeechbubble |  |  |
| src/framework/resources.rs:170 | ImageWaterdrop |  |  |
| src/framework/resources.rs:171 | ImageTreefood |  |  |
| src/framework/resources.rs:172 | ImagePacketPlants |  |  |
| src/framework/resources.rs:173 | ImageZenNeedIcons |  |  |
| src/framework/resources.rs:174 | ImageProjectilepea |  |  |
| src/framework/resources.rs:175 | ImageProjectilesnowpea |  |  |
| src/framework/resources.rs:176 | ImageProjectilecactus |  |  |
| src/framework/resources.rs:177 | ImageDirtsmall |  |  |
| src/framework/resources.rs:178 | ImageDirtbig |  |  |
| src/framework/resources.rs:179 | ImageRocksmall |  |  |
| src/framework/resources.rs:180 | ImageWaterparticle |  |  |
| src/framework/resources.rs:181 | ImageWhitewater |  |  |
| src/framework/resources.rs:182 | ImageWhitewaterShadow |  |  |
| src/framework/resources.rs:183 | ImageMelonpultParticles |  |  |
| src/framework/resources.rs:184 | ImageWintermelonParticles |  |  |
| src/framework/resources.rs:185 | ImageProjectileStar |  |  |
| src/framework/resources.rs:186 | ImageSeedbank |  |  |
| src/framework/resources.rs:187 | ImageConveyorbeltBackdrop |  |  |
| src/framework/resources.rs:188 | ImageConveyorbelt |  |  |
| src/framework/resources.rs:189 | ImageShovelbank |  |  |
| src/framework/resources.rs:190 | ImageSunbank |  |  |
| src/framework/resources.rs:191 | ImageCoinbank |  |  |
| src/framework/resources.rs:192 | ImagePool |  |  |
| src/framework/resources.rs:193 | ImagePoolNight |  |  |
| src/framework/resources.rs:194 | ImagePoolBase |  |  |
| src/framework/resources.rs:195 | ImagePoolBaseNight |  |  |
| src/framework/resources.rs:196 | ImagePoolShading |  |  |
| src/framework/resources.rs:197 | ImagePoolShadingNight |  |  |
| src/framework/resources.rs:198 | ImagePoolCausticEffect |  |  |
| src/framework/resources.rs:199 | ImagePlantshadow |  |  |
| src/framework/resources.rs:200 | ImagePlantshadow2 |  |  |
| src/framework/resources.rs:201 | ImagePeaShadows |  |  |
| src/framework/resources.rs:202 | ImageWallnutBowlingstripe |  |  |
| src/framework/resources.rs:203 | ImageIce |  |  |
| src/framework/resources.rs:204 | ImageIceCap |  |  |
| src/framework/resources.rs:205 | ImageIceSparkles |  |  |
| src/framework/resources.rs:206 | ImageImitaterseed |  |  |
| src/framework/resources.rs:207 | ImageAlmanacImitater |  |  |
| src/framework/resources.rs:208 | ImageImitaterseeddisabled |  |  |
| src/framework/resources.rs:209 | ImageIcetrap |  |  |
| src/framework/resources.rs:210 | ImageIcetrap2 |  |  |
| src/framework/resources.rs:211 | ImageIcetrapParticles |  |  |
| src/framework/resources.rs:212 | ImageZombieBobsled1 |  |  |
| src/framework/resources.rs:213 | ImageZombieBobsled2 |  |  |
| src/framework/resources.rs:214 | ImageZombieBobsled3 |  |  |
| src/framework/resources.rs:215 | ImageZombieBobsled4 |  |  |
| src/framework/resources.rs:216 | ImageZombieBobsledInside |  |  |
| src/framework/resources.rs:217 | ImageBungeecord |  |  |
| src/framework/resources.rs:218 | ImageBungeetarget |  |  |
| src/framework/resources.rs:219 | ImageSpotlight |  |  |
| src/framework/resources.rs:220 | ImageSpotlight2 |  |  |
| src/framework/resources.rs:221 | ImageWhitepixel |  |  |
| src/framework/resources.rs:222 | ImageZombiepolevaulterhead |  |  |
| src/framework/resources.rs:223 | ImageZombiefootballhead |  |  |
| src/framework/resources.rs:224 | ImagePoolsparkly |  |  |
| src/framework/resources.rs:225 | ImageWallnutparticlessmall |  |  |
| src/framework/resources.rs:226 | ImageWallnutparticleslarge |  |  |
| src/framework/resources.rs:227 | ImagePeaSplats |  |  |
| src/framework/resources.rs:228 | ImageStarParticles |  |  |
| src/framework/resources.rs:229 | ImageStarSplats |  |  |
| src/framework/resources.rs:230 | ImagePeaParticles |  |  |
| src/framework/resources.rs:231 | ImageSnowpeaSplats |  |  |
| src/framework/resources.rs:232 | ImageSnowpeaParticles |  |  |
| src/framework/resources.rs:233 | ImageSnowpeaPuff |  |  |
| src/framework/resources.rs:234 | ImageSnowflakes |  |  |
| src/framework/resources.rs:235 | ImagePotatomineParticles |  |  |
| src/framework/resources.rs:236 | ImagePuffshroomPuff1 |  |  |
| src/framework/resources.rs:237 | ImageZambonismoke |  |  |
| src/framework/resources.rs:238 | ImageZombiedancerhead |  |  |
| src/framework/resources.rs:239 | ImageZombieballoonhead |  |  |
| src/framework/resources.rs:240 | ImageZombieimphead |  |  |
| src/framework/resources.rs:241 | ImageZombiediggerhead |  |  |
| src/framework/resources.rs:242 | ImageZombiediggerarm |  |  |
| src/framework/resources.rs:243 | ImageZombiedolphinriderhead |  |  |
| src/framework/resources.rs:244 | ImageZombiepogo |  |  |
| src/framework/resources.rs:245 | ImageZombiebackupdancerhead |  |  |
| src/framework/resources.rs:246 | ImageZombiebobsledhead |  |  |
| src/framework/resources.rs:247 | ImageZombieladderhead |  |  |
| src/framework/resources.rs:248 | ImageZombieyetihead |  |  |
| src/framework/resources.rs:249 | ImageSeedpacketflash |  |  |
| src/framework/resources.rs:250 | ImageZombiejackboxarm |  |  |
| src/framework/resources.rs:251 | ImageImitaterclouds |  |  |
| src/framework/resources.rs:252 | ImageImitaterpuffs |  |  |
| src/framework/resources.rs:253 | ImageZombieBossFireballParticles |  |  |
| src/framework/resources.rs:254 | ImageZombieBossIceballParticles |  |  |
| src/framework/resources.rs:255 | ImageZombieBossFireballGroundparticles |  |  |
| src/framework/resources.rs:256 | ImageZombieBossIceballGroundparticles |  |  |
| src/framework/resources.rs:257 | ImageDoomshroomExplosionBase |  |  |
| src/framework/resources.rs:258 | ImageRain |  |  |
| src/framework/resources.rs:259 | ImageVaseChunks |  |  |
| src/framework/resources.rs:260 | ImageZombossParticles |  |  |
| src/framework/resources.rs:261 | ImageAwardpickupglow |  |  |
| src/framework/resources.rs:262 | ImageZombieSeaweed |  |  |
| src/framework/resources.rs:263 | ImagePinata |  |  |
| src/framework/resources.rs:264 | ImageZombiefutureglasses |  |  |
| src/framework/resources.rs:265 | ImageDustPuffs |  |  |
| src/framework/resources.rs:266 | ImageReanimWallnutBody |  |  |
| src/framework/resources.rs:267 | ImageReanimWallnutCracked1 |  |  |
| src/framework/resources.rs:268 | ImageReanimWallnutCracked2 |  |  |
| src/framework/resources.rs:269 | ImageReanimTallnutCracked1 |  |  |
| src/framework/resources.rs:270 | ImageReanimTallnutCracked2 |  |  |
| src/framework/resources.rs:271 | ImageReanimPumpkinDamage1 |  |  |
| src/framework/resources.rs:272 | ImageReanimPumpkinDamage3 |  |  |
| src/framework/resources.rs:273 | ImageReanimZombieCone1 |  |  |
| src/framework/resources.rs:274 | ImageReanimZombieCone2 |  |  |
| src/framework/resources.rs:275 | ImageReanimZombieCone3 |  |  |
| src/framework/resources.rs:276 | ImageReanimZombieBucket1 |  |  |
| src/framework/resources.rs:277 | ImageReanimZombieBucket2 |  |  |
| src/framework/resources.rs:278 | ImageReanimZombieBucket3 |  |  |
| src/framework/resources.rs:279 | ImageReanimZombieDiggerHardhat |  |  |
| src/framework/resources.rs:280 | ImageReanimZombieDiggerHardhat2 |  |  |
| src/framework/resources.rs:281 | ImageReanimZombieDiggerHardhat3 |  |  |
| src/framework/resources.rs:282 | ImageReanimZombieScreendoor1 |  |  |
| src/framework/resources.rs:283 | ImageReanimZombieScreendoor2 |  |  |
| src/framework/resources.rs:284 | ImageReanimZombieScreendoor3 |  |  |
| src/framework/resources.rs:285 | ImageReanimZombieFlag1 |  |  |
| src/framework/resources.rs:286 | ImageReanimZombieFlag3 |  |  |
| src/framework/resources.rs:287 | ImageReanimZombieOuterarmUpper2 |  |  |
| src/framework/resources.rs:288 | ImageReanimZombieFootballHelmet |  |  |
| src/framework/resources.rs:289 | ImageReanimZombieFootballHelmet2 |  |  |
| src/framework/resources.rs:290 | ImageReanimZombieFootballHelmet3 |  |  |
| src/framework/resources.rs:291 | ImageReanimZombieFootballLeftarmHand |  |  |
| src/framework/resources.rs:292 | ImageReanimZombieFootballLeftarmUpper2 |  |  |
| src/framework/resources.rs:293 | ImageReanimZombiePaperLeftarmUpper2 |  |  |
| src/framework/resources.rs:294 | ImageReanimZombiePaperLeftarmLower |  |  |
| src/framework/resources.rs:295 | ImageReanimZombiePaperPaper2 |  |  |
| src/framework/resources.rs:296 | ImageReanimZombiePaperPaper3 |  |  |
| src/framework/resources.rs:297 | ImageReanimZombiePaperMadhead |  |  |
| src/framework/resources.rs:298 | ImageReanimZombieZamboni1Damage1 |  |  |
| src/framework/resources.rs:299 | ImageReanimZombieZamboni1Damage2 |  |  |
| src/framework/resources.rs:300 | ImageReanimZombieZamboni2Damage1 |  |  |
| src/framework/resources.rs:301 | ImageReanimZombieZamboni2Damage2 |  |  |
| src/framework/resources.rs:302 | ImageReanimZombieZamboniWheelFlat |  |  |
| src/framework/resources.rs:303 | ImageReanimCoinSilverDollar |  |  |
| src/framework/resources.rs:304 | ImageReanimCoinGoldDollar |  |  |
| src/framework/resources.rs:305 | ImageReanimDiamond |  |  |
| src/framework/resources.rs:306 | ImageReanimCoinglow |  |  |
| src/framework/resources.rs:307 | ImageReanimZombieDiscoOuterarmHand |  |  |
| src/framework/resources.rs:308 | ImageReanimZombieDiscoOuterarmUpper2 |  |  |
| src/framework/resources.rs:309 | ImageReanimZombieBackupInnerarmHand |  |  |
| src/framework/resources.rs:310 | ImageReanimZombieBackupOuterarmUpper2 |  |  |
| src/framework/resources.rs:311 | ImageReanimZombiePolevaulterOuterarmUpper2 |  |  |
| src/framework/resources.rs:312 | ImageReanimZombieOuterarmHand |  |  |
| src/framework/resources.rs:313 | ImageReanimZombieBalloonOuterarmUpper2 |  |  |
| src/framework/resources.rs:314 | ImageReanimZombieImpArm1Bone |  |  |
| src/framework/resources.rs:315 | ImageReanimZombieImpArm2 |  |  |
| src/framework/resources.rs:316 | ImageReanimZombieGargantuarBody12 |  |  |
| src/framework/resources.rs:317 | ImageReanimZombieGargantuarBody13 |  |  |
| src/framework/resources.rs:318 | ImageReanimZombieGargantuarDuckxing |  |  |
| src/framework/resources.rs:319 | ImageReanimZombieGargantuarZombie |  |  |
| src/framework/resources.rs:320 | ImageReanimZombieGargantuarFoot2 |  |  |
| src/framework/resources.rs:321 | ImageReanimZombieGargantuarHead2 |  |  |
| src/framework/resources.rs:322 | ImageReanimZombieGargantuarOuterarmLower2 |  |  |
| src/framework/resources.rs:323 | ImageReanimZombieGargantuarHead2Redeye |  |  |
| src/framework/resources.rs:324 | ImageReanimZombieGargantuarHeadRedeye |  |  |
| src/framework/resources.rs:325 | ImageReanimZombieDiggerOuterarmUpper2 |  |  |
| src/framework/resources.rs:326 | ImageReanimZombieDolphinriderOuterarmUpper2 |  |  |
| src/framework/resources.rs:327 | ImageReanimZombiePogoOuterarmUpper2 |  |  |
| src/framework/resources.rs:328 | ImageReanimZombiePogoStickdamage1 |  |  |
| src/framework/resources.rs:329 | ImageReanimZombiePogoStickdamage2 |  |  |
| src/framework/resources.rs:330 | ImageReanimZombiePogoStick2damage1 |  |  |
| src/framework/resources.rs:331 | ImageReanimZombiePogoStick2damage2 |  |  |
| src/framework/resources.rs:332 | ImageReanimZombiePogoStickhands2 |  |  |
| src/framework/resources.rs:333 | ImageReanimZombieBobsledOuterarmUpper2 |  |  |
| src/framework/resources.rs:334 | ImageReanimZombieBobsledOuterarmHand |  |  |
| src/framework/resources.rs:335 | ImageReanimZombieJackboxBox |  |  |
| src/framework/resources.rs:336 | ImageReanimZombieJackboxOuterarmLower2 |  |  |
| src/framework/resources.rs:337 | ImageReanimZombieSnorkleOuterarmUpper2 |  |  |
| src/framework/resources.rs:338 | ImageReanimZombieBungiHeadScared |  |  |
| src/framework/resources.rs:339 | ImageReanimZombieCatapultBasketball |  |  |
| src/framework/resources.rs:340 | ImageReanimZombieCatapultPole |  |  |
| src/framework/resources.rs:341 | ImageReanimZombieCatapultPoleDamage |  |  |
| src/framework/resources.rs:342 | ImageReanimZombieCatapultPoleWithball |  |  |
| src/framework/resources.rs:343 | ImageReanimZombieCatapultPoleDamageWithball |  |  |
| src/framework/resources.rs:344 | ImageReanimZombieCatapultSidingDamage |  |  |
| src/framework/resources.rs:345 | ImageReanimZombieLadder1 |  |  |
| src/framework/resources.rs:346 | ImageReanimZombieLadder1Damage1 |  |  |
| src/framework/resources.rs:347 | ImageReanimZombieLadder1Damage2 |  |  |
| src/framework/resources.rs:348 | ImageReanimZombieLadder5 |  |  |
| src/framework/resources.rs:349 | ImageReanimZombieLadderOuterarmUpper2 |  |  |
| src/framework/resources.rs:350 | ImageReanimZombieBossIceball |  |  |
| src/framework/resources.rs:351 | ImageReanimZombieBossHeadDamage1 |  |  |
| src/framework/resources.rs:352 | ImageReanimZombieBossHeadDamage2 |  |  |
| src/framework/resources.rs:353 | ImageReanimZombieBossJawDamage1 |  |  |
| src/framework/resources.rs:354 | ImageReanimZombieBossJawDamage2 |  |  |
| src/framework/resources.rs:355 | ImageReanimZombieBossOuterarmHandDamage1 |  |  |
| src/framework/resources.rs:356 | ImageReanimZombieBossOuterarmHandDamage2 |  |  |
| src/framework/resources.rs:357 | ImageReanimZombieBossOuterarmThumbDamage1 |  |  |
| src/framework/resources.rs:358 | ImageReanimZombieBossOuterarmThumbDamage2 |  |  |
| src/framework/resources.rs:359 | ImageReanimZombieBossFootDamage1 |  |  |
| src/framework/resources.rs:360 | ImageReanimZombieBossFootDamage2 |  |  |
| src/framework/resources.rs:361 | ImageReanimZombieBossEyeglowBlue |  |  |
| src/framework/resources.rs:362 | ImageReanimZombieBossMouthglowBlue |  |  |
| src/framework/resources.rs:363 | ImageReanimCabbagepultCabbage |  |  |
| src/framework/resources.rs:364 | ImageReanimCornpultKernal |  |  |
| src/framework/resources.rs:365 | ImageReanimCornpultButter |  |  |
| src/framework/resources.rs:366 | ImageReanimCornpultButterSplat |  |  |
| src/framework/resources.rs:367 | ImageReanimMelonpultMelon |  |  |
| src/framework/resources.rs:368 | ImageReanimWintermelonProjectile |  |  |
| src/framework/resources.rs:369 | ImageReanimZombieDiggerPickaxe |  |  |
| src/framework/resources.rs:370 | ImageReanimZombieHeadGrossout |  |  |
| src/framework/resources.rs:371 | ImageReanimZombieHeadSunglasses1 |  |  |
| src/framework/resources.rs:372 | ImageReanimZombieHeadSunglasses2 |  |  |
| src/framework/resources.rs:373 | ImageReanimZombieHeadSunglasses3 |  |  |
| src/framework/resources.rs:374 | ImageReanimZombieHeadSunglasses4 |  |  |
| src/framework/resources.rs:375 | ImageReanimGarlicBody2 |  |  |
| src/framework/resources.rs:376 | ImageReanimGarlicBody3 |  |  |
| src/framework/resources.rs:377 | ImageReanimCobcannonCob |  |  |
| src/framework/resources.rs:378 | ImageReanimZombieYetiOuterarmUpper2 |  |  |
| src/framework/resources.rs:379 | ImageReanimZombieYetiOuterarmHand |  |  |
| src/framework/resources.rs:380 | ImageReanimZombieSnorkleHead |  |  |
| src/framework/resources.rs:381 | ImageReanimSelectorscreenAdventureButton |  |  |
| src/framework/resources.rs:382 | ImageReanimSelectorscreenAdventureHighlight |  |  |
| src/framework/resources.rs:383 | ImageReanimSelectorscreenStartadventureButton |  |  |
| src/framework/resources.rs:384 | ImageReanimSelectorscreenStartadventureHighlight |  |  |
| src/framework/resources.rs:385 | ImageReanimSelectorscreenSurvivalButton |  |  |
| src/framework/resources.rs:386 | ImageReanimSelectorscreenSurvivalHighlight |  |  |
| src/framework/resources.rs:387 | ImageReanimSelectorscreenChallengesButton |  |  |
| src/framework/resources.rs:388 | ImageReanimSelectorscreenChallengesHighlight |  |  |
| src/framework/resources.rs:389 | ImageReanimSelectorscreenVasebreakerButton |  |  |
| src/framework/resources.rs:390 | ImageReanimSelectorscreenVasebreakerHighlight |  |  |
| src/framework/resources.rs:391 | ImageReanimSelectorscreenWoodsign2Press |  |  |
| src/framework/resources.rs:392 | ImageReanimSelectorscreenWoodsign3Press |  |  |
| src/framework/resources.rs:393 | ImageReanimSelectorscreenBg |  |  |
| src/framework/resources.rs:394 | ImageReanimSelectorscreenAlmanacShadow |  |  |
| src/framework/resources.rs:395 | ImageReanimSelectorscreenKeyShadow |  |  |
| src/framework/resources.rs:396 | ImageReanimTreeBg |  |  |
| src/framework/resources.rs:397 | ImageReanimPotTopDark |  |  |
| src/framework/resources.rs:398 | ImageReanimZengardenBugsprayBottle |  |  |
| src/framework/resources.rs:399 | ImageReanimStinkyTurn3 |  |  |
| src/framework/resources.rs:400 | ImageReanimCrazydaveMouth1 |  |  |
| src/framework/resources.rs:401 | ImageReanimCrazydaveMouth4 |  |  |
| src/framework/resources.rs:402 | ImageReanimCrazydaveMouth5 |  |  |
| src/framework/resources.rs:403 | ImageReanimCrazydaveMouth6 |  |  |
| src/framework/resources.rs:404 | ImageReanimZombieMustache2 |  |  |
| src/framework/resources.rs:405 | ImageReanimZombieMustache3 |  |  |
| src/framework/resources.rs:406 | SoundAchievement |  |  |
| src/framework/resources.rs:407 | SoundAwooga |  |  |
| src/framework/resources.rs:408 | SoundBleep |  |  |
| src/framework/resources.rs:410 | SoundChomp |  |  |
| src/framework/resources.rs:411 | SoundChomp2 |  |  |
| src/framework/resources.rs:412 | SoundChompsoft |  |  |
| src/framework/resources.rs:413 | SoundEvillaugh |  |  |
| src/framework/resources.rs:414 | SoundFloop |  |  |
| src/framework/resources.rs:415 | SoundFrozen |  |  |
| src/framework/resources.rs:416 | SoundGulp |  |  |
| src/framework/resources.rs:417 | SoundGroan |  |  |
| src/framework/resources.rs:418 | SoundGroan2 |  |  |
| src/framework/resources.rs:419 | SoundGroan3 |  |  |
| src/framework/resources.rs:420 | SoundGroan4 |  |  |
| src/framework/resources.rs:421 | SoundGroan5 |  |  |
| src/framework/resources.rs:422 | SoundGroan6 |  |  |
| src/framework/resources.rs:425 | SoundPause |  |  |
| src/framework/resources.rs:426 | SoundPlant |  |  |
| src/framework/resources.rs:427 | SoundPlant2 |  |  |
| src/framework/resources.rs:428 | SoundPoints |  |  |
| src/framework/resources.rs:429 | SoundSeedlift |  |  |
| src/framework/resources.rs:430 | SoundSiren |  |  |
| src/framework/resources.rs:431 | SoundSlurp |  |  |
| src/framework/resources.rs:432 | SoundSplat |  |  |
| src/framework/resources.rs:433 | SoundSplat2 |  |  |
| src/framework/resources.rs:434 | SoundSplat3 |  |  |
| src/framework/resources.rs:435 | SoundSukhbir |  |  |
| src/framework/resources.rs:436 | SoundSukhbir2 |  |  |
| src/framework/resources.rs:437 | SoundSukhbir3 |  |  |
| src/framework/resources.rs:438 | SoundSukhbir4 |  |  |
| src/framework/resources.rs:439 | SoundSukhbir5 |  |  |
| src/framework/resources.rs:440 | SoundSukhbir6 |  |  |
| src/framework/resources.rs:442 | SoundTap2 |  |  |
| src/framework/resources.rs:443 | SoundThrow |  |  |
| src/framework/resources.rs:444 | SoundThrow2 |  |  |
| src/framework/resources.rs:445 | SoundBlover |  |  |
| src/framework/resources.rs:446 | SoundWinmusic |  |  |
| src/framework/resources.rs:447 | SoundLawnmower |  |  |
| src/framework/resources.rs:448 | SoundBoing |  |  |
| src/framework/resources.rs:449 | SoundJackinthebox |  |  |
| src/framework/resources.rs:451 | SoundDolphinAppears |  |  |
| src/framework/resources.rs:452 | SoundDolphinBeforeJumping |  |  |
| src/framework/resources.rs:453 | SoundPotatoMine |  |  |
| src/framework/resources.rs:454 | SoundZamboni |  |  |
| src/framework/resources.rs:455 | SoundBalloonPop |  |  |
| src/framework/resources.rs:456 | SoundThunder |  |  |
| src/framework/resources.rs:457 | SoundZombiesplash |  |  |
| src/framework/resources.rs:458 | SoundBowling |  |  |
| src/framework/resources.rs:459 | SoundBowlingimpact |  |  |
| src/framework/resources.rs:461 | SoundGravebusterchomp |  |  |
| src/framework/resources.rs:463 | SoundLimbsPop |  |  |
| src/framework/resources.rs:464 | SoundPlantern |  |  |
| src/framework/resources.rs:465 | SoundPogoZombie |  |  |
| src/framework/resources.rs:466 | SoundSnowPeaSparkles |  |  |
| src/framework/resources.rs:467 | SoundPlantWater |  |  |
| src/framework/resources.rs:468 | SoundZombieEnteringWater |  |  |
| src/framework/resources.rs:469 | SoundZombieFalling1 |  |  |
| src/framework/resources.rs:470 | SoundZombieFalling2 |  |  |
| src/framework/resources.rs:471 | SoundPuff |  |  |
| src/framework/resources.rs:472 | SoundFume |  |  |
| src/framework/resources.rs:473 | SoundHugeWave |  |  |
| src/framework/resources.rs:474 | SoundSlotMachine |  |  |
| src/framework/resources.rs:475 | SoundCoin |  |  |
| src/framework/resources.rs:476 | SoundRollIn |  |  |
| src/framework/resources.rs:477 | SoundDiggerZombie |  |  |
| src/framework/resources.rs:480 | SoundKernelpult |  |  |
| src/framework/resources.rs:481 | SoundKernelpult2 |  |  |
| src/framework/resources.rs:482 | SoundZombaquariumDie |  |  |
| src/framework/resources.rs:483 | SoundBungeeScream |  |  |
| src/framework/resources.rs:484 | SoundBungeeScream2 |  |  |
| src/framework/resources.rs:485 | SoundBungeeScream3 |  |  |
| src/framework/resources.rs:486 | SoundButter |  |  |
| src/framework/resources.rs:487 | SoundJackSurprise |  |  |
| src/framework/resources.rs:488 | SoundJackSurprise2 |  |  |
| src/framework/resources.rs:489 | SoundNewspaperRarrgh |  |  |
| src/framework/resources.rs:490 | SoundNewspaperRarrgh2 |  |  |
| src/framework/resources.rs:491 | SoundNewspaperRip |  |  |
| src/framework/resources.rs:492 | SoundSquashHmm |  |  |
| src/framework/resources.rs:493 | SoundSquashHmm2 |  |  |
| src/framework/resources.rs:494 | SoundVaseBreaking |  |  |
| src/framework/resources.rs:495 | SoundPoolCleaner |  |  |
| src/framework/resources.rs:496 | SoundMagnetshroom |  |  |
| src/framework/resources.rs:497 | SoundLadderZombie |  |  |
| src/framework/resources.rs:498 | SoundGargantuarThump |  |  |
| src/framework/resources.rs:499 | SoundBasketball |  |  |
| src/framework/resources.rs:500 | SoundFirepea |  |  |
| src/framework/resources.rs:501 | SoundIgnite |  |  |
| src/framework/resources.rs:502 | SoundIgnite2 |  |  |
| src/framework/resources.rs:503 | SoundReadysetplant |  |  |
| src/framework/resources.rs:504 | SoundDoomshroom |  |  |
| src/framework/resources.rs:505 | SoundExplosion |  |  |
| src/framework/resources.rs:506 | SoundFinalwave |  |  |
| src/framework/resources.rs:507 | SoundReverseExplosion |  |  |
| src/framework/resources.rs:509 | SoundShieldhit |  |  |
| src/framework/resources.rs:510 | SoundShieldhit2 |  |  |
| src/framework/resources.rs:512 | SoundCherrybomb |  |  |
| src/framework/resources.rs:513 | SoundBonk |  |  |
| src/framework/resources.rs:514 | SoundSwing |  |  |
| src/framework/resources.rs:515 | SoundRain |  |  |
| src/framework/resources.rs:516 | SoundLightfill |  |  |
| src/framework/resources.rs:517 | SoundPlastichit |  |  |
| src/framework/resources.rs:518 | SoundPlastichit2 |  |  |
| src/framework/resources.rs:519 | SoundJalapeno |  |  |
| src/framework/resources.rs:520 | SoundBallooninflate |  |  |
| src/framework/resources.rs:521 | SoundBigchomp |  |  |
| src/framework/resources.rs:522 | SoundMelonimpact |  |  |
| src/framework/resources.rs:523 | SoundMelonimpact2 |  |  |
| src/framework/resources.rs:524 | SoundPlantgrow |  |  |
| src/framework/resources.rs:525 | SoundShoop |  |  |
| src/framework/resources.rs:526 | SoundTapglass |  |  |
| src/framework/resources.rs:527 | SoundJuicy |  |  |
| src/framework/resources.rs:528 | SoundCoffee |  |  |
| src/framework/resources.rs:529 | SoundWakeup |  |  |
| src/framework/resources.rs:530 | SoundLowgroan |  |  |
| src/framework/resources.rs:531 | SoundLowgroan2 |  |  |
| src/framework/resources.rs:532 | SoundPrize |  |  |
| src/framework/resources.rs:533 | SoundYuck |  |  |
| src/framework/resources.rs:534 | SoundYuck2 |  |  |
| src/framework/resources.rs:535 | SoundGrassstep |  |  |
| src/framework/resources.rs:536 | SoundShovel |  |  |
| src/framework/resources.rs:537 | SoundCoblaunch |  |  |
| src/framework/resources.rs:538 | SoundWatering |  |  |
| src/framework/resources.rs:539 | SoundPolevault |  |  |
| src/framework/resources.rs:540 | SoundGravestoneRumble |  |  |
| src/framework/resources.rs:541 | SoundDirtRise |  |  |
| src/framework/resources.rs:542 | SoundFertilizer |  |  |
| src/framework/resources.rs:543 | SoundPortal |  |  |
| src/framework/resources.rs:544 | SoundBugspray |  |  |
| src/framework/resources.rs:545 | SoundScream |  |  |
| src/framework/resources.rs:546 | SoundPaper |  |  |
| src/framework/resources.rs:547 | SoundMoneyfalls |  |  |
| src/framework/resources.rs:548 | SoundImp |  |  |
| src/framework/resources.rs:549 | SoundImp2 |  |  |
| src/framework/resources.rs:550 | SoundHydraulicShort |  |  |
| src/framework/resources.rs:551 | SoundHydraulic |  |  |
| src/framework/resources.rs:552 | SoundGargantudeath |  |  |
| src/framework/resources.rs:553 | SoundCeramic |  |  |
| src/framework/resources.rs:554 | SoundBossboulderattack |  |  |
| src/framework/resources.rs:555 | SoundChime |  |  |
| src/framework/resources.rs:556 | SoundCrazydaveshort1 |  |  |
| src/framework/resources.rs:557 | SoundCrazydaveshort2 |  |  |
| src/framework/resources.rs:558 | SoundCrazydaveshort3 |  |  |
| src/framework/resources.rs:559 | SoundCrazydavelong1 |  |  |
| src/framework/resources.rs:560 | SoundCrazydavelong2 |  |  |
| src/framework/resources.rs:561 | SoundCrazydavelong3 |  |  |
| src/framework/resources.rs:562 | SoundCrazydaveextralong1 |  |  |
| src/framework/resources.rs:563 | SoundCrazydaveextralong2 |  |  |
| src/framework/resources.rs:564 | SoundCrazydaveextralong3 |  |  |
| src/framework/resources.rs:565 | SoundCrazydavecrazy |  |  |
| src/framework/resources.rs:566 | SoundPhonograph |  |  |
| src/framework/resources.rs:567 | SoundDancer |  |  |
| src/framework/resources.rs:568 | SoundFinalfanfare |  |  |
| src/framework/resources.rs:569 | SoundCrazydavescream |  |  |
| src/framework/resources.rs:570 | SoundCrazydavescream2 |  |  |
| src/framework/resources.rs:571 | ImageBackground1 |  |  |
| src/framework/resources.rs:572 | ImageBackground1GameoverInteriorOverlay |  |  |
| src/framework/resources.rs:573 | ImageBackground1GameoverMask |  |  |
| src/framework/resources.rs:574 | ImageBackground1Unsodded |  |  |
| src/framework/resources.rs:575 | ImageSod1row |  |  |
| src/framework/resources.rs:576 | ImageSod3row |  |  |
| src/framework/resources.rs:577 | ImageBackground2 |  |  |
| src/framework/resources.rs:578 | ImageBackground2GameoverInteriorOverlay |  |  |
| src/framework/resources.rs:579 | ImageBackground2GameoverMask |  |  |
| src/framework/resources.rs:580 | ImageBackground3 |  |  |
| src/framework/resources.rs:581 | ImageBackground3GameoverInteriorOverlay |  |  |
| src/framework/resources.rs:582 | ImageBackground3GameoverMask |  |  |
| src/framework/resources.rs:583 | ImageBackground4 |  |  |
| src/framework/resources.rs:584 | ImageBackground4GameoverInteriorOverlay |  |  |
| src/framework/resources.rs:585 | ImageBackground4GameoverMask |  |  |
| src/framework/resources.rs:586 | ImageFog |  |  |
| src/framework/resources.rs:587 | ImageFogSoftware |  |  |
| src/framework/resources.rs:588 | ImageBackground5 |  |  |
| src/framework/resources.rs:589 | ImageBackground5GameoverMask |  |  |
| src/framework/resources.rs:590 | ImageBackground6boss |  |  |
| src/framework/resources.rs:591 | ImageBackground6GameoverMask |  |  |
| src/framework/resources.rs:592 | ImageBackgroundMushroomgarden |  |  |
| src/framework/resources.rs:593 | ImageBackgroundGreenhouse |  |  |
| src/framework/resources.rs:594 | ImageBackgroundGreenhouseOverlay |  |  |
| src/framework/resources.rs:595 | ImageAquarium1 |  |  |
| src/framework/resources.rs:596 | ImageWavecenter |  |  |
| src/framework/resources.rs:597 | ImageWaveside |  |  |
| src/framework/resources.rs:598 | ImageAwardscreenBack |  |  |
| src/framework/resources.rs:599 | ImageChallengeBackground |  |  |
| src/framework/resources.rs:600 | ImageChallengeWindow |  |  |
| src/framework/resources.rs:601 | ImageChallengeWindowHighlight |  |  |
| src/framework/resources.rs:602 | ImageChallengeBlank |  |  |
| src/framework/resources.rs:603 | ImageChallengeThumbnails |  |  |
| src/framework/resources.rs:604 | ImageSurvivalThumbnails |  |  |
| src/framework/resources.rs:605 | ImageLock |  |  |
| src/framework/resources.rs:606 | ImageLockOpen |  |  |
| src/framework/resources.rs:607 | ImageAlmanacIndexback |  |  |
| src/framework/resources.rs:608 | ImageAlmanacPlantback |  |  |
| src/framework/resources.rs:609 | ImageAlmanacZombieback |  |  |
| src/framework/resources.rs:610 | ImageAlmanacPlantcard |  |  |
| src/framework/resources.rs:611 | ImageAlmanacZombiecard |  |  |
| src/framework/resources.rs:612 | ImageAlmanacZombiewindow |  |  |
| src/framework/resources.rs:613 | ImageAlmanacZombiewindow2 |  |  |
| src/framework/resources.rs:614 | ImageAlmanacZombieblank |  |  |
| src/framework/resources.rs:615 | ImageAlmanacGroundday |  |  |
| src/framework/resources.rs:616 | ImageAlmanacGroundnight |  |  |
| src/framework/resources.rs:617 | ImageAlmanacGroundpool |  |  |
| src/framework/resources.rs:618 | ImageAlmanacGroundnightpool |  |  |
| src/framework/resources.rs:619 | ImageAlmanacGroundroof |  |  |
| src/framework/resources.rs:620 | ImageAlmanacGroundice |  |  |
| src/framework/resources.rs:621 | ImageAlmanacClosebutton |  |  |
| src/framework/resources.rs:622 | ImageAlmanacClosebuttonhighlight |  |  |
| src/framework/resources.rs:623 | ImageAlmanacIndexbutton |  |  |
| src/framework/resources.rs:624 | ImageAlmanacIndexbuttonhighlight |  |  |
| src/framework/resources.rs:625 | ImageStoreBackground |  |  |
| src/framework/resources.rs:626 | ImageStoreBackgroundnight |  |  |
| src/framework/resources.rs:627 | ImageStoreCar |  |  |
| src/framework/resources.rs:628 | ImageStoreCarNight |  |  |
| src/framework/resources.rs:629 | ImageStoreCarclosed |  |  |
| src/framework/resources.rs:630 | ImageStoreCarclosedNight |  |  |
| src/framework/resources.rs:631 | ImageStoreHatchbackopen |  |  |
| src/framework/resources.rs:632 | ImageStoreSign |  |  |
| src/framework/resources.rs:633 | ImageStoreMainmenubutton |  |  |
| src/framework/resources.rs:634 | ImageStoreMainmenubuttondown |  |  |
| src/framework/resources.rs:635 | ImageStoreMainmenubuttonhighlight |  |  |
| src/framework/resources.rs:636 | ImageStoreNextbutton |  |  |
| src/framework/resources.rs:637 | ImageStoreNextbuttonhighlight |  |  |
| src/framework/resources.rs:638 | ImageStoreNextbuttondisabled |  |  |
| src/framework/resources.rs:639 | ImageStorePrevbutton |  |  |
| src/framework/resources.rs:640 | ImageStorePrevbuttonhighlight |  |  |
| src/framework/resources.rs:641 | ImageStorePrevbuttondisabled |  |  |
| src/framework/resources.rs:642 | ImageStorePricetag |  |  |
| src/framework/resources.rs:643 | ImageStorePacketupgrade |  |  |
| src/framework/resources.rs:644 | ImageStoreMushroomgardenicon |  |  |
| src/framework/resources.rs:645 | ImageStoreAquariumgardenicon |  |  |
| src/framework/resources.rs:646 | ImageStoreTreeofwisdomicon |  |  |
| src/framework/resources.rs:647 | ImageStoreFirstaidwallnuticon |  |  |
| src/framework/resources.rs:648 | ImageStorePvzicon |  |  |
| src/framework/resources.rs:649 | ImageZombieNote |  |  |
| src/framework/resources.rs:650 | ImageZombieNote1 |  |  |
| src/framework/resources.rs:651 | ImageZombieNote2 |  |  |
| src/framework/resources.rs:652 | ImageZombieNote3 |  |  |
| src/framework/resources.rs:653 | ImageZombieNote4 |  |  |
| src/framework/resources.rs:654 | ImageZombieFinalNote |  |  |
| src/framework/resources.rs:655 | ImageZombieNoteHelp |  |  |
| src/framework/resources.rs:656 | FontBrannetod32 |  |  |
| src/framework/resources.rs:657 | FontBrannetod32Black |  |  |
| src/framework/resources.rs:658 | ImageReanimSunflowerHeadSing1 |  |  |
| src/framework/resources.rs:659 | ImageReanimSunflowerHeadSing2 |  |  |
| src/framework/resources.rs:660 | ImageReanimSunflowerHeadSing3 |  |  |
| src/framework/resources.rs:661 | ImageReanimSunflowerHeadSing4 |  |  |
| src/framework/resources.rs:662 | ImageReanimSunflowerHeadSing5 |  |  |
| src/framework/resources.rs:663 | ImageReanimSunflowerHeadWink |  |  |
| src/framework/resources.rs:664 | ImageReanimCreditsDiscolights |  |  |
| src/framework/resources.rs:665 | ImageReanimCreditsFogmachine |  |  |
| src/framework/resources.rs:666 | ImageCreditsZombienote |  |  |
| src/framework/resources.rs:667 | ImageCreditsPlaybutton |  |  |
| src/framework/resources.rs:668 | ImageZombatarAccessoryBg |  |  |
| src/framework/resources.rs:669 | ImageZombatarPrevButton |  |  |
| src/framework/resources.rs:670 | ImageZombatarPrevButtonHighlight |  |  |
| src/framework/resources.rs:671 | ImageZombatarNextButton |  |  |
| src/framework/resources.rs:672 | ImageZombatarNextButtonHighlight |  |  |
| src/framework/resources.rs:673 | ImageZombatarZombieBlank |  |  |
| src/framework/resources.rs:674 | ImageZombatarZombieBlankSkin |  |  |
| src/framework/resources.rs:675 | ImageZombatarBackgroundCrazydave |  |  |
| src/framework/resources.rs:676 | ImageZombatarBackgroundMenu |  |  |
| src/framework/resources.rs:677 | ImageZombatarBackgroundMenuDos |  |  |
| src/framework/resources.rs:678 | ImageZombatarBackgroundRoof |  |  |
| src/framework/resources.rs:679 | ImageZombatarWidgetBg |  |  |
| src/framework/resources.rs:680 | ImageZombatarWidgetInnerBg |  |  |
| src/framework/resources.rs:681 | ImageZombatarColorsBg |  |  |
| src/framework/resources.rs:682 | ImageZombatarLogo |  |  |
| src/framework/resources.rs:683 | ImageZombatarAccessoryButton |  |  |
| src/framework/resources.rs:684 | ImageZombatarAccessoryButtonHighlight |  |  |
| src/framework/resources.rs:685 | ImageZombatarAccessoryButtonOver |  |  |
| src/framework/resources.rs:686 | ImageZombatarBackdropsButton |  |  |
| src/framework/resources.rs:687 | ImageZombatarBackdropsButtonHighlight |  |  |
| src/framework/resources.rs:688 | ImageZombatarBackdropsButtonOver |  |  |
| src/framework/resources.rs:689 | ImageZombatarClothesButton |  |  |
| src/framework/resources.rs:690 | ImageZombatarClothesButtonHighlight |  |  |
| src/framework/resources.rs:691 | ImageZombatarClothesButtonOver |  |  |
| src/framework/resources.rs:692 | ImageZombatarEyewearButton |  |  |
| src/framework/resources.rs:693 | ImageZombatarEyewearButtonHighlight |  |  |
| src/framework/resources.rs:694 | ImageZombatarEyewearButtonOver |  |  |
| src/framework/resources.rs:695 | ImageZombatarFacialHairButton |  |  |
| src/framework/resources.rs:696 | ImageZombatarFacialHairButtonHighlight |  |  |
| src/framework/resources.rs:697 | ImageZombatarFacialHairButtonOver |  |  |
| src/framework/resources.rs:698 | ImageZombatarHairButton |  |  |
| src/framework/resources.rs:699 | ImageZombatarHairButtonHighlight |  |  |
| src/framework/resources.rs:700 | ImageZombatarHairButtonOver |  |  |
| src/framework/resources.rs:701 | ImageZombatarHatsButton |  |  |
| src/framework/resources.rs:702 | ImageZombatarHatsButtonHighlight |  |  |
| src/framework/resources.rs:703 | ImageZombatarHatsButtonOver |  |  |
| src/framework/resources.rs:704 | ImageZombatarSkinButton |  |  |
| src/framework/resources.rs:705 | ImageZombatarSkinButtonHighlight |  |  |
| src/framework/resources.rs:706 | ImageZombatarTidbitsButton |  |  |
| src/framework/resources.rs:707 | ImageZombatarTidbitsButtonHighlight |  |  |
| src/framework/resources.rs:708 | ImageZombatarTidbitsButtonOver |  |  |
| src/framework/resources.rs:709 | ImageZombatarAccessoryBgHighlight |  |  |
| src/framework/resources.rs:710 | ImageZombatarTriptych |  |  |
| src/framework/resources.rs:711 | ImageZombatarColorpicker |  |  |
| src/framework/resources.rs:712 | ImageZombatarColorpickerHighlight |  |  |
| src/framework/resources.rs:713 | ImageZombatarColorpickerNone |  |  |
| src/framework/resources.rs:714 | ImageZombatarAccessoryBgNone |  |  |
| src/framework/resources.rs:715 | ImageZombatarColorsBgSmall |  |  |
| src/framework/resources.rs:716 | ImageZombatarClothes1 |  |  |
| src/framework/resources.rs:717 | ImageZombatarClothes2 |  |  |
| src/framework/resources.rs:718 | ImageZombatarClothes3 |  |  |
| src/framework/resources.rs:719 | ImageZombatarClothes4 |  |  |
| src/framework/resources.rs:720 | ImageZombatarClothes5 |  |  |
| src/framework/resources.rs:721 | ImageZombatarClothes6 |  |  |
| src/framework/resources.rs:722 | ImageZombatarClothes7 |  |  |
| src/framework/resources.rs:723 | ImageZombatarClothes8 |  |  |
| src/framework/resources.rs:724 | ImageZombatarClothes9 |  |  |
| src/framework/resources.rs:725 | ImageZombatarClothes10 |  |  |
| src/framework/resources.rs:726 | ImageZombatarClothes11 |  |  |
| src/framework/resources.rs:727 | ImageZombatarClothes12 |  |  |
| src/framework/resources.rs:728 | ImageZombatarHats1 |  |  |
| src/framework/resources.rs:729 | ImageZombatarHats2 |  |  |
| src/framework/resources.rs:730 | ImageZombatarHats3 |  |  |
| src/framework/resources.rs:731 | ImageZombatarHats4 |  |  |
| src/framework/resources.rs:732 | ImageZombatarHats5 |  |  |
| src/framework/resources.rs:733 | ImageZombatarHats6 |  |  |
| src/framework/resources.rs:734 | ImageZombatarHats7 |  |  |
| src/framework/resources.rs:735 | ImageZombatarHats8 |  |  |
| src/framework/resources.rs:736 | ImageZombatarHats9 |  |  |
| src/framework/resources.rs:737 | ImageZombatarHats10 |  |  |
| src/framework/resources.rs:738 | ImageZombatarHats11 |  |  |
| src/framework/resources.rs:739 | ImageZombatarHats12 |  |  |
| src/framework/resources.rs:740 | ImageZombatarHats13 |  |  |
| src/framework/resources.rs:741 | ImageZombatarHats14 |  |  |
| src/framework/resources.rs:742 | ImageZombatarHair1 |  |  |
| src/framework/resources.rs:743 | ImageZombatarHair2 |  |  |
| src/framework/resources.rs:744 | ImageZombatarHair3 |  |  |
| src/framework/resources.rs:745 | ImageZombatarHair4 |  |  |
| src/framework/resources.rs:746 | ImageZombatarHair5 |  |  |
| src/framework/resources.rs:747 | ImageZombatarHair6 |  |  |
| src/framework/resources.rs:748 | ImageZombatarHair7 |  |  |
| src/framework/resources.rs:749 | ImageZombatarHair8 |  |  |
| src/framework/resources.rs:750 | ImageZombatarHair9 |  |  |
| src/framework/resources.rs:751 | ImageZombatarHair10 |  |  |
| src/framework/resources.rs:752 | ImageZombatarHair11 |  |  |
| src/framework/resources.rs:753 | ImageZombatarHair12 |  |  |
| src/framework/resources.rs:754 | ImageZombatarHair13 |  |  |
| src/framework/resources.rs:755 | ImageZombatarHair14 |  |  |
| src/framework/resources.rs:756 | ImageZombatarHair15 |  |  |
| src/framework/resources.rs:757 | ImageZombatarHair16 |  |  |
| src/framework/resources.rs:758 | ImageZombatarEyewear1 |  |  |
| src/framework/resources.rs:759 | ImageZombatarEyewear2 |  |  |
| src/framework/resources.rs:760 | ImageZombatarEyewear3 |  |  |
| src/framework/resources.rs:761 | ImageZombatarEyewear4 |  |  |
| src/framework/resources.rs:762 | ImageZombatarEyewear5 |  |  |
| src/framework/resources.rs:763 | ImageZombatarEyewear6 |  |  |
| src/framework/resources.rs:764 | ImageZombatarEyewear7 |  |  |
| src/framework/resources.rs:765 | ImageZombatarEyewear8 |  |  |
| src/framework/resources.rs:766 | ImageZombatarEyewear9 |  |  |
| src/framework/resources.rs:767 | ImageZombatarEyewear10 |  |  |
| src/framework/resources.rs:768 | ImageZombatarEyewear11 |  |  |
| src/framework/resources.rs:769 | ImageZombatarEyewear12 |  |  |
| src/framework/resources.rs:770 | ImageZombatarEyewear13 |  |  |
| src/framework/resources.rs:771 | ImageZombatarEyewear14 |  |  |
| src/framework/resources.rs:772 | ImageZombatarEyewear15 |  |  |
| src/framework/resources.rs:773 | ImageZombatarEyewear16 |  |  |
| src/framework/resources.rs:774 | ImageZombatarFacialhair1 |  |  |
| src/framework/resources.rs:775 | ImageZombatarFacialhair2 |  |  |
| src/framework/resources.rs:776 | ImageZombatarFacialhair3 |  |  |
| src/framework/resources.rs:777 | ImageZombatarFacialhair4 |  |  |
| src/framework/resources.rs:778 | ImageZombatarFacialhair5 |  |  |
| src/framework/resources.rs:779 | ImageZombatarFacialhair6 |  |  |
| src/framework/resources.rs:780 | ImageZombatarFacialhair7 |  |  |
| src/framework/resources.rs:781 | ImageZombatarFacialhair8 |  |  |
| src/framework/resources.rs:782 | ImageZombatarFacialhair9 |  |  |
| src/framework/resources.rs:783 | ImageZombatarFacialhair10 |  |  |
| src/framework/resources.rs:784 | ImageZombatarFacialhair11 |  |  |
| src/framework/resources.rs:785 | ImageZombatarFacialhair12 |  |  |
| src/framework/resources.rs:786 | ImageZombatarFacialhair13 |  |  |
| src/framework/resources.rs:787 | ImageZombatarFacialhair14 |  |  |
| src/framework/resources.rs:788 | ImageZombatarFacialhair15 |  |  |
| src/framework/resources.rs:789 | ImageZombatarFacialhair16 |  |  |
| src/framework/resources.rs:790 | ImageZombatarFacialhair17 |  |  |
| src/framework/resources.rs:791 | ImageZombatarFacialhair18 |  |  |
| src/framework/resources.rs:792 | ImageZombatarFacialhair19 |  |  |
| src/framework/resources.rs:793 | ImageZombatarFacialhair20 |  |  |
| src/framework/resources.rs:794 | ImageZombatarFacialhair21 |  |  |
| src/framework/resources.rs:795 | ImageZombatarFacialhair22 |  |  |
| src/framework/resources.rs:796 | ImageZombatarFacialhair23 |  |  |
| src/framework/resources.rs:797 | ImageZombatarFacialhair24 |  |  |
| src/framework/resources.rs:798 | ImageZombatarTidbits1 |  |  |
| src/framework/resources.rs:799 | ImageZombatarTidbits2 |  |  |
| src/framework/resources.rs:800 | ImageZombatarTidbits3 |  |  |
| src/framework/resources.rs:801 | ImageZombatarTidbits4 |  |  |
| src/framework/resources.rs:802 | ImageZombatarTidbits5 |  |  |
| src/framework/resources.rs:803 | ImageZombatarTidbits6 |  |  |
| src/framework/resources.rs:804 | ImageZombatarTidbits7 |  |  |
| src/framework/resources.rs:805 | ImageZombatarTidbits8 |  |  |
| src/framework/resources.rs:806 | ImageZombatarTidbits9 |  |  |
| src/framework/resources.rs:807 | ImageZombatarTidbits10 |  |  |
| src/framework/resources.rs:808 | ImageZombatarTidbits11 |  |  |
| src/framework/resources.rs:809 | ImageZombatarTidbits12 |  |  |
| src/framework/resources.rs:810 | ImageZombatarTidbits13 |  |  |
| src/framework/resources.rs:811 | ImageZombatarTidbits14 |  |  |
| src/framework/resources.rs:812 | ImageZombatarAccessory1 |  |  |
| src/framework/resources.rs:813 | ImageZombatarAccessory2 |  |  |
| src/framework/resources.rs:814 | ImageZombatarAccessory3 |  |  |
| src/framework/resources.rs:815 | ImageZombatarAccessory4 |  |  |
| src/framework/resources.rs:816 | ImageZombatarAccessory5 |  |  |
| src/framework/resources.rs:817 | ImageZombatarAccessory6 |  |  |
| src/framework/resources.rs:818 | ImageZombatarAccessory7 |  |  |
| src/framework/resources.rs:819 | ImageZombatarAccessory8 |  |  |
| src/framework/resources.rs:820 | ImageZombatarAccessory9 |  |  |
| src/framework/resources.rs:821 | ImageZombatarAccessory10 |  |  |
| src/framework/resources.rs:822 | ImageZombatarAccessory11 |  |  |
| src/framework/resources.rs:823 | ImageZombatarAccessory12 |  |  |
| src/framework/resources.rs:824 | ImageZombatarAccessory13 |  |  |
| src/framework/resources.rs:825 | ImageZombatarAccessory14 |  |  |
| src/framework/resources.rs:826 | ImageZombatarHats1Mask |  |  |
| src/framework/resources.rs:827 | ImageZombatarHair1Mask |  |  |
| src/framework/resources.rs:828 | ImageZombatarHair2Mask |  |  |
| src/framework/resources.rs:829 | ImageZombatarHair11Mask |  |  |
| src/framework/resources.rs:830 | ImageZombatarHair12Mask |  |  |
| src/framework/resources.rs:831 | ImageZombatarHair13Mask |  |  |
| src/framework/resources.rs:832 | ImageZombatarHair14Mask |  |  |
| src/framework/resources.rs:833 | ImageZombatarHair15Mask |  |  |
| src/framework/resources.rs:834 | ImageZombatarFacialhair1Mask |  |  |
| src/framework/resources.rs:835 | ImageZombatarFacialhair4Mask |  |  |
| src/framework/resources.rs:836 | ImageZombatarFacialhair8Mask |  |  |
| src/framework/resources.rs:837 | ImageZombatarFacialhair9Mask |  |  |
| src/framework/resources.rs:838 | ImageZombatarFacialhair10Mask |  |  |
| src/framework/resources.rs:839 | ImageZombatarFacialhair11Mask |  |  |
| src/framework/resources.rs:840 | ImageZombatarFacialhair12Mask |  |  |
| src/framework/resources.rs:841 | ImageZombatarFacialhair14Mask |  |  |
| src/framework/resources.rs:842 | ImageZombatarFacialhair15Mask |  |  |
| src/framework/resources.rs:843 | ImageZombatarFacialhair16Mask |  |  |
| src/framework/resources.rs:844 | ImageZombatarFacialhair18Mask |  |  |
| src/framework/resources.rs:845 | ImageZombatarFacialhair21Mask |  |  |
| src/framework/resources.rs:846 | ImageZombatarFacialhair22Mask |  |  |
| src/framework/resources.rs:847 | ImageZombatarFacialhair23Mask |  |  |
| src/framework/resources.rs:848 | ImageZombatarFacialhair24Mask |  |  |
| src/framework/resources.rs:849 | ImageZombatarEyewear1Mask |  |  |
| src/framework/resources.rs:850 | ImageZombatarEyewear2Mask |  |  |
| src/framework/resources.rs:851 | ImageZombatarEyewear3Mask |  |  |
| src/framework/resources.rs:852 | ImageZombatarEyewear4Mask |  |  |
| src/framework/resources.rs:853 | ImageZombatarEyewear5Mask |  |  |
| src/framework/resources.rs:854 | ImageZombatarEyewear6Mask |  |  |
| src/framework/resources.rs:855 | ImageZombatarEyewear7Mask |  |  |
| src/framework/resources.rs:856 | ImageZombatarEyewear8Mask |  |  |
| src/framework/resources.rs:857 | ImageZombatarEyewear9Mask |  |  |
| src/framework/resources.rs:858 | ImageZombatarEyewear10Mask |  |  |
| src/framework/resources.rs:859 | ImageZombatarEyewear11Mask |  |  |
| src/framework/resources.rs:860 | ImageZombatarEyewear12Mask |  |  |
| src/framework/resources.rs:861 | ImageZombatarHats3Mask |  |  |
| src/framework/resources.rs:862 | ImageZombatarHats6Mask |  |  |
| src/framework/resources.rs:863 | ImageZombatarHats7Mask |  |  |
| src/framework/resources.rs:864 | ImageZombatarHats8Mask |  |  |
| src/framework/resources.rs:865 | ImageZombatarHats9Mask |  |  |
| src/framework/resources.rs:866 | ImageZombatarHats11Mask |  |  |
| src/framework/resources.rs:867 | ImageZombatarBackgroundBlank |  |  |
| src/framework/resources.rs:868 | ImageZombatarAccessory15 |  |  |
| src/framework/resources.rs:869 | ImageZombatarAccessory16 |  |  |
| src/framework/resources.rs:870 | ImageZombatarAcceptButton |  |  |
| src/framework/resources.rs:871 | ImageZombatarAcceptButtonHighlight |  |  |
| src/framework/resources.rs:872 | ImageZombatarFinishedButton |  |  |
| src/framework/resources.rs:873 | ImageZombatarFinishedButtonHighlight |  |  |
| src/framework/resources.rs:874 | ImageZombatarMainBg |  |  |
| src/framework/resources.rs:875 | ImageZombatarBackButton |  |  |
| src/framework/resources.rs:876 | ImageZombatarBackButtonHighlight |  |  |
| src/framework/resources.rs:877 | ImageZombatarNewzombieButton |  |  |
| src/framework/resources.rs:878 | ImageZombatarNewzombieButtonHighlight |  |  |
| src/framework/resources.rs:879 | ImageZombatarTosArrow |  |  |
| src/framework/resources.rs:880 | ImageZombatarBeginButton |  |  |
| src/framework/resources.rs:881 | ImageZombatarBeginButtonHighlight |  |  |
| src/framework/resources.rs:882 | ImageZombatarTosSlider |  |  |
| src/framework/resources.rs:883 | ImageZombatarTosSliderThumb |  |  |
| src/framework/resources.rs:884 | ImageZombatarDisplayWindow |  |  |
| src/framework/resources.rs:885 | ImageZombatarMainmenubackHighlight |  |  |
| src/framework/resources.rs:886 | ImageZombatarViewButton |  |  |
| src/framework/resources.rs:887 | ImageZombatarViewButtonHighlight |  |  |
| src/framework/resources.rs:889 | ResourceIdMax |  |  |
| src/lawn/lawn_app.rs:34 | ZenGarden |  |  |
| src/lawn/lawn_app.rs:35 | Store |  |  |
| src/lawn/lawn_app.rs:36 | Credits |  |  |
| src/lawn/lawn_app.rs:39 | SeedChooser |  |  |
| src/lawn/lawn_app.rs:40 | GameSelector |  |  |
| src/lawn/projectile.rs:38 | Bee |  |  |
| src/lawn/projectile.rs:39 | BeeBackwards |  |  |
| src/lawn/projectile.rs:70 | Stinky |  |  |
| src/lawn/projectile.rs:75 | Guod |  |  |
| src/lawn/projectile.rs:76 | Cobcannon |  |  |
| src/lawn/widget/achievements_screen.rs:15 | NovelPeasPrize |  |  |
| src/lawn/widget/achievements_screen.rs:18 | Spudow |  |  |
| src/lawn/widget/achievements_screen.rs:19 | Explodonator |  |  |
| src/lawn/widget/achievements_screen.rs:22 | RollSomeHeads |  |  |
| src/lawn/widget/achievements_screen.rs:25 | PennyPincher |  |  |
| src/lawn/widget/achievements_screen.rs:27 | PopcornParty |  |  |
| src/lawn/widget/achievements_screen.rs:32 | ToweringWisdom |  |  |

### 3.8 未使用变量（unused_variables）

共 96 项。

| 位置 | 名称 | pub | C++ 出处 |
|---|---|---|---|
| src/framework/desc_parser.rs:164 | child_list |  |  |
| src/framework/graphics/gl_image.rs:405 | clip_rect |  |  |
| src/framework/graphics/graphics.rs:1556 | mc |  |  |
| src/framework/graphics/image_font.rs:238 | list |  |  |
| src/framework/graphics/image_font.rs:868 | layer |  |  |
| src/framework/graphics/image_font.rs:884 | layer |  |  |
| src/framework/graphics/image_font.rs:1082 | image_ref |  |  |
| src/framework/graphics/image_font.rs:1083 | offset_val |  |  |
| src/framework/graphics/image_font.rs:1084 | spacing_val |  |  |
| src/framework/graphics/image_font.rs:1296 | bytes |  |  |
| src/framework/graphics/image_font.rs:1446 | colorize_images |  |  |
| src/framework/graphics/memory_image.rs:206 | linear_filter |  |  |
| src/framework/graphics/memory_image.rs:379 | app |  |  |
| src/framework/graphics/memory_image.rs:396 | app |  |  |
| src/framework/paklib/mod.rs:470 | record |  |  |
| src/framework/paklib/mod.rs:684 | c |  |  |
| src/framework/resource_manager.rs:492 | section_name |  |  |
| src/framework/sound/music_interface.rs:23 | song_id |  |  |
| src/framework/sound/music_interface.rs:27 | song_id |  |  |
| src/framework/widget/button_widget.rs:352 | the_btn | pub |  |
| src/framework/widget/button_widget.rs:361 | the_btn | pub |  |
| src/framework/widget/button_widget.rs:361 | the_click | pub |  |
| src/framework/widget/edit_widget.rs:630 | display_str |  |  |
| src/framework/widget/slider.rs:176 | x | pub |  |
| src/framework/widget/slider.rs:176 | y | pub |  |
| src/framework/widget/widget.rs:52 | g |  |  |
| src/framework/widget/widget.rs:52 | widget |  |  |
| src/framework/widget/widget.rs:53 | g |  |  |
| src/framework/widget/widget.rs:53 | widget |  |  |
| src/framework/widget/widget.rs:56 | widget |  |  |
| src/framework/widget/widget.rs:59 | key |  |  |
| src/framework/widget/widget.rs:59 | widget |  |  |
| src/framework/widget/widget.rs:59 | wm |  |  |
| src/framework/widget/widget.rs:60 | key |  |  |
| src/framework/widget/widget.rs:60 | widget |  |  |
| src/framework/widget/widget.rs:61 | c |  |  |
| src/framework/widget/widget.rs:61 | widget |  |  |
| src/framework/widget/widget.rs:64 | btn |  |  |
| src/framework/widget/widget.rs:64 | click |  |  |
| src/framework/widget/widget.rs:64 | widget |  |  |
| src/framework/widget/widget.rs:64 | x |  |  |
| src/framework/widget/widget.rs:64 | y |  |  |
| src/framework/widget/widget.rs:65 | btn |  |  |
| src/framework/widget/widget.rs:65 | click |  |  |
| src/framework/widget/widget.rs:65 | widget |  |  |
| src/framework/widget/widget.rs:65 | x |  |  |
| src/framework/widget/widget.rs:65 | y |  |  |
| src/framework/widget/widget.rs:66 | widget |  |  |
| src/framework/widget/widget.rs:66 | x |  |  |
| src/framework/widget/widget.rs:66 | y |  |  |
| src/framework/widget/widget.rs:67 | widget |  |  |
| src/framework/widget/widget.rs:67 | x |  |  |
| src/framework/widget/widget.rs:67 | y |  |  |
| src/framework/widget/widget.rs:68 | delta |  |  |
| src/framework/widget/widget.rs:68 | widget |  |  |
| src/framework/widget/widget.rs:69 | widget |  |  |
| src/framework/widget/widget.rs:70 | widget |  |  |
| src/framework/widget/widget.rs:73 | widget |  |  |
| src/framework/widget/widget.rs:74 | widget |  |  |
| src/lawn/board.rs:1184 | grid_x |  |  |
| src/lawn/challenge.rs:3564 | b |  |  |
| src/lawn/challenge.rs:7274 | a_scale |  | C++: 高度标注文字（FONT_HOUSEOFTERROR16 + PvzpDrawStringMatrix） |
| src/lawn/coin.rs:1392 | mode |  |  |
| src/lawn/coin.rs:1638 | a_angle |  |  |
| src/lawn/cursor_object.rs:307 | seed_type |  |  |
| src/lawn/lawn_app.rs:2800 | a_pos_x |  |  |
| src/lawn/lawn_app.rs:2801 | a_pos_y |  |  |
| src/lawn/lawn_app.rs:2825 | a_message_name |  |  |
| src/lawn/plant.rs:1673 | a_damage_range_flags |  |  |
| src/lawn/plant.rs:3309 | a_scale_y |  |  |
| src/lawn/plant.rs:5054 | a_ladder_grid_x |  |  |
| src/lawn/plant.rs:5054 | a_ladder_grid_y |  |  |
| src/lawn/plant.rs:5407 | board |  | C++: mY = PvzpAnimateCurve(10, 0, mStateCountdown, aDestY - 120, aDestY, CURVE_EASE_IN_OUT); |
| src/lawn/plant.rs:5685 | a_pos_x |  |  |
| src/lawn/plant.rs:5686 | a_pos_y |  |  |
| src/lawn/projectile.rs:252 | a_old_row |  |  |
| src/lawn/projectile.rs:253 | a_old_y |  |  |
| src/lawn/projectile.rs:1083 | proj_type |  |  |
| src/lawn/projectile.rs:1211 | my_width |  |  |
| src/lawn/seed_packet.rs:664 | packet |  |  |
| src/lawn/seed_packet.rs:705 | use_current_cost |  |  |
| src/lawn/seed_packet.rs:802 | a_scale |  |  |
| src/lawn/seed_packet.rs:803 | a_draw_seed_in_middle |  |  |
| src/lawn/widget/challenge_screen.rs:270 | a_def |  |  |
| src/lawn/widget/credit_screen.rs:1969 | a_frame_factor |  |  |
| src/lawn/widget/new_options_dialog.rs:574 | app |  |  |
| src/lawn/widget/seed_chooser_screen.rs:822 | app |  |  |
| src/lawn/widget/seed_chooser_screen.rs:1701 | x | pub |  |
| src/lawn/widget/seed_chooser_screen.rs:1701 | y | pub |  |
| src/lawn/widget/seed_chooser_screen.rs:1703 | btn |  |  |
| src/lawn/widget/seed_chooser_screen.rs:1711 | btn |  |  |
| src/lawn/zen_garden.rs:1486 | g | pub |  |
| src/todlib/data_array.rs:57 | i |  |  |
| src/todlib/reanimator.rs:1621 | app |  |  |
| src/todlib/tod_particle.rs:3595 | i |  |  |
| src/todlib/trail.rs:230 | triangle_count |  |  |

### 3.9 未读赋值（unused_assignments）

共 63 项。

| 位置 | 名称 | pub | C++ 出处 |
|---|---|---|---|
| src/framework/graphics/image.rs:555 | a_dest_x |  |  |
| src/framework/graphics/image.rs:596 | a_dest_y |  |  |
| src/framework/sexy_app_base.rs:2832 | pos |  |  |
| src/lawn/board.rs:5706 | bg |  |  |
| src/lawn/challenge.rs:7280 | a_scale |  | C++: PvzpAnimateCurveFloat(55, 20, mChallengeStateCounter, 1.0f, 1.2f, CURVE_BOUNCE) |
| src/lawn/lawn_app.rs:2803 | a_pos_x |  |  |
| src/lawn/lawn_app.rs:2804 | a_pos_y |  |  |
| src/lawn/lawn_app.rs:2806 | a_pos_x |  |  |
| src/lawn/lawn_app.rs:2807 | a_pos_y |  |  |
| src/lawn/lawn_app.rs:2812 | a_bubble_text |  |  |
| src/lawn/plant.rs:2205 | a_offset_x_start |  |  |
| src/lawn/plant.rs:2206 | a_offset_x_end |  |  |
| src/lawn/plant.rs:2207 | a_offset_y_start |  |  |
| src/lawn/plant.rs:2208 | a_offset_y_end |  |  |
| src/lawn/plant.rs:2209 | a_scale_start |  |  |
| src/lawn/plant.rs:2210 | a_scale_end |  |  |
| src/lawn/plant.rs:3321 | a_scale_y |  |  |
| src/lawn/plant.rs:3328 | a_scale_x |  |  |
| src/lawn/plant.rs:3335 | a_scale_x |  |  |
| src/lawn/plant.rs:3336 | a_scale_y |  |  |
| src/lawn/seed_packet.rs:808 | a_scale |  |  |
| src/lawn/seed_packet.rs:813 | a_scale |  |  |
| src/lawn/seed_packet.rs:818 | a_scale |  |  |
| src/lawn/seed_packet.rs:827 | a_scale |  |  |
| src/lawn/seed_packet.rs:845 | a_scale |  |  |
| src/lawn/seed_packet.rs:850 | a_scale |  |  |
| src/lawn/seed_packet.rs:855 | a_scale |  |  |
| src/lawn/seed_packet.rs:860 | a_scale |  |  |
| src/lawn/seed_packet.rs:865 | a_scale |  |  |
| src/lawn/seed_packet.rs:870 | a_scale |  |  |
| src/lawn/seed_packet.rs:875 | a_scale |  |  |
| src/lawn/seed_packet.rs:880 | a_scale |  |  |
| src/lawn/seed_packet.rs:885 | a_scale |  |  |
| src/lawn/seed_packet.rs:890 | a_scale |  |  |
| src/lawn/seed_packet.rs:895 | a_scale |  |  |
| src/lawn/seed_packet.rs:900 | a_scale |  |  |
| src/lawn/seed_packet.rs:905 | a_scale |  |  |
| src/lawn/seed_packet.rs:910 | a_scale |  |  |
| src/lawn/seed_packet.rs:915 | a_scale |  |  |
| src/lawn/seed_packet.rs:920 | a_scale |  |  |
| src/lawn/seed_packet.rs:928 | a_scale |  |  |
| src/lawn/seed_packet.rs:933 | a_scale |  |  |
| src/lawn/seed_packet.rs:938 | a_scale |  |  |
| src/lawn/seed_packet.rs:943 | a_scale |  |  |
| src/lawn/seed_packet.rs:948 | a_scale |  |  |
| src/lawn/seed_packet.rs:953 | a_scale |  |  |
| src/lawn/seed_packet.rs:958 | a_scale |  |  |
| src/lawn/seed_packet.rs:968 | a_draw_seed_in_middle |  |  |
| src/lawn/seed_packet.rs:987 | a_scale |  |  |
| src/lawn/seed_packet.rs:991 | a_offset_x |  |  |
| src/lawn/seed_packet.rs:992 | a_offset_y |  |  |
| src/lawn/system/save_game.rs:1877 | field_data |  |  |
| src/lawn/widget/cheat_dialog.rs:207 | need_space |  |  |
| src/lawn/widget/continue_dialog.rs:252 | need_space |  |  |
| src/lawn/widget/message_widget.rs:132 | a_outline_color |  |  |
| src/lawn/widget/message_widget.rs:253 | a_outline_color |  |  |
| src/lawn/widget/message_widget.rs:258 | a_outline_color |  |  |
| src/lawn/widget/new_user_dialog.rs:236 | need_space |  |  |
| src/lawn/widget/title_screen.rs:259 | an_alpha |  |  |
| src/lawn/widget/user_dialog.rs:418 | need_space |  |  |
| src/lawn/zombie.rs:4310 | a_speed |  | C++: aSpeed = GetTrackVelocity("_ground"); m02 -= aSpeed |
| src/todlib/tod_particle.rs:2415 | a_pos_x |  |  |
| src/todlib/tod_particle.rs:2416 | a_pos_y |  |  |

### 3.10 多余 mut（unused_mut）

共 29 项。

| 位置 | 名称 | pub | C++ 出处 |
|---|---|---|---|
| src/lawn/board.rs:3846 | a_z_pos |  |  |
| src/lawn/board.rs:4296 | a_pos_x |  |  |
| src/lawn/board.rs:4297 | a_pos_y |  |  |
| src/lawn/board.rs:4404 | a_pos_y |  |  |
| src/lawn/board.rs:6472 | a_to_die |  |  |
| src/lawn/challenge.rs:5830 | picks_count |  |  |
| src/lawn/lawn_app.rs:1005 | screen |  |  |
| src/lawn/lawn_app.rs:1610 | a_dialog |  |  |
| src/lawn/lawn_mower.rs:211 | a_speed |  |  |
| src/lawn/lawn_mower.rs:413 | a_scale_x |  |  |
| src/lawn/plant.rs:1099 | a_origin_x |  |  |
| src/lawn/plant.rs:1989 | a_color_override |  |  |
| src/lawn/plant.rs:2299 | a_pos_x |  |  |
| src/lawn/plant.rs:2475 | a_cel_row |  |  |
| src/lawn/plant.rs:2477 | a_image |  |  |
| src/lawn/projectile.rs:868 | get_projectile_image |  |  |
| src/lawn/projectile.rs:1212 | splash_rect |  |  |
| src/lawn/system/reanimation_lawn.rs:290 | a_memory_image |  |  |
| src/lawn/system/reanimation_lawn.rs:402 | a_memory_image |  |  |
| src/lawn/system/reanimation_lawn.rs:465 | a_memory_image |  |  |
| src/lawn/system/reanimation_lawn.rs:483 | a_pos_x |  | C++: PVZP_ASSERT(aZombieDef.mReanimationType != ReanimationType::REANIM_NONE) |
| src/lawn/widget/cheat_dialog.rs:160 | h |  |  |
| src/lawn/widget/store_screen.rs:144 | screen |  |  |
| src/lawn/zombie.rs:9648 | coin_type |  |  |
| src/todlib/effect_system.rs:134 | a_system |  |  |
| src/todlib/reanimator.rs:447 | a_atlas_memory |  |  |
| src/todlib/reanimator.rs:1644 | a_attach_reanim |  |  |
| src/todlib/reanimator.rs:1655 | app |  |  |
| src/todlib/tod_particle.rs:1475 | a_src_rect |  |  |

### 3.11 未使用导入（unused_imports）

共 66 项。

| 位置 | 名称 | pub | C++ 出处 |
|---|---|---|---|
| src/framework/desc_parser.rs:10 | std::fs |  |  |
| src/framework/graphics/gl_image.rs:17 | PixelFormat |  |  |
| src/framework/graphics/gl_image.rs:17 | TextureDataPiece |  |  |
| src/framework/graphics/image.rs:10 | crate::framework::graphics::graphics::Graphics |  |  |
| src/framework/graphics/memory_image.rs:14 | Span |  |  |
| src/framework/graphics/memory_image.rs:15 | crate::framework::point::Point |  |  |
| src/framework/graphics/quantize.rs:9 | crate::framework::common::uchar |  |  |
| src/framework/paklib/mod.rs:22 | Write |  |  |
| src/framework/resource_manager.rs:8 | LinkedList |  |  |
| src/framework/resource_manager.rs:9 | std::ptr |  |  |
| src/framework/resource_manager.rs:13 | crate::framework::graphics::gl_image::GLImage |  |  |
| src/framework/resource_manager.rs:16 | SharedImage |  |  |
| src/framework/sexy_app_base.rs:11 | sdl2::keyboard::Keycode |  |  |
| src/framework/sexy_app_base.rs:20 | crate::framework::graphics::image::Image |  |  |
| src/framework/widget/dialog.rs:11 | crate::framework::widget::button_listener::ButtonListener |  |  |
| src/framework/widget/scrollbar_widget.rs:8 | crate::framework::rect::Rect |  |  |
| src/framework/widget/text_widget.rs:17 | crate::framework::rect::Rect |  |  |
| src/framework/widget/widget.rs:18 | crate::framework::widget::widget_container::WidgetContainer |  |  |
| src/framework/widget/widget_container.rs:9 | std::cmp |  |  |
| src/lawn/board.rs:8 | RandFloat |  |  |
| src/lawn/board.rs:25 | crate::todlib::reanimator::Reanimation |  |  |
| src/lawn/board.rs:31 | crate::todlib::tod_particle::ParticleSystem |  |  |
| src/lawn/cursor_object.rs:5 | HitResult |  |  |
| src/lawn/cutscene.rs:15 | crate::todlib::tod_common::rand_range_float |  |  |
| src/lawn/lawn_app.rs:12 | WidgetImpl |  |  |
| src/lawn/lawn_app.rs:13 | crate::framework::widget::widget_manager::WidgetManager |  |  |
| src/lawn/lawn_app.rs:20 | TitleState |  |  |
| src/lawn/plant.rs:5 | Rand |  |  |
| src/lawn/plant.rs:12 | crate::lawn::zombie::Zombie |  |  |
| src/lawn/projectile.rs:4 | Rand |  |  |
| src/lawn/projectile.rs:4 | RandFloat |  |  |
| src/lawn/projectile.rs:4 | RandRange |  |  |
| src/lawn/seed_packet.rs:5 | crate::framework::rect::Rect |  |  |
| src/lawn/system/data_sync.rs:8 | Seek |  |  |
| src/lawn/system/data_sync.rs:8 | SeekFrom |  |  |
| src/lawn/system/player_info.rs:4 | crate::lawn::system::data_sync::DataReaderException |  |  |
| src/lawn/system/save_game.rs:25 | MAX_MESSAGE_LENGTH |  |  |
| src/lawn/system/save_game.rs:26 | MAX_ZOMBIE_FOLLOWERS |  |  |
| src/lawn/system/save_game.rs:29 | crate::todlib::effect_system::EffectSystem |  |  |
| src/lawn/tool_tip_widget.rs:7 | crate::framework::graphics::font::Font |  |  |
| src/lawn/widget/almanac_dialog.rs:11 | crate::lawn::widget::lawn_dialog::LawnDialog |  |  |
| src/lawn/widget/challenge_screen.rs:17 | tod_animate_curve |  |  |
| src/lawn/widget/lawn_dialog.rs:9 | crate::framework::widget::dialog::Dialog |  |  |
| src/lawn/widget/message_widget.rs:7 | crate::framework::rect::Rect |  |  |
| src/lawn/widget/store_screen.rs:21 | tod_animate_curve |  |  |
| src/lawn/widget/zombatar_tos.rs:11 | crate::lawn::game_enums::* |  |  |
| src/lawn/widget/zombatar_widget.rs:13 | crate::framework::widget::widget_manager::WidgetManager |  |  |
| src/lawn/widget/zombatar_widget.rs:17 | zombatar_read_record_slot |  |  |
| src/lawn/zombie.rs:5 | Rand |  |  |
| src/lawn/zombie.rs:10 | crate::lawn::board::Board |  |  |
| src/lawn/zombie.rs:13 | crate::lawn::lawn_app::LawnApp |  |  |
| src/lawn/zombie.rs:16 | Attachment |  |  |
| src/todlib/definition.rs:7 | crate::framework::rect::Rect |  |  |
| src/todlib/definition.rs:8 | ReanimLoopType |  |  |
| src/todlib/definition.rs:8 | Reanimation |  |  |
| src/todlib/effect_system.rs:7 | ATTACHMENTID_NULL |  |  |
| src/todlib/effect_system.rs:7 | CoinID |  |  |
| src/todlib/mod.rs:23 | tod_string_file::TodStringFile | pub |  |
| src/todlib/reanimator.rs:7 | crate::framework::graphics::gl_interface::TriVertex |  |  |
| src/todlib/reanimator.rs:10 | crate::framework::rect::Rect |  |  |
| src/todlib/reanimator.rs:11 | TodCurves |  |  |
| src/todlib/tod_foley.rs:9 | crate::framework::sound::sound_manager::SoundManager |  |  |
| src/todlib/tod_particle.rs:10 | std::collections::LinkedList |  |  |
| src/todlib/tod_particle.rs:19 | PARTICLESYSTEMID_NULL |  |  |
| src/todlib/tod_particle.rs:19 | ParticleSystemID |  |  |
| src/todlib/trail.rs:6 | std::f32::consts::PI |  |  |

## 4. 口径与局限

1. **证据来源**：全部来自 rustc 静态诊断，不是人工猜测；清单只覆盖“编译器能证明从未被使用”的符号。
   被 #[allow(dead_code)] 屏蔽、导出到 crate 之外、或经函数指针/字符串间接调用的符号不会出现在这里。
2. **target 口径**：同一份 src/ 被编译为 lib(pvz_portable) 与 bin(pvz-portable) 两次；本报告以 bin 为准并去重，
   lib 目标独有的诊断（lib 内 pub fn 无任何调用者等）同样并入。
3. **trait 默认方法**也计入“方法未使用”：它们可能对应 C++ 的虚函数，未被调用通常意味着派生类未覆写或调用点缺失，
   须按 docs/04_PROGRESS.md 的接线状态逐个核对，不能直接判定为死代码。
4. **枚举变体**大多来自整体未接线的枚举（如 ResourceId 的 868 个变体），统计时按变体逐一列出，实际是整类未接线。
5. **运行验证**：为确认统计基线可信，另跑过一次 cargo run —— SDL 初始化与资源加载完成、窗口进入主循环后由人工结束进程（日志 target/run.log）；未做玩法与 .dmo 回放一致性验证。

## 5. 复现方式

```
python scripts/unused_symbols.py --collect   # 重新编译采集诊断并重建本报告
python scripts/unused_symbols.py             # 复用 target/unused_diag.json 重建本报告
```

机器可读清单见 target/unused_items.json（字段：kind / name / file / line / code / msg）。
