# 开发与版本管理

## 当前工程
- 路径：/Users/zhuoru/Documents/Codex/2026-10-09/w/gulu。
- Rust 包 gulu，库 gulu_lib；本机检查使用 tauri 2.12.2、tray-icon 0.25.1。CLI 锁定版本保持原项目安装值。
- package.json 只保留 Tauri CLI，dev/build 分别调用 tauri dev/tauri build，另保留 tauri 脚本。
- 当前版本无需 TypeScript、Vite、WebView 页面或前端开发服务器。
- node_modules、dist、Rust target 和生成的 schemas 由 .gitignore 忽略，历史构建缓存仍可能存在，不属于当前源码或应用入口。

## 运行与构建
- 在项目目录执行 pnpm tauri dev（或 pnpm dev）；点击菜单栏水滴打开菜单，选择“退出咕噜”请求停止应用。开发时也可使用终端 Ctrl+C 停止。
- pnpm tauri build --debug --bundles app 生成本地调试 .app；2026-10-12 已构建成功。
- 当前调试产物：src-tauri/target/debug/bundle/macos/gulu.app，运行验证限制见 [菜单栏 Context](context-menubar.md)。
- 打包不会自动重启已经运行的 .app。2026-10-12 用户点击无反应时确认旧打包进程仍在运行，已停止旧版并启动当前 pnpm tauri dev。检查运行状态应读取真实进程；read_thread_terminal 本轮没有附加终端，不能据此判断应用没有运行。

## Git 版本管理
- 2026-10-12 核实项目和父目录没有 Git 仓库后，在 gulu/ 初始化本地 Git，分支 main，使用用户已有 Git 身份配置。
- 首个提交 38fdc30 保存计算器与手动提醒全部原始代码，标签 learning-baseline。
- 纯图标版本提交 8766034 已创建标签 v0.1.0；后续教学记录和退出菜单作为独立提交保留，不移动该标签。不创建远端或推送。
- 查看旧代码而不改工作目录：git show learning-baseline:src-tauri/src/calculator.rs，或 git show learning-baseline:src-tauri/src/reminder.rs。
- 不使用 git reset 覆盖当前工作。后续改动按功能单独提交，保留可回看的版本。

## 检查
- 当前退出菜单改动：cargo fmt --check、cargo check --offline、调试 .app 打包通过；用户已确认重启最新版后能看到退出菜单，实际选择退出的进程结束行为尚未确认，详见菜单栏 Context。
- 旧计算器和手动提醒的验证属于历史，见对应历史 Context，不套用到当前菜单栏显示。
