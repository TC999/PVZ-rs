// PvZ Portable Rust 翻译 — Challenge（挑战模式）
// 对应 C++ src/Lawn/Challenge.h / Challenge.cpp

use crate::framework::graphics::graphics::Graphics;
use crate::lawn::game_enums::*;
use crate::lawn::board::HitResult;
use crate::lawn::grid_item::GridItem;
use crate::lawn::plant::Plant;
use crate::lawn::zombie::Zombie;
use crate::lawn::seed_packet::SeedPacket;

/// 消除（Beghouled）升级类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeghouledUpgrade {
    Repeater,
    Fumeshroom,
    Tallnut,
    NumBeghouledUpgrades,
}

/// 消除棋盘状态
#[derive(Debug, Clone)]
pub struct BeghouledBoardState {
    pub seed_type: [[SeedType; 6]; 9],
}

/// 僵尸允许等级
pub struct ZombieAllowedLevels {
    pub zombie_type: ZombieType,
    pub allowed_on_level: [i32; 50],
}

/// 挑战模式
pub struct Challenge {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub board: Option<*mut crate::lawn::board::Board>,
    pub beghouled_mouse_capture: i32,
    pub beghouled_mouse_down_x: i32,
    pub beghouled_mouse_down_y: i32,
    pub beghouled_eated: [[i32; 6]; 9],
    pub beghouled_purchased_upgrade: [i32; 4], // NUM_BEGHOULED_UPGRADES = 3 + 1
    pub beghouled_matches_this_move: i32,
    pub challenge_state: ChallengeState,
    pub challenge_state_counter: i32,
    pub conveyor_belt_counter: i32,
    pub challenge_score: i32,
    pub show_bowling_line: i32,
    pub last_conveyor_seed_type: SeedType,
    pub survival_stage: i32,
    pub slot_machine_roll_count: i32,
    pub reanim_challenge: ReanimationID,
    pub reanim_clouds: [ReanimationID; 6],
    pub clouds_counter: [i32; 6],
    pub challenge_grid_x: i32,
    pub challenge_grid_y: i32,
    pub scary_potter_pots: i32,
    pub rain_counter: i32,
    pub tree_of_wisdom_talk_index: i32,
}

impl Challenge {
    pub fn new() -> Self {
        Challenge {
            app: None,
            board: None,
            beghouled_mouse_capture: 0,
            beghouled_mouse_down_x: 0,
            beghouled_mouse_down_y: 0,
            beghouled_eated: [[0; 6]; 9],
            beghouled_purchased_upgrade: [0; 4],
            beghouled_matches_this_move: 0,
            challenge_state: ChallengeState::Normal,
            challenge_state_counter: 0,
            conveyor_belt_counter: 0,
            challenge_score: 0,
            show_bowling_line: 0,
            last_conveyor_seed_type: SeedType::Peashooter,
            survival_stage: 0,
            slot_machine_roll_count: 0,
            reanim_challenge: REANIMATIONID_NULL,
            reanim_clouds: [REANIMATIONID_NULL; 6],
            clouds_counter: [0; 6],
            challenge_grid_x: 0,
            challenge_grid_y: 0,
            scary_potter_pots: 0,
            rain_counter: 0,
            tree_of_wisdom_talk_index: 0,
        }
    }

    // --- 方法存根（待从 Challenge.cpp 翻译具体实现） ---

    fn get_board(&mut self) -> &mut crate::lawn::board::Board {
        unsafe { &mut *self.board.unwrap() }
    }

    fn get_app(&self) -> &crate::lawn::lawn_app::LawnApp {
        unsafe { &*self.app.unwrap() }
    }

    pub fn start_level(&mut self) {
        // [TRANSLATION_NOTE]: 完整逻辑涉及多个 Board 未翻译字段。保留可用逻辑。
        let is_stormy = self.get_app().is_stormy_night_level();
        let a_game_mode = self.get_app().game_mode;
        if is_stormy {
            self.challenge_state = ChallengeState::StormFlash1;
            self.challenge_state_counter = 400;
        }
        if a_game_mode == GameMode::ChallengeBeghouled || a_game_mode == GameMode::ChallengeBeghouledTwist {
            self.challenge_state_counter = 1500;
        }
    }

    pub fn beghouled_populate_board(&mut self) {
        let mut empty_board = BeghouledBoardState { seed_type: [[SeedType::None; 6]; 9] };
        self.load_beghouled_board_state(&mut empty_board);
        let allow_cascades = self.beghouled_board_has_match(&empty_board);
        let mut board_state = BeghouledBoardState { seed_type: [[SeedType::None; 6]; 9] };
        self.load_beghouled_board_state(&mut board_state);
        self.beghouled_fill_holes(&mut board_state, allow_cascades);
        if self.beghouled_check_for_possible_moves(&board_state) == 0 {
            self.beghouled_fill_holes(&mut board_state, allow_cascades);
        }
        self.beghouled_create_plants(&empty_board, &board_state);
    }

    pub fn load_beghouled_board_state(&mut self, state: &mut BeghouledBoardState) {
        for i in 0..9 {
            for j in 0..6 {
                state.seed_type[i][j] = SeedType::None;
            }
        }
        let board = self.get_board();
        for plant in &board.plants {
            if !plant.dead {
                state.seed_type[plant.plant_col as usize][plant.base.row as usize] = plant.seed_type;
            }
        }
    }

    pub fn beghouled_pick_seed(&mut self, grid_x: i32, grid_y: i32, board_state: &mut BeghouledBoardState, allow_matches: i32) -> SeedType {
        let base_types = [SeedType::Puffshroom, SeedType::Starfruit, SeedType::Magnetshroom, SeedType::Snowpea, SeedType::Wallnut, SeedType::Peashooter];
        let mut count = 0;
        let mut pick_array = [SeedType::None; 6];
        for i in 0..6 {
            let mut seed_type = base_types[i];
            if self.beghouled_purchased_upgrade[0] != 0 && seed_type == SeedType::Peashooter { seed_type = SeedType::Repeater; }
            if self.beghouled_purchased_upgrade[1] != 0 && seed_type == SeedType::Puffshroom { seed_type = SeedType::Fumeshroom; }
            if self.beghouled_purchased_upgrade[2] != 0 && seed_type == SeedType::Wallnut { seed_type = SeedType::Tallnut; }
            board_state.seed_type[grid_x as usize][grid_y as usize] = seed_type;
            if allow_matches != 0 || self.beghouled_board_has_match(board_state) == 0 {
                pick_array[count] = seed_type;
                count += 1;
            }
        }
        board_state.seed_type[grid_x as usize][grid_y as usize] = SeedType::None;
        if count > 0 { pick_array[0] } else { SeedType::Peashooter }
    }

