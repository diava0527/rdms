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

    #[error("序列化错误: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("内部错误: {0}")]
    Internal(String),

    #[error("{message}")]
    Request { status: StatusCode, message: String },
}

/// 将 ApiError 转换为 HTTP 响应（JSON 格式）
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::Serde(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::Database(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "资源不存在".to_string())
            }
            ApiError::Database(_) | ApiError::Internal(_) => {
                tracing::error!(error = %self, "请求处理失败");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "服务器内部错误".to_string(),
                )
            }
            ApiError::Request { status, message } => (*status, message.clone()),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

/// 类型别名：所有 handler 的返回值
pub type ApiResult<T> = Result<T, ApiError>;

macro_rules! request_rejection {
    ($rejection:ty) => {
        impl From<$rejection> for ApiError {
            fn from(value: $rejection) -> Self {
                Self::Request {
                    status: value.status(),
                    message: value.body_text(),
                }
            }
        }
    };
}

request_rejection!(axum::extract::rejection::JsonRejection);
request_rejection!(axum::extract::rejection::PathRejection);
request_rejection!(axum::extract::rejection::QueryRejection);

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn errors_have_json_status_and_hide_internal_details() {
        for (error, status, message) in [
            (
                ApiError::NotFound("成员".into()),
                StatusCode::NOT_FOUND,
                "资源不存在: 成员",
            ),
            (
                ApiError::BadRequest("邮箱非法".into()),
                StatusCode::BAD_REQUEST,
                "参数非法: 邮箱非法",
            ),
            (
                ApiError::Database(sqlx::Error::RowNotFound),
                StatusCode::NOT_FOUND,
                "资源不存在",
            ),
            (
                ApiError::Database(sqlx::Error::PoolClosed),
                StatusCode::INTERNAL_SERVER_ERROR,
                "服务器内部错误",
            ),
            (
                ApiError::Internal("secret SQL or path".into()),
                StatusCode::INTERNAL_SERVER_ERROR,
                "服务器内部错误",
            ),
        ] {
            let response = error.into_response();
            assert_eq!(response.status(), status);
            assert_eq!(response.headers()["content-type"], "application/json");
            let body = axum::body::to_bytes(response.into_body(), 4096)
                .await
                .unwrap();
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
                json!({"error": message})
            );
        }
    }
}
