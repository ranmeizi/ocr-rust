use axum::{body::Bytes, extract::Multipart, response::IntoResponse};
use opencv::{
    core::{in_range, Mat, Scalar, Vector},
    imgcodecs::{self, imwrite},
    prelude::*,
};
use std::io::Cursor;
use std::process::{Command, Stdio};
use crate::services::ocr::OCR;

pub async fn ocr_handler() -> impl IntoResponse {
    // "hello"
    // 调用
    let output = Command::new("tesseract")
        .arg("/tmp/hrl1.jpg")
        .arg("stdout")
        .output()
        .expect("error");
    // let std_out = tesseract_child.stdout.expect("Failed to open echo stdout");

    let outstr: String = String::from_utf8(output.stdout).unwrap();

    outstr
}

/**
 * 纯净的ocr
 * 完全不处理文字直接返回
 */
pub async fn ocr_pure(mut multipart: Multipart) -> impl IntoResponse {
    let mut params: OcrParams = OcrParams {
        r#type: None,
        img: None,
    };
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name == "type" {
            params.r#type = Some(String::from_utf8(data.to_vec()).unwrap());
        }

        if name == "img" {
            params.img = Some(data);
        }
    }

    let mat = OCR::binarization(params.img.unwrap());
    let mut result = OCR::ocr(mat).await;
    

    if let Ok(str) = result{
        str
    }else {
        "".to_owned()
    }
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

#[derive(Debug)]
struct OcrParams {
    r#type: Option<String>,
    img: Option<Bytes>,
}
