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

/// 应用基类
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

    // 光标
    pub cursor_num: i32,

    // 图像集
    pub memory_image_set: Vec<*mut MemoryImage>,

    // 游戏循环
    pub update_app_state: i32,
    pub update_app_depth: i32,
    pub update_multiplier: f64,
    pub paused: bool,

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
            title: String::from("PvZ Portable"),
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
            cursor_num: 0,
            memory_image_set: Vec::new(),
            update_app_state: 0,
            update_app_depth: 0,
            update_multiplier: 1.0,
            paused: false,
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
    /// 包含: 事件处理 → 更新游戏逻辑 → 绘制
    pub fn update_app(&mut self) -> bool {
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

    /// 更新帧（对应 C++ SexyAppBase::DoUpdateFrames → UpdateFrames）
    fn do_update_frames(&mut self) -> bool {
        // 对应 C++ UpdateFrames: mWidgetManager->UpdateFrame()
        if let Some(wm) = self.widget_manager {
            unsafe {
                (*wm).update();
            }
        }
        self.m_update_count += 1;
        true
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
                    self.key_down(*kc);
                }

                Event::KeyDown { .. } => {} // no keycode

                Event::KeyUp {
                    keycode: Some(kc), ..
                } => {
                    self.key_up(*kc);
                }

                Event::KeyUp { .. } => {} // no keycode

                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    let btn = match mouse_btn {
                        MouseButton::Left => 1,
                        MouseButton::Middle => 2,
                        MouseButton::Right => 3,
                        _ => 0,
                    };
                    self.mouse_down(x, y, btn, 1);
                }

                Event::MouseButtonUp { x, y, .. } => {
                    self.mouse_up(x, y);
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
        self.dialog_map.remove(&dialog_id);
        true
    }

    pub fn add_dialog(&mut self, dialog_id: i32, dialog: *mut Dialog) {
        self.dialog_map.insert(dialog_id, dialog);
    }

    /// 重绘
    pub fn redraw(&self, _clip_rect: Option<&Rect>) {}

    // ---- 输入事件处理（可被子类重写）----

    pub fn key_down(&mut self, _key: i32) {}
    pub fn key_up(&mut self, _key: i32) {}
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32, _click_count: i32) {}
    pub fn mouse_up(&mut self, _x: i32, _y: i32) {}
    pub fn mouse_move(&mut self, _x: i32, _y: i32) {}

    /// 鼠标滚轮（对应 C++ MouseWheel）
    pub fn mouse_wheel(&mut self, _delta: i32) {}

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
