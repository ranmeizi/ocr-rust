use crate::controllers::ocr;
use axum::{Router,routing::get};

pub fn compose() -> Router {
    Router::new().nest("/ocr", post_api())
}

pub fn post_api() -> Router {
    Router::new().route("/sqsd", get(ocr::ocr_handler))
}
