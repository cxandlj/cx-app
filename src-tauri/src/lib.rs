mod regex_tool;
mod update_tool;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, WindowEvent,
};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_updater::UpdaterExt;

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

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let answer = app
            .dialog()
            .message("存在新版本，是否更新？")
            .title("更新提示")
            .buttons(tauri_plugin_dialog::MessageDialogButtons::OkCancel)
            .blocking_show();

        if !answer {
            return Ok(());
        }
        let mut downloaded = 0;

        // alternatively we could also call update.download() and update.install() separately
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // let handle = app.handle().clone();
            // tauri::async_runtime::spawn(async move {
            //     update(handle).await.unwrap();
            // });

            app.manage(update_tool::PendingUpdate(Mutex::new(None)));
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
            copy_text,
            update_tool::fetch_update,
            update_tool::install_update
        ])
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
