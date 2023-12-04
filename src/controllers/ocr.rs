use crate::res::Res;
use crate::util::props_extractor;
use axum::{
    body::Bytes,
    extract::{Json, Multipart, Query},
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use base64_url;
use cv_self::preprocessing::pet_info;
use opencv::{
    core::{in_range, Mat, Scalar, Vector},
    imgcodecs::{self, imwrite},
    imgproc,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
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
