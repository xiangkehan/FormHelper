#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use std::sync::Mutex;

// 导入数据库模块
mod db;

// 导入命令模块
mod commands;
use commands::person::{get_persons, create_person, update_person, delete_person, DbState as PersonDbState};
use commands::record::{get_records, update_record, delete_record};
use commands::export::{export_to_excel, export_to_csv};

// 欢迎命令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    // 初始化数据库连接
    let db_state = PersonDbState {
        conn: Mutex::new(Some(db::init_db().expect("Failed to initialize database"))),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // 注册数据库状态
        .manage(db_state)
        // 注册所有 Tauri 命令
        .invoke_handler(tauri::generate_handler![
            greet,
            // 人员命令
            get_persons,
            create_person,
            update_person,
            delete_person,
            // 记录命令
            get_records,
            update_record,
            delete_record,
            // 导出命令
            export_to_excel,
            export_to_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
