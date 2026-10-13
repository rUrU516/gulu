# 水滴从菜单栏弹出的提醒动画

## 用户要求与决定
- 用户要求维持当前水滴形象，以真实状态栏图标位置为起点，放大/跳出提醒喝水，再自己缩回。
- 当前先提供“试试提醒”菜单项播放演示，保留“退出”。喝水周期尚未确定，不自动设置定时提醒。
- 采用 Rust 调用 macOS AppKit 原生窗口与图片组件；不引入前端、WebView 或 Vite。

## 实现与证据
- [lib.rs](../src-tauri/src/lib.rs:32) 的菜单回调按 preview ID 调用 [reminder::play](../src-tauri/src/reminder.rs:32)。
- play 经 Tauri with_inner_tray_icon 在主线程读取 NSStatusItem 的按钮和 NSCell 图片绘制区域，再转换到屏幕坐标。全程使用 AppKit 坐标，不猜菜单栏高度或 Retina 缩放倍率。
- 创建 220×244 的无边框透明 NSWindow，以图标中心作为动画锚点；设置忽略鼠标、无窗口背景、位于状态栏上方。工作区/全屏行为仅是当前配置，未独立实测。
- [water-large.png](../src-tauri/icons/water-large.png) 由现有 [tray.svg](../src-tauri/icons/tray.svg) 通过 bundled sharp 渲染为 288×288 PNG，沿用原水滴几何，不另换形象。
- 将原菜单栏图标换成同尺寸透明图片，保留占位；窗口中的 NSImageView 从同一绘制区域接过水滴，原菜单项仍保留。
- NSTimer 在主线程 run loop 中约每 1/60 秒触发更新；这不是实测 60 FPS 的保证。Animation::draw 根据 Instant 的已过去时间计算尺寸/下移/小幅摆动和文字透明度。
- 动画总长 4.8 秒：约 0.65 秒弹出，放大到 96 点并下移约 82 点，停留时显示“喝口水吧”，4.1 秒开始缩回。具体参数以 [draw](../src-tauri/src/reminder.rs:189) 为准。
- [tick_animation](../src-tauri/src/reminder.rs:163) 在结束时隐藏窗口、恢复模板图标、停止 timer、清空 Animation。对象只保留在主线程 thread_local 的 RefCell<Option<Animation>> 中，重复点击不叠加动画。
- Tauri 仍负责入口、菜单与应用事件循环；objc2/AppKit/Foundation/block2 负责原生界面绑定和动画回调，直接依赖见 Cargo.toml。

## 已验证事实与限制
- cargo check --offline、cargo fmt --check、git diff --check 通过；调试 .app 打包成功。
- 用户明确确认“出现了且自动缩回了”，之后误点击退出。上一轮开发进程日志也记录了一次开始/结束与图标恢复，起点尺寸为实际 18×18；不把这个测量推广到所有显示环境。
- 2026-10-13 已按用户要求重启 pnpm tauri dev，编译完成并由进程检查确认 target/debug/gulu 在运行。
- 原生自动检查先前绑定 gulu 超时；本轮库存读取完成但有浏览器策略错误，尝试绑定 Codex 被工具安全规则拒绝。没有绕过规则读取或操作 Codex UI，实际效果以用户确认与应用日志为证据。
- 多屏切换、菜单栏自动隐藏、全屏及深浅外观未逐项验证；图标起点衔接、阴影和动画节奏的审美细节待用户反馈。不在未验证情况下写成完全兼容。

## 参考与后续
- 已核对本机 Tauri 2.12.2 的 tray/mod.rs：with_inner_tray_icon 在主线程执行闭包；其文档建议使用这一底层接口时限制到对应次版本，Cargo.toml 已采用 ~2.12。
- 官方接口：https://docs.rs/tauri/latest/tauri/tray/struct.TrayIcon.html 。
- 原生窗口：https://developer.apple.com/documentation/appkit/nswindow 。图片组件：https://developer.apple.com/documentation/appkit/nsimageview 。本机 objc2-app-kit 0.3.2 的生成绑定亦已核对。
- 接下来按用户问题逐点解释实现，或调整具体动画；自动提醒间隔尚未决定。
