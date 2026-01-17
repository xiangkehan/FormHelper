#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// 导入数据库模块
mod db;
// 导入命令模块
mod commands;

// 导入数据库状态和命令
use commands::{DbState, get_persons, create_person, update_person, delete_person};
use commands::{get_files, add_file, delete_file};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    // 初始化数据库状态
    let db_state = DbState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            // 人员命令
            get_persons,
            create_person,
            update_person,
            delete_person,
            // 文件命令
            get_files,
            add_file,
            delete_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
