# Agent 3 提示词 - 后端 OCR 集成

## 进度管理（开始前必须执行）

```bash
cd progress
python progress_manager.py check phase3_parallel
# 确认依赖满足后再继续
python progress_manager.py start phase3_parallel
```

### 任务对照表（Agent 3）

| 完成后执行 | 任务 |
|-----------|------|
| `python progress_manager.py task phase3_parallel Cargo_deps` | 添加 Cargo 依赖 |
| `python progress_manager.py task phase3_parallel ocr_mod_rs` | OCR 模块 |
| `python progress_manager.py task phase3_parallel pdf_rs` | PDF 解析 |
| `python progress_manager.py task phase3_parallel word_rs` | Word 解析 |
| `python progress_manager.py task phase3_parallel excel_rs` | Excel 解析 |
| `python progress_manager.py task phase3_parallel process_rs` | 文件处理命令 |
| `python progress_manager.py task phase3_parallel ocr_working` | OCR 功能可用 |

---

## 角色定义

你是一个 Rust 开发者，负责实现 FormHelper 的后端文件处理和 OCR 功能。

## 任务目标

实现文件内容提取：Tesseract OCR、PDF、Word、Excel 解析。

## 工作目录

```
FormHelper/src-tauri/
```

## 技术栈

- Rust 1.70+
- Tauri 2.0
- SQLite（rusqlite，已配置）
- OCR：tesseract crate
- PDF：lopdf 或 pdfium
- Word：docx-rs
- Excel：calamine

## Cargo 依赖

在 `src-tauri/Cargo.toml` 添加：

```toml
[dependencies]
# OCR（需要系统安装 tesseract）
tesseract = "0.3"
leptonica-sys = "0.4"

# PDF 解析
lopdf = "0.34"

# Word 解析
docx-rs = "0.4"

# Excel 解析
calamine = "0.20"

# JSON 处理
serde_json = "1.0"

# 日期时间
chrono = { version = "0.4", features = ["serde"] }
```

## 具体任务

### 1. 创建 OCR 处理模块

在 `src-tauri/src/ocr/` 创建 `mod.rs`：

```rust
/// OCR 识别配置
pub struct OcrConfig {
    pub lang: String,        // 语言: "eng", "chi_sim"
    pub psm: i32,            // 页面分割模式
}

impl Default for OcrConfig {
    fn default() -> Self {
        Self {
            lang: "eng".to_string(),
            psm: 6,
        }
    }
}

/// OCR 识别结果
pub struct OcrResult {
    pub text: String,        // 识别文本
    pub confidence: f32,     // 置信度
}

/// 使用 Tesseract 进行图片 OCR 识别
pub fn recognize_image(
    image_path: &str,
    config: OcrConfig,
) -> Result<OcrResult, String> {
    // 调用 tesseract 识别图片文字
    // 返回识别文本和置信度
}

/// 从图片中检测表格区域（简单实现）
pub fn detect_table_regions(image_path: &str) -> Result<Vec<TableRegion>, String> {
    // 使用图像处理检测表格区域
    // 返回表格区域的坐标
}

/// 表格区域
pub struct TableRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
```

### 2. 创建 PDF 解析模块

在 `src-tauri/src/ocr/` 创建 `pdf.rs`：

```rust
use lopdf::{Document, Object};

pub struct PdfProcessor;

impl PdfProcessor {
    /// 从 PDF 提取文本（按页）
    pub fn extract_text(file_path: &str) -> Result<Vec<PageText>, String> {
        let doc = Document::load(file_path).map_err(|e| e.to_string())?;

        let mut pages = Vec::new();
        for (page_num, page_id) in doc.get_pages().keys().enumerate() {
            let text = Self::extract_page_text(&doc, page_id)?;
            pages.push(PageText {
                page: page_num as u32 + 1,
                text,
            });
        }
        Ok(pages)
    }

    fn extract_page_text(doc: &Document, page_id: &lopdf::ObjectId) -> Result<String, String> {
        // 从 PDF 页面提取文本内容
        Ok(String::new())
    }

    /// 简单表格检测（基于文本布局）
    pub fn detect_tables_from_text(pages: &[PageText]) -> Vec<DetectedTable> {
        // 分析文本布局，检测表格结构
        Vec::new()
    }
}

pub struct PageText {
    pub page: u32,
    pub text: String,
}

pub struct DetectedTable {
    pub page: u32,
    pub rows: Vec<Vec<String>>,
}
```

### 3. 创建 Word 解析模块

在 `src-tauri/src/ocr/` 创建 `word.rs`：

```rust
use docx_rs::{Docx, Table, TableRow, TableCell};

pub struct WordProcessor;

impl WordProcessor {
    /// 从 Word 文档提取所有表格
    pub fn extract_tables(file_path: &str) -> Result<Vec<ExtractedTable>, String> {
        let file = std::fs::File::open(file_path).map_err(|e| e.to_string())?;
        let doc = Docx::read(file).map_err(|e| e.to_string())?;

        let mut tables = Vec::new();
        for table in doc.tables {
            let extracted = Self::parse_table(&table)?;
            tables.push(extracted);
        }
        Ok(tables)
    }

    fn parse_table(table: &Table) -> Result<ExtractedTable, String> {
        let mut rows = Vec::new();
        for row in &table.rows {
            let row_data = Self::parse_row(row)?;
            rows.push(row_data);
        }
        Ok(ExtractedTable { rows })
    }

    fn parse_row(row: &TableRow) -> Result<Vec<String>, String> {
        let mut cells = Vec::new();
        for cell in &row.cells {
            let text = Self::parse_cell(cell)?;
            cells.push(text);
        }
        Ok(cells)
    }

    fn parse_cell(cell: &TableCell) -> Result<String, String> {
        // 提取单元格文本
        Ok(String::new())
    }
}

pub struct ExtractedTable {
    pub rows: Vec<Vec<String>>,
}
```

