// PvZ Portable Rust 翻译 — SDL2 FFI 绑定
//
// 使用 extern "C" 声明所需的 SDL2 C API。
// 只声明本游戏实际使用的函数。

#![allow(non_camel_case_types, dead_code)]
// SDL2 结构体字段名和常量名保持与 C 头文件一致
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void, c_double, c_float, c_long, c_short, c_uchar, c_ushort};

// ============================================================
// SDL 初始化标志
// ============================================================
pub const SDL_INIT_TIMER: Uint32 = 0x00000001;
pub const SDL_INIT_AUDIO: Uint32 = 0x00000010;
pub const SDL_INIT_VIDEO: Uint32 = 0x00000020;
pub const SDL_INIT_JOYSTICK: Uint32 = 0x00000200;
pub const SDL_INIT_HAPTIC: Uint32 = 0x00001000;
pub const SDL_INIT_GAMEPAD: Uint32 = 0x00002000;
pub const SDL_INIT_EVENTS: Uint32 = 0x00004000;
pub const SDL_INIT_SENSOR: Uint32 = 0x00008000;
pub const SDL_INIT_NOPARACHUTE: Uint32 = 0x00100000;

// 窗口居中常量
pub const SDL_WINDOWPOS_CENTERED: c_int = 0x2FFF0000;
pub const SDL_WINDOWPOS_UNDEFINED: c_int = 0x1FFF0000;

// ============================================================
// SDL2 类型定义
// ============================================================

pub type SDL_bool = c_int;
pub const SDL_TRUE: SDL_bool = 1;
pub const SDL_FALSE: SDL_bool = 0;

pub type Uint8 = u8;
pub type Uint16 = u16;
pub type Uint32 = u32;
pub type Uint64 = u64;
pub type Sint16 = i16;
pub type Sint32 = i32;
pub type Sint64 = i64;

#[repr(C)]
pub struct SDL_Rect {
    pub x: c_int,
    pub y: c_int,
    pub w: c_int,
    pub h: c_int,
}

#[repr(C)]
pub struct SDL_Point {
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}

#[repr(C)]
pub struct SDL_Palette {
    pub ncolors: c_int,
    pub colors: *mut SDL_Color,
    pub version: Uint32,
    pub refcount: c_int,
}

#[repr(C)]
pub struct SDL_PixelFormat {
    pub format: Uint32,
    pub palette: *mut SDL_Palette,
    pub BitsPerPixel: Uint8,
    pub BytesPerPixel: Uint8,
    pub padding: [Uint8; 2],
    pub Rmask: Uint32,
    pub Gmask: Uint32,
    pub Bmask: Uint32,
    pub Amask: Uint32,
    pub Rloss: Uint8,
    pub Gloss: Uint8,
    pub Bloss: Uint8,
    pub Aloss: Uint8,
    pub Rshift: Uint8,
    pub Gshift: Uint8,
    pub Bshift: Uint8,
    pub Ashift: Uint8,
    pub refcount: c_int,
    pub next: *mut SDL_PixelFormat,
}

#[repr(C)]
pub struct SDL_Surface {
    pub flags: Uint32,
    pub format: *mut SDL_PixelFormat,
    pub w: c_int,
    pub h: c_int,
    pub pitch: c_int,
    pub pixels: *mut c_void,
    pub userdata: *mut c_void,
    pub locked: c_int,
    pub lock_data: *mut c_void,
    pub clip_rect: SDL_Rect,
    pub map: *mut c_void,
    pub refcount: c_int,
}

