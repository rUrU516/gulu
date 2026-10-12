# 历史尝试：手动喝水提醒

此实现已被用户在 2026-10-12 明确替换为“只有菜单栏图标”，源码和当时记录保存在 Git 标签 learning-baseline。

- 2026-10-10 实现了测试按钮调用 Rust，创建喝水提醒窗口并载入 HTML；点击知道啦关闭。
- 当时编译/布局验证通过，但未独立完成原生点击实测。
- 查看代码：git show learning-baseline:src-tauri/src/reminder.rs，以及同标签下的 reminder.html、src/water.ts。
- 当前不保留提醒入口或窗口逻辑，当前范围见 [菜单栏 Context](context-menubar.md)。
