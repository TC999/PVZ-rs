// PvZ Portable Rust 翻译 — 构建脚本
// 用于链接 SDL2、OpenGL 和编译 glad 加载器

use std::path::Path;
use std::process::Command;

fn main() {
    // ---- 链接 SDL2 ----
    let sdl2_lib = Path::new("SDL2.lib");
    if sdl2_lib.exists() {
        let dir = std::env::current_dir().unwrap();
        println!("cargo:rustc-link-search={}", dir.display());
        println!("cargo:rustc-link-lib=SDL2");
    } else {
        println!("cargo:rustc-link-lib=dylib=SDL2");
    }

    // ---- 链接 OpenGL ----
    // Windows: opengl32.lib 包含 glViewport, glClear 等
    println!("cargo:rustc-link-lib=dylib=opengl32");

    // ---- 编译 glad 加载器 C 代码 ----
    // gladLoadGLES2 需要从 C 代码编译提供
    if Path::new("glad_loader.c").exists() {
        // 查找 MSVC 编译器
        if let Some(cl) = find_msvc_tool("cl.exe") {
            let status = Command::new(&cl)
                .args([
                    "/nologo",
                    "/c",
                    "/Fo:glad_loader.obj",
                    "glad_loader.c",
                ])
                .status()
                .expect("无法运行 MSVC 编译器");

            if status.success() {
                // 创建静态库
                if let Some(lib) = find_msvc_tool("lib.exe") {
                    Command::new(&lib)
                        .args([
                            "/nologo",
                            "/out:glad_loader.lib",
                            "glad_loader.obj",
                        ])
                        .status()
                        .ok();
                    println!("cargo:rustc-link-lib=glad_loader");
                }
            }
        } else {
            // 尝试使用 cc 或直接链接
            println!("cargo:warning=未找到 MSVC 编译器，尝试使用已有的 glad_loader.lib");
        }
    }

    // 如果 SDL2.lib 不存在，从 DLL 生成
    if !sdl2_lib.exists() {
        if let (Some(dumpbin), Some(lib_exe)) = (find_msvc_tool("dumpbin.exe"), find_msvc_tool("lib.exe")) {
            generate_sdl2_import_lib(&dumpbin, &lib_exe);
        }
    }

    // 重新运行条件
    println!("cargo:rerun-if-changed=glad_loader.c");
    println!("cargo:rerun-if-changed=SDL2.dll");
}

fn find_msvc_tool(name: &str) -> Option<String> {
    // 检查 PATH
    if let Ok(path) = std::env::var("PATH") {
        for p in path.split(';') {
            let full = format!("{}\\{}", p, name);
            if Path::new(&full).exists() {
                return Some(full);
            }
        }
    }
    // 尝试 VS 工具链路径
    let vs_paths = [
        r"D:\Software\Visual Studio\VC\Tools\MSVC\14.41.34120\bin\Hostx64\x64",
        r"D:\Software\Visual Studio\VC\Tools\MSVC\14.40.33807\bin\Hostx64\x64",
        r"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.41.34120\bin\Hostx64\x64",
        r"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.40.33807\bin\Hostx64\x64",
    ];
    for base in &vs_paths {
        let full = format!("{}\\{}", base, name);
        if Path::new(&full).exists() {
            return Some(full);
        }
    }
    None
}

fn generate_sdl2_import_lib(dumpbin: &str, lib_exe: &str) {
    let output = Command::new(dumpbin)
        .args(["/EXPORTS", "SDL2.dll"])
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut def = String::from("EXPORTS\n");

        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with(|c: char| c.is_ascii_digit()) {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 4 {
                    def.push_str(&format!("{}\n", parts[3]));
                }
            }
        }

        std::fs::write("SDL2.def", &def).ok();

        Command::new(lib_exe)
            .args(["/def:SDL2.def", "/out:SDL2.lib", "/machine:x64"])
            .status()
            .ok();
    }
}
