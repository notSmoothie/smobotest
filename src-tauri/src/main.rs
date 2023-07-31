#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::process::exit;

use tauri::Manager;
use window_vibrancy::apply_blur;
use tauri::http::ResponseBuilder;
use tauri::{
    CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem
};
use tauri_plugin_store::PluginBuilder;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let check_upd = CustomMenuItem::new("check_upd".to_string(), "Check for updates");

    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(check_upd)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    tauri::Builder::default()
        .register_uri_scheme_protocol("smobot", move |_app, request|{
            println!("request {:#?}", request);
            return ResponseBuilder::new().status(404).body(Vec::new());
        })
        .setup(|app| {
        let window = app.get_window("main").unwrap();
        #[cfg(target_os = "macos")]
        apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
          .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
  
        #[cfg(target_os = "windows")]
        apply_blur(&window, Some((18, 18, 18, 230)))
          .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
  
        Ok(())
      })
        .system_tray(SystemTray::new().with_menu(tray_menu)) 
        .on_system_tray_event(|app, event| match event{
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                let item_handle = app.tray_handle().get_item("hide");
                item_handle.set_title("Hide").unwrap();
                window.show().unwrap();
                window.unminimize().unwrap();
                window.set_focus().unwrap();
                println!("system tray received a single click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "quit" => {
                        exit(0);
                    }
                    "check_upd" => {
                        app.emit_all("checkUpdate",{}).unwrap();
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            item_handle.set_title("Show").unwrap();
                            window.hide().unwrap();
                            window.minimize().unwrap();
                        } else {
                            item_handle.set_title("Hide").unwrap();
                            window.show().unwrap();
                            window.unminimize().unwrap();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            cmd::get_md_in_folder,
            cmd::create_file,
            cmd::save_content,
            cmd::get_content,
            cmd::read_folder,
            cmd::hide_window,
        ])
        .plugin(PluginBuilder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
