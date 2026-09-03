//! 团队成员（研发人员）模型。
//!
//! 归属：成员 A

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// 团队成员角色
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// 管理员：拥有全部权限
    Admin,
    /// 项目经理：负责项目与任务分配
    ProjectManager,
    /// 研发工程师：执行任务、填报工时
    Engineer,
}

/// 团队成员实体（对应数据库 users 表）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    /// 姓名
    pub name: String,
    /// 角色
    pub role: Role,
    /// 所属部门（多学科背景，如：软件组 / 硬件组 / 算法组）
    pub department: String,
    /// 邮箱
    pub email: String,
    /// 创建时间
    pub created_at: NaiveDateTime,
}

/// 新增成员的请求体
#[derive(Debug, Clone, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub role: Role,
    pub department: String,
    pub email: String,
}

/// 更新成员的请求体（字段均为可选）
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub role: Option<Role>,
    pub department: Option<String>,
    pub email: Option<String>,
}
