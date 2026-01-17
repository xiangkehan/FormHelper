//! 命令模块入口
//! 导出所有 Tauri 命令

pub mod person;
pub mod record;
pub mod export;
pub mod file;
pub mod process;

// 导出 person 模块中的公共内容
pub use person::DbState;
pub use person::get_persons;
pub use person::create_person;
pub use person::update_person;
pub use person::delete_person;

// 导出 record 模块中的公共内容
pub use record::{get_records, update_record, delete_record};

// 导出 export 模块中的公共内容
pub use export::{export_to_excel, export_to_csv};

// 导出 file 模块中的公共内容
pub use file::get_files;
pub use file::add_file;
pub use file::delete_file;

// 导出 process 模块中的公共内容
pub use process::process_file;
pub use process::get_file_table_records;

// 数据库状态共享结构体
use rusqlite::Connection;
use std::sync::Mutex;

/// 数据库状态（用于命令间共享连接）
pub struct DbState {
    pub conn: Mutex<Connection>,
}

impl DbState {
    /// 获取数据库连接
    pub fn get_conn(&self) -> Result<std::sync::MutexGuard<'_, Connection>, String> {
        self.conn.lock().map_err(|e| e.to_string())
    }
}

/// 创建新的数据库状态
pub fn new_db_state(conn: Connection) -> DbState {
    DbState {
        conn: Mutex::new(conn),
    }
}
