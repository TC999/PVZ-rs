// PvZ Portable Rust 翻译 — 主入口
//
// 对应 C++ 的 main.cpp

// 游戏逻辑串联阶段允许这些 lint（FFI、未串联的桩代码）
//#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(unused_variables)]
//#![allow(non_upper_case_globals)]
//#![allow(non_snake_case)]
//#![allow(unsafe_op_in_unsafe_fn)]
#![allow(rust_2024_compatibility)]

mod ffi;
mod framework;
mod todlib;
mod lawn;

use lawn::LawnApp;
use todlib::init_lawn_string_formats;

fn main() {
    // 初始化字符串格式（对应 C++ TodStringListSetColors(gLawnStringFormats, gLawnStringFormatCount)）
    init_lawn_string_formats();

    // 创建 LawnApp
    let mut app = LawnApp::new();

    // 传递命令行参数（对应 C++ gLawnApp->SetArgs(argc, argv)）
    let args: Vec<String> = std::env::args().collect();
    let mut arg_cstrings: Vec<std::ffi::CString> = args.iter()
        .map(|s| std::ffi::CString::new(s.as_str()).unwrap_or_default())
        .collect();
    let mut arg_ptrs: Vec<*mut std::os::raw::c_char> = arg_cstrings.iter_mut()
        .map(|cs| cs.as_ptr() as *mut std::os::raw::c_char)
        .collect();
    app.base.set_args(arg_ptrs.len() as i32, arg_ptrs.as_mut_ptr());

    // 解析命令行参数（对应 C++ SexyAppBase::DoParseCmdLine，SexyAppBase.cpp:3258-3290）
    // C++ 在遍历中直接调虚函数 HandleCmdLineParam（LawnApp 覆写优先于基类）；
    // Rust 无虚分派，故这里显式分派给 LawnApp（其内部再转基类），最后统一收尾解析 demo 文件。
    for (a_param_name, a_param_value) in
        framework::sexy_app_base::SexyAppBase::parse_cmd_line_params(&args)
    {
        app.handle_cmd_line_param(&a_param_name, &a_param_value);
    }
    app.base.finalize_cmd_line();

    // 初始化 SDL、创建窗口、初始化 OpenGL（通过 SexyAppBase）
    app.init();

    // 启动主循环（会阻塞直到窗口关闭）
    app.start();

    // 清理资源
    app.shutdown();
}
