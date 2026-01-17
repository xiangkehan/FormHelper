//! 文件相关 Tauri 命令模块
//! 提供文件记录的 CRUD 操作命令

use crate::db::{self, FileRecord};
use crate::commands::person::DbState;
use tauri::State;

/// 添加文件记录
#[tauri::command]
pub fn add_file(
    person_id: Option<i32>,
    file_name: String,
    file_path: String,
    file_type: String,
    state: State<DbState>,
) -> Result<i32, String> {
    let conn = state.get_conn()?;
    let id = db::add_file(&conn, person_id, &file_name, &file_path, &file_type)
        .map_err(|e| e.to_string())?;
    Ok(id)
}

/// 获取所有文件记录
#[tauri::command]
pub fn get_files(state: State<DbState>) -> Result<Vec<FileRecord>, String> {
    let conn = state.get_conn()?;
    db::get_files(&conn).map_err(|e| e.to_string())
}

/// 删除文件记录
#[tauri::command]
pub fn delete_file(id: i32, state: State<DbState>) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::delete_file(&conn, id).map_err(|e| e.to_string())?;
    Ok(())
}
