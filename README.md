<p align="center">
  <img src="public/labeldroid.svg" width="100%" alt="LabelDroid Logo"/>
</p>

# LabelDroid

**LabelDroid** 是一个基于 [Tauri](https://tauri.app/) + [Vue 3](https://v3.vuejs.org/) + [TypeScript](https://www.typescriptlang.org/) 构建的现代化跨平台图像标注工具。其旨在提供类似 **LabelMe** 的标注体验，同时借助于 Rust 提供的高效本地计算和 Tauri 带来的极小打包体积，支持在 Desktop 和 Android 平台上运行。

## ✨ 特性

- 🖥️ **跨平台支持**: 提供 Desktop (Windows/macOS/Linux) 以及 Mobile (Android) 应用的支持。
- ⚡ **高性能底座**: 后端使用 Rust 编写，结合内嵌的本地 HTTP 服务（提供图片与数据供给），确保大规模图像浏览及数据处理的流畅性。
- 🎨 **现代化 UI**: 界面使用 Vue 3 + [Element Plus](https://element-plus.org/) 构建，通过 Canvas 实现图像的缩放、平移和多边形 (Polygon) 等形状绘制。
- 📦 **LabelMe 兼容**: 能够读取和生成兼容 LabelMe 格式的 JSON 标注数据。
- 🧩 **Node 绑定**: `bindings/node` 提供了可在 Node.js 生态中直接调用的 Rust 组件/功能支持。

## 🛠️ 技术栈

- **前端**: Vue 3 (`<script setup>`), TypeScript, Vite, Element Plus
- **后端/桌面端**: Rust, Tauri 2.0
- **交互渲染**: HTML5 Canvas

## 📂 项目结构

```
labeldroid/
├── bindings/          # Rust 到 Node.js 等其他环境的绑定 (WASI/NAPI)
├── public/            # 前端静态资源
├── renderer/          # 与图形渲染相关的 Vue 3 前端代码 (CanvasArea, Sidebar 等)
├── share/             # Rust 核心共享模块 (如本地 HTTP 文件服务、配置管理)
├── src-tauri/         # Tauri 主应用逻辑设定、Android Gradle 构建配置
├── types/             # 公共使用到的结构体和类型定义 (Rust端)
└── package.json       # Node.js 环境及包管理信息
```

## 🚀 快速开始

### 推荐的 IDE 环境

- [VS Code](https://code.visualstudio.com/) 
- 插件: [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### 环境依赖

- **Node.js**: (推荐 >= 18) 与 **Yarn** (`yarn@4.13.0`)
- **Rust**: 保证已安装 `cargo` 并配置相关工具链配置
- 开发 Android 端需要配置好相应的 **Android SDK** 与 **NDK**

### 运行脚本

```bash
# 1. 安装依赖
yarn install

# 2. 启动桌面端开发模式
yarn tauri dev

# 3. 编译构建
yarn build
yarn tauri build
```基于 [MIT License](LICENSE) 协议开源

## 📄 协议

本项目相关源码遵守开源协议。