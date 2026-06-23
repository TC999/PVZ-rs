// PvZ Portable Rust 翻译 — SexyAppBase（应用程序基类）
// 对应 C++ SexyAppFramework/SexyAppBase.h / SexyAppBase.cpp

#![allow(dead_code)]

use std::collections::HashMap;
use std::ffi::CString;

use crate::framework::color::Color;
use crate::framework::common;
use crate::framework::rect::Rect;
use crate::framework::buffer::Buffer;
use crate::framework::graphics::image::Image;
use crate::framework::graphics::gl_image::GLImage;
use crate::framework::graphics::memory_image::MemoryImage;
use crate::framework::graphics::gl_interface::GLInterface;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget::Widget;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::dialog::Dialog;
use crate::framework::sound::sound_manager::SoundManager;
use crate::framework::sound::music_interface::MusicInterface;
use crate::framework::resource_manager::ResourceManager;
use crate::ffi::sdl2::*;

/// 应用基类
pub struct SexyAppBase {
    // 窗口和上下文
    pub window: *mut std::ffi::c_void,
    pub context: *mut std::ffi::c_void,
    pub surface: *mut std::ffi::c_void,

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
    pub shutdown: bool,
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

    // 子系统
    pub widget_manager: Option<*mut WidgetManager>,
    pub gl_interface: Option<*mut GLInterface>,
    pub sound_manager: Option<*mut dyn SoundManager>,
    pub music_interface: Option<*mut dyn MusicInterface>,
    pub resource_manager: Option<*mut ResourceManager>,

    // 对话框
    pub dialog_map: HashMap<i32, *mut Dialog>,

    // 光标
    pub cursor_num: i32,

    // 图像集
    pub memory_image_set: Vec<*mut MemoryImage>,

    // 游戏循环
    pub update_app_state: i32,
    pub update_app_depth: i32,
    pub update_multiplier: f64,
    pub paused: bool,

    // 新增：主循环控制
    pub running: bool,
    pub last_time: u32,
    pub frame_time_target: u32,
}

impl SexyAppBase {
    pub fn new() -> Self {
        // 初始化 SDL（对应 C++ 构造函数中的 SDL_Init(SDL_INIT_TIMER)）
        unsafe {
            SDL_Init(SDL_INIT_TIMER);
        }

        SexyAppBase {
            window: std::ptr::null_mut(),
            context: std::ptr::null_mut(),
            surface: std::ptr::null_mut(),
            rand_seed: 0,
            company_name: String::new(),
            prod_name: String::new(),
            title: String::from("PvZ Portable"),
            reg_key: String::new(),
            resource_dir: String::new(),
            custom_save_dir: String::new(),
            width: 800,
            height: 600,
            preferred_x: 0,
            preferred_y: 0,
            fullscreen_bits: 32,
            music_volume: 1.0,
            sfx_volume: 1.0,
            is_windowed: true,
            shutdown: false,
            minimized: false,
            active: true,
            has_focus: true,
            ctrl_down: false,
            alt_down: false,
            fps_count: 0,
            fps_time: 0,
            show_fps: false,
            frame_time: 0,
            widget_manager: None,
            gl_interface: None,
            sound_manager: None,
            music_interface: None,
            resource_manager: None,
            dialog_map: HashMap::new(),
            cursor_num: 0,
            memory_image_set: Vec::new(),
            update_app_state: 0,
            update_app_depth: 0,
            update_multiplier: 1.0,
            paused: false,
            running: false,
            last_time: 0,
            frame_time_target: 16, // ~60 FPS
        }
    }

    /// 初始化（对应 C++ SexyAppBase::Init）
    pub fn init(&mut self) {
        eprintln!("[SexyAppBase::init] 开始...");

        // 设置资源目录
        self.resource_dir = common::get_cur_dir();
        eprintln!("[SexyAppBase::init] 资源目录: {}", self.resource_dir);

        // 创建窗口和 GL 上下文（对应 C++ MakeWindow）
        self.make_window();

        if self.shutdown {
            eprintln!("[SexyAppBase::init] 窗口创建失败！");
            return;
        }

        // 加载资源
        self.load_resource_manifest();
        eprintln!("[SexyAppBase::init] 完成");
    }

