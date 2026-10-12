# 菜单栏最小版本

## 当前决定（2026-10-12）
- 用户明确拒绝上一版手动提醒界面，要求先只做顶部菜单栏小图标，没有其他功能，并做好版本管理。
- 用户随后明确要求图标作为应用入口并支持退出；当前水滴图标的原生菜单只有“退出咕噜”，tooltip 为咕噜。
- 用户询问图标变化、闪烁和从图标弹出内容的可能性；当前只完成退出菜单，这些效果尚未实施。

## 实现证据
- [lib.rs](../src-tauri/src/lib.rs:7)：setup 内创建退出 MenuItem 与 Menu，绑定到 TrayIconBuilder；启用左键打开菜单，右键也可打开系统托盘菜单。on_menu_event 检查 quit ID 后调用 app.exit(0) 请求结束应用；没有 command 或计算器状态。
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
- 退出菜单改动后 cargo fmt --check、cargo check --offline、CARGO_NET_OFFLINE=true pnpm tauri build --debug --bundles app 通过。未独立实测点击退出；不重复使用此前长时间超时的原生绑定验证。
- 未添加镜像实现的单元测试；菜单行为通过类型检查、打包和官方/本机 API 核对验证，UI 实测仍未确认。

## 图标表现能力（已核对 API，效果未实施）
- 本机 tauri-2.12.2/src/tray/mod.rs 的 set_icon 支持运行时替换图片，rect 支持获取菜单栏图标位置；可保留模板图标，或按需要更改模板状态。
- 可行方案推断：切换不同图片帧模拟眨眼/晃动；交替普通帧与透明帧模拟闪烁。未测量动画帧率或系统表现，不自动隐藏/移除整个状态栏项。
- 从图标边缘“吐出”气泡或小水滴，可以用贴着图标定位的小窗口渲染动画，视觉上模拟连贯动作；这会新增窗口及可能的前端，尚未决定具体路线。
- macOS 原生 NSPopover 支持锚定到视图并显示额外内容；这是一条 AppKit 原生路线，不是当前 Tauri 代码已经接入的能力。
- 若以后选择 Tauri 透明 WebView 窗口，需要单独确认 macOS 特性配置；当前不为能力探查添加窗口或依赖。

## 官方证据
- Tauri 托盘 API：https://v2.tauri.app/learn/system-tray/ 。
- TrayIcon 图片替换与位置：https://docs.rs/tauri/latest/tauri/tray/struct.TrayIcon.html （网页当前是 2.12.3；项目仍是 2.12.2，相关 API 已核对本机源码）。
- AppKit 弹出内容：https://developer.apple.com/documentation/appkit/nspopover 。Apple 页面提供的 Markdown 通过只读 HTTP 获取并核对。
- macOS agent 标记：https://developer.apple.com/documentation/bundleresources/information-property-list/lsuielement 。
