# 检查更新与 GitHub Release

## 用户要求与实现决定（2026-10-13）
- 用户要求支持“检查更新”，同步仓库到自己的 GitHub，并通过 Release 发布；授权助手选择合适实现。
- 第一版采用菜单手动检查：后台请求 GitHub 最新正式 Release，比较版本；有新版时用原生对话框提供“下载新版”，打开该 Release 页面，再由用户替换安装。没有引入应用内自动安装。
- 下一版为 0.1.1，三个版本来源 package.json、Cargo.toml、tauri.conf.json 保持一致；不移动旧 v0.1.0 标签。
- GitHub CLI 已验证登录账号 rUrU516，原仓库没有 remote，该账号下查询不到 gulu 仓库。用户已回复确认创建**公开仓库 rUrU516/gulu**；创建、推送、发布正在继续执行，不再重复请求可见性确认。
- 代码和 README 使用 rUrU516/gulu。公开 Release 可匿名下载与检查，应用不嵌入用户 GitHub token。

## 实现证据
- [updates.rs](../src-tauri/src/updates.rs)：reqwest 异步请求 GitHub /repos/rUrU516/gulu/releases/latest；按 SemVer 比较，草稿和预览不提供更新。新版必须已有 gulu_版本_aarch64.dmg，避免展示尚未准备好的安装包。
- 网络请求只发生在点击“检查更新”时，15 秒超时；请求中没有 GitHub 登录凭据。403/429、404、连接失败、JSON 或版本格式错误均显示错误，与“已是最新版本”区分。
- [lib.rs](../src-tauri/src/lib.rs) 增加“检查更新”，请求时菜单暂时变成“正在检查…”，防止重复点击；结果回到主线程，通过 NSAlert 显示，NSWorkspace 打开下载网页。原五分钟提醒保持不变。
- NSAlert 是原生 modal 对话框；显示期间默认 run loop 模式的动画/提醒 timer 可能延后，网络等待本身不阻塞主线程。
- Cargo 使用 reqwest 0.13 的 JSON、native-tls 和 system-proxy，以及 semver、serde；AppKit/Foundation 仅增加 NSAlert、NSApplication、NSWorkspace、NSURL 绑定，不添加前端。

## 发布流程（已准备，远端尚待执行）
- 已核对 GitHub 官方权限文档与 gh auth status：当前登录没有 workflow scope，上传 Actions 工作流需要额外授权。初始云端发布草案已撤回，不要求用户扩大凭据权限；改为本地 `pnpm release` 完成首次和后续发布，已向用户说明。
- [scripts/publish-release.mjs](../scripts/publish-release.mjs) 检查三个版本来源一致与源码已提交，构建 Apple 芯片 .app、.dmg，校验后推送代码/版本标签，上传到草稿 Release，再对外发布。旧标签指向其他提交时使用新版本，不覆盖历史标签。
- 使用 ad-hoc Apple 签名，与既有本地包条件一致；无需新增 Apple 证书或更新私钥。原生更新检查是版本比较与下载入口，不需要 Tauri updater 安装签名私钥。
- README 为对外安装/更新说明；详细开发叙事仍维护在 agent-context/。
- 发布后的预定验证：确认 Release 资产和匿名下载；运行 updates::tests::published_release_can_be_checked_without_github_login（ignored 的实际 HTTPS 检查）。原生菜单交互检查受 CUA 绑定超时限制，不能据此声称视觉效果已通过。

## 已验证与待验证
- 本地 cargo test --lib：3 个版本判断测试通过；实际 GitHub 请求测试显式 ignored，等待公开 Release 后执行，不将其写成已通过。
- 开发依赖新增时，旧离线 dev watcher 因依赖尚未下载而退出；依赖下载与测试成功后已重新启动 pnpm tauri dev，日志确认 300 秒提醒注册成功。
- 0.1.1 Apple 芯片 release 本地构建、arm64 检查、版本元数据、codesign 深度校验、hdiutil DMG 完整性校验已通过；发布脚本 Node 语法检查与 dry-run 通过。最终发布会从提交后的源码再次构建。
- CUA 首次用 Bundle ID 绑定时因本机两个 gulu.app 路径而返回歧义；指定正式包路径后工具仍超时，但实际启动了一个额外进程。已按真实 PID 停止这个测试进程，保留开发版，未操作其他应用绕过；原生对话框视觉效果待用户确认。
- 已核对当前 SSH 连接不通（MAC 协商、严格 host key 检查未通过），未修改全局 SSH 配置、添加主机信任或使用未验证主机密钥；后续仓库同步使用已登录的 HTTPS GitHub 凭据。

## 官方依据
- GitHub Release API：https://docs.github.com/en/rest/releases/releases#get-the-latest-release 。
- Tauri GitHub 发布：https://v2.tauri.app/distribute/pipelines/github/ 。
- Runner：https://docs.github.com/en/actions/reference/runners/github-hosted-runners 。
- 已查 Tauri updater：https://v2.tauri.app/plugin/updater/ ，该插件支持签名校验后安装；当前第一版只检查与打开下载入口，未接入该安装插件。
- GitHub OAuth scopes：https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps 。
