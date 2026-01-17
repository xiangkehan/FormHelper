// OCR 核心模块 - Tesseract 图片文字识别
// 使用 Tesseract 引擎识别图片中的文字内容

use tesseract::TesseractApi;
use crate::ocr::ExtractedTable;

/// OCR 识别配置
#[derive(Debug, Clone)]
pub struct OcrConfig {
    pub lang: String,    // 识别语言，如 "eng"（英语）、"chi_sim"（简体中文）
    pub psm: i32,        // 页面分割模式，6 表示自动分页
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
#[derive(Debug)]
pub struct OcrResult {
    pub text: String,        // 识别出的文本内容
    pub confidence: f32,     // 识别置信度（0.0 - 100.0）
}

/// 使用 Tesseract 进行图片 OCR 识别
///
/// # 参数
/// * `image_path` - 图片文件路径
/// * `config` - OCR 识别配置
///
/// # 返回
/// 识别文本和置信度
pub fn recognize_image(image_path: &str, config: OcrConfig) -> Result<OcrResult, String> {
    // 初始化 Tesseract 引擎
    let mut tess = TesseractApi::new(None).map_err(|e| e.to_string())?;

    // 初始化语言数据
    tess.initialize_default().map_err(|e| e.to_string())?;

    // 设置识别语言
    tess.set_language(&config.lang).map_err(|e| e.to_string())?;

    // 设置页面分割模式
    tess.set_page_segmentation_mode(config.psm);

    // 设置图片路径
    tess.set_image(image_path).map_err(|e| e.to_string())?;

    // 获取识别文本
    let text: String = tess.get_text().map_err(|e| e.to_string())?;

    // 获取置信度
    let confidence = tess.get_mean_confidence().unwrap_or(0.0);

    Ok(OcrResult {
        text: text.trim().to_string(),
        confidence,
    })
}

/// 将 OCR 结果转换为表格格式
/// 当 OCR 识别大段文本时，按行分割转为单列表格
pub fn ocr_result_to_table(result: OcrResult) -> ExtractedTable {
    let lines: Vec<Vec<String>> = result
        .text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| vec![line.trim().to_string()])
        .collect();

    ExtractedTable { rows: lines }
}

/// 从图片中检测表格区域（简单实现）
/// 当前版本返回整张图片作为一个表格区域
///
/// # 返回
/// 表格区域列表，每个区域包含坐标和尺寸
pub fn detect_table_regions(image_path: &str) -> Result<Vec<TableRegion>, String> {
    // 获取图片尺寸（简单实现：返回整张图片）
    // 实际应用中可使用图像处理库分析表格线条
    Ok(vec![TableRegion {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }])
}

/// 表格区域结构体
#[derive(Debug, Clone)]
pub struct TableRegion {
    pub x: u32,          // 区域左上角 X 坐标
    pub y: u32,          // 区域左上角 Y 坐标
    pub width: u32,      // 区域宽度
    pub height: u32,     // 区域高度
}