#[repr(C)]
pub struct SDL_Texture {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SDL_Renderer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SDL_Window {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SDL_Cursor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SDL_GameController {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SDL_Joystick {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SDL_IOStream {
    _private: [u8; 0],
}

// 事件类型
pub const SDL_QUIT: Uint32 = 0x100;
pub const SDL_KEYDOWN: Uint32 = 0x300;
pub const SDL_KEYUP: Uint32 = 0x301;
pub const SDL_TEXTEDITING: Uint32 = 0x302;
pub const SDL_TEXTINPUT: Uint32 = 0x303;
pub const SDL_KEYMAPCHANGED: Uint32 = 0x304;
pub const SDL_MOUSEMOTION: Uint32 = 0x400;
pub const SDL_MOUSEBUTTONDOWN: Uint32 = 0x401;
pub const SDL_MOUSEBUTTONUP: Uint32 = 0x402;
pub const SDL_MOUSEWHEEL: Uint32 = 0x403;
pub const SDL_WINDOWEVENT: Uint32 = 0x200;
pub const SDL_DISPLAYEVENT: Uint32 = 0x150;
pub const SDL_FINGERDOWN: Uint32 = 0x700;
pub const SDL_FINGERUP: Uint32 = 0x701;
pub const SDL_FINGERMOTION: Uint32 = 0x702;
pub const SDL_DROPFILE: Uint32 = 0x1000;
pub const SDL_DROPTEXT: Uint32 = 0x1001;
pub const SDL_DROPBEGIN: Uint32 = 0x1002;
pub const SDL_DROPCOMPLETE: Uint32 = 0x1003;
pub const SDL_CLIPBOARDUPDATE: Uint32 = 0x900;
pub const SDL_RENDER_TARGETS_RESET: Uint32 = 0x2000;
pub const SDL_RENDER_DEVICE_RESET: Uint32 = 0x2001;
pub const SDL_AUDIODEVICEADDED: Uint32 = 0x1100;
pub const SDL_AUDIODEVICEREMOVED: Uint32 = 0x1101;
pub const SDL_JOYAXISMOTION: Uint32 = 0x600;
pub const SDL_JOYBALLMOTION: Uint32 = 0x601;
pub const SDL_JOYHATMOTION: Uint32 = 0x602;
pub const SDL_JOYBUTTONDOWN: Uint32 = 0x603;
pub const SDL_JOYBUTTONUP: Uint32 = 0x604;
pub const SDL_JOYDEVICEADDED: Uint32 = 0x605;
pub const SDL_JOYDEVICEREMOVED: Uint32 = 0x606;
pub const SDL_CONTROLLERAXISMOTION: Uint32 = 0x650;
pub const SDL_CONTROLLERBUTTONDOWN: Uint32 = 0x651;
pub const SDL_CONTROLLERBUTTONUP: Uint32 = 0x652;
pub const SDL_CONTROLLERDEVICEADDED: Uint32 = 0x653;
pub const SDL_CONTROLLERDEVICEREMOVED: Uint32 = 0x654;
pub const SDL_CONTROLLERDEVICEREMAPPED: Uint32 = 0x655;

pub const SDL_WINDOWEVENT_SHOWN: Uint8 = 1;
pub const SDL_WINDOWEVENT_HIDDEN: Uint8 = 2;
pub const SDL_WINDOWEVENT_EXPOSED: Uint8 = 3;
pub const SDL_WINDOWEVENT_MOVED: Uint8 = 4;
pub const SDL_WINDOWEVENT_RESIZED: Uint8 = 5;
pub const SDL_WINDOWEVENT_SIZE_CHANGED: Uint8 = 6;
pub const SDL_WINDOWEVENT_MINIMIZED: Uint8 = 7;
pub const SDL_WINDOWEVENT_MAXIMIZED: Uint8 = 8;
pub const SDL_WINDOWEVENT_RESTORED: Uint8 = 9;
pub const SDL_WINDOWEVENT_ENTER: Uint8 = 10;
pub const SDL_WINDOWEVENT_LEAVE: Uint8 = 11;
pub const SDL_WINDOWEVENT_FOCUS_GAINED: Uint8 = 12;
pub const SDL_WINDOWEVENT_FOCUS_LOST: Uint8 = 13;
pub const SDL_WINDOWEVENT_CLOSE: Uint8 = 14;
pub const SDL_WINDOWEVENT_TAKE_FOCUS: Uint8 = 15;
pub const SDL_WINDOWEVENT_HIT_TEST: Uint8 = 16;
pub const SDL_WINDOWEVENT_ICCPROF_CHANGED: Uint8 = 17;
pub const SDL_WINDOWEVENT_DISPLAY_CHANGED: Uint8 = 18;

#[repr(C)]
pub struct SDL_Keysym {
    pub scancode: SDL_Scancode,
    pub sym: SDL_Keycode,
    pub modifier: Uint16,
    pub unused: Uint32,
}

pub type SDL_Keycode = Sint32;
pub type SDL_Scancode = Sint32;

// 按键码
pub const SDLK_UNKNOWN: SDL_Keycode = 0;
pub const SDLK_BACKSPACE: SDL_Keycode = 8;
pub const SDLK_TAB: SDL_Keycode = 9;
pub const SDLK_RETURN: SDL_Keycode = 13;
pub const SDLK_ESCAPE: SDL_Keycode = 27;
pub const SDLK_SPACE: SDL_Keycode = 32;
pub const SDLK_EXCLAIM: SDL_Keycode = 33;
pub const SDLK_QUOTEDBL: SDL_Keycode = 34;
pub const SDLK_HASH: SDL_Keycode = 35;
pub const SDLK_DOLLAR: SDL_Keycode = 36;
pub const SDLK_PERCENT: SDL_Keycode = 37;
pub const SDLK_AMPERSAND: SDL_Keycode = 38;
pub const SDLK_QUOTE: SDL_Keycode = 39;
pub const SDLK_LEFTPAREN: SDL_Keycode = 40;
pub const SDLK_RIGHTPAREN: SDL_Keycode = 41;
pub const SDLK_ASTERISK: SDL_Keycode = 42;
pub const SDLK_PLUS: SDL_Keycode = 43;
pub const SDLK_COMMA: SDL_Keycode = 44;
pub const SDLK_MINUS: SDL_Keycode = 45;
pub const SDLK_PERIOD: SDL_Keycode = 46;
pub const SDLK_SLASH: SDL_Keycode = 47;
pub const SDLK_0: SDL_Keycode = 48;
pub const SDLK_1: SDL_Keycode = 49;
pub const SDLK_2: SDL_Keycode = 50;
pub const SDLK_3: SDL_Keycode = 51;
pub const SDLK_4: SDL_Keycode = 52;
pub const SDLK_5: SDL_Keycode = 53;
pub const SDLK_6: SDL_Keycode = 54;
pub const SDLK_7: SDL_Keycode = 55;
pub const SDLK_8: SDL_Keycode = 56;
pub const SDLK_9: SDL_Keycode = 57;
pub const SDLK_COLON: SDL_Keycode = 58;
pub const SDLK_SEMICOLON: SDL_Keycode = 59;
pub const SDLK_LESS: SDL_Keycode = 60;
pub const SDLK_EQUALS: SDL_Keycode = 61;
pub const SDLK_GREATER: SDL_Keycode = 62;
pub const SDLK_QUESTION: SDL_Keycode = 63;
pub const SDLK_AT: SDL_Keycode = 64;
pub const SDLK_LEFTBRACKET: SDL_Keycode = 91;
pub const SDLK_BACKSLASH: SDL_Keycode = 92;
pub const SDLK_RIGHTBRACKET: SDL_Keycode = 93;
pub const SDLK_CARET: SDL_Keycode = 94;
pub const SDLK_UNDERSCORE: SDL_Keycode = 95;
pub const SDLK_BACKQUOTE: SDL_Keycode = 96;
pub const SDLK_a: SDL_Keycode = 97;
pub const SDLK_b: SDL_Keycode = 98;
pub const SDLK_c: SDL_Keycode = 99;
pub const SDLK_d: SDL_Keycode = 100;
pub const SDLK_e: SDL_Keycode = 101;
pub const SDLK_f: SDL_Keycode = 102;
pub const SDLK_g: SDL_Keycode = 103;
pub const SDLK_h: SDL_Keycode = 104;
pub const SDLK_i: SDL_Keycode = 105;
pub const SDLK_j: SDL_Keycode = 106;
pub const SDLK_k: SDL_Keycode = 107;
pub const SDLK_l: SDL_Keycode = 108;
pub const SDLK_m: SDL_Keycode = 109;
pub const SDLK_n: SDL_Keycode = 110;
pub const SDLK_o: SDL_Keycode = 111;
pub const SDLK_p: SDL_Keycode = 112;
pub const SDLK_q: SDL_Keycode = 113;
pub const SDLK_r: SDL_Keycode = 114;
pub const SDLK_s: SDL_Keycode = 115;
pub const SDLK_t: SDL_Keycode = 116;
pub const SDLK_u: SDL_Keycode = 117;
pub const SDLK_v: SDL_Keycode = 118;
pub const SDLK_w: SDL_Keycode = 119;
pub const SDLK_x: SDL_Keycode = 120;
pub const SDLK_y: SDL_Keycode = 121;
pub const SDLK_z: SDL_Keycode = 122;
pub const SDLK_DELETE: SDL_Keycode = 127;
pub const SDLK_UP: SDL_Keycode = 273;
pub const SDLK_DOWN: SDL_Keycode = 274;
pub const SDLK_RIGHT: SDL_Keycode = 275;
pub const SDLK_LEFT: SDL_Keycode = 276;
pub const SDLK_INSERT: SDL_Keycode = 277;
pub const SDLK_HOME: SDL_Keycode = 278;
pub const SDLK_END: SDL_Keycode = 279;
pub const SDLK_PAGEUP: SDL_Keycode = 280;
pub const SDLK_PAGEDOWN: SDL_Keycode = 281;
pub const SDLK_F1: SDL_Keycode = 282;
pub const SDLK_F2: SDL_Keycode = 283;
pub const SDLK_F3: SDL_Keycode = 284;
pub const SDLK_F4: SDL_Keycode = 285;
pub const SDLK_F5: SDL_Keycode = 286;
pub const SDLK_F6: SDL_Keycode = 287;
pub const SDLK_F7: SDL_Keycode = 288;
pub const SDLK_F8: SDL_Keycode = 289;
pub const SDLK_F9: SDL_Keycode = 290;
pub const SDLK_F10: SDL_Keycode = 291;
pub const SDLK_F11: SDL_Keycode = 292;
pub const SDLK_F12: SDL_Keycode = 293;
pub const SDLK_CAPSLOCK: SDL_Keycode = 301;
pub const SDLK_RSHIFT: SDL_Keycode = 303;
pub const SDLK_LSHIFT: SDL_Keycode = 304;
pub const SDLK_RCTRL: SDL_Keycode = 305;
pub const SDLK_LCTRL: SDL_Keycode = 306;
pub const SDLK_RALT: SDL_Keycode = 307;
pub const SDLK_LALT: SDL_Keycode = 308;
pub const SDLK_MODE: SDL_Keycode = 313;
pub const SDLK_HELP: SDL_Keycode = 315;
pub const SDLK_PRINTSCREEN: SDL_Keycode = 316;
pub const SDLK_SYSREQ: SDL_Keycode = 317;
pub const SDLK_MENU: SDL_Keycode = 319;
pub const SDLK_POWER: SDL_Keycode = 320;
pub const SDLK_UNDO: SDL_Keycode = 322;
pub const SDLK_KP_0: SDL_Keycode = 256;
pub const SDLK_KP_1: SDL_Keycode = 257;
pub const SDLK_KP_2: SDL_Keycode = 258;
pub const SDLK_KP_3: SDL_Keycode = 259;
pub const SDLK_KP_4: SDL_Keycode = 260;
pub const SDLK_KP_5: SDL_Keycode = 261;
pub const SDLK_KP_6: SDL_Keycode = 262;
pub const SDLK_KP_7: SDL_Keycode = 263;
pub const SDLK_KP_8: SDL_Keycode = 264;
pub const SDLK_KP_9: SDL_Keycode = 265;
pub const SDLK_KP_PERIOD: SDL_Keycode = 266;
pub const SDLK_KP_DIVIDE: SDL_Keycode = 267;
pub const SDLK_KP_MULTIPLY: SDL_Keycode = 268;
pub const SDLK_KP_MINUS: SDL_Keycode = 269;
pub const SDLK_KP_PLUS: SDL_Keycode = 270;
pub const SDLK_KP_ENTER: SDL_Keycode = 271;
pub const SDLK_KP_EQUALS: SDL_Keycode = 272;

// 修饰键
pub const KMOD_NONE: Uint16 = 0x0000;
pub const KMOD_LSHIFT: Uint16 = 0x0001;
pub const KMOD_RSHIFT: Uint16 = 0x0002;
pub const KMOD_LCTRL: Uint16 = 0x0040;
pub const KMOD_RCTRL: Uint16 = 0x0080;
pub const KMOD_LALT: Uint16 = 0x0100;
pub const KMOD_RALT: Uint16 = 0x0200;
pub const KMOD_CTRL: Uint16 = 0x0040 | 0x0080;
pub const KMOD_SHIFT: Uint16 = 0x0001 | 0x0002;
pub const KMOD_ALT: Uint16 = 0x0100 | 0x0200;

// 鼠标按键
pub const SDL_BUTTON_LEFT: c_int = 1;
pub const SDL_BUTTON_MIDDLE: c_int = 2;
pub const SDL_BUTTON_RIGHT: c_int = 3;

// 窗口标志
pub const SDL_WINDOW_FULLSCREEN: Uint32 = 0x00000001;
pub const SDL_WINDOW_OPENGL: Uint32 = 0x00000002;
pub const SDL_WINDOW_SHOWN: Uint32 = 0x00000004;
pub const SDL_WINDOW_HIDDEN: Uint32 = 0x00000008;
pub const SDL_WINDOW_BORDERLESS: Uint32 = 0x00000010;
pub const SDL_WINDOW_RESIZABLE: Uint32 = 0x00000020;
pub const SDL_WINDOW_MINIMIZED: Uint32 = 0x00000040;
pub const SDL_WINDOW_MAXIMIZED: Uint32 = 0x00000080;
pub const SDL_WINDOW_MOUSE_GRABBED: Uint32 = 0x00000100;
pub const SDL_WINDOW_INPUT_FOCUS: Uint32 = 0x00000200;
pub const SDL_WINDOW_MOUSE_FOCUS: Uint32 = 0x00000400;
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: Uint32 = 0x00001001;
pub const SDL_WINDOW_ALLOW_HIGHDPI: Uint32 = 0x00002000;
pub const SDL_WINDOW_MOUSE_CAPTURE: Uint32 = 0x00004000;
pub const SDL_WINDOW_ALWAYS_ON_TOP: Uint32 = 0x00008000;
pub const SDL_WINDOW_SKIP_TASKBAR: Uint32 = 0x00010000;
pub const SDL_WINDOW_UTILITY: Uint32 = 0x00020000;
pub const SDL_WINDOW_TOOLTIP: Uint32 = 0x00040000;
pub const SDL_WINDOW_POPUP_MENU: Uint32 = 0x00080000;
pub const SDL_WINDOW_KEYBOARD_GRABBED: Uint32 = 0x00100000;

// 渲染器标志
pub const SDL_RENDERER_SOFTWARE: Uint32 = 0x00000001;
pub const SDL_RENDERER_ACCELERATED: Uint32 = 0x00000002;
pub const SDL_RENDERER_PRESENTVSYNC: Uint32 = 0x00000004;
pub const SDL_RENDERER_TARGETTEXTURE: Uint32 = 0x00000008;

// 纹理访问
pub const SDL_TEXTUREACCESS_STATIC: c_int = 0;
pub const SDL_TEXTUREACCESS_STREAMING: c_int = 1;
pub const SDL_TEXTUREACCESS_TARGET: c_int = 2;

// 像素格式
pub const SDL_PIXELFORMAT_UNKNOWN: Uint32 = 0;
pub const SDL_PIXELFORMAT_RGB888: Uint32 = 0x16161804;  // 0x1616 0804 for big endian vs little endian
pub const SDL_PIXELFORMAT_RGBA8888: Uint32 = 0x16462004;
pub const SDL_PIXELFORMAT_ARGB8888: Uint32 = 0x16362004;
pub const SDL_PIXELFORMAT_ABGR8888: Uint32 = 0x16382004;

// 混音器 (SDL_Mixer-X 外部库)
// 此处只声明 SDL2 核心 API，混音器通过 SDL_Mixer-X 自身链接

// 图像标志
pub const IMG_INIT_JPG: c_int = 0x00000001;
pub const IMG_INIT_PNG: c_int = 0x00000002;
pub const IMG_INIT_TIF: c_int = 0x00000004;
pub const IMG_INIT_WEBP: c_int = 0x00000008;

#[repr(C)]
pub union SDL_Event {
    pub type_: Uint32,
    pub key: std::mem::ManuallyDrop<SDL_KeyboardEvent>,
    pub motion: std::mem::ManuallyDrop<SDL_MouseMotionEvent>,
    pub button: std::mem::ManuallyDrop<SDL_MouseButtonEvent>,
    pub wheel: std::mem::ManuallyDrop<SDL_MouseWheelEvent>,
    pub window: std::mem::ManuallyDrop<SDL_WindowEvent>,
    pub quit: std::mem::ManuallyDrop<SDL_QuitEvent>,
    pub text: std::mem::ManuallyDrop<SDL_TextInputEvent>,
    pub drop: std::mem::ManuallyDrop<SDL_DropEvent>,
    pub finger: std::mem::ManuallyDrop<SDL_TouchFingerEvent>,
    pub jdevice: std::mem::ManuallyDrop<SDL_JoyDeviceEvent>,
    pub jbutton: std::mem::ManuallyDrop<SDL_JoyButtonEvent>,
    pub jaxis: std::mem::ManuallyDrop<SDL_JoyAxisEvent>,
    pub cdevice: std::mem::ManuallyDrop<SDL_ControllerDeviceEvent>,
    pub cbutton: std::mem::ManuallyDrop<SDL_ControllerButtonEvent>,
    pub caxis: std::mem::ManuallyDrop<SDL_ControllerAxisEvent>,
    pub padding: [Uint8; 56],
}

#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub state: Uint8,
    pub repeat: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub keysym: SDL_Keysym,
}

#[repr(C)]
pub struct SDL_MouseMotionEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub state: Uint32,
    pub x: c_int,
    pub y: c_int,
    pub xrel: c_int,
    pub yrel: c_int,
}

#[repr(C)]
pub struct SDL_MouseButtonEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub button: Uint8,
    pub state: Uint8,
    pub clicks: Uint8,
    pub padding1: Uint8,
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
pub struct SDL_MouseWheelEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub x: Sint32,
    pub y: Sint32,
    pub direction: Uint32,
    pub mouse_x: c_float,
    pub mouse_y: c_float,
}

#[repr(C)]
pub struct SDL_WindowEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub data1: Sint32,
    pub data2: Sint32,
}

