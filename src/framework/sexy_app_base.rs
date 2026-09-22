// PvZ Portable Rust 翻译 — SexyAppBase（应用程序基类）
// 对应 C++ SexyAppFramework/SexyAppBase.h / SexyAppBase.cpp
//
// SDL2 窗口 + OpenGL 渲染（使用 sdl2 crate 的 OpenGL 支持功能）。

#![allow(dead_code)]

use std::collections::HashMap;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::video::{GLContext, GLProfile, Window};

use crate::framework::color::Color;
use crate::framework::common;
use crate::framework::rect::Rect;
use crate::framework::key_codes::*;
use crate::framework::graphics::gl_image::GLImage;
use crate::framework::graphics::memory_image::MemoryImage;
use crate::framework::graphics::image::Image;
use crate::framework::graphics::gl_interface::GLInterface;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::dialog::Dialog;
use crate::framework::sound::sound_manager::SoundManager;
use crate::framework::sound::music_interface::MusicInterface;
use crate::framework::resource_manager::ResourceManager;
use crate::framework::paklib::{init_pak_interface, with_pak_interface_mut, set_resource_folder};
use crate::framework::buffer::Buffer;

/// 对应 C++ `DEMO_FILE_ID`（SexyAppBase.cpp:83）
pub const DEMO_FILE_ID: u32 = 0x42BEEF78;
/// 对应 C++ `DEMO_VERSION`（SexyAppBase.cpp:84；v6：文本输入以 UTF-8 记录的 DEMO_KEY_TEXT）
pub const DEMO_VERSION: u32 = 6;

// ---- 对应 C++ SexyAppBase.h:102-130 的匿名枚举（demo 命令号）----
pub const DEMO_MOUSE_POSITION: i32 = 0;
pub const DEMO_ACTIVATE_APP: i32 = 1;
pub const DEMO_SIZE: i32 = 2;
pub const DEMO_KEY_DOWN: i32 = 3;
pub const DEMO_KEY_UP: i32 = 4;
pub const DEMO_KEY_CHAR: i32 = 5;
pub const DEMO_CLOSE: i32 = 6;
pub const DEMO_MOUSE_ENTER: i32 = 7;
pub const DEMO_MOUSE_EXIT: i32 = 8;
pub const DEMO_LOADING_COMPLETE: i32 = 9;
pub const DEMO_REGISTRY_GETSUBKEYS: i32 = 10;
pub const DEMO_REGISTRY_READ: i32 = 11;
pub const DEMO_REGISTRY_WRITE: i32 = 12;
pub const DEMO_REGISTRY_ERASE: i32 = 13;
pub const DEMO_FILE_EXISTS: i32 = 14;
pub const DEMO_FILE_READ: i32 = 15;
pub const DEMO_FILE_WRITE: i32 = 16;
pub const DEMO_HTTP_RESULT: i32 = 17;
pub const DEMO_SYNC: i32 = 18;
pub const DEMO_ASSERT_STRING_EQUAL: i32 = 19;
pub const DEMO_ASSERT_INT_EQUAL: i32 = 20;
pub const DEMO_MOUSE_WHEEL: i32 = 21;
pub const DEMO_HANDLE_COMPLETE: i32 = 22;
pub const DEMO_VIDEO_DATA: i32 = 23;
pub const DEMO_KEY_TEXT: i32 = 24;
pub const DEMO_IDLE: i32 = 31;

/// 应用基类
pub static mut G_SEXY_APP: Option<*mut SexyAppBase> = None;

/// 全局 SexyApp 实例指针（对应 C++ gSexyApp）
pub fn get_sexy_app() -> Option<&'static mut SexyAppBase> {
    unsafe { G_SEXY_APP.and_then(|p| p.as_mut()) }
}

pub struct SexyAppBase {
    // SDL2 上下文（使用外部 crate，dropped last 以正确清理）
    pub sdl_context: Option<sdl2::Sdl>,
    pub window: Option<Window>,
    pub gl_context: Option<GLContext>,
    pub event_pump: Option<sdl2::EventPump>,

    // 基本属性
    pub rand_seed: u32,
    pub company_name: String,
    pub prod_name: String,
    pub title: String,
    pub reg_key: String,
    pub resource_dir: String,
    pub custom_save_dir: String,

    // 窗口尺寸
    pub width: i32,
    pub height: i32,
    pub preferred_x: i32,
    pub preferred_y: i32,
    pub fullscreen_bits: i32,

    // 音量
    pub music_volume: f64,
    pub sfx_volume: f64,

    // 窗口状态
    pub is_windowed: bool,
    pub m_shutdown_flag: bool,
    pub minimized: bool,
    pub active: bool,
    pub has_focus: bool,

    // 按键状态
    pub ctrl_down: bool,
    pub alt_down: bool,

    // 帧率
    pub fps_count: i32,
    pub fps_time: i32,
    pub show_fps: bool,
    pub frame_time: i32,

    // 子系统（C++ 构造函数中创建这些对象）
    pub widget_manager: Option<*mut WidgetManager>,
    pub gl_interface: Option<*mut GLInterface>,
    pub sound_manager: Option<*mut dyn SoundManager>,
    pub music_interface: Option<*mut dyn MusicInterface>,
    pub resource_manager: Option<*mut ResourceManager>,

    // 对话框
    pub dialog_map: HashMap<i32, *mut Dialog>,
    /// 对话框有序列表（对应 C++ DialogList mDialogList，SexyAppBase.h:188；
    /// std::list 语义，AddDialog push_back / KillDialog erase 维护）
    pub m_dialog_list: Vec<*mut Dialog>,

    // 光标
    pub cursor_num: i32,

    // 图像集
    pub memory_image_set: Vec<*mut MemoryImage>,

    // 游戏循环
    pub update_app_state: i32,
    pub update_app_depth: i32,
    pub update_multiplier: f64,
    pub paused: bool,

    // 帧钩子（对应 C++ 虚函数分派：LawnApp::UpdateFrames 覆写 SexyAppBase::UpdateFrames）。
    // Rust 组合模型无继承虚分派，由 LawnApp 构造时挂载，主循环 DoUpdateFrames 每帧调用，
    // 以驱动 Board::Update / MusicUpdate / CheckForGameEnd（C++ 中这些由 LawnApp::UpdateFrames 完成）。
    pub(crate) lawn_frame_hook: Option<fn()>,

    // 安全删除列表（对应 C++ mSafeDeleteList）
    pub safe_delete_list: Vec<*mut std::ffi::c_void>,

    // ---- 属性系统（对应 C++ mStringProperties/mBoolProperties/mIntProperties/mDoubleProperties） ----
    pub m_string_properties: std::collections::HashMap<String, String>,
    pub m_bool_properties: std::collections::HashMap<String, bool>,
    pub m_int_properties: std::collections::HashMap<String, i32>,
    pub m_double_properties: std::collections::HashMap<String, f64>,
    pub m_string_vector_properties: std::collections::HashMap<String, Vec<String>>,

    // 主循环控制（对应 C++ mRunning, mLastTime 等）
    pub running: bool,
    pub last_time: u32,
    pub last_time_check: u32,
    pub update_f_time_acc: f64,
    pub has_pending_draw: bool,
    pub m_draw_count: i32,
    pub m_update_count: i32,
    pub is_drawing: bool,
    pub last_draw_tick: u32,
    pub next_draw_tick: u32,

    // 屏幕图像（作为绘制目标，对应 C++ mImage）
    pub screen_image: Option<*mut MemoryImage>,
    // 屏幕图像的 GL 纹理 ID（用于渲染到屏幕）
    pub screen_gl_texture: Option<u32>,

    // ---- SexyApp（继承层）字段 ----
    pub build_num: i32,
    pub build_date: String,
    pub user_name: String,
    pub product_version: String,
    pub demo_prefix: String,
    pub demo_file_name: String,

    // ---- Demo 录制/回放（对应 C++ SexyAppBase.h:294-319）----
    /// 对应 C++ mRecordingDemoBuffer
    pub m_recording_demo_buffer: bool,
    /// 对应 C++ mPlayingDemoBuffer
    pub m_playing_demo_buffer: bool,
    /// 对应 C++ mHasCustomDemoFile（显式文件名参数覆盖自动选择）
    pub m_has_custom_demo_file: bool,
    /// 对应 C++ mDemoRecordFileLimit
    pub m_demo_record_file_limit: u32,
    /// 对应 C++ mDemoPlayIndex（-playnum：按时间戳/名称排序的录制列表下标）
    pub m_demo_play_index: usize,
    /// 对应 C++ mDemoBuffer
    pub m_demo_buffer: Buffer,
    /// 对应 C++ mLastDemoMouseX / mLastDemoMouseY
    pub m_last_demo_mouse_x: i32,
    pub m_last_demo_mouse_y: i32,
    /// 对应 C++ mLastDemoUpdateCnt
    pub m_last_demo_update_cnt: i32,
    /// 对应 C++ mDemoStartTime（会话开始的墙上时钟，作为 demo 同步时钟基准）
    pub m_demo_start_time: u64,
    /// 对应 C++ mDemoTimeZoneOffset（录制者本地时间与 UTC 的秒差）
    pub m_demo_time_zone_offset: i32,
    /// 对应 C++ mDemoNeedsCommand
    pub m_demo_needs_command: bool,
    /// 对应 C++ mDemoIsShortCmd
    pub m_demo_is_short_cmd: bool,
    /// 对应 C++ mDemoCmdNum
    pub m_demo_cmd_num: i32,
    /// 对应 C++ mDemoCmdBitPos
    pub m_demo_cmd_bit_pos: i32,
    /// 对应 C++ mDemoCmdUpdateCnt（当前命令头被读取前的 update tick）
    pub m_demo_cmd_update_cnt: i32,
    /// 对应 C++ mDemoQueuedSince（游戏逻辑持有的命令被排队的 tick）
    pub m_demo_queued_since: i32,
    /// 对应 C++ mDemoCommandQueued
    pub m_demo_command_queued: bool,
    /// 对应 C++ mDemoLoadingComplete
    pub m_demo_loading_complete: bool,
    /// 对应 C++ mDemoMarkerList（`std::list<std::pair<std::string, uint32_t>>`）
    pub m_demo_marker_list: Vec<(String, u32)>,
    /// 对应 C++ mFastForwardToMarker
    pub m_fast_forward_to_marker: bool,
    /// 对应 C++ mDemoMute / mDemoMusicVolume / mDemoSfxVolume
    pub m_demo_mute: bool,
    pub m_demo_music_volume: f64,
    pub m_demo_sfx_volume: f64,
    /// 对应 C++ mSyncRefreshRate（DEMO_VIDEO_DATA 回放时设置）
    pub m_sync_refresh_rate: u8,
    /// 对应 C++ mManualShutdown（手动关闭期间 demo 同步不再读取）
    pub m_manual_shutdown: bool,
    /// 对应 C++ mMouseIn
    pub m_mouse_in: bool,

    // ---- 静音计数与 URL 打开状态（对应 C++ mMuteCount 等，ProcessDemo 分支使用）----
    /// 对应 C++ mMuteCount
    pub m_mute_count: i32,
    /// 对应 C++ mAutoMuteCount
    pub m_auto_mute_count: i32,
    /// 对应 C++ mMuteOnLostFocus
    pub m_mute_on_lost_focus: bool,
    /// 对应 C++ mIsOpeningURL
    pub m_is_opening_url: bool,
    /// 对应 C++ mOpeningURL
    pub m_opening_url: String,
    /// 对应 C++ mShutdownOnURLOpen
    pub m_shutdown_on_url_open: bool,
    /// 对应 C++ mSEHOccured（EnforceCursor 用）
    pub m_seh_occurred: bool,
    /// 对应 C++ mAllowAltEnter（录制时过滤 Alt+Enter 屏幕模式切换）
    pub m_allow_alt_enter: bool,
    /// 对应 C++ mLoaded（资源加载完成；demo 回放期间由 DoUpdateFrames 置位）
    pub m_loaded: bool,
    /// 对应 C++ mLoadingThreadCompleted（加载线程完成标志）
    pub m_loading_thread_completed: bool,
    /// 对应 C++ mLastUserInputTick / mLastTimerTime
    pub m_last_user_input_tick: i32,
    pub m_last_timer_time: i32,
}

