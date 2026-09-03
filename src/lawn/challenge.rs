// PvZ Portable Rust 翻译 — Challenge（挑战模式）
// 对应 C++ src/Lawn/Challenge.h / Challenge.cpp

use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;
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

/// 工具函数：从资源管理器获取图片（对应 C++ IMAGE_* 资源）
fn get_image(app: &crate::lawn::lawn_app::LawnApp, name: &str) -> *mut crate::framework::graphics::image::Image {
    if let Some(rm_ptr) = app.base.resource_manager {
        unsafe {
            let rm = &*rm_ptr;
            let shared = rm.get_image(name);
            let img_ptr = shared.as_image_ptr();
            if !img_ptr.is_null() {
                return img_ptr;
            }
            // 尝试小写
            let lower = crate::framework::common::string_to_lower(name);
            let shared2 = rm.get_image(&lower);
            let img_ptr2 = shared2.as_image_ptr();
            if !img_ptr2.is_null() {
                return img_ptr2;
            }
        }
    }
    std::ptr::null_mut()
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
        // 对应 C++ Challenge::StartLevel L439-L576
        if self.get_app().is_whack_a_zombie_level() {
            // C++ L441-L454: 锤子游标与计数
            // [TRANSLATION_NOTE]: mCursorObject->mCursorType/锤子 reanim（ReanimatorEnsureDefinitionLoaded、
            // AddReanimation/ReanimationGetID）未接入，仅保留 mZombieCountDown 设置
            let board = self.get_board();
            board.m_zombie_count_down = 200;
            board.m_zombie_count_down_start = board.m_zombie_count_down;
        }
        if self.get_app().is_stormy_night_level() {
            self.challenge_state = ChallengeState::StormFlash1;
            self.challenge_state_counter = 400;
        }
        let a_game_mode = self.get_app().game_mode;
        if a_game_mode == GameMode::ChallengeBobsledBonanza {
            // C++ L461-L471: 非水池行设为全冰面
            let board = self.get_board();
            for i in 0..crate::lawn::board::MAX_GRID_SIZE_Y {
                if board.m_plant_row[i] != PlantRowType::Pool {
                    board.m_ice_min_x[i] = 400;
                    board.m_ice_timer[i] = 0x7FFFFFFF;
                }
            }
        }
        if self.get_app().is_wallnut_bowling_level() {
            // C++ L472-L479
            let board = self.get_board();
            board.m_zombie_count_down = 200;
            board.m_zombie_count_down_start = board.m_zombie_count_down;
            // C++: mBoard->mSeedBank->AddSeed(SEED_WALLNUT) —— [TRANSLATION_NOTE]: SeedBank 未接入 Rust，跳过
            self.conveyor_belt_counter = 400;
            self.show_bowling_line = 1;
        }
        if a_game_mode == GameMode::ChallengeShovel || a_game_mode == GameMode::ChallengeSquirrel {
            self.shovel_add_wallnuts();
        }
        if self.get_app().is_scary_potter_level() {
            self.scary_potter_start();
        }
        if self.get_app().is_little_trouble_level()
            || self.get_app().is_stormy_night_level()
            || self.get_app().is_bungee_blitz_level()
            || a_game_mode == GameMode::ChallengeInvisighoul
        {
            // C++ L488-L493
            let board = self.get_board();
            board.m_zombie_count_down = 200;
            board.m_zombie_count_down_start = board.m_zombie_count_down;
            self.conveyor_belt_counter = 200;
        }
        if self.get_app().is_survival_mode() && self.survival_stage == 0 {
            // C++ L494-L501: PvzpReplaceNumberString 填充旗子数
            // [TRANSLATION_NOTE]: 数字替换未接入，直接使用原始字符串键
            let a_message = if self.get_app().is_survival_normal(a_game_mode) {
                "[ADVICE_SURVIVE_FLAGS]"
            } else if self.get_app().is_survival_hard(a_game_mode) {
                "[ADVICE_SURVIVE_FLAGS]"
            } else {
                "[ADVICE_SURVIVE_ENDLESS]"
            };
            self.get_board().display_advice(a_message, MessageStyle::HintFast as i32, AdviceType::SurviveFlags);
        }
        if a_game_mode == GameMode::ChallengeLastStand && self.survival_stage == 0 {
            // C++ L502-L505
            self.get_board().display_advice(
                "[ADVICE_SURVIVE_FLAGS]",
                MessageStyle::BigMiddleFast as i32,
                AdviceType::SurviveFlags,
            );
        }
        if a_game_mode == GameMode::ChallengeArtChallengeWallnut {
            self.get_board().display_advice("[ADVICE_FILL_IN_WALLNUTS]", MessageStyle::HintFast as i32, AdviceType::None);
        }
        if a_game_mode == GameMode::ChallengeArtChallengeSunflower {
            self.get_board().display_advice("[ADVICE_FILL_IN_SPACES]", MessageStyle::HintFast as i32, AdviceType::None);
        }
        if a_game_mode == GameMode::ChallengeSeeingStars {
            self.get_board().display_advice("[ADVICE_FILL_IN_STARFRUIT]", MessageStyle::HintFast as i32, AdviceType::None);
        }
        if self.get_app().is_slot_machine_level() {
            // C++ L518-L521: TUTORIAL_SLOT_MACHINE_PULL
            self.get_board().set_tutorial_state(TutorialState::SlotMachinePullTut);
        }
        if a_game_mode == GameMode::ChallengeBeghouled || a_game_mode == GameMode::ChallengeBeghouledTwist {
            // C++ L522-L538
            let board = self.get_board();
            board.m_zombie_count_down = 200;
            board.m_zombie_count_down_start = board.m_zombie_count_down;
            self.beghouled_make_start_board();
            self.beghouled_update_craters();
            self.challenge_state_counter = 1500;
            if a_game_mode == GameMode::ChallengeBeghouled {
                self.get_board().display_advice("[ADVICE_BEGHOULED_DRAG_TO_MATCH_3]", MessageStyle::HintFast as i32, AdviceType::None);
            } else {
                self.get_board().display_advice("[ADVICE_BEGHOULED_TWIST_TO_MATCH_3]", MessageStyle::HintFast as i32, AdviceType::None);
            }
        }
        if self.get_app().is_mini_boss_level() {
            // C++ L539-L544
            let board = self.get_board();
            board.m_zombie_count_down = 100;
            board.m_zombie_count_down_start = board.m_zombie_count_down;
            self.conveyor_belt_counter = 200;
        }
        if a_game_mode == GameMode::ChallengePortalCombat {
            self.portal_start();
        }
        if a_game_mode == GameMode::ChallengeColumns {
            // C++ L549-L553
            let board = self.get_board();
            board.m_current_wave = 9;
            board.m_zombie_count_down = 2400;
        }
        if a_game_mode == GameMode::ChallengeAirRaid || a_game_mode == GameMode::ChallengeBobsledBonanza {
            // C++ L554-L557
            self.get_board().m_zombie_count_down = 4500;
        }
        if a_game_mode == GameMode::ChallengePogoParty {
            // C++ L558-L561
            self.get_board().m_zombie_count_down = 5500;
        }
        if a_game_mode == GameMode::ChallengeZombiquarium {
            // C++ L562-L567
            self.get_board().display_advice(
                "[ADVICE_ZOMBIQUARIUM_CLICK_TO_FEED]",
                MessageStyle::HintTallFast as i32,
                AdviceType::ZombiquariumClickToFeed,
            );
            self.zombiquarium_spawn_snorkle();
            self.zombiquarium_spawn_snorkle();
        }
        if self.get_app().is_izombie_level() {
            self.i_zombie_start();
        }
        if self.get_app().is_squirrel_level() {
            self.squirrel_start();
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

    pub fn clear_cursor(&mut self) {
        // 对应 C++ ClearCursor
        let game_mode = self.get_app().game_mode;
        if game_mode == GameMode::ChallengeBeghouled || game_mode == GameMode::ChallengeBeghouledTwist {
            self.beghouled_drag_cancel();
        }
        if self.get_app().is_whack_a_zombie_level() {
            if let Some(board) = self.board { unsafe {
                let b = &mut *board;
                if !b.has_level_award_dropped() {
                    b.cursor_object.cursor_type = CursorType::Hammer;
                }
            } }
        }
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
        // 对应 C++ Challenge::Update L2139-L2237
        if self.get_app().is_stormy_night_level() {
            self.update_stormy_night();
        }

        let a_game_mode = self.get_app().game_mode;
        let board = self.get_board();
        if board.m_paused {
            if a_game_mode == GameMode::ChallengeBeghouledTwist {
                self.challenge_grid_x = -1;
                self.challenge_grid_y = -1;
            }
            return;
        }
        if a_game_mode == GameMode::ChallengeRainingSeeds || self.get_app().is_stormy_night_level() {
            self.update_rain();
        }
        if self.get_app().game_scene != crate::lawn::lawn_app::GameScenes::Playing
            && a_game_mode != GameMode::ChallengeTreeOfWisdom
        {
            return;
        }
        if self.get_board().has_conveyor_belt_seed_bank() {
            self.update_conveyor_belt();
        }
        if a_game_mode == GameMode::ChallengeBeghouled || a_game_mode == GameMode::ChallengeBeghouledTwist {
            self.update_beghouled();
        }
        if self.get_app().is_scary_potter_level() {
            self.scary_potter_update();
        }
        // C++ L2174-L2184: (ScaryPotter || WhackAZombie) && mSeedBank->mY < 0 时种子栏滑入
        // [TRANSLATION_NOTE]: SeedBank 整体 y 坐标未接入 Rust（board.seed_bank 为 Vec<SeedPacket>），此段跳过
        if self.get_app().is_whack_a_zombie_level() {
            self.whack_a_zombie_update();
        }
        if self.get_app().is_izombie_level() {
            self.i_zombie_update();
        }
        if self.get_app().is_slot_machine_level() {
            self.update_slot_machine();
        }
        if a_game_mode == GameMode::ChallengeZombieNimble {
            // C++ L2200: mBoard->UpdateGame() —— 速度挑战的额外一帧更新（C++ GAMEMODE_CHALLENGE_SPEED）
            self.get_board().update();
        }
        if a_game_mode == GameMode::ChallengeRainingSeeds {
            self.update_raining_seeds();
        }
        if a_game_mode == GameMode::ChallengePortalCombat {
            self.update_portal_combat();
        }
        if self.get_app().is_squirrel_level() {
            self.squirrel_update();
        }
        if a_game_mode == GameMode::ChallengeZombiquarium {
            self.zombiquarium_update();
        }
        if a_game_mode == GameMode::ChallengeTreeOfWisdom {
            self.tree_of_wisdom_update();
        }
        if a_game_mode == GameMode::ChallengeIceLevel && self.get_board().m_main_counter == 3000 {
            // C++ L2222-L2226: 该帧播放 FOLEY_FLOOP 与 SOUND_LOSEMUSIC 音效
            self.get_app().play_foley(crate::todlib::tod_foley::FoleyType::Floop as i32);
            self.get_app().play_sample(crate::framework::resources::ResourceId::SoundLosemusic as i32);
        }
        if a_game_mode == GameMode::ChallengeLastStand {
            self.last_stand_update();
        }
        // C++ L2232-L2236: mReanimChallenge 的 attachment reanim 更新
        // [TRANSLATION_NOTE]: reanim 系统未接入，跳过
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

    pub fn zombie_ate_plant(&mut self, plant: &mut Plant) {
        // 对应 C++ ZombieAtePlant
        let game_mode = self.get_app().game_mode;
        if game_mode != GameMode::ChallengeBeghouled && game_mode != GameMode::ChallengeBeghouledTwist {
            return;
        }
        let col = plant.plant_col;
        let row = plant.base.row;
        self.beghouled_eated[col as usize][row as usize] = 1;

        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            if b.seed_bank.len() == 4 {
                b.seed_bank.push(SeedPacket::new());
                if let Some(packet) = b.seed_bank.get_mut(4) {
                    packet.set_packet_type(SeedType::BeghouledButtonCrater, SeedType::None);
                }
                b.display_advice("[ADVICE_BEGHOULED_USE_CRATER_1]", 2, AdviceType::BeghouledUseCrater1);
            }
        } }
        self.beghouled_check_stuck_state();
        self.beghouled_update_craters();
    }

    pub fn draw_backdrop(&self, g: &mut Graphics) {
        // 对应 C++ Challenge::DrawBackdrop
        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        let game_mode = unsafe { (*app).game_mode };
        if unsafe { (*app).is_art_challenge() } {
            self.draw_art_challenge(g);
        }
        if game_mode == GameMode::ChallengeTreeOfWisdom {
            self.tree_of_wisdom_draw(g);
        }
        if game_mode == GameMode::ChallengeBeghouled || game_mode == GameMode::ChallengeBeghouledTwist {
            self.draw_beghouled(g);
        }

        // C++: DrawImage(IMAGE_WALLNUT_BOWLINGSTRIPE, 268, 77) 保龄球分隔线
        if unsafe { (*app).is_wallnut_bowling_level() } && self.show_bowling_line != 0 {
            let img = unsafe { get_image(&*app, "IMAGE_WALLNUT_BOWLINGSTRIPE") };
            if !img.is_null() {
                unsafe { g.draw_image_xy(&*img, 268, 77); }
            }
        }
        // C++: 我是僵尸各关的分隔线 x 坐标（352/432/512）
        let is_i_zombie = matches!(
            game_mode,
            GameMode::PuzzleIZombie1
                | GameMode::PuzzleIZombie2
                | GameMode::PuzzleIZombie3
                | GameMode::PuzzleIZombie4
                | GameMode::PuzzleIZombie5
                | GameMode::PuzzleIZombie6
                | GameMode::PuzzleIZombie7
                | GameMode::PuzzleIZombie8
                | GameMode::PuzzleIZombieEndless
                | GameMode::PuzzleIZombie9
        );
        if is_i_zombie {
            let x = match game_mode {
                GameMode::PuzzleIZombie1
                | GameMode::PuzzleIZombie2
                | GameMode::PuzzleIZombie3
                | GameMode::PuzzleIZombie4
                | GameMode::PuzzleIZombie5 => 352,
                GameMode::PuzzleIZombie6
                | GameMode::PuzzleIZombie7
                | GameMode::PuzzleIZombie8
                | GameMode::PuzzleIZombieEndless => 432,
                GameMode::PuzzleIZombie9 => 512,
                _ => 352,
            };
            let img = unsafe { get_image(&*app, "IMAGE_WALLNUT_BOWLINGSTRIPE") };
            if !img.is_null() {
                unsafe { g.draw_image_xy(&*img, x, 73); }
            }
        }

        if game_mode == GameMode::ChallengeSlotMachine {
            self.draw_slot_machine(g);
        }
        if game_mode == GameMode::ChallengeZenGarden {
            if let Some(zg) = unsafe { (*app).zen_garden } {
                unsafe { (*zg).draw_backdrop(g); }
            }
        }
    }

    pub fn draw_art_challenge(&self, g: &mut Graphics) {
        // 对应 C++ Challenge::DrawArtChallenge
        g.set_colorize_images(true);
        g.set_color(&crate::framework::color::Color::new(255, 255, 255, 100));

        for a_row in 0..crate::lawn::board::MAX_GRID_SIZE_Y {
            for a_col in 0..crate::lawn::board::MAX_GRID_SIZE_X {
                let a_seed_type = self.get_art_challenge_seed(a_col as i32, a_row as i32);
                if a_seed_type != SeedType::None {
                    let board_ptr = match self.board {
                        Some(b) => b,
                        None => return,
                    };
                    let has_plant = unsafe {
                        (*board_ptr).get_top_plant_at(a_col as i32, a_row as i32).is_some()
                    };
                    if !has_plant {
                        let p_x = unsafe { (*board_ptr).grid_to_pixel_x(a_col as i32, a_row as i32) };
                        let p_y = unsafe { (*board_ptr).grid_to_pixel_y(a_col as i32, a_row as i32) };
                        crate::lawn::plant::Plant::draw_seed_type(
                            g,
                            a_seed_type,
                            SeedType::None,
                            DrawVariation::Normal,
                            p_x as f32,
                            p_y as f32,
                        );
                    }
                }
            }
        }

        // C++: GAMEMODE_CHALLENGE_ART_CHALLENGE_WALLNUT 分支资源已移除，仅保留位置
        g.set_colorize_images(false);
    }

    pub fn check_for_complete_art_challenge(&mut self, _grid_x: i32, _grid_y: i32) {
        let board = self.get_board();
        if board.has_level_award_dropped() { return; }
        let game_mode = self.get_app().game_mode;
        for grid_y in 0..6 {
            for grid_x in 0..9 {
                let seed = Self::get_art_challenge_seed_static(game_mode, grid_x, grid_y);
                if seed != SeedType::None {
                    let has_match = if let Some(board) = self.board { unsafe {
                        (*board).plants.iter().any(|p| !p.dead && p.plant_col == grid_x && p.base.row == grid_y && p.seed_type == seed)
                    } } else { false };
                    if !has_match {
                        return;
                    }
                }
            }
        }
        self.spawn_level_award(_grid_x, _grid_y);
    }

    pub fn get_art_challenge_seed(&self, grid_x: i32, grid_y: i32) -> SeedType {
        Self::get_art_challenge_seed_static(self.get_app().game_mode, grid_x, grid_y)
    }

    pub fn get_art_challenge_seed_static(game_mode: GameMode, grid_x: i32, grid_y: i32) -> SeedType {
        if grid_y < 0 || grid_y >= 6 || grid_x < 0 || grid_x >= 9 {
            return SeedType::None;
        }
        let ry = grid_y as usize;
        let cx = grid_x as usize;
        match game_mode {
            GameMode::ChallengeArtChallengeWallnut => ART_CHALLENGE_WALLNUT[ry][cx],
            GameMode::ChallengeArtChallengeSunflower => ART_CHALLENGE_SUNFLOWER[ry][cx],
            GameMode::ChallengeSeeingStars => ART_CHALLENGE_STARFRUIT[ry][cx],
            _ => SeedType::None,
        }
    }

    pub fn plant_added(&mut self, plant: &mut Plant) {
        let app = self.get_app();
        if !app.is_art_challenge() { return; }
        let art_seed = self.get_art_challenge_seed(plant.plant_col, plant.base.row);
        if art_seed != SeedType::None && art_seed == plant.seed_type {
            self.check_for_complete_art_challenge(plant.plant_col, plant.base.row);
        }
    }

    pub fn can_plant_at(&self, grid_x: i32, grid_y: i32, seed_type: SeedType) -> PlantingReason {
        let app = self.get_app();
        if app.is_wallnut_bowling_level() {
            return if grid_x > 2 { PlantingReason::NotPassedLine } else { PlantingReason::Ok };
        } else if app.is_izombie_level() {
            let mut limit = 6;
            let mode = app.game_mode as i32;
            if mode >= GameMode::PuzzleIZombie1 as i32 && mode <= GameMode::PuzzleIZombie5 as i32 {
                limit = 4;
            } else if (mode >= GameMode::PuzzleIZombie6 as i32 && mode <= GameMode::PuzzleIZombie8 as i32)
                || mode == GameMode::PuzzleIZombieEndless as i32
            {
                limit = 5;
            }
            if seed_type == SeedType::ZombieBungee {
                return if grid_x < limit { PlantingReason::Ok } else { PlantingReason::NotHere };
            } else if Self::is_zombie_seed_type(seed_type) != 0 {
                return if grid_x >= limit { PlantingReason::Ok } else { PlantingReason::NotHere };
            }
        } else if app.is_art_challenge() {
            let art_seed = self.get_art_challenge_seed(grid_x, grid_y);
            if art_seed != SeedType::None
                && art_seed != seed_type
                && seed_type != SeedType::Lilypad
                && seed_type != SeedType::Pumpkinshell
            {
                return PlantingReason::NotOnArt;
            }
            if app.game_mode == GameMode::ChallengeArtChallengeWallnut {
                if (grid_x == 4 || grid_x == 6) && grid_y == 1 {
                    return PlantingReason::NotHere;
                }
            }
        } else if app.is_final_boss_level() && grid_x >= 8 {
            return PlantingReason::NotHere;
        }
        PlantingReason::Ok
    }

    pub fn draw_beghouled(&self, g: &mut Graphics) {
        // 对应 C++ Challenge::DrawBeghouled
        let board_ptr = match self.board {
            Some(b) => b,
            None => return,
        };
        let app_ptr = match self.app {
            Some(a) => a,
            None => return,
        };

        for a_grid_y in 0..crate::lawn::board::MAX_GRID_SIZE_Y {
            for a_grid_x in 0..crate::lawn::board::MAX_GRID_SIZE_X {
                if self.beghouled_eated[a_grid_x][a_grid_y] != 0 {
                    let p_x = unsafe { (*board_ptr).grid_to_pixel_x(a_grid_x as i32, a_grid_y as i32) - 8 };
                    let p_y = unsafe { (*board_ptr).grid_to_pixel_y(a_grid_x as i32, a_grid_y as i32) + 40 };
                    let img = unsafe { get_image(&*app_ptr, "IMAGE_CRATER") };
                    if !img.is_null() {
                        unsafe { g.draw_image_cel_rc(&*img, p_x, p_y, 1, 0); }
                    }
                }
            }
        }

        if unsafe { (*app_ptr).game_mode } == GameMode::ChallengeBeghouledTwist {
            // [TRANSLATION_NOTE]: C++ 分支：mChallengeGridX/Y != -1 且 MouseHitTest 非 COIN 时，
            // 用 PvzpScaleRotateTransformMatrix + PvzpBltMatrix 绘制 IMAGE_BEGHOULED_TWIST_OVERLAY
            // 旋转叠加层（主计数器驱动每秒 2π 旋转）；Rust 侧矩阵绘制/Overlay 未接入，暂略
            let _ = (board_ptr, g);
        }
    }

    pub fn beghouled_is_valid_move(&self, from_x: i32, from_y: i32, to_x: i32, to_y: i32, board_state: &BeghouledBoardState) -> i32 {
        if from_x < 0 || from_x > 8 || to_x < 0 || to_x > 8
            || from_y < 0 || from_y > 5 || to_y < 0 || to_y > 5
            || self.beghouled_eated[from_x as usize][from_y as usize] != 0
            || self.beghouled_eated[to_x as usize][to_y as usize] != 0
        {
            return 0;
        }
        let seed_from = board_state.seed_type[from_x as usize][from_y as usize];
        let seed_to = board_state.seed_type[to_x as usize][to_y as usize];
        if seed_from == SeedType::None {
            return 0;
        }
        let mut swapped = board_state.clone();
        swapped.seed_type[from_x as usize][from_y as usize] = seed_to;
        swapped.seed_type[to_x as usize][to_y as usize] = seed_from;
        self.beghouled_board_has_match(&swapped)
    }

    pub fn beghouled_check_for_possible_moves(&self, board_state: &BeghouledBoardState) -> i32 {
        let game_mode = self.get_app().game_mode;
        for row in 0..5 {
            for col in 0..8 {
                if game_mode == GameMode::ChallengeBeghouled {
                    if self.beghouled_is_valid_move(col, row, col + 1, row, board_state) != 0
                        || self.beghouled_is_valid_move(col, row, col, row + 1, board_state) != 0
                    {
                        return 1;
                    }
                } else if game_mode == GameMode::ChallengeBeghouledTwist {
                    if self.beghouled_twist_move_causes_match(col, row, &mut board_state.clone()) != 0 {
                        return 1;
                    }
                } else {
                    return 0;
                }
            }
        }
        0
    }

    pub fn beghouled_check_stuck_state(&mut self) {
        if self.challenge_state != ChallengeState::Normal { return; }
        let board = self.get_board();
        if board.has_level_award_dropped() { return; }
        let mut board_state = BeghouledBoardState { seed_type: [[SeedType::None; 6]; 9] };
        self.load_beghouled_board_state(&mut board_state);
        if self.beghouled_check_for_possible_moves(&board_state) == 0 {
            let board = self.get_board();
            board.display_advice("[ADVICE_BEGHOULED_NO_MOVES]", 2, AdviceType::BeghouledNoMoves);
            self.challenge_state = ChallengeState::BeghouledNoMatches;
            self.challenge_state_counter = 1500;
        }
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
        // 对应 C++ UpdateSlotMachine
        let sun_money;
        let mut should_award = false;
        {
            let board = self.get_board();
            sun_money = board.m_sun_money.max(0).min(2000);
            if sun_money >= 2000 - 100 {
                board.display_advice("[ADVICE_ALMOST_THERE]", 2, AdviceType::AlmostThere);
            }
            if sun_money >= 2000 {
                should_award = true;
                board.clear_advice(AdviceType::None);
            }
            board.m_progress_meter_width = sun_money;
        }
        if should_award {
            self.spawn_level_award(4, 2);
        }

        if self.challenge_state != ChallengeState::SlotMachineRolling {
            let board = self.get_board();
            if !board.has_level_award_dropped() {
                board.display_advice_again("[ADVICE_SLOT_MACHINE_SPIN_AGAIN]", 3, AdviceType::SlotMachineSpinAgain);
            }
        } else {
            let countdown = {
                let board = self.get_board();
                board.seed_bank.first().map_or(i32::MAX, |p| p.slot_machine_countdown)
            };
            if countdown <= 0 {
                self.challenge_state = ChallengeState::Normal;

                let packets: Vec<SeedType> = {
                    let board = self.get_board();
                    board.seed_bank.iter().take(3).map(|p| p.seed_type).collect()
                };
                let p1 = packets.get(0).copied().unwrap_or(SeedType::None);
                let p2 = packets.get(1).copied().unwrap_or(SeedType::None);
                let p3 = packets.get(2).copied().unwrap_or(SeedType::None);
                if p1 != p2 || p2 != p3 {
                    if p1 == p2 || p2 == p3 || p1 == p3 {
                        let win_seed = if p1 == p2 || p1 == p3 { p1 } else { p2 };
                        if win_seed == SeedType::SlotMachineDiamond {
                            let board = self.get_board();
                            board.display_advice("[ADVICE_SLOT_MACHINE_2_DIAMONDS]", 3, AdviceType::None);
                            board.add_coin(360.0, 85.0, CoinType::Diamond, CoinMotion::Coin);
                        } else if win_seed == SeedType::SlotMachineSun {
                            let board = self.get_board();
                            board.display_advice("[ADVICE_SLOT_MACHINE_2_SUNS]", 3, AdviceType::None);
                            for i in 0..4 {
                                board.add_coin((320 + i * 15) as f32, 85.0, CoinType::Sun, CoinMotion::Coin);
                            }
                        } else {
                            let board = self.get_board();
                            board.display_advice("[ADVICE_SLOT_MACHINE_2_OF_A_KIND]", 3, AdviceType::None);
                            board.add_coin(360.0, 85.0, CoinType::UsableSeedPacket, CoinMotion::Coin);
                        }
                    }
                } else {
                    // 三个相同
                    if p1 == SeedType::SlotMachineDiamond {
                        let board = self.get_board();
                        board.display_advice("[ADVICE_SLOT_MACHINE_DIAMOND_JACKPOT]", 3, AdviceType::None);
                        for i in 0..5 {
                            board.add_coin((320 + i * 12) as f32, 85.0, CoinType::Diamond, CoinMotion::Coin);
                        }
                    } else if p1 == SeedType::SlotMachineSun {
                        let board = self.get_board();
                        board.display_advice("[ADVICE_SLOT_MACHINE_SUN_JACKPOT]", 3, AdviceType::None);
                        for i in 0..20 {
                            board.add_coin((320 + i * 3) as f32, 85.0, CoinType::Sun, CoinMotion::Coin);
                        }
                    } else {
                        let board = self.get_board();
                        board.display_advice("[ADVICE_SLOT_MACHINE_3_OF_A_KIND]", 3, AdviceType::None);
                        for i in 0..3 {
                            board.add_coin((320 + i * 20) as f32, 85.0, CoinType::UsableSeedPacket, CoinMotion::Coin);
                        }
                    }
                }
            }
        }
    }

    pub fn draw_slot_machine(&self, _g: &mut Graphics) {
        // 对应 C++ Challenge::DrawSlotMachine：
        // ① mGameScene == SCENE_ZOMBIES_WON 直接返回；
        // ② 复制 Graphics 为 gBoardParent，当 mSlotMachineRollCount < 3 且
        //    mCursorObject->mCursorType == CURSOR_TYPE_NORMAL 且 challengeState !=
        //    STATECHALLENGE_SLOT_MACHINE_ROLLING 且 !HasLevelAwardDropped() 时，
        //    SetColor(GetFlashingColor(mMainCounter, 75)) + SetColorizeImages(true)；
        // ③ gBoardParent.mTransX/Y = mSeedBank->mX/Y - mBoard->mX/Y；
        // ④ ReanimationGet(mReanimChallenge)->Draw(&gBoardParent)。
        // [TRANSLATION_NOTE]: 依赖 Graphics 复制构造（Rust 无 Clone）、CursorObject、
        // GetFlashingColor 与 SeedBank 坐标体系，绘制时一并接入
    }

    pub fn update_tool_tip(&self, x: i32, y: i32) -> i32 {
        // 对应 C++ UpdateToolTip
        if !self.get_app().is_slot_machine_level() {
            return 0;
        }
        let mut hit_result = HitResult { object: None, object_type: GameObjectType::None };
        let slot_machine_handle_rect;
        let cursor_normal;
        let can_take_sun;
        if let Some(board) = self.board { unsafe {
            let b = &*board;
            b.mouse_hit_test(x, y, &mut hit_result);
            cursor_normal = b.cursor_object.cursor_type == CursorType::Normal;
            can_take_sun = b.can_take_sun_money(25);
        } } else {
            return 0;
        }
        slot_machine_handle_rect = self.slot_machine_get_handle_rect();
        if hit_result.object_type != GameObjectType::SlotMachineHandle
            || !cursor_normal
            || self.challenge_state != ChallengeState::Normal
        {
            return 0;
        }
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            if !can_take_sun {
                b.tool_tip.m_warning_text = "[NOT_ENOUGH_SUN]".to_string();
            }
            b.tool_tip.set_label("[SLOT_MACHINE_PULL_TOOLTIP]");
            b.tool_tip.set_position(slot_machine_handle_rect.x + 15, slot_machine_handle_rect.y + 65);
            b.tool_tip.m_visible = true;
            b.tool_tip.m_center = true;
        } }
        1
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

    pub fn beghouled_update_craters(&mut self) {
        let can_clear = self.beghouled_can_clear_crater() != 0;
        let board = self.get_board();
        if board.seed_bank.len() != 5 { return; }
        let seed_packet = &mut board.seed_bank[4];
        seed_packet.set_activate(can_clear);
    }

    pub fn beghouled_clear_crater(&mut self, mut count: i32) {
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            b.clear_advice(AdviceType::None);
        } }
        for grid_x in 0..9 {
            for grid_y in 0..5 {
                if self.beghouled_eated[grid_x][grid_y] != 0 {
                    self.beghouled_eated[grid_x][grid_y] = 0;
                    count -= 1;
                    if count == 0 {
                        self.beghouled_update_craters();
                        return;
                    }
                }
            }
        }
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

    pub fn spawn_level_award(&self, grid_x: i32, grid_y: i32) {
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            if b.has_level_award_dropped() { return; }

            let pos_x = b.grid_to_pixel_x(grid_x, grid_y) + 40;
            let pos_y = b.grid_to_pixel_y(grid_x, grid_y) + 40;
            let app_ptr = match b.app { Some(a) => a, None => return };
            let app = &mut *app_ptr;
            let coin_type = if app.is_first_time_adventure_mode() {
                CoinType::FinalSeedPacket
            } else if app.is_adventure_mode() || app.has_beaten_challenge(app.game_mode) {
                CoinType::AwardMoneyBag
            } else {
                CoinType::Trophy
            };

            b.m_level_award_spawned = true;
            app.board_result = BoardResult::Won;
            app.play_foley(crate::todlib::tod_foley::FoleyType::SpawnSun as i32);
            b.add_coin(pos_x as f32, pos_y as f32, coin_type, CoinMotion::Coin);
            // AddPvzpParticle(PARTICLE_SCREEN_FLASH) 暂未接入粒子系统

            if app.game_mode == GameMode::ChallengeZombiquarium {
                if let Some(coin) = b.coins.last_mut() {
                    coin.collect();
                }
            } else if !app.is_izombie_level() {
                for zombie in b.zombies.iter_mut() {
                    if zombie.dead { continue; }
                    if !zombie.is_dead_or_dying() {
                        zombie.take_damage(1800, 0);
                    }
                }
            }
        } }
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
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            for item in b.grid_items.iter_mut() {
                if item.dead { continue; }
                if item.is_open_portal() {
                    let mut p = std::ptr::read(item as *const GridItem);
                    self.update_portal(&mut p);
                    std::ptr::write(item as *mut GridItem, p);
                }
            }
        } }

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

    pub fn update_portal(&mut self, portal: &mut GridItem) {
        let other_portal = match self.get_other_portal(portal) {
            Some(p) => p,
            None => return,
        };
        let board_ptr = match self.board {
            Some(b) => b,
            None => return,
        };
        unsafe {
            let board = &mut *board_ptr;
            let portal_grid_y = portal.grid_y;
            let portal_grid_x = portal.grid_x;
            let other_grid_x = (*other_portal).grid_x;
            let other_grid_y = (*other_portal).grid_y;

            // 僵尸穿过传送门
            for zombie in board.zombies.iter_mut() {
                if zombie.dead { continue; }
                if zombie.base.row == portal_grid_y && zombie.last_portal_x != portal_grid_x {
                    let zombie_rect = zombie.get_zombie_rect();
                    let zombie_x = zombie_rect.x + zombie_rect.width / 2;
                    let portal_x = portal_grid_x * 80 + 25;
                    if (zombie_x - portal_x).abs() <= 45 {
                        let mut diff_x = zombie_x - zombie.pos_x as i32;
                        if zombie.is_walking_backwards() { diff_x -= 60; }
                        zombie.pos_x = (other_grid_x * 80 - diff_x) as f32;
                        zombie.set_row(other_grid_y);
                        zombie.pos_y = zombie.get_pos_y_based_on_row(other_grid_y);
                        zombie.last_portal_x = other_grid_x;
                    }
                }
            }

            // 子弹穿过传送门
            for projectile in board.projectiles.iter_mut() {
                if projectile.dead { continue; }
                if projectile.motion == crate::lawn::projectile::ProjectileMotion::Straight
                    && projectile.base.row == portal_grid_y
                    && projectile.last_portal_x != portal_grid_x
                {
                    let proj_rect = projectile.get_projectile_rect();
                    let proj_x = proj_rect.x + proj_rect.width / 2;
                    let portal_x = portal_grid_x * 80 + 55;
                    if (proj_x - portal_x).abs() <= 40 {
                        let delta_y = (other_grid_y - portal_grid_y) * 100;
                        projectile.pos_x += (other_grid_x * 80 - proj_x + 60) as f32;
                        projectile.base.row = other_grid_y;
                        projectile.pos_y += delta_y as f32;
                        projectile.shadow_y += delta_y as f32;
                        projectile.last_portal_x = other_grid_x;
                        projectile.base.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_PROJECTILE, other_grid_y, 0);
                    }
                }
            }

            // 割草机穿过传送门
            for mower in board.lawn_mowers.iter_mut() {
                if mower.dead { continue; }
                if mower.mower_state == LawnMowerState::Triggered
                    && mower.base.row == portal_grid_y
                    && mower.last_portal_x != portal_grid_x
                {
                    let mower_x = mower.pos_x + 45.0;
                    let portal_x = (portal_grid_x * 80 + 25) as f32;
                    if (mower_x - portal_x).abs() <= 20.0 {
                        mower.pos_x = (other_grid_x * 80 + 25) as f32;
                        mower.base.row = other_grid_y;
                        mower.pos_y = ((other_grid_y - portal_grid_y) * 100) as f32;
                        mower.last_portal_x = other_grid_x;
                        mower.base.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_LAWN_MOWER, other_grid_y, 0);
                        mower.update();
                    }
                }
            }
        }
    }

    pub fn portal_combat_row_spawn_weight(&self, grid_y: i32) -> f32 {
        if self.get_portal_distance_to_mower(grid_y) < 5 {
            return 0.01;
        }
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.is_open_portal() && item.grid_y == grid_y {
                    return 1.0;
                }
            }
        } }
        0.2
    }

    pub fn can_target_zombie_with_portals(&self, plant: &Plant, zombie: &Zombie) -> i32 {
        let mut grid_x = plant.plant_col;
        let mut grid_y = plant.base.row;
        for _ in 0..3 {
            let portal = self.get_portal_to_right(grid_x, grid_y);
            if grid_y == zombie.base.row {
                let range_left = grid_x * 80;
                let range_right = if let Some(p) = portal { unsafe { (*p).grid_x * 80 } } else { 800 };
                if zombie.pos_x > range_left as f32 && zombie.pos_x < range_right as f32 {
                    return 1;
                }
            }
            if let Some(p) = portal {
                let other_portal = self.get_other_portal(unsafe { &*p });
                if let Some(op) = other_portal {
                    grid_x = unsafe { (*op).grid_x };
                    grid_y = unsafe { (*op).grid_y };
                    continue;
                }
            }
            break;
        }
        0
    }

    pub fn get_portal_to_right(&self, grid_x: i32, grid_y: i32) -> Option<*mut GridItem> {
        let mut record: Option<*mut GridItem> = None;
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.is_open_portal() && item.grid_x > grid_x && item.grid_y == grid_y {
                    if record.is_none() || unsafe { &*record.unwrap() }.grid_x > item.grid_x {
                        record = Some(item as *const _ as *mut GridItem);
                    }
                }
            }
        } }
        record
    }

    pub fn get_portal_at(&self, grid_x: i32, grid_y: i32) -> Option<*mut GridItem> {
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.grid_x == grid_x && item.grid_y == grid_y && item.is_open_portal() {
                    return Some(item as *const _ as *mut GridItem);
                }
            }
        } }
        None
    }

    pub fn move_a_portal(&mut self) {
        // 收集所有打开的传送门
        let mut portal_picks: Vec<*mut GridItem> = Vec::new();
        if let Some(board) = self.board { unsafe {
            for item in &mut (*board).grid_items {
                if !item.dead && item.is_open_portal() {
                    portal_picks.push(item as *mut GridItem);
                }
            }
        } }
        if portal_picks.is_empty() { return; }

        let portal = portal_picks[crate::framework::common::rand_range(portal_picks.len() as i32) as usize];
        let other_portal = self.get_other_portal(unsafe { &*portal });
        let other_portal = match other_portal { Some(p) => p, None => return };

        let mut grid_array: Vec<crate::todlib::tod_common::TodWeightedGridArray> = Vec::new();
        if let Some(board) = self.board { unsafe {
            let b = &*board;
            let other_x = (*other_portal).grid_x;
            let other_y = (*other_portal).grid_y;
            for grid_x in 0..10 {
                for grid_y in 0..5 {
                    if self.get_portal_at(grid_x, grid_y).is_none() && other_x != grid_x && other_y != grid_y {
                        grid_array.push(crate::todlib::tod_common::TodWeightedGridArray { x: grid_x, y: grid_y, weight: 1 });
                    }
                }
            }
        } }

        let grid_array_len = grid_array.len();
        let pick = crate::todlib::tod_common::tod_pick_from_weighted_grid_array(&mut grid_array, grid_array_len);
        let pick = match pick { Some(p) => p, None => return };
        let grid_x = grid_array[pick].x;
        let grid_y = grid_array[pick].y;

        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            let portal_type = (*portal).grid_item_type;
            let mut new_portal = GridItem::new();
            new_portal.grid_item_type = portal_type;
            new_portal.grid_x = grid_x;
            new_portal.grid_y = grid_y;
            new_portal.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_PARTICLE, grid_y, 0);
            new_portal.open_portal();
            b.grid_items.push(new_portal);
            (*portal).close_portal();
        } }
    }

    pub fn get_portal_distance_to_mower(&self, grid_y: i32) -> i32 {
        let mut grid_x = 10;
        let mut a_grid_y = grid_y;
        let mut distance = 0;
        while distance < 40 {
            let portal = self.get_portal_to_left(grid_x, a_grid_y);
            let portal = match portal { Some(p) => p, None => { distance += grid_x; break; } };
            let other_portal = self.get_other_portal(unsafe { &*portal });
            let other_portal = match other_portal { Some(p) => p, None => break };
            distance += grid_x - unsafe { (*portal).grid_x };
            grid_x = unsafe { (*other_portal).grid_x };
            a_grid_y = unsafe { (*other_portal).grid_y };
        }
        distance
    }

    pub fn get_portal_to_left(&self, grid_x: i32, grid_y: i32) -> Option<*mut GridItem> {
        let mut record: Option<*mut GridItem> = None;
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.is_open_portal() && item.grid_x < grid_x && item.grid_y == grid_y {
                    if record.is_none() || unsafe { &*record.unwrap() }.grid_x < item.grid_x {
                        record = Some(item as *const _ as *mut GridItem);
                    }
                }
            }
        } }
        record
    }

    pub fn get_portal_left_right(&self, grid_x: i32, grid_y: i32, to_left: bool) -> Option<*mut GridItem> {
        let mut record: Option<*mut GridItem> = None;
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if item.dead { continue; }
                let portal_x = item.grid_x;
                if portal_x == grid_x { continue; }
                let is_dir = (portal_x > grid_x) as i32 ^ (to_left as i32);
                if item.is_open_portal() && is_dir != 0 && item.grid_y == grid_y {
                    let is_cls = if let Some(r) = record {
                        let r_x = (*r).grid_x;
                        (r_x > portal_x) as i32 ^ (to_left as i32)
                    } else { 1 };
                    if record.is_none() || is_cls != 0 {
                        record = Some(item as *const _ as *mut GridItem);
                    }
                }
            }
        } }
        record
    }

    pub fn beghouled_packet_clicked(&mut self, seed_packet: &mut SeedPacket) {
        let packet_type = seed_packet.seed_type;
        let cost = {
            let board = self.get_board();
            let cost = board.get_current_plant_cost(packet_type, SeedType::None);
            if !board.can_take_sun_money(cost) {
                return;
            }
            cost
        };

        // 升级索引：SEED_REPEATER→0, SEED_FUMESHROOM→1, SEED_TALLNUT→2
        let upgrade = match packet_type {
            SeedType::Repeater => 0,
            SeedType::Fumeshroom => 1,
            SeedType::Tallnut => 2,
            _ => -1,
        };

        if packet_type == SeedType::BeghouledButtonShuffle {
            if self.challenge_state == ChallengeState::BeghouledFalling
                || self.challenge_state == ChallengeState::BeghouledMoving
            {
                return;
            }
            self.beghouled_shuffle();
        } else if packet_type == SeedType::BeghouledButtonCrater {
            if self.beghouled_can_clear_crater() == 0
                || self.challenge_state == ChallengeState::BeghouledFalling
                || self.challenge_state == ChallengeState::BeghouledMoving
            {
                return;
            }
            self.beghouled_clear_crater(1);
            self.beghouled_start_falling(ChallengeState::BeghouledFalling);
        } else if upgrade != -1 && self.beghouled_purchased_upgrade[upgrade as usize] == 0 {
            self.beghouled_purchased_upgrade[upgrade as usize] = 1;
            let primary = match upgrade {
                0 => SeedType::Peashooter,
                1 => SeedType::Puffshroom,
                _ => SeedType::Wallnut,
            };
            let to_replace: Vec<(i32, i32)> = {
                let board = self.get_board();
                board.plants.iter()
                    .filter(|p| !p.dead && p.seed_type == primary)
                    .map(|p| (p.plant_col, p.base.row))
                    .collect()
            };
            for (col, row) in to_replace {
                let board = self.get_board();
                if let Some(plant) = board.plants.iter_mut().find(|p| !p.dead && p.plant_col == col && p.base.row == row) {
                    plant.die();
                }
                board.add_plant(col, row, packet_type, SeedType::None);
            }
            seed_packet.set_activate(false);
        }
        let board = self.get_board();
        board.take_sun_money(cost);
    }

    pub fn beghouled_shuffle(&mut self) {
        let board = self.get_board();
        board.clear_advice(AdviceType::None);
        for plant in &mut board.plants {
            if !plant.dead {
                plant.die();
            }
        }
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

    pub fn zombiquarium_spawn_snorkle(&mut self) -> Option<*mut Zombie> {
        let board = self.get_board();
        board.add_zombie_in_row(ZombieType::Snorkel, 0, 0);
        let idx = board.zombies.len() - 1;
        let zombie = &mut board.zombies[idx];
        zombie.pos_x = crate::framework::common::rand_range(600) as f32 + 50.0;
        zombie.pos_y = crate::framework::common::rand_range(300) as f32 + 100.0;
        Some(zombie as *mut Zombie)
    }

    pub fn zombiquarium_packet_clicked(&mut self, seed_packet: &mut SeedPacket) {
        let cost = {
            let board = self.get_board();
            board.get_current_plant_cost(seed_packet.seed_type, SeedType::None)
        };
        let can_afford = self.get_board().can_take_sun_money(cost);
        if !can_afford { return; }

        if seed_packet.seed_type == SeedType::ZombiquariumSnorkle {
            if self.get_board().count_zombies_on_screen() > 100 { return; }
            if self.get_board().m_tutorial_state == TutorialState::ZombiquariumBuySnorkel {
                let board = self.get_board();
                board.clear_advice(AdviceType::ZombiquariumBuySnorkel);
                board.m_tutorial_state = TutorialState::ZombiquariumBoughtSnorkel;
            }
            let _zombie = self.zombiquarium_spawn_snorkle();
        } else if seed_packet.seed_type == SeedType::ZombiquariumTrophy {
            self.spawn_level_award(2, 0);
            let board = self.get_board();
            board.clear_advice(AdviceType::None);
        }
        let board = self.get_board();
        board.take_sun_money(cost);
    }

    pub fn zombiquarium_drop_brain(&mut self, x: i32, y: i32) {
        let board = self.get_board();
        board.clear_advice(AdviceType::ZombiquariumClickToFeed);
        let mut brain = crate::lawn::grid_item::GridItem::new();
        brain.grid_item_type = crate::lawn::grid_item::GridItemType::None;
        brain.render_order = 400000;
        brain.grid_x = 0;
        brain.grid_y = 0;
        brain.counter = 0;
        brain.pos_x = (x - 15) as f32;
        brain.pos_y = (y - 15) as f32;
        board.grid_items.push(brain);
        // PlaySample(SOUND_TAP) 未接入
    }

    pub fn zombiquarium_mouse_down(&mut self, x: i32, y: i32) {
        if x < 80 || x > 720 || y < 90 || y > 430 { return; }

        let brains_count = {
            let board = self.get_board();
            board.grid_items.iter().filter(|item| !item.dead && item.grid_item_type == crate::lawn::grid_item::GridItemType::None).count() as i32
        };
        if brains_count < 3 {
            let can_pay = self.get_board().take_sun_money(5);
            if can_pay {
                self.zombiquarium_drop_brain(x, y);
            }
        }
    }

    pub fn zombiquarium_update(&mut self) {
        {
            let board = self.get_board();
            if board.zombies.is_empty() && !board.has_level_award_dropped() {
                board.zombies_won();
                return;
            }
        }
        let board = self.get_board();
        let score = board.m_sun_money.max(0).min(1000);
        board.m_progress_meter_width = score;
        if score >= 1000 - 100 {
            board.display_advice("[ALMOST_THERE]", 3, AdviceType::None);
        }
        if score >= 110 && board.m_tutorial_state == TutorialState::Off {
            board.m_tutorial_state = TutorialState::ZombiquariumBuySnorkel;
            board.display_advice("[ADVICE_ZOMBIQUARIUM_BUY_SNORKEL]", 3, AdviceType::ZombiquariumBuySnorkel);
        } else if score < 100 && board.m_tutorial_state == TutorialState::ZombiquariumBuySnorkel {
            board.clear_advice(AdviceType::ZombiquariumBuySnorkel);
            board.m_tutorial_state = TutorialState::Off;
        }
        if score >= 1000 && board.m_tutorial_state == TutorialState::ZombiquariumBoughtSnorkel {
            board.m_tutorial_state = TutorialState::ZombiquariumClickTrophy;
            board.display_advice("[ADVICE_ZOMBIQUARIUM_CLICK_TROPHY]", 3, AdviceType::ZombiquariumClickTrophy);
        } else if score < 1000 && board.m_tutorial_state == TutorialState::ZombiquariumClickTrophy {
            board.clear_advice(AdviceType::ZombiquariumClickTrophy);
            board.m_tutorial_state = TutorialState::ZombiquariumBoughtSnorkel;
        }

        let board = self.get_board();
        for item in &mut board.grid_items {
            if item.dead { continue; }
            if item.grid_item_type == crate::lawn::grid_item::GridItemType::None {
                item.counter += 1;
                item.pos_y += 0.15;
                if item.pos_y >= 500.0 {
                    item.grid_item_die();
                }
            }
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

    pub fn scary_potter_place_pot(&mut self, pot_type: ScaryPotType, zombie_type: ZombieType, seed_type: SeedType, mut count: i32, grid_array: &mut [crate::todlib::tod_common::TodWeightedGridArray], grid_array_count: i32) {
        while count > 0 {
            let pick = crate::todlib::tod_common::tod_pick_from_weighted_grid_array(grid_array, grid_array_count as usize);
            let pick = match pick { Some(p) => p, None => break };
            let gx = grid_array[pick].x;
            let gy = grid_array[pick].y;
            grid_array[pick].weight = 0;

            let board = self.get_board();
            let mut scary_pot = crate::lawn::grid_item::GridItem::new();
            scary_pot.grid_item_type = crate::lawn::grid_item::GridItemType::ScaryPot;
            scary_pot.grid_item_state = GridItemState::ScaryPotQuestion;
            scary_pot.grid_x = gx;
            scary_pot.grid_y = gy;
            scary_pot.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_PLANT, gy, 0);
            scary_pot.zombie_type = zombie_type;
            scary_pot.seed_type = seed_type;
            scary_pot.scary_pot_type = pot_type;
            if pot_type == ScaryPotType::Sun {
                scary_pot.sun_count = crate::framework::common::rand_range(3) + 1;
            }
            board.grid_items.push(scary_pot);
            count -= 1;
        }
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
        let grid_x = scary_pot.grid_x;
        let grid_y = scary_pot.grid_y;
        let pot_type = scary_pot.scary_pot_type;
        let pot_state = scary_pot.grid_item_state;
        let zombie_type = scary_pot.zombie_type;
        let seed_type = scary_pot.seed_type;
        let sun_count = self.scary_potter_count_sun_in_pot(scary_pot);

        let (a_pos_x, a_pos_y) = {
            let board = self.get_board();
            (board.grid_to_pixel_x(grid_x, grid_y), board.grid_to_pixel_y(grid_x, grid_y))
        };
        let board = self.get_board();
        match pot_type {
            ScaryPotType::Seed => {
                board.add_coin((a_pos_x + 20) as f32, a_pos_y as f32, CoinType::UsableSeedPacket, CoinMotion::FromPlant);
                // 简化：mUsableSeedType 未单独存储
            }
            ScaryPotType::Zombie => {
                board.add_zombie_in_row(zombie_type, grid_y, 0);
            }
            ScaryPotType::Sun => {
                for i in 0..sun_count {
                    board.add_coin((a_pos_x + 15 * i) as f32, a_pos_y as f32, CoinType::Sun, CoinMotion::FromPlant);
                }
            }
            ScaryPotType::None => {}
        }
        let _ = pot_state;

        scary_pot.grid_item_die();
        if self.scary_potter_is_completed() != 0 {
            if self.get_app().is_scary_potter_level() {
                let board = self.get_board();
                if !board.is_final_scary_potter_stage() {
                    self.puzzle_phase_complete(grid_x, grid_y);
                } else {
                    self.spawn_level_award(grid_x, grid_y);
                }
            } else {
                self.spawn_level_award(grid_x, grid_y);
            }
        }
    }

    pub fn scary_potter_jack_explode(&mut self, pos_x: i32, pos_y: i32) {
        let (a_grid_x, a_grid_y) = {
            let board = self.get_board();
            (board.pixel_to_grid_x(pos_x, pos_y), board.pixel_to_grid_y(pos_x, pos_y))
        };
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            let items = &b.grid_items;
            let mut targets = Vec::new();
            for item in items {
                if item.dead { continue; }
                if item.grid_item_type == crate::lawn::grid_item::GridItemType::ScaryPot
                    && item.grid_x >= a_grid_x - 1 && item.grid_x <= a_grid_x + 1
                    && item.grid_y >= a_grid_y - 1 && item.grid_y <= a_grid_y + 1
                {
                    targets.push(item as *const _ as *mut GridItem);
                }
            }
            for ptr in targets {
                let item = &mut *ptr;
                self.scary_potter_open_pot(item);
            }
        } }
    }

    pub fn scary_potter_is_completed(&self) -> i32 {
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.grid_item_type == crate::lawn::grid_item::GridItemType::ScaryPot { return 0; }
            }
        } }
        if let Some(board) = self.board { unsafe {
            if (*board).are_enemy_zombies_on_screen() { return 0; }
        } }
        1
    }

    pub fn scary_potter_change_pot_type(&mut self, pot_type: GridItemState, mut count: i32) {
        let mut pot_picks: Vec<*mut GridItem> = Vec::new();
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            for item in &mut b.grid_items {
                if item.dead { continue; }
                if item.grid_item_type != crate::lawn::grid_item::GridItemType::ScaryPot { continue; }
                let matches = if pot_type == GridItemState::ScaryPotLeaf {
                    item.scary_pot_type == ScaryPotType::Seed
                } else if pot_type == GridItemState::ScaryPotZombie {
                    item.zombie_type == ZombieType::Gargantuar
                } else {
                    false
                };
                if matches {
                    pot_picks.push(item as *mut GridItem);
                }
            }
        } }
        if pot_picks.is_empty() { return; }
        if count > pot_picks.len() as i32 { count = pot_picks.len() as i32; }
        for _ in 0..count {
            let idx = crate::framework::common::rand_range(pot_picks.len() as i32) as usize;
            unsafe { (*pot_picks[idx]).grid_item_state = pot_type; }
        }
    }

    pub fn scary_potter_populate(&mut self) {
        self.scary_potter_pots = self.scary_potter_count_pots();
    }

    pub fn scary_potter_dont_place_in_col(&self, col: i32, grid_array: &mut [crate::todlib::tod_common::TodWeightedGridArray], grid_array_count: i32) {
        for i in 0..grid_array_count {
            if grid_array[i as usize].x == col {
                grid_array[i as usize].weight = 0;
            }
        }
    }

    pub fn scary_potter_fill_column_with_plant(&mut self, col: i32, seed_type: SeedType, grid_array: &mut [crate::todlib::tod_common::TodWeightedGridArray], grid_array_count: i32) {
        self.scary_potter_dont_place_in_col(col, grid_array, grid_array_count);
        let board = self.get_board();
        for row in 0..5 {
            board.add_plant(col, row, seed_type, SeedType::None);
        }
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
            SeedType::ZombieNormal => ZombieType::Normal,
            SeedType::ZombieTrafficCone => ZombieType::TrafficCone,
            SeedType::ZombiePolevaulter => ZombieType::Polevaulter,
            SeedType::ZombiePail => ZombieType::Pail,
            SeedType::ZombieLadder => ZombieType::Ladder,
            SeedType::ZombieDigger => ZombieType::Digger,
            SeedType::ZombieBungee => ZombieType::Bungee,
            SeedType::ZombieFootball => ZombieType::Football,
            SeedType::ZombieBalloon => ZombieType::Balloon,
            SeedType::ZombieScreenDoor => ZombieType::Door,
            SeedType::Zomboni => ZombieType::Zamboni,
            SeedType::ZombiePogo => ZombieType::Pogo,
            SeedType::ZombieDancer => ZombieType::Dancer,
            SeedType::ZombieGargantuar => ZombieType::Gargantuar,
            SeedType::ZombieImp => ZombieType::Imp,
            _ => ZombieType::Invalid,
        }
    }

    pub fn is_zombie_seed_type(seed_type: SeedType) -> i32 {
        matches!(seed_type,
            SeedType::ZombiquariumSnorkle | SeedType::ZombiquariumTrophy
            | SeedType::ZombieNormal | SeedType::ZombieTrafficCone
            | SeedType::ZombiePolevaulter | SeedType::ZombiePail
            | SeedType::ZombieLadder | SeedType::ZombieDigger
            | SeedType::ZombieBungee | SeedType::ZombieFootball
            | SeedType::ZombieBalloon | SeedType::ZombieScreenDoor
            | SeedType::Zomboni | SeedType::ZombiePogo
            | SeedType::ZombieDancer | SeedType::ZombieGargantuar
            | SeedType::ZombieImp
        ).then(|| 1).unwrap_or(0)
    }

    pub fn i_zombie_mouse_down_with_zombie(&mut self, x: i32, y: i32, click_count: i32) {
        if click_count >= 0 {
            let seed_type = {
                let board = self.get_board();
                board.cursor_object.seed_type
            };
            let (grid_x, grid_y) = {
                let board = self.get_board();
                (board.planting_pixel_to_grid_x(x, y, seed_type), board.planting_pixel_to_grid_y(x, y, seed_type))
            };
            if grid_x != -1 && grid_y != -1 && click_count != 0 {
                if self.can_plant_at(grid_x, grid_y, seed_type) == PlantingReason::Ok {
                    let can_afford = {
                        let board = self.get_board();
                        board.can_take_sun_money(board.get_current_plant_cost(seed_type, SeedType::None))
                    };
                    if can_afford {
                        {
                            let board = self.get_board();
                            board.clear_advice(AdviceType::IZombieLeftOfLine);
                            board.clear_advice(AdviceType::IZombieNotPassedLine);
                        }
                        let zombie_type = Self::i_zombie_seed_type_to_zombie_type(seed_type);
                        self.i_zombie_place_zombie(zombie_type, grid_x, grid_y);
                        let seed_bank_index = {
                            let board = self.get_board();
                            board.cursor_object.seed_bank_index
                        };
                        if seed_bank_index >= 0 {
                            let board = self.get_board();
                            if let Some(packet) = board.seed_bank.get_mut(seed_bank_index as usize) {
                                packet.was_planted();
                            }
                        }
                        let board = self.get_board();
                        board.take_sun_money(board.get_current_plant_cost(seed_type, SeedType::None));
                        board.clear_cursor();
                    }
                } else {
                    let board = self.get_board();
                    board.clear_advice(AdviceType::None);
                    if seed_type == SeedType::ZombieBungee {
                        board.display_advice("[ADVICE_I_ZOMBIE_LEFT_OF_LINE]", 5, AdviceType::IZombieLeftOfLine);
                    } else {
                        board.display_advice("[ADVICE_I_ZOMBIE_NOT_PASSED_LINE]", 5, AdviceType::IZombieNotPassedLine);
                    }
                }
                return;
            }
        }
        let board = self.get_board();
        board.refresh_seed_packet_from_cursor();
    }

    pub fn i_zombie_start(&mut self) {
        let board = self.get_board();
        board.display_advice("[I_ZOMBIE_EAT_ALL_BRAINS]", 1, AdviceType::None);
    }

    pub fn i_zombie_place_plants(&mut self, seed_type: SeedType, mut count: i32, grid_y: i32) {
        // 对应 C++ IZombiePlacePlants
        let game_mode = self.get_app().game_mode;
        let mut columns = 6;
        let mode = game_mode as i32;
        if mode >= GameMode::PuzzleIZombie1 as i32 && mode <= GameMode::PuzzleIZombie5 as i32 {
            columns = 4;
        } else if mode != GameMode::PuzzleIZombie9 as i32 {
            columns = 5;
        }

        let (min_grid_y, max_grid_y) = if grid_y == -1 {
            (0, 4)
        } else {
            (grid_y, grid_y)
        };

        let mut grid_array: Vec<crate::todlib::tod_common::TodWeightedGridArray> = Vec::new();
        for row in min_grid_y..=max_grid_y {
            for col in 0..columns {
                let can_plant = {
                    let board = self.get_board();
                    board.can_plant_at(col, row, seed_type) == PlantingReason::Ok
                };
                if !can_plant { continue; }
                // 坚果和火炬木只出现在最右 3 列
                if (seed_type != SeedType::Wallnut && seed_type != SeedType::Torchwood) || columns - col <= 3 {
                    grid_array.push(crate::todlib::tod_common::TodWeightedGridArray { x: col, y: row, weight: 1 });
                }
            }
        }
        let array_count = grid_array.len();
        if count > array_count as i32 {
            count = array_count as i32;
        }
        for _ in 0..count {
            let pick = crate::todlib::tod_common::tod_pick_from_weighted_grid_array(&mut grid_array, array_count);
            let pick = match pick { Some(p) => p, None => break };
            let gx = grid_array[pick].x;
            let gy = grid_array[pick].y;
            grid_array[pick].weight = 0;
            self.i_zombie_place_plant_in_square(seed_type, gx, gy);
        }
    }

    pub fn i_zombie_update(&mut self) {
        let board = self.get_board();
        if board.zombies.len() == 0 && board.m_sun_money < 50 && !board.has_level_award_dropped() {
            board.zombies_won();
        }
    }

    pub fn i_zombie_draw_plant(&self, g: &mut Graphics, plant: &Plant) {
        // 对应 C++ Challenge::IZombieDrawPlant：多重描边阴影绘制（白/褐/浅褐/肤色 4 层偏移）
        let app_ptr = match self.app {
            Some(a) => a,
            None => return,
        };
        let reanim = unsafe { (&mut *app_ptr).reanimation_get_mut(plant.body_reanim_id) };
        if let Some(reanim) = reanim {
            self.i_zombie_set_plant_filter_effect(plant, crate::todlib::filter_effect::FilterEffectType::White);
            g.set_colorize_images(true);

            let a_offset_x = g.trans_x;
            let a_offset_y = g.trans_y;
            g.trans_x = a_offset_x + 4.0;
            g.trans_y = a_offset_y + 4.0;
            g.set_color(&crate::framework::color::Color::from_rgb(122, 86, 58));
            reanim.draw_render_group(g, 0);

            g.trans_x = a_offset_x + 2.0;
            g.trans_y = a_offset_y + 2.0;
            g.set_color(&crate::framework::color::Color::from_rgb(171, 135, 107));
            reanim.draw_render_group(g, 0);

            g.trans_x = a_offset_x - 2.0;
            g.trans_y = a_offset_y - 2.0;
            g.set_color(&crate::framework::color::Color::from_rgb(171, 135, 107));
            reanim.draw_render_group(g, 0);

            g.trans_x = a_offset_x;
            g.trans_y = a_offset_y;
            g.set_color(&crate::framework::color::Color::from_rgb(255, 201, 160));
            self.i_zombie_set_plant_filter_effect(plant, crate::todlib::filter_effect::FilterEffectType::None);
            reanim.draw_render_group(g, 0);

            self.i_zombie_set_plant_filter_effect(plant, crate::todlib::filter_effect::FilterEffectType::None);
            g.set_draw_mode(0);
            g.set_colorize_images(false);
        }
    }

    pub fn i_zombie_set_plant_filter_effect(&self, plant: &Plant, filter_effect: crate::todlib::filter_effect::FilterEffectType) {
        // 对应 C++ Challenge::IZombieSetPlantFilterEffect
        let app_ptr = match self.app {
            Some(a) => a,
            None => return,
        };
        let reanim_ids = [
            plant.body_reanim_id,
            plant.head_reanim_id,
            plant.head_reanim_id2,
            plant.head_reanim_id3,
        ];
        for reanim_id in reanim_ids {
            if let Some(reanim) = unsafe { (&mut *app_ptr).reanimation_get_mut(reanim_id) } {
                // [TRANSLATION_NOTE]: C++ 中 aReanim->mFilterEffect = theFilterEffect；
                // Rust 侧 Reanimation 无 m_filter_effect 字段（filter_effect 系统未接入），暂略
                let _ = reanim;
                let _ = filter_effect;
            }
        }
    }

    pub fn scary_potter_count_sun_in_pot(&self, scary_pot: &GridItem) -> i32 {
        scary_pot.sun_count
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
        // 对应 C++ Challenge::DrawRain：
        // ① mBoard->mCutScene->IsBeforePreloading() 或 !Is3DAccelerated() 直接返回；
        // ② 计算雨滴/雨丝粒子并 DrawParticle 渲染，受 mBoard->mMainCounter 驱动。
        // [TRANSLATION_NOTE]: 依赖 3D 加速标志与粒子系统（ParticleSystem::DrawParticle），
        // Rust 侧 3D/粒子绘制未接入，暂保留主计数器驱动的粒子生成骨架
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
                if item.grid_item_type == crate::lawn::grid_item::GridItemType::Squirrel
                    && item.grid_item_state != GridItemState::SquirrelCaught
                    && item.grid_item_state != GridItemState::SquirrelZombie
                {
                    count += 1;
                }
            }
        } }
        count
    }

    pub fn squirrel_start(&mut self) {
        let mut picks: Vec<crate::todlib::tod_common::TodWeightedGridArray> = Vec::new();
        for col in 0..9 {
            for row in 0..5 {
                picks.push(crate::todlib::tod_common::TodWeightedGridArray { x: col, y: row, weight: 1 });
            }
        }
        let mut picks_count = picks.len();
        for _ in (0..7).rev() {
            let pick_idx = crate::todlib::tod_common::tod_pick_from_weighted_grid_array(&mut picks, picks_count);
            let pick_idx = match pick_idx { Some(i) => i, None => break };
            let gx = picks[pick_idx].x;
            let gy = picks[pick_idx].y;
            picks[pick_idx].weight = 0;
            let board = self.get_board();
            let mut squirrel = crate::lawn::grid_item::GridItem::new();
            squirrel.grid_item_type = crate::lawn::grid_item::GridItemType::Squirrel;
            squirrel.grid_item_state = GridItemState::SquirrelWaiting;
            squirrel.grid_x = gx;
            squirrel.grid_y = gy;
            squirrel.counter = 100 + crate::framework::common::rand_range(401);
            squirrel.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_GRAVE_STONE, gy, 1);
            board.grid_items.push(squirrel);
        }
        for pick in picks.iter_mut() {
            if pick.x < 4 {
                pick.weight = 0;
            }
        }
        let zombie_idx = crate::todlib::tod_common::tod_pick_from_weighted_grid_array(&mut picks, picks_count);
        let zombie_idx = match zombie_idx { Some(i) => i, None => return };
        let board = self.get_board();
        let mut zs = crate::lawn::grid_item::GridItem::new();
        zs.grid_item_type = crate::lawn::grid_item::GridItemType::Squirrel;
        zs.grid_item_state = GridItemState::SquirrelZombie;
        zs.grid_x = picks[zombie_idx].x;
        zs.grid_y = picks[zombie_idx].y;
        zs.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_GRAVE_STONE, zs.grid_y, 1);
        board.grid_items.push(zs);
    }

    pub fn squirrel_found(&mut self, squirrel: &mut GridItem) {
        if squirrel.grid_item_state == GridItemState::SquirrelZombie {
            let (gx, gy) = (squirrel.grid_x, squirrel.grid_y);
            let px = {
                let board = self.get_board();
                board.grid_to_pixel_x(gx, gy)
            };
            let board = self.get_board();
            board.add_zombie_in_row(ZombieType::Normal, gy, 0);
            if let Some(zombie) = board.zombies.last_mut() {
                zombie.pos_x = px as f32;
            }
            squirrel.grid_item_die();
            let board = self.get_board();
            board.display_advice("[ADVICE_SQUIRREL_ZOMBIE]", 2, AdviceType::None);
        } else {
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut picks: Vec<crate::todlib::tod_common::TodWeightedGridArray> = Vec::new();
            for (dx, dy) in neighbors {
                let gx = squirrel.grid_x + dx;
                let gy = squirrel.grid_y + dy;
                if let Some(board) = self.board { unsafe {
                    let b = &*board;
                    let has_squirrel = b.grid_items.iter().any(|item| {
                        !item.dead && item.grid_item_type == crate::lawn::grid_item::GridItemType::Squirrel && item.grid_x == gx && item.grid_y == gy
                    });
                    let has_plant = b.plants.iter().any(|p| !p.dead && p.plant_col == gx && p.base.row == gy);
                    if !has_squirrel && has_plant {
                        picks.push(crate::todlib::tod_common::TodWeightedGridArray { x: gx, y: gy, weight: 1 });
                    }
                } }
            }
            if !picks.is_empty() {
                let pick_count = picks.len();
                let pick_idx = crate::todlib::tod_common::tod_pick_from_weighted_grid_array(&mut picks, pick_count);
                let pick_idx = match pick_idx { Some(i) => i, None => return };
                let (gx, gy) = (picks[pick_idx].x, picks[pick_idx].y);
                if gx != squirrel.grid_x {
                    squirrel.grid_item_state = if gx < squirrel.grid_x { GridItemState::SquirrelRunningLeft } else { GridItemState::SquirrelRunningRight };
                } else {
                    squirrel.grid_item_state = if gy < squirrel.grid_y { GridItemState::SquirrelRunningUp } else { GridItemState::SquirrelRunningDown };
                }
                squirrel.counter = 50;
                squirrel.grid_x = gx;
                squirrel.grid_y = gy;
                squirrel.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_GRAVE_STONE, gy, 1);
            } else {
                squirrel.grid_item_state = GridItemState::SquirrelCaught;
                squirrel.counter = 100;
                let remaining = self.squirrel_count_uncaught();
                if remaining > 0 {
                    let board = self.get_board();
                    let msg = crate::lawn::lawn_app::LawnApp::pluralize(remaining, "[ADVICE_SQUIRRELS_ONE_LEFT]", "[ADVICE_SQUIRRELS_LEFT]");
                    board.display_advice(&msg, 2, AdviceType::None);
                } else {
                    let board = self.get_board();
                    board.clear_advice(AdviceType::None);
                    self.spawn_level_award(squirrel.grid_x, squirrel.grid_y);
                }
            }
        }
    }

    pub fn squirrel_peek(&mut self, squirrel: &mut GridItem) {
        squirrel.counter = 50;
        squirrel.grid_item_state = GridItemState::SquirrelPeeking;
    }

    pub fn squirrel_chew(&mut self, squirrel: &mut GridItem) {
        squirrel.counter = 100 + crate::framework::common::rand_range(401);
        let (gx, gy) = (squirrel.grid_x, squirrel.grid_y);
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            if let Some(plant) = b.plants.iter_mut().find(|p| !p.dead && p.plant_col == gx && p.base.row == gy) {
                if plant.eaten_flash_countdown <= 25 {
                    plant.eaten_flash_countdown = 25;
                }
            }
        } }
    }

    pub fn squirrel_update_one(&mut self, squirrel: &mut GridItem) {
        let mut counter = squirrel.counter;
        if counter > 0 {
            counter -= 1;
            squirrel.counter = counter;
        }
        let state = squirrel.grid_item_state;
        if state == GridItemState::SquirrelWaiting || state == GridItemState::SquirrelZombie {
            let (gx, gy) = (squirrel.grid_x, squirrel.grid_y);
            let has_plant = if let Some(board) = self.board { unsafe {
                (*board).plants.iter().any(|p| !p.dead && p.plant_col == gx && p.base.row == gy)
            } } else { false };
            if !has_plant {
                self.squirrel_found(squirrel);
            }
            if counter == 0 {
                if crate::framework::common::rand_range(2) != 0 && state != GridItemState::SquirrelZombie {
                    self.squirrel_peek(squirrel);
                } else {
                    self.squirrel_chew(squirrel);
                }
            }
        }
        let state = squirrel.grid_item_state;
        if (state == GridItemState::SquirrelPeeking
            || state == GridItemState::SquirrelRunningUp
            || state == GridItemState::SquirrelRunningDown
            || state == GridItemState::SquirrelRunningLeft
            || state == GridItemState::SquirrelRunningRight)
            && squirrel.counter == 0
        {
            squirrel.grid_item_state = GridItemState::SquirrelWaiting;
            squirrel.counter = 100 + crate::framework::common::rand_range(401);
        }
        if state == GridItemState::SquirrelCaught && squirrel.counter == 0 {
            squirrel.grid_item_die();
        }
    }

    pub fn i_zombie_setup_plant(&self, plant: &mut Plant) {
        // 对应 C++ Challenge::IZombieSetupPlant
        let app_ptr = match self.app {
            Some(a) => a,
            None => return,
        };
        let reanim_ids = [
            plant.body_reanim_id,
            plant.head_reanim_id,
            plant.head_reanim_id2,
            plant.head_reanim_id3,
        ];
        for reanim_id in reanim_ids {
            if let Some(reanim) = unsafe { (&mut *app_ptr).reanimation_get_mut(reanim_id) } {
                reanim.m_anim_rate = 0.0;
            }
        }

        if plant.seed_type == SeedType::PotatoMine {
            plant.play_body_reanim("anim_armed", crate::todlib::reanimator::ReanimLoopType::Loop, 0, 0.0);
            plant.state = PlantState::PotatoArmed;
        }

        plant.blink_countdown = 0;
        // [TRANSLATION_NOTE]: C++ 中 thePlant->UpdateReanim()；Rust 侧 Plant 无 update_reanim 方法，
        // reanim 更新由全局 reanimator 系统驱动，暂不单独调用
    }

    pub fn update_rain(&mut self) {
        self.rain_counter -= 1;
        if self.rain_counter < 0 {
            self.rain_counter = 15;
        }
    }

    pub fn i_zombie_eat_brain(&mut self, zombie: &mut Zombie) -> i32 {
        let brain = self.i_zombie_get_brain_target(zombie);
        let brain = match brain { Some(b) => b, None => return 0 };

        zombie.start_eating();
        let mut counter = unsafe { (*brain).counter };
        counter -= 1;
        unsafe { (*brain).counter = counter; }
        if counter <= 0 {
            unsafe { (*brain).grid_item_die(); }
            self.i_zombie_score_brain(unsafe { &mut *brain });
        }
        1
    }

    pub fn i_zombie_get_brain_target(&self, zombie: &Zombie) -> Option<*mut GridItem> {
        if zombie.zombie_type == ZombieType::Bungee || zombie.is_walking_backwards() { return None; }
        let mut attack_rect = zombie.get_zombie_attack_rect();
        // 撑杆跳前状态使用更靠左的矩形（对应 C++ PHASE_POLEVAULTER_PRE_VAULT）
        if zombie.zombie_phase == ZombiePhase::PolevaulterPreVault {
            attack_rect = Rect::new(50 + zombie.base.x, 0, 20, 115);
        }
        if zombie.zombie_type == ZombieType::Balloon {
            attack_rect.x += 25;
        }
        if attack_rect.x > 20 { return None; }
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.grid_x == 0 && item.grid_y == zombie.base.row
                    && item.grid_item_type == crate::lawn::grid_item::GridItemType::None
                    && item.grid_item_state != GridItemState::BrainSquished
                {
                    return Some(item as *const _ as *mut GridItem);
                }
            }
        } }
        None
    }

    pub fn i_zombie_place_plant_in_square(&mut self, seed_type: SeedType, grid_x: i32, grid_y: i32) {
        let board = self.get_board();
        if board.can_plant_at(grid_x, grid_y, seed_type) == crate::lawn::game_enums::PlantingReason::Ok {
            board.add_plant(grid_x, grid_y, seed_type, SeedType::None);
        }
    }

    pub fn advance_crazy_dave_dialog(&mut self) {
        // 对应 C++ AdvanceCrazyDaveDialog
        let board = self.get_board();
        if !board.is_scary_potter_dave_talking() {
            return;
        }
        let app_ptr = match self.app { Some(a) => a, None => return };
        unsafe {
            let app = &mut *app_ptr;
            if app.m_crazy_dave_message_index == -1 {
                return;
            }
            if !app.advance_crazy_dave_text() {
                app.crazy_dave_leave();
                return;
            }
            // "Here, I'll give you more vases." || "This should be their last wave."
            if app.m_crazy_dave_message_index == 2702 || app.m_crazy_dave_message_index == 2801 {
                self.scary_potter_populate();
                let board = self.get_board();
                board.place_rake();
            }
        }
    }

    pub fn beghouled_flash_plant(&mut self, mut flash_x: i32, mut flash_y: i32, from_x: i32, from_y: i32, to_x: i32, to_y: i32) {
        if flash_x == from_x && flash_y == from_y {
            flash_x = to_x;
            flash_y = to_y;
        } else if flash_x == to_x && flash_y == to_y {
            flash_x = from_x;
            flash_y = from_y;
        }
        if let Some(board) = self.board { unsafe {
            for plant in (&mut *board).plants.iter_mut() {
                if !plant.dead && plant.plant_col == flash_x && plant.base.row == flash_y {
                    if plant.beghouled_flash_countdown == 0 {
                        plant.beghouled_flash_countdown = 300;
                    }
                    break;
                }
            }
        } }
    }

    pub fn beghouled_flash_a_match(&mut self) {
        let mut board_state = BeghouledBoardState { seed_type: [[SeedType::None; 6]; 9] };
        self.load_beghouled_board_state(&mut board_state);
        let game_mode = self.get_app().game_mode;
        if game_mode == GameMode::ChallengeBeghouled {
            for row in 0..=4 {
                for col in 0..=7 {
                    if (col < 7 && self.beghouled_flash_from_board_state(&mut board_state, col, row, col + 1, row) != 0)
                        || (row < 4 && self.beghouled_flash_from_board_state(&mut board_state, col, row, col, row + 1) != 0)
                    {
                        return;
                    }
                }
            }
        } else if game_mode == GameMode::ChallengeBeghouledTwist {
            for row in 0..=4 {
                for col in 0..=7 {
                    if self.beghouled_twist_flash_match(&board_state, col, row) != 0 {
                        return;
                    }
                }
            }
        }
    }

    pub fn beghouled_flash_from_board_state(&mut self, board_state: &mut BeghouledBoardState, from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> i32 {
        if from_x < 0 || from_x > 8 || from_y < 0 || from_y > 5
            || to_x < 0 || to_x > 8 || to_y < 0 || to_y > 5
        {
            return 0;
        }
        if self.beghouled_eated[from_x as usize][from_y as usize] != 0
            || self.beghouled_eated[to_x as usize][to_y as usize] != 0
        {
            return 0;
        }
        let from_seed = board_state.seed_type[from_x as usize][from_y as usize];
        let to_seed = board_state.seed_type[to_x as usize][to_y as usize];
        board_state.seed_type[from_x as usize][from_y as usize] = to_seed;
        board_state.seed_type[to_x as usize][to_y as usize] = from_seed;

        let mut has_match = 0;
        'outer: for row in 0..5 {
            for col in 0..8 {
                if self.beghouled_horizontal_match_length(col, row, board_state) >= 3 {
                    for i in 0..3 {
                        self.beghouled_flash_plant(col + i, row, from_x, from_y, to_x, to_y);
                    }
                } else if self.beghouled_vertical_match_length(col, row, board_state) >= 3 {
                    for i in 0..3 {
                        self.beghouled_flash_plant(col, row + i, from_x, from_y, to_x, to_y);
                    }
                } else {
                    continue;
                }
                has_match = 1;
                break 'outer;
            }
        }

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

    pub fn whack_a_zombie_place_graves(&mut self, mut grave_count: i32) {
        let mut picks: Vec<crate::todlib::tod_common::TodWeightedGridArray> = Vec::new();
        if let Some(board) = self.board { unsafe {
            let b = &*board;
            for col in 3..9 {
                for row in 0..5 {
                    if !b.can_add_grave_stone_at(col, row) {
                        continue;
                    }
                    let has_plant = b.plants.iter().any(|p| !p.dead && p.plant_col == col && p.base.row == row);
                    picks.push(crate::todlib::tod_common::TodWeightedGridArray {
                        x: col,
                        y: row,
                        weight: if has_plant { 1 } else { 100000 },
                    });
                }
            }
        } }
        let pick_count = picks.len();
        if grave_count > pick_count as i32 {
            grave_count = pick_count as i32;
        }
        if pick_count == 0 || grave_count <= 0 {
            return;
        }
        for _ in 0..grave_count {
            let pick_idx = crate::todlib::tod_common::tod_pick_from_weighted_grid_array(&mut picks, pick_count);
            let pick_idx = match pick_idx { Some(i) => i, None => break };
            let gx = picks[pick_idx].x;
            let gy = picks[pick_idx].y;
            picks[pick_idx].weight = 0;
            let board = self.get_board();
            for plant in &mut board.plants {
                if !plant.dead && plant.plant_col == gx && plant.base.row == gy {
                    plant.die();
                }
            }
            let board = self.get_board();
            board.add_grave_stone(gx, gy);
        }
    }

    pub fn beghouled_twist_square_from_mouse(&self, x: i32, y: i32, grid_x: &mut i32, grid_y: &mut i32) -> i32 {
        *grid_x = -1;
        *grid_y = -1;
        if let Some(board) = self.board { unsafe {
            let b = &*board;
            let gx = b.pixel_to_grid_x(x - 40, y - 40);
            let gy = b.pixel_to_grid_y(x - 40, y - 40);
            if gx == -1 || gy == -1 || gx > 6 || gy > 3 {
                return 0;
            }
            *grid_x = gx;
            *grid_y = gy;
            return 1;
        } }
        0
    }

    pub fn beghouled_twist_valid_move(&self, grid_x: i32, grid_y: i32, board_state: &BeghouledBoardState) -> i32 {
        if grid_y == -1 || grid_x > 6 || grid_y > 3 {
            return 0;
        }
        if board_state.seed_type[grid_x as usize][grid_y as usize] != SeedType::None
            && board_state.seed_type[(grid_x + 1) as usize][grid_y as usize] != SeedType::None
            && board_state.seed_type[grid_x as usize][(grid_y + 1) as usize] != SeedType::None
            && board_state.seed_type[(grid_x + 1) as usize][(grid_y + 1) as usize] != SeedType::None
        {
            1
        } else {
            0
        }
    }

    pub fn beghouled_twist_mouse_down(&mut self, x: i32, y: i32) {
        let mut grid_x = 0;
        let mut grid_y = 0;
        let mut board_state = BeghouledBoardState { seed_type: [[SeedType::None; 6]; 9] };
        self.load_beghouled_board_state(&mut board_state);
        let can_twist = self.beghouled_twist_square_from_mouse(x, y, &mut grid_x, &mut grid_y) != 0
            && self.beghouled_twist_valid_move(grid_x, grid_y, &board_state) != 0;
        if !can_twist {
            return;
        }
        let causes_match = self.beghouled_twist_move_causes_match(grid_x, grid_y, &mut board_state);
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            // 找到四个角上的植物
            let gx = grid_x as usize;
            let gy = grid_y as usize;
            let cols = [gx, gx + 1, gx, gx + 1];
            let rows = [gy, gy, gy + 1, gy + 1];
            let mut plants = Vec::new();
            for i in 0..4 {
                for plant in b.plants.iter_mut() {
                    if !plant.dead && plant.plant_col == cols[i] as i32 && plant.base.row == rows[i] as i32 {
                        plants.push(plant as *mut Plant);
                        break;
                    }
                }
            }
            if plants.len() < 4 { return; }
            if causes_match == 0 {
                // 未形成消除：四个植物向中心略微偏移，播放旋转音效
                unsafe {
                    (*plants[0]).pos_x = b.grid_to_pixel_x((*plants[0]).plant_col, (*plants[0]).base.row) as f32 + 20.0;
                    (*plants[1]).pos_y = b.grid_to_pixel_y((*plants[1]).plant_col, (*plants[1]).base.row) as f32 + 20.0;
                    (*plants[2]).pos_y = b.grid_to_pixel_y((*plants[2]).plant_col, (*plants[2]).base.row) as f32 - 20.0;
                    (*plants[3]).pos_x = b.grid_to_pixel_x((*plants[3]).plant_col, (*plants[3]).base.row) as f32 - 20.0;
                }
                if let Some(app) = b.app {
                    (*app).play_foley(crate::todlib::tod_foley::FoleyType::Floop as i32);
                }
            } else {
                // 形成消除：交换四角的行列位置
                unsafe {
                    (*plants[0]).plant_col += 1;
                    (*plants[0]).base.render_order = (*plants[0]).calc_render_order();
                    (*plants[1]).base.row += 1;
                    (*plants[1]).base.render_order = (*plants[1]).calc_render_order();
                    (*plants[2]).base.row -= 1;
                    (*plants[2]).base.render_order = (*plants[2]).calc_render_order();
                    (*plants[3]).plant_col -= 1;
                    (*plants[3]).base.render_order = (*plants[3]).calc_render_order();
                }
                self.beghouled_start_falling(ChallengeState::BeghouledMoving);
            }
        } }
    }

    pub fn beghouled_twist_move_causes_match(&self, grid_x: i32, grid_y: i32, board_state: &mut BeghouledBoardState) -> i32 {
        if self.beghouled_twist_valid_move(grid_x, grid_y, board_state) == 0 {
            return 0;
        }
        let gx = grid_x as usize;
        let gy = grid_y as usize;
        let seed1 = board_state.seed_type[gx][gy];
        let seed2 = board_state.seed_type[gx + 1][gy];
        let seed3 = board_state.seed_type[gx][gy + 1];
        let seed4 = board_state.seed_type[gx + 1][gy + 1];

        board_state.seed_type[gx + 1][gy] = seed1;
        board_state.seed_type[gx + 1][gy + 1] = seed2;
        board_state.seed_type[gx][gy + 1] = seed4;
        board_state.seed_type[gx][gy] = seed3;

        let has_match = self.beghouled_board_has_match(board_state);

        board_state.seed_type[gx][gy] = seed1;
        board_state.seed_type[gx + 1][gy] = seed2;
        board_state.seed_type[gx][gy + 1] = seed3;
        board_state.seed_type[gx + 1][gy + 1] = seed4;

        has_match
    }

    pub fn beghouled_twist_flash_match(&mut self, board_state: &BeghouledBoardState, grid_x: i32, grid_y: i32) -> i32 {
        let mut copy = board_state.clone();
        if self.beghouled_twist_move_causes_match(grid_x, grid_y, &mut copy) == 0 {
            return 0;
        }
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            for i in 0..4 {
                let col = grid_x + (i % 2);
                let row = grid_y + (i / 2);
                for plant in b.plants.iter_mut() {
                    if !plant.dead && plant.plant_col == col && plant.base.row == row {
                        if plant.beghouled_flash_countdown == 0 {
                            plant.beghouled_flash_countdown = 300;
                        }
                        break;
                    }
                }
            }
        } }
        1
    }

    pub fn beghouled_cancel_match_flashing(&mut self) {
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            for plant in b.plants.iter_mut() {
                if plant.dead { continue; }
                if plant.eaten_flash_countdown >= 25 {
                    plant.eaten_flash_countdown = 25;
                }
            }
        } }
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

    pub fn beghouled_create_plants(&mut self, old_board_state: &BeghouledBoardState, new_board_state: &BeghouledBoardState) {
        for col in 0..9 {
            let mut fall_y = 80;
            for row in (0..6).rev() {
                let seed_type = new_board_state.seed_type[col as usize][row as usize];
                if old_board_state.seed_type[col as usize][row as usize] == SeedType::None && seed_type != SeedType::None {
                    fall_y -= 100;
                    let board = self.get_board();
                    board.add_plant(col, row, seed_type, SeedType::None);
                    if let Some(plant) = board.plants.last_mut() {
                        plant.pos_y = fall_y as f32;
                    }
                    self.beghouled_start_falling(ChallengeState::BeghouledFalling);
                }
            }
        }
    }

    pub fn puzzle_phase_complete(&mut self, grid_x: i32, grid_y: i32) {
        if self.puzzle_is_award_stage() != 0 {
            let hit = crate::framework::common::rand_range(100);
            let coin_type = if hit < 15 {
                if let Some(board) = self.board { unsafe {
                    if let Some(zen) = (*board).app.map_or(None, |app| unsafe { (*app).zen_garden }) {
                        if (*zen).can_drop_potted_plant_loot() { CoinType::AwardPresent } else { CoinType::AwardMoneyBag }
                    } else { CoinType::AwardMoneyBag }
                } } else { CoinType::AwardMoneyBag }
            } else if hit < 30 {
                if let Some(board) = self.board { unsafe {
                    if let Some(zen) = (*board).app.map_or(None, |app| unsafe { (*app).zen_garden }) {
                        if (*zen).can_drop_chocolate() { CoinType::AwardChocolate } else { CoinType::AwardMoneyBag }
                    } else { CoinType::AwardMoneyBag }
                } } else { CoinType::AwardMoneyBag }
            } else {
                CoinType::AwardBagDiamond
            };
            let (pos_x, pos_y) = {
                let board = self.get_board();
                (board.grid_to_pixel_x(grid_x, grid_y) + 40, board.grid_to_pixel_y(grid_x, grid_y) + 40)
            };
            let board = self.get_board();
            board.add_coin(pos_x as f32, pos_y as f32, coin_type, CoinMotion::Coin);
        } else {
            // FadeOutLevel() 尚未在 Board 中翻译
        }
    }

    pub fn puzzle_is_award_stage(&self) -> i32 {
        let app = self.get_app();
        if app.is_adventure_mode() { return 0; }
        let goal = if app.game_mode == GameMode::PuzzleIZombieEndless { 3 }
            else if app.game_mode == GameMode::ScaryPotterEndless { 10 }
            else { 1 };
        if self.survival_stage % goal == 0 { 1 } else { 0 }
    }

    pub fn i_zombie_place_zombie(&mut self, zombie_type: ZombieType, grid_x: i32, grid_y: i32) {
        let pos_x = {
            let board = self.get_board();
            board.grid_to_pixel_x(grid_x, grid_y)
        };
        let board = self.get_board();
        board.add_zombie_in_row(zombie_type, grid_y, 0);
        if let Some(zombie) = board.zombies.last_mut() {
            if zombie_type == ZombieType::Bungee {
                zombie.target_col = grid_x;
                zombie.set_row(grid_y);
                zombie.pos_x = pos_x as f32;
                zombie.pos_y = zombie.get_pos_y_based_on_row(grid_y);
                zombie.base.render_order = crate::lawn::board::make_render_order(RENDER_LAYER_GRAVE_STONE, grid_y, 7);
            } else {
                zombie.pos_x = (pos_x - 30) as f32;
            }
        }
    }

    pub fn whack_a_zombie_update(&mut self) {
        // 对应 C++ WhackAZombieUpdate
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            if b.m_sun_money > 0 && b.m_tutorial_state == TutorialState::Off {
                b.m_tutorial_state = TutorialState::WhackAZombieBeforePickSeed;
                b.m_tutorial_timer = 1500;
            }
            if b.m_tutorial_state == TutorialState::WhackAZombieBeforePickSeed && b.m_tutorial_timer == 0 {
                b.m_tutorial_state = TutorialState::WhackAZombiePickSeed;
                b.m_tutorial_timer = 400;
            }
            if b.m_tutorial_state == TutorialState::WhackAZombiePickSeed && b.m_tutorial_timer == 0 {
                b.m_tutorial_state = TutorialState::WhackAZombieCompleted;
            }
        } }
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

    pub fn tree_of_wisdom_mouse_on(&self, x: i32, y: i32) -> i32 {
        let mut hit_result = HitResult { object: None, object_type: GameObjectType::None };
        if let Some(board) = self.board { unsafe {
            let b = &*board;
            b.mouse_hit_test(x, y, &mut hit_result);
            if hit_result.object_type == GameObjectType::TreeOfWisdom && b.cursor_object.cursor_type == CursorType::TreeFood {
                return 1;
            }
        } }
        0
    }

    pub fn tree_of_wisdom_get_size(&self) -> i32 {
        if let Some(app) = self.app { unsafe {
            let a = &*app;
            let idx = a.get_current_challenge_index();
            if let Some(player) = &a.player_info {
                return player.m_challenge_records.get(idx as usize).copied().unwrap_or(0);
            }
        } }
        0
    }

    pub fn tree_of_wisdom_draw(&self, _g: &mut Graphics) {
        // 对应 C++ Challenge::TreeOfWisdomDraw：
        // ① DrawRenderGroup(0) 绘背景 + 6 个云彩 reanim；
        // ② 根据高度与鼠标悬停设置 mExtraOverlayColor/mEnableExtraOverlayDraw 后
        //    按组绘树干(2)/地面(3)/根系(4)；
        // ③ STATECHALLENGE_TREE_GIVE_WISDOM/BABBLING 时绘制气泡 + 包裹文字；
        // ④ 高度 >= 50 时用 FONT_HOUSEOFTERROR16 + PvzpDrawStringMatrix 绘制尺寸。
        // [TRANSLATION_NOTE]: 依赖 mEnableExtraOverlayDraw/mExtraOverlayColor（Rust
        // Reanimation 已有 m_extra_overlay_* 字段待接）、字体矩阵绘制与 StrFormat/
        // PvzpReplaceNumberString 字符串系统；TreeOfWisdomMouseOn/GetSize 已实现
    }

    pub fn tree_of_wisdom_next_garden(&self) {
        // 对应 C++ TreeOfWisdomNextGarden：TreeOfWisdomLeave + KillBoard + PreNewGame
        // 注意：Rust 中 self 为 &self，无法直接调用 &mut self 的 leave，保留简化
        if let Some(app) = self.app { unsafe {
            let a = &mut *app;
            a.kill_board();
            a.pre_new_game(GameMode::ChallengeZenGarden, false);
        } }
    }

    pub fn tree_of_wisdom_tool_update(&mut self, zen_tool: &mut GridItem) {
        // 对应 C++ TreeOfWisdomToolUpdate：动画播完即成长
        if zen_tool.grid_item_state == GridItemState::ZenToolFertilizer {
            self.tree_of_wisdom_grow();
            zen_tool.grid_item_die();
        }
    }

    pub fn tree_of_wisdom_open_store(&self) {
        // 对应 C++ TreeOfWisdomOpenStore：TreeOfWisdomLeave + ShowStoreScreen
        crate::lawn::lawn_app::LawnApp::show_store_screen(self.app);
    }

    pub fn tree_of_wisdom_leave(&mut self) {
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            let mut tool_items = Vec::new();
            for item in &b.grid_items {
                if !item.dead && item.grid_item_type == crate::lawn::grid_item::GridItemType::ZenTool {
                    tool_items.push(item as *const _ as *mut GridItem);
                }
            }
            for ptr in tool_items {
                let item = &mut *ptr;
                self.tree_of_wisdom_grow();
                item.grid_item_die();
            }
        } }
    }

    pub fn tree_of_wisdom_grow(&mut self) {
        // 对应 C++ TreeOfWisdomGrow：增加智慧树尺寸并切换到成长动画
        if let Some(app) = self.app { unsafe {
            let a = &mut *app;
            let idx = a.get_current_challenge_index();
            if let Some(player) = &mut a.player_info {
                if let Some(record) = player.m_challenge_records.get_mut(idx as usize) {
                    *record += 1;
                }
            }
        } }
        self.challenge_state = ChallengeState::TreeJustGrew;
        self.challenge_state_counter = 120;
    }

    pub fn tree_of_wisdom_tool(&self, mouse_x: i32, mouse_y: i32) {
        if self.tree_of_wisdom_mouse_on(mouse_x, mouse_y) != 0 {
            // TreeOfWisdomFertilize 需要 reanim 系统，简化处理
        }
        if let Some(board) = self.board { unsafe {
            let b = &mut *board;
            b.clear_cursor();
        } }
    }

    pub fn tree_of_wisdom_hit_test(&self, x: i32, y: i32, hit_result: &mut HitResult) -> i32 {
        let size = self.tree_of_wisdom_get_size();
        let tree_rect = if size <= 1 { Rect::new(310, 275, 175, 175) }
            else if size < 7 { Rect::new(290, 255, 205, 195) }
            else if size < 12 { Rect::new(290, 215, 205, 225) }
            else { Rect::new(280, 155, 225, 305) };
        if tree_rect.contains(x, y) {
            hit_result.object = None;
            hit_result.object_type = GameObjectType::TreeOfWisdom;
            1
        } else {
            hit_result.object = None;
            hit_result.object_type = GameObjectType::None;
            0
        }
    }

    pub fn tree_of_wisdom_babble(&mut self) {
        self.challenge_state = ChallengeState::TreeBabbling;
        self.challenge_state_counter = 400;
        let size = self.tree_of_wisdom_get_size();
        let babble_hit = crate::framework::common::rand_range(3);
        if size <= 1 {
            self.tree_of_wisdom_talk_index = 600;
        } else if babble_hit == 0 && size >= 5 {
            self.tree_of_wisdom_talk_index = 500;
        } else if babble_hit == 1 {
            self.tree_of_wisdom_talk_index = 101 + crate::framework::common::rand_range(10);
        } else {
            self.tree_of_wisdom_talk_index = crate::framework::common::rand_range(4)
                + if size < 12 { 201 } else if size < 50 { 301 } else { 401 };
        }
    }

    pub fn tree_of_wisdom_give_wisdom(&mut self) {
        self.challenge_state = ChallengeState::TreeGiveWisdom;
        self.challenge_state_counter = 1000;
        let size = self.tree_of_wisdom_get_size();
        if size == 100 {
            self.tree_of_wisdom_talk_index = 800;
        } else if size == 500 {
            self.tree_of_wisdom_talk_index = 900;
        } else if size == 1000 {
            self.tree_of_wisdom_talk_index = 1000;
        } else if size > 1000 {
            self.tree_of_wisdom_talk_index = 1100;
        } else {
            self.tree_of_wisdom_talk_index = (size - 1).clamp(1, 49);
        }
    }

    pub fn tree_of_wisdom_say_repeat(&mut self) {
        let size = self.tree_of_wisdom_get_size();
        if size >= 100 && crate::framework::common::rand_range(47) == 0 {
            self.tree_of_wisdom_talk_index = 800;
        } else if size >= 500 && crate::framework::common::rand_range(47) == 0 {
            self.tree_of_wisdom_talk_index = 900;
        } else if size >= 1000 && crate::framework::common::rand_range(47) == 0 {
            self.tree_of_wisdom_talk_index = 1000;
        } else {
            self.tree_of_wisdom_talk_index = 2 + crate::framework::common::rand_range(size.clamp(3, 49) - 2);
        }
        self.challenge_state_counter = 600;
    }

    pub fn tree_of_wisdom_can_feed(&self) -> i32 {
        if self.challenge_state == ChallengeState::TreeJustGrew { return 0; }
        if let Some(board) = self.board { unsafe {
            for item in &(*board).grid_items {
                if !item.dead && item.grid_item_type == crate::lawn::grid_item::GridItemType::ZenTool {
                    return 0;
                }
            }
        } }
        1
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

// 艺术挑战模式图案（对应 C++ Challenge.cpp 中的全局数组）
// 索引为 [row][col]，row 范围 0..5（MAX_GRID_SIZE_Y），col 范围 0..8（MAX_GRID_SIZE_X）
pub const ART_CHALLENGE_WALLNUT: [[SeedType; 9]; 6] = [
    [SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::Wallnut, SeedType::Wallnut, SeedType::Wallnut, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::None, SeedType::Wallnut, SeedType::None, SeedType::None, SeedType::None, SeedType::Wallnut, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::None, SeedType::Wallnut, SeedType::None, SeedType::None, SeedType::None, SeedType::Wallnut, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::None, SeedType::Wallnut, SeedType::None, SeedType::None, SeedType::None, SeedType::Wallnut, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::Wallnut, SeedType::Wallnut, SeedType::Wallnut, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None],
];

pub const ART_CHALLENGE_SUNFLOWER: [[SeedType; 9]; 6] = [
    [SeedType::None, SeedType::None, SeedType::Starfruit, SeedType::Starfruit, SeedType::Starfruit, SeedType::None, SeedType::None, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::Starfruit, SeedType::Wallnut, SeedType::Wallnut, SeedType::Wallnut, SeedType::Starfruit, SeedType::None, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::Starfruit, SeedType::Starfruit, SeedType::Starfruit, SeedType::None, SeedType::None, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::None, SeedType::Umbrella, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::Umbrella, SeedType::Umbrella, SeedType::Umbrella, SeedType::None, SeedType::None, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None],
];

pub const ART_CHALLENGE_STARFRUIT: [[SeedType; 9]; 6] = [
    [SeedType::None, SeedType::None, SeedType::None, SeedType::Starfruit, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::None, SeedType::Starfruit, SeedType::Starfruit, SeedType::None, SeedType::None, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::Starfruit, SeedType::Starfruit, SeedType::Starfruit, SeedType::Starfruit, SeedType::Starfruit, SeedType::Starfruit, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::None, SeedType::Starfruit, SeedType::Starfruit, SeedType::Starfruit, SeedType::None, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::None, SeedType::Starfruit, SeedType::None, SeedType::None, SeedType::Starfruit, SeedType::None, SeedType::None],
    [SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None, SeedType::None],
];

// 对应 C++: extern int gZombieWaves[NUM_LEVELS];
// 对应 C++: extern ZombieAllowedLevels gZombieAllowedLevels[NUM_ZOMBIE_TYPES];