    pub fn beghouled_board_has_match(&self, board_state: &BeghouledBoardState) -> i32 {
        for col in 0..8 {
            for row in 0..5 {
                if self.beghouled_horizontal_match_length(col, row, board_state) >= 3 ||
                   self.beghouled_vertical_match_length(col, row, board_state) >= 3 { return 1; }
            }
        }
        0
    }

    pub fn beghouled_vertical_match_length(&self, grid_x: i32, grid_y: i32, board_state: &BeghouledBoardState) -> i32 {
        let seed_type = self.beghouled_get_plant_at(grid_x, grid_y, board_state);
        if seed_type == SeedType::None || self.beghouled_get_plant_at(grid_x, grid_y - 1, board_state) == seed_type { return 0; }
        let mut length = 1;
        while self.beghouled_get_plant_at(grid_x, grid_y + length, board_state) == seed_type { length += 1; }
        length
    }

    pub fn beghouled_horizontal_match_length(&self, grid_x: i32, grid_y: i32, board_state: &BeghouledBoardState) -> i32 {
        let seed_type = self.beghouled_get_plant_at(grid_x, grid_y, board_state);
        if seed_type == SeedType::None || self.beghouled_get_plant_at(grid_x - 1, grid_y, board_state) == seed_type { return 0; }
        let mut length = 1;
        while self.beghouled_get_plant_at(grid_x + length, grid_y, board_state) == seed_type { length += 1; }
        length
    }

    pub fn beghouled_get_plant_at(&self, grid_x: i32, grid_y: i32, board_state: &BeghouledBoardState) -> SeedType {
        if grid_x >= 0 && grid_x < 9 && grid_y >= 0 && grid_y < 6 {
            board_state.seed_type[grid_x as usize][grid_y as usize]
        } else {
            SeedType::None
        }
    }

    pub fn beghouled_drag_start(&mut self, x: i32, y: i32) {
        self.beghouled_mouse_down_x = x;
        self.beghouled_mouse_down_y = y;
        self.beghouled_mouse_capture = 1;
    }

    pub fn beghouled_drag_update(&mut self, x: i32, y: i32) {
        let delta_x = x - self.beghouled_mouse_down_x;
        let delta_y = y - self.beghouled_mouse_down_y;
        if delta_x.abs() >= 10 || delta_y.abs() >= 10 {
            let mouse_down_x = self.beghouled_mouse_down_x;
            let mouse_down_y = self.beghouled_mouse_down_y;
            {
                let board = self.get_board();
                board.clear_advice(AdviceType::None);
            }
            self.beghouled_mouse_capture = 0;
            let mut board_state = BeghouledBoardState { seed_type: [[SeedType::None; 6]; 9] };
            self.load_beghouled_board_state(&mut board_state);
            let (grid_x_from, grid_y_from) = {
                let board = self.get_board();
                (board.pixel_to_grid_x(mouse_down_x, mouse_down_y), board.pixel_to_grid_y(mouse_down_x, mouse_down_y))
            };
            let (grid_x_to, grid_y_to) = if delta_x.abs() > delta_y.abs() {
                (grid_x_from + if delta_x > 0 { 1 } else { -1 }, grid_y_from)
            } else {
                (grid_x_from, grid_y_from + if delta_y > 0 { 1 } else { -1 })
            };
            if self.beghouled_is_valid_move(grid_x_from, grid_y_from, grid_x_to, grid_y_to, &board_state) == 0 {
                // 无效移动：不做交换
            } else {
                self.beghouled_start_falling(ChallengeState::BeghouledMoving);
            }
        }
    }

    pub fn beghouled_drag_cancel(&mut self) {
        self.beghouled_mouse_capture = 0;
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) -> i32 {
        let app = self.get_app();
        if app.game_mode == GameMode::ChallengeBeghouled {
            if self.beghouled_mouse_capture != 0 {
                self.beghouled_drag_update(x, y);
                return 1;
            }
        }
        if app.game_mode == GameMode::ChallengeZenGarden {
            self.challenge_state_counter = 3000;
        }
        0
    }

    pub fn mouse_down(&mut self, x: i32, y: i32, click_count: i32, hit_result: &mut HitResult) -> i32 {
        let game_mode = self.get_app().game_mode;
        if game_mode == GameMode::ChallengeBeghouled {
            if self.challenge_state != ChallengeState::Normal { return 0; }
            if hit_result.object_type == GameObjectType::Plant {
                self.beghouled_drag_start(x, y);
                return 1;
            }
        }
        if game_mode == GameMode::ChallengeBeghouledTwist {
            if self.challenge_state != ChallengeState::Normal { return 0; }
            self.beghouled_twist_mouse_down(x, y);
        }
        if game_mode == GameMode::ChallengeZombiquarium {
            if click_count > 0 { self.zombiquarium_mouse_down(x, y); return 0; }
            return 1;
        }
        0
    }

    pub fn mouse_up(&mut self, _x: i32, _y: i32) -> i32 {
        let game_mode = self.get_app().game_mode;
        if game_mode == GameMode::ChallengeBeghouled {
            self.beghouled_drag_cancel();
        }
        0
    }

    pub fn clear_cursor(&self) {
        // 简化版：清除光标状态
    }

    pub fn beghouled_remove_horizontal_match(&mut self, grid_x: i32, grid_y: i32, board_state: &mut BeghouledBoardState) {
        let seed_type = board_state.seed_type[grid_x as usize][grid_y as usize];
        let mut cur_x = grid_x;
        while cur_x < 9 && board_state.seed_type[cur_x as usize][grid_y as usize] == seed_type {
            let board = self.get_board();
            let _ = board.get_top_plant_at(cur_x, grid_y);
            cur_x += 1;
        }
    }

