# 检查更新与 GitHub Release

## 用户要求与实现决定（2026-10-13）
- 用户要求支持“检查更新”，同步仓库到自己的 GitHub，并通过 Release 发布；授权助手选择合适实现。
- 第一版采用菜单手动检查：后台请求 GitHub 最新正式 Release，比较版本；有新版时用原生对话框提供“下载新版”，打开该 Release 页面，再由用户替换安装。没有引入应用内自动安装。
- 首次带更新检查的发布为 0.1.1，三个版本来源 package.json、Cargo.toml、tauri.conf.json 保持一致；不移动旧 v0.1.0 标签。
- GitHub CLI 已验证登录账号 rUrU516。用户回复确认创建**公开仓库 rUrU516/gulu**；已创建、同步 main 与历史标签并发布 v0.1.1，不再重复请求可见性或发布确认。
- 代码和 README 使用 rUrU516/gulu。公开 Release 可匿名下载与检查，应用不嵌入用户 GitHub token。

## 实现证据
- [updates.rs](../src-tauri/src/updates.rs)：reqwest 异步请求 GitHub /repos/rUrU516/gulu/releases/latest；按 SemVer 比较，草稿和预览不提供更新。新版必须已有 gulu_版本_aarch64.dmg，避免展示尚未准备好的安装包。
- 网络请求只发生在点击“检查更新”时，15 秒超时；请求中没有 GitHub 登录凭据。403/429、404、连接失败、JSON 或版本格式错误均显示错误，与“已是最新版本”区分。
- [lib.rs](../src-tauri/src/lib.rs) 增加“检查更新”，请求时菜单暂时变成“正在检查…”，防止重复点击；结果回到主线程，通过 NSAlert 显示，NSWorkspace 打开下载网页。原五分钟提醒保持不变。
- NSAlert 是原生 modal 对话框；显示期间默认 run loop 模式的动画/提醒 timer 可能延后，网络等待本身不阻塞主线程。
- Cargo 使用 reqwest 0.13 的 JSON、native-tls 和 system-proxy，以及 semver、serde；AppKit/Foundation 仅增加 NSAlert、NSApplication、NSWorkspace、NSURL 绑定，不添加前端。

## 发布流程（已执行成功）
- 已核对 GitHub 官方权限文档与 gh auth status：当前登录没有 workflow scope，上传 Actions 工作流需要额外授权。初始云端发布草案已撤回，不要求用户扩大凭据权限；改为本地 `pnpm release` 完成首次和后续发布，已向用户说明。
- [scripts/publish-release.mjs](../scripts/publish-release.mjs) 检查三个版本来源一致与源码已提交，构建 Apple 芯片 .app、.dmg，校验后推送代码/版本标签，上传到草稿 Release，再对外发布。旧标签指向其他提交时使用新版本，不覆盖历史标签。
- 使用 ad-hoc Apple 签名，与既有本地包条件一致；无需新增 Apple 证书或更新私钥。原生更新检查是版本比较与下载入口，不需要 Tauri updater 安装签名私钥。
- README 为对外安装/更新说明；详细开发叙事仍维护在 agent-context/。
- `pnpm release` 已从提交 1c6ea6cd9e089971db97492159a01d6a3836b190 成功构建、校验、创建 v0.1.1 标签、推送 main/标签、上传安装包并发布正式 Release；该标签对应此源码提交，后续文档提交不改变发布源码。
- 仓库：https://github.com/rUrU516/gulu ，origin 为 https://github.com/rUrU516/gulu.git，main 跟踪 origin/main，历史 learning-baseline 与 v0.1.0 标签也已推送。
- Release：https://github.com/rUrU516/gulu/releases/tag/v0.1.1 ，isDraft=false、isPrerelease=false，资产 gulu_0.1.1_aarch64.dmg 已上传，大小 2,194,804 字节，约 2.09 MiB。
- 固定最新下载页面：https://github.com/rUrU516/gulu/releases/latest 。之后升版本、提交再执行 pnpm release，不覆盖已发布标签。
- 升版本后先运行 cargo check --manifest-path src-tauri/Cargo.toml 同步 Cargo.lock，再提交所有版本/锁文件改动。发布脚本在构建前后都核实源码未留下未提交改动，保证标签与实际构建的锁文件一致；该辅助脚本完善不改变 0.1.1 应用产物。

