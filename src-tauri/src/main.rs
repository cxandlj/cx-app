#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu};
mod regex_tool;
use arboard::Clipboard;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let close = CustomMenuItem::new("close".to_string(), "关闭窗口");
    let submenu = Submenu::new("常用", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(submenu)
        .add_item(CustomMenuItem::new("hide", "隐藏"));
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            // tauri::api::dialog::blocking::message(Some(&main_window), "Hello", "Welcome back!");
            Ok(())
        })
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            "hide" => {
                event.window().hide().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            match_text,
            replace_text,
            copy_text
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn match_text(input: &str, regexStr: &str, options: Vec<&str>) -> Result<String, String> {
    match regex_tool::match_text(input, regexStr, options) {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn replace_text(
    input: &str,
    regexStr: &str,
    replaceStr: &str,
    options: Vec<&str>,
) -> Result<String, String> {
    match regex_tool::replace_text(input, regexStr, replaceStr, options) {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn copy_text(txt: String) -> Result<String, String> {
    match Clipboard::new() {
        Ok(mut cb) => match cb.set_text(txt) {
            Ok(result) => Ok("".to_string()),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}