    pub fn beghouled_remove_vertical_match(&mut self, grid_x: i32, grid_y: i32, board_state: &mut BeghouledBoardState) {
        let seed_type = board_state.seed_type[grid_x as usize][grid_y as usize];
        let mut cur_y = grid_y;
        while cur_y < 6 && board_state.seed_type[grid_x as usize][cur_y as usize] == seed_type {
            let board = self.get_board();
            let _ = board.get_top_plant_at(grid_x, cur_y);
            cur_y += 1;
        }
    }

    pub fn beghouled_remove_matches(&mut self, board_state: &mut BeghouledBoardState) {
        for grid_y in 0..5 {
            for grid_x in 0..8 {
                let hor_len = self.beghouled_horizontal_match_length(grid_x, grid_y, board_state);
                if hor_len >= 3 {
                    self.beghouled_remove_horizontal_match(grid_x, grid_y, board_state);
                    self.beghouled_score(grid_x, grid_y, hor_len, 1);
                }
                let ver_len = self.beghouled_vertical_match_length(grid_x, grid_y, board_state);
                if ver_len >= 3 {
                    self.beghouled_remove_vertical_match(grid_x, grid_y, board_state);
                    self.beghouled_score(grid_x, grid_y, ver_len, 0);
                }
            }
        }
    }

    pub fn update(&mut self) {
        // [TRANSLATION_NOTE]: 完整逻辑涉及多个未翻译 Board/LawnApp 字段和子函数
        // 保留核心控制流结构
        let is_stormy = self.get_app().is_stormy_night_level();
        let a_game_mode = self.get_app().game_mode;
        let a_game_scene = self.get_app().game_scene;
        if is_stormy {
            // [TRANSLATION_NOTE]: UpdateStormyNight() 暂未实现
        }
        let board = self.get_board();
        if board.m_paused {
            if a_game_mode == GameMode::ChallengeBeghouledTwist {
                self.challenge_grid_x = -1;
                self.challenge_grid_y = -1;
            }
            return;
        }
        if a_game_mode == GameMode::ChallengeRainingSeeds || is_stormy {
            // [TRANSLATION_NOTE]: UpdateRain() 暂未实现
        }
        if a_game_scene != crate::lawn::lawn_app::GameScenes::Playing && a_game_mode != GameMode::ChallengeTreeOfWisdom {
            return;
        }
        if board.has_conveyor_belt_seed_bank() {
            self.update_conveyor_belt();
        }
    }

    pub fn update_beghouled(&mut self) {
        {
            let board = self.get_board();
            board.m_progress_meter_width = 0;
        }
        if self.challenge_state_counter > 0 {
            self.challenge_state_counter -= 1;
        }
    }

    pub fn update_beghouled_plant(&mut self, plant: &mut Plant) -> i32 {
        let board = self.get_board();
        let diff_x = board.grid_to_pixel_x(plant.plant_col, plant.base.row) - plant.pos_x as i32;
        let diff_y = board.grid_to_pixel_y(plant.plant_col, plant.base.row) - plant.pos_y as i32;
        let mut moving = 0;
        if diff_x > 0 || diff_x < 0 { moving = 1; }
        if diff_y > 0 || diff_y < 0 { moving = 1; }
        moving
    }

    pub fn beghouled_fall_into_square(&mut self, grid_x: i32, grid_y: i32, board_state: &mut BeghouledBoardState) {
        if self.beghouled_eated[grid_x as usize][grid_y as usize] != 0 { return; }
        let board = self.get_board();
        for a_grid_y in (0..grid_y).rev() {
            if let Some(p) = board.get_top_plant_at(grid_x, a_grid_y) {
                board_state.seed_type[grid_x as usize][grid_y as usize] = p.seed_type;
                board_state.seed_type[grid_x as usize][a_grid_y as usize] = SeedType::None;
                self.beghouled_start_falling(ChallengeState::BeghouledFalling);
                break;
            }
        }
    }

    pub fn beghouled_make_plants_fall(&mut self, board_state: &mut BeghouledBoardState) {
        for a_grid_y in (0..5).rev() {
            for a_grid_x in 0..8 {
                if self.beghouled_get_plant_at(a_grid_x, a_grid_y, board_state) == SeedType::None {
                    self.beghouled_fall_into_square(a_grid_x, a_grid_y, board_state);
                }
            }
        }
    }

    pub fn zombie_ate_plant(&mut self, _plant: &mut Plant) {
        // 已在 Beghouled 部分实现
    }

    pub fn draw_backdrop(&self, _g: &mut Graphics) {
        // 依赖图片资源
    }

    pub fn draw_art_challenge(&self, _g: &mut Graphics) {
        // 依赖图片资源
    }

    pub fn check_for_complete_art_challenge(&mut self, _grid_x: i32, _grid_y: i32) {
        // 简化版
    }

    pub fn get_art_challenge_seed(&self, _grid_x: i32, _grid_y: i32) -> SeedType {
        SeedType::None
    }

    pub fn plant_added(&mut self, _plant: &mut Plant) {
        // 简化版
    }

    pub fn can_plant_at(&self, _grid_x: i32, _grid_y: i32, _seed_type: SeedType) -> PlantingReason {
        PlantingReason::Ok
    }

    pub fn draw_beghouled(&self, _g: &mut Graphics) {
        // 依赖图片资源
    }

    pub fn beghouled_is_valid_move(&self, _from_x: i32, _from_y: i32, _to_x: i32, _to_y: i32, _board_state: &BeghouledBoardState) -> i32 {
        0
    }

    pub fn beghouled_check_for_possible_moves(&self, _board_state: &BeghouledBoardState) -> i32 {
        0
    }

    pub fn beghouled_check_stuck_state(&mut self) {
        if self.challenge_state != ChallengeState::Normal { return; }
        let board = self.get_board();
        if board.has_level_award_dropped() { return; }
    }

