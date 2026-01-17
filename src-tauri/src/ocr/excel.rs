// Excel 解析模块 - 使用 calamine 读取 Excel 文件
// 支持 .xlsx 和 .xls 格式，提取所有工作表数据

use calamine::{Reader, Xlsx, DataType, Range};
use crate::ocr::ExtractedTable;

/// Excel 处理器
pub struct ExcelProcessor;

/// Excel 数据容器
#[derive(Debug)]
pub struct ExcelData {
    pub sheets: Vec<SheetData>,  // 所有工作表数据
}

/// 单个工作表数据
#[derive(Debug)]
pub struct SheetData {
    pub name: String,                // 工作表名称
    pub data: Vec<Vec<String>>,      // 工作表数据（二维数组）
}

impl ExcelProcessor {
    /// 从 Excel 提取所有数据
    ///
    /// # 参数
    /// * `file_path` - Excel 文件路径
    ///
    /// # 返回
    /// 所有工作表的数据
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

    /// 将 calamine Range 转换为字符串矩阵
    fn range_to_vec(range: &Range<DataType>) -> Vec<Vec<String>> {
        range
            .rows()
            .map(|row| {
                row.iter()
                    .map(|cell| Self::cell_to_string(cell))
                    .collect()
            })
            .collect()
    }

    /// 将单元格转换为字符串
    fn cell_to_string(cell: &DataType) -> String {
        match cell {
            DataType::Empty => String::new(),
            DataType::String(s) => s.clone(),
            DataType::Float(n) => n.to_string(),
            DataType::Int(n) => n.to_string(),
            DataType::Bool(b) => b.to_string(),
            DataType::Error(e) => format!("#ERROR: {}", e),
        }
    }

    /// 提取第一个工作表作为表格
    pub fn extract_first_sheet(file_path: &str) -> Result<ExtractedTable, String> {
        let data = Self::extract_all(file_path)?;

        if let Some(sheet) = data.sheets.first() {
            Ok(ExtractedTable {
                rows: sheet.data.clone(),
            })
        } else {
            Ok(ExtractedTable { rows: Vec::new() })
        }
    }

    /// 合并所有工作表为单个表格
    pub fn extract_all_as_single_table(file_path: &str) -> Result<ExtractedTable, String> {
        let data = Self::extract_all(file_path)?;

        let mut all_rows: Vec<Vec<String>> = Vec::new();

        for sheet in &data.sheets {
            // 添加工作表分隔行
            all_rows.push(vec![format!("=== {} ===", sheet.name)]);
            all_rows.extend(sheet.data.clone());
        }

        Ok(ExtractedTable { rows: all_rows })
    }
}