    /// 创建 SDL 窗口和 OpenGL 上下文（对应 C++ MakeWindow）
    pub fn make_window(&mut self) {
        eprintln!("[SexyAppBase::make_window] 创建窗口...");

        // 初始化 SDL 视频子系统
        unsafe {
            if SDL_Init(SDL_INIT_VIDEO) < 0 {
                let err = SDL_GetError();
                if !err.is_null() {
                    let c_str = std::ffi::CStr::from_ptr(err);
                    eprintln!("SDL_Init(SDL_INIT_VIDEO) 失败: {:?}", c_str);
                }
                self.shutdown = true;
                return;
            }
        }
        eprintln!("[SexyAppBase::make_window] SDL_Init 成功");

        // 设置 OpenGL ES 2.0 属性
        unsafe {
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_ES);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
            SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
            SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 0);
        }

        let title_cstr = CString::new(self.title.clone()).unwrap();

        // 创建窗口
        unsafe {
            self.window = SDL_CreateWindow(
                title_cstr.as_ptr(),
                /*SDL_WINDOWPOS_CENTERED*/ 0x2FFF0000i32,
                /*SDL_WINDOWPOS_CENTERED*/ 0x2FFF0000i32,
                self.width,
                self.height,
                SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN | SDL_WINDOW_RESIZABLE,
            ) as *mut std::ffi::c_void;
        }

        if self.window.is_null() {
            unsafe {
                let err = SDL_GetError();
                if !err.is_null() {
                    let c_str = std::ffi::CStr::from_ptr(err);
                    eprintln!("SDL_CreateWindow 失败: {:?}", c_str);
                }
            }
            self.shutdown = true;
            return;
        }

        // 创建 OpenGL 上下文
        unsafe {
            self.context = SDL_GL_CreateContext(self.window as *mut SDL_Window) as *mut std::ffi::c_void;
        }

        if self.context.is_null() {
            // 回退：尝试桌面 OpenGL 2.1
            unsafe {
                if !self.window.is_null() {
                    SDL_DestroyWindow(self.window as *mut SDL_Window);
                    self.window = std::ptr::null_mut();
                }
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_COMPATIBILITY);
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 1);

                self.window = SDL_CreateWindow(
                    title_cstr.as_ptr(),
                    0x2FFF0000i32,
                    0x2FFF0000i32,
                    self.width,
                    self.height,
                    SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN | SDL_WINDOW_RESIZABLE,
                ) as *mut std::ffi::c_void;

                if !self.window.is_null() {
                    self.context = SDL_GL_CreateContext(self.window as *mut SDL_Window) as *mut std::ffi::c_void;
                }
            }

            if self.window.is_null() || self.context.is_null() {
                eprintln!("无法创建 OpenGL 上下文，请检查显卡驱动。");
                self.shutdown = true;
                return;
            }
        }

        // 启用垂直同步
        unsafe {
            SDL_GL_SetSwapInterval(1);
        }

        // 初始化 GL 接口（加载 glad/GLES2）
        if self.gl_interface.is_none() {
            let mut gl = Box::new(GLInterface::new(None));
            gl.init(true);
            self.gl_interface = Some(Box::into_raw(gl));
        }
    }

    /// 启动主循环（对应 C++ SexyAppBase::Start + DoMainLoop）
    pub fn start(&mut self) {
        if self.shutdown {
            return;
        }

        self.running = true;
        self.last_time = unsafe { SDL_GetTicks() };

        // 主循环（对应 C++ DoMainLoop）
        while !self.shutdown {
            self.update_app();
            // 简单的帧率控制
            let now = unsafe { SDL_GetTicks() };
            let elapsed = now.wrapping_sub(self.last_time);
            if elapsed < self.frame_time_target {
                unsafe {
                    SDL_Delay(self.frame_time_target - elapsed);
                }
            }
            self.last_time = unsafe { SDL_GetTicks() };
        }

        self.running = false;
    }

    /// 主循环单步（处理事件 + 更新 + 绘制）
    pub fn update_app(&mut self) -> bool {
        // 处理所有待处理的 SDL 事件
        self.process_deferred_messages(true);

        if self.shutdown {
            return false;
        }

        // 绘制
        self.draw();
        // 交换缓冲区
        self.swap_buffers();

        true
    }

    /// 处理 SDL 事件队列（对应 C++ ProcessDeferredMessages）
    pub fn process_deferred_messages(&mut self, _allow_quit: bool) -> bool {
        loop {
            let mut event = std::mem::MaybeUninit::<SDL_Event>::uninit();
            let has_event = unsafe { SDL_PollEvent(event.as_mut_ptr()) };
            if has_event == 0 {
                break;
            }

            let event = unsafe { event.assume_init() };
            let event_type = unsafe { event.type_ };

            match event_type {
                SDL_QUIT => {
                    self.shutdown = true;
                    return false;
                }

                SDL_WINDOWEVENT => {
                    let window_event = unsafe { event.window };
                    match window_event.event {
                        SDL_WINDOWEVENT_CLOSE => {
                            self.shutdown = true;
                            return false;
                        }
                        SDL_WINDOWEVENT_RESIZED | SDL_WINDOWEVENT_SIZE_CHANGED => {
                            // 更新窗口大小
                            self.width = window_event.data1 as i32;
                            self.height = window_event.data2 as i32;
                            if let Some(gl) = self.gl_interface.as_mut() {
                                unsafe {
                                    (**gl).width = self.width;
                                    (**gl).height = self.height;
                                    (**gl).update_viewport();
                                }
                            }
                        }
                        SDL_WINDOWEVENT_FOCUS_GAINED => {
                            self.has_focus = true;
                            self.active = true;
                        }
                        SDL_WINDOWEVENT_FOCUS_LOST => {
                            self.has_focus = false;
                            self.active = false;
                        }
                        SDL_WINDOWEVENT_MINIMIZED => {
                            self.minimized = true;
                        }
                        SDL_WINDOWEVENT_RESTORED => {
                            self.minimized = false;
                        }
                        _ => {}
                    }
                }

                SDL_KEYDOWN => {
                    let key = unsafe { event.key };
                    let keycode = key.keysym.sym;
                    let mod_state = unsafe { SDL_GetModState() };
                    self.ctrl_down = (mod_state & KMOD_CTRL) != 0;
                    self.alt_down = (mod_state & KMOD_ALT) != 0;
                    self.key_down(keycode);
                }

                SDL_KEYUP => {
                    let key = unsafe { event.key };
                    let keycode = key.keysym.sym;
                    self.key_up(keycode);
                }

                SDL_MOUSEBUTTONDOWN => {
                    let button = unsafe { event.button };
                    let x = button.x as i32;
                    let y = button.y as i32;
                    let btn = button.button as i32;
                    self.mouse_down(x, y, btn, 1);
                }

                SDL_MOUSEBUTTONUP => {
                    let button = unsafe { event.button };
                    let x = button.x as i32;
                    let y = button.y as i32;
                    self.mouse_up(x, y);
                }

                SDL_MOUSEMOTION => {
                    let motion = unsafe { event.motion };
                    let x = motion.x as i32;
                    let y = motion.y as i32;
                    self.mouse_move(x, y);
                }

                _ => {}
            }
        }

        true
    }

    /// 绘制（由子类重写）
    pub fn draw(&self) {
        if let Some(gl) = self.gl_interface {
            unsafe {
                (*gl).pre_draw();
            }
        }
    }

    /// 交换缓冲区
    pub fn swap_buffers(&self) {
        if !self.window.is_null() {
            unsafe {
                SDL_GL_SwapWindow(self.window as *mut SDL_Window);
            }
        }
    }

    /// 关闭
    pub fn shutdown(&mut self) {
        self.shutdown = true;

        // 清理 GL 接口
        if let Some(gl) = self.gl_interface.take() {
            unsafe {
                let _ = Box::from_raw(gl);
            }
        }

        // 销毁窗口
        if !self.window.is_null() {
            unsafe {
                if !self.context.is_null() {
                    SDL_GL_DeleteContext(self.context as *mut std::ffi::c_void);
                    self.context = std::ptr::null_mut();
                }
                SDL_DestroyWindow(self.window as *mut SDL_Window);
                self.window = std::ptr::null_mut();
            }
        }

        unsafe {
            SDL_QuitSubSystem(SDL_INIT_VIDEO);
            SDL_QuitSubSystem(SDL_INIT_TIMER);
        }
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
    pub fn set_args(&mut self, _argc: i32, _argv: *mut *mut std::os::raw::c_char) {
        // 处理命令行参数
    }

    /// 加载资源清单
    pub fn load_resource_manifest(&mut self) {
        // 加载资源列表
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
        self.dialog_map.remove(&dialog_id);
        true
    }

    pub fn add_dialog(&mut self, dialog_id: i32, dialog: *mut Dialog) {
        self.dialog_map.insert(dialog_id, dialog);
    }

    /// 重绘
    pub fn redraw(&self, _clip_rect: Option<&Rect>) {
        // 由子类实现
    }

    // ---- 输入事件处理（可被子类重写）----

    pub fn key_down(&mut self, _key: i32) {
        // 由子类实现
    }

    pub fn key_up(&mut self, _key: i32) {
        // 由子类实现
    }

    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32, _click_count: i32) {
        // 由子类实现
    }

    pub fn mouse_up(&mut self, _x: i32, _y: i32) {
        // 由子类实现
    }

    pub fn mouse_move(&mut self, _x: i32, _y: i32) {
        // 由子类实现
    }
}

impl Default for SexyAppBase {
    fn default() -> Self {
        SexyAppBase::new()
    }
}
