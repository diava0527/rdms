//! 统一错误类型：把业务错误、数据库错误转换为 HTTP 响应。
//!
//! 归属：成员 B

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// 业务/接口错误
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("资源不存在: {0}")]
    NotFound(String),

    #[error("参数非法: {0}")]
    BadRequest(String),

    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("内部错误: {0}")]
    Internal(String),
}

/// 将 ApiError 转换为 HTTP 响应（JSON 格式）
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

/// 类型别名：所有 handler 的返回值
pub type ApiResult<T> = Result<T, ApiError>;
