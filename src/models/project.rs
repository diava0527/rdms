//! 项目模型：体现“工程管理原理”——进度、里程碑、负责人、成本预算。
//!
//! 归属：成员 A

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// 项目状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
    /// 里程碑（以 JSON 文本存储，如 ["需求评审","原型","上线"]）
    pub milestones: String,
    /// 项目总预算（元）——用于经济决策
    pub budget: f64,
    /// 创建时间
    pub created_at: NaiveDateTime,
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
