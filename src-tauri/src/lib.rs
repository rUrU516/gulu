use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

#[cfg(target_os = "macos")]
mod reminder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // macOS 菜单栏应用：不在 Dock 中显示图标。
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            #[cfg(target_os = "macos")]
            let preview = MenuItem::with_id(app, "preview", "试试提醒", true, None::<&str>)?;
            #[cfg(target_os = "macos")]
            let menu = Menu::with_items(app, &[&preview, &quit])?;
            #[cfg(not(target_os = "macos"))]
            let menu = Menu::with_items(app, &[&quit])?;

            // 点击菜单栏图标打开菜单；Tauri 保存图标直到应用退出。
            TrayIconBuilder::with_id("gulu")
                .icon(tauri::include_image!("icons/tray.png"))
                .icon_as_template(true)
                .tooltip("咕噜")
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| {
                    #[cfg(target_os = "macos")]
                    if event.id().as_ref() == "preview" {
                        if let Err(error) = reminder::play(app) {
                            eprintln!("无法播放喝水提醒：{error}");
                        }
                    }
                    if event.id().as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
