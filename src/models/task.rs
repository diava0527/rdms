//! 任务模型：研发任务的下发、认领、状态流转。
//!
//! 归属：成员 A

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// 任务状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    /// 待认领
    Todo,
    /// 进行中
    InProgress,
    /// 待评审
    InReview,
    /// 已完成
    Done,
}

/// 任务优先级
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

/// 任务实体（对应数据库 tasks 表）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    /// 任务标题
    pub title: String,
    /// 任务描述
    pub description: String,
    /// 所属项目 ID
    pub project_id: i64,
    /// 负责人（工程师）ID
    pub assignee_id: Option<i64>,
    /// 状态
    pub status: TaskStatus,
    /// 优先级
    pub priority: TaskPriority,
    /// 预估工时（小时）
    pub estimate_hours: f64,
    /// 截止时间
    pub due_date: Option<NaiveDateTime>,
    /// 创建时间
    pub created_at: NaiveDateTime,
}

/// 新增任务请求体
#[derive(Debug, Clone, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub project_id: i64,
    pub assignee_id: Option<i64>,
    pub priority: TaskPriority,
    pub estimate_hours: f64,
    pub due_date: Option<NaiveDateTime>,
}

/// 更新任务请求体
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub assignee_id: Option<i64>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub estimate_hours: Option<f64>,
    pub due_date: Option<NaiveDateTime>,
}
