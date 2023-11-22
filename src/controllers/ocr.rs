use axum::response::IntoResponse;
use std::process::{Command, Stdio};

pub async fn ocr_handler() -> impl IntoResponse {
    // "hello"
    // 调用
    let output = Command::new("tesseract")
        .arg("/tmp/hrl1.jpg stdout")
        .output()
        .expect("error");
    // let std_out = tesseract_child.stdout.expect("Failed to open echo stdout");

    let outstr:String = String::from_utf8(output.stdout).unwrap();

    outstr
}

/**
 * 纯净的ocr
 * 完全不处理文字直接返回
 */
pub async fn ocr_pure() -> impl IntoResponse {
    "hello"
}

/**
 * 宠物图鉴
 * 只关注成长值数据
 */
pub async fn ocr_pet_field_guide() -> impl IntoResponse {
    "hello"
}

/**
 * 人物面板
 * 只关人物属性值
 */
pub async fn ocr_stats_panel() -> impl IntoResponse {
    "hello"
}
