# 当前架构：菜单栏入口与原生水滴动画

## 已验证的代码路径
- package.json 的 tauri/dev 脚本调用 Tauri CLI；没有前端开发服务器。
- [main.rs](../src-tauri/src/main.rs:4) 调用 gulu_lib::run。
- [lib.rs](../src-tauri/src/lib.rs:10) 创建 Tauri Builder，在 setup 回调中设置 macOS 激活策略、创建“试试提醒”/“退出”菜单并绑定菜单栏图标，然后 run 进入应用运行循环。
- [tauri.conf.json](../src-tauri/tauri.conf.json) 的 windows 为空，应用启动不创建 WebView 窗口。
- 仅原生核心运行：无前端、command、invoke 或 Mutex 状态。动画通过 Rust 的 objc2 绑定直接调用 AppKit；没有自动喝水提醒周期。

## 图标与生命周期
- TrayIconBuilder::with_id("gulu") 创建一个图标，使用 include_image! 编译时载入 PNG，icon_as_template(true) 交给 macOS 着色。
- MenuItem::with_id 创建 preview/quit 项，Menu::with_items 组成菜单，menu(&menu) 绑定到图标；on_menu_event 接收 AppHandle 和 MenuEvent，检查 ID 并调用 reminder::play 或 app.exit(0)。菜单事件经 Tauri 直接调用 Rust 闭包，没有前端通信。
- 已核对本机 tauri-2.12.2/src/tray/mod.rs:421：build 注册图标时，在应用资源表保留克隆。setup 中的局部返回值结束后，图标仍由应用保存；无需另写状态容器。
- Accessory 激活策略用于运行时隐藏 Dock，Info.plist 的 LSUIElement 用于打包启动的 agent 身份。

## 动画职责
- [reminder.rs](../src-tauri/src/reminder.rs:32) 经 with_inner_tray_icon 在主线程访问真实 NSStatusItem 按钮，读取图片绘制位置并转换到屏幕坐标。
- AppKit 的透明 NSWindow 承载同形水滴 NSImageView 与提示 NSTextField；NSTimer 触发 Rust 更新图像尺寸和位置，4.8 秒后收起窗口、恢复图标并停掉定时器。
- ANIMATION 是主线程 thread_local 中的 RefCell<Option<Animation>>，只保存当前动画的原生对象与进度；重复触发时沿用正在播放的一次，结束后清空。详细证据见 [动画 Context](context-animation.md)。

## 历史教学
- Calculator/State/Mutex、前后端通信和手动提醒窗口的代码保存在 Git 标签 learning-baseline，当前不再使用。
- 用户对这些概念的理解与教学路线见 [学习 Context](context-learning.md)。

## 官方参考
- 原生核心与 WebView 分工：https://v2.tauri.app/concept/process-model/ 。
- 托盘 API：https://v2.tauri.app/learn/system-tray/ 。
- 配置：https://v2.tauri.app/reference/config/ 。
