// PvZ Portable Rust 翻译 — CutScene（过场动画场景管理）
// 对应 C++ src/Lawn/Cutscene.h / Cutscene.cpp

use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::KeyCode;
use crate::lawn::board::{self, Board, MAX_GRID_SIZE_Y, MAX_ZOMBIES_IN_WAVE};
use crate::lawn::game_enums::{
    AdviceType, BackgroundType, ChallengePage, ChallengeState, CrazyDaveState, GameMode,
    GridSquareType, NUM_ZOMBIE_TYPES, ParticleEffect, PlantRowType, PlantingReason,
    ReanimationID, ReanimationType, REANIMATIONID_NULL, SeedType, StoreItem, TutorialState,
    ZombieType,
};
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::widget::challenge_screen::ChallengeScreen;
use crate::lawn::system::music::MusicTune;
use crate::todlib::tod_common::rand_range_float;
use crate::todlib::tod_foley::FoleyType;

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
        // 对应 C++ CancelIntro：跳过入场动画直接进入游戏
        const TIME_PAN_RIGHT_END: i32 = 3500;
        const TIME_SEED_CHOSER_SLIDE_ON_END: i32 = 4250;
        const TIME_PAN_LEFT_START: i32 = 4500;
        const TIME_INTRO_END: i32 = 6000;

        self.preload_resources();
        self.place_street_zombies();

        if self.m_cutscene_time < self.m_crazy_dave_time + TIME_PAN_RIGHT_END {
            self.m_cutscene_time = TIME_SEED_CHOSER_SLIDE_ON_END + self.m_crazy_dave_time - 20;

            if !self.is_non_scrolling_cutscene() {
                // [TRANSLATION_NOTE]: C++ 中 mBoard->Move(mApp->mWidth - BOARD_IMAGE_WIDTH_OFFSET, 0)；
                // Rust 侧 board 无偏移字段，暂略
            }

            if let Some(board) = self.board {
                // C++ 中 mBoard->mAdvice->mMessageStyle == MESSAGE_STYLE_HOUSE_NAME 时清空提示；
                // Rust 侧 m_advice 无 style，直接清理
                unsafe { (*board).clear_advice(AdviceType::None); }
            }

            if self.m_crazy_dave_dialog_start != -1 {
                if let Some(app) = self.app {
                    unsafe {
                        if (*app).m_crazy_dave_state == CrazyDaveState::Off {
                            (*app).crazy_dave_enter();
                        }
                        (*app).m_crazy_dave_message_index = self.m_crazy_dave_dialog_start;
                    }
                }
            }
            while self.app.map_or(false, |app| unsafe { (*app).m_crazy_dave_message_index != -1 }) {
                self.advance_crazy_dave_dialog(true);
            }

            if let Some(board) = self.board {
                unsafe {
                    let level = (*board).level;
                    if level == 5 {
                        for i in 0..(*board).plants.len() {
                            let b = &mut *board;
                            if b.plants[i].dead {
                                continue;
                            }
                            b.plants[i].die();
                        }
                        if let Some(ch) = (*board).challenge.as_mut() {
                            ch.show_bowling_line = 1;
                        }
                    }
                }
            }
        }

        if let Some(app) = self.app {
            unsafe { (*app).crazy_dave_die(); }
        }

        let choose_seeds = self.board.map_or(false, |b| unsafe { (*b).choose_seeds_on_current_level() });
        if self.m_cutscene_time > self.m_crazy_dave_time + TIME_PAN_LEFT_START || !choose_seeds {
            self.m_cutscene_time = TIME_INTRO_END + self.m_lawn_mower_time + self.m_sod_time
                + self.m_grave_stone_time + self.m_crazy_dave_time + self.m_fog_time
                + self.m_boss_time + self.m_ready_set_plant_time - 20;

            self.place_lawn_items();

            if let Some(board) = self.board {
                unsafe {
                    if let Some(app) = self.app {
                        if (*app).is_stormy_night_level() {
                            if let Some(ch) = (*board).challenge.as_mut() {
                                ch.challenge_state_counter = 0;
                            }
                        }
                        if (*app).is_final_boss_level() {
                            (*board).challenge.as_ref().map(|c| c.play_boss_enter());
                        }
                    }
                    // [TRANSLATION_NOTE]: C++ 中 !IsChallengeWithoutSeedBank() 时
                    // mSeedBank->Move(SEED_BANK_OFFSET_X_END, 0)；Rust 侧 board 无 seed_bank 字段

                    (*board).m_enable_grave_stones = true;
                }
            }

            self.show_shovel();

            if let Some(app) = self.app {
                unsafe {
                    if (*app).is_final_boss_level() {
                        if let Some(music) = (*app).music.as_mut() {
                            music.start_game_music();
                        }
                    }
                    if let Some(board) = self.board {
                        if (*board).m_fog_blown_count_down > 0 {
                            (*board).m_fog_blown_count_down = 0;
                            (*board).m_fog_offset = 0.0;
                        }
                        // [TRANSLATION_NOTE]: C++ 中 mMenuButton->mBtnNoDraw = false（非教程水壶状态）
                        if let Some(ss) = (*app).sound_system.as_ref() {
                            ss.stop_foley(FoleyType::Digger);
                        }
                    }
                }
            }
        }
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
        // 对应 C++ AnimateBoard：入场动画逐帧推进
        const TIME_PAN_RIGHT_START: i32 = 1500;
        const TIME_PAN_RIGHT_END: i32 = 3500;
        const TIME_EARLY_DAVE_ENTER_START: i32 = 2000;
        const TIME_EARLY_DAVE_ENTER_END: i32 = 2750;
        const TIME_EARLY_DAVE_LEAVE_END: i32 = 4000;
        const TIME_SEED_CHOSER_SLIDE_ON_START: i32 = 4000;
        const TIME_SEED_CHOSER_SLIDE_ON_END: i32 = 4250;
        const TIME_SEED_CHOSER_SLIDE_OFF_START: i32 = 4500;
        const TIME_SEED_CHOSER_SLIDE_OFF_END: i32 = 4750;
        const TIME_PAN_LEFT_START: i32 = 4500;
        const TIME_PAN_LEFT_END: i32 = 6000;
        const TIME_SEED_BANK_ON_START: i32 = 4000;
        const TIME_SEED_BANK_ON_END: i32 = 4250;
        const TIME_SEED_BANK_RIGHT_START: i32 = 4750;
        const TIME_SEED_BANK_RIGHT_END: i32 = 6000;
        const TIME_ROLL_SOD_START: i32 = 6000;
        const TIME_ROLL_SOD_END: i32 = 8000;
        const TIME_GRAVE_STONE_START: i32 = 6000;
        const TIME_READY_SET_PLANT_START: i32 = 6000;
        const TIME_FOG_ROLL_IN: i32 = 5950;
        const TIME_LAWN_MOWER_DURATION: i32 = 250;
        const TIME_LAWN_MOWER_START: [i32; 6] = [6300, 6250, 6200, 6150, 6100, 6050];
        const BOARD_OFFSET: i32 = 220;
        const BOARD_IMAGE_WIDTH_OFFSET: i32 = 1180;
        const SEED_BANK_OFFSET_X: i32 = 0;
        const SEED_BANK_OFFSET_X_END: i32 = 10;
        const SEED_CHOOSER_OFFSET_Y: i32 = 516;

        let a_time_pan_right_start = TIME_PAN_RIGHT_START + self.m_crazy_dave_time;
        let a_time_pan_right_end = TIME_PAN_RIGHT_END + self.m_crazy_dave_time;
        let a_time_pan_left_start = TIME_PAN_LEFT_START + self.m_crazy_dave_time;
        let a_time_pan_left_end = TIME_PAN_LEFT_END + self.m_crazy_dave_time;

        let app_width = self.app.map_or(0, |app| unsafe { (*app).base.width });

        // Crazy Dave 动画
        if self.m_crazy_dave_time > 0 {
            if self.m_cutscene_time == TIME_EARLY_DAVE_ENTER_START {
                if let Some(app) = self.app {
                    unsafe {
                        (*app).crazy_dave_enter();
                        if (*app).game_mode == GameMode::Upsell {
                            if let Some(r) = (*app).reanimation_get_mut((*app).m_crazy_dave_reanim_id) {
                                r.play_reanim("anim_enterup", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 0, 12.0);
                                r.set_position(150.0, 70.0);
                            }
                        }
                    }
                }
            }

            if self.m_cutscene_time == TIME_EARLY_DAVE_ENTER_END
                && self.m_crazy_dave_dialog_start != -1
                && self.app.map_or(false, |app| unsafe { (*app).game_mode != GameMode::Upsell })
            {
                if let Some(app) = self.app {
                    unsafe { (*app).crazy_dave_talk_index(self.m_crazy_dave_dialog_start); }
                }
                self.m_crazy_dave_dialog_start = -1;
            }

            if self.m_cutscene_time == TIME_EARLY_DAVE_LEAVE_END && self.is_non_scrolling_cutscene() {
                self.m_cutscene_time = a_time_pan_left_end;
            }
        }

        // 向右平移棋盘
        let a_board_offset = if self.is_scrolled_left_at_start() { BOARD_OFFSET } else { 0 };
        if let Some(board) = self.board {
            unsafe {
                let b = &mut *board;
                if self.m_cutscene_time <= a_time_pan_right_start {
                    b.move_by(a_board_offset, 0);
                } else if self.m_cutscene_time <= a_time_pan_right_end {
                    let a_pan_offset = self.calc_position(
                        a_time_pan_right_start, a_time_pan_right_end,
                        -a_board_offset, BOARD_IMAGE_WIDTH_OFFSET - app_width,
                    );
                    b.move_by(-a_pan_offset, 0);
                }
            }
        }

        // 种子选择器滑入/滑出（C++ 中 aSeedChoser->Move + mMenuButton）
        // [TRANSLATION_NOTE]: C++ 中依赖 mSeedChooserScreen 的 Move/mMenuButton；
        // Rust 侧 seed_chooser 为 Option<*mut ()> 且无移动接口，暂略

        // 向左平移棋盘
        if self.m_cutscene_time > a_time_pan_left_start {
            let a_pan_offset = self.calc_position(
                a_time_pan_left_start, a_time_pan_left_end,
                BOARD_IMAGE_WIDTH_OFFSET - app_width, 0,
            );
            if let Some(board) = self.board {
                unsafe { (*board).move_by(-a_pan_offset, 0); }
            }
        }

        // 种子银行动画
        let a_time_prepare_end = if self.board.map_or(false, |b| unsafe { (*b).choose_seeds_on_current_level() }) {
            0
        } else {
            self.m_boss_time + self.m_fog_time + self.m_grave_stone_time + self.m_sod_time
                - TIME_SEED_CHOSER_SLIDE_ON_START + TIME_PAN_LEFT_END
        };
        let a_time_seed_bank_on_start = TIME_SEED_BANK_ON_START + a_time_prepare_end + self.m_crazy_dave_time;
        let a_time_seed_bank_on_end = TIME_SEED_BANK_ON_END + a_time_prepare_end + self.m_crazy_dave_time;
        let no_seed_bank = self.app.map_or(false, |app| unsafe { (*app).is_challenge_without_seed_bank() });
        if !no_seed_bank && self.m_cutscene_time > a_time_seed_bank_on_start && self.m_cutscene_time <= a_time_seed_bank_on_end {
            // [TRANSLATION_NOTE]: C++ 中 aSeedBankY = CalcPosition(..., -IMAGE_SEEDBANK->GetHeight(), 0)
            if let Some(board) = self.board {
                unsafe { (*board).move_seed_bank(SEED_BANK_OFFSET_X); }
            }
        }
        let a_time_seed_bank_right_start = TIME_SEED_BANK_RIGHT_START + self.m_crazy_dave_time;
        let a_time_seed_bank_right_end = TIME_SEED_BANK_RIGHT_END + self.m_crazy_dave_time;
        if self.m_cutscene_time > a_time_seed_bank_right_start {
            let a_seed_bank_x = self.calc_position(
                a_time_seed_bank_right_start, a_time_seed_bank_right_end,
                SEED_BANK_OFFSET_X, SEED_BANK_OFFSET_X_END,
            );
            let a_darken = crate::todlib::tod_common::tod_animate_curve(
                a_time_seed_bank_right_start, a_time_seed_bank_right_end, self.m_cutscene_time,
                255, 128, crate::lawn::game_enums::TodCurves::EaseOut,
            );
            if let Some(board) = self.board {
                unsafe {
                    (*board).move_seed_bank(a_seed_bank_x);
                    (*board).m_seed_bank_darken = a_darken;
                }
            }
        }

        // 早期冒险关卡草皮滚动
        if self.m_sod_time > 0 {
            let a_time_roll_sod_start = TIME_ROLL_SOD_START + self.m_crazy_dave_time;
            let a_time_roll_sod_end = TIME_ROLL_SOD_END + self.m_crazy_dave_time;
            let a_sod_position = crate::todlib::tod_common::tod_animate_curve(
                a_time_roll_sod_start, a_time_roll_sod_end, self.m_cutscene_time,
                0, 1000, crate::lawn::game_enums::TodCurves::Linear,
            );
            if let Some(board) = self.board {
                unsafe { (*board).m_sod_position = a_sod_position; }
            }

            if self.m_cutscene_time == a_time_roll_sod_start {
                if let Some(app) = self.app {
                    unsafe {
                        (*app).play_foley(FoleyType::Digger as i32);
                        let render_order = crate::lawn::board::make_render_order(crate::lawn::game_enums::RENDER_LAYER_TOP, 0, 0);
                        let board_level = self.board.map_or(0, |b| unsafe { (*b).level });
                        if board_level == 1 {
                        (*app).add_reanimation(0.0, 0.0, render_order, ReanimationType::Sodroll as i32);
                        (*app).add_tod_particle(35.0, 348.0, crate::lawn::board::make_render_order(crate::lawn::game_enums::RENDER_LAYER_TOP, 0, 1), crate::lawn::game_enums::ParticleEffect::SodRoll as i32);
                    } else if board_level == 2 {
                        (*app).add_reanimation(0.0, -102.0, render_order, ReanimationType::Sodroll as i32);
                        (*app).add_reanimation(0.0, 111.0, render_order, ReanimationType::Sodroll as i32);
                        (*app).add_tod_particle(35.0, 246.0, crate::lawn::board::make_render_order(crate::lawn::game_enums::RENDER_LAYER_TOP, 0, 1), crate::lawn::game_enums::ParticleEffect::SodRoll as i32);
                        (*app).add_tod_particle(35.0, 459.0, crate::lawn::board::make_render_order(crate::lawn::game_enums::RENDER_LAYER_TOP, 0, 1), crate::lawn::game_enums::ParticleEffect::SodRoll as i32);
                    } else if board_level == 4 {
                        (*app).add_reanimation(-3.0, -198.0, render_order, ReanimationType::Sodroll as i32);
                        (*app).add_reanimation(-3.0, 203.0, render_order, ReanimationType::Sodroll as i32);
                        (*app).add_tod_particle(32.0, 150.0, crate::lawn::board::make_render_order(crate::lawn::game_enums::RENDER_LAYER_TOP, 0, 1), crate::lawn::game_enums::ParticleEffect::SodRoll as i32);
                        (*app).add_tod_particle(32.0, 511.0, crate::lawn::board::make_render_order(crate::lawn::game_enums::RENDER_LAYER_TOP, 0, 1), crate::lawn::game_enums::ParticleEffect::SodRoll as i32);
                        }
                    }
                }
            }

            if self.m_cutscene_time == a_time_roll_sod_end {
                if let Some(app) = self.app {
                    unsafe {
                        if let Some(ss) = (*app).sound_system.as_ref() {
                            ss.stop_foley(FoleyType::Digger);
                        }
                    }
                }
            }
        }

        // 夜间关卡墓碑浮现
        if self.m_grave_stone_time > 0 {
            let a_time_grave_stone_start = self.m_sod_time + TIME_GRAVE_STONE_START + self.m_crazy_dave_time;
            if self.m_cutscene_time == a_time_grave_stone_start {
                if let Some(board) = self.board {
                    unsafe { (*board).m_enable_grave_stones = true; }
                }
                self.add_grave_stone_particles();
            }
        }

        // 棋盘开始左移时放置草坪物品
        if self.m_cutscene_time == a_time_pan_left_start {
            self.place_lawn_items();
        }

        // 割草机驶入
        if !self.is_survival_repick() {
            if let Some(board) = self.board {
                unsafe {
                    let b = &mut *board;
                    for a_grid_y in 0..MAX_GRID_SIZE_Y {
                        let a_time_lawn_mower_start = TIME_LAWN_MOWER_START[a_grid_y] + self.m_sod_time
                            + self.m_grave_stone_time + self.m_crazy_dave_time;
                        if self.m_cutscene_time > a_time_lawn_mower_start {
                            // [TRANSLATION_NOTE]: C++ 中 FindLawnMowerInRow 查找该行割草机并
                            // 设置 visible/posX；Rust 侧割草机无 row 字段，遍历全部设置
                            for mower in b.lawn_mowers.iter_mut() {
                                mower.visible = true;
                                mower.pos_x = self.calc_position(
                                    a_time_lawn_mower_start,
                                    a_time_lawn_mower_start + TIME_LAWN_MOWER_DURATION,
                                    -80, -21,
                                ) as f32;
                            }
                        }
                    }
                }
            }
        }

        // 迷雾滚入
        if let Some(board) = self.board {
            unsafe {
                if (*board).m_fog_blown_count_down > 0 {
                    let a_time_fog_roll_in = TIME_FOG_ROLL_IN + self.m_sod_time + self.m_grave_stone_time + self.m_crazy_dave_time;
                    if self.m_cutscene_time > a_time_fog_roll_in {
                        if (*board).m_fog_blown_count_down > 200 {
                            (*board).m_fog_blown_count_down = 200;
                        }
                        (*board).m_fog_blown_count_down -= 1;
                    }
                }
            }
        }

        // 暴风雨闪电
        let is_stormy = self.app.map_or(false, |app| unsafe { (*app).is_stormy_night_level() });
        if is_stormy
            && (self.m_cutscene_time == a_time_pan_right_end - 1000 || self.m_cutscene_time == a_time_pan_left_end)
        {
            if let Some(board) = self.board {
                unsafe {
                    if let Some(ch) = (*board).challenge.as_mut() {
                        ch.challenge_state = ChallengeState::StormFlash2;
                        ch.challenge_state_counter = 310;
                    }
                }
            }
        }

        // 僵尸王入场
        if self.m_boss_time > 0 {
            let a_time_boss_enter = TIME_READY_SET_PLANT_START + self.m_lawn_mower_time + self.m_crazy_dave_time;
            if self.m_cutscene_time == a_time_boss_enter {
                if let Some(board) = self.board {
                    unsafe { (*board).challenge.as_ref().map(|c| c.play_boss_enter()); }
                }
            }
        }

        // Boss 关音乐
        let is_final_boss = self.app.map_or(false, |app| unsafe { (*app).is_final_boss_level() });
        if is_final_boss && self.m_cutscene_time == a_time_seed_bank_on_start {
            if let Some(app) = self.app {
                unsafe {
                    if let Some(music) = (*app).music.as_mut() {
                        music.start_game_music();
                    }
                }
            }
        }

        // Ready Set Plant 动画
        let a_time_ready_set_plant = TIME_READY_SET_PLANT_START + self.m_lawn_mower_time + self.m_sod_time
            + self.m_grave_stone_time + self.m_crazy_dave_time + self.m_fog_time + self.m_boss_time;
        if self.m_ready_set_plant_time > 0 && self.m_cutscene_time == a_time_ready_set_plant {
            if let Some(app) = self.app {
                let render_order = crate::lawn::board::make_render_order(crate::lawn::game_enums::RENDER_LAYER_SCREEN_FADE, 0, 0);
                unsafe {
                    (*app).add_reanimation(400.0, 324.0, render_order, ReanimationType::Readysetplant as i32);
                    if !is_final_boss {
                        if let Some(music) = (*app).music.as_mut() {
                            music.fade_out(150);
                        }
                    }
                }
            }
        }
        if self.m_ready_set_plant_time == 0 && self.m_cutscene_time == a_time_ready_set_plant - 2000 {
            if !is_final_boss {
                if let Some(app) = self.app {
                    unsafe {
                        if let Some(music) = (*app).music.as_mut() {
                            music.fade_out(200);
                        }
                    }
                }
            }
        }

        // [TRANSLATION_NOTE]: C++ 中 mSeedChooserScreen->mParent->BringToFront(mSeedChooserScreen)；
        // Rust 侧 seed_chooser 无 widget 层次接口，暂略
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
        // 对应 C++ PreloadResources：按关卡内容预加载僵尸/植物 reanim 与资源组
        if self.m_preloaded {
            return;
        }
        self.m_preloaded = true;
        self.m_loaded_resource_names.clear();

        if let Some(board) = self.board {
            unsafe {
                for a_wave in 0..(*board).m_num_waves {
                    for a_zombie_index in 0..MAX_ZOMBIES_IN_WAVE {
                        let a_zombie_type = (*board).m_zombies_in_wave[a_wave as usize][a_zombie_index];
                        if a_zombie_type == ZombieType::Invalid {
                            break;
                        }
                        crate::lawn::zombie::Zombie::preload_zombie_resources(a_zombie_type);
                    }
                }
            }
        }

        if let Some(app) = self.get_app() {
            for seed_index in 0..crate::lawn::game_enums::NUM_SEED_TYPES {
                let a_seed_type = unsafe { std::mem::transmute::<i32, SeedType>(seed_index as i32) };
                if app.has_seed_type(a_seed_type) {
                    crate::lawn::plant::Plant::preload_plant_resources(a_seed_type);
                }
            }
            if app.is_first_time_adventure_mode() {
                let board_level = self.board.map_or(0, |b| unsafe { (*b).level });
                if board_level <= 50 {
                    let award_seed = LawnApp::get_award_seed_for_level(board_level);
                    crate::lawn::plant::Plant::preload_plant_resources(award_seed);
                }
            }
        }

        if self.m_crazy_dave_dialog_start != -1 {
            crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::CrazyDave);
        }
        if let Some(app) = self.get_app() {
            let has_rake = app.player_info.as_ref().map_or(false, |info| {
                info.m_purchases.get(StoreItem::Rake as usize).copied().unwrap_or(0) != 0
            });
            if has_rake {
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::Rake);
            }
            if app.game_mode == GameMode::ChallengeZenGarden {
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Sprout);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Marigold);
            }
        }

        if let Some(board) = self.get_board() {
            if board.stage_has_roof() {
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::RoofCleaner);
            } else {
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::Lawnmower);
            }
            if board.stage_has_pool() {
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::Splash);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::PoolCleaner);
            }
            if board.can_drop_loot() {
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::CoinSilver);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::CoinGold);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::Diamond);
            }
        }

        if self.m_sod_time > 0 {
            crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::Sodroll);
        }
        if let Some(app_ptr) = self.app {
            let app = unsafe { &*app_ptr };
            if app.game_mode == GameMode::ChallengePortalCombat {
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::PortalCircle);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::PortalSquare);
            }
            if app.is_whack_a_zombie_level() || app.is_scary_potter_level() {
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::Hammer);
            }
            if app.is_stormy_night_level() || app.game_mode == GameMode::ChallengeRainingSeeds {
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::RainCircle);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::RainSplash);
            }
            if app.game_mode == GameMode::ChallengeZenGarden {
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::ZengardenWateringcan);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::ZengardenFertilizer);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::ZengardenBugspray);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::ZengardenPhonograph);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::Stinky);
            }
            if app.game_mode == GameMode::ChallengeTreeOfWisdom {
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::ZengardenFertilizer);
            }
            if app.game_mode == GameMode::Upsell {
                self.m_loaded_resource_names.push("DelayLoad_Background3".to_string());
                self.m_loaded_resource_names.push("DelayLoad_Background4".to_string());
                self.m_loaded_resource_names.push("DelayLoad_Background5".to_string());
                self.m_loaded_resource_names.push("DelayLoad_ChallengeScreen".to_string());
                crate::lawn::zombie::Zombie::preload_zombie_resources(ZombieType::Normal);
                crate::lawn::zombie::Zombie::preload_zombie_resources(ZombieType::TrafficCone);
                crate::lawn::zombie::Zombie::preload_zombie_resources(ZombieType::Pail);
                crate::lawn::zombie::Zombie::preload_zombie_resources(ZombieType::Zamboni);
                crate::lawn::zombie::Zombie::preload_zombie_resources(ZombieType::Pogo);
                crate::lawn::zombie::Zombie::preload_zombie_resources(ZombieType::Balloon);
                crate::lawn::zombie::Zombie::preload_zombie_resources(ZombieType::Catapult);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Squash);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Threepeater);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Magnetshroom);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Lilypad);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Torchwood);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Spikeweed);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Tanglekelp);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Sunflower);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Peashooter);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Sunshroom);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Sunshroom); // 故意加载两次（对应 C++）
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Flowerpot);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Plantern);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Fumeshroom);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Cactus);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Puffshroom);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Seashroom);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Cabbagepult);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Wallnut);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Chomper);
            }
            if app.game_mode == GameMode::Intro {
                self.m_loaded_resource_names.push("DelayLoad_Background3".to_string());
                self.m_loaded_resource_names.push("DelayLoad_Credits".to_string());
                crate::lawn::zombie::Zombie::preload_zombie_resources(ZombieType::Normal);
                crate::lawn::zombie::Zombie::preload_zombie_resources(ZombieType::TrafficCone);
                crate::lawn::zombie::Zombie::preload_zombie_resources(ZombieType::Pail);
                crate::lawn::zombie::Zombie::preload_zombie_resources(ZombieType::Zamboni);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Sunflower);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Peashooter);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Squash);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Threepeater);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Lilypad);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Torchwood);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Spikeweed);
                crate::lawn::plant::Plant::preload_plant_resources(SeedType::Tanglekelp);
            }
        }

        if let Some(app_ptr) = self.app {
            unsafe {
                if let Some(rm) = (*app_ptr).base.resource_manager.as_mut() {
                    for resource in &self.m_loaded_resource_names {
                        let _ = (**rm).load_resources(resource);
                    }
                }
            }
        }

        self.place_street_zombies();

        // [TRANSLATION_NOTE]: C++ 中 mBoard->mPreloadTime = aTimer.GetDuration() 记录预加载耗时
        //（用于进度条）；Rust 侧 board 无 m_preload_time 字段
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
        // 对应 C++ ClearUpsellBoard：重置冰面/清空实体并销毁粒子与动画
        if let Some(board) = self.board {
            unsafe {
                for i in 0..MAX_GRID_SIZE_Y {
                    (*board).m_ice_timer[i] = 0;
                    (*board).m_ice_min_x[i] = crate::lawn::game_enums::BOARD_WIDTH;
                }
                (*board).zombies.clear();
                (*board).plants.clear();
                (*board).coins.clear();
                (*board).projectiles.clear();
                (*board).grid_items.clear();
                (*board).lawn_mowers.clear();
                // [TRANSLATION_NOTE]: C++ 中 mPoolSparklyParticleID = PARTICLESYSTEMID_NULL；
                // Rust 侧 board 无该字段
            }
        }
        if let Some(app) = self.app {
            unsafe {
                if let Some(es) = (*app).effect_system.as_mut() {
                    // C++ 中遍历粒子系统调用 ParticleSystemDie（跳过已死）
                    for ps in es.particle_systems.iter_mut() {
                        if !ps.dead {
                            ps.particle_system_die();
                        }
                    }
                    // C++ 中保留 CrazyDave 与其眨眼动画，其余全部 ReanimationDie
                    let dave_id = (*app).m_crazy_dave_reanim_id;
                    let blink_id = (*app).m_crazy_dave_blink_reanim_id;
                    for reanim in es.reanimations.iter_mut() {
                        if !reanim.m_dead && reanim.id != dave_id && reanim.id != blink_id {
                            reanim.reanimation_die();
                        }
                    }
                }
            }
        }
        self.m_upsell_challenge_screen = None;
    }

    /// 加载入场面板
    pub fn load_intro_board(&mut self) {
        // 对应 C++ LoadIntroBoard：布置演示植物与僵尸并预演 100 帧
        self.clear_upsell_board();
        if let Some(app) = self.app {
            unsafe { (*app).m_mute_sounds_for_cutscene = true; }
        }

        if let Some(board) = self.board {
            unsafe {
                let b = &mut *board;
                let _ = b.new_plant(0, 1, SeedType::Threepeater, SeedType::None);
                let _ = b.new_plant(0, 2, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(0, 2, SeedType::Peashooter, SeedType::None);
                let _ = b.new_plant(0, 3, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(0, 3, SeedType::Peashooter, SeedType::None);
                let _ = b.new_plant(0, 4, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(1, 0, SeedType::Threepeater, SeedType::None);
                let _ = b.new_plant(1, 1, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(1, 2, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(1, 2, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(1, 4, SeedType::Threepeater, SeedType::None);
                let _ = b.new_plant(1, 5, SeedType::Threepeater, SeedType::None);
                let _ = b.new_plant(2, 0, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(2, 1, SeedType::Peashooter, SeedType::None);
                let _ = b.new_plant(2, 3, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(2, 3, SeedType::Peashooter, SeedType::None);
                let _ = b.new_plant(2, 4, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(2, 5, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(3, 0, SeedType::Torchwood, SeedType::None);
                let _ = b.new_plant(3, 4, SeedType::Threepeater, SeedType::None);
                let _ = b.new_plant(4, 2, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(4, 2, SeedType::Torchwood, SeedType::None);
                let _ = b.new_plant(5, 1, SeedType::Torchwood, SeedType::None);
                let _ = b.new_plant(5, 4, SeedType::Torchwood, SeedType::None);
                let _ = b.new_plant(5, 5, SeedType::Torchwood, SeedType::None);
                let _ = b.new_plant(6, 0, SeedType::Spikeweed, SeedType::None);
                let _ = b.new_plant(6, 4, SeedType::Spikeweed, SeedType::None);
                let _ = b.new_plant(7, 1, SeedType::Spikeweed, SeedType::None);
            }
        }
        self.add_upsell_zombie(ZombieType::Normal, 460, 0);
        self.add_upsell_zombie(ZombieType::Football, 680, 0);
        self.add_upsell_zombie(ZombieType::TrafficCone, 730, 0);
        self.add_upsell_zombie(ZombieType::Normal, 810, 0);
        self.add_upsell_zombie(ZombieType::TrafficCone, 670, 1);
        self.add_upsell_zombie(ZombieType::Normal, 740, 1);
        self.add_upsell_zombie(ZombieType::Normal, 880, 1);
        self.add_upsell_zombie(ZombieType::Normal, 500, 2);
        self.add_upsell_zombie(ZombieType::TrafficCone, 680, 2);
        self.add_upsell_zombie(ZombieType::Pail, 604, 3);
        self.add_upsell_zombie(ZombieType::Snorkel, 880, 3);
        self.add_upsell_zombie(ZombieType::Normal, 600, 4);
        self.add_upsell_zombie(ZombieType::Pail, 690, 4);
        self.add_upsell_zombie(ZombieType::Normal, 780, 4);
        self.add_upsell_zombie(ZombieType::Catapult, 730, 5);
        self.add_upsell_zombie(ZombieType::Normal, 590, 5);

        self.m_pre_updating_board = true;
        if let Some(board) = self.board {
            for _ in 0..100 {
                unsafe { (*board).update(); }
            }
        }
        self.m_pre_updating_board = false;
    }

    /// 添加升级僵尸
    pub fn add_upsell_zombie(&mut self, zombie_type: ZombieType, pixel_x: i32, grid_y: i32) {
        // 对应 C++ AddUpsellZombie
        if let Some(board) = self.board {
            unsafe {
                let b = &mut *board;
                let idx = b.add_zombie_in_row(zombie_type, grid_y, 0);
                let zombie = &mut b.zombies[idx];
                zombie.pos_x = pixel_x as f32;
                zombie.pos_y = zombie.get_pos_y_based_on_row(grid_y);
                zombie.set_row(grid_y);
                // [TRANSLATION_NOTE]: C++ 中 mX/mY = (int)mPosX/mPosY；Rust 侧 zombie 无 x/y 字段
            }
        }
    }

    /// 加载升级面板（泳池）
    pub fn load_upsell_board_pool(&mut self) {
        // 对应 C++ LoadUpsellBoardPool
        self.clear_upsell_board();
        if let Some(app) = self.app {
            unsafe { (*app).m_mute_sounds_for_cutscene = true; }
        }

        if let Some(board) = self.board {
            unsafe {
                let b = &mut *board;
                let _ = b.new_plant(0, 1, SeedType::Threepeater, SeedType::None);
                let _ = b.new_plant(0, 2, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(0, 2, SeedType::Peashooter, SeedType::None);
                let _ = b.new_plant(0, 3, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(0, 3, SeedType::Peashooter, SeedType::None);
                let _ = b.new_plant(0, 4, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(1, 0, SeedType::Threepeater, SeedType::None);
                let _ = b.new_plant(1, 1, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(1, 2, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(1, 2, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(1, 4, SeedType::Threepeater, SeedType::None);
                let _ = b.new_plant(1, 5, SeedType::Threepeater, SeedType::None);
                let _ = b.new_plant(2, 0, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(2, 1, SeedType::Peashooter, SeedType::None);
                let _ = b.new_plant(2, 3, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(2, 3, SeedType::Peashooter, SeedType::None);
                let _ = b.new_plant(2, 4, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(2, 5, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(3, 4, SeedType::Threepeater, SeedType::None);
                let _ = b.new_plant(4, 0, SeedType::Torchwood, SeedType::None);
                let _ = b.new_plant(4, 2, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(4, 2, SeedType::Torchwood, SeedType::None);
                let _ = b.new_plant(5, 1, SeedType::Torchwood, SeedType::None);
                let _ = b.new_plant(5, 4, SeedType::Torchwood, SeedType::None);
                let _ = b.new_plant(5, 5, SeedType::Torchwood, SeedType::None);
                let _ = b.new_plant(6, 0, SeedType::Spikeweed, SeedType::None);
                let _ = b.new_plant(6, 3, SeedType::Tanglekelp, SeedType::None);
                let _ = b.new_plant(6, 4, SeedType::Spikeweed, SeedType::None);
                let _ = b.new_plant(6, 5, SeedType::Squash, SeedType::None);
                let _ = b.new_plant(7, 1, SeedType::Spikeweed, SeedType::None);
            }
        }
        self.add_upsell_zombie(ZombieType::Normal, 460, 0);
        self.add_upsell_zombie(ZombieType::Zamboni, 680, 0);
        self.add_upsell_zombie(ZombieType::TrafficCone, 670, 1);
        self.add_upsell_zombie(ZombieType::Normal, 740, 1);
        self.add_upsell_zombie(ZombieType::Normal, 500, 2);
        self.add_upsell_zombie(ZombieType::TrafficCone, 680, 2);
        self.add_upsell_zombie(ZombieType::Normal, 604, 3);
        self.add_upsell_zombie(ZombieType::Normal, 690, 4);
        self.add_upsell_zombie(ZombieType::Normal, 740, 4);
        self.add_upsell_zombie(ZombieType::Pail, 730, 5);
        self.add_upsell_zombie(ZombieType::Normal, 590, 5);

        self.m_pre_updating_board = true;
        if let Some(board) = self.board {
            for _ in 0..100 {
                unsafe { (*board).update(); }
            }
        }
        self.m_pre_updating_board = false;
        if let Some(app) = self.app {
            unsafe { (*app).m_mute_sounds_for_cutscene = false; }
        }
    }

    /// 加载升级面板（雾）
    pub fn load_upsell_board_fog(&mut self) {
        // 对应 C++ LoadUpsellBoardFog
        self.clear_upsell_board();
        if let Some(app) = self.app {
            unsafe { (*app).m_mute_sounds_for_cutscene = true; }
        }

        if let Some(board) = self.board {
            unsafe {
                // [TRANSLATION_NOTE]: C++ 中 BackgroundType::BACKGROUND_4_FOG；Rust 用 Fog 近似
                (*board).m_background_type = BackgroundType::Fog;
                (*board).load_background_images(self.app.unwrap());
            }
        }

        if let Some(board) = self.board {
            unsafe {
                let b = &mut *board;
                let _ = b.new_plant(0, 1, SeedType::Sunshroom, SeedType::None);
                let _ = b.new_plant(0, 4, SeedType::Sunshroom, SeedType::None);
                let _ = b.new_plant(1, 0, SeedType::Sunshroom, SeedType::None);
                let _ = b.new_plant(1, 1, SeedType::Sunshroom, SeedType::None);
                let _ = b.new_plant(1, 2, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(1, 2, SeedType::Cactus, SeedType::None);
                let _ = b.new_plant(1, 4, SeedType::Sunshroom, SeedType::None);
                let _ = b.new_plant(1, 5, SeedType::Sunshroom, SeedType::None);
                let _ = b.new_plant(2, 0, SeedType::Cactus, SeedType::None);
                let _ = b.new_plant(2, 4, SeedType::Cactus, SeedType::None);
                let _ = b.new_plant(2, 5, SeedType::Fumeshroom, SeedType::None);
                let _ = b.new_plant(3, 1, SeedType::Fumeshroom, SeedType::None);
                let _ = b.new_plant(3, 2, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(3, 3, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(3, 3, SeedType::Cactus, SeedType::None);
                let _ = b.new_plant(3, 5, SeedType::Puffshroom, SeedType::None);
                let _ = b.new_plant(4, 0, SeedType::Puffshroom, SeedType::None);
                let _ = b.new_plant(4, 1, SeedType::Magnetshroom, SeedType::None);
                let _ = b.new_plant(4, 2, SeedType::Seashroom, SeedType::None);
                let _ = b.new_plant(4, 5, SeedType::Puffshroom, SeedType::None);
                let _ = b.new_plant(5, 1, SeedType::Puffshroom, SeedType::None);
                let _ = b.new_plant(5, 2, SeedType::Lilypad, SeedType::None);
                let _ = b.new_plant(5, 2, SeedType::Plantern, SeedType::None);
                let _ = b.new_plant(5, 3, SeedType::Seashroom, SeedType::None);
                let _ = b.new_plant(6, 2, SeedType::Seashroom, SeedType::None);
                let _ = b.new_plant(6, 3, SeedType::Seashroom, SeedType::None);
            }
        }
        self.add_upsell_zombie(ZombieType::Normal, 460, 0);
        self.add_upsell_zombie(ZombieType::Normal, 680, 0);
        self.add_upsell_zombie(ZombieType::Balloon, 780, 0);
        self.add_upsell_zombie(ZombieType::TrafficCone, 670, 1);
        self.add_upsell_zombie(ZombieType::Balloon, 640, 1);
        self.add_upsell_zombie(ZombieType::Pail, 640, 2);
        self.add_upsell_zombie(ZombieType::TrafficCone, 780, 3);
        self.add_upsell_zombie(ZombieType::Balloon, 704, 4);
        self.add_upsell_zombie(ZombieType::Normal, 690, 4);
        self.add_upsell_zombie(ZombieType::Pail, 590, 5);
        self.add_upsell_zombie(ZombieType::Normal, 740, 5);

        self.m_pre_updating_board = true;
        if let Some(board) = self.board {
            for _ in 0..100 {
                unsafe { (*board).update(); }
            }
        }
        self.m_pre_updating_board = false;
        if let Some(app) = self.app {
            unsafe { (*app).m_mute_sounds_for_cutscene = false; }
        }
    }

    /// 加载挑战升级面板
    pub fn load_upsell_challenge_screen(&mut self) {
        // 对应 C++ LoadUpsellChallengeScreen：创建挑战选择界面
        self.clear_upsell_board();
        let mut screen = Box::new(crate::lawn::widget::challenge_screen::ChallengeScreen::new());
        screen.app = self.app;
        screen.page_index = ChallengePage::Challenge;
        self.m_upsell_challenge_screen = Some(Box::into_raw(screen));
    }

    /// 加载升级面板（屋顶）
    pub fn load_upsell_board_roof(&mut self) {
        // 对应 C++ LoadUpsellBoardRoof
        self.clear_upsell_board();
        if let Some(app) = self.app {
            unsafe { (*app).m_mute_sounds_for_cutscene = true; }
        }

        if let Some(board) = self.board {
            unsafe {
                // [TRANSLATION_NOTE]: C++ 中 BackgroundType::BACKGROUND_5_ROOF；Rust 用 Roof 近似
                (*board).m_background_type = BackgroundType::Roof;
                (*board).load_background_images(self.app.unwrap());
                for y in 0..MAX_GRID_SIZE_Y {
                    (*board).m_plant_row[y] = PlantRowType::Normal;
                }
                (*board).m_plant_row[5] = PlantRowType::Dirt;
                for x in 0..crate::lawn::board::MAX_GRID_SIZE_X {
                    for y in 0..MAX_GRID_SIZE_Y {
                        // C++ 中 mGridSquareType[x][y]；Rust 侧 grid_square_type 为 [y][x]
                        if (*board).m_plant_row[y] == PlantRowType::Dirt {
                            (*board).grid_square_type[y][x] = GridSquareType::Dirt;
                        } else {
                            (*board).grid_square_type[y][x] = GridSquareType::Grass;
                        }
                    }
                }
            }
        }

        if let Some(board) = self.board {
            unsafe {
                let b = &mut *board;
                let _ = b.new_plant(0, 0, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(0, 0, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(0, 1, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(0, 1, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(0, 2, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(0, 2, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(0, 3, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(0, 3, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(0, 4, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(0, 4, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(1, 0, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(1, 0, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(1, 1, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(1, 1, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(1, 2, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(1, 2, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(1, 3, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(1, 3, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(1, 4, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(1, 4, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(2, 0, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(2, 0, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(2, 1, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(2, 1, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(2, 2, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(2, 2, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(2, 3, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(2, 3, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(2, 4, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(2, 4, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(3, 1, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(3, 1, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(3, 2, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(3, 2, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(3, 3, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(3, 3, SeedType::Sunflower, SeedType::None);
                let _ = b.new_plant(3, 4, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(3, 4, SeedType::Cabbagepult, SeedType::None);
                let _ = b.new_plant(4, 0, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(4, 0, SeedType::Chomper, SeedType::None);
                let _ = b.new_plant(4, 1, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(4, 1, SeedType::Chomper, SeedType::None);
                let _ = b.new_plant(4, 2, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(4, 2, SeedType::Repeater, SeedType::None);
                let _ = b.new_plant(4, 3, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(5, 2, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(5, 2, SeedType::Wallnut, SeedType::None);
                let _ = b.new_plant(5, 3, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(5, 3, SeedType::Threepeater, SeedType::None);
                let _ = b.new_plant(5, 4, SeedType::Flowerpot, SeedType::None);
                let _ = b.new_plant(5, 4, SeedType::Wallnut, SeedType::None);
            }
        }
        self.add_upsell_zombie(ZombieType::Normal, 460, 0);
        self.add_upsell_zombie(ZombieType::Normal, 680, 0);
        self.add_upsell_zombie(ZombieType::Catapult, 780, 1);
        self.add_upsell_zombie(ZombieType::TrafficCone, 670, 1);
        self.add_upsell_zombie(ZombieType::Normal, 580, 0);
        self.add_upsell_zombie(ZombieType::Normal, 540, 1);
        self.add_upsell_zombie(ZombieType::Pail, 500, 1);
        self.add_upsell_zombie(ZombieType::Pail, 640, 2);
        self.add_upsell_zombie(ZombieType::TrafficCone, 780, 3);
        self.add_upsell_zombie(ZombieType::Normal, 380, 3);
        self.add_upsell_zombie(ZombieType::Catapult, 704, 4);
        self.add_upsell_zombie(ZombieType::Normal, 690, 4);
        self.add_upsell_zombie(ZombieType::Normal, 590, 4);

        self.m_pre_updating_board = true;
        if let Some(board) = self.board {
            for _ in 0..100 {
                unsafe { (*board).update(); }
            }
        }
        self.m_pre_updating_board = false;
        if let Some(app) = self.app {
            unsafe { (*app).m_mute_sounds_for_cutscene = false; }
        }
    }

    /// 更新升级
    pub fn update_upsell(&mut self) {
        // 对应 C++ UpdateUpsell
        // 光标：按钮未悬停时恢复指针（C++ CURSOR_POINTER）
        if let Some(board) = self.board {
            unsafe {
                let menu_over = (*board).menu_button.map_or(false, |p| unsafe { (*p).is_over });
                let store_over = (*board).store_button.map_or(false, |p| unsafe { (*p).is_over });
                if !menu_over && !store_over {
                    if let Some(app) = self.app {
                        // [TRANSLATION_NOTE]: C++ 中 mApp->SetCursor(CURSOR_POINTER)；Rust 侧基类光标未接入
                        let _ = app;
                    }
                }
            }
        }

        let dave_state = self.app.map_or(CrazyDaveState::Off, |app| unsafe { (*app).m_crazy_dave_state });
        if dave_state == CrazyDaveState::Off || dave_state == CrazyDaveState::Entering {
            return;
        }

        if self.m_crazy_dave_last_talk_index == -1 {
            if let Some(app) = self.app {
                unsafe { (*app).crazy_dave_talk_index(self.m_crazy_dave_dialog_start); }
            }
            self.m_crazy_dave_last_talk_index = self.m_crazy_dave_dialog_start;
            self.m_crazy_dave_dialog_start = -1;
            self.m_crazy_dave_count_down = self.parse_talk_time_from_message();
            return;
        }

        if self.m_crazy_dave_count_down > 0 {
            self.m_crazy_dave_count_down -= 1;
        }

        // "Uh, what are you waiting for?"
        if self.m_crazy_dave_last_talk_index == 3317 {
            if self.m_crazy_dave_count_down == 0 {
                if let Some(board) = self.board {
                    unsafe {
                        if let Some(store) = (*board).get_store_button_mut() {
                            store.resize(510, 420, 210, 46);
                            store.btn_no_draw = false;
                        }
                        if let Some(menu) = (*board).get_menu_button_mut() {
                            menu.resize(510, 480, 210, 46);
                            menu.btn_no_draw = false;
                        }
                    }
                }
            }
            return;
        }
        // "You want to take action?"
        if self.m_crazy_dave_last_talk_index == 3311 && self.m_crazy_dave_count_down == 90 {
            if let Some(app) = self.app {
                unsafe {
                    if let Some(music) = (*app).music.as_mut() {
                        music.make_sure_music_is_playing(MusicTune::MinigameLoonboon);
                    }
                }
            }
        }

        if self.m_crazy_dave_count_down != 0 {
            return;
        }

        let dave_message = self.app.map_or(-1, |app| unsafe { (*app).m_crazy_dave_message_index });
        if dave_message != -1 {
            self.m_crazy_dave_count_down = self.parse_delay_time_from_message();
            if let Some(app) = self.app {
                unsafe { (*app).crazy_dave_stop_talking(); }
            }
            return;
        }

        if let Some(app) = self.app {
            unsafe { (*app).crazy_dave_talk_index(self.m_crazy_dave_last_talk_index + 1); }
        }
        self.m_crazy_dave_last_talk_index += 1;
        self.m_crazy_dave_count_down = self.parse_talk_time_from_message();

        // [TRANSLATION_NOTE]: C++ 中 3305/3306/3307/3309 分支使用 AttachReanim 悬挂
        // 植物/磁力菇动画到戴夫身上（Rust attachment 系统为骨架），此处省略具体悬挂。
        // 3312/3313/3314/3315/3316/3317 分支加载各升级面板。
        match self.m_crazy_dave_last_talk_index {
            3312 | 3313 | 3314 | 3316 => {
                if let Some(app) = self.app {
                    unsafe {
                        if let Some(music) = (*app).music.as_mut() {
                            music.make_sure_music_is_playing(MusicTune::MinigameLoonboon);
                        }
                    }
                }
                match self.m_crazy_dave_last_talk_index {
                    3312 => self.load_upsell_board_pool(),
                    3313 => self.load_upsell_board_fog(),
                    3314 => self.load_upsell_challenge_screen(),
                    _ => self.load_upsell_board_roof(),
                }
                // [TRANSLATION_NOTE]: C++ 中 PlaySample(SOUND_FINALWAVE/SOUND_HUGE_WAVE)
                self.m_upsell_hide_board = false;
            }
            3315 => {
                // "Terra cotta!!!"
                self.clear_upsell_board();
                // C++: mApp->PlaySample(Sexy::SOUND_FINALWAVE);
                if let Some(app) = self.app {
                    unsafe { (*app).play_sample(crate::todlib::tod_foley::SOUND_FINALWAVE); }
                }
                self.m_upsell_hide_board = true;
                if let Some(app) = self.app {
                    let render_position = crate::lawn::board::make_render_order(
                        crate::lawn::game_enums::RENDER_LAYER_SCREEN_FADE, 0, 0,
                    );
                    unsafe {
                        (*app).add_tod_particle(592.0, 240.0, render_position, ParticleEffect::PresentPickUpArrow as i32);
                    }
                }
            }
            3317 => {
                // "Uh, what are you waiting for?"
                self.clear_upsell_board();
                if let Some(board) = self.board {
                    unsafe {
                        if let Some(menu) = (*board).get_menu_button_mut() {
                            menu.btn_no_draw = true;
                        }
                    }
                }
                self.m_upsell_hide_board = true;
            }
            _ => {}
        }
    }

    /// 绘制升级
    pub fn draw_upsell(&mut self, g: &mut Graphics) {
        // 对应 C++ DrawUpsell
        if self.m_crazy_dave_last_talk_index == 3315 {
            // "Terra cotta!"：绘制花盆动画 + 菜单按钮
            let mut a_reanim = crate::todlib::reanimator::Reanimation::new();
            a_reanim.reanimation_initialize_type(565.0, 360.0, ReanimationType::FlowerPot);
            a_reanim.set_frames_for_layer("anim_zengarden");
            a_reanim.override_scale(1.3, 1.3);
            a_reanim.draw(g);
            // [TRANSLATION_NOTE]: C++ 中 mBoard->mMenuButton->Draw(g)；Rust 侧按钮为 Option<i32>
            a_reanim.reanimation_die();
        }

        if let Some(s) = self.m_upsell_challenge_screen {
            unsafe { (*s).draw(g); }
            // [TRANSLATION_NOTE]: C++ 中 mBoard->mMenuButton->Draw(g)
        }
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
                // C++: mApp->PlaySample(Sexy::SOUND_HUGE_WAVE);
                app.play_sample(unsafe { crate::todlib::tod_foley::SOUND_HUGE_WAVE });
                app.m_mute_sounds_for_cutscene = true;
            }
        }
        if scene_time == TIME_INTRO_FADE_OUT - 200 {
            if let Some(app) = self.get_app_mut() {
                app.m_mute_sounds_for_cutscene = false;
                // C++: mApp->PlaySample(Sexy::SOUND_SIREN);
                app.play_sample(unsafe { crate::todlib::tod_foley::SOUND_SIREN });
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
    pub fn draw_intro(&mut self, g: &mut Graphics) {
        // 对应 C++ DrawIntro
        const TIME_INTRO_PRESENTS_FADE_IN: i32 = 1000;
        const TIME_INTRO_LOGO_START: i32 = 5500;
        const TIME_INTRO_LOGO_END: i32 = 5900;
        const TIME_INTRO_PAN_RIGHT_START: i32 = 5890;
        const TIME_INTRO_PAN_RIGHT_END: i32 = 11890;
        const TIME_INTRO_FADE_OUT: i32 = 10890;
        const TIME_INTRO_FADE_OUT_END: i32 = 11890;

        let scene_time = self.m_cutscene_time;
        // [TRANSLATION_NOTE]: C++ 中 mBoard->mX/mY 为棋盘偏移；Rust 侧 board 无偏移字段，以 0 近似
        let board_offset_x = 0;
        let board_offset_y = 0;

        if scene_time <= TIME_INTRO_PAN_RIGHT_START || scene_time > TIME_INTRO_FADE_OUT_END {
            g.set_color(&crate::framework::color::Color::from_rgb(0, 0, 0));
            g.fill_rect_xywh(-board_offset_x, -board_offset_y, crate::lawn::game_enums::BOARD_WIDTH, crate::lawn::game_enums::BOARD_HEIGHT);
        }

        // "PopCap Games presents" 文字
        let a_time_pan_right_start = TIME_INTRO_PAN_RIGHT_START - TIME_INTRO_PRESENTS_FADE_IN;
        if scene_time > TIME_INTRO_PRESENTS_FADE_IN && scene_time <= a_time_pan_right_start {
            let an_alpha = if scene_time < a_time_pan_right_start - 600 {
                crate::todlib::tod_common::tod_animate_curve(
                    TIME_INTRO_PRESENTS_FADE_IN, TIME_INTRO_PRESENTS_FADE_IN + 300, scene_time, 0, 255,
                    crate::lawn::game_enums::TodCurves::Linear,
                )
            } else {
                crate::todlib::tod_common::tod_animate_curve(
                    a_time_pan_right_start - 600, a_time_pan_right_start - 300, scene_time, 255, 0,
                    crate::lawn::game_enums::TodCurves::Linear,
                )
            };
            // [TRANSLATION_NOTE]: C++ 中 PvzpDrawString(FONT_BRIANNETOD32, Color(255,255,255,anAlpha))；
            // Rust 侧字体/alpha 未接入，以 draw_string 近似
            let _alpha = an_alpha;
            g.draw_string("[INTRO_PRESENTS]", crate::lawn::game_enums::BOARD_WIDTH / 2 - board_offset_x, 310 - board_offset_y);
        }

        // "Plants Vs Zombies" 标志
        if scene_time > TIME_INTRO_LOGO_START && scene_time <= TIME_INTRO_PAN_RIGHT_END {
            let a_scale = crate::todlib::tod_common::tod_animate_curve_float(
                TIME_INTRO_LOGO_START, TIME_INTRO_LOGO_END, scene_time, 5.0, 1.0,
                crate::lawn::game_enums::TodCurves::EaseOut,
            );
            let a_center = a_scale * 0.5;
            let a_offset_x = crate::lawn::game_enums::BOARD_WIDTH / 2 - board_offset_x;
            let a_offset_y = crate::lawn::game_enums::BOARD_HEIGHT / 2 - board_offset_y;
            let a_rect = crate::framework::rect::Rect::new(
                a_offset_x - (crate::lawn::game_enums::BOARD_WIDTH as f32 * a_center) as i32,
                a_offset_y - (75.0 * a_scale) as i32,
                (crate::lawn::game_enums::BOARD_WIDTH as f32 * a_scale) as i32,
                (150.0 * a_scale) as i32,
            );
            g.set_color(&crate::framework::color::Color::from_rgb(0, 0, 0));
            g.fill_rect(&a_rect);
            // [TRANSLATION_NOTE]: C++ 中 PvzpDrawImageScaledF(g, IMAGE_PVZ_LOGO, ...) 绘制 Logo；
            // Rust 侧图片资源未接入，暂略
        }

        if scene_time > TIME_INTRO_FADE_OUT && scene_time <= TIME_INTRO_FADE_OUT_END {
            g.set_color(&crate::framework::color::Color::from_rgb(0, 0, 0));
            g.fill_rect_xywh(-board_offset_x, -board_offset_y, crate::lawn::game_enums::BOARD_WIDTH, crate::lawn::game_enums::BOARD_HEIGHT);
        }
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