    pub fn init_zombie_waves_survival(&mut self) {
        let board = self.get_board();
        board.m_zombie_allowed[ZombieType::Normal as usize] = true;
        board.m_zombie_allowed[ZombieType::TrafficCone as usize] = true;
    }

    pub fn init_zombie_waves_from_list(&mut self, zombie_list: &[ZombieType]) {
        let board = self.get_board();
        for &z in zombie_list { board.m_zombie_allowed[z as usize] = true; }
    }

    pub fn init_zombie_waves(&mut self) {
        let board = self.get_board();
        board.m_zombie_allowed[ZombieType::Normal as usize] = true;
    }

    pub fn slot_machine_get_handle_rect(&self) -> crate::framework::rect::Rect {
        crate::framework::rect::Rect::new(0, 0, 55, 80)
    }

    pub fn update_slot_machine(&mut self) {
        let sun_money;
        {
            let board = self.get_board();
            sun_money = board.m_sun_money.max(0).min(2000);
            board.m_progress_meter_width = sun_money;
        }
        if sun_money >= 2000 {
            self.spawn_level_award(4, 2);
        }
    }

    pub fn draw_slot_machine(&self, _g: &mut Graphics) {
        // 依赖图片资源
    }

    pub fn update_tool_tip(&self, _x: i32, _y: i32) -> i32 {
        0
    }

    pub fn whack_a_zombie_spawning(&mut self) {
        let board = self.get_board();
        if board.m_current_wave == board.m_num_waves && board.m_zombie_count_down == 0 { return; }
        board.m_zombie_count_down -= 1;
        if board.m_zombie_count_down == 0 {
            board.m_zombie_count_down = 2000;
            board.m_zombie_count_down_start = board.m_zombie_count_down;
            if board.m_current_wave < board.m_num_waves { board.m_current_wave += 1; }
        }
    }

    pub fn update_zombie_spawning(&mut self) -> i32 {
        if self.get_app().is_whack_a_zombie_level() {
            self.whack_a_zombie_spawning();
            return 1;
        }
        0
    }

    pub fn beghouled_clear_crater(&mut self, _count: i32) {
        let board = self.get_board();
        board.clear_advice(AdviceType::None);
    }

    pub fn mouse_down_whack_a_zombie(&mut self, x: i32, y: i32) {
        let board = self.get_board();
        let mut top_zombie_idx = None;
        for (idx, zombie) in board.zombies.iter().enumerate() {
            if zombie.dead { continue; }
            if !zombie.is_dead_or_dying() {
                if crate::lawn::board::get_circle_rect_overlap(x, y - 20, 45, &zombie.get_zombie_rect()) {
                    top_zombie_idx = Some(idx);
                }
            }
        }
        if let Some(idx) = top_zombie_idx {
            let zombie = &mut board.zombies[idx];
            if zombie.helm_type != HelmType::None {
                zombie.take_helm_damage(900, 0);
            } else {
                zombie.die_with_loot();
            }
        }
    }

    pub fn draw_storm_night(&self, g: &mut Graphics) {
        if self.challenge_state == ChallengeState::StormFlash1 && self.challenge_state_counter < 300 {
            self.draw_storm_flash(g, self.challenge_state_counter, 255);
        } else {
            g.set_color(&crate::framework::color::Color::new(0, 0, 0, 255));
            g.fill_rect(&crate::framework::rect::Rect::new(-1000, -1000, 2800, 2600));
        }
    }

    pub fn update_stormy_night(&mut self) {
        let board = self.get_board();
        if board.m_paused { return; }
        self.challenge_state_counter -= 1;
        if self.challenge_state_counter <= 0 {
            self.challenge_state_counter = 150 + 300;
            self.challenge_state = ChallengeState::StormFlash1;
        }
    }

    pub fn init_level(&mut self) {
        // [TRANSLATION_NOTE]: 完整逻辑涉及多个 Board 未翻译字段（m_zombie_count_down_start, m_level, m_seed_bank.add_seed 等）
        // 当前仅保留状态转换逻辑
        if self.get_app().is_stormy_night_level() {
            self.challenge_state = ChallengeState::StormFlash2;
            self.challenge_state_counter = 100;
        }
        if self.get_app().game_mode == GameMode::ChallengeBeghouledTwist {
            self.challenge_grid_x = -1;
            self.challenge_grid_y = -1;
        }
    }

    pub fn spawn_zombie_wave(&mut self) {
        let board = self.get_board();
        if board.m_current_wave == board.m_num_waves { return; }
        board.m_current_wave += 1;
    }

    pub fn grave_danger_spawn_random_grave(&mut self) {
        self.grave_danger_spawn_grave_at(5, 2);
    }

    pub fn grave_danger_spawn_grave_at(&mut self, grid_x: i32, grid_y: i32) {
        let board = self.get_board();
        board.m_enable_grave_stones = true;
        let mut stone = crate::lawn::grid_item::GridItem::new();
        stone.grid_item_type = crate::lawn::grid_item::GridItemType::Grave;
        stone.grid_x = grid_x;
        stone.grid_y = grid_y;
        board.grid_items.push(stone);
    }

    pub fn spawn_level_award(&self, _grid_x: i32, _grid_y: i32) {
        // 简化版
    }

    pub fn beghouled_score(&mut self, grid_x: i32, grid_y: i32, num_plants: i32, is_horizontal: i32) {
        let pos_x;
        let pos_y;
        {
            let board = self.get_board();
            let px = board.grid_to_pixel_x(grid_x, grid_y) as f32;
            let py = board.grid_to_pixel_y(grid_x, grid_y) as f32;
            pos_x = if is_horizontal != 0 { px + if num_plants == 3 { 80.0 } else if num_plants == 4 { 120.0 } else { 160.0 } } else { px };
            pos_y = if is_horizontal == 0 { py + if num_plants == 3 { 80.0 } else if num_plants == 4 { 120.0 } else { 160.0 } } else { py };
        }
        self.challenge_score += 1;
        let score = self.challenge_score;
        let num_suns = (num_plants - 2 + self.beghouled_matches_this_move).max(1).min(5);
        self.beghouled_matches_this_move += 1;
        if score >= 75 {
            self.spawn_level_award(grid_x, grid_y);
            let board = self.get_board();
            board.clear_advice(AdviceType::None);
        } else {
            let board = self.get_board();
            for i in 0..num_suns {
                board.add_coin(pos_x + 20.0 * i as f32 - 10.0, pos_y, CoinType::Sun, CoinMotion::Coin);
            }
        }
    }

