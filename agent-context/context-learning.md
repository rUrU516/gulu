# 学习进展与沟通方式

## 当前阶段（2026-10-12）
- 用户明确要求只有菜单栏图标、没有其他功能，并要求版本管理。保持这个粒度，不自行恢复提醒或继续旧窗口方案。
- 计算器和手动提醒已移入 Git 历史 learning-baseline；下文指向旧项目源码的位置应在该标签中查看，标准库的本机源码引用仍独立存在。
- 当前仅有 TrayIconBuilder 启动骨架。用户要求讲解这版代码；本轮按 main → Builder → setup → 图标 build → run 的顺序说明职责，重点区分 setup 注册启动回调、build 实际创建图标，以及 run 维持应用运行。
- 本轮同时说明图标 ID、编译时嵌入 PNG、macOS 模板着色、Accessory 隐藏 Dock、? / Ok(()) 处理初始化结果和 windows 空配置；讲解依据为当前源码及本机 tauri-2.12.2 的 app.rs / tray/mod.rs。用户尚未反馈理解程度，不把讲解完成当成掌握；闭包、构建器或事件循环细节继续按提问展开。
- 用户随后明确表示不太了解 Rust 闭包，要求单独讲解。当前从普通 fn 与 |参数| { 函数体 } 的对照、闭包存入变量后调用、访问外层局部变量的捕获能力讲起，再连接 lib.rs:6 的 setup：app 是 Tauri 传入的参数，这段闭包没有捕获外部局部变量；setup 接收并保存它以便启动时调用，参数类型由接口推断。依据：https://doc.rust-lang.org/book/ch13-01-closures.html 与当前源码。尚无用户理解反馈，Fn/FnMut/FnOnce、move 和捕获的所有权规则未展开。

## 用户明确自述与偏好（2026-10-09）
- TypeScript 和 Rust 都只有比较基础的水平；不能因为能运行项目就假定熟悉框架、宏、构建器、IPC 或 Rust 所有权。
- main.rs/lib.rs 的上一轮全面讲解信息过多，用户表示有点懵，许多函数没见过。
- 用户要求多沟通、持续更新学习进展，只讲当前阶段需要理解的内容。
- 创建、安装、首次启动等重要步骤由用户亲自操作，助手解释与检查；不把教学问题自动扩大为实现新功能。
- 用户后来明确授权“帮我做一个简单的计算器出来先，就支持加减乘除”，用于学习 Rust；助手可完成该练习实现与验证。

## 已验证的操作进展
- 用户已创建项目、安装依赖、执行 pnpm tauri dev，并报告看到了示例界面。
- 用户已打开 Rust 文件预览，并询问 main.rs/lib.rs。
- 操作完成不等于已掌握相关原理；不要把助手讲过的内容自动记成用户已理解。

## 当前理解与疑问
- 已确认的概念理解：用户用自己的话描述 Tauri 是前端与 Rust“后端”之间的桥梁。这个描述正确涵盖当前通信职责；不据此推断已掌握注册代码或底层 IPC。
- 用户提出的理解：输入名字后调用 Rust 函数，返回值再渲染到界面。这个流程与当前源码一致，但准确触发点是提交表单，例如点击 Greet，并非仅输入文字。
- 上一轮已用当前 greet 说明输入/提交 → 前端 invoke 请求 → Tauri 转交给已注册的 Rust greet → 返回字符串 → 前端更新显示；用户回应“嗯嗯”，并主动请求进一步讲通信与注册。不据此推断已掌握具体宏或 Promise。
- command 标记、generate_handler/invoke_handler 注册与前端 invoke 已用 Greet 示例讲解；具体语法是否已掌握尚未确认。
- 当前展开的注册细节：command 在编译时生成调用包装代码，generate_handler 生成按命令名分发的处理器，invoke_handler 将其接入应用；标记和注册都需要，前端以字符串命令名发起调用。
- 通信讲解范围：WebView 与 Rust 交换序列化消息，Tauri 根据参数名把当前 name 交给 greet，返回结果完成 invoke 的 Promise；await 等到结果后前端更新显示。不展开平台通道实现、回调编号、事件或安全权限系统。
- 注册语法和底层消息传递的具体理解程度仍待用户反馈；技术证据在 [示例架构与通信](context-architecture.md) 所链接的官方 command/IPC 文档。
- 对 await/Promise、宏、所有权和事件循环的掌握程度未确认，不臆断。

