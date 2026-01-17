//! 人员相关 Tauri 命令模块
//! 提供人员的 CRUD 操作命令

use crate::db::{self, Person};
use tauri::State;

/// 数据库连接状态管理器
/// 使用 Mutex 确保线程安全的数据库连接管理
pub struct DbState {
    pub conn: std::sync::Mutex<Option<rusqlite::Connection>>,
}

impl DbState {
    /// 获取数据库连接
    pub fn get_conn(&self) -> Result<rusqlite::Connection, String> {
        self.conn.lock()
            .map_err(|e| e.to_string())?
            .take()
            .ok_or("数据库连接不存在".to_string())
    }
}

/// 获取所有人员
#[tauri::command]
pub fn get_persons(state: State<DbState>) -> Result<Vec<Person>, String> {
    let conn = state.get_conn()?;
    db::get_persons(&conn).map_err(|e| e.to_string())
}

/// 添加人员
#[tauri::command]
pub fn create_person(name: String, state: State<DbState>) -> Result<i32, String> {
    let conn = state.get_conn()?;
    db::add_person(&conn, &name).map_err(|e| e.to_string())
}

/// 更新人员信息
#[tauri::command]
pub fn update_person(id: i32, name: String, state: State<DbState>) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::update_person(&conn, id, &name)?;
    Ok(())
}

/// 删除人员
#[tauri::command]
pub fn delete_person(id: i32, state: State<DbState>) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::delete_person(&conn, id)?;
    Ok(())
}