#[repr(C)]
pub struct SDL_QuitEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
}

#[repr(C)]
pub struct SDL_TextInputEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: [c_char; 32],
}

#[repr(C)]
pub struct SDL_DropEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub file: *mut c_char,
    pub windowID: Uint32,
}

#[repr(C)]
pub struct SDL_TouchFingerEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub touchId: Sint64,
    pub fingerId: Sint64,
    pub x: c_float,
    pub y: c_float,
    pub dx: c_float,
    pub dy: c_float,
    pub pressure: c_float,
    pub windowID: Uint32,
}

#[repr(C)]
pub struct SDL_JoyDeviceEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}

#[repr(C)]
pub struct SDL_JoyButtonEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}

#[repr(C)]
pub struct SDL_JoyAxisEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}

#[repr(C)]
pub struct SDL_ControllerDeviceEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}

#[repr(C)]
pub struct SDL_ControllerButtonEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}

#[repr(C)]
pub struct SDL_ControllerAxisEvent {
    pub type_: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}

// 使用 raw-dylib 直接链接 SDL2.dll，无需导入库 .lib
// 需要 rustc 1.72+，当前 nightly 1.94 完全支持
#[link(name = "SDL2", kind = "raw-dylib")]
unsafe extern "C" {
    // 初始化/退出
    pub fn SDL_Init(flags: Uint32) -> c_int;
    pub fn SDL_InitSubSystem(flags: Uint32) -> c_int;
    pub fn SDL_Quit();
    pub fn SDL_QuitSubSystem(flags: Uint32);

    // 窗口管理
    pub fn SDL_CreateWindow(title: *const c_char, x: c_int, y: c_int, w: c_int, h: c_int, flags: Uint32) -> *mut SDL_Window;
    pub fn SDL_DestroyWindow(window: *mut SDL_Window);
    pub fn SDL_GetWindowSize(window: *mut SDL_Window, w: *mut c_int, h: *mut c_int);
    pub fn SDL_SetWindowSize(window: *mut SDL_Window, w: c_int, h: c_int);
    pub fn SDL_SetWindowTitle(window: *mut SDL_Window, title: *const c_char);
    pub fn SDL_ShowWindow(window: *mut SDL_Window);
    pub fn SDL_HideWindow(window: *mut SDL_Window);
    pub fn SDL_RaiseWindow(window: *mut SDL_Window);
    pub fn SDL_SetWindowFullscreen(window: *mut SDL_Window, flags: Uint32) -> c_int;
    pub fn SDL_SetWindowResizable(window: *mut SDL_Window, resizable: SDL_bool);
    pub fn SDL_GetWindowFlags(window: *mut SDL_Window) -> Uint32;
    pub fn SDL_SetWindowIcon(window: *mut SDL_Window, icon: *mut SDL_Surface);
    pub fn SDL_GetWindowPosition(window: *mut SDL_Window, x: *mut c_int, y: *mut c_int);
    pub fn SDL_SetWindowPosition(window: *mut SDL_Window, x: c_int, y: c_int);
    pub fn SDL_GetWindowDisplayIndex(window: *mut SDL_Window) -> c_int;
    pub fn SDL_GetDisplayBounds(displayIndex: c_int, rect: *mut SDL_Rect) -> c_int;
    pub fn SDL_SetWindowMouseGrab(window: *mut SDL_Window, grabbed: SDL_bool);
    pub fn SDL_SetWindowKeyboardGrab(window: *mut SDL_Window, grabbed: SDL_bool);

    // OpenGL
    pub fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: c_int) -> c_int;
    pub fn SDL_GL_GetAttribute(attr: SDL_GLattr, value: *mut c_int) -> c_int;
    pub fn SDL_GL_CreateContext(window: *mut SDL_Window) -> *mut c_void;
    pub fn SDL_GL_DeleteContext(context: *mut c_void);
    pub fn SDL_GL_SwapWindow(window: *mut SDL_Window);
    pub fn SDL_GL_MakeCurrent(window: *mut SDL_Window, context: *mut c_void) -> c_int;
    pub fn SDL_GL_SetSwapInterval(interval: c_int) -> c_int;
    pub fn SDL_GL_GetSwapInterval() -> c_int;
    pub fn SDL_GL_ExtensionSupported(extension: *const c_char) -> SDL_bool;
    pub fn SDL_GL_GetDrawableSize(window: *mut SDL_Window, w: *mut c_int, h: *mut c_int);
    pub fn SDL_GL_GetCurrentWindow() -> *mut SDL_Window;
    pub fn SDL_GL_GetCurrentContext() -> *mut c_void;

    // 渲染 (可选，游戏可能直接使用 OpenGL)
    pub fn SDL_CreateRenderer(window: *mut SDL_Window, index: c_int, flags: Uint32) -> *mut SDL_Renderer;
    pub fn SDL_DestroyRenderer(renderer: *mut SDL_Renderer);
    pub fn SDL_RenderPresent(renderer: *mut SDL_Renderer);
    pub fn SDL_RenderClear(renderer: *mut SDL_Renderer) -> c_int;
    pub fn SDL_RenderCopy(renderer: *mut SDL_Renderer, texture: *mut SDL_Texture, srcrect: *const SDL_Rect, dstrect: *const SDL_Rect) -> c_int;

    // 纹理
    pub fn SDL_CreateTexture(renderer: *mut SDL_Renderer, format: Uint32, access: c_int, w: c_int, h: c_int) -> *mut SDL_Texture;
    pub fn SDL_DestroyTexture(texture: *mut SDL_Texture);
    pub fn SDL_UpdateTexture(texture: *mut SDL_Texture, rect: *const SDL_Rect, pixels: *const c_void, pitch: c_int) -> c_int;
    pub fn SDL_UpdateYUVTexture(texture: *mut SDL_Texture, rect: *const SDL_Rect, yplane: *const c_void, ypitch: c_int, uplane: *const c_void, upitch: c_int, vplane: *const c_void, vpitch: c_int) -> c_int;
    pub fn SDL_QueryTexture(texture: *mut SDL_Texture, format: *mut Uint32, access: *mut c_int, w: *mut c_int, h: *mut c_int) -> c_int;
    pub fn SDL_SetTextureBlendMode(texture: *mut SDL_Texture, blendMode: c_int) -> c_int;
    pub fn SDL_SetTextureAlphaMod(texture: *mut SDL_Texture, alpha: Uint8) -> c_int;
    pub fn SDL_SetTextureColorMod(texture: *mut SDL_Texture, r: Uint8, g: Uint8, b: Uint8) -> c_int;
    pub fn SDL_LockTexture(texture: *mut SDL_Texture, rect: *const SDL_Rect, pixels: *mut *mut c_void, pitch: *mut c_int) -> c_int;
    pub fn SDL_UnlockTexture(texture: *mut SDL_Texture);

    // Surface
    pub fn SDL_CreateRGBSurfaceWithFormat(flags: Uint32, width: c_int, height: c_int, depth: c_int, format: Uint32) -> *mut SDL_Surface;
    pub fn SDL_CreateRGBSurfaceFrom(pixels: *mut c_void, width: c_int, height: c_int, depth: c_int, pitch: c_int, Rmask: Uint32, Gmask: Uint32, Bmask: Uint32, Amask: Uint32) -> *mut SDL_Surface;
    pub fn SDL_FreeSurface(surface: *mut SDL_Surface);
    pub fn SDL_BlitSurface(src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    pub fn SDL_BlitScaled(src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    pub fn SDL_FillRect(dst: *mut SDL_Surface, dstrect: *const SDL_Rect, color: Uint32) -> c_int;
    pub fn SDL_ConvertSurface(src: *mut SDL_Surface, fmt: *mut SDL_PixelFormat, flags: Uint32) -> *mut SDL_Surface;
    pub fn SDL_ConvertSurfaceFormat(src: *mut SDL_Surface, pixel_format: Uint32, flags: Uint32) -> *mut SDL_Surface;
    pub fn SDL_SetSurfaceBlendMode(surface: *mut SDL_Surface, blendMode: c_int) -> c_int;
    pub fn SDL_SetSurfaceRLE(surface: *mut SDL_Surface, flag: c_int) -> c_int;
    pub fn SDL_LockSurface(surface: *mut SDL_Surface) -> c_int;
    pub fn SDL_UnlockSurface(surface: *mut SDL_Surface);
    pub fn SDL_UpperBlit(src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;

    // 像素格式
    pub fn SDL_AllocFormat(format: Uint32) -> *mut SDL_PixelFormat;
    pub fn SDL_FreeFormat(format: *mut SDL_PixelFormat);
    pub fn SDL_MapRGB(format: *const SDL_PixelFormat, r: Uint8, g: Uint8, b: Uint8) -> Uint32;
    pub fn SDL_MapRGBA(format: *const SDL_PixelFormat, r: Uint8, g: Uint8, b: Uint8, a: Uint8) -> Uint32;
    pub fn SDL_GetRGB(pixel: Uint32, format: *const SDL_PixelFormat, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8);
    pub fn SDL_GetRGBA(pixel: Uint32, format: *const SDL_PixelFormat, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8, a: *mut Uint8);

    // 事件
    pub fn SDL_PollEvent(event: *mut SDL_Event) -> c_int;
    pub fn SDL_WaitEvent(event: *mut SDL_Event) -> c_int;
    pub fn SDL_PeepEvents(events: *mut SDL_Event, numevents: c_int, action: SDL_eventaction, minType: Uint32, maxType: Uint32) -> c_int;
    pub fn SDL_HasEvent(type_: Uint32) -> SDL_bool;
    pub fn SDL_FlushEvent(type_: Uint32);
    pub fn SDL_FlushEvents(minType: Uint32, maxType: Uint32);
    pub fn SDL_GetKeyboardState(numkeys: *mut c_int) -> *const Uint8;
    pub fn SDL_GetModState() -> Uint16;
    pub fn SDL_SetModState(modstate: Uint16);
    pub fn SDL_StartTextInput();
    pub fn SDL_StopTextInput();
    pub fn SDL_IsTextInputActive() -> SDL_bool;
    pub fn SDL_SetTextInputRect(rect: *mut SDL_Rect);
    pub fn SDL_HasScreenKeyboardSupport() -> SDL_bool;
    pub fn SDL_IsScreenKeyboardShown(window: *mut SDL_Window) -> SDL_bool;

    // 鼠标
    pub fn SDL_GetMouseState(x: *mut c_int, y: *mut c_int) -> Uint32;
    pub fn SDL_GetRelativeMouseState(x: *mut c_int, y: *mut c_int) -> Uint32;
    pub fn SDL_WarpMouseInWindow(window: *mut SDL_Window, x: c_int, y: c_int);
    pub fn SDL_SetRelativeMouseMode(enabled: SDL_bool) -> c_int;
    pub fn SDL_GetRelativeMouseMode() -> SDL_bool;
    pub fn SDL_ShowCursor(toggle: c_int) -> c_int;
    pub fn SDL_CreateCursor(data: *const Uint8, mask: *const Uint8, w: c_int, h: c_int, hot_x: c_int, hot_y: c_int) -> *mut SDL_Cursor;
    pub fn SDL_CreateColorCursor(surface: *mut SDL_Surface, hot_x: c_int, hot_y: c_int) -> *mut SDL_Cursor;
    pub fn SDL_CreateSystemCursor(id: SDL_SystemCursor) -> *mut SDL_Cursor;
    pub fn SDL_SetCursor(cursor: *mut SDL_Cursor);
    pub fn SDL_GetCursor() -> *mut SDL_Cursor;
    pub fn SDL_FreeCursor(cursor: *mut SDL_Cursor);
    pub fn SDL_GetDefaultCursor() -> *mut SDL_Cursor;

    // 剪贴板
    pub fn SDL_SetClipboardText(text: *const c_char) -> c_int;
    pub fn SDL_GetClipboardText() -> *const c_char;
    pub fn SDL_HasClipboardText() -> SDL_bool;

    // 定时器
    pub fn SDL_GetTicks() -> Uint32;
    pub fn SDL_GetPerformanceCounter() -> Uint64;
    pub fn SDL_GetPerformanceFrequency() -> Uint64;
    pub fn SDL_Delay(ms: Uint32);
    pub fn SDL_AddTimer(interval: Uint32, callback: Option<unsafe extern "C" fn(interval: Uint32, param: *mut c_void) -> Uint32>, param: *mut c_void) -> c_int;

    // 显示
    pub fn SDL_GetNumVideoDrivers() -> c_int;
    pub fn SDL_GetVideoDriver(index: c_int) -> *const c_char;
    pub fn SDL_GetCurrentVideoDriver() -> *const c_char;
    pub fn SDL_GetNumVideoDisplays() -> c_int;
    pub fn SDL_GetDisplayName(displayIndex: c_int) -> *const c_char;

    // 文件 I/O (SDL2 的 IOStream)
    pub fn SDL_IOFromFile(file: *const c_char, mode: *const c_char) -> *mut SDL_IOStream;
    pub fn SDL_IOFromMem(mem: *mut c_void, size: usize) -> *mut SDL_IOStream;
    pub fn SDL_IOFromConstMem(mem: *const c_void, size: usize) -> *mut SDL_IOStream;
    pub fn SDL_CloseIO(stream: *mut SDL_IOStream) -> SDL_bool;
    pub fn SDL_ReadIO(stream: *mut SDL_IOStream, ptr: *mut c_void, size: usize) -> usize;
    pub fn SDL_WriteIO(stream: *mut SDL_IOStream, ptr: *const c_void, size: usize) -> usize;
    pub fn SDL_SeekIO(stream: *mut SDL_IOStream, offset: Sint64, whence: c_int) -> Sint64;
    pub fn SDL_TellIO(stream: *mut SDL_IOStream) -> Sint64;
    pub fn SDL_GetIOSize(stream: *mut SDL_IOStream) -> Sint64;

    // 错误处理
    pub fn SDL_GetError() -> *const c_char;
    pub fn SDL_ClearError();
    pub fn SDL_SetError(fmt: *const c_char, ...) -> c_int;

    // 平台信息
    pub fn SDL_GetPlatform() -> *const c_char;
    pub fn SDL_GetRevision() -> *const c_char;

    // 游戏手柄
    pub fn SDL_NumJoysticks() -> c_int;
    pub fn SDL_IsGameController(index: c_int) -> SDL_bool;
    pub fn SDL_GameControllerOpen(index: c_int) -> *mut SDL_GameController;
    pub fn SDL_GameControllerClose(controller: *mut SDL_GameController);
    pub fn SDL_GameControllerName(controller: *mut SDL_GameController) -> *const c_char;
    pub fn SDL_GameControllerGetBindForAxis(controller: *mut SDL_GameController, axis: c_int) -> SDL_GameControllerButtonBind;
    pub fn SDL_GameControllerGetBindForButton(controller: *mut SDL_GameController, button: c_int) -> SDL_GameControllerButtonBind;
    pub fn SDL_JoystickOpen(device_index: c_int) -> *mut SDL_Joystick;
    pub fn SDL_JoystickClose(joystick: *mut SDL_Joystick);
    pub fn SDL_JoystickInstanceID(joystick: *mut SDL_Joystick) -> SDL_JoystickID;

    // 日志
    pub fn SDL_Log(fmt: *const c_char, ...);
    pub fn SDL_LogWarn(category: c_int, fmt: *const c_char, ...);
    pub fn SDL_LogError(category: c_int, fmt: *const c_char, ...);

    // 视频驱动枚举
    pub fn SDL_VideoInit(driver_name: *const c_char) -> c_int;
    pub fn SDL_VideoQuit();

    // 系统光标枚举
    pub fn SDL_GetSystemCursor(id: SDL_SystemCursor) -> *mut SDL_Cursor;
}

pub type SDL_GLattr = c_int;
pub const SDL_GL_RED_SIZE: SDL_GLattr = 0;
pub const SDL_GL_GREEN_SIZE: SDL_GLattr = 1;
pub const SDL_GL_BLUE_SIZE: SDL_GLattr = 2;
pub const SDL_GL_ALPHA_SIZE: SDL_GLattr = 3;
pub const SDL_GL_BUFFER_SIZE: SDL_GLattr = 4;
pub const SDL_GL_DOUBLEBUFFER: SDL_GLattr = 5;
pub const SDL_GL_DEPTH_SIZE: SDL_GLattr = 6;
pub const SDL_GL_STENCIL_SIZE: SDL_GLattr = 7;
pub const SDL_GL_ACCUM_RED_SIZE: SDL_GLattr = 8;
pub const SDL_GL_ACCUM_GREEN_SIZE: SDL_GLattr = 9;
pub const SDL_GL_ACCUM_BLUE_SIZE: SDL_GLattr = 10;
pub const SDL_GL_ACCUM_ALPHA_SIZE: SDL_GLattr = 11;
pub const SDL_GL_STEREO: SDL_GLattr = 12;
pub const SDL_GL_MULTISAMPLEBUFFERS: SDL_GLattr = 13;
pub const SDL_GL_MULTISAMPLESAMPLES: SDL_GLattr = 14;
pub const SDL_GL_ACCELERATED_VISUAL: SDL_GLattr = 15;
pub const SDL_GL_RETAINED_BACKING: SDL_GLattr = 16;
pub const SDL_GL_CONTEXT_MAJOR_VERSION: SDL_GLattr = 17;
pub const SDL_GL_CONTEXT_MINOR_VERSION: SDL_GLattr = 18;
pub const SDL_GL_CONTEXT_EGL: SDL_GLattr = 19;
pub const SDL_GL_CONTEXT_FLAGS: SDL_GLattr = 20;
pub const SDL_GL_CONTEXT_PROFILE_MASK: SDL_GLattr = 21;
pub const SDL_GL_SHARE_WITH_CURRENT_CONTEXT: SDL_GLattr = 22;
pub const SDL_GL_FRAMEBUFFER_SRGB_CAPABLE: SDL_GLattr = 23;
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR: SDL_GLattr = 24;
pub const SDL_GL_CONTEXT_RESET_NOTIFICATION: SDL_GLattr = 25;
pub const SDL_GL_CONTEXT_NO_ERROR: SDL_GLattr = 26;
pub const SDL_GL_FLOATBUFFERS: SDL_GLattr = 27;

pub const SDL_GL_CONTEXT_PROFILE_CORE: c_int = 0x0001;
pub const SDL_GL_CONTEXT_PROFILE_COMPATIBILITY: c_int = 0x0002;
pub const SDL_GL_CONTEXT_PROFILE_ES: c_int = 0x0004;

pub const SDL_GL_CONTEXT_DEBUG_FLAG: c_int = 0x0001;
pub const SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG: c_int = 0x0002;
pub const SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG: c_int = 0x0004;
pub const SDL_GL_CONTEXT_RESET_ISOLATION_FLAG: c_int = 0x0008;

pub type SDL_eventaction = c_int;
pub const SDL_ADDEVENT: SDL_eventaction = 0;
pub const SDL_PEEKEVENT: SDL_eventaction = 1;
pub const SDL_GETEVENT: SDL_eventaction = 2;

pub type SDL_SystemCursor = c_int;
pub const SDL_SYSTEM_CURSOR_ARROW: SDL_SystemCursor = 0;
pub const SDL_SYSTEM_CURSOR_IBEAM: SDL_SystemCursor = 1;
pub const SDL_SYSTEM_CURSOR_WAIT: SDL_SystemCursor = 2;
pub const SDL_SYSTEM_CURSOR_CROSSHAIR: SDL_SystemCursor = 3;
pub const SDL_SYSTEM_CURSOR_WAITARROW: SDL_SystemCursor = 4;
pub const SDL_SYSTEM_CURSOR_SIZENWSE: SDL_SystemCursor = 5;
pub const SDL_SYSTEM_CURSOR_SIZENESW: SDL_SystemCursor = 6;
pub const SDL_SYSTEM_CURSOR_SIZEWE: SDL_SystemCursor = 7;
pub const SDL_SYSTEM_CURSOR_SIZENS: SDL_SystemCursor = 8;
pub const SDL_SYSTEM_CURSOR_SIZEALL: SDL_SystemCursor = 9;
pub const SDL_SYSTEM_CURSOR_NO: SDL_SystemCursor = 10;
pub const SDL_SYSTEM_CURSOR_HAND: SDL_SystemCursor = 11;
pub const SDL_NUM_SYSTEM_CURSORS: SDL_SystemCursor = 12;

// 混合模式
pub const SDL_BLENDMODE_NONE: c_int = 0x00000000;
pub const SDL_BLENDMODE_BLEND: c_int = 0x00000001;
pub const SDL_BLENDMODE_ADD: c_int = 0x00000002;
pub const SDL_BLENDMODE_MOD: c_int = 0x00000004;
pub const SDL_BLENDMODE_MUL: c_int = 0x00000008;

pub type SDL_JoystickID = Sint32;

#[repr(C)]
pub struct SDL_GameControllerButtonBind {
    pub bindType: c_int,
    pub value: SDL_GameControllerButtonBindValue,
}

#[repr(C)]
pub union SDL_GameControllerButtonBindValue {
    pub button: c_int,
    pub axis: c_int,
    pub hat: std::mem::ManuallyDrop<SDL_GameControllerButtonBindHat>,
}

#[repr(C)]
pub struct SDL_GameControllerButtonBindHat {
    pub hat: c_int,
    pub hat_mask: c_int,
}

// SDL2_image
unsafe extern "C" {
    pub fn IMG_Init(flags: c_int) -> c_int;
    pub fn IMG_Quit();
    pub fn IMG_Load(file: *const c_char) -> *mut SDL_Surface;
    pub fn IMG_Load_IO(rwops: *mut SDL_IOStream, freesrc: c_int) -> *mut SDL_Surface;
    pub fn IMG_LoadTyped_IO(rwops: *mut SDL_IOStream, freesrc: c_int, type_: *const c_char) -> *mut SDL_Surface;
}
