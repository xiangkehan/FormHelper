// 文件处理命令模块 - 处理各种文件类型的统一接口
// 支持 PDF、图片（OCR）、Word、Excel 文件的表格提取

use tauri::State;
use serde_json::to_string;
use crate::ocr::{self, ExtractedTable};
use crate::db::{self, TableRecord};
use crate::commands::DbState;

/// 处理文件并提取表格
///
/// # 参数
/// * `file_path` - 文件路径
/// * `file_type` - 文件类型（pdf、image、word、excel）
/// * `person_id` - 可选的人员 ID，用于关联记录
/// * `state` - 数据库状态
///
/// # 返回
/// 处理结果，包含文件 ID 和表格信息
#[tauri::command]
pub async fn process_file(
    file_path: String,
    file_type: String,
    person_id: Option<i32>,
    state: State<DbState>,
) -> Result<ProcessResult, String> {
    // 根据文件类型处理
    let tables = match file_type.to_lowercase().as_str() {
        "pdf" => process_pdf(&file_path)?,
        "image" => process_image(&file_path)?,
        "word" => process_word(&file_path)?,
        "excel" => process_excel(&file_path)?,
        _ => return Err(format!("不支持的文件类型: {}", file_type)),
    };

    // 获取数据库连接
    let conn = state.get_conn()?;

    // 从路径提取文件名
    let file_name = std::path::Path::new(&file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // 保存文件记录
    let file_id = db::add_file(&conn, person_id, file_name, &file_path, &file_type)?;

    // 保存表格记录
    let mut results = Vec::new();

    for (index, table) in tables.iter().enumerate() {
        let content = to_string(table).map_err(|e| e.to_string())?;
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

/// 处理 PDF 文件
fn process_pdf(path: &str) -> Result<Vec<ExtractedTable>, String> {
    // 提取 PDF 文本
    let pages = ocr::pdf::PdfProcessor::extract_text(path)?;

    // 转换为表格格式
    let table = ocr::pdf::PdfProcessor::text_to_table(pages);

    Ok(vec![table])
}

/// 处理图片文件（OCR）
fn process_image(path: &str) -> Result<Vec<ExtractedTable>, String> {
    // OCR 识别
    let ocr_result = ocr::ocr::recognize_image(path, ocr::ocr::OcrConfig::default())?;

    // 转换为表格格式
    let table = ocr::ocr::ocr_result_to_table(ocr_result);

    Ok(vec![table])
}

/// 处理 Word 文件
fn process_word(path: &str) -> Result<Vec<ExtractedTable>, String> {
    ocr::word::WordProcessor::extract_tables(path)
}

/// 处理 Excel 文件
fn process_excel(path: &str) -> Result<Vec<ExtractedTable>, String> {
    // 提取所有工作表
    let excel_data = ocr::excel::ExcelProcessor::extract_all(path)?;

    // 转换为表格格式
    let mut tables = Vec::new();

    for sheet in excel_data.sheets {
        tables.push(ExtractedTable {
            rows: sheet.data,
        });
    }

    Ok(tables)
}

/// 处理结果结构体
#[derive(Debug)]
pub struct ProcessResult {
    pub file_id: i32,       // 保存的文件记录 ID
    pub tables: Vec<TableResult>,  // 处理出的表格信息
}

/// 单个表格结果信息
#[derive(Debug)]
pub struct TableResult {
    pub index: u32,     // 表格索引
    pub rows: usize,    // 行数
    pub cols: usize,    // 列数
}

/// 获取文件的表格记录
#[tauri::command]
pub async fn get_file_table_records(
    file_id: i32,
    state: State<DbState>,
) -> Result<Vec<TableRecord>, String> {
    let conn = state.get_conn()?;
    let records = db::get_table_records(&conn, file_id)?;
    Ok(records)
}
