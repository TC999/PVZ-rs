// PvZ Portable Rust 翻译 — 构建脚本
//
// SDL2 和 OpenGL 通过 sdl2 crate 管理链接，不再使用 raw-dylib。

fn main() {
    // 解决新版 CMake 与 SDL2 源码 CMakeLists.txt 版本兼容性问题
    println!("cargo:rustc-env=CMAKE_POLICY_VERSION_MINIMUM=3.5");

    println!("cargo:rerun-if-changed=build.rs");
}
