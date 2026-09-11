// PvZ Portable Rust 翻译 — LawnApp（游戏应用主类）
// 对应 C++ src/LawnApp.h / LawnApp.cpp
//
// 游戏的核心调度类，管理游戏状态、屏幕切换、资源加载等。

use std::collections::LinkedList;
use crate::lawn::game_enums::*;
use crate::lawn::board::Board;
use crate::lawn::zen_garden::ZenGarden;
use crate::framework::sexy_app_base::SexyAppBase;
use crate::framework::common::{file_exists, string_to_lower};
use crate::lawn::system::save_game::lawn_save_game;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::widget::dialog::{Dialog, BUTTONS_FOOTER, BUTTONS_OK_CANCEL};
use crate::framework::widget::button_widget::ButtonWidget;
use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::widget::title_screen::{TitleScreenImpl, TitleState};
use crate::lawn::widget::game_selector::GameSelectorImpl;
use crate::todlib::reanimator::Reanimation;
use crate::todlib::tod_particle::TodParticleSystem;
use crate::todlib::tod_foley::{FoleyManager, FoleyType};
use crate::todlib::effect_system::EffectSystem;
use crate::lawn::system::reanimation_lawn::ReanimatorCache;
use crate::lawn::game_enums::TrialType;

/// 游戏场景枚举（对应 C++ GameScenes）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameScenes {
    MainMenu = 0,
    Playing = 1,
    ZenGarden = 2,
    Store = 3,
    Credits = 4,
    Award = 5,
    Challenge = 6,
    SeedChooser = 7,
    GameSelector = 8,
    LevelIntro = 9,
    ZombiesWon = 10,
}

/// 关卡统计（对应 C++ LevelStats）
#[derive(Debug, Clone, Copy)]
pub struct LevelStats {
    pub unused_lawn_mowers: i32,
}

impl LevelStats {
    pub fn new() -> Self { LevelStats { unused_lawn_mowers: 0 } }
    pub fn reset(&mut self) { self.unused_lawn_mowers = 0; }
}

/// 按钮列表、图像列表类型
pub type ButtonList = LinkedList<*mut ButtonWidget>;
pub type ImageList = LinkedList<*mut Image>;

/// 游戏应用类（对应 C++ LawnApp : SexyApp）
pub struct LawnApp {
    // ---- SexyAppBase 继承 ----
    pub base: SexyAppBase,

    // ---- 游戏屏幕/面板 ----
    pub board: Option<*mut Board>,
    pub title_screen: Option<*mut Widget>,
    pub game_selector: Option<*mut ()>,
    pub seed_chooser_screen: Option<*mut ()>,
    pub award_screen: Option<*mut ()>,
    pub almanac_dialog: Option<*mut ()>,
    pub credit_screen: Option<*mut ()>,
    pub challenge_screen: Option<*mut ()>,
    pub store_screen: Option<*mut ()>,
    /// 商店 Widget 包装指针（Box 拥有；store_screen 字段拥有 StoreScreen 本体）
    pub store_screen_widget: Option<*mut Widget>,
    /// 各屏幕的 Widget 包装指针（Box 拥有；对应字段拥有屏幕本体）
    pub award_screen_widget: Option<*mut Widget>,
    pub seed_chooser_screen_widget: Option<*mut Widget>,
    pub challenge_screen_widget: Option<*mut Widget>,
    pub credit_screen_widget: Option<*mut Widget>,
    pub zen_garden: Option<*mut ZenGarden>,
    // ---- 对话框持有（对应 C++ WidgetManager 中 AddDialog 的对话框实例；Rust 独立类以裸指针持有）----
    pub user_dialog: Option<*mut crate::lawn::widget::user_dialog::UserDialog>,
    pub new_user_dialog: Option<*mut crate::lawn::widget::new_user_dialog::NewUserDialog>,
    pub rename_user_dialog: Option<*mut crate::lawn::widget::new_user_dialog::NewUserDialog>,
    pub cheat_dialog: Option<*mut crate::lawn::widget::cheat_dialog::CheatDialog>,
    pub new_options_dialog: Option<*mut crate::lawn::widget::new_options_dialog::NewOptionsDialog>,

    // ---- 系统/管理器 ----
    pub sound_system: Option<Box<FoleyManager>>,
    pub effect_system: Option<Box<EffectSystem>>,
    pub profile_mgr: Option<Box<crate::lawn::system::profile_mgr::ProfileMgr>>,
    pub player_info: Option<Box<crate::lawn::system::player_info::PlayerInfo>>,
    pub music: Option<Box<crate::lawn::system::music::Music>>,
    pub pool_effect: Option<Box<crate::lawn::system::pool_effect::PoolEffect>>,

    // ---- 列表 ----
    pub control_button_list: ButtonList,
    pub created_image_list: ImageList,

    // ---- 游戏状态 ----
    pub game_mode: GameMode,
    pub game_scene: GameScenes,
    pub board_result: BoardResult,
    pub m_level: i32,
    pub m_num_levels: i32,
    pub m_close_request: bool,
    pub m_app_counter: u32,
    pub m_app_rand_seed: i32,

    // ---- 调试/作弊 ----
    pub m_debug_keys_enabled: bool,
    pub m_cheat_keys_used: bool,
    pub m_tod_cheat_keys: bool,
    pub m_easy_planting_cheat: bool,

    // ---- Crazy Dave ----
    pub m_crazy_dave_reanim_id: ReanimationID,
    pub m_crazy_dave_state: CrazyDaveState,
    pub m_crazy_dave_blink_counter: i32,
    pub m_crazy_dave_blink_reanim_id: ReanimationID,
    pub m_crazy_dave_message_index: i32,
    pub m_crazy_dave_message_text: String,

    // ---- 打字检查（秘籍输入） ----
    pub m_konami_check: Option<Box<crate::lawn::system::typing_check::TypingCheck>>,
    pub m_mustache_check: Option<Box<crate::lawn::system::typing_check::TypingCheck>>,
    pub m_moustache_check: Option<Box<crate::lawn::system::typing_check::TypingCheck>>,
    pub m_super_mower_check: Option<Box<crate::lawn::system::typing_check::TypingCheck>>,
    pub m_future_check: Option<Box<crate::lawn::system::typing_check::TypingCheck>>,
    pub m_pinata_check: Option<Box<crate::lawn::system::typing_check::TypingCheck>>,
    pub m_dance_check: Option<Box<crate::lawn::system::typing_check::TypingCheck>>,
    pub m_daisy_check: Option<Box<crate::lawn::system::typing_check::TypingCheck>>,
    pub m_sukhbir_check: Option<Box<crate::lawn::system::typing_check::TypingCheck>>,

    // ---- 秘籍模式 ----
    pub m_mustache_mode: bool,
    pub m_super_mower_mode: bool,
    pub m_future_mode: bool,
    pub m_pinata_mode: bool,
    pub m_dance_mode: bool,
    pub m_daisy_mode: bool,
    pub m_sukhbir_mode: bool,

    // ---- 游戏统计 ----
    pub m_games_played: i32,
    pub m_max_executions: i32,
    pub m_max_plays: i32,
    pub m_max_time: i32,
    pub m_play_time_active_session: i32,
    pub m_play_time_inactive_session: i32,
    pub m_saw_yeti: bool,

    // ---- 对话框 ----
    pub m_dialog_id: i32,
    pub m_tutorial_state: TutorialState,

    // ---- 注册/标识信息 ----
    pub m_refer_id: String,
    pub m_register_link: String,
    pub m_mod: String,
    pub m_register_resources_loaded: bool,

    // ---- 其他 ----
    pub m_mute_sounds_for_cutscene: bool,
    pub m_first_time_game_selector: bool,
    pub m_last_level_stats: Option<Box<LevelStats>>,

    // ---- 线程状态 ----
    pub m_loading_zombies_thread_completed: bool,

    // ---- 会话/试用 ----
    pub m_session_id: isize,
    pub m_trial_type: TrialType,
    pub m_debug_trial_locked: bool,

    // ---- 重动画缓存 ----
    pub m_reanimator_cache: Option<*mut ReanimatorCache>,

    // ---- 加载线程状态 ----
    pub m_loading_thread_started: bool,
    pub m_loading_thread_completed: bool,
    pub m_loading_thread_tasks_completed: i32,
    pub m_loading_thread_tasks_total: i32,
}

/// 全局游戏实例
static mut G_LAWN_APP_INSTANCE: Option<*mut LawnApp> = None;

/// 慢速/快速模式全局开关（对应 C++ LawnApp.h 的 extern bool gSlowMo/gFastMo/gSlowMoCounter）
pub static mut G_SLOW_MO: bool = false;
pub static mut G_FAST_MO: bool = false;
pub static mut G_SLOW_MO_COUNTER: i32 = 0;

/// 主循环帧钩子（对应 C++ 虚函数分派：LawnApp::UpdateFrames 覆写 SexyAppBase::UpdateFrames）。
/// 由 SexyAppBase::DoUpdateFrames 每帧调用，经全局实例驱动游戏世界更新。
fn drive_board_frame() {
    if let Some(app) = LawnApp::instance() {
        app.update_frames();
    }
}

impl LawnApp {
    pub fn new() -> Self {
        let mut app = LawnApp {
            base: SexyAppBase::new(),
            board: None, title_screen: None, game_selector: None,
            seed_chooser_screen: None, award_screen: None, almanac_dialog: None,
            credit_screen: None, challenge_screen: None, zen_garden: None,
            store_screen: None, store_screen_widget: None,
            award_screen_widget: None, seed_chooser_screen_widget: None,
            challenge_screen_widget: None, credit_screen_widget: None,
            user_dialog: None, new_user_dialog: None, rename_user_dialog: None,
            cheat_dialog: None, new_options_dialog: None,
            sound_system: None, effect_system: None,
            profile_mgr: None, player_info: None, music: None, pool_effect: None,
            control_button_list: LinkedList::new(),
            created_image_list: LinkedList::new(),
            game_mode: GameMode::Adventure,
            game_scene: GameScenes::MainMenu,
            board_result: BoardResult::None,
            m_level: 1, m_num_levels: NUM_LEVELS,
            m_close_request: false, m_app_counter: 0, m_app_rand_seed: 0,
            m_debug_keys_enabled: false, m_cheat_keys_used: false,
            m_tod_cheat_keys: false, m_easy_planting_cheat: false,
            m_crazy_dave_reanim_id: REANIMATIONID_NULL,
            m_crazy_dave_state: CrazyDaveState::Off,
            m_crazy_dave_blink_counter: 0,
            m_crazy_dave_blink_reanim_id: REANIMATIONID_NULL,
            m_crazy_dave_message_index: 0,
            m_crazy_dave_message_text: String::new(),
            m_konami_check: None, m_mustache_check: None,
            m_moustache_check: None, m_super_mower_check: None,
            m_future_check: None, m_pinata_check: None,
            m_dance_check: None, m_daisy_check: None, m_sukhbir_check: None,
            m_mustache_mode: false, m_super_mower_mode: false,
            m_future_mode: false, m_pinata_mode: false,
            m_dance_mode: false, m_daisy_mode: false, m_sukhbir_mode: false,
            m_games_played: 0, m_max_executions: 0, m_max_plays: 0,
            m_max_time: 0, m_play_time_active_session: 0,
            m_play_time_inactive_session: 0, m_saw_yeti: false,
            m_dialog_id: -1, m_tutorial_state: TutorialState::Off,
            m_mute_sounds_for_cutscene: false,
            m_first_time_game_selector: false,
            m_last_level_stats: None,
            m_register_resources_loaded: false,
            m_refer_id: String::new(),
            m_register_link: String::new(),
            m_mod: String::new(),
            m_loading_zombies_thread_completed: false,
            m_session_id: 0,
            m_trial_type: TrialType::None,
            m_debug_trial_locked: false,
            m_reanimator_cache: None,
            m_loading_thread_started: false,
            m_loading_thread_completed: false,
            m_loading_thread_tasks_completed: 0,
            m_loading_thread_tasks_total: 0,
        };

        // 挂载主循环帧钩子（对应 C++ 虚函数分派：LawnApp::UpdateFrames 覆写 SexyAppBase::UpdateFrames）
        app.base.lawn_frame_hook = Some(drive_board_frame);
        app
    }

