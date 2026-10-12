# 菜单栏最小版本

## 当前决定（2026-10-12）
- 用户明确拒绝上一版手动提醒界面，要求先只做顶部菜单栏小图标，没有其他功能，并做好版本管理。
- 当前只创建一个水滴小图标，tooltip 为咕噜，无菜单或点击处理；无需窗口、前端页面、计时或通知。

## 实现证据
- [lib.rs](../src-tauri/src/lib.rs:4)：setup 内创建 TrayIconBuilder，Tauri 保存图标并进入运行循环；没有 command 或计算器状态。
- [tauri.conf.json](../src-tauri/tauri.conf.json)：app.windows 为空；build 为空，无 Vite/devUrl/前端构建钩子。
- [Cargo.toml](../src-tauri/Cargo.toml)：仅依赖 Tauri，启用 tray-icon；PNG 用 include_image! 在编译时转换为像素。
- [tray.svg](../src-tauri/icons/tray.svg) 为原创矢量水滴图标，[tray.png](../src-tauri/icons/tray.png) 是透明 36×36 位图，由 bundled sharp 从 SVG 渲染。模板图标交给 macOS 根据菜单栏外观着色。
- macOS 设置 Accessory 激活策略；[Info.plist](../src-tauri/Info.plist) 指定 LSUIElement=true，打包应用不显示 Dock 图标。
- 原计算器/提醒的源码、前端资源、权限和相关直接依赖从当前版本移除，保存在 learning-baseline。

## 已验证与限制
- pnpm install --offline --lockfile-only 成功，Node 锁文件只保留 CLI 依赖；Rust 锁文件随 cargo check 更新。
- cargo check --offline、cargo fmt --check、git diff --check 通过。
- pnpm tauri build --debug --bundles app 成功生成 src-tauri/target/debug/bundle/macos/gulu.app；已读出打包 Info.plist 的 LSUIElement=true、Identifier=com.zhuoru.gulu、版本 0.1.0。
- 图标文件为 RGBA 36×36，有透明背景和非空 alpha 区域。
- 使用原生自动化绑定打包应用时工具超时，无法独立确认菜单栏实际显示或深浅色效果；不把编译/打包成功记成 UI 实测。绑定操作可能已启动应用，未重复启动。
- 没有为仅图标版本添加单元测试：当前行为通过编译、打包、配置和资源检查验证。

## 官方证据
- Tauri 托盘 API：https://v2.tauri.app/learn/system-tray/ 。
- macOS agent 标记：https://developer.apple.com/documentation/bundleresources/information-property-list/lsuielement 。
