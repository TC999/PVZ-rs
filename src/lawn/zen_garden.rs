// PvZ Portable Rust 翻译 — ZenGarden（禅境花园）
// 对应 C++ src/Lawn/ZenGarden.h / ZenGarden.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::lawn::game_enums::*;
use crate::lawn::board::Board;
use crate::lawn::board::HitResult;
use crate::lawn::plant::Plant;
use crate::lawn::grid_item::GridItem;
use crate::lawn::system::player_info::PottedPlant;

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
        // [TRANSLATION_NOTE]: 完整逻辑依赖 Board、PottedPlant 等系统
        // 设置当前时间、放置盆栽、添加臭鼬、播放音乐
    }

    pub fn draw_potted_plant_icon(&self, g: &mut Graphics, x: f32, y: f32, potted_plant: &PottedPlant) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn draw_potted_plant(&self, g: &mut Graphics, x: f32, y: f32, potted_plant: &PottedPlant, scale: f32, draw_pot: bool) {
        // TODO: 从 ZenGarden.cpp 翻译
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
        // TODO: 从 ZenGarden.cpp 翻译
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
        // [TRANSLATION_NOTE]: 核心循环：更新植物需求→盆栽→工具/臭鼬→教程检查
    }

    pub fn mouse_down_with_full_wheel_barrow(&mut self, x: i32, y: i32) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn mouse_down_with_empty_wheel_barrow(&mut self, plant: &mut Plant) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn goto_next_garden(&mut self) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn get_potted_plant_in_wheelbarrow(&self) -> Option<*mut PottedPlant> {
        // TODO: 从 ZenGarden.cpp 翻译
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
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn mouse_down_zen_garden(&mut self, x: i32, y: i32, click_count: i32, hit_result: &mut HitResult) -> bool {
        // TODO: 从 ZenGarden.cpp 翻译
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
        // [TRANSLATION_NOTE]: 完整实现依赖 Board 植物查找、浇水/施肥判定与音效；核心逻辑见 PlantWatered/PlantFertilized
        let _ = (x, y, cursor_type);
    }

    pub fn draw_plant_overlay(&self, g: &mut Graphics, plant: &Plant) {
        // TODO: 从 ZenGarden.cpp 翻译
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

    pub fn potted_plant_update(&mut self, _plant: &mut Plant) {
        // [TRANSLATION_NOTE]: PottedPlantUpdate — 检查时间戳、睡眠状态、倒计时、生产、效果状态
    }

    pub fn add_happy_effect(&self, plant: &mut Plant) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn remove_happy_effect(&self, plant: &mut Plant) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn plant_update_production(&self, plant: &mut Plant) {
        // TODO: 从 ZenGarden.cpp 翻译
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
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn zen_garden_start(&mut self) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn update_plant_effect_state(&self, plant: &mut Plant) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn can_use_game_object(&self, object_type: GameObjectType) -> bool {
        // TODO: 从 ZenGarden.cpp 翻译
        false
    }

    pub fn zen_tool_update(&self, _zen_tool: &mut GridItem) {
        // [TRANSLATION_NOTE]: ZenToolUpdate — 更新禅境工具状态（肥料/杀虫剂/音乐盒等）
    }

    pub fn do_feeding_tool(&mut self, x: i32, y: i32, tool_type: GridItemState) {
        // TODO: 从 ZenGarden.cpp 翻译
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

    pub fn open_store(&self) {
        // TODO: 从 ZenGarden.cpp 翻译
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

    pub fn setup_for_zen_tutorial(&self) {
        // TODO: 从 ZenGarden.cpp 翻译
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
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn leave_garden(&self) {
        // TODO: 从 ZenGarden.cpp 翻译
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
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn set_plant_anim_speed(&self, plant: &mut Plant) {
        // TODO: 从 ZenGarden.cpp 翻译
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
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn plant_get_minutes_since_happy(&self, plant: &Plant) -> i32 {
        // TODO: 从 ZenGarden.cpp 翻译
        0
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