## 已验证与待验证
- 本地 cargo test --locked --lib：3 个版本判断测试通过；发布后单独执行 ignored 的 published_release_can_be_checked_without_github_login，真实 HTTPS 请求通过，模拟旧版本能识别更新，当前版本请求也成功。应用请求没有使用 GitHub 登录凭据。
- 开发依赖新增时，旧离线 dev watcher 因依赖尚未下载而退出；依赖下载与测试成功后已重新启动 pnpm tauri dev，日志确认 300 秒提醒注册成功。
- 0.1.1 Apple 芯片 release 本地构建、arm64 检查、版本元数据、codesign 深度校验、hdiutil DMG 完整性校验已通过；发布脚本 Node 语法检查、dry-run 与完整发布实际执行通过。
- 匿名 curl 下载 Release 安装包后逐字节与本地发布文件比较通过；SHA-256 为 24a44ddf1a6f07c62f47f8a196644bc70a3dd7dd74cb4cbea0f8df4028ff4bfb，也与 GitHub 资产 digest 一致。下载校验文件保留在被忽略的 src-tauri/target/release-download-check/，不属于源码。
- 开发日志在发布前已记录一次菜单更新检查返回“暂时无法找到发布版本”，说明实际回调/网络路径已运行；不能据此推断操作者或原生提示的视觉表现。发布后用户对“已是最新版本 / 咕噜 0.1.1”的验证问题回复“显示了最新版提示”，确认当前版本的原生提示显示成功。有新版时的“下载新版”按钮与浏览器打开流程尚未通过实际界面验证。
- CUA 首次用 Bundle ID 绑定时因本机两个 gulu.app 路径而返回歧义；指定正式包路径后工具仍超时，但实际启动了一个额外进程。已按真实 PID 停止这个测试进程，保留开发版，未操作其他应用绕过；当前版本提示的界面验证依据为用户反馈。
- 已核对当前 SSH 连接不通（MAC 协商、严格 host key 检查未通过），未修改全局 SSH 配置、添加主机信任或使用未验证主机密钥；后续仓库同步使用已登录的 HTTPS GitHub 凭据。

## 0.1.2 更新验证版本（已发布）
- 用户在确认 0.1.1 的最新版提示后，要求“把版本往前推一个，然后我再试试”；本轮已发布正式 0.1.2，仅修改应用版本号，提醒与更新逻辑不变。
- 原运行进程为 pnpm tauri dev，修改版本会触发 watcher 自动重启。已先复制并校验现有 0.1.1 正式 .app 到被忽略的 src-tauri/target/update-test/gulu-0.1.1.app，停止原开发进程，单独启动这个旧版本，保留 0.1.1 → 0.1.2 的实际验证条件。
- package.json、Cargo.toml、tauri.conf.json 和 Cargo.lock 已同步为 0.1.2；cargo check --offline 通过，发布源码提交与 v0.1.2 标签均为 81d05492c01b66341d8f40204a9240db448a7ab7。pnpm release 已完成构建、签名/DMG 校验、推送与正式发布；旧 v0.1.1 标签保留。
- 正式 Release：https://github.com/rUrU516/gulu/releases/tag/v0.1.2 。匿名请求 releases/latest 已核实返回 v0.1.2，非草稿、非预览，资产 gulu_0.1.2_aarch64.dmg 已上传。Release 说明明确本次仅升级版本号，用于验证更新流程。
- 本地分享文件为 releases/gulu-0.1.2-apple-silicon.dmg，大小 2,193,763 字节，SHA-256 为 05e043bc6c43f61e955c3ea185b187aa6654c8e87d809e7832383d9003e32580；匿名下载与本地逐字节比较通过，摘要与 GitHub digest 一致。应用为 arm64，Info.plist 版本为 0.1.2，签名条件沿用此前。
- 发布后的 published_release_can_be_checked_without_github_login 真实 HTTPS 测试通过。保留运行的旧版本路径见上文；新版原生提示和“下载新版”按钮的实际效果仍由用户随后测试，未写成已验证。

## 官方依据
- GitHub Release API：https://docs.github.com/en/rest/releases/releases#get-the-latest-release 。
- Tauri GitHub 发布：https://v2.tauri.app/distribute/pipelines/github/ 。
- Runner：https://docs.github.com/en/actions/reference/runners/github-hosted-runners 。
- 已查 Tauri updater：https://v2.tauri.app/plugin/updater/ ，该插件支持签名校验后安装；当前第一版只检查与打开下载入口，未接入该安装插件。
- GitHub OAuth scopes：https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps 。
