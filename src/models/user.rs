//! 团队成员（研发人员）模型。
//!
//! 归属：成员 A

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

/// 团队成员角色
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// 管理员：拥有全部权限
    Admin,
    /// 项目经理：负责项目与任务分配
    ProjectManager,
    /// 研发工程师：执行任务、填报工时
    Engineer,
}

impl Role {
    /// 数据库/JSON 中的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::ProjectManager => "project_manager",
            Role::Engineer => "engineer",
        }
    }
}

impl std::str::FromStr for Role {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "project_manager" => Ok(Role::ProjectManager),
            "engineer" => Ok(Role::Engineer),
            other => Err(format!("未知角色: {other}")),
        }
    }
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

/// 数据库行 → 实体 的中间结构（枚举列以字符串读取）
#[derive(sqlx::FromRow)]
pub(crate) struct RawUser {
    id: i64,
    name: String,
    role: String,
    department: String,
    email: String,
    created_at: NaiveDateTime,
}

impl TryFrom<RawUser> for User {
    type Error = ApiError;
    fn try_from(r: RawUser) -> Result<Self, Self::Error> {
        Ok(User {
            id: r.id,
            name: r.name,
            role: r.role.parse().map_err(ApiError::Internal)?,
            department: r.department,
            email: r.email,
            created_at: r.created_at,
        })
    }
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
