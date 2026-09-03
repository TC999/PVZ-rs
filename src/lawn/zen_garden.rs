// PvZ Portable Rust 翻译 — ZenGarden（禅境花园）
// 对应 C++ src/Lawn/ZenGarden.h / ZenGarden.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::lawn::game_enums::*;
use crate::lawn::board::Board;
use crate::lawn::board::HitResult;
use crate::lawn::plant::Plant;
use crate::lawn::grid_item::GridItem;
use crate::lawn::system::player_info::{FacingDirection, PottedPlant};
use crate::framework::widget::dialog::BUTTONS_YES_NO;
use crate::todlib::tod_foley::FoleyType;
use crate::todlib::reanimator::Reanimation;
use crate::lawn::system::music::MusicTune;

/// 禅境花园网格尺寸（对应 C++ ZEN_MAX_GRIDSIZE_X/Y）
pub const ZEN_MAX_GRIDSIZE_X: i32 = 8;
pub const ZEN_MAX_GRIDSIZE_Y: i32 = 4;

/// 臭鼬睡觉位置 Y（对应 C++ STINKY_SLEEP_POS_Y）
pub const STINKY_SLEEP_POS_Y: f32 = 461.0;

/// 特殊网格布局（对应 C++ SpecialGridPlacement）
pub struct SpecialGridPlacement {
    pub pixel_x: i32,
    pub pixel_y: i32,
    pub grid_x: i32,
    pub grid_y: i32,
}

/// 温室网格表（对应 C++ gGreenhouseGridPlacement）
pub static GREENHOUSE_GRID_PLACEMENT: [SpecialGridPlacement; 32] = [
    SpecialGridPlacement { pixel_x: 73, pixel_y: 73, grid_x: 0, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 155, pixel_y: 71, grid_x: 1, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 239, pixel_y: 68, grid_x: 2, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 321, pixel_y: 73, grid_x: 3, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 406, pixel_y: 71, grid_x: 4, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 484, pixel_y: 67, grid_x: 5, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 566, pixel_y: 70, grid_x: 6, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 648, pixel_y: 72, grid_x: 7, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 67, pixel_y: 168, grid_x: 0, grid_y: 1 },
    SpecialGridPlacement { pixel_x: 150, pixel_y: 165, grid_x: 1, grid_y: 1 },
    SpecialGridPlacement { pixel_x: 232, pixel_y: 170, grid_x: 2, grid_y: 1 },
    SpecialGridPlacement { pixel_x: 314, pixel_y: 175, grid_x: 3, grid_y: 1 },
    SpecialGridPlacement { pixel_x: 416, pixel_y: 173, grid_x: 4, grid_y: 1 },
    SpecialGridPlacement { pixel_x: 497, pixel_y: 170, grid_x: 5, grid_y: 1 },
    SpecialGridPlacement { pixel_x: 578, pixel_y: 164, grid_x: 6, grid_y: 1 },
    SpecialGridPlacement { pixel_x: 660, pixel_y: 168, grid_x: 7, grid_y: 1 },
    SpecialGridPlacement { pixel_x: 41, pixel_y: 268, grid_x: 0, grid_y: 2 },
    SpecialGridPlacement { pixel_x: 130, pixel_y: 266, grid_x: 1, grid_y: 2 },
    SpecialGridPlacement { pixel_x: 219, pixel_y: 260, grid_x: 2, grid_y: 2 },
    SpecialGridPlacement { pixel_x: 310, pixel_y: 266, grid_x: 3, grid_y: 2 },
    SpecialGridPlacement { pixel_x: 416, pixel_y: 267, grid_x: 4, grid_y: 2 },
    SpecialGridPlacement { pixel_x: 504, pixel_y: 261, grid_x: 5, grid_y: 2 },
    SpecialGridPlacement { pixel_x: 594, pixel_y: 265, grid_x: 6, grid_y: 2 },
    SpecialGridPlacement { pixel_x: 684, pixel_y: 269, grid_x: 7, grid_y: 2 },
    SpecialGridPlacement { pixel_x: 37, pixel_y: 371, grid_x: 0, grid_y: 3 },
    SpecialGridPlacement { pixel_x: 124, pixel_y: 369, grid_x: 1, grid_y: 3 },
    SpecialGridPlacement { pixel_x: 211, pixel_y: 368, grid_x: 2, grid_y: 3 },
    SpecialGridPlacement { pixel_x: 302, pixel_y: 369, grid_x: 3, grid_y: 3 },
    SpecialGridPlacement { pixel_x: 425, pixel_y: 375, grid_x: 4, grid_y: 3 },
    SpecialGridPlacement { pixel_x: 512, pixel_y: 368, grid_x: 5, grid_y: 3 },
    SpecialGridPlacement { pixel_x: 602, pixel_y: 365, grid_x: 6, grid_y: 3 },
    SpecialGridPlacement { pixel_x: 691, pixel_y: 368, grid_x: 7, grid_y: 3 },
];

/// 蘑菇园网格表（对应 C++ gMushroomGridPlacement）
pub static MUSHROOM_GRID_PLACEMENT: [SpecialGridPlacement; 8] = [
    SpecialGridPlacement { pixel_x: 110, pixel_y: 441, grid_x: 0, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 237, pixel_y: 360, grid_x: 1, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 298, pixel_y: 458, grid_x: 2, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 355, pixel_y: 296, grid_x: 3, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 387, pixel_y: 203, grid_x: 4, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 460, pixel_y: 385, grid_x: 5, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 486, pixel_y: 478, grid_x: 6, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 552, pixel_y: 283, grid_x: 7, grid_y: 0 },
];

/// 水族馆网格表（对应 C++ gAquariumGridPlacement）
pub static AQUARIUM_GRID_PLACEMENT: [SpecialGridPlacement; 8] = [
    SpecialGridPlacement { pixel_x: 113, pixel_y: 185, grid_x: 0, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 306, pixel_y: 120, grid_x: 1, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 356, pixel_y: 270, grid_x: 2, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 622, pixel_y: 120, grid_x: 3, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 669, pixel_y: 270, grid_x: 4, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 122, pixel_y: 355, grid_x: 5, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 365, pixel_y: 458, grid_x: 6, grid_y: 0 },
    SpecialGridPlacement { pixel_x: 504, pixel_y: 417, grid_x: 7, grid_y: 0 },
];

/// 禅境花园（对应 C++ ZenGarden）
pub struct ZenGarden {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub board: Option<*mut Board>,
    pub garden_type: GardenType,
    pub loaded_resource_names: Vec<String>,
    pub now_time: i64,  // time_t → i64
    pub now_tm: (i32, i32, i32, i32, i32, i32, i32, i32, i32), // tm 结构体简化
}

impl ZenGarden {
    pub fn new() -> Self {
        ZenGarden {
            app: None,
            board: None,
            garden_type: GardenType::Main,
            loaded_resource_names: Vec::new(),
            now_time: 0,
            now_tm: (0, 0, 0, 0, 0, 0, 0, 0, 0),
        }
    }

    // --- 方法存根（待从 ZenGarden.cpp 翻译具体实现） ---

    pub fn zen_garden_init_level(&mut self) {
        // 对应 C++ ZenGardenInitLevel
        let app_ptr = match self.app {
            Some(p) => p,
            None => return,
        };
        self.board = unsafe { (*app_ptr).board };
        self.now_time = unsafe { (*app_ptr).get_now_time() };
        self.now_tm = unsafe { (*app_ptr).get_local_time(self.now_time) };

        let num_potted = unsafe { (*app_ptr).player_info.as_ref().map_or(0, |p| p.m_num_potted_plants) };
        for i in 0..num_potted {
            if let Some(pp) = self.potted_plant_from_index(i as usize) {
                unsafe {
                    if (*pp).which_zen_garden == self.garden_type {
                        self.place_potted_plant(i as usize);
                    }
                }
            }
        }

        if let Some(board_ptr) = self.board {
            unsafe {
                if let Some(challenge) = &mut (*board_ptr).challenge {
                    challenge.challenge_state_counter = 3000;
                }
            }
        }
        self.add_stinky();
        if let Some(music) = unsafe { (*app_ptr).music.as_mut() } {
            music.start_game_music();
        }
    }

    pub fn draw_potted_plant_icon(&self, g: &mut Graphics, x: f32, y: f32, potted_plant: &PottedPlant) {
        // 对应 C++ DrawPottedPlantIcon
        self.draw_potted_plant(g, x, y, potted_plant, 0.7, true);
    }

    pub fn draw_potted_plant(&self, g: &mut Graphics, x: f32, y: f32, potted_plant: &PottedPlant, scale: f32, draw_pot: bool) {
        // 对应 C++ DrawPottedPlant（缩放/变体/偏移计算完整；核心绘制依赖
        // Plant::DrawSeedType，Rust 侧尚未实现，绘制处留注释）
        let mut a_plant_variation = DrawVariation::Normal;
        let mut a_seed_type = potted_plant.seed_type;
        if potted_plant.plant_age == PottedPlantAge::Sprout {
            a_seed_type = SeedType::Sprout;
            if potted_plant.seed_type != SeedType::Marigold {
                a_plant_variation = DrawVariation::SproutNoFlower;
            }
        } else if (a_seed_type == SeedType::Tanglekelp || a_seed_type == SeedType::Seashroom)
            && potted_plant.which_zen_garden == GardenType::Aquarium
        {
            a_plant_variation = DrawVariation::Aquarium;
        } else {
            a_plant_variation = potted_plant.draw_variation;
        }

        let board_ref = self.board.map(|b| unsafe { &*b });
        let mut a_offset_x = 0.0f32;
        let mut a_offset_y = Plant::plant_draw_height_offset(board_ref, None, a_seed_type, -1, -1);
        if draw_pot {
            let a_pot_offset_y = Plant::plant_draw_height_offset(board_ref, None, SeedType::Flowerpot, -1, -1);
            let mut a_pot_variation = DrawVariation::ZenGarden;
            if Plant::is_aquatic(a_seed_type) {
                a_pot_variation = DrawVariation::ZenGardenWater;
            }
            // 对应 C++ Plant::DrawSeedType(&aPottedPlantG, FLOWERPOT, SEED_NONE, aPotVariation2, ...)
            crate::lawn::plant::Plant::draw_seed_type(
                g, SeedType::Flowerpot, SeedType::None, a_pot_variation,
                x, y + a_pot_offset_y * scale,
            );
        }

        if potted_plant.facing == FacingDirection::Left {
            // C++ 中 aPottedPlantG.mScaleX = -scale
            a_offset_x += 80.0 * scale;
        }

        if potted_plant.plant_age == PottedPlantAge::Small {
            a_offset_x += 20.0 * scale;
            a_offset_y += 40.0 * scale;
        } else if potted_plant.plant_age == PottedPlantAge::Medium {
            a_offset_x += 10.0 * scale;
            a_offset_y += 20.0 * scale;
        }

        if draw_pot {
            a_offset_y += Plant::plant_flower_pot_height_offset(a_seed_type, scale);
        }
        a_offset_y += self.plant_potted_draw_height_offset(a_seed_type, scale);

        // 对应 C++ Plant::DrawSeedType(&aPottedPlantG, aSeedType, SEED_NONE, aPlantVariation, ...)
        crate::lawn::plant::Plant::draw_seed_type(
            g, a_seed_type, SeedType::None, a_plant_variation,
            x + a_offset_x, y + a_offset_y,
        );
    }

    pub fn is_zen_garden_full(&self, include_dropped_presents: bool) -> bool {
        // 对应 C++ IsZenGardenFull
        let mut num_dropped_presents = 0;
        if let Some(board) = self.board {
            let b = unsafe { &*board };
            if include_dropped_presents {
                num_dropped_presents += b.count_coin_by_type(CoinType::AwardPresent);
                num_dropped_presents += b.count_coin_by_type(CoinType::PresentPlant);
            }
        }
        let mut num_potted_plants_in_garden = 0;
        if let Some(app) = self.app {
            unsafe {
                let info = &(*app).player_info;
                if let Some(info) = info {
                    for i in 0..info.m_num_potted_plants as usize {
                        if i < info.m_potted_plant.len() {
                            if info.m_potted_plant[i].which_zen_garden == GardenType::Main {
                                num_potted_plants_in_garden += 1;
                            }
                        }
                    }
                }
            }
        }
        num_dropped_presents + num_potted_plants_in_garden >= ZEN_MAX_GRIDSIZE_X * ZEN_MAX_GRIDSIZE_Y
    }

