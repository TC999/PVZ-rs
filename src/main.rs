// PvZ Portable Rust 翻译 — 主入口
//
// 对应 C++ 的 main.cpp

mod ffi;
mod framework;
mod todlib;
mod lawn;

use lawn::LawnApp;
use framework::sexy_app_base::SexyAppBase;

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
