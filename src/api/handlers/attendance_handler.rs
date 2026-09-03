//! 工时接口处理器。
//!
//! 归属：成员 B

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::error::ApiResult;
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
    Json(payload): Json<NewAttendance>,
) -> ApiResult<Json<Attendance>> {
    todo!("实现：调用 attendance_service::create_attendance")
}

/// GET /api/attendance?user_id=1&task_id=2
pub async fn list_attendance(
    State(pool): State<SqlitePool>,
    Query(q): Query<AttendanceQuery>,
) -> ApiResult<Json<Vec<Attendance>>> {
    todo!("实现：调用 attendance_service::list_attendance(pool, q.user_id, q.task_id)")
}
