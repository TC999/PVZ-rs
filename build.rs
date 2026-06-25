// PvZ Portable Rust — build.rs
// 为 SDL2_mixer 提供链接支持
// Windows: 自动下载 SDL2_mixer 开发包，根据目标架构提取对应 .lib 并复制 .dll
// Linux: sdl2-sys 的 mixer 特性会自动输出 -lSDL2_mixer，系统安装 libsdl2-mixer-dev 即可

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(target_os = "windows")]
    setup_sdl2_mixer_windows();
}

#[cfg(target_os = "windows")]
fn setup_sdl2_mixer_windows() {
    use std::path::PathBuf;
    use std::{env, fs};

    let proj = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target = env::var("TARGET").unwrap_or_default();
    let version = "2.6.3";

    // 根据目标架构选择 x64 或 x86
    let arch_dir = if target.starts_with("x86_64") {
        "x64"
    } else if target.starts_with("i686") {
        "x86"
    } else {
        println!("cargo:warning=不支持的架构: {}，跳过 SDL2_mixer 链接", target);
        return;
    };

    let sdk_dir = proj.join(format!("SDL2_mixer-{}", version));
    let dest = sdk_dir.join("lib").join(arch_dir);
    let done_file = sdk_dir.join(format!(".done_{}", arch_dir));

    if !done_file.exists() || !dest.join("SDL2_mixer.lib").exists() {
        // 下载并提取
        println!("cargo:warning=正在下载 SDL2_mixer 开发包 ({})...", arch_dir);
        download_and_extract(version, &dest, &done_file, arch_dir);
    }

    if dest.join("SDL2_mixer.lib").exists() {
        println!("cargo:rustc-link-search={}", dest.display());
        println!("cargo:warning=SDL2_mixer .lib: {}\\SDL2_mixer.lib ({})", dest.display(), arch_dir);

        // 复制 DLL
        let dll_src = dest.join("SDL2_mixer.dll");
        if dll_src.exists() {
            // 从 CARGO_MANIFEST_DIR 向上找 target/debug/ 或 target/release/
            let target_root = proj.parent().map(|p| p.join("target"));
            if let Some(ref target_dir) = target_root {
                for sub in &["debug", "release"] {
                    let dll_dst = target_dir.join(sub).join("SDL2_mixer.dll");
                    if let Some(parent) = dll_dst.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    if fs::copy(&dll_src, &dll_dst).is_ok() {
                        println!("cargo:warning=SDL2_mixer.dll -> {}", dll_dst.display());
                    };
                }
            }
        }
    } else {
        println!("cargo:warning=SDL2_mixer.lib 未找到，请手动下载开发包到:");
        println!("cargo:warning=  {}", dest.display());
        println!("cargo:warning=下载地址: https://github.com/libsdl-org/SDL_mixer/releases");
    }
}

#[cfg(target_os = "windows")]
fn download_and_extract(version: &str, arch_dir: &std::path::Path, done_file: &std::path::Path, arch_label: &str) {
    use std::process::Command;

    // 下载到项目 SDL2_mixer-{version}/ 目录
    let sdk_dir = arch_dir.parent().unwrap().parent().unwrap(); // lib 的父父目录
    let zip_path = sdk_dir.join("SDL2_mixer.zip");
    let extract_dir = sdk_dir.join("_extract");
    let url = format!(
        "https://github.com/libsdl-org/SDL_mixer/releases/download/release-{v}/SDL2_mixer-devel-{v}-VC.zip",
        v = version
    );

    // 清理旧临时目录
    let _ = std::fs::remove_dir_all(&extract_dir);

    // 确保 zip 父目录存在
    let _ = std::fs::create_dir_all(sdk_dir);

    // 如果 zip 已存在不必重新下载
    if zip_path.exists() {
        println!("cargo:warning=使用已有开发包: {}", zip_path.display());
    } else {
        println!("cargo:warning=正在下载 SDL2_mixer 开发包 ({})...", arch_label);
    let dl_ok = Command::new("powershell")
        .args(["-NoProfile", "-Command", &format!(
            "Invoke-WebRequest -Uri '{}' -OutFile '{}' -ErrorAction Stop",
            url, zip_path.to_string_lossy()
        )])
        .status().map(|s| s.success()).unwrap_or(false);

    if !dl_ok {
        println!("cargo:warning=下载失败 (1/2)，尝试 curl...");
        let curl_ok = Command::new("curl")
            .args(["-L", "-o", &zip_path.to_string_lossy(), &url])
            .status().map(|s| s.success()).unwrap_or(false);
        if !curl_ok {
            println!("cargo:warning=下载失败，请手动下载到: {}", zip_path.display());
            return;
        }
    }
    }

    // 解压
    let unzip_ok = Command::new("powershell")
        .args(["-NoProfile", "-Command", &format!(
            "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
            zip_path.to_string_lossy(), extract_dir.to_string_lossy()
        )])
        .status().map(|s| s.success()).unwrap_or(false);

    if !unzip_ok {
        println!("cargo:warning=解压失败");
        let _ = std::fs::remove_file(&zip_path);
        return;
    }

    // 找到 SDK 目录并拷贝到目标
    let mut copied = false;
    if let Ok(entries) = std::fs::read_dir(&extract_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let lib_dir = path.join("lib").join(arch_label);
                    if lib_dir.join("SDL2_mixer.lib").exists() {
                        // 拷贝对应架构目录下的内容
                        let _ = std::fs::create_dir_all(arch_dir);
                        if let Ok(lib_entries) = std::fs::read_dir(&lib_dir) {
                            for le in lib_entries {
                                if let Ok(le) = le {
                                    let _ = std::fs::copy(&le.path(), &arch_dir.join(le.file_name()));
                                }
                            }
                        }
                        copied = true;
                        println!("cargo:warning=已提取 SDL2_mixer {} 开发包", arch_label);
                        break;
                    }
                }
            }
        }
    }

    if !copied {
        // fallback: 遍历查找
        if let Ok(entries) = std::fs::read_dir(&extract_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(files) = find_files(&path, "SDL2_mixer.lib") {
                            for f in files {
                                if let Some(parent) = f.parent() {
                                    let parent_str = parent.to_string_lossy().to_lowercase();
                                    let matches = parent_str.contains(&arch_label.to_lowercase());
                                    if matches || !copied {
                                        let _ = std::fs::create_dir_all(arch_dir);
                                        if let Ok(dentries) = std::fs::read_dir(parent) {
                                            for de in dentries {
                                                if let Ok(de) = de {
                                                    let _ = std::fs::copy(&de.path(), &arch_dir.join(de.file_name()));
                                                }
                                            }
                                        }
                                        copied = true;
                                    }
                                }
                                if copied { break; }
                            }
                        }
                        if copied { break; }
                    }
                }
            }
        }
        if copied {
            println!("cargo:warning=已提取 SDL2_mixer 开发包 (fallback)");
        }
    }

    // 清理临时目录
    let _ = std::fs::remove_dir_all(&extract_dir);
    let _ = std::fs::write(done_file, b"ok");

    if !copied {
        println!("cargo:warning=提取后未找到 SDL2_mixer.lib，请手动处理");
    }
}

#[cfg(target_os = "windows")]
fn find_files(dir: &std::path::Path, name: &str) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut results = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                results.extend(find_files(&path, name)?);
            } else if path.file_name().and_then(|n| n.to_str()) == Some(name) {
                results.push(path);
            }
        }
    }
    Ok(results)
}
