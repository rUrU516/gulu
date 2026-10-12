# 按键与表达式计算器学习练习

## 用户请求与当前决策（2026-10-09）
- 用户先要求助手做加减乘除计算器以学习 Rust，确认看到效果后，要求改成类似 Mac 原生计算器的按键布局：受限字符输入、括号、全部逻辑在 Rust。
- 用户随后明确不要滚动条。保留数字 0–9、小数点、四则运算和括号，增加必需的 AC/退格/等于控制键；不加入科学函数、历史记录或其他产品功能。
- 当前是同一 gulu 项目中的学习练习，未改变应用标识或取消原喝水提醒目标。

## 已实现与代码证据
- [lib.rs](../src-tauri/src/lib.rs:16)：计算器通过 calculator_key 命令访问 Tauri 管理的 Mutex<Calculator>；每次调用传一个 key。2026-10-10 同文件另注册提醒命令。
- [calculator.rs](../src-tauri/src/calculator.rs:22)：Rust 保存表达式、结果和错误，验证按键，处理清空、退格、等于、结果续算、新数字开始新计算；数字与括号相邻时自动补乘号。
- [expression.rs](../src-tauri/src/expression.rs:3)：不依赖新增库的表达式解析，支持加减乘除优先级、嵌套括号、一元正负号、除零和非有限结果错误。
- [main.ts](../src/main.ts)：前端只映射按钮/键盘、依次发送 key、渲染 Rust 返回的 expression/display/error；没有前端表达式拼接或求值。snapshot 用于页面重载后恢复 Rust 状态。
- [calculator.html](../calculator.html) 与 [styles.css](../src/styles.css)：深色显示屏、数字键和橙色运算键；页面不出现滚动条，长表达式的水平滚动隐藏滚动条。2026-10-10 计算器 HTML 从原 index.html 保留到此处，主页面已转为手动喝水提醒。
- 计算器练习阶段窗口为 340×520、标题咕噜计算器；现主窗口参数及喝水提醒见 [提醒 Context](context-reminder.md)，产品名和 Identifier 仍为 gulu/com.zhuoru.gulu。

## 已验证
- 最新 pnpm build 通过，包含 TypeScript 检查与 Vite 构建；cargo fmt --check 通过。
- cargo test --offline --lib：13 组测试通过，源码在 [calculator_tests.rs](../src-tauri/src/calculator_tests.rs)。覆盖优先级、嵌套括号、运算结合顺序、负数/小数、隐式乘法、除零、错误语法、溢出、非法按键、退格/清空、结果续算与快照。
- 当前聊天终端确认用户原 pnpm tauri dev 已重新编译并运行 target/debug/gulu；助手未另外启动开发进程。
- 内置浏览器用 340×520 视口检查布局：页面尺寸等于视口，20 个按键的边界都在视口内，根 overflow 为 hidden、滚动条样式为 none。临时视口设置已恢复。
- 浏览器预览没有原生 Tauri 通道，snapshot 会显示连接失败提示；只验证布局，没有把浏览器预览写成原生 IPC 实测。
- 原生 UI 自动化此前 getApp("gulu") 返回 Invalid app、应用清单未列出 gulu，原因未确定；本轮未再尝试或使用其他方式绕过。没有独立点击原生窗口进行端到端验证。

## 学习与限制
- Rust 使用 f64 浮点运算，没有十进制精确运算；不把显示或输入做任意长度/范围限制。
- 用户认可按键效果并提出滚动条修正；本轮括号计算的原生 UI 实测结果尚未由用户确认。
- press_key、append、State/Mutex 等已按用户问题分段讲解，完整解析器尚未展开。当前已转回喝水提醒；理解进展见 [学习 Context](context-learning.md)。

## 官方参考
- Tauri 管理状态：https://v2.tauri.app/develop/state-management/ 。
- Mac 基本计算器交互参考：https://support.apple.com/guide/calculator/use-the-basic-calculator-calc2c05fe23/mac 。
