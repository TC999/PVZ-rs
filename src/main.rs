// PvZ Portable Rust 翻译 — 主入口
//
// 对应 C++ 的 main.cpp

// 游戏逻辑串联阶段允许这些 lint（FFI、未串联的桩代码）
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(rust_2024_compatibility)]

mod ffi;
mod framework;
mod todlib;
mod lawn;

use lawn::LawnApp;

fn main() {
    // 创建 LawnApp
    let mut app = LawnApp::new();

    // 初始化 SDL、创建窗口、初始化 OpenGL（通过 SexyAppBase）
    app.init();

    // 启动主循环（会阻塞直到窗口关闭）
    app.start();

    // 清理资源
    app.shutdown();
}
