// PvZ Portable Rust 翻译 — CutScene（过场动画场景管理）
// 对应 C++ src/Lawn/Cutscene.h / Cutscene.cpp

use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::KeyCode;
use crate::lawn::board::{self, Board, MAX_GRID_SIZE_Y};
use crate::lawn::game_enums::{
    GameMode, NUM_ZOMBIE_TYPES, PlantingReason, ReanimationID, REANIMATIONID_NULL,
    SeedType, StoreItem, TutorialState, ZombieType,
};
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::widget::challenge_screen::ChallengeScreen;
use crate::todlib::tod_common::rand_range_float;

/// 过场动画场景管理（对应 C++ CutScene）
#[derive(Debug)]
pub struct CutScene {
    pub app: Option<*mut LawnApp>,
    pub board: Option<*mut Board>,

    // 时间控制
    pub m_cutscene_time: i32,
    pub m_sod_time: i32,
    pub m_grave_stone_time: i32,
    pub m_ready_set_plant_time: i32,
    pub m_fog_time: i32,
    pub m_boss_time: i32,
    pub m_crazy_dave_time: i32,
    pub m_lawn_mower_time: i32,
    pub m_crazy_dave_dialog_start: i32,

    // 状态标志
    pub m_seed_choosing: bool,
    pub m_zombies_won_reanim_id: ReanimationID,
    pub m_preloaded: bool,
    pub m_placed_zombies: bool,
    pub m_placed_lawn_items: bool,
    pub m_crazy_dave_count_down: i32,
    pub m_crazy_dave_last_talk_index: i32,
    pub m_upsell_hide_board: bool,
    pub m_upsell_challenge_screen: Option<*mut ChallengeScreen>,
    pub m_pre_updating_board: bool,

    // 预加载资源
    pub m_loaded_resource_names: Vec<String>,
}

impl CutScene {
    pub fn new() -> Self {
        CutScene {
            app: None,
            board: None,
            m_cutscene_time: 0,
            m_sod_time: 0,
            m_grave_stone_time: 0,
            m_ready_set_plant_time: 0,
            m_fog_time: 0,
            m_boss_time: 0,
            m_crazy_dave_time: 0,
            m_lawn_mower_time: 0,
            m_crazy_dave_dialog_start: -1,
            m_seed_choosing: false,
            m_zombies_won_reanim_id: REANIMATIONID_NULL,
            m_preloaded: false,
            m_placed_zombies: false,
            m_placed_lawn_items: false,
            m_crazy_dave_count_down: 0,
            m_crazy_dave_last_talk_index: -1,
            m_upsell_hide_board: false,
            m_upsell_challenge_screen: None,
            m_pre_updating_board: false,
            m_loaded_resource_names: Vec::new(),
        }
    }

    /// 设置 app 和 board 引用（在创建后调用）
    pub fn init_app(&mut self, app: *mut LawnApp) {
        self.app = Some(app);
        let board = unsafe { (*app).board };
        self.board = board;
    }
}

impl Drop for CutScene {
    fn drop(&mut self) {
        // 释放升级面板（对应析构函数中 delete mUpsellChallengeScreen）
        if let Some(screen) = self.m_upsell_challenge_screen {
            let _ = unsafe { Box::from_raw(screen) };
        }

        // 取消音频静音（对应 mApp->mMuteSoundsForCutscene = false）
        if let Some(app) = self.app {
            unsafe {
                (*app).m_mute_sounds_for_cutscene = false;
            }
        }

        // 释放预加载资源（对应 mApp->mResourceManager->ReleaseTrackedResources）
        // TODO: 当 ResourceManager 翻译完成后实现
    }
}

impl CutScene {

    /// 获取 LawnApp 引用
    pub fn get_app(&self) -> Option<&LawnApp> {
        unsafe { self.app.map(|a| &*a) }
    }

    pub fn get_app_mut(&mut self) -> Option<&mut LawnApp> {
        unsafe { self.app.map(|a| &mut *a) }
    }

    /// 获取 Board 引用
    pub fn get_board(&self) -> Option<&Board> {
        unsafe { self.board.map(|b| &*b) }
    }

    pub fn get_board_mut(&mut self) -> Option<&mut Board> {
        unsafe { self.board.map(|b| &mut *b) }
    }

    // ============================
    // 过场动画流程控制
    // ============================