    /// 全局单例访问
    pub fn instance() -> Option<&'static mut Self> {
        unsafe { G_LAWN_APP_INSTANCE.and_then(|p| p.as_mut()) }
    }

    // ==================== 生命周期 ====================

    /// 初始化（对应 C++ Init）
    pub fn init(&mut self) {
        unsafe { G_LAWN_APP_INSTANCE = Some(self as *mut LawnApp); }
        self.base.init();
        if self.base.is_shutdown() { return; }
        self.base.title = "PvZ Portable".to_string();

        // 创建子系统（对应 C++ LawnApp::Init 中的 new Music / new TodFoley / new EffectSystem）
        let app_ptr = self as *mut LawnApp;
        if self.music.is_none() {
            self.music = Some(Box::new(crate::lawn::system::music::Music::new_with_app(app_ptr)));
            eprintln!("[LawnApp] Music 创建完成");
        }
        if self.sound_system.is_none() {
            self.sound_system = Some(Box::new(crate::todlib::tod_foley::FoleyManager::new()));
            eprintln!("[LawnApp] FoleyManager 创建完成");
        }

        // 加载标题屏幕所需的图片资源
        if let Some(rm) = self.base.resource_manager {
            unsafe {
                let needed_images = ["popcap_logo", "partner_logo", "pvz_logo", "titlescreen",
                    "loadbar_dirt", "loadbar_grass", "reanim_sodrollcap"];
                for img_id in &needed_images {
                    let shared = (*rm).get_image(img_id);
                    if shared.unshared_image.is_null() && shared.shared_image.is_null() {
                        let key = string_to_lower(img_id);
                        if let Some(&ptr) = (*rm).image_map.get(&key) {
                            let res = &mut *(ptr as *mut crate::framework::resource_manager::ImageRes);
                            if !res.base.path.is_empty() {
                                (*rm).load_single_image(res);
                            }
                        }
                    }
                }
            }
        }

        // 创建 TitleScreen Widget + WidgetImpl
        let mut title_widget = Box::new(Widget::new());
        title_widget.impl_ = Some(Box::new(TitleScreenImpl::new(self as *mut LawnApp)));
        title_widget.resize(0, 0, self.base.width, self.base.height);
        let title_widget_ptr = Box::into_raw(title_widget);
        self.title_screen = Some(title_widget_ptr);

        // 注册到 WidgetManager
        if let Some(wm) = self.base.widget_manager {
            unsafe {
                (*wm).add_widget(title_widget_ptr);
                (*wm).set_focus(Some(title_widget_ptr));
            }
        }
    }

    /// 启动（对应 C++ Start）
    pub fn start(&mut self) {
        if self.base.is_shutdown() { return; }
        self.base.start();
    }

    /// 关闭（对应 C++ Shutdown）
    pub fn shutdown(&mut self) {
        // 清理 TitleScreen widget（必须在 base.shutdown 销毁 WidgetManager 之前）
        if let Some(ts) = self.title_screen.take() {
            if let Some(wm) = self.base.widget_manager {
                unsafe { (*wm).remove_widget(ts); }
            }
            unsafe { let _ = Box::from_raw(ts); }
        }
        self.kill_board();
        self.base.shutdown();
        unsafe { G_LAWN_APP_INSTANCE = None; }
    }

    /// 主更新循环（对应 C++ LawnApp::UpdateFrames，由 SexyAppBase 每帧经虚分派钩子调用）
    pub fn update_frames(&mut self) {
        // 对应 C++: if ((!mActive || mMinimized) && mBoard) mBoard->ResetFPSStats();
        if (!self.base.active || self.base.minimized) && self.board.is_some() {
            if let Some(board) = self.board {
                unsafe { (*board).reset_fps_stats(); }
            }
        }

        // 对应 C++: aUpdateCount 受 gSlowMo / gFastMo 控制
        let mut update_count = 1;
        unsafe {
            if G_SLOW_MO {
                G_SLOW_MO_COUNTER += 1;
                if G_SLOW_MO_COUNTER < 4 {
                    update_count = 0;
                } else {
                    G_SLOW_MO_COUNTER = 0;
                }
            } else if G_FAST_MO {
                update_count = 20;
            }
        }

        for _ in 0..update_count {
            self.m_app_counter += 1;

            // 对应 C++: if (mBoard) mBoard->ProcessDeleteQueue();
            if let Some(board) = self.board {
                unsafe { (*board).process_delete_queue(); }
            }
            // 对应 C++: if (mLoadingThreadCompleted && mEffectSystem) mEffectSystem->ProcessDeleteQueue();
            if self.m_loading_thread_completed {
                if let Some(effect_system) = self.effect_system.as_mut() {
                    effect_system.process_delete_queue();
                }
            }

            // 对应 C++: SexyApp::UpdateFrames() → mWidgetManager->UpdateFrame()，
            // 该步已在 SexyAppBase::DoUpdateFrames 中先行完成（Board 不在 WidgetManager 中，
            // 故在此直接驱动 Board::Update，与 C++ 中 Board 作为 Widget 参与 UpdateFrame 顺序等效）。

            // 更新游戏世界（对应 C++ Board 作为 Widget 的 Board::Update）
            if let Some(board_ptr) = self.board {
                unsafe { (*board_ptr).update(); }
            }

            // 对应 C++: mMusic->MusicUpdate();
            if let Some(music) = self.music.as_mut() {
                music.music_update();
            }

            // 对应 C++: CheckForGameEnd();
            self.check_for_game_end();
        }
    }

    /// 更新应用单步（对应 C++ UpdateAppStep）
    pub fn update_app_step(&mut self, _updated: Option<&mut bool>) -> bool {
        self.update_frames();
        true
    }

    // ==================== 关卡/游戏 ====================

    /// 开始新游戏（对应 C++ NewGame）
    pub fn new_game(&mut self) {
        self.m_first_time_game_selector = false;

        self.make_new_board();
        if let Some(board) = self.board {
            unsafe {
                (*board).init_level();
            }
        }
        self.board_result = BoardResult::None;
        self.game_scene = GameScenes::LevelIntro;

        self.show_seed_chooser_screen();
        if let Some(board) = self.board {
            unsafe {
                if let Some(cut_scene) = (*board).m_cut_scene {
                    (*cut_scene).start_level_intro();
                }
            }
        }
    }

    /// 预新建游戏（对应 C++ PreNewGame）
    pub fn pre_new_game(&mut self, mode: GameMode, look_for_saved: bool) {
        self.game_mode = mode;
        if look_for_saved && self.try_load_game() {
            return;
        }

        let profile_id = self.player_info.as_ref().map_or(0, |p| p.m_id) as i32;
        let file_name = crate::lawn::lawn_common::get_saved_game_name(mode, profile_id);
        let _ = std::fs::remove_file(&file_name);
        let legacy_file_name = crate::lawn::lawn_common::get_legacy_saved_game_name(mode, profile_id);
        let _ = std::fs::remove_file(&legacy_file_name);
        self.new_game();
    }

    /// 开始关卡
    pub fn start_level(&mut self, level: i32) {
        self.m_level = level;
        self.make_new_board();
        if let Some(board) = self.board {
            unsafe {
                (*board).init_level();
                (*board).m_board_result = BoardResult::None;
            }
        }
        self.game_scene = GameScenes::LevelIntro;
        // [TRANSLATION_NOTE]: ShowSeedChooserScreen + CutScene::StartLevelIntro 暂未实现
    }

    /// 创建新 Board（对应 C++ MakeNewBoard）
    pub fn make_new_board(&mut self) {
        self.kill_board();
        let mut board = Box::new(Board::new());
        board.level = self.m_level;
        board.board_init(self as *mut LawnApp);
        self.board = Some(Box::into_raw(board));
    }

    /// 销毁 Board（对应 C++ KillBoard）
    pub fn kill_board(&mut self) {
        if let Some(b) = self.board.take() {
            unsafe {
                (*b).dispose_board();
                let _ = Box::from_raw(b);
            }
        }
        // [TRANSLATION_NOTE]: 清理种子选择界面、删除存档文件暂未实现
        // 设置光标为指针
    }

    /// 开始游戏（对应 C++ StartPlaying）
    pub fn start_playing(&mut self) {
        self.game_scene = crate::lawn::lawn_app::GameScenes::Playing;
        if let Some(board) = self.board {
            unsafe { (*board).start_level(); }
        }
    }
    /// 结束关卡（对应 C++ EndLevel）
    pub fn end_level(&mut self) {
        self.kill_board();
        if self.is_adventure_mode() {
            self.new_game();
        }

        self.m_first_time_game_selector = true;

        self.make_new_board();
        if let Some(board) = self.board {
            unsafe {
                (*board).init_level();
            }
        }
        self.board_result = BoardResult::None;
        self.game_scene = GameScenes::LevelIntro;
        self.show_seed_chooser_screen();
        if let Some(board) = self.board {
            unsafe {
                if let Some(cut_scene) = (*board).m_cut_scene {
                    (*cut_scene).start_level_intro();
                }
            }
        }
    }

    // [TRANSLATION_NOTE]: 对应 C++ TryLoadGame 依赖的 DoContinueDialog。
    // ContinueDialog 尚未继承 Dialog/Widget 接口，暂以对象创建替代 widget 接入
    //（CenterDialog/AddDialog/SetFocus 待 widget 层翻译后接入）。
    pub fn do_continue_dialog(&mut self) {
        let _dialog = Box::new(
            crate::lawn::widget::continue_dialog::ContinueDialog::new(Some(self as *mut LawnApp)),
        );
    }

    /// 尝试加载游戏（对应 C++ TryLoadGame）
    pub fn try_load_game(&mut self) -> bool {
        let profile_id = self.player_info.as_ref().map_or(0, |p| p.m_id) as i32;
        let save_name = crate::lawn::lawn_common::get_saved_game_name(self.game_mode, profile_id);
        let legacy_save_name = crate::lawn::lawn_common::get_legacy_saved_game_name(self.game_mode, profile_id);
        if let Some(music) = &mut self.music {
            music.stop_all_music();
        }

        if file_exists(&save_name) {
            self.make_new_board();
            if let Some(board) = self.board {
                let ok = unsafe { (*board).load_game(&save_name) };
                if ok {
                    self.m_first_time_game_selector = false;
                    if unsafe { (*board).m_level_award_spawned } {
                        self.board_result = BoardResult::Won;
                    }
                    self.do_continue_dialog();
                    return true;
                }
            }
            self.kill_board();
        }

        if file_exists(&legacy_save_name) {
            self.make_new_board();
            if let Some(board) = self.board {
                let ok = unsafe { (*board).load_game(&legacy_save_name) };
                if ok {
                    if lawn_save_game(self.board, &save_name) {
                        let _ = std::fs::remove_file(&legacy_save_name);
                    }
                    self.m_first_time_game_selector = false;
                    if unsafe { (*board).m_level_award_spawned } {
                        self.board_result = BoardResult::Won;
                    }
                    self.do_continue_dialog();
                    return true;
                }
            }
            self.kill_board();
        }

        false
    }

    /// 是否存在未展示的成就（对应 C++ 静态函数 HasUnshownAchievements）
    fn has_unshown_achievements(&self) -> bool {
        let Some(player_info) = &self.player_info else {
            return false;
        };
        for i in 0..crate::lawn::widget::achievements_screen::MAX_ACHIEVEMENTS {
            if player_info.m_earned_achievements[i] && !player_info.m_shown_achievements[i] {
                return true;
            }
        }
        false
    }

    /// 检查游戏结束（对应 C++ CheckForGameEnd）
    pub fn check_for_game_end(&mut self) {
        if self.board.is_none() {
            return;
        }
        let board = unsafe { &*self.board.unwrap() };
        if !board.m_level_complete {
            return;
        }

        let unlocked_new_challenge = self.update_player_profile_for_finishing_level();
        let adventure_level = board.level;

        if self.is_adventure_mode() {
            self.kill_board();

            if self.is_first_time_adventure_mode() && adventure_level < 50 {
                self.show_award_screen(AwardType::ForLevel as i32, true);
            } else if adventure_level == FINAL_LEVEL {
                let finished = self.player_info.as_ref().map_or(0, |p| p.m_finished_adventure);
                if finished == 1 {
                    self.show_award_screen(AwardType::ForLevel as i32, true);
                } else {
                    self.show_award_screen(AwardType::CreditsZombieNote as i32, true);
                }
            } else if adventure_level == 9 || adventure_level == 19
                || adventure_level == 29 || adventure_level == 39 || adventure_level == 49
            {
                self.show_award_screen(AwardType::ForLevel as i32, true);
            } else if self.has_unshown_achievements() {
                self.show_award_screen(AwardType::AchievementOnly as i32, true);
            } else {
                self.pre_new_game(self.game_mode, false);
            }
        } else if self.is_survival_mode() {
            let mut show_challenge = false;
            {
                let board_ref = unsafe { &mut *self.board.unwrap() };
                if board_ref.is_final_survival_stage() {
                    show_challenge = true;
                } else {
                    if let Some(challenge) = board_ref.challenge.as_mut() {
                        challenge.survival_stage += 1;
                    }
                    self.kill_game_selector();
                    board_ref.init_survival_stage();
                }
            }
            if show_challenge {
                self.kill_board();
                if unlocked_new_challenge && self.has_finished_adventure() {
                    self.show_award_screen(AwardType::ForLevel as i32, true);
                } else if self.has_unshown_achievements() {
                    self.show_award_screen(AwardType::AchievementOnly as i32, true);
                } else {
                    self.show_challenge_screen(ChallengePage::Survival as i32);
                }
            }
        } else if self.is_puzzle_mode() {
            self.kill_board();

            if unlocked_new_challenge {
                self.show_award_screen(AwardType::ForLevel as i32, true);
            } else if self.has_unshown_achievements() {
                self.show_award_screen(AwardType::AchievementOnly as i32, true);
            } else {
                self.show_challenge_screen(ChallengePage::Puzzle as i32);
            }
        } else {
            self.kill_board();

            if unlocked_new_challenge && self.has_finished_adventure() {
                self.show_award_screen(AwardType::ForLevel as i32, true);
            } else if self.has_unshown_achievements() {
                self.show_award_screen(AwardType::AchievementOnly as i32, true);
            } else {
                self.show_challenge_screen(ChallengePage::Challenge as i32);
            }
        }
    }

    // ==================== 屏幕管理 ====================

    /// 显示游戏选择器（对应 C++ ShowGameSelector）
    pub fn show_game_selector(&mut self) {
        self.kill_board();

        // 清理旧的 game_selector
        if let Some(gs) = self.game_selector.take() {
            if let Some(wm) = self.base.widget_manager {
                unsafe {
                    (*wm).remove_widget(gs as *mut Widget);
                }
            }
        }

        self.game_scene = GameScenes::MainMenu;

        // 创建 GameSelector Widget
        let mut gs_widget = Box::new(Widget::new());
        gs_widget.impl_ = Some(Box::new(GameSelectorImpl::new(self as *mut LawnApp)));
        gs_widget.resize(0, 0, self.base.width, self.base.height);
        let gs_ptr = Box::into_raw(gs_widget);
        self.game_selector = Some(gs_ptr as *mut ());

        if let Some(wm) = self.base.widget_manager {
            unsafe {
                (*wm).add_widget(gs_ptr);
                (*wm).set_focus(Some(gs_ptr));
            }
        }
        eprintln!("[LawnApp] 已显示 GameSelector");
    }
    pub fn kill_game_selector(&mut self) {
        if let Some(gs) = self.game_selector.take() {
            if let Some(wm) = self.base.widget_manager {
                unsafe {
                    (*wm).remove_widget(gs as *mut Widget);
                }
            }
            unsafe {
                let gs_ptr = gs as *mut Widget;
                let _ = Box::from_raw(gs_ptr);
            }
        }
    }

    /// 显示奖励屏幕（对应 C++ ShowAwardScreen；含 AddWidget/BringToBack/SetFocus）
    pub fn show_award_screen(&mut self, award: i32, show_achievements: bool) {
        self.game_scene = GameScenes::Award;
        let mut screen = Box::new(crate::lawn::widget::award_screen::AwardScreen::new());
        screen.app = Some(self as *mut LawnApp);
        screen.award_type = unsafe { std::mem::transmute::<i32, AwardType>(award) };
        screen.showing_achievements = show_achievements;
        let screen_ptr = Box::into_raw(screen);
        // 对应 C++ AwardScreen : Widget + AddWidget + BringToBack + SetFocus
        let mut widget = Box::new(Widget::new());
        widget.impl_ = Some(Box::new(crate::lawn::widget::award_screen::AwardScreenImpl::new(screen_ptr)));
        widget.resize(0, 0, self.base.width, self.base.height);
        let widget_ptr = Box::into_raw(widget);
        if let Some(wm) = self.base.widget_manager {
            unsafe {
                (*wm).add_widget(widget_ptr);
                (*wm).set_focus(Some(widget_ptr));
            }
        }
        self.award_screen = Some(screen_ptr as *mut ());
        self.award_screen_widget = Some(widget_ptr);
    }

    /// 销毁奖励屏幕（对应 C++ KillAwardScreen）
    pub fn kill_award_screen(&mut self) {
        if let Some(widget_ptr) = self.award_screen_widget.take() {
            if let Some(wm) = self.base.widget_manager {
                unsafe { (*wm).remove_widget(widget_ptr); }
            }
            unsafe {
                let _ = Box::from_raw(widget_ptr);
            }
        }
        if let Some(screen) = self.award_screen.take() {
            unsafe {
                let _ = Box::from_raw(screen as *mut crate::lawn::widget::award_screen::AwardScreen);
            }
        }
    }

    /// 显示种子选择器（对应 C++ ShowSeedChooserScreen；含 AddWidget/SetFocus）
    pub fn show_seed_chooser_screen(&mut self) {
        let mut screen = Box::new(crate::lawn::widget::seed_chooser_screen::SeedChooserScreen::new());
        screen.app = Some(self as *mut LawnApp);
        let screen_ptr = Box::into_raw(screen);
        // 对应 C++ SeedChooserScreen : Widget + AddWidget + BringToBack + SetFocus
        let mut widget = Box::new(Widget::new());
        widget.impl_ = Some(Box::new(crate::lawn::widget::seed_chooser_screen::SeedChooserScreenImpl::new(screen_ptr)));
        widget.resize(0, 0, self.base.width, self.base.height);
        let widget_ptr = Box::into_raw(widget);
        if let Some(wm) = self.base.widget_manager {
            unsafe {
                (*wm).add_widget(widget_ptr);
                (*wm).set_focus(Some(widget_ptr));
            }
        }
        self.seed_chooser_screen = Some(screen_ptr as *mut ());
        self.seed_chooser_screen_widget = Some(widget_ptr);
    }

    pub fn kill_seed_chooser_screen(&mut self) {
        if let Some(widget_ptr) = self.seed_chooser_screen_widget.take() {
            if let Some(wm) = self.base.widget_manager {
                unsafe { (*wm).remove_widget(widget_ptr); }
            }
            unsafe {
                let _ = Box::from_raw(widget_ptr);
            }
        }
        if let Some(screen) = self.seed_chooser_screen.take() {
            unsafe {
                let _ = Box::from_raw(screen as *mut crate::lawn::widget::seed_chooser_screen::SeedChooserScreen);
            }
        }
    }

    /// 显示商店（对应 C++ ShowStoreScreen）
    pub fn show_store_screen(app: Option<*mut LawnApp>) -> Option<*mut ()> {
        let app_ref = unsafe { app?.as_mut()? };
        // 重复打开时先清理旧的商店（避免泄漏旧 Box / 重复注册 widget）
        if app_ref.store_screen.is_some() {
            app_ref.kill_store_screen();
        }

        // 创建 StoreScreen 本体（Box 拥有，存于 self.store_screen）
        let mut screen = Box::new(crate::lawn::widget::store_screen::StoreScreen::new(app));
        let screen_ptr = Box::into_raw(screen);

        // 创建 Widget 包装（impl_ 非拥有地引用 StoreScreen 本体）并注册进 WidgetManager
        // 对应 C++：AddDialog(DIALOG_STORE, aStoreScreen, true) + SetFocus
        let mut store_widget = Box::new(Widget::new());
        store_widget.impl_ = Some(Box::new(crate::lawn::widget::store_screen::StoreScreenImpl::new(screen_ptr)));
        store_widget.resize(0, 0, app_ref.base.width, app_ref.base.height);
        let store_widget_ptr = Box::into_raw(store_widget);
        if let Some(wm) = app_ref.base.widget_manager {
            unsafe {
                (*wm).add_widget(store_widget_ptr);
                (*wm).set_focus(Some(store_widget_ptr));
            }
        }
        app_ref.store_screen = Some(screen_ptr as *mut ());
        app_ref.store_screen_widget = Some(store_widget_ptr);
        eprintln!("[LawnApp] 已显示 StoreScreen");
        Some(screen_ptr as *mut ())
    }

    /// 销毁商店（对应 C++ KillStoreScreen；含 C++ RemovedFromManager 语义）
    pub fn kill_store_screen(&mut self) {
        // 1) 先移除并释放 Widget 包装（impl_ 非拥有，不触碰 StoreScreen 本体）
        if let Some(widget_ptr) = self.store_screen_widget.take() {
            if let Some(wm) = self.base.widget_manager {
                unsafe {
                    (*wm).remove_widget(widget_ptr);
                }
            }
            unsafe {
                let _ = Box::from_raw(widget_ptr);
            }
        }
        // 2) 再释放 StoreScreen 本体（先跑 removed_from_manager 的 CrazyDaveDie）
        if let Some(screen) = self.store_screen.take() {
            unsafe {
                let screen_ref = &mut *(screen as *mut crate::lawn::widget::store_screen::StoreScreen);
                let wm_ptr = self.base.widget_manager.map_or(std::ptr::null_mut(), |wm| wm);
                screen_ref.removed_from_manager(wm_ptr);
                let _ = Box::from_raw(screen as *mut crate::lawn::widget::store_screen::StoreScreen);
            }
        }
    }

    /// 显示挑战选择（对应 C++ ShowChallengeScreen；含 AddWidget/SetFocus）
    pub fn show_challenge_screen(&mut self, page: i32) {
        self.game_scene = GameScenes::Challenge;
        let mut screen = Box::new(crate::lawn::widget::challenge_screen::ChallengeScreen::new());
        screen.app = Some(self as *mut LawnApp);
        screen.page_index = unsafe { std::mem::transmute::<i32, ChallengePage>(page) };
        let screen_ptr = Box::into_raw(screen);
        // 对应 C++ ChallengeScreen : Widget + AddWidget + BringToBack + SetFocus
        let mut widget = Box::new(Widget::new());
        widget.impl_ = Some(Box::new(crate::lawn::widget::challenge_screen::ChallengeScreenImpl::new(screen_ptr)));
        widget.resize(0, 0, self.base.width, self.base.height);
        let widget_ptr = Box::into_raw(widget);
        if let Some(wm) = self.base.widget_manager {
            unsafe {
                (*wm).add_widget(widget_ptr);
                (*wm).set_focus(Some(widget_ptr));
            }
        }
        self.challenge_screen = Some(screen_ptr as *mut ());
        self.challenge_screen_widget = Some(widget_ptr);
    }

    /// 销毁挑战选择（对应 C++ KillChallengeScreen）
    pub fn kill_challenge_screen(&mut self) {
        if let Some(widget_ptr) = self.challenge_screen_widget.take() {
            if let Some(wm) = self.base.widget_manager {
                unsafe { (*wm).remove_widget(widget_ptr); }
            }
            unsafe {
                let _ = Box::from_raw(widget_ptr);
            }
        }
        if let Some(screen) = self.challenge_screen.take() {
            unsafe {
                let _ = Box::from_raw(screen as *mut crate::lawn::widget::challenge_screen::ChallengeScreen);
            }
        }
    }

    /// 显示制作人员（对应 C++ ShowCreditScreen；含 AddWidget/SetFocus）
    pub fn show_credit_screen(&mut self) {
        let mut screen = Box::new(crate::lawn::widget::credit_screen::CreditScreen::new());
        screen.app = Some(self as *mut LawnApp);
        let screen_ptr = Box::into_raw(screen);
        // 对应 C++ CreditScreen : Widget + AddWidget + BringToBack + SetFocus
        let mut widget = Box::new(Widget::new());
        widget.impl_ = Some(Box::new(crate::lawn::widget::credit_screen::CreditScreenImpl::new(screen_ptr)));
        widget.resize(0, 0, self.base.width, self.base.height);
        let widget_ptr = Box::into_raw(widget);
        if let Some(wm) = self.base.widget_manager {
            unsafe {
                (*wm).add_widget(widget_ptr);
                (*wm).set_focus(Some(widget_ptr));
            }
        }
        self.credit_screen = Some(screen_ptr as *mut ());
        self.credit_screen_widget = Some(widget_ptr);
    }

    /// 销毁制作人员（对应 C++ KillCreditScreen）
    pub fn kill_credit_screen(&mut self) {
        if let Some(widget_ptr) = self.credit_screen_widget.take() {
            if let Some(wm) = self.base.widget_manager {
                unsafe { (*wm).remove_widget(widget_ptr); }
            }
            unsafe {
                let _ = Box::from_raw(widget_ptr);
            }
        }
        if let Some(screen) = self.credit_screen.take() {
            unsafe {
                let _ = Box::from_raw(screen as *mut crate::lawn::widget::credit_screen::CreditScreen);
            }
        }
    }

    /// 显示图鉴（对应 C++ DoAlmanacDialog）
    pub fn do_almanac_dialog(&mut self, seed: SeedType, zombie: ZombieType) {
        let mut screen = Box::new(crate::lawn::widget::almanac_dialog::AlmanacDialog::new());
        screen.app = Some(self as *mut LawnApp);
        if seed != SeedType::None {
            screen.show_plant(seed);
        } else if zombie != ZombieType::Invalid {
            screen.show_zombie(zombie);
        }
        // [TRANSLATION_NOTE]: C++ 中随后 AddDialog(DIALOG_ALMANAC, aDialog) + SetFocus(aDialog)；
        // Rust 侧 AlmanacDialog 非 framework::Dialog 子类，暂以裸指针持有
        self.almanac_dialog = Some(Box::into_raw(screen) as *mut ());
    }
    pub fn kill_almanac_dialog(&mut self) -> bool {
        if let Some(screen) = self.almanac_dialog.take() {
            unsafe {
                let _ = Box::from_raw(screen as *mut crate::lawn::widget::almanac_dialog::AlmanacDialog);
            }
            return true;
        }
        // [TRANSLATION_NOTE]: 对应 C++ KillAlmanacDialog — GetDialog(DIALOG_ALMANAC) 非空则 KillDialog
        self.base.kill_dialog(Dialogs::Almanac as i32)
    }

    /// 返回主菜单（对应 C++ DoBackToMain）
    pub fn do_back_to_main(&mut self) {
        if let Some(music) = &mut self.music {
            music.stop_all_music();
        }
        if let Some(ss) = &self.sound_system {
            ss.cancel_paused_foley();
        }
        self.write_current_user_config();
        self.kill_new_options_dialog();
        self.kill_board();
        self.show_game_selector();
    }

    /// 关闭所有非模态对话框（对应 C++ FinishModelessDialogs）
    /// 关闭绑定到棋盘的对话框；被关闭的对话框按取消处理，删除延后
    pub fn finish_modeless_dialogs(&mut self) {
        self.kill_dialog(Dialogs::ConfirmRestart);
        self.kill_dialog(Dialogs::ConfirmBackToMain);
        self.kill_dialog(Dialogs::Paused);
        // 图鉴可能在 WaitForResult 中途，选项对话框挂在其下
        self.kill_dialog(Dialogs::Almanac);
        self.kill_new_options_dialog();
    }

    /// 移除新选项对话框（对应 C++ KillNewOptionsDialog）
    pub fn kill_new_options_dialog(&mut self) -> bool {
        if let Some(a_ptr) = self.new_options_dialog.take() {
            unsafe {
                let _ = Box::from_raw(a_ptr);
            }
            // [TRANSLATION_NOTE]: C++ 中由全屏/硬件加速复选框状态调用
            // SwitchScreenMode(wantWindowed, want3D, false)；Rust 侧 NewOptionsDialog
            // 尚无复选框字段，略过屏幕模式切换（switch_screen_mode 为空实现）。
            self.base.kill_dialog(Dialogs::NewOptions as i32);
            return true;
        }
        if !self.base.dialog_map.contains_key(&(Dialogs::NewOptions as i32)) {
            return false;
        }
        self.base.kill_dialog(Dialogs::NewOptions as i32);
        true
    }

    // ====================================================================
    // 对话框链（对应 C++ LawnApp.cpp 703-1096）
    // ====================================================================

    /// 新选项对话框（对应 C++ DoNewOptions）
    pub fn do_new_options(&mut self, the_from_game_selector: bool) {
        // [TRANSLATION_NOTE]: C++ 中 CenterDialog(aDialog, IMAGE_OPTIONS_MENUBACK 尺寸) +
        // AddDialog(DIALOG_NEWOPTIONS, aDialog) + SetFocus；Rust 侧 NewOptionsDialog 为
        // 独立类（非 framework::Dialog 子类），以字段持有，居中尺寸用兜底 400x340。
        let mut a_dialog = Box::new(crate::lawn::widget::new_options_dialog::NewOptionsDialog::new());
        a_dialog.app = Some(self as *mut LawnApp);
        a_dialog.from_game_selector = the_from_game_selector;
        a_dialog.x = (BOARD_WIDTH - 400) / 2;
        a_dialog.y = (BOARD_HEIGHT - 340) / 2;
        a_dialog.width = 400;
        a_dialog.height = 340;
        self.new_options_dialog = Some(Box::into_raw(a_dialog));
    }

    /// 用户对话框（对应 C++ DoUserDialog）
    pub fn do_user_dialog(&mut self) {
        self.kill_dialog(Dialogs::UserDialog);
        let mut a_dialog = Box::new(crate::lawn::widget::user_dialog::UserDialog::new(Some(self as *mut LawnApp)));
        // [TRANSLATION_NOTE]: C++ CenterDialog(aDialog, aDialog->mWidth, aDialog->mHeight) +
        // AddDialog(DIALOG_USERDIALOG) + SetFocus；Rust 侧 UserDialog 独立持有。
        a_dialog.x = (BOARD_WIDTH - a_dialog.width) / 2;
        a_dialog.y = (BOARD_HEIGHT - a_dialog.height) / 2;
        self.user_dialog = Some(Box::into_raw(a_dialog));
    }

    /// 完成用户对话框（对应 C++ FinishUserDialog）
    pub fn finish_user_dialog(&mut self, is_yes: bool) {
        if let Some(a_ptr) = self.user_dialog.take() {
            unsafe {
                let a_user_dialog = &mut *a_ptr;
                if is_yes {
                    let a_name = a_user_dialog.get_sel_name();
                    // C++: mProfileMgr->GetProfile(aName) → mPlayerInfo = aProfile
                    if let Some(a_profile) = self.profile_mgr.as_mut().and_then(|pm| pm.get_profile(&a_name)) {
                        self.player_info = Some(Box::new(a_profile.clone()));
                        // C++: mWidgetManager->MarkAllDirty()
                        if let Some(gs) = self.game_selector {
                            // C++: mGameSelector->SyncProfile(true)
                            (*(gs as *mut crate::lawn::widget::game_selector::GameSelectorImpl)).sync_profile(true);
                        }
                    }
                }
                let _ = Box::from_raw(a_ptr);
            }
        }
        self.kill_dialog(Dialogs::UserDialog);
    }

    /// 新建用户对话框（对应 C++ DoCreateUserDialog）
    pub fn do_create_user_dialog(&mut self) {
        self.kill_dialog(Dialogs::CreateUser);
        let mut a_dialog = Box::new(crate::lawn::widget::new_user_dialog::NewUserDialog::new(Some(self as *mut LawnApp)));
        // [TRANSLATION_NOTE]: C++ CenterDialog + AddDialog(DIALOG_CREATEUSER)
        a_dialog.x = (BOARD_WIDTH - a_dialog.width) / 2;
        a_dialog.y = (BOARD_HEIGHT - a_dialog.height) / 2;
        self.new_user_dialog = Some(Box::into_raw(a_dialog));
    }

    /// 完成新建用户对话框（对应 C++ FinishCreateUserDialog）
    pub fn finish_create_user_dialog(&mut self, is_yes: bool) {
        let a_name = if let Some(a_ptr) = self.new_user_dialog.as_ref() {
            unsafe { (*(*a_ptr)).get_name() }
        } else {
            return;
        };

        if is_yes && a_name.is_empty() {
            self.do_dialog(
                Dialogs::CreateUserError as i32,
                true,
                "Enter Your Name",
                "Please enter your name to create a new user profile for storing high score data and game progress.",
                "[DIALOG_BUTTON_OK]",
                BUTTONS_FOOTER,
            );
        } else if self.player_info.is_none() && (!is_yes || a_name.is_empty()) {
            self.do_dialog(
                Dialogs::CreateUserError as i32,
                true,
                "Enter Your Name",
                "Please enter your name to create a new user profile for storing high score data and game progress.",
                "[DIALOG_BUTTON_OK]",
                BUTTONS_FOOTER,
            );
        } else if !is_yes {
            self.kill_dialog(Dialogs::CreateUser);
        } else {
            // C++: mProfileMgr->AddProfile(aName)，冲突则错误框
            if self.profile_mgr.as_mut().and_then(|pm| pm.add_profile(&a_name)).is_none() {
                self.do_dialog(
                    Dialogs::CreateUserError as i32,
                    true,
                    "Name Conflict",
                    "The name you entered is already being used.  Please enter a unique player name.",
                    "[DIALOG_BUTTON_OK]",
                    BUTTONS_FOOTER,
                );
            } else {
                if let Some(pm) = self.profile_mgr.as_mut() {
                    pm.save();
                }
                // C++: mPlayerInfo = aProfile
                if let Some(a_profile) = self.profile_mgr.as_mut().and_then(|pm| pm.get_profile(&a_name)) {
                    self.player_info = Some(Box::new(a_profile.clone()));
                }
                self.kill_dialog(Dialogs::UserDialog);
                self.kill_dialog(Dialogs::CreateUser);
                // C++: mWidgetManager->MarkAllDirty() + mGameSelector->SyncProfile(true)
                if let Some(gs) = self.game_selector {
                    unsafe { (*(gs as *mut crate::lawn::widget::game_selector::GameSelectorImpl)).sync_profile(true); }
                }
            }
        }
    }

    /// 确认删除用户对话框（对应 C++ DoConfirmDeleteUserDialog）
    pub fn do_confirm_delete_user_dialog(&mut self, the_name: &str) {
        self.kill_dialog(Dialogs::ConfirmDeleteUser);
        let a_warning = format!("This will permanently remove '{}' from the player roster!", the_name);
        self.do_dialog(
            Dialogs::ConfirmDeleteUser as i32,
            true,
            "Are You Sure?",
            &a_warning,
            "",
            crate::framework::widget::dialog::BUTTONS_YES_NO,
        );
    }

    /// 完成确认删除用户（对应 C++ FinishConfirmDeleteUserDialog）
    pub fn finish_confirm_delete_user_dialog(&mut self, is_yes: bool) {
        self.kill_dialog(Dialogs::ConfirmDeleteUser);
        if !is_yes {
            return;
        }
        let a_cur_name = self.player_info.as_ref().map(|p| p.name.clone()).unwrap_or_default();
        let a_name = if let Some(a_ptr) = self.user_dialog.as_ref() {
            unsafe { (*(*a_ptr)).get_sel_name() }
        } else {
            return;
        };
        if a_name == a_cur_name {
            self.player_info = None;
        }
        // C++: mProfileMgr->DeleteProfile(aName)
        if let Some(pm) = self.profile_mgr.as_mut() {
            pm.delete_profile(&a_name);
        }
        if let Some(a_ptr) = self.user_dialog.as_ref() {
            unsafe { (*(*a_ptr)).finish_delete_user(); }
        }
        if self.player_info.is_none() {
            // C++: mPlayerInfo = GetProfile(GetSelName())；失败则 GetAnyProfile()
            if let Some(pm) = self.profile_mgr.as_mut() {
                if let Some(a_profile) = pm.get_profile(&a_name) {
                    self.player_info = Some(Box::new(a_profile.clone()));
                }
            }
            if self.player_info.is_none() {
                if let Some(pm) = self.profile_mgr.as_mut() {
                    if let Some(a_profile) = pm.get_any_profile() {
                        self.player_info = Some(Box::new(a_profile.clone()));
                    }
                }
            }
        }
        if let Some(pm) = self.profile_mgr.as_mut() {
            pm.save();
        }
        if self.player_info.is_none() {
            self.do_create_user_dialog();
        }
        // C++: mWidgetManager->MarkAllDirty() + mGameSelector->SyncProfile(true)
        if let Some(gs) = self.game_selector {
            unsafe { (*(gs as *mut crate::lawn::widget::game_selector::GameSelectorImpl)).sync_profile(true); }
        }
    }

    /// 重命名用户对话框（对应 C++ DoRenameUserDialog）
    pub fn do_rename_user_dialog(&mut self, the_name: &str) {
        self.kill_dialog(Dialogs::RenameUser);
        let mut a_dialog = Box::new(crate::lawn::widget::new_user_dialog::NewUserDialog::new(Some(self as *mut LawnApp)));
        // [TRANSLATION_NOTE]: C++ CenterDialog + AddDialog(DIALOG_RENAMEUSER) + SetName
        a_dialog.x = (BOARD_WIDTH - a_dialog.width) / 2;
        a_dialog.y = (BOARD_HEIGHT - a_dialog.height) / 2;
        a_dialog.set_name(the_name);
        self.rename_user_dialog = Some(Box::into_raw(a_dialog));
    }

    /// 完成重命名用户对话框（对应 C++ FinishRenameUserDialog）
    pub fn finish_rename_user_dialog(&mut self, is_yes: bool) {
        if !is_yes {
            self.kill_dialog(Dialogs::RenameUser);
            return;
        }
        let a_old_name = if let Some(a_ptr) = self.user_dialog.as_ref() {
            unsafe { (*(*a_ptr)).get_sel_name() }
        } else {
            return;
        };
        let a_new_name = if let Some(a_ptr) = self.rename_user_dialog.as_ref() {
            unsafe { (*(*a_ptr)).get_name() }
        } else {
            return;
        };
        if a_new_name.is_empty() {
            return;
        }
        // [TRANSLATION_NOTE]: C++ 以 mProfileMgr->GetProfile(anOldName) == mPlayerInfo
        // 指针比较判定"当前用户"；Rust 侧 profile_mgr 与 player_info 为不同对象，按名字等价判定。
        let is_current_user = self.player_info.as_ref().map_or(false, |pi| pi.name == a_old_name);
        if !self.profile_mgr.as_mut().map_or(false, |pm| pm.rename_profile(&a_old_name, &a_new_name)) {
            self.do_dialog(
                Dialogs::RenameUserError as i32,
                true,
                "Name Conflict",
                "The name you entered is already being used.  Please enter a unique player name.",
                "[DIALOG_BUTTON_OK]",
                BUTTONS_FOOTER,
            );
            return;
        }
        if let Some(pm) = self.profile_mgr.as_mut() {
            pm.save();
        }
        if is_current_user {
            // C++: mPlayerInfo = mProfileMgr->GetProfile(aNewName)
            if let Some(pm) = self.profile_mgr.as_mut() {
                if let Some(a_profile) = pm.get_profile(&a_new_name) {
                    self.player_info = Some(Box::new(a_profile.clone()));
                }
            }
        }
        if let Some(a_ptr) = self.user_dialog.as_ref() {
            unsafe { (*(*a_ptr)).finish_rename_user(&a_new_name); }
        }
        // C++: mWidgetManager->MarkAllDirty() + KillDialog(DIALOG_RENAMEUSER) + SetFocus
        self.kill_dialog(Dialogs::RenameUser);
    }

    /// 名称错误对话框关闭（对应 C++ FinishNameError）
    pub fn finish_name_error(&mut self, the_id: i32) {
        // C++: KillDialog(theId)
        self.kill_dialog(if the_id == Dialogs::CreateUserError as i32 {
            Dialogs::CreateUserError
        } else {
            Dialogs::RenameUserError
        });
        // [TRANSLATION_NOTE]: C++ 中随后恢复焦点到 NewUserDialog 的名称编辑框
        //（mNameEditWidget）；Rust 侧编辑框未接入，略。
    }

    /// 作弊对话框（对应 C++ DoCheatDialog）
    pub fn do_cheat_dialog(&mut self) {
        self.kill_dialog(Dialogs::Cheat);
        let mut a_dialog = Box::new(crate::lawn::widget::cheat_dialog::CheatDialog::new(Some(self as *mut LawnApp)));
        // [TRANSLATION_NOTE]: C++ CenterDialog + AddDialog(DIALOG_CHEAT)
        a_dialog.x = (BOARD_WIDTH - a_dialog.width) / 2;
        a_dialog.y = (BOARD_HEIGHT - a_dialog.height) / 2;
        self.cheat_dialog = Some(Box::into_raw(a_dialog));
    }

    /// 完成作弊对话框（对应 C++ FinishCheatDialog）
    pub fn finish_cheat_dialog(&mut self, is_yes: bool) {
        if let Some(a_ptr) = self.cheat_dialog.take() {
            unsafe {
                // C++: isYes && !ApplyCheat() 时 return（对话框保留）
                if is_yes && !(*a_ptr).apply_cheat() {
                    self.cheat_dialog = Some(a_ptr);
                    return;
                }
                let _ = Box::from_raw(a_ptr);
            }
        }
        self.kill_dialog(Dialogs::Cheat);
        if is_yes {
            if let Some(music) = &mut self.music {
                music.stop_all_music();
            }
            self.board_result = BoardResult::Cheat;
            self.pre_new_game(self.game_mode, false);
        }
    }

    /// 关闭时间到对话框（对应 C++ FinishTimesUpDialog）
    pub fn finish_times_up_dialog(&mut self) {
        self.kill_dialog(Dialogs::TimesUp);
    }

    /// 暂停（对应 C++ DoPauseDialog）
    pub fn do_pause_dialog(&mut self) {
        if let Some(board) = self.board {
            unsafe { (*board).pause(true); }
        }

        let the_dialog = self.do_dialog(
            Dialogs::Paused as i32,
            true,
            "GAME PAUSED",
            "Click to resume game",
            "Resume Game",
            BUTTONS_FOOTER,
        );
        if let Some(dialog) = the_dialog {
            unsafe {
                (*dialog).space_after_header = 155;
            }
            // [TRANSLATION_NOTE]: C++ 中随后执行 LawnDialog 的
            // mReanimation->AddReanimation(72,42,REANIM_ZOMBIE_NEWSPAPER)、
            // CalcSize(0,10)、CenterDialog(aDialog, w, h)；前两者依赖
            // LawnDialog 专有成员（framework Dialog 无），CenterDialog 待
            // widget 层接入后补齐。
        }
    }

    /// 对话框（对应 C++ DoDialog — NewDialog + AddDialog）
    pub fn do_dialog(&mut self, the_dialog_id: i32, is_modal: bool, the_dialog_header: &str, the_dialog_lines: &str, the_dialog_footer: &str, the_button_mode: i32) -> Option<*mut Dialog> {
        // C++ 中若当前已有模态对话框（GetModalDialog() != nullptr）则不 AddDialog
        let is_in_modal = self.base.dialog_map.values().any(|d| unsafe { (**d).is_modal });
        let mut a_dialog = Box::new(Dialog::new(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            the_dialog_id,
            is_modal,
            the_dialog_header,
            the_dialog_lines,
            the_dialog_footer,
            the_button_mode,
        ));
        let a_dialog_ptr = Box::into_raw(a_dialog);
        if !is_in_modal {
            self.base.add_dialog(the_dialog_id, a_dialog_ptr);
        }
        Some(a_dialog_ptr)
    }

    /// 延迟创建对话框（对应 C++ DoDialogDelay）
    /// 创建对话框并设置按钮延迟
    pub fn do_dialog_delay(&mut self, id: i32, modal: bool, header: &str, lines: &str, footer: &str, btn_mode: i32) -> Option<*mut Dialog> {
        let dialog = self.do_dialog(id, modal, header, lines, footer, btn_mode);
        if let Some(d) = dialog {
            unsafe {
                // C++ 中 aDialog->WaitForResult(false); aDialog->SetButtonDelay(30);
                (*d).wait_for_result(false);
            }
        }
        dialog
    }

    /// 居中对话框（对应 C++ CenterDialog）
    /// 将对话框置于屏幕中央
    pub fn center_dialog(dialog: *mut Dialog, width: i32, height: i32) {
        unsafe {
            let d = &mut *dialog;
            d.x = (BOARD_WIDTH - width) / 2;
            d.y = (BOARD_HEIGHT - height) / 2;
            d.width = width;
            d.height = height;
        }
    }

    // ==================== 游戏对象管理 ====================

    /// 添加动画（对应 C++ AddReanimation）
    pub fn add_reanimation(&mut self, x: f32, y: f32, render_order: i32, reanim_type: i32) -> Option<*mut Reanimation> {
        let reanim_type = unsafe { std::mem::transmute::<i32, ReanimationType>(reanim_type) };
        if let Some(es) = self.effect_system.as_mut() {
            let mut reanim = Reanimation::new();
            reanim.reanimation_initialize_type(x, y, reanim_type);
            reanim.m_render_order = render_order;
            let id = es.add_reanimation(reanim);
            let idx = id as usize;
            if idx < es.reanimations.len() {
                return Some(&mut es.reanimations[idx] as *mut Reanimation);
            }
        }
        None
    }

    /// 获取动画（对应 C++ ReanimationGet）
    pub fn reanimation_get(&self, id: ReanimationID) -> Option<&Reanimation> {
        self.effect_system.as_ref().and_then(|es| es.reanimations.get(id as usize))
    }

    /// 获取动画（可变版本）
    pub fn reanimation_get_mut(&mut self, id: ReanimationID) -> Option<&mut Reanimation> {
        self.effect_system.as_mut().and_then(|es| es.reanimations.get_mut(id as usize))
    }

    /// 获取动画 ID（对应 C++ ReanimationGetID）
    pub fn reanimation_get_id(&self, reanim: *mut Reanimation) -> ReanimationID {
        if reanim.is_null() {
            return REANIMATIONID_NULL;
        }
        if let Some(es) = self.effect_system.as_ref() {
            for (i, r) in es.reanimations.iter().enumerate() {
                if std::ptr::eq(r, reanim) {
                    return i as ReanimationID;
                }
            }
        }
        REANIMATIONID_NULL
    }

    /// 添加粒子（对应 C++ AddTodParticle）
    pub fn add_tod_particle(&mut self, x: f32, y: f32, render_order: i32, effect: i32) -> Option<*mut TodParticleSystem> {
        let effect = unsafe { std::mem::transmute::<i32, ParticleEffect>(effect) };
        if let Some(es) = self.effect_system.as_mut() {
            let mut ps = TodParticleSystem::new();
            ps.effect_type = effect;
            ps.render_order = render_order;            // [TRANSLATION_NOTE]: 粒子系统位置以 render_order 近似存储（完整版使用 emitter 偏移）
            let _ = (x, y);
            let id = es.add_particle_system(ps);
            // id 为 1-based（0 保留给 PARTICLESYSTEMID_NULL）
            if id != 0 {
                let idx = (id - 1) as usize;
                if idx < es.particle_systems.len() {
                    return Some(&mut es.particle_systems[idx] as *mut TodParticleSystem);
                }
            }
        }
        None
    }

    /// 按 ID 获取粒子系统（对应 C++ ParticleTryToGet）
    pub fn particle_try_to_get(&mut self, id: crate::lawn::game_enums::ParticleSystemID) -> Option<&mut TodParticleSystem> {
        self.effect_system.as_mut().and_then(|es| es.particle_try_to_get(id))
    }

    /// 获取粒子系统 ID（对应 C++ ParticleGetID；找不到或空指针返回 PARTICLESYSTEMID_NULL）
    pub fn particle_get_id(&self, ptr: *mut TodParticleSystem) -> crate::lawn::game_enums::ParticleSystemID {
        self.effect_system.as_ref().map_or(0, |es| es.particle_get_id(ptr))
    }

    /// 移除动画（对应 C++ RemoveReanimation）
    pub fn remove_reanimation(&mut self, id: ReanimationID) {
        if let Some(es) = self.effect_system.as_mut() {
            es.remove_reanimation(id);
        }
    }

    /// 移除粒子（对应 C++ RemoveParticle）
    pub fn remove_particle(&mut self, id: ParticleSystemID) {
        if let Some(es) = self.effect_system.as_mut() {
            es.remove_particle_system(id);
        }
    }

    /// 播放音效（对应 C++ PlayFoley）
    pub fn play_foley(&self, foley_type: i32) {
        if !self.m_mute_sounds_for_cutscene {
            if let Some(ss) = &self.sound_system {
                let ft = unsafe { std::mem::transmute::<i32, FoleyType>(foley_type) };
                ss.play_foley(ft);
            }
        }
    }

    /// 播放指定音调的音效（对应 C++ PlayFoleyPitch）
    pub fn play_foley_pitch(&self, foley_type: i32, pitch: f32) {
        if !self.m_mute_sounds_for_cutscene {
            if let Some(ss) = &self.sound_system {
                let ft = unsafe { std::mem::transmute::<i32, FoleyType>(foley_type) };
                ss.play_foley_pitch(ft, pitch);
            }
        }
    }

    /// 播放采样音效（对应 C++ PlaySample，经 SexyAppBase::PlaySample 播放）
    pub fn play_sample(&self, sound_num: i32) {
        if !self.m_mute_sounds_for_cutscene {
            unsafe {
                if let Some(sm) = self.base.sound_manager {
                    (*sm).play_sound(sound_num);
                }
            }
        }
    }

    /// 打开的对话框数量（对应 C++ WidgetManager::GetDialogCount）
    /// Rust 以已知对话框字段统计（user/new_user/rename/cheat/new_options + store/almanac 对话框）
    pub fn get_dialog_count(&self) -> i32 {
        let mut a_count = 0;
        if self.user_dialog.is_some() { a_count += 1; }
        if self.new_user_dialog.is_some() { a_count += 1; }
        if self.rename_user_dialog.is_some() { a_count += 1; }
        if self.cheat_dialog.is_some() { a_count += 1; }
        if self.new_options_dialog.is_some() { a_count += 1; }
        if self.store_screen.is_some() { a_count += 1; }
        if self.almanac_dialog.is_some() { a_count += 1; }
        a_count
    }

    // ==================== 状态查询 ====================

    pub fn is_adventure_mode(&self) -> bool { self.game_mode == GameMode::Adventure }
    pub fn is_survival_mode(&self) -> bool {
        matches!(self.game_mode,
            GameMode::SurvivalNormalStage1 | GameMode::SurvivalNormalStage2 |
            GameMode::SurvivalNormalStage3 | GameMode::SurvivalNormalStage4 |
            GameMode::SurvivalNormalStage5 | GameMode::SurvivalHardStage1 |
            GameMode::SurvivalHardStage2 | GameMode::SurvivalHardStage3 |
            GameMode::SurvivalHardStage4 | GameMode::SurvivalHardStage5 |
            GameMode::SurvivalEndlessStage1 | GameMode::SurvivalEndlessStage2 |
            GameMode::SurvivalEndlessStage3 | GameMode::SurvivalEndlessStage4 |
            GameMode::SurvivalEndlessStage5
        )
    }
    pub fn is_survival_normal(&self, mode: GameMode) -> bool {
        matches!(mode,
            GameMode::SurvivalNormalStage1 | GameMode::SurvivalNormalStage2 |
            GameMode::SurvivalNormalStage3 | GameMode::SurvivalNormalStage4 |
            GameMode::SurvivalNormalStage5
        )
    }
    pub fn is_survival_hard(&self, mode: GameMode) -> bool {
        matches!(mode,
            GameMode::SurvivalHardStage1 | GameMode::SurvivalHardStage2 |
            GameMode::SurvivalHardStage3 | GameMode::SurvivalHardStage4 |
            GameMode::SurvivalHardStage5
        )
    }
    pub fn is_survival_endless(&self, mode: GameMode) -> bool {
        matches!(mode,
            GameMode::SurvivalEndlessStage1 | GameMode::SurvivalEndlessStage2 |
            GameMode::SurvivalEndlessStage3 | GameMode::SurvivalEndlessStage4 |
            GameMode::SurvivalEndlessStage5
        )
    }
    pub fn is_continuous_challenge(&self) -> bool {
        self.is_art_challenge()
            || self.is_slot_machine_level()
            || self.is_final_boss_level()
            || self.game_mode == GameMode::ChallengeBeghouled
            || self.game_mode == GameMode::ChallengeBeghouledTwist
            || self.game_mode == GameMode::Upsell
            || self.game_mode == GameMode::Intro
    }
    pub fn is_puzzle_mode(&self) -> bool {
        let a_mode = self.game_mode as i32;
        (a_mode >= GameMode::ScaryPotter1 as i32 && a_mode <= GameMode::ScaryPotterEndless as i32)
            || (a_mode >= GameMode::PuzzleIZombie1 as i32 && a_mode <= GameMode::PuzzleIZombieEndless as i32)
    }
    pub fn is_endless_scary_potter(&self, mode: GameMode) -> bool {
        mode == GameMode::ScaryPotterEndless
    }
    pub fn is_endless_izombie(&self, mode: GameMode) -> bool {
        mode == GameMode::PuzzleIZombieEndless
    }
    pub fn is_challenge_mode(&self) -> bool {
        !self.is_adventure_mode() && !self.is_puzzle_mode() && !self.is_survival_mode()
    }
    pub fn is_art_challenge(&self) -> bool {
        if self.board.is_none() {
            return false;
        }
        matches!(self.game_mode,
            GameMode::ChallengeArtChallengeWallnut | GameMode::ChallengeArtChallengeSunflower
            | GameMode::ChallengeSeeingStars
        )
    }
    pub fn is_izombie_level(&self) -> bool {
        if self.board.is_none() {
            return false;
        }
        let a_mode = self.game_mode as i32;
        a_mode >= GameMode::PuzzleIZombie1 as i32 && a_mode <= GameMode::PuzzleIZombieEndless as i32
    }
    pub fn is_scary_potter_level(&self) -> bool {
        let a_mode = self.game_mode as i32;
        if a_mode >= GameMode::ScaryPotter1 as i32 && a_mode <= GameMode::ScaryPotterEndless as i32 {
            return true;
        }
        self.is_adventure_mode() && self.board.map_or(false, |b| unsafe { (*b).level == 35 })
    }
    pub fn is_whack_a_zombie_level(&self) -> bool {
        if self.board.is_none() {
            return false;
        }
        if self.game_mode == GameMode::ChallengeWhackAZombie {
            return true;
        }
        self.is_adventure_mode() && self.board.map_or(false, |b| unsafe { (*b).level == 15 })
    }
    pub fn is_squirrel_level(&self) -> bool {
        self.board.is_some() && self.game_mode == GameMode::ChallengeSquirrel
    }
    pub fn is_shovel_level(&self) -> bool {
        self.board.is_some() && self.game_mode == GameMode::ChallengeShovel
    }
    pub fn is_little_trouble_level(&self) -> bool {
        self.board.map_or(false, |b| unsafe {
            (*b).app.map_or(false, |app| {
                unsafe { (*app).game_mode == GameMode::ChallengeLittleTrouble
                    || ((*app).game_mode == GameMode::Adventure && (*b).level == 25) }
            })
        })
    }
    pub fn is_wallnut_bowling_level(&self) -> bool {
        if self.board.is_none() { return false; }
        if matches!(self.game_mode, GameMode::ChallengeWallnutBowling | GameMode::ChallengeWallnutBowling2) { return true; }
        self.is_adventure_mode() && self.board.map_or(false, |b| unsafe { (*b).level == 5 })
    }
    pub fn is_mini_boss_level(&self) -> bool {
        if self.board.is_none() { return false; }
        self.is_adventure_mode() && self.board.map_or(false, |b| unsafe {
            (*b).level == 10 || (*b).level == 20 || (*b).level == 30
        })
    }
    pub fn is_slot_machine_level(&self) -> bool {
        self.board.is_some() && self.game_mode == GameMode::ChallengeSlotMachine
    }
    pub fn is_stormy_night_level(&self) -> bool {
        if self.board.is_none() {
            return false;
        }
        if self.game_mode == GameMode::ChallengeStormyNight {
            return true;
        }
        self.is_adventure_mode() && self.board.map_or(false, |b| unsafe { (*b).level == 40 })
    }
    pub fn is_final_boss_level(&self) -> bool {
        if self.board.is_none() { return false; }
        if self.game_mode == GameMode::ChallengeFinalBoss { return true; }
        self.is_adventure_mode() && self.board.map_or(false, |b| unsafe { (*b).level == 50 })
    }
    pub fn is_bungee_blitz_level(&self) -> bool {
        if self.board.is_none() {
            return false;
        }
        if self.game_mode == GameMode::ChallengeBungeeBlitz {
            return true;
        }
        self.is_adventure_mode() && self.board.map_or(false, |b| unsafe { (*b).level == 45 })
    }
    pub fn is_night(&self) -> bool {
        self.board.map_or(false, |b| unsafe { (*b).stage_is_night() })
    }
    pub fn is_challenge_without_seed_bank(&self) -> bool {
        self.board.is_some() && !self.board.map_or(false, |b| unsafe { (*b).choose_seeds_on_current_level() })
    }
    pub fn can_show_almanac(&self) -> bool {
        if self.is_ice_demo() {
            return false;
        }
        if self.player_info.is_none() {
            return false;
        }
        self.has_finished_adventure() || self.player_info.as_ref().unwrap().m_level >= 15
    }
    pub fn can_show_store(&self) -> bool {
        if self.is_ice_demo() {
            return false;
        }
        if self.player_info.is_none() {
            return false;
        }
        self.has_finished_adventure()
            || self.player_info.as_ref().unwrap().m_has_seen_upsell != 0
            || self.player_info.as_ref().unwrap().m_level >= 25
    }
    pub fn can_show_zen_garden(&self) -> bool {
        if self.player_info.is_none() {
            return false;
        }
        if self.is_trial_stage_locked() {
            return false;
        }
        self.has_finished_adventure() || self.player_info.as_ref().unwrap().m_level >= 45
    }
    /// 能否使用彩带模式（对应 C++ CanDoPinataMode：智慧树记录 >= 1000）
    pub fn can_do_pinata_mode(&self) -> bool {
        if self.player_info.is_none() {
            return false;
        }
        let index = (GameMode::ChallengeTreeOfWisdom as i32 - GameMode::SurvivalNormalStage1 as i32) as usize;
        self.player_info.as_ref().map_or(0, |info| {
            info.m_challenge_records.get(index).copied().unwrap_or(0)
        }) >= 1000
    }
    /// 能否使用舞蹈模式（对应 C++ CanDoDanceMode：智慧树记录 >= 500）
    pub fn can_do_dance_mode(&self) -> bool {
        if self.player_info.is_none() {
            return false;
        }
        let index = (GameMode::ChallengeTreeOfWisdom as i32 - GameMode::SurvivalNormalStage1 as i32) as usize;
        self.player_info.as_ref().map_or(0, |info| {
            info.m_challenge_records.get(index).copied().unwrap_or(0)
        }) >= 500
    }
    /// 能否使用雏菊模式（对应 C++ CanDoDaisyMode：智慧树记录 >= 100）
    pub fn can_do_daisy_mode(&self) -> bool {
        if self.player_info.is_none() {
            return false;
        }
        let index = (GameMode::ChallengeTreeOfWisdom as i32 - GameMode::SurvivalNormalStage1 as i32) as usize;
        self.player_info.as_ref().map_or(0, |info| {
            info.m_challenge_records.get(index).copied().unwrap_or(0)
        }) >= 100
    }
    pub fn can_pause_now(&self) -> bool { true }
    pub fn can_spawn_yetis(&self) -> bool {
        // [TRANSLATION_NOTE]: 对应 C++ CanSpawnYetis，需要 get_zombie_definition 的 mStartingLevel
        if self.player_info.is_none() {
            return false;
        }
        let zombie_def = crate::lawn::zombie::get_zombie_definition(ZombieType::Yeti);
        self.has_finished_adventure()
            && (self.player_info.as_ref().unwrap().m_finished_adventure >= 2
                || self.player_info.as_ref().unwrap().m_level >= zombie_def.starting_level)
    }
    pub fn has_finished_adventure(&self) -> bool {
        self.player_info.as_ref().map_or(false, |p| p.m_finished_adventure != 0)
    }
    pub fn has_beaten_challenge(&self, mode: GameMode) -> bool {
        if self.player_info.is_none() {
            return false;
        }
        let a_challenge_index = mode as i32 - GameMode::SurvivalNormalStage1 as i32;
        if self.is_survival_normal(mode) {
            return self.player_info.as_ref().unwrap().m_challenge_records[a_challenge_index as usize] >= SURVIVAL_NORMAL_FLAGS;
        }
        if self.is_survival_hard(mode) {
            return self.player_info.as_ref().unwrap().m_challenge_records[a_challenge_index as usize] >= SURVIVAL_HARD_FLAGS;
        }
        // 对应 C++ IsSurvivalEndless || IsEndlessScaryPotter || IsEndlessIZombie
        if self.is_survival_endless(mode)
            || mode == GameMode::ScaryPotterEndless
            || mode == GameMode::PuzzleIZombieEndless
        {
            return false;
        }
        self.player_info.as_ref().unwrap().m_challenge_records[a_challenge_index as usize] > 0
    }
    pub fn has_seed_type(&self, seed: SeedType) -> bool {
        if self.is_trial_stage_locked() && seed as i32 >= SeedType::Jalapeno as i32 {
            return false;
        }
        // [TRANSLATION_NOTE]: C++ 未检查 mPlayerInfo 为 null；Rust 侧为规避读档未加载时的崩溃，None 时返回 false
        let player_info = match self.player_info.as_ref() {
            Some(p) => p,
            None => return false,
        };
        match seed {
            SeedType::Gatlingpea => player_info.m_purchases[StoreItem::PlantGatlingpea as usize] > 0,
            SeedType::Twinsunflower => player_info.m_purchases[StoreItem::PlantTwinsunflower as usize] > 0,
            SeedType::Gloomshroom => player_info.m_purchases[StoreItem::PlantGloomshroom as usize] > 0,
            SeedType::Cattail => player_info.m_purchases[StoreItem::PlantCattail as usize] > 0,
            SeedType::Wintermelon => player_info.m_purchases[StoreItem::PlantWintermelon as usize] > 0,
            SeedType::GoldMagnet => player_info.m_purchases[StoreItem::PlantGoldMagnet as usize] > 0,
            SeedType::Spikerock => player_info.m_purchases[StoreItem::PlantSpikerock as usize] > 0,
            SeedType::Cobcannon => player_info.m_purchases[StoreItem::PlantCobcannon as usize] > 0,
            SeedType::Imitater => player_info.m_purchases[StoreItem::PlantImitater as usize] > 0,
            _ => (seed as i32) < self.get_seeds_available(),
        }
    }

    pub fn get_seeds_available(&self) -> i32 {
        if self.player_info.is_none() {
            return 0;
        }
        let a_level = self.player_info.as_ref().unwrap().m_level;
        if self.has_finished_adventure() || a_level > 50 {
            return 49;
        }
        let a_seed_type_max = Self::get_award_seed_for_level(a_level) as i32;
        std::cmp::min(NUM_SEEDS_IN_CHOOSER, a_seed_type_max)
    }
    pub fn get_award_seed_for_level(level: i32) -> SeedType {
        let a_area = (level - 1) / LEVELS_PER_AREA + 1;
        let a_sub = (level - 1) % LEVELS_PER_AREA + 1;
        let mut a_seeds_has_got = (a_area - 1) * 8 + a_sub;  // in general, each area awards 8 plants and each level awards 1
        if a_sub >= 10 {
            a_seeds_has_got -= 2;  // 2 levels in this area don't award a new plant
        } else if a_sub >= 5 {
            a_seeds_has_got -= 1;  // 1 level in this area doesn't award a new plant
        }
        if a_seeds_has_got > 40 {
            a_seeds_has_got = 40;
        }
        // [TRANSLATION_NOTE]: C++ 直接 (SeedType)aSeedsHasGot 强转；Rust 枚举无法直接转换，用 unsafe transmute 保持等价
        unsafe { std::mem::transmute::<i32, SeedType>(a_seeds_has_got) }
    }
    pub fn get_current_challenge_def() -> u32 { 0 }
    pub fn get_current_challenge_index(&self) -> i32 { 0 }
    pub fn get_current_level_name(&self) -> String { format!("Level {}", self.m_level) }
    pub fn get_stage_string(level: i32) -> String { format!("Stage {}", level) }
    pub fn get_num_trophies(_page: i32) -> i32 { 0 }
    /// 距离金色向日葵奖杯还差多少奖杯（对应 C++ TrophiesNeedForGoldSunflower）
    /// 注意：get_num_trophies 目前为 stub（返回 0），此值为 48 直到奖杯系统接入。
    pub fn trophies_need_for_gold_sunflower(&self) -> i32 {
        48 - Self::get_num_trophies(ChallengePage::Survival as i32)
            - Self::get_num_trophies(ChallengePage::Challenge as i32)
            - Self::get_num_trophies(ChallengePage::Puzzle as i32)
    }
    /// 获取当前时间戳（秒，对应 C++ GetNowTime）
    pub fn get_now_time(&self) -> i64 {
        crate::framework::common::now_time()
    }
    /// 获取本地时间结构（对应 C++ GetLocalTime）
    /// 返回 (年, 月, 日, 时, 分, 秒, 周几, 一年中第几天, 夏令时)
    pub fn get_local_time(&self, _time: i64) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32) {
        crate::framework::common::local_time()
    }
    pub fn get_money_string(amount: i32) -> String {
        // 对应 C++ LawnApp::GetMoneyString（金额以"分"为单位显示，×10）
        let a_value = amount * 10;
        if a_value > 999999 {
            format!(
                "${},{:03},{:03}",
                a_value / 1000000,
                (a_value - a_value / 1000000 * 1000000) / 1000,
                a_value - a_value / 1000 * 1000
            )
        } else if a_value > 9999 {
            format!("${},{:03}", a_value / 1000, a_value - a_value / 1000 * 1000)
        } else {
            format!("${}", a_value)
        }
    }
    pub fn get_close_request(&self) -> bool { self.m_close_request }
    pub fn has_used_cheat_keys(&self) -> bool { self.m_cheat_keys_used }

    // ==================== 疯狂戴夫 ====================

    /// 疯狂戴夫入场（对应 C++ CrazyDaveEnter）
    pub fn crazy_dave_enter(&mut self) {
        // C++ 中 PVZP_ASSERT(mCrazyDaveState == CRAZY_DAVE_OFF) 且当前无戴夫动画
        let reanim_ptr = self.add_reanimation(0.0, 0.0, 0, ReanimationType::CrazyDave as i32);
        if let Some(rp) = reanim_ptr {
            unsafe {
                (*rp).m_is_attachment = true;
                // [TRANSLATION_NOTE]: C++ 中 SetBasePoseFromAnim("anim_idle_handing")；Rust 侧无对应
                (*rp).play_reanim("anim_enter", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 0, 24.0);
            }
            self.m_crazy_dave_reanim_id = self.reanimation_get_id(rp);
        }

        self.m_crazy_dave_state = CrazyDaveState::Entering;
        self.m_crazy_dave_message_index = -1;
        self.m_crazy_dave_message_text.clear();
        self.m_crazy_dave_blink_counter = crate::todlib::tod_common::rand_range_int(400, 800);

        if self.game_scene == GameScenes::LevelIntro && self.is_stormy_night_level() {
            if let Some(rp) = self.reanimation_get_mut(self.m_crazy_dave_reanim_id) {
                rp.m_color_override = crate::framework::color::Color::from_rgb(64, 64, 64);
            }
        }
    }

    /// 疯狂戴夫死亡（对应 C++ CrazyDaveDie）
    pub fn crazy_dave_die(&mut self) {
        if let Some(r) = self.reanimation_get_mut(self.m_crazy_dave_reanim_id) {
            r.reanimation_die();
        }
        self.m_crazy_dave_state = CrazyDaveState::Off;
        self.m_crazy_dave_reanim_id = REANIMATIONID_NULL;
        self.m_crazy_dave_blink_reanim_id = REANIMATIONID_NULL;
        self.m_crazy_dave_message_index = -1;
        self.m_crazy_dave_message_text.clear();
        self.crazy_dave_stop_sound();
    }

    /// 停止戴夫音效（对应 C++ CrazyDaveStopSound）
    fn crazy_dave_stop_sound(&mut self) {
        if let Some(ss) = &self.sound_system {
            ss.stop_foley(FoleyType::CrazyDaveShort);
            ss.stop_foley(FoleyType::CrazyDaveLong);
            ss.stop_foley(FoleyType::CrazyDaveExtraLong);
            ss.stop_foley(FoleyType::CrazyDaveCrazy);
        }
    }

    /// 结束"递物"动作（对应 C++ CrazyDaveDoneHanding）
    fn crazy_dave_done_handing(&mut self) {
        // [TRANSLATION_NOTE]: C++ 中 ReanimationGet(...)->GetTrackInstanceByName("Dave_handinghand")
        // ->mAttachmentID 后 AttachmentDie(...)；Rust 侧 ReanimatorTrackInstance 无
        // m_attachment_id 且 attachment 系统为骨架（attach_reanim 返回 None），暂不处理。
        let _ = self.reanimation_get_mut(self.m_crazy_dave_reanim_id);
    }

    /// 疯狂戴夫离开（对应 C++ CrazyDaveLeave）
    pub fn crazy_dave_leave(&mut self) {
        if self.m_crazy_dave_state == CrazyDaveState::HandingTalking || self.m_crazy_dave_state == CrazyDaveState::HandingIdling {
            self.crazy_dave_done_handing();
        }

        let reanim_ptr: *mut Reanimation = {
            let r = self.reanimation_get_mut(self.m_crazy_dave_reanim_id);
            match r {
                Some(r) => r as *mut Reanimation,
                None => return,
            }
        };
        unsafe {
            (*reanim_ptr).play_reanim("anim_leave", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 20, 24.0);
            (*reanim_ptr).set_image_override("Dave_mouths", std::ptr::null_mut());
        }
        self.m_crazy_dave_state = CrazyDaveState::Leaving;
        self.m_crazy_dave_message_index = -1;
        self.m_crazy_dave_message_text.clear();
        self.crazy_dave_stop_sound();
    }

    /// 切换到指定戴夫台词（对应 C++ CrazyDaveTalkIndex）
    pub fn crazy_dave_talk_index(&mut self, the_message_index: i32) {
        self.m_crazy_dave_message_index = the_message_index;
        let a_message_text = self.get_crazy_dave_text(the_message_index);
        self.crazy_dave_talk_message(&a_message_text);
    }

    /// 获取戴夫台词（对应 C++ GetCrazyDaveText）
    pub(crate) fn get_crazy_dave_text(&self, the_message_index: i32) -> String {
        let mut a_message = format!("[CRAZY_DAVE_{}]", the_message_index);
        let player_name = self.player_info.as_ref().map_or("", |p| p.name.as_str());
        a_message = a_message.replace("{PLAYER_NAME}", player_name);
        let money = Self::get_money_string(self.player_info.as_ref().map_or(0, |p| p.m_coins));
        a_message = a_message.replace("{MONEY}", &money);
        let a_cost = crate::lawn::widget::store_screen::StoreScreen::get_item_cost(StoreItem::PacketUpgrade);
        let upgrade_cost = Self::get_money_string(a_cost);
        a_message = a_message.replace("{UPGRADE_COST}", &upgrade_cost);
        a_message
    }

    /// 戴夫说话（对应 C++ CrazyDaveTalkMessage）
    pub fn crazy_dave_talk_message(&mut self, the_message: &str) {
        let reanim_ptr: *mut Reanimation = {
            let r = self.reanimation_get_mut(self.m_crazy_dave_reanim_id);
            match r {
                Some(r) => r as *mut Reanimation,
                None => return,
            }
        };

        let mut do_handing = false;
        if the_message.contains("{HANDING}") {
            do_handing = true;
        }
        if (self.m_crazy_dave_state == CrazyDaveState::HandingTalking || self.m_crazy_dave_state == CrazyDaveState::HandingIdling) && !do_handing {
            self.crazy_dave_done_handing();
        }

        let mut do_sound = true;
        if the_message.contains("{NO_SOUND}") {
            do_sound = false;
        } else {
            self.crazy_dave_stop_sound();
        }

        let mut words_count = 0;
        let mut is_control_word = false;
        for byte in the_message.bytes() {
            if byte == b'{' {
                is_control_word = true;
            } else if byte == b'}' {
                is_control_word = false;
            } else if !is_control_word {
                words_count += 1;
            }
        }

        unsafe { (*reanim_ptr).set_image_override("Dave_mouths", std::ptr::null_mut()); }

        if self.m_crazy_dave_state != CrazyDaveState::Talking || do_sound {
            if do_handing {
                unsafe { (*reanim_ptr).play_reanim("anim_talk_handing", crate::todlib::reanimator::ReanimLoopType::Loop, 50, 12.0); }
                if do_sound {
                    if the_message.contains("{SHORT_SOUND}") {
                        self.play_foley(FoleyType::CrazyDaveShort as i32);
                    } else if the_message.contains("{SCREAM}") {
                        self.play_foley(FoleyType::CrazyDaveScream as i32);
                    } else {
                        self.play_foley(FoleyType::CrazyDaveLong as i32);
                    }
                }
                self.m_crazy_dave_state = CrazyDaveState::HandingTalking;
            } else if the_message.contains("{SHAKE}") {
                unsafe { (*reanim_ptr).play_reanim("anim_crazy", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 50, 12.0); }
                if do_sound {
                    self.play_foley(FoleyType::CrazyDaveCrazy as i32);
                }
                self.m_crazy_dave_state = CrazyDaveState::Talking;
            } else if the_message.contains("{SCREAM}") {
                unsafe { (*reanim_ptr).play_reanim("anim_smalltalk", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 50, 12.0); }
                if do_sound {
                    self.play_foley(FoleyType::CrazyDaveScream as i32);
                }
                self.m_crazy_dave_state = CrazyDaveState::Talking;
            } else if the_message.contains("{SCREAM2}") {
                unsafe { (*reanim_ptr).play_reanim("anim_mediumtalk", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 50, 12.0); }
                if do_sound {
                    self.play_foley(FoleyType::CrazyDaveScream2 as i32);
                }
                self.m_crazy_dave_state = CrazyDaveState::Talking;
            } else if the_message.contains("{SHOW_WALLNUT}") {
                unsafe {
                    (*reanim_ptr).play_reanim("anim_talk_handing", crate::todlib::reanimator::ReanimLoopType::Loop, 50, 12.0);
                    // [TRANSLATION_NOTE]: C++ 创建 REANIM_WALLNUT 并 AttachReanim 到
                    // "Dave_handinghand" 轨道（mOffset=1.2 缩放）；Rust 侧 attachment 为骨架，暂略
                    let _wallnut = self.add_reanimation(0.0, 0.0, 0, ReanimationType::Wallnut as i32);
                }
                if do_sound {
                    self.play_foley(FoleyType::CrazyDaveScream2 as i32);
                }
                self.m_crazy_dave_state = CrazyDaveState::HandingTalking;
            } else if the_message.contains("{SHOW_HAMMER}") {
                unsafe {
                    (*reanim_ptr).play_reanim("anim_talk_handing", crate::todlib::reanimator::ReanimLoopType::Loop, 50, 12.0);
                    // [TRANSLATION_NOTE]: C++ 创建 REANIM_HAMMER（anim_whack_zombie, mAnimTime=1.0）
                    // 并 AttachReanim 到 "Dave_handinghand" 轨道（mOffset=1.5 缩放）；attachment 为骨架，暂略
                    let _hammer = self.add_reanimation(0.0, 0.0, 0, ReanimationType::Hammer as i32);
                }
                if do_sound {
                    self.play_foley(FoleyType::CrazyDaveLong as i32);
                }
                self.m_crazy_dave_state = CrazyDaveState::HandingTalking;
            } else if the_message.contains("{SHOW_FERTILIZER}") {
                unsafe {
                    (*reanim_ptr).play_reanim("anim_talk_handing", crate::todlib::reanimator::ReanimLoopType::Loop, 50, 12.0);
                    // [TRANSLATION_NOTE]: C++ 创建 REANIM_ZENGARDEN_FERTILIZER（anim "bag",
                    // mAnimRate=0）并 AttachReanim 到 "Dave_handinghand" 轨道；attachment 为骨架，暂略
                    let _fert = self.add_reanimation(0.0, 0.0, 0, ReanimationType::ZengardenFertilizer as i32);
                }
                if do_sound {
                    self.play_foley(FoleyType::CrazyDaveLong as i32);
                }
                self.m_crazy_dave_state = CrazyDaveState::HandingTalking;
            } else if the_message.contains("{SHOW_TREE_FOOD}") {
                unsafe {
                    (*reanim_ptr).play_reanim("anim_talk_handing", crate::todlib::reanimator::ReanimLoopType::Loop, 50, 12.0);
                    // [TRANSLATION_NOTE]: C++ 创建 REANIM_TREEOFWISDOM_TREEFOOD（anim "bag",
                    // mAnimRate=0）并 AttachReanim 到 "Dave_handinghand" 轨道；attachment 为骨架，暂略
                    let _treefood = self.add_reanimation(0.0, 0.0, 0, ReanimationType::TreeofwisdomTreefood as i32);
                }
                if do_sound {
                    self.play_foley(FoleyType::CrazyDaveLong as i32);
                }
                self.m_crazy_dave_state = CrazyDaveState::HandingTalking;
            } else if the_message.contains("{SHOW_MONEYBAG}") {
                unsafe {
                    (*reanim_ptr).play_reanim("anim_talk_handing", crate::todlib::reanimator::ReanimLoopType::Loop, 50, 12.0);
                    // [TRANSLATION_NOTE]: C++ 创建 REANIM_ZENGARDEN_FERTILIZER（anim "bag",
                    // mAnimRate=0，SetImageOverride("bag", IMAGE_MONEYBAG)）并 AttachReanim 到
                    // "Dave_handinghand" 轨道；attachment 为骨架，暂略
                    let _moneybag = self.add_reanimation(0.0, 0.0, 0, ReanimationType::ZengardenFertilizer as i32);
                }
                if do_sound {
                    self.play_foley(FoleyType::CrazyDaveLong as i32);
                }
                self.m_crazy_dave_state = CrazyDaveState::HandingTalking;
            } else {
                if words_count < 23 {
                    unsafe { (*reanim_ptr).play_reanim("anim_smalltalk", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 50, 12.0); }
                    if do_sound {
                        self.play_foley(FoleyType::CrazyDaveShort as i32);
                    }
                    self.m_crazy_dave_state = CrazyDaveState::Talking;
                } else if words_count < 52 {
                    unsafe { (*reanim_ptr).play_reanim("anim_mediumtalk", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 50, 12.0); }
                    if do_sound {
                        self.play_foley(FoleyType::CrazyDaveLong as i32);
                    }
                    self.m_crazy_dave_state = CrazyDaveState::Talking;
                } else {
                    unsafe { (*reanim_ptr).play_reanim("anim_blahblah", crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold, 50, 12.0); }
                    if do_sound {
                        self.play_foley(FoleyType::CrazyDaveExtraLong as i32);
                    }
                    self.m_crazy_dave_state = CrazyDaveState::Talking;
                }
            }
        }

        self.m_crazy_dave_message_text = the_message.to_string();
    }

    /// 停止戴夫说话（对应 C++ CrazyDaveStopTalking）
    pub fn crazy_dave_stop_talking(&mut self) {
        let mut done_handing = true;
        if self.game_mode == GameMode::Upsell {
            done_handing = false;
        }
        if done_handing && self.m_crazy_dave_state == CrazyDaveState::HandingTalking {
            self.crazy_dave_done_handing();
        }

        let reanim_ptr: *mut Reanimation = {
            let r = self.reanimation_get_mut(self.m_crazy_dave_reanim_id);
            match r {
                Some(r) => r as *mut Reanimation,
                None => return,
            }
        };
        unsafe { (*reanim_ptr).set_image_override("Dave_mouths", std::ptr::null_mut()); }

        if self.m_crazy_dave_state == CrazyDaveState::HandingTalking && !done_handing {
            unsafe { (*reanim_ptr).play_reanim("anim_idle_handing", crate::todlib::reanimator::ReanimLoopType::Loop, 20, 12.0); }
            self.m_crazy_dave_state = CrazyDaveState::HandingIdling;
        } else if self.m_crazy_dave_state == CrazyDaveState::Talking || self.m_crazy_dave_state == CrazyDaveState::HandingTalking {
            unsafe { (*reanim_ptr).play_reanim("anim_idle", crate::todlib::reanimator::ReanimLoopType::Loop, 20, 12.0); }
            self.m_crazy_dave_state = CrazyDaveState::Idling;
        }

        self.m_crazy_dave_message_index = -1;
        self.m_crazy_dave_message_text.clear();
        self.crazy_dave_stop_sound();
    }

    /// 更新疯狂戴夫（对应 C++ UpdateCrazyDave）
    pub fn update_crazy_dave(&mut self) {
        let reanim_ptr: *mut Reanimation = {
            let r = self.reanimation_get_mut(self.m_crazy_dave_reanim_id);
            match r {
                Some(r) => r as *mut Reanimation,
                None => return,
            }
        };

        if self.m_crazy_dave_state == CrazyDaveState::Entering || self.m_crazy_dave_state == CrazyDaveState::Talking {
            if unsafe { (*reanim_ptr).m_loop_count > 0 } {
                unsafe { (*reanim_ptr).play_reanim("anim_idle", crate::todlib::reanimator::ReanimLoopType::Loop, 20, 12.0); }
                self.m_crazy_dave_state = CrazyDaveState::Idling;
            }
        } else if self.m_crazy_dave_state == CrazyDaveState::HandingTalking {
            if unsafe { (*reanim_ptr).m_loop_count > 0 } {
                unsafe { (*reanim_ptr).play_reanim("anim_idle_handing", crate::todlib::reanimator::ReanimLoopType::Loop, 20, 12.0); }
                self.m_crazy_dave_state = CrazyDaveState::HandingIdling;
            }
        } else if self.m_crazy_dave_state == CrazyDaveState::Leaving && unsafe { (*reanim_ptr).m_loop_count > 0 } {
            self.crazy_dave_die();
        }

        if self.m_crazy_dave_state == CrazyDaveState::Idling || self.m_crazy_dave_state == CrazyDaveState::HandingIdling {
            // [TRANSLATION_NOTE]: 嘴部图片覆盖（C++ 使用 IMAGE_REANIM_CRAZYDAVE_MOUTH1/4/5/6）
            // Rust 侧无对应图片资源常量，统一以清除覆盖近似
            unsafe { (*reanim_ptr).set_image_override("Dave_mouths", std::ptr::null_mut()); }
        }

        if self.m_crazy_dave_state == CrazyDaveState::Idling || self.m_crazy_dave_state == CrazyDaveState::Talking
            || self.m_crazy_dave_state == CrazyDaveState::HandingTalking || self.m_crazy_dave_state == CrazyDaveState::HandingIdling
        {
            self.m_crazy_dave_blink_counter -= 1;
            if self.m_crazy_dave_blink_counter <= 0 {
                self.m_crazy_dave_blink_counter = crate::todlib::tod_common::rand_range_int(400, 800);
                if let Some(blink_ptr) = self.add_reanimation(0.0, 0.0, 0, ReanimationType::CrazyDave as i32) {
                    unsafe {
                        (*blink_ptr).set_frames_for_layer("anim_blink");
                        (*blink_ptr).m_loop_type = crate::todlib::reanimator::ReanimLoopType::PlayOnceFullLastFrameAndHold;
                        (*blink_ptr).m_anim_rate = 15.0;
                        // [TRANSLATION_NOTE]: C++ 中 AttachToAnotherReanimation(aCrazyDaveReanim,
                        // "Dave_head")；Rust 侧无对应接口
                        (*blink_ptr).m_color_override = (*reanim_ptr).m_color_override;
                        (*reanim_ptr).assign_render_group_to_track("Dave_eye", -1); // RENDER_GROUP_HIDDEN
                    }
                    self.m_crazy_dave_blink_reanim_id = self.reanimation_get_id(blink_ptr);
                }
            }
        }

        let blink_ptr: *mut Reanimation = {
            let r = self.reanimation_get_mut(self.m_crazy_dave_blink_reanim_id);
            match r {
                Some(r) => r as *mut Reanimation,
                None => std::ptr::null_mut(),
            }
        };
        if !blink_ptr.is_null() && unsafe { (*blink_ptr).m_loop_count > 0 } {
            unsafe { (*reanim_ptr).assign_render_group_to_track("Dave_eye", 0); } // RENDER_GROUP_NORMAL
            self.remove_reanimation(self.m_crazy_dave_blink_reanim_id);
            self.m_crazy_dave_blink_reanim_id = REANIMATIONID_NULL;
        }

        unsafe { (*reanim_ptr).update(); }
    }

    /// 绘制疯狂戴夫（对应 C++ DrawCrazyDave）
    pub fn draw_crazy_dave(&self, g: &mut Graphics) {
        let reanim_ptr: *const Reanimation = {
            let r = self.reanimation_get(self.m_crazy_dave_reanim_id);
            match r {
                Some(r) => r as *const Reanimation,
                None => return,
            }
        };

        if !self.m_crazy_dave_message_text.is_empty() {
            // [TRANSLATION_NOTE]: C++ 中绘制 IMAGE_STORE_SPEECHBUBBLE(2) 气泡与
            // PvzpDrawStringWrapped(FONT_BRIANNETOD16) 台词文本、click to continue 提示；
            // Rust 侧对应图片/字体资源未接入，此处仅保留位置计算逻辑
            let mut a_pos_x = 285;
            let mut a_pos_y = 20;
            if self.base.dialog_map.contains_key(&(Dialogs::Store as i32)) {
                a_pos_x -= 180;
                a_pos_y -= 78;
            } else if self.game_mode == GameMode::Upsell {
                a_pos_x += 130;
                a_pos_y += 70;
            }

            let mut a_bubble_text = self.m_crazy_dave_message_text.clone();
            if a_bubble_text.contains("{SHAKE}") {
                a_bubble_text = a_bubble_text.replace("{SHAKE}", "");
                // C++ 中气泡矩形位置每帧随机偏移（rand()%2）
            }
            let _click_to_continue = self.game_mode != GameMode::Upsell;
        }

        unsafe { (*reanim_ptr).draw(g); }
    }

    /// 推进戴夫台词（对应 C++ AdvanceCrazyDaveText）
    pub fn advance_crazy_dave_text(&mut self) -> bool {
        let a_message_name = format!("[CRAZY_DAVE_{}]", self.m_crazy_dave_message_index + 1);
        // [TRANSLATION_NOTE]: C++ 中 PvzpStringListExists(aMessageName) 检查字符串表
        // 是否存在该台词；Rust 侧用 get_string 近似判定
        if self.base.resource_manager.is_none() {
            return false;
        }
        self.crazy_dave_talk_index(self.m_crazy_dave_message_index + 1);
        true
    }

    // ==================== 杂项 ====================

    pub fn pluralize(count: i32, singular: &str, plural: &str) -> String {
        if count == 1 { format!("{} {}", count, singular) } else { format!("{} {}", count, plural) }
    }

    /// 慢速模式开关（对应 C++ ToggleSlowMo）
    pub fn toggle_slow_mo(&mut self) {
        unsafe {
            G_SLOW_MO_COUNTER = 0;
            G_SLOW_MO = !G_SLOW_MO;
            G_FAST_MO = false;
        }
    }

    /// 快速模式开关（对应 C++ ToggleFastMo）
    pub fn toggle_fast_mo(&mut self) {
        unsafe {
            G_SLOW_MO = false;
            G_FAST_MO = !G_FAST_MO;
        }
    }
    pub fn need_pause_game(&self) -> bool { false }
    pub fn need_register(&self) -> bool { false }

    // ==================== 缺失的方法（GameSelector 需要） ====================

    pub fn is_ice_demo(&self) -> bool { false }
    pub fn is_trial_stage_locked(&self) -> bool {
        if self.m_debug_trial_locked {
            return true;
        }
        self.m_trial_type == TrialType::StageLocked
    }
    pub fn save_file_exists(&self) -> bool { false }
    pub fn is_first_time_adventure_mode(&self) -> bool {
        self.is_adventure_mode() && self.player_info.as_ref().map_or(false, |p| p.m_finished_adventure == 0)
    }
    pub fn earned_gold_trophy(&self) -> bool { false }

    /// 关闭对话框（对应 C++ KillDialog）
    pub fn kill_dialog(&mut self, the_dialog_id: Dialogs) -> bool {
        if self.base.kill_dialog(the_dialog_id as i32) {
            // [TRANSLATION_NOTE]: C++ 中若对话框表为空且无焦点 widget，则将焦点交还
            // Board 或 GameSelector；Rust 侧 widget_manager 焦点跟踪未完全接入，暂略。
            if self.board.is_some() && !self.need_pause_game() {
                unsafe { (*self.board.unwrap()).pause(false); }
            }
            return true;
        }
        false
    }

    /// 关闭模式对话框（对应 C++ ModalClose）
    /// 恢复游戏暂停状态
    pub fn modal_close(&mut self) {
        if let Some(board) = self.board.as_mut() {
            unsafe {
                // C++: mBoard->Pause(false);
                let _ = &**board;
            }
        }
    }

    /// 异步关闭请求（对应 C++ CloseRequestAsync）
    /// 设置退出标志
    pub fn close_request_async(&mut self) {
        self.m_close_request = true;
        // C++ 中还需要设置 mExitToTop = true
    }


    /// 按钮点击回调（对应 C++ LawnApp::ButtonDepress，LawnApp.cpp:1860-2017）
    ///
    /// C++ 语义：按 theId 的百分比范围分流——
    /// - `theId % 10000` 在 [2000, 3000) → 对话框 "Yes" 按钮，id - 2000 是对话框类型
    /// - `theId % 10000` 在 [3000, 4000) → 对话框 "No" 按钮，id - 3000 是对话框类型
    ///
    /// [TRANSLATION_NOTE]: C++ 使用 `theId % 10000` 避免按钮 ID 与对话框 ID 冲突
    /// （例如 DIALOG_USERDIALOG+2000=3029 与另一个 DIALOG 值冲突）；Rust 直接 1:1 保留。
    ///
    /// 此方法闭合以下对话框的 `EditWidgetText` / `mApp->ButtonDepress(mId + 2000)` 调用点：
    /// - NewUserDialog::EditWidgetText → DIALOG_CREATEUSER / DIALOG_RENAMEUSER + 2000
    /// - UserDialog::EditWidgetText → DIALOG_USERDIALOG + 2000
    /// - CheatDialog::EditWidgetText → DIALOG_CHEAT + 2000
    pub fn button_depress(&mut self, the_id: i32) {
        let id_mod = the_id % 10000;
        if id_mod >= 2000 && id_mod < 3000 {
            // C++: ids in [2000, 3000): the "Yes" button of dialog (theId - 2000)
            let dialog_id = the_id - 2000;
            // [TRANSLATION_NOTE]: Rust match 不支持 `Dialogs::X as i32` pattern，
            // 此处先用 transmute 转成枚举变体（带范围检查避免 panic），再匹配枚举。
            let dialog_enum: Dialogs = if (0..Dialogs::NumDialogs as i32).contains(&dialog_id) {
                unsafe { std::mem::transmute::<i32, Dialogs>(dialog_id) }
            } else {
                // 越界：走 C++ default 分支
                let _ = self.base.kill_dialog(the_id);
                return;
            };
            match dialog_enum {
                Dialogs::NewGame => {
                    self.kill_dialog(Dialogs::NewGame);
                    self.show_game_selector();
                }
                Dialogs::NewOptions => {
                    self.kill_new_options_dialog();
                }
                Dialogs::PreGameNag => {
                    // C++: DoRegister(); [TRANSLATION_NOTE]: Rust 无 DoRegister
                    self.kill_dialog(Dialogs::PreGameNag);
                }
                Dialogs::LoadGame => {
                    // C++: return; 保留对话框
                }
                Dialogs::ConfirmUpdateCheck => {
                    // C++: KillDialog + CheckForUpdates；Rust 无 CheckForUpdates，仅关对话框
                    self.kill_dialog(Dialogs::ConfirmUpdateCheck);
                }
                Dialogs::Quit => {
                    self.kill_dialog(Dialogs::Quit);
                    // C++: #if !defined(__IPHONEOS__) CloseRequestAsync();
                    self.close_request_async();
                }
                Dialogs::Nag => {
                    // C++: KillDialog + DoRegister
                    self.kill_dialog(Dialogs::Nag);
                }
                Dialogs::Info => {
                    self.kill_dialog(Dialogs::Info);
                }
                Dialogs::Paused => {
                    self.kill_dialog(Dialogs::Paused);
                }
                Dialogs::NoMoreMoney => {
                    // C++: KillDialog + mBoard->AddSunMoney(100)
                    if let Some(board_ptr) = self.board {
                        unsafe { (*board_ptr).add_sun_money(100); }
                    }
                    self.kill_dialog(Dialogs::NoMoreMoney);
                }
                Dialogs::Bonus => {
                    self.kill_dialog(Dialogs::Bonus);
                }
                Dialogs::ConfirmBackToMain => {
                    // C++: KillDialog + mBoardResult = BOARDRESULT_QUIT + mBoard->TryToSaveGame() + DoBackToMain()
                    self.board_result = BoardResult::Quit;
                    if let Some(board_ptr) = self.board {
                        unsafe { (*board_ptr).try_to_save_game(); }
                    }
                    self.kill_dialog(Dialogs::ConfirmBackToMain);
                    self.do_back_to_main();
                }
                Dialogs::UserDialog => {
                    self.finish_user_dialog(true);
                }
                Dialogs::CreateUser => {
                    self.finish_create_user_dialog(true);
                }
                Dialogs::ConfirmDeleteUser => {
                    self.finish_confirm_delete_user_dialog(true);
                }
                Dialogs::RenameUser => {
                    self.finish_rename_user_dialog(true);
                }
                Dialogs::CreateUserError => {
                    self.finish_name_error(the_id - 2000);
                }
                Dialogs::RenameUserError => {
                    self.finish_name_error(the_id - 2000);
                }
                Dialogs::Cheat => {
                    self.finish_cheat_dialog(true);
                }
                Dialogs::RestartConfirm => {
                    // C++: FinishRestartConfirmDialog(); Rust 侧未实现，走 default kill_dialog 分支
                    self.kill_dialog(Dialogs::RestartConfirm);
                }
                Dialogs::TimesUp => {
                    self.finish_times_up_dialog();
                }
                _ => {
                    // C++: default: KillDialog(theId - 2000); 含 20008 字面量分支
                    // [TRANSLATION_NOTE]: C++ 20008 对应"正在检查更新"对话框的额外 ID；
                    // Rust Dialogs 枚举无对应项，此处 fall through 到 base.kill_dialog 兜底
                    if dialog_id == 20008 {
                        self.kill_dialog(Dialogs::CheckingUpdates);
                    }
                    let _ = self.base.kill_dialog(the_id);
                }
            }
            return;
        }

        if id_mod >= 3000 && the_id < 4000 {
            // C++: ids in [3000, 4000): the "No" button of dialog (theId - 3000)
            let dialog_id = the_id - 3000;
            let dialog_enum: Dialogs = if (0..Dialogs::NumDialogs as i32).contains(&dialog_id) {
                unsafe { std::mem::transmute::<i32, Dialogs>(dialog_id) }
            } else {
                let _ = self.base.kill_dialog(the_id);
                return;
            };
            match dialog_enum {
                Dialogs::PreGameNag => {
                    // C++: KillDialog + Shutdown()
                    self.kill_dialog(Dialogs::PreGameNag);
                    self.shutdown();
                }
                Dialogs::LoadGame => {
                    self.kill_dialog(Dialogs::LoadGame);
                }
                Dialogs::UserDialog => {
                    self.finish_user_dialog(false);
                }
                Dialogs::CreateUser => {
                    self.finish_create_user_dialog(false);
                }
                Dialogs::ConfirmDeleteUser => {
                    self.finish_confirm_delete_user_dialog(false);
                }
                Dialogs::RenameUser => {
                    self.finish_rename_user_dialog(false);
                }
                Dialogs::Cheat => {
                    self.finish_cheat_dialog(false);
                }
                Dialogs::TimesUp => {
                    self.finish_times_up_dialog();
                }
                _ => {
                    // C++: default: KillDialog(theId - 3000); 含 10008 字面量分支
                    // [TRANSLATION_NOTE]: C++ 10008 对应"正在检查更新"对话框的额外 ID
                    if dialog_id == 10008 {
                        self.kill_dialog(Dialogs::CheckingUpdates);
                    }
                    let _ = self.base.kill_dialog(the_id);
                }
            }
            return;
        }
    }

