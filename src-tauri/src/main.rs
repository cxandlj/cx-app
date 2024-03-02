#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{rc::Rc, sync::Arc, thread::JoinHandle};

use enigo::*;
use tauri::{
    async_runtime::{Sender, TokioJoinHandle},
    CustomMenuItem, Manager, Menu, MenuItem, Submenu, WindowMenuEvent,
};
mod regex_tool;
use arboard::Clipboard;
use tauri::{SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tokio::{
    sync::{mpsc, Mutex},
    task,
    time::{self, Duration},
};

#[tokio::main]
async fn main() {
    let time = CustomMenuItem::new("time".to_string(), "定时");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let close = CustomMenuItem::new("close".to_string(), "关闭窗口");
    let submenu = Submenu::new(
        "常用",
        Menu::new().add_item(time).add_item(quit).add_item(close),
    );

    let (tx, mut rx) = mpsc::channel(32);
    let tx_clone: Sender<WindowMenuEvent> = tx.clone();

    let menu = Menu::new()
        // .add_native_item(MenuItem::Copy)
        .add_submenu(submenu)
        .add_item(CustomMenuItem::new("hide", "隐藏"));
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "退出"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("hide", "隐藏"));
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .setup(|app| {
            tokio::spawn(async move {
                let mut time_flag = Mutex::new(false);

                let mut handler: Arc<Option<TokioJoinHandle<_>>> = Arc::new(None);
                while let Some(event) = rx.recv().await {
                    println!("recv");
                    let mut time_flag_lock = time_flag.lock().await;
                    *time_flag_lock = !*time_flag_lock;

                    let menu_handle = event.window().menu_handle();
                    let title: &str;
                    if *time_flag_lock {
                        title = "停止定时";
                        handler = Arc::new(Some(tokio::spawn(async move {
                            loop {
                                println!("1");
                                time::sleep(Duration::from_secs(15)).await;
                                // let mut interval = time::interval(time::Duration::from_secs(2));
                                // interval.tick().await;
                                println!("2");
                                let mut enigo = Enigo::new();

                                enigo.mouse_move_to(700, 200);
                                enigo.mouse_move_to(800, 200);

                                // enigo.mouse_click(MouseButton::Left);
                                // enigo.mouse_click(MouseButton::Right);
                                // enigo.key_sequence_parse("cx");
                                // time::sleep(Duration::from_secs(2)).await;
                                // enigo.key_click(Key::Backspace);
                                // enigo.key_click(Key::Backspace);
                                // enigo.key_sequence_parse("{+CTRL}a{-CTRL}{+SHIFT}Hello World{-SHIFT}");
                            }
                        })));
                    } else {
                        title = "启动定时";
                        match handler.as_ref() {
                            Some(v) => v.abort(),
                            _ => (),
                        }
                    }

                    menu_handle
                        .get_item(event.menu_item_id())
                        .set_title(title)
                        .unwrap();
                }
            });

            let main_window = app.get_window("main").unwrap();
            // tauri::api::dialog::blocking::message(Some(&main_window), "Hello", "Welcome back!");
            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                        // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
                        item_handle.set_title("Show").unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .menu(menu)
        .on_menu_event(move |event| {
            let tx_clone2 = tx_clone.clone();
            match event.menu_item_id() {
                "quit" => {
                    std::process::exit(0);
                }
                "close" => {
                    event.window().close().unwrap();
                }
                "hide" => {
                    event.window().hide().unwrap();
                }
                "time" => {
                    tokio::spawn(async move {
                        tx_clone2.send(event).await;
                    });
                }
                _ => {}
            }
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