    /// 开始关卡入场动画
    pub fn start_level_intro(&mut self) {
        // 对应 C++ StartLevelIntro：设置入场时间线参数与戴夫对话起点
        const TIME_ROLL_SOD_START: i32 = 6000;
        const TIME_ROLL_SOD_END: i32 = 8000;
        const TIME_GRAVE_STONE_START: i32 = 6000;
        const TIME_GRAVE_STONE_END: i32 = 7000;
        const TIME_READY_SET_PLANT_START: i32 = 6000;
        const TIME_READY_SET_PLANT_END: i32 = 7830;
        const TIME_FOG_ROLL_IN: i32 = 5950;
        const TIME_PAN_RIGHT_START: i32 = 1500;
        const TIME_EARLY_DAVE_LEAVE_END: i32 = 4000;

        self.m_cutscene_time = 0;
        if let Some(board) = self.get_board_mut() {
            board.m_show_shovel = false;
        }
        self.m_placed_zombies = false;
        self.m_preloaded = false;
        self.m_placed_lawn_items = false;

        let a_level = self.get_board().map_or(0, |b| b.level);
        let is_first_time = self.get_app().map_or(false, |a| a.is_first_time_adventure_mode());
        let game_mode = self.get_app().map_or(GameMode::Adventure, |a| a.game_mode);

        if is_first_time && (a_level == 1 || a_level == 2 || a_level == 4) {
            self.m_sod_time = TIME_ROLL_SOD_END - TIME_ROLL_SOD_START;
            if let Some(board) = self.get_board_mut() {
                board.m_sod_position = 0;
            }
        } else {
            self.m_sod_time = 0;
            if let Some(board) = self.get_board_mut() {
                board.m_sod_position = 1000;
            }
        }

        self.m_grave_stone_time = 0;
        // [TRANSLATION_NOTE]: 墓碑时间细分（WhackAZombie/生存重选）依赖挑战与棋盘状态，核心逻辑保留
        let stage_graves = self.get_board().map_or(false, |b| b.stage_has_grave_stones());
        if stage_graves {
            let is_whack = self.get_app().map_or(false, |a| a.is_whack_a_zombie_level());
            if !is_whack && !self.is_survival_repick() {
                self.m_grave_stone_time = TIME_GRAVE_STONE_END - TIME_GRAVE_STONE_START;
                if let Some(board) = self.get_board_mut() {
                    board.m_enable_grave_stones = true;
                }
            }
        }

        if is_first_time && a_level <= 2 {
            self.m_ready_set_plant_time = 0;
        } else if self.get_app().map_or(false, |a| {
            a.is_shovel_level() || a.is_squirrel_level() || a.is_wallnut_bowling_level()
                || a.game_mode == GameMode::ChallengeZombiquarium
                || a.game_mode == GameMode::ChallengeLastStand
                || a.game_mode == GameMode::ChallengeTreeOfWisdom
                || a.is_izombie_level() || a.is_whack_a_zombie_level() || a.is_scary_potter_level()
        }) {
            self.m_ready_set_plant_time = 0;
        } else {
            self.m_ready_set_plant_time = TIME_READY_SET_PLANT_END - TIME_READY_SET_PLANT_START;
        }

        self.m_lawn_mower_time = 0;
        self.m_crazy_dave_dialog_start = -1;
        // [TRANSLATION_NOTE]: 戴夫对话起点细化分支依赖等级/关卡类型与 packet upgrade 判定，核心冒险分支保留
        if is_first_time && a_level == 11 {
            self.m_crazy_dave_dialog_start = 201;
        } else if self.get_app().map_or(false, |a| a.is_wallnut_bowling_level() && a.is_adventure_mode()) {
            self.m_crazy_dave_dialog_start = if is_first_time { 2400 } else { 2411 };
            if let Some(board) = self.get_board_mut() {
                board.m_show_shovel = true;
            }
        } else if self.get_app().map_or(false, |a| a.is_whack_a_zombie_level() && a.is_adventure_mode()) {
            self.m_crazy_dave_dialog_start = 401;
        } else if self.get_app().map_or(false, |a| a.is_final_boss_level() && a.is_adventure_mode()) {
            self.m_crazy_dave_dialog_start = 2300;
        } else if self.get_app().map_or(false, |a| a.is_scary_potter_level() && a.is_adventure_mode()) {
            self.m_crazy_dave_dialog_start = 2500;
        } else if self.get_app().map_or(false, |a| a.is_stormy_night_level() && a.is_adventure_mode()) {
            self.m_crazy_dave_dialog_start = 1101;
        } else if self.get_app().map_or(false, |a| a.is_bungee_blitz_level() && a.is_adventure_mode()) {
            self.m_crazy_dave_dialog_start = if is_first_time { 1301 } else { 1304 };
        } else if !is_first_time && a_level == 1 {
            self.m_crazy_dave_dialog_start = 1601;
        } else if game_mode == GameMode::PuzzleIZombie1 {
            self.m_crazy_dave_dialog_start = 2200;
        } else if game_mode == GameMode::Upsell {
            self.m_crazy_dave_dialog_start = 3300;
            self.m_upsell_hide_board = true;
        } else if game_mode == GameMode::ScaryPotter1
            && !self.get_app().map_or(false, |a| a.has_beaten_challenge(GameMode::ScaryPotter1))
        {
            self.m_crazy_dave_dialog_start = 3000;
        }

        if self.m_crazy_dave_dialog_start != -1 {
            self.m_crazy_dave_time = TIME_EARLY_DAVE_LEAVE_END - TIME_PAN_RIGHT_START;
            if self.get_app().map_or(false, |a| a.is_final_boss_level() && a.is_adventure_mode()) {
                self.m_crazy_dave_time += 4000;
            }
        }

        let has_fog = self.get_board().map_or(false, |b| b.stage_has_fog());
        self.m_fog_time = if has_fog {
            TIME_FOG_ROLL_IN - self.m_sod_time - self.m_lawn_mower_time - TIME_READY_SET_PLANT_START + 2000
        } else {
            0
        };

        self.m_boss_time = if self.get_app().map_or(false, |a| a.is_final_boss_level()) { 4000 } else { 0 };

        if self.is_scrolled_left_at_start() {
            // [TRANSLATION_NOTE]: C++ mBoard->Move(220, 0) 为渲染平移，Rust Board 无渲染偏移字段，暂不执行
        }
        if self.is_non_scrolling_cutscene() && self.m_crazy_dave_time == 0 {
            self.cancel_intro();
            return;
        }
        // [TRANSLATION_NOTE]: 房屋名提示（DisplayAdvice）与音乐选择依赖提示/音乐系统，已在上轮接入音乐；提示暂略
    }

    /// 取消入场动画
    pub fn cancel_intro(&mut self) {
        // [TRANSLATION_NOTE]: CancelIntro — 跳过入场动画，直接进入游戏
        // 预加载资源、放置僵尸、放置草坪物品、跳过时间到入场结束
    }