pub fn write_to_registry(&mut self) {
        if let Some(player_info) = &self.player_info {
            // C++: RegistryWriteString("CurUser", mPlayerInfo->mName) — 注册表写入未接入
            // C++: mPlayerInfo->SaveDetails() — 接入 PlayerInfo 存档 IO
            player_info.save_details();
        }
        // C++ 中末尾调用 SexyAppBase::WriteToRegistry()
        // [TRANSLATION_NOTE]: SexyAppBase::WriteToRegistry 未接入
    }
    /// 读取注册表（对应 C++ ReadFromRegistry）
    pub fn read_from_registry(&self, _key: &str, _default: &str) -> String { _default.to_string() }
    /// 写入当前用户配置（对应 C++ WriteCurrentUserConfig）
    pub fn write_current_user_config(&mut self) -> bool {
        // 对应 C++ LawnApp::WriteCurrentUserConfig：mPlayerInfo->SaveDetails() 持久化用户档案
        // （Rust 侧 PlayerInfo::save_details 已实现：序列化并写入 userdata/user{id}.dat）
        if let Some(player_info) = &self.player_info {
            player_info.save_details();
        }
        true
    }
    /// 切换画面模式（对应 C++ SwitchScreenMode）
    pub fn switch_screen_mode(&mut self, windowed: bool, _use_3d: bool, _force: bool) {
        // C++: SexyAppBase::SwitchScreenMode(wantWindowed, is3d, force) — 窗口模式切换
        self.base.is_windowed = windowed;
        // C++: 若 NewOptionsDialog 存在则 mFullscreenCheckbox->SetChecked(!mIsWindowed)
        // [TRANSLATION_NOTE]: Rust 侧 NewOptionsDialog 为独立 widget（fullscreen_checked 字段），
        // 对话框实例挂载管理未接入，此处不联动
    }
    /// 弹出高分对话框（对应 C++ DoHighScoreDialog）
    pub fn do_high_score_dialog(&mut self) {
        // C++ 中该函数体为空（DoRegister 等同类注册对话框同样为空）
    }
    /// 更改目录钩子（对应 C++ ChangeDirHook）
    pub fn change_dir_hook(&self, _path: &str) -> bool { false }
    /// 更新完成关卡的玩家档案（对应 C++ UpdatePlayerProfileForFinishingLevel）
    /// 更新通关后的玩家资料（对应 C++ UpdatePlayerProfileForFinishingLevel）
    pub fn update_player_profile_for_finishing_level(&mut self) -> bool {
        let app_ptr = self as *mut LawnApp;
        // 对应 C++ UpdatePlayerProfileForFinishingLevel：通关后推进关卡/生存/解谜/挑战记录并解锁成就
        let mut a_unlocked_new_challenge = false;

        let board_level = self.board.map_or(0, |b| unsafe { (*b).level });
        let board = self.board;

        if self.is_adventure_mode() {
            if board_level == FINAL_LEVEL {
                if let Some(pi) = self.player_info.as_mut() {
                    pi.set_level(1);
                    pi.m_finished_adventure += 1;
                    if pi.m_finished_adventure == 1 {
                        pi.m_needs_message_on_game_selector = 1;
                    }
                }
                crate::lawn::widget::achievements_screen::ReportAchievement::give_achievement(
                    Some(app_ptr),
                    crate::lawn::widget::achievements_screen::AchievementId::HomeSecurity as i32,
                    false,
                );
            } else if let Some(pi) = self.player_info.as_mut() {
                pi.set_level(board_level + 1);
            }

            if !self.has_finished_adventure() && board_level == 34 {
                if let Some(pi) = self.player_info.as_mut() {
                    pi.m_needs_magic_taco_reward = true;
                }
            }
        } else if self.is_survival_mode() {
            let is_final = board.map_or(false, |b| unsafe { (*b).is_final_survival_stage() });
            if is_final {
                a_unlocked_new_challenge = !self.has_beaten_challenge(self.game_mode);
                if let Some(b) = board {
                    unsafe { (*b).survival_save_score(); }
                }
                if a_unlocked_new_challenge && self.has_finished_adventure() {
                    let a_num_trophies = Self::get_num_trophies(ChallengePage::Survival as i32);
                    if a_num_trophies != 8 && a_num_trophies != 9 {
                        if let Some(pi) = self.player_info.as_mut() {
                            pi.m_has_new_survival = 1;
                        }
                    }
                }
            }
        } else if self.is_puzzle_mode() {
            a_unlocked_new_challenge = !self.has_beaten_challenge(self.game_mode);
            let a_index = self.get_current_challenge_index() as usize;
            if let Some(pi) = self.player_info.as_mut() {
                if let Some(rec) = pi.m_challenge_records.get_mut(a_index) {
                    *rec += 1;
                }
            }
            if !self.has_finished_adventure()
                && (self.game_mode == GameMode::ScaryPotter3 || self.game_mode == GameMode::PuzzleIZombie3)
            {
                a_unlocked_new_challenge = false;
            }
            if a_unlocked_new_challenge {
                let is_scary_potter = self.is_scary_potter_level();
                if let Some(pi) = self.player_info.as_mut() {
                    if is_scary_potter {
                        pi.m_has_new_scary_potter = 1;
                    } else {
                        pi.m_has_new_izombie = 1;
                    }
                }
            }
        } else {
            a_unlocked_new_challenge = !self.has_beaten_challenge(self.game_mode);
            let a_index = self.get_current_challenge_index() as usize;
            let has_finished = self.has_finished_adventure();
            if let Some(pi) = self.player_info.as_mut() {
                if let Some(rec) = pi.m_challenge_records.get_mut(a_index) {
                    *rec += 1;
                }
                if a_unlocked_new_challenge && has_finished {
                    let a_num_trophies = Self::get_num_trophies(ChallengePage::Challenge as i32);
                    if a_num_trophies <= 17 {
                        pi.m_has_new_mini_game = 1;
                    }
                }
                let a_num_trophies = Self::get_num_trophies(ChallengePage::Challenge as i32);
                if a_num_trophies == 20 {
                    crate::lawn::widget::achievements_screen::ReportAchievement::give_achievement(
                        Some(app_ptr),
                        crate::lawn::widget::achievements_screen::AchievementId::BeyondTheGrave as i32,
                        false,
                    );
                }
            }
        }

        if (self.is_adventure_mode() || self.is_survival_mode())
            && !self.is_scary_potter_level()
            && !self.is_whack_a_zombie_level()
        {
            if let Some(b) = board {
                unsafe {
                    let br = &*b;
                    if br.stage_is_day_with_pool() && !br.m_pea_shooter_used {
                        crate::lawn::widget::achievements_screen::ReportAchievement::give_achievement(
                            Some(app_ptr),
                            crate::lawn::widget::achievements_screen::AchievementId::DontPea as i32,
                            false,
                        );
                    } else if br.stage_has_roof()
                        && !br.has_conveyor_belt_seed_bank()
                        && !br.m_catapult_plants_used
                    {
                        crate::lawn::widget::achievements_screen::ReportAchievement::give_achievement(
                            Some(app_ptr),
                            crate::lawn::widget::achievements_screen::AchievementId::Grounded as i32,
                            false,
                        );
                    } else if br.stage_is_day_without_pool() && br.m_mushroom_and_coffee_beans_only {
                        crate::lawn::widget::achievements_screen::ReportAchievement::give_achievement(
                            Some(app_ptr),
                            crate::lawn::widget::achievements_screen::AchievementId::GoodMorning as i32,
                            false,
                        );
                    }
                    if br.stage_is_night() && !br.m_mushrooms_used {
                        crate::lawn::widget::achievements_screen::ReportAchievement::give_achievement(
                            Some(app_ptr),
                            crate::lawn::widget::achievements_screen::AchievementId::NoFungusAmongUs as i32,
                            false,
                        );
                    }
                }
            }
        }

        self.write_current_user_config();

        a_unlocked_new_challenge
    }
    /// URL 打开成功回调（对应 C++ URLOpenSucceeded）
    pub fn url_open_succeeded(&self, _url: &str) {
        // [TRANSLATION_NOTE]: C++ 中无对应方法（Rust 侧为占位，浏览器打开回调暂不处理）
    }
    /// 打开 URL（对应 C++ OpenURL）
    pub fn open_url(&self, _url: &str, _minimized: bool) -> bool { false }
    /// 获取布尔属性（对应 C++ GetBoolean）
    pub fn get_boolean(&self, id: &str, default: bool) -> bool {
        // C++ 中从 mBoolProperties 读取，Rust 简化版始终返回默认值
        let _ = id;
        default
    }

    /// 获取整数属性（对应 C++ GetInteger）
    pub fn get_integer(&self, id: &str, default: i32) -> i32 {
        let _ = id;
        default
    }

    /// 获取加载线程进度（对应 C++ GetLoadingThreadProgress）
    pub fn get_loading_thread_progress(&self) -> f32 {
        if !self.m_loading_thread_started {
            return 0.0;
        }
        if self.m_loading_thread_completed {
            return 1.0;
        }
        if self.m_loading_thread_tasks_total == 0 {
            return 0.0;
        }
        (self.m_loading_thread_tasks_completed as f32 / self.m_loading_thread_tasks_total as f32).min(1.0)
    }

    /// 加载完成回调（对应 C++ LoadingCompleted）
    pub fn loading_completed(&mut self) {
        eprintln!("[LawnApp] LoadingCompleted — 移除 TitleScreen，显示 GameSelector");

        // 移除并销毁 TitleScreen widget
        if let Some(ts) = self.title_screen.take() {
            if let Some(wm) = self.base.widget_manager {
                unsafe {
                    (*wm).remove_widget(ts);
                    // 释放 TitleScreenImpl 和 Widget
                    let _ = Box::from_raw(ts);
                }
            }
        }

        // C++: mResourceManager->DeleteImage("IMAGE_TITLESCREEN");
        if let Some(rm) = self.base.resource_manager {
            unsafe {
                (*rm).delete_image("titlescreen");
            }
        }

        // 显示游戏选择器
        self.show_game_selector();
    }

    /// 启动加载线程（对应 C++ StartLoadingThread）
    /// 在 Rust 中同步加载资源，然后标记完成
    pub fn start_loading_thread(&mut self) {
        if self.m_loading_thread_started {
            return;
        }
        self.m_loading_thread_started = true;
        eprintln!("[LawnApp] 开始加载资源（同步模式）");

        // 加载 LoaderBar 资源组
        if let Some(rm) = self.base.resource_manager {
            unsafe {
                let loaderbar_images = ["loadbar_dirt", "loadbar_grass", "sodrollcap",
                    "titlescreen", "pvz_logo", "popcap_logo", "partner_logo"];
                for img_id in &loaderbar_images {
                    let shared = (*rm).get_image(img_id);
                    if shared.unshared_image.is_null() && shared.shared_image.is_null() {
                        let key = string_to_lower(img_id);
                        if let Some(&ptr) = (*rm).image_map.get(&key) {
                            let res = &mut *(ptr as *mut crate::framework::resource_manager::ImageRes);
                            if !res.base.path.is_empty() {
                                (*rm).load_single_image(res);
                            }
                        }
                    }
                }
            }
        }

        // 标记 LoaderBar 已加载（让 TitleScreen 能绘制加载条）
        // 对应 C++ LoadingThreadProc（LawnApp.cpp:1711-1722）：总任务数 = Σ(资源组资源数×权重) + 636 + 预加载任务数 + 音乐任务数
        if let Some(rm) = self.base.resource_manager {
            unsafe {
                let groups = ["LoadingImages", "LoadingFonts", "LoadingSounds"];
                let weights = [9, 54, 54];
                for i in 0..3 {
                    let count = (*rm).get_num_resources_image(groups[i]);
                    self.m_loading_thread_tasks_total += count * weights[i];
                }
            }
        }
        self.m_loading_thread_tasks_total += 636; // C++ 字面量
        self.m_loading_thread_tasks_total += self.get_num_preloading_tasks();
        // Music::MUSIC_LOADING_TASKS = MUSIC_LOADING_TASK_WEIGHT(3500) × MUSIC_LOADING_FILES(2 项)
        self.m_loading_thread_tasks_total += 3500 * 2;
        eprintln!("[LawnApp] LoaderBar 资源加载完成，继续加载更多资源");

        // 对应 C++ LoadingThreadProc（LawnApp.cpp:1706-1707）：加载属性配置
        // （C++ 中 PvzpStringListLoad/ReadFile 处理 LawnStrings/ZombatarTOS，Rust 字符串表未接入，跳过）
        self.load_properties("properties/default.xml", false, false);
        self.load_properties("properties/Layout.xml", false, false);

        // 加载 LoadingImages 资源组（对应 C++ LoadGroup("LoadingImages", 9)：每资源 +9）
        if let Some(rm) = self.base.resource_manager {
            unsafe {
                let count = (*rm).get_num_resources_image("LoadingImages");
                (*rm).load_resources("LoadingImages");
                self.m_loading_thread_tasks_completed += count * 9;
            }
        }

        // 加载 LoadingFonts 资源组（对应 C++ LoadGroup("LoadingFonts", 54)：每资源 +54）
        if let Some(rm) = self.base.resource_manager {
            unsafe {
                let count = (*rm).get_num_resources_image("LoadingFonts");
                (*rm).load_resources("LoadingFonts");
                self.m_loading_thread_tasks_completed += count * 54;
            }
        }

        // 对应 C++ LoadingThreadProc（LawnApp.cpp:1740-1772）：各子系统初始化

        // mMusic->MusicInit()
        if let Some(music) = self.music.as_mut() {
            music.music_init();
        }

        // mPoolEffect = new PoolEffect(); PoolEffectInitialize()
        if let Some(pool_effect) = self.pool_effect.as_mut() {
            pool_effect.initialize();
        }

        // mZenGarden = new ZenGarden()
        if self.zen_garden.is_none() {
            let mut a_zen_garden = Box::new(ZenGarden::new());
            a_zen_garden.app = Some(self as *mut LawnApp);
            self.zen_garden = Some(Box::into_raw(a_zen_garden));
        }

        // mReanimatorCache = new ReanimatorCache(); ReanimatorCacheInitialize()
        if self.m_reanimator_cache.is_none() {
            self.m_reanimator_cache = Some(Box::into_raw(Box::new(ReanimatorCache::new())));
        }
        if let Some(cache) = self.m_reanimator_cache {
            unsafe {
                (*cache).reanimator_cache_initialize();
            }
        }

        // PvzpFoleyInitialize(gLawnFoleyParamArray, LENGTH(gLawnFoleyParamArray)) — Rust G_LAWN_FOLEY_PARAM_ARRAY（104 项）
        crate::todlib::tod_foley::foley_initialize(&crate::todlib::tod_foley::G_LAWN_FOLEY_PARAM_ARRAY);

        // TrailLoadDefinitions(gLawnTrailArray, LENGTH(gLawnTrailArray)) — Rust G_LAWN_TRAIL_ARRAY 同 1 项（TRAIL_ICE）
        crate::todlib::trail::trail_load_definitions(unsafe { &mut crate::todlib::trail::G_LAWN_TRAIL_ARRAY });

        // PvzpParticleLoadDefinitions(gLawnParticleArray, ...) — [TRANSLATION_NOTE]: Rust 无 gLawnParticleArray 表，传空
        crate::todlib::tod_particle::tod_particle_load_definitions(&[]);

        // LoadGroup("LoadingSounds", 54)
        if let Some(rm) = self.base.resource_manager {
            unsafe {
                let count = (*rm).get_num_resources_image("LoadingSounds");
                (*rm).load_resources("LoadingSounds");
                self.m_loading_thread_tasks_completed += count * 54;

                // C++ LoadGroup 逐资源加载（含声音）：Rust load_resources 仅图像，
                // 声音经 SoundManager.load_sound 加载并写回 SoundRes.sound_id（C++ GetSoundThrow 语义，初始 -1）
                let sound_keys: Vec<String> = (*rm).sound_map.keys()
                    .filter(|k| {
                        if let Some(ptr) = (*rm).sound_map.get(*k) {
                            let res = &*(*ptr as *const crate::framework::resource_manager::SoundRes);
                            string_to_lower(&res.base.res_group) == string_to_lower("LoadingSounds")
                        } else { false }
                    })
                    .cloned()
                    .collect();
                for key in &sound_keys {
                    if let Some(&ptr) = (*rm).sound_map.get(key) {
                        let res = &mut *(ptr as *mut crate::framework::resource_manager::SoundRes);
                        if res.base.path.is_empty() || res.sound_id != -1 {
                            continue;
                        }
                        if let Some(sm) = self.base.sound_manager {
                            let id = (*sm).load_sound(&res.base.path);
                            if id >= 0 {
                                res.sound_id = id as isize;
                                // C++ GetSoundThrow：把 SoundManager 槽位 id 赋给对应 SOUND_XXX 变量
                                crate::todlib::tod_foley::assign_sound_id(key, id);
                                self.m_loading_thread_tasks_completed += 54;
                            }
                        }
                    }
                }
            }
        }

        self.m_loading_thread_completed = true;
        eprintln!("[LawnApp] 资源加载完成");
    }

    /// 加载属性配置文件（对应 C++ SexyAppBase::LoadProperties，SexyAppBase.cpp:3032）
    /// 从 main.pak 读取 xml 并解析到属性表；required=false 时文件缺失返回 true
    pub fn load_properties(&mut self, file_name: &str, required: bool, _check_sig: bool) -> bool {
        let data = crate::framework::paklib::with_pak_interface(|pak| pak.load_file(file_name));
        let data = match data {
            Some(d) => d,
            None => {
                if required {
                    eprintln!("Unable to open properties file {}", file_name);
                    return false;
                }
                return true;
            }
        };
        match crate::framework::properties_parser::parse_properties_buffer(&mut self.base, &data) {
            Ok(()) => true,
            Err(e) => {
                eprintln!("Properties error: {}", e);
                false
            }
        }
    }

    /// 预加载任务数（对应 C++ GetNumPreloadingTasks，LawnApp.cpp:3012；LOW_MEMORY 分支 Rust 无）
    /// = (10 + 已解锁种子数 + 已解锁僵尸数) × 68
    pub fn get_num_preloading_tasks(&self) -> i32 {
        let mut a_task_count = 10;
        if let Some(pi) = self.player_info.as_ref() {
            for i in 0..NUM_SEED_TYPES as i32 {
                let seed = unsafe { std::mem::transmute::<i32, SeedType>(i) };
                if self.has_seed_type(seed) || self.has_finished_adventure() {
                    a_task_count += 1;
                }
            }
            for i in 0..NUM_ZOMBIE_TYPES {
                let ztype = unsafe { std::mem::transmute::<i32, ZombieType>(i) };
                if self.has_finished_adventure()
                    || pi.m_level >= crate::lawn::zombie::get_zombie_definition(ztype).starting_level
                {
                    if ztype != ZombieType::Boss
                        && ztype != ZombieType::Catapult
                        && ztype != ZombieType::Gargantuar
                        && ztype != ZombieType::Digger
                        && ztype != ZombieType::Zamboni
                    {
                        a_task_count += 1;
                    }
                }
            }
        }
        a_task_count * 68
    }

    /// 快速加载（对应 C++ FastLoad）
    pub fn fast_load(&mut self, _mode: GameMode) {
        // 简化版本：直接调用 loading_completed
        self.loading_completed();
    }

    /// 确认退出（对应 C++ ConfirmQuit）
    pub fn confirm_quit(&mut self) {
        // [TRANSLATION_NOTE]: C++ 中 PvzpStringTranslate("[QUIT_HEADER]"/"[QUIT_MESSAGE]")；
        // Rust 侧字符串翻译系统未接入，此处以字面量近似
        let a_header = "[QUIT_HEADER]".to_string();
        let a_body = "[QUIT_MESSAGE]".to_string();
        let a_dialog = self.do_dialog(Dialogs::Quit as i32, true, &a_header, &a_body, "", BUTTONS_OK_CANCEL);
        // [TRANSLATION_NOTE]: C++ 中 aDialog->mLawnYesButton->mLabel = "[QUIT_BUTTON]"；
        // CenterDialog(aDialog, aDialog->mWidth, aDialog->mHeight) 待 widget 层接入
        let _ = a_dialog;
    }

    // ==================== 事件处理 ====================

    pub fn mouse_down(&mut self, x: i32, y: i32, _btn: i32, _cc: i32) {
        if let Some(board_ptr) = self.board {
            unsafe { (*board_ptr).mouse_down(x, y, 1); }
        }
    }

    pub fn key_down(&mut self, key: i32) {
        if let Some(board_ptr) = self.board {
            unsafe { (*board_ptr).key_down(key); }
        }
    }

    // ==================== 调试 ====================

    pub fn debug_key_down(&mut self, _key: i32) -> bool { false }
    pub fn show_resource_error(&mut self, do_exit: bool) {
        // 对应 C++ SexyAppBase::ShowResourceError
        let a_error = "".to_string(); // [TRANSLATION_NOTE]: mResourceManager->GetErrorText() 未接入
        let mut a_message = if a_error.is_empty() {
            String::new()
        } else {
            format!("{}\n\n", a_error)
        };
        // [TRANSLATION_NOTE]: GetResourceFolder() 未接入，使用相对路径说明
        a_message += "Please place main.pak and the properties/ folder into: ./";
        self.base.popup(&a_message);
        if do_exit {
            std::process::exit(1); // C++ DoExit(1)
        }
    }
}

// 全局辅助函数
pub fn lawn_get_current_level_name() -> String {
    LawnApp::instance().map(|a| a.get_current_level_name()).unwrap_or_default()
}

pub fn lawn_get_close_request() -> bool {
    LawnApp::instance().map(|a| a.m_close_request).unwrap_or(false)
}