/// 将 SDL2 Keycode（C 风格的 int）转换为框架 KeyCode（Windows VK 兼容）
/// 对应 C++ Input.cpp 中的 SDLKeyToKeyCode
fn sdl_keycode_to_keycode(sdl_key: i32) -> KeyCode {
    // 字母 a-z → 'A'-'Z' (65-90)
    if sdl_key >= 'a' as i32 && sdl_key <= 'z' as i32 {
        return (sdl_key - ('a' as i32) + ('A' as i32)) as KeyCode;
    }
    // 数字 0-9 → 48-57
    if sdl_key >= '0' as i32 && sdl_key <= '9' as i32 {
        return sdl_key as KeyCode;
    }
    match sdl_key {
        8   => KEYCODE_BACKSPACE,      // SDLK_BACKSPACE
        9   => KEYCODE_TAB,            // SDLK_TAB
        13  => KEYCODE_RETURN,         // SDLK_RETURN
        27  => KEYCODE_ESCAPE,         // SDLK_ESCAPE
        32  => KEYCODE_SPACE,          // SDLK_SPACE
        127 => KEYCODE_DELETE,         // SDLK_DELETE (ASCII DEL → VK_DELETE 46...)
        // 这里 SDLK_DELETE=127 映射到 46，但 Rust 版 KEYCODE_DELETE=46
        // 直接用 SDL 原始值可能会错，所以特殊处理
        46  => KEYCODE_DELETE,         // 有些系统用 46

        // 方向键
        1073741903 | 273 => KEYCODE_UP,       // SDLK_UP / 某些平台
        1073741904 | 274 => KEYCODE_DOWN,
        1073741902 | 276 => KEYCODE_LEFT,
        1073741905 | 275 => KEYCODE_RIGHT,

        // 功能键
        1073741882 => KEYCODE_F1,
        1073741883 => KEYCODE_F2,
        1073741884 => KEYCODE_F3,
        1073741885 => KEYCODE_F4,
        1073741886 => KEYCODE_F5,
        1073741887 => KEYCODE_F6,
        1073741888 => KEYCODE_F7,
        1073741889 => KEYCODE_F8,
        1073741890 => KEYCODE_F9,
        1073741891 => KEYCODE_F10,
        1073741892 => KEYCODE_F11,
        1073741893 => KEYCODE_F12,

        // 其他
        1073741897 => KEYCODE_INSERT,    // SDLK_INSERT
        1073741898 => KEYCODE_HOME,      // SDLK_HOME
        1073741899 => KEYCODE_PAGE_UP,   // SDLK_PAGEUP
        1073741901 => KEYCODE_END,       // SDLK_END
        1073741900 => KEYCODE_PAGE_DOWN, // SDLK_PAGEDOWN

        // 修饰键
        1073742049 | 1073742053 => KEYCODE_SHIFT,   // LSHIFT / RSHIFT
        1073742048 | 1073742052 => KEYCODE_CONTROL, // LCTRL / RCTRL
        1073742050 | 1073742054 => KEYCODE_ALT,     // LALT / RALT

        _ => sdl_key as KeyCode,
    }
}

/// 对应 C++ `SDLSynthesizeAsciiCharFromKeyDown()`（Input.cpp:256-330）：
/// 从 KeyDown 合成最小 ASCII 字符流，让旧式 `KeyChar` 热键仍然可用。
fn sdl_synthesize_ascii_char_from_key_down(
    the_sym: i32,
    the_mods: sdl2::keyboard::Mod,
    the_text_input_active: bool,
) -> Option<u8> {
    use sdl2::keyboard::Mod;

    // SDL2 keycode 常量（SDL_keycode.h）。
    // 可打印键即为 ASCII 值；小键盘等扫描码键为 `SDL_SCANCODE_TO_KEYCODE(X) = X | (1 << 30)`。
    const SDLK_KP_DIVIDE: i32 = 0x4000_0000 | 84;
    const SDLK_KP_MULTIPLY: i32 = 0x4000_0000 | 85;
    const SDLK_KP_MINUS: i32 = 0x4000_0000 | 86;
    const SDLK_KP_PLUS: i32 = 0x4000_0000 | 87;
    const SDLK_KP_1: i32 = 0x4000_0000 | 89;
    const SDLK_KP_2: i32 = 0x4000_0000 | 90;
    const SDLK_KP_3: i32 = 0x4000_0000 | 91;
    const SDLK_KP_4: i32 = 0x4000_0000 | 92;
    const SDLK_KP_5: i32 = 0x4000_0000 | 93;
    const SDLK_KP_6: i32 = 0x4000_0000 | 94;
    const SDLK_KP_7: i32 = 0x4000_0000 | 95;
    const SDLK_KP_8: i32 = 0x4000_0000 | 96;
    const SDLK_KP_9: i32 = 0x4000_0000 | 97;
    const SDLK_KP_0: i32 = 0x4000_0000 | 98;
    const SDLK_KP_PERIOD: i32 = 0x4000_0000 | 99;
    const SDLK_KP_EQUALS: i32 = 0x4000_0000 | 103;
    const SDLK_A: i32 = b'a' as i32;

    let a_has_ctrl = the_mods.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD);
    let a_has_alt = the_mods.intersects(Mod::LALTMOD | Mod::RALTMOD);
    let a_has_gui = the_mods.intersects(Mod::LGUIMOD | Mod::RGUIMOD);
    let a_has_shift = the_mods.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);

    if a_has_alt || a_has_gui {
        return None;
    }

    if the_sym >= SDLK_A && the_sym <= SDLK_A + 25 {
        if a_has_ctrl {
            // Ctrl+字母 -> 控制码；SDL_TEXTINPUT 不保证覆盖 Ctrl 组合
            return Some((the_sym - SDLK_A + 1) as u8);
        }

        if the_text_input_active {
            return None;
        }

        return Some(if a_has_shift {
            (the_sym - SDLK_A + 'A' as i32) as u8
        } else {
            the_sym as u8
        });
    }

    if a_has_ctrl || the_text_input_active {
        return None;
    }

    let a_char: u8 = match the_sym {
        SDLK_KP_1 => b'1',
        SDLK_KP_2 => b'2',
        SDLK_KP_3 => b'3',
        SDLK_KP_4 => b'4',
        SDLK_KP_5 => b'5',
        SDLK_KP_6 => b'6',
        SDLK_KP_7 => b'7',
        SDLK_KP_8 => b'8',
        SDLK_KP_9 => b'9',
        SDLK_KP_0 => b'0',
        SDLK_KP_PLUS => b'+',
        SDLK_KP_MINUS => b'-',
        SDLK_KP_MULTIPLY => b'*',
        SDLK_KP_DIVIDE => b'/',
        SDLK_KP_PERIOD => b'.',
        SDLK_KP_EQUALS => b'=',
        49 => if a_has_shift { b'!' } else { b'1' },  // SDLK_1
        50 => if a_has_shift { b'@' } else { b'2' },  // SDLK_2
        51 => if a_has_shift { b'#' } else { b'3' },  // SDLK_3
        52 => if a_has_shift { b'$' } else { b'4' },  // SDLK_4
        53 => if a_has_shift { b'%' } else { b'5' },  // SDLK_5
        54 => if a_has_shift { b'^' } else { b'6' },  // SDLK_6
        55 => if a_has_shift { b'&' } else { b'7' },  // SDLK_7
        56 => if a_has_shift { b'*' } else { b'8' },  // SDLK_8
        57 => if a_has_shift { b'(' } else { b'9' },  // SDLK_9
        48 => if a_has_shift { b')' } else { b'0' },  // SDLK_0
        45 => if a_has_shift { b'_' } else { b'-' },  // SDLK_MINUS
        61 => if a_has_shift { b'+' } else { b'=' },  // SDLK_EQUALS
        91 => if a_has_shift { b'{' } else { b'[' },  // SDLK_LEFTBRACKET
        93 => if a_has_shift { b'}' } else { b']' },  // SDLK_RIGHTBRACKET
        92 => if a_has_shift { b'|' } else { b'\\' }, // SDLK_BACKSLASH
        59 => if a_has_shift { b':' } else { b';' },  // SDLK_SEMICOLON
        39 => if a_has_shift { b'"' } else { b'\'' }, // SDLK_QUOTE
        44 => if a_has_shift { b'<' } else { b',' },  // SDLK_COMMA
        46 => if a_has_shift { b'>' } else { b'.' },  // SDLK_PERIOD
        47 => if a_has_shift { b'?' } else { b'/' },  // SDLK_SLASH
        96 => if a_has_shift { b'~' } else { b'`' },  // SDLK_BACKQUOTE
        32 => b' ',                                   // SDLK_SPACE
        _ => return None,
    };

    Some(a_char)
}

impl SexyAppBase {
    pub fn new() -> Self {
        // 初始化 SDL2（对应 C++ 构造函数中的 SDL_Init）
        let sdl_context = sdl2::init().expect("SDL2 初始化失败");

        // 创建 WidgetManager（对应 C++ 构造函数 line 399: new WidgetManager(this)）
        let wm = WidgetManager::new();

        SexyAppBase {
            sdl_context: Some(sdl_context),
            window: None,
            gl_context: None,
            event_pump: None,
            rand_seed: 0,
            company_name: String::new(),
            prod_name: String::from("Product"),
            title: String::from("Plants vs. Zombies"),
            reg_key: String::new(),
            resource_dir: String::new(),
            custom_save_dir: String::new(),
            width: 800,
            height: 600,
            preferred_x: 0,
            preferred_y: 0,
            fullscreen_bits: 32,
            music_volume: 0.85,
            sfx_volume: 0.85,
            is_windowed: true,
            m_shutdown_flag: false,
            minimized: false,
            active: true,
            has_focus: true,
            ctrl_down: false,
            alt_down: false,
            fps_count: 0,
            fps_time: 0,
            show_fps: false,
            frame_time: 10,
            widget_manager: Some(Box::into_raw(Box::new(wm))),
            gl_interface: None,
            sound_manager: None,
            music_interface: None,
            resource_manager: None,
            dialog_map: HashMap::new(),
            m_dialog_list: Vec::new(),
            build_num: 0,
            build_date: String::new(),
            user_name: String::new(),
            product_version: String::from("1.0"),
            demo_prefix: String::from("pvzp"),
            demo_file_name: String::from("pvzp.dmo"),
            // ---- 对应 C++ SexyAppBase.cpp:374-393 的 demo 字段初始化 ----
            m_recording_demo_buffer: false,
            m_playing_demo_buffer: false,
            m_has_custom_demo_file: false,
            m_demo_record_file_limit: 0,
            m_demo_play_index: 0,
            m_demo_buffer: Buffer::new(),
            m_last_demo_mouse_x: 0,
            m_last_demo_mouse_y: 0,
            m_last_demo_update_cnt: 0,
            m_demo_start_time: 0,
            m_demo_time_zone_offset: 0,
            // C++: mDemoNeedsCommand = true
            m_demo_needs_command: true,
            m_demo_is_short_cmd: false,
            m_demo_cmd_num: 0,
            m_demo_cmd_bit_pos: 0,
            m_demo_cmd_update_cnt: 0,
            m_demo_queued_since: 0,
            m_demo_command_queued: false,
            m_demo_loading_complete: false,
            m_demo_marker_list: Vec::new(),
            m_fast_forward_to_marker: false,
            m_demo_mute: false,
            m_demo_music_volume: 0.0,
            m_demo_sfx_volume: 0.0,
            m_sync_refresh_rate: 0,
            m_manual_shutdown: false,
            m_mouse_in: false,
            m_mute_count: 0,
            m_auto_mute_count: 0,
            m_mute_on_lost_focus: false,
            m_is_opening_url: false,
            m_opening_url: String::new(),
            m_shutdown_on_url_open: false,
            m_seh_occurred: false,
            m_allow_alt_enter: false,
            m_loaded: false,
            m_loading_thread_completed: false,
            m_last_user_input_tick: 0,
            m_last_timer_time: 0,
            cursor_num: 0,
            memory_image_set: Vec::new(),
            update_app_state: 0,
            update_app_depth: 0,
            update_multiplier: 1.0,
            paused: false,
            lawn_frame_hook: None,
            safe_delete_list: Vec::new(),
            m_string_properties: std::collections::HashMap::new(),
            m_bool_properties: std::collections::HashMap::new(),
            m_int_properties: std::collections::HashMap::new(),
            m_double_properties: std::collections::HashMap::new(),
            m_string_vector_properties: std::collections::HashMap::new(),
            running: false,
            last_time: 0,
            last_time_check: 0,
            update_f_time_acc: 0.0,
            has_pending_draw: true,
            m_draw_count: 0,
            m_update_count: 0,
            is_drawing: false,
            last_draw_tick: 0,
            next_draw_tick: 0,
            screen_image: None,
            screen_gl_texture: None,
        }
    }

