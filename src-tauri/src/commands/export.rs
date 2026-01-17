use crate::commands::person::DbState;
use crate::db;
use tauri::State;

/// 导出人员数据到 Excel 文件
#[tauri::command]
pub fn export_to_excel(
    person_id: i32,
    file_path: String,
    state: State<DbState>,
) -> Result<(), String> {
    let conn = state.get_conn()?;

    // 获取人员的所有记录
    let records = db::get_table_records_by_person(&conn, person_id)
        .map_err(|e| e.to_string())?;

    // 构建 Excel 数据
    let mut rows: Vec<Vec<String>> = Vec::new();

    for record in records {
        // 解析 JSON 内容
        if let Ok(content) = serde_json::from_str::<serde_json::Value>(&record.content) {
            // 获取表头
            if let Some(headers) = content["headers"].as_array() {
                let header_row: Vec<String> = headers
                    .iter()
                    .map(|h| h.as_str().unwrap_or("").to_string())
                    .collect();
                if !header_row.is_empty() && rows.is_empty() {
                    rows.push(header_row);
                }
            }

            // 获取数据行
            if let Some(data_rows) = content["rows"].as_array() {
                for row in data_rows {
                    let row_data: Vec<String> = row
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|cell| cell.as_str().unwrap_or("").to_string())
                        .collect();
                    if !row_data.is_empty() {
                        rows.push(row_data);
                    }
                }
            }
        }
    }

    // 使用 simple_xlsx 生成 Excel 文件
    let mut workbook = simple_xlsx::Workbook::create(&file_path);

    // 创建工作表
    let mut sheet = workbook.create_sheet("Sheet1");

    // 写入数据
    for row in &rows {
        sheet.write_row(row.iter().map(|s| s.as_str()))?;
    }

    // 保存文件
    workbook.save()
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 导出人员数据到 CSV 文件
#[tauri::command]
pub fn export_to_csv(
    person_id: i32,
    file_path: String,
    state: State<DbState>,
) -> Result<(), String> {
    let conn = state.get_conn()?;

    // 获取人员的所有记录
    let records = db::get_table_records_by_person(&conn, person_id)
        .map_err(|e| e.to_string())?;

    // 生成 CSV 内容
    let mut csv = String::new();

    for record in records {
        // 解析 JSON 内容
        if let Ok(content) = serde_json::from_str::<serde_json::Value>(&record.content) {
            // 写入表头
            if let Some(headers) = content["headers"].as_array() {
                let header_row: Vec<String> = headers
                    .iter()
                    .map(|h| h.as_str().unwrap_or("").to_string())
                    .collect();
                csv.push_str(&escape_csv_row(&header_row));
                csv.push('\n');
            }

            // 写入数据行
            if let Some(data_rows) = content["rows"].as_array() {
                for row in data_rows {
                    let row_data: Vec<String> = row
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|cell| cell.as_str().unwrap_or("").to_string())
                        .collect();
                    csv.push_str(&escape_csv_row(&row_data));
                    csv.push('\n');
                }
            }

            // 每个记录之间添加分隔
            csv.push('\n');
        }
    }

    // 写入文件（处理 UTF-8 BOM 便于 Excel 正确识别）
    let bom = "\u{FEFF}";
    std::fs::write(&file_path, format!("{}{}", bom, csv))
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 转义 CSV 行中的特殊字符
fn escape_csv_row(cells: &[String]) -> String {
    cells
        .iter()
        .map(|cell| {
            let escaped = cell
                .replace('"', "\"\"")
                .replace('\n', " ")
                .replace('\r', " ");
            if escaped.contains(',') || escaped.contains('"') || escaped.contains('\n') {
                format!("\"{}\"", escaped)
            } else {
                escaped
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}
