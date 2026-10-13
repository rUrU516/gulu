# 咕噜 / Gulu

Rust + Tauri 的 macOS 菜单栏喝水提醒，面向 Apple 芯片 Mac。

启动后常驻顶部菜单栏，每 5 分钟从水滴图标的位置弹出“记得喝水”，约 4.8 秒后自动缩回。菜单提供“试试提醒”“检查更新”和“退出”。

## 安装与更新

从 [GitHub Releases](https://github.com/rUrU516/gulu/releases/latest) 下载 `.dmg`，打开后将 `gulu` 拖入“应用程序”，再启动。无需安装 Rust 或 pnpm。

菜单中的“检查更新”会比较最新发布版本，有新版时提供下载页面。安装新版前先选择“退出”，再用新版替换旧应用并启动。

目前使用本地签名，未经 Apple 公证。首次打开如果提示开发者未验证，可参考 [Apple 官方说明](https://support.apple.com/zh-cn/102445)。

## 开发

本机需有 Rust、macOS 开发工具和 pnpm。执行 `pnpm install`，再执行 `pnpm tauri dev`。

发布时将 `package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json` 的版本号保持一致，运行 `cargo check --manifest-path src-tauri/Cargo.toml` 同步锁定文件，再将改动提交到 Git，执行 `pnpm release`。脚本会构建并校验 Apple 芯片安装包，推送代码和版本标签，再上传并发布 GitHub Release。需要本机 `gh` 已登录，并已设置 Git 远端。

架构、学习背景和详细版本记录见 [agent-context/index.md](agent-context/index.md)。
