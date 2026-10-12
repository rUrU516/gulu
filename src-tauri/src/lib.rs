mod calculator;
mod expression;
mod reminder;

#[cfg(test)]
mod calculator_tests;

use std::sync::Mutex;
use tauri::State;

use calculator::{Calculator, CalculatorView};
use reminder::remind_now;

// 前端每按一个键就调用一次；表达式由 Rust 保存和处理。
#[tauri::command]
fn calculator_key(
    key: &str,
    state: State<'_, Mutex<Calculator>>,
) -> Result<CalculatorView, String> {
    let mut calculator = state
        .lock()
        .map_err(|_| "计算器暂时无法使用，请重新打开应用".to_string())?;
    calculator.press_key(key)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Calculator::default()))
        .invoke_handler(tauri::generate_handler![calculator_key, remind_now])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
