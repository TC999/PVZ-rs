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
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn move_plant(&mut self, plant: &mut Plant, grid_x: i32, grid_y: i32) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn mouse_down_with_money_sign(&mut self, plant: &mut Plant) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn place_potted_plant(&mut self, potted_plant_index: usize) -> Option<*mut Plant> {
        // TODO: 从 ZenGarden.cpp 翻译
        None
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
        // TODO: 从 ZenGarden.cpp 翻译
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
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn stinky_update(&mut self, _stinky: &mut GridItem) {
        // [TRANSLATION_NOTE]: StinkyUpdate — FallingAsleep/Sleeping/WakingUp/行走 状态机
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
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn setup_for_zen_tutorial(&self) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn wake_stinky(&self) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn should_stinky_be_awake(&self) -> bool {
        // TODO: 从 ZenGarden.cpp 翻译
        false
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
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn stinky_start_falling_asleep(&self, stinky: &mut GridItem) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn stinky_finish_falling_asleep(&self, stinky: &mut GridItem, blend_time: i32) {
        // TODO: 从 ZenGarden.cpp 翻译
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
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn reset_plant_timers(&self, potted_plant: &mut PottedPlant) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn reset_stinky_timers(&self) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn plant_set_launch_counter(&self, plant: &mut Plant) {
        // TODO: 从 ZenGarden.cpp 翻译
    }

    pub fn plant_get_minutes_since_happy(&self, plant: &Plant) -> i32 {
        // TODO: 从 ZenGarden.cpp 翻译
        0
    }

    pub fn is_stinky_high_on_chocolate(&self) -> bool {
        // TODO: 从 ZenGarden.cpp 翻译
        false
    }

    pub fn stinky_anim_rate_update(&self, stinky: &mut GridItem) {
        // TODO: 从 ZenGarden.cpp 翻译
    }
}

impl Default for ZenGarden {
    fn default() -> Self {
        ZenGarden::new()
    }
}