    /// 逐帧更新过场动画状态
    pub fn update(&mut self) {
        // 对应 C++ Update
        if self.m_pre_updating_board {
            return;
        }

        if self.is_showing_crazy_dave() && !self.get_board().map_or(false, |b| b.m_paused) {
            if let Some(app) = self.get_app_mut() {
                app.update_crazy_dave();
            }
        }

        if self.get_board().map_or(false, |b| b.m_paused) {
            return;
        }

        let scene = self.get_app().map_or(crate::lawn::lawn_app::GameScenes::Playing, |a| a.game_scene);
        if scene == crate::lawn::lawn_app::GameScenes::ZombiesWon {
            self.m_cutscene_time += 10;
            self.update_zombies_won();
            return;
        }

        let board_update_counter = self.get_board().map_or(0, |b| b.m_update_count);
        if scene != crate::lawn::lawn_app::GameScenes::LevelIntro || board_update_counter <= 1 {
            return;
        }

        if !self.m_preloaded {
            self.preload_resources();
        }
        if !self.m_placed_zombies {
            self.place_street_zombies();
        }
        if self.is_non_scrolling_cutscene() || !self.get_board().map_or(false, |b| b.choose_seeds_on_current_level()) {
            self.place_lawn_items();
        }

        // 选种/对话/铲子教程期间暂停过场时间
        let mut cutscene_time_stop = false;
        if self.m_seed_choosing
            || self.get_app().map_or(false, |a| a.m_crazy_dave_message_index != -1)
            || self.is_in_shovel_tutorial()
        {
            cutscene_time_stop = true;
        }

        let game_mode = self.get_app().map_or(GameMode::Adventure, |a| a.game_mode);
        if game_mode == GameMode::Upsell {
            self.update_upsell();
            let dave_state = self.get_app().map_or(0, |a| a.m_crazy_dave_state as i32);
            // C++: 非 OFF/ENTERING 时暂停
            if dave_state != 0 && dave_state != 1 {
                cutscene_time_stop = true;
            }
        }
        if game_mode == GameMode::Intro {
            self.m_cutscene_time += 10;
            self.update_intro();
            return;
        }
        if !cutscene_time_stop {
            self.m_cutscene_time += 10;
            if self.m_cutscene_time == 4250 + self.m_crazy_dave_time
                && self.get_board().map_or(false, |b| b.choose_seeds_on_current_level())
            {
                self.start_seed_chooser();
            }
        }

        // 检查过场是否结束
        let time_start = 6000 + self.m_lawn_mower_time + self.m_sod_time + self.m_grave_stone_time
            + self.m_crazy_dave_time + self.m_fog_time + self.m_boss_time + self.m_ready_set_plant_time;
        if self.m_cutscene_time >= time_start {
            if let Some(board) = self.get_board_mut() {
                board.remove_cutscene_zombies();
            }
            self.show_shovel();
            if let Some(app) = self.get_app_mut() {
                app.start_playing();
            }
            return;
        }

        self.animate_board();
    }

    /// 动画板块移动
    pub fn animate_board(&mut self) {
        // 对应 C++ AnimateBoard — 板车、草坪等动画
    }

    /// 开始选择种子
    pub fn start_seed_chooser(&mut self) {
        // 对应 C++ StartSeedChooser
        self.m_seed_choosing = true;
    }

    /// 结束选择种子
    pub fn end_seed_chooser(&mut self) {
        // 对应 C++ EndSeedChooser: mCutsceneTime = mCrazyDaveTime + TimeSeedChoserSlideOnEnd + 10
        self.m_seed_choosing = false;
        self.m_cutscene_time = self.m_crazy_dave_time + 4250 + 10;
        // 放置草坪物品（花盆、墓碑等）
    }

    /// 计算动画位置（线性插值）
    pub fn calc_position(
        &self,
        time_start: i32,
        time_end: i32,
        position_start: i32,
        position_end: i32,
    ) -> i32 {
        if time_start >= time_end {
            return position_end;
        }
        let elapsed = self.m_cutscene_time - time_start;
        let duration = time_end - time_start;
        if elapsed <= 0 {
            position_start
        } else if elapsed >= duration {
            position_end
        } else {
            position_start + (position_end - position_start) * elapsed / duration
        }
    }

    /// 放置街道僵尸
    pub fn place_street_zombies(&mut self) {
        // 对应 C++ PlaceStreetZombies
        if self.m_placed_zombies { return; }
        self.m_placed_zombies = true;
        if self.get_app().map_or(false, |app| app.is_final_boss_level()) { return; }

        let mut zombie_type_count = [0i32; NUM_ZOMBIE_TYPES as usize];
        let mut total_zombie_count = 0i32;

        let (num_waves, waves, zombie_allowed) = {
            let board = match self.get_board() { Some(b) => b, None => return };
            (board.m_num_waves, &board.m_zombies_in_wave, &board.m_zombie_allowed)
        };
        let app = self.get_app();
        let game_mode = app.map_or(GameMode::Adventure, |a| a.game_mode);

        for wave in 0..num_waves {
            for &zombie_type in &waves[wave as usize] {
                if zombie_type == ZombieType::Invalid { break; }
                if zombie_type == ZombieType::Flag { continue; }
                if zombie_type == ZombieType::Yeti && !app.map_or(false, |a| a.is_stormy_night_level()) { continue; }
                if zombie_type == ZombieType::Bobsled && game_mode != GameMode::ChallengeBobsledBonanza { continue; }
                if (zombie_type as i32) < 0 || (zombie_type as usize) >= NUM_ZOMBIE_TYPES as usize { continue; }
                zombie_type_count[zombie_type as usize] += 1;
                total_zombie_count += 1;
                if zombie_type == ZombieType::Bungee || zombie_type == ZombieType::Bobsled {
                    zombie_type_count[zombie_type as usize] = 1;
                }
            }
        }

        if game_mode == GameMode::ChallengeLastStand {
            for zombie_type in 0..NUM_ZOMBIE_TYPES as usize {
                if zombie_type != ZombieType::Yeti as usize && zombie_allowed[zombie_type] {
                    zombie_type_count[zombie_type] = zombie_type_count[zombie_type].max(1);
                }
            }
        }
        let has_pool = {
            let board = match self.get_board() { Some(b) => b, None => return };
            board.stage_has_pool()
        };
        if has_pool {
            zombie_type_count[ZombieType::DuckyTube as usize] = 1;
        }

        let mut zombie_grid = [[false; 5]; 5];
        let mut preview_capacity = 10;
        if app.map_or(false, |a| a.is_little_trouble_level()) {
            preview_capacity = 15;
        } else if (app.map_or(false, |a| a.is_stormy_night_level()) && app.map_or(false, |a| a.is_adventure_mode()))
            || app.map_or(false, |a| a.is_mini_boss_level())
        {
            preview_capacity = 18;
        }

        // 先放大体型僵尸（2x2 或雪橇车），再放普通僵尸
        for zombie_type in 0..NUM_ZOMBIE_TYPES as usize {
            if zombie_type_count[zombie_type] != 0
                && (Self::is_2x2_zombie(unsafe { std::mem::transmute::<i32, ZombieType>(zombie_type as i32) })
                    || zombie_type == ZombieType::Zamboni as usize)
            {
                self.find_and_place_zombie(unsafe { std::mem::transmute::<i32, ZombieType>(zombie_type as i32) }, &mut zombie_grid);
            }
        }
        for zombie_type in 0..NUM_ZOMBIE_TYPES as usize {
            if zombie_type_count[zombie_type] != 0
                && !Self::is_2x2_zombie(unsafe { std::mem::transmute::<i32, ZombieType>(zombie_type as i32) })
                && zombie_type != ZombieType::Zamboni as usize
            {
                let zombie_num_in_wave = zombie_type_count[zombie_type];
                let mut zombie_preview_num = if total_zombie_count > 0 {
                    zombie_num_in_wave * preview_capacity / total_zombie_count
                } else { 0 };
                zombie_preview_num = zombie_preview_num.clamp(1, zombie_num_in_wave);
                for _ in 0..zombie_preview_num {
                    self.find_and_place_zombie(unsafe { std::mem::transmute::<i32, ZombieType>(zombie_type as i32) }, &mut zombie_grid);
                }
            }
        }
    }

