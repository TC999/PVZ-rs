# PVZ-rs

一个使用 Rust 实现的植物大战僵尸年度版（ GOTY 版），基于[PvZ-Portable](https://github.com/wszqkzqk/PvZ-Portable)翻译

## 使用方法：
1. 从[GitHub Actions](https://github.com/TC999/PVZ-rs/actions)下载 CI 产物
3. 从[此处](https://lib.openmpt.org/files/libopenmpt/dev/)下载带`windows`的版本，提取`bin/架构代号/libopenmpt.dll`
4. 把上述文件全部复制到原版游戏目录下

## 编译方法
1. 安装 Rust 和 Git
2. 克隆项目
```bash
  git clone https://github.com/TC999/PVZ-rs.git
```
3. 安装依赖（仅限 Linux）
- Debian/Ubuntu
```bash
  sudo apt-get install -y libsdl2-dev libsdl2-mixer-dev
```
4. 编译
```bash
  cd PVZ-rs
  cargo build --release
```

## ⚠️ 开发者叠甲时间(关于本项目含硅量的声明)
> **太长不看版**: 没错,这是一个纯正的 Vibe-coded(**凭感觉编程**) 项目!
>
> **坦白局**: 本项目含有大量由 LLM 辅助生成的底层代码。如果你在翻看 commit 记录时闻到了一股浓浓的"AI 味",自信点,你的直觉非常准确。
>
> 我的核心目的其实非常纯粹：**快速将原始 C++ 代码翻译为 Rust 代码**，并且摆脱 C++ 的限制，充分利用 Rust 的安全性和性能优势。为了达成目标，什么所有权、各种细节问题就全部交给 AI 来处理。
>
> 主打一个"代码虽然抽象,但它在现代机器上真能跑"。🛠️