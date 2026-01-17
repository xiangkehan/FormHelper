// OCR 模块 - 公共定义和类型导出
// 提供 OCR、PDF、Word、Excel 解析功能的统一接口

pub mod ocr;
pub mod pdf;
pub mod word;
pub mod excel;

// 表格提取结果结构体
#[derive(Debug, Clone)]
pub struct ExtractedTable {
    pub rows: Vec<Vec<String>>,  // 表格数据，二维字符串数组
}

// 页面文本结构体（用于 PDF）
#[derive(Debug)]
pub struct PageText {
    pub page: u32,       // 页码
    pub text: String,    // 页面文本内容
}

// 检测到的表格结构体
#[derive(Debug)]
pub struct DetectedTable {
    pub page: u32,               // 所在页码
    pub rows: Vec<Vec<String>>,  // 表格数据
}
