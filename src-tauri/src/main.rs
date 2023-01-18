#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use tauri_plugin_store::PluginBuilder;

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::get_md_in_folder,
      cmd::create_file,
      cmd::save_content,
      cmd::get_content,
      cmd::read_folder
    ])
    .plugin(PluginBuilder::default().build())
    .run(context)
    .expect("error while running tauri application");
}