    /// 添加墓碑粒子效果
    pub fn add_grave_stone_particles(&mut self) {
        // 对应 C++ AddGraveStoneParticles
        // GridItem::AddGraveStoneParticles 尚未翻译，这里保留遍历逻辑
        if let Some(board) = self.get_board_mut() {
            for item in &mut board.grid_items {
                if item.dead { continue; }
                if item.grid_item_type == crate::lawn::grid_item::GridItemType::Grave {
                    // item.add_grave_stone_particles() — 待 GridItem 翻译后接入
                }
            }
        }
    }

    /// 在指定网格位置放置一个僵尸
    pub fn place_a_zombie(
        &mut self,
        mut zombie_type: ZombieType,
        grid_x: i32,
        grid_y: i32,
    ) {
        // 对应 C++ PlaceAZombie
        let mut put_on_ducky_tube = false;
        if zombie_type == ZombieType::DuckyTube {
            if let Some(app) = self.get_app() {
                if app.game_mode == GameMode::ChallengeWarAndPeas2 {
                    zombie_type = ZombieType::PeaHead;
                    put_on_ducky_tube = true;
                }
            }
        }

        // 预先计算判断值，避免借用冲突
        let stage_has_roof = self.get_board().map_or(false, |b| b.stage_has_roof());
        let is_little_trouble = self.get_app().map_or(false, |app| app.is_little_trouble_level());
        let can_show_something = self.get_app().map_or(false, |app| app.can_show_almanac() || app.can_show_store());

        let board = match self.get_board_mut() {
            Some(b) => b,
            None => return,
        };
        board.add_zombie_in_row(zombie_type, grid_y, -2);
        let zombie = match board.zombies.last_mut() {
            Some(z) => z,
            None => return,
        };
        zombie.pos_x = (grid_x * 56 + 830) as f32;
        zombie.pos_y = (grid_y * 90 + 70) as f32;
        if grid_x % 2 == 1 {
            zombie.pos_y += 30.0;
        }
        let _ = put_on_ducky_tube;

        if stage_has_roof {
            zombie.pos_y -= (grid_y * 2 - grid_x * 7 + 30) as f32;
            zombie.pos_x -= 5.0;
        }
        if zombie_type == ZombieType::Zamboni {
            zombie.pos_y -= 10.0;
            zombie.pos_x -= 30.0;
        } else if is_little_trouble {
            zombie.pos_y += crate::framework::common::rand_range(50) as f32 - 25.0;
            zombie.pos_x += crate::framework::common::rand_range(50) as f32 - 25.0;
        } else if Self::is_2x2_zombie(zombie_type) {
            zombie.pos_x += crate::framework::common::rand_range(15) as f32 - 20.0;
        } else if grid_y == 4 && can_show_something {
            zombie.pos_x += crate::framework::common::rand_range(15) as f32;
        } else {
            zombie.pos_y += crate::framework::common::rand_range(15) as f32;
            zombie.pos_x += crate::framework::common::rand_range(15) as f32;
        }
        zombie.base.render_order = board::make_render_order(0, 0, (grid_x % 2) * 2 + grid_y * 4);

        if zombie_type == ZombieType::Bungee {
            zombie.base.render_order = board::make_render_order(0, 0, 0);
            zombie.base.row = 0;
            zombie.pos_x = grid_x as f32 * 50.0 + 950.0;
            zombie.pos_y = 50.0;
        } else if zombie_type == ZombieType::Bobsled {
            zombie.base.render_order = board::make_render_order(0, 0, 1000);
            zombie.base.row = 0;
            zombie.pos_x = 1105.0;
            zombie.pos_y = 480.0;
        }
    }

    /// 检查僵尸能否放在某个网格位置
    pub fn can_zombie_go_in_grid_spot(
        &self,
        zombie_type: ZombieType,
        grid_x: i32,
        grid_y: i32,
        zombie_grid: [[bool; 5]; 5],
    ) -> bool {
        if grid_x < 0 || grid_x > 4 || grid_y < 0 || grid_y > 4 {
            return false;
        }
        if zombie_grid[grid_x as usize][grid_y as usize] {
            return false;
        }

        if Self::is_2x2_zombie(zombie_type) {
            if grid_x == 0 || grid_y == 0 {
                return false;
            }
            if zombie_grid[(grid_x - 1) as usize][grid_y as usize]
                || zombie_grid[grid_x as usize][(grid_y - 1) as usize]
                || zombie_grid[(grid_x - 1) as usize][(grid_y - 1) as usize]
            {
                return false;
            }
        }

        // 边缘格限制
        if grid_x == 4 && grid_y == 0 {
            return false;
        }
        if grid_x == 0 && grid_y == 0 {
            return false;
        }

        // 大型怪物不能放在边缘
        if Self::is_2x2_zombie(zombie_type) || zombie_type == ZombieType::Zamboni {
            if grid_x == 0 {
                return false;
            }
            if grid_x == 1 && grid_y == 0 {
                return false;
            }
        }

        true
    }

