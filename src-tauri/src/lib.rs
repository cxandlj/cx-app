mod regex_tool;
use arboard::Clipboard;
use tauri::{
    menu::{Menu, MenuBuilder, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
fn match_text(input: &str, regex_str: &str, options: Vec<&str>) -> Result<String, String> {
    match regex_tool::match_text(input, regex_str, options) {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn replace_text(
    input: &str,
    regex_str: &str,
    replace_str: &str,
    options: Vec<&str>,
) -> Result<String, String> {
    match regex_tool::replace_text(input, regex_str, replace_str, options) {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn copy_text(txt: String, app: tauri::AppHandle) -> Result<String, String> {
    // match Clipboard::new() {
    //     Ok(mut cb) => match cb.set_text(txt) {
    //         Ok(result) => Ok("".to_string()),
    //         Err(err) => Err(err.to_string()),
    //     },
    //     Err(err) => Err(err.to_string()),
    // }
    match app.clipboard().write_text(txt) {
        Ok(_) => Ok("".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // let menu = MenuBuilder::new(app)
            //     .text("open", "Open")
            //     .text("close", "Close")
            //     .build()?;

            // app.set_menu(menu)?;

            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .build(app)?;
            Ok(())
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .on_window_event(|window_, event| match event {
            WindowEvent::Resized(_) => {
                // 检查窗口是否最小化
                if window_.is_minimized().unwrap_or(false) {
                    // 如果是最小化状态，隐藏窗口
                    let _ = window_.hide();
                }
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested { api, .. } => {
                window_.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            match_text,
            replace_text,
            copy_text
        ])
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