    pub fn draw_storm_flash(&self, g: &mut Graphics, _time: i32, _max_amount: i32) {
        g.set_color(&crate::framework::color::Color::new(0, 0, 0, 128));
        g.fill_rect(&crate::framework::rect::Rect::new(-1000, -1000, 2800, 2600));
    }

    pub fn update_raining_seeds(&mut self) {
        let board = self.get_board();
        if board.has_level_award_dropped() { return; }
        self.challenge_state_counter -= 1;
    }

    pub fn play_boss_enter(&self) {
        if let Some(board) = self.board { unsafe {
            (*board).add_zombie(ZombieType::Boss, 0);
        } }
    }

    pub fn update_conveyor_belt(&mut self) {
        if self.get_board().has_level_award_dropped() { return; }
        self.conveyor_belt_counter -= 1;
        if self.conveyor_belt_counter <= 0 {
            self.conveyor_belt_counter = 400;
        }
    }

    pub fn portal_start(&mut self) {
        self.challenge_state_counter = 9000;
        let board = self.get_board();
        board.m_zombie_count_down = 200;
        board.m_zombie_count_down_start = board.m_zombie_count_down;
        self.conveyor_belt_counter = 200;
    }

    pub fn update_portal_combat(&mut self) {
        let has_award = self.get_board().has_level_award_dropped();
        if has_award {
            let board = self.get_board();
            board.clear_advice(AdviceType::None);
        } else {
            self.challenge_state_counter -= 1;
            if self.challenge_state_counter <= 0 {
                let board = self.get_board();
                board.clear_advice(AdviceType::None);
                self.challenge_state_counter = 6000;
                self.move_a_portal();
            }
        }
    }

