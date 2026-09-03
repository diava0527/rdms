//! 工时/考勤模型：记录研发人员投入，为成本核算提供数据来源。
//!
//! 归属：成员 A

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

/// 工时记录实体（对应数据库 attendance 表）
///
/// 该实体无枚举字段，可直接用 `#[derive(sqlx::FromRow)]` 映射数据库行。
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attendance {
    pub id: i64,
    /// 成员 ID
    pub user_id: i64,
    /// 关联任务 ID（可为空，表示非任务性工作）
    pub task_id: Option<i64>,
    /// 记录日期
    pub work_date: NaiveDate,
    /// 工时（小时）
    pub hours: f64,
    /// 工作内容说明
    pub note: String,
    /// 记录时间
    pub created_at: NaiveDateTime,
}

/// 新增工时记录请求体
#[derive(Debug, Clone, Deserialize)]
pub struct NewAttendance {
    pub user_id: i64,
    pub task_id: Option<i64>,
    pub work_date: NaiveDate,
    pub hours: f64,
    pub note: String,
}
