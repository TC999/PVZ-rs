// PvZ Portable Rust 翻译 — LawnApp（游戏应用主类）
// 对应 C++ src/LawnApp.h / LawnApp.cpp
//
// 游戏的核心调度类，管理游戏状态、屏幕切换、资源加载等。

use std::collections::LinkedList;
use crate::lawn::game_enums::*;
use crate::lawn::board::Board;
use crate::lawn::zen_garden::ZenGarden;
use crate::framework::sexy_app_base::SexyAppBase;
use crate::framework::common::string_to_lower;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::widget::dialog::Dialog;
use crate::framework::widget::button_widget::ButtonWidget;
use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::widget::title_screen::{TitleScreenImpl, TitleState};
use crate::lawn::widget::game_selector::GameSelectorImpl;
use crate::todlib::reanimator::Reanimation;
use crate::todlib::tod_particle::TodParticleSystem;
use crate::todlib::tod_foley::FoleyManager;
use crate::todlib::effect_system::EffectSystem;

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
    pub credit_screen: Option<*mut ()>,
    pub challenge_screen: Option<*mut ()>,
    pub zen_garden: Option<*mut ZenGarden>,

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

    // ---- 其他 ----
    pub m_mute_sounds_for_cutscene: bool,
    pub m_first_time_game_selector: bool,
    pub m_register_resources_loaded: bool,
    pub m_last_level_stats: Option<Box<LevelStats>>,

    // ---- 加载线程状态 ----
    pub m_loading_thread_started: bool,
    pub m_loading_thread_completed: bool,
    pub m_loading_thread_tasks_completed: i32,
    pub m_loading_thread_tasks_total: i32,
}

/// 全局游戏实例
static mut G_LAWN_APP_INSTANCE: Option<*mut LawnApp> = None;

