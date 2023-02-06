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
  let show = CustomMenuItem::new("show".to_string(), "Show");

  
let tray_menu = SystemTrayMenu::new()
  .add_item(quit)
  .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(hide)
  .add_item(show);

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
            println!("{:#?}",window.is_visible());
            window.hide().unwrap();
            window.minimize().unwrap();
            // hide.enabled = false
            // item_handle.set_title("Show").unwrap();
            // item_handle.set_selected(true).unwrap();
            
          }
          "show" => {
            let window = app.get_window("main").unwrap();
            println!("{:#?}",window.is_visible());
            window.show().unwrap();
            window.unminimize().unwrap();
            window.set_focus().unwrap();
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
