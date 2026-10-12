use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

// 前端点击“测试提醒”时调用；窗口的内容来自 reminder.html。
#[tauri::command]
pub async fn remind_now(app: AppHandle) -> Result<(), String> {
    let window = if let Some(window) = app.get_webview_window("reminder") {
        window
    } else {
        WebviewWindowBuilder::new(&app, "reminder", WebviewUrl::App("reminder.html".into()))
            .title("咕噜 · 喝口水吧")
            .inner_size(340.0, 280.0)
            .build()
            .map_err(|error| error.to_string())?
    };

    window.show().map_err(|error| error.to_string())?;
    window.set_focus().map_err(|error| error.to_string())
}
