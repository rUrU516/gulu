# Apple 芯片版打包与分发

## 当前可分发版本 0.1.1
- 用户后来要求检查更新与 GitHub Release，并确认公开仓库 rUrU516/gulu；0.1.1 已成功发布：https://github.com/rUrU516/gulu/releases/tag/v0.1.1 。
- 最新下载页面：https://github.com/rUrU516/gulu/releases/latest ，可直接分享这个固定入口。
- 本地副本：[gulu-0.1.1-apple-silicon.dmg](../releases/gulu-0.1.1-apple-silicon.dmg)，大小 2,194,804 字节；SHA-256 为 24a44ddf1a6f07c62f47f8a196644bc70a3dd7dd74cb4cbea0f8df4028ff4bfb。
- arm64、版本号、应用签名、DMG 完整性、匿名下载与本地文件一致性均已验证。仍为 ad-hoc 本地签名、未 Apple 公证。接收方先安装 0.1.1，才有“检查更新”菜单；当前入口是检查后打开新版下载页、退出旧应用并替换安装。
- 后续发布命令及源码标签见 [检查更新与 GitHub 发布](context-updates.md)。下文 0.1.0 记录作为首次本地分发历史保留。

## 用户要求与决定
- 2026-10-13 用户要求将当前每 5 分钟自动提醒的版本打包，方便分享给女朋友，并明确主要面向 Apple 芯片。
- 只构建 aarch64-apple-darwin 的正式版本，保留当前图标、菜单和动画，不改变产品行为。
- 分享产物放在项目 releases/，由 .gitignore 忽略；源码和构建记录保存在 Git。没有上传、发布到远端或向他人发送文件。

## 已生成的产物
- 可分享文件：[gulu-0.1.0-apple-silicon.dmg](../releases/gulu-0.1.0-apple-silicon.dmg)。大小 1,953,818 字节，约 1.86 MiB。
- SHA-256：f3e1e4ef791ee03f71340962d6db786d7cbc0aed422b263d50421404ce469edb。
- 构建源码提交：86cbf8a6c31332293416b61913aa4311968a42fb；版本号 0.1.0、Identifier com.zhuoru.gulu、应用文件名 gulu.app。
- Tauri 原始输出为 src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/gulu_0.1.0_aarch64.dmg，正式应用输出为同级 bundle/macos/gulu.app。releases/ 中是原 DMG 的直接副本。
- 旧 v0.1.0 标签是纯图标的历史检查点，保持不动；不要把它当作这次包对应的源码提交。

## 构建与签名条件
在项目目录执行：

```sh
CI=true APPLE_SIGNING_IDENTITY=- CARGO_NET_OFFLINE=true pnpm tauri build --target aarch64-apple-darwin --bundles app,dmg
```

- release 构建成功，.app 约 4.00 MiB；不是 debug 构建。
- CI=true 跳过需要 Finder AppleScript 的 DMG 布局定制；包里仍有应用和指向 /Applications 的安装快捷方式。
- 已读取本机 codesigning 身份，没有 Developer ID Application 分发证书。已有两个 SpatialCaptureProbe 本地证书与本项目无关，未使用。
- APPLE_SIGNING_IDENTITY=- 生成 ad-hoc 签名；应用签名校验通过，但没有 Apple 信任证书或公证。构建器明确记录跳过 notarization。
- 这是可分享的本地测试包；接收方首次打开可能被 Gatekeeper 阻止。正式无此提示的对外分发仍需要 Developer ID 签名与公证，当前没有实施。
- 用户明确 Apple 芯片范围后，停止了未完成的 Intel Rust target 下载；没有生成 universal/Intel 包。

## 已验证事实与限制
- file 确认应用主程序为 Mach-O arm64。
- codesign --verify --deep --strict 通过；codesign -dv 显示 Signature=adhoc、TeamIdentifier 未设置。
- Info.plist 核实版本 0.1.0、Identifier com.zhuoru.gulu、LSUIElement=true（菜单栏应用）。
- otool -L 显示依赖均为 macOS 系统库或系统 Framework，没有本机开发目录的动态库路径。
- hdiutil verify 确认 DMG 校验和有效。只读挂载检查包含 gulu.app 与 Applications -> /Applications，包内签名再次校验通过，主程序与原构建产物逐字节一致；检查后已卸载镜像并移除临时挂载目录。
- 当前开发应用仍在运行，没有额外启动正式包；动画此前已获用户确认，300 秒配置的实际自动回调已在开发日志中出现。接收方机器、下载后的 Gatekeeper 行为及跨 macOS 版本兼容性未实测，不写成已验证。

## 转发与安装说明
- 发给对方 releases/gulu-0.1.0-apple-silicon.dmg 即可；不需要 Rust、pnpm 或源码。
- 对方打开 DMG，把 gulu.app 拖到 Applications（应用程序），再启动应用。入口在顶部菜单栏水滴，无主窗口；启动 5 分钟后首次提示“记得喝水”，以后每 5 分钟重复。菜单可“试试提醒”或“退出”。
- 若系统因开发者未验证/未公证而阻止，先尝试打开，然后按 Apple 官方说明到“系统设置 → 隐私与安全 → 仍要打开”。没有执行删除隔离属性或关闭 Gatekeeper 等全局安全设置操作。
- Apple 官方说明：https://support.apple.com/zh-cn/102445 。
- Tauri DMG：https://v2.tauri.app/distribute/dmg/ 。签名：https://v2.tauri.app/distribute/sign/macos/ 。
