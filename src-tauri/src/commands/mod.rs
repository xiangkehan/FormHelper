//! 命令模块入口
//! 导出所有 Tauri 命令

pub mod person;
pub mod file;

// 导出 person 模块中的公共内容
pub use person::DbState;
pub use person::get_persons;
pub use person::create_person;
pub use person::update_person;
pub use person::delete_person;

// 导出 file 模块中的公共内容
pub use file::get_files;
pub use file::add_file;
pub use file::delete_file;
