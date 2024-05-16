// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::OnceCell;
use tauri::AppHandle;

pub static INSTANCE: OnceCell<AppHandle> = OnceCell::new();

mod core;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            INSTANCE.set(app.handle()).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            core::launcher::run_minecraft,
            utils::reveal_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}