### 4. 创建 Excel 解析模块

在 `src-tauri/src/ocr/` 创建 `excel.rs`：

```rust
use calamine::{Reader, Xlsx, DataType};

pub struct ExcelProcessor;

impl ExcelProcessor {
    /// 从 Excel 提取所有数据
    pub fn extract_all(file_path: &str) -> Result<ExcelData, String> {
        let workbook: Xlsx<_> = calamine::open_reader(file_path)
            .map_err(|e| e.to_string())?;

        let mut sheets = Vec::new();
        for (name, range) in workbook.worksheets() {
            let data = Self::range_to_vec(&range);
            sheets.push(SheetData {
                name,
                data,
            });
        }
        Ok(ExcelData { sheets })
    }

    fn range_to_vec(range: &calamine::Range<DataType>) -> Vec<Vec<String>> {
        // 将单元格数据转换为字符串矩阵
        Vec::new()
    }
}

pub struct ExcelData {
    pub sheets: Vec<SheetData>,
}

pub struct SheetData {
    pub name: String,
    pub data: Vec<Vec<String>>,
}
```

### 5. 创建文件处理命令

在 `src-tauri/src/commands/` 创建 `process.rs`：

```rust
use crate::ocr::{self, OcrConfig};
use crate::db::{self, TableRecord};
use crate::commands::person::DbState;
use tauri::State;

/// 处理文件并提取表格
#[tauri::command]
pub async fn process_file(
    file_path: String,
    file_type: String,
    person_id: Option<i32>,
    state: State<DbState>,
) -> Result<ProcessResult, String> {
    let tables = match file_type.to_lowercase().as_str() {
        "pdf" => process_pdf(&file_path)?,
        "image" => process_image(&file_path)?,
        "word" => process_word(&file_path)?,
        "excel" => process_excel(&file_path)?,
        _ => return Err("不支持的文件类型".to_string()),
    };

    // 保存文件记录
    let conn = state.get_conn()?;
    let file_id = db::add_file(&conn, person_id, &file_path, &file_type)?;

    // 保存表格记录
    let mut results = Vec::new();
    for (index, table) in tables.iter().enumerate() {
        let content = serde_json::to_string(table)
            .map_err(|e| e.to_string())?;

        db::add_table_record(&conn, file_id, person_id, &content)?;

        results.push(TableResult {
            index: index as u32,
            rows: table.rows.len(),
            cols: table.rows.first().map(|r| r.len()).unwrap_or(0),
        });
    }

    Ok(ProcessResult {
        file_id,
        tables: results,
    })
}

fn process_pdf(path: &str) -> Result<Vec<ExtractedTable>, String> {
    // PDF 处理
    Ok(Vec::new())
}

fn process_image(path: &str) -> Result<Vec<ExtractedTable>, String> {
    // OCR 处理
    let ocr_result = ocr::recognize_image(path, OcrConfig::default())?;
    // 将 OCR 结果转换为表格格式
    Ok(vec![ExtractedTable {
        rows: vec![vec![ocr_result.text]],
    }])
}

fn process_word(path: &str) -> Result<Vec<ExtractedTable>, String> {
    // Word 处理
    Ok(Vec::new())
}

fn process_excel(path: &str) -> Result<Vec<ExtractedTable>, String> {
    // Excel 处理
    Ok(Vec::new())
}

pub struct ProcessResult {
    pub file_id: i32,
    pub tables: Vec<TableResult>,
}

pub struct TableResult {
    pub index: u32,
    pub rows: usize,
    pub cols: usize,
}
```

### 6. 更新 db/mod.rs 添加表格记录操作

```rust
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

/// 获取表格记录
pub fn get_table_records(conn: &Connection, file_id: i32) -> Result<Vec<TableRecord>> {
    // 实现
    Ok(Vec::new())
}
```

### 7. 更新 main.rs

添加新命令到 `invoke_handler`：

```rust
use commands::process::process_file;

invoke_handler![
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
    // 处理命令
    process_file,
]
```

## 模块结构

```
src-tauri/src/
├── main.rs
├── commands/
│   ├── mod.rs
│   ├── person.rs
│   ├── file.rs
│   └── process.rs      # 新增
├── db/
│   ├── mod.rs          # 更新：添加 add_table_record
│   ├── person.rs
│   ├── file.rs
│   └── schema.sql
└── ocr/
    ├── mod.rs          # 新增：OCR 公共定义
    ├── ocr.rs          # 新增：OCR 核心
    ├── pdf.rs          # 新增：PDF 解析
    ├── word.rs         # 新增：Word 解析
    └── excel.rs        # 新增：Excel 解析
```

## 产出要求

- [ ] Tesseract OCR 集成
- [ ] PDF 文本提取
- [ ] Word 表格提取
- [ ] Excel 数据读取
- [ ] `process_file` 命令

## 注意事项

1. 每个文件都要有简洁清晰的中文注释
2. 处理大文件时考虑异步和进度反馈
3. 错误处理要返回友好提示
4. 文件路径处理要安全

## 依赖检查

执行前请确认：

- [ ] Agent 1 已完成（Tauri 命令框架）
- [ ] 系统已安装 Tesseract

如果依赖不满足，请停止并告知用户。

## 系统依赖

**Windows:**

- 下载 Tesseract: https://github.com/UB-Mannheim/tesseract/wiki

**Linux:**

```bash
sudo apt install tesseract-ocr libtesseract-dev
```

**macOS:**

```bash
brew install tesseract
```