    /// 初始化（对应 C++ SexyAppBase::Init）
    pub fn init(&mut self) {
        // 设置资源目录
        self.resource_dir = common::get_cur_dir();

        // 初始化 PakInterface 并加载 main.pak（对应 C++ Init 中的 AddPakFile）
        init_pak_interface();
        set_resource_folder(&self.resource_dir);
        with_pak_interface_mut(|pak| {
            let pak_path = format!("{}/main.pak", self.resource_dir.trim_end_matches('/'));
            pak.add_pak_file(&pak_path);
        });

        // 创建 ResourceManager（对应 C++ 构造函数中的 new ResourceManager(this)）
        if self.resource_manager.is_none() {
            let app_ptr: *mut SexyAppBase = self;
            let rm = Box::new(ResourceManager::new(Some(app_ptr)));
            self.resource_manager = Some(Box::into_raw(rm));
        }

        // 初始化音频系统（对应 C++ Init 中的 new SDLSoundManager + CreateMusicInterface）
        self.init_sound_system();

        // 创建窗口和 GL 上下文（对应 C++ MakeWindow）
        self.make_window();

        if self.m_shutdown_flag {
            return;
        }

        // WidgetManager resize（对应 C++ Init 中的 mWidgetManager->Resize）
        if let Some(wm) = self.widget_manager {
            unsafe {
                (*wm).resize(Rect::new(0, 0, self.width, self.height), Rect::new(0, 0, self.width, self.height));
            }
        }

        // 创建屏幕图像（对应 C++ 中 Graphics aScrG(mImage) 的 mImage）
        let mut screen = Box::new(MemoryImage::new(self.width, self.height));
        screen.create(self.width, self.height);
        screen.clear(Color::BLACK);
        self.screen_image = Some(Box::into_raw(screen));

        self.load_resource_manifest();
    }

