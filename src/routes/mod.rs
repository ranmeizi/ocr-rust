use crate::controllers::ocr;
use axum::{
    routing::{get, post},
    Router,
};

pub fn compose() -> Router {
    Router::new().nest("/ocr", post_api())
}

pub fn post_api() -> Router {
    Router::new()
        .route("/sqsd", get(ocr::ocr_handler))
        .route("/sqsd/pure", post(ocr::ocr_pure))
        .route("/sqsd/base64", post(ocr::ocr_base64))
        .route("/sqsd/oss", post(ocr::ocr_cloud_oss))
}
