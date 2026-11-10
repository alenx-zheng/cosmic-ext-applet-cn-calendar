# Cosmic Ext Applet: CN Calendar (中国农历日历)

一个专为 [COSMIC™ 桌面环境](https://system76.com/cosmic) 开发的轻量级、高颜值的日历 Applet 插件。除了基础的时间显示，它还深度集成了中国农历、法定节假日调休标记以及本地化时间格式。

## 🌟 核心特性

- **基础时间显示**：支持在顶栏显示完整的日期与时间，支持 12/24 小时制切换，并可选择是否显示秒数。
- **中国农历集成**：可选在顶栏及日历弹窗中显示农历日期、传统节日（如春节、中秋）及二十四节气。
- **法定节假日与调休标记**：自动在日历网格中标记“休”（节假日）与“班”（调休工作日），使用醒目的右上角角标展示，包含 2026 及 2027 年官方安排。
- **相对日期计算**：选择非当天的日期时，会自动显示“xx天后”或“xx天前”，方便规划日程。
- **高度可定制**：弹窗内提供设置面板，可自由开关农历、秒数、节假日标记、24小时制等功能。
- **多语言支持**：界面与日期格式无缝接入系统本地化（i18n）。
- **完美的 UI 适配**：自适应面板尺寸（Autosize），无论水平还是垂直面板，均能完美撑开并展示完整信息，周末日期自动灰度显示。

## 📸 界面预览

|                 完整农历与节假日展示                 |                 相对日期计算与节假日角标                 |                 顶栏简洁模式                 |
| :--------------------------------------------------: | :------------------------------------------------------: | :------------------------------------------: |
| ![完整农历与节假日展示](resources/screenshots/1.png) | ![相对日期计算与节假日角标](resources/screenshots/2.png) | ![顶栏简洁模式](resources/screenshots/3.png) |

## 🛠️ 编译与安装

### 前置依赖

在编译此项目之前，请确保你的系统已经安装了 Rust 环境以及 COSMIC 桌面环境所需的编译依赖：

**For Ubuntu / Pop!\_OS:**

```bash
sudo apt update
sudo apt install build-essential git just pkg-config libxkbcommon-dev libwayland-dev libssl-dev
```

**For Arch linux**

```bash
sudo pacman -S base-devel git just pkg-config libxkbcommon wayland openssl
```
