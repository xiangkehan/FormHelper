use rusqlite::{Connection, Result, params};
use serde::Serialize;

// 人员结构体
#[derive(Debug, Serialize, Clone)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

// 文件记录结构体
#[derive(Debug, Serialize, Clone)]
pub struct FileRecord {
    pub id: i32,
    pub person_id: Option<i32>,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
    pub created_at: String,
}

// 表格记录结构体
#[derive(Debug, Serialize, Clone)]
pub struct TableRecord {
    pub id: i32,
    pub file_id: i32,
    pub person_id: Option<i32>,
    pub content: String,
    pub created_at: String,
}

// 数据库初始化
pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("formhelper.db")?;

    // 创建人员表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS persons (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // 创建文件表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            person_id INTEGER,
            file_name TEXT NOT NULL,
            file_path TEXT NOT NULL,
            file_type TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (person_id) REFERENCES persons(id)
        )",
        [],
    )?;

    // 创建表格记录表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS table_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_id INTEGER NOT NULL,
            person_id INTEGER,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (file_id) REFERENCES files(id),
            FOREIGN KEY (person_id) REFERENCES persons(id)
        )",
        [],
    )?;

    Ok(conn)
}

// 人员相关操作
pub fn add_person(conn: &Connection, name: &str) -> Result<i32> {
    let mut stmt = conn.prepare("INSERT INTO persons (name) VALUES (?1)")?;
    stmt.execute([name])?;
    Ok(conn.last_insert_rowid() as i32)
}

pub fn get_persons(conn: &Connection) -> Result<Vec<Person>> {
    let mut stmt = conn.prepare("SELECT id, name, created_at, updated_at FROM persons ORDER BY created_at DESC")?;
    let persons = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
        })
    })?.collect::<Result<Vec<Person>>>()?;
    Ok(persons)
}

pub fn update_person(conn: &Connection, id: i32, name: &str) -> Result<()> {
    conn.execute(
        "UPDATE persons SET name = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
        [name, &id.to_string()],
    )?;
    Ok(())
}

pub fn delete_person(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM persons WHERE id = ?1", [&id.to_string()])?;
    Ok(())
}

// 文件相关操作
pub fn add_file(
    conn: &Connection,
    person_id: Option<i32>,
    file_name: &str,
    file_path: &str,
    file_type: &str,
) -> Result<i32> {
    let mut stmt = conn.prepare(
        "INSERT INTO files (person_id, file_name, file_path, file_type) VALUES (?1, ?2, ?3, ?4)",
    )?;
    stmt.execute(params![person_id, file_name, file_path, file_type])?;
    Ok(conn.last_insert_rowid() as i32)
}

pub fn get_files(conn: &Connection) -> Result<Vec<FileRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, person_id, file_name, file_path, file_type, created_at FROM files ORDER BY created_at DESC",
    )?;
    let files = stmt.query_map([], |row| {
        Ok(FileRecord {
            id: row.get(0)?,
            person_id: row.get(1)?,
            file_name: row.get(2)?,
            file_path: row.get(3)?,
            file_type: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?.collect::<Result<Vec<FileRecord>>>()?;
    Ok(files)
}

pub fn delete_file(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM files WHERE id = ?1", [&id.to_string()])?;
    Ok(())
}

// 表格记录相关操作

/// 获取指定文件的所有表格记录
pub fn get_table_records_by_file(conn: &Connection, file_id: i32) -> Result<Vec<TableRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, file_id, person_id, content, created_at FROM table_records WHERE file_id = ?1 ORDER BY created_at DESC",
    )?;
    let records = stmt.query_map([file_id], |row| {
        Ok(TableRecord {
            id: row.get(0)?,
            file_id: row.get(1)?,
            person_id: row.get(2)?,
            content: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?.collect::<Result<Vec<TableRecord>>>()?;
    Ok(records)
}

/// 获取指定人员的所有表格记录
pub fn get_table_records_by_person(conn: &Connection, person_id: i32) -> Result<Vec<TableRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, file_id, person_id, content, created_at FROM table_records WHERE person_id = ?1 ORDER BY created_at DESC",
    )?;
    let records = stmt.query_map([person_id], |row| {
        Ok(TableRecord {
            id: row.get(0)?,
            file_id: row.get(1)?,
            person_id: row.get(2)?,
            content: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?.collect::<Result<Vec<TableRecord>>>()?;
    Ok(records)
}

/// 获取所有表格记录
pub fn get_all_table_records(conn: &Connection) -> Result<Vec<TableRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, file_id, person_id, content, created_at FROM table_records ORDER BY created_at DESC",
    )?;
    let records = stmt.query_map([], |row| {
        Ok(TableRecord {
            id: row.get(0)?,
            file_id: row.get(1)?,
            person_id: row.get(2)?,
            content: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?.collect::<Result<Vec<TableRecord>>>()?;
    Ok(records)
}

/// 获取文件的表格记录
pub fn get_table_records(conn: &Connection, file_id: i32) -> Result<Vec<TableRecord>> {
    get_table_records_by_file(conn, file_id)
}

/// 添加表格记录
pub fn add_table_record(
    conn: &Connection,
    file_id: i32,
    person_id: Option<i32>,
    content: &str,
) -> Result<i32> {
    let mut stmt = conn.prepare(
        "INSERT INTO table_records (file_id, person_id, content) VALUES (?1, ?2, ?3)",
    )?;
    stmt.execute(params![file_id, person_id, content])?;
    Ok(conn.last_insert_rowid() as i32)
}

/// 更新表格记录内容
pub fn update_table_record(conn: &Connection, id: i32, content: &str) -> Result<()> {
    conn.execute(
        "UPDATE table_records SET content = ?1 WHERE id = ?2",
        [content, &id.to_string()],
    )?;
    Ok(())
}

/// 删除表格记录
pub fn delete_table_record(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM table_records WHERE id = ?1", [&id.to_string()])?;
    Ok(())
}
