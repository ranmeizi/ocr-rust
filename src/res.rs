use super::error::{CustErr};
use axum::{
    body::{self},
    extract::rejection::{JsonRejection, QueryRejection},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fmt::Debug, println};
// use std::error::Error;
use anyhow::Error;

/**
 * 统一响应结构
 */
#[derive(Debug, Serialize)]
pub struct Res<T: Serialize = ()> {
    pub code: Option<u16>,
    pub data: Option<T>,
    pub msg: Option<String>,
}

impl<T: Serialize> Res<T> {
    /**
     * 普遍成功body
     */
    pub fn success(data: T) -> Self {
        Self {
            code: Some(StatusCode::OK.as_u16()),
            data: Some(data),
            msg: Some(String::from("success")),
        }
    }

    /**
     * 普遍成功body 带有msg
     */
    pub fn success_msg(data: T, msg: &str) -> Self {
        Self {
            code: Some(StatusCode::OK.as_u16()),
            data: Some(data),
            msg: Some(String::from(msg)),
        }
    }

    /**
     * 自定义错误body
     */
    pub fn error(e: Error) -> Self {
        // 判断code值 默认为500，因为预期之外的错误统一归为服务端错误
        let code = if e.downcast_ref::<CustErr>().is_some() {
            match e.downcast_ref::<CustErr>() {
                // Some(CustErr::ReqParamError(_)) => 400,
                // Some(CustErr::ReqDeleteFail(_)) => 400,
                _ => 400,
            }
        } else {
            500
        };

        Self {
            code: Some(code),
            data: None,
            msg: Some(e.to_string()),
        }
    }
}

/**
 * 解构 JsonRejection 时的响应结构
 */
impl From<JsonRejection> for Res<()> {
    fn from(value: JsonRejection) -> Self {
        Self::error(CustErr::ReqParamError(value.body_text()).into())
    }
}

/**
 * 解构 QueryRejection 时的响应结构
 */
impl From<QueryRejection> for Res<()> {
    fn from(value: QueryRejection) -> Self {
        Self::error(CustErr::ReqParamError(value.body_text()).into())
    }
}

impl<T: Serialize> IntoResponse for Res<T> {
    fn into_response(self) -> axum::response::Response {
        let payload = json!(self);

        if self.code.unwrap() == 401 {
            (StatusCode::UNAUTHORIZED, Json(payload)).into_response()
        } else {
            (StatusCode::OK, Json(payload)).into_response()
        }
    }
}