    pub fn get_other_portal(&self, portal: &GridItem) -> Option<*mut GridItem> {
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.grid_item_type == portal.grid_item_type
                    && std::ptr::eq(item, portal) == false {
                    return Some(item as *const _ as *mut GridItem);
                }
            }
        } }
        None
    }

    pub fn update_portal(&mut self, _portal: &mut GridItem) {
        // 简化版
    }

    pub fn portal_combat_row_spawn_weight(&self, _grid_y: i32) -> f32 {
        0.2
    }

    pub fn can_target_zombie_with_portals(&self, _plant: &Plant, _zombie: &Zombie) -> i32 {
        1
    }

    pub fn get_portal_to_right(&self, grid_x: i32, grid_y: i32) -> Option<*mut GridItem> {
        let mut record: Option<*mut GridItem> = None;
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.grid_x > grid_x && item.grid_y == grid_y {
                    if record.is_none() || unsafe { &*record.unwrap() }.grid_x > item.grid_x {
                        record = Some(item as *const _ as *mut GridItem);
                    }
                }
            }
        } }
        record
    }

    pub fn get_portal_at(&self, _grid_x: i32, _grid_y: i32) -> Option<*mut GridItem> {
        None
    }

    pub fn move_a_portal(&mut self) {
        // MoveAPortal — 简化版
        if let Some(_board) = self.board { unsafe {
            // 简化：不做实际传送门移动
        } }
    }

    pub fn get_portal_distance_to_mower(&self, _grid_y: i32) -> i32 {
        0
    }

    pub fn get_portal_to_left(&self, grid_x: i32, grid_y: i32) -> Option<*mut GridItem> {
        let mut record: Option<*mut GridItem> = None;
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.grid_x < grid_x && item.grid_y == grid_y {
                    if record.is_none() || unsafe { &*record.unwrap() }.grid_x < item.grid_x {
                        record = Some(item as *const _ as *mut GridItem);
                    }
                }
            }
        } }
        record
    }

    pub fn beghouled_packet_clicked(&mut self, seed_packet: &mut SeedPacket) {
        let board = self.get_board();
        let _cost = board.get_current_plant_cost(seed_packet.seed_type, SeedType::None);
        board.take_sun_money(_cost);
        board.refresh_seed_packet_from_cursor();
    }

    pub fn beghouled_shuffle(&mut self) {
        let board = self.get_board();
        board.clear_advice(AdviceType::None);
        self.beghouled_start_falling(ChallengeState::BeghouledFalling);
    }

    pub fn beghouled_can_clear_crater(&self) -> i32 {
        for row in 0..5 {
            for col in 0..8 {
                if self.beghouled_eated[col][row] != 0 { return 1; }
            }
        }
        0
    }

    pub fn beghouled_update_craters(&mut self) {
        // 简化版
    }

    pub fn zombiquarium_spawn_snorkle(&mut self) -> Option<*mut Zombie> {
        let board = self.get_board();
        // 简化版：返回 None
        None
    }

    pub fn zombiquarium_packet_clicked(&mut self, _seed_packet: &mut SeedPacket) {
        // 简化版
    }

    pub fn zombiquarium_mouse_down(&mut self, _x: i32, _y: i32) {
        // 简化版
    }

    pub fn zombiquarium_drop_brain(&mut self, _x: i32, _y: i32) {
        // 简化版
    }

    pub fn zombiquarium_update(&mut self) {
        let board = self.get_board();
        let score = board.m_sun_money.max(0).min(1000);
        board.m_progress_meter_width = score;
        if score >= 1000 - 100 {
            board.display_advice("[ALMOST_THERE]", 3, AdviceType::None);
        }
    }

    pub fn shovel_add_wallnuts(&self) {
        if let Some(board) = self.board { unsafe {
            for col in 0..9 {
                for row in 0..5 {
                    (*board).add_plant(col, row, SeedType::Wallnut, SeedType::None);
                }
            }
        } }
    }

    pub fn scary_potter_place_pot(&mut self, _pot_type: ScaryPotType, _zombie_type: ZombieType, _seed_type: SeedType, _count: i32, _grid_array: &mut [crate::todlib::tod_common::TodWeightedGridArray], _grid_array_count: i32) {
        // 简化版
    }

    pub fn scary_potter_start(&mut self) {
        let board = self.get_board();
        board.display_advice("[USE_SHOVEL_ON_POTS]", 4, AdviceType::None);
    }

    pub fn scary_potter_update(&mut self) {
        if self.challenge_state == ChallengeState::ScaryPotterMaletting {
            self.challenge_state = ChallengeState::Normal;
        }
    }

    pub fn scary_potter_open_pot(&mut self, scary_pot: &mut GridItem) {
        scary_pot.grid_item_die();
        if self.scary_potter_is_completed() != 0 {
            self.spawn_level_award(scary_pot.grid_x, scary_pot.grid_y);
        }
    }

    pub fn scary_potter_jack_explode(&self, _pos_x: i32, _pos_y: i32) {
        // 简化版
    }

    pub fn scary_potter_is_completed(&self) -> i32 {
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.grid_item_type == crate::lawn::grid_item::GridItemType::ScaryPot { return 0; }
            }
        } }
        1
    }

    pub fn scary_potter_change_pot_type(&mut self, _pot_type: GridItemState, _count: i32) {
        // 简化版
    }

    pub fn scary_potter_populate(&mut self) {
        self.scary_potter_pots = self.scary_potter_count_pots();
    }

    pub fn scary_potter_dont_place_in_col(&self, _col: i32, _grid_array: &mut [crate::todlib::tod_common::TodWeightedGridArray], _grid_array_count: i32) {
        // 简化版
    }

    pub fn scary_potter_fill_column_with_plant(&mut self, _col: i32, _seed_type: SeedType, _grid_array: &mut [crate::todlib::tod_common::TodWeightedGridArray], _grid_array_count: i32) {
        // 简化版
    }

    pub fn puzzle_next_stage_clear(&mut self) {
        self.survival_stage += 1;
        let board = self.get_board();
        board.clear_advice_immediately();
        board.m_level_award_spawned = false;
    }

    pub fn scary_potter_mallet_pot(&mut self, scary_pot: &mut GridItem) {
        self.challenge_grid_x = scary_pot.grid_x;
        self.challenge_grid_y = scary_pot.grid_y;
        self.challenge_state = ChallengeState::ScaryPotterMaletting;
        self.scary_potter_open_pot(scary_pot);
    }

    pub fn i_zombie_seed_type_to_zombie_type(seed_type: SeedType) -> ZombieType {
        match seed_type {
            SeedType::Sprout => ZombieType::Normal,
            _ => ZombieType::Normal,
        }
    }

    pub fn is_zombie_seed_type(seed_type: SeedType) -> i32 {
        matches!(seed_type, SeedType::Sprout).then(|| 1).unwrap_or(0)
    }

    pub fn i_zombie_mouse_down_with_zombie(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // 简化版
    }

    pub fn i_zombie_start(&mut self) {
        let board = self.get_board();
        board.display_advice("[I_ZOMBIE_EAT_ALL_BRAINS]", 1, AdviceType::None);
    }

    pub fn i_zombie_place_plants(&mut self, seed_type: SeedType, count: i32, grid_y: i32) {
        let board = self.get_board();
        for _ in 0..count.min(10) {
            if grid_y == -1 {
                for row in 0..5 {
                    if board.can_plant_at(0, row, seed_type) == PlantingReason::Ok {
                        board.add_plant(0, row, seed_type, SeedType::None);
                        break;
                    }
                }
            } else {
                if board.can_plant_at(0, grid_y, seed_type) == PlantingReason::Ok {
                    board.add_plant(0, grid_y, seed_type, SeedType::None);
                }
            }
        }
    }

    pub fn i_zombie_update(&mut self) {
        let board = self.get_board();
        if board.zombies.len() == 0 && board.m_sun_money < 50 && !board.has_level_award_dropped() {
            board.zombies_won();
        }
    }

    pub fn i_zombie_draw_plant(&self, _g: &mut Graphics, _plant: &Plant) {
        // 依赖 Reanimation 系统
    }

    pub fn i_zombie_set_plant_filter_effect(&self, _plant: &mut Plant, _filter_effect: crate::todlib::filter_effect::FilterEffectType) {
        // 依赖 Reanimation 系统
    }

    pub fn scary_potter_count_sun_in_pot(&self, _scary_pot: &GridItem) -> i32 {
        0
    }

    pub fn scary_potter_count_pots(&self) -> i32 {
        let mut count = 0;
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.grid_item_type == crate::lawn::grid_item::GridItemType::ScaryPot { count += 1; }
            }
        } }
        count
    }

    pub fn i_zombie_init_level(&mut self) {
        self.challenge_score = 0;
        let board = self.get_board();
        for row in 0..5 {
            let mut brain = crate::lawn::grid_item::GridItem::new();
            brain.grid_item_type = crate::lawn::grid_item::GridItemType::None;
            brain.grid_x = 0;
            brain.grid_y = row;
            brain.render_order = 0;
            brain.counter = 70;
            brain.pos_x = (board.grid_to_pixel_x(0, row) - 40) as f32;
            brain.pos_y = (board.grid_to_pixel_y(0, row) + 40) as f32;
            board.grid_items.push(brain);
        }
    }

    pub fn draw_rain(&self, _g: &mut Graphics) {
        // 依赖图片资源
    }

    pub fn draw_weather(&self, g: &mut Graphics) {
        if self.get_app().is_stormy_night_level() || self.get_app().game_mode == GameMode::ChallengeRainingSeeds {
            self.draw_rain(g);
        }
        if self.get_app().is_stormy_night_level() {
            self.draw_storm_night(g);
        }
    }

    pub fn squirrel_update(&mut self) {
        let score = self.squirrel_count_uncaught();
        self.challenge_score = 7 - score;
        let progress = self.challenge_score;
        let board = self.get_board();
        board.m_progress_meter_width = progress;
    }

    pub fn squirrel_count_uncaught(&self) -> i32 {
        let mut count = 0;
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if item.grid_item_type == crate::lawn::grid_item::GridItemType::None && item.dead == false {
                    count += 1;
                }
            }
        } }
        count
    }

    pub fn squirrel_start(&mut self) {
        let board = self.get_board();
        for _ in 0..7 {
            let mut squirrel = crate::lawn::grid_item::GridItem::new();
            squirrel.grid_item_type = crate::lawn::grid_item::GridItemType::None;
            squirrel.grid_x = 0;
            squirrel.grid_y = 0;
            board.grid_items.push(squirrel);
        }
    }

    pub fn squirrel_found(&mut self, _squirrel: &mut GridItem) {
        // 简化版
    }

    pub fn squirrel_peek(&mut self, _squirrel: &mut GridItem) {
        // 简化版
    }

    pub fn squirrel_chew(&mut self, _squirrel: &mut GridItem) {
        // 简化版
    }

    pub fn squirrel_update_one(&mut self, _squirrel: &mut GridItem) {
        // 简化版
    }

    pub fn i_zombie_setup_plant(&self, _plant: &mut Plant) {
        // 简化版：不依赖 Reanimation 系统
    }

    pub fn update_rain(&mut self) {
        self.rain_counter -= 1;
        if self.rain_counter < 0 {
            self.rain_counter = 15;
        }
    }

    pub fn i_zombie_eat_brain(&mut self, zombie: &mut Zombie) -> i32 {
        let brain = self.i_zombie_get_brain_target(zombie);
        if brain.is_none() { return 0; }
        1
    }

    pub fn i_zombie_get_brain_target(&self, zombie: &Zombie) -> Option<*mut GridItem> {
        if zombie.zombie_type == ZombieType::Bungee || zombie.is_walking_backwards() { return None; }
        let rect = zombie.get_zombie_attack_rect();
        if rect.x > 20 { return None; }
        None
    }

    pub fn i_zombie_place_plant_in_square(&mut self, seed_type: SeedType, grid_x: i32, grid_y: i32) {
        let board = self.get_board();
        if board.can_plant_at(grid_x, grid_y, seed_type) == crate::lawn::game_enums::PlantingReason::Ok {
            board.add_plant(grid_x, grid_y, seed_type, SeedType::None);
        }
    }

    pub fn advance_crazy_dave_dialog(&mut self) {
        // 简化版
    }

    pub fn beghouled_flash_plant(&mut self, flash_x: i32, flash_y: i32, from_x: i32, from_y: i32, to_x: i32, to_y: i32) {
        let (fx, fy) = if flash_x == from_x && flash_y == from_y {
            (to_x, to_y)
        } else if flash_x == to_x && flash_y == to_y {
            (from_x, from_y)
        } else {
            (flash_x, flash_y)
        };
        let board = self.get_board();
        let _ = board.get_top_plant_at(fx, fy);
    }

    pub fn beghouled_flash_a_match(&mut self) {
        let mut board_state = BeghouledBoardState { seed_type: [[SeedType::None; 6]; 9] };
        self.load_beghouled_board_state(&mut board_state);
        let game_mode = self.get_app().game_mode;
        if game_mode == GameMode::ChallengeBeghouled {
            for row in 0..=4 {
                for col in 0..=7 {
                    if col < 7 && self.beghouled_flash_from_board_state(&mut board_state, col, row, col + 1, row) != 0 { return; }
                    if row < 4 && self.beghouled_flash_from_board_state(&mut board_state, col, row, col, row + 1) != 0 { return; }
                }
            }
        }
    }

    pub fn beghouled_flash_from_board_state(&mut self, board_state: &mut BeghouledBoardState, from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> i32 {
        if self.beghouled_eated[from_x as usize][from_y as usize] != 0 || self.beghouled_eated[to_x as usize][to_y as usize] != 0 { return 0; }
        let from_seed = board_state.seed_type[from_x as usize][from_y as usize];
        let to_seed = board_state.seed_type[to_x as usize][to_y as usize];
        board_state.seed_type[from_x as usize][from_y as usize] = to_seed;
        board_state.seed_type[to_x as usize][to_y as usize] = from_seed;
        let has_match = self.beghouled_board_has_match(board_state);
        board_state.seed_type[from_x as usize][from_y as usize] = from_seed;
        board_state.seed_type[to_x as usize][to_y as usize] = to_seed;
        has_match
    }

    pub fn i_zombie_plant_drop_remaining_sun(&self, plant: &Plant) {
        if plant.seed_type == SeedType::Sunflower {
            let sun_count = plant.plant_health / 40 + 1;
            if let Some(board) = self.board { unsafe {
                for i in 0..sun_count {
                    (*board).add_coin(plant.pos_x + 5.0 * i as f32, plant.pos_y, CoinType::Sun, CoinMotion::FromPlant);
                }
            } }
        }
    }

    pub fn i_zombie_squish_brain(&mut self, brain: &mut GridItem) {
        brain.render_order = 0;
        brain.grid_item_state = GridItemState::BrainSquished;
        brain.counter = 500;
        self.i_zombie_score_brain(brain);
    }

    pub fn i_zombie_score_brain(&mut self, brain: &mut GridItem) {
        self.challenge_score += 1;
        let board = self.get_board();
        board.m_progress_meter_width = 0;
        if self.challenge_score >= 5 {
            self.spawn_level_award(0, brain.grid_y);
        }
    }

    pub fn last_stand_update(&mut self) {
        if self.challenge_state == ChallengeState::LastStandOnslaught {
            self.challenge_state_counter += 1;
        }
    }

    pub fn whack_a_zombie_place_graves(&mut self, _grave_count: i32) {
        // 简化版
    }

    pub fn beghouled_twist_square_from_mouse(&self, _x: i32, _y: i32, grid_x: &mut i32, grid_y: &mut i32) -> i32 {
        *grid_x = -1; *grid_y = -1; 0
    }

    pub fn beghouled_twist_valid_move(&self, _grid_x: i32, _grid_y: i32, _board_state: &BeghouledBoardState) -> i32 {
        0
    }

    pub fn beghouled_twist_mouse_down(&mut self, _x: i32, _y: i32) {
        // 简化版
    }

    pub fn beghouled_twist_move_causes_match(&self, _grid_x: i32, _grid_y: i32, _board_state: &BeghouledBoardState) -> i32 {
        0
    }

    pub fn beghouled_twist_flash_match(&mut self, _board_state: &BeghouledBoardState, _grid_x: i32, _grid_y: i32) -> i32 {
        0
    }

    pub fn beghouled_cancel_match_flashing(&mut self) {
        // 简化版
    }

    pub fn beghouled_start_falling(&mut self, state: ChallengeState) {
        self.challenge_state = state;
        self.challenge_state_counter = 100;
        let board = self.get_board();
        board.clear_advice(AdviceType::None);
    }

    pub fn beghouled_fill_holes(&mut self, board_state: &mut BeghouledBoardState, allow_matches: i32) {
        for col in 0..8 {
            for row in 0..5 {
                if board_state.seed_type[col as usize][row as usize] == SeedType::None && self.beghouled_eated[col as usize][row as usize] == 0 {
                    board_state.seed_type[col as usize][row as usize] = self.beghouled_pick_seed(col, row, board_state, allow_matches);
                }
            }
        }
    }

    pub fn beghouled_make_start_board(&mut self) {
        let mut board_state = BeghouledBoardState { seed_type: [[SeedType::None; 6]; 9] };
        self.load_beghouled_board_state(&mut board_state);
        self.beghouled_fill_holes(&mut board_state, 0);
    }

    pub fn beghouled_create_plants(&mut self, _old_board_state: &BeghouledBoardState, _new_board_state: &BeghouledBoardState) {
        // 简化版
    }

    pub fn puzzle_phase_complete(&mut self, _grid_x: i32, _grid_y: i32) {
        // 简化版
    }

    pub fn puzzle_is_award_stage(&self) -> i32 {
        if self.survival_stage % 1 == 0 { 1 } else { 0 }
    }

    pub fn i_zombie_place_zombie(&mut self, zombie_type: ZombieType, grid_x: i32, grid_y: i32) {
        let board = self.get_board();
        board.add_zombie_in_row(zombie_type, grid_y, 0);
    }

    pub fn whack_a_zombie_update(&mut self) {
        let _board = self.get_board();
        // 简化版：教程状态检查略
    }

    pub fn last_stand_completed_stage(&mut self) {
        self.challenge_state = ChallengeState::Normal;
        self.survival_stage += 1;
        let board = self.get_board();
        board.m_level_complete = false;
    }

    pub fn tree_of_wisdom_update(&mut self) {
        if self.challenge_state_counter > 0 {
            self.challenge_state_counter -= 1;
        }
        if self.challenge_state_counter == 0 {
            if self.challenge_state == ChallengeState::TreeJustGrew {
                self.tree_of_wisdom_give_wisdom();
            } else if self.challenge_state == ChallengeState::TreeWaitingToBabble {
                self.tree_of_wisdom_babble();
            } else {
                self.challenge_state = ChallengeState::TreeWaitingToBabble;
                self.challenge_state_counter = 1000;
            }
        }
    }

    pub fn tree_of_wisdom_fertilize(&mut self) {
        self.challenge_state = ChallengeState::Normal;
        let board = self.get_board();
        board.clear_cursor();
    }

    pub fn tree_of_wisdom_init(&mut self) {
        self.challenge_state = ChallengeState::TreeWaitingToBabble;
        self.challenge_state_counter = 1000;
    }

    pub fn tree_of_wisdom_mouse_on(&self, _x: i32, _y: i32) -> i32 {
        0
    }

    pub fn tree_of_wisdom_get_size(&self) -> i32 {
        30
    }

    pub fn tree_of_wisdom_draw(&self, _g: &mut Graphics) {
        // 占位绘图
    }

    pub fn tree_of_wisdom_next_garden(&self) {
        // 简化版
    }

    pub fn tree_of_wisdom_tool_update(&self, _zen_tool: &mut GridItem) {
        // 简化版
    }

    pub fn tree_of_wisdom_open_store(&self) {
        // 简化版
    }

    pub fn tree_of_wisdom_leave(&mut self) {
        // 简化版
    }

    pub fn tree_of_wisdom_grow(&mut self) {
        self.challenge_state = ChallengeState::TreeJustGrew;
        self.challenge_state_counter = 120;
    }

    pub fn tree_of_wisdom_tool(&self, _mouse_x: i32, _mouse_y: i32) {
        // 简化版
    }

    pub fn tree_of_wisdom_hit_test(&self, _x: i32, _y: i32, hit_result: &mut HitResult) -> i32 {
        hit_result.object_type = GameObjectType::None;
        0
    }

    pub fn tree_of_wisdom_babble(&mut self) {
        self.challenge_state = ChallengeState::TreeBabbling;
        self.challenge_state_counter = 400;
        self.tree_of_wisdom_talk_index = 101;
    }

    pub fn tree_of_wisdom_give_wisdom(&mut self) {
        self.challenge_state = ChallengeState::TreeGiveWisdom;
        self.challenge_state_counter = 1000;
    }

    pub fn tree_of_wisdom_say_repeat(&self) {
        // 简化版
    }

    pub fn tree_of_wisdom_can_feed(&self) -> i32 {
        if self.challenge_state == ChallengeState::TreeJustGrew { return 0; }
        1
    }

    pub fn get_portal_left_right(&self, _grid_x: i32, _grid_y: i32, _to_left: bool) -> Option<*mut GridItem> {
        None
    }
}

impl Default for Challenge {
    fn default() -> Self {
        Challenge::new()
    }
}

// --- 全局变量 ---
// 对应 C++: extern SeedType gArtChallengeWallnut[6][9];
// 对应 C++: extern SeedType gArtChallengeSunFlower[6][9];
// 对应 C++: extern SeedType gArtChallengeStarFruit[6][9];
// 这些数据在 Challenge.cpp 中定义，后续翻译具体实现时添加

// 对应 C++: extern int gZombieWaves[NUM_LEVELS];
// 对应 C++: extern ZombieAllowedLevels gZombieAllowedLevels[NUM_ZOMBIE_TYPES];



