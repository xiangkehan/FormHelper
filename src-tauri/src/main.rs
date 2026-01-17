#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::State;
use std::sync::Mutex;

// 导入数据库模块
mod db;

// 导入命令模块
mod commands;
use commands::person::{get_persons, create_person, update_person, delete_person, DbState as PersonDbState};
use commands::record::{get_records, update_record, delete_record};
use commands::export::{export_to_excel, export_to_csv};
use commands::{get_files, add_file, delete_file};
use commands::process;
use commands::DbState;

// 导入 OCR 模块
mod ocr;

// 欢迎命令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// 获取所有人员
#[tauri::command]
fn get_persons(state: State<DbState>) -> Result<Vec<db::Person>, String> {
    let conn = state.get_conn()?;
    db::get_persons(&conn).map_err(|e| e.to_string())
}

// 创建人员
#[tauri::command]
fn create_person(name: String, state: State<DbState>) -> Result<i32, String> {
    let conn = state.get_conn()?;
    db::add_person(&conn, &name).map_err(|e| e.to_string())
}

// 更新人员
#[tauri::command]
fn update_person(id: i32, name: String, state: State<DbState>) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::update_person(&conn, id, &name).map_err(|e| e.to_string())
}

// 删除人员
#[tauri::command]
fn delete_person(id: i32, state: State<DbState>) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::delete_person(&conn, id).map_err(|e| e.to_string())
}

// 获取所有文件
#[tauri::command]
fn get_files(state: State<DbState>) -> Result<Vec<db::FileRecord>, String> {
    let conn = state.get_conn()?;
    db::get_files(&conn).map_err(|e| e.to_string())
}

// 添加文件
#[tauri::command]
fn add_file(
    file_name: String,
    file_path: String,
    file_type: String,
    person_id: Option<i32>,
    state: State<DbState>,
) -> Result<i32, String> {
    let conn = state.get_conn()?;
    db::add_file(&conn, person_id, &file_name, &file_path, &file_type).map_err(|e| e.to_string())
}

// 删除文件
#[tauri::command]
fn delete_file(id: i32, state: State<DbState>) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::delete_file(&conn, id).map_err(|e| e.to_string())
}

// 获取表格记录
#[tauri::command]
fn get_table_records(file_id: i32, state: State<DbState>) -> Result<Vec<db::TableRecord>, String> {
    let conn = state.get_conn()?;
    db::get_table_records(&conn, file_id).map_err(|e| e.to_string())
}

fn main() {
    // 初始化数据库
    let conn = db::init_db().expect("Failed to initialize database");

    // 创建 Tauri 应用
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // 共享数据库状态
        .manage(commands::new_db_state(conn))
        // 注册命令处理器
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
            // 文件命令
            get_files,
            add_file,
            delete_file,
            // 表格命令
            get_table_records,
            // 处理命令
            process::process_file,
            process::get_file_table_records,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
