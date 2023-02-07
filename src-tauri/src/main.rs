#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use tauri_plugin_store::PluginBuilder;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu,SystemTrayMenuItem, SystemTrayEvent, WindowEvent, Event};
use tauri::Manager;

fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");

  
let tray_menu = SystemTrayMenu::new()
  .add_item(quit)
  .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(hide);

  tauri::Builder::default()
    .system_tray(SystemTray::new().with_menu(tray_menu))
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
            if window.is_visible().unwrap(){
              item_handle.set_title("Show").unwrap();
              window.minimize().unwrap();
              window.hide().unwrap();
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
      cmd::read_folder
    ])
    .plugin(PluginBuilder::default().build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