    pub fn find_open_zen_garden_spot(&self, spot_x: &mut i32, spot_y: &mut i32) {
        // 对应 C++ FindOpenZenGardenSpot
        let mut picks: Vec<crate::todlib::tod_common::TodWeightedGridArray> = Vec::new();
        let crazy_dave_talking = self.app.map_or(false, |app| unsafe {
            (*app).m_crazy_dave_message_index != -1
        });
        let num_potted = self.app.map_or(0, |app| unsafe {
            (*app).player_info.as_ref().map_or(0, |p| p.m_num_potted_plants)
        });
        for x in 0..ZEN_MAX_GRIDSIZE_X {
            'col: for y in 0..ZEN_MAX_GRIDSIZE_Y {
                if crazy_dave_talking && (x < 2 || y < 1) {
                    continue;
                }
                if let Some(app) = self.app {
                    unsafe {
                        let info = &(*app).player_info;
                        if let Some(info) = info {
                            for i in 0..num_potted as usize {
                                if i < info.m_potted_plant.len() {
                                    let p = &info.m_potted_plant[i];
                                    if p.which_zen_garden == GardenType::Main && p.x == x && p.y == y {
                                        continue 'col;
                                    }
                                }
                            }
                        }
                    }
                }
                picks.push(crate::todlib::tod_common::TodWeightedGridArray { x, y, weight: 1 });
            }
        }
        if picks.is_empty() {
            *spot_x = 0;
            *spot_y = 0;
            return;
        }
        let pick_count = picks.len();
        let pick = crate::todlib::tod_common::tod_pick_from_weighted_grid_array(&mut picks, pick_count);
        match pick {
            Some(i) => { *spot_x = picks[i].x; *spot_y = picks[i].y; }
            None => { *spot_x = 0; *spot_y = 0; }
        }
    }

    pub fn add_potted_plant(&mut self, potted_plant: &mut PottedPlant) {
        // 对应 C++ AddPottedPlant
        let potted_plant_index;
        if let Some(app) = self.app {
            unsafe {
                let info = &mut (*app).player_info;
                if let Some(info) = info.as_mut() {
                    if info.m_potted_plant.len() >= crate::lawn::system::player_info::MAX_POTTED_PLANTS {
                        return;
                    }
                    potted_plant_index = info.m_num_potted_plants;
                    info.m_potted_plant.push(potted_plant.clone());
                    let new_plant = &mut info.m_potted_plant[potted_plant_index as usize];
                    new_plant.which_zen_garden = GardenType::Main;
                    new_plant.last_watered_time = 0;
                    let mut x = 0;
                    let mut y = 0;
                    self.find_open_zen_garden_spot(&mut x, &mut y);
                    new_plant.x = x;
                    new_plant.y = y;
                    info.m_num_potted_plants += 1;
                } else {
                    return;
                }
            }
        } else {
            return;
        }
        let _ = potted_plant_index;
        if let Some(app) = self.app {
            unsafe {
                if (*app).game_mode == GameMode::ChallengeZenGarden {
                    if let Some(board) = self.board {
                        let _ = board;
                        // PlacePottedPlant + DoPlantingEffects 依赖种植系统，暂略
                    }
                }
            }
        }
    }

    pub fn potted_plant_from_index(&self, potted_plant_index: usize) -> Option<*mut PottedPlant> {
        // 对应 C++ PottedPlantFromIndex
        if let Some(app) = self.app {
            unsafe {
                let info = &mut (*app).player_info;
                if let Some(info) = info.as_mut() {
                    if potted_plant_index < info.m_potted_plant.len() {
                        return Some(&mut info.m_potted_plant[potted_plant_index] as *mut PottedPlant);
                    }
                }
            }
        }
        None
    }

    pub fn mouse_down_with_tool(&mut self, x: i32, y: i32, cursor_type: CursorType) {
        // 对应 C++ MouseDownWithTool：工具分发
        if cursor_type == CursorType::Wheelbarrow && self.get_potted_plant_in_wheelbarrow().is_some() {
            self.mouse_down_with_full_wheel_barrow(x, y);
            if let Some(board_ptr) = self.board {
                let board = unsafe { &mut *board_ptr };
                board.clear_cursor();
            }
            return;
        }

        if cursor_type == CursorType::WateringCan
            || cursor_type == CursorType::Fertilizer
            || cursor_type == CursorType::BugSpray
            || cursor_type == CursorType::Phonograph
            || cursor_type == CursorType::Chocolate
        {
            self.mouse_down_with_feeding_tool(x, y, cursor_type);
            return;
        }

        // 简单命中：找鼠标下方带盆栽索引的植物
        let a_plant_idx = self.find_plant_at(x, y);
        match a_plant_idx {
            None => {
                // C++: PlayFoley(FOLEY_DROP) + ClearCursor
                if let Some(board_ptr) = self.board {
                    let board = unsafe { &mut *board_ptr };
                    board.clear_cursor();
                }
            }
            Some(idx) => {
                let (a_seed_type, a_imitater, a_pot_index) = {
                    let board_ptr = match self.board {
                        Some(b) => b,
                        None => return,
                    };
                    let b = unsafe { &*board_ptr };
                    let p = &b.plants[idx];
                    (p.seed_type, p.imitater_type, p.potted_plant_index)
                };
                if a_pot_index == -1 {
                    if let Some(board_ptr) = self.board {
                        let board = unsafe { &mut *board_ptr };
                        board.clear_cursor();
                    }
                    return;
                }
                match cursor_type {
                    CursorType::MoneySign => {
                        if let Some(board_ptr) = self.board {
                            let board = unsafe { &mut *board_ptr };
                            let plant = &mut board.plants[idx];
                            self.mouse_down_with_money_sign(plant);
                        }
                    }
                    CursorType::Wheelbarrow => {
                        if let Some(board_ptr) = self.board {
                            let board = unsafe { &mut *board_ptr };
                            let plant = &mut board.plants[idx];
                            self.mouse_down_with_empty_wheel_barrow(plant);
                            board.clear_cursor();
                        }
                    }
                    CursorType::Glove => {
                        if let Some(board_ptr) = self.board {
                            let board = unsafe { &mut *board_ptr };
                            board.cursor_object.cursor_type = CursorType::PlantFromGlove;
                            board.cursor_object.glove_plant_id = idx as PlantID;
                            let _ = (a_seed_type, a_imitater);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// 简单命中：按鼠标坐标找前方植物（对应 C++ ToolHitTest 简化）
    fn find_plant_at(&self, x: i32, y: i32) -> Option<usize> {
        let board_ptr = self.board?;
        let board = unsafe { &*board_ptr };
        for (i, p) in board.plants.iter().enumerate() {
            if p.dead { continue; }
            let (px, pw) = (p.base.x, p.base.width);
            let (py, ph) = (p.base.y, p.base.height);
            if x >= px && x <= px + pw && y >= py && y <= py + ph {
                return Some(i);
            }
        }
        None
    }

    pub fn move_plant(&mut self, plant: &mut Plant, grid_x: i32, grid_y: i32) {
        // 对应 C++ MovePlant：将植物（连带下方花盆）移到新格
        if let Some(app) = self.app {
            unsafe {
                if (*app).game_mode != GameMode::ChallengeZenGarden {
                    return;
                }
            }
        }
        let board_ptr = match self.board {
            Some(b) => b,
            None => return,
        };
        let (a_pos_x, a_pos_y) = unsafe {
            let b = &*board_ptr;
            (b.grid_to_pixel_x(grid_x, grid_y), b.grid_to_pixel_y(grid_x, grid_y))
        };

        // 找到下方花盆（同格 Flowerpot），一起移动
        let a_old_col = plant.plant_col;
        let a_old_row = plant.base.row;
        let mut a_pot_positions: Option<(i32, i32)> = None;
        let board = unsafe { &*board_ptr };
        for p in board.plants.iter() {
            if p.dead { continue; }
            if p.plant_col == a_old_col && p.base.row == a_old_row && p.seed_type == SeedType::Flowerpot {
                a_pot_positions = Some((p.pos_x as i32, p.pos_y as i32));
                break;
            }
        }
        let a_delta_x = a_pos_x - plant.base.x;
        let a_delta_y = a_pos_y - plant.base.y;

        // 移动植物本体
        plant.base.x = a_pos_x;
        plant.base.y = a_pos_y;
        plant.plant_col = grid_x;
        plant.base.row = grid_y;
        plant.base.render_order = crate::lawn::board::make_render_order(
            crate::lawn::game_enums::RENDER_LAYER_PLANT, 0, a_pos_y + 1,
        );

        // 移动花盆
        if let Some((pot_x, pot_y)) = a_pot_positions {
            let board = unsafe { &mut *board_ptr };
            for p in board.plants.iter_mut() {
                if p.dead { continue; }
                if p.plant_col == a_old_col && p.base.row == a_old_row && p.seed_type == SeedType::Flowerpot {
                    p.base.x = pot_x + a_delta_x;
                    p.base.y = pot_y + a_delta_y;
                    p.plant_col = grid_x;
                    p.base.row = grid_y;
                    p.base.render_order = crate::lawn::board::make_render_order(
                        crate::lawn::game_enums::RENDER_LAYER_PLANT, 0, a_pos_y,
                    );
                    break;
                }
            }
        }

        // 更新 PottedPlant 数据
        let a_potted_index = plant.potted_plant_index;
        if a_potted_index >= 0 {
            if let Some(app) = self.app {
                unsafe {
                    if let Some(info) = (*app).player_info.as_mut() {
                        if (a_potted_index as usize) < info.m_potted_plant.len() {
                            info.m_potted_plant[a_potted_index as usize].x = grid_x;
                            info.m_potted_plant[a_potted_index as usize].y = grid_y;
                        }
                    }
                }
            }
        }
        // [TRANSLATION_NOTE]: 粒子系统移动（Particle SystemMove）与 DoPlantingEffects 依赖粒子/种植系统，暂不执行
    }

    pub fn mouse_down_with_money_sign(&mut self, plant: &mut Plant) {
        // 对应 C++ MouseDownWithMoneySign：确认出售盆栽植物
        if let Some(board) = self.board {
            unsafe { (*board).clear_cursor(); }
        }
        let a_header = "[ZEN_SELL_HEADER]".to_string();
        let a_lines = "[ZEN_SELL_LINES]".to_string();
        let a_price = self.get_plant_sell_price(plant);

        if let Some(app) = self.app {
            unsafe { if (*app).m_crazy_dave_state == CrazyDaveState::Off { (*app).crazy_dave_enter(); } }
        }

        let a_potted_plant = self.potted_plant_from_index(plant.potted_plant_index as usize);
        let mut a_message_text = self.app.map_or(String::new(), |app| unsafe {
            (*app).get_crazy_dave_text(1700)
        });
        a_message_text = a_message_text.replace("{SELL_PRICE}", &a_price.to_string());

        let a_plant_name = if plant.seed_type == SeedType::Sprout
            && a_potted_plant.map_or(false, |pp| unsafe { (*pp).seed_type == SeedType::Marigold })
        {
            "[MARIGOLD_SPROUT]".to_string()
        } else {
            crate::lawn::plant::Plant::get_name_string(plant.seed_type, plant.imitater_type)
        };
        a_message_text = a_message_text.replace("{PLANT_TYPE}", &a_plant_name);

        if let Some(app) = self.app {
            unsafe {
                (*app).crazy_dave_talk_message(&a_message_text);
                if let Some(r) = (*app).reanimation_get_mut((*app).m_crazy_dave_reanim_id) {
                    r.play_reanim("anim_blahblah", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 20, 12.0);
                }
            }
        }

        // 对应 C++ DataArrayGetID(thePlant)：记录植物索引
        let a_plant_id = self.plant_index_of(plant);

        let a_dialog = self.app.and_then(|app| unsafe {
            (*app).do_dialog(Dialogs::ZenSell as i32, true, &a_header, &a_lines, "", BUTTONS_YES_NO)
        });
        if let Some(d) = a_dialog {
            unsafe {
                (*d).x += 120;
                (*d).y += 60;
            }
        }
        if let Some(board) = self.board {
            unsafe { (*board).show_coin_bank(0); }
        }
        let a_result = a_dialog.map_or(0, |d| unsafe { (*d).wait_for_result(true) });
        if let Some(app) = self.app {
            unsafe { (*app).crazy_dave_leave(); }
        }

        if a_result == crate::framework::widget::dialog::ID_YES {
            // 模态等待期间植物可能已被替换
            if self.plant_index_of(plant) != a_plant_id {
                return;
            }
            let sell_index = self.plant_index_of(plant);
            let Some(sell_index) = sell_index else { return };
            let sell_plant_index;
            {
                let board = self.board;
                let Some(board) = board else { return };
                unsafe {
                    let b = &mut *board;
                    if sell_index >= b.plants.len() || b.plants[sell_index].dead {
                        return;
                    }
                    sell_plant_index = b.plants[sell_index].potted_plant_index;
                }
            }

            if let Some(app) = self.app {
                unsafe {
                    if let Some(info) = (*app).player_info.as_mut() {
                        info.add_coins(a_price);
                    }
                    if let Some(board) = self.board {
                        (*board).m_coins_collected += a_price;
                    }
                    // 数组前移（对应 C++ memmove）+ 其余植物索引修正
                    if let Some(info) = (*app).player_info.as_mut() {
                        let num_after = info.m_num_potted_plants - sell_plant_index - 1;
                        if num_after > 0 {
                            let idx = sell_plant_index as usize;
                            if idx + 1 < info.m_potted_plant.len() {
                                info.m_potted_plant.remove(idx);
                            }
                            if let Some(board) = self.board {
                                unsafe {
                                    let b = &mut *board;
                                    for i in 0..b.plants.len() {
                                        if b.plants[i].dead {
                                            continue;
                                        }
                                        if b.plants[i].potted_plant_index > sell_plant_index {
                                            b.plants[i].potted_plant_index -= 1;
                                        }
                                    }
                                }
                            }
                        }
                        info.m_num_potted_plants -= 1;
                    }
                    (*app).play_foley(FoleyType::UseShovel as i32);
                }
            }

            if let Some(board) = self.board {
                unsafe {
                    let b = &mut *board;
                    let plant_ptr = &mut b.plants[sell_index] as *mut Plant;
                    self.remove_potted_plant(unsafe { &mut *plant_ptr });
                }
            }
        }
    }

    /// 查找植物在棋盘植物数组中的索引（对应 C++ DataArrayGetID）
    fn plant_index_of(&self, target: &Plant) -> Option<usize> {
        if let Some(board) = self.board {
            unsafe {
                let b = &*board;
                for i in 0..b.plants.len() {
                    if std::ptr::eq(&b.plants[i] as *const Plant, target as *const Plant) {
                        return Some(i);
                    }
                }
            }
        }
        None
    }

    pub fn place_potted_plant(&mut self, potted_plant_index: usize) -> Option<*mut Plant> {
        // 对应 C++ PlacePottedPlant：放花盆（如需）+ 放植物
        let (a_x, a_y, a_seed_type, a_maturity) = {
            let app = self.app?;
            unsafe {
                let info = (*app).player_info.as_ref()?;
                let pp = info.m_potted_plant.get(potted_plant_index)?;
                (pp.x, pp.y, pp.seed_type, pp.plant_age)
            }
        };
        let a_plant_seed = if a_maturity == PottedPlantAge::Sprout {
            SeedType::Sprout
        } else {
            a_seed_type
        };
        let board_ptr = self.board?;
        let board = unsafe { &mut *board_ptr };

        // 需花盆判定（C++ needPot）：蘑菇园非水生不用花盆，水族馆不用
        let a_garden_type = self.garden_type;
        let a_need_pot = !(a_garden_type == GardenType::Mushroom && !Plant::is_aquatic(a_plant_seed))
            && a_garden_type != GardenType::Aquarium;
        if a_need_pot {
            if let Some(_pot) = board.new_plant(a_x, a_y, SeedType::Flowerpot, SeedType::None) {
                // [TRANSLATION_NOTE]: 花盆 reanim 帧层（anim_waterplants/anim_zengarden）依赖体动画，暂不设置
            }
        }

        let a_plant = board.new_plant(a_x, a_y, a_plant_seed, SeedType::None)?;
        a_plant.potted_plant_index = potted_plant_index as i32;
        a_plant.base.render_order = crate::lawn::board::make_render_order(
            crate::lawn::game_enums::RENDER_LAYER_PLANT, 0, a_plant.base.y + 1,
        );
        // [TRANSLATION_NOTE]: 发芽/水族馆/变体帧层与 UpdateReanim 依赖 reanim 系统，暂留基础动画
        let idx = board.plants.len() - 1;
        Some(&mut board.plants[idx] as *mut Plant)
    }

    pub fn zen_garden_update(&mut self) {
        // 对应 C++ ZenGardenUpdate：商店打开时跳过更新
        if let Some(app) = self.app {
            unsafe {
                if (*app).base.dialog_map.contains_key(&(Dialogs::Store as i32)) {
                    return;
                }
                self.now_time = (*app).get_now_time();
                self.now_tm = (*app).get_local_time(self.now_time);
                (*app).update_crazy_dave();
            }
        }

        if let Some(board) = self.board {
            unsafe {
                if (*board).cursor_object.cursor_type != CursorType::Normal {
                    if let Some(ch) = (*board).challenge.as_mut() {
                        ch.challenge_state = ChallengeState::Normal;
                        ch.challenge_state_counter = 3000;
                    }
                } else if (*board).m_tutorial_state == TutorialState::Off {
                    if let Some(ch) = (*board).challenge.as_mut() {
                        if ch.challenge_state_counter > 0 {
                            ch.challenge_state_counter -= 1;
                        }
                        if ch.challenge_state == ChallengeState::Normal && ch.challenge_state_counter == 0 {
                            ch.challenge_state = ChallengeState::ZenFading;
                            ch.challenge_state_counter = 50;
                        }
                    }
                }
            }
        }

        self.update_plant_needs();

        if let Some(board) = self.board {
            unsafe {
                let b = &mut *board;
                for i in 0..b.plants.len() {
                    if b.plants[i].dead {
                        continue;
                    }
                    if b.plants[i].potted_plant_index != -1 {
                        let plant_ptr = &mut b.plants[i] as *mut Plant;
                        self.potted_plant_update(unsafe { &mut *plant_ptr });
                    }
                }
                for i in 0..b.grid_items.len() {
                    if b.grid_items[i].dead {
                        continue;
                    }
                    let item_type = b.grid_items[i].grid_item_type;
                    if item_type == crate::lawn::grid_item::GridItemType::ZenTool {
                        let item_ptr = &mut b.grid_items[i] as *mut GridItem;
                        self.zen_tool_update(unsafe { &mut *item_ptr });
                    } else if item_type == crate::lawn::grid_item::GridItemType::PlantStinky {
                        let item_ptr = &mut b.grid_items[i] as *mut GridItem;
                        self.stinky_update(unsafe { &mut *item_ptr });
                    }
                }
            }
        }

        if let Some(board) = self.board {
            unsafe {
                if (*board).m_tutorial_state == TutorialState::ZenGardenKeepWatering
                    && self.count_plants_needing_fertilizer() > 0
                {
                    (*board).display_advice(
                        "[ADVICE_ZEN_GARDEN_VISIT_STORE]",
                        MessageStyle::HintTallLong as i32,
                        AdviceType::None,
                    );
                    (*board).m_tutorial_state = TutorialState::ZenGardenVisitStore;
                    // [TRANSLATION_NOTE]: C++ 中 mStoreButton->mDisabled=false /
                    // mBtnNoDraw=false；Rust 侧 store_button 仅为 Option<i32>，无按钮对象
                }
            }
        }
    }

    pub fn mouse_down_with_full_wheel_barrow(&mut self, x: i32, y: i32) {
        // 对应 C++ MouseDownWithFullWheelBarrow：从独轮车取出盆栽放到网格
        let a_potted_plant = self.get_potted_plant_in_wheelbarrow();
        let Some(a_potted_plant) = a_potted_plant else { return };

        unsafe {
            if self.garden_type == GardenType::Aquarium
                && !Plant::is_aquatic((*a_potted_plant).seed_type)
            {
                if let Some(board) = self.board {
                    (*board).display_advice(
                        "[ZEN_ONLY_AQUATIC_PLANTS]",
                        MessageStyle::HintTallFast as i32,
                        AdviceType::None,
                    );
                }
                return;
            }

            let a_grid_x = self.board.map_or(-1, |b| unsafe { (*b).pixel_to_grid_x(x, y) });
            let a_grid_y = self.board.map_or(-1, |b| unsafe { (*b).pixel_to_grid_y(x, y) });
            if a_grid_x == -1 || a_grid_y == -1 {
                return;
            }
            let plant_ok = self.board.map_or(PlantingReason::NotHere, |b| unsafe {
                (*b).can_plant_at(a_grid_x, a_grid_y, (*a_potted_plant).seed_type)
            });
            if plant_ok != PlantingReason::Ok {
                return;
            }

            (*a_potted_plant).which_zen_garden = self.garden_type;
            (*a_potted_plant).x = a_grid_x;
            (*a_potted_plant).y = a_grid_y;

            // 对应 C++ 指针差计算 PottedPlantIndex
            let a_potted_plant_index = self.potted_plant_index_of(a_potted_plant);
            let Some(a_potted_plant_index) = a_potted_plant_index else { return };

            let a_plant = self.place_potted_plant(a_potted_plant_index);
            if let Some(board) = self.board {
                if a_plant.is_some() {
                    (*board).do_planting_effects(
                        (*a_potted_plant).x,
                        (*a_potted_plant).y,
                        (*a_potted_plant).seed_type,
                    );
                }
            }
        }
    }

    /// 查找盆栽在玩家资料数组中的索引（对应 C++ 指针差）
    fn potted_plant_index_of(&self, target: *mut PottedPlant) -> Option<usize> {
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_ref() {
                    for (i, pp) in info.m_potted_plant.iter().enumerate() {
                        if pp as *const PottedPlant == target as *const PottedPlant {
                            return Some(i);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn mouse_down_with_empty_wheel_barrow(&mut self, plant: &mut Plant) {
        // 对应 C++ MouseDownWithEmptyWheelBarrow：装入独轮车并重置坐标
        let a_potted_plant = self.potted_plant_from_index(plant.potted_plant_index as usize);
        self.remove_potted_plant(plant);
        if let Some(pp) = a_potted_plant {
            unsafe {
                (*pp).which_zen_garden = GardenType::Wheelbarrow;
                (*pp).x = 0;
                (*pp).y = 0;
            }
        }
        if let Some(app) = self.app {
            unsafe { (*app).play_foley(FoleyType::Plant as i32); }
        }
    }

    pub fn goto_next_garden(&mut self) {
        // 对应 C++ GotoNextGarden：离开当前花园并切换到下一个已购花园（或智慧树）
        self.leave_garden();
        if let Some(board) = self.board {
            unsafe { (*board).clear_advice(AdviceType::None); }
        }
        if let Some(app) = self.app {
            unsafe { (*app).crazy_dave_die(); }
        }
        if let Some(board) = self.board {
            unsafe {
                (*board).plants.clear();
                (*board).coins.clear();
            }
        }
        // 对应 C++ mEffectSystem->EffectSystemFreeAll()：清空全部特效
        if let Some(app) = self.app {
            unsafe {
                if let Some(es) = (*app).effect_system.as_mut() {
                    es.effect_system_free_all();
                }
            }
        }

        let mut a_go_to_tree = false;
        if self.garden_type == GardenType::Main {
            if self.has_purchased_item(StoreItem::MushroomGarden) {
                self.garden_type = GardenType::Mushroom;
                if let Some(board) = self.board {
                    unsafe { (*board).m_background_type = BackgroundType::MushroomGarden; }
                }
            } else if self.has_purchased_item(StoreItem::AquariumGarden) {
                self.garden_type = GardenType::Aquarium;
                if let Some(board) = self.board {
                    unsafe { (*board).m_background_type = BackgroundType::Zombiquarium; }
                }
            } else if self.has_purchased_item(StoreItem::TreeOfWisdom) {
                a_go_to_tree = true;
            }
        } else if self.garden_type == GardenType::Mushroom {
            if self.has_purchased_item(StoreItem::AquariumGarden) {
                self.garden_type = GardenType::Aquarium;
                if let Some(board) = self.board {
                    unsafe { (*board).m_background_type = BackgroundType::Zombiquarium; }
                }
            } else if self.has_purchased_item(StoreItem::TreeOfWisdom) {
                a_go_to_tree = true;
            } else {
                self.garden_type = GardenType::Main;
                if let Some(board) = self.board {
                    unsafe { (*board).m_background_type = BackgroundType::Greenhouse; }
                }
            }
        } else if self.garden_type == GardenType::Aquarium {
            if self.has_purchased_item(StoreItem::TreeOfWisdom) {
                a_go_to_tree = true;
            } else {
                self.garden_type = GardenType::Main;
                if let Some(board) = self.board {
                    unsafe { (*board).m_background_type = BackgroundType::Greenhouse; }
                }
            }
        }

        if a_go_to_tree {
            if let Some(app) = self.app {
                unsafe {
                    // [TRANSLATION_NOTE]: C++ 中先 ReleaseTrackedResources(mLoadedResourceNames)
                    (*app).kill_board();
                    (*app).pre_new_game(GameMode::ChallengeTreeOfWisdom, false);
                }
            }
            return;
        }

        // [TRANSLATION_NOTE]: C++ 中 ReleaseTrackedResources 后按背景加载资源组；
        // Rust 侧 mLoadedResourceNames 跟踪未接入，直接加载对应组
        if let Some(app) = self.app {
            unsafe {
                if let Some(rm) = (*app).base.resource_manager.as_mut() {
                    let background = self.board.map_or(BackgroundType::Greenhouse, |b| unsafe { (*b).m_background_type });
                    match background {
                        BackgroundType::MushroomGarden => {
                            let _ = (**rm).load_resources("DelayLoad_MushroomGarden");
                        }
                        BackgroundType::Greenhouse => {
                            let _ = (**rm).load_resources("DelayLoad_GreenHouseGarden");
                            let _ = (**rm).load_resources("DelayLoad_GreenHouseOverlay");
                        }
                        BackgroundType::Zombiquarium => {
                            let _ = (**rm).load_resources("DelayLoad_Zombiquarium");
                            let _ = (**rm).load_resources("DelayLoad_GreenHouseOverlay");
                        }
                        _ => {}
                    }
                }
            }
        }

        if let Some(board) = self.board {
            unsafe {
                let background = (*board).m_background_type;
                if (background == BackgroundType::MushroomGarden || background == BackgroundType::Zombiquarium)
                    && !self.has_purchased_item(StoreItem::WheelBarrow)
                {
                    (*board).display_advice(
                        "[ADVICE_NEED_WHEELBARROW]",
                        MessageStyle::HintTallFast as i32,
                        AdviceType::NeedWheelbarrow,
                    );
                }
            }
        }

        self.zen_garden_init_level();
    }

    /// 玩家是否已购买指定商店物品（对应 C++ mPurchases[item] 非零判断）
    fn has_purchased_item(&self, item: StoreItem) -> bool {
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_ref() {
                    let idx = item as usize;
                    return info.m_purchases.get(idx).copied().unwrap_or(0) != 0;
                }
            }
        }
        false
    }

    pub fn get_potted_plant_in_wheelbarrow(&self) -> Option<*mut PottedPlant> {
        // 对应 C++ GetPottedPlantInWheelbarrow：查找 whichZenGarden==WHEELBARROW 的盆栽
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_ref() {
                    for (i, pp) in info.m_potted_plant.iter().enumerate() {
                        if pp.which_zen_garden == GardenType::Wheelbarrow {
                            return self.potted_plant_from_index(i);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn remove_potted_plant(&mut self, plant: &mut Plant) {
        // 对应 C++ RemovePottedPlant：碾碎植物 + 压碎下方花盆
        plant.die();
        let a_plant_col = plant.plant_col;
        let a_plant_row = plant.base.row;
        if let Some(board_ptr) = self.board {
            let board = unsafe { &mut *board_ptr };
            // C++: GetTopPlantAt(col, row, TOPPLANT_ONLY_UNDER_PLANT) 找下方花盆
            let mut a_pot_idx: Option<usize> = None;
            for (i, p) in board.plants.iter().enumerate() {
                if p.dead { continue; }
                if p.plant_col == a_plant_col && p.base.row == a_plant_row
                    && p.seed_type == SeedType::Flowerpot
                {
                    a_pot_idx = Some(i);
                    break;
                }
            }
            if let Some(i) = a_pot_idx {
                board.plants[i].die();
            }
        }
    }

    pub fn get_special_grid_placements(&self, count: &mut i32) -> Option<*const SpecialGridPlacement> {
        // 对应 C++ GetSpecialGridPlacements：按背景类型返回对应静态网格表
        let background = self.board.map_or(BackgroundType::Greenhouse, |b| unsafe { (*b).m_background_type });
        match background {
            BackgroundType::MushroomGarden => {
                *count = MUSHROOM_GRID_PLACEMENT.len() as i32;
                Some(MUSHROOM_GRID_PLACEMENT.as_ptr())
            }
            BackgroundType::Zombiquarium => {
                *count = AQUARIUM_GRID_PLACEMENT.len() as i32;
                Some(AQUARIUM_GRID_PLACEMENT.as_ptr())
            }
            BackgroundType::Greenhouse | BackgroundType::TreeOfWisdom => {
                *count = GREENHOUSE_GRID_PLACEMENT.len() as i32;
                Some(GREENHOUSE_GRID_PLACEMENT.as_ptr())
            }
            _ => {
                *count = 0;
                None
            }
        }
    }

    pub fn pixel_to_grid_x(&self, x: i32, y: i32) -> i32 {
        // 对应 C++ PixelToGridX
        let mut count = 0;
        if let Some(placements) = self.get_special_grid_placements(&mut count) {
            let placements = unsafe { std::slice::from_raw_parts(placements, count as usize) };
            for grid in placements {
                if x >= grid.pixel_x && x <= grid.pixel_x + 80 && y >= grid.pixel_y && y <= grid.pixel_y + 85 {
                    return grid.grid_x;
                }
            }
        }
        -1
    }

    pub fn pixel_to_grid_y(&self, x: i32, y: i32) -> i32 {
        // 对应 C++ PixelToGridY
        let mut count = 0;
        if let Some(placements) = self.get_special_grid_placements(&mut count) {
            let placements = unsafe { std::slice::from_raw_parts(placements, count as usize) };
            for grid in placements {
                if x >= grid.pixel_x && x <= grid.pixel_x + 80 && y >= grid.pixel_y && y <= grid.pixel_y + 85 {
                    return grid.grid_y;
                }
            }
        }
        -1
    }

    pub fn grid_to_pixel_x(&self, grid_x: i32, grid_y: i32) -> i32 {
        // 对应 C++ GridToPixelX
        let mut count = 0;
        if let Some(placements) = self.get_special_grid_placements(&mut count) {
            let placements = unsafe { std::slice::from_raw_parts(placements, count as usize) };
            for grid in placements {
                if grid_x == grid.grid_x && grid_y == grid.grid_y {
                    return grid.pixel_x;
                }
            }
        }
        -1
    }

    pub fn grid_to_pixel_y(&self, grid_x: i32, grid_y: i32) -> i32 {
        // 对应 C++ GridToPixelY
        let mut count = 0;
        if let Some(placements) = self.get_special_grid_placements(&mut count) {
            let placements = unsafe { std::slice::from_raw_parts(placements, count as usize) };
            for grid in placements {
                if grid_x == grid.grid_x && grid_y == grid.grid_y {
                    return grid.pixel_y;
                }
            }
        }
        -1
    }

    pub fn draw_backdrop(&self, g: &mut Graphics) {
        // 对应 C++ DrawBackdrop：水族馆中在空位绘制植物阴影（引导放置）
        if self.garden_type != GardenType::Aquarium {
            return;
        }
        if let Some(board) = self.board {
            unsafe {
                let cursor_type = (*board).cursor_object.cursor_type;
                if cursor_type == CursorType::PlantFromWheelBarrow
                    || cursor_type == CursorType::Wheelbarrow
                    || cursor_type == CursorType::PlantFromGlove
                {
                    let mut count = 0;
                    let placements = self.get_special_grid_placements(&mut count);
                    if let Some(p) = placements {
                        for i in 0..count {
                            let a_grid = unsafe { &*p.add(i as usize) };
                            if (*board).get_top_plant_at(a_grid.grid_x, a_grid.grid_y).is_none() {
                                // [TRANSLATION_NOTE]: C++ 中 PvzpDrawImageCelScaled(
                                // IMAGE_PLANTSHADOW, aGrid.mPixelX-35, aGrid.mPixelY+33, 0,0, 1.7,1.7)
                                // 绘制植物阴影；Rust 侧图片资源未接入，暂略
                                let _ = (a_grid.pixel_x, a_grid.pixel_y);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn mouse_down_zen_garden(&mut self, x: i32, y: i32, click_count: i32, hit_result: &mut HitResult) -> bool {
        // 对应 C++ MouseDownZenGarden
        if let Some(board) = self.board {
            unsafe {
                if let Some(ch) = (*board).challenge.as_mut() {
                    if ch.challenge_state == ChallengeState::ZenFading {
                        ch.challenge_state = ChallengeState::Normal;
                    }
                    ch.challenge_state_counter = 3000;
                }

                if hit_result.object_type == GameObjectType::Stinky
                    && (*board).cursor_object.cursor_type == CursorType::Normal
                {
                    self.wake_stinky();
                } else if (*board).cursor_object.cursor_type == CursorType::Glove {
                    if (*board).can_use_game_object(GameObjectType::Wheelbarrow) {
                        let a_button_rect = (*board).get_zen_button_rect(GameObjectType::Wheelbarrow);
                        let a_potted_plant = self.get_potted_plant_in_wheelbarrow();
                        if a_button_rect.contains(x, y) && a_potted_plant.is_some() {
                            (*board).clear_cursor();
                            let pp = a_potted_plant.unwrap();
                            (*board).cursor_object.seed_type = (*pp).seed_type;
                            (*board).cursor_object.imitater_type = SeedType::None;
                            (*board).cursor_object.cursor_type = CursorType::PlantFromWheelBarrow;
                            return true;
                        }
                    }
                } else if (*board).cursor_object.cursor_type == CursorType::PlantFromGlove {
                    if (*board).can_use_game_object(GameObjectType::Wheelbarrow) {
                        let a_button_rect = (*board).get_zen_button_rect(GameObjectType::Wheelbarrow);
                        let glove_id = (*board).cursor_object.glove_plant_id as usize;
                        let b = &mut *board;
                        let plant_alive = glove_id < b.plants.len() && !b.plants[glove_id].dead;
                        if plant_alive && a_button_rect.contains(x, y)
                            && self.get_potted_plant_in_wheelbarrow().is_none()
                        {
                            let plant_ptr = &mut b.plants[glove_id] as *mut crate::lawn::plant::Plant;
                            self.mouse_down_with_empty_wheel_barrow(unsafe { &mut *plant_ptr });
                            b.clear_cursor();
                            return true;
                        }
                    }
                } else if hit_result.object_type == GameObjectType::None
                    && (*board).cursor_object.cursor_type == CursorType::Normal
                    && self.garden_type == GardenType::Aquarium
                    && click_count <= -1
                {
                    // [TRANSLATION_NOTE]: C++ 中 PlaySample(SOUND_TAPGLASS)
                }
            }
        }

        if let Some(app) = self.app {
            unsafe {
                if (*app).m_crazy_dave_message_index != -1 {
                    self.advance_crazy_dave_dialog();
                    return true;
                }
            }
        }

        false
    }

    pub fn plant_fulfill_need(&mut self, plant: &mut Plant) {
        // 对应 C++ PlantFulfillNeed
        let a_potted_index = plant.potted_plant_index;
        let a_seed_type = plant.seed_type;
        let a_x = plant.base.x;
        let a_y = plant.base.y;
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_mut() {
                    if a_potted_index >= 0 && (a_potted_index as usize) < info.m_potted_plant.len() {
                        let pp = &mut info.m_potted_plant[a_potted_index as usize];
                        pp.last_need_fulfilled_time = self.now_time;
                        pp.plant_need = PottedPlantNeed::None;
                        pp.times_fed = 0;
                    }
                }
                (*app).play_foley(crate::todlib::tod_foley::FoleyType::Prize as i32);
                (*app).play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
            }
        }
        if let Some(board) = self.board {
            let b = unsafe { &mut *board };
            b.add_coin((a_x + 40) as f32, a_y as f32, CoinType::Gold, CoinMotion::Coin);
            if Plant::is_nocturnal(a_seed_type) || Plant::is_aquatic(a_seed_type) {
                b.add_coin((a_x + 10) as f32, a_y as f32, CoinType::Gold, CoinMotion::Coin);
                b.add_coin((a_x + 70) as f32, a_y as f32, CoinType::Gold, CoinMotion::Coin);
            }
        }
    }

    pub fn plant_watered(&mut self, plant: &mut Plant) {
        // 对应 C++ PlantWatered
        let a_potted_index = plant.potted_plant_index;
        let a_x = plant.base.x;
        let a_y = plant.base.y;
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_mut() {
                    if a_potted_index >= 0 && (a_potted_index as usize) < info.m_potted_plant.len() {
                        let pp = &mut info.m_potted_plant[a_potted_index as usize];
                        pp.times_fed += 1;
                        let mut a_time_span = crate::framework::common::rand_range(9); // RandRangeInt(0, 8)
                        let tutorial = self.board.map_or(TutorialState::Off, |b| unsafe { (*b).m_tutorial_state });
                        if tutorial == TutorialState::ZenGardenWaterPlant
                            || tutorial == TutorialState::ZenGardenKeepWatering
                        {
                            a_time_span = 9;
                        }
                        pp.last_watered_time = self.now_time - a_time_span as i64;
                    }
                }
                (*app).play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
            }
        }
        if let Some(board) = self.board {
            let b = unsafe { &mut *board };
            b.add_coin((a_x + 40) as f32, a_y as f32, CoinType::Silver, CoinMotion::Coin);
        }
    }

    pub fn get_plants_need(&self, potted_plant: &PottedPlant) -> PottedPlantNeed {
        // 对应 C++ GetPlantsNeed
        if potted_plant.plant_age != PottedPlantAge::Sprout
            && Plant::is_nocturnal(potted_plant.seed_type)
            && potted_plant.which_zen_garden == GardenType::Main
        {
            return PottedPlantNeed::None;
        }
        if potted_plant.which_zen_garden == GardenType::Wheelbarrow {
            return PottedPlantNeed::None;
        }

        let a_now = self.now_time;
        let a_too_long_since_watering = a_now - potted_plant.last_watered_time > 15;
        let a_too_short_since_watering = a_now - potted_plant.last_watered_time < 3;

        if self.was_plant_fertilized_in_last_hour(potted_plant)
            || self.was_plant_need_fulfilled_today(potted_plant)
        {
            return PottedPlantNeed::None;
        }
        if Plant::is_aquatic(potted_plant.seed_type) && potted_plant.plant_age != PottedPlantAge::Sprout {
            if potted_plant.plant_age == PottedPlantAge::Full {
                if self.plant_should_refresh_need(potted_plant) {
                    return PottedPlantNeed::None;
                }
                return potted_plant.plant_need;
            } else {
                if potted_plant.which_zen_garden != GardenType::Aquarium {
                    return PottedPlantNeed::None;
                }
                return PottedPlantNeed::Fertilizer;
            }
        }
        if !a_too_long_since_watering {
            return PottedPlantNeed::None;
        }
        if potted_plant.times_fed < potted_plant.feedings_per_grow {
            return PottedPlantNeed::Water;
        }
        if a_too_short_since_watering {
            return PottedPlantNeed::None;
        }
        if potted_plant.plant_age != PottedPlantAge::Full {
            return PottedPlantNeed::Fertilizer;
        }
        if self.plant_should_refresh_need(potted_plant) {
            return PottedPlantNeed::None;
        }
        if potted_plant.plant_need != PottedPlantNeed::None {
            return potted_plant.plant_need;
        }
        PottedPlantNeed::Water
    }

    pub fn mouse_down_with_feeding_tool(&mut self, x: i32, y: i32, cursor_type: CursorType) {
        // 对应 C++ ZenGarden::MouseDownWithFeedingTool
        let board_ptr = match self.board {
            Some(b) => b,
            None => return,
        };

        // C++: 遍历 mBoard->mPlants 找 mHighlighted && mPottedPlantIndex != -1 的植物
        let mut a_plant_to_feed_idx: Option<usize> = None;
        unsafe {
            let board = &mut *board_ptr;
            for (i, plant) in board.plants.iter().enumerate() {
                if plant.dead {
                    continue;
                }
                if plant.highlighted && plant.potted_plant_index != -1 {
                    a_plant_to_feed_idx = Some(i);
                    break;
                }
            }
        }

        if cursor_type == CursorType::Chocolate {
            if let Some(stinky_ptr) = self.get_stinky() {
                unsafe {
                    if (*stinky_ptr).highlighted {
                        self.wake_stinky();
                        // [TRANSLATION_NOTE]: C++ 中 AddPvzpParticle(stinky, PARTICLE_PRESENT_PICKUP)
                        // 粒子附加未接入，暂略
                        if let Some(app) = self.app {
                            if let Some(info) = (*app).player_info.as_mut() {
                                info.m_last_stinky_chocolate_time = self.now_time as u32;
                                let idx = StoreItem::Chocolate as usize;
                                if idx < info.m_purchases.len() {
                                    info.m_purchases[idx] -= 1;
                                }
                            }
                            (*app).play_foley(FoleyType::WakeUp as i32);
                            (*app).play_sample(
                                crate::framework::resources::ResourceId::SoundMindcontrolled as i32,
                            );
                        }
                    }
                }
            }
            if let Some(idx) = a_plant_to_feed_idx {
                unsafe {
                    let plants = &mut (*board_ptr).plants;
                    let plant = &mut plants[idx];
                    if let Some(app) = self.app {
                        if let Some(info) = (*app).player_info.as_mut() {
                            let cidx = StoreItem::Chocolate as usize;
                            if cidx < info.m_purchases.len() {
                                info.m_purchases[cidx] -= 1;
                            }
                        }
                    }
                    self.feed_chocolate_to_plant(plant);
                    if let Some(app) = self.app {
                        (*app).play_foley(FoleyType::WakeUp as i32);
                    }
                }
            }
        }

        if let Some(idx) = a_plant_to_feed_idx {
            unsafe {
                let board = &mut *board_ptr;
                let plant = &board.plants[idx];
                let mut a_zen_tool = GridItem::new();
                a_zen_tool.grid_item_type = crate::lawn::grid_item::GridItemType::ZenTool;
                a_zen_tool.grid_x = plant.plant_col;
                a_zen_tool.grid_y = plant.base.row;
                a_zen_tool.pos_x = plant.pos_x + 40.0;
                a_zen_tool.pos_y = plant.pos_y + 40.0;
                a_zen_tool.render_order =
                    crate::lawn::board::make_render_order(RENDER_LAYER_ABOVE_UI, 0, 0);

                let app = self.app;
                if cursor_type == CursorType::WateringCan {
                    let has_gold = app.map_or(false, |a| unsafe {
                        let info = (*a).player_info.as_ref().unwrap();
                        let gi = StoreItem::GoldWateringcan as usize;
                        gi < info.m_purchases.len() && info.m_purchases[gi] != 0
                    });
                    if has_gold {
                        a_zen_tool.pos_x = x as f32;
                        a_zen_tool.pos_y = y as f32;
                        if let Some(a) = app {
                            if let Some(rp) = (*a).add_reanimation(
                                x as f32,
                                y as f32,
                                0,
                                ReanimationType::ZengardenWateringcan as i32,
                            ) {
                                (*rp).play_reanim("anim_water_area", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 0, 8.0);
                                a_zen_tool.grid_item_reanim_id = (*a).reanimation_get_id(rp);
                            }
                            a_zen_tool.grid_item_state = GridItemState::ZenToolGoldWateringCan;
                            (*a).play_foley(FoleyType::Watering as i32);
                        }
                    } else {
                        if let Some(a) = app {
                            if let Some(rp) = (*a).add_reanimation(
                                plant.pos_x + 32.0,
                                plant.pos_y,
                                0,
                                ReanimationType::ZengardenWateringcan as i32,
                            ) {
                                (*rp).play_reanim("anim_water", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 0, 0.0);
                                a_zen_tool.grid_item_reanim_id = (*a).reanimation_get_id(rp);
                            }
                            a_zen_tool.grid_item_state = GridItemState::ZenToolWateringCan;
                            (*a).play_foley(FoleyType::Watering as i32);
                        }
                    }
                } else if cursor_type == CursorType::Fertilizer {
                    if let Some(a) = app {
                        if let Some(rp) = (*a).add_reanimation(
                            plant.pos_x,
                            plant.pos_y,
                            0,
                            ReanimationType::ZengardenFertilizer as i32,
                        ) {
                            (*rp).m_loop_type = crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold;
                            a_zen_tool.grid_item_reanim_id = (*a).reanimation_get_id(rp);
                        }
                        a_zen_tool.grid_item_state = GridItemState::ZenToolFertilizer;
                        (*a).play_foley(FoleyType::Fertilizer as i32);
                        let info = (*a).player_info.as_mut().unwrap();
                        let fi = StoreItem::Fertilizer as usize;
                        if fi < info.m_purchases.len() {
                            info.m_purchases[fi] -= 1;
                        }
                    }
                } else if cursor_type == CursorType::BugSpray {
                    if let Some(a) = app {
                        if let Some(rp) = (*a).add_reanimation(
                            plant.pos_x + 54.0,
                            plant.pos_y,
                            0,
                            ReanimationType::ZengardenBugspray as i32,
                        ) {
                            (*rp).m_loop_type = crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold;
                            a_zen_tool.grid_item_reanim_id = (*a).reanimation_get_id(rp);
                        }
                        a_zen_tool.grid_item_state = GridItemState::ZenToolBugSpray;
                        (*a).play_foley(FoleyType::BugSpray as i32);
                        let info = (*a).player_info.as_mut().unwrap();
                        let bi = StoreItem::BugSpray as usize;
                        if bi < info.m_purchases.len() {
                            info.m_purchases[bi] -= 1;
                        }
                    }
                } else if cursor_type == CursorType::Phonograph {
                    if let Some(a) = app {
                        if let Some(rp) = (*a).add_reanimation(
                            plant.pos_x + 20.0,
                            plant.pos_y + 34.0,
                            0,
                            ReanimationType::ZengardenPhonograph as i32,
                        ) {
                            (*rp).m_anim_rate = 20.0;
                            (*rp).m_loop_type = crate::todlib::reanimator::ReanimLoopType::Loop;
                            a_zen_tool.grid_item_reanim_id = (*a).reanimation_get_id(rp);
                        }
                        a_zen_tool.grid_item_state = GridItemState::ZenToolPhonograph;
                        (*a).play_foley(FoleyType::Phonograph as i32);
                    }
                }
                board.grid_items.push(a_zen_tool);
            }
        }

        unsafe { (*board_ptr).clear_cursor() };
    }

    pub fn draw_plant_overlay(&self, g: &mut Graphics, plant: &Plant) {
        // 对应 C++ DrawPlantOverlay：按需求在植物上方显示气泡提示
        if plant.potted_plant_index == -1 {
            return;
        }
        let a_potted_plant = self.potted_plant_from_index(plant.potted_plant_index as usize);
        let a_plant_need = a_potted_plant.map_or(PottedPlantNeed::None, |pp| unsafe {
            self.get_plants_need(&*pp)
        });
        if a_plant_need == PottedPlantNeed::None {
            return;
        }
        // [TRANSLATION_NOTE]: C++ 中绘制 IMAGE_PLANTSPEECHBUBBLE 并按下述需求绘制
        // IMAGE_ZEN_NEED_ICONS（列 0/1/2）或 IMAGE_WATERDROP 图标；Rust 侧图片资源未接入，暂略
        match a_plant_need {
            PottedPlantNeed::Fertilizer | PottedPlantNeed::Bugspray
            | PottedPlantNeed::Phonograph | PottedPlantNeed::Water => {}
            _ => {}
        }
        let _ = g;
    }

    pub fn was_plant_need_fulfilled_today(&self, potted_plant: &PottedPlant) -> bool {
        // 对应 C++ WasPlantNeedFulfilledToday
        let a_now = self.now_time;
        if a_now - potted_plant.last_need_fulfilled_time < 3600 {
            return true;
        }
        let a_last_need_fulfilled_tm = self.app.map_or((0,0,0,0,0,0,0,0,0), |app| unsafe {
            (*app).get_local_time(potted_plant.last_need_fulfilled_time)
        });
        let m_tm = self.now_tm;
        // C++: tm_year 与 tm_yday 比较（now <= last）
        m_tm.0 <= a_last_need_fulfilled_tm.0 && m_tm.7 <= a_last_need_fulfilled_tm.7
    }

    pub fn plant_should_refresh_need(&self, potted_plant: &PottedPlant) -> bool {
        // 对应 C++ PlantShouldRefreshNeed
        let a_now = self.now_time;
        if a_now - potted_plant.last_watered_time < 3600 {
            return false;
        }
        let a_last_watered_tm = self.app.map_or((0,0,0,0,0,0,0,0,0), |app| unsafe {
            (*app).get_local_time(potted_plant.last_watered_time)
        });
        let m_tm = self.now_tm;
        m_tm.0 > a_last_watered_tm.0 || m_tm.7 > a_last_watered_tm.7
    }

    pub fn refresh_plant_needs(&self, potted_plant: &mut PottedPlant) {
        // 对应 C++ RefreshPlantNeeds
        if potted_plant.plant_age != PottedPlantAge::Full || !self.plant_should_refresh_need(potted_plant) {
            return;
        }
        if Plant::is_aquatic(potted_plant.seed_type) {
            potted_plant.last_watered_time = self.now_time;
            let min_need = PottedPlantNeed::Bugspray as i32;
            let max_need = PottedPlantNeed::Phonograph as i32;
            potted_plant.plant_need = unsafe {
                std::mem::transmute::<i32, PottedPlantNeed>(
                    crate::framework::common::rand_range(max_need - min_need + 1) + min_need
                )
            };
        } else {
            potted_plant.times_fed = 0;
            potted_plant.plant_need = PottedPlantNeed::None;
        }
    }

    pub fn update_plant_needs(&mut self) {
        // 对应 C++ UpdatePlantNeeds
        if let Some(app) = self.app {
            unsafe {
                let info = (*app).player_info.as_ref();
                if info.is_none() {
                    return;
                }
                self.now_time = (*app).get_now_time();
                self.now_tm = (*app).get_local_time(self.now_time);
            }
        }
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_mut() {
                    let mut to_refresh: Vec<usize> = Vec::new();
                    for i in 0..info.m_num_potted_plants as usize {
                        if i < info.m_potted_plant.len() {
                            to_refresh.push(i);
                        }
                    }
                    for i in to_refresh {
                        let pp = &mut info.m_potted_plant[i];
                        self.refresh_plant_needs(pp);
                    }
                }
            }
        }
    }

    pub fn was_plant_fertilized_in_last_hour(&self, potted_plant: &PottedPlant) -> bool {
        // 对应 C++ WasPlantFertilizedInLastHour
        self.now_time - potted_plant.last_fertilized_time < 3600
    }

    pub fn plants_need_water(&self) -> bool {
        // 对应 C++ PlantsNeedWater
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_ref() {
                    for i in 0..info.m_num_potted_plants as usize {
                        if i < info.m_potted_plant.len() {
                            let pp = &info.m_potted_plant[i];
                            if self.get_plants_need(pp) == PottedPlantNeed::Water {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn plant_can_be_watered(&self, plant: &Plant) -> bool {
        // 对应 C++ PlantCanBeWatered
        if plant.potted_plant_index == -1 {
            return false;
        }
        let a_potted = self.potted_plant_from_index(plant.potted_plant_index as usize);
        match a_potted {
            Some(pp) => {
                let pp_ref = unsafe { &*pp };
                self.get_plants_need(pp_ref) == PottedPlantNeed::Water
            }
            None => false,
        }
    }

    pub fn count_plants_needing_fertilizer(&self) -> i32 {
        // 对应 C++ CountPlantsNeedingFertilizer
        let mut a_count = 0;
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_ref() {
                    for i in 0..info.m_num_potted_plants as usize {
                        if i < info.m_potted_plant.len() {
                            let pp = &info.m_potted_plant[i];
                            if self.get_plants_need(pp) == PottedPlantNeed::Fertilizer {
                                a_count += 1;
                            }
                        }
                    }
                }
            }
        }
        a_count
    }

    pub fn all_plants_have_been_fertilized(&self) -> bool {
        // 对应 C++ AllPlantsHaveBeenFertilized
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_ref() {
                    for i in 0..info.m_num_potted_plants as usize {
                        if i < info.m_potted_plant.len() {
                            if info.m_potted_plant[i].plant_age == PottedPlantAge::Sprout {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }

    pub fn plant_high_on_chocolate(&self, potted_plant: &PottedPlant) -> bool {
        // 对应 C++ PlantHighOnChocolate
        self.now_time - potted_plant.last_chocolate_time < 300
    }

    pub fn plant_can_have_chocolate(&self, plant: &Plant) -> bool {
        // 对应 C++ PlantCanHaveChocolate
        if plant.potted_plant_index == -1 {
            return false;
        }
        let a_potted = self.potted_plant_from_index(plant.potted_plant_index as usize);
        match a_potted {
            Some(pp) => {
                let pp_ref = unsafe { &*pp };
                pp_ref.plant_age == PottedPlantAge::Full
                    && self.was_plant_need_fulfilled_today(pp_ref)
                    && !self.plant_high_on_chocolate(pp_ref)
            }
            None => false,
        }
    }

    pub fn has_purchased_stinky(&self) -> bool {
        // 对应 C++ HasPurchasedStinky
        self.app.map_or(false, |app| unsafe {
            (*app).player_info.as_ref().map_or(false, |info| {
                let idx = StoreItem::StinkyTheSnail as usize;
                info.m_purchases.get(idx).copied().unwrap_or(0) != 0
            })
        })
    }

    pub fn get_plant_sell_price(&self, plant: &Plant) -> i32 {
        // 对应 C++ GetPlantSellPrice
        let a_potted = self.potted_plant_from_index(plant.potted_plant_index as usize);
        if let Some(pp) = a_potted {
            let pp_ref = unsafe { &*pp };
            if pp_ref.seed_type == SeedType::Marigold {
                return match pp_ref.plant_age {
                    PottedPlantAge::Sprout => 150,
                    PottedPlantAge::Small => 200,
                    PottedPlantAge::Medium => 250,
                    PottedPlantAge::Full => 300,
                };
            }
            match pp_ref.plant_age {
                PottedPlantAge::Sprout => 150,
                PottedPlantAge::Small => 300,
                PottedPlantAge::Medium => 500,
                PottedPlantAge::Full => {
                    if Plant::is_nocturnal(pp_ref.seed_type) || Plant::is_aquatic(pp_ref.seed_type) {
                        1000
                    } else {
                        800
                    }
                }
            }
        } else {
            0
        }
    }

    pub fn plant_potted_draw_height_offset(&self, seed_type: SeedType, scale: f32) -> f32 {
        // 对应 C++ PlantPottedDrawHeightOffset
        let mut a_scale_offset_fix = 0.0f32;
        let mut a_height_offset = 0.0f32;
        match seed_type {
            SeedType::Gravebuster => {
                a_height_offset += 50.0;
                a_scale_offset_fix += 15.0;
            }
            SeedType::Puffshroom => {
                a_height_offset += 10.0;
                a_scale_offset_fix += 24.0;
            }
            SeedType::Sunshroom => {
                a_height_offset += 10.0;
                a_scale_offset_fix += 17.0;
            }
            SeedType::Scaredyshroom => {
                a_height_offset += 5.0;
                a_scale_offset_fix += 5.0;
            }
            SeedType::Tanglekelp => {
                a_height_offset -= 18.0;
                a_scale_offset_fix += 20.0;
            }
            SeedType::Seashroom => {
                a_height_offset -= 20.0;
                a_scale_offset_fix += 15.0;
            }
            SeedType::Lilypad => {
                a_height_offset -= 10.0;
                a_scale_offset_fix += 30.0;
            }
            SeedType::Hypnoshroom | SeedType::Marigold | SeedType::Peashooter
            | SeedType::Repeater | SeedType::Leftpeater | SeedType::Snowpea
            | SeedType::Threepeater | SeedType::Sunflower => {
                a_scale_offset_fix += 10.0;
            }
            SeedType::Starfruit => {
                a_height_offset += 10.0;
                a_scale_offset_fix += 24.0;
            }
            SeedType::Cabbagepult | SeedType::Melonpult => {
                a_scale_offset_fix += 10.0;
                a_height_offset += 3.0;
            }
            SeedType::PotatoMine => {
                a_scale_offset_fix += 5.0;
            }
            SeedType::Torchwood => {
                a_scale_offset_fix += 3.0;
            }
            SeedType::Spikeweed => {
                a_scale_offset_fix += 10.0;
                a_height_offset -= 13.0;
            }
            SeedType::Blover => {
                a_scale_offset_fix += 10.0;
            }
            SeedType::Pumpkinshell => {
                a_scale_offset_fix += 20.0;
            }
            SeedType::Plantern => {
                a_scale_offset_fix -= 1.0;
            }
            _ => {}
        }
        a_height_offset + (a_scale_offset_fix * scale - a_scale_offset_fix)
    }

    pub fn zen_plant_offset_x(potted_plant: &PottedPlant) -> f32 {
        // 对应 C++ ZenPlantOffsetX
        let mut a_offset_x = 0;
        if potted_plant.facing == crate::lawn::system::player_info::FacingDirection::Left
            && potted_plant.seed_type == SeedType::PotatoMine
        {
            a_offset_x -= 6;
        }
        a_offset_x as f32
    }

    pub fn plant_fertilized(&self, plant: &mut Plant) {
        // 对应 C++ PlantFertilized
        let a_potted_index = plant.potted_plant_index;
        let a_x = plant.base.x;
        let a_y = plant.base.y;
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_mut() {
                    if a_potted_index >= 0 && (a_potted_index as usize) < info.m_potted_plant.len() {
                        let pp = &mut info.m_potted_plant[a_potted_index as usize];
                        pp.last_fertilized_time = self.now_time;
                        pp.plant_age = unsafe { std::mem::transmute::<i32, PottedPlantAge>(pp.plant_age as i32 + 1) };
                        pp.plant_need = PottedPlantNeed::None;
                        pp.times_fed = 0;
                    }
                }
                (*app).play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
            }
        }
        if let Some(board) = self.board {
            let b = unsafe { &mut *board };
            if let Some(pp) = self.potted_plant_from_index(a_potted_index as usize) {
                let pp_ref = unsafe { &*pp };
                match pp_ref.plant_age {
                    PottedPlantAge::Small => {
                        b.add_coin((a_x + 40) as f32, a_y as f32, CoinType::Gold, CoinMotion::Coin);
                    }
                    PottedPlantAge::Medium => {
                        b.add_coin((a_x + 30) as f32, a_y as f32, CoinType::Gold, CoinMotion::Coin);
                        b.add_coin((a_x + 50) as f32, a_y as f32, CoinType::Gold, CoinMotion::Coin);
                    }
                    PottedPlantAge::Full => {
                        if pp_ref.seed_type == SeedType::Marigold {
                            b.add_coin((a_x + 40) as f32, a_y as f32, CoinType::Diamond, CoinMotion::Coin);
                        } else {
                            b.add_coin((a_x + 10) as f32, a_y as f32, CoinType::Diamond, CoinMotion::Coin);
                            b.add_coin((a_x + 70) as f32, a_y as f32, CoinType::Diamond, CoinMotion::Coin);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn potted_plant_update(&mut self, plant: &mut Plant) {
        // 对应 C++ PottedPlantUpdate：检查时间戳倒流并重置计时器
        let a_potted_plant = self.potted_plant_from_index(plant.potted_plant_index as usize);
        let a_now = self.now_time;
        let need_reset = a_potted_plant.map_or(false, |pp| unsafe {
            (*pp).last_watered_time > a_now
                || (*pp).last_need_fulfilled_time > a_now
                || (*pp).last_fertilized_time > a_now
                || (*pp).last_chocolate_time > a_now
        });
        if need_reset {
            if let Some(pp) = a_potted_plant {
                self.reset_plant_timers(unsafe { &mut *pp });
            }
        }

        if plant.is_asleep {
            return;
        }
        if plant.state_countdown > 0 {
            plant.state_countdown -= 1;
        }

        let is_full_and_fulfilled = a_potted_plant.map_or(false, |pp| unsafe {
            (*pp).plant_age == PottedPlantAge::Full && self.was_plant_need_fulfilled_today(&*pp)
        });
        if is_full_and_fulfilled {
            self.plant_update_production(plant);
        }
        self.update_plant_effect_state(plant);
    }

    pub fn add_happy_effect(&self, plant: &mut Plant) {
        // 对应 C++ AddHappyEffect：在植物或花盆上附加开心发光粒子
        let a_flower_pot = self.board.and_then(|b| unsafe {
            (*b).get_top_plant_at(plant.plant_col, plant.start_row)
        });

        // 通过数组地址匹配获得花盆可变引用（避免 &T -> &mut T 的 UB 转换）
        let mut a_flower_pot_ptr: *mut Plant = std::ptr::null_mut();
        if let (Some(board), Some(fp)) = (self.board, a_flower_pot) {
            unsafe {
                let b = &mut *board;
                for i in 0..b.plants.len() {
                    if std::ptr::eq(&b.plants[i] as *const Plant, fp as *const Plant) {
                        a_flower_pot_ptr = &mut b.plants[i] as *mut Plant;
                        break;
                    }
                }
            }
        }

        if a_flower_pot_ptr.is_null() {
            plant.add_attached_particle(
                plant.pos_x as i32 + 40,
                plant.pos_y as i32 + 60,
                plant.base.render_order - 1,
                ParticleEffect::PottedZenGlow,
            );
        } else {
            unsafe {
                let pot = &mut *a_flower_pot_ptr;
                if Plant::is_aquatic(plant.seed_type) {
                    pot.add_attached_particle(
                        pot.pos_x as i32 + 40,
                        pot.pos_y as i32 + 61,
                        pot.base.render_order - 1,
                        ParticleEffect::PottedWaterPlantGlow,
                    );
                } else {
                    pot.add_attached_particle(
                        pot.pos_x as i32 + 40,
                        pot.pos_y as i32 + 63,
                        pot.base.render_order - 1,
                        ParticleEffect::PottedZenGlow,
                    );
                }
            }
        }
    }

    pub fn remove_happy_effect(&self, plant: &mut Plant) {
        // 对应 C++ RemoveHappyEffect：销毁花盆或植物上的特效粒子
        let a_flower_pot = self.board.and_then(|b| unsafe {
            (*b).get_top_plant_at(plant.plant_col, plant.start_row)
        });
        let particle_id = if let Some(fp) = a_flower_pot {
            fp.particle_id
        } else {
            plant.particle_id
        };
        if particle_id == PARTICLESYSTEMID_NULL {
            return;
        }
        if let Some(app) = self.app {
            unsafe {
                if let Some(es) = (*app).effect_system.as_mut() {
                    if let Some(ps) = es.particle_systems.get_mut(particle_id as usize) {
                        ps.particle_system_die();
                    }
                }
            }
        }
    }

    pub fn plant_update_production(&self, plant: &mut Plant) {
        // 对应 C++ PlantUpdateProduction：植物生产倒计时与产币
        plant.launch_counter -= 1;
        self.set_plant_anim_speed(plant);
        let a_potted_plant = self.potted_plant_from_index(plant.potted_plant_index as usize);
        let a_high_on_chocolate = a_potted_plant.map_or(false, |pp| unsafe {
            self.plant_high_on_chocolate(&*pp)
        });
        if a_high_on_chocolate {
            plant.launch_counter -= 1;
        }

        if plant.launch_counter <= 0 {
            self.plant_set_launch_counter(plant);
            if let Some(app) = self.app {
                unsafe { (*app).play_foley(FoleyType::SpawnSun as i32); }
            }

            let mut a_coin_hit = crate::framework::common::rand_range(1000);
            a_coin_hit += crate::todlib::tod_common::tod_animate_curve(
                5, 30, self.plant_get_minutes_since_happy(plant), 0, 80,
                crate::lawn::game_enums::TodCurves::Linear,
            );
            let mut a_coin_type = CoinType::Silver;
            if a_coin_hit < 100 {
                a_coin_type = CoinType::Gold;
            }
            if let Some(board) = self.board {
                unsafe { (*board).add_coin(plant.pos_x, plant.pos_y, a_coin_type, CoinMotion::Coin); }
            }
        }
    }

    pub fn can_drop_potted_plant_loot(&self) -> bool {
        if let Some(app) = self.app {
            unsafe {
                (*app).has_finished_adventure() && !self.is_zen_garden_full(true)
            }
        } else {
            false
        }
    }

    pub fn show_tutorial_arrow_on_watering_can(&self) {
        // 对应 C++ ShowTutorialArrowOnWateringCan
        if let Some(board) = self.board {
            unsafe {
                let a_button_rect = (*board).get_zen_button_rect(GameObjectType::WateringCan);
                (*board).tutorial_arrow_show(a_button_rect.x + 10, a_button_rect.y + 10);
                (*board).display_advice(
                    "[ADVICE_ZEN_GARDEN_PICK_UP_WATER]",
                    MessageStyle::ZenGardenLong as i32,
                    AdviceType::None,
                );
                (*board).m_tutorial_state = TutorialState::ZenGardenPickupWater;
            }
        }
    }

    pub fn zen_garden_start(&mut self) {
        // 对应 C++ ZenGardenStart：C++ 中该函数体为空
    }

    pub fn update_plant_effect_state(&self, plant: &mut Plant) {
        // 对应 C++ UpdatePlantEffectState：按需求/睡眠更新植物状态
        let a_potted_plant = self.potted_plant_from_index(plant.potted_plant_index as usize);
        let a_original_state = plant.state;
        let a_plant_need = a_potted_plant.map_or(PottedPlantNeed::None, |pp| unsafe {
            self.get_plants_need(&*pp)
        });
        if a_plant_need == PottedPlantNeed::Water {
            plant.state = PlantState::NotReady;
        } else if a_plant_need == PottedPlantNeed::None {
            if a_potted_plant.map_or(false, |pp| unsafe {
                self.was_plant_need_fulfilled_today(&*pp)
            }) {
                plant.state = PlantState::ZenGardenHappy;
            } else if plant.is_asleep {
                plant.state = PlantState::NotReady;
            } else {
                plant.state = PlantState::ZenGardenWatered;
            }
        } else {
            plant.state = PlantState::ZenGardenNeedy;
        }
        if a_original_state == plant.state {
            return;
        }

        // [TRANSLATION_NOTE]: C++ 中检查下方花盆（GetTopPlantAt TOPPLANT_ONLY_UNDER_PLANT）
        // 并 SetImageOverride("Pot_top", IMAGE_REANIM_POT_TOP_DARK)；Rust 侧无该图片常量，暂略
        if a_original_state == PlantState::ZenGardenHappy {
            self.remove_happy_effect(plant);
        }

        if plant.state == PlantState::ZenGardenHappy {
            plant.set_sleeping(false);
            self.add_happy_effect(plant);
        } else if Plant::is_nocturnal(plant.seed_type)
            && !self.board.map_or(false, |b| unsafe { (*b).stage_is_night() })
        {
            plant.set_sleeping(true);
        }
    }

    pub fn can_use_game_object(&self, object_type: GameObjectType) -> bool {
        // TODO: 从 ZenGarden.cpp 翻译
        false
    }

    pub fn zen_tool_update(&mut self, zen_tool: &mut GridItem) {
        // 对应 C++ ZenToolUpdate：工具动画播完后执行喂养并销毁
        let a_tool_reanim = self.app.and_then(|app| unsafe {
            (*app).reanimation_get(zen_tool.grid_item_reanim_id).map(|r| r as *const Reanimation)
        });
        let Some(a_tool_reanim) = a_tool_reanim else { return };

        let mut a_play_time = 1;
        if zen_tool.grid_item_state == GridItemState::ZenToolPhonograph {
            a_play_time = 2;
        }
        if unsafe { (*a_tool_reanim).m_loop_count >= a_play_time } {
            self.do_feeding_tool(zen_tool.pos_x as i32, zen_tool.pos_y as i32, zen_tool.grid_item_state);
            zen_tool.grid_item_die();
        }
    }

    pub fn do_feeding_tool(&mut self, x: i32, y: i32, tool_type: GridItemState) {
        // 对应 C++ DoFeedingTool
        if tool_type == GridItemState::ZenToolGoldWateringCan {
            if let Some(board) = self.board {
                unsafe {
                    let b = &mut *board;
                    for i in 0..b.plants.len() {
                        if b.plants[i].dead || b.plants[i].potted_plant_index == -1 {
                            continue;
                        }
                        let in_range = b.is_plant_in_gold_watering_can_range(x, y, &b.plants[i]);
                        if !in_range {
                            continue;
                        }
                        let pidx = b.plants[i].potted_plant_index as usize;
                        if let Some(pp) = self.potted_plant_from_index(pidx) {
                            if unsafe { self.get_plants_need(&*pp) } == PottedPlantNeed::Water {
                                let plant_ptr = &mut b.plants[i] as *mut Plant;
                                self.plant_watered(unsafe { &mut *plant_ptr });
                            }
                        }
                    }
                }
            }
            return;
        }

        let a_grid_x = self.board.map_or(-1, |b| unsafe { (*b).pixel_to_grid_x(x, y) });
        let a_grid_y = self.board.map_or(-1, |b| unsafe { (*b).pixel_to_grid_y(x, y) });
        let a_plant = self.board.and_then(|b| unsafe { (*b).get_top_plant_at(a_grid_x, a_grid_y) });
        let Some(a_plant_ref) = a_plant else { return };
        if a_plant_ref.potted_plant_index == -1 {
            return;
        }

        // 通过数组地址匹配获得可变引用（避免 &T -> &mut T 的 UB 转换）
        let mut a_plant_ptr: *mut Plant = std::ptr::null_mut();
        if let Some(board) = self.board {
            unsafe {
                let b = &mut *board;
                for i in 0..b.plants.len() {
                    if std::ptr::eq(&b.plants[i] as *const Plant, a_plant_ref as *const Plant) {
                        a_plant_ptr = &mut b.plants[i] as *mut Plant;
                        break;
                    }
                }
            }
        }
        if a_plant_ptr.is_null() {
            return;
        }

        let a_potted_plant = self.potted_plant_from_index(a_plant_ref.potted_plant_index as usize);
        let a_need = a_potted_plant.map_or(PottedPlantNeed::None, |pp| unsafe {
            self.get_plants_need(&*pp)
        });

        if a_need == PottedPlantNeed::Water && tool_type == GridItemState::ZenToolWateringCan {
            self.plant_watered(unsafe { &mut *a_plant_ptr });
        } else if a_need == PottedPlantNeed::Fertilizer && tool_type == GridItemState::ZenToolFertilizer {
            self.plant_fertilized(unsafe { &mut *a_plant_ptr });
        } else if a_need == PottedPlantNeed::Bugspray && tool_type == GridItemState::ZenToolBugSpray {
            self.plant_fulfill_need(unsafe { &mut *a_plant_ptr });
        } else if a_need == PottedPlantNeed::Phonograph && tool_type == GridItemState::ZenToolPhonograph {
            self.plant_fulfill_need(unsafe { &mut *a_plant_ptr });
        }

        if let Some(board) = self.board {
            unsafe {
                if (*board).m_tutorial_state == TutorialState::ZenGardenFertilizePlants
                    && tool_type == GridItemState::ZenToolFertilizer
                {
                    if self.all_plants_have_been_fertilized() {
                        (*board).m_tutorial_state = TutorialState::ZenGardenCompleted;
                        (*board).display_advice(
                            "[ADVICE_ZEN_GARDEN_CONTINUE_ADVENTURE]",
                            MessageStyle::HintTallFast as i32,
                            AdviceType::None,
                        );
                        // [TRANSLATION_NOTE]: C++ 中 mMenuButton->mDisabled=false / mBtnNoDraw=false
                    } else if let Some(app) = self.app {
                        unsafe {
                            if let Some(info) = (*app).player_info.as_mut() {
                                let fidx = StoreItem::Fertilizer as usize;
                                if info.m_purchases.get(fidx).copied().unwrap_or(0) == 1000 {
                                    // PURCHASE_COUNT_OFFSET == 1000
                                    if info.m_purchases.len() > fidx {
                                        info.m_purchases[fidx] = 1000 + 5;
                                    }
                                    (*board).display_advice(
                                        "[ADVICE_ZEN_GARDEN_NEED_MORE_FERTILIZER]",
                                        MessageStyle::HintTallFast as i32,
                                        AdviceType::None,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn add_stinky(&mut self) {
        // 对应 C++ AddStinky
        if !self.has_purchased_stinky() || self.garden_type != GardenType::Main {
            return;
        }
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_mut() {
                    if info.m_has_seen_stinky == 0 {
                        info.m_has_seen_stinky = 1;
                        let mut a_time = self.now_time as u32;
                        if a_time == 0 {
                            a_time = 1;
                        }
                        let idx = StoreItem::StinkyTheSnail as usize;
                        if info.m_purchases.len() > idx {
                            info.m_purchases[idx] = a_time as i32;
                        }
                    }
                }
            }
        }
        // [TRANSLATION_NOTE]: 创建 GridItem(REANIM_STINKY) + StinkyPickGoal/ShouldStinkyBeAwake 分支依赖 Board 网格物品与动画系统，暂不执行
    }

    pub fn stinky_update(&mut self, stinky: &mut GridItem) {
        // 对应 C++ StinkyUpdate 核心状态机
        let a_stinky_high_on_chocolate = self.is_stinky_high_on_chocolate();
        self.update_stinky_motion_trail(stinky, a_stinky_high_on_chocolate);

        let mut a_loop_count = 0i32;
        if let Some(app) = self.app {
            unsafe {
                if let Some(reanim) = (*app).reanimation_get(stinky.grid_item_reanim_id) {
                    a_loop_count = reanim.m_loop_count;
                }
            }
        }

        if stinky.grid_item_state == GridItemState::StinkyFallingAsleep {
            if a_loop_count > 0 {
                self.stinky_finish_falling_asleep(stinky, 20);
            }
            return;
        }

        if stinky.grid_item_state == GridItemState::StinkySleeping {
            if self.should_stinky_be_awake() {
                self.stinky_wake_up(stinky);
            }
            return;
        }

        if stinky.grid_item_state == GridItemState::StinkyWakingUp {
            if a_loop_count > 0 {
                stinky.grid_item_state = GridItemState::StinkyWalkingLeft;
                if let Some(app) = self.app {
                    unsafe {
                        if let Some(reanim) = (*app).reanimation_get_mut(stinky.grid_item_reanim_id) {
                            reanim.play_reanim("anim_crawl", crate::todlib::reanimator::ReanimLoopType::Loop, 10, 6.0);
                        }
                    }
                }
                self.stinky_pick_goal(stinky);
            }
            return;
        }

        if !self.should_stinky_be_awake() {
            if stinky.pos_y >= STINKY_SLEEP_POS_Y {
                if stinky.grid_item_state == GridItemState::StinkyWalkingLeft {
                    self.stinky_start_falling_asleep(stinky);
                    return;
                } else if stinky.grid_item_state == GridItemState::StinkyWalkingRight {
                    stinky.grid_item_state = GridItemState::StinkyTurningLeft;
                    stinky.motion_trail_count = 0;
                    stinky.goal_x = stinky.pos_x;
                    stinky.goal_y = stinky.pos_y;
                    return;
                }
            }
        }

        // C++: 靠近硬币自动收集
        if let Some(board_ptr) = self.board {
            let board = unsafe { &mut *board_ptr };
            for coin in board.coins.iter_mut() {
                if coin.dead { continue; }
                if !coin.is_being_collected
                    && crate::todlib::tod_common::distance(
                        coin.pos_x, coin.pos_y + 30.0,
                        stinky.pos_x, stinky.pos_y,
                    ) < 20.0
                {
                    coin.collect();
                }
            }
        }

        if stinky.grid_item_state == GridItemState::StinkyWalkingLeft
            || stinky.grid_item_state == GridItemState::StinkyWalkingRight
        {
            if stinky.counter > 0 {
                stinky.counter -= 1;
            }
            // 简化：走到目标后重新选目标
            let a_delta_x = (stinky.pos_x - stinky.goal_x).abs();
            let a_delta_y = (stinky.pos_y - stinky.goal_y).abs();
            if (a_delta_x < 5.0 && a_delta_y < 5.0) || stinky.counter == 0 {
                self.stinky_pick_goal(stinky);
            }
        }

        if stinky.grid_item_state == GridItemState::StinkyTurningLeft
            || stinky.grid_item_state == GridItemState::StinkyTurningRight
        {
            if a_loop_count > 0 {
                let new_state = if stinky.grid_item_state == GridItemState::StinkyTurningLeft {
                    GridItemState::StinkyWalkingLeft
                } else {
                    GridItemState::StinkyWalkingRight
                };
                stinky.grid_item_state = new_state;
                if let Some(app) = self.app {
                    unsafe {
                        if let Some(reanim) = (*app).reanimation_get_mut(stinky.grid_item_reanim_id) {
                            reanim.play_reanim("anim_crawl", crate::todlib::reanimator::ReanimLoopType::Loop, 10, 6.0);
                        }
                    }
                }
            }
        }

        self.stinky_anim_rate_update(stinky);
    }

    pub fn open_store(&mut self) {
        // 对应 C++ OpenStore：离开花园打开商店（教程时赠送肥料）
        self.leave_garden();
        let a_store = crate::lawn::lawn_app::LawnApp::show_store_screen(self.app);

        if let Some(board) = self.board {
            unsafe {
                if (*board).m_tutorial_state == TutorialState::ZenGardenVisitStore {
                    if let Some(s) = a_store {
                        unsafe { (*(s as *mut crate::lawn::widget::store_screen::StoreScreen)).setup_for_intro(2600); }
                    }
                    if let Some(app) = self.app {
                        unsafe {
                            if let Some(info) = (*app).player_info.as_mut() {
                                let fidx = StoreItem::Fertilizer as usize;
                                if info.m_purchases.len() > fidx {
                                    info.m_purchases[fidx] = 1000 + 5; // PURCHASE_COUNT_OFFSET + 5
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(s) = a_store {
            unsafe {
                let store = &mut *(s as *mut crate::lawn::widget::store_screen::StoreScreen);
                // [TRANSLATION_NOTE]: C++ 中 mBackButton->SetLabel("[STORE_BACK_TO_GAME]")
                store.page = StorePages::Zen1;
                // C++ 中 WaitForResult(true) 模态等待；Rust 侧以 wait_for_dialog 字段近似
                store.wait_for_dialog = true;
            }
        }

        let a_go_to_tree_now = a_store.map_or(false, |s| unsafe {
            (*(s as *mut crate::lawn::widget::store_screen::StoreScreen)).go_to_tree_now
        });
        if a_go_to_tree_now {
            if let Some(app) = self.app {
                unsafe {
                    (*app).kill_board();
                    (*app).pre_new_game(GameMode::ChallengeTreeOfWisdom, false);
                }
            }
        } else {
            if let Some(app) = self.app {
                unsafe {
                    self.now_time = (*app).get_now_time();
                    self.now_tm = (*app).get_local_time(self.now_time);
                    if let Some(music) = (*app).music.as_mut() {
                        music.make_sure_music_is_playing(MusicTune::ZenGarden);
                    }
                    if let Some(board) = self.board {
                        if (*board).m_tutorial_state == TutorialState::ZenGardenVisitStore {
                            (*board).display_advice(
                                "[ADVICE_ZEN_GARDEN_FERTILIZE]",
                                MessageStyle::ZenGardenLong as i32,
                                AdviceType::None,
                            );
                            (*board).m_tutorial_state = TutorialState::ZenGardenFertilizePlants;
                        }
                    }
                }
            }
            self.add_stinky();
        }
    }

    pub fn get_stinky(&self) -> Option<*mut GridItem> {
        // 对应 C++ GetStinky：遍历网格物品找臭鼬
        if let Some(board_ptr) = self.board {
            let board = unsafe { &mut *board_ptr };
            for item in board.grid_items.iter_mut() {
                if !item.dead && item.grid_item_type == crate::lawn::grid_item::GridItemType::PlantStinky {
                    return Some(item as *mut GridItem);
                }
            }
        }
        None
    }

    pub fn stinky_pick_goal(&mut self, stinky: &mut GridItem) {
        // 对应 C++ StinkyPickGoal
        let a_cur_dist_to_goal = crate::todlib::tod_common::distance(
            stinky.goal_x, stinky.goal_y, stinky.pos_x, stinky.pos_y,
        );

        // C++: 找最近的未收集金币
        let mut a_best_coin: Option<usize> = None;
        let mut a_cur_weight = 0.0f32;
        let mut a_goal_set = false;
        if let Some(board_ptr) = self.board {
            let board = unsafe { &*board_ptr };
            for (i, coin) in board.coins.iter().enumerate() {
                if coin.dead { continue; }
                if !coin.is_being_collected && coin.pos_y == coin.ground_y {
                    let mut a_weight = crate::todlib::tod_common::distance(
                        coin.pos_x, coin.pos_y + 30.0, stinky.pos_x, stinky.pos_y,
                    );
                    if coin.coin_type == CoinType::Gold {
                        a_weight -= 40.0;
                    } else if coin.coin_type == CoinType::Diamond {
                        a_weight -= 80.0;
                    }
                    let a_dist_from_last_goal = crate::todlib::tod_common::distance(
                        coin.pos_x, coin.pos_y + 30.0, stinky.goal_x, stinky.goal_y,
                    );
                    if a_dist_from_last_goal < 5.0 {
                        a_weight -= 20.0;
                    }
                    if a_best_coin.is_none() || a_weight < a_cur_weight {
                        a_best_coin = Some(i);
                        a_cur_weight = a_weight;
                    }
                }
            }
        }

        if let Some(i) = a_best_coin {
            if let Some(board_ptr) = self.board {
                let board = unsafe { &*board_ptr };
                if let Some(coin) = board.coins.get(i) {
                    stinky.goal_x = coin.pos_x;
                    stinky.goal_y = coin.pos_y + 30.0;
                    a_goal_set = true;
                }
            }
        } else if a_cur_dist_to_goal <= 10.0 {
            // C++: 无硬币时随机选一个网格目标（简化：选最近网格中心）
            let mut count = 0;
            let mut target = crate::lawn::zen_garden::SpecialGridPlacement { pixel_x: 0, pixel_y: 0, grid_x: 0, grid_y: 0 };
            if let Some(placements) = self.get_special_grid_placements(&mut count) {
                let placements = unsafe { std::slice::from_raw_parts(placements, count as usize) };
                if !placements.is_empty() {
                    let idx = crate::framework::common::rand_range(placements.len() as i32) as usize;
                    target = crate::lawn::zen_garden::SpecialGridPlacement {
                        pixel_x: placements[idx].pixel_x,
                        pixel_y: placements[idx].pixel_y,
                        grid_x: placements[idx].grid_x,
                        grid_y: placements[idx].grid_y,
                    };
                }
            }
            stinky.goal_x = (target.pixel_x + 15) as f32;
            stinky.goal_y = (target.pixel_y + 80) as f32;
            a_goal_set = true;
        }

        if !a_goal_set {
            return;
        }

        stinky.counter = 100;
        // C++: 目标方向与当前行走方向不符时转身
        let turn_to = if stinky.goal_x < stinky.pos_x && stinky.grid_item_state == GridItemState::StinkyWalkingRight {
            Some(GridItemState::StinkyTurningLeft)
        } else if stinky.goal_x > stinky.pos_x && stinky.grid_item_state == GridItemState::StinkyWalkingLeft {
            Some(GridItemState::StinkyTurningRight)
        } else {
            None
        };
        if let Some(new_state) = turn_to {
            stinky.grid_item_state = new_state;
            stinky.motion_trail_count = 0;
            if let Some(app) = self.app {
                unsafe {
                    if let Some(reanim) = (*app).reanimation_get_mut(stinky.grid_item_reanim_id) {
                        reanim.play_reanim("turn", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 10, 6.0);
                    }
                }
            }
        }
    }

    pub fn setup_for_zen_tutorial(&mut self) {
        // 对应 C++ SetupForZenTutorial
        if let Some(board) = self.board {
            unsafe {
                // [TRANSLATION_NOTE]: C++ 中 mMenuButton/mStoreButton 的 SetLabel/mDisabled/mBtnNoDraw；
                // Rust 侧按钮为 Option<i32>，暂略
                let _ = board;
            }
        }
        if let Some(app) = self.app {
            unsafe {
                (*app).crazy_dave_enter();
                (*app).crazy_dave_talk_index(2100);
            }
        }
    }

    pub fn wake_stinky(&self) {
        // 对应 C++ WakeStinky
        let mut a_time = self.now_time as u32;
        if a_time == 0 {
            a_time = 1;
        }
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_mut() {
                    let idx = StoreItem::StinkyTheSnail as usize;
                    if info.m_purchases.len() > idx {
                        info.m_purchases[idx] = a_time as i32;
                    }
                    info.m_has_woken_stinky = 1;
                }
            }
        }
        // [TRANSLATION_NOTE]: PlaySample(SOUND_TAP) 与 ClearAdvice(ADVICE_STINKY_SLEEPING) 依赖音效/提示系统，暂不执行
    }

    pub fn should_stinky_be_awake(&self) -> bool {
        // 对应 C++ ShouldStinkyBeAwake
        if self.is_stinky_high_on_chocolate() {
            return true;
        }
        // C++ 无符号算术处理回绕；Rust 侧以 i64 直接相减并转 u32 近似
        let a_now = self.now_time as u32;
        let a_purchase = self.app.map_or(0u32, |app| unsafe {
            (*app).player_info.as_ref().map_or(0u32, |info| {
                let idx = StoreItem::StinkyTheSnail as usize;
                info.m_purchases.get(idx).copied().unwrap_or(0) as u32
            })
        });
        a_now.wrapping_sub(a_purchase) < 180
    }

    pub fn is_stinky_sleeping(&self) -> bool {
        // 对应 C++ IsStinkySleeping
        if let Some(stinky) = self.get_stinky() {
            unsafe {
                (*stinky).grid_item_state == GridItemState::StinkySleeping
            }
        } else {
            false
        }
    }

    pub fn pick_random_seed_type() -> SeedType {
        // TODO: 从 ZenGarden.cpp 翻译
        SeedType::Peashooter
    }

    pub fn stinky_wake_up(&self, stinky: &mut GridItem) {
        // 对应 C++ StinkyWakeUp
        if let Some(app) = self.app {
            unsafe {
                if let Some(reanim) = (*app).reanimation_get_mut(stinky.grid_item_reanim_id) {
                    reanim.play_reanim("anim_out", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 20, 6.0);
                }
            }
        }
        stinky.grid_item_state = GridItemState::StinkyWakingUp;
        // [TRANSLATION_NOTE]: FindReanimAttachment(shell) + ReanimationDie 依赖附着系统，暂不执行
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_mut() {
                    info.m_has_woken_stinky = 1;
                }
            }
        }
    }

    pub fn stinky_start_falling_asleep(&self, stinky: &mut GridItem) {
        // 对应 C++ StinkyStartFallingAsleep
        if let Some(app) = self.app {
            unsafe {
                if let Some(reanim) = (*app).reanimation_get_mut(stinky.grid_item_reanim_id) {
                    reanim.play_reanim("anim_in", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 20, 6.0);
                }
            }
        }
        stinky.grid_item_state = GridItemState::StinkyFallingAsleep;
    }

    pub fn stinky_finish_falling_asleep(&self, stinky: &mut GridItem, blend_time: i32) {
        // 对应 C++ StinkyFinishFallingAsleep
        if let Some(app) = self.app {
            unsafe {
                if let Some(reanim) = (*app).reanimation_get_mut(stinky.grid_item_reanim_id) {
                    reanim.play_reanim("anim_out", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, blend_time, 0.0);
                    reanim.m_anim_rate = 0.0;
                }
                // [TRANSLATION_NOTE]: AddReanimation(REANIM_SLEEPING) + AttachReanim 依赖附着系统，暂不执行
            }
        }
        stinky.grid_item_state = GridItemState::StinkySleeping;
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_ref() {
                    if info.m_has_woken_stinky == 0 {
                        // [TRANSLATION_NOTE]: DisplayAdvice("[ADVICE_STINKY_SLEEPING]") 依赖提示系统，暂不执行
                    }
                }
            }
        }
    }

    pub fn advance_crazy_dave_dialog(&mut self) {
        // 对应 C++ AdvanceCrazyDaveDialog
        let Some(app) = self.app else { return };
        unsafe {
            if (*app).m_crazy_dave_message_index == -1
                || (*app).base.dialog_map.contains_key(&(Dialogs::Store as i32))
                || (*app).base.dialog_map.contains_key(&(Dialogs::ZenSell as i32))
            {
                return;
            }

            if (*app).m_crazy_dave_message_index == 2104 {
                self.show_tutorial_arrow_on_watering_can();
            }

            if !(*app).advance_crazy_dave_text() {
                (*app).crazy_dave_leave();
                return;
            }

            if (*app).m_crazy_dave_message_index == 2102 {
                let num_potted = (*app).player_info.as_ref().map_or(0, |info| info.m_num_potted_plants);
                if num_potted == 0 {
                    for _ in 0..2 {
                        let mut a_potted_plant = PottedPlant::new();
                        a_potted_plant.initialize_potted_plant(SeedType::Marigold);
                        a_potted_plant.draw_variation = unsafe {
                            std::mem::transmute::<i32, DrawVariation>(
                                crate::todlib::tod_common::rand_range_int(
                                    DrawVariation::MarigoldWhite as i32,
                                    DrawVariation::MarigoldLightGreen as i32,
                                ),
                            )
                        };
                        a_potted_plant.feedings_per_grow = 3;
                        self.add_potted_plant(&mut a_potted_plant);
                    }
                }
            }
        }
    }

    pub fn leave_garden(&mut self) {
        // 对应 C++ LeaveGarden：处理地面工具与臭鼬，收集未收集金币
        if let Some(board) = self.board {
            unsafe {
                let b = &mut *board;
                for i in 0..b.grid_items.len() {
                    if b.grid_items[i].dead {
                        continue;
                    }
                    let item_type = b.grid_items[i].grid_item_type;
                    if item_type == crate::lawn::grid_item::GridItemType::ZenTool {
                        let tool = &mut b.grid_items[i];
                        self.do_feeding_tool(tool.pos_x as i32, tool.pos_y as i32, tool.grid_item_state);
                        tool.grid_item_die();
                    } else if item_type == crate::lawn::grid_item::GridItemType::PlantStinky {
                        if let Some(app) = self.app {
                            unsafe {
                                if let Some(info) = (*app).player_info.as_mut() {
                                    info.stinky_pos_x = b.grid_items[i].pos_x as i32;
                                    info.stinky_pos_y = b.grid_items[i].pos_y as i32;
                                }
                            }
                        }
                        b.grid_items[i].grid_item_die();
                    }
                }
                for i in 0..b.coins.len() {
                    if b.coins[i].dead {
                        continue;
                    }
                    if b.coins[i].is_being_collected {
                        b.coins[i].score_coin();
                    } else {
                        b.coins[i].die();
                    }
                }
            }
        }
    }

    pub fn can_drop_chocolate(&self) -> bool {
        if let Some(app) = self.app {
            unsafe {
                self.has_purchased_stinky()
                    && (*app).player_info.as_ref().map_or(false, |p| {
                        p.m_purchases[StoreItem::Chocolate as usize] < 1000 + 10
                    })
            }
        } else {
            false
        }
    }

    pub fn feed_chocolate_to_plant(&self, plant: &mut Plant) {
        // 对应 C++ FeedChocolateToPlant：记录巧克力时间并重置生产倒计时
        if let Some(pp) = self.potted_plant_from_index(plant.potted_plant_index as usize) {
            unsafe { (*pp).last_chocolate_time = self.now_time; }
        }
        plant.launch_counter = 60;
        // [TRANSLATION_NOTE]: C++ 中 AddPvzpParticle(PARTICLE_PRESENT_PICKUP) 附加粒子；
        // Rust 侧粒子附加未接入，暂略
    }

    pub fn set_plant_anim_speed(&self, plant: &mut Plant) {
        // 对应 C++ SetPlantAnimSpeed
        let a_body_reanim = self.app.and_then(|app| unsafe {
            (*app).reanimation_get_mut(plant.body_reanim_id).map(|r| r as *mut Reanimation)
        });
        let Some(a_body_reanim) = a_body_reanim else { return };
        let a_potted_plant = self.potted_plant_from_index(plant.potted_plant_index as usize);
        let a_plant_high_on_chocolate = a_potted_plant.map_or(false, |pp| unsafe {
            self.plant_high_on_chocolate(&*pp)
        });
        let a_plant_at_high_rate = unsafe { (*a_body_reanim).m_anim_rate >= 25.0 };
        if a_plant_at_high_rate == a_plant_high_on_chocolate {
            return;
        }

        let mut a_target_rate;
        match plant.seed_type {
            SeedType::Peashooter | SeedType::Snowpea | SeedType::Repeater
            | SeedType::Leftpeater | SeedType::Gatlingpea | SeedType::Splitpea
            | SeedType::Threepeater | SeedType::Marigold => {
                a_target_rate = crate::todlib::tod_common::rand_range_float(15.0, 20.0);
            }
            SeedType::PotatoMine => {
                a_target_rate = 12.0;
            }
            _ => {
                a_target_rate = crate::todlib::tod_common::rand_range_float(10.0, 15.0);
            }
        }

        if a_plant_high_on_chocolate {
            a_target_rate *= 2.0;
            a_target_rate = a_target_rate.max(25.0);
        }

        unsafe { (*a_body_reanim).m_anim_rate = a_target_rate; }
        let body_anim_time = unsafe { (*a_body_reanim).m_anim_time };
        for head_id in [plant.head_reanim_id, plant.head_reanim_id2, plant.head_reanim_id3] {
            if let Some(app) = self.app {
                unsafe {
                    if let Some(h) = (*app).reanimation_get_mut(head_id) {
                        h.m_anim_rate = a_target_rate;
                        h.m_anim_time = body_anim_time;
                    }
                }
            }
        }
    }

    pub fn update_stinky_motion_trail(&self, stinky: &mut GridItem, stinky_high_on_chocolate: bool) {
        // 对应 C++ UpdateStinkyMotionTrail
        let a_stinky_reanim_time = self.app.and_then(|app| unsafe {
            (*app).reanimation_get(stinky.grid_item_reanim_id).map(|r| r.m_anim_time)
        }).unwrap_or(0.0);
        if !stinky_high_on_chocolate {
            stinky.motion_trail_count = 0;
            return;
        }
        if stinky.grid_item_state != GridItemState::StinkyWalkingRight
            && stinky.grid_item_state != GridItemState::StinkyWalkingLeft
        {
            stinky.motion_trail_count = 0;
            return;
        }

        if stinky.motion_trail_count == crate::lawn::grid_item::NUM_MOTION_TRAIL_FRAMES as i32 {
            stinky.motion_trail_count -= 1;
        }
        if stinky.motion_trail_count > 0 {
            // C++ memmove：右移一帧
            let count = stinky.motion_trail_count as usize;
            for i in (1..=count).rev() {
                stinky.motion_trail_frames[i] = stinky.motion_trail_frames[i - 1];
            }
        }
        let mut f = crate::lawn::grid_item::MotionTrailFrame::new();
        f.pos_x = stinky.pos_x;
        f.pos_y = stinky.pos_y;
        f.anim_time = a_stinky_reanim_time;
        stinky.motion_trail_frames[0] = f;
        stinky.motion_trail_count += 1;
    }

    pub fn reset_plant_timers(&self, potted_plant: &mut PottedPlant) {
        // 对应 C++ ResetPlantTimers
        potted_plant.last_watered_time = self.now_time;
        potted_plant.last_need_fulfilled_time = 0;
        potted_plant.last_fertilized_time = 0;
        potted_plant.last_chocolate_time = 0;
        potted_plant.plant_need = PottedPlantNeed::None;
        potted_plant.times_fed = 0;
    }

    pub fn reset_stinky_timers(&self) {
        // 对应 C++ ResetStinkyTimers
        if let Some(app) = self.app {
            unsafe {
                if let Some(info) = (*app).player_info.as_mut() {
                    let idx = StoreItem::StinkyTheSnail as usize;
                    if info.m_purchases.len() > idx {
                        info.m_purchases[idx] = 2;
                    }
                    info.m_last_stinky_chocolate_time = 0;
                }
            }
        }
    }

    pub fn plant_set_launch_counter(&self, plant: &mut Plant) {
        // 对应 C++ PlantSetLaunchCounter
        let a_time = self.plant_get_minutes_since_happy(plant);
        let a_counter_max = crate::todlib::tod_common::tod_animate_curve(
            5, 30, a_time, 3000, 15000, crate::lawn::game_enums::TodCurves::Linear,
        );
        plant.launch_counter = crate::todlib::tod_common::rand_range_int(1800, a_counter_max);
    }

    pub fn plant_get_minutes_since_happy(&self, plant: &Plant) -> i32 {
        // 对应 C++ PlantGetMinutesSinceHappy
        let a_potted_plant = self.potted_plant_from_index(plant.potted_plant_index as usize);
        let a_minutes = a_potted_plant.map_or(0, |pp| unsafe {
            ((self.now_time - (*pp).last_need_fulfilled_time) / 60) as i32
        });
        if let Some(pp) = a_potted_plant {
            if unsafe { self.plant_high_on_chocolate(&*pp) } {
                return 0;
            }
        }
        a_minutes
    }

    pub fn is_stinky_high_on_chocolate(&self) -> bool {
        // 对应 C++ IsStinkyHighOnChocolate（无符号比较近似）
        let a_now = self.now_time as u32;
        let a_last = self.app.map_or(0u32, |app| unsafe {
            (*app).player_info.as_ref().map_or(0u32, |info| info.m_last_stinky_chocolate_time)
        });
        a_now.wrapping_sub(a_last) < 3600
    }

    pub fn stinky_anim_rate_update(&self, stinky: &mut GridItem) {
        // 对应 C++ StinkyAnimRateUpdate
        let walking = stinky.grid_item_state == GridItemState::StinkyWalkingLeft
            || stinky.grid_item_state == GridItemState::StinkyWalkingRight
            || stinky.grid_item_state == GridItemState::StinkyTurningRight
            || stinky.grid_item_state == GridItemState::StinkyTurningLeft;
        if !walking {
            return;
        }
        let a_rate = if self.is_stinky_high_on_chocolate() { 12.0 } else { 6.0 };
        if let Some(app) = self.app {
            unsafe {
                if let Some(reanim) = (*app).reanimation_get_mut(stinky.grid_item_reanim_id) {
                    reanim.m_anim_rate = a_rate;
                }
            }
        }
    }
}

impl Default for ZenGarden {
    fn default() -> Self {
        ZenGarden::new()
    }
}








