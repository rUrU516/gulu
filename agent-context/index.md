# Gulu / 咕噜：项目上下文

开始任务或恢复上下文时，先读本文件，再按任务读取关联 Context。

## 当前范围（用户明确要求，2026-10-12）
- 只做一个常驻 macOS 顶部菜单栏的小图标，点击没有动作。
- 不创建窗口；没有提醒、计时、通知、记录、菜单或其他功能。
- 名称 gulu / 咕噜，Identifier 为 com.zhuoru.gulu。
- 当前实现是 Rust + Tauri 原生启动代码；无需前端页面或 Vite。
- 用户要求做好版本管理：项目已初始化本地 Git，旧计算器/手动提醒代码保存在 learning-baseline。

## 工作约束
- 默认中文；不确定的信息必须查证或询问，不能臆断。
- 保持小步推进，不自行扩展功能。用户的 TypeScript/Rust 基础较初步，按当前代码逐点解释。
- 叙事性资料维护在 agent-context/；明确区分已验证事实、决定、计划和待验证结论。

## 路由
- [菜单栏最小版本](context-menubar.md)：当前实现、验证与范围。
- [开发与版本管理](context-development.md)：运行/构建命令、Git 标签与旧代码恢复。
- [当前架构](context-architecture.md)：Rust/Tauri 启动链与图标生命周期。
- [产品范围](context-product.md)：当前要求和被替换的方案。
- [学习进展](context-learning.md)：用户偏好和历史教学进展。
- [计算器历史](context-calculator.md)、[手动提醒历史](context-reminder.md)：保留学习背景，源码在 Git 标签中。
