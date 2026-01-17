//! 命令模块入口
//! 导出所有 Tauri 命令

pub mod person;
pub mod record;
pub mod export;

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
