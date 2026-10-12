# 当前示例的启动与通信

## 已验证的启动链
- 用户执行 pnpm tauri dev；[package.json](../package.json) 的 tauri 脚本调用 Tauri CLI。
- CLI 根据 [tauri.conf.json](../src-tauri/tauri.conf.json) 的 beforeDevCommand 启动 pnpm dev，即 Vite；当前页面地址为 http://localhost:1420/。
- CLI 同时通过 Cargo 编译并启动 Rust 桌面程序。首次编译已在 app 终端中确认成功，随后执行 target/debug/gulu；用户报告看到示例结果。
- [main.rs](../src-tauri/src/main.rs:4) 调用 gulu_lib::run；[lib.rs](../src-tauri/src/lib.rs:27) 初始化 Tauri，管理 Mutex<Calculator> 状态，注册 calculator_key/remind_now 命令并进入应用运行循环。
- Tauri 根据配置创建窗口；macOS 使用 WKWebView 渲染 HTML/CSS 和执行 JavaScript。TypeScript 通过 Vite 转换为 JavaScript。

## 当前计算器调用链
- 原 Greet 示例曾用于解释 command/注册/invoke，用户已用自己的话确认 Tauri 桥梁概念；当前代码已按用户请求替换为计算器。
- [calculator.html](../calculator.html) 定义显示屏与受限按键，[styles.css](../src/styles.css) 定义 Mac 风格深色布局并隐藏滚动条；原 index.html 在 2026-10-10 转为喝水提醒主页面。
- [main.ts](../src/main.ts) 把按钮和键盘映射成 key，按顺序 invoke calculator_key；不拼接表达式或计算。页面重载用 snapshot 读取 Rust 当前状态，前端只渲染返回的 expression/display/error。
- [lib.rs](../src-tauri/src/lib.rs:16) 的 calculator_key 由 Tauri 注入管理状态，加锁后交给 Calculator::press_key。命令返回 CalculatorView 或通信/输入错误；数学错误保存在 View.error，让表达式保留并可继续编辑。
- 2026-10-10 核对当前本机依赖实现：tauri-2.12.2/src/app.rs:2055 的 Builder::manage 按值接收状态并调用 StateManager::set；src/state.rs:119 将状态装入 Box 并按 TypeId 存入表，State 本身借用保存值，命令参数转换负责查找状态。当前保存的类型是 Mutex<Calculator>，不是以变量名或命令名查找；源码根为 /Users/zhuoru/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tauri-2.12.2/。
- 当前 rustc 1.94.1 的 std/src/sync/poison/mutex.rs:227、350、490、724 已核对：Mutex 包含锁信息与 UnsafeCell<T>，new 按值移入数据；lock 返回 MutexGuard，guard 的 DerefMut 提供 &mut T，Drop 解锁。本项目 calculator_key 内的 calculator 变量实际是 guard，方法调用访问内部 Calculator；不把 lock 说成移出或复制实例。源码根为 /Users/zhuoru/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/。
- [calculator.rs](../src-tauri/src/calculator.rs:22) 维护输入、清空、退格和结果续算；[expression.rs](../src-tauri/src/expression.rs:3) 处理先乘除后加减、括号和一元正负号。
- invoke 通过本地 IPC 消息桥路由给 Rust，序列化的显示状态完成前端 Promise；命令层 Err 会进入 catch。Vite 的 HTTP 页面资源服务不承担计算业务。
- 退格也会发起一次 calculator_key 请求并接收新的 View；跨 WebView/Rust 的 invoke 是 IPC，进入 Rust 后的 String::pop 则是普通方法调用。IPC 涉及消息处理与调度，会有额外耗时；当前应用的调用往返或完整输入到显示时延尚未实测，不能给出确定数值。
- 构建、运算与界面预览验证范围见 [计算器 Context](context-calculator.md)，没有将其写成原生 IPC 端到端实测。

## 当前职责与后续设计计划
- 窗口及应用生命周期由 Rust/Tauri 管理；页面内部的布局、样式和 DOM 交互由 WebView 前端处理。
- 窗口创建入口有配置、Rust API 和 JavaScript API；应用原生层执行实际窗口创建。现有主窗口来自 tauri.conf.json 的 app.windows 配置。喝水提醒建议由 Rust 决定显示时机、前端编写窗口内容，是应用分工选择，不能说成所有窗口必须由用户编写的 Rust 命令创建。JavaScript API 官方证据：https://v2.tauri.app/zh-cn/reference/javascript/api/namespacewebviewwindow/ 。
- 发布构建先生成前端静态文件，再由 Rust/Tauri 将它们打包进应用；Vite 开发服务不是发布后应用的运行依赖。
- 喝水提醒已有手动窗口链路：water.ts 调用 remind_now，Rust 通过 AppHandle 创建或显示 reminder 窗口，载入 reminder.html；窗口前端调用 Tauri 的 close API 关闭自身。计时与菜单栏仍未实现，细节见 [提醒 Context](context-reminder.md)。

## Rust 入口与语法讲解（2026-10-09）
- 用户已打开 Rust 文件预览并请求理解 main.rs 和 lib.rs；全面讲解后用户反馈信息过多，改为按 [学习进展与沟通方式](context-learning.md) 小步讲解。现已按用户新请求实现计算器，供继续学习。
- [Cargo.toml](../src-tauri/Cargo.toml:14) 的 lib.name 为 gulu_lib，main.rs 通过这个库名调用公开的 run；不是另一个服务。Tauri 官方项目结构说明：桌面 main 与移动端库入口复用 lib.rs 的应用逻辑。
- main.rs 的条件属性用于 Windows 非 debug_assertions 构建的子系统设置；lib.rs 的 mobile_entry_point 只在 mobile 条件下应用，当前 macOS 构建不需要这两项平台行为。
- Builder 链先配置插件与命令处理器，再运行应用；命令函数的定义位置不决定执行时机。command 属性和 generate_handler 注册共同支持前端 invoke。
- 已核对本机依赖源码：tauri-macros-2.7.1/src/lib.rs 的 generate_context 文档说明编译期读取配置并生成 Context；tauri-2.12.2/src/app.rs 的 Builder::run 先 build 再运行事件循环，返回 Result。源码位于 ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/。
- 讲解包含借用的 &str、拥有数据的 String、format! 宏、末尾表达式返回值、pub 可见性和 expect 错误处理；后续按用户问题逐步深入。

## 官方证据
- 进程与 WebView：https://v2.tauri.app/concept/process-model/ 。
- IPC 消息机制：https://v2.tauri.app/concept/inter-process-communication/ 。
- command、invoke 与返回值：https://v2.tauri.app/develop/calling-rust/ 。
- 项目分层与静态资源打包：https://v2.tauri.app/start/project-structure/ 。
- TypeScript 开发转换：https://vite.dev/guide/features.html#typescript 。
- 插件初始化与 opener 用途：https://v2.tauri.app/plugin/opener/ 。
- Rust 二进制与库入口约定：https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html 。
- Tauri 管理状态与 Mutex：https://v2.tauri.app/develop/state-management/ 。
