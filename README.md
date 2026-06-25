# PVZ-rs

一个使用 Rust 实现的植物大战僵尸年度版（ GOTY 版），基于[PvZ-Portable](https://github.com/wszqkzqk/PvZ-Portable)翻译

## 使用方法：
1. 从[GitHub Actions](https://github.com/TC999/PVZ-rs/actions)下载 CI 产物
2. 下载[这个文件](https://github.com/libsdl-org/SDL_mixer/releases/download/release-2.8.1/SDL2_mixer-devel-2.8.1-VC.zip)，提取`bin/架构代号/SDL2_mixer.dll`
3. 从[此处](https://lib.openmpt.org/files/libopenmpt/dev/)下载带`windows`的版本，提取`bin/架构代号/libopenmpt.dll`
4. 把上述文件全部复制到原版游戏目录下

## 编译方法
1. 安装 Rust 和 Git
2. 克隆项目
```bash
  git clone https://github.com/TC999/PVZ-rs.git
```
3. 安装依赖（仅限 Linux）
Debian/Ubuntu
```bash
  sudo apt-get install -y libsdl2-dev libsdl2-mixer-dev
```
4. 编译
```bash
  cd PVZ-rs
  cargo build --release
```
