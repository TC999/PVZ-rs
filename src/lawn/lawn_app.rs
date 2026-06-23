// PvZ Portable Rust 翻译 — LawnApp（游戏应用）
// 对应 C++ src/LawnApp.h / LawnApp.cpp

use crate::lawn::game_enums::*;
use crate::lawn::board::Board;
use crate::framework::sexy_app_base::SexyAppBase;

/// 游戏应用 — 管理游戏状态、屏幕切换和全局游戏逻辑
pub struct LawnApp {
    /// SexyAppBase 基类（直接内嵌，而非指针）
    pub base: SexyAppBase,

    // 游戏核心
    pub board: Option<*mut Board>,
    pub game_mode: GameMode,
    pub game_screen: GameScreen,

    // 关卡信息
    pub m_level: i32,
    pub m_num_levels: i32,

    // 玩家信息
    pub m_player_info: Option<*mut PlayerInfo>,

    // 对话/界面
    pub m_dialog_id: i32,
    pub m_tutorial_state: TutorialState,

    // 调试
    pub m_debug_keys_enabled: bool,
    pub m_cheat_keys_used: bool,

    // 请求关闭标志
    pub m_close_request: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameScreen {
    Intro = 0,
    Title,
    Loading,
    MainMenu,
    GameSelector,
    Playing,
    ChallengeMenu,
    Store,
    ZenGarden,
    Credits,
    Achievements,
}

/// 玩家信息
pub struct PlayerInfo {
    pub m_name: String,
    pub m_use_seed: [bool; 100],
    pub m_seed_packs: [i32; 100],
    pub m_sun_count: i32,
    pub m_coin_bank: i64,
    pub m_has_used_keys: bool,
}

impl PlayerInfo {
    pub fn new() -> Self {
        PlayerInfo {
            m_name: String::new(),
            m_use_seed: [false; 100],
            m_seed_packs: [0; 100],
            m_sun_count: 50,
            m_coin_bank: 0,
            m_has_used_keys: false,
        }
    }
}

/// 全局游戏实例
static mut g_lawn_app_instance: Option<*mut LawnApp> = None;

impl LawnApp {
    pub fn new() -> Self {
        LawnApp {
            base: SexyAppBase::new(),
            board: None,
            game_mode: GameMode::Adventure,
            game_screen: GameScreen::Intro,
            m_level: 1,
            m_num_levels: NUM_LEVELS,
            m_player_info: None,
            m_dialog_id: -1,
            m_tutorial_state: TutorialState::Off,
            m_debug_keys_enabled: false,
            m_cheat_keys_used: false,
            m_close_request: false,
        }
    }

    /// 全局单例访问
    pub fn instance() -> Option<&'static mut Self> {
        unsafe {
            let ptr = std::ptr::addr_of_mut!(g_lawn_app_instance);
            (*ptr).and_then(|p| p.as_mut())
        }
    }

    /// 初始化游戏（对应 C++ LawnApp::Init → SexyApp::Init → SexyAppBase::Init）
    pub fn init(&mut self) {
        eprintln!("[LawnApp::init] 开始初始化...");

        unsafe {
            g_lawn_app_instance = Some(self as *mut LawnApp);
        }

        // 初始化 SexyAppBase（SDL、窗口、OpenGL、资源等）
        self.base.init();

        eprintln!("[LawnApp::init] base.shutdown={}", self.base.shutdown);
        if self.base.shutdown {
            eprintln!("[LawnApp::init] base 初始化失败，跳过");
            return;
        }

        // 设置标题
        self.base.title = "PvZ Portable".to_string();

        // 初始化完成，设置游戏屏幕
        self.game_screen = GameScreen::Title;
        eprintln!("[LawnApp::init] 初始化完成");
    }

    /// 启动游戏（对应 C++ LawnApp::Start → SexyAppBase::Start → DoMainLoop）
    pub fn start(&mut self) {
        eprintln!("[LawnApp::start] 启动游戏循环...");
        if self.base.shutdown {
            eprintln!("[LawnApp::start] base 已关闭，跳过");
            return;
        }

        // 设置游戏屏幕
        self.game_screen = GameScreen::Title;

        // 调用 SexyAppBase::start() 进入主循环
        // （这会在 while !shutdown 中循环，使用 base 的 start 方法）
        eprintln!("[LawnApp::start] 进入 base.start() 主循环...");
        self.base.start();
        eprintln!("[LawnApp::start] 主循环结束");
    }

    /// 关闭游戏（对应 C++ LawnApp::Shutdown）
    pub fn shutdown(&mut self) {
        self.base.shutdown();
        unsafe {
            g_lawn_app_instance = None;
        }
    }

    /// 开始新游戏
    pub fn new_game(&mut self) {
        self.m_level = 1;
        self.start_level(1);
    }

    /// 开始关卡
    pub fn start_level(&mut self, level: i32) {
        self.m_level = level;
        self.game_screen = GameScreen::Loading;

        // 创建 Board
        let mut board = Box::new(Board::new());
        board.level = level;
        board.board_init(self as *mut LawnApp);

        // 将 Board 转移到全局状态
        self.board = Some(Box::into_raw(board));
        self.game_screen = GameScreen::Playing;
    }

    /// 主更新循环（由 SexyAppBase 每帧调用）
    pub fn update(&mut self) {
        match self.game_screen {
            GameScreen::Playing => {
                if let Some(board_ptr) = self.board {
                    unsafe { (*board_ptr).update(); }
                }
            },
            _ => {
                // 更新菜单等
            }
        }
    }

    /// 主绘制循环（由 SexyAppBase 每帧调用）
    pub fn draw(&self) {
        // 由 GLInterface 处理
    }

    /// 处理鼠标按下
    pub fn mouse_down(&mut self, x: i32, y: i32, _btn: i32, _click_count: i32) {
        // 注意：btn 参数保留以备将来右击支持
        match self.game_screen {
            GameScreen::Playing => {
                if let Some(board_ptr) = self.board {
                    unsafe { (*board_ptr).mouse_down(x, y, 1); }
                }
            },
            _ => {}
        }
    }

    /// 处理按键
    pub fn key_down(&mut self, _key: i32) {
        match self.game_screen {
            GameScreen::Playing => {
                if let Some(board_ptr) = self.board {
                    unsafe { (*board_ptr).key_down(_key); }
                }
            },
            _ => {
                // 菜单按键处理
            }
        }
    }

    /// 获取当前关卡名称
    pub fn get_current_level_name(&self) -> String {
        format!("Level {}", self.m_level)
    }

    /// 检查是否请求关闭
    pub fn get_close_request(&self) -> bool {
        self.m_close_request
    }

    /// 检查是否使用了作弊键
    pub fn has_used_cheat_keys(&self) -> bool {
        self.m_cheat_keys_used
    }
}

// 从 C++ 中全局函数指针对应的 Rust 函数
pub fn lawn_get_current_level_name() -> String {
    if let Some(app) = LawnApp::instance() {
        app.get_current_level_name()
    } else {
        String::new()
    }
}

pub fn lawn_get_close_request() -> bool {
    if let Some(app) = LawnApp::instance() {
        app.get_close_request()
    } else {
        false
    }
}

pub fn lawn_has_used_cheat_keys() -> bool {
    if let Some(app) = LawnApp::instance() {
        app.has_used_cheat_keys()
    } else {
        false
    }
}
