# 开发与版本管理

## 当前工程
- 路径：/Users/zhuoru/Documents/Codex/2026-10-09/w/gulu。
- Rust 包 gulu，库 gulu_lib；本机检查使用 tauri 2.12.2、tray-icon 0.25.1。CLI 锁定版本保持原项目安装值。
- package.json 只保留 Tauri CLI，dev/build 分别调用 tauri dev/tauri build，另保留 tauri 脚本。
- 当前版本无需 TypeScript、Vite、WebView 页面或前端开发服务器。
- 水滴弹出动画增加 macOS 专属 objc2 0.6 / objc2-app-kit、objc2-foundation 0.3 / block2 0.6 直接依赖，复用已有锁定版本。使用 with_inner_tray_icon 接触底层图标，按 Tauri API 文档把版本范围限制到 ~2.12，锁定仍为 2.12.2。
- node_modules、dist、Rust target、生成的 schemas 和本地 releases/ 安装包由 .gitignore 忽略，历史构建缓存仍可能存在，不属于当前源码或应用入口。

## 运行与构建
- 在项目目录执行 pnpm tauri dev（或 pnpm dev）；点击菜单栏水滴打开菜单，选择“退出”请求停止应用，用户已确认这个退出动作成功。开发时也可使用终端 Ctrl+C 停止。
- 每 5 分钟自动播放一次约 4.8 秒的水滴动画，显示“记得喝水”；启动后首次等待 5 分钟，点击“试试提醒”仍可立即手动播放。
- pnpm tauri build --debug --bundles app 生成本地调试 .app；2026-10-12 已构建成功。
- 当前调试产物：src-tauri/target/debug/bundle/macos/gulu.app，运行验证限制见 [菜单栏 Context](context-menubar.md)。
- 2026-10-13 已按用户要求生成 Apple 芯片正式构建 .app 与 .dmg，可分享文件为 releases/gulu-0.1.0-apple-silicon.dmg；命令、签名限制与校验结果见 [分发 Context](context-distribution.md)。构建未替换当前开发进程，也未重复启动安装包中的应用。
- 打包不会自动重启已经运行的 .app。2026-10-12 用户点击无反应时确认旧打包进程仍在运行，已停止旧版并启动当前 pnpm tauri dev。检查运行状态应读取真实进程；read_thread_terminal 本轮没有附加终端，不能据此判断应用没有运行。

## Git 版本管理
- 2026-10-12 核实项目和父目录没有 Git 仓库后，在 gulu/ 初始化本地 Git，分支 main，使用用户已有 Git 身份配置。
- 首个提交 38fdc30 保存计算器与手动提醒全部原始代码，标签 learning-baseline。
- 纯图标版本提交 8766034 已创建标签 v0.1.0；后续教学记录和退出菜单作为独立提交保留，不移动该标签。不创建远端或推送。
- 查看旧代码而不改工作目录：git show learning-baseline:src-tauri/src/calculator.rs，或 git show learning-baseline:src-tauri/src/reminder.rs。
- 不使用 git reset 覆盖当前工作。后续改动按功能单独提交，保留可回看的版本。

## 检查
- 当前退出菜单实现：cargo fmt --check、cargo check --offline、调试 .app 打包通过；用户已确认重启最新版后能看到退出菜单且选择后成功退出，详见菜单栏 Context。
- 随后把菜单文字简化为“退出”，cargo fmt --check、git diff --check 通过；pnpm tauri dev 编译完成并启动 target/debug/gulu，进程检查确认运行。旧打包 .app 本轮没有重新生成，最新文字在开发进程中。
- 动画版本 cargo check --offline、cargo fmt --check、git diff --check 通过；CARGO_NET_OFFLINE=true pnpm tauri build --debug --bundles app 成功生成新的调试 .app。用户确认动画出现并自动缩回，随后误退出；2026-10-13 已再次通过 pnpm tauri dev 启动，编译完成并由进程检查确认 target/debug/gulu 在运行。
- 自动提醒以临时 10 秒间隔验证至少两次连续回调、播放和收回，cargo check --offline 通过；随后恢复为 300 秒，cargo fmt --check、git diff --check、调试 .app 打包通过。开发进程已重新启动，日志确认每 300 秒注册成功，进程检查确认运行。
- 正式 300 秒配置在分发打包期间已观察到两次自动触发并完成收回；Apple 芯片 release 构建、应用签名验证、DMG 完整性及只读挂载内容检查通过，见 [分发 Context](context-distribution.md)。
- 旧计算器和手动提醒的验证属于历史，见对应历史 Context，不套用到当前菜单栏显示。
