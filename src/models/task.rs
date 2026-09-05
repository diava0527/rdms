//! 任务模型：研发任务的下发、认领、状态流转。
//!
//! 归属：成员 A

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "todo",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::InReview => "in_review",
            TaskStatus::Done => "done",
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "todo" => Ok(TaskStatus::Todo),
            "in_progress" => Ok(TaskStatus::InProgress),
            "in_review" => Ok(TaskStatus::InReview),
            "done" => Ok(TaskStatus::Done),
            other => Err(format!("未知任务状态: {other}")),
        }
    }
}

/// 任务优先级
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskPriority::Low => "low",
            TaskPriority::Medium => "medium",
            TaskPriority::High => "high",
            TaskPriority::Urgent => "urgent",
        }
    }
}

impl std::str::FromStr for TaskPriority {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(TaskPriority::Low),
            "medium" => Ok(TaskPriority::Medium),
            "high" => Ok(TaskPriority::High),
            "urgent" => Ok(TaskPriority::Urgent),
            other => Err(format!("未知任务优先级: {other}")),
        }
    }
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

/// 数据库行 → 实体 的中间结构（枚举列以字符串读取）
#[derive(sqlx::FromRow)]
pub(crate) struct RawTask {
    id: i64,
    title: String,
    description: String,
    project_id: i64,
    assignee_id: Option<i64>,
    status: String,
    priority: String,
    estimate_hours: f64,
    due_date: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
}

impl TryFrom<RawTask> for Task {
    type Error = ApiError;
    fn try_from(r: RawTask) -> Result<Self, Self::Error> {
        Ok(Task {
            id: r.id,
            title: r.title,
            description: r.description,
            project_id: r.project_id,
            assignee_id: r.assignee_id,
            status: r.status.parse().map_err(ApiError::Internal)?,
            priority: r.priority.parse().map_err(ApiError::Internal)?,
            estimate_hours: r.estimate_hours,
            due_date: r.due_date,
            created_at: r.created_at,
        })
    }
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
