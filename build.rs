// PvZ Portable Rust 翻译 — 构建脚本
//
// SDL2 通过 raw-dylib 链接（在 sdl2.rs 中用 #[link(kind = "raw-dylib")]）
// OpenGL 通过 raw-dylib 链接 opengl32.dll（在 opengl.rs 中用 #[link(kind = "raw-dylib")]）
// 无需导入库 .lib 文件

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/ffi/sdl2.rs");
    println!("cargo:rerun-if-changed=src/ffi/opengl.rs");
}
