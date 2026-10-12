# 当前架构：只有菜单栏图标

## 已验证的代码路径
- package.json 的 tauri/dev 脚本调用 Tauri CLI；没有前端开发服务器。
- [main.rs](../src-tauri/src/main.rs:4) 调用 gulu_lib::run。
- [lib.rs](../src-tauri/src/lib.rs:4) 创建 Tauri Builder，在 setup 回调中设置 macOS 激活策略、创建菜单栏图标，然后 run 进入应用运行循环。
- [tauri.conf.json](../src-tauri/tauri.conf.json) 的 windows 为空，应用启动不创建 WebView 窗口。
- 仅原生核心运行：无前端、command、invoke、Mutex 状态、计时或提醒逻辑。

## 图标与生命周期
- TrayIconBuilder::with_id("gulu") 创建一个图标，使用 include_image! 编译时载入 PNG，icon_as_template(true) 交给 macOS 着色；没有菜单/事件监听器。
- 已核对本机 tauri-2.12.2/src/tray/mod.rs:421：build 注册图标时，在应用资源表保留克隆。setup 中的局部返回值结束后，图标仍由应用保存；无需另写状态容器。
- Accessory 激活策略用于运行时隐藏 Dock，Info.plist 的 LSUIElement 用于打包启动的 agent 身份。

## 历史教学
- Calculator/State/Mutex、前后端通信和手动提醒窗口的代码保存在 Git 标签 learning-baseline，当前不再使用。
- 用户对这些概念的理解与教学路线见 [学习 Context](context-learning.md)。

## 官方参考
- 原生核心与 WebView 分工：https://v2.tauri.app/concept/process-model/ 。
- 托盘 API：https://v2.tauri.app/learn/system-tray/ 。
- 配置：https://v2.tauri.app/reference/config/ 。