## 历史学习练习：计算器
- 用户确认看到了原双输入框计算器效果，然后要求 Mac 风格按键输入，支持括号，逻辑全部交给 Rust；实施过程中又要求不要滚动条。当前 [计算器练习](context-calculator.md) 已升级。
- 前端每按一个键，只调用 calculator_key 传递 key；Rust 保存表达式与结果，返回可显示的内容。按钮与键盘使用同一条路径。
- 用户已从 Calculator/CalculatorView 两个 struct 的字段读到 press_key，并开始看 append；按用户的阅读顺序推进，尚未展开表达式解析。
- 已确认的类型识别：用户自主识别 Calculator.expression 是 String，result/error 是 Option 枚举，并询问 CalculatorView 是否类似。不据此推断掌握 Some/None 的全部用法或状态转换。
- 已讲解 Calculator 保存内部状态、CalculatorView 返回显示数据；View.display 是 String，没有数字 result 字段。用户继续询问 View.expression 的用途，职责区分是否已理解仍待反馈。
- 已讲解 View.expression 对应结果上方的小字算式，display 对应大字主显示；输入时前者留空，计算后分别显示原算式与结果。用户回应“ok”并继续阅读 press_key，不据此推断掌握 view() 的全部实现。
- 已确认的初步理解：用户自主描述 Rust 中有保存表达式、结果和错误状态的计算器实例；识别 clear 是给解引用后的 self 赋值、backspace 修改表达式并清空结果/错误、equals 计算并更新状态。尚不代表掌握借用或 Default trait。
- 已讲解 press_key 的分发与返回 view、snapshot 读取状态、Default 的字段初值和 equals 的执行条件；用户要求逐个说清楚，继续聚焦 pop，不把“ok”记成已掌握所有分支。
- 已讲解 pop 通过可变借用原地修改并返回 Option<char>，以及每次退格的本地 IPC 请求/响应；用户回应“明白了”，进一步追问 pop 内部是否解引用、怎样改变字符串。不据此推断已掌握全部借用规则。
- 已讲解 pop 的标准库实现：self 是 &mut String，字段访问自动解引用；pop 减去末尾 char 的 UTF-8 字节数，通过 set_len 修改内部 Vec 的长度。用户继续追问“String 本质上存了一个 vector”的含义，表示这层表示方式还需要展开。
- 已讲解 String 是含私有 Vec<u8> 字段的 struct，Vec 管理字节存储的指针、长度与容量，String 保持 UTF-8 有效性；用户回应“明白了”，继续询问“我”的字节数是否固定、字符与数字编码的关系。不据此推断已掌握指针与分配规则。
- 已讲解 Unicode 码点与 UTF-8 编码的区别，以及 String 中的字符字节数和 char 类型自身大小；用户回应“ok”，继续追问 struct 实例与 String/Vec“三元组”的关系。未逐项确认掌握编码规则。
- 已讲解 struct 类型定义与实例、String 按值包含 Vec，元信息与堆上内容分别存储；用户回应“呃，好吧”后转问 from 实现，不能视作已掌握布局或指针。
- 已沿 String::from("我") 的标准库源码讲解借用已有 UTF-8 字节、复制到 Vec 并以它构造 String；用户随后提出“String 不是标准库”的理解，需要纠正，不把已阅读源码记成已掌握库与语言的界限。
- 已讲解 Rust 的语言规则、标准库与编译器层次：String/Vec 的主体实现使用 Rust，alloc 实现 String 并由 std 重新导出；rustc 主要用 Rust 写，常用 LLVM 后端为 C++，旧版编译新版是自举，最初编译器为 OCaml。用户在 2026-10-10 回应“ok”并要求回到计算器代码，不据此推断掌握全部编译原理。
- equals 分支已按条件、evaluate 返回 Result、match 取值与更新 Option、返回 view 的顺序讲解；用户明确回应“ok 没问题”，可记录本段解释已获认可，不泛化为已掌握 Result 的全部用法。
- 已讲解 calculator.rs:42 的单竖线模式候选，多个按键共用 append(key) 分支；match 每次选一个匹配分支，再返回 view。用户回应“ok”，主动观察到 append 不只做追加，并要求讲解其内部逻辑。
- 当前讲解聚焦 append（calculator.rs:50）的输入规则：take 取出旧 Option 并置 None，if let 只处理 Some；四则运算符承接旧结果，其它输入键开始新表达式；清除旧错误；括号/数字相邻时补乘号；小数点在数的开头时补 0；最后 push_str 追加按键。不把这些输入调整说成已经计算表达式或完整语法校验。
- 当前新增语法涉及 if let、matches!、Option::take、chars().last()、is_some_and 的小函数参数 |c| 及 push/push_str；按用户反馈逐项深入，不假定一次讲解即掌握。本机 Option::take 的实际实现已核对：[option.rs:1841](</Users/zhuoru/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:1841>)。
- 用户对 append 回应“感觉大致明白了”，随后转问 Calculator 如何被使用、怎样与前端建立联系；记录为对整体逻辑的初步认可，不推断已掌握每个新增 API。
- 当前讲解连接实际调用链：lib.rs:27 创建一个 Calculator 并用 Mutex 包装后交给 manage 保存；calculator_key 是注册给前端的命令入口，State 由 Tauri 提供，lock 后调用同一个实例的 press_key；main.ts:31 的 invoke 只传 key，返回 CalculatorView 后 render 更新界面。强调实例跨按键保留、Calculator 方法不会自动成为前端命令。State/Mutex 的完整语法与内部实现仍未确认理解；依据为当前源码和 https://v2.tauri.app/develop/state-management/ 。
- 用户对调用链回应“大致明白了”，继续要求展开 Mutex 怎样包装 Calculator、calculator_key 的职责、交给 Tauri 管理实例的具体含义。当前聚焦所有权与访问：Calculator 移入 Mutex，Mutex 随 manage 移入 Tauri 状态存储；State 按类型借用已有值，lock 成功得到 MutexGuard，通过它可变访问 Calculator，guard 销毁时自动解锁。保存状态与注册命令分别负责数据保留和前端入口；这些细节尚待用户反馈。
- 用户回应“ok”后追问 map_err。当前以 lib.rs:20 说明 Result::map_err 保留 Ok、用给定小函数转换 Err；|_| 接收并忽略原错误，to_string 生成 String，与命令的错误返回类型一致。随后 ? 才负责成功取出 guard 或失败提前返回 Err；用等价 match 连接用户已读过的语法。源码核对为本机 core/src/result.rs:962 和 std/src/sync/poison.rs:235；不据“ok”泛化 Mutex/所有权已掌握。
- 用户对 map_err 回应“呃好吧，大致明白了”，转问 state。当前区分小写参数名 state 与 Tauri 类型 State；依据本机 tauri-2.12.2/src/state.rs:20 的元组 struct 定义，说明 State 包含对管理值的共享引用，当前 T 为 Mutex<Calculator>，由 Tauri 按类型提供。它借用既有 Mutex，借助 Deref 可直接调用 lock；生命周期标注只解释当前引用作用，不展开完整规则。map_err、借用、Deref 和生命周期的掌握程度仍未确认。
- 用户随后明确回应“ok 大致理解了，讲得不错”。记录 State 这段整体解释已获初步理解和讲解方式认可；继续保持当前代码、单点拆解的粒度，不推断已掌握全部生命周期或 Deref 规则。
- 在喝水提醒起点建议后，用户询问是否 Tauri 架构中由 Rust 打开窗口。当前澄清：Rust 控制提醒窗口是建议的应用分工；Tauri 也提供 JavaScript 窗口创建 API，最终由原生层创建窗口，内部前端内容通过 WebView 显示。现有计算器主窗口由 tauri.conf.json 的 app.windows 在启动时创建。尚未确认此区分已理解或提醒窗口方案已确定。
- 用户进一步询问 Rust 的职责是否相当于“内核”或“调度器”。当前从启动与组成澄清：Rust 是语言，Rust 写出的本地主程序对应 Tauri Core Process；它包含 Tauri 的窗口/IPC/状态等框架能力及我们写的 Calculator 等业务代码，WebView 执行前端页面。应用核心运行在 macOS 之上，计时调度仅是未来可能承担的一项业务，不等同于操作系统内核。依据为当前 main.rs/lib.rs 及 https://v2.tauri.app/concept/process-model/ 。用户对这层定位的理解尚待反馈。
- 用户随后自主概括“Tauri 也是 Rust 写出来的代码，提供与操作系统交互的 API”，并继续询问第一步写什么。这个理解准确涵盖原生核心；Tauri 另有 JS/TS 接口，避免泛化成所有组件只用 Rust。已实施 [手动提醒第一步](context-reminder.md)，下一段适合从 reminder.rs 的 remind_now 阅读，AppHandle/Manager/WebviewWindowBuilder/async 的完整语法尚未讲解，不能因构建通过记成已掌握。
- 本轮依据：本机 std/src/lib.rs:574 的 alloc_crate::string 再导出；https://rustc-dev-guide.rust-lang.org/overview.html（编译流程与 rustc 自身使用 Rust）、https://rustc-dev-guide.rust-lang.org/building/bootstrapping/intro.html（自举与 OCaml 起点）、https://llvm.org/docs/FAQ.html#in-what-language-is-llvm-written（LLVM 使用 C++）。是否理解语言/库/编译器区分待反馈。
- 本机 Rust 1.94.1 源码链已核对：[From<&str>](</Users/zhuoru/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/string.rs:3110>) → [str::to_owned](</Users/zhuoru/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/str.rs:206>) → [切片 to_owned](</Users/zhuoru/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs:836>) / [to_vec_in](</Users/zhuoru/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs:396>) → [from_utf8_unchecked](</Users/zhuoru/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/string.rs:1013>)。已调用 Codex 文件预览打开 from 所在位置，工具返回 queued，尚无用户可见确认。
- 2026-10-09 本机临时 Rust 程序核对：size_of::<String>() 与 size_of::<Vec<u8>>() 都为 24 字节，usize 为 8 字节；内容分别为“我”和十个“我”时 String 实例大小均为 24，内容长度分别为 3 和 30 字节。这是当前工具链/平台的测量，不泛化为所有平台的固定大小；未改项目源码。
- 2026-10-09 用本机 Rust 编译运行的临时示例已核对：“我”的码点 U+6211（25105），UTF-8 为 [230, 136, 145]，String 中占 3 字节；Rust char 类型占 4 字节。临时程序完成后清理，未改项目源码。用户对这层区分的理解待反馈。
- 核对依据：Rust String::pop 官方文档 https://doc.rust-lang.org/std/string/struct.String.html#method.pop；本项目 main.ts 的 sendKey、calculator.rs 的 backspace 分支及 [通信 Context](context-architecture.md)。未测量当前应用的 IPC 时延，不给出毫秒估计。
- 标准库实现已核对本机 rustc 1.94.1 的 rust-src：[String::pop](</Users/zhuoru/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/string.rs:1497>)、[Vec::set_len](</Users/zhuoru/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2099>)。字段自动解引用依据：https://doc.rust-lang.org/reference/expressions/field-expr.html#automatic-dereferencing 。
- String/Vec 表示的本机证据：[String 定义](</Users/zhuoru/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/string.rs:353>)、同文件 Representation 文档，以及 [Vec 定义](</Users/zhuoru/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:440>)。不把概念上的指针/长度/容量三项顺序写成保证的内存字段顺序。
- f64、match、Result/Ok/Err、impl、State/Mutex、递归解析尚未逐项确认理解；代码实现和测试通过不代表用户已学会。

## 后续讲解方式
- 原手动提醒后的计时路线已被 2026-10-12 的纯菜单栏范围替换；不自动推进旧计划。
- 一轮集中解决一个当前疑问，用当前示例和少量代码连接输入、处理、输出。
- 新名词出现时用一句普通话解释，避免同时展开多条背景知识；框架实现细节按用户提问逐步深入。
- 区分“已讲解”“用户已确认理解”“仍有疑问”；依据用户反馈更新，避免每轮机械测验。
- 原始代码与架构证据见 [示例架构与通信](context-architecture.md)，不在这里复制源码。
