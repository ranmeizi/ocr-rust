use crate::error::OcrErr;
use crate::res::Res;
use crate::services::{cv, tesseract};
use axum::{body::Bytes, extract::Json, response::IntoResponse};
use axum_extra::extract::WithRejection;
use opencv::{
    imgcodecs::{self, imwrite},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::process::{Command, Stdio};

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

#[axum::debug_handler]
pub async fn ocr_pet_cloud(Json(param): Json<CloudFileParams>) -> impl IntoResponse {
    // 加载图像
    let src = tesseract::TesseractService::get_mat_from_oss(param.fileid).await.unwrap();

    // 图像识别
    let res = tesseract::TesseractService::ocr_pet_growth(src).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

#[derive(Debug)]
struct OcrParams {
    r#type: Option<String>,
    img: Option<Bytes>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64Params {
    r#type: Option<String>,
    img: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudFileParams {
    fileid: String,
}
