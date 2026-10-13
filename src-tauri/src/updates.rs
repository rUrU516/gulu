use std::time::Duration;

use objc2::MainThreadMarker;
use objc2_app_kit::{NSAlert, NSAlertFirstButtonReturn, NSApplication, NSWorkspace};
use objc2_foundation::{NSString, NSURL};
use semver::Version;
use serde::Deserialize;
use tauri::{menu::MenuItem, AppHandle};

const REPOSITORY: &str = "rUrU516/gulu";

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    draft: bool,
    prerelease: bool,
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
}

#[derive(Debug, PartialEq)]
enum UpdateStatus {
    Current,
    Available { version: String, url: String },
}

// 网络请求放在后台，菜单栏和喝水动画仍由主线程处理。
pub fn check(app: &AppHandle, item: &MenuItem<tauri::Wry>) -> tauri::Result<()> {
    if !item.is_enabled()? {
        return Ok(());
    }
    item.set_enabled(false)?;
    item.set_text("正在检查…")?;
    let app = app.clone();
    let item = item.clone();
    let current = app.package_info().version.to_string();

    tauri::async_runtime::spawn(async move {
        let result = fetch_latest(&current).await;
        if let Err(error) = &result {
            eprintln!("检查更新失败：{error}");
        }
        if let Err(error) = app.run_on_main_thread(move || {
            show_result(&current, result);
            if let Err(error) = item
                .set_text("检查更新")
                .and_then(|_| item.set_enabled(true))
            {
                eprintln!("无法恢复检查更新菜单：{error}");
            }
        }) {
            eprintln!("无法显示检查更新结果：{error}");
        }
    });
    Ok(())
}

async fn fetch_latest(current: &str) -> Result<UpdateStatus, String> {
    let client = reqwest::Client::builder()
        .user_agent(format!("gulu/{current}"))
        .https_only(true)
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|error| format!("无法创建网络连接：{error}"))?;
    let response = client
        .get(format!(
            "https://api.github.com/repos/{REPOSITORY}/releases/latest"
        ))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2026-03-10")
        .send()
        .await
        .map_err(|_| "连接 GitHub 失败，请检查网络后重试。".to_string())?;
    match response.status() {
        reqwest::StatusCode::NOT_FOUND => {
            return Err("暂时无法找到发布版本，请稍后再试。".into());
        }
        reqwest::StatusCode::FORBIDDEN | reqwest::StatusCode::TOO_MANY_REQUESTS => {
            return Err("GitHub 暂时限制了请求，请稍后再试。".into());
        }
        status if !status.is_success() => {
            return Err(format!("GitHub 暂时无法提供更新信息（{status}）。"));
        }
        _ => {}
    }
    let release = response
        .json::<Release>()
        .await
        .map_err(|_| "无法读取发布信息，请稍后再试。".to_string())?;
    evaluate_release(current, &release)
}

fn evaluate_release(current: &str, release: &Release) -> Result<UpdateStatus, String> {
    let current = Version::parse(current).map_err(|_| "当前版本号无效。")?;
    let latest = Version::parse(
        release
            .tag_name
            .strip_prefix('v')
            .unwrap_or(&release.tag_name),
    )
    .map_err(|_| "发布版本号无效。")?;
    if release.draft
        || release.prerelease
        || !latest.pre.is_empty()
        || latest.cmp_precedence(&current).is_le()
    {
        return Ok(UpdateStatus::Current);
    }
    let expected_asset = format!("gulu_{latest}_aarch64.dmg");
    if !release
        .assets
        .iter()
        .any(|asset| asset.name == expected_asset)
    {
        return Err("新版安装包尚未准备好，请稍后再试。".into());
    }
    Ok(UpdateStatus::Available {
        version: latest.to_string(),
        url: format!(
            "https://github.com/{REPOSITORY}/releases/tag/{}",
            release.tag_name
        ),
    })
}