impl LawnApp {
    pub fn new() -> Self {
        LawnApp {
            base: SexyAppBase::new(),
            board: None, title_screen: None, game_selector: None,
            seed_chooser_screen: None, award_screen: None,
            credit_screen: None, challenge_screen: None, zen_garden: None,
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
            m_crazy_dave_state: CrazyDaveState::NotHere,
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
            m_register_resources_loaded: false,
            m_last_level_stats: None,
            m_loading_thread_started: false,
            m_loading_thread_completed: false,
            m_loading_thread_tasks_completed: 0,
            m_loading_thread_tasks_total: 0,
        }
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
            self.sound_system = Some(Box::new(crate::todlib::tod_foley::FoleyManager::new(app_ptr)));
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

    /// 主更新循环（由 SexyAppBase 每帧调用）
    pub fn update_frames(&mut self) {
        // 更新游戏世界
        if let Some(board_ptr) = self.board {
            unsafe { (*board_ptr).update(); }
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
        self.m_level = 1;
        self.pre_new_game(self.game_mode, true);
    }

    /// 预新建游戏（对应 C++ PreNewGame）
    pub fn pre_new_game(&mut self, _mode: GameMode, _look_for_saved: bool) {}

    /// 开始关卡
    pub fn start_level(&mut self, level: i32) {
        self.m_level = level;
        self.make_new_board();
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
            unsafe { let _ = Box::from_raw(b); }
        }
    }

    /// 开始游戏（对应 C++ StartPlaying）
    pub fn start_playing(&mut self) {}

    /// 结束关卡（对应 C++ EndLevel）
    pub fn end_level(&mut self) {}

    /// 尝试加载游戏（对应 C++ TryLoadGame）
    pub fn try_load_game(&mut self) -> bool { false }

    /// 检查游戏结束（对应 C++ CheckForGameEnd）
    pub fn check_for_game_end(&mut self) {}

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

    /// 显示奖励屏幕（对应 C++ ShowAwardScreen）
    pub fn show_award_screen(&mut self, _award: i32, _show_achievements: bool) {}
    pub fn kill_award_screen(&mut self) {}

    /// 显示种子选择器
    pub fn show_seed_chooser_screen(&mut self) {}
    pub fn kill_seed_chooser_screen(&mut self) {}

    /// 显示商店（对应 C++ ShowStoreScreen）
    pub fn show_store_screen() {}
    pub fn kill_store_screen() {}

    /// 显示挑战选择
    pub fn show_challenge_screen(&mut self, _page: i32) {
        // 简化版本：仅记录日志
        eprintln!("[LawnApp] 显示挑战选择页 {}", _page);
    }
    pub fn kill_challenge_screen(&mut self) {}

    /// 显示制作人员
    pub fn show_credit_screen(&mut self) {
        eprintln!("[LawnApp] 显示制作人员");
    }
    pub fn kill_credit_screen(&mut self) {}

    /// 显示图鉴（对应 C++ DoAlmanacDialog）
    pub fn do_almanac_dialog(&mut self, _seed: SeedType, _zombie: ZombieType) {}
    pub fn kill_almanac_dialog(&mut self) -> bool { false }

    /// 返回主菜单（对应 C++ DoBackToMain）
    pub fn do_back_to_main(&mut self) {}

    /// 暂停
    pub fn do_pause_dialog(&mut self) {}

    /// 对话框（对应 C++ DoDialog）
    pub fn do_dialog(&mut self, _id: i32, _modal: bool, _header: &str, _lines: &str, _footer: &str, _btn_mode: i32) -> Option<*mut Dialog> { None }

    // ==================== 游戏对象管理 ====================

    /// 添加动画（对应 C++ AddReanimation）
    pub fn add_reanimation(&mut self, _x: f32, _y: f32, _render_order: i32, _type: i32) -> Option<*mut Reanimation> { None }

    /// 添加粒子（对应 C++ AddTodParticle）
    pub fn add_tod_particle(&mut self, _x: f32, _y: f32, _render_order: i32, _effect: i32) -> Option<*mut TodParticleSystem> { None }

    /// 播放音效（对应 C++ PlayFoley）
    pub fn play_foley(&self, _type: i32) {}
    pub fn play_foley_pitch(&self, _type: i32, _pitch: f32) {}

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
    pub fn is_puzzle_mode(&self) -> bool { false }
    pub fn is_challenge_mode(&self) -> bool { false }
    pub fn is_art_challenge(&self) -> bool { false }
    pub fn is_izombie_level(&self) -> bool { false }
    pub fn is_scary_potter_level(&self) -> bool { false }
    pub fn is_whack_a_zombie_level(&self) -> bool { false }
    pub fn is_squirrel_level(&self) -> bool { false }
    pub fn is_shovel_level(&self) -> bool { false }
    pub fn is_wallnut_bowling_level(&self) -> bool { false }
    pub fn is_mini_boss_level(&self) -> bool { false }
    pub fn is_slot_machine_level(&self) -> bool { false }
    pub fn is_stormy_night_level(&self) -> bool { false }
    pub fn is_final_boss_level(&self) -> bool { false }
    pub fn is_bungee_blitz_level(&self) -> bool { false }
    pub fn is_night(&self) -> bool { false }
    pub fn is_challenge_without_seed_bank(&self) -> bool { false }
    pub fn can_show_almanac(&self) -> bool { false }
    pub fn can_show_store(&self) -> bool { false }
    pub fn can_show_zen_garden(&self) -> bool { false }
    pub fn can_pause_now(&self) -> bool { true }
    pub fn can_spawn_yetis(&self) -> bool { false }
    pub fn has_finished_adventure(&self) -> bool { false }
    pub fn has_beaten_challenge(&self, _mode: GameMode) -> bool { false }
    pub fn has_seed_type(&self, _seed: SeedType) -> bool { false }

    pub fn get_seeds_available(&self) -> i32 { 0 }
    pub fn get_current_challenge_def() -> u32 { 0 }
    pub fn get_current_challenge_index(&self) -> i32 { 0 }
    pub fn get_current_level_name(&self) -> String { format!("Level {}", self.m_level) }
    pub fn get_stage_string(level: i32) -> String { format!("Stage {}", level) }
    pub fn get_num_trophies(_page: i32) -> i32 { 0 }
    pub fn get_award_seed_for_level(_level: i32) -> SeedType { SeedType::Peashooter }
    pub fn get_money_string(amount: i32) -> String { format!("${}", amount) }
    pub fn get_close_request(&self) -> bool { self.m_close_request }
    pub fn has_used_cheat_keys(&self) -> bool { self.m_cheat_keys_used }

    // ==================== 疯狂戴夫 ====================

    pub fn crazy_dave_enter(&mut self) {}
    pub fn update_crazy_dave(&mut self) {}
    pub fn crazy_dave_talk_index(&mut self, _idx: i32) {}
    pub fn crazy_dave_talk_message(&mut self, _msg: &str) {}
    pub fn crazy_dave_leave(&mut self) {}
    pub fn draw_crazy_dave(&self, _g: &mut Graphics) {}
    pub fn crazy_dave_die(&mut self) {}
    pub fn crazy_dave_stop_talking(&mut self) {}
    pub fn advance_crazy_dave_text(&mut self) -> bool { false }

    // ==================== 杂项 ====================

    pub fn pluralize(count: i32, singular: &str, plural: &str) -> String {
        if count == 1 { format!("{} {}", count, singular) } else { format!("{} {}", count, plural) }
    }

    pub fn toggle_slow_mo(&mut self) {}
    pub fn toggle_fast_mo(&mut self) {}
    pub fn need_pause_game(&self) -> bool { false }
    pub fn need_register(&self) -> bool { false }

    // ==================== 缺失的方法（GameSelector 需要） ====================

    pub fn is_ice_demo(&self) -> bool { false }
    pub fn is_trial_stage_locked(&self) -> bool { false }
    pub fn save_file_exists(&self) -> bool { false }
    pub fn is_first_time_adventure_mode(&self) -> bool { false }
    pub fn earned_gold_trophy(&self) -> bool { false }
    pub fn kill_dialog(&mut self, _dialog: Dialogs) {}

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
                                self.m_loading_thread_tasks_completed += 1;
                            }
                        }
                    }
                }
            }
        }

        // 标记 LoaderBar 已加载（让 TitleScreen 能绘制加载条）
        self.m_loading_thread_tasks_total = 100;
        eprintln!("[LawnApp] LoaderBar 资源加载完成，继续加载更多资源");

        // 加载 LoadingFonts 资源组
        if let Some(rm) = self.base.resource_manager {
            unsafe {
                (*rm).load_resources("LoadingFonts");
            }
        }
        self.m_loading_thread_tasks_completed += 20;

        // 加载 LoadingImages 资源组
        if let Some(rm) = self.base.resource_manager {
            unsafe {
                (*rm).load_resources("LoadingImages");
            }
        }
        self.m_loading_thread_tasks_completed += 60;

        // TODO: MusicInit, PoolEffect, ZenGarden, ReanimatorCache, Foley, Trails, Particles
        // 这些系统需要后续 Phase 实现

        self.m_loading_thread_tasks_completed = self.m_loading_thread_tasks_total;
        self.m_loading_thread_completed = true;
        eprintln!("[LawnApp] 资源加载完成");
    }

    /// 快速加载（对应 C++ FastLoad）
    pub fn fast_load(&mut self, _mode: GameMode) {
        // 简化版本：直接调用 loading_completed
        self.loading_completed();
    }

    pub fn confirm_quit(&mut self) {}

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
    pub fn show_resource_error(&mut self, _exit: bool) {}
}

// 全局辅助函数
pub fn lawn_get_current_level_name() -> String {
    LawnApp::instance().map(|a| a.get_current_level_name()).unwrap_or_default()
}

pub fn lawn_get_close_request() -> bool {
    LawnApp::instance().map(|a| a.m_close_request).unwrap_or(false)
}
