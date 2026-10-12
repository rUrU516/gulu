# 产品范围与形式

## 已确认需求与决策
- 用户想做一个有趣、可用于女朋友或两人互动的小 macOS 应用，希望有可爱的宠物感。
- 第一版明确只做提醒喝水。曾提过记录功能，随后明确从第一版移除。
- 名称确定为 gulu / 咕噜，替代此前暂称的蹲伏小宠物。
- 用户希望边写边体会，先运行最小骨架；暂不展开具体视觉设计。
- 用户随后明确要求助手先做一个加减乘除计算器，以便学习 Rust；已在同一项目中替换示例，见 [计算器学习练习](context-calculator.md)。这是一项当前学习请求，用户未明确取消原喝水提醒目标。
- 用户确认看到双输入框计算器后要求改成类似 Mac 原生计算器的按键形式，输入数字、四则运算和括号，逻辑全部交给 Rust；随后明确不要滚动条。当前练习已按此升级。
- 2026-10-10 用户在 State 教学后提出开始写喝水提醒，询问从哪里开始，明确要求一点一点来；随后继续询问第一步写什么，已开始手动提醒窗口。完整交互仍待决定，不把第一步的实现当成最终产品形式。

## 当前起点实现（2026-10-10）
- 用户继续询问第一步写什么后，已实施手动触发的最小提醒：测试按钮调用 Rust 命令，Rust 打开简单的喝水提醒小窗口，用户可关闭。复用 invoke/command；实现与验证范围见 [喝水提醒第一步](context-reminder.md)，原生点击效果仍待用户确认。最终完整交互尚未确定。
- 后续先加短时间的自动触发以便观察，再加入实际提醒周期，最后接菜单栏入口；具体周期、窗口位置与外观仍待决定。
- Tauri 支持由 Rust 创建 Webview 窗口，官方示例依据：https://v2.tauri.app/zh-cn/learn/security/capabilities-for-windows-and-platforms/ 。此步未新增依赖。

## 候选形式与待确认事项
- 已讨论常驻应用、macOS 菜单栏入口、到点显示短暂提醒小窗口的形式。菜单栏加小卡片是助手建议，用户尚未明确确定完整交互行为。
- 其他曾讨论的候选包括顶部提醒条和系统通知。
- 提醒周期、出现位置、消失方式、声音、打扰程度和宠物形象尚未确定。

## 已核实能力与边界
- Tauri 提供系统托盘/菜单栏 API。证据：https://v2.tauri.app/learn/system-tray/ 。
- 系统通知需要用户授权，呈现方式受系统通知设置影响。证据：https://v2.tauri.app/plugin/notification/ 、https://developer.apple.com/design/human-interface-guidelines/notifications 。
- macOS 完全透明窗口涉及 macos-private-api；半透明背景可研究 windowEffects。尚未决定采用哪种方式。证据：https://v2.tauri.app/reference/config/ 。

## 既有设计实验
- 创建过候选形式的概念预览，文件为 /Users/zhuoru/.codex/visualizations/2026/10/09/01a11e8e-4868-79f2-b5c0-7697d8880154/water-reminder-forms.html。
- 它是外观与本地交互演示，不是可安装应用；用户随后暂停具体设计讨论。若重新修改预览，先完整读取 visualize 技能。
