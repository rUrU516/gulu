# 开发与运行

## 已验证事实（2026-10-09 至 2026-10-10）
- 用户完成 pnpm create tauri-app，选择 gulu / com.zhuoru.gulu / TypeScript-JavaScript / pnpm / Vanilla / TypeScript。
- [package.json](../package.json) 使用 Vite 开发服务；build 执行 tsc 和 vite build；tauri 脚本调用 Tauri CLI。
- 声明的依赖范围：Tauri API/CLI ^2，Vite ^8.0.16，TypeScript ~6.0.3。安装后读取 node_modules 中的元数据，已确认 Tauri CLI 2.12.1、Vite 8.3.4、TypeScript 6.0.3；API 的精确版本未单独核验。
- [tauri.conf.json](../src-tauri/tauri.conf.json) 的 productName 为 gulu，现主窗口 title 为咕噜，初始尺寸 360×340，identifier 是 com.zhuoru.gulu；devUrl 为 http://localhost:1420，beforeDevCommand 为 pnpm dev。旧计算器窗口设置已由手动提醒第一步替换。
- [Cargo.toml](../src-tauri/Cargo.toml) 声明 Tauri 2、opener 插件及 serde 依赖；Rust 包名 gulu，库名 gulu_lib。
- [lib.rs](../src-tauri/src/lib.rs) 注册 calculator_key/remind_now 并管理计算器状态；[calculator.rs](../src-tauri/src/calculator.rs) 处理按键，[expression.rs](../src-tauri/src/expression.rs) 解析表达式，[reminder.rs](../src-tauri/src/reminder.rs) 手动显示喝水提醒。计时与菜单栏尚未实现。
- 本机此前核验过 Rust/Cargo 1.94.1、Node v25.8.0、pnpm 可用；Xcode Command Line Tools 位于 /Library/Developer/CommandLineTools。
- 项目已有脚手架生成的 README.md；助手未额外创建 README。

## 职责与设计计划
- HTML/CSS 负责外观，TypeScript 负责界面交互，Rust 负责计时和系统操作。这是当前应用设计计划，尚未实现。
- Tauri 核心和 WebView 通过本地 IPC 通信；前端用 invoke 调用 Rust command。macOS 使用 WKWebView。证据：https://v2.tauri.app/concept/process-model/ 、https://v2.tauri.app/develop/calling-rust/ 。
- 中文名咕噜已由用户确认；当前主窗口标题咕噜，提醒窗口标题咕噜 · 喝口水吧，打包的 productName 仍为 gulu。

## 当前进度与下一步
- 首次项目生成、安装依赖和启动由用户操作，助手初始化并维护 agent-context/；计算器练习由用户明确授权助手实现和验证。
- 用户结束 Yazi 字体排查后，已自行安装依赖并执行 pnpm tauri dev。项目中现有 pnpm-lock.yaml、node_modules、Cargo.lock；用户输出确认 Vite 就绪于 http://localhost:1420/。
- 读取当前聊天的 app 终端，已确认首次 Rust 编译完成并执行 target/debug/gulu；用户回复“嗯嗯我看到了”，随后要求解释底层和各层代码职责。助手未独立点击 UI。
- 已将双输入框练习升级为 [按键与表达式计算器](context-calculator.md)，前端构建、Rust 格式与 13 组 Rust 单元测试通过；用户的原开发进程已自动重新编译运行。默认尺寸布局无滚动条且所有按键在视口内。
- 桌面流程无需初始化移动端；脚手架同时打印 Android/iOS 命令，不代表本项目已选择移动端。
- 计算器已用于学习 Rust 与通信；2026-10-10 已开始 [手动喝水提醒窗口](context-reminder.md)，主页面改为咕噜测试入口，旧计算器页面保留为 calculator.html。本轮前端构建、Rust 格式/编译检查与布局验证通过，现有开发进程自动重启；原生点击结果待用户验证。用户希望小步讲解，具体理解进展以 [学习 Context](context-learning.md) 为准。
- 全屏、跨桌面、多屏及窗口焦点行为待实际实现后验证，不提前承诺。

## 官方参考
- 项目创建与启动：https://v2.tauri.app/start/create-project/ 。
- macOS 依赖：https://v2.tauri.app/start/prerequisites/ 。
- pnpm create：https://pnpm.io/cli/create 。

## 本机终端问题（2026-10-09，用户报告恢复正常）
- 用户在创建项目后提出 Yazi 显示方框的问题。截图中文件名可读，异常集中在文件/目录图标；排查后已返回项目安装和启动流程。
- 本机 Yazi 版本为 26.9.1（Homebrew），且 ~/Library/Fonts 下存在 JetBrainsMono Nerd Font / Nerd Font Mono 和 0xProto Nerd Font 的字体文件。
- Yazi 官方 FAQ 明确说明：终端未支持 Nerd Font 时图标会显示成方框/问号；需设置合适字体并重启终端。证据：https://yazi-rs.github.io/docs/faq/ 。
- Codex 官方文档说明 Code font 同时控制代码显示、review pane 和内置终端。证据：https://learn.chatgpt.com/docs/reference/troubleshooting 。
- 用户 12:55:50 提供的设置截图确认：Content font 为 JetBrainsMono Nerd Font Mono，而 Code font 仍为 System；同批终端截图依然显示缺字方框。结合官方文档，确认当时用户尚未把终端使用的 Code font 切换为 Nerd Font。已引导修改 Code font 并重开终端/Yazi，用户随后报告“好像没问题了”并要求继续开发；没有复核新的设置截图。助手没有修改应用设置、Yazi 配置或安装字体。
- 用户进一步询问统一方案。建议所有常用终端选择同一个已安装的 Nerd Font，一次设置可供该终端中的 CLI 工具共享；各终端的字体配置仍需分别设置。
- 另有 Symbols Only 字体回退候选，官方安装命令为 brew install --cask font-symbols-only-nerd-font。Nerd Fonts 官方说明支持 SymbolsOnly 作为缺字回退来源，但效果受实际回退机制影响；尚未验证用户的各终端能否自动使用它，不能承诺安装一次覆盖全部应用。证据：https://github.com/ryanoasis/nerd-fonts#option-7-font-fallback 、https://formulae.brew.sh/cask/font-symbols-only-nerd-font 。没有执行安装。
