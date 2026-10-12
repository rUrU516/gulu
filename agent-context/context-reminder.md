# 喝水提醒：手动触发第一步

## 当前范围与决策（2026-10-10）
- 用户要求回到喝水提醒，并一点一点推进；在了解 Rust/Tauri 职责后继续询问第一步写什么。助手实施之前建议的手动提醒窗口，供继续学习。最终常驻形态、提醒周期、位置和宠物外观仍待决定。
- 当前主页面为咕噜测试入口：点击“测试提醒”打开独立窗口；窗口显示“喝口水吧”和“知道啦”，点击后关闭。此步使用应用窗口，尚无系统通知、自动计时或菜单栏。
- 原计算器的 HTML 保留为 calculator.html，原 main.ts/styles.css、Rust 模块和测试保留；没有加入计算器入口按钮。

## 实现与证据
- [reminder.rs](../src-tauri/src/reminder.rs:5)：remind_now 接收 Tauri 提供的 AppHandle，用 Manager 查找 label 为 reminder 的窗口；已有则复用，否则用 WebviewWindowBuilder 创建并加载 reminder.html；随后显示并聚焦。关闭后下次重新创建。
- 采用 async 命令，符合本机 tauri-2.12.2/src/webview/webview_window.rs:56 的窗口创建用法说明；没有新增依赖。
- [lib.rs](../src-tauri/src/lib.rs:30)：注册 calculator_key 与 remind_now。
- [water.ts](../src/water.ts)：测试按钮 await invoke("remind_now")，等待期间禁用以避免连续创建；提醒页通过 getCurrentWindow().close() 关闭自身，错误显示在 status。
- [index.html](../index.html) 是主页面；[reminder.html](../reminder.html) 是提醒页，共享 [water.css](../src/water.css)。页面隐藏滚动条，当前采用简单浅蓝背景、水滴 emoji 和文字，没有确定最终视觉设计。
- [capabilities/reminder.json](../src-tauri/capabilities/reminder.json) 只给 reminder 窗口关闭自身的权限；已核对生成的 capabilities.json 包含该权限。
- [tauri.conf.json](../src-tauri/tauri.conf.json) 主窗口标题咕噜、尺寸 360×340；提醒窗口由 Rust 动态创建，尺寸 340×280。
- [vite.config.ts](../vite.config.ts) 将 index.html、reminder.html、calculator.html 纳入生产构建，避免只在开发服务中存在提醒页。

## 已验证与未验证
- pnpm build、cargo fmt --check、cargo check --offline 均通过；dist 中三个 HTML 入口及相关资源已生成。
- 当前聊天 app 终端证实现有用户 pnpm tauri dev 自动重新编译并运行 target/debug/gulu；没有另起开发进程。
- 内置浏览器分别以 340×280 与 360×340 检查提醒页和主页面：根内容尺寸等于视口，按钮和文字可见，没有滚动条。临时视口已恢复，预览 tab 已关闭。
- 原生应用清单未列出 gulu，无法通过该清单绑定当前窗口；没有把浏览器布局验证记为原生 IPC 或窗口关闭的端到端实测。实际弹出、重复点击、关闭再打开仍待用户在运行中的咕噜验证。
- 已请求在 Codex 打开 reminder.rs，工具返回 queued，不能宣称用户已经看到文件。

## 下一步
- 先按用户节奏读 remind_now，只解释当前需要的 AppHandle、窗口查找/创建和载入页面。
- 手动提醒行为确认后，下一小步为短时间延迟触发，再考虑周期性提醒和菜单栏。具体参数与交互未确定。

## 官方参考
- 核心进程和 WebView：https://v2.tauri.app/concept/process-model/ 。
- 窗口示例：https://v2.tauri.app/zh-cn/learn/security/capabilities-for-windows-and-platforms/ 。
- 多 HTML 入口构建：https://vite.dev/guide/build.html#multi-page-app 。
