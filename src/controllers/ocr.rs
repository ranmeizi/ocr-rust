use crate::res::Res;
use crate::services::ocr::OCR;
use crate::util::props_extractor;
use axum::{
    body::Bytes,
    extract::{Json, Multipart, Query},
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use base64_url;
use opencv::{
    core::{in_range, Mat, Scalar, Vector},
    imgcodecs::{self, imwrite},
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

    if let Ok(str) = result {
        json!(props_extractor::pet_info::pet_info(&str)).to_string()
    } else {
        "".to_owned()
    }
}

pub async fn ocr_base64(
    WithRejection(Json(param), _): WithRejection<Json<Base64Params>, Res<()>>,
) -> impl IntoResponse {
    let r#type = param.r#type.unwrap();
    let img = param.img.unwrap();

    let data = base64_url::base64::decode(img);

    let mat = OCR::binarization_vec(data.unwrap());
    let mut result = OCR::ocr(mat).await;

    if let Ok(str) = result {
        json!(props_extractor::pet_info::pet_info(&str)).to_string()
    } else {
        "".to_owned()
    }
}

pub async fn ocr_cloud_oss(
    WithRejection(Json(param), _): WithRejection<Json<CloudFileParams>, Res<()>>,
) -> impl IntoResponse {
    println!("in,{:?}", &param);
    let url_res = OCR::get_img_from_cloud(param.fileid).await;

    if let Err(x) = &url_res {
        println!("get_img_from_cloud, err msg{:?}", x);
    }

    let url = url_res.unwrap();

    let bytes_res = OCR::download_url(url).await;

    if let Err(x) = &bytes_res {
        println!("download_url, err msg{:?}", x);
    }

    let bytes = bytes_res.unwrap();

    let mat = OCR::binarization(bytes);

    let mut result = OCR::ocr(mat).await;

    if let Ok(str) = result {
        json!(props_extractor::pet_info::pet_info(&str)).to_string()
    } else {
        "".to_owned()
    }
}

/**
 * 宠物图鉴
 * 只关注成长值数据
 */
pub async fn ocr_pet_field_guide() -> impl IntoResponse {
    "hello"
    // 排序  总成长/生命/攻击/防御/敏捷

    // 取出
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64Params {
    r#type: Option<String>,
    img: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudFileParams {
    fileid: String,
}