    /// 判断是否是生存模式重选
    pub fn is_survival_repick(&self) -> bool {
        // 对应 C++ IsSurvivalRepick
        if let Some(app) = self.get_app() {
            if !app.is_survival_mode() { return false; }
            if app.game_scene != crate::lawn::lawn_app::GameScenes::LevelIntro { return false; }
            if let Some(board) = self.get_board() {
                return board.challenge.as_ref().map_or(false, |c| c.survival_stage > 0);
            }
        }
        false
    }

    /// 判断是否在选种后
    pub fn is_after_seed_chooser(&self) -> bool {
        // 对应 C++ IsAfterSeedChooser: mCutsceneTime > TimeSeedChoserSlideOffStart + mCrazyDaveTime
        self.m_cutscene_time > 4500 + self.m_crazy_dave_time
    }

    /// 添加花盆
    pub fn add_flower_pots(&mut self) {
        // 对应 C++ AddFlowerPots
        let mut pot_columns = 0;
        if let Some(board) = self.get_board() {
            if board.level == 41 {
                pot_columns = 5;
            } else if board.level == 42 {
                pot_columns = 4;
            } else if board.level >= 43 && board.level <= 50 {
                pot_columns = 3;
            } else if self.get_app().map_or(false, |app| app.game_mode == GameMode::ChallengeColumns) {
                pot_columns = 8;
            } else if board.stage_has_roof() {
                pot_columns = 3;
            }
        }

        for x in 0..pot_columns {
            for y in 0..MAX_GRID_SIZE_Y as i32 {
                let can_plant = self.get_board().map_or(false, |board| {
                    board.can_plant_at(x, y, SeedType::Flowerpot) == PlantingReason::Ok
                });
                if can_plant {
                    if let Some(board) = self.get_board_mut() {
                        board.add_plant(x, y, SeedType::Flowerpot, SeedType::None);
                    }
                }
            }
        }
    }

