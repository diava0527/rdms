//! 成员接口处理器。
//!
//! 归属：成员 B

use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::error::ApiResult;
use crate::models::{NewUser, UpdateUser, User};
use crate::services::user_service;

/// POST /api/users
pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<NewUser>,
) -> ApiResult<Json<User>> {
    todo!("实现：调用 user_service::create_user")
}

/// GET /api/users
pub async fn list_users(State(pool): State<SqlitePool>) -> ApiResult<Json<Vec<User>>> {
    todo!("实现：调用 user_service::list_users")
}

/// GET /api/users/:id
pub async fn get_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> ApiResult<Json<User>> {
    todo!("实现：调用 user_service::get_user")
}

/// PUT /api/users/:id
pub async fn update_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUser>,
) -> ApiResult<Json<User>> {
    todo!("实现：调用 user_service::update_user")
}

/// DELETE /api/users/:id
pub async fn delete_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> ApiResult<Json<()>> {
    todo!("实现：调用 user_service::delete_user")
}
