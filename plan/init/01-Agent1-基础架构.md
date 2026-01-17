# Agent 1 提示词 - 基础架构（补充）

## 进度管理（开始前必须执行）

```batch
cd progress
progress.bat check phase1
REM 确认依赖满足后再继续
progress.bat start phase1
```

### 任务对照表

| 完成后执行 | 任务 |
|-----------|------|
| `progress.bat task phase1 commands_person_rs` | 创建 person.rs |
| `progress.bat task phase1 commands_file_rs` | 创建 file.rs |
| `progress.bat task phase1 commands_mod_rs` | 创建 mod.rs |
| `progress.bat task phase1 main_rs_update` | 更新 main.rs |
| `progress.bat task phase1 build_success` | 编译成功 |
| `progress.bat complete phase1` | 完成阶段1 |

---

## 角色定义

你是一个 Full Stack 开发者，负责完善 FormHelper 项目的基础架构。

## 任务目标

**注意：基础架构已部分完成（约 70%）。你的任务是补充 Tauri 命令实现。**

## 工作目录

```
FormHelper/
```

## 当前完成情况

### 已完成的文件结构

```
FormHelper/
├── src/
│   ├── main.ts              # ✅ 已配置 i18n, Pinia, Naive UI
│   ├── App.vue              # ✅ 基础布局
│   ├── router/index.ts      # ✅ 路由配置
│   ├── stores/index.ts      # ✅ appStore
│   ├── locales/
│   │   ├── zh-CN.json       # ✅ 中文
│   │   └── en-US.json       # ✅ 英文
│   └── views/
│       ├── HomeView.vue     # ✅ 首页
│       ├── PersonsView.vue  # ✅ 人员管理
│       ├── FilesView.vue    # ✅ 文件处理
│       └── SettingsView.vue # ✅ 设置
├── src-tauri/
│   ├── src/
│   │   ├── main.rs          # ✅ 基础框架
│   │   └── db/mod.rs        # ✅ SQLite 数据库
│   ├── Cargo.toml           # ✅ 依赖配置
│   └── tauri.conf.json      # ✅ 配置
└── package.json             # ✅ 前端依赖
```

### 已完成的数据库（src-tauri/src/db/mod.rs）

- `Person` 结构体和 CRUD 操作
- `FileRecord` 结构体和 CRUD 操作
- `TableRecord` 结构体
- `init_db()` 数据库初始化

### 仍需完成

在 `src-tauri/src/commands/` 实现 Tauri 命令。

## 具体任务

### 1. 创建人员命令模块

在 `src-tauri/src/commands/` 创建 `person.rs`：

```rust
use crate::db::{self, Person};
use tauri::State;

// 数据库连接状态
pub struct DbState {
    pub conn: std::sync::Mutex<Option<rusqlite::Connection>>,
}

impl DbState {
    pub fn new() -> Self {
        Self {
            conn: std::sync::Mutex::new(None),
        }
    }

    pub fn get_conn(&self) -> Result<rusqlite::Connection, String> {
        let mut conn_guard = self.conn.lock().map_err(|e| e.to_string())?;
        if conn_guard.is_none() {
            let new_conn = db::init_db().map_err(|e| e.to_string())?;
            *conn_guard = Some(new_conn);
        }
        Ok(conn_guard.take().unwrap())
    }
}

/// 获取所有人员
#[tauri::command]
pub fn get_persons(state: State<DbState>) -> Result<Vec<Person>, String> {
    let conn = state.get_conn()?;
    db::get_persons(&conn).map_err(|e| e.to_string())
}

/// 创建人员
#[tauri::command]
pub fn create_person(name: String, state: State<DbState>) -> Result<i32, String> {
    let conn = state.get_conn()?;
    let id = db::add_person(&conn, &name)?;
    Ok(id)
}

/// 更新人员
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
```

### 2. 创建文件命令模块

在 `src-tauri/src/commands/` 创建 `file.rs`：

```rust
use crate::db::{self, FileRecord};
use tauri::State;
use crate::commands::person::DbState;

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
    let id = db::add_file(&conn, person_id, &file_name, &file_path, &file_type)?;
    Ok(id)
}

/// 获取所有文件
#[tauri::command]
pub fn get_files(state: State<DbState>) -> Result<Vec<FileRecord>, String> {
    let conn = state.get_conn()?;
    db::get_files(&conn).map_err(|e| e.to_string())
}

/// 删除文件
#[tauri::command]
pub fn delete_file(id: i32, state: State<DbState>) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::delete_file(&conn, id)?;
    Ok(())
}
```

### 3. 创建命令模块入口

在 `src-tauri/src/commands/` 创建 `mod.rs`：

```rust
pub mod person;
pub mod file;

pub use person::*;
pub use file::*;
```

### 4. 更新 main.rs

修改 `src-tauri/src/main.rs`：

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use crate::commands::person::DbState;

mod commands;
mod db;
mod ocr;

// 导入命令
use commands::{
    get_persons,
    create_person,
    update_person,
    delete_person,
    get_files,
    add_file,
    delete_file,
};

fn main() {
    // 初始化数据库状态
    let db_state = DbState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            // 人员命令
            get_persons,
            create_person,
            update_person,
            delete_person,
            // 文件命令
            get_files,
            add_file,
            delete_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

## 产出要求

- [ ] `src-tauri/src/commands/person.rs` 实现人员 CRUD 命令
- [ ] `src-tauri/src/commands/file.rs` 实现文件 CRUD 命令
- [ ] `src-tauri/src/commands/mod.rs` 模块入口
- [ ] `src-tauri/src/main.rs` 更新命令注册
- [ ] 项目能正常编译运行

## 注意事项

1. 每个文件都要有简洁清晰的中文注释
2. 使用 `tauri::State` 管理数据库连接
3. 错误处理要返回友好的错误信息
4. 完成后暂停，等待 Agent 2 对接

## 依赖检查

执行前请确认：
- [ ] Rust 和 Cargo 已安装
- [ ] Tauri CLI 已安装

如果依赖不满足，请停止并告知用户。
