//! 项目模型：体现“工程管理原理”——进度、里程碑、负责人、成本预算。
//!
//! 归属：成员 A

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

/// 项目状态
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    /// 未启动
    Pending,
    /// 进行中
    Active,
    /// 已暂停
    Paused,
    /// 已完成
    Done,
}

impl ProjectStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectStatus::Pending => "pending",
            ProjectStatus::Active => "active",
            ProjectStatus::Paused => "paused",
            ProjectStatus::Done => "done",
        }
    }
}

impl std::str::FromStr for ProjectStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(ProjectStatus::Pending),
            "active" => Ok(ProjectStatus::Active),
            "paused" => Ok(ProjectStatus::Paused),
            "done" => Ok(ProjectStatus::Done),
            other => Err(format!("未知项目状态: {other}")),
        }
    }
}

/// 项目实体（对应数据库 projects 表）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    /// 项目名称
    pub name: String,
    /// 项目描述
    pub description: String,
    /// 状态
    pub status: ProjectStatus,
    /// 负责人（项目经理）ID，外键指向 users.id
    pub manager_id: i64,
    /// 计划开始 / 结束日期
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    /// 里程碑（如 ["需求评审","原型","上线"]）
    pub milestones: Vec<String>,
    /// 项目总预算（元）——用于经济决策
    pub budget: f64,
    /// 创建时间
    pub created_at: NaiveDateTime,
}

/// 数据库行 → 实体 的中间结构（枚举列以字符串、milestones 以 JSON 文本读取）
#[derive(sqlx::FromRow)]
pub(crate) struct RawProject {
    id: i64,
    name: String,
    description: String,
    status: String,
    manager_id: i64,
    start_date: Option<NaiveDateTime>,
    end_date: Option<NaiveDateTime>,
    milestones: String,
    budget: f64,
    created_at: NaiveDateTime,
}

impl TryFrom<RawProject> for Project {
    type Error = ApiError;
    fn try_from(r: RawProject) -> Result<Self, Self::Error> {
        Ok(Project {
            id: r.id,
            name: r.name,
            description: r.description,
            status: r.status.parse().map_err(ApiError::Internal)?,
            manager_id: r.manager_id,
            start_date: r.start_date,
            end_date: r.end_date,
            milestones: serde_json::from_str(&r.milestones)?,
            budget: r.budget,
            created_at: r.created_at,
        })
    }
}

/// 新增项目请求体
#[derive(Debug, Clone, Deserialize)]
pub struct NewProject {
    pub name: String,
    pub description: String,
    pub manager_id: i64,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub milestones: Vec<String>,
    pub budget: f64,
}

/// 更新项目请求体
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<ProjectStatus>,
    pub manager_id: Option<i64>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub milestones: Option<Vec<String>>,
    pub budget: Option<f64>,
}