    /// 更新僵尸胜利动画
    pub fn update_zombies_won(&mut self) {
        // 对应 C++ UpdateZombiesWon
        const LOST_TIME_PAN_RIGHT_START: i32 = 1500;
        const LOST_TIME_PAN_RIGHT_END: i32 = 3500;
        const LOST_TIME_BRAIN_GRAPHIC_START: i32 = 6000;
        const LOST_TIME_BRAIN_GRAPHIC_SHAKE: i32 = 7000;
        const LOST_TIME_BRAIN_GRAPHIC_CANCEL_SHAKE: i32 = 8000;
        const LOST_TIME_BRAIN_GRAPHIC_END: i32 = 11000;
        const LOST_TIME_END: i32 = 11000;

        // C++: 镜头向右平移（mBoard->Move(CalcPosition(...), 0)；Rust 侧 board 无渲染偏移字段，暂不执行）
        let _ = (LOST_TIME_PAN_RIGHT_START, LOST_TIME_PAN_RIGHT_END);

        // C++: 脑图出现前的咀嚼音效
        if self.m_cutscene_time == LOST_TIME_BRAIN_GRAPHIC_START - 400
            || self.m_cutscene_time == LOST_TIME_BRAIN_GRAPHIC_START - 900
        {
            if let Some(app) = self.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Chomp as i32);
            }
        }

        // C++: 脑图动画 + 尖叫
        if self.m_cutscene_time == LOST_TIME_BRAIN_GRAPHIC_START {
            crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(crate::lawn::game_enums::ReanimationType::ZombiesWon);
            let a_render_position = crate::lawn::board::make_render_order(
                crate::lawn::game_enums::RENDER_LAYER_SCREEN_FADE,
                0,
                0,
            );
            let a_reanim_ptr = self.get_app_mut().and_then(|app| {
                app.add_reanimation(
                    -220.0, // -BOARD_OFFSET
                    0.0,
                    a_render_position,
                    crate::lawn::game_enums::ReanimationType::ZombiesWon as i32,
                )
            });
            if let Some(ptr) = a_reanim_ptr {
                if let Some(app) = self.get_app_mut() {
                    let a_id = app.reanimation_get_id(ptr);
                    if let Some(reanim) = app.reanimation_get_mut(a_id) {
                        reanim.m_anim_rate = 12.0;
                        reanim.m_loop_type = crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold;
                    }
                    self.m_zombies_won_reanim_id = a_id;
                }
            }
            if let Some(app) = self.get_app() {
                app.play_foley(crate::todlib::tod_foley::FoleyType::Scream as i32);
            }
        }

        // C++: 脑图抖动与取消（SetShakeOverride 在 Rust reanim 侧为 stub）
        if self.m_cutscene_time == LOST_TIME_BRAIN_GRAPHIC_SHAKE
            || self.m_cutscene_time == LOST_TIME_BRAIN_GRAPHIC_CANCEL_SHAKE
        {
            // [TRANSLATION_NOTE]: SetShakeOverride("ZombiesWon", ...) 依赖轨道实例 shake 字段，暂不执行
        }
        if self.m_cutscene_time == LOST_TIME_BRAIN_GRAPHIC_END {
            let a_reanim_id = self.m_zombies_won_reanim_id;
            if let Some(app) = self.get_app_mut() {
                if let Some(reanim) = app.reanimation_get_mut(a_reanim_id) {
                    // [TRANSLATION_NOTE]: SetFramesForLayer("anim_screen") 在 Rust reanim 侧为 stub
                    reanim.m_loop_type = crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold;
                }
            }
        }

        // C++: 结束时间弹出 GameOver 对话框
        if self.m_cutscene_time == LOST_TIME_END {
            // [TRANSLATION_NOTE]: GameOverDialog 依赖对话框系统，暂以注释保留
        }
    }

    /// 开始僵尸胜利动画
    pub fn start_zombies_won(&mut self) {
        // 对应 C++ StartZombiesWon
        self.m_cutscene_time = 0;
        if let Some(board) = self.get_board_mut() {
            board.m_show_shovel = false;
            board.stop_all_zombie_sounds();
        }
        // mApp->mMusic->StopAllMusic() 与 PlaySample(SOUND_LOSEMUSIC) 暂未接入音频系统
    }

    /// 显示僵尸行走
    pub fn show_zombie_walking(&self) -> bool {
        // 对应 C++ ShowZombieWalking: mCutsceneTime > LostTimePanRightStart
        self.m_cutscene_time > 1500
    }

    /// 过场动画是否结束
    pub fn is_cut_scene_over(&self) -> bool {
        // 对应 C++ IsCutSceneOver: mCutsceneTime >= LostTimeEnd
        self.m_cutscene_time >= 11000
    }

    /// 僵尸胜利时的点击处理
    pub fn zombie_won_click(&mut self) {
        // 对应 C++ ZombieWonClick
        if self.is_cut_scene_over() {
            if let Some(app) = self.get_app_mut() {
                app.end_level();
            }
        }
    }

    /// 鼠标按下
    pub fn mouse_down(&mut self, _x: i32, _y: i32) {
        // [TRANSLATION_NOTE]: MouseDown — 处理点击跳过对话/入场
        if self.m_seed_choosing { return; }
        self.cancel_intro();
    }

    /// 键盘按下
    pub fn key_down(&mut self, _key: KeyCode) {
        // [TRANSLATION_NOTE]: KeyDown — 处理按键跳过入场
        if self.m_seed_choosing { return; }
        self.cancel_intro();
    }

    /// 推进疯狂戴夫的对话
    pub fn advance_crazy_dave_dialog(&mut self, just_skipping: bool) {
        // 对应 C++ AdvanceCrazyDaveDialog
        let game_mode = self.get_app().map_or(GameMode::Adventure, |a| a.game_mode);
        if game_mode == GameMode::Upsell {
            return;
        }
        let message_index = self.get_app().map_or(-1, |a| a.m_crazy_dave_message_index);
        if message_index == -1 {
            return;
        }

        // "Pick up the shovel and start digging"
        if message_index == 2406 && !just_skipping {
            if let Some(board) = self.get_board_mut() {
                board.m_tutorial_state = TutorialState::ShovelPickup;
            }
            if let Some(app) = self.get_app_mut() {
                app.crazy_dave_leave();
            }
            return;
        }

        // 推进戴夫对话；没有下一句时戴夫离开
        if !self.get_app_mut().map_or(false, |app| app.advance_crazy_dave_text()) {
            if let Some(app) = self.get_app_mut() {
                app.crazy_dave_leave();
            }
            return;
        }

        let message_index = self.get_app().map_or(-1, |a| a.m_crazy_dave_message_index);
        // Now_Unused
        if message_index == 107 || message_index == 2407 {
            if let Some(board) = self.get_board_mut() {
                if let Some(challenge) = &mut board.challenge {
                    challenge.shovel_add_wallnuts();
                }
            }
        }
        // "And it's not a shovel, it's a mallet" || "Let's go bowling!"
        if message_index == 405 || message_index == 2411 {
            if let Some(board) = self.get_board_mut() {
                if let Some(challenge) = &mut board.challenge {
                    challenge.show_bowling_line = 1;
                }
            }
        }
        // "Of course it wasn't me, it was you!"
        if message_index == 406 {
            if let Some(board) = self.get_board_mut() {
                board.m_enable_grave_stones = true;
            }
            self.add_grave_stone_particles();
        }
    }

    /// 能否获得升级包
    pub fn can_get_packet_upgrade(&self) -> bool {
        // 对应 C++ CanGetPacketUpgrade
        let cost = crate::lawn::widget::store_screen::StoreScreen::get_item_cost(StoreItem::PacketUpgrade);
        if let Some(app) = self.get_app() {
            if let Some(player) = &app.player_info {
                let purchase = player.m_purchases.get(StoreItem::PacketUpgrade as usize).copied().unwrap_or(0);
                return purchase == 0
                    && player.m_coins >= cost
                    && player.m_didnt_purchase_packet_upgrade < 2;
            }
        }
        false
    }

    /// 能否获得指定索引的升级包
    pub fn can_get_packet_upgrade_index(&self, index: i32) -> bool {
        // 对应 C++ CanGetPacketUpgrade(int theUpgradeIndex)
        let cost = crate::lawn::widget::store_screen::StoreScreen::get_item_cost(StoreItem::PacketUpgrade);
        if let Some(app) = self.get_app() {
            if let Some(player) = &app.player_info {
                let purchase = player.m_purchases.get(StoreItem::PacketUpgrade as usize).copied().unwrap_or(0);
                return purchase == index
                    && player.m_coins >= cost
                    && player.m_didnt_purchase_packet_upgrade < 2;
            }
        }
        false
    }

    /// 为街道僵尸找位
    pub fn find_place_for_street_zombies(
        &self,
        zombie_type: ZombieType,
        zombie_grid: &[[bool; 5]; 5],
        pos_x: &mut i32,
        pos_y: &mut i32,
    ) {
        // 对应 C++ FindPlaceForStreetZombies
        if zombie_type == ZombieType::Bungee {
            *pos_x = 0;
            *pos_y = 0;
            return;
        }

        let mut picks: Vec<crate::todlib::tod_common::TodWeightedGridArray> = Vec::new();
        for grid_x in 0..5 {
            for grid_y in 0..5 {
                if self.can_zombie_go_in_grid_spot(zombie_type, grid_x, grid_y, *zombie_grid) {
                    picks.push(crate::todlib::tod_common::TodWeightedGridArray { x: grid_x, y: grid_y, weight: 1 });
                }
            }
        }

        if picks.is_empty() {
            *pos_x = 2;
            *pos_y = 2;
        } else {
            let pick_count = picks.len();
            let pick = crate::todlib::tod_common::tod_pick_from_weighted_grid_array(&mut picks, pick_count);
            let pick = match pick { Some(p) => p, None => { *pos_x = 2; *pos_y = 2; return; } };
            *pos_x = picks[pick].x;
            *pos_y = picks[pick].y;
        }
    }

    /// 查找并放置僵尸
    pub fn find_and_place_zombie(
        &mut self,
        zombie_type: ZombieType,
        zombie_grid: &mut [[bool; 5]; 5],
    ) {
        // 对应 C++ FindAndPlaceZombie
        let mut grid_x = 0;
        let mut grid_y = 0;
        self.find_place_for_street_zombies(zombie_type, zombie_grid, &mut grid_x, &mut grid_y);

        if zombie_type != ZombieType::Bungee {
            zombie_grid[grid_x as usize][grid_y as usize] = true;
        }
        if Self::is_2x2_zombie(zombie_type) {
            zombie_grid[(grid_x - 1) as usize][grid_y as usize] = true;
            zombie_grid[grid_x as usize][(grid_y - 1) as usize] = true;
            zombie_grid[(grid_x - 1) as usize][(grid_y - 1) as usize] = true;
        }

        self.place_a_zombie(zombie_type, grid_x, grid_y);
        if zombie_type == ZombieType::Bungee && self.get_app().map_or(false, |app| app.is_bungee_blitz_level()) {
            self.place_a_zombie(ZombieType::Bungee, 1, grid_y);
            self.place_a_zombie(ZombieType::Bungee, 2, grid_y);
        }
    }

    /// 判断是否是 2x2 大僵尸
    pub fn is_2x2_zombie(zombie_type: ZombieType) -> bool {
        zombie_type == ZombieType::Gargantuar
            || zombie_type == ZombieType::RedeEyeGargantuar
    }

    /// 预加载资源
    pub fn preload_resources(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ PreloadResources）
    }

    /// 预加载前检查
    pub fn is_before_preloading(&self) -> bool {
        // 对应 C++ IsBeforePreloading: mGameScene == SCENE_LEVEL_INTRO && !mPreloaded
        if let Some(app) = self.get_app() {
            return app.game_scene == crate::lawn::lawn_app::GameScenes::LevelIntro && !self.m_preloaded;
        }
        false
    }

    /// 疯狂戴夫是否在说话
    pub fn is_showing_crazy_dave(&self) -> bool {
        // 对应 C++ IsShowingCrazyDave
        if let Some(app) = self.get_app() {
            return app.game_scene == crate::lawn::lawn_app::GameScenes::LevelIntro
                && self.m_crazy_dave_time > 0
                && self.m_cutscene_time < 3500 + self.m_crazy_dave_time;
        }
        false
    }

    /// 是否是非滚动过场
    pub fn is_non_scrolling_cutscene(&self) -> bool {
        // 对应 C++ IsNonScrollingCutscene
        if let Some(app) = self.get_app() {
            return app.game_mode == GameMode::ChallengeIceLevel
                || app.game_mode == GameMode::Upsell
                || app.game_mode == GameMode::ChallengeZenGarden
                || app.game_mode == GameMode::ChallengeTreeOfWisdom
                || app.game_mode == GameMode::ChallengeZombiquarium
                || app.is_scary_potter_level()
                || app.is_izombie_level()
                || app.is_whack_a_zombie_level()
                || app.is_shovel_level()
                || app.is_squirrel_level()
                || app.is_wallnut_bowling_level();
        }
        false
    }

    /// 开始时是否左滚
    pub fn is_scrolled_left_at_start(&self) -> bool {
        // 对应 C++ IsScrolledLeftAtStart
        if let Some(app) = self.get_app() {
            if app.is_survival_mode() {
                if let Some(board) = self.get_board() {
                    if board.challenge.as_ref().map_or(false, |c| c.survival_stage > 0) {
                        return false;
                    }
                }
            }
        }
        !self.is_non_scrolling_cutscene()
    }

    /// 是否在铲子教程中
    pub fn is_in_shovel_tutorial(&self) -> bool {
        // 对应 C++ IsInShovelTutorial
        if let Some(board) = self.get_board() {
            return board.m_tutorial_state == TutorialState::ShovelPickup
                || board.m_tutorial_state == TutorialState::ShovelDig
                || board.m_tutorial_state == TutorialState::ShovelKeepDigging;
        }
        false
    }

    /// 显示铲子
    pub fn show_shovel(&mut self) {
        // 对应 C++ ShowShovel
        if let Some(app) = self.get_app() {
            if app.is_whack_a_zombie_level()
                || app.is_wallnut_bowling_level()
                || app.game_mode == GameMode::ChallengeBeghouled
                || app.game_mode == GameMode::ChallengeBeghouledTwist
                || app.game_mode == GameMode::ChallengeZenGarden
                || app.game_mode == GameMode::ChallengeZombiquarium
                || app.game_mode == GameMode::ChallengeTreeOfWisdom
                || app.is_izombie_level()
            {
                return;
            }
        }
        if let Some(app) = self.get_app() {
            if !app.is_first_time_adventure_mode() {
                if let Some(board) = self.get_board_mut() {
                    board.m_show_shovel = true;
                }
            } else if let Some(board) = self.get_board() {
                if board.level > 4 {
                    if let Some(board) = self.get_board_mut() {
                        board.m_show_shovel = true;
                    }
                }
            }
        }
    }

    /// 放置草坪物品
    pub fn place_lawn_items(&mut self) {
        // 对应 C++ PlaceLawnItems
        if self.m_placed_lawn_items { return; }
        self.m_placed_lawn_items = true;

        if !self.is_survival_repick() {
            if let Some(board) = self.get_board_mut() {
                board.init_lawn_mowers();
            }
            self.add_flower_pots();
        }

        if !self.is_survival_repick() {
            if let Some(board) = self.get_board_mut() {
                board.place_rake();
            }
        }
    }

    /// 能否获得第二个升级包
    pub fn can_get_second_packet_upgrade(&self) -> bool {
        // 对应 C++ CanGetSecondPacketUpgrade
        let cost = crate::lawn::widget::store_screen::StoreScreen::get_item_cost(StoreItem::PacketUpgrade);
        if let Some(app) = self.get_app() {
            if let Some(player) = &app.player_info {
                let purchase = player.m_purchases.get(StoreItem::PacketUpgrade as usize).copied().unwrap_or(0);
                return purchase == 1
                    && player.m_coins >= cost
                    && player.m_didnt_purchase_packet_upgrade < 2;
            }
        }
        false
    }

    /// 从消息中解析延迟时间
    pub fn parse_delay_time_from_message(&mut self) -> i32 {
        // 对应 C++ ParseDelayTimeFromMessage: 解析文本中的 {DELAY_数字}
        if let Some(app) = self.get_app() {
            let text = &app.m_crazy_dave_message_text;
            if let Some(start) = text.find("{DELAY_") {
                let rest = &text[start + 7..];
                if let Some(end) = rest.find('}') {
                    let num = rest[..end].trim();
                    if let Ok(val) = num.parse::<i32>() {
                        self.m_crazy_dave_count_down = val;
                        return val;
                    }
                }
            }
        }
        100
    }

    /// 从消息中解析对话时间
    pub fn parse_talk_time_from_message(&mut self) -> i32 {
        // 对应 C++ ParseTalkTimeFromMessage: 解析文本中的 {TIME_数字}
        if let Some(app) = self.get_app() {
            let text = &app.m_crazy_dave_message_text;
            if let Some(start) = text.find("{TIME_") {
                let rest = &text[start + 6..];
                if let Some(end) = rest.find('}') {
                    let num = rest[..end].trim();
                    if let Ok(val) = num.parse::<i32>() {
                        self.m_crazy_dave_count_down = val;
                        return val;
                    }
                }
            }
        }
        100
    }

    /// 清除升级面板
    pub fn clear_upsell_board(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ ClearUpsellBoard）
    }

    /// 加载入场面板
    pub fn load_intro_board(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ LoadIntroBoard）
    }

    /// 添加升级僵尸
    pub fn add_upsell_zombie(&mut self, _zombie_type: ZombieType, _pixel_x: i32, _grid_y: i32) {
        // 内联函数
    }

    /// 加载升级面板（泳池）
    pub fn load_upsell_board_pool(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ LoadUpsellBoardPool）
    }

    /// 加载升级面板（雾）
    pub fn load_upsell_board_fog(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ LoadUpsellBoardFog）
    }

    /// 加载挑战升级面板
    pub fn load_upsell_challenge_screen(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ LoadUpsellChallengeScreen）
    }

    /// 加载升级面板（屋顶）
    pub fn load_upsell_board_roof(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ LoadUpsellBoardRoof）
    }

    /// 更新升级
    pub fn update_upsell(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ UpdateUpsell）
    }

    /// 绘制升级
    pub fn draw_upsell(&mut self, _g: &mut Graphics) {
        // TODO: 实现完整逻辑（对应 C++ DrawUpsell）
    }

    /// 更新入场动画
    pub fn update_intro(&mut self) {
        // 对应 C++ UpdateIntro：开场动画时间线
        const TIME_INTRO_PAN_RIGHT_START: i32 = 5890;
        const TIME_INTRO_PAN_RIGHT_END: i32 = 11890;
        const TIME_INTRO_FADE_OUT: i32 = 10890;
        const TIME_INTRO_LOGO_END: i32 = 5900;
        const TIME_INTRO_END: i32 = 13890;

        // C++: mBoard->Move(-AnimateCurve(...), 0) 渲染平移，Rust Board 无偏移字段
        let _ = (TIME_INTRO_PAN_RIGHT_START, TIME_INTRO_PAN_RIGHT_END);

        let scene_time = self.m_cutscene_time;
        if scene_time == 10 {
            self.load_intro_board();
        }
        if scene_time == TIME_INTRO_FADE_OUT {
            if let Some(app) = self.get_app_mut() {
                app.music.as_mut().map(|m| m.fade_out(250));
            }
        }
        if scene_time == TIME_INTRO_LOGO_END {
            let a_render_position = crate::lawn::board::make_render_order(
                crate::lawn::game_enums::RENDER_LAYER_TOP, 0, 0,
            );
            if let Some(app) = self.get_app_mut() {
                app.add_tod_particle(400.0, 300.0, a_render_position, crate::lawn::game_enums::ParticleEffect::ScreenFlash as i32);
            }
            if let Some(app) = self.get_app_mut() {
                app.m_mute_sounds_for_cutscene = false;
                // [TRANSLATION_NOTE]: PlaySample(SOUND_HUGE_WAVE) 依赖音效系统
                app.m_mute_sounds_for_cutscene = true;
            }
        }
        if scene_time == TIME_INTRO_FADE_OUT - 200 {
            if let Some(app) = self.get_app_mut() {
                app.m_mute_sounds_for_cutscene = false;
                // [TRANSLATION_NOTE]: PlaySample(SOUND_SIREN) 依赖音效系统
                app.m_mute_sounds_for_cutscene = true;
            }
        }
        if scene_time == TIME_INTRO_END {
            if let Some(app) = self.get_app_mut() {
                app.pre_new_game(GameMode::Adventure, false);
            }
        }
    }

    /// 绘制入场动画
    pub fn draw_intro(&mut self, _g: &mut Graphics) {
        // TODO: 实现完整逻辑（对应 C++ DrawIntro）
    }

    /// 是否应该运行升级面板
    pub fn should_run_upsell_board(&self) -> bool {
        // 对应 C++ ShouldRunUpsellBoard
        if let Some(app) = self.get_app() {
            return (app.game_mode == GameMode::Upsell || app.game_mode == GameMode::Intro) && !self.m_upsell_hide_board;
        }
        false
    }
}

impl Default for CutScene {
    fn default() -> Self {
        CutScene::new()
    }
}


