use crate::db::{self, TableRecord};
use crate::commands::person::DbState;
use tauri::State;

// 数据库状态共享结构
pub struct DbState {
    pub conn: std::sync::Mutex<Option<rusqlite::Connection>>,
}

impl DbState {
    // 获取数据库连接
    pub fn get_conn(&self) -> Result<rusqlite::Connection, String> {
        self.conn.lock()
            .map_err(|e| e.to_string())?
            .take()
            .ok_or("数据库连接不存在".to_string())
    }
}

/// 获取表格记录 - 支持按文件ID或人员ID筛选
#[tauri::command]
pub fn get_records(
    file_id: Option<i32>,
    person_id: Option<i32>,
    state: State<DbState>,
) -> Result<Vec<TableRecord>, String> {
    let conn = state.get_conn()?;
    let records = match (file_id, person_id) {
        (Some(fid), _) => db::get_table_records_by_file(&conn, fid),
        (_, Some(pid)) => db::get_table_records_by_person(&conn, pid),
        _ => db::get_all_table_records(&conn),
    };
    records.map_err(|e| e.to_string())
}

/// 更新表格内容 - 支持编辑功能
#[tauri::command]
pub fn update_record(
    id: i32,
    content: String,
    state: State<DbState>,
) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::update_table_record(&conn, id, &content)?;
    Ok(())
}

/// 删除记录
#[tauri::command]
pub fn delete_record(id: i32, state: State<DbState>) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::delete_table_record(&conn, id)?;
    Ok(())
}
