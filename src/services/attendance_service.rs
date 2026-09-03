//! 工时管理业务逻辑。
//!
//! 归属：成员 A

use sqlx::SqlitePool;

use crate::error::ApiResult;
use crate::models::attendance::{Attendance, NewAttendance};

/// 记录工时
pub async fn create_attendance(
    pool: &SqlitePool,
    new_attendance: NewAttendance,
) -> ApiResult<Attendance> {
    let result = sqlx::query(
        "INSERT INTO attendance (user_id, task_id, work_date, hours, note) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(new_attendance.user_id)
    .bind(new_attendance.task_id)
    .bind(new_attendance.work_date)
    .bind(new_attendance.hours)
    .bind(&new_attendance.note)
    .execute(pool)
    .await?;

    let row = sqlx::query_as::<_, Attendance>("SELECT * FROM attendance WHERE id = ?")
        .bind(result.last_insert_rowid())
        .fetch_one(pool)
        .await?;

    Ok(row)
}

/// 查询工时（可按 user_id 或 task_id 过滤，None 表示不过滤）
pub async fn list_attendance(
    pool: &SqlitePool,
    user_id: Option<i64>,
    task_id: Option<i64>,
) -> ApiResult<Vec<Attendance>> {
    let rows = sqlx::query_as::<_, Attendance>(
        "SELECT * FROM attendance \
         WHERE (? IS NULL OR user_id = ?) AND (? IS NULL OR task_id = ?) \
         ORDER BY work_date DESC",
    )
    .bind(user_id)
    .bind(user_id)
    .bind(task_id)
    .bind(task_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// 统计某成员某段时间的总工时（供成本核算调用）
pub async fn sum_hours_by_user(
    pool: &SqlitePool,
    user_id: i64,
    start: &str,
    end: &str,
) -> ApiResult<f64> {
    let row: (Option<f64>,) = sqlx::query_as(
        "SELECT SUM(hours) FROM attendance WHERE user_id = ? AND work_date BETWEEN ? AND ?",
    )
    .bind(user_id)
    .bind(start)
    .bind(end)
    .fetch_one(pool)
    .await?;

    Ok(row.0.unwrap_or(0.0))
}
