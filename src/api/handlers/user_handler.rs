//! 成员接口处理器。
//!
//! 归属：成员 B

use axum::extract::{Path, State};
use axum::Json;

use sqlx::SqlitePool;

use crate::error::{ApiError, ApiResult};
use crate::models::{NewUser, UpdateUser, User};
use crate::services::user_service;

/// POST /api/users
pub async fn create_user(
    State(pool): State<SqlitePool>,
    payload: Result<Json<NewUser>, axum::extract::rejection::JsonRejection>,
) -> ApiResult<Json<User>> {
    let Json(payload) = payload.map_err(ApiError::from)?;
    Ok(Json(user_service::create_user(&pool, payload).await?))
}

/// GET /api/users
pub async fn list_users(State(pool): State<SqlitePool>) -> ApiResult<Json<Vec<User>>> {
    Ok(Json(user_service::list_users(&pool).await?))
}

/// GET /api/users/:id
pub async fn get_user(
    State(pool): State<SqlitePool>,
    id: Result<Path<i64>, axum::extract::rejection::PathRejection>,
) -> ApiResult<Json<User>> {
    let Path(id) = id.map_err(ApiError::from)?;
    Ok(Json(user_service::get_user(&pool, id).await?))
}

/// PUT /api/users/:id
pub async fn update_user(
    State(pool): State<SqlitePool>,
    id: Result<Path<i64>, axum::extract::rejection::PathRejection>,
    payload: Result<Json<UpdateUser>, axum::extract::rejection::JsonRejection>,
) -> ApiResult<Json<User>> {
    let Path(id) = id.map_err(ApiError::from)?;
    let Json(payload) = payload.map_err(ApiError::from)?;
    Ok(Json(user_service::update_user(&pool, id, payload).await?))
}

/// DELETE /api/users/:id
pub async fn delete_user(
    State(pool): State<SqlitePool>,
    id: Result<Path<i64>, axum::extract::rejection::PathRejection>,
) -> ApiResult<axum::http::StatusCode> {
    let Path(id) = id.map_err(ApiError::from)?;
    user_service::delete_user(&pool, id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
