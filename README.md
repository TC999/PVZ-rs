# PVZ-rs

一个使用 Rust 实现的植物大战僵尸年度版（ GOTY 版），基于[PvZ-Portable](https://github.com/wszqkzqk/PvZ-Portable)

## 为什么要重写？

原版《植物大战僵尸》虽然经典，但毕竟是十几年前的游戏。原版使用了 DirectX 7 等过时技术，且只提供了 32 位可执行文件，在现代操作系统上运行可能会遇到兼容性问题，而且它从未官方支持过 Linux，也没有适配过 RISC-V, 龙芯的 LoongArch 等新兴指令集架构。

在开源社区，已经有前人基于逆向工程得到的文档和社区研究，重写出了最初的游戏引擎。PvZ-Portable 在此基础上进一步开发，提供了跨平台移植方案，但 C++ 项目配置繁琐且浪费时间。

本项目则是以此为基础将使用 Rust 重写。目前实现了：
- Windows，Linux 支持
- 标题界面加载

## ⚠️ 版权与使用说明

**重要：本项目仅包含代码引擎，不包含任何游戏素材！**

PVZ-rs 严格遵守版权协议。游戏的 IP（植物大战僵尸）属于 PopCap/EA。

要研究或使用此项目，你**必须**拥有正版游戏（如果没有，请在 [Steam][steam-link] 或 [EA 官网][ea-link] 上购买）。你需要从正版游戏中提取以下文件放到 PvZ-rs 的程序所在目录中：
```bash
main.pak
properties/ 目录
```
本项目仅提供引擎代码，用于技术学习，不包含上述任何游戏资源文件，任何游戏资源均需要用户自行提供正版游戏文件。

本项目的源代码以 [GPL-3.0][gpl3] 许可证开源，欢迎学习和贡献。

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

## 致谢

这个项目站在了巨人的肩膀上。

- 特别感谢[wszqkzqk][wszqkzqk]对原始代码进行跨平台适配！
- 感谢 SDL 开发团队提供的强大跨平台库！
- 感谢所有为游戏研究做出贡献的社区成员！
- 感谢宝开创造了这个经典游戏，并使用宽松的 [PopCap Games Framework License](https://github.com/wszqkzqk/PvZ-Portable/blob/main/src/SexyAppFramework/LICENSE) 开放 `SexyAppFramework`，让开源社区能够更方便地研究这个游戏引擎！
- 感谢 DeepSeek 提供的廉价大模型！

## ⚠️ 开发者叠甲时间(关于本项目含硅量的声明)
> **太长不看版**: 没错,这是一个纯正的 Vibe-coded(**凭感觉编程**) 项目!
>
> **坦白局**: 本项目含有大量由 LLM 辅助生成的底层代码。如果你在翻看 commit 记录时闻到了一股浓浓的"AI 味",自信点,你的直觉非常准确。
>
> 我的核心目的其实非常纯粹：**快速将原始 C++ 代码翻译为 Rust 代码**，并且摆脱 C++ 的限制，充分利用 Rust 的安全性和性能优势。为了达成目标，什么所有权、各种细节问题就全部交给 AI 来处理。
>
> 主打一个"代码虽然抽象,但它在现代机器上真能跑"。🛠️

[gpl3]: https://www.gnu.org/licenses/gpl-3.0.en.html
[steam-link]: https://store.steampowered.com/app/3590/Plants_vs_Zombies_GOTY_Edition/
[ea-link]: https://www.ea.com/games/plants-vs-zombies/plants-vs-zombies

[wszqkzqk]: https://github.com/wszqkzqk