    /// 创建 SDL 窗口和 OpenGL 上下文（使用 sdl2 crate 的 GL 支持）
    pub fn make_window(&mut self) {
        let sdl = match self.sdl_context.as_ref() {
            Some(s) => s,
            None => {
                eprintln!("SDL2 未初始化");
                self.m_shutdown_flag = true;
                return;
            }
        };

        let video = match sdl.video() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("SDL 视频子系统初始化失败: {e}");
                self.m_shutdown_flag = true;
                return;
            }
        };

        // 尝试 GLES 2.0 配置
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(GLProfile::GLES);
        gl_attr.set_context_version(2, 0);
        gl_attr.set_double_buffer(true);
        gl_attr.set_depth_size(0);

        // 创建窗口（带 OpenGL 标记）
        let window_result = video
            .window(&self.title, self.width as u32, self.height as u32)
            .position_centered()
            .opengl()
            .resizable()
            .build();

        let (window, gl_context) = match window_result {
            Ok(win) => {
                match win.gl_create_context() {
                    Ok(ctx) => (win, ctx),
                    Err(e) => {
                        eprintln!("GLES 2.0 上下文创建失败: {e}，回退到桌面 GL 2.1");
                        drop(win);
                        // 回退到 desktop GL 2.1
                        gl_attr.set_context_profile(GLProfile::Compatibility);
                        gl_attr.set_context_version(2, 1);

                        let win2 = video
                            .window(&self.title, self.width as u32, self.height as u32)
                            .position_centered()
                            .opengl()
                            .resizable()
                            .build();

                        match win2 {
                            Ok(w) => match w.gl_create_context() {
                                Ok(ctx) => {
                                    // 标记使用桌面 GL，着色器用 #version 120
                                    unsafe {
                                        crate::framework::graphics::gl_interface::G_DESKTOP_GL_FALLBACK = true;
                                    }
                                    (w, ctx)
                                }
                                Err(e2) => {
                                    eprintln!("回退 GL 上下文创建失败: {e2}");
                                    self.m_shutdown_flag = true;
                                    return;
                                }
                            },
                            Err(e2) => {
                                eprintln!("回退窗口创建失败: {e2}");
                                self.m_shutdown_flag = true;
                                return;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("SDL_CreateWindow 失败: {e}");
                self.m_shutdown_flag = true;
                return;
            }
        };

        // 设置交换间隔
        if let Err(e) = video.gl_set_swap_interval(1) {
            eprintln!("设置交换间隔失败: {e}");
        }

        // 保存窗口和上下文
        self.window = Some(window);
        self.gl_context = Some(gl_context);

        // 初始化 GLInterface（编译着色器、创建 VBO 等）
        if self.gl_interface.is_none() {
            let mut gl = Box::new(GLInterface::new(None));
            gl.init(true);
            self.gl_interface = Some(Box::into_raw(gl));
        }
    }

    // ==================== 主循环 ====================

    /// 启动主循环（对应 C++ SexyAppBase::Start + DoMainLoop）
    pub fn start(&mut self) {
        if self.m_shutdown_flag {
            return;
        }

        // 创建事件泵
        if let Some(ref sdl) = self.sdl_context {
            self.event_pump = sdl.event_pump().ok();
        }

        self.running = true;
        // sdl2::timer 模块的 ticks() 函数不可用，直接调用 SDL_GetTicks 通过 sys
        self.last_time = unsafe { sdl2::sys::SDL_GetTicks() };

        // 对应 C++ DoMainLoop: while (!mShutdown) { UpdateApp(); }
        while !self.m_shutdown_flag {
            self.update_app();
        }

        self.running = false;
    }

    /// 主循环单步（对应 C++ UpdateApp 的简化版本）
    /// 包含: 回放 demo → 事件处理 → 更新游戏逻辑 → 绘制
    pub fn update_app(&mut self) -> bool {
        // 0. 回放 demo 命令流
        // 对应 C++ UpdateApp 中 UPDATESTATE_MESSAGES 阶段的 ProcessDemo()（SexyAppBase.cpp:2912），
        // 位置在 ProcessDeferredMessages 之前
        self.process_demo();

        if self.m_shutdown_flag {
            return false;
        }

        // 1. 处理 SDL 事件（对应 C++ ProcessDeferredMessages）
        if !self.process_deferred_messages(true) {
            return false;
        }

        if self.m_shutdown_flag {
            return false;
        }

        // 2. 更新帧（对应 C++ Process → DoUpdateFrames → UpdateFrames）
        self.do_update_frames();

        // 3. 绘制脏区域（对应 C++ DrawDirtyStuff）
        self.draw_dirty_stuff();

        // 4. 交换缓冲区（对应 C++ Redraw → GLInterface 的 SwapBuffers）
        self.swap_buffers();

        true
    }

    /// 更新帧（对应 C++ `SexyAppBase::DoUpdateFrames`，SexyAppBase.cpp:1688-1725）
    fn do_update_frames(&mut self) -> bool {
        if self.m_playing_demo_buffer {
            // demo 回放：等 DEMO_LOADING_COMPLETE 信号后才进入正常帧更新
            if self.m_loading_thread_completed && !self.m_loaded && self.m_demo_loading_complete {
                self.m_loaded = true;
                // C++: mYieldMainThread = false; LoadingThreadCompleted();
            }

            // 排队中的命令要等游戏逻辑认领；放行 tick 以便认领或判定为回放跑偏
            if (self.m_loaded == self.m_demo_loading_complete)
                && ((self.m_update_count != self.m_last_demo_update_cnt) || self.m_demo_command_queued)
            {
                self.update_frames_internal();
                return true;
            }

            return false;
        }

        if self.m_loading_thread_completed && !self.m_loaded {
            self.m_loaded = true;
            // C++: mYieldMainThread = false; LoadingThreadCompleted();

            if self.m_recording_demo_buffer {
                self.write_demo_timing_block();
                self.m_demo_buffer.write_num_bits(0, 1);
                self.m_demo_buffer.write_num_bits(DEMO_LOADING_COMPLETE, 5);
            }
        }

        self.update_frames_internal();
        true
    }

    /// 对应 C++ `SexyAppBase::UpdateFrames`（含到 LawnApp 覆写的虚分派部分）
    fn update_frames_internal(&mut self) {
        // 对应 C++ UpdateFrames: mWidgetManager->UpdateFrame()
        if let Some(wm) = self.widget_manager {
            unsafe {
                (*wm).update();
            }
        }
        // 对应 C++ 虚函数分派：调用 LawnApp::UpdateFrames 覆写中除 WidgetManager 外的其余部分
        //（mBoard->ProcessDeleteQueue / Board::Update / mMusic->MusicUpdate / CheckForGameEnd）。
        if let Some(hook) = self.lawn_frame_hook {
            hook();
        }
        self.m_update_count += 1;
    }

    /// 绘制脏区域（对应 C++ SexyAppBase::DrawDirtyStuff）
    ///
    /// 软件渲染到 screen_image（MemoryImage），再通过 OpenGL 纹理上传并渲染全屏四边形。
    fn draw_dirty_stuff(&mut self) -> bool {
        // ===== 软件渲染：WidgetManager 绘制到 screen_image =====
        if let Some(wm) = self.widget_manager {
            unsafe {
                let screen_ptr = self.screen_image
                    .map(|p| p as *mut crate::framework::graphics::image::Image)
                    .unwrap_or(std::ptr::null_mut());
                let mut g = crate::framework::graphics::graphics::Graphics::new_with_image(screen_ptr);
                (*wm).draw(&mut g);
            }
        }

        // ===== 将 screen_image 通过 OpenGL 渲染到屏幕 =====
        if let Some(screen_ptr) = self.screen_image {
            unsafe {
                use crate::ffi::opengl::*;
                use crate::framework::graphics::gl_interface::GLVertex;
                use std::mem::size_of;
                use std::ptr;

                let mem = &*screen_ptr;
                let w = mem.base.width;
                let h = mem.base.height;

                if w <= 0 || h <= 0 || mem.base.pixels.is_empty() {
                    self.m_draw_count += 1;
                    return true;
                }

                // 清除屏幕
                if let Some(gl) = self.gl_interface {
                    (*gl).pre_draw();
                }

                // 获取或创建 GL 纹理
                let tex_id = match self.screen_gl_texture {
                    Some(id) => id,
                    None => {
                        let mut id: GLuint = 0;
                        glGenTextures(1, &mut id);
                        self.screen_gl_texture = Some(id);
                        id
                    }
                };

                // 上传像素数据到纹理
                glActiveTexture(GL_TEXTURE0);
                glBindTexture(GL_TEXTURE_2D, tex_id);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as GLint);
                glTexImage2D(
                    GL_TEXTURE_2D, 0, GL_RGBA as GLint, w, h, 0,
                    GL_RGBA, GL_UNSIGNED_BYTE,
                    mem.base.pixels.as_ptr() as *const std::ffi::c_void,
                );

                // 使用现有 GL 着色器程序
                glUseProgram(crate::framework::graphics::gl_interface::G_PROGRAM);

                // 设置正交投影矩阵（对应 C++ MakeOrthoMatrix，Y 轴与 OpenGL 屏幕坐标对齐）
                let ortho: [f32; 16] = [
                    2.0 / w as f32, 0.0, 0.0, 0.0,
                    0.0, -2.0 / h as f32, 0.0, 0.0,
                    0.0, 0.0, -2.0 / 20.0, 0.0,
                    -1.0, 1.0, -10.0 / 20.0, 1.0,
                ];
                glUniformMatrix4fv(
                    crate::framework::graphics::gl_interface::G_UF_VIEW_PROJ_MTX,
                    1, GL_FALSE, ortho.as_ptr(),
                );
                glUniform1i(
                    crate::framework::graphics::gl_interface::G_UF_TEXTURE,
                    0,
                );
                glUniform1i(
                    crate::framework::graphics::gl_interface::G_UF_USE_TEXTURE,
                    1,
                );
                glUniform4fv(
                    crate::framework::graphics::gl_interface::G_UF_UV_BOUNDS,
                    1,
                    [0.0f32, 0.0, 1.0, 1.0].as_ptr(),
                );
                glUniform1i(
                    crate::framework::graphics::gl_interface::G_UF_CLAMP_UV_ENABLED,
                    1,
                );

                // 渲染全屏四边形（两个三角形组成 Triangle Strip）
                let verts: [GLVertex; 4] = [
                    GLVertex { sx: 0.0, sy: 0.0, sz: 0.0, color: 0xFFFFFFFF, tu: 0.0, tv: 0.0 },
                    GLVertex { sx: w as f32, sy: 0.0, sz: 0.0, color: 0xFFFFFFFF, tu: 1.0, tv: 0.0 },
                    GLVertex { sx: 0.0, sy: h as f32, sz: 0.0, color: 0xFFFFFFFF, tu: 0.0, tv: 1.0 },
                    GLVertex { sx: w as f32, sy: h as f32, sz: 0.0, color: 0xFFFFFFFF, tu: 1.0, tv: 1.0 },
                ];

                // 使用已有 VBO 和顶点属性配置
                let vbo = crate::framework::graphics::gl_interface::G_VBO;
                glBindBuffer(GL_ARRAY_BUFFER, vbo);
                glBufferSubData(
                    GL_ARRAY_BUFFER, 0,
                    (size_of::<GLVertex>() * 4) as GLsizeiptr,
                    verts.as_ptr() as *const std::ffi::c_void,
                );

                // Position (3 floats, offset 0)
                glEnableVertexAttribArray(0);
                glVertexAttribPointer(
                    0, 3, GL_FLOAT, GL_FALSE, size_of::<GLVertex>() as GLsizei,
                    ptr::null(),
                );
                // Color (4 ubytes normalized, offset 12 = 3*4)
                glEnableVertexAttribArray(1);
                glVertexAttribPointer(
                    1, 4, GL_UNSIGNED_BYTE, GL_TRUE, size_of::<GLVertex>() as GLsizei,
                    (size_of::<f32>() * 3) as *const std::ffi::c_void,
                );
                // UV (2 floats, offset 16 = 3*4 + 4)
                glEnableVertexAttribArray(2);
                glVertexAttribPointer(
                    2, 2, GL_FLOAT, GL_FALSE, size_of::<GLVertex>() as GLsizei,
                    (size_of::<f32>() * 3 + size_of::<u32>()) as *const std::ffi::c_void,
                );

                glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);
            }
        }

        self.m_draw_count += 1;
        true
    }

    // ==================== 安全删除 ====================

    /// 处理安全删除列表（对应 C++ ProcessSafeDeleteList）
    /// 仅在 update_app_depth 不增加时删除挂起的 widget
    pub fn process_safe_delete_list(&mut self) {
        let depth = self.update_app_depth;
        self.safe_delete_list.retain(|&ptr| {
            // 在 C++ 中这里 delete 了 widget
            // Rust 中由调用方管理生命周期，此处仅清除列表
            false // 始终移除（简化实现）
        });
    }

    // ==================== 事件处理 ====================

    /// 处理 SDL 事件队列（对应 C++ ProcessDeferredMessages）
    pub fn process_deferred_messages(&mut self, _allow_quit: bool) -> bool {
        let pump = match self.event_pump.as_mut() {
            Some(p) => p,
            None => return true,
        };

        // 先收集所有待处理事件，避免对 self 的双重可变借用
        let events: Vec<Event> = pump.poll_iter().collect();

        for event in events {
            // 对应 C++ ProcessDeferredMessages：录制时把事件写进 demo 流
            if self.m_recording_demo_buffer && !self.m_shutdown_flag {
                self.record_demo_event(&event);
            }

            // 对应 C++：回放模式下输入由 demo 流重放，这里只处理窗口管理事件
            if self.m_playing_demo_buffer {
                match &event {
                    Event::Quit { .. } => {
                        self.m_shutdown_flag = true;
                        return false;
                    }
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Close => {
                            self.m_shutdown_flag = true;
                            return false;
                        }
                        WindowEvent::Resized(w, h) | WindowEvent::SizeChanged(w, h) => {
                            self.width = *w;
                            self.height = *h;
                            if let Some(gl) = self.gl_interface.as_mut() {
                                unsafe {
                                    (**gl).width = self.width;
                                    (**gl).height = self.height;
                                    (**gl).update_viewport();
                                }
                            }
                            if let Some(wm) = self.widget_manager {
                                unsafe {
                                    (*wm).resize(
                                        Rect::new(0, 0, self.width, self.height),
                                        Rect::new(0, 0, self.width, self.height),
                                    );
                                    (*wm).mark_all_dirty();
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
                continue;
            }

            match event {
                Event::Quit { .. } => {
                    self.m_shutdown_flag = true;
                    return false;
                }

                Event::Window { win_event, .. } => {
                    match win_event {
                        WindowEvent::Close => {
                            self.m_shutdown_flag = true;
                            return false;
                        }
                        WindowEvent::Resized(w, h) | WindowEvent::SizeChanged(w, h) => {
                            self.width = w;
                            self.height = h;
                            if let Some(gl) = self.gl_interface.as_mut() {
                                unsafe {
                                    (**gl).width = self.width;
                                    (**gl).height = self.height;
                                    (**gl).update_viewport();
                                }
                            }
                            if let Some(wm) = self.widget_manager {
                                unsafe {
                                    (*wm).resize(
                                        Rect::new(0, 0, self.width, self.height),
                                        Rect::new(0, 0, self.width, self.height),
                                    );
                                }
                            }
                        }
                        WindowEvent::FocusGained => {
                            self.has_focus = true;
                            self.active = true;
                        }
                        WindowEvent::FocusLost => {
                            self.has_focus = false;
                            self.active = false;
                        }
                        WindowEvent::Minimized => {
                            self.minimized = true;
                        }
                        WindowEvent::Restored => {
                            self.minimized = false;
                        }
                        _ => {}
                    }
                }

                Event::KeyDown {
                    keycode: Some(kc),
                    keymod,
                    ..
                } => {
                    self.ctrl_down = keymod.intersects(
                        sdl2::keyboard::Mod::LCTRLMOD | sdl2::keyboard::Mod::RCTRLMOD,
                    );
                    self.alt_down = keymod.intersects(
                        sdl2::keyboard::Mod::LALTMOD | sdl2::keyboard::Mod::RALTMOD,
                    );
                    let key: i32 = (*kc).into();
                    self.key_down(sdl_keycode_to_keycode(key));
                }

                Event::KeyDown { .. } => {} // no keycode

                Event::KeyUp {
                    keycode: Some(kc), ..
                } => {
                    let key: i32 = (*kc).into();
                    self.key_up(sdl_keycode_to_keycode(key));
                }

                Event::KeyUp { .. } => {} // no keycode

                Event::MouseButtonDown {
                    mouse_btn, x, y, clicks, ..
                } => {
                    // C++ 编码：左键=1/2, 右键=-1/-2, 中键=3
                    let btn = match mouse_btn {
                        MouseButton::Left => clicks as i32,       // 单击=1, 双击=2
                        MouseButton::Right => -(clicks as i32),   // 单击=-1, 双击=-2
                        MouseButton::Middle => 3,                 // 中键=3
                        _ => clicks as i32,
                    };
                    self.mouse_down(x, y, btn, clicks as i32);
                }

                Event::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => {
                    let btn = match mouse_btn {
                        MouseButton::Left => 1,
                        MouseButton::Right => -1,
                        MouseButton::Middle => 3,
                        _ => 0,
                    };
                    self.mouse_up(x, y, btn);
                }

                Event::MouseMotion { x, y, .. } => {
                    self.mouse_move(x, y);
                }

                _ => {}
            }
        }

        true
    }

    // ==================== 绘制 ====================

    /// 交换缓冲区（对应 C++ SDL_GL_SwapWindow）
    fn swap_buffers(&self) {
        if let Some(ref win) = self.window {
            win.gl_swap_window();
        }
    }

    // ==================== 关闭清理 ====================

    /// 关闭（对应 C++ SexyAppBase::Shutdown）
    pub fn shutdown(&mut self) {
        self.m_shutdown_flag = true;

        // 清理屏幕图像
        if let Some(si) = self.screen_image.take() {
            unsafe { let _ = Box::from_raw(si); }
        }

        // 清理 WidgetManager
        if let Some(wm) = self.widget_manager.take() {
            unsafe {
                let _ = Box::from_raw(wm);
            }
        }

        // 清理 GL 接口
        if let Some(gl) = self.gl_interface.take() {
            unsafe {
                let _ = Box::from_raw(gl);
            }
        }

        // 清理 GL 纹理
        if let Some(tex) = self.screen_gl_texture.take() {
            unsafe {
                crate::ffi::opengl::glDeleteTextures(1, &tex);
            }
        }

        // 清理窗口和 GL 上下文（上下文 Drop 自动清理，Window 先于 GLContext 被 Drop）
        self.gl_context = None;
        self.window = None;
        self.event_pump = None;
        self.sdl_context = None;

        // 对应 C++ Shutdown 末尾的 WriteDemoBuffer()（SexyAppBase.cpp:450）：
        // 录制模式下落盘 demo 文件（并做自动命名录制的保留清理）
        self.write_demo_buffer();
    }

    /// 初始化 GL 接口
    pub fn init_gl_interface(&mut self) -> i32 {
        if let Some(gl) = self.gl_interface {
            unsafe { (*gl).init(self.is_windowed) }
        } else {
            0
        }
    }

    /// 设置参数
    pub fn set_args(&mut self, _argc: i32, _argv: *mut *mut std::os::raw::c_char) {}

    /// 加载资源清单（对应 C++ LoadResourceManifest）
    pub fn load_resource_manifest(&mut self) {
        if let Some(rm) = self.resource_manager {
            unsafe {
                (*rm).parse_resources_file("properties/resources.xml");
            }
        }
    }

    /// 初始化音频系统（对应 C++ 中创建 SDLSoundManager + CreateMusicInterface）
    ///
    /// SDL2_mixer 通过 sdl2-sys 编译时链接（Windows 由 build.rs 提供 .lib，Linux 通过系统库），
    /// libopenmpt 仍然通过运行时 DLL 加载（外置依赖）。
    pub fn init_sound_system(&mut self) {
        use crate::ffi::sdl_mixer;
        use crate::ffi::libopenmpt;

        // 加载 libopenmpt.dll（运行时，编译时不需要，Windows 用户自备 DLL）
        if libopenmpt::load_library() {
            eprintln!("[libopenmpt] libopenmpt.dll 已加载（支持 MO3/IT/XM 格式）");
        } else {
            eprintln!("[libopenmpt] libopenmpt.dll 未找到，MO3/IT/XM 格式将用 SDL2_mixer 自身解码");
        }

        // SDL2_mixer 编译时静态链接，不再需要运行时加载 DLL
        if sdl_mixer::open_audio(44100, 0x8010u16, 2, 2048) == 0 {
            let n = sdl_mixer::allocate_channels(32);
            eprintln!("[音频] Mix_OpenAudio 成功，声道: {}/32", n);
        } else {
            eprintln!("[音频] Mix_OpenAudio 失败，无声运行");
        }

        if self.sound_manager.is_none() {
            use crate::framework::sound::sdl_sound_manager::SDLSoundManager;
            let sm = Box::new(SDLSoundManager::new());
            self.sound_manager = Some(Box::into_raw(sm) as *mut dyn crate::framework::sound::sound_manager::SoundManager);
        }
        if self.music_interface.is_none() {
            use crate::framework::sound::sdl_music_interface::SDLMusicInterface;
            let mi = Box::new(SDLMusicInterface::new());
            self.music_interface = Some(Box::into_raw(mi) as *mut dyn crate::framework::sound::music_interface::MusicInterface);
        }
        self.set_sfx_volume(self.sfx_volume);
        self.set_music_volume(self.music_volume);
    }

    /// 获取图像
    pub fn get_image(&self, _filename: &str, _commit_bits: bool) -> Option<&mut GLImage> {
        None
    }

    /// 设置光标
    pub fn set_cursor(&mut self, cursor_num: i32) {
        self.cursor_num = cursor_num;
    }

    /// 设置音量
    pub fn set_music_volume(&mut self, volume: f64) {
        self.music_volume = volume;
    }

    pub fn set_sfx_volume(&mut self, volume: f64) {
        self.sfx_volume = volume;
    }

    /// 对话框管理
    pub fn kill_dialog(&mut self, dialog_id: i32) -> bool {
        if let Some(d) = self.dialog_map.remove(&dialog_id) {
            // 对应 C++ KillDialog（SexyAppBase.cpp:857-859）：从 mDialogList 移除该对话框
            self.m_dialog_list.retain(|x| *x != d);
        }
        true
    }

    pub fn add_dialog(&mut self, dialog_id: i32, dialog: *mut Dialog) {
        self.dialog_map.insert(dialog_id, dialog);
        // 对应 C++ AddDialog（SexyAppBase.cpp:912）：mDialogList.push_back(theDialog)
        self.m_dialog_list.push(dialog);
    }

    /// 重绘
    pub fn redraw(&self, _clip_rect: Option<&Rect>) {}

    // ---- 输入事件处理（可被子类重写）----

    /// 按键按下（转发到 WidgetManager）
    pub fn key_down(&mut self, key: i32) {
        if let Some(wm) = self.widget_manager {
            unsafe { (*wm).key_down(key as KeyCode); }
        }
    }

    /// 按键释放（转发到 WidgetManager）
    pub fn key_up(&mut self, key: i32) {
        if let Some(wm) = self.widget_manager {
            unsafe { (*wm).key_up(key as KeyCode); }
        }
    }

    /// 鼠标按下（转发到 WidgetManager，btn 采用 C++ 编码：左=1/2, 右=-1/-2, 中=3）
    pub fn mouse_down(&mut self, x: i32, y: i32, btn: i32, _click_count: i32) {
        if let Some(wm) = self.widget_manager {
            unsafe {
                (*wm).mouse_move(x, y);
                (*wm).mouse_down(x, y, btn);
            }
        }
    }

    /// 鼠标释放（转发到 WidgetManager）
    pub fn mouse_up(&mut self, x: i32, y: i32, btn: i32) {
        if let Some(wm) = self.widget_manager {
            unsafe {
                (*wm).mouse_move(x, y);
                (*wm).mouse_up(x, y, btn);
            }
        }
    }

    /// 鼠标移动（转发到 WidgetManager）
    pub fn mouse_move(&mut self, x: i32, y: i32) {
        if let Some(wm) = self.widget_manager {
            unsafe { (*wm).mouse_move(x, y); }
        }
    }

    /// 鼠标滚轮（转发到 WidgetManager）
    pub fn mouse_wheel(&mut self, delta: i32) {
        if let Some(wm) = self.widget_manager {
            unsafe { (*wm).mouse_wheel(delta); }
        }
    }

    /// 更新应用单步（对应 C++ UpdateAppStep，供 Dialog::WaitForResult 使用）
    pub fn update_app_step(&mut self) -> bool {
        self.update_app()
    }

    /// 弹出消息框（对应 C++ Popup）
    pub fn popup(&self, msg: &str) {
        eprintln!("Popup: {}", msg);
    }

    /// 从文件读取 UTF-8 字符串（对应 C++ ReadUTF8StringFromFile）
    pub fn read_utf8_string_from_file(&self, path: &str) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }

    /// 复制到剪贴板（对应 C++ CopyToClipboard）
    pub fn copy_to_clipboard(&self, _text: &str) {}

    /// 获取剪贴板内容（对应 C++ GetClipboard）
    pub fn get_clipboard(&self) -> String { String::new() }

    /// 设置光标类型（对应 C++ SetCursor 的完整版本）
    pub fn set_cursor_type(&mut self, _cursor_type: i32) {}

    /// 开始文本输入（对应 C++ StartTextInput）
    pub fn start_text_input(&mut self, _initial_text: &str) -> bool { false }

    /// 停止文本输入（对应 C++ StopTextInput）
    pub fn stop_text_input(&mut self) {}

    /// 检查是否已关闭
    pub fn is_shutdown(&self) -> bool { self.m_shutdown_flag }

    // ==================== 命令行参数（对应 C++ SexyAppBase::DoParseCmdLine / HandleCmdLineParam） ====================

    /// 参数切分（对应 C++ `DoParseCmdLine()` 的遍历部分，SexyAppBase.cpp:3258-3278）。
    ///
    /// 支持 `-name=value` 与 `-name value` 两种形式；后者仅在本参数在
    /// `ParamTakesValue` 列表中且下一项不以 '-' 开头时生效。
    /// C++ 侧遍历后直接调虚函数 `HandleCmdLineParam`，Rust 无虚分派，
    /// 故返回切分结果由调用方分派（LawnApp 覆写优先于基类）。
    pub fn parse_cmd_line_params(args: &[String]) -> Vec<(String, String)> {
        let mut a_result: Vec<(String, String)> = Vec::new();
        let a_argc = args.len() as i32;

        let mut i = 1i32;
        while i < a_argc {
            let mut a_param = args[i as usize].clone();
            let mut a_value = String::new();

            if let Some(an_equals_pos) = a_param.find('=') {
                a_value = a_param[an_equals_pos + 1..].to_string();
                a_param = a_param[..an_equals_pos].to_string();
            } else if i + 1 < a_argc
                && !args[(i + 1) as usize].starts_with('-')
                && param_takes_value(&a_param)
            {
                i += 1;
                a_value = args[i as usize].clone();
            }

            a_result.push((a_param, a_value));
            i += 1;
        }

        a_result
    }

    /// 参数解析收尾（对应 C++ `DoParseCmdLine()` 尾部，SexyAppBase.cpp:3280-3290）：
    /// 所有参数解析完后再确定 demo 文件，使显式指定的文件无论顺序都优先。
    pub fn finalize_cmd_line(&mut self) {
        if self.m_playing_demo_buffer && !self.m_has_custom_demo_file {
            let a_demo_files = find_demo_files(&self.demo_prefix, false);
            if a_demo_files.is_empty() {
                self.popup("No demo recordings found");
                std::process::exit(1);
            }
            let a_idx = self.m_demo_play_index.min(a_demo_files.len() - 1);
            self.demo_file_name = a_demo_files[a_idx].clone();
        } else if self.m_recording_demo_buffer && !self.m_has_custom_demo_file {
            self.demo_file_name = get_timestamped_demo_file_name(&self.demo_prefix);
        }
    }

    /// 命令行参数处理（对应 C++ `SexyAppBase::HandleCmdLineParam`，SexyAppBase.cpp:3307-3350）
    /// 处理 `-version` / `-license` / `-play` / `-playnum` / `-record` / `-recnum`
    pub fn handle_cmd_line_param(&mut self, param_name: &str, param_value: &str) {
        match param_name {
            "-version" => {
                // 打印版本信息后退出
                let version_string = format!(
                    "Product: {}\nVersion: {}\nBuild Num: {}\nBuild Date: {}\nLicense: LGPL-3.0-or-later",
                    self.prod_name, self.product_version, self.build_num, self.build_date
                );
                eprintln!("{}", version_string);
                std::process::exit(0);
            }
            "-license" | "-copyright" => {
                eprintln!("PvZ-Portable - LGPL-3.0-or-later");
                std::process::exit(0);
            }

            "-play" | "-playnum" => {
                // 每次出现都完整重定义请求：以最后一次为准
                self.m_has_custom_demo_file = false;
                self.m_demo_play_index = 0;
                if param_name == "-play" && !param_value.is_empty() {
                    self.demo_file_name = param_value.to_string();
                    self.m_has_custom_demo_file = true;
                } else if param_name == "-playnum" {
                    let a_num: i32 = param_value.trim().parse().unwrap_or(0);
                    self.m_demo_play_index = (a_num.max(1) - 1) as usize;
                }
                self.m_playing_demo_buffer = true;
                self.m_recording_demo_buffer = false;
            }

            "-record" | "-recnum" => {
                if param_name == "-recnum" {
                    // 按时间戳/名称序只保留前 N 个录制
                    let mut a_num: i32 = param_value.trim().parse().unwrap_or(0);
                    if a_num <= 0 {
                        a_num = 5;
                    }
                    self.m_demo_record_file_limit = a_num as u32;
                } else {
                    self.m_has_custom_demo_file = false;
                    if !param_value.is_empty() {
                        self.demo_file_name = param_value.to_string();
                        self.m_has_custom_demo_file = true;
                    }
                }
                self.m_recording_demo_buffer = true;
                self.m_playing_demo_buffer = false;
            }

            _ => {
                // 其他参数传递给调用方/子类处理
            }
        }
    }

    /// 获取游戏 SEH 信息（用于诊断，对应 C++ SexyApp::GetGameSEHInfo）
    pub fn get_game_seh_info(&self) -> String {
        format!(
            "Build Num: {}\r\nBuild Date: {}\r\n",
            self.build_num, self.build_date
        )
    }

    /// 显示前钩子（对应 C++ SexyApp::PreDisplayHook）
    pub fn pre_display_hook(&mut self) {}

    /// 属性初始化钩子（对应 C++ SexyApp::InitPropertiesHook）
    /// 加载 properties/partner.xml 配置
    pub fn init_properties_hook(&mut self) {
        // 在 Rust 中，属性通过 ResourceManager 加载
        // 这里简化为从配置中读取标题
        eprintln!("[SexyApp] 初始化属性配置");
        // 实际实现需加载 XML 配置
    }

    // ---- 属性读取（对应 C++ SexyAppBase::GetBoolean/GetInteger/GetDouble/GetString，SexyAppBase.cpp:3105） ----

    pub fn get_boolean(&self, the_id: &str) -> bool {
        self.m_bool_properties.get(the_id).copied().unwrap_or(false)
    }

    pub fn get_boolean_default(&self, the_id: &str, the_default: bool) -> bool {
        self.m_bool_properties.get(the_id).copied().unwrap_or(the_default)
    }

    pub fn get_integer(&self, the_id: &str) -> i32 {
        self.m_int_properties.get(the_id).copied().unwrap_or(0)
    }

    pub fn get_integer_default(&self, the_id: &str, the_default: i32) -> i32 {
        self.m_int_properties.get(the_id).copied().unwrap_or(the_default)
    }

    pub fn get_double(&self, the_id: &str) -> f64 {
        self.m_double_properties.get(the_id).copied().unwrap_or(0.0)
    }

    pub fn get_double_default(&self, the_id: &str, the_default: f64) -> f64 {
        self.m_double_properties.get(the_id).copied().unwrap_or(the_default)
    }

    pub fn get_string(&self, the_id: &str) -> String {
        self.m_string_properties.get(the_id).cloned().unwrap_or_default()
    }

    pub fn get_string_default(&self, the_id: &str, the_default: &str) -> String {
        self.m_string_properties.get(the_id).cloned().unwrap_or_else(|| the_default.to_string())
    }

    pub fn get_string_vector(&self, the_id: &str) -> Vec<String> {
        self.m_string_vector_properties.get(the_id).cloned().unwrap_or_default()
    }

    // ---- 属性写入（对应 C++ SetBoolean/SetInteger/SetDouble/SetString，SexyAppBase.cpp:3208） ----

    pub fn set_boolean(&mut self, the_id: &str, the_value: bool) {
        self.m_bool_properties.insert(the_id.to_string(), the_value);
    }

    pub fn set_integer(&mut self, the_id: &str, the_value: i32) {
        self.m_int_properties.insert(the_id.to_string(), the_value);
    }

    pub fn set_double(&mut self, the_id: &str, the_value: f64) {
        self.m_double_properties.insert(the_id.to_string(), the_value);
    }

    pub fn set_string(&mut self, the_id: &str, the_value: &str) {
        self.m_string_properties.insert(the_id.to_string(), the_value.to_string());
    }

    /// 设置字符串数组属性（对应 C++ mStringVectorProperties 写入）
    pub fn set_string_vector(&mut self, the_id: &str, the_value: Vec<String>) {
        self.m_string_vector_properties.insert(the_id.to_string(), the_value);
    }

    /// 终止前钩子（对应 C++ SexyApp::PreTerminate）
    pub fn pre_terminate(&mut self) {}

    /// 更新帧（对应 C++ SexyApp::UpdateFrames）
    /// 在 SexyApp 层只是调用基类方法，Rust 中直接保留空实现
    pub fn update_frames(&mut self) {
        // C++ 中：SexyAppBase::UpdateFrames();
        // 在合并后的 Rust 结构中，此方法直接调用 base 的对应功能
        // 当前的 update 循环由外部引擎驱动
    }

    // ==================== Demo 录制/回放 ====================
    // 对应 C++ SexyAppBase.cpp:472-800（文件格式与同步辅助）/ 2060-2280（时序与回放）/ 4193（刷新率）

    /// 对应 C++ `SexyAppBase::IsInDemoMode()`（SexyAppBase.h:399）
    pub fn is_in_demo_mode(&self) -> bool {
        self.m_recording_demo_buffer || self.m_playing_demo_buffer
    }

    /// 对应 C++ `SexyAppBase::GetNowTime()`（SexyAppBase.h:402-407）
    ///
    /// demo 会话期间用「会话起始墙上时钟 + update tick / 100」派生，保证回放时间可复现。
    pub fn demo_now_time(&self) -> i64 {
        if self.is_in_demo_mode() {
            return self.m_demo_start_time as i64 + (self.m_update_count as i64) / 100;
        }

        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }

    /// 对应 C++ `SexyAppBase::WriteDemoTimingBlock()`（SexyAppBase.cpp:2060-2076）
    ///
    /// 把距上次记录的 update tick 增量写进 demo 流；超过 15 时拆成多个 15 并以 DEMO_IDLE 填充。
    pub fn write_demo_timing_block(&mut self) {
        // C++: DBG_ASSERTE(IsOnPrimaryThread());
        while self.m_update_count - self.m_last_demo_update_cnt > 15 {
            self.m_demo_buffer.write_num_bits(15, 4);
            self.m_last_demo_update_cnt += 15;

            self.m_demo_buffer.write_num_bits(0, 1);
            self.m_demo_buffer.write_num_bits(DEMO_IDLE, 5);
        }

        self.m_demo_buffer
            .write_num_bits(self.m_update_count - self.m_last_demo_update_cnt, 4);
        self.m_last_demo_update_cnt = self.m_update_count;
    }

    /// 对应 C++ `SexyAppBase::PrepareDemoCommand()`（SexyAppBase.cpp:2083-2107）
    ///
    /// 读取一条 demo 命令头（4 bit 时间增量 + 1 bit 短命令标志 + 1/5 bit 命令号）。
    pub fn prepare_demo_command(&mut self, required: bool) -> bool {
        if self.m_demo_needs_command {
            self.m_demo_cmd_bit_pos = self.m_demo_buffer.read_bit_pos;
            self.m_demo_cmd_update_cnt = self.m_last_demo_update_cnt;
            if required {
                // 游戏逻辑的调用点认领了排队的命令
                self.m_demo_command_queued = false;
            }

            self.m_last_demo_update_cnt += self.m_demo_buffer.read_num_bits(4, false);

            self.m_demo_is_short_cmd = self.m_demo_buffer.read_num_bits(1, false) == 1;

            if self.m_demo_is_short_cmd {
                self.m_demo_cmd_num = self.m_demo_buffer.read_num_bits(1, false);
            } else {
                self.m_demo_cmd_num = self.m_demo_buffer.read_num_bits(5, false);
            }

            self.m_demo_needs_command = false;
        }

        // C++: DBG_ASSERTE((mUpdateCount >= mLastDemoUpdateCnt) || (!required));
        self.m_update_count >= self.m_last_demo_update_cnt
    }

    /// 对应 C++ `SexyAppBase::DemoSyncBuffer()`（SexyAppBase.cpp:700-727）
    pub fn demo_sync_buffer(&mut self, the_buffer: &mut Buffer) {
        if self.m_playing_demo_buffer {
            if self.m_manual_shutdown {
                return;
            }

            self.prepare_demo_command(true);
            self.m_demo_needs_command = true;

            // C++: DBG_ASSERTE(!mDemoIsShortCmd); DBG_ASSERTE(mDemoCmdNum == DEMO_SYNC);
            let a_len = self.m_demo_buffer.read_u32();

            the_buffer.clear();
            for _ in 0..(a_len as i32) {
                let a_byte = self.m_demo_buffer.read_byte();
                the_buffer.write_byte(a_byte);
            }
        } else if self.m_recording_demo_buffer {
            self.write_demo_timing_block();
            self.m_demo_buffer.write_num_bits(0, 1);
            self.m_demo_buffer.write_num_bits(DEMO_SYNC, 5);
            self.m_demo_buffer
                .write_u32(the_buffer.get_data_len() as u32);

            let a_len = the_buffer.get_data_len() as usize;
            let a_data = the_buffer.data()[..a_len].to_vec();
            self.m_demo_buffer.write_bytes(&a_data);
        }
    }

    /// 对应 C++ `SexyAppBase::DemoSyncString()`（SexyAppBase.cpp:729-735）
    pub fn demo_sync_string(&mut self, the_string: &mut String) {
        let mut a_buffer = Buffer::new();
        a_buffer.write_string(the_string);
        self.demo_sync_buffer(&mut a_buffer);
        *the_string = a_buffer.read_string();
    }

    /// 对应 C++ `SexyAppBase::DemoSyncInt()`（SexyAppBase.cpp:737-743）
    pub fn demo_sync_int(&mut self, the_int: &mut i32) {
        let mut a_buffer = Buffer::new();
        a_buffer.write_i32(*the_int);
        self.demo_sync_buffer(&mut a_buffer);
        *the_int = a_buffer.read_i32();
    }

    /// 对应 C++ `SexyAppBase::DemoSyncBool()`（SexyAppBase.cpp:745-751）
    pub fn demo_sync_bool(&mut self, the_bool: &mut bool) {
        let mut a_buffer = Buffer::new();
        a_buffer.write_boolean(*the_bool);
        self.demo_sync_buffer(&mut a_buffer);
        *the_bool = a_buffer.read_boolean();
    }

    /// 对应 C++ `SexyAppBase::DemoAssertStringEqual()`（SexyAppBase.cpp:753-776）
    pub fn demo_assert_string_equal(&mut self, the_string: &str) {
        if self.m_playing_demo_buffer {
            if self.m_manual_shutdown {
                return;
            }

            self.prepare_demo_command(true);
            self.m_demo_needs_command = true;

            // C++: DBG_ASSERTE(!mDemoIsShortCmd); DBG_ASSERTE(mDemoCmdNum == DEMO_ASSERT_STRING_EQUAL);
            let a_string = self.m_demo_buffer.read_string();
            // C++: DBG_ASSERTE(aString == theString); —— Release 下仅记录，不中断
            let _ = a_string == the_string;
        } else if self.m_recording_demo_buffer {
            self.write_demo_timing_block();
            self.m_demo_buffer.write_num_bits(0, 1);
            self.m_demo_buffer.write_num_bits(DEMO_ASSERT_STRING_EQUAL, 5);
            self.m_demo_buffer.write_string(the_string);
        }
    }

    /// 对应 C++ `SexyAppBase::DemoAssertIntEqual()`（SexyAppBase.cpp:790-814）
    pub fn demo_assert_int_equal(&mut self, the_int: i32) {
        if self.m_playing_demo_buffer {
            if self.m_manual_shutdown {
                return;
            }

            self.prepare_demo_command(true);
            self.m_demo_needs_command = true;

            // C++: DBG_ASSERTE(!mDemoIsShortCmd); DBG_ASSERTE(mDemoCmdNum == DEMO_ASSERT_INT_EQUAL);
            let an_int = self.m_demo_buffer.read_i32();
            // C++: (void)anInt; DBG_ASSERTE(anInt == theInt); —— Release 下未使用
            let _ = an_int == the_int;
        } else if self.m_recording_demo_buffer {
            self.write_demo_timing_block();
            self.m_demo_buffer.write_num_bits(0, 1);
            self.m_demo_buffer.write_num_bits(DEMO_ASSERT_INT_EQUAL, 5);
            self.m_demo_buffer.write_i32(the_int);
        }
    }

    /// 对应 C++ `SexyAppBase::DemoAddMarker()`（SexyAppBase.cpp:778-788）
    pub fn demo_add_marker(&mut self, the_string: &str) {
        if self.m_playing_demo_buffer {
            self.m_fast_forward_to_marker = false;
        } else if self.m_recording_demo_buffer {
            self.m_demo_marker_list
                .push((the_string.to_string(), self.m_update_count as u32));
        }
    }

    /// 对应 C++ `SexyAppBase::DemoSyncRefreshRate()`（SexyAppBase.cpp:4193-4206）
    pub fn demo_sync_refresh_rate(&mut self) {
        // C++: mSyncRefreshRate = mGLInterface->mRefreshRate;
        // [TRANSLATION_NOTE]: Rust 侧 GLInterface 未接入 mRefreshRate，此处保持 m_sync_refresh_rate 现值
        if self.m_recording_demo_buffer {
            self.write_demo_timing_block();
            self.m_demo_buffer.write_num_bits(0, 1);
            self.m_demo_buffer.write_num_bits(DEMO_VIDEO_DATA, 5);
            self.m_demo_buffer.write_boolean(self.is_windowed);
            let a_byte = self.m_sync_refresh_rate;
            self.m_demo_buffer.write_byte(a_byte);
        }
    }

    /// 对应 C++ `RecordDemoMousePosition()`（Input.cpp:338-369）
    ///
    /// 流中坐标是无符号 12 bit；位移小于 32 时走短命令（1 bit 短标志 + 1 bit 命令号 0 + 6 bit 有符号增量 ×2）。
    fn record_demo_mouse_position(&mut self, the_x: i32, the_y: i32) {
        let a_x = the_x & 4095;
        let a_y = the_y & 4095;

        let a_diff_x = a_x - self.m_last_demo_mouse_x;
        let a_diff_y = a_y - self.m_last_demo_mouse_y;

        if a_diff_x.abs() < 32 && a_diff_y.abs() < 32 {
            if a_diff_x != 0 || a_diff_y != 0 {
                self.write_demo_timing_block();
                self.m_demo_buffer.write_num_bits(1, 1);
                self.m_demo_buffer.write_num_bits(0, 1);
                self.m_demo_buffer.write_num_bits(a_diff_x, 6);
                self.m_demo_buffer.write_num_bits(a_diff_y, 6);
            }
        } else {
            self.write_demo_timing_block();
            self.m_demo_buffer.write_num_bits(0, 1);
            self.m_demo_buffer.write_num_bits(DEMO_MOUSE_POSITION, 5);
            self.m_demo_buffer.write_num_bits(a_x, 12);
            self.m_demo_buffer.write_num_bits(a_y, 12);
        }

        self.m_last_demo_mouse_x = a_x;
        self.m_last_demo_mouse_y = a_y;
    }

    /// 鼠标坐标重映射（对应 C++ `mWidgetManager->RemapMouse(x, y)`）
    fn remap_demo_mouse_point(&self, x: i32, y: i32) -> (i32, i32) {
        if let Some(wm) = self.widget_manager {
            unsafe { (*wm).remap_mouse(x, y) }
        } else {
            (x, y)
        }
    }

    /// 鼠标尚未进入窗口时补一条 `DEMO_MOUSE_ENTER`（对应 C++ `if (!mMouseIn) {...}`）
    fn record_demo_mouse_enter_if_needed(&mut self) {
        if !self.m_mouse_in {
            self.write_demo_timing_block();
            self.m_demo_buffer.write_num_bits(0, 1);
            self.m_demo_buffer.write_num_bits(DEMO_MOUSE_ENTER, 5);
        }
    }

    /// 对应 C++ `RecordDemoEvent()`（Input.cpp:372-490）：把输入事件写进 demo 流，
    /// 格式与 `ProcessDemo` 读取的一致。
    fn record_demo_event(&mut self, the_event: &Event) {
        match the_event {
            Event::Window { win_event, .. } => match win_event {
                WindowEvent::Minimized | WindowEvent::Restored => {
                    self.write_demo_timing_block();
                    self.m_demo_buffer.write_num_bits(0, 1);
                    self.m_demo_buffer.write_num_bits(DEMO_SIZE, 5);
                    self.m_demo_buffer
                        .write_boolean(matches!(win_event, WindowEvent::Minimized));
                }
                WindowEvent::FocusGained | WindowEvent::FocusLost => {
                    self.write_demo_timing_block();
                    self.m_demo_buffer.write_num_bits(0, 1);
                    self.m_demo_buffer.write_num_bits(DEMO_ACTIVATE_APP, 5);
                    self.m_demo_buffer.write_num_bits(
                        if matches!(win_event, WindowEvent::FocusGained) { 1 } else { 0 },
                        1,
                    );
                }
                _ => {}
            },

            Event::MouseMotion { x, y, .. } => {
                let (a_x, a_y) = self.remap_demo_mouse_point(*x, *y);
                self.record_demo_mouse_position(a_x, a_y);
                self.record_demo_mouse_enter_if_needed();
            }

            Event::MouseButtonDown { mouse_btn, x, y, clicks, .. } => {
                let (a_x, a_y) = self.remap_demo_mouse_point(*x, *y);
                self.record_demo_mouse_position(a_x, a_y);

                let mut a_btn_num = match mouse_btn {
                    MouseButton::Left => 1,
                    MouseButton::Right => -1,
                    _ => 3,
                };
                if *clicks == 2 {
                    a_btn_num = match mouse_btn {
                        MouseButton::Left => 2,
                        MouseButton::Right => -2,
                        _ => a_btn_num,
                    };
                }

                self.write_demo_timing_block();
                self.m_demo_buffer.write_num_bits(1, 1);
                self.m_demo_buffer.write_num_bits(1, 1);
                self.m_demo_buffer.write_num_bits(1, 1);
                self.m_demo_buffer.write_num_bits(a_btn_num, 3);

                self.record_demo_mouse_enter_if_needed();
            }

            Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                let (a_x, a_y) = self.remap_demo_mouse_point(*x, *y);
                self.record_demo_mouse_position(a_x, a_y);

                let a_btn_num = match mouse_btn {
                    MouseButton::Left => 1,
                    MouseButton::Right => -1,
                    _ => 3,
                };

                self.write_demo_timing_block();
                self.m_demo_buffer.write_num_bits(1, 1);
                self.m_demo_buffer.write_num_bits(1, 1);
                self.m_demo_buffer.write_num_bits(0, 1);
                self.m_demo_buffer.write_num_bits(a_btn_num, 3);

                self.record_demo_mouse_enter_if_needed();
            }

            Event::MouseWheel { y, .. } => {
                self.write_demo_timing_block();
                self.m_demo_buffer.write_num_bits(0, 1);
                self.m_demo_buffer.write_num_bits(DEMO_MOUSE_WHEEL, 5);
                self.m_demo_buffer.write_num_bits((*y).clamp(-128, 127), 8);
            }

            Event::KeyDown { keycode, keymod, repeat, .. } => {
                let Some(kc) = keycode else { return };

                let a_key_sym: i32 = (*kc).into();

                // SDLK_RETURN=13；SDLK_KP_ENTER 为扫描码键 88 | (1<<30)
                const SDLK_RETURN: i32 = 13;
                const SDLK_KP_ENTER: i32 = 0x4000_0000 | 88;

                if self.m_allow_alt_enter
                    && !*repeat
                    && (a_key_sym == SDLK_RETURN || a_key_sym == SDLK_KP_ENTER)
                    && keymod.intersects(sdl2::keyboard::Mod::LALTMOD | sdl2::keyboard::Mod::RALTMOD)
                {
                    // 屏幕模式切换，不发给 widget manager，也不记录
                    return;
                }

                self.write_demo_timing_block();
                self.m_demo_buffer.write_num_bits(0, 1);
                self.m_demo_buffer.write_num_bits(DEMO_KEY_DOWN, 5);
                self.m_demo_buffer
                    .write_num_bits(sdl_keycode_to_keycode(a_key_sym), 8);

                // 对应 C++ `SDL_IsTextInputActive()`（返回 SDL_bool 枚举，非整数）
                let a_text_input_active =
                    unsafe { sdl2::sys::SDL_IsTextInputActive() as i32 != 0 };
                if let Some(a_char) =
                    sdl_synthesize_ascii_char_from_key_down(a_key_sym, *keymod, a_text_input_active)
                {
                    self.write_demo_timing_block();
                    self.m_demo_buffer.write_num_bits(0, 1);
                    self.m_demo_buffer.write_num_bits(DEMO_KEY_CHAR, 5);
                    self.m_demo_buffer.write_num_bits(0, 1);
                    self.m_demo_buffer.write_num_bits(a_char as i32, 8);
                }
            }

            Event::KeyUp { keycode, .. } => {
                let Some(kc) = keycode else { return };
                let a_key_sym: i32 = (*kc).into();

                self.write_demo_timing_block();
                self.m_demo_buffer.write_num_bits(0, 1);
                self.m_demo_buffer.write_num_bits(DEMO_KEY_UP, 5);
                self.m_demo_buffer
                    .write_num_bits(sdl_keycode_to_keycode(a_key_sym), 8);
            }

            // C++: if (theEvent.text.text[0] != 0) —— 经由 KeyText 派发，故录制整个 UTF-8 串
            Event::TextInput { text, .. } => {
                if !text.is_empty() {
                    self.write_demo_timing_block();
                    self.m_demo_buffer.write_num_bits(0, 1);
                    self.m_demo_buffer.write_num_bits(DEMO_KEY_TEXT, 5);
                    self.m_demo_buffer.write_string(text);
                }
            }

            _ => {}
        }
    }

    /// 对应 C++ `SexyAppBase::ReadDemoBuffer()`（SexyAppBase.cpp:472-596）
    pub fn read_demo_buffer(&mut self) -> Result<(), String> {
        let a_file = match std::fs::read(&self.demo_file_name) {
            Ok(b) => b,
            Err(_) => {
                return Err(format!("Demo file not found: {}", self.demo_file_name));
            }
        };

        let mut pos = 0usize;
        // C++ 用 std::ifstream 顺序读取，任何一段读不满都直接 return false（此即 Err）
        macro_rules! take {
            ($n:expr) => {{
                let n: usize = $n;
                if pos + n > a_file.len() {
                    return Err(String::from("Invalid demo file."));
                }
                let s = &a_file[pos..pos + n];
                pos += n;
                s
            }};
        }

        let a_file_id = u32::from_le_bytes(take!(4).try_into().unwrap());
        // C++: DBG_ASSERTE(aFileID == DEMO_FILE_ID); if (aFileID != DEMO_FILE_ID) return false;
        if a_file_id != DEMO_FILE_ID {
            return Err(String::from("Invalid demo file."));
        }

        let a_version = u32::from_le_bytes(take!(4).try_into().unwrap());
        if a_version != DEMO_VERSION {
            return Err(String::from("Incompatible demo file version."));
        }

        self.rand_seed = u32::from_le_bytes(take!(4).try_into().unwrap());
        common::srand(self.rand_seed);

        self.m_demo_start_time = u64::from_le_bytes(take!(8).try_into().unwrap());

        let a_time_zone_offset = u32::from_le_bytes(take!(4).try_into().unwrap());
        self.m_demo_time_zone_offset = a_time_zone_offset as i32;

        // 记录的程序版本（旧文件为空）；未知或不匹配仅告警
        let a_str_len = u16::from_le_bytes(take!(2).try_into().unwrap()) as usize;
        let a_recorded_version =
            String::from_utf8_lossy(take!(a_str_len)).to_string();
        if a_recorded_version.is_empty() {
            eprintln!("Demo has no program version tag; replay may diverge.");
        } else if self.product_version != a_recorded_version {
            eprintln!(
                "Demo was recorded with a different program version (recorded: {}, current: {}); replay may diverge.",
                a_recorded_version, self.product_version
            );
        }

        let mut a_bytes_left = a_file.len() as i32 - pos as i32;

        // 读取 marker 列表（v2 起）
        if a_version >= 2 {
            let a_size_raw = i32::from_le_bytes(take!(4).try_into().unwrap());
            a_bytes_left -= 4;

            if a_size_raw < 0 || a_size_raw >= a_bytes_left {
                return Err(String::from("Invalid demo file."));
            }

            let a_size = a_size_raw as usize;
            let a_marker_bytes = take!(a_size).to_vec();

            let mut a_marker_buffer = Buffer::from_bytes(&a_marker_bytes);
            a_marker_buffer.seek_front();

            let a_num_items = a_marker_buffer.read_u32();
            let mut i: u32 = 0;
            while i < a_num_items && !a_marker_buffer.at_end() {
                let a_first = a_marker_buffer.read_string();
                let a_second = a_marker_buffer.read_u32();
                self.m_demo_marker_list.push((a_first, a_second));
                i += 1;
            }

            if i != a_num_items {
                return Err(String::from("Invalid demo file."));
            }

            a_bytes_left -= a_size as i32;
        }

        // 读取 demo 命令流
        // C++: 这个长度回放不使用，只用于保持流对齐
        let _a_demo_length = u32::from_le_bytes(take!(4).try_into().unwrap());
        a_bytes_left -= 4;

        if a_bytes_left <= 0 {
            return Err(String::from("Invalid demo file."));
        }

        let a_commands = take!(a_bytes_left as usize).to_vec();
        self.m_demo_buffer.write_bytes(&a_commands);
        self.m_demo_buffer.seek_front();

        Ok(())
    }

    /// 对应 C++ `SexyAppBase::WriteDemoBuffer()`（SexyAppBase.cpp:641-698）
    pub fn write_demo_buffer(&mut self) {
        if !self.m_recording_demo_buffer {
            return;
        }

        let mut out: Vec<u8> = Vec::new();

        // Demo 文件为小端格式（C++ ToLE* 在小端机上为 no-op）
        out.extend_from_slice(&DEMO_FILE_ID.to_le_bytes());
        out.extend_from_slice(&DEMO_VERSION.to_le_bytes());
        out.extend_from_slice(&self.rand_seed.to_le_bytes());
        out.extend_from_slice(&self.m_demo_start_time.to_le_bytes());
        out.extend_from_slice(&(self.m_demo_time_zone_offset as u32).to_le_bytes());

        let a_version_bytes = self.product_version.as_bytes();
        out.extend_from_slice(&(a_version_bytes.len() as u16).to_le_bytes());
        out.extend_from_slice(a_version_bytes);

        let mut a_marker_buffer = Buffer::new();
        a_marker_buffer.write_u32(self.m_demo_marker_list.len() as u32);
        for (a_name, a_cnt) in self.m_demo_marker_list.iter() {
            a_marker_buffer.write_string(a_name);
            a_marker_buffer.write_u32(*a_cnt);
        }
        let a_marker_size = a_marker_buffer.get_data_len();
        out.extend_from_slice(&(a_marker_size as u32).to_le_bytes());
        out.extend_from_slice(&a_marker_buffer.data()[..a_marker_size as usize]);

        out.extend_from_slice(&(self.m_update_count as u32).to_le_bytes());

        let a_demo_len = self.m_demo_buffer.get_data_len() as usize;
        out.extend_from_slice(&self.m_demo_buffer.data()[..a_demo_len]);

        let a_written = std::fs::write(&self.demo_file_name, &out).is_ok();

        // 仅清理符合自动命名模式的录制；显式指定的目标文件不触发保留策略
        if a_written
            && self.m_demo_record_file_limit != 0
            && !self.m_has_custom_demo_file
            && is_stamped_demo_file_name(&self.demo_prefix, &self.demo_file_name)
        {
            let a_demo_files = find_demo_files(&self.demo_prefix, true);
            for i in (self.m_demo_record_file_limit as usize)..a_demo_files.len() {
                let _ = std::fs::remove_file(&a_demo_files[i]);
            }
        }
    }

    // ==================== Demo 回放依赖的辅助方法 ====================

    /// 对应 C++ `SexyAppBase::IsMuted()`（SexyAppBase.cpp:4088-4091）
    pub fn is_muted(&self) -> bool {
        self.m_mute_count > 0
    }

    /// 对应 C++ `SexyAppBase::Mute()`（SexyAppBase.cpp:4092-4100）
    pub fn mute(&mut self, auto_mute: bool) {
        self.m_mute_count += 1;
        if auto_mute {
            self.m_auto_mute_count += 1;
        }

        self.set_music_volume(self.music_volume);
        self.set_sfx_volume(self.sfx_volume);
    }

    /// 对应 C++ `SexyAppBase::Unmute()`（SexyAppBase.cpp:4102-4114）
    pub fn unmute(&mut self, auto_mute: bool) {
        if self.m_mute_count > 0 {
            self.m_mute_count -= 1;
            if auto_mute {
                self.m_auto_mute_count -= 1;
            }
        }

        self.set_music_volume(self.music_volume);
        self.set_sfx_volume(self.sfx_volume);
    }

    /// 对应 C++ `SexyAppBase::URLOpenSucceeded()`（SexyAppBase.cpp:965-973）
    pub fn url_open_succeeded(&mut self, the_url: &str) {
        let _ = the_url;
        self.m_is_opening_url = false;

        if self.m_shutdown_on_url_open {
            self.shutdown();
        }
    }

    /// 对应 C++ `SexyAppBase::RehupFocus()`（SexyAppBase.cpp:2020-2047）
    pub fn rehup_focus(&mut self) {
        let a_want_has_focus = self.active && !self.minimized;

        if a_want_has_focus != self.has_focus {
            self.has_focus = a_want_has_focus;

            if self.has_focus {
                if self.m_mute_on_lost_focus {
                    self.unmute(true);
                }

                if let Some(wm) = self.widget_manager {
                    unsafe { (*wm).got_focus(); }
                }
                // C++: GotFocus(); —— SexyAppBase 的虚函数，Rust 侧无覆写者
            } else {
                if self.m_mute_on_lost_focus {
                    self.mute(true);
                }

                if let Some(wm) = self.widget_manager {
                    unsafe {
                        (*wm).lost_focus();
                        (*wm).do_mouse_ups();
                    }
                }
                // C++: LostFocus(); —— SexyAppBase 的虚函数，Rust 侧无覆写者
            }
        }
    }

    /// 对应 C++ `SexyAppBase::EnforceCursor()`（SexyAppBase.cpp:2484-2530）
    pub fn enforce_cursor(&mut self) {
        // C++: int aCursorNum = mSEHOccured ? CURSOR_POINTER : mCursorNum;
        let mut a_cursor_num = self.cursor_num;
        if self.m_seh_occurred {
            a_cursor_num = 0; // CURSOR_POINTER
        }
        if a_cursor_num < 0 {
            a_cursor_num = 0;
        }

        // [TRANSLATION_NOTE]: C++ 此处按 cursorNum 设置 SDL_Cursor / mCursorImages；
        // Rust 侧未接入光标图像系统，仅保留游标号归一化
        self.cursor_num = a_cursor_num;
    }

    /// 对应 C++ `SexyAppBase::ProcessDemo()`（SexyAppBase.cpp:2109-2280）
    ///
    /// 回放 demo 命令流：把流中的鼠标/键盘/窗口命令重放到 `WidgetManager`。
    pub fn process_demo(&mut self) {
        if !self.m_playing_demo_buffer {
            return;
        }

        // C++: 没有以 DEMO_CLOSE 结束的录制，把流结束视作 demo 结束
        if self.m_demo_buffer.at_end() {
            self.shutdown();
            return;
        }

        while !self.m_shutdown_flag
            && self.m_update_count >= self.m_last_demo_update_cnt
            && !self.m_demo_buffer.at_end()
        {
            if self.prepare_demo_command(false) {
                self.m_demo_needs_command = true;

                if self.m_demo_is_short_cmd {
                    match self.m_demo_cmd_num {
                        // 短命令 0：相对鼠标移动（6 bit 有符号增量 ×2）
                        0 => {
                            let a_delta_x = self.m_demo_buffer.read_num_bits(6, true);
                            let a_delta_y = self.m_demo_buffer.read_num_bits(6, true);
                            self.m_last_demo_mouse_x += a_delta_x;
                            self.m_last_demo_mouse_y += a_delta_y;

                            if let Some(wm) = self.widget_manager {
                                unsafe {
                                    (*wm).mouse_move(self.m_last_demo_mouse_x, self.m_last_demo_mouse_y);
                                }
                            }
                        }
                        // 短命令 1：鼠标按下/抬起（1 bit down + 3 bit 有符号按钮号）
                        1 => {
                            let down = self.m_demo_buffer.read_num_bits(1, false) != 0;
                            let a_btn_count = self.m_demo_buffer.read_num_bits(3, true);

                            if let Some(wm) = self.widget_manager {
                                unsafe {
                                    if down {
                                        (*wm).mouse_down(self.m_last_demo_mouse_x, self.m_last_demo_mouse_y, a_btn_count);
                                    } else {
                                        (*wm).mouse_up(self.m_last_demo_mouse_x, self.m_last_demo_mouse_y, a_btn_count);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                } else {
                    match self.m_demo_cmd_num {
                        DEMO_MOUSE_POSITION => {
                            self.m_last_demo_mouse_x = self.m_demo_buffer.read_num_bits(12, false);
                            self.m_last_demo_mouse_y = self.m_demo_buffer.read_num_bits(12, false);

                            if let Some(wm) = self.widget_manager {
                                unsafe {
                                    (*wm).mouse_move(self.m_last_demo_mouse_x, self.m_last_demo_mouse_y);
                                }
                            }
                        }
                        DEMO_ACTIVATE_APP => {
                            self.active = self.m_demo_buffer.read_num_bits(1, false) != 0;

                            self.rehup_focus();

                            if self.active && !self.is_windowed {
                                if let Some(wm) = self.widget_manager {
                                    unsafe { (*wm).mark_all_dirty(); }
                                }
                            }

                            if self.m_is_opening_url && !self.active {
                                let a_url = self.m_opening_url.clone();
                                self.url_open_succeeded(&a_url);
                            }
                        }
                        DEMO_SIZE => {
                            let is_minimized = self.m_demo_buffer.read_boolean();

                            if !self.m_shutdown_flag && is_minimized != self.minimized {
                                self.minimized = is_minimized;

                                // 最小化期间不应有任何声音（或音乐）播放
                                if self.minimized {
                                    self.mute(true);
                                } else {
                                    self.unmute(true);
                                    if let Some(wm) = self.widget_manager {
                                        unsafe { (*wm).mark_all_dirty(); }
                                    }
                                }
                            }

                            self.rehup_focus();
                        }
                        DEMO_MOUSE_WHEEL => {
                            let a_scroll = self.m_demo_buffer.read_num_bits(8, true);
                            if let Some(wm) = self.widget_manager {
                                unsafe { (*wm).mouse_wheel(a_scroll); }
                            }
                        }
                        DEMO_KEY_DOWN => {
                            let a_key_code = self.m_demo_buffer.read_num_bits(8, false);
                            if let Some(wm) = self.widget_manager {
                                unsafe { (*wm).key_down(a_key_code); }
                            }
                        }
                        DEMO_KEY_UP => {
                            let a_key_code = self.m_demo_buffer.read_num_bits(8, false);
                            if let Some(wm) = self.widget_manager {
                                unsafe { (*wm).key_up(a_key_code); }
                            }
                        }
                        DEMO_KEY_CHAR => {
                            // 1 表示单字节，2 表示双字节
                            let size_mult = self.m_demo_buffer.read_num_bits(1, false) + 1;
                            let a_char = self.m_demo_buffer.read_num_bits(8 * size_mult, false) as u8;
                            if let Some(wm) = self.widget_manager {
                                unsafe { (*wm).key_char(a_char); }
                            }
                        }
                        DEMO_KEY_TEXT => {
                            let a_text = self.m_demo_buffer.read_string();
                            if !a_text.is_empty() {
                                if let Some(wm) = self.widget_manager {
                                    unsafe { (*wm).key_text(&a_text); }
                                }
                            }
                        }
                        DEMO_CLOSE => {
                            self.shutdown();
                        }
                        DEMO_MOUSE_ENTER => {
                            self.m_mouse_in = true;
                            self.enforce_cursor();
                        }
                        DEMO_MOUSE_EXIT => {
                            if let Some(wm) = self.widget_manager {
                                unsafe {
                                    (*wm).mouse_exit(self.m_last_demo_mouse_x, self.m_last_demo_mouse_y);
                                }
                            }
                            self.m_mouse_in = false;
                            self.enforce_cursor();
                        }
                        DEMO_LOADING_COMPLETE => {
                            self.m_demo_loading_complete = true;
                        }
                        DEMO_VIDEO_DATA => {
                            self.is_windowed = self.m_demo_buffer.read_boolean();
                            self.m_sync_refresh_rate = self.m_demo_buffer.read_byte();
                        }
                        DEMO_IDLE => {}
                        DEMO_REGISTRY_GETSUBKEYS
                        | DEMO_REGISTRY_READ
                        | DEMO_REGISTRY_WRITE
                        | DEMO_REGISTRY_ERASE
                        | DEMO_FILE_EXISTS
                        | DEMO_FILE_READ
                        | DEMO_FILE_WRITE
                        | DEMO_SYNC
                        | DEMO_ASSERT_STRING_EQUAL
                        | DEMO_ASSERT_INT_EQUAL => {
                            // 跨 tick 仍未被游戏逻辑认领 ⇒ 回放已经跑偏
                            if self.m_demo_command_queued && self.m_update_count != self.m_demo_queued_since {
                                self.shutdown();
                                return;
                            }
                            self.m_demo_queued_since = self.m_update_count;
                            self.m_demo_command_queued = true;
                            // 回退读取游标，把命令留给游戏逻辑的调用点去消费
                            self.m_demo_buffer.read_bit_pos = self.m_demo_cmd_bit_pos;
                            self.m_last_demo_update_cnt = self.m_demo_cmd_update_cnt;
                            self.m_demo_needs_command = true;
                            return;
                        }
                        // C++: default: DBG_ASSERTE("Invalid Demo Command" == 0); break;
                        _ => {}
                    }
                }
            }
        }
    }
}

/// 对应 C++ `IsStampedDemoFileName()`（SexyAppBase.cpp:599-614）
///
/// 匹配保留的自动命名模式：`theDemoPrefix + "-YYYYMMDD-HHMMSS[-N].dmo"`。
fn is_stamped_demo_file_name(the_demo_prefix: &str, the_name: &str) -> bool {
    let a_prefix = the_demo_prefix.as_bytes();
    let a_name = the_name.as_bytes();
    let a_stamp_len = a_prefix.len() + 16; // "-YYYYMMDD-HHMMSS"

    if a_name.len() < a_stamp_len + 4
        || !a_name.starts_with(a_prefix)
        || !a_name.ends_with(b".dmo")
    {
        return false;
    }

    let a_digits = |s: &[u8]| !s.is_empty() && s.iter().all(|c| c.is_ascii_digit());

    let a_stamp = &a_name[a_prefix.len()..a_prefix.len() + 16];
    if a_stamp[0] != b'-' || a_stamp[9] != b'-' || !a_digits(&a_stamp[1..9]) || !a_digits(&a_stamp[10..16]) {
        return false;
    }

    let a_suffix = &a_name[a_stamp_len..a_name.len() - 4];
    a_suffix.is_empty() || (a_suffix[0] == b'-' && a_digits(&a_suffix[1..]))
}

/// 对应 C++ `FindDemoFiles()`（SexyAppBase.cpp:616-639）
///
/// 按「时间戳降序 → 名称更长（带后缀）优先 → 字典序」排序。
fn find_demo_files(the_demo_prefix: &str, the_stamped_only: bool) -> Vec<String> {
    let mut a_files: Vec<String> = Vec::new();

    let a_filter = format!("{}-", the_demo_prefix);
    if let Ok(a_entries) = std::fs::read_dir(".") {
        for an_entry in a_entries.flatten() {
            let a_path = an_entry.path();
            if !a_path.is_file() {
                continue;
            }
            let a_name = match a_path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => continue,
            };
            if a_name.starts_with(&a_filter)
                && a_name.ends_with(".dmo")
                && (!the_stamped_only || is_stamped_demo_file_name(the_demo_prefix, &a_name))
            {
                a_files.push(a_name);
            }
        }
    }

    let a_stamp_len = the_demo_prefix.len() + 16; // "-YYYYMMDD-HHMMSS"
    let a_key = |s: &str| -> (Vec<u8>, usize, String) {
        let b = s.as_bytes();
        let a_stamp: Vec<u8> = if b.len() >= a_stamp_len {
            b[..a_stamp_len].to_vec()
        } else {
            b.to_vec()
        };
        (a_stamp, b.len(), s.to_string())
    };

    // C++ 按 aKeyOf(a) > aKeyOf(b) 排序（降序）
    a_files.sort_by(|a, b| a_key(b).cmp(&a_key(a)));

    a_files
}

/// 对应 C++ `ParamTakesValue()`（SexyAppBase.cpp:3250-3256）
fn param_takes_value(the_param_name: &str) -> bool {
    const K_VALUE_PARAMS: [&str; 6] = ["-play", "-playnum", "-record", "-recnum", "-resdir", "-savedir"];
    K_VALUE_PARAMS.contains(&the_param_name)
}

extern "C" {
    /// C 运行时 `localtime`（返回指向 `struct tm` 的指针；
    /// 前 6 个 int 字段依次为 sec/min/hour/mday/mon/year，Windows 与 glibc 布局一致）
    fn localtime(timep: *const i64) -> *const i32;
}

/// 对应 C++ `GetTimestampedDemoFileName()`（SexyAppBase.cpp:3229-3248）
///
/// 文件名形如 `<prefix>-YYYYMMDD-HHMMSS.dmo`；若同秒记录已存在则追加 `-N`（N 递增）。
fn get_timestamped_demo_file_name(the_demo_prefix: &str) -> String {
    let a_now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let (a_year, a_mon, a_mday, a_hour, a_min, a_sec) = unsafe {
        let a_tm = localtime(&a_now);
        if a_tm.is_null() {
            (1970, 1, 1, 0, 0, 0)
        } else {
            (
                *a_tm.add(5) + 1900, // tm_year
                *a_tm.add(4) + 1,    // tm_mon
                *a_tm.add(3),        // tm_mday
                *a_tm.add(2),        // tm_hour
                *a_tm.add(1),        // tm_min
                *a_tm.add(0),        // tm_sec
            )
        }
    };

    let a_base_name = format!(
        "{}-{:04}{:02}{:02}-{:02}{:02}{:02}",
        the_demo_prefix, a_year, a_mon, a_mday, a_hour, a_min, a_sec
    );
    let a_name = format!("{}.dmo", a_base_name);
    let a_suffix_prefix = format!("{}-", a_base_name);

    let a_demo_files = find_demo_files(the_demo_prefix, true);
    for a_file_name in a_demo_files.iter() {
        if *a_file_name == a_name {
            return format!("{}-2.dmo", a_base_name);
        }
        if a_file_name.starts_with(&a_suffix_prefix) {
            let a_num: i32 = a_file_name[a_suffix_prefix.len()..]
                .trim_end_matches(".dmo")
                .parse()
                .unwrap_or(0);
            return format!("{}-{}.dmo", a_base_name, a_num + 1);
        }
    }

    a_name
}

// ---- DialogListener / ButtonListener 实现 ----
// 对应 C++ SexyAppBase 同时继承 ButtonListener 和 DialogListener

/// DialogListener 接口实现
pub trait DialogListenerImpl {
    fn dialog_button_press(&mut self, dialog_id: i32, button_id: i32);
    fn dialog_button_depress(&mut self, dialog_id: i32, button_id: i32);
}

/// ButtonListener 接口实现
pub trait ButtonListenerImpl {
    fn button_press(&mut self, the_id: i32);
    fn button_depress(&mut self, the_id: i32);
    fn button_down_tick(&mut self, the_id: i32);
    fn button_mouse_enter(&mut self, the_id: i32);
    fn button_mouse_leave(&mut self, the_id: i32);
    fn button_mouse_move(&mut self, the_id: i32, x: i32, y: i32);
}

impl DialogListenerImpl for SexyAppBase {
    fn dialog_button_press(&mut self, _dialog_id: i32, _button_id: i32) {}
    fn dialog_button_depress(&mut self, dialog_id: i32, button_id: i32) {
        // 对话框按钮释放事件 - 由 DialogManager 处理
        let _ = (dialog_id, button_id);
    }
}

impl Default for SexyAppBase {
    fn default() -> Self {
        SexyAppBase::new()
    }
}

impl ButtonListenerImpl for SexyAppBase {
    fn button_press(&mut self, _the_id: i32) {}
    fn button_depress(&mut self, _the_id: i32) {}
    fn button_down_tick(&mut self, _the_id: i32) {}
    fn button_mouse_enter(&mut self, _the_id: i32) {}
    fn button_mouse_leave(&mut self, _the_id: i32) {}
    fn button_mouse_move(&mut self, _the_id: i32, _x: i32, _y: i32) {}
}
