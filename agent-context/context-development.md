# 开发与版本管理

## 当前工程
- 路径：/Users/zhuoru/Documents/Codex/2026-10-09/w/gulu。
- Rust 包 gulu，库 gulu_lib；本机检查使用 tauri 2.12.2、tray-icon 0.25.1。CLI 锁定版本保持原项目安装值。
- package.json 只保留 Tauri CLI，dev/build 分别调用 tauri dev/tauri build，另保留 tauri 脚本。
- 当前版本无需 TypeScript、Vite、WebView 页面或前端开发服务器。
- node_modules、dist、Rust target 和生成的 schemas 由 .gitignore 忽略，历史构建缓存仍可能存在，不属于当前源码或应用入口。

## 运行与构建
- 在项目目录执行 pnpm tauri dev（或 pnpm dev）；开发时使用终端 Ctrl+C 停止。图标没有退出菜单。
- pnpm tauri build --debug --bundles app 生成本地调试 .app；2026-10-12 已构建成功。
- 当前调试产物：src-tauri/target/debug/bundle/macos/gulu.app，运行验证限制见 [菜单栏 Context](context-menubar.md)。

## Git 版本管理
- 2026-10-12 核实项目和父目录没有 Git 仓库后，在 gulu/ 初始化本地 Git，分支 main，使用用户已有 Git 身份配置。
- 首个提交 38fdc30 保存计算器与手动提醒全部原始代码，标签 learning-baseline。
- 本次菜单栏版本完成提交后创建标签 v0.1.0；不创建远端或推送。
- 查看旧代码而不改工作目录：git show learning-baseline:src-tauri/src/calculator.rs，或 git show learning-baseline:src-tauri/src/reminder.rs。
- 不使用 git reset 覆盖当前工作。后续改动按功能单独提交，保留可回看的版本。

## 检查
- 当前：cargo fmt --check、cargo check --offline、git diff --check、调试 .app 打包通过。
- 旧计算器和手动提醒的验证属于历史，见对应历史 Context，不套用到当前菜单栏显示。
