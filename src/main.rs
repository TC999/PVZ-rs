// PvZ Portable Rust 翻译 — 主入口
//
// 对应 C++ 的 main.cpp

mod ffi;
mod framework;
mod todlib;
mod lawn;

use lawn::LawnApp;
use todlib::TodStringFile;

fn main() {
    // 设置字符串颜色
    // TodStringListSetColors(gLawnStringFormats, gLawnStringFormatCount);
    // 在 todlib 模块中处理

    // 全局函数指针（原 C++ 中通过函数指针注册）
    // gGetCurrentLevelName = LawnGetCurrentLevelName;
    // gAppCloseRequest = LawnGetCloseRequest;
    // gAppHasUsedCheatKeys = LawnHasUsedCheatKeys;
    // gExtractResourcesByName = Sexy::ExtractResourcesByName;

    // 创建 LawnApp
    let mut app = LawnApp::new();

    // 初始化并运行
    app.init();
    app.start();
    app.shutdown();
}
