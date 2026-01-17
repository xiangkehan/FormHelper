// Word 解析模块 - 使用 docx-rs 提取 Word 文档中的表格
// 解析 .docx 格式文档，提取所有表格数据

use docx_rs::{Docx, Table, TableRow, TableCell};
use crate::ocr::ExtractedTable;

/// Word 文档处理器
pub struct WordProcessor;

/// 表格单元格数据
#[derive(Debug)]
struct CellData {
    text: String,
    row_span: u32,
    col_span: u32,
}

impl WordProcessor {
    /// 从 Word 文档提取所有表格
    ///
    /// # 参数
    /// * `file_path` - Word 文件路径（.docx）
    ///
    /// # 返回
    /// 提取的表格列表
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

    /// 解析单个表格
    fn parse_table(table: &Table) -> Result<ExtractedTable, String> {
        let mut rows = Vec::new();

        for row in &table.rows {
            let row_data = Self::parse_row(row)?;
            rows.push(row_data);
        }

        Ok(ExtractedTable { rows })
    }

    /// 解析表格行
    fn parse_row(row: &TableRow) -> Result<Vec<String>, String> {
        let mut cells = Vec::new();

        for cell in &row.cells {
            let text = Self::parse_cell(cell)?;
            cells.push(text);
        }

        Ok(cells)
    }

    /// 解析表格单元格
    fn parse_cell(cell: &TableCell) -> Result<String, String> {
        let mut text = String::new();

        // 提取单元格内的所有段落文本
        for paragraph in &cell.children {
            for run in &paragraph.children {
                // 提取文本内容
                if let Some(text_run) = run.text.as_ref() {
                    text.push_str(&text_run.text);
                }
            }
            // 段落之间添加空格
            if !text.is_empty() {
                text.push(' ');
            }
        }

        Ok(text.trim().to_string())
    }

    /// 提取文档中所有文本（不分表格）
    pub fn extract_all_text(file_path: &str) -> Result<String, String> {
        let file = std::fs::File::open(file_path).map_err(|e| e.to_string())?;

        let doc = Docx::read(file).map_err(|e| e.to_string())?;

        let mut all_text = String::new();

        // 提取所有段落的文本
        for paragraph in doc.paragraphs {
            for run in &paragraph.children {
                if let Some(text_run) = run.text.as_ref() {
                    all_text.push_str(&text_run.text);
                }
            }
            all_text.push('\n');
        }

        Ok(all_text)
    }
}
