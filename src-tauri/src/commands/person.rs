//! 人员相关 Tauri 命令模块
//! 提供人员的 CRUD 操作命令

use crate::db::{self, Person};
use tauri::State;

/// 数据库连接状态管理器
/// 使用 Mutex 确保线程安全的数据库连接管理
pub struct DbState {
    /// 存储可选的数据库连接
    pub conn: std::sync::Mutex<Option<rusqlite::Connection>>,
}

impl DbState {
    /// 创建新的数据库状态实例
    pub fn new() -> Self {
        Self {
            conn: std::sync::Mutex::new(None),
        }
    }

    /// 获取数据库连接
    /// 如果连接不存在则创建新连接
    pub fn get_conn(&self) -> Result<rusqlite::Connection, String> {
        let mut conn_guard = self.conn.lock().map_err(|e| e.to_string())?;
        if conn_guard.is_none() {
            let new_conn = db::init_db().map_err(|e| e.to_string())?;
            *conn_guard = Some(new_conn);
        }
        Ok(conn_guard.take().unwrap())
    }
}

/// 获取所有人员列表
#[tauri::command]
pub fn get_persons(state: State<DbState>) -> Result<Vec<Person>, String> {
    let conn = state.get_conn()?;
    db::get_persons(&conn).map_err(|e| e.to_string())
}

/// 创建新人员
#[tauri::command]
pub fn create_person(name: String, state: State<DbState>) -> Result<i32, String> {
    let conn = state.get_conn()?;
    let id = db::add_person(&conn, &name).map_err(|e| e.to_string())?;
    Ok(id)
}

/// 更新人员信息
#[tauri::command]
pub fn update_person(id: i32, name: String, state: State<DbState>) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::update_person(&conn, id, &name).map_err(|e| e.to_string())?;
    Ok(())
}

/// 删除人员
#[tauri::command]
pub fn delete_person(id: i32, state: State<DbState>) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::delete_person(&conn, id).map_err(|e| e.to_string())?;
    Ok(())
}
