//! 工时管理业务逻辑。
//!
//! 归属：成员 A

use sqlx::SqlitePool;

use crate::error::ApiError;
use crate::models::{Attendance, NewAttendance};

/// 记录工时
pub async fn create_attendance(
    pool: &SqlitePool,
    new_attendance: NewAttendance,
) -> Result<Attendance, ApiError> {
    todo!("实现：INSERT INTO attendance")
}

/// 查询工时（可按 user_id 或 task_id 过滤）
pub async fn list_attendance(
    pool: &SqlitePool,
    user_id: Option<i64>,
    task_id: Option<i64>,
) -> Result<Vec<Attendance>, ApiError> {
    todo!("实现：按条件过滤查询")
}

/// 统计某成员某段时间的总工时（供成本核算调用）
pub async fn sum_hours_by_user(
    pool: &SqlitePool,
    user_id: i64,
    start: &str,
    end: &str,
) -> Result<f64, ApiError> {
    todo!("实现：SELECT SUM(hours) FROM attendance WHERE user_id = ? AND work_date BETWEEN ? AND ?")
}
