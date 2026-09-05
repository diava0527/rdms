//! 工时接口处理器。
//!
//! 归属：成员 B

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::error::{ApiError, ApiResult};
use crate::models::{Attendance, NewAttendance};
use crate::services::attendance_service;

/// GET /api/attendance 的查询参数
#[derive(Debug, Deserialize)]
pub struct AttendanceQuery {
    pub user_id: Option<i64>,
    pub task_id: Option<i64>,
}

/// POST /api/attendance
pub async fn create_attendance(
    State(pool): State<SqlitePool>,
    payload: Result<Json<NewAttendance>, axum::extract::rejection::JsonRejection>,
) -> ApiResult<Json<Attendance>> {
    let Json(payload) = payload.map_err(ApiError::from)?;
    Ok(Json(
        attendance_service::create_attendance(&pool, payload).await?,
    ))
}

/// GET /api/attendance?user_id=1&task_id=2
pub async fn list_attendance(
    State(pool): State<SqlitePool>,
    q: Result<Query<AttendanceQuery>, axum::extract::rejection::QueryRejection>,
) -> ApiResult<Json<Vec<Attendance>>> {
    let Query(q) = q.map_err(ApiError::from)?;
    Ok(Json(
        attendance_service::list_attendance(&pool, q.user_id, q.task_id).await?,
    ))
}