fn show_result(current: &str, result: Result<UpdateStatus, String>) {
    let Some(mtm) = MainThreadMarker::new() else {
        eprintln!("更新提示必须在 macOS 主线程显示");
        return;
    };
    NSApplication::sharedApplication(mtm).activate();
    match result {
        Ok(UpdateStatus::Current) => {
            alert(mtm, "已是最新版本", &format!("咕噜 {current}"), false);
        }
        Ok(UpdateStatus::Available { version, url }) => {
            if alert(
                mtm,
                &format!("发现新版本 {version}"),
                &format!(
                    "当前版本：{current}\n下载后先退出咕噜，再将新版拖入“应用程序”替换旧版并启动。"
                ),
                true,
            ) {
                let opened = NSURL::URLWithString(&NSString::from_str(&url))
                    .is_some_and(|url| NSWorkspace::sharedWorkspace().openURL(&url));
                if !opened {
                    alert(mtm, "无法打开下载页面", &url, false);
                }
            }
        }
        Err(message) => {
            alert(mtm, "暂时无法检查更新", &message, false);
        }
    }
}

fn alert(mtm: MainThreadMarker, title: &str, message: &str, download: bool) -> bool {
    let alert = NSAlert::new(mtm);
    alert.setMessageText(&NSString::from_str(title));
    alert.setInformativeText(&NSString::from_str(message));
    alert.addButtonWithTitle(&NSString::from_str(if download {
        "下载新版"
    } else {
        "好"
    }));
    if download {
        let cancel = alert.addButtonWithTitle(&NSString::from_str("稍后"));
        cancel.setKeyEquivalent(&NSString::from_str("\u{1b}"));
    }
    alert.runModal() == NSAlertFirstButtonReturn
}

#[cfg(test)]
mod tests {
    use super::*;

    fn release(tag: &str, assets: &[&str]) -> Release {
        Release {
            tag_name: tag.into(),
            draft: false,
            prerelease: false,
            assets: assets
                .iter()
                .map(|name| Asset {
                    name: (*name).into(),
                })
                .collect(),
        }
    }

    #[test]
    fn compares_versions_numerically_and_requires_apple_silicon_installer() {
        let latest = release("v0.1.10", &["gulu_0.1.10_aarch64.dmg"]);
        assert_eq!(
            evaluate_release("0.1.9", &latest).unwrap(),
            UpdateStatus::Available {
                version: "0.1.10".into(),
                url: "https://github.com/rUrU516/gulu/releases/tag/v0.1.10".into(),
            }
        );
        assert!(evaluate_release("0.1.9", &release("v0.1.10", &["gulu_0.1.10_x64.dmg"])).is_err());
    }

    #[test]
    fn equal_older_and_preview_releases_do_not_offer_updates() {
        for tag in ["v0.1.1", "v0.1.1+build.2", "v0.1.0", "v0.2.0-beta.1"] {
            assert_eq!(
                evaluate_release("0.1.1", &release(tag, &[])).unwrap(),
                UpdateStatus::Current
            );
        }
        let mut latest = release("v0.2.0", &["gulu_0.2.0_aarch64.dmg"]);
        latest.draft = true;
        assert_eq!(
            evaluate_release("0.1.1", &latest).unwrap(),
            UpdateStatus::Current
        );
        latest.draft = false;
        latest.prerelease = true;
        assert_eq!(
            evaluate_release("0.1.1", &latest).unwrap(),
            UpdateStatus::Current
        );
    }

    #[test]
    fn malformed_versions_are_errors_instead_of_claiming_up_to_date() {
        assert!(evaluate_release("0.1.1", &release("latest", &[])).is_err());
        assert!(evaluate_release("bad", &release("v0.2.0", &[])).is_err());
    }

    #[test]
    #[ignore = "需要公开仓库已发布 Release；会实际请求 GitHub"]
    fn published_release_can_be_checked_without_github_login() {
        let status = tauri::async_runtime::block_on(fetch_latest("0.0.0")).unwrap();
        assert!(matches!(status, UpdateStatus::Available { .. }));
        tauri::async_runtime::block_on(fetch_latest(env!("CARGO_PKG_VERSION"))).unwrap();
    }
}
